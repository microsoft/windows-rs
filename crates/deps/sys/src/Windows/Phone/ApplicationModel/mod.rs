#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ApplicationProfile(i32);
pub struct ApplicationProfileModes(i32);
pub struct IApplicationProfileStatics(pub *mut ::core::ffi::c_void);
