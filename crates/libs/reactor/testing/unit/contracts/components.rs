use super::*;

struct OrdinaryProps {
    text: String,
}

#[derive(PartialEq)]
struct NumberProps {
    value: usize,
}

struct CallbackProps {
    version: usize,
    format: Rc<dyn Fn(usize) -> String>,
}

impl PartialEq for CallbackProps {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

#[test]
fn ordinary_props_components_render_borrowed_values_and_update_in_place() {
    let value = Rc::new(RefCell::new(None::<State<usize>>));
    let renders = Rc::new(Cell::new(0));
    let value_for_render = Rc::clone(&value);
    let renders_for_child = Rc::clone(&renders);
    let root = component(move |cx| {
        let state = cx.use_state(|| 1usize);
        *value_for_render.borrow_mut() = Some(state.clone());
        let renders = Rc::clone(&renders_for_child);
        component_with_props(
            OrdinaryProps {
                text: format!("value {}", state.value()),
            },
            move |_, props| {
                renders.set(renders.get() + 1);
                text_block(&props.text)
            },
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    assert_eq!(renders.get(), 1);

    assert!(value.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();

    assert_eq!(renders.get(), 2);
    assert!(reactor.engine().runtime().contains(target));
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command) == Some((target, "value 2")))
    );
}

#[test]
fn memo_props_skip_equal_values_and_render_changed_values() {
    let value = Rc::new(RefCell::new(None::<State<usize>>));
    let renders = Rc::new(Cell::new(0));
    let value_for_render = Rc::clone(&value);
    let renders_for_child = Rc::clone(&renders);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *value_for_render.borrow_mut() = Some(state.clone());
        let renders = Rc::clone(&renders_for_child);
        memo_component_with_props(
            NumberProps {
                value: state.value() / 2,
            },
            move |_, props| {
                renders.set(renders.get() + 1);
                text_block(props.value.to_string())
            },
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(renders.get(), 1);

    assert!(value.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(renders.get(), 1);

    assert!(value.borrow().as_ref().unwrap().try_set(2));
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
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "1"))
    );
}

#[test]
fn equal_props_with_local_state_use_the_latest_owned_props() {
    let latest = Rc::new(RefCell::new(None::<State<bool>>));
    let local = Rc::new(RefCell::new(None::<State<usize>>));
    let renders = Rc::new(Cell::new(0));
    let latest_for_render = Rc::clone(&latest);
    let local_for_render = Rc::clone(&local);
    let renders_for_child = Rc::clone(&renders);
    let root = component(move |cx| {
        let use_latest = cx.use_state(|| false);
        *latest_for_render.borrow_mut() = Some(use_latest.clone());
        let prefix = if use_latest.value() {
            "latest"
        } else {
            "initial"
        };
        let local = Rc::clone(&local_for_render);
        let renders = Rc::clone(&renders_for_child);
        memo_component_with_props(
            CallbackProps {
                version: 0,
                format: Rc::new(move |value| format!("{prefix}:{value}")),
            },
            move |cx, props| {
                renders.set(renders.get() + 1);
                let state = cx.use_state(|| 0usize);
                *local.borrow_mut() = Some(state.clone());
                text_block((props.format)(state.value()))
            },
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(renders.get(), 1);

    assert!(latest.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    assert_eq!(renders.get(), 1);

    assert!(local.borrow().as_ref().unwrap().try_set(1));
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
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "latest:1"))
    );
}

#[test]
fn hook_state_is_preserved_across_props_changes() {
    let value = Rc::new(RefCell::new(None::<State<usize>>));
    let local = Rc::new(RefCell::new(None::<State<usize>>));
    let value_for_render = Rc::clone(&value);
    let local_for_render = Rc::clone(&local);
    let root = component(move |cx| {
        let state = cx.use_state(|| 1usize);
        *value_for_render.borrow_mut() = Some(state.clone());
        let local = Rc::clone(&local_for_render);
        memo_component_with_props(
            NumberProps {
                value: state.value(),
            },
            move |cx, props| {
                let state = cx.use_state(|| 10usize);
                *local.borrow_mut() = Some(state.clone());
                text_block(format!("{}:{}", props.value, state.value()))
            },
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let component = local.borrow().as_ref().unwrap().node();

    assert!(local.borrow().as_ref().unwrap().try_set(17));
    reactor.pump();
    assert!(value.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();

    assert_eq!(local.borrow().as_ref().unwrap().node(), component);
    assert_eq!(local.borrow().as_ref().unwrap().value(), 17);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "2:17"))
    );
}

struct IdentityProps {
    states: Rc<RefCell<Vec<State<usize>>>>,
}

impl PartialEq for IdentityProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

struct OtherIdentityProps {
    states: Rc<RefCell<Vec<State<usize>>>>,
}

impl PartialEq for OtherIdentityProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

fn render_identity_a(cx: &mut RenderCx<'_>, props: &IdentityProps) -> Element {
    let state = cx.use_state(|| 1usize);
    props.states.borrow_mut().push(state.clone());
    text_block(format!("a:{}", state.value()))
}

fn render_identity_b(cx: &mut RenderCx<'_>, props: &IdentityProps) -> Element {
    let state = cx.use_state(|| 2usize);
    props.states.borrow_mut().push(state.clone());
    text_block(format!("b:{}", state.value()))
}

fn render_other_identity(cx: &mut RenderCx<'_>, props: &OtherIdentityProps) -> Element {
    let state = cx.use_state(|| 3usize);
    props.states.borrow_mut().push(state.clone());
    text_block(format!("other:{}", state.value()))
}

#[test]
fn memo_comparison_rejects_different_value_types() {
    let left = crate::hooks::ComponentMemo::new(1u32);
    let right = crate::hooks::ComponentMemo::new(1u64);
    assert!(!left.equivalent(&right));
}

#[test]
fn prop_types_and_render_identities_do_not_reuse_component_state() {
    let mode = Rc::new(RefCell::new(None::<State<u8>>));
    let states = Rc::new(RefCell::new(Vec::<State<usize>>::new()));
    let mode_for_render = Rc::clone(&mode);
    let states_for_render = Rc::clone(&states);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0u8);
        *mode_for_render.borrow_mut() = Some(state.clone());
        match state.value() {
            0 => memo_component_with_props(
                IdentityProps {
                    states: Rc::clone(&states_for_render),
                },
                render_identity_a,
            ),
            1 => memo_component_with_props(
                IdentityProps {
                    states: Rc::clone(&states_for_render),
                },
                render_identity_b,
            ),
            _ => memo_component_with_props(
                OtherIdentityProps {
                    states: Rc::clone(&states_for_render),
                },
                render_other_identity,
            ),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let first = states.borrow()[0].clone();

    assert!(mode.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let second = states.borrow()[1].clone();
    assert_eq!(first.node().index(), second.node().index());
    assert_ne!(first.node().generation(), second.node().generation());
    assert!(!first.try_set(10));

    assert!(mode.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let third = states.borrow()[2].clone();
    assert_eq!(second.node().index(), third.node().index());
    assert_ne!(second.node().generation(), third.node().generation());
    assert!(!second.try_set(10));
}

struct KeyedProps {
    key: u64,
    states: Rc<RefCell<BTreeMap<u64, State<usize>>>>,
    drops: Rc<RefCell<Vec<u64>>>,
}

impl PartialEq for KeyedProps {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Drop for KeyedProps {
    fn drop(&mut self) {
        self.drops.borrow_mut().push(self.key);
    }
}

fn render_keyed_props(cx: &mut RenderCx<'_>, props: &KeyedProps) -> Element {
    let state = cx.use_state(|| props.key as usize * 10);
    props.states.borrow_mut().insert(props.key, state.clone());
    text_block(format!("{}:{}", props.key, state.value()))
}

#[test]
fn keyed_props_components_move_preserve_state_and_replacements_teardown() {
    let order = Rc::new(RefCell::new(None::<State<Vec<u64>>>));
    let states = Rc::new(RefCell::new(BTreeMap::new()));
    let drops = Rc::new(RefCell::new(Vec::new()));
    let order_for_render = Rc::clone(&order);
    let states_for_render = Rc::clone(&states);
    let drops_for_render = Rc::clone(&drops);
    let root = component(move |cx| {
        let state = cx.use_state(|| vec![1, 2, 3]);
        *order_for_render.borrow_mut() = Some(state.clone());
        stack_panel(state.value().into_iter().map(|key| {
            memo_component_with_props(
                KeyedProps {
                    key,
                    states: Rc::clone(&states_for_render),
                    drops: Rc::clone(&drops_for_render),
                },
                render_keyed_props,
            )
            .key(key)
        }))
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let before = states.borrow().clone();

    assert!(order.borrow().as_ref().unwrap().try_set(vec![3, 1, 2]));
    reactor.pump();
    assert_eq!(
        states
            .borrow()
            .iter()
            .map(|(key, state)| (*key, state.node()))
            .collect::<BTreeMap<_, _>>(),
        before
            .iter()
            .map(|(key, state)| (*key, state.node()))
            .collect::<BTreeMap<_, _>>()
    );
    drops.borrow_mut().clear();
    let removed = before[&2].clone();

    assert!(order.borrow().as_ref().unwrap().try_set(vec![3, 1, 4]));
    reactor.pump();

    assert_eq!(states.borrow()[&1].node(), before[&1].node());
    assert_eq!(states.borrow()[&3].node(), before[&3].node());
    assert!(!reactor.engine().contains(removed.node()));
    assert!(!removed.try_set(99));
    assert!(drops.borrow().contains(&2));
}

struct StaleWorkProps {
    states: Rc<RefCell<Vec<State<usize>>>>,
    renders: Rc<Cell<usize>>,
}

impl PartialEq for StaleWorkProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

fn render_stale_a(cx: &mut RenderCx<'_>, props: &StaleWorkProps) -> Element {
    props.renders.set(props.renders.get() + 1);
    let state = cx.use_state(|| 1usize);
    props.states.borrow_mut().push(state.clone());
    text_block(format!("a:{}", state.value()))
}

fn render_stale_b(cx: &mut RenderCx<'_>, props: &StaleWorkProps) -> Element {
    props.renders.set(props.renders.get() + 1);
    let state = cx.use_state(|| 2usize);
    props.states.borrow_mut().push(state.clone());
    text_block(format!("b:{}", state.value()))
}

#[test]
fn stale_scheduled_props_work_is_rejected_after_replacement() {
    let replace = Rc::new(RefCell::new(None::<State<bool>>));
    let old_states = Rc::new(RefCell::new(Vec::new()));
    let new_states = Rc::new(RefCell::new(Vec::new()));
    let old_renders = Rc::new(Cell::new(0));
    let new_renders = Rc::new(Cell::new(0));
    let replace_for_render = Rc::clone(&replace);
    let old_states_for_render = Rc::clone(&old_states);
    let new_states_for_render = Rc::clone(&new_states);
    let old_renders_for_render = Rc::clone(&old_renders);
    let new_renders_for_render = Rc::clone(&new_renders);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *replace_for_render.borrow_mut() = Some(state.clone());
        if state.value() {
            memo_component_with_props(
                StaleWorkProps {
                    states: Rc::clone(&new_states_for_render),
                    renders: Rc::clone(&new_renders_for_render),
                },
                render_stale_b,
            )
        } else {
            memo_component_with_props(
                StaleWorkProps {
                    states: Rc::clone(&old_states_for_render),
                    renders: Rc::clone(&old_renders_for_render),
                },
                render_stale_a,
            )
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let stale = old_states.borrow()[0].clone();

    assert!(stale.try_set(9));
    assert!(replace.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    let current = new_states.borrow()[0].clone();
    assert_eq!(old_renders.get(), 1);
    assert_eq!(new_renders.get(), 1);
    assert_eq!(stale.node().index(), current.node().index());
    assert_ne!(stale.node().generation(), current.node().generation());
    assert!(!stale.try_set(10));
    assert_eq!(current.value(), 2);
}

struct DropProps {
    value: usize,
    name: &'static str,
    drops: Rc<RefCell<Vec<&'static str>>>,
}

impl PartialEq for DropProps {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Drop for DropProps {
    fn drop(&mut self) {
        self.drops.borrow_mut().push(self.name);
    }
}

fn render_drop_props(_: &mut RenderCx<'_>, props: &DropProps) -> Element {
    text_block(props.value.to_string())
}

fn render_replacement_drop_props(_: &mut RenderCx<'_>, props: &DropProps) -> Element {
    text_block(format!("replacement {}", props.value))
}

#[test]
fn memo_props_drop_once_when_skipped_changed_replaced_and_unmounted() {
    let mode = Rc::new(RefCell::new(None::<State<u8>>));
    let drops = Rc::new(RefCell::new(Vec::new()));
    let mode_for_render = Rc::clone(&mode);
    let drops_for_render = Rc::clone(&drops);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0u8);
        *mode_for_render.borrow_mut() = Some(state.clone());
        let mode = state.value();
        if mode == 3 {
            memo_component_with_props(
                DropProps {
                    value: 2,
                    name: "replacement",
                    drops: Rc::clone(&drops_for_render),
                },
                render_replacement_drop_props,
            )
        } else {
            let (value, name) = match mode {
                0 => (0, "initial"),
                1 => (0, "equal"),
                _ => (1, "changed"),
            };
            memo_component_with_props(
                DropProps {
                    value,
                    name,
                    drops: Rc::clone(&drops_for_render),
                },
                render_drop_props,
            )
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert!(drops.borrow().is_empty());

    assert!(mode.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(*drops.borrow(), ["initial"]);

    assert!(mode.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(*drops.borrow(), ["initial", "equal"]);

    assert!(mode.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(*drops.borrow(), ["initial", "equal", "changed"]);

    drop(reactor);
    assert_eq!(
        *drops.borrow(),
        ["initial", "equal", "changed", "replacement"]
    );
}

fn render_panicking_props(
    _: &mut RenderCx<'_>,
    props: &DropProps,
    attempts: &Cell<usize>,
) -> Element {
    attempts.set(attempts.get() + 1);
    assert_ne!(props.value, 1, "injected props render panic");
    text_block(props.value.to_string())
}

#[test]
fn panicking_render_does_not_commit_new_memo_props() {
    let value = Rc::new(RefCell::new(None::<State<usize>>));
    let drops = Rc::new(RefCell::new(Vec::new()));
    let attempts = Rc::new(Cell::new(0));
    let value_for_render = Rc::clone(&value);
    let drops_for_render = Rc::clone(&drops);
    let attempts_for_render = Rc::clone(&attempts);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *value_for_render.borrow_mut() = Some(state.clone());
        let value = state.value();
        let attempts = Rc::clone(&attempts_for_render);
        memo_component_with_props(
            DropProps {
                value,
                name: match value {
                    0 if attempts.get() == 0 => "initial",
                    0 => "retry",
                    _ => "failed",
                },
                drops: Rc::clone(&drops_for_render),
            },
            move |cx, props| render_panicking_props(cx, props, &attempts),
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    assert_eq!(attempts.get(), 1);

    assert!(value.borrow().as_ref().unwrap().try_set(1));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reactor.pump()));
    assert!(result.is_err());
    assert_eq!(attempts.get(), 2);
    assert_eq!(*drops.borrow(), ["failed"]);

    assert!(value.borrow().as_ref().unwrap().try_set(0));
    reactor.pump();
    assert_eq!(attempts.get(), 2);
    assert_eq!(*drops.borrow(), ["failed", "initial"]);

    drop(reactor);
    assert_eq!(*drops.borrow(), ["failed", "initial", "retry"]);
}
