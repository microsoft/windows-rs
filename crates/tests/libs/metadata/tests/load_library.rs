#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type HANDLE = *mut std::ffi::c_void;
pub type HMODULE = *mut std::ffi::c_void;
pub type LOAD_LIBRARY_FLAGS = u32;

pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = 32u32;

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn LoadLibraryExA(
        lplibfilename: *const u8,
        hfile: HANDLE,
        dwflags: LOAD_LIBRARY_FLAGS,
    ) -> HMODULE;
}

#[test]
fn test() {
    unsafe {
        let module = LoadLibraryExA(
            c"../../../libs/bindgen/default/Windows.winmd".as_ptr() as _,
            core::ptr::null_mut(),
            LOAD_LIBRARY_AS_IMAGE_RESOURCE,
        );
        assert!(!module.is_null());
    }
}
