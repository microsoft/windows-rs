#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub type GetTickCount = unsafe extern "system" fn() -> u32;
windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
