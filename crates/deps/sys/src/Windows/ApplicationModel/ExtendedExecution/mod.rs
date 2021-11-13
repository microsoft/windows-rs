#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ExtendedExecutionReason(pub i32);
impl ExtendedExecutionReason {
    pub const Unspecified: Self = Self(0i32);
    pub const LocationTracking: Self = Self(1i32);
    pub const SavingData: Self = Self(2i32);
}
impl ::core::marker::Copy for ExtendedExecutionReason {}
impl ::core::clone::Clone for ExtendedExecutionReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionResult(pub i32);
impl ExtendedExecutionResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionResult {}
impl ::core::clone::Clone for ExtendedExecutionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionRevokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExtendedExecutionRevokedEventArgs {}
impl ::core::clone::Clone for ExtendedExecutionRevokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionRevokedReason(pub i32);
impl ExtendedExecutionRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionRevokedReason {}
impl ::core::clone::Clone for ExtendedExecutionRevokedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ExtendedExecutionSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ExtendedExecutionSession {}
impl ::core::clone::Clone for ExtendedExecutionSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendedExecutionRevokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendedExecutionRevokedEventArgs {}
impl ::core::clone::Clone for IExtendedExecutionRevokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExtendedExecutionSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExtendedExecutionSession {}
impl ::core::clone::Clone for IExtendedExecutionSession {
    fn clone(&self) -> Self {
        *self
    }
}
