#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, RenderCx, Tooltip, TooltipPlacement, vstack};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            Button::new("Top")
                .build()
                .tooltip_with(Tooltip::text("Anchored above").placement(TooltipPlacement::Top)),
            Button::new("Bottom")
                .build()
                .tooltip_with(Tooltip::text("Anchored below").placement(TooltipPlacement::Bottom)),
            Button::new("Left").build().tooltip_with(
                Tooltip::text("Anchored to the left").placement(TooltipPlacement::Left),
            ),
            Button::new("Right").build().tooltip_with(
                Tooltip::text("Anchored to the right").placement(TooltipPlacement::Right),
            ),
            Button::new("Mouse").build().tooltip_with(
                Tooltip::text("Follows the cursor").placement(TooltipPlacement::Mouse),
            ),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TooltipPlacement", app)
}
