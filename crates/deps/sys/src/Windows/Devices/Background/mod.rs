#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeviceServicingDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceUseDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceServicingDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceUseDetails(pub *mut ::core::ffi::c_void);
