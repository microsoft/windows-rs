#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Enumeration_Pnp")]
pub mod Pnp;
#[link(name = "windows")]
extern "system" {}
