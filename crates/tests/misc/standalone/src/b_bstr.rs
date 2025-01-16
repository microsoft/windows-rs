#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("oleaut32.dll" "system" fn SysAllocString(psz : PCWSTR) -> BSTR);
pub type BSTR = *const u16;
pub type PCWSTR = *const u16;
