#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DisplayRequest(i32);
pub struct IDisplayRequest(pub *mut ::core::ffi::c_void);
