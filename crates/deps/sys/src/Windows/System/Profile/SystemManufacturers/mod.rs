#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IOemSupportInfo(pub *mut ::core::ffi::c_void);
pub struct ISmbiosInformationStatics(pub *mut ::core::ffi::c_void);
pub struct ISystemSupportDeviceInfo(pub *mut ::core::ffi::c_void);
pub struct ISystemSupportInfoStatics(pub *mut ::core::ffi::c_void);
pub struct ISystemSupportInfoStatics2(pub *mut ::core::ffi::c_void);
pub struct OemSupportInfo(i32);
pub struct SmbiosInformation(i32);
pub struct SystemManufacturersContract(i32);
pub struct SystemSupportDeviceInfo(i32);
pub struct SystemSupportInfo(i32);
