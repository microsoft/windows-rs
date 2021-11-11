#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn WSDAllocateLinkedMemory();
    fn WSDAttachLinkedMemory();
    fn WSDCreateDeviceHost();
    fn WSDCreateDeviceHost2();
    fn WSDCreateDeviceHostAdvanced();
    fn WSDCreateDeviceProxy();
    fn WSDCreateDeviceProxy2();
    fn WSDCreateDeviceProxyAdvanced();
    fn WSDCreateDiscoveryProvider();
    fn WSDCreateDiscoveryProvider2();
    fn WSDCreateDiscoveryPublisher();
    fn WSDCreateDiscoveryPublisher2();
    fn WSDCreateHttpAddress();
    fn WSDCreateHttpMessageParameters();
    fn WSDCreateOutboundAttachment();
    fn WSDCreateUdpAddress();
    fn WSDCreateUdpMessageParameters();
    fn WSDDetachLinkedMemory();
    fn WSDFreeLinkedMemory();
    fn WSDGenerateFault();
    fn WSDGenerateFaultEx();
    fn WSDGetConfigurationOption();
    fn WSDSetConfigurationOption();
    fn WSDUriDecode();
    fn WSDUriEncode();
    fn WSDXMLAddChild();
    fn WSDXMLAddSibling();
    fn WSDXMLBuildAnyForSingleElement();
    fn WSDXMLCleanupElement();
    fn WSDXMLCreateContext();
    fn WSDXMLGetNameFromBuiltinNamespace();
    fn WSDXMLGetValueFromAny();
}
