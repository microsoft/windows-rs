use windows::Win32::System::Com::*;

fn main() -> windows::core::Result<()> {
    unsafe {
        let uri = CreateUri(pcwstr(&"http://kennykerr.ca".into()), URI_CREATE_FLAGS::default(), 0)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{} ({})", domain, port);
        Ok(())
    }
}

fn pcwstr(s: &windows::core::HSTRING) -> windows::core::PCWSTR {
    windows::core::PCWSTR(s.as_wide().as_ptr())
}
