#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IOfflineMapPackage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineMapPackage {}
impl ::core::clone::Clone for IOfflineMapPackage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineMapPackageQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineMapPackageQueryResult {}
impl ::core::clone::Clone for IOfflineMapPackageQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineMapPackageStartDownloadResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineMapPackageStartDownloadResult {}
impl ::core::clone::Clone for IOfflineMapPackageStartDownloadResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOfflineMapPackageStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOfflineMapPackageStatics {}
impl ::core::clone::Clone for IOfflineMapPackageStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OfflineMapPackage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OfflineMapPackage {}
impl ::core::clone::Clone for OfflineMapPackage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OfflineMapPackageQueryResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OfflineMapPackageQueryResult {}
impl ::core::clone::Clone for OfflineMapPackageQueryResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OfflineMapPackageQueryStatus(pub i32);
impl OfflineMapPackageQueryStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageQueryStatus {}
impl ::core::clone::Clone for OfflineMapPackageQueryStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OfflineMapPackageStartDownloadResult {}
impl ::core::clone::Clone for OfflineMapPackageStartDownloadResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadStatus(pub i32);
impl OfflineMapPackageStartDownloadStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const DeniedWithoutCapability: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageStartDownloadStatus {}
impl ::core::clone::Clone for OfflineMapPackageStartDownloadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OfflineMapPackageStatus(pub i32);
impl OfflineMapPackageStatus {
    pub const NotDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Deleting: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageStatus {}
impl ::core::clone::Clone for OfflineMapPackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
