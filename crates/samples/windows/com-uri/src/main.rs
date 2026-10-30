fn main() -> windows::core::Result<()> {
    use windows::Win32::*;
    use windows::core::*;

    unsafe {
        let mut uri = None;
        CreateUri(
            w!("https://github.com/microsoft/windows-rs"),
            0,
            None,
            &mut uri,
        )
        .ok()?;
        let uri = uri.unwrap();

        let mut domain = BSTR::new();
        uri.GetDomain(&mut domain).ok()?;
        let mut port = 0;
        uri.GetPort(&mut port).ok()?;

        println!("{domain:?} ({port})");
        Ok(())
    }
}
