#![windows_subsystem = "windows"]

use std::time::Duration;

use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Bump,
    Completed(i32),
}

struct AsyncStateSample {
    busy: bool,
    count: i32,
}

impl Component for AsyncStateSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            busy: false,
            count: 0,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Bump if !self.busy => {
                self.busy = true;
                let count = self.count + 1;
                _ = context.spawn_background(move |_| {
                    std::thread::sleep(Duration::from_millis(500));
                    Message::Completed(count)
                });
            }
            Message::Completed(count) => {
                self.count = count;
                self.busy = false;
            }
            Message::Bump => {}
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("AsyncState");
        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text(format!("count = {}", self.count))
                .font_size(24.0),
            TextBlock::new()
                .text(if self.busy {
                    "working off the UI thread..."
                } else {
                    "idle"
                })
                .font_size(12.0),
            Button::new()
                .is_enabled(!self.busy)
                .on_click(context.message(Message::Bump))
                .content(TextBlock::new().text("Bump (off-thread)")),
        ))
    }
}

fn main() {
    App::run_component::<AsyncStateSample>(()).unwrap();
}
