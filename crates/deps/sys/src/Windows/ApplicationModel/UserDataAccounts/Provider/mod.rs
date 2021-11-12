#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IUserDataAccountPartnerAccountInfo(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountProviderAddAccountOperation(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountProviderOperation(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountProviderResolveErrorsOperation(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountProviderSettingsOperation(pub *mut ::core::ffi::c_void);
pub struct UserDataAccountPartnerAccountInfo(i32);
pub struct UserDataAccountProviderAddAccountOperation(i32);
pub struct UserDataAccountProviderOperationKind(i32);
pub struct UserDataAccountProviderPartnerAccountKind(i32);
pub struct UserDataAccountProviderResolveErrorsOperation(i32);
pub struct UserDataAccountProviderSettingsOperation(i32);
