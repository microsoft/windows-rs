#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppRecordingContract(i32);
#[repr(transparent)]
pub struct AppRecordingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppRecordingResult(pub *mut ::core::ffi::c_void);
pub struct AppRecordingSaveScreenshotOption(i32);
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppRecordingSavedScreenshotInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppRecordingStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppRecordingStatusDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppRecordingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppRecordingManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppRecordingResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppRecordingSaveScreenshotResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppRecordingSavedScreenshotInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppRecordingStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppRecordingStatusDetails(pub *mut ::core::ffi::c_void);
