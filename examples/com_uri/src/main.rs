use bindings::Windows::Win32::System::Com::CreateUri;

fn main() -> windows::Result<()> {
    unsafe {
        // TODO: need to apply retval to functions too
        let mut uri = None;
        let uri =
            CreateUri("http://kennykerr.ca", Default::default(), 0, &mut uri).and_some(uri)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{} ({})", domain, port);
        Ok(())
    }
}
