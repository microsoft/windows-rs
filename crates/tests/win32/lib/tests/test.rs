#[test]
fn linker() -> windows::core::Result<()> {
    unsafe {
        windows::Win32::Graphics::Direct3D::Fxc::D3DCreateLinker()?;
        Ok(())
    }
}
