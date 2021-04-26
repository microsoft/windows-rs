use bindings::Windows::Win32::WindowsAndMessaging::{MessageBoxA, MB_OK};

fn main() {
    unsafe {
        MessageBoxA(None, "Hello", "World", MB_OK);
    }
}
