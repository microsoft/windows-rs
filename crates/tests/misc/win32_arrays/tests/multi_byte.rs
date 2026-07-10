#![cfg(windows)]
use windows::Win32::{stringapiset::*, winnls::*};

#[test]
fn test() {
    unsafe {
        let a = b"hello";
        let mut b: [u16; 5] = [0xFFFF; 5];
        let len = MultiByteToWideChar(CP_UTF8, 0, a.as_ptr() as _, a.len() as i32, Some(&mut b));
        assert_eq!(len, 5);

        let mut c: [u8; 5] = [0xFF; 5];
        let len = WideCharToMultiByte(
            CP_UTF8,
            0,
            b.as_ptr(),
            b.len() as i32,
            Some(&mut c),
            None,
            None,
        );
        assert_eq!(len, 5);

        assert_eq!(&c, b"hello");
    }
}
