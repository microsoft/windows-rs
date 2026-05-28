//! Minimal sample for the `ProgressRing` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    vstack((
        text_block("Determinate (40%)"),
        ProgressRing::new(40.0),
        text_block("Indeterminate (busy)"),
        ProgressRing::indeterminate(),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
