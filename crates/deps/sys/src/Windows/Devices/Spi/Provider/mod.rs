#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IProviderSpiConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderSpiConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiDeviceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProviderSpiConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProviderSpiMode(pub i32);
impl ProviderSpiMode {
    pub const Mode0: ProviderSpiMode = ProviderSpiMode(0i32);
    pub const Mode1: ProviderSpiMode = ProviderSpiMode(1i32);
    pub const Mode2: ProviderSpiMode = ProviderSpiMode(2i32);
    pub const Mode3: ProviderSpiMode = ProviderSpiMode(3i32);
}
#[repr(transparent)]
pub struct ProviderSpiSharingMode(pub i32);
impl ProviderSpiSharingMode {
    pub const Exclusive: ProviderSpiSharingMode = ProviderSpiSharingMode(0i32);
    pub const Shared: ProviderSpiSharingMode = ProviderSpiSharingMode(1i32);
}
