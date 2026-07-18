fn main() -> windows::core::Result<()> {
    use windows::core::*;
    use windows::Win32::*;

    unsafe {
        let uri = CreateUri(w!("http://kennykerr.ca"), 0, None)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{domain:?} ({port})");
        Ok(())
    }
}
