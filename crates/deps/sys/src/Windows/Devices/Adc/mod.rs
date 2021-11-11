#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Adc_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    fn AdcChannel();
    fn AdcChannelMode();
    fn AdcController();
    fn IAdcChannel();
    fn IAdcController();
    fn IAdcControllerStatics();
    fn IAdcControllerStatics2();
}
