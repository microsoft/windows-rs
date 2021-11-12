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
    pub const Stopped: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(0i32);
    pub const Started: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(1i32);
    pub const Failed: AppBroadcastCameraCaptureState = AppBroadcastCameraCaptureState(2i32);
}
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastCameraOverlayLocation(pub i32);
impl AppBroadcastCameraOverlayLocation {
    pub const TopLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(0i32);
    pub const TopCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(1i32);
    pub const TopRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(2i32);
    pub const MiddleLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(3i32);
    pub const MiddleCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(4i32);
    pub const MiddleRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(5i32);
    pub const BottomLeft: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(6i32);
    pub const BottomCenter: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(7i32);
    pub const BottomRight: AppBroadcastCameraOverlayLocation = AppBroadcastCameraOverlayLocation(8i32);
}
#[repr(transparent)]
pub struct AppBroadcastCameraOverlaySize(pub i32);
impl AppBroadcastCameraOverlaySize {
    pub const Small: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(0i32);
    pub const Medium: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(1i32);
    pub const Large: AppBroadcastCameraOverlaySize = AppBroadcastCameraOverlaySize(2i32);
}
#[repr(transparent)]
pub struct AppBroadcastCaptureTargetType(pub i32);
impl AppBroadcastCaptureTargetType {
    pub const AppView: AppBroadcastCaptureTargetType = AppBroadcastCaptureTargetType(0i32);
    pub const EntireDisplay: AppBroadcastCaptureTargetType = AppBroadcastCaptureTargetType(1i32);
}
#[repr(C)]
pub struct AppBroadcastContract(i32);
#[repr(transparent)]
pub struct AppBroadcastExitBroadcastModeReason(pub i32);
impl AppBroadcastExitBroadcastModeReason {
    pub const NormalExit: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(0i32);
    pub const UserCanceled: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(1i32);
    pub const AuthorizationFail: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(2i32);
    pub const ForegroundAppActivated: AppBroadcastExitBroadcastModeReason = AppBroadcastExitBroadcastModeReason(3i32);
}
#[repr(transparent)]
pub struct AppBroadcastGlobalSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureState(pub i32);
impl AppBroadcastMicrophoneCaptureState {
    pub const Stopped: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(0i32);
    pub const Started: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(1i32);
    pub const Failed: AppBroadcastMicrophoneCaptureState = AppBroadcastMicrophoneCaptureState(2i32);
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
    pub const Unknown: AppBroadcastPlugInState = AppBroadcastPlugInState(0i32);
    pub const Initialized: AppBroadcastPlugInState = AppBroadcastPlugInState(1i32);
    pub const MicrosoftSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(2i32);
    pub const OAuthSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(3i32);
    pub const ProviderSignInRequired: AppBroadcastPlugInState = AppBroadcastPlugInState(4i32);
    pub const InBandwidthTest: AppBroadcastPlugInState = AppBroadcastPlugInState(5i32);
    pub const ReadyToBroadcast: AppBroadcastPlugInState = AppBroadcastPlugInState(6i32);
}
#[repr(transparent)]
pub struct AppBroadcastPlugInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPreviewState(pub i32);
impl AppBroadcastPreviewState {
    pub const Started: AppBroadcastPreviewState = AppBroadcastPreviewState(0i32);
    pub const Stopped: AppBroadcastPreviewState = AppBroadcastPreviewState(1i32);
    pub const Failed: AppBroadcastPreviewState = AppBroadcastPreviewState(2i32);
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
    pub const Success: AppBroadcastSignInResult = AppBroadcastSignInResult(0i32);
    pub const AuthenticationFailed: AppBroadcastSignInResult = AppBroadcastSignInResult(1i32);
    pub const Unauthorized: AppBroadcastSignInResult = AppBroadcastSignInResult(2i32);
    pub const ServiceUnavailable: AppBroadcastSignInResult = AppBroadcastSignInResult(3i32);
    pub const Unknown: AppBroadcastSignInResult = AppBroadcastSignInResult(4i32);
}
#[repr(transparent)]
pub struct AppBroadcastSignInState(pub i32);
impl AppBroadcastSignInState {
    pub const NotSignedIn: AppBroadcastSignInState = AppBroadcastSignInState(0i32);
    pub const MicrosoftSignInInProgress: AppBroadcastSignInState = AppBroadcastSignInState(1i32);
    pub const MicrosoftSignInComplete: AppBroadcastSignInState = AppBroadcastSignInState(2i32);
    pub const OAuthSignInInProgress: AppBroadcastSignInState = AppBroadcastSignInState(3i32);
    pub const OAuthSignInComplete: AppBroadcastSignInState = AppBroadcastSignInState(4i32);
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
    pub const Initializing: AppBroadcastStreamState = AppBroadcastStreamState(0i32);
    pub const StreamReady: AppBroadcastStreamState = AppBroadcastStreamState(1i32);
    pub const Started: AppBroadcastStreamState = AppBroadcastStreamState(2i32);
    pub const Paused: AppBroadcastStreamState = AppBroadcastStreamState(3i32);
    pub const Terminated: AppBroadcastStreamState = AppBroadcastStreamState(4i32);
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
    pub const NormalTermination: AppBroadcastTerminationReason = AppBroadcastTerminationReason(0i32);
    pub const LostConnectionToService: AppBroadcastTerminationReason = AppBroadcastTerminationReason(1i32);
    pub const NoNetworkConnectivity: AppBroadcastTerminationReason = AppBroadcastTerminationReason(2i32);
    pub const ServiceAbort: AppBroadcastTerminationReason = AppBroadcastTerminationReason(3i32);
    pub const ServiceError: AppBroadcastTerminationReason = AppBroadcastTerminationReason(4i32);
    pub const ServiceUnavailable: AppBroadcastTerminationReason = AppBroadcastTerminationReason(5i32);
    pub const InternalError: AppBroadcastTerminationReason = AppBroadcastTerminationReason(6i32);
    pub const UnsupportedFormat: AppBroadcastTerminationReason = AppBroadcastTerminationReason(7i32);
    pub const BackgroundTaskTerminated: AppBroadcastTerminationReason = AppBroadcastTerminationReason(8i32);
    pub const BackgroundTaskUnresponsive: AppBroadcastTerminationReason = AppBroadcastTerminationReason(9i32);
}
#[repr(transparent)]
pub struct AppBroadcastTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingBitrateMode(pub i32);
impl AppBroadcastVideoEncodingBitrateMode {
    pub const Custom: AppBroadcastVideoEncodingBitrateMode = AppBroadcastVideoEncodingBitrateMode(0i32);
    pub const Auto: AppBroadcastVideoEncodingBitrateMode = AppBroadcastVideoEncodingBitrateMode(1i32);
}
#[repr(transparent)]
pub struct AppBroadcastVideoEncodingResolutionMode(pub i32);
impl AppBroadcastVideoEncodingResolutionMode {
    pub const Custom: AppBroadcastVideoEncodingResolutionMode = AppBroadcastVideoEncodingResolutionMode(0i32);
    pub const Auto: AppBroadcastVideoEncodingResolutionMode = AppBroadcastVideoEncodingResolutionMode(1i32);
}
#[repr(transparent)]
pub struct AppBroadcastViewerCountChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureAlternateShortcutKeys(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AppCaptureContract(i32);
#[repr(transparent)]
pub struct AppCaptureDurationGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureFileGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureHistoricalBufferLengthUnit(pub i32);
impl AppCaptureHistoricalBufferLengthUnit {
    pub const Megabytes: AppCaptureHistoricalBufferLengthUnit = AppCaptureHistoricalBufferLengthUnit(0i32);
    pub const Seconds: AppCaptureHistoricalBufferLengthUnit = AppCaptureHistoricalBufferLengthUnit(1i32);
}
#[repr(C)]
pub struct AppCaptureMetadataContract(i32);
#[repr(transparent)]
pub struct AppCaptureMetadataPriority(pub i32);
impl AppCaptureMetadataPriority {
    pub const Informational: AppCaptureMetadataPriority = AppCaptureMetadataPriority(0i32);
    pub const Important: AppCaptureMetadataPriority = AppCaptureMetadataPriority(1i32);
}
#[repr(transparent)]
pub struct AppCaptureMetadataWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureState(pub i32);
impl AppCaptureMicrophoneCaptureState {
    pub const Stopped: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(0i32);
    pub const Started: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(1i32);
    pub const Failed: AppCaptureMicrophoneCaptureState = AppCaptureMicrophoneCaptureState(2i32);
}
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureRecordOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureRecordingState(pub i32);
impl AppCaptureRecordingState {
    pub const InProgress: AppCaptureRecordingState = AppCaptureRecordingState(0i32);
    pub const Completed: AppCaptureRecordingState = AppCaptureRecordingState(1i32);
    pub const Failed: AppCaptureRecordingState = AppCaptureRecordingState(2i32);
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
    pub const Custom: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(0i32);
    pub const High: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(1i32);
    pub const Standard: AppCaptureVideoEncodingBitrateMode = AppCaptureVideoEncodingBitrateMode(2i32);
}
#[repr(transparent)]
pub struct AppCaptureVideoEncodingFrameRateMode(pub i32);
impl AppCaptureVideoEncodingFrameRateMode {
    pub const Standard: AppCaptureVideoEncodingFrameRateMode = AppCaptureVideoEncodingFrameRateMode(0i32);
    pub const High: AppCaptureVideoEncodingFrameRateMode = AppCaptureVideoEncodingFrameRateMode(1i32);
}
#[repr(transparent)]
pub struct AppCaptureVideoEncodingResolutionMode(pub i32);
impl AppCaptureVideoEncodingResolutionMode {
    pub const Custom: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(0i32);
    pub const High: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(1i32);
    pub const Standard: AppCaptureVideoEncodingResolutionMode = AppCaptureVideoEncodingResolutionMode(2i32);
}
#[repr(transparent)]
pub struct CameraCaptureUI(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CameraCaptureUIContract(i32);
#[repr(transparent)]
pub struct CameraCaptureUIMaxPhotoResolution(pub i32);
impl CameraCaptureUIMaxPhotoResolution {
    pub const HighestAvailable: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(0i32);
    pub const VerySmallQvga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(1i32);
    pub const SmallVga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(2i32);
    pub const MediumXga: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(3i32);
    pub const Large3M: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(4i32);
    pub const VeryLarge5M: CameraCaptureUIMaxPhotoResolution = CameraCaptureUIMaxPhotoResolution(5i32);
}
#[repr(transparent)]
pub struct CameraCaptureUIMaxVideoResolution(pub i32);
impl CameraCaptureUIMaxVideoResolution {
    pub const HighestAvailable: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(0i32);
    pub const LowDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(1i32);
    pub const StandardDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(2i32);
    pub const HighDefinition: CameraCaptureUIMaxVideoResolution = CameraCaptureUIMaxVideoResolution(3i32);
}
#[repr(transparent)]
pub struct CameraCaptureUIMode(pub i32);
impl CameraCaptureUIMode {
    pub const PhotoOrVideo: CameraCaptureUIMode = CameraCaptureUIMode(0i32);
    pub const Photo: CameraCaptureUIMode = CameraCaptureUIMode(1i32);
    pub const Video: CameraCaptureUIMode = CameraCaptureUIMode(2i32);
}
#[repr(transparent)]
pub struct CameraCaptureUIPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraCaptureUIPhotoFormat(pub i32);
impl CameraCaptureUIPhotoFormat {
    pub const Jpeg: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(0i32);
    pub const Png: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(1i32);
    pub const JpegXR: CameraCaptureUIPhotoFormat = CameraCaptureUIPhotoFormat(2i32);
}
#[repr(transparent)]
pub struct CameraCaptureUIVideoCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CameraCaptureUIVideoFormat(pub i32);
impl CameraCaptureUIVideoFormat {
    pub const Mp4: CameraCaptureUIVideoFormat = CameraCaptureUIVideoFormat(0i32);
    pub const Wmv: CameraCaptureUIVideoFormat = CameraCaptureUIVideoFormat(1i32);
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
    pub const SignInRequired: ForegroundActivationArgument = ForegroundActivationArgument(0i32);
    pub const MoreSettings: ForegroundActivationArgument = ForegroundActivationArgument(1i32);
}
#[repr(transparent)]
pub struct GameBarCommand(pub i32);
impl GameBarCommand {
    pub const OpenGameBar: GameBarCommand = GameBarCommand(0i32);
    pub const RecordHistoricalBuffer: GameBarCommand = GameBarCommand(1i32);
    pub const ToggleStartStopRecord: GameBarCommand = GameBarCommand(2i32);
    pub const StartRecord: GameBarCommand = GameBarCommand(3i32);
    pub const StopRecord: GameBarCommand = GameBarCommand(4i32);
    pub const TakeScreenshot: GameBarCommand = GameBarCommand(5i32);
    pub const StartBroadcast: GameBarCommand = GameBarCommand(6i32);
    pub const StopBroadcast: GameBarCommand = GameBarCommand(7i32);
    pub const PauseBroadcast: GameBarCommand = GameBarCommand(8i32);
    pub const ResumeBroadcast: GameBarCommand = GameBarCommand(9i32);
    pub const ToggleStartStopBroadcast: GameBarCommand = GameBarCommand(10i32);
    pub const ToggleMicrophoneCapture: GameBarCommand = GameBarCommand(11i32);
    pub const ToggleCameraCapture: GameBarCommand = GameBarCommand(12i32);
    pub const ToggleRecordingIndicator: GameBarCommand = GameBarCommand(13i32);
}
#[repr(transparent)]
pub struct GameBarCommandOrigin(pub i32);
impl GameBarCommandOrigin {
    pub const ShortcutKey: GameBarCommandOrigin = GameBarCommandOrigin(0i32);
    pub const Cortana: GameBarCommandOrigin = GameBarCommandOrigin(1i32);
    pub const AppCommand: GameBarCommandOrigin = GameBarCommandOrigin(2i32);
}
#[repr(C)]
pub struct GameBarContract(i32);
#[repr(transparent)]
pub struct GameBarServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesCommandEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesDisplayMode(pub i32);
impl GameBarServicesDisplayMode {
    pub const Windowed: GameBarServicesDisplayMode = GameBarServicesDisplayMode(0i32);
    pub const FullScreenExclusive: GameBarServicesDisplayMode = GameBarServicesDisplayMode(1i32);
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
    pub const EnabledBySystem: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(0i32);
    pub const EnabledByUser: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(1i32);
    pub const NotEnabled: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(2i32);
    pub const ProhibitedBySystem: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(3i32);
    pub const ProhibitedByPublisher: GameBarTargetCapturePolicy = GameBarTargetCapturePolicy(4i32);
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
    pub const VideoRecording: KnownVideoProfile = KnownVideoProfile(0i32);
    pub const HighQualityPhoto: KnownVideoProfile = KnownVideoProfile(1i32);
    pub const BalancedVideoAndPhoto: KnownVideoProfile = KnownVideoProfile(2i32);
    pub const VideoConferencing: KnownVideoProfile = KnownVideoProfile(3i32);
    pub const PhotoSequence: KnownVideoProfile = KnownVideoProfile(4i32);
    pub const HighFrameRate: KnownVideoProfile = KnownVideoProfile(5i32);
    pub const VariablePhotoSequence: KnownVideoProfile = KnownVideoProfile(6i32);
    pub const HdrWithWcgVideo: KnownVideoProfile = KnownVideoProfile(7i32);
    pub const HdrWithWcgPhoto: KnownVideoProfile = KnownVideoProfile(8i32);
    pub const VideoHdr8: KnownVideoProfile = KnownVideoProfile(9i32);
    pub const CompressedCamera: KnownVideoProfile = KnownVideoProfile(10i32);
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
    pub const ExclusiveControlAvailable: MediaCaptureDeviceExclusiveControlStatus = MediaCaptureDeviceExclusiveControlStatus(0i32);
    pub const SharedReadOnlyAvailable: MediaCaptureDeviceExclusiveControlStatus = MediaCaptureDeviceExclusiveControlStatus(1i32);
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
    pub const Auto: MediaCaptureMemoryPreference = MediaCaptureMemoryPreference(0i32);
    pub const Cpu: MediaCaptureMemoryPreference = MediaCaptureMemoryPreference(1i32);
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
    pub const ExclusiveControl: MediaCaptureSharingMode = MediaCaptureSharingMode(0i32);
    pub const SharedReadOnly: MediaCaptureSharingMode = MediaCaptureSharingMode(1i32);
}
#[repr(transparent)]
pub struct MediaCaptureStopResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureThermalStatus(pub i32);
impl MediaCaptureThermalStatus {
    pub const Normal: MediaCaptureThermalStatus = MediaCaptureThermalStatus(0i32);
    pub const Overheated: MediaCaptureThermalStatus = MediaCaptureThermalStatus(1i32);
}
#[repr(transparent)]
pub struct MediaCaptureVideoProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureVideoProfileMediaDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCategory(pub i32);
impl MediaCategory {
    pub const Other: MediaCategory = MediaCategory(0i32);
    pub const Communications: MediaCategory = MediaCategory(1i32);
    pub const Media: MediaCategory = MediaCategory(2i32);
    pub const GameChat: MediaCategory = MediaCategory(3i32);
    pub const Speech: MediaCategory = MediaCategory(4i32);
    pub const FarFieldSpeech: MediaCategory = MediaCategory(5i32);
    pub const UniformSpeech: MediaCategory = MediaCategory(6i32);
    pub const VoiceTyping: MediaCategory = MediaCategory(7i32);
}
#[repr(transparent)]
pub struct MediaStreamType(pub i32);
impl MediaStreamType {
    pub const VideoPreview: MediaStreamType = MediaStreamType(0i32);
    pub const VideoRecord: MediaStreamType = MediaStreamType(1i32);
    pub const Audio: MediaStreamType = MediaStreamType(2i32);
    pub const Photo: MediaStreamType = MediaStreamType(3i32);
    pub const Metadata: MediaStreamType = MediaStreamType(4i32);
}
#[repr(transparent)]
pub struct OptionalReferencePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoCaptureSource(pub i32);
impl PhotoCaptureSource {
    pub const Auto: PhotoCaptureSource = PhotoCaptureSource(0i32);
    pub const VideoPreview: PhotoCaptureSource = PhotoCaptureSource(1i32);
    pub const Photo: PhotoCaptureSource = PhotoCaptureSource(2i32);
}
#[repr(transparent)]
pub struct PhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoConfirmationCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PowerlineFrequency(pub i32);
impl PowerlineFrequency {
    pub const Disabled: PowerlineFrequency = PowerlineFrequency(0i32);
    pub const FiftyHertz: PowerlineFrequency = PowerlineFrequency(1i32);
    pub const SixtyHertz: PowerlineFrequency = PowerlineFrequency(2i32);
    pub const Auto: PowerlineFrequency = PowerlineFrequency(3i32);
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
    pub const AudioAndVideo: StreamingCaptureMode = StreamingCaptureMode(0i32);
    pub const Audio: StreamingCaptureMode = StreamingCaptureMode(1i32);
    pub const Video: StreamingCaptureMode = StreamingCaptureMode(2i32);
}
#[repr(transparent)]
pub struct VideoDeviceCharacteristic(pub i32);
impl VideoDeviceCharacteristic {
    pub const AllStreamsIndependent: VideoDeviceCharacteristic = VideoDeviceCharacteristic(0i32);
    pub const PreviewRecordStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(1i32);
    pub const PreviewPhotoStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(2i32);
    pub const RecordPhotoStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(3i32);
    pub const AllStreamsIdentical: VideoDeviceCharacteristic = VideoDeviceCharacteristic(4i32);
}
#[repr(transparent)]
pub struct VideoRotation(pub i32);
impl VideoRotation {
    pub const None: VideoRotation = VideoRotation(0i32);
    pub const Clockwise90Degrees: VideoRotation = VideoRotation(1i32);
    pub const Clockwise180Degrees: VideoRotation = VideoRotation(2i32);
    pub const Clockwise270Degrees: VideoRotation = VideoRotation(3i32);
}
#[repr(transparent)]
pub struct VideoStreamConfiguration(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WhiteBalanceGain(i32);
