use windows_sys::{core::*, Win32::Graphics::Direct2D::*, Win32::System::LibraryLoader::*};

#[test]
fn test() {
    unsafe {
        assert_eq!(0, GetModuleHandleA(s!("d2d1.dll")));

        assert_eq!(0.0, D2D1Tan(0.0));

        assert_ne!(0, GetModuleHandleA(s!("d2d1.dll")));
    }
}
