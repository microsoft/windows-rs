#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ExtendedExecutionReason(pub i32);
impl ExtendedExecutionReason {
    pub const Unspecified: ExtendedExecutionReason = ExtendedExecutionReason(0i32);
    pub const LocationTracking: ExtendedExecutionReason = ExtendedExecutionReason(1i32);
    pub const SavingData: ExtendedExecutionReason = ExtendedExecutionReason(2i32);
}
#[repr(transparent)]
pub struct ExtendedExecutionResult(pub i32);
impl ExtendedExecutionResult {
    pub const Allowed: ExtendedExecutionResult = ExtendedExecutionResult(0i32);
    pub const Denied: ExtendedExecutionResult = ExtendedExecutionResult(1i32);
}
#[repr(transparent)]
pub struct ExtendedExecutionRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExtendedExecutionRevokedReason(pub i32);
impl ExtendedExecutionRevokedReason {
    pub const Resumed: ExtendedExecutionRevokedReason = ExtendedExecutionRevokedReason(0i32);
    pub const SystemPolicy: ExtendedExecutionRevokedReason = ExtendedExecutionRevokedReason(1i32);
}
#[repr(transparent)]
pub struct ExtendedExecutionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendedExecutionRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendedExecutionSession(pub *mut ::core::ffi::c_void);
