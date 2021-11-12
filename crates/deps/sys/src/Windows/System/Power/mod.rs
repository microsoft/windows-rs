#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BatteryStatus(pub i32);
impl BatteryStatus {
    pub const NotPresent: BatteryStatus = BatteryStatus(0i32);
    pub const Discharging: BatteryStatus = BatteryStatus(1i32);
    pub const Idle: BatteryStatus = BatteryStatus(2i32);
    pub const Charging: BatteryStatus = BatteryStatus(3i32);
}
#[repr(transparent)]
pub struct EnergySaverStatus(pub i32);
impl EnergySaverStatus {
    pub const Disabled: EnergySaverStatus = EnergySaverStatus(0i32);
    pub const Off: EnergySaverStatus = EnergySaverStatus(1i32);
    pub const On: EnergySaverStatus = EnergySaverStatus(2i32);
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
    pub const NotPresent: PowerSupplyStatus = PowerSupplyStatus(0i32);
    pub const Inadequate: PowerSupplyStatus = PowerSupplyStatus(1i32);
    pub const Adequate: PowerSupplyStatus = PowerSupplyStatus(2i32);
}
