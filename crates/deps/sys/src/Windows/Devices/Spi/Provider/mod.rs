#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct ProviderSpiMode(i32);
#[repr(C)]
pub struct ProviderSpiSharingMode(i32);
