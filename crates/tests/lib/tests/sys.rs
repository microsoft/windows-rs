use windows_sys::{Win32::Graphics::Gdi::*, Win32::System::Threading::*, Win32::Web::InternetExplorer::*};

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

#[test]
fn browser() {
    unsafe {
        IECreateFile(std::ptr::null(), 0, 0, std::ptr::null(), 0, 0, 0);
    }
}
