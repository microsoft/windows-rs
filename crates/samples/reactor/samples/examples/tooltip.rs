#![windows_subsystem = "windows"]

use windows_reactor::*;

struct TooltipSample;

impl Component for TooltipSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Tooltip");
        StackPanel::new().spacing(12.0).children((
            Button::new()
                .content("Hover me")
                .tooltip("This is a tooltip"),
            TextBlock::new()
                .text("Plain text also tips")
                .tooltip("Even on TextBlock"),
        ))
    }
}

fn main() {
    App::run_component::<TooltipSample>(()).unwrap();
}
