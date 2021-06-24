use bindings::Windows::Win32::System::Com::CreateUri;

fn main() -> windows::Result<()> {
    unsafe {
        let uri =
            CreateUri("http://kennykerr.ca", Default::default(), 0)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{} ({})", domain, port);
        Ok(())
    }
}
