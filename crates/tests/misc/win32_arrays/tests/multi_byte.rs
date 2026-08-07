#![cfg(windows)]
use windows::Win32::*;
use windows::core::{PSTR, PWSTR};

#[test]
fn test() {
    unsafe {
        let a = b"hello";
        let mut b: [u16; 5] = [0xFFFF; 5];
        let len = MultiByteToWideChar(
            CP_UTF8 as u32,
            0,
            a.as_ptr() as _,
            a.len() as i32,
            Some(PWSTR(b.as_mut_ptr())),
            b.len() as i32,
        );
        assert_eq!(len, 5);

        let mut c: [u8; 5] = [0xFF; 5];
        let len = WideCharToMultiByte(
            CP_UTF8 as u32,
            0,
            b.as_ptr(),
            b.len() as i32,
            Some(PSTR(c.as_mut_ptr())),
            c.len() as i32,
            None,
            None,
        );
        assert_eq!(len, 5);

        assert_eq!(&c, b"hello");
    }
}
