//! Measuring and drawing with `TextLayout` on a demand-driven canvas.
//!
//! The `canvas` harness repaints only on the first layout and on resize or DPI
//! change, so the layout is shaped and measured just when the size changes, never
//! per frame. The sample fits a paragraph to the window, outlines the layout box
//! and the measured text bounds, and reports the line count.
//!
//! Compare with the Win2D "Text layouts" demo and with `draw_text`, which draws a
//! single centered string without measuring it.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::DrawContext;

const TEXT: &str = "TextLayout shapes this paragraph and measures it, then draws it without \
re-shaping. Resize the window and the text reflows to fit the box - redrawing only when the size \
changes, not every frame.";

const MARGIN: f32 = 40.0;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));

    let box_rect = Rect::new(MARGIN, MARGIN, ctx.width - MARGIN, ctx.height - MARGIN);

    let format = TextFormat::new("Segoe UI", 28.0)?.with_word_wrapping(WordWrapping::Wrap);
    let layout = TextLayout::new(TEXT, &format, box_rect.width(), box_rect.height())?;
    let metrics = layout.metrics();

    let outline = ctx.create_solid_brush(ColorF::from_rgb8(60, 70, 90))?;
    let accent = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    let white = ctx.create_solid_brush(ColorF::WHITE)?;

    // The box we asked the text to fill, the measured bounds of the inked text,
    // and the paragraph itself.
    ctx.draw_rect(&box_rect, &outline, 1.0);
    ctx.draw_rect(&metrics.bounds().offset(MARGIN, MARGIN), &accent, 1.5);
    ctx.draw_text_layout(Vector2::new(box_rect.left, box_rect.top), &layout, &white);

    // A readout of the measured line count and size.
    let label_format = TextFormat::new("Consolas", 16.0)?;
    let label = format!(
        "{} lines  -  {:.0} x {:.0} px",
        metrics.line_count, metrics.width, metrics.height
    );
    let label_rect = Rect::new(MARGIN, 8.0, ctx.width - MARGIN, MARGIN);
    ctx.draw_text(&label, &label_format, &label_rect, &accent);
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run("Text Layout", draw)
}
