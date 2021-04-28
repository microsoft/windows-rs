use crate::*;

use bindings::Windows::Win32::System::SystemServices::{FreeLibrary, GetProcAddress, LoadLibraryA};

pub fn delay_load(library: &str, function: &str) -> std::result::Result<RawPtr, HRESULT> {
    unsafe {
        let library = LoadLibraryA(library);

        if library.is_null() {
            return Err(HRESULT::from_thread());
        }

        if let Some(address) = GetProcAddress(library, function) {
            Ok(address as _)
        } else {
            FreeLibrary(library);
            Err(HRESULT::from_thread())
        }
    }
}
