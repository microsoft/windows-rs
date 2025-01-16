#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("kernel32.dll" "system" fn lstrlenA(lpstring : PCSTR) -> i32);
pub type PCSTR = *const u8;
