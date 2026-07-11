#![cfg(windows)]
use windows::{
    core::*,
    minwindef::HKEY,
    threadpoolapiset::CreateThreadpool,
    winnt::{ACCESS_MASK, KEY_QUERY_VALUE},
    winreg::{HKEY_CLASSES_ROOT, RegOpenKeyExA, RegQueryValueExA},
    winuser::*,
};

/// Tests a few APIs that have reserved parameters to ensure they can be called with `None`.
#[test]
fn test() -> Result<()> {
    unsafe {
        assert_eq!(InSendMessageEx(None), ISMEX_NOSEND);
        assert!(!CreateThreadpool(None).is_null());

        assert_eq!(
            TrackPopupMenu(
                Default::default(),
                TPM_LEFTBUTTON,
                1,
                2,
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            BOOL(0)
        );

        let mut key = HKEY::default();
        WIN32_ERROR(
            RegOpenKeyExA(
                HKEY_CLASSES_ROOT,
                s!(r".txt"),
                None,
                ACCESS_MASK(KEY_QUERY_VALUE),
                &mut key,
            )
            .0,
        )
        .ok()?;
        let mut len = 0;
        WIN32_ERROR(RegQueryValueExA(key, s!("Content Type"), None, None, None, &mut len).0)
            .ok()?;
        let mut buffer = vec![0u8; (len) as usize];
        WIN32_ERROR(
            RegQueryValueExA(
                key,
                s!("Content Type"),
                None,
                None,
                Some(buffer.as_mut_ptr() as _),
                &mut len,
            )
            .0,
        )
        .ok()?;
        assert_eq!(String::from_utf8_lossy(&buffer), "text/plain\0");
        Ok(())
    }
}
