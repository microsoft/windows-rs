#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IXboxLiveDeviceAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveDeviceAddressStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveEndpointPair(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveEndpointPairCreationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveEndpointPairStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveEndpointPairTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveEndpointPairTemplateStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMeasurement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMetricResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveDeviceAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveEndpointPair(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationBehaviors(pub u32);
impl XboxLiveEndpointPairCreationBehaviors {
    pub const None: XboxLiveEndpointPairCreationBehaviors = XboxLiveEndpointPairCreationBehaviors(0u32);
    pub const ReevaluatePath: XboxLiveEndpointPairCreationBehaviors = XboxLiveEndpointPairCreationBehaviors(1u32);
}
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationStatus(pub i32);
impl XboxLiveEndpointPairCreationStatus {
    pub const Succeeded: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(0i32);
    pub const NoLocalNetworks: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(1i32);
    pub const NoCompatibleNetworkPaths: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(2i32);
    pub const LocalSystemNotAuthorized: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(3i32);
    pub const Canceled: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(4i32);
    pub const TimedOut: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(5i32);
    pub const RemoteSystemNotAuthorized: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(6i32);
    pub const RefusedDueToConfiguration: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(7i32);
    pub const UnexpectedInternalError: XboxLiveEndpointPairCreationStatus = XboxLiveEndpointPairCreationStatus(8i32);
}
#[repr(transparent)]
pub struct XboxLiveEndpointPairState(pub i32);
impl XboxLiveEndpointPairState {
    pub const Invalid: XboxLiveEndpointPairState = XboxLiveEndpointPairState(0i32);
    pub const CreatingOutbound: XboxLiveEndpointPairState = XboxLiveEndpointPairState(1i32);
    pub const CreatingInbound: XboxLiveEndpointPairState = XboxLiveEndpointPairState(2i32);
    pub const Ready: XboxLiveEndpointPairState = XboxLiveEndpointPairState(3i32);
    pub const DeletingLocally: XboxLiveEndpointPairState = XboxLiveEndpointPairState(4i32);
    pub const RemoteEndpointTerminating: XboxLiveEndpointPairState = XboxLiveEndpointPairState(5i32);
    pub const Deleted: XboxLiveEndpointPairState = XboxLiveEndpointPairState(6i32);
}
#[repr(transparent)]
pub struct XboxLiveEndpointPairStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveEndpointPairTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveNetworkAccessKind(pub i32);
impl XboxLiveNetworkAccessKind {
    pub const Open: XboxLiveNetworkAccessKind = XboxLiveNetworkAccessKind(0i32);
    pub const Moderate: XboxLiveNetworkAccessKind = XboxLiveNetworkAccessKind(1i32);
    pub const Strict: XboxLiveNetworkAccessKind = XboxLiveNetworkAccessKind(2i32);
}
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurementStatus(pub i32);
impl XboxLiveQualityOfServiceMeasurementStatus {
    pub const NotStarted: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(0i32);
    pub const InProgress: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(1i32);
    pub const InProgressWithProvisionalResults: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(2i32);
    pub const Succeeded: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(3i32);
    pub const NoLocalNetworks: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(4i32);
    pub const NoCompatibleNetworkPaths: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(5i32);
    pub const LocalSystemNotAuthorized: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(6i32);
    pub const Canceled: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(7i32);
    pub const TimedOut: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(8i32);
    pub const RemoteSystemNotAuthorized: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(9i32);
    pub const RefusedDueToConfiguration: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(10i32);
    pub const UnexpectedInternalError: XboxLiveQualityOfServiceMeasurementStatus = XboxLiveQualityOfServiceMeasurementStatus(11i32);
}
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetric(pub i32);
impl XboxLiveQualityOfServiceMetric {
    pub const AverageLatencyInMilliseconds: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(0i32);
    pub const MinLatencyInMilliseconds: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(1i32);
    pub const MaxLatencyInMilliseconds: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(2i32);
    pub const AverageOutboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(3i32);
    pub const MinOutboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(4i32);
    pub const MaxOutboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(5i32);
    pub const AverageInboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(6i32);
    pub const MinInboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(7i32);
    pub const MaxInboundBitsPerSecond: XboxLiveQualityOfServiceMetric = XboxLiveQualityOfServiceMetric(8i32);
}
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetricResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveQualityOfServicePrivatePayloadResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct XboxLiveSecureSocketsContract(i32);
#[repr(transparent)]
pub struct XboxLiveSocketKind(pub i32);
impl XboxLiveSocketKind {
    pub const None: XboxLiveSocketKind = XboxLiveSocketKind(0i32);
    pub const Datagram: XboxLiveSocketKind = XboxLiveSocketKind(1i32);
    pub const Stream: XboxLiveSocketKind = XboxLiveSocketKind(2i32);
}
