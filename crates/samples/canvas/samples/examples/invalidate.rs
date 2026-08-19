#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{CanvasDrawContext, Element, RenderCx, swap_chain_canvas_invalidated};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let points = cx.use_ref(Vec::<Vector2>::new);
    let invalidator = cx.use_canvas_invalidator();
    let draw_points = points.clone();
    let invalidate = invalidator.clone();

    swap_chain_canvas_invalidated(&invalidator, move |ctx| {
        draw_points.with(|points| draw(ctx, points)).unwrap()
    })
    .on_pointer_pressed(move |info| {
        points
            .with_mut(|points| points.push(Vector2::new(info.x, info.y)))
            .unwrap();
        invalidate.invalidate();
    })
    .build()
}

fn draw(ctx: &CanvasDrawContext<'_>, points: &[Vector2]) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(0x10, 0x12, 0x18));

    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

    for pair in points.windows(2) {
        ctx.draw_line(pair[0], pair[1], &brush, 2.0);
    }
    for &p in points {
        ctx.fill_ellipse(&Ellipse::circle(p, 4.0), &brush);
    }
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run_component("Invalidate", app)
}
