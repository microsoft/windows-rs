#![cfg(windows)]
use windows::{Win32::pathcch::*, Win32::urlmon::*, Win32::winstring::*, core::*};

#[test]
fn path() -> Result<()> {
    unsafe {
        let extension: PCWSTR = PathCchFindExtension(w!("A:\\file.txt"), 12)?;
        assert_eq!(extension.to_string()?, ".txt");
        Ok(())
    }
}

#[test]
fn hstring() -> Result<()> {
    unsafe {
        let raw: PCWSTR = WindowsGetStringRawBuffer(h!("test"), None);
        assert_eq!(raw.to_string()?, "test");
        Ok(())
    }
}

#[test]
fn uri() -> Result<()> {
    unsafe {
        let uri = CreateUri(w!("http://kennykerr.ca"), 0, None)?;
        let builder = CreateIUriBuilder(&uri, 0, 0)?;

        let mut host_len = 0u32;
        let mut host = PCWSTR::null();
        builder.GetHost(&mut host_len, &mut host).ok()?;
        assert_eq!(host_len, 12);
        assert_eq!(host.to_string()?, "kennykerr.ca");

        Ok(())
    }
}
