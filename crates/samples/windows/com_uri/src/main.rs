#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
mod imp {
    use windows::core::*;
    use windows::Win32::System::Com::*;

    pub fn main() -> windows::core::Result<()> {
        unsafe {
            let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), None)?;

            let domain = uri.GetDomain()?;
            let port = uri.GetPort()?;

            println!("{:?} ({port})", domain);
            Ok(())
        }
    }
}

#[cfg(windows)]
fn main() -> impl std::process::Termination {
    imp::main()
}
