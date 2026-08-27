#![windows_subsystem = "windows"]

use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::*;

struct UseCallbackSample {
    callback: Callback<()>,
    fires: Rc<Cell<u32>>,
    rerenders: u32,
}

impl Component for UseCallbackSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        let fires = Rc::new(Cell::new(0_u32));
        let callback_fires = Rc::clone(&fires);
        Self {
            callback: Callback::new(move |()| {
                callback_fires.set(callback_fires.get() + 1);
            }),
            fires,
            rerenders: 0,
        }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.rerenders += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseCallback");
        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text(format!("callback fired {} time(s)", self.fires.get()))
                .font_size(18.0),
            TextBlock::new()
                .text(format!("forced rerenders = {}", self.rerenders))
                .font_size(12.0),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    Button::new()
                        .on_click(self.callback.clone())
                        .content(TextBlock::new().text("Fire (A)")),
                    Button::new()
                        .on_click(self.callback.clone())
                        .content(TextBlock::new().text("Fire (B)")),
                    Button::new()
                        .on_click(context.message(()))
                        .content(TextBlock::new().text("Force rerender")),
                )),
        ))
    }
}

fn main() {
    App::run_component::<UseCallbackSample>(()).unwrap();
}
