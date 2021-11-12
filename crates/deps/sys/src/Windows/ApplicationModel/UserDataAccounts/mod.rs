#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub mod Provider;
#[cfg(feature = "ApplicationModel_UserDataAccounts_SystemAccess")]
pub mod SystemAccess;
#[link(name = "windows")]
extern "system" {}
pub struct IUserDataAccount(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccount2(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccount3(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccount4(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountManagerForUser(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountStore(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountStore2(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountStore3(pub *mut ::core::ffi::c_void);
pub struct IUserDataAccountStoreChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct UserDataAccount(i32);
pub struct UserDataAccountContentKinds(i32);
pub struct UserDataAccountManager(i32);
pub struct UserDataAccountManagerForUser(i32);
pub struct UserDataAccountOtherAppReadAccess(i32);
pub struct UserDataAccountStore(i32);
pub struct UserDataAccountStoreAccessType(i32);
pub struct UserDataAccountStoreChangedEventArgs(i32);
