#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[link(name = "windows")]
extern "system" {
    fn BackgroundEnergyManager();
    fn BatteryStatus();
    fn EnergySaverStatus();
    fn ForegroundEnergyManager();
    fn IBackgroundEnergyManagerStatics();
    fn IForegroundEnergyManagerStatics();
    fn IPowerManagerStatics();
    fn PowerManager();
    fn PowerSupplyStatus();
}
