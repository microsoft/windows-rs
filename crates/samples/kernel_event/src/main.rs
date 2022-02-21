use windows::{
    Win32::Foundation::CloseHandle,
    Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject, WAIT_OBJECT_0},
};

fn main() -> windows::core::Result<()> {
    unsafe {
        let event = CreateEventW(std::ptr::null(), true, false, None);

        assert!(event.0 != 0);

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == WAIT_OBJECT_0);

        CloseHandle(event).ok()
    }
}
