#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ExtendedExecutionReason(i32);
#[repr(C)]
pub struct ExtendedExecutionResult(i32);
#[repr(transparent)]
pub struct ExtendedExecutionRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ExtendedExecutionRevokedReason(i32);
#[repr(transparent)]
pub struct ExtendedExecutionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendedExecutionRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendedExecutionSession(pub *mut ::core::ffi::c_void);
