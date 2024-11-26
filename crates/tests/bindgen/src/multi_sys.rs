#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub const HTTP_VERSION: windows_sys::core::PCWSTR = windows_sys::core::w!("HTTP/1.0");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
