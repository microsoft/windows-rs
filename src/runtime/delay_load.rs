use crate::*;
use bindings::*;

use Windows::Win32::System::Diagnostics::Debug::GetLastError;
use Windows::Win32::System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA};

pub fn delay_load(library: &str, function: &str) -> std::result::Result<RawPtr, HRESULT> {
    unsafe {
        let library = LoadLibraryA(library);

        if library.is_invalid() {
            return Err(GetLastError().into());
        }

        if let Some(address) = GetProcAddress(library, function) {
            Ok(address as _)
        } else {
            FreeLibrary(library);
            Err(GetLastError().into())
        }
    }
}
