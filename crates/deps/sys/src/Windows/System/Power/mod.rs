#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[link(name = "windows")]
extern "system" {}
pub struct BackgroundEnergyManager(i32);
pub struct BatteryStatus(i32);
pub struct EnergySaverStatus(i32);
pub struct ForegroundEnergyManager(i32);
pub struct IBackgroundEnergyManagerStatics(i32);
pub struct IForegroundEnergyManagerStatics(i32);
pub struct IPowerManagerStatics(i32);
pub struct PowerManager(i32);
pub struct PowerSupplyStatus(i32);
