#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISecondaryAuthenticationFactorAuthentication(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorAuthenticationResult(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorAuthenticationStageInfo(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorAuthenticationStatics(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorInfo(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorInfo2(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorRegistration(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorRegistrationResult(pub *mut ::core::ffi::c_void);
pub struct ISecondaryAuthenticationFactorRegistrationStatics(pub *mut ::core::ffi::c_void);
pub struct SecondaryAuthenticationFactorAuthentication(i32);
pub struct SecondaryAuthenticationFactorAuthenticationMessage(i32);
pub struct SecondaryAuthenticationFactorAuthenticationResult(i32);
pub struct SecondaryAuthenticationFactorAuthenticationScenario(i32);
pub struct SecondaryAuthenticationFactorAuthenticationStage(i32);
pub struct SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(i32);
pub struct SecondaryAuthenticationFactorAuthenticationStageInfo(i32);
pub struct SecondaryAuthenticationFactorAuthenticationStatus(i32);
pub struct SecondaryAuthenticationFactorDeviceCapabilities(i32);
pub struct SecondaryAuthenticationFactorDeviceFindScope(i32);
pub struct SecondaryAuthenticationFactorDevicePresence(i32);
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringMode(i32);
pub struct SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus(i32);
pub struct SecondaryAuthenticationFactorFinishAuthenticationStatus(i32);
pub struct SecondaryAuthenticationFactorInfo(i32);
pub struct SecondaryAuthenticationFactorRegistration(i32);
pub struct SecondaryAuthenticationFactorRegistrationResult(i32);
pub struct SecondaryAuthenticationFactorRegistrationStatus(i32);
