use crate::*;

use bindings::windows::win32::system_services::{
    FreeLibrary, GetProcAddress, LoadLibraryExW, LoadLibraryEx_dwFlags, HANDLE,
};

pub fn delay_load(
    library: &str,
    function: &str,
    flags: u32,
) -> std::result::Result<RawPtr, ErrorCode> {
    unsafe {
        // TODO: https://github.com/microsoft/win32metadata/issues/331
        let library = LoadLibraryExW(library, HANDLE(0), LoadLibraryEx_dwFlags(flags));

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
