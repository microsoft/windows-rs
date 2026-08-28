#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, PartialEq)]
struct GreetingInput {
    clicks: u32,
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
        StackPanel::new().spacing(4.0).children((
            TextBlock::new()
                .text(format!("Hello, {}!", input.name))
                .font_size(20.0),
            format!("You have clicked the button {} times.", input.clicks),
        ))
    }
}

struct ComponentInputSample {
    clicks: u32,
    name: String,
}

impl Component for ComponentInputSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            clicks: 0,
            name: "world".into(),
        }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("ComponentInput");
        Border::new().padding(16.0).content(
            StackPanel::new().spacing(12.0).children((
                TextBlock::new()
                    .text("windows-reactor - component input")
                    .font_size(24.0),
                View::component::<Greeting>(GreetingInput {
                    clicks: self.clicks,
                    name: self.name.clone(),
                }),
                Button::new()
                    .on_click(context.forward())
                    .content("Click me"),
            )),
        )
    }
}

fn main() {
    App::run_component::<ComponentInputSample>(()).unwrap();
}
