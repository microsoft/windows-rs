use crate::*;

use bindings::windows::win32::system_services::{
    FreeLibrary, GetProcAddress, LoadLibraryExW, HANDLE,
};

pub fn delay_load(
    library: &str,
    function: &str,
    flags: u32,
) -> std::result::Result<RawPtr, ErrorCode> {
    unsafe {
        let library = LoadLibraryExW(library, HANDLE(0), flags);

        if library == 0 {
            return Err(ErrorCode::from_thread());
        }

        if let Some(address) = GetProcAddress(library, function) {
            Ok(std::mem::transmute(address))
        } else {
            FreeLibrary(library);
            Err(ErrorCode::from_thread())
        }
    }
}
