#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use windows_canvas::*;
use windows_reactor::{
    Application, CanvasDrawContext, PointerEvent, Window, WindowBackdrop, component,
    run_reactor_winui_app, swap_chain_canvas_invalidated,
};

fn main() -> Result<()> {
    let pointer = Rc::new(Cell::new(None::<(f32, f32)>));
    let star_cache = Rc::new(RefCell::new(None::<(f32, f32, Path)>));
    let root = component(move |cx| {
        let invalidator = cx.use_canvas_invalidator();
        let draw_pointer = Rc::clone(&pointer);
        let draw_star_cache = Rc::clone(&star_cache);

        let canvas =
            swap_chain_canvas_invalidated(&invalidator, move |ctx: &CanvasDrawContext<'_>| {
                ctx.clear(ColorF::DARK_SLATE_BLUE);

                let center_x = ctx.width / 2.0;
                let center_y = ctx.height / 2.0;
                let radius = center_x.min(center_y) * 0.8;

                let stale = ctx.device_changed()
                    || match &*draw_star_cache.borrow() {
                        Some((w, h, _)) => {
                            (*w - ctx.width).abs() > 0.5 || (*h - ctx.height).abs() > 0.5
                        }
                        None => true,
                    };
                if stale && let Ok(path) = build_star(ctx.device(), center_x, center_y, radius) {
                    *draw_star_cache.borrow_mut() = Some((ctx.width, ctx.height, path));
                }

                let cache = draw_star_cache.borrow();
                let Some((_, _, star)) = &*cache else {
                    return Ok(());
                };

                let brush = ctx.create_solid_brush(ColorF::new(1.0, 1.0, 1.0, 0.3))?;
                let b = star.compute_bounds();
                ctx.draw_rect(&Rect::new(b.left, b.top, b.right, b.bottom), &brush, 1.0);

                let inside = draw_pointer
                    .get()
                    .is_some_and(|(x, y)| star.fill_contains_point(Vector2::new(x, y)));

                let fill = if inside {
                    ColorF::new(0.3, 0.85, 0.4, 1.0)
                } else {
                    ColorF::new(1.0, 0.8, 0.0, 1.0)
                };
                let brush = ctx.create_solid_brush(fill)?;
                ctx.fill_path(star, &brush);

                let format = TextFormat::with_weight("Segoe UI", 18.0, FontWeight::BOLD)?
                    .with_alignment(TextAlignment::Center);
                let brush = ctx.create_solid_brush(ColorF::WHITE)?;
                let label = if inside {
                    "Inside the star"
                } else {
                    "Move the pointer over the star"
                };
                let rect = Rect::new(0.0, ctx.height - 36.0, ctx.width, ctx.height);
                ctx.draw_text(label, &format, &rect, &brush);
                Ok(())
            });
        let moved_pointer = Rc::clone(&pointer);
        let moved_invalidator = invalidator.clone();
        let exited_pointer = Rc::clone(&pointer);
        let exited_invalidator = invalidator;
        let content = canvas
            .on_pointer_moved(move |event: PointerEvent| {
                moved_pointer.set(Some((event.x, event.y)));
                moved_invalidator.invalidate();
            })
            .on_pointer_exited(move |_event: PointerEvent| {
                exited_pointer.set(None);
                exited_invalidator.invalidate();
            })
            .build();
        Application::new([Window::new("Canvas hit-testing", content, || {})
            .backdrop(WindowBackdrop::Mica)
            .build()
            .key(0)])
        .build()
    });
    run_reactor_winui_app(root)
}

fn build_star(device: &GpuDevice, cx: f32, cy: f32, r: f32) -> Result<Path> {
    let points = (0..10).map(|i| {
        let radius = if i % 2 == 0 { r } else { r * 0.5 };
        star_point(cx, cy, radius, i)
    });
    PathBuilder::new(device)?.polygon(points)
}

fn star_point(cx: f32, cy: f32, r: f32, i: u32) -> Vector2 {
    let angle = std::f32::consts::PI / 5.0 * i as f32 - std::f32::consts::FRAC_PI_2;
    Vector2::new(cx + r * angle.cos(), cy + r * angle.sin())
}
