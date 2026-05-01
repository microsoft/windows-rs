#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
mod imp {
    use windows::{
        core::Owned,
        Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
    };

    pub fn main() -> windows::core::Result<()> {
        unsafe {
            let event = Owned::new(CreateEventW(None, true, false, None)?);
            SetEvent(*event)?;
            WaitForSingleObject(*event, 0);
        }
        Ok(())
    }
}

#[cfg(windows)]
fn main() -> impl std::process::Termination {
    imp::main()
}
