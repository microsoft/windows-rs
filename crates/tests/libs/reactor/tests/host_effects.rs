use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_reactor::Component;
use windows_reactor::RenderHost;
use windows_reactor::{Dispatcher, DispatcherQueuePriority};
use windows_reactor::{Element, TextBlock};
use windows_reactor::{Op, RecordingBackend};
use windows_reactor::{RenderCx, SetState};

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
