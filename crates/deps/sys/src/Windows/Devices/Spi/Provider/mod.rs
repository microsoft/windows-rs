#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IProviderSpiConnectionSettings(pub *mut ::core::ffi::c_void);
pub struct IProviderSpiConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
pub struct ISpiControllerProvider(pub *mut ::core::ffi::c_void);
pub struct ISpiDeviceProvider(pub *mut ::core::ffi::c_void);
pub struct ISpiProvider(pub *mut ::core::ffi::c_void);
pub struct ProviderSpiConnectionSettings(i32);
pub struct ProviderSpiMode(i32);
pub struct ProviderSpiSharingMode(i32);
