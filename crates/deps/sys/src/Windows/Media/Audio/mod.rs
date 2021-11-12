#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioDeviceInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
#[repr(transparent)]
pub struct AudioDeviceOutputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioFileInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FileNotFound: Self = Self(1i32);
    pub const InvalidFileType: Self = Self(2i32);
    pub const FormatNotSupported: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
#[repr(transparent)]
pub struct AudioFileOutputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioFrameCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioFrameInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioFrameOutputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioGraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioGraphBatchUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioGraphConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
#[repr(transparent)]
pub struct AudioGraphSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: Self = Self(0i32);
    pub const AudioDeviceLost: Self = Self(1i32);
    pub const AudioSessionDisconnected: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
#[repr(transparent)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterConeProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AudioNodeEmitterDecayModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: Self = Self(0u32);
    pub const DisableDoppler: Self = Self(1u32);
}
#[repr(transparent)]
pub struct AudioNodeEmitterShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: Self = Self(0i32);
    pub const Cone: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AudioNodeListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioPlaybackConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: Self = Self(0i32);
    pub const RequestTimedOut: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
#[repr(transparent)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AudioStateMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioSubmixNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateAudioDeviceInputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateAudioDeviceOutputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateAudioFileInputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateAudioFileOutputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateAudioGraphResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CreateMediaSourceAudioInputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EchoEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EqualizerBand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EqualizerEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FrameInputNodeQuantumStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioDeviceInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioDeviceOutputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioFileInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioFileOutputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioFrameCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioFrameInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioFrameOutputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraph2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraph3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraphConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraphSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraphSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraphSettingsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraphStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioInputNode2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitterConeProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitterShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeEmitterShapeStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioNodeWithListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioPlaybackConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioPlaybackConnectionOpenResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioPlaybackConnectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStateMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStateMonitorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioGraphResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateAudioGraphResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEchoEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEchoEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEqualizerBand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEqualizerEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEqualizerEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameInputNodeQuantumStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILimiterEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILimiterEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSourceAudioInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReverbEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReverbEffectDefinitionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISetDefaultSpatialAudioFormatResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfigurationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioFormatConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioFormatConfigurationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LimiterEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceAudioInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
#[repr(transparent)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: Self = Self(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: Self = Self(1i32);
}
#[repr(transparent)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: Self = Self(0i32);
    pub const LowestLatency: Self = Self(1i32);
    pub const ClosestToDesired: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ReverbEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const LicenseExpired: Self = Self(2i32);
    pub const LicenseNotValidForAudioEndpoint: Self = Self(3i32);
    pub const NotSupportedOnAudioEndpoint: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
#[repr(transparent)]
pub struct SpatialAudioDeviceConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAudioFormatConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: Self = Self(0i32);
    pub const FoldDown: Self = Self(1i32);
}
