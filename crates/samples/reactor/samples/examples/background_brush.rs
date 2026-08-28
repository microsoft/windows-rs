#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("BackgroundBrush", || {
        Grid::new()
            .background(Color::rgb(255, 0, 0))
            .children(["Sample"])
    })
    .unwrap();
}
