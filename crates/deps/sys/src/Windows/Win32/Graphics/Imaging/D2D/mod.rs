#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWICImageEncoder(pub *mut ::core::ffi::c_void);
pub struct IWICImagingFactory2(pub *mut ::core::ffi::c_void);
