//! Minimal sample for the `Shape` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    vstack((
        text_block("Rectangle (fill + corner_radius)"),
        Shape::rectangle()
            .fill_rgb(40, 120, 200)
            .corner_radius(8.0)
            .width(160.0)
            .height(48.0),
        text_block("Ellipse (fill only)"),
        Shape::ellipse()
            .fill_rgb(220, 80, 120)
            .width(80.0)
            .height(80.0),
        text_block("Line (stroke + stroke_thickness)"),
        Shape::line(0.0, 0.0, 200.0, 0.0)
            .stroke(Color::rgb(80, 80, 80))
            .stroke_thickness(3.0),
        text_block("Rectangle outline (stroke, no fill)"),
        Shape::rectangle()
            .stroke(Color::rgb(40, 120, 200))
            .stroke_thickness(2.0)
            .width(160.0)
            .height(48.0),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
