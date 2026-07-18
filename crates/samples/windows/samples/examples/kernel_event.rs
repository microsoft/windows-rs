fn main() -> windows::core::Result<()> {
    use windows::{
        Win32::CloseHandle,
        Win32::{CreateEventW, SetEvent, WaitForSingleObject},
        core::*,
    };

    unsafe {
        let event = CreateEventW(None, true, false, None);
        if event.0.is_null() {
            return Err(Error::from_thread());
        }

        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;
    }
    Ok(())
}
