//! Standalone WebView2 sample — no `windows` crate dependency.
//!
//! Hosts a WebView2 browser in a plain Win32 HWND message loop using the safe
//! `windows-webview` wrapper. The build.rs generates the minimal
//! window-management bindings via windows-bindgen.
//!
//! Requires the Microsoft Edge WebView2 runtime to be installed.

#![windows_subsystem = "windows"]

#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;

use bindings::*;
use std::cell::RefCell;
use windows_core::*;
use windows_webview::Controller;

thread_local! {
    static CONTROLLER: RefCell<Option<Controller>> = const { RefCell::new(None) };
}

fn resize(controller: &Controller, window: HWND) {
    let mut rect = RECT::default();
    unsafe {
        if GetClientRect(window, &mut rect).as_bool() {
            let _ = controller.set_bounds(rect.left, rect.top, rect.right, rect.bottom);
        }
    }
}

unsafe extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_SIZE => {
            CONTROLLER.with(|controller| {
                if let Some(controller) = controller.borrow().as_ref() {
                    resize(controller, window);
                }
            });
            0
        }
        WM_DESTROY => {
            CONTROLLER.with(|controller| controller.borrow_mut().take());
            unsafe { PostQuitMessage(0) };
            0
        }
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    }
}

fn main() -> Result<()> {
    unsafe {
        _ = CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32);

        let class_name = s!("webview_minimal");

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
            s!("WebView2 - windows-rs"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1024,
            768,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );

        windows_webview::create_environment(move |environment| {
            let Ok(environment) = environment else {
                return;
            };

            let _ = environment.create_controller(window, move |controller| {
                let Ok(controller) = controller else {
                    return;
                };

                resize(&controller, window);

                if let Ok(webview) = controller.webview() {
                    let _ = webview.navigate("https://github.com/microsoft/windows-rs");
                }

                CONTROLLER.with(|slot| *slot.borrow_mut() = Some(controller));
            });
        })?;

        let mut message = MSG::default();
        while GetMessageA(&mut message, std::ptr::null_mut(), 0, 0).as_bool() {
            DispatchMessageA(&message);
        }
    }

    Ok(())
}
