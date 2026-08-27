#![windows_subsystem = "windows"]

use std::result::Result as StdResult;
use std::time::Duration;

use windows_reactor::*;

fn fetch_weather(attempt: i32) -> StdResult<String, String> {
    std::thread::sleep(Duration::from_millis(600));
    if attempt % 3 == 2 {
        Err("Network timeout - server unreachable".to_string())
    } else {
        Ok(format!("72F Sunny (attempt #{})", attempt + 1))
    }
}

enum ResourceState {
    Loading,
    Ready(String),
    Error(String),
    Rejected,
}

struct UseResourceRetrySample {
    attempt: i32,
    state: ResourceState,
    task: Option<ComponentTask>,
}

#[derive(Clone)]
enum Message {
    Refresh,
    Completed {
        attempt: i32,
        result: StdResult<String, String>,
    },
    Rejected {
        attempt: i32,
    },
}

impl UseResourceRetrySample {
    fn load(&mut self, attempt: i32, context: &ComponentContext<Self>) {
        if let Some(task) = self.task.take() {
            task.cancel();
        }
        self.attempt = attempt;
        self.state = ResourceState::Loading;
        self.task = Some(context.spawn_background_with_rejection(
            move |_| Message::Completed {
                attempt,
                result: fetch_weather(attempt),
            },
            Message::Rejected { attempt },
        ));
    }
}

impl Component for UseResourceRetrySample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let mut sample = Self {
            attempt: 0,
            state: ResourceState::Loading,
            task: None,
        };
        sample.load(0, context);
        sample
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Refresh => self.load(self.attempt + 1, context),
            Message::Completed { attempt, result } if attempt == self.attempt => {
                self.task = None;
                self.state = match result {
                    Ok(weather) => ResourceState::Ready(weather),
                    Err(error) => ResourceState::Error(error),
                };
            }
            Message::Rejected { attempt } if attempt == self.attempt => {
                self.task = None;
                self.state = ResourceState::Rejected;
            }
            Message::Completed { .. } | Message::Rejected { .. } => {}
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseResourceRetry");
        let content: View = match &self.state {
            ResourceState::Loading => ProgressRing::new().is_indeterminate(true).into(),
            ResourceState::Ready(weather) => TextBlock::new().text(weather).into(),
            ResourceState::Error(error) => StackPanel::new().children((
                TextBlock::new().text(format!("Error: {error}")),
                Button::new()
                    .on_click(context.message(Message::Refresh))
                    .content(TextBlock::new().text("Retry")),
            )),
            ResourceState::Rejected => StackPanel::new().children((
                TextBlock::new().text("Error: background task rejected"),
                Button::new()
                    .on_click(context.message(Message::Refresh))
                    .content(TextBlock::new().text("Retry")),
            )),
        };

        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text("Weather Service (flaky API demo)")
                .font_size(20.0),
            TextBlock::new().text(format!("Attempt: {}", self.attempt + 1)),
            content,
            Button::new()
                .on_click(context.message(Message::Refresh))
                .content(TextBlock::new().text("Refresh")),
        ))
    }
}

fn main() {
    App::run_component::<UseResourceRetrySample>(()).unwrap();
}
