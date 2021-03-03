use bindings::{
    windows::win32::system_services::{CreateEventW, SetEvent, WaitForSingleObject, PWSTR},
    windows::win32::windows_programming::CloseHandle,
};

fn main() -> windows::Result<()> {
    unsafe {
        let event = CreateEventW(
            std::ptr::null_mut(),
            true,
            false,
            PWSTR::default(),
        );

        assert!(event.0 != 0);

        SetEvent(event).ok()?;

        let result = WaitForSingleObject(event, 0);
        assert!(result == 0); // https://github.com/microsoft/win32metadata/issues/77

        CloseHandle(event).ok()
    }
}
