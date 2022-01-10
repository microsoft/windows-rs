use super::*;
use bindings::*;

// TODO: return Option to make calling this function easier
pub fn delay_load(library: &[u8], function: &[u8]) -> RawPtr {
    unsafe {
        let library = LoadLibraryA(PSTR(library.as_ptr() as *mut _));

        if library.is_invalid() {
            return core::ptr::null_mut();
        }

        if let Some(address) = GetProcAddress(library, PSTR(function.as_ptr() as *mut _)) {
            address as _
        } else {
            FreeLibrary(library);
            core::ptr::null_mut()
        }
    }
}
