//! Demonstrates brush reuse and ColorF changes.

#![windows_subsystem = "windows"]

use windows_canvas::*;

fn draw(ctx: &DrawContext) {
    ctx.clear(ColorF::BLACK);

    let Ok(brush) = ctx.create_solid_brush(ColorF::RED) else {
        return;
    };

    let colors = [ColorF::RED, ColorF::GREEN, ColorF::BLUE, ColorF::WHITE];
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
    canvas_samples::run("Brush", draw)
}
