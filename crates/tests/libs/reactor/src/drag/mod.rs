use super::*;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::thread::JoinHandle;

mod source;

#[derive(Clone)]
struct Input {
    mounted: bool,
    session: Rc<Session>,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.mounted == other.mounted && Rc::ptr_eq(&self.session, &other.session)
    }
}

#[derive(Default)]
struct Session {
    worker: RefCell<Option<JoinHandle<windows_core::Result<()>>>>,
    cancelled: Arc<AtomicBool>,
    stopping: Arc<AtomicBool>,
    thread: Arc<AtomicU32>,
}

impl Session {
    fn start(&self, completion: ComponentCompletion<Message>) {
        self.cancelled.store(false, Ordering::Relaxed);
        if self.worker.borrow().is_some() {
            assert!(completion.complete(Message::Ready));
            return;
        }
        let cancelled = self.cancelled.clone();
        let stopping = self.stopping.clone();
        let thread = self.thread.clone();
        *self.worker.borrow_mut() = Some(std::thread::spawn(move || {
            let result = source::run(completion.clone(), cancelled, stopping, thread);
            if let Err(error) = &result {
                _ = completion.complete(Message::Finished(Err(error.clone())));
            }
            result
        }));
    }

    fn finish(&self) -> windows_core::Result<()> {
        self.stopping.store(true, Ordering::Relaxed);
        self.cancelled.store(true, Ordering::Relaxed);
        if let Some(worker) = self.worker.borrow_mut().take() {
            let thread = self.thread.load(Ordering::Relaxed);
            let wake = source::stop(thread);
            if wake.is_err() && !worker.is_finished() {
                wake?;
            }
            worker.join().unwrap()?;
        }
        Ok(())
    }
}

struct Fixture {
    policy: Option<DragDropOperation>,
    feedback: &'static str,
    result: String,
    session: Rc<Session>,
    running: bool,
}

#[derive(Clone)]
enum Message {
    Policy(DragDropOperation),
    Start,
    Ready,
    Cancel,
    Feedback(Option<DragDropOperation>),
    Finished(windows_core::Result<source::Outcome>),
}

impl Component for Fixture {
    type Input = Input;
    type Message = Message;

    fn create(input: &Input, _: &ComponentContext<Self>) -> Self {
        Self {
            policy: input.mounted.then_some(DragDropOperation::Copy),
            feedback: "Not started",
            result: "Not started".to_owned(),
            session: input.session.clone(),
            running: false,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Policy(operation) => self.policy = Some(operation),
            Message::Start => {
                assert!(!self.running, "drag already running");
                self.running = true;
                self.feedback = "Starting";
                "Starting".clone_into(&mut self.result);
                self.session.start(context.completion());
            }
            Message::Ready => "Ready".clone_into(&mut self.result),
            Message::Cancel => self.session.cancelled.store(true, Ordering::Relaxed),
            Message::Feedback(operation) => {
                self.feedback = operation_name(operation);
            }
            Message::Finished(result) => {
                self.running = false;
                let result = result.unwrap();
                self.result = match (result.cancelled, result.operation) {
                    (true, _) => "Cancelled".to_owned(),
                    (false, Some(operation)) => format!(
                        "Dropped {}; text read {}",
                        operation_name(Some(operation)),
                        result.text_read,
                    ),
                    (false, None) => "Not accepted".to_owned(),
                };
            }
        }
    }

    fn view(&self, input: &Input, context: &mut ViewContext<Self>) -> View {
        let mut target = Border::new()
            .width(220.0)
            .height(140.0)
            .background(Color::rgb(220, 240, 220))
            .content(TextBlock::new().automation_id("target").text("Drop target"));
        if let Some(operation) = self.policy {
            target = target.drop_policy(DragDropPolicy::new().text(DragDropAction::new(operation)));
        }
        context.window_visuals(WindowVisuals::new().client_size(680.0, 440.0));
        context.window_frame(
            if input.mounted {
                "Reactor UI fixture: drag-mounted"
            } else {
                "Reactor UI fixture: drag-added"
            },
            StackPanel::new().margin(20.0).spacing(12.0).children((
                Button::new()
                    .automation_id("baseline")
                    .content("Focus baseline"),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(12.0)
                    .children((
                        Button::new()
                            .automation_id("start")
                            .content("Start drag")
                            .on_click(context.message(Message::Start)),
                        Button::new()
                            .automation_id("copy")
                            .content("Copy policy")
                            .on_click(context.message(Message::Policy(DragDropOperation::Copy))),
                        Button::new()
                            .automation_id("move")
                            .content("Move policy")
                            .on_click(context.message(Message::Policy(DragDropOperation::Move))),
                        Button::new()
                            .automation_id("cancel")
                            .content("Cancel drag")
                            .on_click(context.message(Message::Cancel)),
                    )),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(40.0)
                    .children((
                        Border::new()
                            .width(220.0)
                            .height(140.0)
                            .background(Color::rgb(240, 240, 240))
                            .content(TextBlock::new().automation_id("source").text("Text source")),
                        target,
                    )),
                TextBlock::new()
                    .automation_id("policy")
                    .text(operation_name(self.policy)),
                TextBlock::new()
                    .automation_id("feedback")
                    .text(self.feedback),
                TextBlock::new().automation_id("result").text(&self.result),
            )),
        )
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.session.cancelled.store(true, Ordering::Relaxed);
    }
}

fn operation_name(operation: Option<DragDropOperation>) -> &'static str {
    match operation {
        None => "None",
        Some(DragDropOperation::Copy) => "Copy",
        Some(DragDropOperation::Move) => "Move",
        Some(DragDropOperation::Link) => "Link",
    }
}

pub fn run(mounted: bool) -> windows_core::Result<()> {
    let session = Rc::new(Session::default());
    let result = App::run_component::<Fixture>(Input {
        mounted,
        session: session.clone(),
    });
    session.finish()?;
    result
}
