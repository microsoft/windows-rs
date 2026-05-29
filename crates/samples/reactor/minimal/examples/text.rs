//! Minimal sample for the `TextBlock` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Plain text"),
        text_block("Larger text").font_size(20.0),
        text_block("Bold + larger").bold().font_size(28.0),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
