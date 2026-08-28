#![windows_subsystem = "windows"]

use windows_reactor::*;

struct Notepad {
    text: String,
}

impl Component for Notepad {
    type Message = String;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            text: String::new(),
        }
    }

    fn update(&mut self, message: String, _context: &ComponentContext<Self>) {
        self.text = message;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("windows_reactor — notepad");
        TextBox::new()
            .text(self.text.clone())
            .accepts_return(true)
            .placeholder_text("Start typing…")
            .on_text_changed(context.forward())
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .vertical_alignment(VerticalAlignment::Stretch)
            .into()
    }
}

fn main() {
    App::run_component::<Notepad>(()).unwrap();
}
