//! Standalone canvas sample — no windows-reactor dependency.
//!
//! Demonstrates `windows-canvas` with a plain Win32 window provided by
//! `windows-window`, driving a continuous render loop.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_window::*;

fn main() -> Result<()> {
    let window = Window::new("Canvas Standalone").size(800, 600).create()?;

    let device = GpuDevice::new()?;
    let mut chain = unsafe { device.create_swap_chain_for_hwnd(window.hwnd(), 800, 600)? };

    run_with(|| {
        let width = chain.width() as f32;
        let height = chain.height() as f32;
        let session = chain.begin_draw()?;
        session.clear(ColorF::DARK_SLATE_BLUE);
        let brush = session.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
        let r = width.min(height) * 0.3;

        session.fill_ellipse(
            &Ellipse::circle(Vector2::new(width / 2.0, height / 2.0), r),
            &brush,
        );

        brush.set_color(ColorF::WHITE);

        let format = TextFormat::new("Segoe UI", 24.0)?
            .with_alignment(TextAlignment::Center)
            .with_paragraph_alignment(ParagraphAlignment::Center);

        session.draw_text(
            "Hello from windows-canvas!",
            &format,
            &Rect::new(0.0, 0.0, width, height),
            &brush,
        );

        drop(session);
        chain.present()?;
        Ok(())
    })
}
