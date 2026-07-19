fn main() {
    use windows::{Win32::*, core::*};

    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK);
        MessageBoxW(None, h!("WinRT"), h!("World"), MB_OK);
        ShellMessageBoxW(None, None, w!("Wide"), w!("World"), MB_ICONERROR);
    }
}
