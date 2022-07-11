use windows::{
    core::PCWSTR,
    Win32::Foundation::{CloseHandle, WAIT_OBJECT_0},
    Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
};

fn main() -> windows::core::Result<()> {
    unsafe {
        let event = CreateEventW(std::ptr::null(), true, false, PCWSTR::default())?;

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == WAIT_OBJECT_0.0);

        CloseHandle(event).ok()
    }
}
