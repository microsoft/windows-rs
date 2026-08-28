#![windows_subsystem = "windows"]

use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Decrement,
    Increment,
    Toggle,
}

struct UseEffectSample {
    count: i32,
    flag: bool,
    last_seen: Rc<Cell<i32>>,
}

impl Component for UseEffectSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            count: 0,
            flag: false,
            last_seen: Rc::new(Cell::new(0)),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Decrement => self.count -= 1,
            Message::Increment => self.count += 1,
            Message::Toggle => self.flag = !self.flag,
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseEffect");
        let count = self.count;
        let last_seen = Rc::clone(&self.last_seen);
        context.use_effect("count", count, move || {
            last_seen.set(count);
            None
        });
        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text(format!("count = {}", self.count))
                .font_size(24.0),
            format!("use_effect last observed: {}", self.last_seen.get()),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    Button::new()
                        .on_click(context.message(Message::Decrement))
                        .content("-"),
                    Button::new()
                        .on_click(context.message(Message::Increment))
                        .content("+"),
                    Button::new()
                        .on_click(context.message(Message::Toggle))
                        .content("toggle unrelated state"),
                )),
            TextBlock::new()
                .text(format!("unrelated flag = {}", self.flag))
                .font_size(12.0),
        ))
    }
}

fn main() {
    App::run_component::<UseEffectSample>(()).unwrap();
}
