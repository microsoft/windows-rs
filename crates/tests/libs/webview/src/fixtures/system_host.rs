use crate::harness::Harness;
use std::cell::Cell;
use std::rc::Rc;
use windows_core::{Error, HRESULT};
use windows_webview::{WebViewHost, WebViewWindow};
use windows_window::Window;

const E_ABORT: HRESULT = HRESULT(0x8000_4004_u32 as i32);
const E_FAIL: HRESULT = HRESULT(0x8000_4005_u32 as i32);
const E_INVALIDARG: HRESULT = HRESULT(0x8007_0057_u32 as i32);
const WM_CLOSE: u32 = 0x0010;

windows_link::link!("user32.dll" "system" fn FindWindowW(class_name: *const u16, window_name: *const u16) -> *mut core::ffi::c_void);
windows_link::link!("user32.dll" "system" fn IsWindow(hwnd: *mut core::ffi::c_void) -> i32);
windows_link::link!("user32.dll" "system" fn SendMessageW(hwnd: *mut core::ffi::c_void, message: u32, wparam: usize, lparam: isize) -> isize);

pub fn framework_host_creates_and_closes(harness: &Harness) {
    let Ok(window) = Window::new("windows-webview - framework host fixture")
        .client_size(640, 480)
        .quit_on_close(false)
        .create()
    else {
        harness.check("framework parent window created", false);
        return;
    };
    let Some(result) =
        Harness::complete(|handler| WebViewHost::builder().create_for_hwnd(window.hwnd(), handler))
    else {
        harness.check("framework host creation completed", false);
        return;
    };
    let Ok(host) = result else {
        harness.check("framework host creation succeeded", false);
        return;
    };

    let (width, height) = window.client_size();
    harness.check(
        "framework host controller sized",
        host.controller().set_bounds(0, 0, width, height).is_ok(),
    );
    harness.check(
        "framework host exposes its browser",
        host.webview()
            .navigate_to_string("<!DOCTYPE html><html></html>")
            .is_ok(),
    );
    harness.check("framework host closes cleanly", host.close().is_ok());
}

pub fn framework_host_rejects_null_parent(harness: &Harness) {
    let called = Rc::new(Cell::new(false));
    let callback_called = Rc::clone(&called);
    let result = WebViewHost::builder().create_for_hwnd(core::ptr::null_mut(), move |_| {
        callback_called.set(true);
    });

    harness.check(
        "framework null parent rejected",
        matches!(result, Err(ref error) if error.code() == E_INVALIDARG),
    );
    harness.check("framework null parent callback skipped", !called.get());
}

pub fn creates_and_closes(harness: &Harness) {
    let closed = Rc::new(Cell::new(0));
    let close_callback = Rc::clone(&closed);
    let Some(result) = Harness::complete(|handler| {
        WebViewWindow::new("windows-webview - system host fixture")
            .client_size(640, 480)
            .quit_on_close(false)
            .on_close(move || close_callback.set(close_callback.get() + 1))
            .create(handler)
    }) else {
        harness.check("system host creation completed", false);
        return;
    };
    let Ok(host) = result else {
        harness.check("system host creation succeeded", false);
        return;
    };

    harness.check(
        "system host has a live window",
        !host.window().hwnd().is_null(),
    );
    harness.check(
        "system host exposes its controller",
        host.controller().zoom_factor().is_finite(),
    );
    harness.check("system host closes cleanly", host.close().is_ok());
    host.window().close();
    harness.check(
        "system parent closes after host shutdown",
        host.window().hwnd().is_null(),
    );
    harness.check("system close callback runs once", closed.get() == 1);

    let dropped = Rc::new(Cell::new(0));
    let drop_callback = Rc::clone(&dropped);
    let Some(result) = Harness::complete(|handler| {
        WebViewWindow::new("windows-webview - system host drop fixture")
            .client_size(640, 480)
            .quit_on_close(false)
            .on_close(move || drop_callback.set(drop_callback.get() + 1))
            .create(handler)
    }) else {
        harness.check("system drop host creation completed", false);
        return;
    };
    let Ok(host) = result else {
        harness.check("system drop host creation succeeded", false);
        return;
    };

    drop(host);
    harness.check("system drop skips close callback", dropped.get() == 0);
}

pub fn close_during_creation_cancels_and_destroys(harness: &Harness) {
    let title = "windows-webview - pending close fixture";
    let title_wide: Vec<u16> = title.encode_utf16().chain([0]).collect();
    let mut parent = core::ptr::null_mut();
    let result = Harness::complete(|handler| {
        WebViewWindow::new(title)
            .client_size(640, 480)
            .quit_on_close(false)
            .create(handler)?;
        parent = unsafe { FindWindowW(core::ptr::null(), title_wide.as_ptr()) };
        if parent.is_null() {
            return Err(Error::new(E_FAIL, "WebView2 parent window was not found"));
        }
        unsafe {
            SendMessageW(parent, WM_CLOSE, 0, 0);
        }
        Ok(())
    });

    harness.check("pending close completion received", result.is_some());
    harness.check(
        "pending close reports cancellation",
        matches!(result, Some(Err(ref error)) if error.code() == E_ABORT),
    );
    harness.check(
        "pending close destroys parent",
        !parent.is_null() && unsafe { IsWindow(parent) } == 0,
    );
}
