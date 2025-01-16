#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("ole32.dll" "system" fn CoInitialize(pvreserved : *const core::ffi::c_void) -> HRESULT);
pub type HRESULT = i32;
