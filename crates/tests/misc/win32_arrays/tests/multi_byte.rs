use windows::Win32::Globalization::*;

#[test]
fn test() {
    unsafe {
        let a = b"hello";
        let mut b: [u16; 5] = [0xFFFF; 5];
        let len = MultiByteToWideChar(
            CP_UTF8,
            MULTI_BYTE_TO_WIDE_CHAR_FLAGS::default(),
            a,
            Some(&mut b),
        );
        assert_eq!(len, 5);

        let mut c: [u8; 5] = [0xFF; 5];
        let len = WideCharToMultiByte(CP_UTF8, Default::default(), &b, Some(&mut c), None, None);
        assert_eq!(len, 5);

        assert_eq!(&c, b"hello");
    }
}
