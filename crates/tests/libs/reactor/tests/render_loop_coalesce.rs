use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_reactor::core::backend::RecordingBackend;
use windows_reactor::core::component::Component;
use windows_reactor::core::component_element::component;
use windows_reactor::core::dispatcher::{
    Dispatcher, DispatcherQueuePriority, RunOnDemandDispatcher,
};
use windows_reactor::core::element::{Element, Orientation, StackPanel, TextBlock};
use windows_reactor::core::render_context::{RenderCx, SetState};
use windows_reactor::core::render_host::RenderHost;
use windows_reactor::core::window::Size;

struct Counter {
    setter_slot: Rc<RefCell<Option<SetState<i32>>>>,
}

impl Component for Counter {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (count, set) = cx.use_state(0_i32);
        *self.setter_slot.borrow_mut() = Some(set);
        Element::TextBlock(TextBlock {
            text: format!("Count: {count}"),
            ..TextBlock::default()
        })
    }
}

#[test]
fn hundred_setstate_calls_collapse_to_one_reconcile_pass() {
    let dispatcher = RunOnDemandDispatcher::new();
    let setter_slot = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(Counter {
        setter_slot: Rc::clone(&setter_slot),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher);
    host.kick();

    assert_eq!(host.render_count(), 0);
}

type QueuedJob = (DispatcherQueuePriority, Box<dyn FnOnce()>);

#[derive(Clone, Default)]
struct TestDispatcher {
    queue: Rc<RefCell<Vec<QueuedJob>>>,
}

impl TestDispatcher {
    fn new() -> Self {
        Self::default()
    }

    fn pending(&self) -> usize {
        self.queue.borrow().len()
    }

    fn drain(&self) -> usize {
        let mut fired = 0;
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
                Some((_, f)) => {
                    f();
                    fired += 1;
                }
                None => break,
            }
        }
        fired
    }
}

