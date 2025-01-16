use windows_sys::{
    core::*, Win32::Graphics::Gdi::*, Win32::System::ProcessStatus::*, Win32::System::Threading::*,
    Win32::Web::InternetExplorer::*,
};

#[test]
fn gdi() {
    unsafe {
        AlphaBlend(
            core::ptr::null_mut(),
            0,
            0,
            0,
            0,
            core::ptr::null_mut(),
            0,
            0,
            0,
            0,
            std::mem::zeroed(),
        );
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
        IECreateFile(
            std::ptr::null(),
            0,
            0,
            std::ptr::null(),
            0,
            0,
            core::ptr::null_mut(),
        );
    }
}

// This test is for https://github.com/microsoft/windows-rs/issues/2410
#[test]
fn psapi() {
    windows_link::link!("kernel32.dll" "system" fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> BOOL);

    unsafe {
        assert_eq!(
            0,
            K32EnumProcesses(std::ptr::null_mut(), 0, std::ptr::null_mut())
        );

        assert_eq!(
            0,
            EnumProcesses(std::ptr::null_mut(), 0, std::ptr::null_mut())
        );
    }
}
