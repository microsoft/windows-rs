//! Text rendering with `TextFormat` and `draw_text`.

#![windows_subsystem = "windows"]

use canvas_minimal::*;

fn draw(ctx: &DrawContext) {
    ctx.clear(Color::BLACK);

    let Ok(format) = TextFormat::new("Segoe UI", 32.0)
        .map(|f| f.with_alignment(TextAlignment::Center))
        .map(|f| f.with_paragraph_alignment(ParagraphAlignment::Center))
    else {
        return;
    };

    let Ok(brush) = ctx.create_solid_brush(Color::WHITE) else {
        return;
    };

    let rect = Rect::new(0.0, 0.0, ctx.width, ctx.height);
    ctx.draw_text("Hello, Canvas!", &format, &rect, &brush);
}

fn main() -> Result<()> {
    canvas_minimal::run("Text", draw)
}
