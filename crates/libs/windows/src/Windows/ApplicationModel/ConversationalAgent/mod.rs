#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40d8be16_5217_581c_9ab2_ce9b2f2e8e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEnabledAsync: usize,
    pub AvailabilityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailabilityChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetModelDataAsync: usize,
    pub GetModelDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetModelDataTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetModelDataTypeAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetModelData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetModelDataAsync: usize,
    pub ClearModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearModelDataAsync: usize,
    pub TrainingStepsCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TrainingStepsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TrainingDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionTrainingDataFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ApplyTrainingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: *mut ::core::ffi::c_void, result__: *mut DetectionConfigurationTrainingStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ApplyTrainingData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ApplyTrainingDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ApplyTrainingDataAsync: usize,
    pub ClearTrainingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearTrainingDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearTrainingDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IActivationSignalDetectionConfiguration2 {
    type Vtable = IActivationSignalDetectionConfiguration2_Vtbl;
}
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfiguration2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71d9b022_562c_57ce_a78b_8b4ff0145bab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataWithResult: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetModelDataWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetModelDataWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetEnabledWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEnabledWithResultAsync: usize,
    pub SetEnabledWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ActivationSignalDetectionConfigurationStateChangeResult) -> ::windows::core::HRESULT,
    pub TrainingStepCompletionMaxAllowedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfigurationCreationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfigurationCreationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c89bc1b_8d12_5e48_a71c_7f6bc1cd66e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfigurationCreationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionConfigurationCreationStatus) -> ::windows::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetector(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IActivationSignalDetector {
    type Vtable = IActivationSignalDetector_Vtbl;
}
unsafe impl ::windows::core::Interface for IActivationSignalDetector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bf345f_a4d0_5b2b_8e65_b3c55ee756ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT,
    pub CanCreateConfigurations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModelDataTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModelDataTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTrainingDataFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTrainingDataFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPowerStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPowerStates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalId: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalIdAsync: usize,
    pub CreateConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateConfigurationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurationsAsync: usize,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConfigurationAsync: usize,
    pub RemoveConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetector2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IActivationSignalDetector2 {
    type Vtable = IActivationSignalDetector2_Vtbl;
}
unsafe impl ::windows::core::Interface for IActivationSignalDetector2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7e2490a_baa5_59d2_85d1_ba42f7cf78c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalId: usize,
    #[cfg(feature = "Foundation")]
    pub CreateConfigurationWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateConfigurationWithResultAsync: usize,
    pub CreateConfigurationWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationWithResultAsync: usize,
    pub RemoveConfigurationWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: *mut ::core::ffi::c_void, modelid: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionConfigurationRemovalResult) -> ::windows::core::HRESULT,
    pub DetectorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde94fbb0_597a_5df8_8cfb_9dbb583ba3ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectorsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectorsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentDetectorManager2 {
    type Vtable = IConversationalAgentDetectorManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84610f31_d7f3_52fe_9311_c9eb4e3eb30a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetActivationSignalDetectorFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, detectorid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetActivationSignalDetectorFromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, detectorid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetActivationSignalDetectorFromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentDetectorManagerStatics {
    type Vtable = IConversationalAgentDetectorManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36a8d283_fa0e_5693_8489_0fb2f0ab40d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentSession {
    type Vtable = IConversationalAgentSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaaae09a_b7ba_57e5_ad13_df520f9b6fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SessionInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionInterrupted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionInterrupted: usize,
    #[cfg(feature = "Foundation")]
    pub SignalDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSignalDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSignalDetected: usize,
    #[cfg(feature = "Foundation")]
    pub SystemStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemStateChanged: usize,
    pub AgentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentState) -> ::windows::core::HRESULT,
    pub Signal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsIndicatorLightAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsScreenAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUserAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVoiceActivationAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInterruptible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestInterruptibleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestInterruptibleAsync: usize,
    pub RequestInterruptible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAgentStateChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAgentStateChangeAsync: usize,
    pub RequestAgentStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestForegroundActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestForegroundActivationAsync: usize,
    pub RequestForegroundActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAudioClientAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioClientAsync: usize,
    pub GetAudioClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))]
    pub CreateAudioDeviceInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, graph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Audio")))]
    CreateAudioDeviceInputNodeAsync: usize,
    #[cfg(feature = "Media_Audio")]
    pub CreateAudioDeviceInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, graph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    CreateAudioDeviceInputNode: usize,
    #[cfg(feature = "Foundation")]
    pub GetAudioCaptureDeviceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioCaptureDeviceIdAsync: usize,
    pub GetAudioCaptureDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAudioRenderDeviceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioRenderDeviceIdAsync: usize,
    pub GetAudioRenderDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetSignalModelIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSignalModelIdAsync: usize,
    pub GetSignalModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetSignalModelIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalModelIdAsync: usize,
    pub SetSignalModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSession2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentSession2 {
    type Vtable = IConversationalAgentSession2_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentSession2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7a9fbf9_ac78_57ff_9596_acc7a1c9a607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestActivationAsync: usize,
    pub RequestActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ConversationalAgentActivationResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetSupportLockScreenActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSupportLockScreenActivationAsync: usize,
    pub SetSupportLockScreenActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisites: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisitesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisitesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSessionInterruptedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentSessionInterruptedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9766591f_f63d_5d3e_9bf2_bd0760552686);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionInterruptedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSessionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentSessionStatics {
    type Vtable = IConversationalAgentSessionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentSessionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa005166e_e954_576e_be04_11b8ed10f37b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCurrentSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentSessionAsync: usize,
    pub GetCurrentSessionSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignal(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentSignal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20ed25f7_b120_51f2_8603_265d6a47f232);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSignalVerificationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSignalVerificationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SignalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSignalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SignalContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSignalContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SignalStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetSignalStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalStart: usize,
    #[cfg(feature = "Foundation")]
    pub SignalEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetSignalEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalEnd: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignal2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentSignal2 {
    type Vtable = IConversationalAgentSignal2_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentSignal2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0cc7ba9_9a7b_5c34_880e_b6146c904ecb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DetectorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DetectorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignalDetectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentSignalDetectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d57eb8f_f88a_599b_91d3_d604876708bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignalDetectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSystemStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IConversationalAgentSystemStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2c6e3e_2785_59a7_8e71_38adeef79928);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSystemStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SystemStateChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSystemStateChangeType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5129c9fb_4be8_5f14_af2b_88d62b1b4462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DetectionConfigurationAvailabilityChangeKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5affeb0_40f0_5398_b838_91979c2c6208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasSystemResourceAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasLockScreenPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDetectionConfigurationAvailabilityInfo2 {
    type Vtable = IDetectionConfigurationAvailabilityInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30e06433_38b3_5c4b_84c3_62b6e685b2ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub UnavailableSystemResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnavailableSystemResources: usize,
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfiguration(::windows::core::IUnknown);
impl ActivationSignalDetectionConfiguration {
    pub fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignalId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsActive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEnabledAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetEnabledAsync)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AvailabilityInfo(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AvailabilityInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AvailabilityChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AvailabilityChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailabilityChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAvailabilityChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelData<P0, E0>(&self, datatype: &::windows::core::HSTRING, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetModelData)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(datatype), data.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetModelDataAsync<P0, E0>(&self, datatype: &::windows::core::HSTRING, data: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetModelDataAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(datatype), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetModelDataType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetModelDataType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetModelDataTypeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetModelDataTypeAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetModelData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetModelData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetModelDataAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ClearModelData(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ClearModelData)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClearModelDataAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrainingStepsCompleted(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrainingStepsCompleted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrainingStepsRemaining(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrainingStepsRemaining)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrainingDataFormat(&self) -> ::windows::core::Result<ActivationSignalDetectionTrainingDataFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrainingDataFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ApplyTrainingData<P0, E0>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: P0) -> ::windows::core::Result<DetectionConfigurationTrainingStatus>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ApplyTrainingData)(::windows::core::Vtable::as_raw(this), trainingdataformat, trainingdata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ApplyTrainingDataAsync<P0, E0>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ApplyTrainingDataAsync)(::windows::core::Vtable::as_raw(this), trainingdataformat, trainingdata.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ClearTrainingData(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ClearTrainingData)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearTrainingDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClearTrainingDataAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataWithResult<P0, E0>(&self, datatype: &::windows::core::HSTRING, data: P0) -> ::windows::core::Result<ActivationSignalDetectionConfigurationSetModelDataResult>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetModelDataWithResult)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(datatype), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetModelDataWithResultAsync<P0, E0>(&self, datatype: &::windows::core::HSTRING, data: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetModelDataWithResultAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(datatype), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEnabledWithResultAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetEnabledWithResultAsync)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetEnabledWithResult(&self, value: bool) -> ::windows::core::Result<ActivationSignalDetectionConfigurationStateChangeResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetEnabledWithResult)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TrainingStepCompletionMaxAllowedTime(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrainingStepCompletionMaxAllowedTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ActivationSignalDetectionConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetectionConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetectionConfiguration {}
impl ::core::fmt::Debug for ActivationSignalDetectionConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration;{40d8be16-5217-581c-9ab2-ce9b2f2e8e00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for ActivationSignalDetectionConfiguration {
    const IID: ::windows::core::GUID = <IActivationSignalDetectionConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivationSignalDetectionConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration";
}
::windows::core::interface_hierarchy!(ActivationSignalDetectionConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ActivationSignalDetectionConfiguration> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ActivationSignalDetectionConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ActivationSignalDetectionConfiguration> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ActivationSignalDetectionConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ActivationSignalDetectionConfiguration> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ActivationSignalDetectionConfiguration) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetectionConfiguration {}
unsafe impl ::core::marker::Sync for ActivationSignalDetectionConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationResult(::windows::core::IUnknown);
impl ActivationSignalDetectionConfigurationCreationResult {
    pub fn Status(&self) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows::core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Configuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetectionConfigurationCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetectionConfigurationCreationResult {}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationCreationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult;{4c89bc1b-8d12-5e48-a71c-7f6bc1cd66e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ActivationSignalDetectionConfigurationCreationResult {
    const IID: ::windows::core::GUID = <IActivationSignalDetectionConfigurationCreationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivationSignalDetectionConfigurationCreationResult {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult";
}
::windows::core::interface_hierarchy!(ActivationSignalDetectionConfigurationCreationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ActivationSignalDetectionConfigurationCreationResult {}
unsafe impl ::core::marker::Sync for ActivationSignalDetectionConfigurationCreationResult {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetector(::windows::core::IUnknown);
impl ActivationSignalDetector {
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProviderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanCreateConfigurations(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanCreateConfigurations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModelDataTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedModelDataTypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTrainingDataFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedTrainingDataFormats)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPowerStates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedPowerStates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalId(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSupportedModelIdsForSignalId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalIdAsync(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSupportedModelIdsForSignalIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CreateConfiguration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), ::core::mem::transmute_copy(displayname)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateConfigurationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConfigurations)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConfigurationsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConfiguration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConfigurationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveConfiguration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveConfigurationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalIdAsync(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAvailableModelIdsForSignalIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalId(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAvailableModelIdsForSignalId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateConfigurationWithResultAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateConfigurationWithResultAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateConfigurationWithResult(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateConfigurationWithResult)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationWithResultAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveConfigurationWithResultAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveConfigurationWithResult(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfigurationRemovalResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveConfigurationWithResult)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalid), ::core::mem::transmute_copy(modelid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetectorId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ActivationSignalDetector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetector {}
impl ::core::fmt::Debug for ActivationSignalDetector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector;{b5bf345f-a4d0-5b2b-8e65-b3c55ee756ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ActivationSignalDetector {
    type Vtable = IActivationSignalDetector_Vtbl;
}
unsafe impl ::windows::core::Interface for ActivationSignalDetector {
    const IID: ::windows::core::GUID = <IActivationSignalDetector as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivationSignalDetector {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector";
}
::windows::core::interface_hierarchy!(ActivationSignalDetector, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ActivationSignalDetector {}
unsafe impl ::core::marker::Sync for ActivationSignalDetector {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentDetectorManager(::windows::core::IUnknown);
impl ConversationalAgentDetectorManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAllActivationSignalDetectors)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAllActivationSignalDetectorsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectors(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetActivationSignalDetectors)(::windows::core::Vtable::as_raw(this), kind, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectorsAsync(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetActivationSignalDetectorsAsync)(::windows::core::Vtable::as_raw(this), kind, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetActivationSignalDetectorFromId(&self, detectorid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetector> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetActivationSignalDetectorFromId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(detectorid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetActivationSignalDetectorFromIdAsync(&self, detectorid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetActivationSignalDetectorFromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(detectorid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Default() -> ::windows::core::Result<ConversationalAgentDetectorManager> {
        Self::IConversationalAgentDetectorManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Default)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IConversationalAgentDetectorManagerStatics<R, F: FnOnce(&IConversationalAgentDetectorManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ConversationalAgentDetectorManager, IConversationalAgentDetectorManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ConversationalAgentDetectorManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentDetectorManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentDetectorManager {}
impl ::core::fmt::Debug for ConversationalAgentDetectorManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentDetectorManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentDetectorManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager;{de94fbb0-597a-5df8-8cfb-9dbb583ba3ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_Vtbl;
}
unsafe impl ::windows::core::Interface for ConversationalAgentDetectorManager {
    const IID: ::windows::core::GUID = <IConversationalAgentDetectorManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentDetectorManager {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager";
}
::windows::core::interface_hierarchy!(ConversationalAgentDetectorManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ConversationalAgentDetectorManager {}
unsafe impl ::core::marker::Sync for ConversationalAgentDetectorManager {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSession(::windows::core::IUnknown);
impl ConversationalAgentSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SessionInterrupted(&self, handler: &super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionInterrupted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionInterrupted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSessionInterrupted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignalDetected(&self, handler: &super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignalDetected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSignalDetected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSignalDetected)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemStateChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSystemStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AgentState(&self) -> ::windows::core::Result<ConversationalAgentState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AgentState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Signal(&self) -> ::windows::core::Result<ConversationalAgentSignal> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Signal)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsIndicatorLightAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsIndicatorLightAvailable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsScreenAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsScreenAvailable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsUserAuthenticated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsUserAuthenticated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsVoiceActivationAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVoiceActivationAvailable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsInterruptible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInterruptible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsInterrupted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInterrupted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestInterruptibleAsync(&self, interruptible: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestInterruptibleAsync)(::windows::core::Vtable::as_raw(this), interruptible, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RequestInterruptible(&self, interruptible: bool) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestInterruptible)(::windows::core::Vtable::as_raw(this), interruptible, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAgentStateChangeAsync(&self, state: ConversationalAgentState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAgentStateChangeAsync)(::windows::core::Vtable::as_raw(this), state, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RequestAgentStateChange(&self, state: ConversationalAgentState) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAgentStateChange)(::windows::core::Vtable::as_raw(this), state, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestForegroundActivationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestForegroundActivationAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RequestForegroundActivation(&self) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestForegroundActivation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAudioClientAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioClientAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetAudioClient(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioClient)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Audio\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))]
    pub fn CreateAudioDeviceInputNodeAsync(&self, graph: &super::super::Media::Audio::AudioGraph) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAudioDeviceInputNodeAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(graph), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    #[cfg(feature = "Media_Audio")]
    pub fn CreateAudioDeviceInputNode(&self, graph: &super::super::Media::Audio::AudioGraph) -> ::windows::core::Result<super::super::Media::Audio::AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAudioDeviceInputNode)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(graph), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAudioCaptureDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioCaptureDeviceIdAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetAudioCaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioCaptureDeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAudioRenderDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioRenderDeviceIdAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetAudioRenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioRenderDeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSignalModelIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSignalModelIdAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSignalModelId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSignalModelId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSignalModelIdAsync(&self, signalmodelid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetSignalModelIdAsync)(::windows::core::Vtable::as_raw(this), signalmodelid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSignalModelId(&self, signalmodelid: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetSignalModelId)(::windows::core::Vtable::as_raw(this), signalmodelid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSupportedSignalModelIdsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSupportedSignalModelIds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestActivationAsync(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestActivationAsync)(::windows::core::Vtable::as_raw(this), activationkind, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RequestActivation(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<ConversationalAgentActivationResult> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestActivation)(::windows::core::Vtable::as_raw(this), activationkind, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSupportLockScreenActivationAsync(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetSupportLockScreenActivationAsync)(::windows::core::Vtable::as_raw(this), lockscreenactivationsupported, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSupportLockScreenActivation(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSupportLockScreenActivation)(::windows::core::Vtable::as_raw(this), lockscreenactivationsupported).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMissingPrerequisites)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisitesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMissingPrerequisitesAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentSessionAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentSessionAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetCurrentSessionSync() -> ::windows::core::Result<ConversationalAgentSession> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentSessionSync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IConversationalAgentSessionStatics<R, F: FnOnce(&IConversationalAgentSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ConversationalAgentSession, IConversationalAgentSessionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ConversationalAgentSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSession {}
impl ::core::fmt::Debug for ConversationalAgentSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession;{daaae09a-b7ba-57e5-ad13-df520f9b6fa7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ConversationalAgentSession {
    type Vtable = IConversationalAgentSession_Vtbl;
}
unsafe impl ::windows::core::Interface for ConversationalAgentSession {
    const IID: ::windows::core::GUID = <IConversationalAgentSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSession {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession";
}
::windows::core::interface_hierarchy!(ConversationalAgentSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ConversationalAgentSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ConversationalAgentSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ConversationalAgentSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ConversationalAgentSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ConversationalAgentSession> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ConversationalAgentSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSession {}
unsafe impl ::core::marker::Sync for ConversationalAgentSession {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSessionInterruptedEventArgs(::windows::core::IUnknown);
impl ConversationalAgentSessionInterruptedEventArgs {}
impl ::core::clone::Clone for ConversationalAgentSessionInterruptedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSessionInterruptedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSessionInterruptedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSessionInterruptedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSessionInterruptedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSessionInterruptedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs;{9766591f-f63d-5d3e-9bf2-bd0760552686})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ConversationalAgentSessionInterruptedEventArgs {
    const IID: ::windows::core::GUID = <IConversationalAgentSessionInterruptedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSessionInterruptedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs";
}
::windows::core::interface_hierarchy!(ConversationalAgentSessionInterruptedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ConversationalAgentSessionInterruptedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSessionInterruptedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSignal(::windows::core::IUnknown);
impl ConversationalAgentSignal {
    pub fn IsSignalVerificationRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSignalVerificationRequired)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsSignalVerificationRequired(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsSignalVerificationRequired)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignalId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSignalId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSignalId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SignalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSignalName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSignalName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SignalContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignalContext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSignalContext<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSignalContext)(::windows::core::Vtable::as_raw(this), value.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignalStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignalStart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSignalStart(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSignalStart)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignalEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SignalEnd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSignalEnd(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSignalEnd)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetectorId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DetectorKind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetectorKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ConversationalAgentSignal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSignal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSignal {}
impl ::core::fmt::Debug for ConversationalAgentSignal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSignal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSignal {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal;{20ed25f7-b120-51f2-8603-265d6a47f232})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_Vtbl;
}
unsafe impl ::windows::core::Interface for ConversationalAgentSignal {
    const IID: ::windows::core::GUID = <IConversationalAgentSignal as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSignal {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal";
}
::windows::core::interface_hierarchy!(ConversationalAgentSignal, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ConversationalAgentSignal {}
unsafe impl ::core::marker::Sync for ConversationalAgentSignal {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSignalDetectedEventArgs(::windows::core::IUnknown);
impl ConversationalAgentSignalDetectedEventArgs {}
impl ::core::clone::Clone for ConversationalAgentSignalDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSignalDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSignalDetectedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSignalDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSignalDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSignalDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs;{4d57eb8f-f88a-599b-91d3-d604876708bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ConversationalAgentSignalDetectedEventArgs {
    const IID: ::windows::core::GUID = <IConversationalAgentSignalDetectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSignalDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs";
}
::windows::core::interface_hierarchy!(ConversationalAgentSignalDetectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ConversationalAgentSignalDetectedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSignalDetectedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangedEventArgs(::windows::core::IUnknown);
impl ConversationalAgentSystemStateChangedEventArgs {
    pub fn SystemStateChangeType(&self) -> ::windows::core::Result<ConversationalAgentSystemStateChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemStateChangeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ConversationalAgentSystemStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSystemStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSystemStateChangedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSystemStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSystemStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSystemStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs;{1c2c6e3e-2785-59a7-8e71-38adeef79928})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ConversationalAgentSystemStateChangedEventArgs {
    const IID: ::windows::core::GUID = <IConversationalAgentSystemStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSystemStateChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs";
}
::windows::core::interface_hierarchy!(ConversationalAgentSystemStateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ConversationalAgentSystemStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSystemStateChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangedEventArgs(::windows::core::IUnknown);
impl DetectionConfigurationAvailabilityChangedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityChangeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DetectionConfigurationAvailabilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DetectionConfigurationAvailabilityChangedEventArgs {}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs;{5129c9fb-4be8-5f14-af2b-88d62b1b4462})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for DetectionConfigurationAvailabilityChangedEventArgs {
    const IID: ::windows::core::GUID = <IDetectionConfigurationAvailabilityChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DetectionConfigurationAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs";
}
::windows::core::interface_hierarchy!(DetectionConfigurationAvailabilityChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DetectionConfigurationAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for DetectionConfigurationAvailabilityChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityInfo(::windows::core::IUnknown);
impl DetectionConfigurationAvailabilityInfo {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasSystemResourceAccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasSystemResourceAccess)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasPermission(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasPermission)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasLockScreenPermission(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasLockScreenPermission)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnavailableSystemResources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>> {
        let this = &::windows::core::Interface::cast::<IDetectionConfigurationAvailabilityInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnavailableSystemResources)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DetectionConfigurationAvailabilityInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DetectionConfigurationAvailabilityInfo {}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo;{b5affeb0-40f0-5398-b838-91979c2c6208})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for DetectionConfigurationAvailabilityInfo {
    const IID: ::windows::core::GUID = <IDetectionConfigurationAvailabilityInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DetectionConfigurationAvailabilityInfo {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo";
}
::windows::core::interface_hierarchy!(DetectionConfigurationAvailabilityInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DetectionConfigurationAvailabilityInfo {}
unsafe impl ::core::marker::Sync for DetectionConfigurationAvailabilityInfo {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(pub i32);
impl ActivationSignalDetectionConfigurationCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SignalIdNotAvailable: Self = Self(1i32);
    pub const ModelIdNotSupported: Self = Self(2i32);
    pub const InvalidSignalId: Self = Self(3i32);
    pub const InvalidModelId: Self = Self(4i32);
    pub const InvalidDisplayName: Self = Self(5i32);
    pub const ConfigurationAlreadyExists: Self = Self(6i32);
    pub const CreationNotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationCreationStatus {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(pub i32);
impl ActivationSignalDetectionConfigurationRemovalResult {
    pub const Success: Self = Self(0i32);
    pub const NotFound: Self = Self(1i32);
    pub const CurrentlyEnabled: Self = Self(2i32);
    pub const RemovalNotSupported: Self = Self(3i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationRemovalResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationRemovalResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationRemovalResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationRemovalResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationRemovalResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationRemovalResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationRemovalResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(pub i32);
impl ActivationSignalDetectionConfigurationSetModelDataResult {
    pub const Success: Self = Self(0i32);
    pub const EmptyModelData: Self = Self(1i32);
    pub const UnsupportedFormat: Self = Self(2i32);
    pub const ConfigurationCurrentlyEnabled: Self = Self(3i32);
    pub const InvalidData: Self = Self(4i32);
    pub const SetModelDataNotSupported: Self = Self(5i32);
    pub const ConfigurationNotFound: Self = Self(6i32);
    pub const UnknownError: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationSetModelDataResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationSetModelDataResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationSetModelDataResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationSetModelDataResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(pub i32);
impl ActivationSignalDetectionConfigurationStateChangeResult {
    pub const Success: Self = Self(0i32);
    pub const NoModelData: Self = Self(1i32);
    pub const ConfigurationNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationStateChangeResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationStateChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationStateChangeResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationStateChangeResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationStateChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationStateChangeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationStateChangeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationSignalDetectionTrainingDataFormat(pub i32);
impl ActivationSignalDetectionTrainingDataFormat {
    pub const Voice8kHz8BitMono: Self = Self(0i32);
    pub const Voice8kHz16BitMono: Self = Self(1i32);
    pub const Voice16kHz8BitMono: Self = Self(2i32);
    pub const Voice16kHz16BitMono: Self = Self(3i32);
    pub const VoiceOEMDefined: Self = Self(4i32);
    pub const Audio44kHz8BitMono: Self = Self(5i32);
    pub const Audio44kHz16BitMono: Self = Self(6i32);
    pub const Audio48kHz8BitMono: Self = Self(7i32);
    pub const Audio48kHz16BitMono: Self = Self(8i32);
    pub const AudioOEMDefined: Self = Self(9i32);
    pub const OtherOEMDefined: Self = Self(10i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionTrainingDataFormat {}
impl ::core::clone::Clone for ActivationSignalDetectionTrainingDataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionTrainingDataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionTrainingDataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionTrainingDataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionTrainingDataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionTrainingDataFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationSignalDetectorKind(pub i32);
impl ActivationSignalDetectorKind {
    pub const AudioPattern: Self = Self(0i32);
    pub const AudioImpulse: Self = Self(1i32);
    pub const HardwareEvent: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectorKind {}
impl ::core::clone::Clone for ActivationSignalDetectorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationSignalDetectorPowerState(pub i32);
impl ActivationSignalDetectorPowerState {
    pub const HighPower: Self = Self(0i32);
    pub const ConnectedLowPower: Self = Self(1i32);
    pub const DisconnectedLowPower: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectorPowerState {}
impl ::core::clone::Clone for ActivationSignalDetectorPowerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectorPowerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectorPowerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectorPowerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectorPowerState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectorPowerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConversationalAgentActivationKind(pub i32);
impl ConversationalAgentActivationKind {
    pub const VoiceActivationPreview: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
}
impl ::core::marker::Copy for ConversationalAgentActivationKind {}
impl ::core::clone::Clone for ConversationalAgentActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentActivationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentActivationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentActivationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConversationalAgentActivationResult(pub i32);
impl ConversationalAgentActivationResult {
    pub const Success: Self = Self(0i32);
    pub const AgentInactive: Self = Self(1i32);
    pub const ScreenNotAvailable: Self = Self(2i32);
    pub const AgentInterrupted: Self = Self(3i32);
}
impl ::core::marker::Copy for ConversationalAgentActivationResult {}
impl ::core::clone::Clone for ConversationalAgentActivationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentActivationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentActivationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentActivationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentActivationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentActivationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConversationalAgentSessionUpdateResponse(pub i32);
impl ConversationalAgentSessionUpdateResponse {
    pub const Success: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for ConversationalAgentSessionUpdateResponse {}
impl ::core::clone::Clone for ConversationalAgentSessionUpdateResponse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentSessionUpdateResponse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentSessionUpdateResponse {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentSessionUpdateResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSessionUpdateResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSessionUpdateResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConversationalAgentState(pub i32);
impl ConversationalAgentState {
    pub const Inactive: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const Listening: Self = Self(2i32);
    pub const Working: Self = Self(3i32);
    pub const Speaking: Self = Self(4i32);
    pub const ListeningAndSpeaking: Self = Self(5i32);
}
impl ::core::marker::Copy for ConversationalAgentState {}
impl ::core::clone::Clone for ConversationalAgentState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConversationalAgentSystemStateChangeType(pub i32);
impl ConversationalAgentSystemStateChangeType {
    pub const UserAuthentication: Self = Self(0i32);
    pub const ScreenAvailability: Self = Self(1i32);
    pub const IndicatorLightAvailability: Self = Self(2i32);
    pub const VoiceActivationAvailability: Self = Self(3i32);
}
impl ::core::marker::Copy for ConversationalAgentSystemStateChangeType {}
impl ::core::clone::Clone for ConversationalAgentSystemStateChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentSystemStateChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentSystemStateChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentSystemStateChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSystemStateChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSystemStateChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(pub i32);
impl ConversationalAgentVoiceActivationPrerequisiteKind {
    pub const MicrophonePermission: Self = Self(0i32);
    pub const KnownAgents: Self = Self(1i32);
    pub const AgentAllowed: Self = Self(2i32);
    pub const AppCapability: Self = Self(3i32);
    pub const BackgroundTaskRegistration: Self = Self(4i32);
    pub const PolicyPermission: Self = Self(5i32);
}
impl ::core::marker::Copy for ConversationalAgentVoiceActivationPrerequisiteKind {}
impl ::core::clone::Clone for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentVoiceActivationPrerequisiteKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentVoiceActivationPrerequisiteKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentVoiceActivationPrerequisiteKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DetectionConfigurationAvailabilityChangeKind(pub i32);
impl DetectionConfigurationAvailabilityChangeKind {
    pub const SystemResourceAccess: Self = Self(0i32);
    pub const Permission: Self = Self(1i32);
    pub const LockScreenPermission: Self = Self(2i32);
}
impl ::core::marker::Copy for DetectionConfigurationAvailabilityChangeKind {}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DetectionConfigurationAvailabilityChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DetectionConfigurationAvailabilityChangeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityChangeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityChangeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DetectionConfigurationTrainingStatus(pub i32);
impl DetectionConfigurationTrainingStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const VoiceTooQuiet: Self = Self(2i32);
    pub const VoiceTooLoud: Self = Self(3i32);
    pub const VoiceTooFast: Self = Self(4i32);
    pub const VoiceTooSlow: Self = Self(5i32);
    pub const VoiceQualityProblem: Self = Self(6i32);
    pub const TrainingSystemInternalError: Self = Self(7i32);
    pub const TrainingTimedOut: Self = Self(8i32);
    pub const ConfigurationNotFound: Self = Self(9i32);
}
impl ::core::marker::Copy for DetectionConfigurationTrainingStatus {}
impl ::core::clone::Clone for DetectionConfigurationTrainingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DetectionConfigurationTrainingStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DetectionConfigurationTrainingStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DetectionConfigurationTrainingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationTrainingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationTrainingStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SignalDetectorResourceKind(pub i32);
impl SignalDetectorResourceKind {
    pub const ParallelModelSupport: Self = Self(0i32);
    pub const ParallelModelSupportForAgent: Self = Self(1i32);
    pub const ParallelSignalSupport: Self = Self(2i32);
    pub const ParallelSignalSupportForAgent: Self = Self(3i32);
    pub const DisplayOffSupport: Self = Self(4i32);
    pub const PluggedInPower: Self = Self(5i32);
    pub const Detector: Self = Self(6i32);
    pub const SupportedSleepState: Self = Self(7i32);
    pub const SupportedBatterySaverState: Self = Self(8i32);
    pub const ScreenAvailability: Self = Self(9i32);
    pub const InputHardware: Self = Self(10i32);
    pub const AcousticEchoCancellation: Self = Self(11i32);
    pub const ModelIdSupport: Self = Self(12i32);
    pub const DataChannel: Self = Self(13i32);
}
impl ::core::marker::Copy for SignalDetectorResourceKind {}
impl ::core::clone::Clone for SignalDetectorResourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SignalDetectorResourceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SignalDetectorResourceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SignalDetectorResourceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignalDetectorResourceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SignalDetectorResourceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
