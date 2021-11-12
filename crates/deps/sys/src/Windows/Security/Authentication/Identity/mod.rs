#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Identity_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct EnterpriseKeyCredentialRegistrationInfo(i32);
pub struct EnterpriseKeyCredentialRegistrationManager(i32);
pub struct IEnterpriseKeyCredentialRegistrationInfo(pub *mut ::core::ffi::c_void);
pub struct IEnterpriseKeyCredentialRegistrationManager(pub *mut ::core::ffi::c_void);
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics(pub *mut ::core::ffi::c_void);
