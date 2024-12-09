use windows::{
    core::*, Win32::Graphics::Direct3D::Fxc::*, Win32::Graphics::Gdi::*,
    Win32::System::ClrHosting::*, Win32::System::Threading::*,
};

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
        WaitOnAddress(std::ptr::null(), std::ptr::null(), 0, None).unwrap_err();
    }
}

#[test]
fn clr() -> Result<()> {
    unsafe {
        let mut version = vec![0; 20];
        let mut len = 0;
        GetFileVersion(
            w!("../../../libs/bindgen/default/Windows.winmd"),
            Some(&mut version),
            &mut len,
        )?;
        let version = String::from_utf16_lossy(&version[..len as usize - 1]);
        assert_eq!(version, "WindowsRuntime 1.4");
        Ok(())
    }
}
