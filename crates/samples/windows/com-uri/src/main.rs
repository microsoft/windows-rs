fn main() -> windows::core::Result<()> {
    use windows::Win32::*;
    use windows::core::*;

    unsafe {
        let uri = CreateUri(w!("https://github.com/microsoft/windows-rs"), 0, None)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{domain:?} ({port})");
        Ok(())
    }
}
