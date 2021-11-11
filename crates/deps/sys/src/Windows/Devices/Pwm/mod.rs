#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Pwm_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    fn IPwmController();
    fn IPwmControllerStatics();
    fn IPwmControllerStatics2();
    fn IPwmControllerStatics3();
    fn IPwmPin();
    fn PwmController();
    fn PwmPin();
    fn PwmPulsePolarity();
}
