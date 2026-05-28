//! Minimal sample for the `Canvas` element.
//!
//! `Canvas` positions children at `(canvas_left, canvas_top)` and has
//! no intrinsic size, so `width`/`height` are set on the `Canvas` itself.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    Canvas::new([
        // Outline rectangle so the canvas region is visible in both themes.
        Shape::rectangle()
            .stroke(Color::rgb(128, 128, 128))
            .stroke_thickness(1.0)
            .width(260.0)
            .height(120.0)
            .canvas_left(0.0)
            .canvas_top(0.0),
        Shape::rectangle()
            .fill_rgb(40, 120, 200)
            .width(80.0)
            .height(40.0)
            .canvas_left(20.0)
            .canvas_top(20.0),
        Shape::ellipse()
            .fill_rgb(220, 80, 120)
            .width(40.0)
            .height(40.0)
            .canvas_left(180.0)
            .canvas_top(40.0),
    ])
    .width(260.0)
    .height(120.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
