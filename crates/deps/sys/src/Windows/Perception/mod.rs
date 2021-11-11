#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
#[link(name = "windows")]
extern "system" {}
