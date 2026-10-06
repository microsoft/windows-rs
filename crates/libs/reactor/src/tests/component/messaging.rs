use super::*;

#[test]
fn default_services_deliver_timer_callbacks() {
    let (sender, receiver) = std::sync::mpsc::channel();
    let _timer = DefaultComponentServices
        .set_timeout(Duration::ZERO, Box::new(move || sender.send(()).unwrap()));
    receiver.recv_timeout(Duration::from_secs(5)).unwrap();
}

#[test]
fn default_services_cancel_timers_and_release_callbacks() {
    let (sender, receiver) = std::sync::mpsc::channel();
    let timer = DefaultComponentServices.set_timeout(
        Duration::from_secs(3_600),
        Box::new(move || sender.send(()).unwrap()),
    );
    timer.cancel();
    assert_eq!(
        receiver.recv_timeout(Duration::from_secs(5)),
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected)
    );
}

#[derive(Clone)]
struct LocalMessageInput {
    seen: Rc<Cell<usize>>,
    sender: Rc<RefCell<Option<LocalSender<Rc<Cell<usize>>>>>>,
}

impl PartialEq for LocalMessageInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.seen, &other.seen) && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct LocalMessageProbe {
    _timer: ComponentTimer,
    seen: Rc<Cell<usize>>,
}

impl Component for LocalMessageProbe {
    type Input = LocalMessageInput;
    type Message = Rc<Cell<usize>>;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        input.sender.replace(Some(context.sender()));
        Self {
            _timer: context.set_local_timeout(Duration::ZERO, || Rc::new(Cell::new(2))),
            seen: Rc::clone(&input.seen),
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.seen.set(message.get());
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Grid::new().into()
    }
}

#[derive(Clone)]
struct MessageOrderInput {
    local: Rc<RefCell<Option<LocalSender<u8>>>>,
    seen: Rc<RefCell<Vec<u8>>>,
}

impl PartialEq for MessageOrderInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.local, &other.local) && Rc::ptr_eq(&self.seen, &other.seen)
    }
}

struct MessageOrderProbe {
    seen: Rc<RefCell<Vec<u8>>>,
}

impl Component for MessageOrderProbe {
    type Input = MessageOrderInput;
    type Message = u8;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        input.local.replace(Some(context.sender()));
        Self {
            seen: Rc::clone(&input.seen),
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.seen.borrow_mut().push(message);
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Grid::new().into()
    }
}

#[derive(Clone)]
struct OrderedInput {
    rendered: Rc<RefCell<String>>,
    seen: Rc<RefCell<Vec<(String, String)>>>,
}

impl PartialEq for OrderedInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.rendered, &other.rendered) && Rc::ptr_eq(&self.seen, &other.seen)
    }
}

struct OrderedText {
    callback: Callback<Rc<str>>,
    text: String,
}

impl Component for OrderedText {
    type Input = OrderedInput;
    type Message = String;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let sender = context.sender();
        let rendered = Rc::clone(&input.rendered);
        let seen = Rc::clone(&input.seen);
        Self {
            callback: Callback::new(move |value: Rc<str>| {
                seen.borrow_mut()
                    .push((value.to_string(), rendered.borrow().clone()));
                let _ = sender.send(value.to_string());
            }),
            text: String::from("Before"),
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.text = message;
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        input.rendered.replace(self.text.clone());
        TextBox::new(self.text.clone())
            .on_text_changed(self.callback.clone())
            .into()
    }
}

#[derive(Clone)]
struct WorkerInput {
    cancelled: Arc<AtomicUsize>,
    started: Arc<AtomicUsize>,
    task: Arc<Mutex<Option<ComponentTask>>>,
}

impl PartialEq for WorkerInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.cancelled, &other.cancelled)
            && Arc::ptr_eq(&self.started, &other.started)
            && Arc::ptr_eq(&self.task, &other.task)
    }
}

enum WorkerMessage {
    Complete,
    Start,
}

struct Worker(WorkerInput);

impl Component for Worker {
    type Input = WorkerInput;
    type Message = WorkerMessage;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(input.clone())
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            WorkerMessage::Complete => {}
            WorkerMessage::Start => {
                let started = Arc::clone(&self.0.started);
                let cancelled = Arc::clone(&self.0.cancelled);
                let task = context.spawn_background(move |token| {
                    started.store(1, Ordering::Release);
                    while !token.is_cancelled() {
                        std::thread::yield_now();
                    }
                    cancelled.store(1, Ordering::Release);
                    WorkerMessage::Complete
                });
                *self.0.task.lock().unwrap() = Some(task);
            }
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text("Worker").into()
    }
}

