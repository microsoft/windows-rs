#![windows_subsystem = "windows"]

use windows_reactor::{Color, Element, RenderCx, Shape, TextBlock, vstack};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Rectangle (fill + corner radius)").build(),
            Shape::rectangle()
                .fill_rgb(40, 120, 200)
                .corner_radius(8.0)
                .width(160.0)
                .height(48.0)
                .build(),
            TextBlock::new("Ellipse (fill only)").build(),
            Shape::ellipse()
                .fill_rgb(220, 80, 120)
                .width(80.0)
                .height(80.0)
                .build(),
            TextBlock::new("Line (stroke + stroke thickness)").build(),
            Shape::line(0.0, 0.0, 200.0, 0.0)
                .stroke(Color::rgb(80, 80, 80))
                .stroke_thickness(3.0)
                .build(),
            TextBlock::new("Rectangle outline (stroke, no fill)").build(),
            Shape::rectangle()
                .stroke(Color::rgb(40, 120, 200))
                .stroke_thickness(2.0)
                .width(160.0)
                .height(48.0)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Shape", app)
}
