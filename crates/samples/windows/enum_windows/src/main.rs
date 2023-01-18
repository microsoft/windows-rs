use windows::{
    Win32::Foundation::{BOOL, HWND, LPARAM},
    Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowInfo, GetWindowTextW, WINDOWINFO, WS_VISIBLE,
    },
};

fn main() -> windows::core::Result<()> {
    unsafe { EnumWindows(Some(enum_window), LPARAM(0)).ok() }
}

extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        let mut info = WINDOWINFO {
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };
        GetWindowInfo(window, &mut info).unwrap();

        if !text.is_empty() && info.dwStyle & WS_VISIBLE.0 != 0 {
            println!("{} ({}, {})", text, info.rcWindow.left, info.rcWindow.top);
        }

        true.into()
    }
}
