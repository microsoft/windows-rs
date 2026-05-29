//! Minimal sample for tooltip `placement`.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        button("Top")
            .tooltip_with(Tooltip::text("Anchored above").placement(TooltipPlacement::Top)),
        button("Bottom")
            .tooltip_with(Tooltip::text("Anchored below").placement(TooltipPlacement::Bottom)),
        button("Left")
            .tooltip_with(Tooltip::text("Anchored to the left").placement(TooltipPlacement::Left)),
        button("Right").tooltip_with(
            Tooltip::text("Anchored to the right").placement(TooltipPlacement::Right),
        ),
        button("Mouse")
            .tooltip_with(Tooltip::text("Follows the cursor").placement(TooltipPlacement::Mouse)),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("tooltip_placement sample").render(app)
}
