#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWICImageEncoder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWICImageEncoder {}
impl ::core::clone::Clone for IWICImageEncoder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWICImagingFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWICImagingFactory2 {}
impl ::core::clone::Clone for IWICImagingFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
