//! Sample demonstrating type-ramp factory functions.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        title("Title — 28px Semibold"),
        subtitle("Subtitle — 20px Semibold"),
        body_large("BodyLarge — 18px Normal"),
        body_strong("BodyStrong — 14px Semibold"),
        body("Body — 14px Normal"),
        caption("Caption — 12px Normal"),
        text_block("Custom weight").font_weight(300),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("TypeRamp", app)
}
