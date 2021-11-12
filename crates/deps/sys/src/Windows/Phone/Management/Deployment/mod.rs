#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Enterprise(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnterpriseEnrollmentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EnterpriseEnrollmentResult(pub *mut ::core::ffi::c_void);
pub struct EnterpriseEnrollmentStatus(i32);
pub struct EnterpriseStatus(i32);
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
pub struct InstallationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PackageInstallResult(pub *mut ::core::ffi::c_void);
