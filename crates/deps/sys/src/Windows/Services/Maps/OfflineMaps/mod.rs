#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IOfflineMapPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineMapPackageQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineMapPackageStartDownloadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOfflineMapPackageStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OfflineMapPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OfflineMapPackageQueryResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OfflineMapPackageQueryStatus(pub i32);
impl OfflineMapPackageQueryStatus {
    pub const Success: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(0i32);
    pub const UnknownError: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(1i32);
    pub const InvalidCredentials: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(2i32);
    pub const NetworkFailure: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(3i32);
}
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadStatus(pub i32);
impl OfflineMapPackageStartDownloadStatus {
    pub const Success: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(0i32);
    pub const UnknownError: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(1i32);
    pub const InvalidCredentials: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(2i32);
    pub const DeniedWithoutCapability: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(3i32);
}
#[repr(transparent)]
pub struct OfflineMapPackageStatus(pub i32);
impl OfflineMapPackageStatus {
    pub const NotDownloaded: OfflineMapPackageStatus = OfflineMapPackageStatus(0i32);
    pub const Downloading: OfflineMapPackageStatus = OfflineMapPackageStatus(1i32);
    pub const Downloaded: OfflineMapPackageStatus = OfflineMapPackageStatus(2i32);
    pub const Deleting: OfflineMapPackageStatus = OfflineMapPackageStatus(3i32);
}
