#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct I2cBusSpeed(pub i32);
impl I2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
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
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
#[repr(C)]
pub struct I2cTransferResult(i32);
#[repr(transparent)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
    pub const ClockStretchTimeout: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
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
