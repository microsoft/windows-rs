#![cfg(windows)]
use windows::core::{PSTR, PWSTR, s, w};
use windows::{fileapi::*, oleauto::*, shlwapi::*, winuser::*, wtypesbase::OLECHAR};

#[test]
fn keyboard_state() {
    unsafe {
        let zeroed: [u8; 256] = std::mem::zeroed();
        let mut state: [u8; 256] = std::mem::zeroed();
        GetKeyboardState(&mut state).unwrap();
        assert!(state != zeroed);
        SetKeyboardState(&state).unwrap();
    }
}

#[test]
fn temp_file_ansi() {
    unsafe {
        let mut buffer: [u8; 260] = std::mem::zeroed();
        let a = GetTempFileNameA(s!("."), s!("test"), 0x7b, PSTR(buffer.as_mut_ptr()));
        assert_eq!(a, 0x7b);
        assert_eq!(&buffer[..12], b".\\tes7B.tmp\0");
    }
}

#[test]
fn temp_file_wide() {
    unsafe {
        let mut buffer: [u16; 260] = std::mem::zeroed();
        let a = GetTempFileNameW(w!("."), w!("test"), 0x7b, PWSTR(buffer.as_mut_ptr()));
        assert_eq!(a, 0x7b);
        let buffer = std::slice::from_raw_parts(buffer.as_ptr() as *const OLECHAR, 12);
        assert_eq!(SysAllocStringLen(Some(buffer)), ".\\tes7B.tmp\0");
    }
}

#[test]
fn common_prefix() {
    unsafe {
        let count = PathCommonPrefixA(s!("same\\ABC"), s!("same\\DEF"), None);
        assert_eq!(count, 4);

        let mut path = [0; 260];
        let count = PathCommonPrefixA(
            s!("same\\ABC"),
            s!("same\\DEF"),
            Some(PSTR(path.as_mut_ptr())),
        );
        assert_eq!(count, 4);
        let (left, right) = path.split_at(4);
        assert_eq!(left, b"same");
        assert_eq!(right, [0; 260 - 4]);
    }
}
