#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("InfoBadge", || {
        StackPanel::new().spacing(8.0).children((
            "Dot (attention indicator)",
            InfoBadge::new(),
            "Numeric (small / large counts)",
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(12.0)
                .children((
                    InfoBadge::new().value(1),
                    InfoBadge::new().value(9),
                    InfoBadge::new().value(42),
                    InfoBadge::new().value(999),
                )),
        ))
    })
    .unwrap();
}
