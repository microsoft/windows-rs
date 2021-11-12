#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Adc_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct AdcChannel(i32);
pub struct AdcChannelMode(i32);
pub struct AdcController(i32);
pub struct IAdcChannel(i32);
pub struct IAdcController(i32);
pub struct IAdcControllerStatics(i32);
pub struct IAdcControllerStatics2(i32);
