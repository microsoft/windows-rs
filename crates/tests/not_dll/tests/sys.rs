use windows_sys::Win32::Graphics::Printing::*;

#[test]
fn test() {
    unsafe {
        GetSpoolFileHandle(0);
    }
}
