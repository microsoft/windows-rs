#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioDeviceInputNodeImpl: Sized + IAudioInputNodeImpl + IAudioNodeImpl + IClosableImpl {
    fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioDeviceOutputNodeImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFileInputNodeImpl: Sized + IAudioInputNodeImpl + IAudioNodeImpl + IClosableImpl {
    fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Seek(&self, position: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetEndTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetLoopCount(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SourceFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
    fn FileCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioFileInputNode, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFileCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFileOutputNodeImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn FileEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile>;
    fn FinalizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioFrameCompletedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<super::AudioFrame>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFrameInputNodeImpl: Sized + IAudioInputNodeImpl + IAudioNodeImpl + IClosableImpl {
    fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64>;
    fn AddFrame(&self, frame: &::core::option::Option<super::AudioFrame>) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
    fn QueuedSampleCount(&self) -> ::windows::core::Result<u64>;
    fn AudioFrameCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioFrameCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn QuantumStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuantumStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioFrameOutputNodeImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn GetFrame(&self) -> ::windows::core::Result<super::AudioFrame>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioGraphImpl: Sized + IClosableImpl {
    fn CreateFrameInputNode(&self) -> ::windows::core::Result<AudioFrameInputNode>;
    fn CreateFrameInputNodeWithFormat(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioFrameInputNode>;
    fn CreateDeviceInputNodeAsync(&self, category: super::Capture::MediaCategory) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>;
    fn CreateDeviceInputNodeWithFormatAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>;
    fn CreateDeviceInputNodeWithFormatOnDeviceAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>, device: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>;
    fn CreateFrameOutputNode(&self) -> ::windows::core::Result<AudioFrameOutputNode>;
    fn CreateFrameOutputNodeWithFormat(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioFrameOutputNode>;
    fn CreateDeviceOutputNodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>>;
    fn CreateFileInputNodeAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>;
    fn CreateFileOutputNodeAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>;
    fn CreateFileOutputNodeWithFileProfileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>, fileencodingprofile: &::core::option::Option<super::MediaProperties::MediaEncodingProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>;
    fn CreateSubmixNode(&self) -> ::windows::core::Result<AudioSubmixNode>;
    fn CreateSubmixNodeWithFormat(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioSubmixNode>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn ResetAllNodes(&self) -> ::windows::core::Result<()>;
    fn QuantumStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuantumStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn QuantumProcessed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveQuantumProcessed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnrecoverableErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnrecoverableErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompletedQuantumCount(&self) -> ::windows::core::Result<u64>;
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn LatencyInSamples(&self) -> ::windows::core::Result<i32>;
    fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
    fn RenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing>;
    fn SamplesPerQuantum(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioGraph2Impl: Sized + IAudioGraphImpl + IClosableImpl {
    fn CreateFrameInputNodeWithFormatAndEmitter(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<AudioFrameInputNode>;
    fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>, device: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>;
    fn CreateFileInputNodeWithEmitterAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>;
    fn CreateSubmixNodeWithFormatAndEmitter(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<AudioSubmixNode>;
    fn CreateBatchUpdater(&self) -> ::windows::core::Result<AudioGraphBatchUpdater>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraph3Impl: Sized {
    fn CreateMediaSourceAudioInputNodeAsync(&self, mediasource: &::core::option::Option<super::Core::MediaSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>;
    fn CreateMediaSourceAudioInputNodeWithEmitterAsync(&self, mediasource: &::core::option::Option<super::Core::MediaSource>, emitter: &::core::option::Option<AudioNodeEmitter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphConnectionImpl: Sized {
    fn Destination(&self) -> ::windows::core::Result<IAudioNode>;
    fn SetGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn Gain(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphSettingsImpl: Sized {
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn SetEncodingProperties(&self, value: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation>;
    fn SetPrimaryRenderDevice(&self, value: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<()>;
    fn QuantumSizeSelectionMode(&self) -> ::windows::core::Result<QuantumSizeSelectionMode>;
    fn SetQuantumSizeSelectionMode(&self, value: QuantumSizeSelectionMode) -> ::windows::core::Result<()>;
    fn DesiredSamplesPerQuantum(&self) -> ::windows::core::Result<i32>;
    fn SetDesiredSamplesPerQuantum(&self, value: i32) -> ::windows::core::Result<()>;
    fn AudioRenderCategory(&self) -> ::windows::core::Result<super::Render::AudioRenderCategory>;
    fn SetAudioRenderCategory(&self, value: super::Render::AudioRenderCategory) -> ::windows::core::Result<()>;
    fn DesiredRenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing>;
    fn SetDesiredRenderDeviceAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphSettings2Impl: Sized {
    fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn MaxPlaybackSpeedFactor(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphSettingsFactoryImpl: Sized {
    fn Create(&self, audiorendercategory: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioGraphSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphStaticsImpl: Sized {
    fn CreateAsync(&self, settings: &::core::option::Option<AudioGraphSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioGraphUnrecoverableErrorOccurredEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<AudioGraphUnrecoverableError>;
}
#[cfg(feature = "Foundation")]
pub trait IAudioInputNodeImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>;
    fn AddOutgoingConnection(&self, destination: &::core::option::Option<IAudioNode>) -> ::windows::core::Result<()>;
    fn AddOutgoingConnectionWithGain(&self, destination: &::core::option::Option<IAudioNode>, gain: f64) -> ::windows::core::Result<()>;
    fn RemoveOutgoingConnection(&self, destination: &::core::option::Option<IAudioNode>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
pub trait IAudioInputNode2Impl: Sized + IAudioInputNodeImpl + IAudioNodeImpl + IClosableImpl {
    fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter>;
}
#[cfg(feature = "Foundation")]
pub trait IAudioNodeImpl: Sized + IClosableImpl {
    fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>;
    fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn OutgoingGain(&self) -> ::windows::core::Result<f64>;
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn ConsumeInput(&self) -> ::windows::core::Result<bool>;
    fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn DisableEffectsByDefinition(&self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<()>;
    fn EnableEffectsByDefinition(&self, definition: &::core::option::Option<super::Effects::IAudioEffectDefinition>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetPosition(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDirection(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Shape(&self) -> ::windows::core::Result<AudioNodeEmitterShape>;
    fn DecayModel(&self) -> ::windows::core::Result<AudioNodeEmitterDecayModel>;
    fn Gain(&self) -> ::windows::core::Result<f64>;
    fn SetGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn DistanceScale(&self) -> ::windows::core::Result<f64>;
    fn SetDistanceScale(&self, value: f64) -> ::windows::core::Result<()>;
    fn DopplerScale(&self) -> ::windows::core::Result<f64>;
    fn SetDopplerScale(&self, value: f64) -> ::windows::core::Result<()>;
    fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDopplerVelocity(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn IsDopplerDisabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitter2Impl: Sized {
    fn SpatialAudioModel(&self) -> ::windows::core::Result<SpatialAudioModel>;
    fn SetSpatialAudioModel(&self, value: SpatialAudioModel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterConePropertiesImpl: Sized {
    fn InnerAngle(&self) -> ::windows::core::Result<f64>;
    fn OuterAngle(&self) -> ::windows::core::Result<f64>;
    fn OuterAngleGain(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterDecayModelImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterDecayKind>;
    fn MinGain(&self) -> ::windows::core::Result<f64>;
    fn MaxGain(&self) -> ::windows::core::Result<f64>;
    fn NaturalProperties(&self) -> ::windows::core::Result<AudioNodeEmitterNaturalDecayModelProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterDecayModelStaticsImpl: Sized {
    fn CreateNatural(&self, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel>;
    fn CreateCustom(&self, mingain: f64, maxgain: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterFactoryImpl: Sized {
    fn CreateAudioNodeEmitter(&self, shape: &::core::option::Option<AudioNodeEmitterShape>, decaymodel: &::core::option::Option<AudioNodeEmitterDecayModel>, settings: AudioNodeEmitterSettings) -> ::windows::core::Result<AudioNodeEmitter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterNaturalDecayModelPropertiesImpl: Sized {
    fn UnityGainDistance(&self) -> ::windows::core::Result<f64>;
    fn CutoffDistance(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterShapeImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterShapeKind>;
    fn ConeProperties(&self) -> ::windows::core::Result<AudioNodeEmitterConeProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeEmitterShapeStaticsImpl: Sized {
    fn CreateCone(&self, innerangle: f64, outerangle: f64, outeranglegain: f64) -> ::windows::core::Result<AudioNodeEmitterShape>;
    fn CreateOmnidirectional(&self) -> ::windows::core::Result<AudioNodeEmitterShape>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioNodeListenerImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetPosition(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn SpeedOfSound(&self) -> ::windows::core::Result<f64>;
    fn SetSpeedOfSound(&self, value: f64) -> ::windows::core::Result<()>;
    fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetDopplerVelocity(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
pub trait IAudioNodeWithListenerImpl: Sized + IAudioNodeImpl + IClosableImpl {
    fn SetListener(&self, value: &::core::option::Option<AudioNodeListener>) -> ::windows::core::Result<()>;
    fn Listener(&self) -> ::windows::core::Result<AudioNodeListener>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioPlaybackConnectionImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<AudioPlaybackConnectionState>;
    fn Open(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResult>;
    fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioPlaybackConnection, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioPlaybackConnectionOpenResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResultStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioPlaybackConnectionStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryCreateFromId(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<AudioPlaybackConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStateMonitorImpl: Sized {
    fn SoundLevelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioStateMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSoundLevelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SoundLevel(&self) -> ::windows::core::Result<super::SoundLevel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStateMonitorStaticsImpl: Sized {
    fn CreateForRenderMonitoring(&self) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForRenderMonitoringWithCategory(&self, category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForRenderMonitoringWithCategoryAndDeviceRole(&self, category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForRenderMonitoringWithCategoryAndDeviceId(&self, category: super::Render::AudioRenderCategory, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForCaptureMonitoring(&self) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForCaptureMonitoringWithCategory(&self, category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForCaptureMonitoringWithCategoryAndDeviceRole(&self, category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor>;
    fn CreateForCaptureMonitoringWithCategoryAndDeviceId(&self, category: super::Capture::MediaCategory, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioStateMonitor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioDeviceInputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus>;
    fn DeviceInputNode(&self) -> ::windows::core::Result<AudioDeviceInputNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioDeviceInputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioDeviceOutputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus>;
    fn DeviceOutputNode(&self) -> ::windows::core::Result<AudioDeviceOutputNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioDeviceOutputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioFileInputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus>;
    fn FileInputNode(&self) -> ::windows::core::Result<AudioFileInputNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioFileInputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioFileOutputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus>;
    fn FileOutputNode(&self) -> ::windows::core::Result<AudioFileOutputNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioFileOutputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioGraphResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AudioGraphCreationStatus>;
    fn Graph(&self) -> ::windows::core::Result<AudioGraph>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateAudioGraphResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateMediaSourceAudioInputNodeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MediaSourceAudioInputNodeCreationStatus>;
    fn Node(&self) -> ::windows::core::Result<MediaSourceAudioInputNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateMediaSourceAudioInputNodeResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IEchoEffectDefinitionImpl: Sized + IAudioEffectDefinitionImpl {
    fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()>;
    fn WetDryMix(&self) -> ::windows::core::Result<f64>;
    fn SetFeedback(&self, value: f64) -> ::windows::core::Result<()>;
    fn Feedback(&self) -> ::windows::core::Result<f64>;
    fn SetDelay(&self, value: f64) -> ::windows::core::Result<()>;
    fn Delay(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEchoEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, audiograph: &::core::option::Option<AudioGraph>) -> ::windows::core::Result<EchoEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEqualizerBandImpl: Sized {
    fn Bandwidth(&self) -> ::windows::core::Result<f64>;
    fn SetBandwidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn FrequencyCenter(&self) -> ::windows::core::Result<f64>;
    fn SetFrequencyCenter(&self, value: f64) -> ::windows::core::Result<()>;
    fn Gain(&self) -> ::windows::core::Result<f64>;
    fn SetGain(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IEqualizerEffectDefinitionImpl: Sized + IAudioEffectDefinitionImpl {
    fn Bands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EqualizerBand>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEqualizerEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, audiograph: &::core::option::Option<AudioGraph>) -> ::windows::core::Result<EqualizerEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameInputNodeQuantumStartedEventArgsImpl: Sized {
    fn RequiredSamples(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait ILimiterEffectDefinitionImpl: Sized + IAudioEffectDefinitionImpl {
    fn SetRelease(&self, value: u32) -> ::windows::core::Result<()>;
    fn Release(&self) -> ::windows::core::Result<u32>;
    fn SetLoudness(&self, value: u32) -> ::windows::core::Result<()>;
    fn Loudness(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILimiterEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, audiograph: &::core::option::Option<AudioGraph>) -> ::windows::core::Result<LimiterEffectDefinition>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaSourceAudioInputNodeImpl: Sized + IAudioInputNodeImpl + IAudioInputNode2Impl + IAudioNodeImpl + IClosableImpl {
    fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Seek(&self, position: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetStartTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetEndTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetLoopCount(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource>;
    fn MediaSourceCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSourceAudioInputNode, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMediaSourceCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IReverbEffectDefinitionImpl: Sized + IAudioEffectDefinitionImpl {
    fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()>;
    fn WetDryMix(&self) -> ::windows::core::Result<f64>;
    fn SetReflectionsDelay(&self, value: u32) -> ::windows::core::Result<()>;
    fn ReflectionsDelay(&self) -> ::windows::core::Result<u32>;
    fn SetReverbDelay(&self, value: u8) -> ::windows::core::Result<()>;
    fn ReverbDelay(&self) -> ::windows::core::Result<u8>;
    fn SetRearDelay(&self, value: u8) -> ::windows::core::Result<()>;
    fn RearDelay(&self) -> ::windows::core::Result<u8>;
    fn SetPositionLeft(&self, value: u8) -> ::windows::core::Result<()>;
    fn PositionLeft(&self) -> ::windows::core::Result<u8>;
    fn SetPositionRight(&self, value: u8) -> ::windows::core::Result<()>;
    fn PositionRight(&self) -> ::windows::core::Result<u8>;
    fn SetPositionMatrixLeft(&self, value: u8) -> ::windows::core::Result<()>;
    fn PositionMatrixLeft(&self) -> ::windows::core::Result<u8>;
    fn SetPositionMatrixRight(&self, value: u8) -> ::windows::core::Result<()>;
    fn PositionMatrixRight(&self) -> ::windows::core::Result<u8>;
    fn SetEarlyDiffusion(&self, value: u8) -> ::windows::core::Result<()>;
    fn EarlyDiffusion(&self) -> ::windows::core::Result<u8>;
    fn SetLateDiffusion(&self, value: u8) -> ::windows::core::Result<()>;
    fn LateDiffusion(&self) -> ::windows::core::Result<u8>;
    fn SetLowEQGain(&self, value: u8) -> ::windows::core::Result<()>;
    fn LowEQGain(&self) -> ::windows::core::Result<u8>;
    fn SetLowEQCutoff(&self, value: u8) -> ::windows::core::Result<()>;
    fn LowEQCutoff(&self) -> ::windows::core::Result<u8>;
    fn SetHighEQGain(&self, value: u8) -> ::windows::core::Result<()>;
    fn HighEQGain(&self) -> ::windows::core::Result<u8>;
    fn SetHighEQCutoff(&self, value: u8) -> ::windows::core::Result<()>;
    fn HighEQCutoff(&self) -> ::windows::core::Result<u8>;
    fn SetRoomFilterFreq(&self, value: f64) -> ::windows::core::Result<()>;
    fn RoomFilterFreq(&self) -> ::windows::core::Result<f64>;
    fn SetRoomFilterMain(&self, value: f64) -> ::windows::core::Result<()>;
    fn RoomFilterMain(&self) -> ::windows::core::Result<f64>;
    fn SetRoomFilterHF(&self, value: f64) -> ::windows::core::Result<()>;
    fn RoomFilterHF(&self) -> ::windows::core::Result<f64>;
    fn SetReflectionsGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReflectionsGain(&self) -> ::windows::core::Result<f64>;
    fn SetReverbGain(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReverbGain(&self) -> ::windows::core::Result<f64>;
    fn SetDecayTime(&self, value: f64) -> ::windows::core::Result<()>;
    fn DecayTime(&self) -> ::windows::core::Result<f64>;
    fn SetDensity(&self, value: f64) -> ::windows::core::Result<()>;
    fn Density(&self) -> ::windows::core::Result<f64>;
    fn SetRoomSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn RoomSize(&self) -> ::windows::core::Result<f64>;
    fn SetDisableLateField(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisableLateField(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IReverbEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, audiograph: &::core::option::Option<AudioGraph>) -> ::windows::core::Result<ReverbEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetDefaultSpatialAudioFormatResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SetDefaultSpatialAudioFormatStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioDeviceConfigurationImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsSpatialAudioSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSpatialAudioFormatSupported(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn ActiveSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultSpatialAudioFormatAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>>;
    fn ConfigurationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConfigurationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioDeviceConfigurationStaticsImpl: Sized {
    fn GetForDeviceId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<SpatialAudioDeviceConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioFormatConfigurationImpl: Sized {
    fn ReportLicenseChangedAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ReportConfigurationChangedAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MixedRealityExclusiveModePolicy(&self) -> ::windows::core::Result<MixedRealitySpatialAudioFormatPolicy>;
    fn SetMixedRealityExclusiveModePolicy(&self, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioFormatConfigurationStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SpatialAudioFormatConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioFormatSubtypeStaticsImpl: Sized {
    fn WindowsSonic(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DolbyAtmosForHeadphones(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DolbyAtmosForHomeTheater(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DolbyAtmosForSpeakers(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DTSHeadphoneX(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DTSXUltra(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialAudioFormatSubtypeStatics2Impl: Sized {
    fn DTSXForHomeTheater(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
