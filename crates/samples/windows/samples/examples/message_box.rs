fn main() {
    use windows::{Win32::*, core::*};

    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK as u32);
        MessageBoxW(None, h!("WinRT"), h!("World"), MB_OK as u32);
        MessageBoxW(None, w!("Wide"), w!("World"), MB_ICONERROR as u32);
    }
}
