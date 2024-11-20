use windows::Win32::Graphics::Printing::*;

// Validates that the target libs resolve this function to "winspool.drv"

#[test]
fn test() {
    unsafe {
        _ = GetSpoolFileHandle(PRINTER_HANDLE::default());
    }
}
