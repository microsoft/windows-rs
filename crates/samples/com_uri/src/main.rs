use windows::core::PCWSTR;
use windows::Win32::System::Com::*;

fn main() -> windows::core::Result<()> {
    unsafe {
        let uri = CreateUri(PCWSTR::from(&"http://kennykerr.ca".into()), URI_CREATE_FLAGS::default(), 0)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{} ({})", domain, port);
        Ok(())
    }
}
