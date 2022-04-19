use super::*;
use bindings::*;

pub fn delay_load(library: &[u8], function: &[u8]) -> Result<RawPtr> {
    unsafe {
        let library = LoadLibraryA(PCSTR(library.as_ptr()))?;

        if let Some(address) = GetProcAddress(library, PCSTR(function.as_ptr())) {
            Ok(address as _)
        } else {
            FreeLibrary(library);
            Err(Error::from_win32())
        }
    }
}
