#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILicenseManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILicenseManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILicenseSatisfactionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILicenseSatisfactionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LicenseRefreshOption(pub i32);
impl LicenseRefreshOption {
    pub const RunningLicenses: LicenseRefreshOption = LicenseRefreshOption(0i32);
    pub const AllLicenses: LicenseRefreshOption = LicenseRefreshOption(1i32);
}
#[repr(transparent)]
pub struct LicenseSatisfactionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LicenseSatisfactionResult(pub *mut ::core::ffi::c_void);
