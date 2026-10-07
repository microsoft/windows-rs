use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use windows::Win32::*;
use windows_core::{BOOL, PWSTR};
use windows_reactor::{App, Menu, MenuItem, ScreenPoint};

unsafe extern "system" fn find_host(hwnd: HWND, parameter: LPARAM) -> BOOL {
    let mut title = [0; 64];
    let len = unsafe { GetWindowTextW(hwnd, PWSTR(title.as_mut_ptr()), title.len() as i32) };
    if String::from_utf16_lossy(&title[..len as usize]) == "TransientMenuHost" {
        unsafe { *(parameter as *mut HWND) = hwnd };
        return false.into();
    }
    true.into()
}

fn main() -> windows_core::Result<()> {
    let mode = std::env::args().nth(1).unwrap();
    let process = match mode.as_str() {
        "unaware" => DPI_AWARENESS_CONTEXT_UNAWARE,
        "system" => DPI_AWARENESS_CONTEXT_SYSTEM_AWARE,
        "pmv2" | "thread-unaware" => DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        _ => panic!("expected DPI mode"),
    };
    unsafe { SetProcessDpiAwarenessContext(process).ok()? };
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(20));
        eprintln!("menu position fixture timed out");
        std::process::exit(1);
    });
    let observed = Rc::new(Cell::new(None));
    let run_observed = Rc::clone(&observed);
    App::run_with(move |app| {
        let context = if mode == "thread-unaware" {
            DPI_AWARENESS_CONTEXT_UNAWARE
        } else {
            process
        };
        let previous = unsafe { SetThreadDpiAwarenessContext(context) };
        assert!(!previous.is_null());
        let result = app.show_menu_at(
            ScreenPoint::new(600, 400),
            Menu::new([MenuItem::item("open", "Open")], |_| {}),
        );
        let after = unsafe { GetThreadDpiAwarenessContext() };
        let duplicate = app.show_menu_at(
            ScreenPoint::new(700, 500),
            Menu::new([MenuItem::item("duplicate", "Duplicate")], |_| {}),
        );
        let after_error = unsafe { GetThreadDpiAwarenessContext() };
        unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
        let mut hwnd: HWND = std::ptr::null_mut();
        unsafe {
            _ = EnumThreadWindows(
                GetCurrentThreadId(),
                Some(find_host),
                &mut hwnd as *mut HWND as LPARAM,
            );
        }
        assert!(!hwnd.is_null());
        let mut rect = RECT::default();
        let measured = unsafe { GetWindowRect(hwnd, &mut rect).ok() };
        let host_pmv2 = unsafe {
            AreDpiAwarenessContextsEqual(
                GetWindowDpiAwarenessContext(hwnd),
                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
            )
            .as_bool()
        };
        eprintln!(
            "{mode}: requested (600, 400), physical {rect:?}, host DPI {}",
            unsafe { GetDpiForWindow(hwnd) }
        );
        unsafe { SetThreadDpiAwarenessContext(previous) };
        result?;
        measured?;
        run_observed.set(Some((
            rect,
            host_pmv2,
            unsafe { AreDpiAwarenessContextsEqual(context, after) }.as_bool(),
            duplicate.is_err()
                && unsafe { AreDpiAwarenessContextsEqual(context, after_error) }.as_bool(),
        )));
        app.proxy().exit()?;
        Ok(())
    })?;
    let (rect, host_pmv2, restored, restored_error) = observed.get().unwrap();
    assert_eq!(
        (rect.left, rect.top, rect.right, rect.bottom),
        (600, 400, 602, 402)
    );
    assert!(host_pmv2, "menu host must be created under PMv2");
    assert!(restored, "menu placement changed the caller's DPI context");
    assert!(
        restored_error,
        "rejected menu changed the caller's DPI context"
    );
    Ok(())
}
