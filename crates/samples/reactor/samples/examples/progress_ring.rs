#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("ProgressRing", || {
        StackPanel::new().spacing(8.0).children((
            "Determinate (40%)",
            ProgressRing::new()
                .minimum(0.0)
                .maximum(100.0)
                .value(40.0)
                .is_indeterminate(false)
                .is_active(true),
            "Indeterminate (busy)",
            ProgressRing::new().is_indeterminate(true).is_active(true),
        ))
    })
    .unwrap();
}
