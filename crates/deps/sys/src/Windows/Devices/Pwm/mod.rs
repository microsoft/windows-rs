#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Pwm_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct IPwmController(i32);
pub struct IPwmControllerStatics(i32);
pub struct IPwmControllerStatics2(i32);
pub struct IPwmControllerStatics3(i32);
pub struct IPwmPin(i32);
pub struct PwmController(i32);
pub struct PwmPin(i32);
pub struct PwmPulsePolarity(i32);
