#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IXboxLiveDeviceAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveDeviceAddress {}
impl ::core::clone::Clone for IXboxLiveDeviceAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveDeviceAddressStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveDeviceAddressStatics {}
impl ::core::clone::Clone for IXboxLiveDeviceAddressStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveEndpointPair(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveEndpointPair {}
impl ::core::clone::Clone for IXboxLiveEndpointPair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveEndpointPairCreationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveEndpointPairCreationResult {}
impl ::core::clone::Clone for IXboxLiveEndpointPairCreationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveEndpointPairStateChangedEventArgs {}
impl ::core::clone::Clone for IXboxLiveEndpointPairStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveEndpointPairStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveEndpointPairStatics {}
impl ::core::clone::Clone for IXboxLiveEndpointPairStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveEndpointPairTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveEndpointPairTemplate {}
impl ::core::clone::Clone for IXboxLiveEndpointPairTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveEndpointPairTemplateStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveEndpointPairTemplateStatics {}
impl ::core::clone::Clone for IXboxLiveEndpointPairTemplateStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveInboundEndpointPairCreatedEventArgs {}
impl ::core::clone::Clone for IXboxLiveInboundEndpointPairCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMeasurement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveQualityOfServiceMeasurement {}
impl ::core::clone::Clone for IXboxLiveQualityOfServiceMeasurement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveQualityOfServiceMeasurementStatics {}
impl ::core::clone::Clone for IXboxLiveQualityOfServiceMeasurementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveQualityOfServiceMetricResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveQualityOfServiceMetricResult {}
impl ::core::clone::Clone for IXboxLiveQualityOfServiceMetricResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXboxLiveQualityOfServicePrivatePayloadResult {}
impl ::core::clone::Clone for IXboxLiveQualityOfServicePrivatePayloadResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XboxLiveDeviceAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveDeviceAddress {}
impl ::core::clone::Clone for XboxLiveDeviceAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XboxLiveEndpointPair(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveEndpointPair {}
impl ::core::clone::Clone for XboxLiveEndpointPair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveEndpointPairCreationResult {}
impl ::core::clone::Clone for XboxLiveEndpointPairCreationResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for XboxLiveEndpointPairCreationStatus {}
impl ::core::clone::Clone for XboxLiveEndpointPairCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for XboxLiveEndpointPairState {}
impl ::core::clone::Clone for XboxLiveEndpointPairState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XboxLiveEndpointPairStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveEndpointPairStateChangedEventArgs {}
impl ::core::clone::Clone for XboxLiveEndpointPairStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XboxLiveEndpointPairTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveEndpointPairTemplate {}
impl ::core::clone::Clone for XboxLiveEndpointPairTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveInboundEndpointPairCreatedEventArgs {}
impl ::core::clone::Clone for XboxLiveInboundEndpointPairCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveQualityOfServiceMeasurement {}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMeasurement {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for XboxLiveQualityOfServiceMeasurementStatus {}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMeasurementStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for XboxLiveQualityOfServiceMetric {}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMetric {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetricResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveQualityOfServiceMetricResult {}
impl ::core::clone::Clone for XboxLiveQualityOfServiceMetricResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XboxLiveQualityOfServicePrivatePayloadResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XboxLiveQualityOfServicePrivatePayloadResult {}
impl ::core::clone::Clone for XboxLiveQualityOfServicePrivatePayloadResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
