#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, FontWeight, RenderCx, TextBlock, vstack};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    let rich = vstack(
        4.0,
        [
            TextBlock::new("Action: Save")
                .font_weight(FontWeight::BOLD)
                .build(),
            TextBlock::new("Writes the current document to disk.").build(),
        ],
    );

    vstack(
        8.0,
        [
            Button::new("Save").build().tooltip(rich),
            Button::new("Open")
                .build()
                .tooltip(TextBlock::new("Opens a document").build()),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TooltipRich", app)
}
