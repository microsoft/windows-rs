use windows::core::{s, w};
use windows::{
    Win32::Foundation::*, Win32::Storage::FileSystem::*, Win32::UI::Input::KeyboardAndMouse::*,
    Win32::UI::Shell::*,
};

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
        let a = GetTempFileNameA(s!("."), s!("test"), 0x7b, &mut buffer);
        assert_eq!(a, 0x7b);
        assert_eq!(&buffer[..12], b".\\tes7B.tmp\0");
    }
}

#[test]
fn temp_file_wide() {
    unsafe {
        let mut buffer: [u16; 260] = std::mem::zeroed();
        let a = GetTempFileNameW(w!("."), w!("test"), 0x7b, &mut buffer);
        assert_eq!(a, 0x7b);
        assert_eq!(SysAllocStringLen(Some(&buffer[..12])), ".\\tes7B.tmp\0");
    }
}

#[test]
fn common_prefix() {
    unsafe {
        let count = PathCommonPrefixA(s!("same\\ABC"), s!("same\\DEF"), None);
        assert_eq!(count, 4);

        let mut path = [0; 260];
        let count = PathCommonPrefixA(s!("same\\ABC"), s!("same\\DEF"), Some(&mut path));
        assert_eq!(count, 4);
        let (left, right) = path.split_at(4);
        assert_eq!(left, b"same");
        assert_eq!(right, [0; 260 - 4]);
    }
}
