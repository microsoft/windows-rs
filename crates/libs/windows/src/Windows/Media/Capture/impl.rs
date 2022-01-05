#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedCapturedPhotoImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn Mode(&self) -> ::windows::core::Result<super::Devices::AdvancedPhotoMode>;
    fn Context(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedCapturedPhoto2Impl: Sized {
    fn FrameBoundsRelativeToReferencePhoto(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedPhotoCaptureImpl: Sized {
    fn CaptureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>;
    fn CaptureWithContextAsync(&self, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>;
    fn OptionalReferencePhotoCaptured(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOptionalReferencePhotoCaptured(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllPhotosCaptured(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAllPhotosCaptured(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceImpl: Sized {
    fn SetPlugInState(&self, value: AppBroadcastPlugInState) -> ::windows::core::Result<()>;
    fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState>;
    fn SetSignInInfo(&self, value: &::core::option::Option<AppBroadcastBackgroundServiceSignInInfo>) -> ::windows::core::Result<()>;
    fn SignInInfo(&self) -> ::windows::core::Result<AppBroadcastBackgroundServiceSignInInfo>;
    fn SetStreamInfo(&self, value: &::core::option::Option<AppBroadcastBackgroundServiceStreamInfo>) -> ::windows::core::Result<()>;
    fn StreamInfo(&self) -> ::windows::core::Result<AppBroadcastBackgroundServiceStreamInfo>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetViewerCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn ViewerCount(&self) -> ::windows::core::Result<u32>;
    fn TerminateBroadcast(&self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::Result<()>;
    fn HeartbeatRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeartbeatRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundService2Impl: Sized {
    fn SetBroadcastTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastChannel(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastChannel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastTitleChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastTitleChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BroadcastLanguageChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastLanguageChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BroadcastChannelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBroadcastChannelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceSignInInfoImpl: Sized {
    fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn SetOAuthRequestUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OAuthRequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetOAuthCallbackUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OAuthCallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
    fn SetUserName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignInStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSignInStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceSignInInfo2Impl: Sized {
    fn UserNameChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserNameChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceStreamInfoImpl: Sized {
    fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState>;
    fn SetDesiredVideoEncodingBitrate(&self, value: u64) -> ::windows::core::Result<()>;
    fn DesiredVideoEncodingBitrate(&self) -> ::windows::core::Result<u64>;
    fn SetBandwidthTestBitrate(&self, value: u64) -> ::windows::core::Result<()>;
    fn BandwidthTestBitrate(&self) -> ::windows::core::Result<u64>;
    fn SetAudioCodec(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AudioCodec(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BroadcastStreamReader(&self) -> ::windows::core::Result<AppBroadcastStreamReader>;
    fn StreamStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStreamStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoEncodingResolutionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoEncodingBitrateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastBackgroundServiceStreamInfo2Impl: Sized {
    fn ReportProblemWithStream(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastCameraCaptureStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AppBroadcastCameraCaptureState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastGlobalSettingsImpl: Sized {
    fn IsBroadcastEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledByPolicy(&self) -> ::windows::core::Result<bool>;
    fn IsGpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn HasHardwareEncoder(&self) -> ::windows::core::Result<bool>;
    fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAudioCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool>;
    fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEchoCancellationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSystemAudioGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn SystemAudioGain(&self) -> ::windows::core::Result<f64>;
    fn SetMicrophoneGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn MicrophoneGain(&self) -> ::windows::core::Result<f64>;
    fn SetIsCameraCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCameraCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool>;
    fn SetSelectedCameraId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SelectedCameraId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCameraOverlayLocation(&self, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::Result<()>;
    fn CameraOverlayLocation(&self) -> ::windows::core::Result<AppBroadcastCameraOverlayLocation>;
    fn SetCameraOverlaySize(&self, value: AppBroadcastCameraOverlaySize) -> ::windows::core::Result<()>;
    fn CameraOverlaySize(&self) -> ::windows::core::Result<AppBroadcastCameraOverlaySize>;
    fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCursorImageCaptureEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastHeartbeatRequestedEventArgsImpl: Sized {
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastManagerStaticsImpl: Sized {
    fn GetGlobalSettings(&self) -> ::windows::core::Result<AppBroadcastGlobalSettings>;
    fn ApplyGlobalSettings(&self, value: &::core::option::Option<AppBroadcastGlobalSettings>) -> ::windows::core::Result<()>;
    fn GetProviderSettings(&self) -> ::windows::core::Result<AppBroadcastProviderSettings>;
    fn ApplyProviderSettings(&self, value: &::core::option::Option<AppBroadcastProviderSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastMicrophoneCaptureStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInImpl: Sized {
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderSettings(&self) -> ::windows::core::Result<AppBroadcastProviderSettings>;
    fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInManagerImpl: Sized {
    fn IsBroadcastProviderAvailable(&self) -> ::windows::core::Result<bool>;
    fn PlugInList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>>;
    fn DefaultPlugIn(&self) -> ::windows::core::Result<AppBroadcastPlugIn>;
    fn SetDefaultPlugIn(&self, value: &::core::option::Option<AppBroadcastPlugIn>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppBroadcastPlugInManager>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<AppBroadcastPlugInManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPlugInStateChangedEventArgsImpl: Sized {
    fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewImpl: Sized {
    fn StopPreview(&self) -> ::windows::core::Result<()>;
    fn PreviewState(&self) -> ::windows::core::Result<AppBroadcastPreviewState>;
    fn ErrorCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PreviewStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewStreamReader(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamReader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStateChangedEventArgsImpl: Sized {
    fn PreviewState(&self) -> ::windows::core::Result<AppBroadcastPreviewState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStreamReaderImpl: Sized {
    fn VideoWidth(&self) -> ::windows::core::Result<u32>;
    fn VideoHeight(&self) -> ::windows::core::Result<u32>;
    fn VideoStride(&self) -> ::windows::core::Result<u32>;
    fn VideoBitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn VideoBitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode>;
    fn TryGetNextVideoFrame(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoFrame>;
    fn VideoFrameArrived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStreamVideoFrameImpl: Sized {
    fn VideoHeader(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoHeader>;
    fn VideoBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastPreviewStreamVideoHeaderImpl: Sized {
    fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn FrameId(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastProviderSettingsImpl: Sized {
    fn SetDefaultBroadcastTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DefaultBroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn AudioEncodingBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingHeight(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingWidth(&self) -> ::windows::core::Result<u32>;
    fn SetVideoEncodingBitrateMode(&self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateMode(&self) -> ::windows::core::Result<AppBroadcastVideoEncodingBitrateMode>;
    fn SetVideoEncodingResolutionMode(&self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionMode(&self) -> ::windows::core::Result<AppBroadcastVideoEncodingResolutionMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastServicesImpl: Sized {
    fn CaptureTargetType(&self) -> ::windows::core::Result<AppBroadcastCaptureTargetType>;
    fn SetCaptureTargetType(&self, value: AppBroadcastCaptureTargetType) -> ::windows::core::Result<()>;
    fn BroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BroadcastLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetBroadcastLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanCapture(&self) -> ::windows::core::Result<bool>;
    fn EnterBroadcastModeAsync(&self, plugin: &::core::option::Option<AppBroadcastPlugIn>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn ExitBroadcastMode(&self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::Result<()>;
    fn StartBroadcast(&self) -> ::windows::core::Result<()>;
    fn PauseBroadcast(&self) -> ::windows::core::Result<()>;
    fn ResumeBroadcast(&self) -> ::windows::core::Result<()>;
    fn StartPreview(&self, desiredsize: &super::super::Foundation::Size) -> ::windows::core::Result<AppBroadcastPreview>;
    fn State(&self) -> ::windows::core::Result<AppBroadcastState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastSignInStateChangedEventArgsImpl: Sized {
    fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn Result(&self) -> ::windows::core::Result<AppBroadcastSignInResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStateImpl: Sized {
    fn IsCaptureTargetRunning(&self) -> ::windows::core::Result<bool>;
    fn ViewerCount(&self) -> ::windows::core::Result<u32>;
    fn ShouldCaptureMicrophone(&self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::core::Result<()>;
    fn RestartMicrophoneCapture(&self) -> ::windows::core::Result<()>;
    fn ShouldCaptureCamera(&self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureCamera(&self, value: bool) -> ::windows::core::Result<()>;
    fn RestartCameraCapture(&self) -> ::windows::core::Result<()>;
    fn EncodedVideoSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MicrophoneCaptureState(&self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState>;
    fn MicrophoneCaptureError(&self) -> ::windows::core::Result<u32>;
    fn CameraCaptureState(&self) -> ::windows::core::Result<AppBroadcastCameraCaptureState>;
    fn CameraCaptureError(&self) -> ::windows::core::Result<u32>;
    fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState>;
    fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState>;
    fn OAuthRequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn OAuthCallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult>;
    fn SetAuthenticationResult(&self, value: &::core::option::Option<super::super::Security::Authentication::Web::WebAuthenticationResult>) -> ::windows::core::Result<()>;
    fn SetSignInState(&self, value: AppBroadcastSignInState) -> ::windows::core::Result<()>;
    fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState>;
    fn TerminationReason(&self) -> ::windows::core::Result<AppBroadcastTerminationReason>;
    fn TerminationReasonPlugInSpecific(&self) -> ::windows::core::Result<u32>;
    fn ViewerCountChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveViewerCountChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MicrophoneCaptureStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMicrophoneCaptureStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraCaptureStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraCaptureStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlugInStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePlugInStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StreamStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStreamStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureTargetClosed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureTargetClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamAudioFrameImpl: Sized {
    fn AudioHeader(&self) -> ::windows::core::Result<AppBroadcastStreamAudioHeader>;
    fn AudioBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamAudioHeaderImpl: Sized {
    fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn HasDiscontinuity(&self) -> ::windows::core::Result<bool>;
    fn FrameId(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamReaderImpl: Sized {
    fn AudioChannels(&self) -> ::windows::core::Result<u32>;
    fn AudioSampleRate(&self) -> ::windows::core::Result<u32>;
    fn AudioAacSequence(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn AudioBitrate(&self) -> ::windows::core::Result<u32>;
    fn TryGetNextAudioFrame(&self) -> ::windows::core::Result<AppBroadcastStreamAudioFrame>;
    fn VideoWidth(&self) -> ::windows::core::Result<u32>;
    fn VideoHeight(&self) -> ::windows::core::Result<u32>;
    fn VideoBitrate(&self) -> ::windows::core::Result<u32>;
    fn TryGetNextVideoFrame(&self) -> ::windows::core::Result<AppBroadcastStreamVideoFrame>;
    fn AudioFrameArrived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoFrameArrived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamStateChangedEventArgsImpl: Sized {
    fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamVideoFrameImpl: Sized {
    fn VideoHeader(&self) -> ::windows::core::Result<AppBroadcastStreamVideoHeader>;
    fn VideoBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastStreamVideoHeaderImpl: Sized {
    fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsKeyFrame(&self) -> ::windows::core::Result<bool>;
    fn HasDiscontinuity(&self) -> ::windows::core::Result<bool>;
    fn FrameId(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerDetailsImpl: Sized {
    fn BackgroundService(&self) -> ::windows::core::Result<AppBroadcastBackgroundService>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastViewerCountChangedEventArgsImpl: Sized {
    fn ViewerCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureImpl: Sized {
    fn IsCapturingAudio(&self) -> ::windows::core::Result<bool>;
    fn IsCapturingVideo(&self) -> ::windows::core::Result<bool>;
    fn CapturingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCapturingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureAlternateShortcutKeysImpl: Sized {
    fn SetToggleGameBarKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleGameBarKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleGameBarKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleGameBarKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetSaveHistoricalVideoKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn SaveHistoricalVideoKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetSaveHistoricalVideoKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn SaveHistoricalVideoKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleRecordingKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleRecordingKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleRecordingKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleRecordingKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetTakeScreenshotKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn TakeScreenshotKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetTakeScreenshotKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn TakeScreenshotKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleRecordingIndicatorKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleRecordingIndicatorKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleRecordingIndicatorKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleRecordingIndicatorKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureAlternateShortcutKeys2Impl: Sized {
    fn SetToggleMicrophoneCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleMicrophoneCaptureKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleMicrophoneCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleMicrophoneCaptureKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureAlternateShortcutKeys3Impl: Sized {
    fn SetToggleCameraCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleCameraCaptureKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleCameraCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleCameraCaptureKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn SetToggleBroadcastKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn ToggleBroadcastKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn SetToggleBroadcastKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn ToggleBroadcastKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureDurationGeneratedEventArgsImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureFileGeneratedEventArgsImpl: Sized {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureManagerStaticsImpl: Sized {
    fn GetCurrentSettings(&self) -> ::windows::core::Result<AppCaptureSettings>;
    fn ApplySettings(&self, appcapturesettings: &::core::option::Option<AppCaptureSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureMetadataWriterImpl: Sized {
    fn AddStringEvent(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn AddInt32Event(&self, name: &::windows::core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn AddDoubleEvent(&self, name: &::windows::core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartStringState(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartInt32State(&self, name: &::windows::core::HSTRING, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StartDoubleState(&self, name: &::windows::core::HSTRING, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()>;
    fn StopState(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StopAllStates(&self) -> ::windows::core::Result<()>;
    fn RemainingStorageBytesAvailable(&self) -> ::windows::core::Result<u64>;
    fn MetadataPurged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMetadataPurged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureMicrophoneCaptureStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureRecordOperationImpl: Sized {
    fn StopRecording(&self) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<AppCaptureRecordingState>;
    fn ErrorCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
    fn IsFileTruncated(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn StateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DurationGenerated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDurationGenerated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FileGenerated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileGenerated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureRecordingStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AppCaptureRecordingState>;
    fn ErrorCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureServicesImpl: Sized {
    fn Record(&self) -> ::windows::core::Result<AppCaptureRecordOperation>;
    fn RecordTimeSpan(&self, starttime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<AppCaptureRecordOperation>;
    fn CanCapture(&self) -> ::windows::core::Result<bool>;
    fn State(&self) -> ::windows::core::Result<AppCaptureState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettingsImpl: Sized {
    fn SetAppCaptureDestinationFolder(&self, value: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<()>;
    fn AppCaptureDestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
    fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn AudioEncodingBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAudioCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingHeight(&self) -> ::windows::core::Result<u32>;
    fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn CustomVideoEncodingWidth(&self) -> ::windows::core::Result<u32>;
    fn SetHistoricalBufferLength(&self, value: u32) -> ::windows::core::Result<()>;
    fn HistoricalBufferLength(&self) -> ::windows::core::Result<u32>;
    fn SetHistoricalBufferLengthUnit(&self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::Result<()>;
    fn HistoricalBufferLengthUnit(&self) -> ::windows::core::Result<AppCaptureHistoricalBufferLengthUnit>;
    fn SetIsHistoricalCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHistoricalCaptureOnBatteryAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureOnBatteryAllowed(&self) -> ::windows::core::Result<bool>;
    fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHistoricalCaptureOnWirelessDisplayAllowed(&self) -> ::windows::core::Result<bool>;
    fn SetMaximumRecordLength(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MaximumRecordLength(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetScreenshotDestinationFolder(&self, value: &::core::option::Option<super::super::Storage::StorageFolder>) -> ::windows::core::Result<()>;
    fn ScreenshotDestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
    fn SetVideoEncodingBitrateMode(&self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingBitrateMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingBitrateMode>;
    fn SetVideoEncodingResolutionMode(&self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::Result<()>;
    fn VideoEncodingResolutionMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingResolutionMode>;
    fn SetIsAppCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAppCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsCpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledByPolicy(&self) -> ::windows::core::Result<bool>;
    fn IsMemoryConstrained(&self) -> ::windows::core::Result<bool>;
    fn HasHardwareEncoder(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings2Impl: Sized {
    fn IsGpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn AlternateShortcutKeys(&self) -> ::windows::core::Result<AppCaptureAlternateShortcutKeys>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings3Impl: Sized {
    fn SetIsMicrophoneCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings4Impl: Sized {
    fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool>;
    fn SetSystemAudioGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn SystemAudioGain(&self) -> ::windows::core::Result<f64>;
    fn SetMicrophoneGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn MicrophoneGain(&self) -> ::windows::core::Result<f64>;
    fn SetVideoEncodingFrameRateMode(&self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::Result<()>;
    fn VideoEncodingFrameRateMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingFrameRateMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureSettings5Impl: Sized {
    fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsEchoCancellationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCursorImageCaptureEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureStateImpl: Sized {
    fn IsTargetRunning(&self) -> ::windows::core::Result<bool>;
    fn IsHistoricalCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn ShouldCaptureMicrophone(&self) -> ::windows::core::Result<bool>;
    fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::core::Result<()>;
    fn RestartMicrophoneCapture(&self) -> ::windows::core::Result<()>;
    fn MicrophoneCaptureState(&self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState>;
    fn MicrophoneCaptureError(&self) -> ::windows::core::Result<u32>;
    fn MicrophoneCaptureStateChanged(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMicrophoneCaptureStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CaptureTargetClosed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureTargetClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<AppCapture>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppCaptureStatics2Impl: Sized {
    fn SetAllowedAsync(&self, allowed: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraCaptureUIImpl: Sized {
    fn PhotoSettings(&self) -> ::windows::core::Result<CameraCaptureUIPhotoCaptureSettings>;
    fn VideoSettings(&self) -> ::windows::core::Result<CameraCaptureUIVideoCaptureSettings>;
    fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraCaptureUIPhotoCaptureSettingsImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<CameraCaptureUIPhotoFormat>;
    fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> ::windows::core::Result<()>;
    fn MaxResolution(&self) -> ::windows::core::Result<CameraCaptureUIMaxPhotoResolution>;
    fn SetMaxResolution(&self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::Result<()>;
    fn CroppedSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCroppedSizeInPixels(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn CroppedAspectRatio(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCroppedAspectRatio(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn AllowCropping(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCropping(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraCaptureUIVideoCaptureSettingsImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<CameraCaptureUIVideoFormat>;
    fn SetFormat(&self, value: CameraCaptureUIVideoFormat) -> ::windows::core::Result<()>;
    fn MaxResolution(&self) -> ::windows::core::Result<CameraCaptureUIMaxVideoResolution>;
    fn SetMaxResolution(&self, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::Result<()>;
    fn MaxDurationInSeconds(&self) -> ::windows::core::Result<f32>;
    fn SetMaxDurationInSeconds(&self, value: f32) -> ::windows::core::Result<()>;
    fn AllowTrimming(&self) -> ::windows::core::Result<bool>;
    fn SetAllowTrimming(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOptionsUIStaticsImpl: Sized {
    fn Show(&self, mediacapture: &::core::option::Option<MediaCapture>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICapturedFrameImpl: Sized + IClosableImpl + IContentTypeProviderImpl + IInputStreamImpl + IOutputStreamImpl + IRandomAccessStreamImpl + IRandomAccessStreamWithContentTypeImpl {
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedFrame2Impl: Sized {
    fn ControlValues(&self) -> ::windows::core::Result<CapturedFrameControlValues>;
    fn BitmapProperties(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedFrameControlValuesImpl: Sized {
    fn Exposure(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn ExposureCompensation(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn IsoSpeed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn Focus(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SceneMode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>>;
    fn Flashed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn FlashPowerPercent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
    fn WhiteBalance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn ZoomFactor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedFrameControlValues2Impl: Sized {
    fn FocusState(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>>;
    fn IsoDigitalGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn IsoAnalogGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SensorFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn WhiteBalanceGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<WhiteBalanceGain>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedFrameWithSoftwareBitmapImpl: Sized {
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICapturedPhotoImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn Thumbnail(&self) -> ::windows::core::Result<CapturedFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesImpl: Sized {
    fn TargetCapturePolicy(&self) -> ::windows::core::Result<GameBarTargetCapturePolicy>;
    fn EnableCapture(&self) -> ::windows::core::Result<()>;
    fn DisableCapture(&self) -> ::windows::core::Result<()>;
    fn TargetInfo(&self) -> ::windows::core::Result<GameBarServicesTargetInfo>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppBroadcastServices(&self) -> ::windows::core::Result<AppBroadcastServices>;
    fn AppCaptureServices(&self) -> ::windows::core::Result<AppCaptureServices>;
    fn CommandReceived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesCommandEventArgsImpl: Sized {
    fn Command(&self) -> ::windows::core::Result<GameBarCommand>;
    fn Origin(&self) -> ::windows::core::Result<GameBarCommandOrigin>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesManagerImpl: Sized {
    fn GameBarServicesCreated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGameBarServicesCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesManagerGameBarServicesCreatedEventArgsImpl: Sized {
    fn GameBarServices(&self) -> ::windows::core::Result<GameBarServices>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<GameBarServicesManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameBarServicesTargetInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayMode(&self) -> ::windows::core::Result<GameBarServicesDisplayMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagMediaRecordingImpl: Sized {
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagMediaRecording2Impl: Sized {
    fn PauseAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResumeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagMediaRecording3Impl: Sized {
    fn PauseWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>;
    fn StopWithResultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagPhotoCaptureImpl: Sized {
    fn CaptureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CapturedPhoto>>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagPhotoSequenceCaptureImpl: Sized {
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PhotoCaptured(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePhotoCaptured(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureImpl: Sized {
    fn InitializeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InitializeWithSettingsAsync(&self, mediacaptureinitializationsettings: &::core::option::Option<MediaCaptureInitializationSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToStorageFileAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToStreamAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToCustomSinkAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartRecordToCustomSinkIdAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CapturePhotoToStorageFileAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CapturePhotoToStreamAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AddEffectAsync(&self, mediastreamtype: MediaStreamType, effectactivationid: &::windows::core::HSTRING, effectsettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearEffectsAsync(&self, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetEncoderProperty(&self, mediastreamtype: MediaStreamType, propertyid: &::windows::core::GUID, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetEncoderProperty(&self, mediastreamtype: MediaStreamType, propertyid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Failed(&self, erroreventhandler: &::core::option::Option<MediaCaptureFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFailed(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RecordLimitationExceeded(&self, recordlimitationexceededeventhandler: &::core::option::Option<RecordLimitationExceededEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecordLimitationExceeded(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaCaptureSettings(&self) -> ::windows::core::Result<MediaCaptureSettings>;
    fn AudioDeviceController(&self) -> ::windows::core::Result<super::Devices::AudioDeviceController>;
    fn VideoDeviceController(&self) -> ::windows::core::Result<super::Devices::VideoDeviceController>;
    fn SetPreviewMirroring(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPreviewMirroring(&self) -> ::windows::core::Result<bool>;
    fn SetPreviewRotation(&self, value: VideoRotation) -> ::windows::core::Result<()>;
    fn GetPreviewRotation(&self) -> ::windows::core::Result<VideoRotation>;
    fn SetRecordRotation(&self, value: VideoRotation) -> ::windows::core::Result<()>;
    fn GetRecordRotation(&self) -> ::windows::core::Result<VideoRotation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture2Impl: Sized {
    fn PrepareLowLagRecordToStorageFileAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToStreamAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToCustomSinkAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagRecordToCustomSinkIdAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>;
    fn PrepareLowLagPhotoCaptureAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>>;
    fn PrepareLowLagPhotoSequenceCaptureAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>;
    fn SetEncodingPropertiesAsync(&self, mediastreamtype: MediaStreamType, mediaencodingproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>, encoderproperties: &::core::option::Option<super::MediaProperties::MediaPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture3Impl: Sized {
    fn PrepareVariablePhotoSequenceCaptureAsync(&self, r#type: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>;
    fn FocusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFocusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PhotoConfirmationCaptured(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePhotoConfirmationCaptured(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture4Impl: Sized {
    fn AddAudioEffectAsync(&self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>;
    fn AddVideoEffectAsync(&self, definition: &::core::option::Option<super::Effects::IVideoEffectDefinition>, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>;
    fn PauseRecordAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResumeRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CameraStreamStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraStreamStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraStreamState(&self) -> ::windows::core::Result<super::Devices::CameraStreamState>;
    fn GetPreviewFrameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>>;
    fn GetPreviewFrameCopyAsync(&self, destination: &::core::option::Option<super::VideoFrame>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>>;
    fn ThermalStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveThermalStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ThermalStatus(&self) -> ::windows::core::Result<MediaCaptureThermalStatus>;
    fn PrepareAdvancedPhotoCaptureAsync(&self, encodingproperties: &::core::option::Option<super::MediaProperties::ImageEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture5Impl: Sized {
    fn RemoveEffectAsync(&self, effect: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PauseRecordWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>;
    fn StopRecordWithResultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>;
    fn FrameSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, Frames::MediaFrameSource>>;
    fn CreateFrameReaderAsync(&self, inputsource: &::core::option::Option<Frames::MediaFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
    fn CreateFrameReaderWithSubtypeAsync(&self, inputsource: &::core::option::Option<Frames::MediaFrameSource>, outputsubtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
    fn CreateFrameReaderWithSubtypeAndSizeAsync(&self, inputsource: &::core::option::Option<Frames::MediaFrameSource>, outputsubtype: &::windows::core::HSTRING, outputsize: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture6Impl: Sized {
    fn CaptureDeviceExclusiveControlStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCaptureDeviceExclusiveControlStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateMultiSourceFrameReaderAsync(&self, inputsources: &::core::option::Option<super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapture7Impl: Sized {
    fn CreateRelativePanelWatcher(&self, capturemode: StreamingCaptureMode, displayregion: &::core::option::Option<super::super::UI::WindowManagement::DisplayRegion>) -> ::windows::core::Result<MediaCaptureRelativePanelWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureDeviceExclusiveControlStatusChangedEventArgsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<MediaCaptureDeviceExclusiveControlStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureFailedEventArgsImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Code(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureFocusChangedEventArgsImpl: Sized {
    fn FocusState(&self) -> ::windows::core::Result<super::Devices::MediaCaptureFocusState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettingsImpl: Sized {
    fn SetAudioDeviceId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AudioDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVideoDeviceId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStreamingCaptureMode(&self, value: StreamingCaptureMode) -> ::windows::core::Result<()>;
    fn StreamingCaptureMode(&self) -> ::windows::core::Result<StreamingCaptureMode>;
    fn SetPhotoCaptureSource(&self, value: PhotoCaptureSource) -> ::windows::core::Result<()>;
    fn PhotoCaptureSource(&self) -> ::windows::core::Result<PhotoCaptureSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings2Impl: Sized {
    fn SetMediaCategory(&self, value: MediaCategory) -> ::windows::core::Result<()>;
    fn MediaCategory(&self) -> ::windows::core::Result<MediaCategory>;
    fn SetAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()>;
    fn AudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings3Impl: Sized {
    fn SetAudioSource(&self, value: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
    fn AudioSource(&self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn SetVideoSource(&self, value: &::core::option::Option<super::Core::IMediaSource>) -> ::windows::core::Result<()>;
    fn VideoSource(&self) -> ::windows::core::Result<super::Core::IMediaSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings4Impl: Sized {
    fn VideoProfile(&self) -> ::windows::core::Result<MediaCaptureVideoProfile>;
    fn SetVideoProfile(&self, value: &::core::option::Option<MediaCaptureVideoProfile>) -> ::windows::core::Result<()>;
    fn PreviewMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetPreviewMediaDescription(&self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
    fn RecordMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetRecordMediaDescription(&self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
    fn PhotoMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription>;
    fn SetPhotoMediaDescription(&self, value: &::core::option::Option<MediaCaptureVideoProfileMediaDescription>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings5Impl: Sized {
    fn SourceGroup(&self) -> ::windows::core::Result<Frames::MediaFrameSourceGroup>;
    fn SetSourceGroup(&self, value: &::core::option::Option<Frames::MediaFrameSourceGroup>) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<MediaCaptureSharingMode>;
    fn SetSharingMode(&self, value: MediaCaptureSharingMode) -> ::windows::core::Result<()>;
    fn MemoryPreference(&self) -> ::windows::core::Result<MediaCaptureMemoryPreference>;
    fn SetMemoryPreference(&self, value: MediaCaptureMemoryPreference) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings6Impl: Sized {
    fn AlwaysPlaySystemShutterSound(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysPlaySystemShutterSound(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureInitializationSettings7Impl: Sized {
    fn DeviceUriPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetDeviceUriPasswordCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn DeviceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetDeviceUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCapturePauseResultImpl: Sized {
    fn LastFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn RecordDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureRelativePanelWatcherImpl: Sized {
    fn RelativePanel(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureSettingsImpl: Sized {
    fn AudioDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StreamingCaptureMode(&self) -> ::windows::core::Result<StreamingCaptureMode>;
    fn PhotoCaptureSource(&self) -> ::windows::core::Result<PhotoCaptureSource>;
    fn VideoDeviceCharacteristic(&self) -> ::windows::core::Result<VideoDeviceCharacteristic>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureSettings2Impl: Sized {
    fn ConcurrentRecordAndPhotoSupported(&self) -> ::windows::core::Result<bool>;
    fn ConcurrentRecordAndPhotoSequenceSupported(&self) -> ::windows::core::Result<bool>;
    fn CameraSoundRequiredForRegion(&self) -> ::windows::core::Result<bool>;
    fn Horizontal35mmEquivalentFocalLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PitchOffsetDegrees(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn Vertical35mmEquivalentFocalLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn MediaCategory(&self) -> ::windows::core::Result<MediaCategory>;
    fn AudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureSettings3Impl: Sized {
    fn Direct3D11Device(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureStaticsImpl: Sized {
    fn IsVideoProfileSupported(&self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn FindAllVideoProfiles(&self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
    fn FindConcurrentProfiles(&self, videodeviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
    fn FindKnownVideoProfiles(&self, videodeviceid: &::windows::core::HSTRING, name: KnownVideoProfile) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureStopResultImpl: Sized {
    fn LastFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn RecordDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoPreviewImpl: Sized {
    fn StartPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartPreviewToCustomSinkAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, custommediasink: &::core::option::Option<super::IMediaExtension>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartPreviewToCustomSinkIdAsync(&self, encodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>, customsinkactivationid: &::windows::core::HSTRING, customsinksettings: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfileImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedPreviewMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn SupportedRecordMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn SupportedPhotoMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>;
    fn GetConcurrency(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfile2Impl: Sized {
    fn FrameSourceInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfileMediaDescriptionImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn FrameRate(&self) -> ::windows::core::Result<f64>;
    fn IsVariablePhotoSequenceSupported(&self) -> ::windows::core::Result<bool>;
    fn IsHdrVideoSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCaptureVideoProfileMediaDescription2Impl: Sized {
    fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOptionalReferencePhotoCapturedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn Context(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoCapturedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn Thumbnail(&self) -> ::windows::core::Result<CapturedFrame>;
    fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoConfirmationCapturedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<CapturedFrame>;
    fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScreenCaptureImpl: Sized {
    fn AudioSource(&self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn VideoSource(&self) -> ::windows::core::Result<super::Core::IMediaSource>;
    fn IsAudioSuspended(&self) -> ::windows::core::Result<bool>;
    fn IsVideoSuspended(&self) -> ::windows::core::Result<bool>;
    fn SourceSuspensionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceSuspensionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScreenCaptureStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ScreenCapture>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISourceSuspensionChangedEventArgsImpl: Sized {
    fn IsAudioSuspended(&self) -> ::windows::core::Result<bool>;
    fn IsVideoSuspended(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamConfigurationImpl: Sized {
    fn InputProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn OutputProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
}
