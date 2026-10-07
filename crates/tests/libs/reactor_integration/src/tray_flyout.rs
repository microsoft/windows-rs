use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use windows::Win32::uiautomationclient::{CUIAutomation, IUIAutomation, TreeScope_Subtree};
use windows::Win32::winuser::{
    EnumWindows, GetWindowRect, IsWindowVisible, WM_CONTEXTMENU, WM_USER,
};
use windows::Win32::{
    CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
    GetClassNameW, GetMonitorInfoW, GetWindowThreadProcessId, HWND, LPARAM,
    MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint, POINT, PostMessageW, RECT, WPARAM,
};
use windows_core::{BOOL, PWSTR};
use windows_notifyicon::{NotifyIcon, NotifyIconEvent};
use windows_reactor::{App, Menu, MenuItem, ScreenPoint};

fn flyout_outside_work_area(flyout: &RECT, workarea: &RECT) -> bool {
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
        unsafe { *(lparam as *mut Option<HWND>) = Some(hwnd) };
        return BOOL(0);
    }
    BOOL(1)
}

fn find_menu_window() -> Option<HWND> {
    let mut found = None;
    unsafe { _ = EnumWindows(Some(callback), &mut found as *mut _ as LPARAM) };
    found
}

fn menu_item_rects(automation: &IUIAutomation, hwnd: HWND) -> windows_core::Result<Vec<RECT>> {
    unsafe {
        let root = automation.ElementFromHandle(hwnd.cast())?;
        let elements = root.FindAll(TreeScope_Subtree, &automation.CreateTrueCondition()?)?;
        let mut rects = Vec::new();
        for index in 0..elements.Length()? {
            let element = elements.GetElement(index)?;
            let name = element.CurrentName()?;
            if (name == "Open" || name == "Exit") && element.CurrentClassName()? == "MenuFlyoutItem"
            {
                rects.push(element.CurrentBoundingRectangle()?);
            }
        }
        Ok(rects)
    }
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("Reactor tray flyout fixture timed out");
        std::process::exit(1);
    });

    let completed = Rc::new(Cell::new(false));
    let run_completed = Rc::clone(&completed);
    App::run_with(move |app| {
        let exit_context = app.clone();
        let exit = app.callback(move || {
            run_completed.set(true);
            exit_context.exit()
        });
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
                .unwrap();
            }
        })
        .build()?;

        let r = icon.rect()?;
        let (x, y) = ((r.left + r.right) / 2, (r.top + r.bottom) / 2);
        let mut monitor = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        unsafe {
            GetMonitorInfoW(
                MonitorFromPoint(POINT { x, y }, MONITOR_DEFAULTTONEAREST as u32),
                &mut monitor,
            )
            .ok()?;
        }
        let packed = ((x as u16 as u32) | ((y as u16 as u32) << 16)) as WPARAM;
        let hwnd = icon.hwnd() as isize;

        std::thread::spawn(move || {
            unsafe {
                CoInitializeEx(None, COINIT_MULTITHREADED as u32)
                    .ok()
                    .unwrap();
            };
            let automation: IUIAutomation =
                unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER) }.unwrap();
            unsafe {
                PostMessageW(
                    Some(hwnd as HWND),
                    (WM_USER + 1) as u32,
                    packed,
                    WM_CONTEXTMENU as LPARAM,
                )
            }
            .ok()
            .unwrap();

            let rects = loop {
                if let Some(hwnd) = find_menu_window() {
                    let rects = menu_item_rects(&automation, hwnd).unwrap();
                    if rects.len() == 2 {
                        break rects;
                    }
                }
                std::thread::sleep(Duration::from_millis(100));
            };

            // The popup HWND includes its shadow, which can cross the work area even when the
            // menu is constrained. Measure the visible menu items instead.
            if !rects
                .iter()
                .any(|rect| flyout_outside_work_area(rect, &monitor.rcWork))
            {
                eprintln!(
                    "menu items {rects:?} did not escape work area {:?}",
                    monitor.rcWork
                );
                std::process::exit(1);
            }

            drop(automation);
            unsafe { CoUninitialize() };
            exit.invoke().unwrap();
        });

        Ok(icon)
    })?;
    assert!(completed.get());
    assert!(
        find_menu_window().is_none(),
        "menu survived application shutdown"
    );
    Ok(())
}
