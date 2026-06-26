//! Demonstrates drawing lines with varying widths.

#![windows_subsystem = "windows"]

use windows_canvas::*;

fn draw(ctx: &DrawContext) {
    ctx.clear(ColorF::BLACK);

    let Ok(brush) = ctx.create_solid_brush(ColorF::WHITE) else {
        return;
    };

    let margin = 30.0;
    let count = 8;
    let spacing = (ctx.height - margin * 2.0) / count as f32;

    for i in 0..count {
        let y = margin + i as f32 * spacing + spacing / 2.0;
        let width = (i + 1) as f32;

        let t = i as f32 / (count - 1) as f32;
        brush.set_color(ColorF::new(1.0 - t * 0.7, 1.0 - t * 0.5, 1.0, 1.0));

        ctx.draw_line(
            Vector2::new(margin, y),
            Vector2::new(ctx.width - margin, y),
            &brush,
            width,
        );
    }
}

fn main() -> Result<()> {
    canvas_samples::run("Lines", draw)
}
