#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveDeviceAddressImpl: Sized {
    fn SnapshotChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveDeviceAddress, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSnapshotChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetSnapshotAsBase64(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSnapshotAsBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn GetSnapshotAsBytes(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType], byteswritten: &mut u32) -> ::windows::core::Result<()>;
    fn Compare(&self, otherdeviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<i32>;
    fn IsValid(&self) -> ::windows::core::Result<bool>;
    fn IsLocal(&self) -> ::windows::core::Result<bool>;
    fn NetworkAccessKind(&self) -> ::windows::core::Result<XboxLiveNetworkAccessKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveDeviceAddressStaticsImpl: Sized {
    fn CreateFromSnapshotBase64(&self, base64: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn CreateFromSnapshotBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn CreateFromSnapshotBytes(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn GetLocal(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn MaxSnapshotBytesSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairImpl: Sized {
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetRemoteSocketAddressBytes(&self, socketaddress: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetLocalSocketAddressBytes(&self, socketaddress: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
    fn Template(&self) -> ::windows::core::Result<XboxLiveEndpointPairTemplate>;
    fn RemoteDeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn RemoteHostName(&self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalHostName(&self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairCreationResultImpl: Sized {
    fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Status(&self) -> ::windows::core::Result<XboxLiveEndpointPairCreationStatus>;
    fn IsExistingPathEvaluation(&self) -> ::windows::core::Result<bool>;
    fn EndpointPair(&self) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairStateChangedEventArgsImpl: Sized {
    fn OldState(&self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
    fn NewState(&self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairStaticsImpl: Sized {
    fn FindEndpointPairBySocketAddressBytes(&self, localsocketaddress: &[<u8 as ::windows::core::DefaultType>::DefaultType], remotesocketaddress: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<XboxLiveEndpointPair>;
    fn FindEndpointPairByHostNamesAndPorts(&self, localhostname: &::core::option::Option<super::HostName>, localport: &::windows::core::HSTRING, remotehostname: &::core::option::Option<super::HostName>, remoteport: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairTemplateImpl: Sized {
    fn InboundEndpointPairCreated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInboundEndpointPairCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateEndpointPairDefaultAsync(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairWithBehaviorsAsync(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairForPortsDefaultAsync(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, initiatorport: &::windows::core::HSTRING, acceptorport: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairForPortsWithBehaviorsAsync(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, initiatorport: &::windows::core::HSTRING, acceptorport: &::windows::core::HSTRING, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SocketKind(&self) -> ::windows::core::Result<XboxLiveSocketKind>;
    fn InitiatorBoundPortRangeLower(&self) -> ::windows::core::Result<u16>;
    fn InitiatorBoundPortRangeUpper(&self) -> ::windows::core::Result<u16>;
    fn AcceptorBoundPortRangeLower(&self) -> ::windows::core::Result<u16>;
    fn AcceptorBoundPortRangeUpper(&self) -> ::windows::core::Result<u16>;
    fn EndpointPairs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPair>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairTemplateStaticsImpl: Sized {
    fn GetTemplateByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveEndpointPairTemplate>;
    fn Templates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveInboundEndpointPairCreatedEventArgsImpl: Sized {
    fn EndpointPair(&self) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveQualityOfServiceMeasurementImpl: Sized {
    fn MeasureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetMetricResultsForDevice(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn GetMetricResultsForMetric(&self, metric: XboxLiveQualityOfServiceMetric) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn GetMetricResult(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, metric: XboxLiveQualityOfServiceMetric) -> ::windows::core::Result<XboxLiveQualityOfServiceMetricResult>;
    fn GetPrivatePayloadResult(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<XboxLiveQualityOfServicePrivatePayloadResult>;
    fn Metrics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>>;
    fn DeviceAddresses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<XboxLiveDeviceAddress>>;
    fn ShouldRequestPrivatePayloads(&self) -> ::windows::core::Result<bool>;
    fn SetShouldRequestPrivatePayloads(&self, value: bool) -> ::windows::core::Result<()>;
    fn TimeoutInMilliseconds(&self) -> ::windows::core::Result<u32>;
    fn SetTimeoutInMilliseconds(&self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfProbesToAttempt(&self) -> ::windows::core::Result<u32>;
    fn SetNumberOfProbesToAttempt(&self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfResultsPending(&self) -> ::windows::core::Result<u32>;
    fn MetricResults(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn PrivatePayloadResults(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveQualityOfServiceMeasurementStaticsImpl: Sized {
    fn PublishPrivatePayloadBytes(&self, payload: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ClearPrivatePayload(&self) -> ::windows::core::Result<()>;
    fn MaxSimultaneousProbeConnections(&self) -> ::windows::core::Result<u32>;
    fn SetMaxSimultaneousProbeConnections(&self, value: u32) -> ::windows::core::Result<()>;
    fn IsSystemOutboundBandwidthConstrained(&self) -> ::windows::core::Result<bool>;
    fn SetIsSystemOutboundBandwidthConstrained(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSystemInboundBandwidthConstrained(&self) -> ::windows::core::Result<bool>;
    fn SetIsSystemInboundBandwidthConstrained(&self, value: bool) -> ::windows::core::Result<()>;
    fn PublishedPrivatePayload(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetPublishedPrivatePayload(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn MaxPrivatePayloadSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveQualityOfServiceMetricResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMeasurementStatus>;
    fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Metric(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMetric>;
    fn Value(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveQualityOfServicePrivatePayloadResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMeasurementStatus>;
    fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