#[derive(Clone, Default)]
struct ServiceProbeInput {
    task: Arc<Mutex<Option<ComponentTask>>>,
    timer: Arc<Mutex<Option<ComponentTimer>>>,
}

impl PartialEq for ServiceProbeInput {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.task, &other.task) && Arc::ptr_eq(&self.timer, &other.timer)
    }
}

enum ServiceProbeMessage {
    Background,
    BackgroundComplete,
    Timer,
    LocalTimer,
    TimerComplete,
}

struct ServiceProbe {
    input: ServiceProbeInput,
    value: usize,
}

impl Component for ServiceProbe {
    type Input = ServiceProbeInput;
    type Message = ServiceProbeMessage;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            input: input.clone(),
            value: 0,
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        match message {
            ServiceProbeMessage::Background => {
                let task = context.spawn_background(|_| ServiceProbeMessage::BackgroundComplete);
                *self.input.task.lock().unwrap() = Some(task);
                self.value += 1;
            }
            ServiceProbeMessage::BackgroundComplete => self.value += 1,
            ServiceProbeMessage::Timer => {
                let timer = context.set_timeout(Duration::ZERO, ServiceProbeMessage::TimerComplete);
                *self.input.timer.lock().unwrap() = Some(timer);
                self.value += 1;
            }
            ServiceProbeMessage::LocalTimer => {
                let timer = context
                    .set_local_timeout(Duration::ZERO, || ServiceProbeMessage::TimerComplete);
                *self.input.timer.lock().unwrap() = Some(timer);
                self.value += 1;
            }
            ServiceProbeMessage::TimerComplete => self.value += 1,
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(self.value.to_string()).into()
    }
}

struct DropPayload(Arc<AtomicUsize>);

impl Drop for DropPayload {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

struct PayloadComponent;

impl Component for PayloadComponent {
    type Input = usize;
    type Message = DropPayload;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(input.to_string()).into()
    }
}

struct StableForward;

impl Component for StableForward {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        Button::new().on_click(context.forward()).into()
    }
}

#[test]
fn retirement_cancels_background_delivery() {
    let input = WorkerInput {
        cancelled: Arc::new(AtomicUsize::new(0)),
        started: Arc::new(AtomicUsize::new(0)),
        task: Arc::new(Mutex::new(None)),
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<Worker>("worker", input.clone())],
    )
    .unwrap();
    assert!(
        host.sender::<Worker>(&Key::from("worker"))
            .unwrap()
            .send(WorkerMessage::Start)
    );
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    while input.started.load(Ordering::Acquire) == 0 {
        std::thread::yield_now();
    }

    host.remove(&Key::from("worker")).unwrap();
    while input.cancelled.load(Ordering::Acquire) == 0 {
        std::thread::yield_now();
    }

    assert_eq!(
        input.task.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Cancelled
    );
    assert_eq!(host.drain(1).unwrap(), ComponentDrain::default());
}

#[test]
fn local_messages_and_timers_support_ui_thread_values() {
    let services = Arc::new(TestServices::default());
    let input = LocalMessageInput {
        seen: Rc::new(Cell::new(0)),
        sender: Rc::new(RefCell::new(None)),
    };
    let mut host = ComponentHost::mount_with_services(
        RecordingAdapter::default(),
        services.clone(),
        [component::<LocalMessageProbe>("probe", input.clone())],
    )
    .unwrap();

    let message = Rc::new(Cell::new(1));
    assert!(input.sender.borrow().as_ref().unwrap().send(message));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(input.seen.get(), 1);

    services.fire_timer();
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(input.seen.get(), 2);
}

#[test]
fn local_and_cross_thread_messages_preserve_enqueue_order() {
    let input = MessageOrderInput {
        local: Rc::new(RefCell::new(None)),
        seen: Rc::new(RefCell::new(Vec::new())),
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<MessageOrderProbe>("probe", input.clone())],
    )
    .unwrap();
    let remote = host
        .sender::<MessageOrderProbe>(&Key::from("probe"))
        .unwrap();
    let local = input.local.borrow().as_ref().unwrap().clone();

    assert!(remote.send(1));
    assert!(local.send(2));
    assert!(local.send(3));
    assert!(remote.send(4));
    assert_eq!(host.drain(4).unwrap().dispatched, 4);
    assert_eq!(&*input.seen.borrow(), &[1, 2, 3, 4]);
}

