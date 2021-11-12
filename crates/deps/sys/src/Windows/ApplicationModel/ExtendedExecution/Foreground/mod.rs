#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ExtendedExecutionForegroundReason(i32);
pub struct ExtendedExecutionForegroundResult(i32);
pub struct ExtendedExecutionForegroundRevokedEventArgs(i32);
pub struct ExtendedExecutionForegroundRevokedReason(i32);
pub struct ExtendedExecutionForegroundSession(i32);
pub struct IExtendedExecutionForegroundRevokedEventArgs(i32);
pub struct IExtendedExecutionForegroundSession(i32);
