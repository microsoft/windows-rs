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
use std::time::{Duration, Instant};
use windows_trayicon::{TrayIcon, TrayIconEvent};

const CALLBACK_MESSAGE: u32 = WM_USER as u32 + 1;

#[test]
#[ignore = "requires an interactive Windows shell"]
fn raw_message_loop_dispatches_posted_events() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\icon.ico");
    let activated = std::rc::Rc::new(std::cell::Cell::new(false));
    let callback_activated = std::rc::Rc::clone(&activated);
    let icon = TrayIcon::new(path)
        .on_event(move |event| {
            if matches!(event, TrayIconEvent::Activate { .. }) {
                callback_activated.set(true);
            }
        })
        .build()
        .unwrap();
    let hwnd = icon.hwnd() as usize;
    let driver = std::thread::spawn(move || unsafe {
        SendMessageW(hwnd as _, CALLBACK_MESSAGE, 0, NIN_SELECT as isize);
    });

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut message = MSG::default();
    while Instant::now() < deadline && !activated.get() {
        while unsafe { PeekMessageW(&mut message, core::ptr::null_mut(), 0, 0, PM_REMOVE as u32) }
            .as_bool()
        {
            unsafe {
                _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    driver.join().unwrap();
    assert!(activated.get());
}

#[test]
#[ignore = "requires an interactive Windows shell"]
fn nested_message_loop_defers_callback_until_the_active_handler_returns() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\icon.ico");
    let delivered = std::rc::Rc::new(std::cell::Cell::new(0));
    let callback_delivered = std::rc::Rc::clone(&delivered);
    let callback_hwnd = std::rc::Rc::new(std::cell::Cell::new(0_usize));
    let handler_hwnd = std::rc::Rc::clone(&callback_hwnd);
    let icon = TrayIcon::new(path)
        .on_event(move |event| {
            if matches!(event, TrayIconEvent::Activate { .. }) {
                let count = callback_delivered.get() + 1;
                callback_delivered.set(count);
                if count == 1 {
                    unsafe {
                        SendMessageW(
                            handler_hwnd.get() as _,
                            CALLBACK_MESSAGE,
                            0,
                            NIN_SELECT as isize,
                        );
                    }
                    assert!(windows_window::pump());
                    assert_eq!(callback_delivered.get(), 1);
                }
            }
        })
        .build()
        .unwrap();
    callback_hwnd.set(icon.hwnd() as usize);

    unsafe {
        SendMessageW(icon.hwnd(), CALLBACK_MESSAGE, 0, NIN_SELECT as isize);
    }
    assert!(windows_window::pump());
    assert_eq!(delivered.get(), 2);
}
