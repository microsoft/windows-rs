use windows_canvas::*;
use windows_core::Result;
use windows_numerics::Vector2;

const WIDTH: u32 = 48;
const HEIGHT: u32 = 24;

fn main() -> Result<()> {
    let device = GpuDevice::new_or_warp()?;
    let target = device.create_render_target(WIDTH, HEIGHT)?;

    target.draw(|session| {
        session.clear(ColorF::DARK_SLATE_BLUE);
        let brush = session.create_solid_brush(ColorF::WHITE)?;
        let center = Vector2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
        session.fill_ellipse(&Ellipse::circle(center, HEIGHT as f32 * 0.4), &brush);
        Ok(())
    })?;

    let pixels = target.read_pixels()?;
    println!(
        "Rendered {WIDTH}x{HEIGHT} off-screen, read back {} bytes of BGRA:\n",
        pixels.len()
    );
    print_ascii(&pixels);
    Ok(())
}

fn print_ascii(pixels: &[u8]) {
    const RAMP: &[u8] = b" .:-=+*#%@";
    for y in 0..HEIGHT as usize {
        let mut line = String::with_capacity(WIDTH as usize);
        for x in 0..WIDTH as usize {
            let i = (y * WIDTH as usize + x) * 4;
            let luma = (pixels[i] as u32 + pixels[i + 1] as u32 + pixels[i + 2] as u32) / 3;
            let idx = (luma as usize * (RAMP.len() - 1)) / 255;
            line.push(RAMP[idx] as char);
        }
        println!("{line}");
    }
}
