//! Standalone canvas sample — no windows-reactor dependency.
//!
//! Demonstrates `windows-canvas` with a plain Win32 HWND message loop.
//! The build.rs generates minimal window-management bindings via windows-bindgen.

#![windows_subsystem = "windows"]

#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;

use bindings::*;
use windows_canvas::*;
use windows_core::*;

fn main() -> Result<()> {
    unsafe {
        let class_name = s!("canvas_standalone");

        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            lpszClassName: class_name,
            ..Default::default()
        };

        RegisterClassA(&wc);

        let window = CreateWindowExA(
            0,
            class_name,
            s!("Canvas Standalone"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        let device = GpuDevice::new()?;
        let mut chain = device.create_swap_chain_for_hwnd(window, 800, 600)?;
        let mut msg = MSG::default();

        loop {
            while PeekMessageA(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    return Ok(());
                }

                _ = TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }

            let width = chain.width() as f32;
            let height = chain.height() as f32;
            let session = chain.begin_draw()?;
            session.clear(Color::DARK_SLATE_BLUE);
            let brush = session.create_solid_brush(Color::CORNFLOWER_BLUE)?;
            let r = width.min(height) * 0.3;

            session.fill_ellipse(
                &Ellipse::circle(Vector2::new(width / 2.0, height / 2.0), r),
                &brush,
            );

            brush.set_color(Color::WHITE);

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
        }
    }
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            0
        }
        _ => unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) },
    }
}
