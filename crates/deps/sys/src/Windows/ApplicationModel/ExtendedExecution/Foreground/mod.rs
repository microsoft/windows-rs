#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ExtendedExecutionForegroundReason(i32);
#[repr(C)]
pub struct ExtendedExecutionForegroundResult(i32);
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ExtendedExecutionForegroundRevokedReason(i32);
#[repr(transparent)]
pub struct ExtendedExecutionForegroundSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendedExecutionForegroundRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendedExecutionForegroundSession(pub *mut ::core::ffi::c_void);
