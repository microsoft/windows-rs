#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

extern "C" {
    pub fn wsprintfA(param0: windows_sys::core::PSTR, param1: windows_sys::core::PCSTR, ...)
        -> i32;
}
