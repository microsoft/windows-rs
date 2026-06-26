fn main() -> windows::core::Result<()> {
    use windows::Win32::System::Com::*;
    use windows::core::*;

    unsafe {
        let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), None)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{domain:?} ({port})");
        Ok(())
    }
}
