#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ActivationSignalDetectionConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(pub i32);
impl ActivationSignalDetectionConfigurationCreationStatus {
    pub const Success: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(0i32);
    pub const SignalIdNotAvailable: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(1i32);
    pub const ModelIdNotSupported: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(2i32);
    pub const InvalidSignalId: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(3i32);
    pub const InvalidModelId: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(4i32);
    pub const InvalidDisplayName: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(5i32);
    pub const ConfigurationAlreadyExists: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(6i32);
    pub const CreationNotSupported: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(7i32);
}
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(pub i32);
impl ActivationSignalDetectionConfigurationRemovalResult {
    pub const Success: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(0i32);
    pub const NotFound: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(1i32);
    pub const CurrentlyEnabled: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(2i32);
    pub const RemovalNotSupported: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(3i32);
}
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(pub i32);
impl ActivationSignalDetectionConfigurationSetModelDataResult {
    pub const Success: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(0i32);
    pub const EmptyModelData: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(1i32);
    pub const UnsupportedFormat: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(2i32);
    pub const ConfigurationCurrentlyEnabled: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(3i32);
    pub const InvalidData: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(4i32);
    pub const SetModelDataNotSupported: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(5i32);
    pub const ConfigurationNotFound: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(6i32);
    pub const UnknownError: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(7i32);
}
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(pub i32);
impl ActivationSignalDetectionConfigurationStateChangeResult {
    pub const Success: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(0i32);
    pub const NoModelData: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(1i32);
    pub const ConfigurationNotFound: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(2i32);
}
#[repr(transparent)]
pub struct ActivationSignalDetectionTrainingDataFormat(pub i32);
impl ActivationSignalDetectionTrainingDataFormat {
    pub const Voice8kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(0i32);
    pub const Voice8kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(1i32);
    pub const Voice16kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(2i32);
    pub const Voice16kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(3i32);
    pub const VoiceOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(4i32);
    pub const Audio44kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(5i32);
    pub const Audio44kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(6i32);
    pub const Audio48kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(7i32);
    pub const Audio48kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(8i32);
    pub const AudioOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(9i32);
    pub const OtherOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(10i32);
}
#[repr(transparent)]
pub struct ActivationSignalDetector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivationSignalDetectorKind(pub i32);
impl ActivationSignalDetectorKind {
    pub const AudioPattern: ActivationSignalDetectorKind = ActivationSignalDetectorKind(0i32);
    pub const AudioImpulse: ActivationSignalDetectorKind = ActivationSignalDetectorKind(1i32);
    pub const HardwareEvent: ActivationSignalDetectorKind = ActivationSignalDetectorKind(2i32);
}
#[repr(transparent)]
pub struct ActivationSignalDetectorPowerState(pub i32);
impl ActivationSignalDetectorPowerState {
    pub const HighPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(0i32);
    pub const ConnectedLowPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(1i32);
    pub const DisconnectedLowPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(2i32);
}
#[repr(transparent)]
pub struct ConversationalAgentActivationKind(pub i32);
impl ConversationalAgentActivationKind {
    pub const VoiceActivationPreview: ConversationalAgentActivationKind = ConversationalAgentActivationKind(0i32);
    pub const Foreground: ConversationalAgentActivationKind = ConversationalAgentActivationKind(1i32);
}
#[repr(transparent)]
pub struct ConversationalAgentActivationResult(pub i32);
impl ConversationalAgentActivationResult {
    pub const Success: ConversationalAgentActivationResult = ConversationalAgentActivationResult(0i32);
    pub const AgentInactive: ConversationalAgentActivationResult = ConversationalAgentActivationResult(1i32);
    pub const ScreenNotAvailable: ConversationalAgentActivationResult = ConversationalAgentActivationResult(2i32);
    pub const AgentInterrupted: ConversationalAgentActivationResult = ConversationalAgentActivationResult(3i32);
}
#[repr(transparent)]
pub struct ConversationalAgentDetectorManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentSessionInterruptedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentSessionUpdateResponse(pub i32);
impl ConversationalAgentSessionUpdateResponse {
    pub const Success: ConversationalAgentSessionUpdateResponse = ConversationalAgentSessionUpdateResponse(0i32);
    pub const Failed: ConversationalAgentSessionUpdateResponse = ConversationalAgentSessionUpdateResponse(1i32);
}
#[repr(transparent)]
pub struct ConversationalAgentSignal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentSignalDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentState(pub i32);
impl ConversationalAgentState {
    pub const Inactive: ConversationalAgentState = ConversationalAgentState(0i32);
    pub const Detecting: ConversationalAgentState = ConversationalAgentState(1i32);
    pub const Listening: ConversationalAgentState = ConversationalAgentState(2i32);
    pub const Working: ConversationalAgentState = ConversationalAgentState(3i32);
    pub const Speaking: ConversationalAgentState = ConversationalAgentState(4i32);
    pub const ListeningAndSpeaking: ConversationalAgentState = ConversationalAgentState(5i32);
}
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangeType(pub i32);
impl ConversationalAgentSystemStateChangeType {
    pub const UserAuthentication: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(0i32);
    pub const ScreenAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(1i32);
    pub const IndicatorLightAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(2i32);
    pub const VoiceActivationAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(3i32);
}
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(pub i32);
impl ConversationalAgentVoiceActivationPrerequisiteKind {
    pub const MicrophonePermission: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(0i32);
    pub const KnownAgents: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(1i32);
    pub const AgentAllowed: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(2i32);
    pub const AppCapability: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(3i32);
    pub const BackgroundTaskRegistration: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(4i32);
    pub const PolicyPermission: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(5i32);
}
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangeKind(pub i32);
impl DetectionConfigurationAvailabilityChangeKind {
    pub const SystemResourceAccess: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(0i32);
    pub const Permission: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(1i32);
    pub const LockScreenPermission: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(2i32);
}
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DetectionConfigurationTrainingStatus(pub i32);
impl DetectionConfigurationTrainingStatus {
    pub const Success: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(0i32);
    pub const FormatNotSupported: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(1i32);
    pub const VoiceTooQuiet: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(2i32);
    pub const VoiceTooLoud: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(3i32);
    pub const VoiceTooFast: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(4i32);
    pub const VoiceTooSlow: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(5i32);
    pub const VoiceQualityProblem: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(6i32);
    pub const TrainingSystemInternalError: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(7i32);
    pub const TrainingTimedOut: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(8i32);
    pub const ConfigurationNotFound: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(9i32);
}
#[repr(transparent)]
pub struct IActivationSignalDetectionConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivationSignalDetectionConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivationSignalDetectionConfigurationCreationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivationSignalDetector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivationSignalDetector2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentDetectorManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentDetectorManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentDetectorManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentSessionInterruptedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentSessionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentSignal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentSignal2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentSignalDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConversationalAgentSystemStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SignalDetectorResourceKind(pub i32);
impl SignalDetectorResourceKind {
    pub const ParallelModelSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(0i32);
    pub const ParallelModelSupportForAgent: SignalDetectorResourceKind = SignalDetectorResourceKind(1i32);
    pub const ParallelSignalSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(2i32);
    pub const ParallelSignalSupportForAgent: SignalDetectorResourceKind = SignalDetectorResourceKind(3i32);
    pub const DisplayOffSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(4i32);
    pub const PluggedInPower: SignalDetectorResourceKind = SignalDetectorResourceKind(5i32);
    pub const Detector: SignalDetectorResourceKind = SignalDetectorResourceKind(6i32);
    pub const SupportedSleepState: SignalDetectorResourceKind = SignalDetectorResourceKind(7i32);
    pub const SupportedBatterySaverState: SignalDetectorResourceKind = SignalDetectorResourceKind(8i32);
    pub const ScreenAvailability: SignalDetectorResourceKind = SignalDetectorResourceKind(9i32);
    pub const InputHardware: SignalDetectorResourceKind = SignalDetectorResourceKind(10i32);
    pub const AcousticEchoCancellation: SignalDetectorResourceKind = SignalDetectorResourceKind(11i32);
    pub const ModelIdSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(12i32);
    pub const DataChannel: SignalDetectorResourceKind = SignalDetectorResourceKind(13i32);
}
