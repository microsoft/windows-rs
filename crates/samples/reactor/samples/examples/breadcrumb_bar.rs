//! Sample for the `BreadcrumbBar` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Multi-segment trail"),
        BreadcrumbBar::new(["Home", "Documents", "Projects", "windows-reactor-rs"]),
        text_block("Two-segment trail"),
        BreadcrumbBar::new(["Home", "Settings"]),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("BreadcrumbBar", app)
}
