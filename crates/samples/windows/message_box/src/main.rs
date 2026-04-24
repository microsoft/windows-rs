use windows::{core::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        MessageBoxA(None, Some(s!("Ansi")), Some(s!("World")), MB_OK);
        MessageBoxW(None, Some(w!("WinRT")), Some(w!("World")), MB_OK);
        ShellMessageBoxW(None, None, w!("Wide"), Some(w!("World")), MB_ICONERROR);
    }
}
