fn main() -> windows::core::Result<()> {
    use windows::Win32::urlmon::*;
    use windows::core::*;

    unsafe {
        let uri = CreateUri(w!("http://kennykerr.ca"), 0, None)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{domain:?} ({port})");
        Ok(())
    }
}
