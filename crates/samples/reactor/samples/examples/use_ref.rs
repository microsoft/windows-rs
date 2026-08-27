#![windows_subsystem = "windows"]

use std::cell::Cell;

use windows_reactor::*;

struct UseRefSample {
    clicks: u32,
    renders: Cell<u64>,
}

impl Component for UseRefSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            clicks: 0,
            renders: Cell::new(0),
        }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseRef");
        self.renders.set(self.renders.get() + 1);
        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text(format!("clicks (component state) = {}", self.clicks))
                .font_size(18.0),
            TextBlock::new()
                .text(format!("renders (Cell) = {}", self.renders.get()))
                .font_size(18.0),
            Button::new()
                .on_click(context.message(()))
                .content(TextBlock::new().text("Click me")),
            TextBlock::new()
                .text(
                    "The Cell counter increments every render; the state counter only on button \
                     click. Cell mutation never schedules a render.",
                )
                .font_size(12.0),
        ))
    }
}

fn main() {
    App::run_component::<UseRefSample>(()).unwrap();
}
