#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdvancedCapturedPhoto(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdvancedCapturedPhoto {}
impl ::core::clone::Clone for AdvancedCapturedPhoto {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdvancedPhotoCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdvancedPhotoCapture {}
impl ::core::clone::Clone for AdvancedPhotoCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastBackgroundService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastBackgroundService {}
impl ::core::clone::Clone for AppBroadcastBackgroundService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceSignInInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastBackgroundServiceSignInInfo {}
impl ::core::clone::Clone for AppBroadcastBackgroundServiceSignInInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceStreamInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastBackgroundServiceStreamInfo {}
impl ::core::clone::Clone for AppBroadcastBackgroundServiceStreamInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureState(pub i32);
impl AppBroadcastCameraCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastCameraCaptureState {}
impl ::core::clone::Clone for AppBroadcastCameraCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastCameraCaptureStateChangedEventArgs {}
impl ::core::clone::Clone for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastCameraOverlayLocation(pub i32);
impl AppBroadcastCameraOverlayLocation {
    pub const TopLeft: Self = Self(0i32);
    pub const TopCenter: Self = Self(1i32);
    pub const TopRight: Self = Self(2i32);
    pub const MiddleLeft: Self = Self(3i32);
    pub const MiddleCenter: Self = Self(4i32);
    pub const MiddleRight: Self = Self(5i32);
    pub const BottomLeft: Self = Self(6i32);
    pub const BottomCenter: Self = Self(7i32);
    pub const BottomRight: Self = Self(8i32);
}
impl ::core::marker::Copy for AppBroadcastCameraOverlayLocation {}
impl ::core::clone::Clone for AppBroadcastCameraOverlayLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastCameraOverlaySize {}
impl ::core::clone::Clone for AppBroadcastCameraOverlaySize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastCaptureTargetType {}
impl ::core::clone::Clone for AppBroadcastCaptureTargetType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
    pub const AuthorizationFail: Self = Self(2i32);
    pub const ForegroundAppActivated: Self = Self(3i32);
}
impl ::core::marker::Copy for AppBroadcastExitBroadcastModeReason {}
impl ::core::clone::Clone for AppBroadcastExitBroadcastModeReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastGlobalSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastGlobalSettings {}
impl ::core::clone::Clone for AppBroadcastGlobalSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastHeartbeatRequestedEventArgs {}
impl ::core::clone::Clone for AppBroadcastHeartbeatRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastMicrophoneCaptureState {}
impl ::core::clone::Clone for AppBroadcastMicrophoneCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
impl ::core::clone::Clone for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPlugIn(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastPlugIn {}
impl ::core::clone::Clone for AppBroadcastPlugIn {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPlugInManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastPlugInManager {}
impl ::core::clone::Clone for AppBroadcastPlugInManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPlugInState(pub i32);
impl AppBroadcastPlugInState {
    pub const Unknown: Self = Self(0i32);
    pub const Initialized: Self = Self(1i32);
    pub const MicrosoftSignInRequired: Self = Self(2i32);
    pub const OAuthSignInRequired: Self = Self(3i32);
    pub const ProviderSignInRequired: Self = Self(4i32);
    pub const InBandwidthTest: Self = Self(5i32);
    pub const ReadyToBroadcast: Self = Self(6i32);
}
impl ::core::marker::Copy for AppBroadcastPlugInState {}
impl ::core::clone::Clone for AppBroadcastPlugInState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPlugInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastPlugInStateChangedEventArgs {}
impl ::core::clone::Clone for AppBroadcastPlugInStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastPreview {}
impl ::core::clone::Clone for AppBroadcastPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBroadcastPreviewState {}
impl ::core::clone::Clone for AppBroadcastPreviewState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPreviewStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastPreviewStateChangedEventArgs {}
impl ::core::clone::Clone for AppBroadcastPreviewStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastPreviewStreamReader {}
impl ::core::clone::Clone for AppBroadcastPreviewStreamReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastPreviewStreamVideoFrame {}
impl ::core::clone::Clone for AppBroadcastPreviewStreamVideoFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastPreviewStreamVideoHeader {}
impl ::core::clone::Clone for AppBroadcastPreviewStreamVideoHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastProviderSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastProviderSettings {}
impl ::core::clone::Clone for AppBroadcastProviderSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastServices {}
impl ::core::clone::Clone for AppBroadcastServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastSignInResult(pub i32);
impl AppBroadcastSignInResult {
    pub const Success: Self = Self(0i32);
    pub const AuthenticationFailed: Self = Self(1i32);
    pub const Unauthorized: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastSignInResult {}
impl ::core::clone::Clone for AppBroadcastSignInResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastSignInState(pub i32);
impl AppBroadcastSignInState {
    pub const NotSignedIn: Self = Self(0i32);
    pub const MicrosoftSignInInProgress: Self = Self(1i32);
    pub const MicrosoftSignInComplete: Self = Self(2i32);
    pub const OAuthSignInInProgress: Self = Self(3i32);
    pub const OAuthSignInComplete: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastSignInState {}
impl ::core::clone::Clone for AppBroadcastSignInState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastSignInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastSignInStateChangedEventArgs {}
impl ::core::clone::Clone for AppBroadcastSignInStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastState {}
impl ::core::clone::Clone for AppBroadcastState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamAudioFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastStreamAudioFrame {}
impl ::core::clone::Clone for AppBroadcastStreamAudioFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamAudioHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastStreamAudioHeader {}
impl ::core::clone::Clone for AppBroadcastStreamAudioHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastStreamReader {}
impl ::core::clone::Clone for AppBroadcastStreamReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamState(pub i32);
impl AppBroadcastStreamState {
    pub const Initializing: Self = Self(0i32);
    pub const StreamReady: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Paused: Self = Self(3i32);
    pub const Terminated: Self = Self(4i32);
}
impl ::core::marker::Copy for AppBroadcastStreamState {}
impl ::core::clone::Clone for AppBroadcastStreamState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastStreamStateChangedEventArgs {}
impl ::core::clone::Clone for AppBroadcastStreamStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamVideoFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastStreamVideoFrame {}
impl ::core::clone::Clone for AppBroadcastStreamVideoFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastStreamVideoHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastStreamVideoHeader {}
impl ::core::clone::Clone for AppBroadcastStreamVideoHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastTerminationReason(pub i32);
impl AppBroadcastTerminationReason {
    pub const NormalTermination: Self = Self(0i32);
    pub const LostConnectionToService: Self = Self(1i32);
    pub const NoNetworkConnectivity: Self = Self(2i32);
    pub const ServiceAbort: Self = Self(3i32);
    pub const ServiceError: Self = Self(4i32);
    pub const ServiceUnavailable: Self = Self(5i32);
    pub const InternalError: Self = Self(6i32);
    pub const UnsupportedFormat: Self = Self(7i32);
    pub const BackgroundTaskTerminated: Self = Self(8i32);
    pub const BackgroundTaskUnresponsive: Self = Self(9i32);
}
impl ::core::marker::Copy for AppBroadcastTerminationReason {}
impl ::core::clone::Clone for AppBroadcastTerminationReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastTriggerDetails {}
impl ::core::clone::Clone for AppBroadcastTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastVideoEncodingBitrateMode {}
impl ::core::clone::Clone for AppBroadcastVideoEncodingBitrateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
impl ::core::marker::Copy for AppBroadcastVideoEncodingResolutionMode {}
impl ::core::clone::Clone for AppBroadcastVideoEncodingResolutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastViewerCountChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastViewerCountChangedEventArgs {}
impl ::core::clone::Clone for AppBroadcastViewerCountChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCapture {}
impl ::core::clone::Clone for AppCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureAlternateShortcutKeys(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureAlternateShortcutKeys {}
impl ::core::clone::Clone for AppCaptureAlternateShortcutKeys {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureDurationGeneratedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureDurationGeneratedEventArgs {}
impl ::core::clone::Clone for AppCaptureDurationGeneratedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureFileGeneratedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureFileGeneratedEventArgs {}
impl ::core::clone::Clone for AppCaptureFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: Self = Self(0i32);
    pub const Seconds: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureHistoricalBufferLengthUnit {}
impl ::core::clone::Clone for AppCaptureHistoricalBufferLengthUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureMetadataPriority {}
impl ::core::clone::Clone for AppCaptureMetadataPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureMetadataWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureMetadataWriter {}
impl ::core::clone::Clone for AppCaptureMetadataWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureMicrophoneCaptureState {}
impl ::core::clone::Clone for AppCaptureMicrophoneCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
impl ::core::clone::Clone for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureRecordOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureRecordOperation {}
impl ::core::clone::Clone for AppCaptureRecordOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureRecordingState {}
impl ::core::clone::Clone for AppCaptureRecordingState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureRecordingStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureRecordingStateChangedEventArgs {}
impl ::core::clone::Clone for AppCaptureRecordingStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureServices {}
impl ::core::clone::Clone for AppCaptureServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureSettings {}
impl ::core::clone::Clone for AppCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCaptureState {}
impl ::core::clone::Clone for AppCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureVideoEncodingBitrateMode(pub i32);
impl AppCaptureVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingBitrateMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingBitrateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingFrameRateMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingFrameRateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
impl ::core::marker::Copy for AppCaptureVideoEncodingResolutionMode {}
impl ::core::clone::Clone for AppCaptureVideoEncodingResolutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraCaptureUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraCaptureUI {}
impl ::core::clone::Clone for CameraCaptureUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIMaxPhotoResolution(pub i32);
impl CameraCaptureUIMaxPhotoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const VerySmallQvga: Self = Self(1i32);
    pub const SmallVga: Self = Self(2i32);
    pub const MediumXga: Self = Self(3i32);
    pub const Large3M: Self = Self(4i32);
    pub const VeryLarge5M: Self = Self(5i32);
}
impl ::core::marker::Copy for CameraCaptureUIMaxPhotoResolution {}
impl ::core::clone::Clone for CameraCaptureUIMaxPhotoResolution {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const LowDefinition: Self = Self(1i32);
    pub const StandardDefinition: Self = Self(2i32);
    pub const HighDefinition: Self = Self(3i32);
}
impl ::core::marker::Copy for CameraCaptureUIMaxVideoResolution {}
impl ::core::clone::Clone for CameraCaptureUIMaxVideoResolution {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraCaptureUIMode {}
impl ::core::clone::Clone for CameraCaptureUIMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraCaptureUIPhotoCaptureSettings {}
impl ::core::clone::Clone for CameraCaptureUIPhotoCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
impl ::core::marker::Copy for CameraCaptureUIPhotoFormat {}
impl ::core::clone::Clone for CameraCaptureUIPhotoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIVideoCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CameraCaptureUIVideoCaptureSettings {}
impl ::core::clone::Clone for CameraCaptureUIVideoCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
impl ::core::marker::Copy for CameraCaptureUIVideoFormat {}
impl ::core::clone::Clone for CameraCaptureUIVideoFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CapturedFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CapturedFrame {}
impl ::core::clone::Clone for CapturedFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CapturedFrameControlValues(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CapturedFrameControlValues {}
impl ::core::clone::Clone for CapturedFrameControlValues {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CapturedPhoto(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CapturedPhoto {}
impl ::core::clone::Clone for CapturedPhoto {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ForegroundActivationArgument(pub i32);
impl ForegroundActivationArgument {
    pub const SignInRequired: Self = Self(0i32);
    pub const MoreSettings: Self = Self(1i32);
}
impl ::core::marker::Copy for ForegroundActivationArgument {}
impl ::core::clone::Clone for ForegroundActivationArgument {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarCommand(pub i32);
impl GameBarCommand {
    pub const OpenGameBar: Self = Self(0i32);
    pub const RecordHistoricalBuffer: Self = Self(1i32);
    pub const ToggleStartStopRecord: Self = Self(2i32);
    pub const StartRecord: Self = Self(3i32);
    pub const StopRecord: Self = Self(4i32);
    pub const TakeScreenshot: Self = Self(5i32);
    pub const StartBroadcast: Self = Self(6i32);
    pub const StopBroadcast: Self = Self(7i32);
    pub const PauseBroadcast: Self = Self(8i32);
    pub const ResumeBroadcast: Self = Self(9i32);
    pub const ToggleStartStopBroadcast: Self = Self(10i32);
    pub const ToggleMicrophoneCapture: Self = Self(11i32);
    pub const ToggleCameraCapture: Self = Self(12i32);
    pub const ToggleRecordingIndicator: Self = Self(13i32);
}
impl ::core::marker::Copy for GameBarCommand {}
impl ::core::clone::Clone for GameBarCommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: Self = Self(0i32);
    pub const Cortana: Self = Self(1i32);
    pub const AppCommand: Self = Self(2i32);
}
impl ::core::marker::Copy for GameBarCommandOrigin {}
impl ::core::clone::Clone for GameBarCommandOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameBarServices {}
impl ::core::clone::Clone for GameBarServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarServicesCommandEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameBarServicesCommandEventArgs {}
impl ::core::clone::Clone for GameBarServicesCommandEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: Self = Self(0i32);
    pub const FullScreenExclusive: Self = Self(1i32);
}
impl ::core::marker::Copy for GameBarServicesDisplayMode {}
impl ::core::clone::Clone for GameBarServicesDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarServicesManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameBarServicesManager {}
impl ::core::clone::Clone for GameBarServicesManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
impl ::core::clone::Clone for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarServicesTargetInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GameBarServicesTargetInfo {}
impl ::core::clone::Clone for GameBarServicesTargetInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GameBarTargetCapturePolicy(pub i32);
impl GameBarTargetCapturePolicy {
    pub const EnabledBySystem: Self = Self(0i32);
    pub const EnabledByUser: Self = Self(1i32);
    pub const NotEnabled: Self = Self(2i32);
    pub const ProhibitedBySystem: Self = Self(3i32);
    pub const ProhibitedByPublisher: Self = Self(4i32);
}
impl ::core::marker::Copy for GameBarTargetCapturePolicy {}
impl ::core::clone::Clone for GameBarTargetCapturePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedCapturedPhoto {}
impl ::core::clone::Clone for IAdvancedCapturedPhoto {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedCapturedPhoto2 {}
impl ::core::clone::Clone for IAdvancedCapturedPhoto2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedPhotoCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedPhotoCapture {}
impl ::core::clone::Clone for IAdvancedPhotoCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastBackgroundService {}
impl ::core::clone::Clone for IAppBroadcastBackgroundService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastBackgroundService2 {}
impl ::core::clone::Clone for IAppBroadcastBackgroundService2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastBackgroundServiceSignInInfo {}
impl ::core::clone::Clone for IAppBroadcastBackgroundServiceSignInInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastBackgroundServiceSignInInfo2 {}
impl ::core::clone::Clone for IAppBroadcastBackgroundServiceSignInInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastBackgroundServiceStreamInfo {}
impl ::core::clone::Clone for IAppBroadcastBackgroundServiceStreamInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastBackgroundServiceStreamInfo2 {}
impl ::core::clone::Clone for IAppBroadcastBackgroundServiceStreamInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastCameraCaptureStateChangedEventArgs {}
impl ::core::clone::Clone for IAppBroadcastCameraCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastGlobalSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastGlobalSettings {}
impl ::core::clone::Clone for IAppBroadcastGlobalSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastHeartbeatRequestedEventArgs {}
impl ::core::clone::Clone for IAppBroadcastHeartbeatRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastManagerStatics {}
impl ::core::clone::Clone for IAppBroadcastManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {}
impl ::core::clone::Clone for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPlugIn(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPlugIn {}
impl ::core::clone::Clone for IAppBroadcastPlugIn {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPlugInManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPlugInManager {}
impl ::core::clone::Clone for IAppBroadcastPlugInManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPlugInManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPlugInManagerStatics {}
impl ::core::clone::Clone for IAppBroadcastPlugInManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPlugInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPlugInStateChangedEventArgs {}
impl ::core::clone::Clone for IAppBroadcastPlugInStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPreview {}
impl ::core::clone::Clone for IAppBroadcastPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPreviewStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPreviewStateChangedEventArgs {}
impl ::core::clone::Clone for IAppBroadcastPreviewStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPreviewStreamReader {}
impl ::core::clone::Clone for IAppBroadcastPreviewStreamReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPreviewStreamVideoFrame {}
impl ::core::clone::Clone for IAppBroadcastPreviewStreamVideoFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastPreviewStreamVideoHeader {}
impl ::core::clone::Clone for IAppBroadcastPreviewStreamVideoHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastProviderSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastProviderSettings {}
impl ::core::clone::Clone for IAppBroadcastProviderSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastServices {}
impl ::core::clone::Clone for IAppBroadcastServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastSignInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastSignInStateChangedEventArgs {}
impl ::core::clone::Clone for IAppBroadcastSignInStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastState {}
impl ::core::clone::Clone for IAppBroadcastState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastStreamAudioFrame {}
impl ::core::clone::Clone for IAppBroadcastStreamAudioFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastStreamAudioHeader {}
impl ::core::clone::Clone for IAppBroadcastStreamAudioHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastStreamReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastStreamReader {}
impl ::core::clone::Clone for IAppBroadcastStreamReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastStreamStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastStreamStateChangedEventArgs {}
impl ::core::clone::Clone for IAppBroadcastStreamStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastStreamVideoFrame {}
impl ::core::clone::Clone for IAppBroadcastStreamVideoFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastStreamVideoHeader {}
impl ::core::clone::Clone for IAppBroadcastStreamVideoHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastTriggerDetails {}
impl ::core::clone::Clone for IAppBroadcastTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastViewerCountChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastViewerCountChangedEventArgs {}
impl ::core::clone::Clone for IAppBroadcastViewerCountChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCapture {}
impl ::core::clone::Clone for IAppCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureAlternateShortcutKeys {}
impl ::core::clone::Clone for IAppCaptureAlternateShortcutKeys {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureAlternateShortcutKeys2 {}
impl ::core::clone::Clone for IAppCaptureAlternateShortcutKeys2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureAlternateShortcutKeys3 {}
impl ::core::clone::Clone for IAppCaptureAlternateShortcutKeys3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureDurationGeneratedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureDurationGeneratedEventArgs {}
impl ::core::clone::Clone for IAppCaptureDurationGeneratedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureFileGeneratedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureFileGeneratedEventArgs {}
impl ::core::clone::Clone for IAppCaptureFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureManagerStatics {}
impl ::core::clone::Clone for IAppCaptureManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureMetadataWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureMetadataWriter {}
impl ::core::clone::Clone for IAppCaptureMetadataWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureMicrophoneCaptureStateChangedEventArgs {}
impl ::core::clone::Clone for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureRecordOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureRecordOperation {}
impl ::core::clone::Clone for IAppCaptureRecordOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureRecordingStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureRecordingStateChangedEventArgs {}
impl ::core::clone::Clone for IAppCaptureRecordingStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureServices {}
impl ::core::clone::Clone for IAppCaptureServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureSettings {}
impl ::core::clone::Clone for IAppCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureSettings2 {}
impl ::core::clone::Clone for IAppCaptureSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureSettings3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureSettings3 {}
impl ::core::clone::Clone for IAppCaptureSettings3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureSettings4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureSettings4 {}
impl ::core::clone::Clone for IAppCaptureSettings4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureSettings5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureSettings5 {}
impl ::core::clone::Clone for IAppCaptureSettings5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureState {}
impl ::core::clone::Clone for IAppCaptureState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureStatics {}
impl ::core::clone::Clone for IAppCaptureStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCaptureStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCaptureStatics2 {}
impl ::core::clone::Clone for IAppCaptureStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraCaptureUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraCaptureUI {}
impl ::core::clone::Clone for ICameraCaptureUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraCaptureUIPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraCaptureUIPhotoCaptureSettings {}
impl ::core::clone::Clone for ICameraCaptureUIPhotoCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraCaptureUIVideoCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraCaptureUIVideoCaptureSettings {}
impl ::core::clone::Clone for ICameraCaptureUIVideoCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICameraOptionsUIStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICameraOptionsUIStatics {}
impl ::core::clone::Clone for ICameraOptionsUIStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICapturedFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICapturedFrame {}
impl ::core::clone::Clone for ICapturedFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICapturedFrame2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICapturedFrame2 {}
impl ::core::clone::Clone for ICapturedFrame2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICapturedFrameControlValues(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICapturedFrameControlValues {}
impl ::core::clone::Clone for ICapturedFrameControlValues {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICapturedFrameControlValues2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICapturedFrameControlValues2 {}
impl ::core::clone::Clone for ICapturedFrameControlValues2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICapturedFrameWithSoftwareBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICapturedFrameWithSoftwareBitmap {}
impl ::core::clone::Clone for ICapturedFrameWithSoftwareBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICapturedPhoto(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICapturedPhoto {}
impl ::core::clone::Clone for ICapturedPhoto {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameBarServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameBarServices {}
impl ::core::clone::Clone for IGameBarServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameBarServicesCommandEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameBarServicesCommandEventArgs {}
impl ::core::clone::Clone for IGameBarServicesCommandEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameBarServicesManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameBarServicesManager {}
impl ::core::clone::Clone for IGameBarServicesManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameBarServicesManagerGameBarServicesCreatedEventArgs {}
impl ::core::clone::Clone for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameBarServicesManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameBarServicesManagerStatics {}
impl ::core::clone::Clone for IGameBarServicesManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGameBarServicesTargetInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGameBarServicesTargetInfo {}
impl ::core::clone::Clone for IGameBarServicesTargetInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLagMediaRecording(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLagMediaRecording {}
impl ::core::clone::Clone for ILowLagMediaRecording {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLagMediaRecording2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLagMediaRecording2 {}
impl ::core::clone::Clone for ILowLagMediaRecording2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLagMediaRecording3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLagMediaRecording3 {}
impl ::core::clone::Clone for ILowLagMediaRecording3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLagPhotoCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLagPhotoCapture {}
impl ::core::clone::Clone for ILowLagPhotoCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLagPhotoSequenceCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLagPhotoSequenceCapture {}
impl ::core::clone::Clone for ILowLagPhotoSequenceCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCapture {}
impl ::core::clone::Clone for IMediaCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCapture2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCapture2 {}
impl ::core::clone::Clone for IMediaCapture2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCapture3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCapture3 {}
impl ::core::clone::Clone for IMediaCapture3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCapture4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCapture4 {}
impl ::core::clone::Clone for IMediaCapture4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCapture5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCapture5 {}
impl ::core::clone::Clone for IMediaCapture5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCapture6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCapture6 {}
impl ::core::clone::Clone for IMediaCapture6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCapture7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCapture7 {}
impl ::core::clone::Clone for IMediaCapture7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
impl ::core::clone::Clone for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureFailedEventArgs {}
impl ::core::clone::Clone for IMediaCaptureFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureFocusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureFocusChangedEventArgs {}
impl ::core::clone::Clone for IMediaCaptureFocusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureInitializationSettings {}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureInitializationSettings2 {}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureInitializationSettings3 {}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureInitializationSettings4 {}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureInitializationSettings5 {}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureInitializationSettings6 {}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureInitializationSettings7 {}
impl ::core::clone::Clone for IMediaCaptureInitializationSettings7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCapturePauseResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCapturePauseResult {}
impl ::core::clone::Clone for IMediaCapturePauseResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureRelativePanelWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureRelativePanelWatcher {}
impl ::core::clone::Clone for IMediaCaptureRelativePanelWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureSettings {}
impl ::core::clone::Clone for IMediaCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureSettings2 {}
impl ::core::clone::Clone for IMediaCaptureSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureSettings3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureSettings3 {}
impl ::core::clone::Clone for IMediaCaptureSettings3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureStatics {}
impl ::core::clone::Clone for IMediaCaptureStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureStopResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureStopResult {}
impl ::core::clone::Clone for IMediaCaptureStopResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureVideoPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureVideoPreview {}
impl ::core::clone::Clone for IMediaCaptureVideoPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureVideoProfile {}
impl ::core::clone::Clone for IMediaCaptureVideoProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureVideoProfile2 {}
impl ::core::clone::Clone for IMediaCaptureVideoProfile2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureVideoProfileMediaDescription {}
impl ::core::clone::Clone for IMediaCaptureVideoProfileMediaDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaCaptureVideoProfileMediaDescription2 {}
impl ::core::clone::Clone for IMediaCaptureVideoProfileMediaDescription2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOptionalReferencePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOptionalReferencePhotoCapturedEventArgs {}
impl ::core::clone::Clone for IOptionalReferencePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoCapturedEventArgs {}
impl ::core::clone::Clone for IPhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhotoConfirmationCapturedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhotoConfirmationCapturedEventArgs {}
impl ::core::clone::Clone for IPhotoConfirmationCapturedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScreenCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScreenCapture {}
impl ::core::clone::Clone for IScreenCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScreenCaptureStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScreenCaptureStatics {}
impl ::core::clone::Clone for IScreenCaptureStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISourceSuspensionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISourceSuspensionChangedEventArgs {}
impl ::core::clone::Clone for ISourceSuspensionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoStreamConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoStreamConfiguration {}
impl ::core::clone::Clone for IVideoStreamConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KnownVideoProfile(pub i32);
impl KnownVideoProfile {
    pub const VideoRecording: Self = Self(0i32);
    pub const HighQualityPhoto: Self = Self(1i32);
    pub const BalancedVideoAndPhoto: Self = Self(2i32);
    pub const VideoConferencing: Self = Self(3i32);
    pub const PhotoSequence: Self = Self(4i32);
    pub const HighFrameRate: Self = Self(5i32);
    pub const VariablePhotoSequence: Self = Self(6i32);
    pub const HdrWithWcgVideo: Self = Self(7i32);
    pub const HdrWithWcgPhoto: Self = Self(8i32);
    pub const VideoHdr8: Self = Self(9i32);
    pub const CompressedCamera: Self = Self(10i32);
}
impl ::core::marker::Copy for KnownVideoProfile {}
impl ::core::clone::Clone for KnownVideoProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LowLagMediaRecording(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LowLagMediaRecording {}
impl ::core::clone::Clone for LowLagMediaRecording {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LowLagPhotoCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LowLagPhotoCapture {}
impl ::core::clone::Clone for LowLagPhotoCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LowLagPhotoSequenceCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LowLagPhotoSequenceCapture {}
impl ::core::clone::Clone for LowLagPhotoSequenceCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCapture {}
impl ::core::clone::Clone for MediaCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatus(pub i32);
impl MediaCaptureDeviceExclusiveControlStatus {
    pub const ExclusiveControlAvailable: Self = Self(0i32);
    pub const SharedReadOnlyAvailable: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureDeviceExclusiveControlStatus {}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureFailedEventArgs {}
impl ::core::clone::Clone for MediaCaptureFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureFailedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureFailedEventHandler {}
impl ::core::clone::Clone for MediaCaptureFailedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureFocusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureFocusChangedEventArgs {}
impl ::core::clone::Clone for MediaCaptureFocusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureInitializationSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureInitializationSettings {}
impl ::core::clone::Clone for MediaCaptureInitializationSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureMemoryPreference(pub i32);
impl MediaCaptureMemoryPreference {
    pub const Auto: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureMemoryPreference {}
impl ::core::clone::Clone for MediaCaptureMemoryPreference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCapturePauseResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCapturePauseResult {}
impl ::core::clone::Clone for MediaCapturePauseResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureRelativePanelWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureRelativePanelWatcher {}
impl ::core::clone::Clone for MediaCaptureRelativePanelWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureSettings {}
impl ::core::clone::Clone for MediaCaptureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureSharingMode(pub i32);
impl MediaCaptureSharingMode {
    pub const ExclusiveControl: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureSharingMode {}
impl ::core::clone::Clone for MediaCaptureSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureStopResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureStopResult {}
impl ::core::clone::Clone for MediaCaptureStopResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: Self = Self(0i32);
    pub const Overheated: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaCaptureThermalStatus {}
impl ::core::clone::Clone for MediaCaptureThermalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureVideoProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureVideoProfile {}
impl ::core::clone::Clone for MediaCaptureVideoProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCaptureVideoProfileMediaDescription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaCaptureVideoProfileMediaDescription {}
impl ::core::clone::Clone for MediaCaptureVideoProfileMediaDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaCategory(pub i32);
impl MediaCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const GameChat: Self = Self(3i32);
    pub const Speech: Self = Self(4i32);
    pub const FarFieldSpeech: Self = Self(5i32);
    pub const UniformSpeech: Self = Self(6i32);
    pub const VoiceTyping: Self = Self(7i32);
}
impl ::core::marker::Copy for MediaCategory {}
impl ::core::clone::Clone for MediaCategory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: Self = Self(0i32);
    pub const VideoRecord: Self = Self(1i32);
    pub const Audio: Self = Self(2i32);
    pub const Photo: Self = Self(3i32);
    pub const Metadata: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaStreamType {}
impl ::core::clone::Clone for MediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OptionalReferencePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OptionalReferencePhotoCapturedEventArgs {}
impl ::core::clone::Clone for OptionalReferencePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: Self = Self(0i32);
    pub const VideoPreview: Self = Self(1i32);
    pub const Photo: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoCaptureSource {}
impl ::core::clone::Clone for PhotoCaptureSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoCapturedEventArgs {}
impl ::core::clone::Clone for PhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhotoConfirmationCapturedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhotoConfirmationCapturedEventArgs {}
impl ::core::clone::Clone for PhotoConfirmationCapturedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: Self = Self(0i32);
    pub const FiftyHertz: Self = Self(1i32);
    pub const SixtyHertz: Self = Self(2i32);
    pub const Auto: Self = Self(3i32);
}
impl ::core::marker::Copy for PowerlineFrequency {}
impl ::core::clone::Clone for PowerlineFrequency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RecordLimitationExceededEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RecordLimitationExceededEventHandler {}
impl ::core::clone::Clone for RecordLimitationExceededEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScreenCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScreenCapture {}
impl ::core::clone::Clone for ScreenCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SourceSuspensionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SourceSuspensionChangedEventArgs {}
impl ::core::clone::Clone for SourceSuspensionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StreamingCaptureMode(pub i32);
impl StreamingCaptureMode {
    pub const AudioAndVideo: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for StreamingCaptureMode {}
impl ::core::clone::Clone for StreamingCaptureMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoDeviceCharacteristic(pub i32);
impl VideoDeviceCharacteristic {
    pub const AllStreamsIndependent: Self = Self(0i32);
    pub const PreviewRecordStreamsIdentical: Self = Self(1i32);
    pub const PreviewPhotoStreamsIdentical: Self = Self(2i32);
    pub const RecordPhotoStreamsIdentical: Self = Self(3i32);
    pub const AllStreamsIdentical: Self = Self(4i32);
}
impl ::core::marker::Copy for VideoDeviceCharacteristic {}
impl ::core::clone::Clone for VideoDeviceCharacteristic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for VideoRotation {}
impl ::core::clone::Clone for VideoRotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoStreamConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoStreamConfiguration {}
impl ::core::clone::Clone for VideoStreamConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WhiteBalanceGain {
    pub R: f64,
    pub G: f64,
    pub B: f64,
}
impl ::core::marker::Copy for WhiteBalanceGain {}
impl ::core::clone::Clone for WhiteBalanceGain {
    fn clone(&self) -> Self {
        *self
    }
}
