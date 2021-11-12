#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ILockApplicationHost(pub *mut ::core::ffi::c_void);
pub struct ILockApplicationHostStatics(pub *mut ::core::ffi::c_void);
pub struct ILockScreenBadge(pub *mut ::core::ffi::c_void);
pub struct ILockScreenInfo(pub *mut ::core::ffi::c_void);
pub struct ILockScreenUnlockingDeferral(pub *mut ::core::ffi::c_void);
pub struct ILockScreenUnlockingEventArgs(pub *mut ::core::ffi::c_void);
pub struct LockApplicationHost(i32);
pub struct LockScreenBadge(i32);
pub struct LockScreenInfo(i32);
pub struct LockScreenUnlockingDeferral(i32);
pub struct LockScreenUnlockingEventArgs(i32);
