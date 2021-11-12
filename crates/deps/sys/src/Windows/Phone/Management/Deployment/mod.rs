#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Enterprise(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnterpriseEnrollmentResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnterpriseEnrollmentStatus(pub i32);
impl EnterpriseEnrollmentStatus {
    pub const Success: Self = Self(0i32);
    pub const CancelledByUser: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for EnterpriseEnrollmentStatus {}
impl ::core::clone::Clone for EnterpriseEnrollmentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnterpriseStatus(pub i32);
impl EnterpriseStatus {
    pub const Enrolled: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Expired: Self = Self(3i32);
}
impl ::core::marker::Copy for EnterpriseStatus {}
impl ::core::clone::Clone for EnterpriseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnterprise(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnterpriseEnrollmentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnterpriseEnrollmentResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInstallationManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageInstallResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageInstallResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageInstallResult(pub *mut ::core::ffi::c_void);
