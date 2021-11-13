#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppRecordingManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppRecordingManager {}
impl ::core::clone::Clone for AppRecordingManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppRecordingResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppRecordingResult {}
impl ::core::clone::Clone for AppRecordingResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotOption(pub i32);
impl AppRecordingSaveScreenshotOption {
    pub const None: Self = Self(0i32);
    pub const HdrContentVisible: Self = Self(1i32);
}
impl ::core::marker::Copy for AppRecordingSaveScreenshotOption {}
impl ::core::clone::Clone for AppRecordingSaveScreenshotOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppRecordingSaveScreenshotResult {}
impl ::core::clone::Clone for AppRecordingSaveScreenshotResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppRecordingSavedScreenshotInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppRecordingSavedScreenshotInfo {}
impl ::core::clone::Clone for AppRecordingSavedScreenshotInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppRecordingStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppRecordingStatus {}
impl ::core::clone::Clone for AppRecordingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppRecordingStatusDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppRecordingStatusDetails {}
impl ::core::clone::Clone for AppRecordingStatusDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppRecordingManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppRecordingManager {}
impl ::core::clone::Clone for IAppRecordingManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppRecordingManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppRecordingManagerStatics {}
impl ::core::clone::Clone for IAppRecordingManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppRecordingResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppRecordingResult {}
impl ::core::clone::Clone for IAppRecordingResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppRecordingSaveScreenshotResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppRecordingSaveScreenshotResult {}
impl ::core::clone::Clone for IAppRecordingSaveScreenshotResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppRecordingSavedScreenshotInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppRecordingSavedScreenshotInfo {}
impl ::core::clone::Clone for IAppRecordingSavedScreenshotInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppRecordingStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppRecordingStatus {}
impl ::core::clone::Clone for IAppRecordingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppRecordingStatusDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppRecordingStatusDetails {}
impl ::core::clone::Clone for IAppRecordingStatusDetails {
    fn clone(&self) -> Self {
        *self
    }
}
