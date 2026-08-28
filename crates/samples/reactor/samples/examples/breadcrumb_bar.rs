#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("BreadcrumbBar", || {
        StackPanel::new().spacing(8.0).children((
            "Multi-segment trail",
            BreadcrumbBar::new().items_source([
                "Home",
                "Documents",
                "Projects",
                "windows-reactor-rs",
            ]),
            "Two-segment trail",
            BreadcrumbBar::new().items_source(["Home", "Settings"]),
        ))
    })
    .unwrap();
}
