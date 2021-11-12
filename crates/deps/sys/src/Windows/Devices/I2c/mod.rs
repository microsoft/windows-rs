#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct I2cBusSpeed(pub i32);
impl I2cBusSpeed {
    pub const StandardMode: I2cBusSpeed = I2cBusSpeed(0i32);
    pub const FastMode: I2cBusSpeed = I2cBusSpeed(1i32);
}
#[repr(transparent)]
pub struct I2cConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct I2cController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct I2cDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct I2cSharingMode(pub i32);
impl I2cSharingMode {
    pub const Exclusive: I2cSharingMode = I2cSharingMode(0i32);
    pub const Shared: I2cSharingMode = I2cSharingMode(1i32);
}
#[repr(C)]
pub struct I2cTransferResult(i32);
#[repr(transparent)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: I2cTransferStatus = I2cTransferStatus(0i32);
    pub const PartialTransfer: I2cTransferStatus = I2cTransferStatus(1i32);
    pub const SlaveAddressNotAcknowledged: I2cTransferStatus = I2cTransferStatus(2i32);
    pub const ClockStretchTimeout: I2cTransferStatus = I2cTransferStatus(3i32);
    pub const UnknownError: I2cTransferStatus = I2cTransferStatus(4i32);
}
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
