use bindings::{windows::win32::automation::BSTR, windows::win32::com::CreateUri};

fn main() -> windows::Result<()> {
    unsafe {
        let mut uri = None;
        let uri =
            CreateUri("http://kennykerr.ca", Default::default(), 0, &mut uri).and_some(uri)?;

        let mut domain = BSTR::default();
        uri.GetDomain(&mut domain).ok()?;

        let mut port = 0;
        uri.GetPort(&mut port).ok()?;

        println!("{} ({})", domain, port);
        Ok(())
    }
}
