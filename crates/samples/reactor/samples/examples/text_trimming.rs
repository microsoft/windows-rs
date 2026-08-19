#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, RenderCx, StackPanel, TextBlock, TextTrimming, TextWrapping, Thickness,
};

const LONG_TEXT: &str =
    "A long line of text that should stop at the declared width instead of extending or wrapping.";

fn app(_cx: &mut RenderCx<'_>) -> Element {
    StackPanel::new([
        TextBlock::new("No trimming:").build(),
        TextBlock::new(LONG_TEXT).width(240.0).build(),
        TextBlock::new("Character ellipsis:").build(),
        TextBlock::new(LONG_TEXT)
            .width(240.0)
            .text_trimming(TextTrimming::CharacterEllipsis)
            .build(),
        TextBlock::new("Word ellipsis:").build(),
        TextBlock::new(LONG_TEXT)
            .width(240.0)
            .text_trimming(TextTrimming::WordEllipsis)
            .build(),
        TextBlock::new("Wrapped text is not trimmed:").build(),
        TextBlock::new(LONG_TEXT)
            .width(240.0)
            .text_wrapping(TextWrapping::Wrap)
            .build(),
    ])
    .spacing(8.0)
    .margin(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Text Trimming", app)
}
