#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AuthenticationProtocol(i32);
pub struct CredentialPicker(i32);
pub struct CredentialPickerOptions(i32);
pub struct CredentialPickerResults(i32);
pub struct CredentialSaveOption(i32);
pub struct ICredentialPickerOptions(pub *mut ::core::ffi::c_void);
pub struct ICredentialPickerResults(pub *mut ::core::ffi::c_void);
pub struct ICredentialPickerStatics(pub *mut ::core::ffi::c_void);
pub struct IUserConsentVerifierStatics(pub *mut ::core::ffi::c_void);
pub struct UserConsentVerificationResult(i32);
pub struct UserConsentVerifier(i32);
pub struct UserConsentVerifierAvailability(i32);
