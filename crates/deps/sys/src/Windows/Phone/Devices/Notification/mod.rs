#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IVibrationDevice(pub *mut ::core::ffi::c_void);
pub struct IVibrationDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct VibrationDevice(i32);
