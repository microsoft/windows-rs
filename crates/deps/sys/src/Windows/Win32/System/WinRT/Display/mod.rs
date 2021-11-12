#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IDisplayDeviceInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayDeviceInterop {}
impl ::core::clone::Clone for IDisplayDeviceInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayPathInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayPathInterop {}
impl ::core::clone::Clone for IDisplayPathInterop {
    fn clone(&self) -> Self {
        *self
    }
}
