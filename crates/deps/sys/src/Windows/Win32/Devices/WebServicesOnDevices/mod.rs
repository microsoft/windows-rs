#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHost(pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, ppdevicehost: *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHost2(pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdevicehost: *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHostAdvanced(pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, pphostaddresses: *const IWSDAddress, dwhostaddresscount: u32, ppdevicehost: *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxy(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, ppdeviceproxy: *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxy2(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdeviceproxy: *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxyAdvanced(pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: IWSDAddress, pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, ppdeviceproxy: *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryProvider(pcontext: IWSDXMLContext, ppprovider: *mut IWSDiscoveryProvider) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryProvider2(pcontext: IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppprovider: *mut IWSDiscoveryProvider) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryPublisher(pcontext: IWSDXMLContext, pppublisher: *mut IWSDiscoveryPublisher) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateDiscoveryPublisher2(pcontext: IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, pppublisher: *mut IWSDiscoveryPublisher) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateHttpAddress(ppaddress: *mut IWSDHttpAddress) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateHttpMessageParameters(pptxparams: *mut IWSDHttpMessageParameters) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateOutboundAttachment(ppattachment: *mut IWSDOutboundAttachment) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateUdpAddress(ppaddress: *mut IWSDUdpAddress) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDCreateUdpMessageParameters(pptxparams: *mut IWSDUdpMessageParameters) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDGenerateFault(pszcode: super::super::Foundation::PWSTR, pszsubcode: super::super::Foundation::PWSTR, pszreason: super::super::Foundation::PWSTR, pszdetail: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDGenerateFaultEx(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: super::super::Foundation::PWSTR, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDUriDecode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDUriEncode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLBuildAnyForSingleElement(pelementname: *mut WSDXML_NAME, psztext: super::super::Foundation::PWSTR, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`*"]
    pub fn WSDXMLCreateContext(ppcontext: *mut IWSDXMLContext) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLGetNameFromBuiltinNamespace(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_WebServicesOnDevices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLGetValueFromAny(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
