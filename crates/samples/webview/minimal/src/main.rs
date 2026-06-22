//! Standalone WebView2 sample — no `windows` crate dependency.
//!
//! Hosts a WebView2 browser in a plain Win32 HWND message loop using the safe
//! `windows-webview` wrapper. The build.rs generates the minimal
//! window-management bindings via windows-bindgen.
//!
//! Requires the Microsoft Edge WebView2 runtime to be installed.

//#![windows_subsystem = "windows"]

#[expect(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;

use bindings::*;
use std::cell::RefCell;
use windows_core::*;
use windows_webview::{
    Controller, Environment, EnvironmentOptions, EventRegistration, PermissionState,
};

thread_local! {
    static CONTROLLER: RefCell<Option<Controller>> = const { RefCell::new(None) };
    static REGISTRATIONS: RefCell<Vec<EventRegistration>> = const { RefCell::new(Vec::new()) };
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
            REGISTRATIONS.with(|registrations| registrations.borrow_mut().clear());
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

        let options = EnvironmentOptions::new()
            .additional_browser_arguments("--disable-features=msSmartScreenProtection");
        let environment = Environment::with_options(&options)?;
        let controller = environment.create_controller(window)?;

        resize(&controller, window);
        let webview = controller.webview()?;

        let settings = webview.settings()?;
        settings.set_dev_tools_enabled(true)?;
        settings.set_status_bar_enabled(false)?;

        webview.add_script_to_execute_on_document_created(
            "window.chrome.webview.postMessage('document created: ' + location.href);",
        )?;

        let message_registration = webview.on_web_message_received(|args| {
            println!("page sent: {}", args.web_message_as_json());
        })?;

        let starting_registration = webview.on_navigation_starting(|args| {
            println!(
                "navigation {} starting: {} (user initiated = {})",
                args.navigation_id(),
                args.uri(),
                args.is_user_initiated()
            );
        })?;

        let post = webview.clone();
        let navigation_registration = webview.on_navigation_completed(move |args| {
            println!(
                "navigation {} completed: success = {}, title = {:?}",
                args.navigation_id(),
                args.is_success(),
                post.document_title()
            );
            if args.is_success() {
                let _ = post.execute_script(
                    "window.chrome.webview.postMessage('hello from ' + document.title);",
                    |_| {},
                );
            }
        })?;

        let title_registration = webview.on_document_title_changed(|title| {
            println!("document title changed: {title}");
        })?;

        let loading_registration = webview.on_content_loading(|args| {
            println!(
                "content loading: navigation {} (error page = {})",
                args.navigation_id(),
                args.is_error_page()
            );
        })?;

        let close_registration = webview.on_window_close_requested(|| {
            println!("page requested window close");
        })?;

        let new_window_registration = webview.on_new_window_requested(|args| {
            println!("blocking new window for: {}", args.uri());
            let _ = args.set_handled(true);
        })?;

        let permission_registration = webview.on_permission_requested(|args| {
            println!("denying {:?} permission for {}", args.kind(), args.uri());
            let _ = args.set_state(PermissionState::Deny);
        })?;

        webview.navigate("https://github.com/microsoft/windows-rs")?;
        CONTROLLER.with(|slot| *slot.borrow_mut() = Some(controller));
        REGISTRATIONS.with(|slot| {
            slot.borrow_mut().extend([
                message_registration,
                starting_registration,
                navigation_registration,
                title_registration,
                loading_registration,
                close_registration,
                new_window_registration,
                permission_registration,
            ]);
        });

        let mut message = MSG::default();
        while GetMessageA(&mut message, std::ptr::null_mut(), 0, 0).as_bool() {
            DispatchMessageA(&message);
        }
    }

    Ok(())
}
