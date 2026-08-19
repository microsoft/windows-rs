#![windows_subsystem = "windows"]

use windows_reactor::{Border, Color, Element, RenderCx, TextBlock, Thickness};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    Border::new(TextBlock::new("Sample").build())
        .background(Color::rgb(220, 45, 45))
        .padding(Thickness::uniform(24.0))
        .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Background Brush", app)
}
