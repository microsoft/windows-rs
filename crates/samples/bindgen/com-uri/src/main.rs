#[expect(nonstandard_style, clippy::upper_case_acronyms)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() -> windows_core::Result<()> {
    use bindings::*;
    use windows_core::*;

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
