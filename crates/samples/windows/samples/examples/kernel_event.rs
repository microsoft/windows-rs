fn main() -> windows::core::Result<()> {
    use windows::{
        Win32::{
            handleapi::CloseHandle,
            synchapi::{CreateEventW, SetEvent, WaitForSingleObject},
        },
        core::*,
    };

    unsafe {
        let event = CreateEventW(None, true, false, None);
        if event.is_invalid() {
            return Err(Error::from_thread());
        }

        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;
    }
    Ok(())
}
