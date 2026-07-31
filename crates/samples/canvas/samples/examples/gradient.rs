#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::DrawContext;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::BLACK);

    let margin = 40.0;
    let half = ctx.height / 2.0;

    let linear = ctx.create_linear_gradient(
        Vector2::new(margin, 0.0),
        Vector2::new(ctx.width - margin, 0.0),
        &[
            GradientStop::new(0.0, ColorF::CORNFLOWER_BLUE),
            GradientStop::new(1.0, ColorF::new(1.0, 0.5, 0.0, 1.0)),
        ],
    )?;

    ctx.fill_rounded_rect(
        &RoundedRect::uniform(
            Rect::new(margin, margin, ctx.width - margin, half - 10.0),
            20.0,
        ),
        &linear,
    );

    let cx = ctx.width / 2.0;
    let cy = half + (ctx.height - half) / 2.0;
    let r = (ctx.width - margin * 2.0).min(ctx.height - half - margin) / 2.0;

    let radial = ctx.create_radial_gradient(
        Vector2::new(cx, cy),
        r,
        r,
        &[
            GradientStop::new(0.0, ColorF::WHITE),
            GradientStop::new(0.6, ColorF::CORNFLOWER_BLUE),
            GradientStop::new(1.0, ColorF::BLACK),
        ],
    )?;

    ctx.fill_ellipse(&Ellipse::circle(Vector2::new(cx, cy), r), &radial);
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run("Gradient", draw)
}
