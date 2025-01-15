#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
