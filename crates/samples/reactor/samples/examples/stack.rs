#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("Stack", || {
        StackPanel::new().spacing(12.0).children((
            "Vertical orientation",
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children(("left", "middle", "right")),
            "Back to the vertical stack",
        ))
    })
    .unwrap();
}
