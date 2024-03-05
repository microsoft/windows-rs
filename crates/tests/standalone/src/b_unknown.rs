#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
::windows_targets::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : * mut core::ffi::c_void) -> BOOL);
pub type BOOL = i32;
