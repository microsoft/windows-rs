#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, RenderCx, TextBlock, vstack};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        12.0,
        [
            Button::new("Hover me")
                .build()
                .tooltip(TextBlock::new("This is a tooltip").build()),
            TextBlock::new("Plain text also has a tooltip")
                .build()
                .tooltip(TextBlock::new("Tooltips can contain elements").build()),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Tooltip", app)
}
