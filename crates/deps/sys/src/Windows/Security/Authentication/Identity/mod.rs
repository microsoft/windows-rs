#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Identity_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics(pub *mut ::core::ffi::c_void);
