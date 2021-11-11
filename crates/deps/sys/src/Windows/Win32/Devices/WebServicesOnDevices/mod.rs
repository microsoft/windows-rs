#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDAllocateLinkedMemory();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDAttachLinkedMemory();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHost();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHost2();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHostAdvanced();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxy();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxy2();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxyAdvanced();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryProvider();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryProvider2();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryPublisher();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryPublisher2();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateHttpAddress();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateHttpMessageParameters();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateOutboundAttachment();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateUdpAddress();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateUdpMessageParameters();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDDetachLinkedMemory();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDFreeLinkedMemory();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDGenerateFault();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDGenerateFaultEx();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDGetConfigurationOption();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDSetConfigurationOption();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDUriDecode();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDUriEncode();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLAddChild();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLAddSibling();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLBuildAnyForSingleElement();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLCleanupElement();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDXMLCreateContext();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLGetNameFromBuiltinNamespace();
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLGetValueFromAny();
}
