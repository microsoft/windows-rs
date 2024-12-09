#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HTTP_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl Default for HTTP_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HTTP_VERSION {
    type TypeKind = windows_core::CopyType;
}
pub const HTTP_VERSION: windows_core::PCWSTR = windows_core::w!("HTTP/1.0");
