//! Demonstrates rounded rectangles.

#![windows_subsystem = "windows"]

use canvas_minimal::*;

fn draw(ctx: &DrawContext) {
    ctx.clear(Color::DARK_SLATE_BLUE);

    let Ok(brush) = ctx.create_solid_brush(Color::CORNFLOWER_BLUE) else {
        return;
    };

    let cx = ctx.width / 2.0;
    let cy = ctx.height / 2.0;
    let size = cx.min(cy) * 0.6;

    // Filled rounded rect.
    ctx.fill_rounded_rect(
        &RoundedRect::uniform(Rect::new(cx - size, cy - size, cx, cy), 15.0),
        &brush,
    );

    // Outlined rounded rect.
    brush.set_color(Color::WHITE);
    ctx.draw_rounded_rect(
        &RoundedRect::uniform(Rect::new(cx, cy, cx + size, cy + size), 25.0),
        &brush,
        3.0,
    );
}

fn main() -> Result<()> {
    canvas_minimal::run("Shapes", draw)
}
