#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Spi_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct ISpiBusInfo(i32);
pub struct ISpiConnectionSettings(i32);
pub struct ISpiConnectionSettingsFactory(i32);
pub struct ISpiController(i32);
pub struct ISpiControllerStatics(i32);
pub struct ISpiDevice(i32);
pub struct ISpiDeviceStatics(i32);
pub struct SpiBusInfo(i32);
pub struct SpiConnectionSettings(i32);
pub struct SpiController(i32);
pub struct SpiDevice(i32);
pub struct SpiMode(i32);
pub struct SpiSharingMode(i32);
