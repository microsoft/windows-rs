#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserDataAccountPartnerAccountInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountProviderAddAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountProviderOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountProviderResolveErrorsOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountProviderSettingsOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountPartnerAccountInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountProviderAddAccountOperation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataAccountProviderOperationKind(i32);
#[repr(C)]
pub struct UserDataAccountProviderPartnerAccountKind(i32);
#[repr(transparent)]
pub struct UserDataAccountProviderResolveErrorsOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountProviderSettingsOperation(pub *mut ::core::ffi::c_void);
