use windows::core::{HSTRING, PCSTR, PCWSTR};
use windows::{Win32::Foundation::*, Win32::Storage::FileSystem::*, Win32::UI::Input::KeyboardAndMouse::*};

#[test]
fn keyboard_state() {
    unsafe {
        let zeroed: [u8; 256] = std::mem::zeroed();
        let mut state: [u8; 256] = std::mem::zeroed();
        assert!(GetKeyboardState(&mut state).as_bool());
        assert!(state != zeroed);
        assert!(SetKeyboardState(&state).as_bool());
    }
}

#[test]
fn temp_file_ansi() {
    unsafe {
        let mut buffer: [u8; 260] = std::mem::zeroed();
        let dot = &mut String::from(".");
        let test = &mut String::from("test");
        let a = GetTempFileNameA(pcstr(dot), pcstr(test), 0x7b, &mut buffer);
        assert_eq!(a, 0x7b);
        assert_eq!(&buffer[..12], b".\\tes7B.tmp\0");
    }
}

#[test]
fn temp_file_wide() {
    unsafe {
        let mut buffer: [u16; 260] = std::mem::zeroed();
        let dot = &HSTRING::from(".");
        let test = &HSTRING::from("test");
        let a = GetTempFileNameW(pcwstr(dot), pcwstr(test), 0x7b, &mut buffer);
        assert_eq!(a, 0x7b);
        assert_eq!(SysAllocStringLen(&buffer[..12]), ".\\tes7B.tmp\0");
    }
}

fn pcstr(s: &mut String) -> PCSTR {
    s.push('\0');
    PCSTR(s.as_bytes().as_ptr())
}

fn pcwstr(s: &HSTRING) -> PCWSTR {
    PCWSTR(s.as_wide().as_ptr())
}
