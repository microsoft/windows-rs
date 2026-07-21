//! Demonstrates uploading CPU-generated pixels into a GPU bitmap.
//!
//! This is the path a real app uses to draw images it produces or decodes
//! itself — for example a cache of shell icons. The pixels are 32-bit
//! premultiplied BGRA, laid out row by row with no padding, and handed to
//! `create_bitmap`. The bitmap is a GPU resource, so it is rebuilt each draw
//! here for simplicity; a real app would cache it and rebuild only after
//! device-lost.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::DrawContext;

/// Builds a 64x64 checkerboard in premultiplied BGRA.
fn checkerboard() -> ([u8; 64 * 64 * 4], u32, u32) {
    const SIZE: u32 = 64;
    let mut pixels = [0u8; (SIZE * SIZE * 4) as usize];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let i = ((y * SIZE + x) * 4) as usize;
            let dark = ((x / 8) + (y / 8)) % 2 == 0;
            let (b, g, r) = if dark { (60, 40, 20) } else { (230, 180, 90) };
            // Opaque, so premultiplied values equal the raw channels.
            pixels[i] = b;
            pixels[i + 1] = g;
            pixels[i + 2] = r;
            pixels[i + 3] = 255;
        }
    }

    (pixels, SIZE, SIZE)
}

fn draw(ctx: &DrawContext) {
    ctx.clear(ColorF::new(0.1, 0.1, 0.1, 1.0));

    let (pixels, w, h) = checkerboard();
    let Ok(bitmap) = ctx.create_bitmap(&pixels, w, h) else {
        return;
    };

    // Original size, top-left.
    ctx.draw_bitmap(
        &bitmap,
        &Rect::from_xywh(20.0, 20.0, w as f32, h as f32),
        1.0,
    );

    // Scaled up to fill the rest of the surface.
    ctx.draw_bitmap(
        &bitmap,
        &Rect::new(120.0, 20.0, ctx.width - 20.0, ctx.height - 20.0),
        1.0,
    );
}

fn main() -> Result<()> {
    canvas_samples::run("Bitmap from bytes", draw)
}
