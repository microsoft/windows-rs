use windows::core::*;
use windows::Win32::System::Com::*;

fn main() -> windows::core::Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        
        let uri = CreateUri(w!("http://kennykerr.ca"), URI_CREATE_FLAGS::default(), 0)?;

        let domain = uri.GetDomain()?;
        let port = uri.GetPort()?;

        println!("{domain} ({port})");
        Ok(())
    }
}
