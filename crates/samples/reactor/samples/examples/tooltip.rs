//! Sample for the `tooltip` modifier.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        button("Hover me").tooltip("This is a tooltip"),
        text_block("Plain text also tips").tooltip("Even on TextBlock"),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Tooltip", app)
}
