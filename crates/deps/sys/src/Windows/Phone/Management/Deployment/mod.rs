#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Enterprise(i32);
pub struct EnterpriseEnrollmentManager(i32);
pub struct EnterpriseEnrollmentResult(i32);
pub struct EnterpriseEnrollmentStatus(i32);
pub struct EnterpriseStatus(i32);
pub struct IEnterprise(pub *mut ::core::ffi::c_void);
pub struct IEnterpriseEnrollmentManager(pub *mut ::core::ffi::c_void);
pub struct IEnterpriseEnrollmentResult(pub *mut ::core::ffi::c_void);
pub struct IInstallationManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IInstallationManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct IPackageInstallResult(pub *mut ::core::ffi::c_void);
pub struct IPackageInstallResult2(pub *mut ::core::ffi::c_void);
pub struct InstallationManager(i32);
pub struct PackageInstallResult(i32);
