#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IProviderSpiConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProviderSpiConnectionSettings {}
impl ::core::clone::Clone for IProviderSpiConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProviderSpiConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProviderSpiConnectionSettingsFactory {}
impl ::core::clone::Clone for IProviderSpiConnectionSettingsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiControllerProvider {}
impl ::core::clone::Clone for ISpiControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiDeviceProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiDeviceProvider {}
impl ::core::clone::Clone for ISpiDeviceProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpiProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpiProvider {}
impl ::core::clone::Clone for ISpiProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProviderSpiConnectionSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProviderSpiConnectionSettings {}
impl ::core::clone::Clone for ProviderSpiConnectionSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProviderSpiMode(pub i32);
impl ProviderSpiMode {
    pub const Mode0: Self = Self(0i32);
    pub const Mode1: Self = Self(1i32);
    pub const Mode2: Self = Self(2i32);
    pub const Mode3: Self = Self(3i32);
}
impl ::core::marker::Copy for ProviderSpiMode {}
impl ::core::clone::Clone for ProviderSpiMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProviderSpiSharingMode(pub i32);
impl ProviderSpiSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const Shared: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderSpiSharingMode {}
impl ::core::clone::Clone for ProviderSpiSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
