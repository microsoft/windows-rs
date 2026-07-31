#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::DrawContext;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::DARK_SLATE_BLUE);

    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

    let cx = ctx.width / 2.0;
    let cy = ctx.height / 2.0;
    let size = cx.min(cy) * 0.6;

    ctx.fill_rounded_rect(
        &RoundedRect::uniform(Rect::new(cx - size, cy - size, cx, cy), 15.0),
        &brush,
    );

    brush.set_color(ColorF::WHITE);
    ctx.draw_rounded_rect(
        &RoundedRect::uniform(Rect::new(cx, cy, cx + size, cy + size), 25.0),
        &brush,
        3.0,
    );
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run("Shapes", draw)
}
