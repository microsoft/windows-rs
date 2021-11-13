#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Enterprise(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Enterprise {}
impl ::core::clone::Clone for Enterprise {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EnterpriseEnrollmentResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EnterpriseEnrollmentResult {}
impl ::core::clone::Clone for EnterpriseEnrollmentResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for IEnterprise {}
impl ::core::clone::Clone for IEnterprise {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnterpriseEnrollmentManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnterpriseEnrollmentManager {}
impl ::core::clone::Clone for IEnterpriseEnrollmentManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnterpriseEnrollmentResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnterpriseEnrollmentResult {}
impl ::core::clone::Clone for IEnterpriseEnrollmentResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInstallationManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInstallationManagerStatics {}
impl ::core::clone::Clone for IInstallationManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInstallationManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInstallationManagerStatics2 {}
impl ::core::clone::Clone for IInstallationManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageInstallResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageInstallResult {}
impl ::core::clone::Clone for IPackageInstallResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPackageInstallResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPackageInstallResult2 {}
impl ::core::clone::Clone for IPackageInstallResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageInstallResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PackageInstallResult {}
impl ::core::clone::Clone for PackageInstallResult {
    fn clone(&self) -> Self {
        *self
    }
}
