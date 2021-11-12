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
    pub const Success: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(0i32);
    pub const CancelledByUser: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(1i32);
    pub const UnknownFailure: EnterpriseEnrollmentStatus = EnterpriseEnrollmentStatus(2i32);
}
#[repr(transparent)]
pub struct EnterpriseStatus(pub i32);
impl EnterpriseStatus {
    pub const Enrolled: EnterpriseStatus = EnterpriseStatus(0i32);
    pub const Disabled: EnterpriseStatus = EnterpriseStatus(1i32);
    pub const Revoked: EnterpriseStatus = EnterpriseStatus(2i32);
    pub const Expired: EnterpriseStatus = EnterpriseStatus(3i32);
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
