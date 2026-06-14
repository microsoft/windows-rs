use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_reactor::Component;
use windows_reactor::RecordingBackend;
use windows_reactor::RenderHost;
use windows_reactor::{Dispatcher, DispatcherQueuePriority};
use windows_reactor::{Element, TextBlock};
use windows_reactor::{RenderCx, SetState};

type Job = Box<dyn FnOnce()>;

#[derive(Clone, Default)]
struct TestDispatcher {
    queue: Rc<RefCell<Vec<Job>>>,
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
                Some(f) => f(),
                None => break,
            }
        }
    }
}

impl Dispatcher for TestDispatcher {
    fn enqueue(&self, _p: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool {
        self.queue.borrow_mut().push(f);
        true
    }
}

struct StableTree {
    children: usize,
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
}

impl Component for StableTree {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (_, set) = cx.use_state(0_u64);
        *self.setter_out.borrow_mut() = Some(set);
        let children: Vec<Element> = (0..self.children)
            .map(|i| Element::TextBlock(TextBlock::new(format!("row-{i}"))))
            .collect();
        let mut s = windows_reactor::vstack(children);
        s.spacing = 4.0;
        Element::StackPanel(s)
    }
}

#[test]
fn reconciling_same_tree_twice_skips_every_child() {
    let dispatcher = TestDispatcher::default();
    let setter_out = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(StableTree {
        children: 5,
        setter_out: Rc::clone(&setter_out),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();
    dispatcher.drain();

    let s = host.stats();
    assert_eq!(s.last_skipped, 0, "initial mount has nothing to skip");

    setter_out.borrow().as_ref().unwrap().call(1);
    dispatcher.drain();

    let s2 = host.stats();

    assert_eq!(s2.last_skipped, 1, "expected 1 root-level skip, got {s2:?}");
}

#[test]
fn stats_total_renders_grows_monotonically() {
    let dispatcher = TestDispatcher::default();
    let setter_out = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(StableTree {
        children: 0,
        setter_out: Rc::clone(&setter_out),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();
    dispatcher.drain();

    let mut prev = host.stats().total_renders;

    for i in 1..=5_u64 {
        setter_out.borrow().as_ref().unwrap().call(i);
        dispatcher.drain();
        let now = host.stats().total_renders;
        assert!(now > prev, "total_renders must grow");
        prev = now;
    }
    assert_eq!(host.render_count() as u64, host.stats().total_renders);
}

#[test]
fn stats_last_created_captures_mount_count() {
    let dispatcher = TestDispatcher::default();
    let setter_out = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(StableTree {
        children: 3,
        setter_out: Rc::clone(&setter_out),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();
    dispatcher.drain();

    assert_eq!(host.stats().last_created, 4, "stats: {:?}", host.stats());

    let _unused: &Cell<u64> = &Cell::new(0);
    setter_out.borrow().as_ref().unwrap().call(1);
    dispatcher.drain();
    assert_eq!(host.stats().last_created, 0);
}
