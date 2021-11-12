#![allow(non_snake_case, non_camel_case_types)]
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
pub struct XboxLiveEndpointPairCreationBehaviors(i32);
#[repr(transparent)]
pub struct XboxLiveEndpointPairCreationResult(pub *mut ::core::ffi::c_void);
pub struct XboxLiveEndpointPairCreationStatus(i32);
pub struct XboxLiveEndpointPairState(i32);
#[repr(transparent)]
pub struct XboxLiveEndpointPairStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveEndpointPairTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(pub *mut ::core::ffi::c_void);
pub struct XboxLiveNetworkAccessKind(i32);
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMeasurement(pub *mut ::core::ffi::c_void);
pub struct XboxLiveQualityOfServiceMeasurementStatus(i32);
pub struct XboxLiveQualityOfServiceMetric(i32);
#[repr(transparent)]
pub struct XboxLiveQualityOfServiceMetricResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XboxLiveQualityOfServicePrivatePayloadResult(pub *mut ::core::ffi::c_void);
pub struct XboxLiveSecureSocketsContract(i32);
pub struct XboxLiveSocketKind(i32);
