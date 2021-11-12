#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
#[link(name = "windows")]
extern "system" {}
pub struct IPerceptionTimestamp(i32);
pub struct IPerceptionTimestamp2(i32);
pub struct IPerceptionTimestampHelperStatics(i32);
pub struct IPerceptionTimestampHelperStatics2(i32);
pub struct PerceptionTimestamp(i32);
pub struct PerceptionTimestampHelper(i32);
