#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct UserDataAccountProviderOperationKind(pub i32);
impl UserDataAccountProviderOperationKind {
    pub const AddAccount: Self = Self(0i32);
    pub const Settings: Self = Self(1i32);
    pub const ResolveErrors: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataAccountProviderOperationKind {}
impl ::core::clone::Clone for UserDataAccountProviderOperationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataAccountProviderPartnerAccountKind(pub i32);
impl UserDataAccountProviderPartnerAccountKind {
    pub const Exchange: Self = Self(0i32);
    pub const PopOrImap: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataAccountProviderPartnerAccountKind {}
impl ::core::clone::Clone for UserDataAccountProviderPartnerAccountKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataAccountProviderResolveErrorsOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountProviderSettingsOperation(pub *mut ::core::ffi::c_void);
