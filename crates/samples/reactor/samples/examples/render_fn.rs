#![windows_subsystem = "windows"]

use windows_reactor::*;

struct RenderFnSample {
    count: i32,
}

#[derive(Clone, Copy)]
enum Message {
    Decrement,
    Increment,
}

impl Component for RenderFnSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Decrement => self.count -= 1,
            Message::Increment => self.count += 1,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("RenderFn");
        StackPanel::new().spacing(12.0).children((
            TextBlock::new()
                .text(format!("Count: {}", self.count))
                .font_size(24.0)
                .font_weight(700),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    Button::new()
                        .on_click(context.message(Message::Decrement))
                        .content(TextBlock::new().text("-")),
                    Button::new()
                        .on_click(context.message(Message::Increment))
                        .content(TextBlock::new().text("+")),
                )),
        ))
    }
}

fn main() {
    App::run_component::<RenderFnSample>(()).unwrap();
}
