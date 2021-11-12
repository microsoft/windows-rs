#![allow(non_snake_case, non_camel_case_types)]
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
pub struct AppBroadcastCameraCaptureState(i32);
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct AppBroadcastCameraOverlayLocation(i32);
pub struct AppBroadcastCameraOverlaySize(i32);
pub struct AppBroadcastCaptureTargetType(i32);
pub struct AppBroadcastContract(i32);
pub struct AppBroadcastExitBroadcastModeReason(i32);
#[repr(transparent)]
pub struct AppBroadcastGlobalSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastManager(pub *mut ::core::ffi::c_void);
pub struct AppBroadcastMicrophoneCaptureState(i32);
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPlugIn(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPlugInManager(pub *mut ::core::ffi::c_void);
pub struct AppBroadcastPlugInState(i32);
#[repr(transparent)]
pub struct AppBroadcastPlugInStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastPreview(pub *mut ::core::ffi::c_void);
pub struct AppBroadcastPreviewState(i32);
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
pub struct AppBroadcastSignInResult(i32);
pub struct AppBroadcastSignInState(i32);
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
pub struct AppBroadcastStreamState(i32);
#[repr(transparent)]
pub struct AppBroadcastStreamStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastStreamVideoFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastStreamVideoHeader(pub *mut ::core::ffi::c_void);
pub struct AppBroadcastTerminationReason(i32);
#[repr(transparent)]
pub struct AppBroadcastTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct AppBroadcastVideoEncodingBitrateMode(i32);
pub struct AppBroadcastVideoEncodingResolutionMode(i32);
#[repr(transparent)]
pub struct AppBroadcastViewerCountChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureAlternateShortcutKeys(pub *mut ::core::ffi::c_void);
pub struct AppCaptureContract(i32);
#[repr(transparent)]
pub struct AppCaptureDurationGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureFileGeneratedEventArgs(pub *mut ::core::ffi::c_void);
pub struct AppCaptureHistoricalBufferLengthUnit(i32);
#[repr(transparent)]
pub struct AppCaptureManager(pub *mut ::core::ffi::c_void);
pub struct AppCaptureMetadataContract(i32);
pub struct AppCaptureMetadataPriority(i32);
#[repr(transparent)]
pub struct AppCaptureMetadataWriter(pub *mut ::core::ffi::c_void);
pub struct AppCaptureMicrophoneCaptureState(i32);
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureRecordOperation(pub *mut ::core::ffi::c_void);
pub struct AppCaptureRecordingState(i32);
#[repr(transparent)]
pub struct AppCaptureRecordingStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCaptureState(pub *mut ::core::ffi::c_void);
pub struct AppCaptureVideoEncodingBitrateMode(i32);
pub struct AppCaptureVideoEncodingFrameRateMode(i32);
pub struct AppCaptureVideoEncodingResolutionMode(i32);
#[repr(transparent)]
pub struct CameraCaptureUI(pub *mut ::core::ffi::c_void);
pub struct CameraCaptureUIContract(i32);
pub struct CameraCaptureUIMaxPhotoResolution(i32);
pub struct CameraCaptureUIMaxVideoResolution(i32);
pub struct CameraCaptureUIMode(i32);
#[repr(transparent)]
pub struct CameraCaptureUIPhotoCaptureSettings(pub *mut ::core::ffi::c_void);
pub struct CameraCaptureUIPhotoFormat(i32);
#[repr(transparent)]
pub struct CameraCaptureUIVideoCaptureSettings(pub *mut ::core::ffi::c_void);
pub struct CameraCaptureUIVideoFormat(i32);
#[repr(transparent)]
pub struct CameraOptionsUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CapturedFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CapturedFrameControlValues(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CapturedPhoto(pub *mut ::core::ffi::c_void);
pub struct ForegroundActivationArgument(i32);
pub struct GameBarCommand(i32);
pub struct GameBarCommandOrigin(i32);
pub struct GameBarContract(i32);
#[repr(transparent)]
pub struct GameBarServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesCommandEventArgs(pub *mut ::core::ffi::c_void);
pub struct GameBarServicesDisplayMode(i32);
#[repr(transparent)]
pub struct GameBarServicesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GameBarServicesTargetInfo(pub *mut ::core::ffi::c_void);
pub struct GameBarTargetCapturePolicy(i32);
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
pub struct KnownVideoProfile(i32);
#[repr(transparent)]
pub struct LowLagMediaRecording(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLagPhotoCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LowLagPhotoSequenceCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCapture(pub *mut ::core::ffi::c_void);
pub struct MediaCaptureDeviceExclusiveControlStatus(i32);
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
pub struct MediaCaptureMemoryPreference(i32);
#[repr(transparent)]
pub struct MediaCapturePauseResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureRelativePanelWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureSettings(pub *mut ::core::ffi::c_void);
pub struct MediaCaptureSharingMode(i32);
#[repr(transparent)]
pub struct MediaCaptureStopResult(pub *mut ::core::ffi::c_void);
pub struct MediaCaptureThermalStatus(i32);
#[repr(transparent)]
pub struct MediaCaptureVideoProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaCaptureVideoProfileMediaDescription(pub *mut ::core::ffi::c_void);
pub struct MediaCategory(i32);
pub struct MediaStreamType(i32);
#[repr(transparent)]
pub struct OptionalReferencePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
pub struct PhotoCaptureSource(i32);
#[repr(transparent)]
pub struct PhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoConfirmationCapturedEventArgs(pub *mut ::core::ffi::c_void);
pub struct PowerlineFrequency(i32);
#[repr(transparent)]
pub struct RecordLimitationExceededEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScreenCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SourceSuspensionChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct StreamingCaptureMode(i32);
pub struct VideoDeviceCharacteristic(i32);
pub struct VideoRotation(i32);
#[repr(transparent)]
pub struct VideoStreamConfiguration(pub *mut ::core::ffi::c_void);
pub struct WhiteBalanceGain(i32);
