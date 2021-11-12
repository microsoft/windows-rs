#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPlatformTelemetryClientStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformTelemetryClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationSettings(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PlatformTelemetryRegistrationStatus(i32);
