#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Embedded_DeviceLockdown")]
pub mod DeviceLockdown;
#[link(name = "windows")]
extern "system" {}
