#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct II2cControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cControllerProvider {}
impl ::core::clone::Clone for II2cControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct II2cDeviceProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cDeviceProvider {}
impl ::core::clone::Clone for II2cDeviceProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct II2cProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for II2cProvider {}
impl ::core::clone::Clone for II2cProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProviderI2cConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProviderI2cConnectionSettings {}
impl ::core::clone::Clone for IProviderI2cConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProviderI2cBusSpeed(pub i32);
impl ProviderI2cBusSpeed {
    pub const StandardMode: Self = Self(0i32);
    pub const FastMode: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderI2cBusSpeed {}
impl ::core::clone::Clone for ProviderI2cBusSpeed {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProviderI2cConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProviderI2cConnectionSettings {}
impl ::core::clone::Clone for ProviderI2cConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProviderI2cSharingMode(pub i32);
impl ProviderI2cSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderI2cSharingMode {}
impl ::core::clone::Clone for ProviderI2cSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ProviderI2cTransferResult {
    pub Status: ProviderI2cTransferStatus,
    pub BytesTransferred: u32,
}
impl ::core::marker::Copy for ProviderI2cTransferResult {}
impl ::core::clone::Clone for ProviderI2cTransferResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProviderI2cTransferStatus(pub i32);
impl ProviderI2cTransferStatus {
    pub const FullTransfer: Self = Self(0i32);
    pub const PartialTransfer: Self = Self(1i32);
    pub const SlaveAddressNotAcknowledged: Self = Self(2i32);
}
impl ::core::marker::Copy for ProviderI2cTransferStatus {}
impl ::core::clone::Clone for ProviderI2cTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
