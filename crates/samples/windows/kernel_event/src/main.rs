#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() -> windows::core::Result<()> {
    use windows::{
        core::Owned,
        Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
    };

    unsafe {
        let event = Owned::new(CreateEventW(None, true, false, None)?);
        SetEvent(*event)?;
        WaitForSingleObject(*event, 0);
    }
    Ok(())
}
