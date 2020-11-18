use crate::*;

pub fn delay_load(
    library: &str,
    function: &str,
    flags: u32,
) -> std::result::Result<RawPtr, ErrorCode> {
    let library = library
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let mut terminated_function = String::with_capacity(function.len() + 1);
    terminated_function.push_str(function);
    terminated_function.push('\0');

    let library = unsafe { LoadLibraryExW(library.as_ptr(), std::ptr::null_mut(), flags) };

    if library.is_null() {
        return Err(ErrorCode::from_thread());
    }

    let address = unsafe { GetProcAddress(library, terminated_function.as_ptr()) };

    if address.is_null() {
        unsafe {
            FreeLibrary(library);
        }

        return Err(ErrorCode::from_thread());
    }

    Ok(address)
}

#[link(name = "kernel32")]
extern "system" {
    fn GetProcAddress(library: RawPtr, name: *const u8) -> RawPtr;
    fn LoadLibraryExW(name: *const u16, file: RawPtr, flags: u32) -> RawPtr;
    fn FreeLibrary(library: RawPtr) -> i32;
}
