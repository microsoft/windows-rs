use windows_sys::{core::*, Win32::Foundation::*, Win32::UI::Shell::*};

unsafe extern "C" {
    fn wcslen(s: PCWSTR) -> usize;
}

unsafe fn to_string(s: PCWSTR) -> String {
    unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(s, wcslen(s))) }
}

#[test]
fn path() {
    unsafe {
        let mut extension: PCWSTR = std::ptr::null();
        assert_eq!(
            PathCchFindExtension(w!("A:\\file.txt"), 12, &mut extension),
            S_OK
        );
        assert_eq!(to_string(extension), ".txt");
    }
}
