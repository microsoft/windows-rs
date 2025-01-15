#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : windows_sys::core::PCSTR) -> FARPROC);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : windows_sys::core::PCSTR, hfile : HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE);
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type HANDLE = *mut core::ffi::c_void;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
