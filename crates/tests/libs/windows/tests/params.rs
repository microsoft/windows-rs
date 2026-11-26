#![cfg(windows)]
use windows::{
    Win32::System::Com::StructuredStorage::*, Win32::System::Com::*,
    Win32::System::Diagnostics::Debug::Extensions::*, Win32::System::Registry::*,
    Win32::System::Threading::*, Win32::System::WinRT::*, Win32::UI::Shell::*,
    Win32::UI::WindowsAndMessaging::*, core::*,
};

#[test]
fn const_path() -> Result<()> {
    unsafe {
        let extension = PathCchFindExtension(w!("A:\\file.txt"), 12)?;
        assert_eq!(extension.to_string()?, ".txt");
        Ok(())
    }
}

#[test]
fn const_hstring() -> Result<()> {
    unsafe {
        let raw: PCWSTR = WindowsGetStringRawBuffer(h!("test"), None);
        assert_eq!(raw.to_string()?, "test");
        Ok(())
    }
}

#[test]
fn const_uri() -> Result<()> {
    unsafe {
        let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), None)?;
        let builder = CreateIUriBuilder(&uri, 0, 0)?;

        let mut host_len = 0u32;
        let mut host = PCWSTR::null();
        builder.GetHost(&mut host_len, &mut host)?;
        assert_eq!(host_len, 12);
        assert_eq!(host.to_string()?, "kennykerr.ca");

        Ok(())
    }
}

#[test]
fn const_ptr_path() {
    unsafe {
        let result = PathFindSuffixArrayA(s!("path.txt"), &[s!("doc"), s!("txt"), s!("rs")]);
        assert_eq!(result.to_string().unwrap(), "txt");

        let result = PathFindSuffixArrayW(w!("path.rs"), &[w!("doc"), w!("txt"), w!("rs")]);
        assert_eq!(result.to_string().unwrap(), "rs");
    }
}

#[test]
fn const_ptr_variant() -> Result<()> {
    unsafe {
        let variant = InitPropVariantFromStringVector(Some(&[w!("first"), w!("second")]))?;
        let result = PropVariantToBSTR(&variant)?;
        assert_eq!(result, "first; second");
        Ok(())
    }
}

/// Tests a few APIs that have reserved parameters to ensure they can be called with `None`.
#[test]
fn reserved() -> Result<()> {
    unsafe {
        assert_eq!(InSendMessageEx(None), ISMEX_NOSEND);
        assert!(!CreateThreadpool(None)?.is_invalid());

        assert_eq!(
            TrackPopupMenu(
                Default::default(),
                TPM_LEFTBUTTON,
                1,
                2,
                None,
                Default::default(),
                None,
            ),
            BOOL(0)
        );

        let mut key = HKEY::default();
        RegOpenKeyExA(
            HKEY_CLASSES_ROOT,
            s!(r".txt"),
            None,
            KEY_QUERY_VALUE,
            &mut key,
        )
        .ok()?;
        let mut len = 0;
        RegQueryValueExA(key, s!("Content Type"), None, None, None, Some(&mut len)).ok()?;
        let mut buffer = vec![0u8; (len) as usize];
        RegQueryValueExA(
            key,
            s!("Content Type"),
            None,
            None,
            Some(buffer.as_mut_ptr() as _),
            Some(&mut len),
        )
        .ok()?;
        assert_eq!(String::from_utf8_lossy(&buffer), "text/plain\0");
        Ok(())
    }
}

#[test]
fn query_signature() {
    unsafe {
        let _debug: IDebugClient = DebugCreateEx(0).unwrap();
    }
}
