#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub mod Provider;
#[cfg(feature = "ApplicationModel_UserDataAccounts_SystemAccess")]
pub mod SystemAccess;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserDataAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccount2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccount3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccount4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountStore3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataAccountStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountContentKinds(pub u32);
impl UserDataAccountContentKinds {
    pub const Email: Self = Self(1u32);
    pub const Contact: Self = Self(2u32);
    pub const Appointment: Self = Self(4u32);
}
#[repr(transparent)]
pub struct UserDataAccountManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountOtherAppReadAccess(pub i32);
impl UserDataAccountOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
#[repr(transparent)]
pub struct UserDataAccountStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountStoreAccessType(pub i32);
impl UserDataAccountStoreAccessType {
    pub const AllAccountsReadOnly: Self = Self(0i32);
    pub const AppAccountsReadWrite: Self = Self(1i32);
}
#[repr(transparent)]
pub struct UserDataAccountStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
