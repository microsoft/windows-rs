#![windows_subsystem = "windows"]

use windows_reactor_next::*;

struct Counter {
    count: u32,
}

impl Component for Counter {
    type Message = ();
    type Props = ();

    fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {
        self.count += 1;
    }

    fn view(&self, _props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        let increment = context.sender();
        StackPanel::new().spacing(8.0).children([
            TextBlock::new().text(self.count.to_string()).into(),
            Button::new()
                .on_click(move || {
                    _ = increment.send(());
                })
                .content(TextBlock::new().text("+")),
        ])
    }
}

fn main() {
    bootstrap().unwrap();
    App::run_component::<Counter>(()).unwrap();
}
