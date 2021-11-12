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
    pub const StandardMode: ProviderI2cBusSpeed = ProviderI2cBusSpeed(0i32);
    pub const FastMode: ProviderI2cBusSpeed = ProviderI2cBusSpeed(1i32);
}
#[repr(transparent)]
pub struct ProviderI2cConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProviderI2cSharingMode(pub i32);
impl ProviderI2cSharingMode {
    pub const Exclusive: ProviderI2cSharingMode = ProviderI2cSharingMode(0i32);
    pub const Shared: ProviderI2cSharingMode = ProviderI2cSharingMode(1i32);
}
#[repr(C)]
pub struct ProviderI2cTransferResult(i32);
#[repr(transparent)]
pub struct ProviderI2cTransferStatus(pub i32);
impl ProviderI2cTransferStatus {
    pub const FullTransfer: ProviderI2cTransferStatus = ProviderI2cTransferStatus(0i32);
    pub const PartialTransfer: ProviderI2cTransferStatus = ProviderI2cTransferStatus(1i32);
    pub const SlaveAddressNotAcknowledged: ProviderI2cTransferStatus = ProviderI2cTransferStatus(2i32);
}
