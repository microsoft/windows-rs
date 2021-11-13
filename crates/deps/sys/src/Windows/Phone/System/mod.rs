#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for ISystemProtectionStatics {}
impl ::core::clone::Clone for ISystemProtectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemProtectionUnlockStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemProtectionUnlockStatics {}
impl ::core::clone::Clone for ISystemProtectionUnlockStatics {
    fn clone(&self) -> Self {
        *self
    }
}
