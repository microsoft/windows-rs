use windows_sys::Win32::Graphics::Printing::*;

// Validates that the target libs resolve this function to "winspool.drv"

#[test]
fn test() {
    unsafe {
        _ = GetSpoolFileHandle(PRINTER_HANDLE {
            Value: std::ptr::null_mut(),
        });
    }
}
