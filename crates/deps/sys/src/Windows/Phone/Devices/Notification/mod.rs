#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IVibrationDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVibrationDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VibrationDevice(pub *mut ::core::ffi::c_void);
