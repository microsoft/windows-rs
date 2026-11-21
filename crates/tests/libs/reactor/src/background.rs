use super::*;
use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, ThreadId};
use std::time::Duration;

pub(super) fn run(shutdown: bool) -> windows_core::Result<()> {
    if !shutdown {
        return App::run_component::<BackgroundFixture>(Input(None));
    }

    let (acknowledge, acknowledged) = mpsc::channel();
    let shutdown = Rc::new(Shutdown {
        pending: RefCell::new(None),
        timer_released: Arc::new(AtomicBool::new(false)),
        timer_released_at_cleanup: Cell::new(None),
        acknowledge,
    });
    App::run_component::<BackgroundFixture>(Input(Some(shutdown.clone())))?;

    let pending = shutdown.pending.borrow_mut().take().unwrap();
    pending.0.send(()).unwrap();
    let cancelled = acknowledged.recv_timeout(Duration::from_secs(5)).unwrap();
    // Keep both handles alive until the worker has reported its pre-enqueue cancellation state.
    drop(pending);
    assert_eq!(
        (shutdown.timer_released_at_cleanup.get(), cancelled),
        (Some(true), true),
        "host shutdown must release the timer before owner cleanup and cancel the blocked worker"
    );
    Ok(())
}

struct Shutdown {
    pending: RefCell<Option<(Sender<()>, ComponentTask, ComponentTimer)>>,
    timer_released: Arc<AtomicBool>,
    timer_released_at_cleanup: Cell<Option<bool>>,
    acknowledge: Sender<bool>,
}

#[derive(Clone)]
struct Input(Option<Rc<Shutdown>>);

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().map(Rc::as_ptr) == other.0.as_ref().map(Rc::as_ptr)
    }
}

struct BackgroundFixture {
    shutdown: Input,
    generation: u32,
    delivered: Vec<u32>,
    started: Vec<u32>,
    released: Vec<(u32, bool)>,
    timers_released: Vec<u32>,
    release: Option<Sender<()>>,
    retained: Option<(ComponentTask, ComponentTimer)>,
    report: Rc<ComponentCompletion<Message>>,
    ui_thread: ThreadId,
}

pub(super) enum Message {
    Delivered(u32),
    Started(u32),
    Released(u32, bool),
    TimerReleased(u32),
    Armed(u32, Sender<()>, ComponentTask, ComponentTimer),
    Replace,
    Release,
}

impl Component for BackgroundFixture {
    type Input = Input;
    type Message = Message;

