use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};

fn main() {
    unsafe {
        MessageBoxA(HWND::default(), PCSTR("Hello".as_bytes().as_ptr()), PCSTR("World".as_bytes().as_ptr()), MB_OK);
    }
}
