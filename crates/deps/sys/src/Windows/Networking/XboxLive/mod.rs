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
    pub const None: Self = Self(0u32);
    pub const ReevaluatePath: Self = Self(1u32);
}
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct XboxLiveEndpointPairStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveEndpointPairTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveNetworkAccessKind(pub i32);
impl XboxLiveNetworkAccessKind {
    pub const Open: Self = Self(0i32);
    pub const Moderate: Self = Self(1i32);
    pub const Strict: Self = Self(2i32);
}
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetricResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveQualityOfServicePrivatePayloadResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct XboxLiveSecureSocketsContract(i32);
#[repr(transparent)]
pub struct XboxLiveSocketKind(pub i32);
impl XboxLiveSocketKind {
    pub const None: Self = Self(0i32);
    pub const Datagram: Self = Self(1i32);
    pub const Stream: Self = Self(2i32);
}
