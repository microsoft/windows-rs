use windows::{Win32::Foundation::*, Win32::Graphics::Printing::*};

// Validates that the target libs resolve this function to "winspool.drv"

#[test]
fn test() {
    unsafe {
        let _ = GetSpoolFileHandle(HANDLE(0));
    }
}
