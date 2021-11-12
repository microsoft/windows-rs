#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundReason(pub i32);
impl ExtendedExecutionForegroundReason {
    pub const Unspecified: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(0i32);
    pub const SavingData: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(1i32);
    pub const BackgroundAudio: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(2i32);
    pub const Unconstrained: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(3i32);
}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundResult(pub i32);
impl ExtendedExecutionForegroundResult {
    pub const Allowed: ExtendedExecutionForegroundResult = ExtendedExecutionForegroundResult(0i32);
    pub const Denied: ExtendedExecutionForegroundResult = ExtendedExecutionForegroundResult(1i32);
}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedReason(pub i32);
impl ExtendedExecutionForegroundRevokedReason {
    pub const Resumed: ExtendedExecutionForegroundRevokedReason = ExtendedExecutionForegroundRevokedReason(0i32);
    pub const SystemPolicy: ExtendedExecutionForegroundRevokedReason = ExtendedExecutionForegroundRevokedReason(1i32);
}
#[repr(transparent)]
pub struct ExtendedExecutionForegroundSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendedExecutionForegroundRevokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtendedExecutionForegroundSession(pub *mut ::core::ffi::c_void);
