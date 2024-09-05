use windows_sys::{
    core::*, Win32::System::Registry::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

/// Tests a few APIs that have reserved parameters to ensure they can be called with `None`.
#[test]
fn test() {
    unsafe {
        assert_eq!(InSendMessageEx(std::ptr::null_mut()), ISMEX_NOSEND);
        assert!(CreateThreadpool(std::ptr::null_mut()) != 0);
        assert_eq!(
            TrackPopupMenu(
                core::ptr::null_mut(),
                TPM_LEFTBUTTON,
                1,
                2,
                0,
                core::ptr::null_mut(),
                std::ptr::null()
            ),
            0
        );

        let mut key = core::ptr::null_mut();
        RegOpenKeyExA(HKEY_CLASSES_ROOT, s!(r".txt"), 0, KEY_QUERY_VALUE, &mut key);
        let mut len = 0;
        RegQueryValueExA(
            key,
            s!("Content Type"),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut len,
        );
        let mut buffer = vec![0u8; (len) as usize];
        RegQueryValueExA(
            key,
            s!("Content Type"),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            buffer.as_mut_ptr() as _,
            &mut len,
        );
        assert_eq!(String::from_utf8_lossy(&buffer), "text/plain\0");
    }
}
