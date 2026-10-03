#![windows_subsystem = "windows"]

use std::time::Duration;

use windows_reactor::*;

const RED_PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0xb8, 0xa3, 0xa1, 0xf1,
    0x1f, 0x00, 0x05, 0x3c, 0x02, 0x2c, 0x0e, 0xc4, 0x2f, 0xc5, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];
const BLUE_PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0xd0, 0xa8, 0xb8, 0xf3,
    0x1f, 0x00, 0x04, 0xc4, 0x02, 0x7c, 0xd2, 0xca, 0xda, 0x4c, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
];

#[derive(Clone)]
enum Message {
    Tick,
    Toggle,
    Opened,
    Failed,
}

struct Sample {
    running: bool,
    swaps: u64,
    opened: u64,
    failed: u64,
}

impl Sample {
    fn schedule(context: &ComponentContext<Self>) {
        _ = context.spawn_background(|_| {
            std::thread::sleep(Duration::from_millis(16));
            Message::Tick
        });
    }
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), context: &ComponentContext<Self>) -> Self {
        Self::schedule(context);
        Self {
            running: true,
            swaps: 0,
            opened: 0,
            failed: 0,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Tick if self.running => {
                self.swaps += 1;
                Self::schedule(context);
            }
            Message::Toggle => {
                self.running = !self.running;
                if self.running {
                    Self::schedule(context);
                }
            }
            Message::Opened => self.opened += 1,
            Message::Failed => self.failed += 1,
            Message::Tick => {}
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Encoded image");
        let source = if self.swaps.is_multiple_of(2) {
            RED_PNG
        } else {
            BLUE_PNG
        };

        Border::new()
            .padding(Thickness::uniform(16.0))
            .content(
                StackPanel::new().spacing(12.0).children((
                    Image::new()
                        .width(256.0)
                        .height(256.0)
                        .source_data(EncodedImage::from_static(source))
                        .on_opened(context.message(Message::Opened))
                        .on_failed(context.message(Message::Failed)),
                    TextBlock::new().text(format!(
                        "{} swaps, {} opened, {} failed",
                        self.swaps, self.opened, self.failed
                    )),
                    Button::new()
                        .on_click(context.message(Message::Toggle))
                        .content(if self.running { "Pause" } else { "Resume" }),
                )),
            )
            .into()
    }
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
