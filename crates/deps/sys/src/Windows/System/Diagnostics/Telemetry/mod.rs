#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPlatformTelemetryClientStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationStatus(pub i32);
impl PlatformTelemetryRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const SettingsOutOfRange: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
