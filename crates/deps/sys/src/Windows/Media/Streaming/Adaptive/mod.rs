#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdaptiveMediaSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSource {}
impl ::core::clone::Clone for AdaptiveMediaSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceAdvancedSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceAdvancedSettings {}
impl ::core::clone::Clone for AdaptiveMediaSourceAdvancedSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceCorrelatedTimes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceCorrelatedTimes {}
impl ::core::clone::Clone for AdaptiveMediaSourceCorrelatedTimes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceCreationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceCreationResult {}
impl ::core::clone::Clone for AdaptiveMediaSourceCreationResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AdaptiveMediaSourceCreationStatus {}
impl ::core::clone::Clone for AdaptiveMediaSourceCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticAvailableEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AdaptiveMediaSourceDiagnosticType {}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnosticType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnostics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDiagnostics {}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnostics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadBitrateChangedReason {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadCompletedEventArgs {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadFailedEventArgs {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadRequestedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadRequestedDeferral {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadRequestedEventArgs {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadResult {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadStatistics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadStatistics {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdaptiveMediaSourcePlaybackBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
impl ::core::clone::Clone for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AdaptiveMediaSourceResourceType {}
impl ::core::clone::Clone for AdaptiveMediaSourceResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSource {}
impl ::core::clone::Clone for IAdaptiveMediaSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSource2 {}
impl ::core::clone::Clone for IAdaptiveMediaSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSource3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSource3 {}
impl ::core::clone::Clone for IAdaptiveMediaSource3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceAdvancedSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceAdvancedSettings {}
impl ::core::clone::Clone for IAdaptiveMediaSourceAdvancedSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCorrelatedTimes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceCorrelatedTimes {}
impl ::core::clone::Clone for IAdaptiveMediaSourceCorrelatedTimes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCreationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceCreationResult {}
impl ::core::clone::Clone for IAdaptiveMediaSourceCreationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCreationResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceCreationResult2 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceCreationResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnostics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDiagnostics {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDiagnostics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadCompletedEventArgs {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadFailedEventArgs {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadFailedEventArgs2 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadFailedEventArgs3 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadRequestedDeferral {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadRequestedDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadRequestedEventArgs {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadResult {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadResult2 {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadStatistics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceDownloadStatistics {}
impl ::core::clone::Clone for IAdaptiveMediaSourceDownloadStatistics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
impl ::core::clone::Clone for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdaptiveMediaSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdaptiveMediaSourceStatics {}
impl ::core::clone::Clone for IAdaptiveMediaSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
