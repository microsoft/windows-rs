use super::*;
use bindings::*;

use Windows::Win32::System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA};

pub fn delay_load(library: &str, function: &str) -> RawPtr {
    unsafe {
        let library = LoadLibraryA(library);

        if library.is_invalid() {
            return core::ptr::null_mut();
        }

        if let Some(address) = GetProcAddress(library, function) {
            address as _
        } else {
            FreeLibrary(library);
            core::ptr::null_mut()
        }
    }
}
