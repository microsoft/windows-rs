#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthentication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorRegistrationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthentication(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SecondaryAuthenticationFactorAuthenticationMessage(i32);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SecondaryAuthenticationFactorAuthenticationScenario(i32);
#[repr(C)]
pub struct SecondaryAuthenticationFactorAuthenticationStage(i32);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationStageInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SecondaryAuthenticationFactorAuthenticationStatus(i32);
#[repr(C)]
pub struct SecondaryAuthenticationFactorDeviceCapabilities(i32);
#[repr(C)]
pub struct SecondaryAuthenticationFactorDeviceFindScope(i32);
#[repr(C)]
pub struct SecondaryAuthenticationFactorDevicePresence(i32);
#[repr(C)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringMode(i32);
#[repr(C)]
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(i32);
#[repr(C)]
pub struct SecondaryAuthenticationFactorFinishAuthenticationStatus(i32);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorRegistrationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SecondaryAuthenticationFactorRegistrationStatus(i32);