    fn create(input: &Input, context: &ComponentContext<Self>) -> Self {
        Self {
            shutdown: input.clone(),
            generation: 0,
            delivered: Vec::new(),
            started: Vec::new(),
            released: Vec::new(),
            timers_released: Vec::new(),
            release: None,
            retained: None,
            report: Rc::new(context.completion()),
            ui_thread: thread::current().id(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        assert_eq!(thread::current().id(), self.ui_thread);
        match message {
            Message::Delivered(generation) => self.delivered.push(generation),
            Message::Started(generation) => self.started.push(generation),
            Message::Released(generation, cancelled) => self.released.push((generation, cancelled)),
            Message::TimerReleased(generation) => self.timers_released.push(generation),
            Message::Armed(generation, release, task, timer) => {
                assert_eq!(generation, self.generation);
                if let Some(shutdown) = &self.shutdown.0 {
                    assert!(
                        shutdown
                            .pending
                            .replace(Some((release, task, timer)))
                            .is_none()
                    );
                } else {
                    assert!(self.release.replace(release).is_none());
                    assert!(self.retained.replace((task, timer)).is_none());
                }
            }
            Message::Replace => {
                assert!(self.retained.is_some());
                assert!(self.started.contains(&self.generation));
                self.generation += 1;
            }
            Message::Release => self.release.take().unwrap().send(()).unwrap(),
        }
    }

    fn view(&self, _: &Input, context: &mut ViewContext<Self>) -> View {
        context.window_title(if self.shutdown.0.is_some() {
            "Reactor UI fixture: background-shutdown"
        } else {
            "Reactor UI fixture: background"
        });
        context.window_visuals(WindowVisuals::new().client_size(500.0, 450.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                TextBlock::new().automation_id("state").text(format!(
                    "Generation {}; retained {}",
                    self.generation,
                    self.retained.is_some()
                        || self
                            .shutdown
                            .0
                            .as_ref()
                            .is_some_and(|shutdown| shutdown.pending.borrow().is_some())
                )),
                TextBlock::new()
                    .automation_id("delivered")
                    .text(format!("Delivered {:?}", self.delivered)),
                TextBlock::new()
                    .automation_id("started")
                    .text(format!("Started {:?}", self.started)),
                TextBlock::new().automation_id("released").text(format!(
                    "Released {:?}; timers {:?}",
                    self.released, self.timers_released
                )),
                StackPanel::new().keyed_children([keyed(
                    self.generation,
                    View::component::<Worker>(WorkerInput {
                        generation: self.generation,
                        report: Rc::clone(&self.report),
                        shutdown: self.shutdown.clone(),
                    }),
                )]),
                Button::new()
                    .automation_id("replace")
                    .content("Replace component")
                    .on_click(context.callback(|_| Message::Replace)),
                Button::new()
                    .automation_id("release")
                    .content("Release worker")
                    .on_click(context.callback(|_| Message::Release)),
                View::component::<LocalFactory>(()),
            ))
            .into()
    }
}

struct LocalFactory {
    value: Rc<str>,
    owner: ThreadId,
    order: Vec<u8>,
}

enum LocalFactoryMessage {
    Start(ComponentCompletion<Self>),
    Ordered(u8),
    Completed(Rc<str>),
}

impl Component for LocalFactory {
    type Input = ();
    type Message = LocalFactoryMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            value: Rc::from("Waiting for factory"),
            owner: thread::current().id(),
            order: Vec::new(),
        }
    }

    fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
        assert_eq!(thread::current().id(), self.owner);
        match message {
            LocalFactoryMessage::Start(view_completion) => {
                self.order.clear();
                assert!(context.sender().send(LocalFactoryMessage::Ordered(1)));
                assert!(view_completion.complete_with(|| LocalFactoryMessage::Ordered(2)));
                assert!(context.sender().send(LocalFactoryMessage::Ordered(3)));
                let completion = context.completion();
                let owner = self.owner;
                assert!(
                    thread::spawn(move || {
                        completion.complete_with(move || {
                            assert_eq!(thread::current().id(), owner);
                            LocalFactoryMessage::Completed(Rc::from("Created on UI thread"))
                        })
                    })
                    .join()
                    .unwrap()
                );
            }
            LocalFactoryMessage::Ordered(value) => self.order.push(value),
            LocalFactoryMessage::Completed(value) => {
                assert_eq!(self.order, [1, 2, 3]);
                self.value = value;
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let completion = context.completion();
        StackPanel::new()
            .children((
                TextBlock::new()
                    .automation_id("factory-result")
                    .text(self.value.clone()),
                Button::new()
                    .automation_id("factory-start")
                    .content("Submit message factory")
                    .on_click(
                        context.callback(move |_| LocalFactoryMessage::Start(completion.clone())),
                    ),
            ))
            .into()
    }
}

#[derive(Clone)]
struct WorkerInput {
    generation: u32,
    report: Rc<ComponentCompletion<Message>>,
    shutdown: Input,
}

impl PartialEq for WorkerInput {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation
            && Rc::ptr_eq(&self.report, &other.report)
            && self.shutdown == other.shutdown
    }
}

struct LateResult {
    generation: u32,
    cancelled: bool,
    report: ComponentCompletion<Message>,
    acknowledge: Option<Sender<bool>>,
}

impl Drop for LateResult {
    fn drop(&mut self) {
        _ = self
            .report
            .clone()
            .complete(Message::Released(self.generation, self.cancelled));
        if let Some(acknowledge) = self.acknowledge.take() {
            acknowledge.send(self.cancelled).unwrap();
        }
    }
}

