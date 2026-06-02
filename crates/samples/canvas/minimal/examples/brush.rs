//! Demonstrates brush reuse and color changes.

#![windows_subsystem = "windows"]

use canvas_minimal::*;

fn draw(ctx: &DrawContext) {
    ctx.clear(Color::BLACK);

    let Ok(brush) = ctx.create_solid_brush(Color::RED) else {
        return;
    };

    let colors = [Color::RED, Color::GREEN, Color::BLUE, Color::WHITE];
    let stripe_width = ctx.width / colors.len() as f32;

    for (i, color) in colors.iter().enumerate() {
        brush.set_color(*color);
        ctx.fill_rect(
            &Rect::new(
                i as f32 * stripe_width,
                0.0,
                (i + 1) as f32 * stripe_width,
                ctx.height,
            ),
            &brush,
        );
    }
}

fn main() -> Result<()> {
    canvas_minimal::run("Brush", draw)
}
