//! Geometry hit-testing with `windows-canvas`.
//!
//! A star is filled gold and turns green only when the pointer is inside its
//! *actual filled geometry* — not merely its bounding box (drawn as a faint
//! outline for contrast). Pointer position comes from the reactor element
//! callbacks; the inside/outside test uses `Path::fill_contains_point`.
//!
//! Pointer coordinates and the canvas drawing surface are both in
//! device-independent pixels, so the pointer position can be fed straight into
//! the geometry query with no conversion.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    // Latest pointer position in DIPs, shared with the draw closure. Updating a
    // `use_ref` does not trigger a re-render; the per-frame draw loop reads it.
    let pointer = cx.use_ref(None::<(f32, f32)>);

    // Cache the star geometry; rebuilding a `Path` every frame is wasteful.
    let star_cache = cx.use_ref(None::<(f32, f32, Path)>);

    let on_move = cx.use_callback((), {
        let pointer = pointer.clone();
        move |info: PointerEventInfo| pointer.set(Some((info.x as f32, info.y as f32)))
    });
    let on_exit = cx.use_callback((), {
        let pointer = pointer.clone();
        move |()| pointer.set(None)
    });

    animated_canvas({
        move |ctx| {
            ctx.clear(ColorF::DARK_SLATE_BLUE);

            let center_x = ctx.width / 2.0;
            let center_y = ctx.height / 2.0;
            let radius = center_x.min(center_y) * 0.8;

            // Rebuild the star only on resize or device loss, not every frame.
            let stale = ctx.device_changed()
                || match &*star_cache.borrow() {
                    Some((w, h, _)) => {
                        (*w - ctx.width).abs() > 0.5 || (*h - ctx.height).abs() > 0.5
                    }
                    None => true,
                };
            if stale && let Ok(path) = build_star(ctx.device(), center_x, center_y, radius) {
                star_cache.set(Some((ctx.width, ctx.height, path)));
            }

            let cache = star_cache.borrow();
            let Some((_, _, star)) = &*cache else {
                return;
            };

            // Bounding box: where a naive rectangle hit-test would report a hit.
            if let Ok(brush) = ctx.create_solid_brush(ColorF::new(1.0, 1.0, 1.0, 0.3)) {
                let b = star.compute_bounds();
                ctx.draw_rect(&Rect::new(b.left, b.top, b.right, b.bottom), &brush, 1.0);
            }

            let inside = (*pointer.borrow())
                .is_some_and(|(x, y)| star.fill_contains_point(Vector2::new(x, y)));

            let fill = if inside {
                ColorF::new(0.3, 0.85, 0.4, 1.0)
            } else {
                ColorF::new(1.0, 0.8, 0.0, 1.0)
            };
            if let Ok(brush) = ctx.create_solid_brush(fill) {
                ctx.fill_path(star, &brush);
            }

            if let Ok(format) = TextFormat::with_weight("Segoe UI", 18.0, FontWeight::BOLD)
                .map(|f| f.with_alignment(TextAlignment::Center))
                && let Ok(brush) = ctx.create_solid_brush(ColorF::WHITE)
            {
                let label = if inside {
                    "Inside the star"
                } else {
                    "Move the pointer over the star"
                };
                let rect = Rect::new(0.0, ctx.height - 36.0, ctx.width, ctx.height);
                ctx.draw_text(label, &format, &rect, &brush);
            }
        }
    })
    .on_pointer_moved(on_move)
    .on_pointer_exited(on_exit)
    .into()
}

fn build_star(device: &GpuDevice, cx: f32, cy: f32, r: f32) -> Result<Path> {
    let mut builder = PathBuilder::new(device)?.begin(star_point(cx, cy, r, 0));
    for i in 1..10 {
        let radius = if i % 2 == 0 { r } else { r * 0.5 };
        builder = builder.line_to(star_point(cx, cy, radius, i));
    }
    builder.close().build()
}

fn star_point(cx: f32, cy: f32, r: f32, i: u32) -> Vector2 {
    let angle = std::f32::consts::PI / 5.0 * i as f32 - std::f32::consts::FRAC_PI_2;
    Vector2::new(cx + r * angle.cos(), cy + r * angle.sin())
}

fn main() -> Result<()> {
    App::new()
        .title("Canvas hit-testing")
        .backdrop(Backdrop::Mica)
        .render(app)
}
