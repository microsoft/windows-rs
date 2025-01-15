#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn WindowsGetStringLen(string : HSTRING) -> u32);
pub type HSTRING = *mut core::ffi::c_void;
