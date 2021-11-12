#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPlatformTelemetryClientStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlatformTelemetryClientStatics {}
impl ::core::clone::Clone for IPlatformTelemetryClientStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlatformTelemetryRegistrationResult {}
impl ::core::clone::Clone for IPlatformTelemetryRegistrationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlatformTelemetryRegistrationSettings {}
impl ::core::clone::Clone for IPlatformTelemetryRegistrationSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlatformTelemetryRegistrationResult {}
impl ::core::clone::Clone for PlatformTelemetryRegistrationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlatformTelemetryRegistrationSettings {}
impl ::core::clone::Clone for PlatformTelemetryRegistrationSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationStatus(pub i32);
impl PlatformTelemetryRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const SettingsOutOfRange: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformTelemetryRegistrationStatus {}
impl ::core::clone::Clone for PlatformTelemetryRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
