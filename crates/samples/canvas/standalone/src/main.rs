//! Standalone canvas sample — no windows-reactor dependency.
//!
//! Demonstrates `windows-canvas` with a plain Win32 HWND message loop.
//! The build.rs generates minimal window-management bindings via windows-bindgen.

#![windows_subsystem = "windows"]
#![allow(non_snake_case)]

#[allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
mod bindings;

use bindings::*;
use windows_canvas::*;
use windows_core::PCSTR;

fn main() -> windows_core::Result<()> {
    unsafe {
        let class_name = PCSTR(b"canvas_standalone\0".as_ptr());

        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            lpszClassName: class_name,
            ..Default::default()
        };
        RegisterClassA(&wc);

        let hwnd = CreateWindowExA(
            0,
            class_name,
            PCSTR(b"Canvas Standalone\0".as_ptr()),
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
        let mut chain = device.create_swap_chain_for_hwnd(hwnd, 800, 600)?;

        let mut msg = MSG::default();
        loop {
            while PeekMessageA(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE).as_bool() {
                if msg.message == 0x0012 {
                    // WM_QUIT
                    return Ok(());
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }

            // Render a frame.
            let w = chain.width() as f32;
            let h = chain.height() as f32;
            let session = chain.begin_draw()?;
            session.clear(Color::DARK_SLATE_BLUE);

            let brush = session.create_solid_brush(Color::CORNFLOWER_BLUE)?;
            let r = w.min(h) * 0.3;
            session.fill_ellipse(&Ellipse::circle(Vector2::new(w / 2.0, h / 2.0), r), &brush);

            brush.set_color(Color::WHITE);
            let format = TextFormat::new("Segoe UI", 24.0)?
                .with_alignment(TextAlignment::Center)
                .with_paragraph_alignment(ParagraphAlignment::Center);
            session.draw_text(
                "Hello from windows-canvas!",
                &format,
                &Rect::new(0.0, 0.0, w, h),
                &brush,
            );

            drop(session);
            chain.present()?;

            std::thread::sleep(std::time::Duration::from_millis(1));
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
