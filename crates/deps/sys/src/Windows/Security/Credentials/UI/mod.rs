#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AuthenticationProtocol(i32);
#[repr(transparent)]
pub struct CredentialPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CredentialPickerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CredentialPickerResults(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CredentialSaveOption(i32);
#[repr(transparent)]
pub struct ICredentialPickerOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialPickerResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialPickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserConsentVerifierStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserConsentVerificationResult(i32);
#[repr(transparent)]
pub struct UserConsentVerifier(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserConsentVerifierAvailability(i32);
