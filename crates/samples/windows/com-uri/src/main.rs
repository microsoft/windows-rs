fn main() -> windows::core::Result<()> {
    use windows::Win32::*;
    use windows::core::*;

    unsafe {
        let uri = CreateUri(w!("https://github.com/microsoft/windows-rs"), 0, None)?;

        let mut domain = BSTR::new();
        uri.GetDomain(&mut domain).ok()?;
        let mut port = 0;
        uri.GetPort(&mut port).ok()?;

        println!("{domain:?} ({port})");
        Ok(())
    }
}
