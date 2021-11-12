#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IXboxLiveDeviceAddress(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveDeviceAddressStatics(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveEndpointPair(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveEndpointPairCreationResult(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveEndpointPairStateChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveEndpointPairStatics(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveEndpointPairTemplate(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveEndpointPairTemplateStatics(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveQualityOfServiceMeasurement(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveQualityOfServiceMeasurementStatics(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveQualityOfServiceMetricResult(pub *mut ::core::ffi::c_void);
pub struct IXboxLiveQualityOfServicePrivatePayloadResult(pub *mut ::core::ffi::c_void);
pub struct XboxLiveDeviceAddress(i32);
pub struct XboxLiveEndpointPair(i32);
pub struct XboxLiveEndpointPairCreationBehaviors(i32);
pub struct XboxLiveEndpointPairCreationResult(i32);
pub struct XboxLiveEndpointPairCreationStatus(i32);
pub struct XboxLiveEndpointPairState(i32);
pub struct XboxLiveEndpointPairStateChangedEventArgs(i32);
pub struct XboxLiveEndpointPairTemplate(i32);
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(i32);
pub struct XboxLiveNetworkAccessKind(i32);
pub struct XboxLiveQualityOfServiceMeasurement(i32);
pub struct XboxLiveQualityOfServiceMeasurementStatus(i32);
pub struct XboxLiveQualityOfServiceMetric(i32);
pub struct XboxLiveQualityOfServiceMetricResult(i32);
pub struct XboxLiveQualityOfServicePrivatePayloadResult(i32);
pub struct XboxLiveSecureSocketsContract(i32);
pub struct XboxLiveSocketKind(i32);
