//! Minimal sample for the `StackPanel` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("vstack — vertical orientation"),
        hstack((
            text_block("left"),
            text_block("middle"),
            text_block("right"),
        ))
        .spacing(8.0),
        text_block("…back to the vstack"),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
