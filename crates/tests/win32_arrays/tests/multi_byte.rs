use windows::{core::*, Win32::Globalization::*};

#[test]
fn test() {
    unsafe {
        let a = b"hello";
        let mut b: [u16; 5] = [0xFFFF; 5];
        let len = MultiByteToWideChar(CP_UTF8, Default::default(), a, &mut b);
        assert_eq!(len, 5);

        let mut c: [u8; 5] = [0xFF; 5];
        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/820
        let len = WideCharToMultiByte(CP_UTF8, Default::default(), &b, PSTR(c.as_mut_ptr()), c.len() as _, None, std::ptr::null_mut());
        assert_eq!(len, 5);

        assert_eq!(&c, b"hello");
    }
}
