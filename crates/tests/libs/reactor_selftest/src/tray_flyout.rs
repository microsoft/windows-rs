use std::time::Duration;
use windows::Win32::winuser::{
    EnumWindows, GetWindowRect, IsWindowVisible, SPI_GETWORKAREA, SystemParametersInfoW,
    WM_CONTEXTMENU, WM_USER,
};
use windows::Win32::{
    GetClassNameW, GetWindowThreadProcessId, HWND, LPARAM, PostMessageW, RECT, WPARAM,
};
use windows_core::{BOOL, PWSTR};
use windows_notifyicon::{NotifyIcon, NotifyIconEvent};
use windows_reactor::{App, Menu, MenuItem, ScreenPoint};

fn flyout_outside_work_area(flyout: &RECT) -> bool {
    let mut workarea = RECT::default();
    unsafe {
        SystemParametersInfoW(
            SPI_GETWORKAREA as u32,
            0,
            &mut workarea as *mut _ as *mut _,
            0,
        )
    }
    .ok()
    .expect("failed to get work area");

    flyout.left < workarea.left
        || flyout.top < workarea.top
        || flyout.right > workarea.right
        || flyout.bottom > workarea.bottom
}

fn class_name(hwnd: HWND) -> String {
    let mut buf = [0u16; 256];
    let len = unsafe { GetClassNameW(hwnd, PWSTR(buf.as_mut_ptr()), buf.len() as i32) };
    String::from_utf16_lossy(&buf[..len.max(0) as usize])
}

unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut pid = 0;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };

    let mut rect = RECT::default();
    if pid == std::process::id()
        && unsafe { IsWindowVisible(hwnd) }.as_bool()
        && class_name(hwnd) == "Microsoft.UI.Content.PopupWindowSiteBridge"
        && unsafe { GetWindowRect(hwnd, &mut rect) }.as_bool()
        && rect.right - rect.left > 10
        && rect.bottom - rect.top > 10
    {
        unsafe { *(lparam as *mut Option<RECT>) = Some(rect) };
        return BOOL(0);
    }
    BOOL(1)
}

fn find_menu_rect() -> Option<RECT> {
    let mut found = None;
    unsafe { _ = EnumWindows(Some(callback), &mut found as *mut _ as LPARAM) };
    found
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(10));
        eprintln!("Reactor tray flyout fixture timed out");
        std::process::exit(1);
    });

    App::run_with(|app| {
        let app = app.clone();

        let icon = NotifyIcon::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../notifyicon/assets/icon.ico"
        ))
        .on_event(move |event| {
            if let NotifyIconEvent::ContextMenu { position } = event {
                app.show_menu_at(
                    ScreenPoint::new(position.x, position.y),
                    Menu::new(
                        [
                            MenuItem::item("open", "Open"),
                            MenuItem::separator("separator"),
                            MenuItem::item("exit", "Exit"),
                        ],
                        |_| {},
                    ),
                )
                .expect("failed to show menu");
            }
        })
        .build()?;

        let r = icon.rect()?;
        let (x, y) = ((r.left + r.right) / 2, (r.top + r.bottom) / 2);
        let packed = ((x as u16 as u32) | ((y as u16 as u32) << 16)) as WPARAM;
        let hwnd = icon.hwnd() as isize;

        std::thread::spawn(move || {
            _ = unsafe {
                PostMessageW(
                    Some(hwnd as HWND),
                    (WM_USER + 1) as u32,
                    packed,
                    WM_CONTEXTMENU as LPARAM,
                )
            };

            let rect = loop {
                if let Some(r) = find_menu_rect() {
                    break r;
                }
                std::thread::sleep(Duration::from_millis(100));
            };

            if !flyout_outside_work_area(&rect) {
                eprintln!("menu {rect:?} did not escape work area as expected");
                std::process::exit(1);
            }

            std::process::exit(0);
        });

        Ok(icon)
    })
}
