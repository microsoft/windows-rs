#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IXboxLiveDeviceAddress();
    fn IXboxLiveDeviceAddressStatics();
    fn IXboxLiveEndpointPair();
    fn IXboxLiveEndpointPairCreationResult();
    fn IXboxLiveEndpointPairStateChangedEventArgs();
    fn IXboxLiveEndpointPairStatics();
    fn IXboxLiveEndpointPairTemplate();
    fn IXboxLiveEndpointPairTemplateStatics();
    fn IXboxLiveInboundEndpointPairCreatedEventArgs();
    fn IXboxLiveQualityOfServiceMeasurement();
    fn IXboxLiveQualityOfServiceMeasurementStatics();
    fn IXboxLiveQualityOfServiceMetricResult();
    fn IXboxLiveQualityOfServicePrivatePayloadResult();
    fn XboxLiveDeviceAddress();
    fn XboxLiveEndpointPair();
    fn XboxLiveEndpointPairCreationBehaviors();
    fn XboxLiveEndpointPairCreationResult();
    fn XboxLiveEndpointPairCreationStatus();
    fn XboxLiveEndpointPairState();
    fn XboxLiveEndpointPairStateChangedEventArgs();
    fn XboxLiveEndpointPairTemplate();
    fn XboxLiveInboundEndpointPairCreatedEventArgs();
    fn XboxLiveNetworkAccessKind();
    fn XboxLiveQualityOfServiceMeasurement();
    fn XboxLiveQualityOfServiceMeasurementStatus();
    fn XboxLiveQualityOfServiceMetric();
    fn XboxLiveQualityOfServiceMetricResult();
    fn XboxLiveQualityOfServicePrivatePayloadResult();
    fn XboxLiveSecureSocketsContract();
    fn XboxLiveSocketKind();
}
