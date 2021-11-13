#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for I2cBusSpeed {}
impl ::core::clone::Clone for I2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct I2cConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for I2cConnectionSettings {}
impl ::core::clone::Clone for I2cConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct I2cController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for I2cController {}
impl ::core::clone::Clone for I2cController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct I2cDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for I2cDevice {}
impl ::core::clone::Clone for I2cDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct I2cSharingMode(pub i32);
impl I2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for I2cSharingMode {}
impl ::core::clone::Clone for I2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct I2cTransferResult {
    pub Status: I2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for I2cTransferResult {}
impl ::core::clone::Clone for I2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct I2cTransferStatus(pub i32);
impl I2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
    pub const ClockStretchTimeout: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for I2cTransferStatus {}
impl ::core::clone::Clone for I2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct II2cConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cConnectionSettings {}
impl ::core::clone::Clone for II2cConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct II2cConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cConnectionSettingsFactory {}
impl ::core::clone::Clone for II2cConnectionSettingsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct II2cController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cController {}
impl ::core::clone::Clone for II2cController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct II2cControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cControllerStatics {}
impl ::core::clone::Clone for II2cControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct II2cDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cDevice {}
impl ::core::clone::Clone for II2cDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct II2cDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cDeviceStatics {}
impl ::core::clone::Clone for II2cDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