#[test]
fn injected_services_preserve_task_and_timer_statuses() {
    let services = Arc::new(TestServices::default());
    let input = ServiceProbeInput::default();
    let mut host = ComponentHost::mount_with_services(
        RecordingAdapter::default(),
        services.clone(),
        [component::<ServiceProbe>("probe", input.clone())],
    )
    .unwrap();
    let sender = host.sender::<ServiceProbe>(&Key::from("probe")).unwrap();

    assert!(sender.send(ServiceProbeMessage::Background));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(
        input.task.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Running
    );
    assert!(!input.task.lock().unwrap().as_ref().unwrap().is_rejected());
    services.run_background();
    assert_eq!(
        input.task.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Queued
    );
    assert!(!input.task.lock().unwrap().as_ref().unwrap().is_rejected());
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(
        input.task.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Delivered
    );
    assert!(!input.task.lock().unwrap().as_ref().unwrap().is_rejected());

    assert!(sender.send(ServiceProbeMessage::Timer));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(
        input.timer.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Running
    );
    assert!(!input.timer.lock().unwrap().as_ref().unwrap().is_rejected());
    services.fire_timer();
    assert_eq!(
        input.timer.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Queued
    );
    assert!(!input.timer.lock().unwrap().as_ref().unwrap().is_rejected());
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(
        input.timer.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Delivered
    );
    assert!(!input.timer.lock().unwrap().as_ref().unwrap().is_rejected());
}

#[test]
fn task_and_timer_rejection_distinguishes_capacity_from_cancellation() {
    for cancelled in [false, true] {
        for message in [
            ServiceProbeMessage::Background,
            ServiceProbeMessage::Timer,
            ServiceProbeMessage::LocalTimer,
        ] {
            let background = matches!(message, ServiceProbeMessage::Background);
            let services = Arc::new(TestServices::default());
            let input = ServiceProbeInput::default();
            let mut host = ComponentHost::mount_with_services(
                RecordingAdapter::default(),
                services.clone(),
                [component::<ServiceProbe>("probe", input.clone())],
            )
            .unwrap();
            let sender = host.sender::<ServiceProbe>(&Key::from("probe")).unwrap();
            assert!(sender.send(message));
            assert_eq!(host.drain(1).unwrap().dispatched, 1);
            for _ in 0..MESSAGE_CAPACITY {
                assert!(sender.send(ServiceProbeMessage::BackgroundComplete));
            }
            assert!(!sender.send(ServiceProbeMessage::BackgroundComplete));

            if background {
                let task = input.task.lock().unwrap().as_ref().unwrap().clone();
                assert!(!task.is_rejected());
                if cancelled {
                    task.cancel();
                }
                services.run_background();
                assert_eq!(task.is_rejected(), !cancelled);
                task.cancel();
                assert_eq!(task.is_rejected(), !cancelled);
                assert_eq!(
                    task.status(),
                    if cancelled {
                        ComponentTaskStatus::Cancelled
                    } else {
                        ComponentTaskStatus::Rejected
                    }
                );
            } else {
                let timer = input.timer.lock().unwrap().take().unwrap();
                assert!(!timer.is_rejected());
                if cancelled {
                    timer.cancel();
                }
                services.fire_timer();
                assert_eq!(timer.is_rejected(), !cancelled);
                timer.cancel();
                assert_eq!(timer.is_rejected(), !cancelled);
                assert_eq!(
                    timer.status(),
                    if cancelled {
                        ComponentTaskStatus::Cancelled
                    } else {
                        ComponentTaskStatus::Rejected
                    }
                );
            }
            assert_eq!(host.queue.lock().unwrap().messages.len(), MESSAGE_CAPACITY);
        }
    }
}

#[test]
fn retirement_cancels_injected_timer() {
    let services = Arc::new(TestServices::default());
    let input = ServiceProbeInput::default();
    let mut host = ComponentHost::mount_with_services(
        RecordingAdapter::default(),
        services.clone(),
        [component::<ServiceProbe>("probe", input.clone())],
    )
    .unwrap();
    assert!(
        host.sender::<ServiceProbe>(&Key::from("probe"))
            .unwrap()
            .send(ServiceProbeMessage::Timer)
    );
    assert_eq!(host.drain(1).unwrap().dispatched, 1);

    host.remove(&Key::from("probe")).unwrap();
    assert_eq!(
        input.timer.lock().unwrap().as_ref().unwrap().status(),
        ComponentTaskStatus::Cancelled
    );
    services.fire_timer();
    assert_eq!(host.drain(1).unwrap(), ComponentDrain::default());
}

