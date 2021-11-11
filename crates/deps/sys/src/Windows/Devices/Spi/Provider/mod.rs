#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IProviderSpiConnectionSettings();
    fn IProviderSpiConnectionSettingsFactory();
    fn ISpiControllerProvider();
    fn ISpiDeviceProvider();
    fn ISpiProvider();
    fn ProviderSpiConnectionSettings();
    fn ProviderSpiMode();
    fn ProviderSpiSharingMode();
}
