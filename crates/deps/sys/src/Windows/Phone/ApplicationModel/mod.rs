#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ApplicationProfileModes(i32);
#[repr(transparent)]
pub struct IApplicationProfileStatics(pub *mut ::core::ffi::c_void);
