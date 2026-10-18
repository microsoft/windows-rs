use super::*;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, ThreadId};
use std::time::Duration;

pub(super) struct BackgroundFixture {
    generation: u32,
    delivered: Vec<u32>,
    started: Vec<u32>,
    released: Vec<(u32, bool)>,
    release: Option<Sender<()>>,
    retained: Option<ComponentTask>,
    report: Rc<ComponentCompletion<Message>>,
    ui_thread: ThreadId,
}

pub(super) enum Message {
    Delivered(u32),
    Started(u32),
    Released(u32, bool),
    Armed(u32, Sender<()>, ComponentTask),
    Replace,
    Release,
}

impl Component for BackgroundFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        Self {
            generation: 0,
            delivered: Vec::new(),
            started: Vec::new(),
            released: Vec::new(),
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
            Message::Armed(generation, release, task) => {
                assert_eq!(generation, self.generation);
                assert!(self.release.replace(release).is_none());
                assert!(self.retained.replace(task).is_none());
            }
            Message::Replace => {
                assert!(self.retained.is_some());
                assert!(self.started.contains(&self.generation));
                self.generation += 1;
            }
            Message::Release => self.release.take().unwrap().send(()).unwrap(),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: background");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 450.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                TextBlock::new().automation_id("state").text(format!(
                    "Generation {}; retained {}",
                    self.generation,
                    self.retained.is_some()
                )),
                TextBlock::new()
                    .automation_id("delivered")
                    .text(format!("Delivered {:?}", self.delivered)),
                TextBlock::new()
                    .automation_id("started")
                    .text(format!("Started {:?}", self.started)),
                TextBlock::new()
                    .automation_id("released")
                    .text(format!("Released {:?}", self.released)),
                StackPanel::new().keyed_children([keyed(
                    self.generation,
                    View::component::<Worker>(WorkerInput {
                        generation: self.generation,
                        report: Rc::clone(&self.report),
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
            ))
            .into()
    }
}

#[derive(Clone)]
struct WorkerInput {
    generation: u32,
    report: Rc<ComponentCompletion<Message>>,
}

impl PartialEq for WorkerInput {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation && Rc::ptr_eq(&self.report, &other.report)
    }
}

struct LateResult {
    generation: u32,
    cancelled: bool,
    report: ComponentCompletion<Message>,
}

impl Drop for LateResult {
    fn drop(&mut self) {
        assert!(
            self.report
                .clone()
                .complete(Message::Released(self.generation, self.cancelled)),
            "late result release was rejected"
        );
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
                let task = context.spawn_background(move |cancellation| {
                    assert_ne!(thread::current().id(), ui_thread);
                    assert!(!cancellation.is_cancelled());
                    assert!(worker_report.clone().complete(Message::Started(generation)));
                    wait.recv_timeout(Duration::from_secs(30)).unwrap();
                    // Return a result after cancellation so Reactor must discard its payload.
                    WorkerMessage::Late(LateResult {
                        generation,
                        cancelled: cancellation.is_cancelled(),
                        report: worker_report,
                    })
                });
                assert!(report.complete(Message::Armed(generation, release, task)));
            }
            WorkerMessage::Late(_result) => panic!("retired worker result was delivered"),
        }
    }

    fn view(&self, _: &WorkerInput, context: &mut ViewContext<Self>) -> View {
        Button::new()
            .automation_id("start")
            .content("Start pending work")
            .on_click(context.callback(|_| WorkerMessage::Start))
            .into()
    }
}
