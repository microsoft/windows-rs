#![cfg(windows)]
use windows::{
    Win32::Foundation::*, Win32::Globalization::*, Win32::Graphics::Gdi::*,
    Win32::Storage::FileSystem::*, Win32::UI::ColorSystem::*,
    Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::Shell::*, core::*,
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

#[test]
fn multi_byte() {
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
        let len = WideCharToMultiByte(CP_UTF8, 0, &b, Some(&mut c), None, None);
        assert_eq!(len, 5);

        assert_eq!(&c, b"hello");
    }
}

#[test]
fn shared_size() -> Result<()> {
    unsafe {
        let mut results: [u8; 2] = [255; 2];
        let dc = GetDC(None);

        assert!(0 != SetICMMode(dc, ICM_ON));

        let input = [
            RGBTRIPLE {
                rgbtBlue: 1,
                rgbtGreen: 2,
                rgbtRed: 3,
            },
            RGBTRIPLE {
                rgbtBlue: 4,
                rgbtGreen: 5,
                rgbtRed: 6,
            },
        ];

        assert_eq!(results[0], 255);
        assert_eq!(results[1], 255);

        CheckColorsInGamut(dc, input.as_ptr(), results.as_mut_ptr() as _, 2).ok()?;

        assert_eq!(results[0], 0);
        assert_eq!(results[1], 0);

        Ok(())
    }
}
