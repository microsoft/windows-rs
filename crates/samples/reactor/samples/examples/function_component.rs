#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, PartialEq)]
struct GreetingInput {
    name: String,
}

struct Greeting;

impl Component for Greeting {
    type Message = ();
    type Input = GreetingInput;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        TextBlock::new()
            .text(format!("Hello, {}!", input.name))
            .font_size(20.0)
            .font_weight(700)
            .into()
    }
}

struct Counter {
    count: i32,
}

impl Component for Counter {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.count += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text(format!("count = {}", self.count))
                .font_size(24.0)
                .font_weight(700),
            Button::new()
                .on_click(context.message(()))
                .content("Increment"),
        ))
    }
}

struct FunctionComponentSample {
    name: String,
}

impl Component for FunctionComponentSample {
    type Message = String;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            name: "world".into(),
        }
    }

    fn update(&mut self, name: String, _context: &ComponentContext<Self>) {
        self.name = name;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("FunctionComponent");
        StackPanel::new().spacing(12.0).children((
            View::component::<Greeting>(GreetingInput {
                name: self.name.clone(),
            }),
            TextBox::new()
                .text(self.name.clone())
                .placeholder_text("Type a name...")
                .on_text_changed(context.callback(std::convert::identity))
                .slots([SlotView::new(TextBoxSlot::Header, "Your name")]),
            View::component::<Counter>(()),
        ))
    }
}

fn main() {
    App::run_component::<FunctionComponentSample>(()).unwrap();
}
