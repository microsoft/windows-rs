#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("ProgressBar", || {
        StackPanel::new().spacing(8.0).children((
            "Determinate (65%)",
            ProgressBar::new().minimum(0.0).maximum(100.0).value(65.0),
            "Determinate, custom range (3 / 5 steps)",
            ProgressBar::new().minimum(0.0).maximum(5.0).value(3.0),
            "Indeterminate (busy)",
            ProgressBar::new().is_indeterminate(true),
        ))
    })
    .unwrap();
}
