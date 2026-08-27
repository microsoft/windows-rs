use windows_reactor::*;

#[derive(Clone)]
enum Message {
    Decrement,
    Increment,
    Reset,
}

struct Counter {
    count: i32,
}

impl Component for Counter {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Decrement => self.count -= 1,
            Message::Increment => self.count += 1,
            Message::Reset => self.count = 0,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("windows_reactor - counter");
        StackPanel::new().spacing(12.0).children((
            "Phase 1 demo",
            TextBlock::new()
                .text(format!("Count: {}", self.count))
                .font_weight(700)
                .font_size(28.0)
                .automation_heading_level(AutomationHeadingLevel::Level1)
                .automation_id("count-label"),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    Button::new()
                        .on_click(context.message(Message::Decrement))
                        .automation_name("Decrement")
                        .automation_id("decrement-button")
                        .content("-"),
                    Button::new()
                        .on_click(context.message(Message::Increment))
                        .automation_name("Increment")
                        .automation_id("increment-button")
                        .content("+"),
                    Button::new()
                        .on_click(context.message(Message::Reset))
                        .key_accelerators(KeyAccelerators::new([KeyAccelerator::new(
                            AcceleratorKey::R,
                            AcceleratorModifiers::Control,
                            context.message(Message::Reset),
                        )]))
                        .automation_id("reset-button")
                        .content("reset (Ctrl+R)"),
                )),
        ))
    }
}

fn main() {
    App::run_component::<Counter>(()).unwrap();
}
