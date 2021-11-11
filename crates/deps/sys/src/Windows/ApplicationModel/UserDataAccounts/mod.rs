#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub mod Provider;
#[cfg(feature = "ApplicationModel_UserDataAccounts_SystemAccess")]
pub mod SystemAccess;
#[link(name = "windows")]
extern "system" {}
