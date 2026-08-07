#![windows_subsystem = "windows"]

use windows_reactor::*;

const LONG_TEXT: &str =
    "A long line of text that should stop at the declared width instead of extending or wrapping.";

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("No trimming:"),
        text_block(LONG_TEXT).width(240.0),
        text_block("One line with character ellipsis:"),
        text_block(LONG_TEXT)
            .width(240.0)
            .max_lines(1)
            .text_trimming(TextTrimming::CharacterEllipsis),
        text_block("Two lines with word ellipsis:"),
        text_block(LONG_TEXT)
            .width(240.0)
            .wrap()
            .max_lines(2)
            .text_trimming(TextTrimming::WordEllipsis),
    ))
    .spacing(8.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("TextTrimming", app)
}