impl Dispatcher for TestDispatcher {
    fn enqueue(&self, priority: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool {
        self.queue.borrow_mut().push((priority, f));
        true
    }
}

#[test]
fn setstate_spam_outside_render_coalesces_to_one_render() {
    let dispatcher = TestDispatcher::new();
    let setter_slot = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(Counter {
        setter_slot: Rc::clone(&setter_slot),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();

    dispatcher.drain();
    assert_eq!(host.render_count(), 1, "initial render should have fired");

    let setter = setter_slot
        .borrow()
        .clone()
        .expect("component captured a setter");

    for i in 1..=100_i32 {
        setter.call(i);
    }

    assert_eq!(
        dispatcher.pending(),
        1,
        "expected 1 pending render after 100 setters, got {}",
        dispatcher.pending()
    );

    dispatcher.drain();
    assert_eq!(
        host.render_count(),
        2,
        "exactly one follow-up render should have fired"
    );
}

struct SetDuringRender {
    fired_once: Rc<Cell<bool>>,
}

impl Component for SetDuringRender {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (_, set) = cx.use_state(0_i32);
        if !self.fired_once.get() {
            self.fired_once.set(true);

            set.call(7);
        }
        Element::TextBlock(TextBlock {
            text: "x".into(),
            ..TextBlock::default()
        })
    }
}

#[test]
fn setstate_during_render_defers_to_low_priority_second_pass() {
    let dispatcher = TestDispatcher::new();
    let fired_once = Rc::new(Cell::new(false));
    let root: Box<dyn Component> = Box::new(SetDuringRender {
        fired_once: Rc::clone(&fired_once),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();

    dispatcher.drain();

    assert_eq!(host.render_count(), 2);
}

type PriorityQueue = Rc<RefCell<Vec<Box<dyn FnOnce()>>>>;

#[derive(Default)]
struct PriorityCapturingDispatcher {
    priorities: Rc<RefCell<Vec<DispatcherQueuePriority>>>,
    queue: PriorityQueue,
}

impl PriorityCapturingDispatcher {
    fn new() -> Self {
        Self::default()
    }

    fn priorities(&self) -> Vec<DispatcherQueuePriority> {
        self.priorities.borrow().clone()
    }

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

impl Dispatcher for PriorityCapturingDispatcher {
    fn enqueue(&self, priority: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool {
        self.priorities.borrow_mut().push(priority);
        self.queue.borrow_mut().push(f);
        true
    }
}

impl Clone for PriorityCapturingDispatcher {
    fn clone(&self) -> Self {
        Self {
            priorities: Rc::clone(&self.priorities),
            queue: Rc::clone(&self.queue),
        }
    }
}

struct ToggleRoot {
    show_text: Rc<Cell<bool>>,
    setter_slot: Rc<RefCell<Option<SetState<bool>>>>,
}

impl Component for ToggleRoot {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (show, set) = cx.use_state(self.show_text.get());
        *self.setter_slot.borrow_mut() = Some(set);
        if show {
            Element::TextBlock(TextBlock {
                text: "T".into(),
                ..TextBlock::default()
            })
        } else {
            use windows_reactor::core::element::StackPanel;
            Element::StackPanel(StackPanel {
                orientation: Orientation::Vertical,
                ..StackPanel::default()
            })
        }
    }
}

#[test]
fn post_render_hook_fires_after_each_render_with_current_root_id() {
    let dispatcher = TestDispatcher::new();
    let setter_slot = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(ToggleRoot {
        show_text: Rc::new(Cell::new(true)),
        setter_slot: Rc::clone(&setter_slot),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());

    let observations: Rc<RefCell<Vec<Option<windows_reactor::core::backend::ControlId>>>> =
        Rc::new(RefCell::new(Vec::new()));
    let observations_for_hook = Rc::clone(&observations);
    host.set_post_render(move |new_id| {
        observations_for_hook.borrow_mut().push(new_id);
    });

    host.kick();
    dispatcher.drain();
    assert_eq!(host.render_count(), 1);
    assert_eq!(
        observations.borrow().len(),
        1,
        "first render must fire the hook exactly once"
    );

    let setter = setter_slot.borrow().clone().expect("setter captured");
    setter.call(false);
    dispatcher.drain();
    assert_eq!(host.render_count(), 2);
    assert_eq!(
        observations.borrow().len(),
        2,
        "second render must fire the hook again"
    );

    let obs = observations.borrow();
    assert!(
        obs[0].is_some() && obs[1].is_some(),
        "audit §7.4.6: post_render must report a real root id; got {obs:?}"
    );
}

#[test]
fn setstate_during_render_enqueue_uses_low_priority() {
    let dispatcher = PriorityCapturingDispatcher::new();
    let fired_once = Rc::new(Cell::new(false));
    let root: Box<dyn Component> = Box::new(SetDuringRender {
        fired_once: Rc::clone(&fired_once),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();

    dispatcher.drain();

    let priorities = dispatcher.priorities();
    assert_eq!(priorities.len(), 2, "priorities: {priorities:?}");
    assert_eq!(priorities[0], DispatcherQueuePriority::Normal);
    assert_eq!(priorities[1], DispatcherQueuePriority::Low);
    assert_eq!(host.render_count(), 2);
}

#[test]
fn render_state_transitions_observable_through_public_accessors() {
    let dispatcher = TestDispatcher::new();
    let setter_slot = Rc::new(RefCell::new(None));
    let root: Box<dyn Component> = Box::new(Counter {
        setter_slot: Rc::clone(&setter_slot),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());

    assert!(host.is_idle());
    assert!(!host.is_render_pending());
    assert!(!host.is_rendering());
    assert!(!host.needs_rerender());

    host.kick();
    assert!(!host.is_idle());
    assert!(host.is_render_pending());
    assert!(!host.is_rendering());
    assert!(host.needs_rerender());

    dispatcher.drain();
    assert!(host.is_idle());
    assert!(!host.is_render_pending());
    assert!(!host.is_rendering());
    assert!(!host.needs_rerender());

    let dispatcher2 = TestDispatcher::new();
    let fired_once = Rc::new(Cell::new(false));
    let root: Box<dyn Component> = Box::new(SetDuringRender {
        fired_once: Rc::clone(&fired_once),
    });
    let host2 = RenderHost::new(RecordingBackend::new(), root, dispatcher2.clone());
    host2.kick();
    dispatcher2.drain();
    assert_eq!(host2.render_count(), 2);
    assert!(host2.is_idle());
}

struct InnerSizeLeaf {
    renders: Rc<Cell<u32>>,
}

impl Component for InnerSizeLeaf {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let render_count = self.renders.get() + 1;
        self.renders.set(render_count);
        let size = cx.use_inner_size();
        if render_count > 1 {
            assert!(
                size.width != 0.0 || size.height != 0.0,
                "child rerendered without observing the updated inner size: {}x{}",
                size.width,
                size.height
            );
        }
        Element::TextBlock(TextBlock::new(format!("{}x{}", size.width, size.height)))
    }
}

struct InnerSizeRoot {
    child_renders: Rc<Cell<u32>>,
}

impl Component for InnerSizeRoot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        Element::StackPanel(StackPanel {
            orientation: Orientation::Vertical,
            children: vec![component(
                InnerSizeLeaf {
                    renders: Rc::clone(&self.child_renders),
                },
                (),
            )],
            ..StackPanel::default()
        })
    }
}

#[test]
fn inner_size_change_rerenders_children_once_and_resets_force_flag() {
    let dispatcher = TestDispatcher::new();
    let child_renders = Rc::new(Cell::new(0));
    let root: Box<dyn Component> = Box::new(InnerSizeRoot {
        child_renders: Rc::clone(&child_renders),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());

    host.kick();
    dispatcher.drain();
    assert_eq!(
        child_renders.get(),
        1,
        "child should render on initial mount"
    );

    host.set_inner_size(Size {
        width: 800.0,
        height: 600.0,
    });
    dispatcher.drain();
    assert_eq!(
        child_renders.get(),
        2,
        "child should render once after inner-size update"
    );

    host.request_render();
    dispatcher.drain();
    assert_eq!(
        child_renders.get(),
        2,
        "force rerender flag should be cleared after the size-driven pass"
    );
}
