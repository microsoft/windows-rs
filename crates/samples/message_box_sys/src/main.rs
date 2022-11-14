use windows_sys::{core::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        MessageBoxA(0, s!("Ansi"), s!("World"), MB_OK);
        ShellMessageBoxW(0, 0, w!("Wide"), w!("World"), MB_ICONERROR);
    }
}
