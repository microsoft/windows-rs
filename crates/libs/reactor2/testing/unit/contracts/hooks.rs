use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn static_string_default() -> String {
    "static default".to_string()
}

static FIRST_STATIC_CONTEXT: ContextKey<String> = ContextKey::new(static_string_default);
static SECOND_STATIC_CONTEXT: ContextKey<String> = ContextKey::new(static_string_default);

fn local_rc_default() -> Rc<Cell<usize>> {
    Rc::new(Cell::new(7))
}

static LOCAL_RC_CONTEXT: ContextKey<Rc<Cell<usize>>> = ContextKey::new(local_rc_default);

static DEFAULT_FACTORY_CALLS: AtomicUsize = AtomicUsize::new(0);
static DEFAULT_DROPS: AtomicUsize = AtomicUsize::new(0);

struct DefaultProbe(usize);

impl Drop for DefaultProbe {
    fn drop(&mut self) {
        DEFAULT_DROPS.fetch_add(1, Ordering::Relaxed);
    }
}

fn tracked_default() -> Rc<DefaultProbe> {
    let id = DEFAULT_FACTORY_CALLS.fetch_add(1, Ordering::Relaxed);
    Rc::new(DefaultProbe(id))
}

static TRACKED_STATIC_CONTEXT: ContextKey<Rc<DefaultProbe>> = ContextKey::new(tracked_default);

struct ProviderProbe;

fn provider_probe_default() -> Rc<ProviderProbe> {
    Rc::new(ProviderProbe)
}

static PROVIDER_PROBE_CONTEXT: ContextKey<Rc<ProviderProbe>> =
    ContextKey::new(provider_probe_default);

#[test]
fn state_common_and_status_updates_schedule_the_same_live_render() {
    let state = Rc::new(RefCell::new(None::<State<usize>>));
    let state_for_render = Rc::clone(&state);
    let renders = Rc::new(Cell::new(0));
    let renders_for_render = Rc::clone(&renders);
    let root = component(move |cx| {
        renders_for_render.set(renders_for_render.get() + 1);
        let current = cx.use_state(|| 1usize);
        *state_for_render.borrow_mut() = Some(current.clone());
        text_block(current.value().to_string())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let state = state.borrow().as_ref().unwrap().clone();
    state.set(2);
    state.update(|value| *value += 3);
    assert_eq!(state.value(), 5);
    assert_eq!(renders.get(), 1);

    reactor.pump();
    assert_eq!(renders.get(), 2);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "5"))
    );

    assert!(state.try_set(4));
    assert!(state.try_update(|value| *value *= 2));
    assert_eq!(state.value(), 8);
    assert_eq!(renders.get(), 2);

    reactor.pump();
    assert_eq!(renders.get(), 3);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "8"))
    );
}

