use super::*;
use std::thread::{self, ThreadId};
use std::time::Duration;

pub(super) struct TimerFixture {
    generation: u32,
    retained: Option<ComponentTimer>,
    fired: Vec<u32>,
    released: Vec<u32>,
    report: Rc<ComponentCompletion<Message>>,
}

pub(super) enum Message {
    Fired(u32),
    Released(u32),
    Replace(u32, ComponentTimer),
}

impl Component for TimerFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        Self {
            generation: 0,
            retained: None,
            fired: Vec::new(),
            released: Vec::new(),
            report: Rc::new(context.completion()),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Fired(generation) => self.fired.push(generation),
            Message::Released(generation) => self.released.push(generation),
            Message::Replace(generation, timer) => {
                assert_eq!(generation, self.generation);
                // Keep the handle alive so its Drop cannot hide a retirement cancellation bug.
                self.retained = Some(timer);
                self.generation += 1;
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: timer");
        context.window_visuals(WindowVisuals::new().client_size(500.0, 400.0));
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
                    .automation_id("fired")
                    .text(format!("Fired {:?}", self.fired)),
                TextBlock::new()
                    .automation_id("released")
                    .text(format!("Released {:?}", self.released)),
                StackPanel::new().keyed_children([keyed(
                    self.generation,
                    View::component::<TimedChild>(TimerInput {
                        generation: self.generation,
                        report: Rc::clone(&self.report),
                    }),
                )]),
            ))
            .into()
    }
}

#[derive(Clone)]
struct TimerInput {
    generation: u32,
    report: Rc<ComponentCompletion<Message>>,
}

impl PartialEq for TimerInput {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation && Rc::ptr_eq(&self.report, &other.report)
    }
}

struct ReleaseGuard {
    generation: u32,
    report: ComponentCompletion<Message>,
}

impl Drop for ReleaseGuard {
    fn drop(&mut self) {
        assert!(
            self.report
                .clone()
                .complete(Message::Released(self.generation)),
            "timer payload release was rejected"
        );
    }
}

struct TimedChild {
    input: TimerInput,
    ui_thread: ThreadId,
    _timer: ComponentTimer,
}

enum TimerMessage {
    Tick,
    Replace,
    Pending(ReleaseGuard),
}

impl Component for TimedChild {
    type Input = TimerInput;
    type Message = TimerMessage;

    fn create(input: &TimerInput, context: &ComponentContext<Self>) -> Self {
        let ui_thread = thread::current().id();
        Self {
            input: input.clone(),
            ui_thread,
            _timer: context.set_local_timeout(Duration::from_millis(1), move || {
                assert_eq!(thread::current().id(), ui_thread);
                TimerMessage::Tick
            }),
        }
    }

    fn update(&mut self, message: TimerMessage, context: &ComponentContext<Self>) {
        assert_eq!(thread::current().id(), self.ui_thread);
        let report = self.input.report.as_ref().clone();
        let generation = self.input.generation;
        match message {
            TimerMessage::Tick => {
                assert!(report.complete(Message::Fired(generation)));
            }
            TimerMessage::Replace => {
                let pending = ReleaseGuard {
                    generation,
                    report: report.clone(),
                };
                // Cancellation must release the payload without waiting for this deadline.
                let timer =
                    context.set_timeout(Duration::from_secs(3_600), TimerMessage::Pending(pending));
                assert!(report.complete(Message::Replace(generation, timer)));
            }
            TimerMessage::Pending(_guard) => panic!("pending timer fired before cancellation"),
        }
    }

    fn view(&self, _: &TimerInput, context: &mut ViewContext<Self>) -> View {
        Button::new()
            .automation_id("replace")
            .content("Arm timer and replace component")
            .on_click(context.callback(|_| TimerMessage::Replace))
            .into()
    }
}
