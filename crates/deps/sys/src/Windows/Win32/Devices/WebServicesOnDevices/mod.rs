#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct DeviceDiscoveryMechanism(pub i32);
pub const MulticastDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(0i32);
pub const DirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(1i32);
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = DeviceDiscoveryMechanism(2i32);
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
#[cfg(feature = "Win32_Foundation")]
pub type PWSD_SOAP_MESSAGE_HANDLER = unsafe extern "system" fn(thisunknown: ::windows_sys::core::IUnknown, event: *mut WSD_EVENT) -> ::windows_sys::core::HRESULT;
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
#[repr(transparent)]
pub struct WSDEventType(pub i32);
pub const WSDET_NONE: WSDEventType = WSDEventType(0i32);
pub const WSDET_INCOMING_MESSAGE: WSDEventType = WSDEventType(1i32);
pub const WSDET_INCOMING_FAULT: WSDEventType = WSDEventType(2i32);
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = WSDEventType(3i32);
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = WSDEventType(4i32);
#[repr(transparent)]
pub struct WSDUdpMessageType(pub i32);
pub const ONE_WAY: WSDUdpMessageType = WSDUdpMessageType(0i32);
pub const TWO_WAY: WSDUdpMessageType = WSDUdpMessageType(1i32);
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
#[repr(transparent)]
pub struct WSDXML_OP(pub i32);
pub const OpNone: WSDXML_OP = WSDXML_OP(0i32);
pub const OpEndOfTable: WSDXML_OP = WSDXML_OP(1i32);
pub const OpBeginElement_: WSDXML_OP = WSDXML_OP(2i32);
pub const OpBeginAnyElement: WSDXML_OP = WSDXML_OP(3i32);
pub const OpEndElement: WSDXML_OP = WSDXML_OP(4i32);
pub const OpElement_: WSDXML_OP = WSDXML_OP(5i32);
pub const OpAnyElement: WSDXML_OP = WSDXML_OP(6i32);
pub const OpAnyElements: WSDXML_OP = WSDXML_OP(7i32);
pub const OpAnyText: WSDXML_OP = WSDXML_OP(8i32);
pub const OpAttribute_: WSDXML_OP = WSDXML_OP(9i32);
pub const OpBeginChoice: WSDXML_OP = WSDXML_OP(10i32);
pub const OpEndChoice: WSDXML_OP = WSDXML_OP(11i32);
pub const OpBeginSequence: WSDXML_OP = WSDXML_OP(12i32);
pub const OpEndSequence: WSDXML_OP = WSDXML_OP(13i32);
pub const OpBeginAll: WSDXML_OP = WSDXML_OP(14i32);
pub const OpEndAll: WSDXML_OP = WSDXML_OP(15i32);
pub const OpAnything: WSDXML_OP = WSDXML_OP(16i32);
pub const OpAnyNumber: WSDXML_OP = WSDXML_OP(17i32);
pub const OpOneOrMore: WSDXML_OP = WSDXML_OP(18i32);
pub const OpOptional: WSDXML_OP = WSDXML_OP(19i32);
pub const OpFormatBool_: WSDXML_OP = WSDXML_OP(20i32);
pub const OpFormatInt8_: WSDXML_OP = WSDXML_OP(21i32);
pub const OpFormatInt16_: WSDXML_OP = WSDXML_OP(22i32);
pub const OpFormatInt32_: WSDXML_OP = WSDXML_OP(23i32);
pub const OpFormatInt64_: WSDXML_OP = WSDXML_OP(24i32);
pub const OpFormatUInt8_: WSDXML_OP = WSDXML_OP(25i32);
pub const OpFormatUInt16_: WSDXML_OP = WSDXML_OP(26i32);
pub const OpFormatUInt32_: WSDXML_OP = WSDXML_OP(27i32);
pub const OpFormatUInt64_: WSDXML_OP = WSDXML_OP(28i32);
pub const OpFormatUnicodeString_: WSDXML_OP = WSDXML_OP(29i32);
pub const OpFormatDom_: WSDXML_OP = WSDXML_OP(30i32);
pub const OpFormatStruct_: WSDXML_OP = WSDXML_OP(31i32);
pub const OpFormatUri_: WSDXML_OP = WSDXML_OP(32i32);
pub const OpFormatUuidUri_: WSDXML_OP = WSDXML_OP(33i32);
pub const OpFormatName_: WSDXML_OP = WSDXML_OP(34i32);
pub const OpFormatListInsertTail_: WSDXML_OP = WSDXML_OP(35i32);
pub const OpFormatType_: WSDXML_OP = WSDXML_OP(36i32);
pub const OpFormatDynamicType_: WSDXML_OP = WSDXML_OP(37i32);
pub const OpFormatLookupType_: WSDXML_OP = WSDXML_OP(38i32);
pub const OpFormatDuration_: WSDXML_OP = WSDXML_OP(39i32);
pub const OpFormatDateTime_: WSDXML_OP = WSDXML_OP(40i32);
pub const OpFormatFloat_: WSDXML_OP = WSDXML_OP(41i32);
pub const OpFormatDouble_: WSDXML_OP = WSDXML_OP(42i32);
pub const OpProcess_: WSDXML_OP = WSDXML_OP(43i32);
pub const OpQualifiedAttribute_: WSDXML_OP = WSDXML_OP(44i32);
pub const OpFormatXMLDeclaration_: WSDXML_OP = WSDXML_OP(45i32);
pub const OpFormatMax: WSDXML_OP = WSDXML_OP(46i32);
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
#[repr(transparent)]
pub struct WSD_CONFIG_PARAM_TYPE(pub i32);
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(1i32);
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(2i32);
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(3i32);
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(4i32);
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(5i32);
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(6i32);
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(7i32);
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(8i32);
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(9i32);
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(10i32);
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(11i32);
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(12i32);
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = WSD_CONFIG_PARAM_TYPE(13i32);
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
#[repr(transparent)]
pub struct WSD_PROTOCOL_TYPE(pub i32);
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(0i32);
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(1i32);
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(2i32);
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(4i32);
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = WSD_PROTOCOL_TYPE(255i32);
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
#[cfg(feature = "Win32_Foundation")]
pub type WSD_STUB_FUNCTION = unsafe extern "system" fn(server: ::windows_sys::core::IUnknown, session: IWSDServiceMessaging, event: *mut WSD_EVENT) -> ::windows_sys::core::HRESULT;
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
