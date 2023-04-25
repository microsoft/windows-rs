use windows::{core::*, Win32::Foundation::*};

#[test]
fn test() {
    unsafe {
        SetLastError(ERROR_BUSY_DRIVE);

        let utf8 = "test\0".as_bytes();
        let utf16 = HSTRING::from("test\0");
        let utf16 = utf16.as_wide();
        let len = 5;
        assert_eq!(utf8.len(), len);
        assert_eq!(utf16.len(), len);

        assert_eq!(GetLastError().unwrap_err().code(), ERROR_BUSY_DRIVE.into());
    }
}
