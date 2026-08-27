use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("Canvas", || {
        Canvas::new().width(260.0).height(120.0).children((
            Rectangle::new()
                .stroke(Color::rgb(128, 128, 128))
                .stroke_thickness(1.0)
                .width(260.0)
                .height(120.0)
                .canvas_left(0.0)
                .canvas_top(0.0),
            Rectangle::new()
                .fill(Color::rgb(40, 120, 200))
                .width(80.0)
                .height(40.0)
                .canvas_left(20.0)
                .canvas_top(20.0),
            Ellipse::new()
                .fill(Color::rgb(220, 80, 120))
                .width(40.0)
                .height(40.0)
                .canvas_left(180.0)
                .canvas_top(40.0),
        ))
    })
    .unwrap();
}
