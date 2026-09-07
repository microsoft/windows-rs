#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, ContentControl, PointerEventInfo, View, ViewContext,
    WindowBackdrop, WindowVisuals,
};

struct Sample {
    pointer: Rc<RefCell<Option<(f32, f32)>>>,
    star: Rc<RefCell<Option<(f32, f32, Path)>>>,
    invalidator: Invalidator,
}

enum Message {
    Move(PointerEventInfo),
    Exit,
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            pointer: Rc::new(RefCell::new(None)),
            star: Rc::new(RefCell::new(None)),
            invalidator: Invalidator::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Move(info) => {
                *self.pointer.borrow_mut() = Some((info.x as f32, info.y as f32));
            }
            Message::Exit => *self.pointer.borrow_mut() = None,
        }
        self.invalidator.invalidate();
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Canvas hit-testing");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let pointer = Rc::clone(&self.pointer);
        let star = Rc::clone(&self.star);
        Border::new()
            .on_pointer_moved(context.callback(Message::Move))
            .on_pointer_exited(context.callback(|_| Message::Exit))
            .content(canvas_invalidated(&self.invalidator, move |ctx| {
                ctx.clear(ColorF::DARK_SLATE_BLUE);
                let center_x = ctx.width / 2.0;
                let center_y = ctx.height / 2.0;
                let radius = center_x.min(center_y) * 0.8;
                let stale = ctx.device_changed()
                    || match &*star.borrow() {
                        Some((width, height, _)) => {
                            (*width - ctx.width).abs() > 0.5 || (*height - ctx.height).abs() > 0.5
                        }
                        None => true,
                    };
                if stale && let Ok(path) = build_star(ctx.device(), center_x, center_y, radius) {
                    *star.borrow_mut() = Some((ctx.width, ctx.height, path));
                }
                let cache = star.borrow();
                let Some((_, _, path)) = &*cache else {
                    return Ok(());
                };
                let brush = ctx.create_solid_brush(ColorF::new(1.0, 1.0, 1.0, 0.3))?;
                let bounds = path.compute_bounds();
                ctx.draw_rect(
                    &Rect::new(bounds.left, bounds.top, bounds.right, bounds.bottom),
                    &brush,
                    1.0,
                );
                let inside = pointer
                    .borrow()
                    .is_some_and(|(x, y)| path.fill_contains_point(Vector2::new(x, y)));
                let fill = if inside {
                    ColorF::new(0.3, 0.85, 0.4, 1.0)
                } else {
                    ColorF::new(1.0, 0.8, 0.0, 1.0)
                };
                ctx.fill_path(path, &ctx.create_solid_brush(fill)?);
                let format = TextFormat::with_weight("Segoe UI", 18.0, CanvasFontWeight::BOLD)?
                    .with_alignment(TextAlignment::Center);
                let label = if inside {
                    "Inside the star"
                } else {
                    "Move the pointer over the star"
                };
                ctx.draw_text(
                    label,
                    &format,
                    &Rect::new(0.0, ctx.height - 36.0, ctx.width, ctx.height),
                    &ctx.create_solid_brush(ColorF::WHITE)?,
                );
                Ok(())
            }))
    }
}

fn build_star(device: &GpuDevice, x: f32, y: f32, radius: f32) -> Result<Path> {
    let points = (0..10).map(|index| {
        let radius = if index % 2 == 0 { radius } else { radius * 0.5 };
        let angle = std::f32::consts::PI / 5.0 * index as f32 - std::f32::consts::FRAC_PI_2;
        Vector2::new(x + radius * angle.cos(), y + radius * angle.sin())
    });
    PathBuilder::new(device)?.polygon(points)
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}
