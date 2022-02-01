use windows::{Win32::Foundation::*, Win32::Graphics::Printing::*};

#[test]
fn test() {
    unsafe {
        GetSpoolFileHandle(HANDLE(0));
    }
}
