#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct I2cBusSpeed(i32);
#[repr(transparent)]
pub struct I2cConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct I2cController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct I2cDevice(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct I2cSharingMode(i32);
#[repr(C)]
pub struct I2cTransferResult(i32);
#[repr(C)]
pub struct I2cTransferStatus(i32);
#[repr(transparent)]
pub struct II2cConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct II2cConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct II2cController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct II2cControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct II2cDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct II2cDeviceStatics(pub *mut ::core::ffi::c_void);
