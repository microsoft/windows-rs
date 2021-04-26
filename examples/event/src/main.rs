use bindings::{
    Windows::Win32::SystemServices::{CreateEventW, SetEvent, WaitForSingleObject, WAIT_OBJECT_0},
    Windows::Win32::WindowsProgramming::CloseHandle,
};

fn main() -> windows::Result<()> {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, None);

        assert!(event.0 != 0);

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == WAIT_OBJECT_0);

        CloseHandle(event).ok()
    }
}
