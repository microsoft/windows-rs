#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Discharging: Self = Self(1i32);
    pub const Idle: Self = Self(2i32);
    pub const Charging: Self = Self(3i32);
}
#[repr(transparent)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Disabled: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const On: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IBackgroundEnergyManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IForegroundEnergyManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPowerManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PowerSupplyStatus(pub i32);
impl PowerSupplyStatus {
    pub const NotPresent: Self = Self(0i32);
    pub const Inadequate: Self = Self(1i32);
    pub const Adequate: Self = Self(2i32);
}