#[test]
fn installing_waker_rearms_queued_messages() {
    let wakes = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<Label>("label", Rc::from("Label"))],
    )
    .unwrap();
    let sender = host.sender::<Label>(&Key::from("label")).unwrap();
    assert!(sender.send(()));
    assert_eq!(wakes.load(Ordering::Relaxed), 0);

    let callback_wakes = Arc::clone(&wakes);
    host.set_waker(move || {
        callback_wakes.fetch_add(1, Ordering::Relaxed);
    });

    assert_eq!(wakes.load(Ordering::Relaxed), 1);
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
}

#[test]
fn queued_messages_coalesce_host_wake() {
    let wakes = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<Label>("label", Rc::from("Label"))],
    )
    .unwrap();
    let callback_wakes = Arc::clone(&wakes);
    host.set_waker(move || {
        callback_wakes.fetch_add(1, Ordering::Relaxed);
    });
    let sender = host.sender::<Label>(&Key::from("label")).unwrap();

    assert!(sender.send(()));
    assert!(sender.send(()));
    assert!(sender.send(()));
    assert_eq!(wakes.load(Ordering::Relaxed), 1);
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(wakes.load(Ordering::Relaxed), 2);
    assert_eq!(host.drain(2).unwrap().dispatched, 2);

    assert!(sender.send(()));
    assert_eq!(wakes.load(Ordering::Relaxed), 3);
}

#[test]
fn native_callbacks_reconcile_before_the_next_native_occurrence() {
    let rendered = Rc::new(RefCell::new(String::new()));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<OrderedText>(
            "ordered",
            OrderedInput {
                rendered: Rc::clone(&rendered),
                seen: Rc::clone(&seen),
            },
        )],
    )
    .unwrap();
    let object = host
        .reference_at(&[Key::from("ordered")])
        .unwrap()
        .get()
        .unwrap();
    let callback = match &host.runtime().graph().events(object).unwrap()[0].value {
        EventValue::String(callback) => callback.clone(),
        _ => unreachable!(),
    };
    for value in ["A", "B"] {
        host.runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                object,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }

    let report = host.drain(usize::MAX).unwrap();

    assert_eq!(report.dispatched, 2);
    assert_eq!(
        &*seen.borrow(),
        &[
            (String::from("A"), String::from("Before")),
            (String::from("B"), String::from("A")),
        ]
    );
    assert_eq!(rendered.borrow().as_str(), "B");
}

