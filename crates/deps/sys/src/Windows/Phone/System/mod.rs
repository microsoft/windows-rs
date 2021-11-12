#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Phone_System_Power")]
pub mod Power;
#[cfg(feature = "Phone_System_Profile")]
pub mod Profile;
#[cfg(feature = "Phone_System_UserProfile")]
pub mod UserProfile;
#[link(name = "windows")]
extern "system" {}
pub struct ISystemProtectionStatics(i32);
pub struct ISystemProtectionUnlockStatics(i32);
pub struct SystemProtection(i32);
