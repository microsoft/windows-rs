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
    pub const Success: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(0i32);
    pub const ManifestDownloadFailure: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(1i32);
    pub const ManifestParseFailure: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(2i32);
    pub const UnsupportedManifestContentType: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(3i32);
    pub const UnsupportedManifestVersion: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(4i32);
    pub const UnsupportedManifestProfile: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(5i32);
    pub const UnknownFailure: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(6i32);
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticAvailableEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticType(pub i32);
impl AdaptiveMediaSourceDiagnosticType {
    pub const ManifestUnchangedUponReload: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(0i32);
    pub const ManifestMismatchUponReload: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(1i32);
    pub const ManifestSignaledEndOfLiveEventUponReload: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(2i32);
    pub const MediaSegmentSkipped: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(3i32);
    pub const ResourceNotFound: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(4i32);
    pub const ResourceTimedOut: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(5i32);
    pub const ResourceParsingError: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(6i32);
    pub const BitrateDisabled: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(7i32);
    pub const FatalMediaSourceError: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(8i32);
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnostics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedReason(pub i32);
impl AdaptiveMediaSourceDownloadBitrateChangedReason {
    pub const SufficientInboundBitsPerSecond: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(0i32);
    pub const InsufficientInboundBitsPerSecond: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(1i32);
    pub const LowBufferLevel: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(2i32);
    pub const PositionChanged: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(3i32);
    pub const TrackSelectionChanged: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(4i32);
    pub const DesiredBitratesChanged: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(5i32);
    pub const ErrorInPreviousBitrate: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(6i32);
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
    pub const Manifest: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(0i32);
    pub const InitializationSegment: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(1i32);
    pub const MediaSegment: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(2i32);
    pub const Key: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(3i32);
    pub const InitializationVector: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(4i32);
    pub const MediaSegmentIndex: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(5i32);
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
