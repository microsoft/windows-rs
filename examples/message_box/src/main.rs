use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};

fn main() {
    unsafe {
        MessageBoxA(None, "Hello", "World", MB_OK);
    }
}
