use windows_reactor::*;

struct Sample;

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Shape");
        StackPanel::new().spacing(8.0).children((
            "Rectangle (fill + corner radius)",
            Rectangle::new()
                .fill(Color::rgb(40, 120, 200))
                .radius_x(8.0)
                .radius_y(8.0)
                .width(160.0)
                .height(48.0),
            "Ellipse (fill only)",
            Ellipse::new()
                .fill(Color::rgb(220, 80, 120))
                .width(80.0)
                .height(80.0),
            "Line (stroke + stroke thickness)",
            Line::new()
                .x1(0.0)
                .y1(0.0)
                .x2(200.0)
                .y2(0.0)
                .stroke(Color::rgb(80, 80, 80))
                .stroke_thickness(3.0),
            "Rectangle outline (stroke, no fill)",
            Rectangle::new()
                .stroke(Color::rgb(40, 120, 200))
                .stroke_thickness(2.0)
                .width(160.0)
                .height(48.0),
        ))
    }
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
