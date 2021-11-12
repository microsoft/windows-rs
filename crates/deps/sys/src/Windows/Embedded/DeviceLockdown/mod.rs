#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DeviceLockdownProfileInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceLockdownProfileInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceLockdownProfileStatics(pub *mut ::core::ffi::c_void);
