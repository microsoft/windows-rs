use crate::*;

use bindings::Windows::Win32::SystemServices::{FreeLibrary, GetProcAddress, LoadLibraryA};

pub fn delay_load(library: &str, function: &str) -> std::result::Result<RawPtr, ErrorCode> {
    unsafe {
        let library = LoadLibraryA(library);

        if library == 0 {
            return Err(ErrorCode::from_thread());
        }

        if let Some(address) = GetProcAddress(library, function) {
            Ok(address as _)
        } else {
            FreeLibrary(library);
            Err(ErrorCode::from_thread())
        }
    }
}
