#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct UserDataAccountContentKinds(i32);
#[repr(transparent)]
pub struct UserDataAccountManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataAccountManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataAccountOtherAppReadAccess(i32);
#[repr(transparent)]
pub struct UserDataAccountStore(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataAccountStoreAccessType(i32);
#[repr(transparent)]
pub struct UserDataAccountStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
