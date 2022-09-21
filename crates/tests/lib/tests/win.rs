use windows::{core::*, Win32::Graphics::Direct3D::Fxc::*, Win32::Graphics::Gdi::*, Win32::System::Threading::*};

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
        AlphaBlend(HDC::default(), 0, 0, 0, 0, HDC::default(), 0, 0, 0, 0, BLENDFUNCTION::default());
    }
}

#[test]
fn wait_on_address() {
    unsafe {
        WaitOnAddress(std::ptr::null(), std::ptr::null(), 0, 0);
    }
}
