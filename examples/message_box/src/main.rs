use bindings::Windows::Win32::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};

fn main() {
    unsafe {
        MessageBoxA(None, "Hello", "World", MESSAGEBOX_STYLE::MB_OK);
    }
}
