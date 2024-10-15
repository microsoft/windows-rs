#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub mod Windows {
    pub mod Win32 {
        pub mod Foundation {
            windows_targets::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> BOOL);
            pub type BOOL = i32;
            pub type HANDLE = *mut core::ffi::c_void;
            pub type HMODULE = *mut core::ffi::c_void;
            pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
        }
        pub mod System {
            pub mod LibraryLoader {
                windows_targets::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : super::super::Foundation:: HMODULE, lpprocname : super::super::super::super::PCSTR) -> super::super::Foundation:: FARPROC);
                windows_targets::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : super::super::super::super::PCSTR, hfile : super::super::Foundation:: HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> super::super::Foundation:: HMODULE);
                pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
                pub type LOAD_LIBRARY_FLAGS = u32;
            }
        }
    }
}
pub type PCSTR = *const u8;
