#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IOemSupportInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmbiosInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemSupportDeviceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemSupportInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemSupportInfoStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OemSupportInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SystemManufacturersContract(i32);
#[repr(transparent)]
pub struct SystemSupportDeviceInfo(pub *mut ::core::ffi::c_void);
