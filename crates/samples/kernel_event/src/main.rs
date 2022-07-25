use windows::{
    Win32::Foundation::{CloseHandle, WAIT_OBJECT_0},
    Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
};

fn main() -> windows::core::Result<()> {
    unsafe {
        let event = CreateEventW(None, true, false, None)?;

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == WAIT_OBJECT_0.0);

        CloseHandle(event).ok()
    }
}
