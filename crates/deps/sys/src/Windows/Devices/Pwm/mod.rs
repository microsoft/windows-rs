#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Pwm_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPwmController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPwmControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPwmControllerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPwmControllerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPwmPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PwmController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PwmPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PwmPulsePolarity(pub i32);
impl PwmPulsePolarity {
    pub const ActiveHigh: Self = Self(0i32);
    pub const ActiveLow: Self = Self(1i32);
}
impl ::core::marker::Copy for PwmPulsePolarity {}
impl ::core::clone::Clone for PwmPulsePolarity {
    fn clone(&self) -> Self {
        *self
    }
}
