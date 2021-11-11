#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHost(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, ppdevicehost: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHost2(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdevicehost: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHostAdvanced(pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, pphostaddresses: *const ::windows::runtime::RawPtr, dwhostaddresscount: u32, ppdevicehost: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxy(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, ppdeviceproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxy2(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdeviceproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxyAdvanced(pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: ::windows::runtime::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, ppdeviceproxy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryProvider(pcontext: ::windows::runtime::RawPtr, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryProvider2(pcontext: ::windows::runtime::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryPublisher(pcontext: ::windows::runtime::RawPtr, pppublisher: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryPublisher2(pcontext: ::windows::runtime::RawPtr, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, pppublisher: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateHttpAddress(ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateHttpMessageParameters(pptxparams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateOutboundAttachment(ppattachment: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateUdpAddress(ppaddress: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateUdpMessageParameters(pptxparams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDGenerateFault(pszcode: super::super::Foundation::PWSTR, pszsubcode: super::super::Foundation::PWSTR, pszreason: super::super::Foundation::PWSTR, pszdetail: super::super::Foundation::PWSTR, pcontext: ::windows::runtime::RawPtr, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDGenerateFaultEx(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: super::super::Foundation::PWSTR, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDUriDecode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDUriEncode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLBuildAnyForSingleElement(pelementname: *mut WSDXML_NAME, psztext: super::super::Foundation::PWSTR, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDXMLCreateContext(ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLGetNameFromBuiltinNamespace(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLGetValueFromAny(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
}
