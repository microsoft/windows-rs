use bindings::windows::win32::windows_and_messaging::{MessageBoxA, HWND, MB_FLAGS};

fn main() {
    unsafe {
        MessageBoxA(HWND(0), "Hello", "World", MB_FLAGS::MB_OK);
    }
}
