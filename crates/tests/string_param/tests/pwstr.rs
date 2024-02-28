use windows::{core::*, Win32::Foundation::*, Win32::UI::Shell::*};

#[test]
fn error() {
    unsafe {
        SetLastError(ERROR_BUSY_DRIVE);

        let utf8 = "test\0".as_bytes();
        let utf16 = HSTRING::from("test\0");
        let utf16 = utf16.as_wide();
        let len = 5;
        assert_eq!(utf8.len(), len);
        assert_eq!(utf16.len(), len);

        assert_eq!(GetLastError(), ERROR_BUSY_DRIVE);
    }
}

#[test]
fn convert() {
    unsafe {
        let pcwstr: PCWSTR = w!("https://github.com/microsoft");
        let pwstr = PWSTR(pcwstr.0 as _);
        let pcstr: PCSTR = s!("https://github.com/microsoft");
        let pstr = PSTR(pcstr.0 as _);

        assert_eq!(0, UrlCompareW(pcwstr, pwstr, true));
        assert_eq!(0, UrlCompareA(pcstr, pstr, true));
    }
}
