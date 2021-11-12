#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioDeviceInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(0i32);
    pub const DeviceNotAvailable: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(1i32);
    pub const FormatNotSupported: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(2i32);
    pub const UnknownFailure: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(3i32);
    pub const AccessDenied: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(4i32);
}
#[repr(transparent)]
pub struct AudioDeviceOutputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioFileInputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(0i32);
    pub const FileNotFound: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(1i32);
    pub const InvalidFileType: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(2i32);
    pub const FormatNotSupported: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(3i32);
    pub const UnknownFailure: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(4i32);
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
    pub const Success: AudioGraphCreationStatus = AudioGraphCreationStatus(0i32);
    pub const DeviceNotAvailable: AudioGraphCreationStatus = AudioGraphCreationStatus(1i32);
    pub const FormatNotSupported: AudioGraphCreationStatus = AudioGraphCreationStatus(2i32);
    pub const UnknownFailure: AudioGraphCreationStatus = AudioGraphCreationStatus(3i32);
}
#[repr(transparent)]
pub struct AudioGraphSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(0i32);
    pub const AudioDeviceLost: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(1i32);
    pub const AudioSessionDisconnected: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(2i32);
    pub const UnknownFailure: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(3i32);
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
    pub const Natural: AudioNodeEmitterDecayKind = AudioNodeEmitterDecayKind(0i32);
    pub const Custom: AudioNodeEmitterDecayKind = AudioNodeEmitterDecayKind(1i32);
}
#[repr(transparent)]
pub struct AudioNodeEmitterDecayModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: AudioNodeEmitterSettings = AudioNodeEmitterSettings(0u32);
    pub const DisableDoppler: AudioNodeEmitterSettings = AudioNodeEmitterSettings(1u32);
}
#[repr(transparent)]
pub struct AudioNodeEmitterShape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: AudioNodeEmitterShapeKind = AudioNodeEmitterShapeKind(0i32);
    pub const Cone: AudioNodeEmitterShapeKind = AudioNodeEmitterShapeKind(1i32);
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
    pub const Success: AudioPlaybackConnectionOpenResultStatus = AudioPlaybackConnectionOpenResultStatus(0i32);
    pub const RequestTimedOut: AudioPlaybackConnectionOpenResultStatus = AudioPlaybackConnectionOpenResultStatus(1i32);
    pub const DeniedBySystem: AudioPlaybackConnectionOpenResultStatus = AudioPlaybackConnectionOpenResultStatus(2i32);
    pub const UnknownFailure: AudioPlaybackConnectionOpenResultStatus = AudioPlaybackConnectionOpenResultStatus(3i32);
}
#[repr(transparent)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: AudioPlaybackConnectionState = AudioPlaybackConnectionState(0i32);
    pub const Opened: AudioPlaybackConnectionState = AudioPlaybackConnectionState(1i32);
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
    pub const Success: MediaSourceAudioInputNodeCreationStatus = MediaSourceAudioInputNodeCreationStatus(0i32);
    pub const FormatNotSupported: MediaSourceAudioInputNodeCreationStatus = MediaSourceAudioInputNodeCreationStatus(1i32);
    pub const NetworkError: MediaSourceAudioInputNodeCreationStatus = MediaSourceAudioInputNodeCreationStatus(2i32);
    pub const UnknownFailure: MediaSourceAudioInputNodeCreationStatus = MediaSourceAudioInputNodeCreationStatus(3i32);
}
#[repr(transparent)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: MixedRealitySpatialAudioFormatPolicy = MixedRealitySpatialAudioFormatPolicy(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: MixedRealitySpatialAudioFormatPolicy = MixedRealitySpatialAudioFormatPolicy(1i32);
}
#[repr(transparent)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: QuantumSizeSelectionMode = QuantumSizeSelectionMode(0i32);
    pub const LowestLatency: QuantumSizeSelectionMode = QuantumSizeSelectionMode(1i32);
    pub const ClosestToDesired: QuantumSizeSelectionMode = QuantumSizeSelectionMode(2i32);
}
#[repr(transparent)]
pub struct ReverbEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(0i32);
    pub const AccessDenied: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(1i32);
    pub const LicenseExpired: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(2i32);
    pub const LicenseNotValidForAudioEndpoint: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(3i32);
    pub const NotSupportedOnAudioEndpoint: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(4i32);
    pub const UnknownError: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(5i32);
}
#[repr(transparent)]
pub struct SpatialAudioDeviceConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAudioFormatConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: SpatialAudioModel = SpatialAudioModel(0i32);
    pub const FoldDown: SpatialAudioModel = SpatialAudioModel(1i32);
}
