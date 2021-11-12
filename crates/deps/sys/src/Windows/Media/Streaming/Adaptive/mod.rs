#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdaptiveMediaSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceAdvancedSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceCorrelatedTimes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceCreationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AdaptiveMediaSourceCreationStatus(i32);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AdaptiveMediaSourceDiagnosticType(i32);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnostics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedReason(i32);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadRequestedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadStatistics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourcePlaybackBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AdaptiveMediaSourceResourceType(i32);
#[repr(transparent)]
pub struct IAdaptiveMediaSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSource3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceAdvancedSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCorrelatedTimes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCreationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCreationResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnostics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadStatistics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveMediaSourceStatics(pub *mut ::core::ffi::c_void);
