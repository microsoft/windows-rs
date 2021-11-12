#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILockApplicationHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockApplicationHostStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenBadge(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenUnlockingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenUnlockingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockApplicationHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenBadge(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenUnlockingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LockScreenUnlockingEventArgs(pub *mut ::core::ffi::c_void);
