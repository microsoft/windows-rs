#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() {
    use windows::{core::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*};

    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK);
        MessageBoxW(None, h!("WinRT"), h!("World"), MB_OK);
        ShellMessageBoxW(None, None, w!("Wide"), w!("World"), MB_ICONERROR);
    }
}
