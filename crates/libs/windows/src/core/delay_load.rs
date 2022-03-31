use super::*;
use bindings::*;

// TODO: return Option to make calling this function easier
pub fn delay_load(library: &[u8], function: &[u8]) -> RawPtr {
    unsafe {
        let library = LoadLibraryA(PCSTR(library.as_ptr()));

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/863
        if library.0 == 0 {
            return core::ptr::null_mut();
        }

        if let Some(address) = GetProcAddress(library, PCSTR(function.as_ptr())) {
            address as _
        } else {
            FreeLibrary(library);
            core::ptr::null_mut()
        }
    }
}
