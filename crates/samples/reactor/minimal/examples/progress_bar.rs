//! Minimal sample for the `ProgressBar` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Determinate (65%)"),
        ProgressBar::new(65.0).range(0.0, 100.0),
        text_block("Determinate, custom range (3 / 5 steps)"),
        ProgressBar::new(3.0).range(0.0, 5.0),
        text_block("Indeterminate (busy)"),
        ProgressBar::indeterminate(),
    ))
    .spacing(8.0)
    .max_width(320.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
