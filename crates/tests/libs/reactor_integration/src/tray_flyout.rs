use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};
use windows::Win32::uiautomationclient::{CUIAutomation, IUIAutomation, TreeScope_Subtree};
use windows::Win32::winuser::{
    EnumWindows, GetWindowRect, IsWindowVisible, WM_CONTEXTMENU, WM_USER,
};
use windows::Win32::{
    CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, CoUninitialize,
    FindWindowExW, GetClassNameW, GetMonitorInfoW, GetWindowThreadProcessId, HWND, LPARAM,
    MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint, NIN_SELECT, NOTIFYICONIDENTIFIER,
    POINT, PostMessageW, RECT, Shell_NotifyIconGetRect, WPARAM,
};
use windows_core::{BOOL, PWSTR, w};
use windows_reactor::*;

enum Message {
    Ready,
    Exit,
}

struct TrayApp(Sender<()>);

impl Application for TrayApp {
    type Input = Sender<()>;
    type Message = Message;

    fn create(input: &Self::Input, context: &ApplicationContext<Self>) -> Self {
        assert!(context.sender().send(Message::Ready));
        Self(input.clone())
    }

    fn update(&mut self, message: Message, context: &ApplicationContext<Self>) {
        match message {
            Message::Ready => self.0.send(()).unwrap(),
            Message::Exit => context.exit(),
        }
    }

    fn view(&self, _: &Self::Input, context: &ApplicationViewContext<Self>) -> ApplicationView {
        ApplicationView::new().notify_icon(
            "tray",
            NotifyIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico"))
                .on_activate(context.callback(|_| Message::Exit))
                .menu(Menu::new(
                    [
                        MenuItem::item("open", "Open"),
                        MenuItem::separator("separator"),
                        MenuItem::item("exit", "Exit"),
                    ],
                    |_| {},
                )),
        )
    }
}

fn find_icon_window() -> Option<HWND> {
    let mut after = None;
    loop {
        let hwnd = unsafe { FindWindowExW(None, after, None, w!("windows-notifyicon")) };
        if hwnd.is_null() {
            return None;
        }
        let mut pid = 0;
        unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
        if pid == std::process::id() {
            return Some(hwnd);
        }
        after = Some(hwnd);
    }
}

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

fn drive(ready: Receiver<()>) -> windows_core::Result<()> {
    ready.recv_timeout(Duration::from_secs(10)).unwrap();
    let hwnd = find_icon_window().unwrap();
    let r = unsafe {
        Shell_NotifyIconGetRect(&NOTIFYICONIDENTIFIER {
            cbSize: size_of::<NOTIFYICONIDENTIFIER>() as u32,
            hWnd: hwnd,
            uID: 1,
            ..Default::default()
        })?
    };
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
    let automation: IUIAutomation =
        unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER) }?;
    let packed = ((x as u16 as u32) | ((y as u16 as u32) << 16)) as WPARAM;
    unsafe {
        PostMessageW(
            Some(hwnd),
            (WM_USER + 1) as u32,
            packed,
            WM_CONTEXTMENU as LPARAM,
        )
    }
    .ok()?;

    let deadline = Instant::now() + Duration::from_secs(10);
    let rects = loop {
        if let Some(hwnd) = find_menu_window() {
            let rects = menu_item_rects(&automation, hwnd)?;
            if rects.len() == 2 {
                break rects;
            }
        }
        assert!(Instant::now() < deadline, "notification menu did not open");
        std::thread::sleep(Duration::from_millis(100));
    };

    assert!(
        rects.iter().all(|rect| {
            rect.left < rect.right
                && rect.top < rect.bottom
                && rect.left < monitor.rcMonitor.right
                && rect.right > monitor.rcMonitor.left
                && rect.top < monitor.rcMonitor.bottom
                && rect.bottom > monitor.rcMonitor.top
        }),
        "menu items {rects:?} did not overlap icon monitor {:?}",
        monitor.rcMonitor
    );
    // The popup HWND includes its shadow, which can cross the work area even when the
    // menu is constrained. Measure the visible menu items instead.
    assert!(
        rects
            .iter()
            .any(|rect| flyout_outside_work_area(rect, &monitor.rcWork)),
        "menu items {rects:?} did not escape work area {:?}",
        monitor.rcWork
    );

    // Exit through the application message queue while its notification menu is still open.
    unsafe {
        PostMessageW(
            Some(hwnd),
            (WM_USER + 1) as u32,
            packed,
            NIN_SELECT as LPARAM,
        )
    }
    .ok()
}

fn main() -> windows_core::Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        eprintln!("Reactor tray flyout fixture timed out");
        std::process::exit(1);
    });

    let (ready, receiver) = channel();
    let driver = std::thread::spawn(move || {
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED as u32) }
            .ok()
            .unwrap();
        let result = drive(receiver);
        unsafe { CoUninitialize() };
        result.unwrap();
    });
    App::run_application::<TrayApp>(ready)?;
    driver.join().unwrap();
    assert!(
        find_icon_window().is_none(),
        "icon survived application shutdown"
    );
    assert!(
        find_menu_window().is_none(),
        "menu survived application shutdown"
    );
    Ok(())
}
