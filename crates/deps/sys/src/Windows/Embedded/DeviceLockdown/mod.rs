#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DeviceLockdownContract(i32);
pub struct DeviceLockdownProfile(i32);
pub struct DeviceLockdownProfileInformation(i32);
pub struct IDeviceLockdownProfileInformation(pub *mut ::core::ffi::c_void);
pub struct IDeviceLockdownProfileStatics(pub *mut ::core::ffi::c_void);
