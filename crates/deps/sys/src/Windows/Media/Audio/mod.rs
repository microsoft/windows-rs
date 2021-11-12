#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioDeviceInputNode(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioDeviceNodeCreationStatus(i32);
#[repr(transparent)]
pub struct AudioDeviceOutputNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioFileInputNode(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioFileNodeCreationStatus(i32);
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
#[repr(C)]
pub struct AudioGraphCreationStatus(i32);
#[repr(transparent)]
pub struct AudioGraphSettings(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioGraphUnrecoverableError(i32);
#[repr(transparent)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterConeProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioNodeEmitterDecayKind(i32);
#[repr(transparent)]
pub struct AudioNodeEmitterDecayModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioNodeEmitterSettings(i32);
#[repr(transparent)]
pub struct AudioNodeEmitterShape(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioNodeEmitterShapeKind(i32);
#[repr(transparent)]
pub struct AudioNodeListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioPlaybackConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AudioPlaybackConnectionOpenResultStatus(i32);
#[repr(C)]
pub struct AudioPlaybackConnectionState(i32);
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
#[repr(C)]
pub struct MediaSourceAudioInputNodeCreationStatus(i32);
#[repr(C)]
pub struct MixedRealitySpatialAudioFormatPolicy(i32);
#[repr(C)]
pub struct QuantumSizeSelectionMode(i32);
#[repr(transparent)]
pub struct ReverbEffectDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SetDefaultSpatialAudioFormatStatus(i32);
#[repr(transparent)]
pub struct SpatialAudioDeviceConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAudioFormatConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAudioFormatSubtype(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SpatialAudioModel(i32);
