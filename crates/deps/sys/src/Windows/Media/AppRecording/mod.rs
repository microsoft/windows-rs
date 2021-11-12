#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppRecordingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppRecordingResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotOption(pub i32);
impl AppRecordingSaveScreenshotOption {
    pub const None: Self = Self(0i32);
    pub const HdrContentVisible: Self = Self(1i32);
}
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
