#![windows_subsystem = "windows"]

use windows_canvas::*;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::BLACK);

    let format = TextFormat::new("Segoe UI", 32.0)?
        .with_alignment(TextAlignment::Center)
        .with_paragraph_alignment(ParagraphAlignment::Center);

    let brush = ctx.create_solid_brush(ColorF::WHITE)?;

    let rect = Rect::new(0.0, 0.0, ctx.width, ctx.height);
    ctx.draw_text("Hello, Canvas!", &format, &rect, &brush);
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run("Draw Text", draw)
}
