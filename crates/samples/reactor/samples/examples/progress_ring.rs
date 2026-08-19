#![windows_subsystem = "windows"]

use windows_reactor::{Element, ProgressRing, RenderCx, TextBlock, vstack};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Determinate (40%)").build(),
            ProgressRing::new(40.0).build(),
            TextBlock::new("Indeterminate (busy)").build(),
            ProgressRing::indeterminate().build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ProgressRing", app)
}
