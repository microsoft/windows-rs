#![cfg(windows)]
use windows::{
    Wdk::System::OfflineRegistry::*, Win32::Foundation::*, Win32::Graphics::Direct3D::Fxc::*,
    Win32::Graphics::Gdi::*, Win32::Graphics::Printing::*, Win32::System::Threading::*, core::*,
};

#[test]
fn linker() -> Result<()> {
    unsafe {
        let linker = D3DCreateLinker()?;
        assert!(!linker.as_raw().is_null());
        Ok(())
    }
}

#[test]
fn gdi() {
    unsafe {
        _ = AlphaBlend(
            HDC::default(),
            0,
            0,
            0,
            0,
            HDC::default(),
            0,
            0,
            0,
            0,
            BLENDFUNCTION::default(),
        );
    }
}

#[test]
fn wait_on_address() {
    unsafe {
        WaitOnAddress(std::ptr::null(), std::ptr::null(), 0, None).unwrap_err();
    }
}

// Validates that the target libs resolve this function to "winspool.drv"
#[test]
fn not_dll() {
    unsafe {
        _ = GetSpoolFileHandle(PRINTER_HANDLE::default());
    }
}

#[test]
fn wdk() {
    unsafe {
        let mut hive = ORHKEY::default();
        assert_eq!(ORCreateHive(&mut hive), ERROR_SUCCESS);
        assert_eq!(ORCloseHive(hive), ERROR_SUCCESS);
    }
}
