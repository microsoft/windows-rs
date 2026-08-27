//! Scope-owned background task completion and cancellation tests.

use super::super::*;
use crate::native::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Input {
    create: Option<(Arc<Barrier>, String)>,
    handle: Rc<RefCell<Option<ComponentTask>>>,
    sender: Rc<RefCell<Option<LocalSender<Message>>>>,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        let create_equal = match (&self.create, &other.create) {
            (Some((left_barrier, left_value)), Some((right_barrier, right_value))) => {
                Arc::ptr_eq(left_barrier, right_barrier) && left_value == right_value
            }
            (None, None) => true,
            _ => false,
        };
        create_equal
            && Rc::ptr_eq(&self.handle, &other.handle)
            && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

enum Message {
    Complete(String),
    Rejected,
    StartPanicking,
    StartWithRejection(Arc<Barrier>, String),
    StartUntilCancelled(Arc<AtomicBool>),
}

struct BackgroundComponent {
    handle: Option<ComponentTask>,
    input: Input,
    text: String,
}

impl Component for BackgroundComponent {
    type Message = Message;
    type Input = Input;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        let handle = input.create.as_ref().map(|(barrier, value)| {
            let barrier = Arc::clone(barrier);
            let value = value.clone();
            context.spawn_background(move |_| {
                barrier.wait();
                Message::Complete(value)
            })
        });
        *input.handle.borrow_mut() = handle.clone();
        Self {
            handle,
            input: input.clone(),
            text: "idle".to_string(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.input = input.clone();
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Complete(value) => self.text = value,
            Message::Rejected => self.text = "rejected".to_string(),
            Message::StartPanicking => {
                let handle = context.spawn_background_with_rejection(
                    move |_| panic!("injected task panic"),
                    Message::Rejected,
                );
                *self.input.handle.borrow_mut() = Some(handle.clone());
                self.handle = Some(handle);
            }
            Message::StartWithRejection(barrier, value) => {
                let handle = context.spawn_background_with_rejection(
                    move |_| {
                        barrier.wait();
                        Message::Complete(value)
                    },
                    Message::Rejected,
                );
                *self.input.handle.borrow_mut() = Some(handle.clone());
                self.handle = Some(handle);
            }
            Message::StartUntilCancelled(observed) => {
                let handle = context.spawn_background_with_rejection(
                    move |cancellation| {
                        while !cancellation.is_cancelled() {
                            std::thread::sleep(Duration::from_millis(1));
                        }
                        observed.store(true, Ordering::Release);
                        Message::Complete("stale".to_string())
                    },
                    Message::Rejected,
                );
                *self.input.handle.borrow_mut() = Some(handle.clone());
                self.handle = Some(handle);
            }
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(self.text.clone()))
    }
}

fn fixture() -> (
    Pump<RecordingRuntime>,
    Rc<RefCell<Option<ComponentTask>>>,
    Rc<RefCell<Option<LocalSender<Message>>>>,
) {
    let handle = Rc::new(RefCell::new(None));
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<BackgroundComponent>(Input {
        create: None,
        handle: Rc::clone(&handle),
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    (pump, handle, sender)
}

struct WakeRuntime {
    inner: RecordingRuntime,
    wakes: Arc<AtomicUsize>,
}

impl NativeRuntime for WakeRuntime {
    fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError> {
        self.inner.apply(commands)
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    fn component_background_waker(&self) -> Option<Arc<dyn Fn() -> bool + Send + Sync>> {
        let wakes = Arc::clone(&self.wakes);
        Some(Arc::new(move || {
            wakes.fetch_add(1, Ordering::AcqRel);
            false
        }))
    }
}

fn wait_until(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while !condition() {
        assert!(Instant::now() < deadline, "background task timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn text(pump: &Pump<RecordingRuntime>) -> String {
    let native = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
    let Some(PropertyValue::Str(value)) = pump
        .runtime()
        .node(native)
        .unwrap()
        .property(PropertyId::TextBlockText)
    else {
        panic!("missing text");
    };
    value.clone()
}

#[test]
fn background_completion_routes_to_the_owning_component() {
    let (mut pump, handle, sender) = fixture();
    let barrier = Arc::new(Barrier::new(2));
    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::StartWithRejection(
                Arc::clone(&barrier),
                "complete".to_string()
            ))
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    barrier.wait();
    wait_until(|| pump.native_work_pending());

    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(text(&pump), "complete");
    assert_eq!(
        handle.borrow().as_ref().unwrap().status(),
        ComponentTaskStatus::Delivered
    );
}

#[test]
fn create_can_start_background_work() {
    let handle = Rc::new(RefCell::new(None));
    let sender = Rc::new(RefCell::new(None));
    let barrier = Arc::new(Barrier::new(2));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<BackgroundComponent>(Input {
        create: Some((Arc::clone(&barrier), "created".to_string())),
        handle: Rc::clone(&handle),
        sender,
    }))
    .unwrap();

    barrier.wait();
    wait_until(|| pump.native_work_pending());
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(text(&pump), "created");
    assert_eq!(
        handle.borrow().as_ref().unwrap().status(),
        ComponentTaskStatus::Delivered
    );
}

#[test]
fn synchronous_rejection_dispatches_the_typed_fallback() {
    let (mut pump, handle, sender) = fixture();
    pump.components.exhaust_task_capacity();
    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::StartPanicking)
    );

    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(
        handle.borrow().as_ref().unwrap().status(),
        ComponentTaskStatus::Rejected
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(text(&pump), "rejected");
}

#[test]
fn retirement_discards_a_queued_rejection() {
    let (mut pump, handle, sender) = fixture();
    pump.components.exhaust_task_capacity();
    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::StartPanicking)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(
        handle.borrow().as_ref().unwrap().status(),
        ComponentTaskStatus::Rejected
    );
    assert_eq!(pump.components.pending(), 1);

    pump.update_view(View::native(TextBlock::new().text("replacement")))
        .unwrap();
    assert_eq!(pump.components.pending(), 0);
    assert_eq!(text(&pump), "replacement");
}

#[test]
fn panicking_work_dispatches_the_typed_fallback() {
    let (mut pump, handle, sender) = fixture();
    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::StartPanicking)
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    wait_until(|| pump.native_work_pending());

    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(text(&pump), "rejected");
    assert_eq!(
        handle.borrow().as_ref().unwrap().status(),
        ComponentTaskStatus::Rejected
    );
}

#[test]
fn rejected_dispatcher_wake_converts_completion_to_the_typed_fallback() {
    let handle = Rc::new(RefCell::new(None));
    let sender = Rc::new(RefCell::new(None));
    let wakes = Arc::new(AtomicUsize::new(0));
    let mut pump = Pump::new(WakeRuntime {
        inner: RecordingRuntime::default(),
        wakes: Arc::clone(&wakes),
    });
    pump.mount_view(View::component::<BackgroundComponent>(Input {
        create: None,
        handle: Rc::clone(&handle),
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    let barrier = Arc::new(Barrier::new(2));
    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::StartWithRejection(
                Arc::clone(&barrier),
                "ignored".to_string()
            ))
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    barrier.wait();
    wait_until(|| handle.borrow().as_ref().unwrap().status() == ComponentTaskStatus::Rejected);
    assert_eq!(wakes.load(Ordering::Acquire), 1);
    assert_eq!(pump.components.pending(), 1);
    assert_eq!(pump.dispatch_components(1), Ok(1));
    let native = Pump::<WakeRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
    assert_eq!(
        pump.runtime()
            .inner
            .node(native)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("rejected".to_string()))
    );
}

#[test]
fn retirement_cancels_running_task_and_discards_completion() {
    let (mut pump, handle, sender) = fixture();
    let observed = Arc::new(AtomicBool::new(false));
    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::StartUntilCancelled(Arc::clone(&observed)))
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    pump.update_view(View::native(TextBlock::new().text("replacement")))
        .unwrap();
    wait_until(|| observed.load(Ordering::Acquire));
    assert_eq!(
        handle.borrow().as_ref().unwrap().status(),
        ComponentTaskStatus::Cancelled
    );
    assert_eq!(pump.components.pending(), 0);
    assert_eq!(text(&pump), "replacement");
}

#[test]
fn explicit_cancellation_removes_a_queued_completion() {
    let (mut pump, handle, sender) = fixture();
    let barrier = Arc::new(Barrier::new(2));
    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::StartWithRejection(
                Arc::clone(&barrier),
                "complete".to_string()
            ))
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));
    barrier.wait();
    wait_until(|| pump.native_work_pending());

    handle.borrow().as_ref().unwrap().cancel();
    assert_eq!(
        handle.borrow().as_ref().unwrap().status(),
        ComponentTaskStatus::Cancelled
    );
    assert_eq!(pump.components.pending(), 0);
    assert_eq!(text(&pump), "idle");
}

#[test]
fn shutdown_cancels_scope_tasks() {
    let (mut pump, handle, sender) = fixture();
    let observed = Arc::new(AtomicBool::new(false));
    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(Message::StartUntilCancelled(Arc::clone(&observed)))
    );
    assert_eq!(pump.dispatch_components(1), Ok(1));

    pump.shutdown();
    wait_until(|| observed.load(Ordering::Acquire));
    assert_eq!(
        handle.borrow().as_ref().unwrap().status(),
        ComponentTaskStatus::Cancelled
    );
}
