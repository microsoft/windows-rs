use windows_reactor::*;

struct SplitButtonSample {
    clicks: u32,
}

impl Component for SplitButtonSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { clicks: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("SplitButton");
        StackPanel::new().spacing(8.0).children((
            SplitButton::new()
                .on_click(context.forward())
                .content(format!("Primary action ({})", self.clicks)),
            SplitButton::new().is_enabled(false).content("Disabled"),
        ))
    }
}

fn main() {
    App::run_component::<SplitButtonSample>(()).unwrap();
}
