#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppRecordingContract(i32);
pub struct AppRecordingManager(i32);
pub struct AppRecordingResult(i32);
pub struct AppRecordingSaveScreenshotOption(i32);
pub struct AppRecordingSaveScreenshotResult(i32);
pub struct AppRecordingSavedScreenshotInfo(i32);
pub struct AppRecordingStatus(i32);
pub struct AppRecordingStatusDetails(i32);
pub struct IAppRecordingManager(pub *mut ::core::ffi::c_void);
pub struct IAppRecordingManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IAppRecordingResult(pub *mut ::core::ffi::c_void);
pub struct IAppRecordingSaveScreenshotResult(pub *mut ::core::ffi::c_void);
pub struct IAppRecordingSavedScreenshotInfo(pub *mut ::core::ffi::c_void);
pub struct IAppRecordingStatus(pub *mut ::core::ffi::c_void);
pub struct IAppRecordingStatusDetails(pub *mut ::core::ffi::c_void);
