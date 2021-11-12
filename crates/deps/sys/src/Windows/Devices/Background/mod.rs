#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DeviceServicingDetails(i32);
pub struct DeviceUseDetails(i32);
pub struct IDeviceServicingDetails(pub *mut ::core::ffi::c_void);
pub struct IDeviceUseDetails(pub *mut ::core::ffi::c_void);
