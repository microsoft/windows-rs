#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedPhotoCaptureSettingsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<AdvancedPhotoMode>;
    fn SetMode(&self, value: AdvancedPhotoMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedPhotoControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AdvancedPhotoMode>>;
    fn Mode(&self) -> ::windows::core::Result<AdvancedPhotoMode>;
    fn Configure(&self, settings: &::core::option::Option<AdvancedPhotoCaptureSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceControllerImpl: Sized {
    fn SetDeviceProperty(&self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetDeviceProperty(&self, propertyid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController10Impl: Sized {
    fn CameraOcclusionInfo(&self) -> ::windows::core::Result<CameraOcclusionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController2Impl: Sized {
    fn LowLagPhotoSequence(&self) -> ::windows::core::Result<LowLagPhotoSequenceControl>;
    fn LowLagPhoto(&self) -> ::windows::core::Result<LowLagPhotoControl>;
    fn SceneModeControl(&self) -> ::windows::core::Result<SceneModeControl>;
    fn TorchControl(&self) -> ::windows::core::Result<TorchControl>;
    fn FlashControl(&self) -> ::windows::core::Result<FlashControl>;
    fn WhiteBalanceControl(&self) -> ::windows::core::Result<WhiteBalanceControl>;
    fn ExposureControl(&self) -> ::windows::core::Result<ExposureControl>;
    fn FocusControl(&self) -> ::windows::core::Result<FocusControl>;
    fn ExposureCompensationControl(&self) -> ::windows::core::Result<ExposureCompensationControl>;
    fn IsoSpeedControl(&self) -> ::windows::core::Result<IsoSpeedControl>;
    fn RegionsOfInterestControl(&self) -> ::windows::core::Result<RegionsOfInterestControl>;
    fn PrimaryUse(&self) -> ::windows::core::Result<CaptureUse>;
    fn SetPrimaryUse(&self, value: CaptureUse) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController3Impl: Sized {
    fn VariablePhotoSequenceController(&self) -> ::windows::core::Result<Core::VariablePhotoSequenceController>;
    fn PhotoConfirmationControl(&self) -> ::windows::core::Result<PhotoConfirmationControl>;
    fn ZoomControl(&self) -> ::windows::core::Result<ZoomControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController4Impl: Sized {
    fn ExposurePriorityVideoControl(&self) -> ::windows::core::Result<ExposurePriorityVideoControl>;
    fn DesiredOptimization(&self) -> ::windows::core::Result<MediaCaptureOptimization>;
    fn SetDesiredOptimization(&self, value: MediaCaptureOptimization) -> ::windows::core::Result<()>;
    fn HdrVideoControl(&self) -> ::windows::core::Result<HdrVideoControl>;
    fn OpticalImageStabilizationControl(&self) -> ::windows::core::Result<OpticalImageStabilizationControl>;
    fn AdvancedPhotoControl(&self) -> ::windows::core::Result<AdvancedPhotoControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController5Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDevicePropertyById(&self, propertyid: &::windows::core::HSTRING, maxpropertyvaluesize: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult>;
    fn SetDevicePropertyById(&self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus>;
    fn GetDevicePropertyByExtendedId(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], maxpropertyvaluesize: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyResult>;
    fn SetDevicePropertyByExtendedId(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VideoDeviceControllerSetDevicePropertyStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController6Impl: Sized {
    fn VideoTemporalDenoisingControl(&self) -> ::windows::core::Result<VideoTemporalDenoisingControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController7Impl: Sized {
    fn InfraredTorchControl(&self) -> ::windows::core::Result<InfraredTorchControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController8Impl: Sized {
    fn PanelBasedOptimizationControl(&self) -> ::windows::core::Result<PanelBasedOptimizationControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedVideoCaptureDeviceController9Impl: Sized {
    fn DigitalWindowControl(&self) -> ::windows::core::Result<DigitalWindowControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceControllerImpl: Sized + IMediaDeviceControllerImpl {
    fn SetMuted(&self, value: bool) -> ::windows::core::Result<()>;
    fn Muted(&self) -> ::windows::core::Result<bool>;
    fn SetVolumePercent(&self, value: f32) -> ::windows::core::Result<()>;
    fn VolumePercent(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModuleImpl: Sized {
    fn ClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstanceId(&self) -> ::windows::core::Result<u32>;
    fn MajorVersion(&self) -> ::windows::core::Result<u32>;
    fn MinorVersion(&self) -> ::windows::core::Result<u32>;
    fn SendCommandAsync(&self, command: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ModuleCommandResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModuleNotificationEventArgsImpl: Sized {
    fn Module(&self) -> ::windows::core::Result<AudioDeviceModule>;
    fn NotificationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModulesManagerImpl: Sized {
    fn ModuleNotificationReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioDeviceModulesManager, AudioDeviceModuleNotificationEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveModuleNotificationReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindAllById(&self, moduleid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>;
    fn FindAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioDeviceModule>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioDeviceModulesManagerFactoryImpl: Sized {
    fn Create(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioDeviceModulesManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallControlImpl: Sized {
    fn IndicateNewIncomingCall(&self, enableringer: bool, callerid: &::windows::core::HSTRING) -> ::windows::core::Result<u64>;
    fn IndicateNewOutgoingCall(&self) -> ::windows::core::Result<u64>;
    fn IndicateActiveCall(&self, calltoken: u64) -> ::windows::core::Result<()>;
    fn EndCall(&self, calltoken: u64) -> ::windows::core::Result<()>;
    fn HasRinger(&self) -> ::windows::core::Result<bool>;
    fn AnswerRequested(&self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnswerRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HangUpRequested(&self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHangUpRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DialRequested(&self, handler: &::core::option::Option<DialRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDialRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RedialRequested(&self, handler: &::core::option::Option<RedialRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRedialRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeypadPressed(&self, handler: &::core::option::Option<KeypadPressedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeypadPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AudioTransferRequested(&self, handler: &::core::option::Option<CallControlEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioTransferRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICallControlStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<CallControl>;
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<CallControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOcclusionInfoImpl: Sized {
    fn GetState(&self) -> ::windows::core::Result<CameraOcclusionState>;
    fn IsOcclusionKindSupported(&self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CameraOcclusionInfo, CameraOcclusionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOcclusionStateImpl: Sized {
    fn IsOccluded(&self) -> ::windows::core::Result<bool>;
    fn IsOcclusionKind(&self, occlusionkind: CameraOcclusionKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraOcclusionStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<CameraOcclusionState>;
}
pub trait IDefaultAudioDeviceChangedEventArgsImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Role(&self) -> ::windows::core::Result<AudioDeviceRole>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDialRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<()>;
    fn Contact(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDigitalWindowBoundsImpl: Sized {
    fn NormalizedOriginTop(&self) -> ::windows::core::Result<f64>;
    fn SetNormalizedOriginTop(&self, value: f64) -> ::windows::core::Result<()>;
    fn NormalizedOriginLeft(&self) -> ::windows::core::Result<f64>;
    fn SetNormalizedOriginLeft(&self, value: f64) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<f64>;
    fn SetScale(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDigitalWindowCapabilityImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<i32>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn MinScaleValue(&self) -> ::windows::core::Result<f64>;
    fn MaxScaleValue(&self) -> ::windows::core::Result<f64>;
    fn MinScaleValueWithoutUpsampling(&self) -> ::windows::core::Result<f64>;
    fn NormalizedFieldOfViewLimit(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDigitalWindowControlImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<::windows::core::Array<DigitalWindowMode>>;
    fn CurrentMode(&self) -> ::windows::core::Result<DigitalWindowMode>;
    fn GetBounds(&self) -> ::windows::core::Result<DigitalWindowBounds>;
    fn Configure(&self, digitalwindowmode: DigitalWindowMode) -> ::windows::core::Result<()>;
    fn ConfigureWithBounds(&self, digitalwindowmode: DigitalWindowMode, digitalwindowbounds: &::core::option::Option<DigitalWindowBounds>) -> ::windows::core::Result<()>;
    fn SupportedCapabilities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DigitalWindowCapability>>;
    fn GetCapabilityForSize(&self, width: i32, height: i32) -> ::windows::core::Result<DigitalWindowCapability>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExposureCompensationControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<f32>;
    fn Max(&self) -> ::windows::core::Result<f32>;
    fn Step(&self) -> ::windows::core::Result<f32>;
    fn Value(&self) -> ::windows::core::Result<f32>;
    fn SetValueAsync(&self, value: f32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExposureControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAutoAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Max(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Step(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Value(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetValueAsync(&self, shutterduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExposurePriorityVideoControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlashControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn PowerSupported(&self) -> ::windows::core::Result<bool>;
    fn RedEyeReductionSupported(&self) -> ::windows::core::Result<bool>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAuto(&self, value: bool) -> ::windows::core::Result<()>;
    fn RedEyeReduction(&self) -> ::windows::core::Result<bool>;
    fn SetRedEyeReduction(&self, value: bool) -> ::windows::core::Result<()>;
    fn PowerPercent(&self) -> ::windows::core::Result<f32>;
    fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlashControl2Impl: Sized {
    fn AssistantLightSupported(&self) -> ::windows::core::Result<bool>;
    fn AssistantLightEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAssistantLightEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedPresets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusPreset>>;
    fn Preset(&self) -> ::windows::core::Result<FocusPreset>;
    fn SetPresetAsync(&self, preset: FocusPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetPresetWithCompletionOptionAsync(&self, preset: FocusPreset, completebeforefocus: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&self, focus: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn FocusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusControl2Impl: Sized {
    fn FocusChangedSupported(&self) -> ::windows::core::Result<bool>;
    fn WaitForFocusSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedFocusModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<FocusMode>>;
    fn SupportedFocusDistances(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ManualFocusDistance>>;
    fn SupportedFocusRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AutoFocusRange>>;
    fn Mode(&self) -> ::windows::core::Result<FocusMode>;
    fn FocusState(&self) -> ::windows::core::Result<MediaCaptureFocusState>;
    fn UnlockAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn LockAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Configure(&self, settings: &::core::option::Option<FocusSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusSettingsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<FocusMode>;
    fn SetMode(&self, value: FocusMode) -> ::windows::core::Result<()>;
    fn AutoFocusRange(&self) -> ::windows::core::Result<AutoFocusRange>;
    fn SetAutoFocusRange(&self, value: AutoFocusRange) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn Distance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<ManualFocusDistance>>;
    fn SetDistance(&self, value: &::core::option::Option<super::super::Foundation::IReference<ManualFocusDistance>>) -> ::windows::core::Result<()>;
    fn WaitForFocus(&self) -> ::windows::core::Result<bool>;
    fn SetWaitForFocus(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisableDriverFallback(&self) -> ::windows::core::Result<bool>;
    fn SetDisableDriverFallback(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHdrVideoControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HdrVideoMode>>;
    fn Mode(&self) -> ::windows::core::Result<HdrVideoMode>;
    fn SetMode(&self, value: HdrVideoMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInfraredTorchControlImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<InfraredTorchMode>>;
    fn CurrentMode(&self) -> ::windows::core::Result<InfraredTorchMode>;
    fn SetCurrentMode(&self, value: InfraredTorchMode) -> ::windows::core::Result<()>;
    fn MinPower(&self) -> ::windows::core::Result<i32>;
    fn MaxPower(&self) -> ::windows::core::Result<i32>;
    fn PowerStep(&self) -> ::windows::core::Result<i32>;
    fn Power(&self) -> ::windows::core::Result<i32>;
    fn SetPower(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsoSpeedControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedPresets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsoSpeedPreset>>;
    fn Preset(&self) -> ::windows::core::Result<IsoSpeedPreset>;
    fn SetPresetAsync(&self, preset: IsoSpeedPreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsoSpeedControl2Impl: Sized {
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&self, isospeed: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAutoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeypadPressedEventArgsImpl: Sized {
    fn TelephonyKey(&self) -> ::windows::core::Result<TelephonyKey>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagPhotoControlImpl: Sized {
    fn GetHighestConcurrentFrameRate(&self, captureproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn ThumbnailEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetThumbnailEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ThumbnailFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat>;
    fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()>;
    fn DesiredThumbnailSize(&self) -> ::windows::core::Result<u32>;
    fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows::core::Result<()>;
    fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLagPhotoSequenceControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn MaxPastPhotos(&self) -> ::windows::core::Result<u32>;
    fn MaxPhotosPerSecond(&self) -> ::windows::core::Result<f32>;
    fn PastPhotoLimit(&self) -> ::windows::core::Result<u32>;
    fn SetPastPhotoLimit(&self, value: u32) -> ::windows::core::Result<()>;
    fn PhotosPerSecondLimit(&self) -> ::windows::core::Result<f32>;
    fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetHighestConcurrentFrameRate(&self, captureproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio>;
    fn ThumbnailEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetThumbnailEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ThumbnailFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaThumbnailFormat>;
    fn SetThumbnailFormat(&self, value: super::MediaProperties::MediaThumbnailFormat) -> ::windows::core::Result<()>;
    fn DesiredThumbnailSize(&self) -> ::windows::core::Result<u32>;
    fn SetDesiredThumbnailSize(&self, value: u32) -> ::windows::core::Result<()>;
    fn HardwareAcceleratedThumbnailSupported(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaDeviceControlImpl: Sized {
    fn Capabilities(&self) -> ::windows::core::Result<MediaDeviceControlCapabilities>;
    fn TryGetValue(&self, value: &mut f64) -> ::windows::core::Result<bool>;
    fn TrySetValue(&self, value: f64) -> ::windows::core::Result<bool>;
    fn TryGetAuto(&self, value: &mut bool) -> ::windows::core::Result<bool>;
    fn TrySetAuto(&self, value: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaDeviceControlCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<f64>;
    fn Max(&self) -> ::windows::core::Result<f64>;
    fn Step(&self) -> ::windows::core::Result<f64>;
    fn Default(&self) -> ::windows::core::Result<f64>;
    fn AutoModeSupported(&self) -> ::windows::core::Result<bool>;
}
pub trait IMediaDeviceControllerImpl: Sized {
    fn GetAvailableMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>;
    fn GetMediaStreamProperties(&self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties>;
    fn SetMediaStreamPropertiesAsync(&self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaDeviceStaticsImpl: Sized {
    fn GetAudioCaptureSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAudioRenderSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetVideoCaptureSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAudioCaptureId(&self, role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefaultAudioRenderId(&self, role: AudioDeviceRole) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultAudioCaptureDeviceChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioCaptureDeviceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultAudioCaptureDeviceChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DefaultAudioRenderDeviceChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, DefaultAudioRenderDeviceChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDefaultAudioRenderDeviceChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IModuleCommandResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SendCommandStatus>;
    fn Result(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOpticalImageStabilizationControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OpticalImageStabilizationMode>>;
    fn Mode(&self) -> ::windows::core::Result<OpticalImageStabilizationMode>;
    fn SetMode(&self, value: OpticalImageStabilizationMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPanelBasedOptimizationControlImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn Panel(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel>;
    fn SetPanel(&self, value: super::super::Devices::Enumeration::Panel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhotoConfirmationControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PixelFormat(&self) -> ::windows::core::Result<super::MediaProperties::MediaPixelFormat>;
    fn SetPixelFormat(&self, format: super::MediaProperties::MediaPixelFormat) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRedialRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRegionOfInterestImpl: Sized {
    fn AutoFocusEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAutoFocusEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoWhiteBalanceEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAutoWhiteBalanceEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoExposureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAutoExposureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetBounds(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRegionOfInterest2Impl: Sized {
    fn Type(&self) -> ::windows::core::Result<RegionOfInterestType>;
    fn SetType(&self, value: RegionOfInterestType) -> ::windows::core::Result<()>;
    fn BoundsNormalized(&self) -> ::windows::core::Result<bool>;
    fn SetBoundsNormalized(&self, value: bool) -> ::windows::core::Result<()>;
    fn Weight(&self) -> ::windows::core::Result<u32>;
    fn SetWeight(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRegionsOfInterestControlImpl: Sized {
    fn MaxRegions(&self) -> ::windows::core::Result<u32>;
    fn SetRegionsAsync(&self, regions: &::core::option::Option<super::super::Foundation::Collections::IIterable<RegionOfInterest>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetRegionsWithLockAsync(&self, regions: &::core::option::Option<super::super::Foundation::Collections::IIterable<RegionOfInterest>>, lockvalues: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearRegionsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AutoFocusSupported(&self) -> ::windows::core::Result<bool>;
    fn AutoWhiteBalanceSupported(&self) -> ::windows::core::Result<bool>;
    fn AutoExposureSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneModeControlImpl: Sized {
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CaptureSceneMode>>;
    fn Value(&self) -> ::windows::core::Result<CaptureSceneMode>;
    fn SetValueAsync(&self, scenemode: CaptureSceneMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITorchControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn PowerSupported(&self) -> ::windows::core::Result<bool>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PowerPercent(&self) -> ::windows::core::Result<f32>;
    fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDeviceControllerImpl: Sized + IMediaDeviceControllerImpl {
    fn Brightness(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Contrast(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Hue(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn WhiteBalance(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn BacklightCompensation(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Pan(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Tilt(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Zoom(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Roll(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Exposure(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn Focus(&self) -> ::windows::core::Result<MediaDeviceControl>;
    fn TrySetPowerlineFrequency(&self, value: super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool>;
    fn TryGetPowerlineFrequency(&self, value: &mut super::Capture::PowerlineFrequency) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoDeviceControllerGetDevicePropertyResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<VideoDeviceControllerGetDevicePropertyStatus>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTemporalDenoisingControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VideoTemporalDenoisingMode>>;
    fn Mode(&self) -> ::windows::core::Result<VideoTemporalDenoisingMode>;
    fn SetMode(&self, value: VideoTemporalDenoisingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWhiteBalanceControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Preset(&self) -> ::windows::core::Result<ColorTemperaturePreset>;
    fn SetPresetAsync(&self, preset: ColorTemperaturePreset) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
    fn Value(&self) -> ::windows::core::Result<u32>;
    fn SetValueAsync(&self, temperature: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IZoomControlImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<f32>;
    fn Max(&self) -> ::windows::core::Result<f32>;
    fn Step(&self) -> ::windows::core::Result<f32>;
    fn Value(&self) -> ::windows::core::Result<f32>;
    fn SetValue(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IZoomControl2Impl: Sized {
    fn SupportedModes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ZoomTransitionMode>>;
    fn Mode(&self) -> ::windows::core::Result<ZoomTransitionMode>;
    fn Configure(&self, settings: &::core::option::Option<ZoomSettings>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IZoomSettingsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<ZoomTransitionMode>;
    fn SetMode(&self, value: ZoomTransitionMode) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<f32>;
    fn SetValue(&self, value: f32) -> ::windows::core::Result<()>;
}
