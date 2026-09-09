//! Message and resize handlers dispatch through the window procedure.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use test_window::{DestroyWindow, IsWindow, SendMessageW, WM_CLOSE, WM_MOVE, WM_SIZE, WM_USER};
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

#[test]
fn reentrant_close_invalidates_the_window() {
    let window = Window::new("test")
        .on_message(|hwnd, message, _wparam, _lparam| {
            if message == WM_USER {
                unsafe {
                    SendMessageW(hwnd, WM_CLOSE, 0, 0);
                }
                Some(0)
            } else {
                None
            }
        })
        .create()
        .unwrap();
    let hwnd = window.hwnd();

    unsafe {
        SendMessageW(hwnd, WM_USER, 0, 0);
    }

    assert!(window.hwnd().is_null());
    assert!(unsafe { IsWindow(hwnd) } == 0);
}

#[test]
fn reentrant_close_from_resize_runs_the_close_handler() {
    let close_count = Rc::new(Cell::new(0));
    let captured_count = close_count.clone();
    let hwnd = Rc::new(Cell::new(std::ptr::null_mut()));
    let captured_hwnd = hwnd.clone();
    let window = Window::new("test")
        .on_resize(move |_width, _height| unsafe {
            SendMessageW(captured_hwnd.get(), WM_CLOSE, 0, 0);
        })
        .on_close(move || captured_count.set(captured_count.get() + 1))
        .create()
        .unwrap();
    hwnd.set(window.hwnd());

    unsafe {
        SendMessageW(window.hwnd(), WM_SIZE, 0, 0);
    }

    assert_eq!(close_count.get(), 1);
    assert!(window.hwnd().is_null());
}

#[test]
fn reentrant_close_from_close_handler_is_suppressed() {
    let close_count = Rc::new(Cell::new(0));
    let captured_count = close_count.clone();
    let hwnd = Rc::new(Cell::new(std::ptr::null_mut()));
    let captured_hwnd = hwnd.clone();
    let window = Window::new("test")
        .on_close(move || {
            captured_count.set(captured_count.get() + 1);
            unsafe {
                SendMessageW(captured_hwnd.get(), WM_CLOSE, 0, 0);
            }
        })
        .create()
        .unwrap();
    hwnd.set(window.hwnd());

    window.close();

    assert_eq!(close_count.get(), 1);
    assert!(window.hwnd().is_null());
}

#[test]
fn reentrant_message_does_not_enter_another_handler() {
    let value = Rc::new(RefCell::new(0));
    let resize_value = Rc::clone(&value);
    let move_value = Rc::clone(&value);
    let hwnd = Rc::new(Cell::new(std::ptr::null_mut()));
    let resize_hwnd = Rc::clone(&hwnd);
    let window = Window::new("test")
        .on_resize(move |_width, _height| {
            let mut value = resize_value.borrow_mut();
            *value += 1;
            unsafe {
                SendMessageW(resize_hwnd.get(), WM_MOVE, 0, 0);
            }
        })
        .on_move(move || *move_value.borrow_mut() += 10)
        .create()
        .unwrap();
    hwnd.set(window.hwnd());
    let initial = *value.borrow();

    unsafe {
        SendMessageW(window.hwnd(), WM_SIZE, 0, 0);
    }
    assert_eq!(*value.borrow(), initial + 1);

    unsafe {
        SendMessageW(window.hwnd(), WM_MOVE, 0, 0);
    }
    assert_eq!(*value.borrow(), initial + 11);
}

#[test]
fn nested_message_cannot_replay_close_before_handler_returns() {
    let close_count = Rc::new(Cell::new(0));
    let captured_count = Rc::clone(&close_count);
    let hwnd = Rc::new(Cell::new(std::ptr::null_mut()));
    let resize_hwnd = Rc::clone(&hwnd);
    let window = Window::new("test")
        .on_resize(move |_width, _height| unsafe {
            SendMessageW(resize_hwnd.get(), WM_CLOSE, 0, 0);
            SendMessageW(resize_hwnd.get(), WM_MOVE, 0, 0);
        })
        .on_close(move || captured_count.set(captured_count.get() + 1))
        .create()
        .unwrap();
    hwnd.set(window.hwnd());

    unsafe {
        SendMessageW(window.hwnd(), WM_SIZE, 0, 0);
    }

    assert_eq!(close_count.get(), 1);
    assert!(window.hwnd().is_null());
}

#[test]
fn handler_can_destroy_its_window() {
    let window = Window::new("test")
        .on_message(|hwnd, message, _wparam, _lparam| {
            if message == WM_USER {
                unsafe {
                    DestroyWindow(hwnd);
                }
            }
            None
        })
        .create()
        .unwrap();
    let hwnd = window.hwnd();

    unsafe {
        SendMessageW(hwnd, WM_USER, 0, 0);
    }

    assert!(window.hwnd().is_null());
    assert_eq!(unsafe { IsWindow(hwnd) }, 0);
}
