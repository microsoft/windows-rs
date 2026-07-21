//! Off-screen rendering with CPU pixel readback.
//!
//! Not every drawing surface is destined for the screen: thumbnails,
//! tray/notification icons, image export, and automated tests all need the
//! *finished pixels* on the CPU instead. `windows-canvas` provides this with a
//! [`RenderTarget`] — the equivalent of Win2D's `CanvasRenderTarget`. Create one
//! from a shared [`GpuDevice`], draw into it like any other surface, then call
//! [`RenderTarget::read_pixels`] to copy the result back as tightly packed BGRA.
//!
//! This sample renders a filled disc on a dark background entirely off-screen,
//! reads the pixels back, and prints an ASCII preview to the console — so you can
//! see the rendered output without opening a window or writing a file.

use windows_canvas::*;
use windows_core::Result;
use windows_numerics::Vector2;

const WIDTH: u32 = 48;
const HEIGHT: u32 = 24;

fn main() -> Result<()> {
    // One device backs the off-screen target; it falls back to WARP when no GPU
    // is available, so this runs on any machine (VMs, RDP, CI).
    let device = GpuDevice::new_or_warp()?;
    let target = device.create_render_target(WIDTH, HEIGHT)?;

    // Draw off-screen: a white disc on a dark background.
    target.draw(|session| {
        session.clear(ColorF::DARK_SLATE_BLUE);
        let brush = session.create_solid_brush(ColorF::WHITE).unwrap();
        let center = Vector2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
        session.fill_ellipse(&Ellipse::circle(center, HEIGHT as f32 * 0.4), &brush);
    })?;

    // Copy the finished pixels back to CPU memory as tightly packed BGRA.
    let pixels = target.read_pixels()?;
    println!(
        "Rendered {WIDTH}x{HEIGHT} off-screen, read back {} bytes of BGRA:\n",
        pixels.len()
    );
    print_ascii(&pixels);
    Ok(())
}

/// Prints an ASCII preview by mapping each pixel's brightness to a ramp glyph.
fn print_ascii(pixels: &[u8]) {
    const RAMP: &[u8] = b" .:-=+*#%@";
    for y in 0..HEIGHT as usize {
        let mut line = String::with_capacity(WIDTH as usize);
        for x in 0..WIDTH as usize {
            let i = (y * WIDTH as usize + x) * 4;
            // BGRA order; approximate luminance from the three color channels.
            let luma = (pixels[i] as u32 + pixels[i + 1] as u32 + pixels[i + 2] as u32) / 3;
            let idx = (luma as usize * (RAMP.len() - 1)) / 255;
            line.push(RAMP[idx] as char);
        }
        println!("{line}");
    }
}
