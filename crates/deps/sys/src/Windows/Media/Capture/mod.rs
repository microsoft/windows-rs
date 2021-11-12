#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdvancedCapturedPhoto(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdvancedPhotoCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastBackgroundService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceSignInInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceStreamInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureState(pub i32);
impl AppBroadcastCameraCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
    pub const AuthorizationFail: Self = Self(2i32);
    pub const ForegroundAppActivated: Self = Self(3i32);
}
#[repr(transparent)]
pub struct AppBroadcastGlobalSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPlugIn(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPlugInManager(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct AppBroadcastPlugInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AppBroadcastPreviewStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastProviderSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastSignInResult(pub i32);
impl AppBroadcastSignInResult {
    pub const Success: Self = Self(0i32);
    pub const AuthenticationFailed: Self = Self(1i32);
    pub const Unauthorized: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
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
#[repr(transparent)]
pub struct AppBroadcastSignInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastStreamAudioFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastStreamAudioHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastStreamReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastStreamState(pub i32);
impl AppBroadcastStreamState {
    pub const Initializing: Self = Self(0i32);
    pub const StreamReady: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Paused: Self = Self(3i32);
    pub const Terminated: Self = Self(4i32);
}
#[repr(transparent)]
pub struct AppBroadcastStreamStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastStreamVideoFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastStreamVideoHeader(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct AppBroadcastTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AppBroadcastViewerCountChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureAlternateShortcutKeys(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureDurationGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureFileGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: Self = Self(0i32);
    pub const Seconds: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AppCaptureMetadataWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureRecordOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AppCaptureRecordingStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureVideoEncodingBitrateMode(pub i32);
impl AppCaptureVideoEncodingBitrateMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
}
#[repr(transparent)]
pub struct CameraCaptureUI(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: Self = Self(0i32);
    pub const LowDefinition: Self = Self(1i32);
    pub const StandardDefinition: Self = Self(2i32);
    pub const HighDefinition: Self = Self(3i32);
}
#[repr(transparent)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[repr(transparent)]
pub struct CameraCaptureUIPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: Self = Self(0i32);
    pub const Png: Self = Self(1i32);
    pub const JpegXR: Self = Self(2i32);
}
#[repr(transparent)]
pub struct CameraCaptureUIVideoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: Self = Self(0i32);
    pub const Wmv: Self = Self(1i32);
}
#[repr(transparent)]
pub struct CapturedFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CapturedFrameControlValues(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CapturedPhoto(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ForegroundActivationArgument(pub i32);
impl ForegroundActivationArgument {
    pub const SignInRequired: Self = Self(0i32);
    pub const MoreSettings: Self = Self(1i32);
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
#[repr(transparent)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: Self = Self(0i32);
    pub const Cortana: Self = Self(1i32);
    pub const AppCommand: Self = Self(2i32);
}
#[repr(transparent)]
pub struct GameBarServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesCommandEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: Self = Self(0i32);
    pub const FullScreenExclusive: Self = Self(1i32);
}
#[repr(transparent)]
pub struct GameBarServicesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesTargetInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarTargetCapturePolicy(pub i32);
impl GameBarTargetCapturePolicy {
    pub const EnabledBySystem: Self = Self(0i32);
    pub const EnabledByUser: Self = Self(1i32);
    pub const NotEnabled: Self = Self(2i32);
    pub const ProhibitedBySystem: Self = Self(3i32);
    pub const ProhibitedByPublisher: Self = Self(4i32);
}
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvancedPhotoCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastGlobalSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPlugIn(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPlugInManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPlugInManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPlugInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPreviewStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastProviderSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastSignInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastStreamReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastStreamStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastViewerCountChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureDurationGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureFileGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureMetadataWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureRecordOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureRecordingStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureSettings3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureSettings4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureSettings5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCaptureStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraCaptureUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraCaptureUIPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraCaptureUIVideoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraOptionsUIStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICapturedFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICapturedFrame2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICapturedFrameControlValues(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICapturedFrameControlValues2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICapturedFrameWithSoftwareBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICapturedPhoto(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameBarServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameBarServicesCommandEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameBarServicesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameBarServicesManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameBarServicesTargetInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLagMediaRecording(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLagMediaRecording2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLagMediaRecording3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLagPhotoCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILowLagPhotoSequenceCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCapture2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCapture3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCapture4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCapture5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCapture6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCapture7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureFocusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCapturePauseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureRelativePanelWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureSettings3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureStopResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureVideoPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOptionalReferencePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoConfirmationCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScreenCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScreenCaptureStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISourceSuspensionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoStreamConfiguration(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct LowLagMediaRecording(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLagPhotoCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLagPhotoSequenceCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatus(pub i32);
impl MediaCaptureDeviceExclusiveControlStatus {
    pub const ExclusiveControlAvailable: Self = Self(0i32);
    pub const SharedReadOnlyAvailable: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureFailedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureFocusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureInitializationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureMemoryPreference(pub i32);
impl MediaCaptureMemoryPreference {
    pub const Auto: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MediaCapturePauseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureRelativePanelWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureSharingMode(pub i32);
impl MediaCaptureSharingMode {
    pub const ExclusiveControl: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MediaCaptureStopResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: Self = Self(0i32);
    pub const Overheated: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MediaCaptureVideoProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureVideoProfileMediaDescription(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: Self = Self(0i32);
    pub const VideoRecord: Self = Self(1i32);
    pub const Audio: Self = Self(2i32);
    pub const Photo: Self = Self(3i32);
    pub const Metadata: Self = Self(4i32);
}
#[repr(transparent)]
pub struct OptionalReferencePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: Self = Self(0i32);
    pub const VideoPreview: Self = Self(1i32);
    pub const Photo: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoConfirmationCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: Self = Self(0i32);
    pub const FiftyHertz: Self = Self(1i32);
    pub const SixtyHertz: Self = Self(2i32);
    pub const Auto: Self = Self(3i32);
}
#[repr(transparent)]
pub struct RecordLimitationExceededEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScreenCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SourceSuspensionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreamingCaptureMode(pub i32);
impl StreamingCaptureMode {
    pub const AudioAndVideo: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
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
#[repr(transparent)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
#[repr(transparent)]
pub struct VideoStreamConfiguration(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WhiteBalanceGain(i32);
