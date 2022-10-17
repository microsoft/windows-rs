use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() {
    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("World"), MB_OK);
        MessageBoxW(None, h!("WinRT"), h!("World"), MB_OK);
    }
}