#[test]
fn drain_budget_counts_native_work_and_rearms() {
    let rendered = Rc::new(RefCell::new(String::new()));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<OrderedText>(
            "ordered",
            OrderedInput {
                rendered,
                seen: Rc::clone(&seen),
            },
        )],
    )
    .unwrap();
    let object = host
        .reference_at(&[Key::from("ordered")])
        .unwrap()
        .get()
        .unwrap();
    let callback = match &host.runtime().graph().events(object).unwrap()[0].value {
        EventValue::String(callback) => callback.clone(),
        _ => unreachable!(),
    };
    for value in ["A", "B"] {
        host.runtime.adapter_mut().queue_native_event(
            None,
            Some(EventDispatch::new(
                object,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }
    let continuations = Rc::new(Cell::new(0));
    let woken = Rc::clone(&continuations);
    host.set_continuation_waker(move || woken.set(woken.get() + 1));

    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(seen.borrow().len(), 1);
    assert_eq!(continuations.get(), 1);
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(seen.borrow().len(), 2);
}

#[test]
fn closed_queue_rejects_stale_sender_completion_and_callback() {
    let fail_after = Rc::new(Cell::new(None));
    let rendered = Rc::new(RefCell::new(String::new()));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let input = OrderedInput {
        rendered: Rc::clone(&rendered),
        seen: Rc::clone(&seen),
    };
    let changes = Rc::new(Cell::new(0));
    let mut host = ComponentHost::mount(
        ControlledApplyAdapter {
            fail_after: Rc::clone(&fail_after),
            inner: RecordingAdapter::default(),
            successful: Rc::new(Cell::new(0)),
        },
        [
            component::<OrderedText>("ordered", input),
            component::<StatefulInputComponent>(
                "poison",
                StatefulInput {
                    changes: Rc::clone(&changes),
                    value: 0,
                },
            ),
        ],
    )
    .unwrap();
    let sender = host.sender::<OrderedText>(&Key::from("ordered")).unwrap();
    let completion = sender.completion();
    let object = host
        .reference(&Key::from("ordered"))
        .unwrap()
        .get()
        .unwrap();
    let EventValue::String(callback) = host.runtime().graph().events(object).unwrap()[0]
        .value
        .clone()
    else {
        panic!("expected string callback");
    };
    fail_after.set(Some(0));

    assert!(matches!(
        host.update_input::<StatefulInputComponent>(
            &Key::from("poison"),
            StatefulInput { changes, value: 1 }
        ),
        Err(ComponentError::Runtime(UpdateError::Adapter(())))
    ));
    assert!(host.queue.lock().unwrap().closed);
    assert!(!sender.send(String::from("sender")));
    assert!(!completion.complete(String::from("completion")));
    callback.call(Rc::from("callback"));
    assert!(host.queue.lock().unwrap().messages.is_empty());
    assert_eq!(
        seen.borrow().as_slice(),
        [(String::from("callback"), String::from("Before"))]
    );
}

#[test]
fn queue_capacity_and_closure_release_all_payloads() {
    let fail_after = Rc::new(Cell::new(None));
    let drops = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        ControlledApplyAdapter {
            fail_after: Rc::clone(&fail_after),
            inner: RecordingAdapter::default(),
            successful: Rc::new(Cell::new(0)),
        },
        [component::<PayloadComponent>("payload", 0)],
    )
    .unwrap();
    let sender = host
        .sender::<PayloadComponent>(&Key::from("payload"))
        .unwrap();
    for _ in 0..MESSAGE_CAPACITY {
        assert!(sender.send(DropPayload(Arc::clone(&drops))));
    }
    assert!(!sender.send(DropPayload(Arc::clone(&drops))));
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    fail_after.set(Some(0));

    assert!(matches!(
        host.update_input::<PayloadComponent>(&Key::from("payload"), 1),
        Err(ComponentError::Runtime(UpdateError::Adapter(())))
    ));
    assert_eq!(drops.load(Ordering::Relaxed), MESSAGE_CAPACITY + 1);
    assert!(!sender.send(DropPayload(Arc::clone(&drops))));
    assert_eq!(drops.load(Ordering::Relaxed), MESSAGE_CAPACITY + 2);
    assert!(host.queue.lock().unwrap().messages.is_empty());
}

#[test]
fn controlled_task_send_after_closure_is_cancelled_or_rejected() {
    let services = Arc::new(TestServices::default());
    let task_input = ServiceProbeInput::default();
    let fail_after = Rc::new(Cell::new(None));
    let changes = Rc::new(Cell::new(0));
    let mut host = ComponentHost::mount_with_services(
        ControlledApplyAdapter {
            fail_after: Rc::clone(&fail_after),
            inner: RecordingAdapter::default(),
            successful: Rc::new(Cell::new(0)),
        },
        services,
        [
            component::<ServiceProbe>("task", task_input.clone()),
            component::<StatefulInputComponent>(
                "poison",
                StatefulInput {
                    changes: Rc::clone(&changes),
                    value: 0,
                },
            ),
        ],
    )
    .unwrap();
    let sender = host.sender::<ServiceProbe>(&Key::from("task")).unwrap();
    assert!(sender.send(ServiceProbeMessage::Background));
    host.drain(1).unwrap();
    let task = task_input.task.lock().unwrap().as_ref().unwrap().clone();
    let control = Arc::clone(&task.control);
    assert!(control.queue());
    fail_after.set(Some(0));

    assert!(matches!(
        host.update_input::<StatefulInputComponent>(
            &Key::from("poison"),
            StatefulInput { changes, value: 1 }
        ),
        Err(ComponentError::Runtime(UpdateError::Adapter(())))
    ));
    assert_eq!(task.status(), ComponentTaskStatus::Cancelled);
    assert!(!sender.send_controlled(
        ServiceProbeMessage::BackgroundComplete,
        Arc::clone(&control)
    ));
    assert_eq!(control.status(), ComponentTaskStatus::Cancelled);

    let untracked = Arc::new(TaskControl::default());
    assert!(untracked.queue());
    assert!(!sender.send_controlled(
        ServiceProbeMessage::BackgroundComplete,
        Arc::clone(&untracked)
    ));
    assert_eq!(untracked.status(), ComponentTaskStatus::Rejected);
    assert!(ComponentTask { control: untracked }.is_rejected());
}

#[test]
fn captureless_component_callbacks_keep_event_identity() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<StableForward>("callback", ())],
    )
    .unwrap();
    let object = host
        .reference(&Key::from("callback"))
        .unwrap()
        .get()
        .unwrap();
    let before = host.runtime().graph().events(object).unwrap()[0].clone();
    host.queue_event(EventDispatch::new(
        object,
        before.id,
        before.value.clone(),
        EventPayload::Unit,
    ));

    assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 1);
    let after = host.runtime().graph().events(object).unwrap()[0].clone();
    assert_eq!(before.value, after.value);
}
