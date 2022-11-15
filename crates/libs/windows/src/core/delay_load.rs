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
        return Some(std::mem::transmute_copy(&address));
    }

    FreeLibrary(library);
    None
}

#[cfg(not(windows_raw_dylib))]
#[link(name = "windows")]
extern "system" {
    fn GetProcAddress(library: isize, name: PCSTR) -> *const std::ffi::c_void;
    fn LoadLibraryA(name: PCSTR) -> isize;
    fn FreeLibrary(library: isize) -> i32;
}

#[cfg(all(windows_raw_dylib, target_arch = "x86"))]
#[link(name = "kernel32.dll", kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated")]
extern "system" {
    fn GetProcAddress(library: isize, name: PCSTR) -> *const std::ffi::c_void;
    fn LoadLibraryA(name: PCSTR) -> isize;
    fn FreeLibrary(library: isize) -> i32;
}

#[cfg(all(windows_raw_dylib, not(target_arch = "x86")))]
#[link(name = "kernel32.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    fn GetProcAddress(library: isize, name: PCSTR) -> *const std::ffi::c_void;
    fn LoadLibraryA(name: PCSTR) -> isize;
    fn FreeLibrary(library: isize) -> i32;
}
