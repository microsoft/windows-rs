#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ILicenseManagerStatics(pub *mut ::core::ffi::c_void);
pub struct ILicenseManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct ILicenseSatisfactionInfo(pub *mut ::core::ffi::c_void);
pub struct ILicenseSatisfactionResult(pub *mut ::core::ffi::c_void);
pub struct LicenseManager(i32);
pub struct LicenseRefreshOption(i32);
pub struct LicenseSatisfactionInfo(i32);
pub struct LicenseSatisfactionResult(i32);
