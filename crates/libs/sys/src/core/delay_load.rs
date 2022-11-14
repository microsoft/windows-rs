use super::*;

/// Attempts to load a function from a given library.
///
/// This is a small wrapper around `LoadLibrary` and `GetProcAddress`.
///
/// # Safety
///
/// * Both the library and function names must be valid null-terminated strings.
pub unsafe fn delay_load<T>(library: PCSTR, function: PCSTR) -> Option<T> {
    let library = LoadLibraryA(library);

    if library == 0 {
        return None;
    }

    let address = GetProcAddress(library, function);

    if !address.is_null() {
        return Some(core::mem::transmute_copy(&address));
    }

    FreeLibrary(library);
    None
}

#[link(name = "windows")]
extern "system" {
    fn GetProcAddress(library: isize, name: PCSTR) -> *const core::ffi::c_void;
    fn LoadLibraryA(name: PCSTR) -> isize;
    fn FreeLibrary(library: isize) -> i32;
}
