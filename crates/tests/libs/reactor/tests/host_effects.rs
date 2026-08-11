use std::cell::{Cell, RefCell};
use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Component;
use windows_reactor::RenderHost;
use windows_reactor::{Dispatcher, DispatcherQueuePriority};
use windows_reactor::{Element, TextBlock};
use windows_reactor::{RenderCx, SetState};
use windows_reactor::{component, composition_host, swap_chain_panel, text_block, vstack};

type QueuedJob = (DispatcherQueuePriority, Box<dyn FnOnce()>);

#[derive(Clone, Default)]
struct TestDispatcher {
    queue: Rc<RefCell<Vec<QueuedJob>>>,
}

impl TestDispatcher {
    fn drain(&self) {
        loop {
            let item = {
                let mut q = self.queue.borrow_mut();
                if q.is_empty() {
                    None
                } else {
                    Some(q.remove(0))
                }
            };
            match item {
                Some((_, f)) => f(),
                None => break,
            }
        }
    }
}

impl Dispatcher for TestDispatcher {
    fn enqueue(&self, priority: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool {
        self.queue.borrow_mut().push((priority, f));
        true
    }
}

struct EffectsAfterReconcile {
    observed_ops: Rc<Cell<usize>>,
}

impl Component for EffectsAfterReconcile {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let observed = Rc::clone(&self.observed_ops);

        cx.use_effect((), move || {
            observed.set(observed.get() + 1);
        });
        Element::TextBlock(TextBlock {
            text: "hi".into(),
            ..TextBlock::default()
        })
    }
}

#[test]
fn effect_runs_after_reconcile_ops_are_emitted() {
    let dispatcher = TestDispatcher::default();
    let observed = Rc::new(Cell::new(0_usize));
    let root: Box<dyn Component> = Box::new(EffectsAfterReconcile {
        observed_ops: Rc::clone(&observed),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();

    dispatcher.drain();

    assert_eq!(host.render_count(), 1);

    assert_eq!(observed.get(), 1, "effect must fire on first flush");

    let (created_count, set_prop_count) = host.with_reconciler(|r| {
        let mut c = 0;
        let mut s = 0;
        for op in &r.backend.ops {
            match op {
                Op::Create { .. } => c += 1,
                Op::SetProp { .. } => s += 1,
                _ => {}
            }
        }
        (c, s)
    });
    assert_eq!(created_count, 1, "one TextBlock created");
    assert!(
        set_prop_count >= 1,
        "at least one SetProp for the TextBlock"
    );
}

struct NestedCommitEffect {
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for NestedCommitEffect {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        self.log.borrow_mut().push("nested render");
        let log = Rc::clone(&self.log);
        cx.use_effect((), move || log.borrow_mut().push("nested effect"));
        text_block("nested").into()
    }
}

struct NestedCommitParent {
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for NestedCommitParent {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        self.log.borrow_mut().push("parent render");
        let log = Rc::clone(&self.log);
        cx.use_effect((), move || log.borrow_mut().push("parent effect"));
        component(
            NestedCommitEffect {
                log: Rc::clone(&self.log),
            },
            (),
        )
    }
}

struct CommitOrderRoot {
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for CommitOrderRoot {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        self.log.borrow_mut().push("root render");
        let root_effect_log = Rc::clone(&self.log);
        cx.use_effect((), move || root_effect_log.borrow_mut().push("root effect"));
        let mounted_log = Rc::clone(&self.log);
        vstack((
            component(
                NestedCommitParent {
                    log: Rc::clone(&self.log),
                },
                (),
            ),
            swap_chain_panel()
                .on_mounted(move |_| mounted_log.borrow_mut().push("later native mounted")),
        ))
        .into()
    }
}

#[test]
fn nested_effects_run_after_the_native_tree_commits() {
    let dispatcher = TestDispatcher::default();
    let log = Rc::new(RefCell::new(Vec::new()));
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(CommitOrderRoot {
            log: Rc::clone(&log),
        }),
        dispatcher.clone(),
    );
    host.kick();

    dispatcher.drain();

    assert_eq!(
        &*log.borrow(),
        &[
            "root render",
            "parent render",
            "nested render",
            "later native mounted",
            "nested effect",
            "parent effect",
            "root effect",
        ]
    );
}

struct PanickingNestedEffect;

impl Component for PanickingNestedEffect {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        cx.use_effect((), || panic!("nested effect failed"));
        text_block("committed").into()
    }
}

struct PanickingEffectRoot;

impl Component for PanickingEffectRoot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        component(PanickingNestedEffect, ())
    }
}

