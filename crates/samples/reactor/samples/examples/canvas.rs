#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Canvas, CanvasChild, Color, Element, RenderCx, TextBlock, Thickness,
};

fn block(label: &str, color: Color, width: f64, height: f64) -> Element {
    Border::new(TextBlock::new(label).build())
        .background(color)
        .padding(Thickness::uniform(8.0))
        .width(width)
        .height(height)
        .build()
}

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    Canvas::new([
        CanvasChild::new(block("Background", Color::rgb(210, 220, 235), 260.0, 120.0))
            .left(0.0)
            .top(0.0)
            .z_index(0),
        CanvasChild::new(block("Blue", Color::rgb(40, 120, 200), 100.0, 52.0))
            .left(20.0)
            .top(20.0)
            .z_index(1),
        CanvasChild::new(block("Rose", Color::rgb(220, 80, 120), 100.0, 52.0))
            .left(90.0)
            .top(48.0)
            .z_index(2),
    ])
    .width(260.0)
    .height(120.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Canvas", app)
}
