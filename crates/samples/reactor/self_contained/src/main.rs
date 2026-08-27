#![windows_subsystem = "windows"]

use windows_reactor::*;

struct HelloWorld;

impl Component for HelloWorld {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor app");
        TextBlock::new().text("Hello, world!").into()
    }
}

fn main() {
    App::run_component::<HelloWorld>(()).unwrap();
}
