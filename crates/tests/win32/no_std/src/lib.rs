#![no_std]

#[test]
pub fn main() -> windows::runtime::Result<()> {
    use windows::{
        Win32::Foundation::CloseHandle,
        Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject, WAIT_OBJECT_0},
    };

    unsafe {
        let event = CreateEventW(core::ptr::null_mut(), true, false, None);

        assert!(event.0 != 0);

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == WAIT_OBJECT_0);

        CloseHandle(event).ok()
    }
}
