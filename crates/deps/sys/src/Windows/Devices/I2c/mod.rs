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
pub struct II2cConnectionSettings(i32);
pub struct II2cConnectionSettingsFactory(i32);
pub struct II2cController(i32);
pub struct II2cControllerStatics(i32);
pub struct II2cDevice(i32);
pub struct II2cDeviceStatics(i32);
