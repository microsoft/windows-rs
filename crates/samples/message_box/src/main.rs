use windows::Win32::UI::WindowsAndMessaging::*;
use windows::*;

fn main() {
    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("World"), MB_OK);
    }
}
