use windows_sys::{Win32::Graphics::Direct3D::Fxc::*, Win32::Graphics::Gdi::*, Win32::System::Threading::*};

#[test]
fn linker() {
    unsafe {
        D3DCreateLinker(std::ptr::null_mut());
    }
}

#[test]
fn gdi() {
    unsafe {
        AlphaBlend(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, std::mem::zeroed());
    }
}

#[test]
fn wait_on_address() {
    unsafe {
        WaitOnAddress(std::ptr::null(), std::ptr::null(), 0, 0);
    }
}
