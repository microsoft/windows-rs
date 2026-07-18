fn main() {
    use windows_sys::{core::*, Win32::*};

    unsafe {
        MessageBoxA(core::ptr::null_mut(), s!("Ansi"), s!("World"), MB_OK);

        ShellMessageBoxW(
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            w!("Wide"),
            w!("World"),
            MB_ICONERROR,
        );
    }
}
