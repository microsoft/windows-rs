#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct EasClientDeviceInformation(i32);
pub struct EasClientSecurityPolicy(i32);
pub struct EasComplianceResults(i32);
pub struct EasContract(i32);
pub struct EasDisallowConvenienceLogonResult(i32);
pub struct EasEncryptionProviderType(i32);
pub struct EasMaxInactivityTimeLockResult(i32);
pub struct EasMaxPasswordFailedAttemptsResult(i32);
pub struct EasMinPasswordComplexCharactersResult(i32);
pub struct EasMinPasswordLengthResult(i32);
pub struct EasPasswordExpirationResult(i32);
pub struct EasPasswordHistoryResult(i32);
pub struct EasRequireEncryptionResult(i32);
pub struct IEasClientDeviceInformation(pub *mut ::core::ffi::c_void);
pub struct IEasClientDeviceInformation2(pub *mut ::core::ffi::c_void);
pub struct IEasClientSecurityPolicy(pub *mut ::core::ffi::c_void);
pub struct IEasComplianceResults(pub *mut ::core::ffi::c_void);
pub struct IEasComplianceResults2(pub *mut ::core::ffi::c_void);
