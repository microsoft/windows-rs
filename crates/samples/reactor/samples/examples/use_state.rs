#![windows_subsystem = "windows"]

use windows_reactor::*;

struct UseStateSample {
    count: u32,
}

impl Component for UseStateSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.count += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseState");
        StackPanel::new().children((
            Button::new().on_click(context.forward()).content("Click"),
            TextBlock::new()
                .text(format!("count = {}", self.count))
                .font_size(18.0),
        ))
    }
}

fn main() {
    App::run_component::<UseStateSample>(()).unwrap();
}
