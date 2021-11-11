#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Spi_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    fn ISpiBusInfo();
    fn ISpiConnectionSettings();
    fn ISpiConnectionSettingsFactory();
    fn ISpiController();
    fn ISpiControllerStatics();
    fn ISpiDevice();
    fn ISpiDeviceStatics();
    fn SpiBusInfo();
    fn SpiConnectionSettings();
    fn SpiController();
    fn SpiDevice();
    fn SpiMode();
    fn SpiSharingMode();
}
