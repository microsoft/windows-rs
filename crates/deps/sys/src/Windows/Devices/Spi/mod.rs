#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Spi_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct ISpiBusInfo(pub *mut ::core::ffi::c_void);
pub struct ISpiConnectionSettings(pub *mut ::core::ffi::c_void);
pub struct ISpiConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
pub struct ISpiController(pub *mut ::core::ffi::c_void);
pub struct ISpiControllerStatics(pub *mut ::core::ffi::c_void);
pub struct ISpiDevice(pub *mut ::core::ffi::c_void);
pub struct ISpiDeviceStatics(pub *mut ::core::ffi::c_void);
pub struct SpiBusInfo(i32);
pub struct SpiConnectionSettings(i32);
pub struct SpiController(i32);
pub struct SpiDevice(i32);
pub struct SpiMode(i32);
pub struct SpiSharingMode(i32);
