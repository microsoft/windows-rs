#![windows_subsystem = "windows"]

use windows_canvas::*;

fn checkerboard() -> ([u8; 64 * 64 * 4], u32, u32) {
    const SIZE: u32 = 64;
    let mut pixels = [0u8; (SIZE * SIZE * 4) as usize];

    for y in 0..SIZE {
        for x in 0..SIZE {
            let i = ((y * SIZE + x) * 4) as usize;
            let dark = ((x / 8) + (y / 8)) % 2 == 0;
            let (b, g, r) = if dark { (60, 40, 20) } else { (230, 180, 90) };
            pixels[i] = b;
            pixels[i + 1] = g;
            pixels[i + 2] = r;
            pixels[i + 3] = 255;
        }
    }

    (pixels, SIZE, SIZE)
}

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::new(0.1, 0.1, 0.1, 1.0));

    let (pixels, w, h) = checkerboard();
    let bitmap = ctx.create_bitmap(&pixels, w, h)?;

    ctx.draw_bitmap(
        &bitmap,
        &Rect::from_xywh(20.0, 20.0, w as f32, h as f32),
        1.0,
    );

    ctx.draw_bitmap(
        &bitmap,
        &Rect::new(120.0, 20.0, ctx.width - 20.0, ctx.height - 20.0),
        1.0,
    );
    Ok(())
}

fn main() -> Result<()> {
    canvas_samples::run("Bitmap from bytes", draw)
}
