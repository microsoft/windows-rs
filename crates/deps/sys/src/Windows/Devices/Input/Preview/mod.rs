#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GazeDeviceConfigurationStatePreview(i32);
pub struct GazeDevicePreview(i32);
pub struct GazeDeviceWatcherAddedPreviewEventArgs(i32);
pub struct GazeDeviceWatcherPreview(i32);
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(i32);
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(i32);
pub struct GazeEnteredPreviewEventArgs(i32);
pub struct GazeExitedPreviewEventArgs(i32);
pub struct GazeInputSourcePreview(i32);
pub struct GazeMovedPreviewEventArgs(i32);
pub struct GazePointPreview(i32);
pub struct IGazeDevicePreview(pub *mut ::core::ffi::c_void);
pub struct IGazeDeviceWatcherAddedPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGazeDeviceWatcherPreview(pub *mut ::core::ffi::c_void);
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGazeEnteredPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGazeExitedPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGazeInputSourcePreview(pub *mut ::core::ffi::c_void);
pub struct IGazeInputSourcePreviewStatics(pub *mut ::core::ffi::c_void);
pub struct IGazeMovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGazePointPreview(pub *mut ::core::ffi::c_void);