#[test]
fn state_common_updates_are_noops_and_status_updates_report_stale_after_unmount() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let child = Rc::new(RefCell::new(None::<State<usize>>));
    let visible_for_render = Rc::clone(&visible);
    let child_for_render = Rc::clone(&child);
    let root = component(move |cx| {
        let current = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(current.clone());
        if current.value() {
            let child = Rc::clone(&child_for_render);
            component(move |cx| {
                let current = cx.use_state(|| 1usize);
                *child.borrow_mut() = Some(current.clone());
                text_block(current.value().to_string())
            })
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stale = child.borrow().as_ref().unwrap().clone();

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();

    let called = Rc::new(Cell::new(false));
    let called_from_common_update = Rc::clone(&called);
    stale.set(2);
    stale.update(move |_| called_from_common_update.set(true));
    assert!(!called.get());

    let called_from_status_update = Rc::clone(&called);
    assert!(!stale.try_set(2));
    assert!(!stale.try_update(move |_| called_from_status_update.set(true)));
    assert!(!called.get());
}

#[test]
fn state_updates_report_stale_after_generation_replacement() {
    let replace = Rc::new(RefCell::new(None::<State<bool>>));
    let states = Rc::new(RefCell::new(Vec::<State<usize>>::new()));
    let replace_for_render = Rc::clone(&replace);
    let states_for_render = Rc::clone(&states);
    let root = component(move |cx| {
        let current = cx.use_state(|| false);
        *replace_for_render.borrow_mut() = Some(current.clone());
        if current.value() {
            let states = Rc::clone(&states_for_render);
            component(move |cx| {
                let current = cx.use_state(|| 2usize);
                states.borrow_mut().push(current.clone());
                text_block(current.value().to_string())
            })
        } else {
            let states = Rc::clone(&states_for_render);
            component(move |cx| {
                let current = cx.use_state(|| 1usize);
                states.borrow_mut().push(current.clone());
                text_block(current.value().to_string())
            })
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stale = states.borrow()[0].clone();

    assert!(replace.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let current = states.borrow()[1].clone();
    assert_eq!(stale.node().index(), current.node().index());
    assert_ne!(stale.node().generation(), current.node().generation());
    assert!(!stale.try_set(3));
    assert_eq!(current.value(), 2);
}

#[test]
fn state_updates_work_from_reentrant_callbacks() {
    let callback = Rc::new(RefCell::new(None::<Callback<()>>));
    let callback_for_render = Rc::clone(&callback);
    let renders = Rc::new(Cell::new(0));
    let renders_for_render = Rc::clone(&renders);
    let root = component(move |cx| {
        renders_for_render.set(renders_for_render.get() + 1);
        let first = cx.use_state(|| 0usize);
        let second = cx.use_state(|| 0usize);
        let update_second_state = second.clone();
        let update_second = cx.use_callback((), move |()| {
            assert!(update_second_state.try_update(|value| *value += 1));
        });
        let update_first = first.clone();
        let update_both = cx.use_callback((), move |()| {
            assert!(update_first.try_update(|value| {
                *value += 1;
                update_second.call(());
            }));
        });
        *callback_for_render.borrow_mut() = Some(update_both);
        text_block(format!("{}:{}", first.value(), second.value()))
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    callback.borrow().as_ref().unwrap().call(());
    reactor.pump();

    assert_eq!(renders.get(), 2);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "1:1"))
    );
}

#[test]
fn hook_ref_retains_values_without_scheduling_updates() {
    let reference = Rc::new(RefCell::new(None::<HookRef<NotClone>>));
    let clone_reference = Rc::new(RefCell::new(None::<HookRef<usize>>));
    let update = Rc::new(RefCell::new(None::<State<usize>>));
    let reference_for_render = Rc::clone(&reference);
    let clone_reference_for_render = Rc::clone(&clone_reference);
    let update_for_render = Rc::clone(&update);
    let root = component(move |cx| {
        let value = cx.use_ref(|| NotClone(1));
        let clone_value = cx.use_ref(|| 2usize);
        let generation = cx.use_state(|| 0usize);
        *reference_for_render.borrow_mut() = Some(value.clone());
        *clone_reference_for_render.borrow_mut() = Some(clone_value);
        *update_for_render.borrow_mut() = Some(generation.clone());
        text_block(format!(
            "{}:{}",
            value.with(|value| value.0).unwrap(),
            generation.get().unwrap()
        ))
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let batches = reactor.engine().runtime().batches().len();

    assert!(
        reference
            .borrow()
            .as_ref()
            .unwrap()
            .with_mut(|value| value.0 = 7)
            .is_some()
    );
    reactor.pump();
    assert_eq!(reactor.engine().runtime().batches().len(), batches);

    assert!(update.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "7:1"))
    );
    reference.borrow().as_ref().unwrap().set(NotClone(8));
    assert_eq!(
        reference.borrow().as_ref().unwrap().with(|value| value.0),
        Some(8)
    );
    assert!(reference.borrow().as_ref().unwrap().try_set(NotClone(9)));
    assert_eq!(
        reference.borrow().as_ref().unwrap().with(|value| value.0),
        Some(9)
    );
    assert_eq!(clone_reference.borrow().as_ref().unwrap().get(), Some(2));
    drop(reactor);
    reference.borrow().as_ref().unwrap().set(NotClone(10));
    assert!(!reference.borrow().as_ref().unwrap().try_set(NotClone(11)));
}

#[test]
fn element_reference_mounts_after_commit_and_clears_before_destroy() {
    let reference = Rc::new(RefCell::new(None::<ElementRef<TextBox>>));
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let log = Rc::new(RefCell::new(Vec::new()));
    let reference_for_render = Rc::clone(&reference);
    let visible_for_render = Rc::clone(&visible);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let visible = cx.use_state(|| true);
        let mounted_log = Rc::clone(&log_for_render);
        let unmounted_log = Rc::clone(&log_for_render);
        let reference = cx.use_element_ref_with_lifecycle::<TextBox>(
            move || mounted_log.borrow_mut().push("mounted"),
            move || unmounted_log.borrow_mut().push("unmounted"),
        );
        *reference_for_render.borrow_mut() = Some(reference.clone());
        *visible_for_render.borrow_mut() = Some(visible.clone());
        if visible.get().unwrap() {
            TextBox::new("value", |_| {}).reference(&reference).build()
        } else {
            text_block("hidden")
        }
    });
    let runtime = ReferenceRuntime {
        inner: RecordingRuntime::default(),
        reference: Rc::clone(&reference),
        log: Rc::clone(&log),
    };
    let mut reactor = Reactor::new(runtime, root);
    reactor.pump();
    let reference = reference.borrow().as_ref().unwrap().clone();
    let first = reference.node().unwrap();
    assert!(reference.is_mounted());
    assert_eq!(*log.borrow(), ["commit", "mounted"]);
    assert!(reference.focus());
    reactor.pump();
    assert_eq!(reactor.engine().runtime().inner.focused_elements(), [first]);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert!(!reference.is_mounted());
    assert!(!reference.focus());
    assert_eq!(
        *log.borrow(),
        ["commit", "mounted", "commit", "destroy", "unmounted"]
    );

    assert!(visible.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    let second = reference.node().unwrap();
    assert_eq!(first.index(), second.index());
    assert_ne!(first.generation(), second.generation());
    assert_eq!(
        *log.borrow(),
        [
            "commit",
            "mounted",
            "commit",
            "destroy",
            "unmounted",
            "destroy",
            "mounted"
        ]
    );
}

#[test]
fn retained_element_reference_swap_uses_committed_lifecycle_callbacks() {
    let first = Rc::new(RefCell::new(None::<ElementRef<TextBox>>));
    let second = Rc::new(RefCell::new(None::<ElementRef<TextBox>>));
    let select_second = Rc::new(RefCell::new(None::<State<bool>>));
    let log = Rc::new(RefCell::new(Vec::new()));
    let first_for_render = Rc::clone(&first);
    let second_for_render = Rc::clone(&second);
    let select_for_render = Rc::clone(&select_second);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let select_second = cx.use_state(|| false);
        let latest = select_second.get().unwrap();
        let first_mounted = Rc::clone(&log_for_render);
        let first_unmounted = Rc::clone(&log_for_render);
        let first = cx.use_element_ref_with_lifecycle::<TextBox>(
            move || first_mounted.borrow_mut().push("first mounted"),
            move || {
                first_unmounted.borrow_mut().push(if latest {
                    "first unmounted uncommitted"
                } else {
                    "first unmounted committed"
                });
            },
        );
        let second_mounted = Rc::clone(&log_for_render);
        let second_unmounted = Rc::clone(&log_for_render);
        let second = cx.use_element_ref_with_lifecycle::<TextBox>(
            move || second_mounted.borrow_mut().push("second mounted"),
            move || second_unmounted.borrow_mut().push("second unmounted"),
        );
        *first_for_render.borrow_mut() = Some(first.clone());
        *second_for_render.borrow_mut() = Some(second.clone());
        *select_for_render.borrow_mut() = Some(select_second);
        TextBox::new("value", |_| {})
            .reference(if latest { &second } else { &first })
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let batches = reactor.engine().runtime().batches().len();
    assert!(first.borrow().as_ref().unwrap().is_mounted());
    assert!(!second.borrow().as_ref().unwrap().is_mounted());

    assert!(select_second.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().batches().len(), batches);
    assert!(!first.borrow().as_ref().unwrap().is_mounted());
    assert!(second.borrow().as_ref().unwrap().is_mounted());
    assert_eq!(
        *log.borrow(),
        [
            "first mounted",
            "first unmounted committed",
            "second mounted"
        ]
    );
}

#[test]
fn element_references_publish_in_structural_order_after_keyed_reordering() {
    let reordered = Rc::new(RefCell::new(None::<State<bool>>));
    let reordered_for_render = Rc::clone(&reordered);
    let log = Rc::new(RefCell::new(Vec::new()));
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let reordered = cx.use_state(|| false);
        *reordered_for_render.borrow_mut() = Some(reordered.clone());
        let mut reference = |name: &'static str| {
            let log = Rc::clone(&log_for_render);
            cx.use_element_ref_with_lifecycle::<TextBox>(move || log.borrow_mut().push(name), || {})
        };
        let first = reference("first");
        let second = reference("second");
        let third = reference("third");
        let fourth = reference("fourth");
        if reordered.get().unwrap() {
            stack_panel([
                TextBox::new("second", |_| {})
                    .reference(&third)
                    .build()
                    .key(2),
                TextBox::new("first", |_| {})
                    .reference(&fourth)
                    .build()
                    .key(1),
            ])
        } else {
            stack_panel([
                TextBox::new("first", |_| {})
                    .reference(&first)
                    .build()
                    .key(1),
                TextBox::new("second", |_| {})
                    .reference(&second)
                    .build()
                    .key(2),
            ])
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    log.borrow_mut().clear();

    assert!(reordered.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert_eq!(*log.borrow(), ["third", "fourth"]);
}

#[test]
fn failed_mount_never_publishes_reference_or_lifecycle() {
    let reference = Rc::new(RefCell::new(None::<ElementRef<TextBox>>));
    let mounts = Rc::new(Cell::new(0));
    let unmounts = Rc::new(Cell::new(0));
    let reference_for_render = Rc::clone(&reference);
    let mounts_for_render = Rc::clone(&mounts);
    let unmounts_for_render = Rc::clone(&unmounts);
    let root = component(move |cx| {
        let mounts = Rc::clone(&mounts_for_render);
        let unmounts = Rc::clone(&unmounts_for_render);
        let reference = cx.use_element_ref_with_lifecycle::<TextBox>(
            move || mounts.set(mounts.get() + 1),
            move || unmounts.set(unmounts.get() + 1),
        );
        *reference_for_render.borrow_mut() = Some(reference.clone());
        TextBox::new("value", |_| {}).reference(&reference).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.engine().runtime().fail_next("mount failed");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));
    assert!(result.is_err());
    _ = (reference, mounts, unmounts);
}

#[test]
fn failed_update_unmounts_with_last_committed_lifecycle() {
    let reference = Rc::new(RefCell::new(None::<ElementRef<TextBox>>));
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let unmounted = Rc::new(RefCell::new(Vec::new()));
    let reference_for_render = Rc::clone(&reference);
    let version_for_render = Rc::clone(&version);
    let unmounted_for_render = Rc::clone(&unmounted);
    let root = component(move |cx| {
        let version = cx.use_state(|| 0usize);
        let current = version.get().unwrap();
        let unmounted = Rc::clone(&unmounted_for_render);
        let reference = cx.use_element_ref_with_lifecycle::<TextBox>(
            || {},
            move || unmounted.borrow_mut().push(current),
        );
        *reference_for_render.borrow_mut() = Some(reference.clone());
        *version_for_render.borrow_mut() = Some(version);
        TextBox::new(format!("value {current}"), |_| {})
            .reference(&reference)
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.engine().runtime().fail_next("update failed");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));
    assert!(result.is_err());
    _ = (reference, unmounted);
}

#[test]
fn duplicate_element_reference_attachment_panics() {
    let reference = Rc::new(RefCell::new(None::<ElementRef<TextBox>>));
    let reference_for_render = Rc::clone(&reference);
    let root = component(move |cx| {
        let reference = cx.use_element_ref::<TextBox>();
        *reference_for_render.borrow_mut() = Some(reference.clone());
        stack_panel([
            TextBox::new("first", |_| {}).reference(&reference).build(),
            TextBox::new("second", |_| {}).reference(&reference).build(),
        ])
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));
    assert!(panic.is_err());
    assert!(!reference.borrow().as_ref().unwrap().is_mounted());
}

#[test]
fn memo_recomputes_only_when_dependencies_change() {
    let dependency = Rc::new(RefCell::new(None::<State<usize>>));
    let unrelated = Rc::new(RefCell::new(None::<State<usize>>));
    let dependency_for_render = Rc::clone(&dependency);
    let unrelated_for_render = Rc::clone(&unrelated);
    let calls = Rc::new(Cell::new(0));
    let calls_for_render = Rc::clone(&calls);
    let root = component(move |cx| {
        let dependency = cx.use_state(|| 1usize);
        let unrelated = cx.use_state(|| 0usize);
        *dependency_for_render.borrow_mut() = Some(dependency.clone());
        *unrelated_for_render.borrow_mut() = Some(unrelated.clone());
        let current = dependency.get().unwrap();
        let calls = Rc::clone(&calls_for_render);
        let memo = cx.use_memo(current, move || {
            calls.set(calls.get() + 1);
            current * 10
        });
        text_block(format!("{memo}:{}", unrelated.get().unwrap()))
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(calls.get(), 1);

    assert!(unrelated.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(calls.get(), 1);

    assert!(dependency.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(calls.get(), 2);
}

#[test]
fn callback_identity_and_capture_follow_dependencies() {
    let dependency = Rc::new(RefCell::new(None::<State<usize>>));
    let unrelated = Rc::new(RefCell::new(None::<State<usize>>));
    let callback = Rc::new(RefCell::new(None::<Callback<usize>>));
    let dependency_for_render = Rc::clone(&dependency);
    let unrelated_for_render = Rc::clone(&unrelated);
    let callback_for_render = Rc::clone(&callback);
    let seen = Rc::new(RefCell::new(Vec::new()));
    let seen_for_render = Rc::clone(&seen);
    let root = component(move |cx| {
        let dependency = cx.use_state(|| 1usize);
        let unrelated = cx.use_state(|| 0usize);
        *dependency_for_render.borrow_mut() = Some(dependency.clone());
        *unrelated_for_render.borrow_mut() = Some(unrelated.clone());
        let current = dependency.get().unwrap();
        let seen = Rc::clone(&seen_for_render);
        let current_callback = cx.use_callback(current, move |value| {
            seen.borrow_mut().push(current + value);
        });
        *callback_for_render.borrow_mut() = Some(current_callback);
        text_block(unrelated.get().unwrap().to_string())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let first = callback.borrow().as_ref().unwrap().clone();
    first.call(10);
    assert_eq!(*seen.borrow(), [11]);

    assert!(unrelated.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let unchanged = callback.borrow().as_ref().unwrap().clone();
    assert!(first.ptr_eq(&unchanged));

    assert!(dependency.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let changed = callback.borrow().as_ref().unwrap().clone();
    assert!(!first.ptr_eq(&changed));
    changed.call(10);
    assert_eq!(*seen.borrow(), [11, 12]);
}

#[test]
fn reducer_dispatch_uses_current_state_and_stable_identity() {
    let dispatch = Rc::new(RefCell::new(None::<Callback<usize>>));
    let dispatch_for_render = Rc::clone(&dispatch);
    let root = component(move |cx| {
        let (value, current_dispatch) = cx.use_reducer(|| 1usize, |value, action| value + action);
        *dispatch_for_render.borrow_mut() = Some(current_dispatch);
        text_block(value.to_string())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let first = dispatch.borrow().as_ref().unwrap().clone();

    first.call(2);
    reactor.pump();
    let second = dispatch.borrow().as_ref().unwrap().clone();
    assert!(first.ptr_eq(&second));
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "3"))
    );

    second.call(4);
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "7"))
    );
}

#[test]
fn effects_run_after_native_commit() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let effect_log = Rc::clone(&log);
    let root = component(move |cx| {
        let log = Rc::clone(&effect_log);
        cx.use_effect((), move || log.borrow_mut().push("effect"));
        text_block("effect")
    });
    let runtime = OrderedRuntime {
        inner: RecordingRuntime::default(),
        log: Rc::clone(&log),
    };
    let mut reactor = Reactor::new(runtime, root);

    reactor.pump();

    assert_eq!(*log.borrow(), ["commit", "effect"]);
}

#[test]
fn changed_effect_dependencies_clean_up_before_restarting() {
    let dependency = Rc::new(RefCell::new(None::<State<usize>>));
    let dependency_for_render = Rc::clone(&dependency);
    let log = Rc::new(RefCell::new(Vec::new()));
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *dependency_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        let effect_log = Rc::clone(&log_for_render);
        cx.use_effect_with_cleanup(current, move || {
            effect_log.borrow_mut().push(format!("effect {current}"));
            move || effect_log.borrow_mut().push(format!("cleanup {current}"))
        });
        text_block(current.to_string())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(*log.borrow(), ["effect 0"]);

    reactor.pump();
    assert_eq!(*log.borrow(), ["effect 0"]);

    assert!(dependency.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(*log.borrow(), ["effect 0", "cleanup 0", "effect 1"]);
}

#[test]
fn timers_start_after_commit_with_their_requested_mode() {
    let root = component(|cx| {
        cx.use_timeout((), Duration::from_millis(10), || {});
        cx.use_interval((), Duration::from_millis(20), || {});
        text_block("timers")
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    let timers = reactor
        .engine()
        .runtime()
        .timers()
        .values()
        .collect::<Vec<_>>();
    assert_eq!(timers.len(), 2);
    assert_eq!(timers[0].interval, Duration::from_millis(10));
    assert!(!timers[0].repeating);
    assert_eq!(timers[1].interval, Duration::from_millis(20));
    assert!(timers[1].repeating);
}

#[test]
fn unchanged_timer_uses_the_latest_rendered_callback() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let fired = Rc::new(RefCell::new(Vec::new()));
    let fired_for_render = Rc::clone(&fired);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        let fired = Rc::clone(&fired_for_render);
        let message = format!("fired {current}");
        cx.use_timeout((), Duration::from_millis(10), move || {
            fired.borrow_mut().push(message);
        });
        text_block(current.to_string())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let initial = *reactor.engine().runtime().timers().values().next().unwrap();

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let current = *reactor.engine().runtime().timers().values().next().unwrap();
    assert_eq!(current, initial);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TimerFired {
            owner: current.owner,
            slot: current.slot,
            revision: current.revision,
        });
    reactor.pump();
    assert_eq!(*fired.borrow(), ["fired 1"]);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TimerFired {
            owner: current.owner,
            slot: current.slot,
            revision: current.revision,
        });
    reactor.pump();
    assert_eq!(*fired.borrow(), ["fired 1"]);
}

#[test]
fn changed_timer_dependencies_reject_stale_revisions() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let fired = Rc::new(RefCell::new(Vec::new()));
    let fired_for_render = Rc::clone(&fired);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        let fired = Rc::clone(&fired_for_render);
        cx.use_interval(current, Duration::from_millis(10), move || {
            fired.borrow_mut().push(current);
        });
        text_block(current.to_string())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stale = *reactor.engine().runtime().timers().values().next().unwrap();

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let current = *reactor.engine().runtime().timers().values().next().unwrap();
    assert_eq!(current.revision, stale.revision + 1);

    for timer in [stale, current] {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::TimerFired {
                owner: timer.owner,
                slot: timer.slot,
                revision: timer.revision,
            });
    }
    reactor.pump();
    assert_eq!(*fired.borrow(), [1]);
}

#[test]
fn unmount_stops_hook_owned_timers() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            component(|cx| {
                cx.use_interval((), Duration::from_millis(10), || {});
                text_block("timer")
            })
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(reactor.engine().runtime().timers().len(), 1);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    assert!(reactor.engine().runtime().timers().is_empty());
}

#[test]
fn async_state_accepts_off_thread_updates_and_rejects_stale_setters() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let setter = Arc::new(Mutex::new(None::<AsyncSetState<usize>>));
    let setter_for_render = Arc::clone(&setter);
    let root = component(move |cx| {
        let visible = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(visible.clone());
        if visible.value() {
            let setter = Arc::clone(&setter_for_render);
            component(move |cx| {
                let (value, set_value) = cx.use_async_state(0usize);
                *setter.lock().unwrap() = Some(set_value);
                text_block(format!("async {value}"))
            })
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let set_value = setter.lock().unwrap().as_ref().unwrap().clone();
    std::thread::spawn(move || set_value.set(21))
        .join()
        .unwrap();
    pump_until_text(&mut reactor, "async 21");

    let set_value = setter.lock().unwrap().as_ref().unwrap().clone();
    assert!(
        std::thread::spawn(move || set_value.try_set(42))
            .join()
            .unwrap()
    );
    pump_until_text(&mut reactor, "async 42");

    let stale = setter.lock().unwrap().as_ref().unwrap().clone();
    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    stale.set(98);
    assert!(!stale.try_set(99));
    reactor.pump();
    assert!(
        !reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| {
                text_update(command)
                    .is_some_and(|(_, text)| text == "async 98" || text == "async 99")
            })
    );
}

#[test]
fn mutation_reports_success_error_and_reset() {
    let trigger = Arc::new(Mutex::new(None::<MutationTrigger<usize>>));
    let trigger_for_render = Arc::clone(&trigger);
    let root = component(move |cx| {
        let (state, mutation) = cx.use_mutation::<usize>();
        *trigger_for_render.lock().unwrap() = Some(mutation);
        text_block(match state {
            MutationState::Idle => "idle".to_string(),
            MutationState::Loading => "loading".to_string(),
            MutationState::Success(value) => format!("success {value}"),
            MutationState::Error(error) => format!("error {error}"),
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    trigger.lock().unwrap().as_ref().unwrap().fire(|| Ok(7));
    pump_until_text(&mut reactor, "success 7");

    trigger
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .fire(|| Err("failed".to_string()));
    pump_until_text(&mut reactor, "error failed");

    trigger.lock().unwrap().as_ref().unwrap().reset();
    pump_until_text(&mut reactor, "idle");
}

#[test]
fn resource_dependencies_cancel_and_reject_stale_completions() {
    let dependency = Rc::new(RefCell::new(None::<State<usize>>));
    let dependency_for_render = Rc::clone(&dependency);
    let (first_sender, first_receiver) = mpsc::channel();
    let (second_sender, second_receiver) = mpsc::channel();
    let receivers = Arc::new(Mutex::new(BTreeMap::from([
        (0usize, first_receiver),
        (1usize, second_receiver),
    ])));
    let (started_sender, started_receiver) = mpsc::channel();
    let (cancelled_sender, cancelled_receiver) = mpsc::channel();
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *dependency_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        let receivers = Arc::clone(&receivers);
        let started = started_sender.clone();
        let cancelled = cancelled_sender.clone();
        let resource = cx.use_resource(current, move |token, dependency| {
            let receiver = receivers.lock().unwrap().remove(&dependency).unwrap();
            started.send(dependency).unwrap();
            let result = receiver.recv().unwrap();
            cancelled.send((dependency, token.is_cancelled())).unwrap();
            result
        });
        text_block(match resource {
            Resource::Loading => "loading".to_string(),
            Resource::Ready(value) => format!("ready {}", *value),
            Resource::Failed(_) => "failed".to_string(),
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(
        started_receiver
            .recv_timeout(Duration::from_secs(1))
            .unwrap(),
        0
    );

    assert!(dependency.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        started_receiver
            .recv_timeout(Duration::from_secs(1))
            .unwrap(),
        1
    );

    second_sender.send(Ok(1usize)).unwrap();
    pump_until_text(&mut reactor, "ready 1");
    assert_eq!(
        cancelled_receiver
            .recv_timeout(Duration::from_secs(1))
            .unwrap(),
        (1, false)
    );

    first_sender.send(Ok(0usize)).unwrap();
    assert_eq!(
        cancelled_receiver
            .recv_timeout(Duration::from_secs(1))
            .unwrap(),
        (0, true)
    );
    reactor.pump();
    assert!(
        !reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "ready 0"))
    );
}

#[test]
fn resource_failures_reach_the_component_unchanged() {
    let root = component(|cx| {
        let resource = cx.use_resource((), |_token, ()| {
            Err::<usize, _>(windows_core::Error::new(
                windows_core::HRESULT(0x80004005_u32 as i32),
                "resource failure",
            ))
        });
        text_block(match resource {
            Resource::Loading => "loading",
            Resource::Ready(_) => "ready",
            Resource::Failed(error)
                if error.code() == windows_core::HRESULT(0x80004005_u32 as i32) =>
            {
                "expected failure"
            }
            Resource::Failed(_) => "wrong failure",
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    pump_until_text(&mut reactor, "expected failure");
}

#[test]
fn unmount_cancels_resource_work_and_ignores_its_completion() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let (release_sender, release_receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(Some(release_receiver)));
    let (started_sender, started_receiver) = mpsc::channel();
    let (cancelled_sender, cancelled_receiver) = mpsc::channel();
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            let receiver = Arc::clone(&receiver);
            let started = started_sender.clone();
            let cancelled = cancelled_sender.clone();
            component(move |cx| {
                let receiver = Arc::clone(&receiver);
                let started = started.clone();
                let cancelled = cancelled.clone();
                let resource = cx.use_resource((), move |token, ()| {
                    started.send(()).unwrap();
                    let result = receiver.lock().unwrap().take().unwrap().recv().unwrap();
                    cancelled.send(token.is_cancelled()).unwrap();
                    result
                });
                text_block(match resource {
                    Resource::Loading => "loading",
                    Resource::Ready(_) => "unexpected ready",
                    Resource::Failed(_) => "unexpected failure",
                })
            })
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    started_receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    release_sender
        .send(Ok::<_, windows_core::Error>(7usize))
        .unwrap();
    assert!(
        cancelled_receiver
            .recv_timeout(Duration::from_secs(1))
            .unwrap()
    );
    reactor.pump();
    assert!(
        !reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command)
                .is_some_and(|(_, text)| text == "unexpected ready"))
    );
}

#[test]
fn unmount_cleanup_runs_after_native_destruction() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let log = Rc::new(RefCell::new(Vec::new()));
    let log_for_runtime = Rc::clone(&log);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            let log = Rc::clone(&log_for_render);
            component(move |cx| {
                let log = Rc::clone(&log);
                cx.use_effect_with_cleanup((), move || move || log.borrow_mut().push("cleanup"));
                text_block("child")
            })
        } else {
            text_block("removed")
        }
    });
    let runtime = OrderedRuntime {
        inner: RecordingRuntime::default(),
        log: log_for_runtime,
    };
    let mut reactor = Reactor::new(runtime, root);
    reactor.pump();
    log.borrow_mut().clear();

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();

    assert_eq!(*log.borrow(), ["destroy", "cleanup"]);
}

#[test]
fn effect_state_updates_reach_quiescence_in_the_same_pump() {
    let root = component(|cx| {
        let state = cx.use_state(|| 0usize);
        let current = state.get().unwrap();
        cx.use_effect((), move || {
            if current == 0 {
                assert!(state.try_set(1));
            }
        });
        text_block(current.to_string())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "1"))
    );
}

#[test]
fn failed_native_commit_does_not_run_pending_effects() {
    let effects = Rc::new(Cell::new(0));
    let effects_for_render = Rc::clone(&effects);
    let root = component(move |cx| {
        let effects = Rc::clone(&effects_for_render);
        cx.use_effect((), move || effects.set(effects.get() + 1));
        text_block("effect")
    });
    let runtime = RecordingRuntime::default();
    runtime.fail_next("injected effect commit failure");
    let mut reactor = Reactor::new(runtime, root);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));
    assert!(result.is_err());
    assert_eq!(effects.get(), 0);
}

#[test]
fn committed_effect_cleans_up_after_later_native_failure() {
    let state = Rc::new(RefCell::new(None::<State<String>>));
    let state_for_render = Rc::clone(&state);
    let cleanups = Rc::new(Cell::new(0));
    let cleanups_for_render = Rc::clone(&cleanups);
    let root = component(move |cx| {
        let value = cx.use_state(|| "before".to_string());
        *state_for_render.borrow_mut() = Some(value.clone());
        let cleanups = Rc::clone(&cleanups_for_render);
        cx.use_effect_with_cleanup((), move || move || cleanups.set(cleanups.get() + 1));
        text_block(value.get().unwrap())
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .fail_next("injected update failure");
    assert!(
        state
            .borrow()
            .as_ref()
            .unwrap()
            .try_set("after".to_string())
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));
    assert!(result.is_err());
    assert_eq!(cleanups.get(), 0);
}

#[test]
fn effect_panic_is_terminal() {
    let root = component(|cx| {
        cx.use_effect((), || panic!("injected effect panic"));
        text_block("effect")
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));

    assert!(result.is_err());
}

#[test]
fn cleanup_panic_does_not_skip_other_retired_cleanups() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let completed = Rc::new(Cell::new(0));
    let completed_for_render = Rc::clone(&completed);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            let completed = Rc::clone(&completed_for_render);
            stack_panel([
                component(|cx| {
                    cx.use_effect_with_cleanup((), || || panic!("injected cleanup panic"));
                    text_block("first")
                }),
                component(move |cx| {
                    let completed = Rc::clone(&completed);
                    cx.use_effect_with_cleanup((), move || {
                        move || completed.set(completed.get() + 1)
                    });
                    text_block("second")
                }),
            ])
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert!(visible.borrow().as_ref().unwrap().try_set(false));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));

    assert!(result.is_err());
    assert_eq!(completed.get(), 1);
}

#[test]
fn virtual_row_recycling_runs_effect_cleanup() {
    let cleanups = Rc::new(Cell::new(0));
    let cleanups_for_render = Rc::clone(&cleanups);
    let root = virtual_list(100, 300.0, move |index| {
        let cleanups = Rc::clone(&cleanups_for_render);
        component(move |cx| {
            let cleanups = Rc::clone(&cleanups);
            cx.use_effect_with_cleanup(index, move || move || cleanups.set(cleanups.get() + 1));
            text_block(index.to_string())
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = virtual_host(&reactor);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host,
            index: 7,
            lease: 1,
        });
    reactor.pump();
    assert_eq!(cleanups.get(), 0);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Recycle {
            host,
            index: 7,
            lease: 1,
        });
    reactor.pump();

    assert_eq!(cleanups.get(), 1);
}

#[test]
fn cleanup_sees_same_component_state_as_unmounted() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let cleanup_set = Rc::new(RefCell::new(None));
    let cleanup_set_for_render = Rc::clone(&cleanup_set);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            let cleanup_set = Rc::clone(&cleanup_set_for_render);
            component(move |cx| {
                let state = cx.use_state(|| 0usize);
                let cleanup_set = Rc::clone(&cleanup_set);
                cx.use_effect_with_cleanup((), move || {
                    move || *cleanup_set.borrow_mut() = Some(state.try_set(1))
                });
                text_block("child")
            })
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();

    assert_eq!(*cleanup_set.borrow(), Some(false));
}

#[test]
fn dropping_reactor_runs_effect_cleanup() {
    let cleanups = Rc::new(Cell::new(0));
    let cleanups_for_render = Rc::clone(&cleanups);
    let root = component(move |cx| {
        let cleanups = Rc::clone(&cleanups_for_render);
        cx.use_effect_with_cleanup((), move || move || cleanups.set(cleanups.get() + 1));
        text_block("effect")
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    drop(reactor);

    assert_eq!(cleanups.get(), 1);
}

#[test]
fn dropping_reactor_cleans_up_children_before_parents() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let parent_log = Rc::clone(&log);
    let child_log = Rc::clone(&log);
    let root = component(move |cx| {
        let parent_log = Rc::clone(&parent_log);
        cx.use_effect_with_cleanup((), move || move || parent_log.borrow_mut().push("parent"));
        let child_log = Rc::clone(&child_log);
        component(move |cx| {
            let child_log = Rc::clone(&child_log);
            cx.use_effect_with_cleanup((), move || move || child_log.borrow_mut().push("child"));
            text_block("child")
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    drop(reactor);

    assert_eq!(*log.borrow(), ["child", "parent"]);
}

#[test]
fn context_defaults_and_provider_identities_are_typed_and_distinct() {
    let first = Context::new("first default".to_string());
    let second = Context::new("second default".to_string());
    let root = component(move |cx| {
        text_block(format!(
            "{} / {}",
            cx.use_context(&first),
            cx.use_context(&second)
        ))
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command)
                .is_some_and(|(_, text)| text == "first default / second default"))
    );
}

#[test]
fn static_context_keys_with_the_same_type_and_default_are_distinct() {
    let root = provide_context_key(
        &FIRST_STATIC_CONTEXT,
        "first".to_string(),
        provide_context_key(
            &SECOND_STATIC_CONTEXT,
            "second".to_string(),
            component(|cx| {
                text_block(format!(
                    "{} / {}",
                    cx.use_context_key(&FIRST_STATIC_CONTEXT),
                    cx.use_context_key(&SECOND_STATIC_CONTEXT)
                ))
            }),
        ),
    );
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "first / second"))
    );
}

#[test]
fn static_context_uses_nearest_provider_with_local_rc_values() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<ContextKey<Rc<Cell<usize>>>>();

    let root = provide_context_key(
        &LOCAL_RC_CONTEXT,
        Rc::new(Cell::new(10)),
        stack_panel([
            component(|cx| {
                text_block(format!(
                    "outer: {}",
                    cx.use_context_key(&LOCAL_RC_CONTEXT).get()
                ))
            }),
            provide_context_key(
                &LOCAL_RC_CONTEXT,
                Rc::new(Cell::new(20)),
                component(|cx| {
                    text_block(format!(
                        "inner: {}",
                        cx.use_context_key(&LOCAL_RC_CONTEXT).get()
                    ))
                }),
            ),
        ]),
    );
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    let texts = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(text_update)
        .map(|(_, text)| text)
        .collect::<Vec<_>>();
    assert!(texts.contains(&"outer: 10"));
    assert!(texts.contains(&"inner: 20"));
}

#[test]
fn static_context_defaults_are_cached_and_owned_per_reactor() {
    DEFAULT_FACTORY_CALLS.store(0, Ordering::Relaxed);
    DEFAULT_DROPS.store(0, Ordering::Relaxed);

    let rerender = Rc::new(RefCell::new(None::<State<usize>>));
    let rerender_for_render = Rc::clone(&rerender);
    let first_root = component(move |cx| {
        let state = cx.use_state(|| 0);
        *rerender_for_render.borrow_mut() = Some(state.clone());
        let first = cx.use_context_key(&TRACKED_STATIC_CONTEXT);
        let second = cx.use_context_key(&TRACKED_STATIC_CONTEXT);
        assert!(Rc::ptr_eq(&first, &second));
        text_block(format!("{}:{}", first.0, state.value()))
    });
    let second_root = component(|cx| {
        let first = cx.use_context_key(&TRACKED_STATIC_CONTEXT);
        let second = cx.use_context_key(&TRACKED_STATIC_CONTEXT);
        assert!(Rc::ptr_eq(&first, &second));
        text_block(second.0.to_string())
    });
    let mut first = Reactor::new(RecordingRuntime::default(), first_root);
    let mut second = Reactor::new(RecordingRuntime::default(), second_root);

    first.pump();
    second.pump();
    assert_eq!(DEFAULT_FACTORY_CALLS.load(Ordering::Relaxed), 2);
    assert_eq!(DEFAULT_DROPS.load(Ordering::Relaxed), 0);

    assert!(rerender.borrow().as_ref().unwrap().try_set(1));
    first.pump();
    assert_eq!(DEFAULT_FACTORY_CALLS.load(Ordering::Relaxed), 2);

    drop(first);
    assert_eq!(DEFAULT_DROPS.load(Ordering::Relaxed), 1);
    second.pump();
    assert_eq!(DEFAULT_FACTORY_CALLS.load(Ordering::Relaxed), 2);

    drop(second);
    assert_eq!(DEFAULT_DROPS.load(Ordering::Relaxed), 2);
}

#[test]
fn static_context_provider_values_are_owned_per_reactor() {
    let first_value = Rc::new(ProviderProbe);
    let second_value = Rc::new(ProviderProbe);
    let first_weak = Rc::downgrade(&first_value);
    let second_weak = Rc::downgrade(&second_value);
    let first_root = provide_context_key(
        &PROVIDER_PROBE_CONTEXT,
        Rc::clone(&first_value),
        component(|cx| {
            drop(cx.use_context_key(&PROVIDER_PROBE_CONTEXT));
            text_block("first")
        }),
    );
    let second_root = provide_context_key(
        &PROVIDER_PROBE_CONTEXT,
        Rc::clone(&second_value),
        component(|cx| {
            drop(cx.use_context_key(&PROVIDER_PROBE_CONTEXT));
            text_block("second")
        }),
    );
    drop(first_value);
    drop(second_value);
    let mut first = Reactor::new(RecordingRuntime::default(), first_root);
    let mut second = Reactor::new(RecordingRuntime::default(), second_root);

    first.pump();
    second.pump();
    drop(first);

    assert!(first_weak.upgrade().is_none());
    assert!(second_weak.upgrade().is_some());

    drop(second);
    assert!(second_weak.upgrade().is_none());
}

#[test]
fn nested_context_provider_uses_nearest_value() {
    let context = Context::new("default".to_string());
    let outer_context = context.clone();
    let inner_context = context.clone();
    let root = provide_context(
        &context,
        "outer".to_string(),
        stack_panel([
            component(move |cx| text_block(format!("outer: {}", cx.use_context(&outer_context)))),
            provide_context(
                &context,
                "inner".to_string(),
                component(move |cx| {
                    text_block(format!("inner: {}", cx.use_context(&inner_context)))
                }),
            ),
        ]),
    );
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    let texts = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(text_update)
        .map(|(_, text)| text)
        .collect::<Vec<_>>();
    assert!(texts.contains(&"outer: outer"));
    assert!(texts.contains(&"inner: inner"));
}

#[test]
fn provider_updates_and_independent_child_renders_rebuild_context_scope() {
    let context = Context::new(0usize);
    let provider = Rc::new(RefCell::new(None::<State<usize>>));
    let child = Rc::new(RefCell::new(None::<State<usize>>));
    let provider_for_render = Rc::clone(&provider);
    let child_for_render = Rc::clone(&child);
    let context_for_child = context;
    let root = component(move |cx| {
        let value = cx.use_state(|| 1usize);
        *provider_for_render.borrow_mut() = Some(value.clone());
        let child = Rc::clone(&child_for_render);
        let context = context_for_child.clone();
        provide_context(
            &context_for_child,
            value.get().unwrap(),
            component(move |cx| {
                let local = cx.use_state(|| 10usize);
                *child.borrow_mut() = Some(local.clone());
                text_block(format!(
                    "{}:{}",
                    cx.use_context(&context),
                    local.get().unwrap()
                ))
            }),
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(child.borrow().as_ref().unwrap().try_set(11));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "1:11"))
    );

    assert!(provider.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "2:11"))
    );
}

