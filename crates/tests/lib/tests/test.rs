#[test]
fn linker() -> windows::core::Result<()> {
    unsafe {
        windows::Win32::Graphics::Direct3D::Fxc::D3DCreateLinker()?;
        Ok(())
    }
}

#[test]
fn gdi() {
    use windows::Win32::Graphics::Gdi::*;
    unsafe {
        AlphaBlend(HDC::default(), 0, 0, 0, 0, HDC::default(), 0, 0, 0, 0, BLENDFUNCTION::default());
    }
}