#[test]
fn nested_effect_panics_leave_the_host_tree_committed() {
    let dispatcher = TestDispatcher::default();
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(PanickingEffectRoot),
        dispatcher.clone(),
    );
    host.kick();

    dispatcher.drain();

    assert!(host.root_id().is_some());
    assert_eq!(
        host.with_reconciler(|reconciler| reconciler.backend.live_control_count()),
        1
    );
}

#[derive(Clone)]
struct EffectCommitProps {
    updated: bool,
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl PartialEq for EffectCommitProps {
    fn eq(&self, other: &Self) -> bool {
        self.updated == other.updated && Rc::ptr_eq(&self.log, &other.log)
    }
}

struct EffectCommitOrder;

impl Component<EffectCommitProps> for EffectCommitOrder {
    fn render(&self, props: &EffectCommitProps, cx: &mut RenderCx) -> Element {
        let updated = props.updated;
        let setup_log = Rc::clone(&props.log);
        let cleanup_log = Rc::clone(&props.log);
        cx.use_effect_with_cleanup((updated,), move || {
            setup_log
                .borrow_mut()
                .push(if updated { "effect new" } else { "effect old" });
            Some(move || {
                cleanup_log.borrow_mut().push(if updated {
                    "cleanup new"
                } else {
                    "cleanup old"
                });
            })
        });

        if updated {
            let mounted_log = Rc::clone(&props.log);
            composition_host()
                .on_mounted(move |_| mounted_log.borrow_mut().push("new mounted"))
                .into()
        } else {
            let mounted_log = Rc::clone(&props.log);
            let unmounted_log = Rc::clone(&props.log);
            swap_chain_panel()
                .on_mounted(move |_| mounted_log.borrow_mut().push("old mounted"))
                .on_unmounted(move |_| unmounted_log.borrow_mut().push("old unmounted"))
                .into()
        }
    }
}

#[test]
fn changed_effects_clean_up_and_restart_after_the_native_update() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let old = component(
        EffectCommitOrder,
        EffectCommitProps {
            updated: false,
            log: Rc::clone(&log),
        },
    );
    let new = component(
        EffectCommitOrder,
        EffectCommitProps {
            updated: true,
            log: Rc::clone(&log),
        },
    );
    let mut reconciler = windows_reactor::Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, &old, None, Rc::new(|| {}));
    assert_eq!(&*log.borrow(), &["old mounted", "effect old"]);

    log.borrow_mut().clear();
    reconciler.reconcile(Some(&old), &new, None, Rc::new(|| {}));

    assert_eq!(
        &*log.borrow(),
        &["old unmounted", "new mounted", "cleanup old", "effect new"]
    );
}

struct EffectWithStateDep {
    log: Rc<RefCell<Vec<i32>>>,
    setter_out: Rc<RefCell<Option<SetState<i32>>>>,
}

impl Component for EffectWithStateDep {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (count, set) = cx.use_state(0_i32);
        *self.setter_out.borrow_mut() = Some(set);
        let log_c = Rc::clone(&self.log);
        cx.use_effect((count,), move || {
            log_c.borrow_mut().push(count);
        });
        Element::TextBlock(TextBlock {
            text: format!("n={count}"),
            ..TextBlock::default()
        })
    }
}

#[test]
fn effect_deps_on_use_state_reruns_on_setter_change() {
    let dispatcher = TestDispatcher::default();
    let log: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let setter_out = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(EffectWithStateDep {
        log: Rc::clone(&log),
        setter_out: Rc::clone(&setter_out),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();
    dispatcher.drain();

    assert_eq!(*log.borrow(), vec![0]);

    setter_out
        .borrow()
        .clone()
        .expect("setter captured")
        .call(5);
    dispatcher.drain();

    assert_eq!(*log.borrow(), vec![0, 5], "effect re-ran with new deps");
}

struct HostDropChild {
    cleanups: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for HostDropChild {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let cleanups = Rc::clone(&self.cleanups);
        cx.use_effect_with_cleanup((), move || {
            Some(move || cleanups.borrow_mut().push("child"))
        });
        text_block("child").into()
    }
}

struct HostDropRoot {
    cleanups: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for HostDropRoot {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let cleanups = Rc::clone(&self.cleanups);
        cx.use_effect_with_cleanup((), move || Some(move || cleanups.borrow_mut().push("root")));
        component(
            HostDropChild {
                cleanups: Rc::clone(&self.cleanups),
            },
            (),
        )
    }
}

#[test]
fn host_drop_runs_nested_then_root_cleanup_once() {
    let dispatcher = TestDispatcher::default();
    let cleanups = Rc::new(RefCell::new(Vec::new()));
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(HostDropRoot {
            cleanups: Rc::clone(&cleanups),
        }),
        dispatcher.clone(),
    );
    host.kick();
    dispatcher.drain();
    assert!(cleanups.borrow().is_empty());

    drop(host);

    assert_eq!(&*cleanups.borrow(), &["child", "root"]);
}
