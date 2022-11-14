use windows::{core::*, Win32::Graphics::Direct2D::*, Win32::System::LibraryLoader::*};

#[test]
fn test() {
    unsafe {
        assert!(GetModuleHandleA(s!("d2d1.dll")).is_err());

        assert_eq!(0.0, D2D1Tan(0.0));

        assert!(GetModuleHandleA(s!("d2d1.dll")).is_ok());
    }
}
