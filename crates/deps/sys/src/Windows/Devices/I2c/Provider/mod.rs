#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct II2cControllerProvider(pub *mut ::core::ffi::c_void);
pub struct II2cDeviceProvider(pub *mut ::core::ffi::c_void);
pub struct II2cProvider(pub *mut ::core::ffi::c_void);
pub struct IProviderI2cConnectionSettings(pub *mut ::core::ffi::c_void);
pub struct ProviderI2cBusSpeed(i32);
pub struct ProviderI2cConnectionSettings(i32);
pub struct ProviderI2cSharingMode(i32);
pub struct ProviderI2cTransferResult(i32);
pub struct ProviderI2cTransferStatus(i32);
