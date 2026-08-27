#![windows_subsystem = "windows"]

use windows_reactor::*;

struct Counter {
    count: u32,
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
                .font_size(20.0),
            Button::new()
                .on_click(context.message(()))
                .content(TextBlock::new().text("Increment")),
        ))
    }
}

struct PassThrough;

impl Component for PassThrough {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::component::<Counter>(())
    }
}

struct PassThroughSample;

impl Component for PassThroughSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("PassThroughComponent");
        Border::new()
            .padding(16.0)
            .content(StackPanel::new().spacing(12.0).children((
                TextBlock::new().text("The wrapper returns the stateful component directly."),
                TextBlock::new().text("Clicking Increment must continue to update the count."),
                View::component::<PassThrough>(()),
            )))
    }
}

fn main() {
    App::run_component::<PassThroughSample>(()).unwrap();
}
