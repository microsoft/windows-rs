#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_I2c_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    fn I2cBusSpeed();
    fn I2cConnectionSettings();
    fn I2cController();
    fn I2cDevice();
    fn I2cSharingMode();
    fn I2cTransferResult();
    fn I2cTransferStatus();
    fn II2cConnectionSettings();
    fn II2cConnectionSettingsFactory();
    fn II2cController();
    fn II2cControllerStatics();
    fn II2cDevice();
    fn II2cDeviceStatics();
}
