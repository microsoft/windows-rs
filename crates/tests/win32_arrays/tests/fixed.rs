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
        let a = GetTempFileNameA(".", "test", 0x7b, &mut buffer);
        assert_eq!(a, 0x7b);
        assert_eq!(&buffer[..12], b".\\tes7B.tmp\0");
    }
}

#[test]
fn temp_file_wide() {
    unsafe {
        let mut buffer: [u16; 260] = std::mem::zeroed();
        let a = GetTempFileNameW(".", "test", 0x7b, &mut buffer);
        assert_eq!(a, 0x7b);
        assert_eq!(SysAllocStringLen(&buffer[..12]), ".\\tes7B.tmp\0");
    }
}
