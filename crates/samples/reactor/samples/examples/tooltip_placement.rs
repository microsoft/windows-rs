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
                .content(TextBlock::new().text("Top"))
                .tooltip_with(Tooltip::text("Anchored above").placement(TooltipPlacement::Top)),
            Button::new()
                .content(TextBlock::new().text("Bottom"))
                .tooltip_with(Tooltip::text("Anchored below").placement(TooltipPlacement::Bottom)),
            Button::new()
                .content(TextBlock::new().text("Left"))
                .tooltip_with(
                    Tooltip::text("Anchored to the left").placement(TooltipPlacement::Left),
                ),
            Button::new()
                .content(TextBlock::new().text("Right"))
                .tooltip_with(
                    Tooltip::text("Anchored to the right").placement(TooltipPlacement::Right),
                ),
            Button::new()
                .content(TextBlock::new().text("Mouse"))
                .tooltip_with(
                    Tooltip::text("Follows the cursor").placement(TooltipPlacement::Mouse),
                ),
        ))
    }
}

fn main() {
    App::run_component::<TooltipPlacementSample>(()).unwrap();
}
