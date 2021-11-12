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
    pub const RunningLicenses: Self = Self(0i32);
    pub const AllLicenses: Self = Self(1i32);
}
impl ::core::marker::Copy for LicenseRefreshOption {}
impl ::core::clone::Clone for LicenseRefreshOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LicenseSatisfactionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LicenseSatisfactionResult(pub *mut ::core::ffi::c_void);
