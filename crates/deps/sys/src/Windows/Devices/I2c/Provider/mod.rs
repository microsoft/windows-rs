#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct II2cControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct II2cDeviceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct II2cProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderI2cConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProviderI2cBusSpeed(pub i32);
impl ProviderI2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ProviderI2cConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProviderI2cSharingMode(pub i32);
impl ProviderI2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
#[repr(C)]
pub struct ProviderI2cTransferResult(i32);
#[repr(transparent)]
pub struct ProviderI2cTransferStatus(pub i32);
impl ProviderI2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
}
