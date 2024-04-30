use windows::{
    core::Owned,
    Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
};

fn main() -> windows::core::Result<()> {
    unsafe {
        let event = Owned::new(CreateEventW(None, true, false, None)?);
        SetEvent(*event)?;
        WaitForSingleObject(*event, 0);
    }
    Ok(())
}
