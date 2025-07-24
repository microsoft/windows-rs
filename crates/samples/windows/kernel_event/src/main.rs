use windows::{
    core::{Owned, HRESULT},
    Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
};

fn main() -> Result<(), HRESULT> {
    unsafe {
        let event = Owned::new(CreateEventW(None, true, false, None)?);
        SetEvent(*event)?;
        WaitForSingleObject(*event, 0);
    }
    Ok(())
}
