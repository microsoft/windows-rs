#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{CanvasDrawContext, Element, RenderCx, animated_canvas};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let frame = cx.use_ref(|| 0_u64);
    animated_canvas(move |ctx| {
        let frame = frame
            .with_mut(|frame| {
                *frame += 1;
                *frame
            })
            .unwrap();
        draw(ctx, frame as f32 * 0.01)
    })
    .build()
}

fn draw(ctx: &CanvasDrawContext<'_>, t: f32) -> Result<()> {
    ctx.clear(ColorF::BLACK);

    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

    let cx = ctx.width / 2.0;
    let cy = ctx.height / 2.0;
    let size = cx.min(cy) * 0.25;

    for i in 0..6 {
        let angle = t + i as f32 * std::f32::consts::PI / 3.0;
        let (sin, cos) = (angle.sin(), angle.cos());
        let transform = Matrix3x2 {
            m11: cos,
            m12: sin,
            m21: -sin,
            m22: cos,
            m31: cx * (1.0 - cos) + cy * sin,
            m32: cy * (1.0 - cos) - cx * sin,
        };

        let frac = i as f32 / 6.0;
        brush.set_color(ColorF::new(0.3 + frac * 0.7, 0.5, 1.0 - frac * 0.5, 0.8));

        ctx.with_transform(&transform, || {
            ctx.fill_rect(
                &Rect::new(cx - size, cy - size, cx + size, cy + size),
                &brush,
            );
        });
    }
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run_component("Transform", app)
}
