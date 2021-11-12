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
    pub const AddAccount: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(0i32);
    pub const Settings: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(1i32);
    pub const ResolveErrors: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(2i32);
}
#[repr(transparent)]
pub struct UserDataAccountProviderPartnerAccountKind(pub i32);
impl UserDataAccountProviderPartnerAccountKind {
    pub const Exchange: UserDataAccountProviderPartnerAccountKind = UserDataAccountProviderPartnerAccountKind(0i32);
    pub const PopOrImap: UserDataAccountProviderPartnerAccountKind = UserDataAccountProviderPartnerAccountKind(1i32);
}
#[repr(transparent)]
pub struct UserDataAccountProviderResolveErrorsOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountProviderSettingsOperation(pub *mut ::core::ffi::c_void);
