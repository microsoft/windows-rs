#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Increment,
    Decrement,
}

struct RepeatButtonSample {
    count: i32,
}

impl Component for RepeatButtonSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("RepeatButton");
        StackPanel::new().spacing(8.0).children((
            format!("Count: {}", self.count),
            RepeatButton::new()
                .on_click(context.message(Message::Increment))
                .delay(300)
                .interval(50)
                .content("+1 (hold to repeat)"),
            RepeatButton::new()
                .on_click(context.message(Message::Decrement))
                .delay(300)
                .interval(50)
                .content("-1 (hold to repeat)"),
            RepeatButton::new().is_enabled(false).content("Disabled"),
        ))
    }
}

fn main() {
    App::run_component::<RepeatButtonSample>(()).unwrap();
}
