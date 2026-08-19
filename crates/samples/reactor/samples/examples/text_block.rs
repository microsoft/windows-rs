#![windows_subsystem = "windows"]

use windows_reactor::{Element, FontWeight, RenderCx, TextBlock, TextWrapping, vstack};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Plain text").build(),
            TextBlock::new("Larger text").font_size(20.0).build(),
            TextBlock::new("Bold + larger")
                .font_weight(FontWeight::BOLD)
                .font_size(28.0)
                .build(),
            TextBlock::new("Selectable text - try selecting this with your mouse")
                .text_selection_enabled(true)
                .build(),
            TextBlock::new(
                "Selectable + wrapped text that demonstrates both features working together on a \
             TextBlock element",
            )
            .text_selection_enabled(true)
            .text_wrapping(TextWrapping::Wrap)
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Text Block", app)
}
