#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IDummyHICONIncluder(pub *mut ::core::ffi::c_void);
pub struct IThumbnailExtractor(pub *mut ::core::ffi::c_void);
