#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Battery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBattery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBatteryStatics(pub *mut ::core::ffi::c_void);
