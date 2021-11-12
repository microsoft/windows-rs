#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[link(name = "windows")]
extern "system" {}
pub struct ExtendedExecutionReason(i32);
pub struct ExtendedExecutionResult(i32);
pub struct ExtendedExecutionRevokedEventArgs(i32);
pub struct ExtendedExecutionRevokedReason(i32);
pub struct ExtendedExecutionSession(i32);
pub struct IExtendedExecutionRevokedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IExtendedExecutionSession(pub *mut ::core::ffi::c_void);
