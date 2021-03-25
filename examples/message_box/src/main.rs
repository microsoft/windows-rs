use bindings::Windows::Win32::WindowsAndMessaging::{MessageBoxA, HWND, MESSAGEBOX_STYLE};

fn main() {
    unsafe {
        MessageBoxA(HWND(0), "Hello", "World", MESSAGEBOX_STYLE::MB_OK);
    }
}
