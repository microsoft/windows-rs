#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[link(name = "windows")]
extern "system" {}
pub struct ILamp(i32);
pub struct ILampArray(i32);
pub struct ILampArrayStatics(i32);
pub struct ILampAvailabilityChangedEventArgs(i32);
pub struct ILampInfo(i32);
pub struct ILampStatics(i32);
pub struct Lamp(i32);
pub struct LampArray(i32);
pub struct LampArrayKind(i32);
pub struct LampAvailabilityChangedEventArgs(i32);
pub struct LampInfo(i32);
pub struct LampPurposes(i32);
