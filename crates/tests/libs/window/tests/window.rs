//! Window lifecycle: creation, handle validity, client size, and destruction.

use test_window::IsWindow;
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
fn drop_destroys_the_window() {
    let window = Window::new("test").size(400, 300).create().unwrap();
    let hwnd = window.hwnd();
    assert!(unsafe { IsWindow(hwnd) } != 0);
    drop(window);
    assert!(unsafe { IsWindow(hwnd) } == 0);
}
