#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Pwm_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct IPwmController(pub *mut ::core::ffi::c_void);
pub struct IPwmControllerStatics(pub *mut ::core::ffi::c_void);
pub struct IPwmControllerStatics2(pub *mut ::core::ffi::c_void);
pub struct IPwmControllerStatics3(pub *mut ::core::ffi::c_void);
pub struct IPwmPin(pub *mut ::core::ffi::c_void);
pub struct PwmController(i32);
pub struct PwmPin(i32);
pub struct PwmPulsePolarity(i32);
