use windows_reactor::*;

struct DropDownButtonSample {
    clicks: u32,
}

impl Component for DropDownButtonSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { clicks: 0 }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.clicks += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("DropDownButton");
        StackPanel::new().spacing(8.0).children((
            DropDownButton::new()
                .on_click(context.message(()))
                .content(TextBlock::new().text("Options")),
            TextBlock::new().text(format!("Clicked {} time(s)", self.clicks)),
        ))
    }
}

fn main() {
    App::run_component::<DropDownButtonSample>(()).unwrap();
}