#[test]
fn memoized_components_rerender_when_their_context_scope_changes() {
    let context = Context::new(0usize);
    let provider = Rc::new(RefCell::new(None::<State<usize>>));
    let wrapper = Rc::new(RefCell::new(None::<State<usize>>));
    let renders = Rc::new(Cell::new(0usize));
    let provider_for_render = Rc::clone(&provider);
    let wrapper_for_render = Rc::clone(&wrapper);
    let renders_for_render = Rc::clone(&renders);
    let root = component(move |cx| {
        let value = cx.use_state(|| 1usize);
        *provider_for_render.borrow_mut() = Some(value.clone());
        let wrapper = Rc::clone(&wrapper_for_render);
        let wrapper_context = context.clone();
        let wrapper_renders = Rc::clone(&renders_for_render);
        provide_context(
            &context,
            value.get().unwrap(),
            component(move |cx| {
                let unrelated = cx.use_state(|| 0usize);
                *wrapper.borrow_mut() = Some(unrelated);
                let context = wrapper_context.clone();
                let renders = Rc::clone(&wrapper_renders);
                memo_component((), move |cx| {
                    renders.set(renders.get() + 1);
                    text_block(cx.use_context(&context).to_string())
                })
            }),
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(wrapper.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(renders.get(), 1);

    assert!(provider.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(renders.get(), 2);

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "2"))
    );
}

#[test]
fn static_context_changes_invalidate_equal_memo_props() {
    let provider = Rc::new(RefCell::new(None::<State<String>>));
    let wrapper = Rc::new(RefCell::new(None::<State<usize>>));
    let renders = Rc::new(Cell::new(0usize));
    let provider_for_render = Rc::clone(&provider);
    let wrapper_for_render = Rc::clone(&wrapper);
    let renders_for_render = Rc::clone(&renders);
    let root = component(move |cx| {
        let value = cx.use_state(|| "first".to_string());
        *provider_for_render.borrow_mut() = Some(value.clone());
        let wrapper = Rc::clone(&wrapper_for_render);
        let renders = Rc::clone(&renders_for_render);
        provide_context_key(
            &FIRST_STATIC_CONTEXT,
            value.value(),
            component(move |cx| {
                let unrelated = cx.use_state(|| 0usize);
                *wrapper.borrow_mut() = Some(unrelated);
                let renders = Rc::clone(&renders);
                memo_component_with_props((), move |cx, _| {
                    renders.set(renders.get() + 1);
                    text_block(cx.use_context_key(&FIRST_STATIC_CONTEXT))
                })
            }),
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(wrapper.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(renders.get(), 1);

    assert!(
        provider
            .borrow()
            .as_ref()
            .unwrap()
            .try_set("second".to_string())
    );
    reactor.pump();
    assert_eq!(renders.get(), 2);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "second"))
    );
}

#[test]
fn static_provider_replacement_restores_defaults_and_rejects_stale_state() {
    let replace = Rc::new(RefCell::new(None::<State<bool>>));
    let child_state = Rc::new(RefCell::new(None::<State<usize>>));
    let replace_for_render = Rc::clone(&replace);
    let child_state_for_render = Rc::clone(&child_state);
    let root = component(move |cx| {
        let second = cx.use_state(|| false);
        *replace_for_render.borrow_mut() = Some(second.clone());
        let child_state = Rc::clone(&child_state_for_render);
        let child = component(move |cx| {
            let state = cx.use_state(|| 0usize);
            *child_state.borrow_mut() = Some(state);
            text_block(format!(
                "{} / {}",
                cx.use_context_key(&FIRST_STATIC_CONTEXT),
                cx.use_context_key(&SECOND_STATIC_CONTEXT)
            ))
        })
        .key(7);
        if second.value() {
            provide_context_key(&SECOND_STATIC_CONTEXT, "second".to_string(), child)
        } else {
            provide_context_key(&FIRST_STATIC_CONTEXT, "first".to_string(), child)
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stale = child_state.borrow().as_ref().unwrap().clone();

    assert!(stale.try_set(1));
    assert!(replace.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert!(!stale.try_set(1));
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command)
                .is_some_and(|(_, text)| text == "static default / second"))
    );
}

#[test]
fn keyed_static_providers_move_with_their_context_scopes() {
    let reverse = Rc::new(RefCell::new(None::<State<bool>>));
    let first_child = Rc::new(RefCell::new(None::<State<usize>>));
    let second_child = Rc::new(RefCell::new(None::<State<usize>>));
    let reverse_for_render = Rc::clone(&reverse);
    let first_child_for_render = Rc::clone(&first_child);
    let second_child_for_render = Rc::clone(&second_child);
    let root = component(move |cx| {
        let reversed = cx.use_state(|| false);
        *reverse_for_render.borrow_mut() = Some(reversed.clone());
        let first_child = Rc::clone(&first_child_for_render);
        let second_child = Rc::clone(&second_child_for_render);
        let first = provide_context_key(
            &FIRST_STATIC_CONTEXT,
            "first".to_string(),
            component(move |cx| {
                let state = cx.use_state(|| 10usize);
                *first_child.borrow_mut() = Some(state.clone());
                text_block(format!(
                    "{}:{}",
                    cx.use_context_key(&FIRST_STATIC_CONTEXT),
                    state.value()
                ))
            }),
        )
        .key(1);
        let second = provide_context_key(
            &FIRST_STATIC_CONTEXT,
            "second".to_string(),
            component(move |cx| {
                let state = cx.use_state(|| 20usize);
                *second_child.borrow_mut() = Some(state.clone());
                text_block(format!(
                    "{}:{}",
                    cx.use_context_key(&FIRST_STATIC_CONTEXT),
                    state.value()
                ))
            }),
        )
        .key(2);
        if reversed.value() {
            stack_panel([second, first])
        } else {
            stack_panel([first, second])
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(reverse.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    assert!(first_child.borrow().as_ref().unwrap().try_set(11));
    assert!(second_child.borrow().as_ref().unwrap().try_set(21));
    reactor.pump();

    let texts = reactor
        .engine()
        .runtime()
        .batches()
        .last()
        .unwrap()
        .iter()
        .filter_map(text_update)
        .map(|(_, text)| text)
        .collect::<Vec<_>>();
    assert!(texts.contains(&"first:11"));
    assert!(texts.contains(&"second:21"));
}

#[test]
fn dynamic_and_static_contexts_do_not_collide() {
    let dynamic = Context::new("dynamic default".to_string());
    let dynamic_for_render = dynamic.clone();
    let root = provide_context(
        &dynamic,
        "dynamic".to_string(),
        provide_context_key(
            &FIRST_STATIC_CONTEXT,
            "static".to_string(),
            component(move |cx| {
                text_block(format!(
                    "{} / {}",
                    cx.use_context(&dynamic_for_render),
                    cx.use_context_key(&FIRST_STATIC_CONTEXT)
                ))
            }),
        ),
    );
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    assert!(reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .any(|command| text_update(command)
            .is_some_and(|(_, text)| text == "dynamic / static")));
}

#[test]
fn independently_rerendered_components_pass_context_to_new_descendants() {
    let context = Context::new("default".to_string());
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let child_context = context.clone();
    let root = provide_context(
        &context,
        "provided".to_string(),
        component(move |cx| {
            let visible = cx.use_state(|| false);
            *visible_for_render.borrow_mut() = Some(visible.clone());
            if visible.get().unwrap() {
                let context = child_context.clone();
                component(move |cx| text_block(cx.use_context(&context)))
            } else {
                text_block("hidden")
            }
        }),
    );
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    assert!(visible.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "provided"))
    );
}

#[test]
fn virtual_rows_reconstruct_context_from_the_host_ancestry() {
    let context = Context::new("default".to_string());
    let context_for_row = context.clone();
    let root = provide_context(
        &context,
        "provided".to_string(),
        virtual_list(100, 300.0, move |index| {
            let context = context_for_row.clone();
            component(move |cx| text_block(format!("{} {index}", cx.use_context(&context))))
        }),
    );
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let host = virtual_host(&reactor);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host,
            index: 7,
            lease: 1,
        });

    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "provided 7"))
    );
}

#[test]
fn context_providers_are_transparent_logical_nodes() {
    let context = Context::new(0usize);
    let root = provide_context(&context, 1, text_block("root"));
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);

    reactor.pump();

    assert_eq!(reactor.engine().node_count(), 2);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter(|command| matches!(command, Command::Create { .. }))
            .count(),
        1
    );
}
