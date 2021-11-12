#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GazeDeviceConfigurationStatePreview(i32);
#[repr(transparent)]
pub struct GazeDevicePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazeDeviceWatcherAddedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazeDeviceWatcherPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazeEnteredPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazeExitedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazeInputSourcePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazeMovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GazePointPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeDevicePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeDeviceWatcherPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeEnteredPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeExitedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeInputSourcePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeInputSourcePreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazeMovedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGazePointPreview(pub *mut ::core::ffi::c_void);
