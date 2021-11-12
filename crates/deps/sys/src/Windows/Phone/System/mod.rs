#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Phone_System_Power")]
pub mod Power;
#[cfg(feature = "Phone_System_Profile")]
pub mod Profile;
#[cfg(feature = "Phone_System_UserProfile")]
pub mod UserProfile;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISystemProtectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemProtectionUnlockStatics(pub *mut ::core::ffi::c_void);
