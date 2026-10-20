#![cfg(windows)]
use windows::{Win32::*, core::*};

#[test]
fn linker() -> Result<()> {
    unsafe {
        D3DCreateLinker()?;
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
        WaitOnAddress(std::ptr::null(), std::ptr::null(), 0, None)
            .ok()
            .unwrap_err();
    }
}

// Validates that the target libs resolve this function to "winspool.drv"
#[test]
fn not_dll() {
    unsafe {
        _ = GetSpoolFileHandle(HANDLE::default());
    }
}

#[test]
fn wdk() {
    unsafe {
        let mut hive: ORHKEY = core::ptr::null_mut();
        assert_eq!(ORCreateHive(&mut hive), 0);
        assert_eq!(ORCloseHive(hive), 0);
    }
}