struct Worker {
    input: WorkerInput,
    ui_thread: ThreadId,
}

enum WorkerMessage {
    Completed,
    Start,
    Late(LateResult),
    PendingTimer(TimerPayload),
}

struct TimerPayload {
    generation: u32,
    report: ComponentCompletion<Message>,
    released: Option<Arc<AtomicBool>>,
}

impl Drop for TimerPayload {
    fn drop(&mut self) {
        if let Some(released) = &self.released {
            released.store(true, Ordering::SeqCst);
        }
        // Shutdown closes the host queue before cancelling pending timers.
        _ = self
            .report
            .clone()
            .complete(Message::TimerReleased(self.generation));
    }
}

impl Component for Worker {
    type Input = WorkerInput;
    type Message = WorkerMessage;

    fn create(input: &WorkerInput, context: &ComponentContext<Self>) -> Self {
        let ui_thread = thread::current().id();
        context.spawn_background(move |cancellation| {
            assert_ne!(thread::current().id(), ui_thread);
            assert!(!cancellation.is_cancelled());
            WorkerMessage::Completed
        });
        Self {
            input: input.clone(),
            ui_thread,
        }
    }

    fn update(&mut self, message: WorkerMessage, context: &ComponentContext<Self>) {
        assert_eq!(thread::current().id(), self.ui_thread);
        let report = self.input.report.as_ref().clone();
        let generation = self.input.generation;
        match message {
            WorkerMessage::Completed => {
                assert!(report.complete(Message::Delivered(generation)));
            }
            WorkerMessage::Start => {
                let (release, wait) = mpsc::channel();
                let worker_report = report.clone();
                let ui_thread = self.ui_thread;
                let acknowledge = self
                    .input
                    .shutdown
                    .0
                    .as_ref()
                    .map(|shutdown| shutdown.acknowledge.clone());
                let task = context.spawn_background(move |cancellation| {
                    assert_ne!(thread::current().id(), ui_thread);
                    assert!(!cancellation.is_cancelled());
                    assert!(worker_report.clone().complete(Message::Started(generation)));
                    match wait.recv_timeout(Duration::from_secs(30)) {
                        Ok(()) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => {
                            assert!(cancellation.is_cancelled());
                        }
                        Err(error) => panic!("{error}"),
                    }
                    // Return a result after cancellation so Reactor must discard its payload.
                    WorkerMessage::Late(LateResult {
                        generation,
                        cancelled: cancellation.is_cancelled(),
                        report: worker_report,
                        acknowledge,
                    })
                });
                let pending = TimerPayload {
                    generation,
                    report: report.clone(),
                    released: self
                        .input
                        .shutdown
                        .0
                        .as_ref()
                        .map(|shutdown| shutdown.timer_released.clone()),
                };
                let timer = context.set_local_timeout(Duration::from_secs(3_600), move || {
                    WorkerMessage::PendingTimer(pending)
                });
                // The parent retains these across replacement, or forwards them outside the host.
                assert!(report.complete(Message::Armed(generation, release, task, timer)));
            }
            WorkerMessage::Late(_result) => panic!("retired worker result was delivered"),
            WorkerMessage::PendingTimer(_payload) => panic!("pending local timer fired"),
        }
    }

    fn view(&self, _: &WorkerInput, context: &mut ViewContext<Self>) -> View {
        if let Some(shutdown) = self.input.shutdown.0.clone() {
            context.use_effect("shutdown", (), move || {
                Some(Box::new(move || {
                    // Capture before scope destruction and final application timer cleanup.
                    assert!(
                        shutdown
                            .timer_released_at_cleanup
                            .replace(Some(shutdown.timer_released.load(Ordering::SeqCst)))
                            .is_none()
                    );
                }))
            });
        }
        Button::new()
            .automation_id("start")
            .content("Start pending work")
            .on_click(context.callback(|_| WorkerMessage::Start))
            .into()
    }
}
