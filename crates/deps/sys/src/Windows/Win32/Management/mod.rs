#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Management_MobileDeviceManagementRegistration")]
pub mod MobileDeviceManagementRegistration;
#[link(name = "windows")]
extern "system" {}
