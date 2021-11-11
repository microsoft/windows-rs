#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn II2cControllerProvider();
    fn II2cDeviceProvider();
    fn II2cProvider();
    fn IProviderI2cConnectionSettings();
    fn ProviderI2cBusSpeed();
    fn ProviderI2cConnectionSettings();
    fn ProviderI2cSharingMode();
    fn ProviderI2cTransferResult();
    fn ProviderI2cTransferStatus();
}
