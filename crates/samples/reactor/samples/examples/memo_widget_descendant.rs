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
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(12.0)
            .children((
                TextBlock::new()
                    .text(format!("count = {}", self.count))
                    .font_size(20.0),
                Button::new()
                    .on_click(context.forward())
                    .content("Increment"),
            ))
    }
}

struct MemoizedFrame;

impl Component for MemoizedFrame {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Border::new()
            .padding(12.0)
            .content(View::component::<Counter>(()))
    }
}

struct MemoWidgetDescendantSample;

impl Component for MemoWidgetDescendantSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("MemoWidgetDescendant");
        Border::new()
            .padding(16.0)
            .content(StackPanel::new().spacing(12.0).children((
                "A dirty child must update through a memoized component with a widget root.",
                "Click Increment. The count must advance on every click.",
                View::component::<MemoizedFrame>(()),
            )))
    }
}

fn main() {
    App::run_component::<MemoWidgetDescendantSample>(()).unwrap();
}
