use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() {
    unsafe {
        // TODO: these can now be replaced with the s! and w! macros (coming up next).

        MessageBoxA(None, PCSTR("Ansi\0".as_bytes().as_ptr()), PCSTR("World\0".as_bytes().as_ptr()), MB_OK);

        let hello = HSTRING::from("Wide");
        let world = HSTRING::from("World");
        MessageBoxW(None, &hello, &world, MB_OK);
    }
}
