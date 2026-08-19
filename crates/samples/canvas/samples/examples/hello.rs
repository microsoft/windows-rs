#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::CanvasDrawContext;

fn draw(ctx: &CanvasDrawContext<'_>) -> Result<()> {
    ctx.clear(ColorF::DARK_SLATE_BLUE);
    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    let r = ctx.width.min(ctx.height) * 0.3;
    ctx.fill_ellipse(
        &Ellipse::circle(Vector2::new(ctx.width / 2.0, ctx.height / 2.0), r),
        &brush,
    );
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run("Hello Canvas", draw)
}
