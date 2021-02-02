use bindings::{
    windows::win32::windows_and_messaging::{EnumWindows, GetWindowTextW, HWND, LPARAM},
    windows::{Result, BOOL},
};

fn main() -> Result<()> {
    unsafe { EnumWindows(Some(enum_window), LPARAM(0)).ok() }
}

extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() {
            println!("{}", text);
        }

        return true.into();
    }
}
