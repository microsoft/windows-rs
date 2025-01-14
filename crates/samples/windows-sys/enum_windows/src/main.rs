use windows_sys::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        EnumWindows(Some(enum_window), 0);
    }
}

extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() {
            println!("{text}");
        }

        1
    }
}
