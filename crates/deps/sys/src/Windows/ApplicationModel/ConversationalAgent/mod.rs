#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ActivationSignalDetectionConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(i32);
#[repr(C)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(i32);
#[repr(C)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(i32);
#[repr(C)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(i32);
#[repr(C)]
pub struct ActivationSignalDetectionTrainingDataFormat(i32);
#[repr(transparent)]
pub struct ActivationSignalDetector(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ActivationSignalDetectorKind(i32);
#[repr(C)]
pub struct ActivationSignalDetectorPowerState(i32);
#[repr(C)]
pub struct ConversationalAgentActivationKind(i32);
#[repr(C)]
pub struct ConversationalAgentActivationResult(i32);
#[repr(transparent)]
pub struct ConversationalAgentDetectorManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentSessionInterruptedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ConversationalAgentSessionUpdateResponse(i32);
#[repr(transparent)]
pub struct ConversationalAgentSignal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentSignalDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ConversationalAgentState(i32);
#[repr(C)]
pub struct ConversationalAgentSystemStateChangeType(i32);
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(i32);
#[repr(C)]
pub struct DetectionConfigurationAvailabilityChangeKind(i32);
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DetectionConfigurationTrainingStatus(i32);
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
#[repr(C)]
pub struct SignalDetectorResourceKind(i32);
