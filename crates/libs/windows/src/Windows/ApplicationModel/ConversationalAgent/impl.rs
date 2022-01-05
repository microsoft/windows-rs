#[cfg(feature = "implement_exclusive")]
pub trait IActivationSignalDetectionConfigurationImpl: Sized {
    fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetEnabledAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AvailabilityInfo(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityInfo>;
    fn AvailabilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailabilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetModelData(&self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn SetModelDataAsync(&self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetModelDataType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetModelDataTypeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetModelData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>>;
    fn ClearModelData(&self) -> ::windows::core::Result<()>;
    fn ClearModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrainingStepsCompleted(&self) -> ::windows::core::Result<u32>;
    fn TrainingStepsRemaining(&self) -> ::windows::core::Result<u32>;
    fn TrainingDataFormat(&self) -> ::windows::core::Result<ActivationSignalDetectionTrainingDataFormat>;
    fn ApplyTrainingData(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<DetectionConfigurationTrainingStatus>;
    fn ApplyTrainingDataAsync(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>;
    fn ClearTrainingData(&self) -> ::windows::core::Result<()>;
    fn ClearTrainingDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivationSignalDetectionConfiguration2Impl: Sized {
    fn SetModelDataWithResult(&self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<ActivationSignalDetectionConfigurationSetModelDataResult>;
    fn SetModelDataWithResultAsync(&self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>;
    fn SetEnabledWithResultAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>>;
    fn SetEnabledWithResult(&self, value: bool) -> ::windows::core::Result<ActivationSignalDetectionConfigurationStateChangeResult>;
    fn TrainingStepCompletionMaxAllowedTime(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivationSignalDetectionConfigurationCreationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationStatus>;
    fn Configuration(&self) -> ::windows::core::Result<ActivationSignalDetectionConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivationSignalDetectorImpl: Sized {
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind>;
    fn CanCreateConfigurations(&self) -> ::windows::core::Result<bool>;
    fn SupportedModelDataTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SupportedTrainingDataFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>>;
    fn SupportedPowerStates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>>;
    fn GetSupportedModelIdsForSignalId(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetSupportedModelIdsForSignalIdAsync(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn CreateConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CreateConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetConfigurations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>;
    fn GetConfigurationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>>;
    fn GetConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfiguration>;
    fn GetConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>>;
    fn RemoveConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivationSignalDetector2Impl: Sized {
    fn GetAvailableModelIdsForSignalIdAsync(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn GetAvailableModelIdsForSignalId(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn CreateConfigurationWithResultAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>>;
    fn CreateConfigurationWithResult(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationResult>;
    fn RemoveConfigurationWithResultAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>>;
    fn RemoveConfigurationWithResult(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfigurationRemovalResult>;
    fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentDetectorManagerImpl: Sized {
    fn GetAllActivationSignalDetectors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>;
    fn GetAllActivationSignalDetectorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>;
    fn GetActivationSignalDetectors(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>;
    fn GetActivationSignalDetectorsAsync(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentDetectorManager2Impl: Sized {
    fn GetActivationSignalDetectorFromId(&self, detectorid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetector>;
    fn GetActivationSignalDetectorFromIdAsync(&self, detectorid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentDetectorManagerStaticsImpl: Sized {
    fn Default(&self) -> ::windows::core::Result<ConversationalAgentDetectorManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSessionImpl: Sized {
    fn SessionInterrupted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionInterrupted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SignalDetected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSignalDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AgentState(&self) -> ::windows::core::Result<ConversationalAgentState>;
    fn Signal(&self) -> ::windows::core::Result<ConversationalAgentSignal>;
    fn IsIndicatorLightAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsScreenAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsUserAuthenticated(&self) -> ::windows::core::Result<bool>;
    fn IsVoiceActivationAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsInterruptible(&self) -> ::windows::core::Result<bool>;
    fn IsInterrupted(&self) -> ::windows::core::Result<bool>;
    fn RequestInterruptibleAsync(&self, interruptible: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestInterruptible(&self, interruptible: bool) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn RequestAgentStateChangeAsync(&self, state: ConversationalAgentState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestAgentStateChange(&self, state: ConversationalAgentState) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn RequestForegroundActivationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestForegroundActivation(&self) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn GetAudioClientAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetAudioClient(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateAudioDeviceInputNodeAsync(&self, graph: &::core::option::Option<super::super::Media::Audio::AudioGraph>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>>;
    fn CreateAudioDeviceInputNode(&self, graph: &::core::option::Option<super::super::Media::Audio::AudioGraph>) -> ::windows::core::Result<super::super::Media::Audio::AudioDeviceInputNode>;
    fn GetAudioCaptureDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetAudioCaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAudioRenderDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetAudioRenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSignalModelIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetSignalModelId(&self) -> ::windows::core::Result<u32>;
    fn SetSignalModelIdAsync(&self, signalmodelid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SetSignalModelId(&self, signalmodelid: u32) -> ::windows::core::Result<bool>;
    fn GetSupportedSignalModelIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>;
    fn GetSupportedSignalModelIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSession2Impl: Sized {
    fn RequestActivationAsync(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>>;
    fn RequestActivation(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<ConversationalAgentActivationResult>;
    fn SetSupportLockScreenActivationAsync(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetSupportLockScreenActivation(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<()>;
    fn GetMissingPrerequisites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>;
    fn GetMissingPrerequisitesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSessionInterruptedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSessionStaticsImpl: Sized {
    fn GetCurrentSessionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>>;
    fn GetCurrentSessionSync(&self) -> ::windows::core::Result<ConversationalAgentSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSignalImpl: Sized {
    fn IsSignalVerificationRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsSignalVerificationRequired(&self, value: bool) -> ::windows::core::Result<()>;
    fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSignalId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SignalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSignalName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SignalContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSignalContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SignalStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetSignalStart(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SignalEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetSignalEnd(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSignal2Impl: Sized {
    fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DetectorKind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSignalDetectedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSystemStateChangedEventArgsImpl: Sized {
    fn SystemStateChangeType(&self) -> ::windows::core::Result<ConversationalAgentSystemStateChangeType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDetectionConfigurationAvailabilityChangedEventArgsImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityChangeKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDetectionConfigurationAvailabilityInfoImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn HasSystemResourceAccess(&self) -> ::windows::core::Result<bool>;
    fn HasPermission(&self) -> ::windows::core::Result<bool>;
    fn HasLockScreenPermission(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDetectionConfigurationAvailabilityInfo2Impl: Sized {
    fn UnavailableSystemResources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>>;
}
