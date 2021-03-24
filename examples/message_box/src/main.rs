use bindings::windows::win32::windows_and_messaging::{MessageBoxA, HWND, MESSAGEBOX_STYLE};

fn main() {
    unsafe {
        MessageBoxA(HWND(0), "Hello", "World", MESSAGEBOX_STYLE::MB_OK);
    }
}
