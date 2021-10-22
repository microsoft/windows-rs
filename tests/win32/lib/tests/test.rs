use test_win32_lib::*;

#[test]
fn linker() -> windows::runtime::Result<()> {
    unsafe {
        Windows::Win32::Graphics::Hlsl::D3DCreateLinker()?;
        Ok(())
    }
}
