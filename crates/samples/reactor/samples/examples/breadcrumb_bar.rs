#![windows_subsystem = "windows"]

use windows_reactor::{BreadcrumbBar, Element, RenderCx, TextBlock, vstack};

pub fn app(_: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Multi-segment trail").build(),
            BreadcrumbBar::new([
                (1, "Home"),
                (2, "Documents"),
                (3, "Projects"),
                (4, "windows-reactor-rs"),
            ])
            .build(),
            TextBlock::new("Two-segment trail").build(),
            BreadcrumbBar::new([(1, "Home"), (2, "Settings")]).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("BreadcrumbBar", app)
}
