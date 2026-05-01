#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() -> windows::core::Result<()> {
    use windows::core::*;
    use windows::Win32::System::Com::*;

    unsafe {
        let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), None)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{:?} ({port})", domain);
        Ok(())
    }
}
