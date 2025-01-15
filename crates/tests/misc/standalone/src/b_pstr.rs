#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("oleaut32.dll" "system" fn VarI1FromDate(datein : f64, pcout : PSTR) -> HRESULT);
pub type HRESULT = i32;
pub type PSTR = *mut u8;
