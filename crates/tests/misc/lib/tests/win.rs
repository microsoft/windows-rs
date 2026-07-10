#![cfg(windows)]
use windows::{core::*, d3dcompiler::*, synchapi::*, windef::*, wingdi::*};

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
