//! Demonstrates loading and drawing a bitmap image.
//!
//! Bitmaps are GPU resources and must be reloaded after device-lost.
//! For simplicity this example loads per-frame; a real app would cache
//! the bitmap and reload only when the device is recreated.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::DrawContext;

fn draw(ctx: &DrawContext) {
    ctx.clear(ColorF::new(0.1, 0.1, 0.1, 1.0));

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("sample.png");

    let Ok(bitmap) = ctx.load_bitmap(&path) else {
        return;
    };

    // Draw at original size in top-left.
    let w = bitmap.width();
    let h = bitmap.height();
    ctx.draw_bitmap(&bitmap, &Rect::new(20.0, 20.0, 20.0 + w, 20.0 + h), 1.0);

    // Draw scaled to fill a larger area.
    ctx.draw_bitmap(
        &bitmap,
        &Rect::new(120.0, 20.0, ctx.width - 20.0, ctx.height - 20.0),
        0.8,
    );
}

fn main() -> Result<()> {
    canvas_samples::run("Bitmap", draw)
}
