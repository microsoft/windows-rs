#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_transmute_annotations,
    clippy::upper_case_acronyms
)]
#[path = "support/bindings.rs"]
mod bindings;

use bindings::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use windows_core::Result;
use windows_trayicon::{Menu, TrayIcon, TrayIconEvent};

const UIA_MENU_ITEM_CONTROL_TYPE_ID: i32 = 50011;
const UIA_INVOKE_PATTERN_ID: i32 = 10000;
const CALLBACK_MESSAGE: u32 = WM_USER as u32 + 1;

#[test]
#[ignore = "requires an unlocked interactive Windows desktop"]
fn automates_native_menu_selection() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\..\\..\\..\\samples\\reactor\\icon\\icon.ico"
    );
    let activated = Arc::new(AtomicBool::new(false));
    let selected = Arc::new(AtomicBool::new(false));
    let callback_activated = Arc::clone(&activated);
    let callback_selected = Arc::clone(&selected);
    let icon = TrayIcon::new(path)
        .tooltip(format!("windows-trayicon UIA test {}", std::process::id()))
        .menu(Menu::new().item(7, "Exit UIA test"))
        .on_event(move |event| match event {
            TrayIconEvent::Activate { .. } => {
                callback_activated.store(true, Ordering::Release);
            }
            TrayIconEvent::MenuItem { id: 7 } => {
                callback_selected.store(true, Ordering::Release);
            }
            _ => {}
        })
        .build()
        .unwrap();
    let rect = icon.rect().unwrap();
    let point = encoded_point((rect.left + rect.right) / 2, (rect.top + rect.bottom) / 2);
    let hwnd = icon.hwnd() as usize;
    let driver_activated = Arc::clone(&activated);
    let driver = std::thread::spawn(move || automate(hwnd, point, &driver_activated));

    let deadline = Instant::now() + Duration::from_secs(20);
    while Instant::now() < deadline && !selected.load(Ordering::Acquire) {
        assert!(windows_window::pump());
        std::thread::sleep(Duration::from_millis(10));
    }

    driver.join().unwrap().unwrap();
    assert!(activated.load(Ordering::Acquire));
    assert!(selected.load(Ordering::Acquire));
}

fn automate(hwnd: usize, point: usize, activated: &AtomicBool) -> Result<()> {
    unsafe {
        CoInitializeEx(core::ptr::null(), COINIT_MULTITHREADED as u32).ok()?;
        SendMessageW(hwnd as _, CALLBACK_MESSAGE, point, NIN_SELECT as isize);
    }
    wait_until(Duration::from_secs(5), || activated.load(Ordering::Acquire))?;

    unsafe {
        SendMessageW(hwnd as _, CALLBACK_MESSAGE, point, WM_CONTEXTMENU as isize);
    }

    let automation: IUIAutomation = windows_core::create_instance(&CUIAutomation)?;
    let root = unsafe { automation.GetRootElement()? };
    let item = find_named(
        &automation,
        &root,
        "Exit UIA test",
        UIA_MENU_ITEM_CONTROL_TYPE_ID,
        Duration::from_secs(5),
    )?;
    let invoke: IUIAutomationInvokePattern =
        unsafe { item.GetCurrentPatternAs(UIA_INVOKE_PATTERN_ID)? };
    unsafe {
        invoke.Invoke().ok()?;
    }
    Ok(())
}

fn find_named(
    automation: &IUIAutomation,
    root: &IUIAutomationElement,
    name: &str,
    control_type: i32,
    timeout: Duration,
) -> Result<IUIAutomationElement> {
    let condition = unsafe { automation.CreateTrueCondition()? };
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        let elements = unsafe { root.FindAll(TreeScope_Descendants, &condition)? };
        for index in 0..unsafe { elements.Length()? } {
            let element = unsafe { elements.GetElement(index)? };
            if unsafe { element.CurrentControlType()? } == control_type
                && name == unsafe { element.CurrentName()? }
            {
                return Ok(element);
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err(windows_core::Error::new(
        windows_core::HRESULT(0x8000_4005_u32 as i32),
        format!("UI Automation element not found: {name}"),
    ))
}

fn wait_until(timeout: Duration, condition: impl Fn() -> bool) -> Result<()> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if condition() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    Err(windows_core::Error::new(
        windows_core::HRESULT(0x8000_4005_u32 as i32),
        "timed out waiting for tray icon event",
    ))
}

fn encoded_point(x: i32, y: i32) -> usize {
    ((y as i16 as u16 as usize) << 16) | x as i16 as u16 as usize
}
