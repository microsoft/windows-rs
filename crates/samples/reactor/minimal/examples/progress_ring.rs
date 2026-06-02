//! Minimal sample for the `ProgressRing` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Determinate (40%)"),
        ProgressRing::new(40.0),
        text_block("Indeterminate (busy)"),
        ProgressRing::indeterminate(),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("Sample").render(app)
}
