#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct I2cBusSpeed(i32);
pub struct I2cConnectionSettings(i32);
pub struct I2cController(i32);
pub struct I2cDevice(i32);
pub struct I2cSharingMode(i32);
pub struct I2cTransferResult(i32);
pub struct I2cTransferStatus(i32);
pub struct II2cConnectionSettings(pub *mut ::core::ffi::c_void);
pub struct II2cConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
pub struct II2cController(pub *mut ::core::ffi::c_void);
pub struct II2cControllerStatics(pub *mut ::core::ffi::c_void);
pub struct II2cDevice(pub *mut ::core::ffi::c_void);
pub struct II2cDeviceStatics(pub *mut ::core::ffi::c_void);
