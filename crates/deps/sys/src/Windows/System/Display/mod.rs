#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DisplayRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayRequest {}
impl ::core::clone::Clone for DisplayRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayRequest {}
impl ::core::clone::Clone for IDisplayRequest {
    fn clone(&self) -> Self {
        *self
    }
}
