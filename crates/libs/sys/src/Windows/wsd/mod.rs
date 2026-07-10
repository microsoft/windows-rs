windows_link::link!("wsdapi.dll" "system" fn WSDAllocateLinkedMemory(pparent : *mut core::ffi::c_void, cbsize : usize) -> *mut core::ffi::c_void);
windows_link::link!("wsdapi.dll" "system" fn WSDAttachLinkedMemory(pparent : *mut core::ffi::c_void, pchild : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost2(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHostAdvanced(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pphostaddresses : *const *mut core::ffi::c_void, dwhostaddresscount : u32, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy(pszdeviceid : windows_sys::core::PCWSTR, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy2(pszdeviceid : windows_sys::core::PCWSTR, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxyAdvanced(pszdeviceid : windows_sys::core::PCWSTR, pdeviceaddress : *mut core::ffi::c_void, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider(pcontext : *mut core::ffi::c_void, ppprovider : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider2(pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppprovider : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher(pcontext : *mut core::ffi::c_void, pppublisher : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher2(pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : u32, pppublisher : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateHttpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateHttpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateOutboundAttachment(ppattachment : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateUdpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateUdpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDDetachLinkedMemory(pvoid : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDFreeLinkedMemory(pvoid : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDGenerateFault(pszcode : windows_sys::core::PCWSTR, pszsubcode : windows_sys::core::PCWSTR, pszreason : windows_sys::core::PCWSTR, pszdetail : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppfault : *mut *mut WSD_SOAP_FAULT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDGenerateFaultEx(pcode : *const WSDXML_NAME, psubcode : *const WSDXML_NAME, preasons : *const WSD_LOCALIZED_STRING_LIST, pszdetail : windows_sys::core::PCWSTR, ppfault : *mut *mut WSD_SOAP_FAULT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDGetConfigurationOption(dwoption : u32, pvoid : *mut core::ffi::c_void, cboutbuffer : u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDSetConfigurationOption(dwoption : u32, pvoid : *const core::ffi::c_void, cbinbuffer : u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDUriDecode(source : windows_sys::core::PCWSTR, cchsource : u32, destout : *mut windows_sys::core::PWSTR, cchdestout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDUriEncode(source : windows_sys::core::PCWSTR, cchsource : u32, destout : *mut windows_sys::core::PWSTR, cchdestout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLAddChild(pparent : *mut WSDXML_ELEMENT, pchild : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLAddSibling(pfirst : *mut WSDXML_ELEMENT, psecond : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLBuildAnyForSingleElement(pelementname : *mut WSDXML_NAME, psztext : windows_sys::core::PCWSTR, ppany : *mut *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLCleanupElement(pany : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLCreateContext(ppcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLGetNameFromBuiltinNamespace(psznamespace : windows_sys::core::PCWSTR, pszname : windows_sys::core::PCWSTR, ppname : *mut *mut WSDXML_NAME) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLGetValueFromAny(psznamespace : windows_sys::core::PCWSTR, pszname : windows_sys::core::PCWSTR, pany : *mut WSDXML_ELEMENT, ppszvalue : *mut windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub const DPP_AppSequence: u32 = 524299;
pub const DPP_Bye: u32 = 524289;
pub const DPP_DiscoveryProxy: u32 = 524305;
pub const DPP_Element: u32 = 524307;
pub const DPP_Hello: u32 = 524288;
pub const DPP_Id: u32 = 524309;
pub const DPP_InstanceId: u32 = 524300;
pub const DPP_KeyId: u32 = 524313;
pub const DPP_MatchBy: u32 = 524303;
pub const DPP_MessageNumber: u32 = 524302;
pub const DPP_MetadataVersion: u32 = 524297;
pub const DPP_Probe: u32 = 524290;
pub const DPP_ProbeMatch: u32 = 524292;
pub const DPP_ProbeMatches: u32 = 524291;
pub const DPP_Refs: u32 = 524314;
pub const DPP_Resolve: u32 = 524293;
pub const DPP_ResolveMatch: u32 = 524294;
pub const DPP_ResolveMatches: u32 = 524308;
pub const DPP_Scheme: u32 = 524312;
pub const DPP_Scopes: u32 = 524296;
pub const DPP_Security: u32 = 524310;
pub const DPP_SequenceId: u32 = 524301;
pub const DPP_Sig: u32 = 524311;
pub const DPP_TargetService: u32 = 524306;
pub const DPP_Transport: u32 = 524298;
pub const DPP_Types: u32 = 524295;
pub const DPP_XAddrs: u32 = 524304;
pub type DeviceDiscoveryMechanism = i32;
pub const DirectedDiscovery: DeviceDiscoveryMechanism = 1;
pub const ElementType: i32 = 0;
pub const MulticastDiscovery: DeviceDiscoveryMechanism = 0;
pub const NAMESPACE_ENCODING_ADDRESSING: u32 = 1;
pub const NAMESPACE_ENCODING_DEVPROF: u32 = 5;
pub const NAMESPACE_ENCODING_DISCOVERY: u32 = 2;
pub const NAMESPACE_ENCODING_DISCOVERY11: u32 = 8;
pub const NAMESPACE_ENCODING_ENVELOPE: u32 = 7;
pub const NAMESPACE_ENCODING_EVENTING: u32 = 0;
pub const NAMESPACE_ENCODING_INCLUDE: u32 = 3;
pub const NAMESPACE_ENCODING_MEX: u32 = 6;
pub const NAMESPACE_ENCODING_XML: u32 = 4;
pub const ONE_WAY: WSDUdpMessageType = 0;
pub const OpAnyElement: i32 = 6;
pub const OpAnyElements: i32 = 7;
pub const OpAnyNumber: i32 = 17;
pub const OpAnyText: i32 = 8;
pub const OpAnything: i32 = 16;
pub const OpAttribute_: i32 = 9;
pub const OpBeginAll: i32 = 14;
pub const OpBeginAnyElement: i32 = 3;
pub const OpBeginChoice: i32 = 10;
pub const OpBeginElement_: i32 = 2;
pub const OpBeginSequence: i32 = 12;
pub const OpElement_: i32 = 5;
pub const OpEndAll: i32 = 15;
pub const OpEndChoice: i32 = 11;
pub const OpEndElement: i32 = 4;
pub const OpEndOfTable: i32 = 1;
pub const OpEndSequence: i32 = 13;
pub const OpFormatBool_: i32 = 20;
pub const OpFormatDateTime_: i32 = 40;
pub const OpFormatDom_: i32 = 30;
pub const OpFormatDouble_: i32 = 42;
pub const OpFormatDuration_: i32 = 39;
pub const OpFormatDynamicType_: i32 = 37;
pub const OpFormatFloat_: i32 = 41;
pub const OpFormatInt16_: i32 = 22;
pub const OpFormatInt32_: i32 = 23;
pub const OpFormatInt64_: i32 = 24;
pub const OpFormatInt8_: i32 = 21;
pub const OpFormatListInsertTail_: i32 = 35;
pub const OpFormatLookupType_: i32 = 38;
pub const OpFormatMax: i32 = 46;
pub const OpFormatName_: i32 = 34;
pub const OpFormatStruct_: i32 = 31;
pub const OpFormatType_: i32 = 36;
pub const OpFormatUInt16_: i32 = 26;
pub const OpFormatUInt32_: i32 = 27;
pub const OpFormatUInt64_: i32 = 28;
pub const OpFormatUInt8_: i32 = 25;
pub const OpFormatUnicodeString_: i32 = 29;
pub const OpFormatUri_: i32 = 32;
pub const OpFormatUuidUri_: i32 = 33;
pub const OpFormatXMLDeclaration_: i32 = 45;
pub const OpNone: i32 = 0;
pub const OpOneOrMore: i32 = 18;
pub const OpOptional: i32 = 19;
pub const OpProcess_: i32 = 43;
pub const OpQualifiedAttribute_: i32 = 44;
pub type PCWSDXML_NAMESPACE = *const WSDXML_NAMESPACE;
pub type PCWSDXML_TYPE = *const WSDXML_TYPE;
pub type PWSD_CONFIG_ADDRESSES = *mut WSD_CONFIG_ADDRESSES;
pub type PWSD_CONFIG_PARAM = *mut WSD_CONFIG_PARAM;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PWSD_SECURITY_CERT_VALIDATION = *mut WSD_SECURITY_CERT_VALIDATION;
pub type PWSD_SECURITY_HTTP_AUTH_SCHEMES = *mut u32;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PWSD_SECURITY_SIGNATURE_VALIDATION = *mut WSD_SECURITY_SIGNATURE_VALIDATION;
pub type PWSD_SOAP_MESSAGE_HANDLER = Option<unsafe extern "system" fn(thisunknown: *mut core::ffi::c_void, event: *mut WSD_EVENT) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
impl Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
impl Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
impl Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: *const u16,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SOAP_Actor: u32 = 458757;
pub const SOAP_Body: u32 = 458754;
pub const SOAP_Code: u32 = 458759;
pub const SOAP_DataEncodingUnknown: u32 = 458772;
pub const SOAP_Detail: u32 = 458766;
pub const SOAP_Element: u32 = 458775;
pub const SOAP_Envelope: u32 = 458752;
pub const SOAP_Fault: u32 = 458758;
pub const SOAP_Header: u32 = 458753;
pub const SOAP_MustUnderstand: u32 = 458755;
pub const SOAP_MustUnderstand_1: u32 = 458771;
pub const SOAP_Node: u32 = 458764;
pub const SOAP_NotUnderstood: u32 = 458769;
pub const SOAP_Reason: u32 = 458762;
pub const SOAP_Receiver: u32 = 458774;
pub const SOAP_Relay: u32 = 458776;
pub const SOAP_Role: u32 = 458756;
pub const SOAP_Role_1: u32 = 458765;
pub const SOAP_Sender: u32 = 458773;
pub const SOAP_Subcode: u32 = 458760;
pub const SOAP_SupportedEnvelope: u32 = 458768;
pub const SOAP_Text: u32 = 458763;
pub const SOAP_Upgrade: u32 = 458767;
pub const SOAP_Value: u32 = 458761;
pub const SOAP_VersionMismatch: u32 = 458770;
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = 2;
pub const TWO_WAY: WSDUdpMessageType = 1;
pub const TYPE_ENCODING_WSD11_BYE: u32 = 38;
pub const TYPE_ENCODING_WSD11_HELLO: u32 = 39;
pub const TYPE_ENCODING_WSD11_PROBE: u32 = 37;
pub const TYPE_ENCODING_WSD11_PROBE_MATCH: u32 = 35;
pub const TYPE_ENCODING_WSD11_PROBE_MATCHES: u32 = 36;
pub const TYPE_ENCODING_WSD11_RESOLVE: u32 = 34;
pub const TYPE_ENCODING_WSD11_RESOLVE_MATCH: u32 = 32;
pub const TYPE_ENCODING_WSD11_RESOLVE_MATCHES: u32 = 33;
pub const TYPE_ENCODING_WSD_APP_SEQUENCE: u32 = 9;
pub const TYPE_ENCODING_WSD_BYE: u32 = 19;
pub const TYPE_ENCODING_WSD_ENDPOINT_REFERENCE: u32 = 24;
pub const TYPE_ENCODING_WSD_EVENTING_DELIVERY_MODE: u32 = 30;
pub const TYPE_ENCODING_WSD_EVENTING_DELIVERY_MODE_PUSH: u32 = 29;
pub const TYPE_ENCODING_WSD_EVENTING_EXPIRES: u32 = 28;
pub const TYPE_ENCODING_WSD_EVENTING_FILTER: u32 = 27;
pub const TYPE_ENCODING_WSD_EVENTING_FILTER_ACTION: u32 = 26;
pub const TYPE_ENCODING_WSD_HEADER_RELATESTO: u32 = 10;
pub const TYPE_ENCODING_WSD_HELLO: u32 = 21;
pub const TYPE_ENCODING_WSD_HOST_METADATA: u32 = 1;
pub const TYPE_ENCODING_WSD_LOCALIZED_STRING: u32 = 31;
pub const TYPE_ENCODING_WSD_METADATA_SECTION: u32 = 25;
pub const TYPE_ENCODING_WSD_PROBE: u32 = 18;
pub const TYPE_ENCODING_WSD_PROBE_MATCH: u32 = 16;
pub const TYPE_ENCODING_WSD_PROBE_MATCHES: u32 = 17;
pub const TYPE_ENCODING_WSD_REFERENCE_PARAMETERS: u32 = 22;
pub const TYPE_ENCODING_WSD_REFERENCE_PROPERTIES: u32 = 23;
pub const TYPE_ENCODING_WSD_RELATIONSHIP_METADATA: u32 = 0;
pub const TYPE_ENCODING_WSD_RESOLVE: u32 = 15;
pub const TYPE_ENCODING_WSD_RESOLVE_MATCH: u32 = 14;
pub const TYPE_ENCODING_WSD_RESOLVE_MATCHES: u32 = 13;
pub const TYPE_ENCODING_WSD_SCOPES: u32 = 20;
pub const TYPE_ENCODING_WSD_SERVICE_METADATA: u32 = 2;
pub const TYPE_ENCODING_WSD_SOAP_FAULT: u32 = 8;
pub const TYPE_ENCODING_WSD_SOAP_FAULT_CODE: u32 = 7;
pub const TYPE_ENCODING_WSD_SOAP_FAULT_REASON: u32 = 5;
pub const TYPE_ENCODING_WSD_SOAP_FAULT_SUBCODE: u32 = 6;
pub const TYPE_ENCODING_WSD_SOAP_HEADER: u32 = 11;
pub const TYPE_ENCODING_WSD_SOAP_MESSAGE: u32 = 12;
pub const TYPE_ENCODING_WSD_THIS_DEVICE_METADATA: u32 = 3;
pub const TYPE_ENCODING_WSD_THIS_MODEL_METADATA: u32 = 4;
pub const TextType: i32 = 1;
pub const WSA_Action: u32 = 65550;
pub const WSA_ActionNotSupported: u32 = 65555;
pub const WSA_Address: u32 = 65537;
pub const WSA_DestinationUnreachable: u32 = 65554;
pub const WSA_Element: u32 = 65558;
pub const WSA_EndpointReference: u32 = 65536;
pub const WSA_EndpointUnavailable: u32 = 65556;
pub const WSA_FaultTo: u32 = 65548;
pub const WSA_From: u32 = 65547;
pub const WSA_InvalidMessageInformationHeader: u32 = 65552;
pub const WSA_MessageID: u32 = 65542;
pub const WSA_MessageInformationHeaderRequired: u32 = 65553;
pub const WSA_PortName: u32 = 65541;
pub const WSA_PortType: u32 = 65539;
pub const WSA_Recipient: u32 = 65551;
pub const WSA_ReferenceParameters: u32 = 65538;
pub const WSA_ReferenceProperties: u32 = 65557;
pub const WSA_RelatesTo: u32 = 65543;
pub const WSA_RelationshipType: u32 = 65544;
pub const WSA_Reply: u32 = 65545;
pub const WSA_ReplyAfter: u32 = 65560;
pub const WSA_ReplyTo: u32 = 65546;
pub const WSA_RetryAfter: u32 = 65559;
pub const WSA_ServiceName: u32 = 65540;
pub const WSA_To: u32 = 65549;
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1;
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: u32 = 1;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: u32 = 2;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: u32 = 3;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4;
pub const WSDET_INCOMING_FAULT: WSDEventType = 2;
pub const WSDET_INCOMING_MESSAGE: WSDEventType = 1;
pub const WSDET_NONE: WSDEventType = 0;
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = 4;
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = 3;
pub type WSDEventType = i32;
pub const WSDISCO_AppSequence: u32 = 131083;
pub const WSDISCO_Bye: u32 = 131073;
pub const WSDISCO_DiscoveryProxy: u32 = 131089;
pub const WSDISCO_Element: u32 = 131091;
pub const WSDISCO_Hello: u32 = 131072;
pub const WSDISCO_Id: u32 = 131093;
pub const WSDISCO_InstanceId: u32 = 131084;
pub const WSDISCO_KeyId: u32 = 131097;
pub const WSDISCO_MatchBy: u32 = 131087;
pub const WSDISCO_MessageNumber: u32 = 131086;
pub const WSDISCO_MetadataVersion: u32 = 131081;
pub const WSDISCO_Probe: u32 = 131074;
pub const WSDISCO_ProbeMatch: u32 = 131076;
pub const WSDISCO_ProbeMatches: u32 = 131075;
pub const WSDISCO_Refs: u32 = 131098;
pub const WSDISCO_Resolve: u32 = 131077;
pub const WSDISCO_ResolveMatch: u32 = 131078;
pub const WSDISCO_ResolveMatches: u32 = 131092;
pub const WSDISCO_Scheme: u32 = 131096;
pub const WSDISCO_Scopes: u32 = 131080;
pub const WSDISCO_Security: u32 = 131094;
pub const WSDISCO_SequenceId: u32 = 131085;
pub const WSDISCO_Sig: u32 = 131095;
pub const WSDISCO_TargetService: u32 = 131090;
pub const WSDISCO_Transport: u32 = 131082;
pub const WSDISCO_Types: u32 = 131079;
pub const WSDISCO_XAddrs: u32 = 131088;
pub type WSDUdpMessageType = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut Self,
    pub Name: *mut WSDXML_NAME,
    pub Value: *mut u16,
}
impl Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
impl Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSDXML_ELEMENT,
}
impl Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: *mut u16,
}
impl Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NAMESPACE {
    pub Uri: *const u16,
    pub PreferredPrefix: *const u16,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
impl Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut Self,
}
impl Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut Self,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: *mut u16,
}
impl Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: *mut u16,
}
impl Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_TYPE {
    pub Uri: *const u16,
    pub Table: *const u8,
}
impl Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_APP_SEQUENCE {
    pub InstanceId: u64,
    pub SequenceId: *const u16,
    pub MessageNumber: u64,
}
impl Default for WSD_APP_SEQUENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_Actions: u32 = 327682;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_BYE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut *mut core::ffi::c_void,
    pub dwAddressCount: u32,
}
impl Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 10;
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 9;
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 1;
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSD_CONFIG_PARAM_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSD_DATETIME {
    pub isPositive: windows_sys::core::BOOL,
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u32,
    pub TZIsLocal: windows_sys::core::BOOL,
    pub TZIsPositive: windows_sys::core::BOOL,
    pub TZHour: u8,
    pub TZMinute: u8,
}
pub const WSD_DEFAULT_EVENTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("http://*:5357/");
pub const WSD_DEFAULT_HOSTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("http://*:5357/");
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("https://*:5358/");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSD_DURATION {
    pub isPositive: windows_sys::core::BOOL,
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}
pub const WSD_Data: u32 = 327701;
pub const WSD_Device: u32 = 327680;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: *const u16,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
impl Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENT {
    pub Hr: windows_sys::core::HRESULT,
    pub EventType: u32,
    pub DispatchTag: *mut u16,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: *mut core::ffi::c_void,
}
impl Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: *const u16,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut core::ffi::c_void,
}
impl Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
impl Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
impl Default for WSD_EVENTING_EXPIRES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_FILTER {
    pub Dialect: *const u16,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut core::ffi::c_void,
}
impl Default for WSD_EVENTING_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
impl Default for WSD_EVENTING_FILTER_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_Element: u32 = 327683;
pub const WSD_FilterActionNotSupported: u32 = 327681;
pub const WSD_FirmwareVersion: u32 = 327693;
pub const WSD_FriendlyName: u32 = 327692;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut core::ffi::c_void,
    pub Unknown: *mut core::ffi::c_void,
}
impl Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: *const u16,
}
impl Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
impl Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_Host: u32 = 327697;
pub const WSD_Hosted: u32 = 327698;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_LOCALIZED_STRING {
    pub lang: *const u16,
    pub String: *const u16,
}
impl Default for WSD_LOCALIZED_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
impl Default for WSD_LOCALIZED_STRING_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_METADATA_SECTION {
    pub Dialect: *const u16,
    pub Identifier: *const u16,
    pub Data: *mut core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: *const u16,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_METADATA_SECTION,
}
impl Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_Manufacturer: u32 = 327685;
pub const WSD_ManufacturerUrl: u32 = 327686;
pub const WSD_ModelName: u32 = 327687;
pub const WSD_ModelNumber: u32 = 327688;
pub const WSD_ModelUrl: u32 = 327689;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_NAME_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSDXML_NAME,
}
impl Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
impl Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
impl Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_PROBE_MATCH,
}
impl Default for WSD_PROBE_MATCH_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSD_PROTOCOL_TYPE = i32;
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = 255;
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = 2;
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = 4;
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = 0;
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = 1;
pub const WSD_PresentationUrl: u32 = 327690;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: *const u16,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_Relationship: u32 = 327699;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SCOPES {
    pub MatchBy: *const u16,
    pub Scopes: *mut WSD_URI_LIST,
}
impl Default for WSD_SCOPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::wincrypt::HCERTSTORE,
    pub hCertIssuerStore: super::wincrypt::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: windows_sys::core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::wincrypt::HCERTSTORE,
    pub hCertIssuerStore: super::wincrypt::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = 7;
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = 8;
pub type WSD_SECURITY_HTTP_AUTH_SCHEMES = u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2;
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 12;
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 11;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy)]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: super::wincrypt::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 3;
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 5;
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = 6;
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 4;
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 13;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: *const u16,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_SERVICE_METADATA,
}
impl Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: *const u16,
    pub Role: *const u16,
    pub Detail: *mut WSDXML_ELEMENT,
}
impl Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
impl Default for WSD_SOAP_FAULT_CODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
impl Default for WSD_SOAP_FAULT_REASON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut Self,
}
impl Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_HEADER {
    pub To: *const u16,
    pub Action: *const u16,
    pub MessageID: *const u16,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
impl Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
impl Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSD_STUB_FUNCTION = Option<unsafe extern "system" fn(server: *mut core::ffi::c_void, session: *mut core::ffi::c_void, event: *mut WSD_EVENT) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: windows_sys::core::HRESULT,
    pub eventHandle: super::winnt::HANDLE,
    pub messageParameters: *mut core::ffi::c_void,
    pub results: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SerialNumber: u32 = 327694;
pub const WSD_ServiceId: u32 = 327696;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: *const u16,
    pub SerialNumber: *const u16,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: *const u16,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: *const u16,
    pub ModelUrl: *const u16,
    pub PresentationUrl: *const u16,
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_ThisDevice: u32 = 327691;
pub const WSD_ThisModel: u32 = 327684;
pub const WSD_Type: u32 = 327700;
pub const WSD_Types: u32 = 327695;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
impl Default for WSD_UNKNOWN_LOOKUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSD_URI_LIST {
    pub Next: *mut Self,
    pub Element: *const u16,
}
impl Default for WSD_URI_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSE_Any: u32 = 27;
pub const WSE_Any_1: u32 = 29;
pub const WSE_Data: u32 = 19;
pub const WSE_DateTime: u32 = 22;
pub const WSE_Delivery: u32 = 20;
pub const WSE_DeliveryModeRequestedUnavailable: u32 = 1;
pub const WSE_Dialect: u32 = 24;
pub const WSE_Duration: u32 = 21;
pub const WSE_EndTo: u32 = 15;
pub const WSE_EventSink: u32 = 36;
pub const WSE_EventSourceUnableToProcess: u32 = 6;
pub const WSE_Eventing: u32 = 11;
pub const WSE_Expires: u32 = 23;
pub const WSE_Filter: u32 = 26;
pub const WSE_FilterAction: u32 = 25;
pub const WSE_FilteringNotSupported: u32 = 4;
pub const WSE_FilteringRequestedUnavailable: u32 = 5;
pub const WSE_GetStatus: u32 = 32;
pub const WSE_GetStatusResponse: u32 = 33;
pub const WSE_Identifier: u32 = 0;
pub const WSE_InvalidExpirationTime: u32 = 2;
pub const WSE_InvalidMessage: u32 = 8;
pub const WSE_Mode: u32 = 16;
pub const WSE_NotifyTo: u32 = 18;
pub const WSE_Push: u32 = 17;
pub const WSE_Reason: u32 = 40;
pub const WSE_Renew: u32 = 30;
pub const WSE_RenewResponse: u32 = 31;
pub const WSE_ReturnValue: u32 = 14;
pub const WSE_Status: u32 = 38;
pub const WSE_String: u32 = 39;
pub const WSE_Subscribe: u32 = 12;
pub const WSE_SubscribeResponse: u32 = 13;
pub const WSE_SubscriptionEnd: u32 = 37;
pub const WSE_SubscriptionManager: u32 = 28;
pub const WSE_SupportedDeliveryMode: u32 = 10;
pub const WSE_SupportedDialect: u32 = 9;
pub const WSE_UnableToRenew: u32 = 7;
pub const WSE_Unsubscribe: u32 = 34;
pub const WSE_UnsubscribeResponse: u32 = 35;
pub const WSE_UnsupportedExpirationTime: u32 = 3;
pub const WSX_Data: u32 = 393223;
pub const WSX_Dialect: u32 = 393221;
pub const WSX_Element: u32 = 393226;
pub const WSX_GetMetadata: u32 = 393217;
pub const WSX_GetMetadataResponse: u32 = 393218;
pub const WSX_Identifier: u32 = 393222;
pub const WSX_Location: u32 = 393225;
pub const WSX_Metadata: u32 = 393227;
pub const WSX_MetadataReference: u32 = 393224;
pub const WSX_MetadataSection: u32 = 393220;
pub const WSX_Mex: u32 = 393216;
pub const WSX_ReturnValue: u32 = 393219;
pub const XML_Lang: u32 = 262144;
pub const XML_Space: u32 = 262145;
pub const XOP_Href: u32 = 196609;
pub const XOP_Include: u32 = 196608;
