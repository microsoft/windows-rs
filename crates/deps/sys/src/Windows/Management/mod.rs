#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Management_Core")]
pub mod Core;
#[cfg(feature = "Management_Deployment")]
pub mod Deployment;
#[cfg(feature = "Management_Policies")]
pub mod Policies;
#[cfg(feature = "Management_Update")]
pub mod Update;
#[cfg(feature = "Management_Workplace")]
pub mod Workplace;
