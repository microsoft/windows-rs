#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct OfflineMapPackageQueryStatus(i32);
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct OfflineMapPackageStartDownloadStatus(i32);
#[repr(C)]
pub struct OfflineMapPackageStatus(i32);
