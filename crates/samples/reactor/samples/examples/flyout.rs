#![windows_subsystem = "windows"]

use windows_reactor::*;

struct FlyoutSample(u32);

impl Component for FlyoutSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(0)
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {
        self.0 += 1;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Flyout");
        StackPanel::new().spacing(8.0).children((
            Button::new()
                .content("Show Flyout")
                .flyout("Hello from the flyout!"),
            Button::new().content("Bottom Flyout").flyout_with(
                Flyout::text(format!("Clicked {} times", self.0))
                    .placement(FlyoutPlacement::Bottom),
            ),
            Button::new()
                .on_click(context.message(()))
                .content("Increment"),
        ))
    }
}

fn main() {
    App::run_component::<FlyoutSample>(()).unwrap();
}
