#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn WSDAllocateLinkedMemory(pparent: *mut ::core::ffi::c_void, cbsize: usize) -> *mut ::core::ffi::c_void;
    pub fn WSDAttachLinkedMemory(pparent: *mut ::core::ffi::c_void, pchild: *mut ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHost(pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, ppdevicehost: *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHost2(pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdevicehost: *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceHostAdvanced(pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, pphostaddresses: *const IWSDAddress, dwhostaddresscount: u32, ppdevicehost: *mut IWSDDeviceHost) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxy(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, ppdeviceproxy: *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxy2(pszdeviceid: super::super::Foundation::PWSTR, pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppdeviceproxy: *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDCreateDeviceProxyAdvanced(pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: IWSDAddress, pszlocalid: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, ppdeviceproxy: *mut IWSDDeviceProxy) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateDiscoveryProvider(pcontext: IWSDXMLContext, ppprovider: *mut IWSDiscoveryProvider) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateDiscoveryProvider2(pcontext: IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, ppprovider: *mut IWSDiscoveryProvider) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateDiscoveryPublisher(pcontext: IWSDXMLContext, pppublisher: *mut IWSDiscoveryPublisher) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateDiscoveryPublisher2(pcontext: IWSDXMLContext, pconfigparams: *const WSD_CONFIG_PARAM, dwconfigparamcount: u32, pppublisher: *mut IWSDiscoveryPublisher) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateHttpAddress(ppaddress: *mut IWSDHttpAddress) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateHttpMessageParameters(pptxparams: *mut IWSDHttpMessageParameters) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateOutboundAttachment(ppattachment: *mut IWSDOutboundAttachment) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateUdpAddress(ppaddress: *mut IWSDUdpAddress) -> ::windows_sys::core::HRESULT;
    pub fn WSDCreateUdpMessageParameters(pptxparams: *mut IWSDUdpMessageParameters) -> ::windows_sys::core::HRESULT;
    pub fn WSDDetachLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    pub fn WSDFreeLinkedMemory(pvoid: *mut ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDGenerateFault(pszcode: super::super::Foundation::PWSTR, pszsubcode: super::super::Foundation::PWSTR, pszreason: super::super::Foundation::PWSTR, pszdetail: super::super::Foundation::PWSTR, pcontext: IWSDXMLContext, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDGenerateFaultEx(pcode: *const WSDXML_NAME, psubcode: *const WSDXML_NAME, preasons: *const WSD_LOCALIZED_STRING_LIST, pszdetail: super::super::Foundation::PWSTR, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_sys::core::HRESULT;
    pub fn WSDGetConfigurationOption(dwoption: u32, pvoid: *mut ::core::ffi::c_void, cboutbuffer: u32) -> ::windows_sys::core::HRESULT;
    pub fn WSDSetConfigurationOption(dwoption: u32, pvoid: *const ::core::ffi::c_void, cbinbuffer: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDUriDecode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDUriEncode(source: super::super::Foundation::PWSTR, cchsource: u32, destout: *mut super::super::Foundation::PWSTR, cchdestout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLAddChild(pparent: *mut WSDXML_ELEMENT, pchild: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLAddSibling(pfirst: *mut WSDXML_ELEMENT, psecond: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLBuildAnyForSingleElement(pelementname: *mut WSDXML_NAME, psztext: super::super::Foundation::PWSTR, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLCleanupElement(pany: *mut WSDXML_ELEMENT) -> ::windows_sys::core::HRESULT;
    pub fn WSDXMLCreateContext(ppcontext: *mut IWSDXMLContext) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLGetNameFromBuiltinNamespace(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSDXMLGetValueFromAny(psznamespace: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, pany: *mut WSDXML_ELEMENT, ppszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct DeviceDiscoveryMechanism(i32);
#[repr(transparent)]
pub struct IWSDAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDAsyncCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDAsyncResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDDeviceHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDDeviceHostNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDDeviceProxy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDEndpointProxy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDEventingStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDHttpAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDHttpAuthParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDHttpMessageParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDInboundAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDMessageParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDMetadataExchange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDOutboundAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDSSLClientCertificate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDScopeMatchingRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDServiceMessaging(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDServiceProxy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDServiceProxyEventing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDSignatureProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDTransportAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDUdpAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDUdpMessageParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDXMLContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDiscoveredService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDiscoveryProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDiscoveryProviderNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDiscoveryPublisher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWSDiscoveryPublisherNotify(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PWSD_SOAP_MESSAGE_HANDLER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REQUESTBODY_GetStatus(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REQUESTBODY_Renew(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REQUESTBODY_Subscribe(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REQUESTBODY_Unsubscribe(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RESPONSEBODY_GetMetadata(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RESPONSEBODY_GetStatus(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RESPONSEBODY_Renew(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RESPONSEBODY_Subscribe(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RESPONSEBODY_SubscriptionEnd(i32);
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1u32;
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2u32;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1u32;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2u32;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3u32;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0u32;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2u32;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16u32;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1u32;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8u32;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4u32;
#[repr(C)]
pub struct WSDEventType(i32);
#[repr(C)]
pub struct WSDUdpMessageType(i32);
#[repr(C)]
pub struct WSDUdpRetransmitParams(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_ATTRIBUTE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_ELEMENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_ELEMENT_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_NAME(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_NAMESPACE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_NODE(i32);
#[repr(C)]
pub struct WSDXML_OP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_PREFIX_MAPPING(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_TEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSDXML_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_APP_SEQUENCE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_BYE(i32);
#[repr(C)]
pub struct WSD_CONFIG_ADDRESSES(i32);
#[repr(C)]
pub struct WSD_CONFIG_PARAM(i32);
#[repr(C)]
pub struct WSD_CONFIG_PARAM_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_DATETIME(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_DURATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_ENDPOINT_REFERENCE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_ENDPOINT_REFERENCE_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_EVENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_EVENTING_DELIVERY_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_EVENTING_EXPIRES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_EVENTING_FILTER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_EVENTING_FILTER_ACTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_HANDLER_CONTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_HEADER_RELATESTO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_HELLO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_HOST_METADATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_LOCALIZED_STRING(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_LOCALIZED_STRING_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_METADATA_SECTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_METADATA_SECTION_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_NAME_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_OPERATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_PORT_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_PROBE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_PROBE_MATCH(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_PROBE_MATCHES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_PROBE_MATCH_LIST(i32);
#[repr(C)]
pub struct WSD_PROTOCOL_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_REFERENCE_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_REFERENCE_PROPERTIES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_RELATIONSHIP_METADATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_RESOLVE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_RESOLVE_MATCH(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_RESOLVE_MATCHES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SCOPES(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WSD_SECURITY_CERT_VALIDATION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WSD_SECURITY_CERT_VALIDATION_V1(i32);
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[repr(C)]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SERVICE_METADATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SERVICE_METADATA_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SOAP_FAULT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SOAP_FAULT_CODE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SOAP_FAULT_REASON(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SOAP_FAULT_SUBCODE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SOAP_HEADER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SOAP_MESSAGE(i32);
#[repr(C)]
pub struct WSD_STUB_FUNCTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_THIS_DEVICE_METADATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_THIS_MODEL_METADATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_UNKNOWN_LOOKUP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSD_URI_LIST(i32);
