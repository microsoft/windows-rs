#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Identity_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EnterpriseKeyCredentialRegistrationInfo {}
impl ::core::clone::Clone for EnterpriseKeyCredentialRegistrationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EnterpriseKeyCredentialRegistrationManager {}
impl ::core::clone::Clone for EnterpriseKeyCredentialRegistrationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnterpriseKeyCredentialRegistrationInfo {}
impl ::core::clone::Clone for IEnterpriseKeyCredentialRegistrationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnterpriseKeyCredentialRegistrationManager {}
impl ::core::clone::Clone for IEnterpriseKeyCredentialRegistrationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnterpriseKeyCredentialRegistrationManagerStatics {}
impl ::core::clone::Clone for IEnterpriseKeyCredentialRegistrationManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
