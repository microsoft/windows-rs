#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("Stack", || {
        StackPanel::new().spacing(12.0).children((
            TextBlock::new().text("Vertical orientation"),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    TextBlock::new().text("left"),
                    TextBlock::new().text("middle"),
                    TextBlock::new().text("right"),
                )),
            TextBlock::new().text("Back to the vertical stack"),
        ))
    })
    .unwrap();
}
