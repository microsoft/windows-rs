//! Message and resize handlers dispatch through the window procedure.

use std::cell::Cell;
use std::rc::Rc;
use test_window::{SendMessageW, WM_SIZE, WM_USER};
use windows_window::Window;

#[test]
fn on_message_receives_dispatched_messages() {
    let count = Rc::new(Cell::new(0));
    let seen = count.clone();
    let window = Window::new("test")
        .on_message(move |_hwnd, message, _wparam, _lparam| {
            if message == WM_USER {
                seen.set(seen.get() + 1);
                Some(0)
            } else {
                None
            }
        })
        .create()
        .unwrap();

    unsafe {
        SendMessageW(window.hwnd(), WM_USER, 0, 0);
    }
    assert_eq!(count.get(), 1);
}

#[test]
fn on_resize_receives_the_new_client_size() {
    let size = Rc::new(Cell::new((0, 0)));
    let captured = size.clone();
    let window = Window::new("test")
        .on_resize(move |width, height| captured.set((width, height)))
        .create()
        .unwrap();

    let lparam = 640 | (480 << 16);
    unsafe {
        SendMessageW(window.hwnd(), WM_SIZE, 0, lparam);
    }
    assert_eq!(size.get(), (640, 480));
}
