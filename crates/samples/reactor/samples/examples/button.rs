#![windows_subsystem = "windows"]

use windows_reactor::*;

struct ButtonSample {
    clicks: u32,
}

impl Component for ButtonSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { clicks: 0 }
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Button");
        StackPanel::new().spacing(8.0).children((
            Button::new()
                .on_click(context.message(()))
                .content(format!("Clicked {} times", self.clicks)),
            Button::new().is_enabled(false).content("Disabled"),
            Button::new()
                .style(ButtonStyle::Accent)
                .content("Accent (Primary Action)"),
        ))
    }
}

fn main() {
    App::run_component::<ButtonSample>(()).unwrap();
}
