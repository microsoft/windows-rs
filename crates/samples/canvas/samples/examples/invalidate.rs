#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::Ellipse as CanvasEllipse;
use windows_canvas::*;
use windows_reactor::*;

struct Sample {
    points: Rc<RefCell<Vec<Vector2>>>,
    invalidator: Invalidator,
}

impl Component for Sample {
    type Input = ();
    type Message = PointerEventInfo;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            points: Rc::new(RefCell::new(Vec::new())),
            invalidator: Invalidator::new(),
        }
    }

    fn update(&mut self, info: PointerEventInfo, _context: &ComponentContext<Self>) {
        self.points
            .borrow_mut()
            .push(Vector2::new(info.x as f32, info.y as f32));
        self.invalidator.invalidate();
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Invalidate");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let points = Rc::clone(&self.points);
        Border::new()
            .on_pointer_pressed(context.callback(std::convert::identity))
            .content(canvas_invalidated(&self.invalidator, move |context| {
                draw(context, &points.borrow())
            }))
    }
}

fn draw(context: &DrawContext, points: &[Vector2]) -> Result<()> {
    context.clear(ColorF::from_rgb8(0x10, 0x12, 0x18));
    let brush = context.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    for pair in points.windows(2) {
        context.draw_line(pair[0], pair[1], &brush, 2.0);
    }
    for &point in points {
        context.fill_ellipse(&CanvasEllipse::circle(point, 4.0), &brush);
    }
    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
