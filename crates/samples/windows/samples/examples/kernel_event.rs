fn main() -> windows::core::Result<()> {
    use windows::{
        Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
        core::Owned,
    };

    unsafe {
        let event = Owned::new(CreateEventW(None, true, false, None)?);
        SetEvent(*event)?;
        WaitForSingleObject(*event, 0);
    }
    Ok(())
}
