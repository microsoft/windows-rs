//! Minimal canvas example: a filled ellipse.

#![windows_subsystem = "windows"]

use windows_canvas::*;

fn draw(ctx: &DrawContext) {
    ctx.clear(ColorF::DARK_SLATE_BLUE);
    let Ok(brush) = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE) else {
        return;
    };
    let r = ctx.width.min(ctx.height) * 0.3;
    ctx.fill_ellipse(
        &Ellipse::circle(Vector2::new(ctx.width / 2.0, ctx.height / 2.0), r),
        &brush,
    );
}

fn main() -> Result<()> {
    canvas_minimal::run("Hello Canvas", draw)
}
