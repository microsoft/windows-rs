#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[link(name = "windows")]
extern "system" {
    fn ILamp();
    fn ILampArray();
    fn ILampArrayStatics();
    fn ILampAvailabilityChangedEventArgs();
    fn ILampInfo();
    fn ILampStatics();
    fn Lamp();
    fn LampArray();
    fn LampArrayKind();
    fn LampAvailabilityChangedEventArgs();
    fn LampInfo();
    fn LampPurposes();
}
