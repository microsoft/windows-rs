#![windows_subsystem = "windows"]

use windows_reactor::{Element, ProgressBar, RenderCx, StackPanel, TextBlock};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    StackPanel::new([
        TextBlock::new("Determinate (65%)").build(),
        ProgressBar::new(65.0).range(0.0, 100.0).build(),
        TextBlock::new("Determinate, custom range (3 / 5 steps)").build(),
        ProgressBar::new(3.0).range(0.0, 5.0).build(),
        TextBlock::new("Indeterminate (busy)").build(),
        ProgressBar::indeterminate().build(),
    ])
    .spacing(8.0)
    .max_width(320.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ProgressBar", app)
}
