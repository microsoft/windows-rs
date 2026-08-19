#![windows_subsystem = "windows"]

use windows_reactor::{Element, RelativePanel, RelativePanelChild, RenderCx, TextBlock};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    RelativePanel::new([
        RelativePanelChild::new(TextBlock::new("Top Left").build())
            .align_left(true)
            .align_top(true),
        RelativePanelChild::new(TextBlock::new("Top Right").build())
            .align_right(true)
            .align_top(true),
        RelativePanelChild::new(TextBlock::new("Bottom Left").build())
            .align_left(true)
            .align_bottom(true),
        RelativePanelChild::new(TextBlock::new("Bottom Right").build())
            .align_right(true)
            .align_bottom(true),
        RelativePanelChild::new(TextBlock::new("Center").build())
            .align_horizontal_center(true)
            .align_vertical_center(true),
    ])
    .width(300.0)
    .height(200.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("RelativePanel", app)
}
