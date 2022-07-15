#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveDeviceAddress(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveDeviceAddress {
    type Vtable = IXboxLiveDeviceAddress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5bbd279_3c86_4b57_a31a_b9462408fd01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveDeviceAddress_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SnapshotChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SnapshotChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSnapshotChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSnapshotChanged: usize,
    pub GetSnapshotAsBase64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetSnapshotAsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetSnapshotAsBuffer: usize,
    pub GetSnapshotAsBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, otherdeviceaddress: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub NetworkAccessKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveNetworkAccessKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveDeviceAddressStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveDeviceAddressStatics {
    type Vtable = IXboxLiveDeviceAddressStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5954a819_4a79_4931_827c_7f503e963263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveDeviceAddressStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateFromSnapshotBase64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, base64: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromSnapshotBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromSnapshotBuffer: usize,
    pub CreateFromSnapshotBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxSnapshotBytesSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPair(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveEndpointPair {
    type Vtable = IXboxLiveEndpointPair_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e9a839b_813e_44e0_b87f_c87a093475e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPair_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    pub GetRemoteSocketAddressBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows::core::HRESULT,
    pub GetLocalSocketAddressBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteDeviceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairCreationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveEndpointPairCreationResult {
    type Vtable = IXboxLiveEndpointPairCreationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9a8bb95_2aab_4d1e_9794_33ecc0dcf0fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairCreationResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairCreationStatus) -> ::windows::core::HRESULT,
    pub IsExistingPathEvaluation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub EndpointPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveEndpointPairStateChangedEventArgs {
    type Vtable = IXboxLiveEndpointPairStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x592e3b55_de08_44e7_ac3b_b9b9a169583a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub OldState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveEndpointPairStatics {
    type Vtable = IXboxLiveEndpointPairStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64316b30_217a_4243_8ee1_6729281d27db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FindEndpointPairBySocketAddressBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localSocketAddress_array_size: u32, localsocketaddress: *const u8, remoteSocketAddress_array_size: u32, remotesocketaddress: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindEndpointPairByHostNamesAndPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: *mut ::core::ffi::c_void, localport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostname: *mut ::core::ffi::c_void, remoteport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairTemplate(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveEndpointPairTemplate {
    type Vtable = IXboxLiveEndpointPairTemplate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b286ecf_3457_40ce_b9a1_c0cfe0213ea7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairTemplate_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InboundEndpointPairCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InboundEndpointPairCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInboundEndpointPairCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInboundEndpointPairCreated: usize,
    #[cfg(feature = "Foundation")]
    pub CreateEndpointPairDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateEndpointPairDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateEndpointPairWithBehaviorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: *mut ::core::ffi::c_void, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateEndpointPairWithBehaviorsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateEndpointPairForPortsDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: *mut ::core::ffi::c_void, initiatorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateEndpointPairForPortsDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateEndpointPairForPortsWithBehaviorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: *mut ::core::ffi::c_void, initiatorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateEndpointPairForPortsWithBehaviorsAsync: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SocketKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveSocketKind) -> ::windows::core::HRESULT,
    pub InitiatorBoundPortRangeLower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub InitiatorBoundPortRangeUpper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub AcceptorBoundPortRangeLower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub AcceptorBoundPortRangeUpper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EndpointPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EndpointPairs: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveEndpointPairTemplateStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveEndpointPairTemplateStatics {
    type Vtable = IXboxLiveEndpointPairTemplateStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e13137b_737b_4a23_bc64_0870f75655ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveEndpointPairTemplateStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetTemplateByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Templates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Templates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveInboundEndpointPairCreatedEventArgs {
    type Vtable = IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc183b62_22ba_48d2_80de_c23968bd198b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub EndpointPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMeasurement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveQualityOfServiceMeasurement {
    type Vtable = IXboxLiveQualityOfServiceMeasurement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d682bce_a5d6_47e6_a236_cfde5fbdf2ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMeasurement_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub MeasureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MeasureAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMetricResultsForDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMetricResultsForDevice: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMetricResultsForMetric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metric: XboxLiveQualityOfServiceMetric, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMetricResultsForMetric: usize,
    pub GetMetricResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: *mut ::core::ffi::c_void, metric: XboxLiveQualityOfServiceMetric, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrivatePayloadResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Metrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metrics: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceAddresses: usize,
    pub ShouldRequestPrivatePayloads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShouldRequestPrivatePayloads: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub TimeoutInMilliseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTimeoutInMilliseconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub NumberOfProbesToAttempt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetNumberOfProbesToAttempt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub NumberOfResultsPending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MetricResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MetricResults: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrivatePayloadResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrivatePayloadResults: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveQualityOfServiceMeasurementStatics {
    type Vtable = IXboxLiveQualityOfServiceMeasurementStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e352dca_23cf_440a_b077_5e30857a8234);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PublishPrivatePayloadBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, payload_array_size: u32, payload: *const u8) -> ::windows::core::HRESULT,
    pub ClearPrivatePayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MaxSimultaneousProbeConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxSimultaneousProbeConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub IsSystemOutboundBandwidthConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSystemOutboundBandwidthConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsSystemInboundBandwidthConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSystemInboundBandwidthConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PublishedPrivatePayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishedPrivatePayload: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPublishedPrivatePayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPublishedPrivatePayload: usize,
    pub MaxPrivatePayloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMetricResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveQualityOfServiceMetricResult {
    type Vtable = IXboxLiveQualityOfServiceMetricResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaeec53d1_3561_4782_b0cf_d3ae29d9fa87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServiceMetricResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows::core::HRESULT,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Metric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMetric) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IXboxLiveQualityOfServicePrivatePayloadResult {
    type Vtable = IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a6302ae_6f38_41c0_9fcc_ea6cb978cafc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows::core::HRESULT,
    pub DeviceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveDeviceAddress(::windows::core::IUnknown);
impl XboxLiveDeviceAddress {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SnapshotChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<XboxLiveDeviceAddress, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SnapshotChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSnapshotChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSnapshotChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetSnapshotAsBase64(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetSnapshotAsBase64)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetSnapshotAsBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetSnapshotAsBuffer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn GetSnapshotAsBytes(&self, buffer: &mut [u8], byteswritten: &mut u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetSnapshotAsBytes)(::windows::core::Interface::as_raw(this), buffer.len() as u32, buffer.as_mut_ptr(), byteswritten).ok() }
    }
    pub fn Compare<'a, P0>(&self, otherdeviceaddress: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XboxLiveDeviceAddress>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Compare)(::windows::core::Interface::as_raw(this), otherdeviceaddress.into().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn IsValid(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsValid)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsLocal(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsLocal)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn NetworkAccessKind(&self) -> ::windows::core::Result<XboxLiveNetworkAccessKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NetworkAccessKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveNetworkAccessKind>(result__)
        }
    }
    pub fn CreateFromSnapshotBase64(base64: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromSnapshotBase64)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(base64), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromSnapshotBuffer<'a, P0, E0>(buffer: P0) -> ::windows::core::Result<XboxLiveDeviceAddress>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromSnapshotBuffer)(::windows::core::Interface::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    pub fn CreateFromSnapshotBytes(buffer: &[u8]) -> ::windows::core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromSnapshotBytes)(::windows::core::Interface::as_raw(this), buffer.len() as u32, buffer.as_ptr(), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    pub fn GetLocal() -> ::windows::core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetLocal)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        })
    }
    pub fn MaxSnapshotBytesSize() -> ::windows::core::Result<u32> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxSnapshotBytesSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXboxLiveDeviceAddressStatics<R, F: FnOnce(&IXboxLiveDeviceAddressStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XboxLiveDeviceAddress, IXboxLiveDeviceAddressStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XboxLiveDeviceAddress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveDeviceAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveDeviceAddress {}
impl ::core::fmt::Debug for XboxLiveDeviceAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveDeviceAddress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveDeviceAddress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveDeviceAddress;{f5bbd279-3c86-4b57-a31a-b9462408fd01})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveDeviceAddress {
    type Vtable = IXboxLiveDeviceAddress_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveDeviceAddress as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveDeviceAddress {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveDeviceAddress";
}
impl ::core::convert::From<XboxLiveDeviceAddress> for ::windows::core::IUnknown {
    fn from(value: XboxLiveDeviceAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveDeviceAddress> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveDeviceAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveDeviceAddress> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveDeviceAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveDeviceAddress> for ::windows::core::IInspectable {
    fn from(value: XboxLiveDeviceAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveDeviceAddress> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveDeviceAddress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveDeviceAddress> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveDeviceAddress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveDeviceAddress {}
unsafe impl ::core::marker::Sync for XboxLiveDeviceAddress {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveEndpointPair(::windows::core::IUnknown);
impl XboxLiveEndpointPair {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetRemoteSocketAddressBytes(&self, socketaddress: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetRemoteSocketAddressBytes)(::windows::core::Interface::as_raw(this), socketaddress.len() as u32, socketaddress.as_mut_ptr()).ok() }
    }
    pub fn GetLocalSocketAddressBytes(&self, socketaddress: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetLocalSocketAddressBytes)(::windows::core::Interface::as_raw(this), socketaddress.len() as u32, socketaddress.as_mut_ptr()).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
    pub fn Template(&self) -> ::windows::core::Result<XboxLiveEndpointPairTemplate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Template)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairTemplate>(result__)
        }
    }
    pub fn RemoteDeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoteDeviceAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    pub fn RemoteHostName(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoteHostName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HostName>(result__)
        }
    }
    pub fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemotePort)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LocalHostName(&self) -> ::windows::core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalHostName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::HostName>(result__)
        }
    }
    pub fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalPort)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FindEndpointPairBySocketAddressBytes(localsocketaddress: &[u8], remotesocketaddress: &[u8]) -> ::windows::core::Result<XboxLiveEndpointPair> {
        Self::IXboxLiveEndpointPairStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindEndpointPairBySocketAddressBytes)(::windows::core::Interface::as_raw(this), localsocketaddress.len() as u32, localsocketaddress.as_ptr(), remotesocketaddress.len() as u32, remotesocketaddress.as_ptr(), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPair>(result__)
        })
    }
    pub fn FindEndpointPairByHostNamesAndPorts<'a, P0, P1>(localhostname: P0, localport: &::windows::core::HSTRING, remotehostname: P1, remoteport: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveEndpointPair>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::HostName>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::HostName>>,
    {
        Self::IXboxLiveEndpointPairStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindEndpointPairByHostNamesAndPorts)(::windows::core::Interface::as_raw(this), localhostname.into().abi(), ::core::mem::transmute_copy(localport), remotehostname.into().abi(), ::core::mem::transmute_copy(remoteport), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPair>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXboxLiveEndpointPairStatics<R, F: FnOnce(&IXboxLiveEndpointPairStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XboxLiveEndpointPair, IXboxLiveEndpointPairStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XboxLiveEndpointPair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveEndpointPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveEndpointPair {}
impl ::core::fmt::Debug for XboxLiveEndpointPair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPair").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveEndpointPair {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPair;{1e9a839b-813e-44e0-b87f-c87a093475e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveEndpointPair {
    type Vtable = IXboxLiveEndpointPair_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveEndpointPair as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveEndpointPair {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPair";
}
impl ::core::convert::From<XboxLiveEndpointPair> for ::windows::core::IUnknown {
    fn from(value: XboxLiveEndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPair> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveEndpointPair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveEndpointPair> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveEndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveEndpointPair> for ::windows::core::IInspectable {
    fn from(value: XboxLiveEndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPair> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveEndpointPair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveEndpointPair> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveEndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPair {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPair {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XboxLiveEndpointPairCreationBehaviors(pub u32);
impl XboxLiveEndpointPairCreationBehaviors {
    pub const None: Self = Self(0u32);
    pub const ReevaluatePath: Self = Self(1u32);
}
impl ::core::marker::Copy for XboxLiveEndpointPairCreationBehaviors {}
impl ::core::clone::Clone for XboxLiveEndpointPairCreationBehaviors {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XboxLiveEndpointPairCreationBehaviors {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XboxLiveEndpointPairCreationBehaviors {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveEndpointPairCreationBehaviors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairCreationBehaviors").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XboxLiveEndpointPairCreationBehaviors {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XboxLiveEndpointPairCreationBehaviors {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveEndpointPairCreationBehaviors {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationBehaviors;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationResult(::windows::core::IUnknown);
impl XboxLiveEndpointPairCreationResult {
    pub fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<XboxLiveEndpointPairCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairCreationStatus>(result__)
        }
    }
    pub fn IsExistingPathEvaluation(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsExistingPathEvaluation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn EndpointPair(&self) -> ::windows::core::Result<XboxLiveEndpointPair> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EndpointPair)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPair>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveEndpointPairCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveEndpointPairCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveEndpointPairCreationResult {}
impl ::core::fmt::Debug for XboxLiveEndpointPairCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveEndpointPairCreationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult;{d9a8bb95-2aab-4d1e-9794-33ecc0dcf0fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveEndpointPairCreationResult {
    type Vtable = IXboxLiveEndpointPairCreationResult_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveEndpointPairCreationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveEndpointPairCreationResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult";
}
impl ::core::convert::From<XboxLiveEndpointPairCreationResult> for ::windows::core::IUnknown {
    fn from(value: XboxLiveEndpointPairCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairCreationResult> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveEndpointPairCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairCreationResult> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveEndpointPairCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveEndpointPairCreationResult> for ::windows::core::IInspectable {
    fn from(value: XboxLiveEndpointPairCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairCreationResult> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveEndpointPairCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairCreationResult> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveEndpointPairCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairCreationResult {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairCreationResult {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XboxLiveEndpointPairCreationStatus(pub i32);
impl XboxLiveEndpointPairCreationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoLocalNetworks: Self = Self(1i32);
    pub const NoCompatibleNetworkPaths: Self = Self(2i32);
    pub const LocalSystemNotAuthorized: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const TimedOut: Self = Self(5i32);
    pub const RemoteSystemNotAuthorized: Self = Self(6i32);
    pub const RefusedDueToConfiguration: Self = Self(7i32);
    pub const UnexpectedInternalError: Self = Self(8i32);
}
impl ::core::marker::Copy for XboxLiveEndpointPairCreationStatus {}
impl ::core::clone::Clone for XboxLiveEndpointPairCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XboxLiveEndpointPairCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XboxLiveEndpointPairCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveEndpointPairCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveEndpointPairCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XboxLiveEndpointPairState(pub i32);
impl XboxLiveEndpointPairState {
    pub const Invalid: Self = Self(0i32);
    pub const CreatingOutbound: Self = Self(1i32);
    pub const CreatingInbound: Self = Self(2i32);
    pub const Ready: Self = Self(3i32);
    pub const DeletingLocally: Self = Self(4i32);
    pub const RemoteEndpointTerminating: Self = Self(5i32);
    pub const Deleted: Self = Self(6i32);
}
impl ::core::marker::Copy for XboxLiveEndpointPairState {}
impl ::core::clone::Clone for XboxLiveEndpointPairState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XboxLiveEndpointPairState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XboxLiveEndpointPairState {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveEndpointPairState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveEndpointPairState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveEndpointPairStateChangedEventArgs(::windows::core::IUnknown);
impl XboxLiveEndpointPairStateChangedEventArgs {
    pub fn OldState(&self) -> ::windows::core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OldState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
    pub fn NewState(&self) -> ::windows::core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NewState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairState>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveEndpointPairStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveEndpointPairStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveEndpointPairStateChangedEventArgs {}
impl ::core::fmt::Debug for XboxLiveEndpointPairStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveEndpointPairStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs;{592e3b55-de08-44e7-ac3b-b9b9a169583a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveEndpointPairStateChangedEventArgs {
    type Vtable = IXboxLiveEndpointPairStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveEndpointPairStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveEndpointPairStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs";
}
impl ::core::convert::From<XboxLiveEndpointPairStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairStateChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveEndpointPairStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairStateChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveEndpointPairStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairStateChangedEventArgs {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveEndpointPairTemplate(::windows::core::IUnknown);
impl XboxLiveEndpointPairTemplate {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InboundEndpointPairCreated<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InboundEndpointPairCreated)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInboundEndpointPairCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveInboundEndpointPairCreated)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateEndpointPairDefaultAsync<'a, P0>(&self, deviceaddress: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XboxLiveDeviceAddress>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateEndpointPairDefaultAsync)(::windows::core::Interface::as_raw(this), deviceaddress.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateEndpointPairWithBehaviorsAsync<'a, P0>(&self, deviceaddress: P0, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XboxLiveDeviceAddress>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateEndpointPairWithBehaviorsAsync)(::windows::core::Interface::as_raw(this), deviceaddress.into().abi(), behaviors, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateEndpointPairForPortsDefaultAsync<'a, P0>(&self, deviceaddress: P0, initiatorport: &::windows::core::HSTRING, acceptorport: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XboxLiveDeviceAddress>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateEndpointPairForPortsDefaultAsync)(::windows::core::Interface::as_raw(this), deviceaddress.into().abi(), ::core::mem::transmute_copy(initiatorport), ::core::mem::transmute_copy(acceptorport), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateEndpointPairForPortsWithBehaviorsAsync<'a, P0>(&self, deviceaddress: P0, initiatorport: &::windows::core::HSTRING, acceptorport: &::windows::core::HSTRING, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XboxLiveDeviceAddress>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateEndpointPairForPortsWithBehaviorsAsync)(::windows::core::Interface::as_raw(this), deviceaddress.into().abi(), ::core::mem::transmute_copy(initiatorport), ::core::mem::transmute_copy(acceptorport), behaviors, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SocketKind(&self) -> ::windows::core::Result<XboxLiveSocketKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SocketKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveSocketKind>(result__)
        }
    }
    pub fn InitiatorBoundPortRangeLower(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InitiatorBoundPortRangeLower)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn InitiatorBoundPortRangeUpper(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InitiatorBoundPortRangeUpper)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn AcceptorBoundPortRangeLower(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AcceptorBoundPortRangeLower)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn AcceptorBoundPortRangeUpper(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AcceptorBoundPortRangeUpper)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EndpointPairs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPair>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EndpointPairs)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPair>>(result__)
        }
    }
    pub fn GetTemplateByName(name: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveEndpointPairTemplate> {
        Self::IXboxLiveEndpointPairTemplateStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetTemplateByName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPairTemplate>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Templates() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>> {
        Self::IXboxLiveEndpointPairTemplateStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Templates)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXboxLiveEndpointPairTemplateStatics<R, F: FnOnce(&IXboxLiveEndpointPairTemplateStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XboxLiveEndpointPairTemplate, IXboxLiveEndpointPairTemplateStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XboxLiveEndpointPairTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveEndpointPairTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveEndpointPairTemplate {}
impl ::core::fmt::Debug for XboxLiveEndpointPairTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairTemplate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveEndpointPairTemplate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate;{6b286ecf-3457-40ce-b9a1-c0cfe0213ea7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveEndpointPairTemplate {
    type Vtable = IXboxLiveEndpointPairTemplate_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveEndpointPairTemplate as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveEndpointPairTemplate {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate";
}
impl ::core::convert::From<XboxLiveEndpointPairTemplate> for ::windows::core::IUnknown {
    fn from(value: XboxLiveEndpointPairTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairTemplate> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveEndpointPairTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairTemplate> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveEndpointPairTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveEndpointPairTemplate> for ::windows::core::IInspectable {
    fn from(value: XboxLiveEndpointPairTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairTemplate> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveEndpointPairTemplate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveEndpointPairTemplate> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveEndpointPairTemplate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveEndpointPairTemplate {}
unsafe impl ::core::marker::Sync for XboxLiveEndpointPairTemplate {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(::windows::core::IUnknown);
impl XboxLiveInboundEndpointPairCreatedEventArgs {
    pub fn EndpointPair(&self) -> ::windows::core::Result<XboxLiveEndpointPair> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EndpointPair)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveEndpointPair>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveInboundEndpointPairCreatedEventArgs {}
impl ::core::fmt::Debug for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveInboundEndpointPairCreatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveInboundEndpointPairCreatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs;{dc183b62-22ba-48d2-80de-c23968bd198b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveInboundEndpointPairCreatedEventArgs {
    type Vtable = IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveInboundEndpointPairCreatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveInboundEndpointPairCreatedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs";
}
impl ::core::convert::From<XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveInboundEndpointPairCreatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveInboundEndpointPairCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveInboundEndpointPairCreatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveInboundEndpointPairCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveInboundEndpointPairCreatedEventArgs {}
unsafe impl ::core::marker::Sync for XboxLiveInboundEndpointPairCreatedEventArgs {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XboxLiveNetworkAccessKind(pub i32);
impl XboxLiveNetworkAccessKind {
    pub const Open: Self = Self(0i32);
    pub const Moderate: Self = Self(1i32);
    pub const Strict: Self = Self(2i32);
}
impl ::core::marker::Copy for XboxLiveNetworkAccessKind {}
impl ::core::clone::Clone for XboxLiveNetworkAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XboxLiveNetworkAccessKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XboxLiveNetworkAccessKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveNetworkAccessKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveNetworkAccessKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveNetworkAccessKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveNetworkAccessKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurement(::windows::core::IUnknown);
impl XboxLiveQualityOfServiceMeasurement {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XboxLiveQualityOfServiceMeasurement, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MeasureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MeasureAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMetricResultsForDevice<'a, P0>(&self, deviceaddress: P0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XboxLiveDeviceAddress>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetMetricResultsForDevice)(::windows::core::Interface::as_raw(this), deviceaddress.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMetricResultsForMetric(&self, metric: XboxLiveQualityOfServiceMetric) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetMetricResultsForMetric)(::windows::core::Interface::as_raw(this), metric, result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    pub fn GetMetricResult<'a, P0>(&self, deviceaddress: P0, metric: XboxLiveQualityOfServiceMetric) -> ::windows::core::Result<XboxLiveQualityOfServiceMetricResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XboxLiveDeviceAddress>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetMetricResult)(::windows::core::Interface::as_raw(this), deviceaddress.into().abi(), metric, result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServiceMetricResult>(result__)
        }
    }
    pub fn GetPrivatePayloadResult<'a, P0>(&self, deviceaddress: P0) -> ::windows::core::Result<XboxLiveQualityOfServicePrivatePayloadResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, XboxLiveDeviceAddress>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetPrivatePayloadResult)(::windows::core::Interface::as_raw(this), deviceaddress.into().abi(), result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServicePrivatePayloadResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metrics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Metrics)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceAddresses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<XboxLiveDeviceAddress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAddresses)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVector<XboxLiveDeviceAddress>>(result__)
        }
    }
    pub fn ShouldRequestPrivatePayloads(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShouldRequestPrivatePayloads)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldRequestPrivatePayloads(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShouldRequestPrivatePayloads)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TimeoutInMilliseconds(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TimeoutInMilliseconds)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetTimeoutInMilliseconds(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTimeoutInMilliseconds)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfProbesToAttempt(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NumberOfProbesToAttempt)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetNumberOfProbesToAttempt(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNumberOfProbesToAttempt)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfResultsPending(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NumberOfResultsPending)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MetricResults(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MetricResults)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrivatePayloadResults(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrivatePayloadResults)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>>(result__)
        }
    }
    pub fn PublishPrivatePayloadBytes(payload: &[u8]) -> ::windows::core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::core::Interface::vtable(this).PublishPrivatePayloadBytes)(::windows::core::Interface::as_raw(this), payload.len() as u32, payload.as_ptr()).ok() })
    }
    pub fn ClearPrivatePayload() -> ::windows::core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ClearPrivatePayload)(::windows::core::Interface::as_raw(this)).ok() })
    }
    pub fn MaxSimultaneousProbeConnections() -> ::windows::core::Result<u32> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxSimultaneousProbeConnections)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    pub fn SetMaxSimultaneousProbeConnections(value: u32) -> ::windows::core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetMaxSimultaneousProbeConnections)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsSystemOutboundBandwidthConstrained() -> ::windows::core::Result<bool> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSystemOutboundBandwidthConstrained)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsSystemOutboundBandwidthConstrained(value: bool) -> ::windows::core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetIsSystemOutboundBandwidthConstrained)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsSystemInboundBandwidthConstrained() -> ::windows::core::Result<bool> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSystemInboundBandwidthConstrained)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsSystemInboundBandwidthConstrained(value: bool) -> ::windows::core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetIsSystemInboundBandwidthConstrained)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishedPrivatePayload() -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PublishedPrivatePayload)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPublishedPrivatePayload<'a, P0, E0>(value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPublishedPrivatePayload)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    pub fn MaxPrivatePayloadSize() -> ::windows::core::Result<u32> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxPrivatePayloadSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IXboxLiveQualityOfServiceMeasurementStatics<R, F: FnOnce(&IXboxLiveQualityOfServiceMeasurementStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<XboxLiveQualityOfServiceMeasurement, IXboxLiveQualityOfServiceMeasurementStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMeasurement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveQualityOfServiceMeasurement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveQualityOfServiceMeasurement {}
impl ::core::fmt::Debug for XboxLiveQualityOfServiceMeasurement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMeasurement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveQualityOfServiceMeasurement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement;{4d682bce-a5d6-47e6-a236-cfde5fbdf2ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveQualityOfServiceMeasurement {
    type Vtable = IXboxLiveQualityOfServiceMeasurement_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveQualityOfServiceMeasurement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveQualityOfServiceMeasurement {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement";
}
impl ::core::convert::From<XboxLiveQualityOfServiceMeasurement> for ::windows::core::IUnknown {
    fn from(value: XboxLiveQualityOfServiceMeasurement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMeasurement> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveQualityOfServiceMeasurement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMeasurement> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveQualityOfServiceMeasurement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveQualityOfServiceMeasurement> for ::windows::core::IInspectable {
    fn from(value: XboxLiveQualityOfServiceMeasurement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMeasurement> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveQualityOfServiceMeasurement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMeasurement> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveQualityOfServiceMeasurement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServiceMeasurement {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServiceMeasurement {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XboxLiveQualityOfServiceMeasurementStatus(pub i32);
impl XboxLiveQualityOfServiceMeasurementStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const InProgressWithProvisionalResults: Self = Self(2i32);
    pub const Succeeded: Self = Self(3i32);
    pub const NoLocalNetworks: Self = Self(4i32);
    pub const NoCompatibleNetworkPaths: Self = Self(5i32);
    pub const LocalSystemNotAuthorized: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const TimedOut: Self = Self(8i32);
    pub const RemoteSystemNotAuthorized: Self = Self(9i32);
    pub const RefusedDueToConfiguration: Self = Self(10i32);
    pub const UnexpectedInternalError: Self = Self(11i32);
}
impl ::core::marker::Copy for XboxLiveQualityOfServiceMeasurementStatus {}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMeasurementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XboxLiveQualityOfServiceMeasurementStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XboxLiveQualityOfServiceMeasurementStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveQualityOfServiceMeasurementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMeasurementStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveQualityOfServiceMeasurementStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurementStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XboxLiveQualityOfServiceMetric(pub i32);
impl XboxLiveQualityOfServiceMetric {
    pub const AverageLatencyInMilliseconds: Self = Self(0i32);
    pub const MinLatencyInMilliseconds: Self = Self(1i32);
    pub const MaxLatencyInMilliseconds: Self = Self(2i32);
    pub const AverageOutboundBitsPerSecond: Self = Self(3i32);
    pub const MinOutboundBitsPerSecond: Self = Self(4i32);
    pub const MaxOutboundBitsPerSecond: Self = Self(5i32);
    pub const AverageInboundBitsPerSecond: Self = Self(6i32);
    pub const MinInboundBitsPerSecond: Self = Self(7i32);
    pub const MaxInboundBitsPerSecond: Self = Self(8i32);
}
impl ::core::marker::Copy for XboxLiveQualityOfServiceMetric {}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMetric {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XboxLiveQualityOfServiceMetric {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XboxLiveQualityOfServiceMetric {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveQualityOfServiceMetric {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMetric").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveQualityOfServiceMetric {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetricResult(::windows::core::IUnknown);
impl XboxLiveQualityOfServiceMetricResult {
    pub fn Status(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMeasurementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServiceMeasurementStatus>(result__)
        }
    }
    pub fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    pub fn Metric(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMetric> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Metric)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServiceMetric>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMetricResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveQualityOfServiceMetricResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveQualityOfServiceMetricResult {}
impl ::core::fmt::Debug for XboxLiveQualityOfServiceMetricResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMetricResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveQualityOfServiceMetricResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult;{aeec53d1-3561-4782-b0cf-d3ae29d9fa87})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveQualityOfServiceMetricResult {
    type Vtable = IXboxLiveQualityOfServiceMetricResult_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveQualityOfServiceMetricResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveQualityOfServiceMetricResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult";
}
impl ::core::convert::From<XboxLiveQualityOfServiceMetricResult> for ::windows::core::IUnknown {
    fn from(value: XboxLiveQualityOfServiceMetricResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMetricResult> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveQualityOfServiceMetricResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMetricResult> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveQualityOfServiceMetricResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveQualityOfServiceMetricResult> for ::windows::core::IInspectable {
    fn from(value: XboxLiveQualityOfServiceMetricResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMetricResult> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveQualityOfServiceMetricResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServiceMetricResult> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveQualityOfServiceMetricResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServiceMetricResult {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServiceMetricResult {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
pub struct XboxLiveQualityOfServicePrivatePayloadResult(::windows::core::IUnknown);
impl XboxLiveQualityOfServicePrivatePayloadResult {
    pub fn Status(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMeasurementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveQualityOfServiceMeasurementStatus>(result__)
        }
    }
    pub fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<XboxLiveDeviceAddress>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for XboxLiveQualityOfServicePrivatePayloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XboxLiveQualityOfServicePrivatePayloadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XboxLiveQualityOfServicePrivatePayloadResult {}
impl ::core::fmt::Debug for XboxLiveQualityOfServicePrivatePayloadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServicePrivatePayloadResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveQualityOfServicePrivatePayloadResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult;{5a6302ae-6f38-41c0-9fcc-ea6cb978cafc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for XboxLiveQualityOfServicePrivatePayloadResult {
    type Vtable = IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl;
    const IID: ::windows::core::GUID = <IXboxLiveQualityOfServicePrivatePayloadResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for XboxLiveQualityOfServicePrivatePayloadResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult";
}
impl ::core::convert::From<XboxLiveQualityOfServicePrivatePayloadResult> for ::windows::core::IUnknown {
    fn from(value: XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServicePrivatePayloadResult> for ::windows::core::IUnknown {
    fn from(value: &XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServicePrivatePayloadResult> for &::windows::core::IUnknown {
    fn from(value: &XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<XboxLiveQualityOfServicePrivatePayloadResult> for ::windows::core::IInspectable {
    fn from(value: XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServicePrivatePayloadResult> for ::windows::core::IInspectable {
    fn from(value: &XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&XboxLiveQualityOfServicePrivatePayloadResult> for &::windows::core::IInspectable {
    fn from(value: &XboxLiveQualityOfServicePrivatePayloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for XboxLiveQualityOfServicePrivatePayloadResult {}
unsafe impl ::core::marker::Sync for XboxLiveQualityOfServicePrivatePayloadResult {}
#[doc = "*Required features: `\"Networking_XboxLive\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XboxLiveSocketKind(pub i32);
impl XboxLiveSocketKind {
    pub const None: Self = Self(0i32);
    pub const Datagram: Self = Self(1i32);
    pub const Stream: Self = Self(2i32);
}
impl ::core::marker::Copy for XboxLiveSocketKind {}
impl ::core::clone::Clone for XboxLiveSocketKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XboxLiveSocketKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XboxLiveSocketKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for XboxLiveSocketKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XboxLiveSocketKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for XboxLiveSocketKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveSocketKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
