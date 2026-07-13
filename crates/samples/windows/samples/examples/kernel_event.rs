fn main() -> windows::core::Result<()> {
    use windows::{
        core::*,
        handleapi::CloseHandle,
        synchapi::{CreateEventW, SetEvent, WaitForSingleObject},
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
