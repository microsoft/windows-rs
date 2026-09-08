#![windows_subsystem = "windows"]

use windows_reactor::*;

struct TooltipPlacementSample;

impl Component for TooltipPlacementSample {
    type Message = ();
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TooltipPlacement");
        StackPanel::new().spacing(8.0).children((
            Button::new()
                .content("Top")
                .tooltip_with(Tooltip::text("Anchored above").placement(TooltipPlacement::Top)),
            Button::new()
                .content("Bottom")
                .tooltip_with(Tooltip::text("Anchored below").placement(TooltipPlacement::Bottom)),
            Button::new().content("Left").tooltip_with(
                Tooltip::text("Anchored to the left").placement(TooltipPlacement::Left),
            ),
            Button::new().content("Right").tooltip_with(
                Tooltip::text("Anchored to the right").placement(TooltipPlacement::Right),
            ),
            Button::new().content("Mouse").tooltip_with(
                Tooltip::text("Follows the cursor").placement(TooltipPlacement::Mouse),
            ),
        ))
    }
}

fn main() {
    App::run_component::<TooltipPlacementSample>(()).unwrap();
}
