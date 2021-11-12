#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EasClientDeviceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EasClientSecurityPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EasComplianceResults(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct EasContract(i32);
#[repr(C)]
pub struct EasDisallowConvenienceLogonResult(i32);
#[repr(C)]
pub struct EasEncryptionProviderType(i32);
#[repr(C)]
pub struct EasMaxInactivityTimeLockResult(i32);
#[repr(C)]
pub struct EasMaxPasswordFailedAttemptsResult(i32);
#[repr(C)]
pub struct EasMinPasswordComplexCharactersResult(i32);
#[repr(C)]
pub struct EasMinPasswordLengthResult(i32);
#[repr(C)]
pub struct EasPasswordExpirationResult(i32);
#[repr(C)]
pub struct EasPasswordHistoryResult(i32);
#[repr(C)]
pub struct EasRequireEncryptionResult(i32);
#[repr(transparent)]
pub struct IEasClientDeviceInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasClientDeviceInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasClientSecurityPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasComplianceResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEasComplianceResults2(pub *mut ::core::ffi::c_void);
