#![windows_subsystem = "windows"]

use windows_reactor::*;

struct Counter {
    count: u32,
}

impl Component for Counter {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.count += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        StackPanel::new().spacing(8.0).children((
            self.count.to_string(),
            Button::new().on_click(context.forward()).content("+"),
        ))
    }
}

fn main() {
    App::run_component::<Counter>(()).unwrap();
}
