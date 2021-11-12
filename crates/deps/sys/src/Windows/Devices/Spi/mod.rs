#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Spi_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpiBusInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiConnectionSettingsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpiDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpiBusInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpiConnectionSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpiController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpiDevice(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SpiMode(i32);
#[repr(C)]
pub struct SpiSharingMode(i32);
