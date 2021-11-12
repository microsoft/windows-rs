#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IOfflineMapPackage(pub *mut ::core::ffi::c_void);
pub struct IOfflineMapPackageQueryResult(pub *mut ::core::ffi::c_void);
pub struct IOfflineMapPackageStartDownloadResult(pub *mut ::core::ffi::c_void);
pub struct IOfflineMapPackageStatics(pub *mut ::core::ffi::c_void);
pub struct OfflineMapPackage(i32);
pub struct OfflineMapPackageQueryResult(i32);
pub struct OfflineMapPackageQueryStatus(i32);
pub struct OfflineMapPackageStartDownloadResult(i32);
pub struct OfflineMapPackageStartDownloadStatus(i32);
pub struct OfflineMapPackageStatus(i32);
