#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWICImageEncoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWICImagingFactory2(pub *mut ::core::ffi::c_void);
