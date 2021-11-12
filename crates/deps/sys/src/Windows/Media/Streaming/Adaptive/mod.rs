#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct AdaptiveMediaSourceCreationStatus(pub i32);
impl AdaptiveMediaSourceCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const ManifestDownloadFailure: Self = Self(1i32);
    pub const ManifestParseFailure: Self = Self(2i32);
    pub const UnsupportedManifestContentType: Self = Self(3i32);
    pub const UnsupportedManifestVersion: Self = Self(4i32);
    pub const UnsupportedManifestProfile: Self = Self(5i32);
    pub const UnknownFailure: Self = Self(6i32);
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticType(pub i32);
impl AdaptiveMediaSourceDiagnosticType {
    pub const ManifestUnchangedUponReload: Self = Self(0i32);
    pub const ManifestMismatchUponReload: Self = Self(1i32);
    pub const ManifestSignaledEndOfLiveEventUponReload: Self = Self(2i32);
    pub const MediaSegmentSkipped: Self = Self(3i32);
    pub const ResourceNotFound: Self = Self(4i32);
    pub const ResourceTimedOut: Self = Self(5i32);
    pub const ResourceParsingError: Self = Self(6i32);
    pub const BitrateDisabled: Self = Self(7i32);
    pub const FatalMediaSourceError: Self = Self(8i32);
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnostics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedReason(pub i32);
impl AdaptiveMediaSourceDownloadBitrateChangedReason {
    pub const SufficientInboundBitsPerSecond: Self = Self(0i32);
    pub const InsufficientInboundBitsPerSecond: Self = Self(1i32);
    pub const LowBufferLevel: Self = Self(2i32);
    pub const PositionChanged: Self = Self(3i32);
    pub const TrackSelectionChanged: Self = Self(4i32);
    pub const DesiredBitratesChanged: Self = Self(5i32);
    pub const ErrorInPreviousBitrate: Self = Self(6i32);
}
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
#[repr(transparent)]
pub struct AdaptiveMediaSourceResourceType(pub i32);
impl AdaptiveMediaSourceResourceType {
    pub const Manifest: Self = Self(0i32);
    pub const InitializationSegment: Self = Self(1i32);
    pub const MediaSegment: Self = Self(2i32);
    pub const Key: Self = Self(3i32);
    pub const InitializationVector: Self = Self(4i32);
    pub const MediaSegmentIndex: Self = Self(5i32);
}
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
