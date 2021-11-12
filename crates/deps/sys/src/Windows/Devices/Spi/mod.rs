#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct SpiMode(pub i32);
impl SpiMode {
    pub const Mode0: SpiMode = SpiMode(0i32);
    pub const Mode1: SpiMode = SpiMode(1i32);
    pub const Mode2: SpiMode = SpiMode(2i32);
    pub const Mode3: SpiMode = SpiMode(3i32);
}
#[repr(transparent)]
pub struct SpiSharingMode(pub i32);
impl SpiSharingMode {
    pub const Exclusive: SpiSharingMode = SpiSharingMode(0i32);
    pub const Shared: SpiSharingMode = SpiSharingMode(1i32);
}
