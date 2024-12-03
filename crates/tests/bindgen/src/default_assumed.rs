#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_targets::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
