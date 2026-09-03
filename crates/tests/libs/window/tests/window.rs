//! Window lifecycle: creation, handle validity, client size, and destruction.

use test_window::{IsWindow, WS_EX_NOREDIRECTIONBITMAP, get_window_ex_style};
use windows_window::Window;

#[test]
fn create_returns_a_live_window() {
    let window = Window::new("test").size(400, 300).create().unwrap();
    assert!(!window.hwnd().is_null());
    assert!(unsafe { IsWindow(window.hwnd()) } != 0);
}

#[test]
fn client_size_fits_within_the_requested_size() {
    let window = Window::new("test").size(400, 300).create().unwrap();
    let (width, height) = window.client_size();
    assert!(width > 0 && height > 0);
    assert!(width <= 400 && height <= 300);
}

#[test]
fn client_size_matches_the_requested_size() {
    let window = Window::new("test").client_size(400, 300).create().unwrap();
    assert_eq!(window.client_size(), (400, 300));
}

#[test]
fn no_redirection_bitmap_sets_the_extended_style() {
    let window = Window::new("test")
        .no_redirection_bitmap()
        .create()
        .unwrap();
    let ex_style = get_window_ex_style(&window);
    assert_ne!(ex_style & WS_EX_NOREDIRECTIONBITMAP, 0);
}

#[test]
fn drop_destroys_the_window() {
    let window = Window::new("test").size(400, 300).create().unwrap();
    let hwnd = window.hwnd();
    assert!(unsafe { IsWindow(hwnd) } != 0);
    drop(window);
    assert!(unsafe { IsWindow(hwnd) } == 0);
}
