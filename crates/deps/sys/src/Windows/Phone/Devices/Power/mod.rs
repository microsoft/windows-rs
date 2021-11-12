#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Battery(i32);
pub struct IBattery(pub *mut ::core::ffi::c_void);
pub struct IBatteryStatics(pub *mut ::core::ffi::c_void);
