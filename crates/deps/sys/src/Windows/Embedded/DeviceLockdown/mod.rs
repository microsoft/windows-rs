#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DeviceLockdownContract(i32);
#[repr(transparent)]
pub struct DeviceLockdownProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceLockdownProfileInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceLockdownProfileInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceLockdownProfileStatics(pub *mut ::core::ffi::c_void);
