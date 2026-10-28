#[cfg(feature = "shared")]
windows_link::link!("wsdapi.dll" "system" fn WSDAllocateLinkedMemory(pparent : *mut core::ffi::c_void, cbsize : super::size_t) -> *mut core::ffi::c_void);
windows_link::link!("wsdapi.dll" "system" fn WSDAttachLinkedMemory(pparent : *mut core::ffi::c_void, pchild : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost2(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : super::DWORD, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceHostAdvanced(pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pphostaddresses : *const *mut core::ffi::c_void, dwhostaddresscount : super::DWORD, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy(pszdeviceid : windows_sys::core::PCWSTR, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy2(pszdeviceid : windows_sys::core::PCWSTR, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : super::DWORD, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxyAdvanced(pszdeviceid : windows_sys::core::PCWSTR, pdeviceaddress : *mut core::ffi::c_void, pszlocalid : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider(pcontext : *mut core::ffi::c_void, ppprovider : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider2(pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : super::DWORD, ppprovider : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher(pcontext : *mut core::ffi::c_void, pppublisher : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher2(pcontext : *mut core::ffi::c_void, pconfigparams : *const WSD_CONFIG_PARAM, dwconfigparamcount : super::DWORD, pppublisher : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateHttpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateHttpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateOutboundAttachment(ppattachment : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateUdpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDCreateUdpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDDetachLinkedMemory(pvoid : *mut core::ffi::c_void));
windows_link::link!("wsdapi.dll" "system" fn WSDFreeLinkedMemory(pvoid : *mut core::ffi::c_void));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDGenerateFault(pszcode : windows_sys::core::PCWSTR, pszsubcode : windows_sys::core::PCWSTR, pszreason : windows_sys::core::PCWSTR, pszdetail : windows_sys::core::PCWSTR, pcontext : *mut core::ffi::c_void, ppfault : *mut *mut WSD_SOAP_FAULT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDGenerateFaultEx(pcode : *const WSDXML_NAME, psubcode : *const WSDXML_NAME, preasons : *const WSD_LOCALIZED_STRING_LIST, pszdetail : windows_sys::core::PCWSTR, ppfault : *mut *mut WSD_SOAP_FAULT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("wsdapi.dll" "system" fn WSDGetConfigurationOption(dwoption : super::DWORD, pvoid : super::LPVOID, cboutbuffer : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("wsdapi.dll" "system" fn WSDSetConfigurationOption(dwoption : super::DWORD, pvoid : super::LPVOID, cbinbuffer : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("wsdapi.dll" "system" fn WSDUriDecode(source : windows_sys::core::PCWSTR, cchsource : super::DWORD, destout : *mut windows_sys::core::PWSTR, cchdestout : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("wsdapi.dll" "system" fn WSDUriEncode(source : windows_sys::core::PCWSTR, cchsource : super::DWORD, destout : *mut windows_sys::core::PWSTR, cchdestout : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLAddChild(pparent : *mut WSDXML_ELEMENT, pchild : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLAddSibling(pfirst : *mut WSDXML_ELEMENT, psecond : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLBuildAnyForSingleElement(pelementname : *mut WSDXML_NAME, psztext : windows_sys::core::PCWSTR, ppany : *mut *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLCleanupElement(pany : *mut WSDXML_ELEMENT) -> windows_sys::core::HRESULT);
windows_link::link!("wsdapi.dll" "system" fn WSDXMLCreateContext(ppcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLGetNameFromBuiltinNamespace(psznamespace : windows_sys::core::PCWSTR, pszname : windows_sys::core::PCWSTR, ppname : *mut *mut WSDXML_NAME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wsdapi.dll" "system" fn WSDXMLGetValueFromAny(psznamespace : windows_sys::core::PCWSTR, pszname : windows_sys::core::PCWSTR, pany : *mut WSDXML_ELEMENT, ppszvalue : *mut windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
pub const DPP_AppSequence: super::DWORD = 524299;
#[cfg(feature = "shared")]
pub const DPP_Bye: super::DWORD = 524289;
#[cfg(feature = "shared")]
pub const DPP_DiscoveryProxy: super::DWORD = 524305;
#[cfg(feature = "shared")]
pub const DPP_Element: super::DWORD = 524307;
#[cfg(feature = "shared")]
pub const DPP_Hello: super::DWORD = 524288;
#[cfg(feature = "shared")]
pub const DPP_Id: super::DWORD = 524309;
#[cfg(feature = "shared")]
pub const DPP_InstanceId: super::DWORD = 524300;
#[cfg(feature = "shared")]
pub const DPP_KeyId: super::DWORD = 524313;
#[cfg(feature = "shared")]
pub const DPP_MatchBy: super::DWORD = 524303;
#[cfg(feature = "shared")]
pub const DPP_MessageNumber: super::DWORD = 524302;
#[cfg(feature = "shared")]
pub const DPP_MetadataVersion: super::DWORD = 524297;
#[cfg(feature = "shared")]
pub const DPP_Probe: super::DWORD = 524290;
#[cfg(feature = "shared")]
pub const DPP_ProbeMatch: super::DWORD = 524292;
#[cfg(feature = "shared")]
pub const DPP_ProbeMatches: super::DWORD = 524291;
#[cfg(feature = "shared")]
pub const DPP_Refs: super::DWORD = 524314;
#[cfg(feature = "shared")]
pub const DPP_Resolve: super::DWORD = 524293;
#[cfg(feature = "shared")]
pub const DPP_ResolveMatch: super::DWORD = 524294;
#[cfg(feature = "shared")]
pub const DPP_ResolveMatches: super::DWORD = 524308;
#[cfg(feature = "shared")]
pub const DPP_Scheme: super::DWORD = 524312;
#[cfg(feature = "shared")]
pub const DPP_Scopes: super::DWORD = 524296;
#[cfg(feature = "shared")]
pub const DPP_Security: super::DWORD = 524310;
#[cfg(feature = "shared")]
pub const DPP_SequenceId: super::DWORD = 524301;
#[cfg(feature = "shared")]
pub const DPP_Sig: super::DWORD = 524311;
#[cfg(feature = "shared")]
pub const DPP_TargetService: super::DWORD = 524306;
#[cfg(feature = "shared")]
pub const DPP_Transport: super::DWORD = 524298;
#[cfg(feature = "shared")]
pub const DPP_Types: super::DWORD = 524295;
#[cfg(feature = "shared")]
pub const DPP_XAddrs: super::DWORD = 524304;
pub type DeviceDiscoveryMechanism = i32;
pub const DirectedDiscovery: DeviceDiscoveryMechanism = 1;
pub const MulticastDiscovery: DeviceDiscoveryMechanism = 0;
pub const NAMESPACE_ENCODING_ADDRESSING: i32 = 1;
pub const NAMESPACE_ENCODING_DEVPROF: i32 = 5;
pub const NAMESPACE_ENCODING_DISCOVERY: i32 = 2;
pub const NAMESPACE_ENCODING_DISCOVERY11: i32 = 8;
pub const NAMESPACE_ENCODING_ENVELOPE: i32 = 7;
pub const NAMESPACE_ENCODING_EVENTING: i32 = 0;
pub const NAMESPACE_ENCODING_INCLUDE: i32 = 3;
pub const NAMESPACE_ENCODING_MEX: i32 = 6;
pub const NAMESPACE_ENCODING_XML: i32 = 4;
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
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWSDXML_NAMESPACE = *const _WSDXML_NAMESPACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWSDXML_TYPE = *const _WSDXML_TYPE;
#[cfg(feature = "shared")]
pub type PWSD_CONFIG_ADDRESSES = *mut WSD_CONFIG_ADDRESSES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWSD_CONFIG_PARAM = *mut WSD_CONFIG_PARAM;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
pub type PWSD_SECURITY_CERT_VALIDATION = *mut WSD_SECURITY_CERT_VALIDATION;
pub type PWSD_SECURITY_HTTP_AUTH_SCHEMES = *mut u32;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
pub type PWSD_SECURITY_SIGNATURE_VALIDATION = *mut WSD_SECURITY_SIGNATURE_VALIDATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWSD_SOAP_MESSAGE_HANDLER = Option<unsafe extern "C" fn(thisunknown: *mut core::ffi::c_void, event: *mut WSD_EVENT) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: *const super::WCHAR,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "shared")]
pub const SOAP_Actor: super::DWORD = 458757;
#[cfg(feature = "shared")]
pub const SOAP_Body: super::DWORD = 458754;
#[cfg(feature = "shared")]
pub const SOAP_Code: super::DWORD = 458759;
#[cfg(feature = "shared")]
pub const SOAP_DataEncodingUnknown: super::DWORD = 458772;
#[cfg(feature = "shared")]
pub const SOAP_Detail: super::DWORD = 458766;
#[cfg(feature = "shared")]
pub const SOAP_Element: super::DWORD = 458775;
#[cfg(feature = "shared")]
pub const SOAP_Envelope: super::DWORD = 458752;
#[cfg(feature = "shared")]
pub const SOAP_Fault: super::DWORD = 458758;
#[cfg(feature = "shared")]
pub const SOAP_Header: super::DWORD = 458753;
#[cfg(feature = "shared")]
pub const SOAP_MustUnderstand: super::DWORD = 458755;
#[cfg(feature = "shared")]
pub const SOAP_MustUnderstand_1: super::DWORD = 458771;
#[cfg(feature = "shared")]
pub const SOAP_Node: super::DWORD = 458764;
#[cfg(feature = "shared")]
pub const SOAP_NotUnderstood: super::DWORD = 458769;
#[cfg(feature = "shared")]
pub const SOAP_Reason: super::DWORD = 458762;
#[cfg(feature = "shared")]
pub const SOAP_Receiver: super::DWORD = 458774;
#[cfg(feature = "shared")]
pub const SOAP_Relay: super::DWORD = 458776;
#[cfg(feature = "shared")]
pub const SOAP_Role: super::DWORD = 458756;
#[cfg(feature = "shared")]
pub const SOAP_Role_1: super::DWORD = 458765;
#[cfg(feature = "shared")]
pub const SOAP_Sender: super::DWORD = 458773;
#[cfg(feature = "shared")]
pub const SOAP_Subcode: super::DWORD = 458760;
#[cfg(feature = "shared")]
pub const SOAP_SupportedEnvelope: super::DWORD = 458768;
#[cfg(feature = "shared")]
pub const SOAP_Text: super::DWORD = 458763;
#[cfg(feature = "shared")]
pub const SOAP_Upgrade: super::DWORD = 458767;
#[cfg(feature = "shared")]
pub const SOAP_Value: super::DWORD = 458761;
#[cfg(feature = "shared")]
pub const SOAP_VersionMismatch: super::DWORD = 458770;
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = 2;
pub const TWO_WAY: WSDUdpMessageType = 1;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_BYE: super::DWORD = 38;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_HELLO: super::DWORD = 39;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE: super::DWORD = 37;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE_MATCH: super::DWORD = 35;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE_MATCHES: super::DWORD = 36;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE: super::DWORD = 34;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE_MATCH: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE_MATCHES: super::DWORD = 33;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_APP_SEQUENCE: super::DWORD = 9;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_BYE: super::DWORD = 19;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_ENDPOINT_REFERENCE: super::DWORD = 24;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_DELIVERY_MODE: super::DWORD = 30;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_DELIVERY_MODE_PUSH: super::DWORD = 29;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_EXPIRES: super::DWORD = 28;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_FILTER: super::DWORD = 27;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_FILTER_ACTION: super::DWORD = 26;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HEADER_RELATESTO: super::DWORD = 10;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HELLO: super::DWORD = 21;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HOST_METADATA: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_LOCALIZED_STRING: super::DWORD = 31;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_METADATA_SECTION: super::DWORD = 25;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE: super::DWORD = 18;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE_MATCH: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE_MATCHES: super::DWORD = 17;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_REFERENCE_PARAMETERS: super::DWORD = 22;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_REFERENCE_PROPERTIES: super::DWORD = 23;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RELATIONSHIP_METADATA: super::DWORD = 0;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE: super::DWORD = 15;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE_MATCH: super::DWORD = 14;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE_MATCHES: super::DWORD = 13;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SCOPES: super::DWORD = 20;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SERVICE_METADATA: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_CODE: super::DWORD = 7;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_REASON: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_SUBCODE: super::DWORD = 6;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_HEADER: super::DWORD = 11;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_MESSAGE: super::DWORD = 12;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_THIS_DEVICE_METADATA: super::DWORD = 3;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_THIS_MODEL_METADATA: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const WSA_Action: super::DWORD = 65550;
#[cfg(feature = "shared")]
pub const WSA_ActionNotSupported: super::DWORD = 65555;
#[cfg(feature = "shared")]
pub const WSA_Address: super::DWORD = 65537;
#[cfg(feature = "shared")]
pub const WSA_DestinationUnreachable: super::DWORD = 65554;
#[cfg(feature = "shared")]
pub const WSA_Element: super::DWORD = 65558;
#[cfg(feature = "shared")]
pub const WSA_EndpointReference: super::DWORD = 65536;
#[cfg(feature = "shared")]
pub const WSA_EndpointUnavailable: super::DWORD = 65556;
#[cfg(feature = "shared")]
pub const WSA_FaultTo: super::DWORD = 65548;
#[cfg(feature = "shared")]
pub const WSA_From: super::DWORD = 65547;
#[cfg(feature = "shared")]
pub const WSA_InvalidMessageInformationHeader: super::DWORD = 65552;
#[cfg(feature = "shared")]
pub const WSA_MessageID: super::DWORD = 65542;
#[cfg(feature = "shared")]
pub const WSA_MessageInformationHeaderRequired: super::DWORD = 65553;
#[cfg(feature = "shared")]
pub const WSA_PortName: super::DWORD = 65541;
#[cfg(feature = "shared")]
pub const WSA_PortType: super::DWORD = 65539;
#[cfg(feature = "shared")]
pub const WSA_Recipient: super::DWORD = 65551;
#[cfg(feature = "shared")]
pub const WSA_ReferenceParameters: super::DWORD = 65538;
#[cfg(feature = "shared")]
pub const WSA_ReferenceProperties: super::DWORD = 65557;
#[cfg(feature = "shared")]
pub const WSA_RelatesTo: super::DWORD = 65543;
#[cfg(feature = "shared")]
pub const WSA_RelationshipType: super::DWORD = 65544;
#[cfg(feature = "shared")]
pub const WSA_Reply: super::DWORD = 65545;
#[cfg(feature = "shared")]
pub const WSA_ReplyAfter: super::DWORD = 65560;
#[cfg(feature = "shared")]
pub const WSA_ReplyTo: super::DWORD = 65546;
#[cfg(feature = "shared")]
pub const WSA_RetryAfter: super::DWORD = 65559;
#[cfg(feature = "shared")]
pub const WSA_ServiceName: super::DWORD = 65540;
#[cfg(feature = "shared")]
pub const WSA_To: super::DWORD = 65549;
pub const WSDAPI_ADDRESSFAMILY_IPV4: i32 = 1;
pub const WSDAPI_ADDRESSFAMILY_IPV6: i32 = 2;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: i32 = 1;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: i32 = 1;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: i32 = 2;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: i32 = 3;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: i32 = 0;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: i32 = 2;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: i32 = 16;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: i32 = 1;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: i32 = 8;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: i32 = 4;
pub const WSDET_INCOMING_FAULT: WSDEventType = 2;
pub const WSDET_INCOMING_MESSAGE: WSDEventType = 1;
pub const WSDET_NONE: WSDEventType = 0;
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = 4;
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = 3;
pub type WSDEventType = i32;
#[cfg(feature = "shared")]
pub const WSDISCO_AppSequence: super::DWORD = 131083;
#[cfg(feature = "shared")]
pub const WSDISCO_Bye: super::DWORD = 131073;
#[cfg(feature = "shared")]
pub const WSDISCO_DiscoveryProxy: super::DWORD = 131089;
#[cfg(feature = "shared")]
pub const WSDISCO_Element: super::DWORD = 131091;
#[cfg(feature = "shared")]
pub const WSDISCO_Hello: super::DWORD = 131072;
#[cfg(feature = "shared")]
pub const WSDISCO_Id: super::DWORD = 131093;
#[cfg(feature = "shared")]
pub const WSDISCO_InstanceId: super::DWORD = 131084;
#[cfg(feature = "shared")]
pub const WSDISCO_KeyId: super::DWORD = 131097;
#[cfg(feature = "shared")]
pub const WSDISCO_MatchBy: super::DWORD = 131087;
#[cfg(feature = "shared")]
pub const WSDISCO_MessageNumber: super::DWORD = 131086;
#[cfg(feature = "shared")]
pub const WSDISCO_MetadataVersion: super::DWORD = 131081;
#[cfg(feature = "shared")]
pub const WSDISCO_Probe: super::DWORD = 131074;
#[cfg(feature = "shared")]
pub const WSDISCO_ProbeMatch: super::DWORD = 131076;
#[cfg(feature = "shared")]
pub const WSDISCO_ProbeMatches: super::DWORD = 131075;
#[cfg(feature = "shared")]
pub const WSDISCO_Refs: super::DWORD = 131098;
#[cfg(feature = "shared")]
pub const WSDISCO_Resolve: super::DWORD = 131077;
#[cfg(feature = "shared")]
pub const WSDISCO_ResolveMatch: super::DWORD = 131078;
#[cfg(feature = "shared")]
pub const WSDISCO_ResolveMatches: super::DWORD = 131092;
#[cfg(feature = "shared")]
pub const WSDISCO_Scheme: super::DWORD = 131096;
#[cfg(feature = "shared")]
pub const WSDISCO_Scopes: super::DWORD = 131080;
#[cfg(feature = "shared")]
pub const WSDISCO_Security: super::DWORD = 131094;
#[cfg(feature = "shared")]
pub const WSDISCO_SequenceId: super::DWORD = 131085;
#[cfg(feature = "shared")]
pub const WSDISCO_Sig: super::DWORD = 131095;
#[cfg(feature = "shared")]
pub const WSDISCO_TargetService: super::DWORD = 131090;
#[cfg(feature = "shared")]
pub const WSDISCO_Transport: super::DWORD = 131082;
#[cfg(feature = "shared")]
pub const WSDISCO_Types: super::DWORD = 131079;
#[cfg(feature = "shared")]
pub const WSDISCO_XAddrs: super::DWORD = 131088;
pub type WSDUdpMessageType = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: super::ULONG,
    pub ulRepeat: super::ULONG,
    pub ulRepeatMinDelay: super::ULONG,
    pub ulRepeatMaxDelay: super::ULONG,
    pub ulRepeatUpperDelay: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_ATTRIBUTE = _WSDXML_ATTRIBUTE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_ELEMENT = _WSDXML_ELEMENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_ELEMENT_LIST = _WSDXML_ELEMENT_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_NAME = _WSDXML_NAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_NAMESPACE = _WSDXML_NAMESPACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_NODE = _WSDXML_NODE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_PREFIX_MAPPING = _WSDXML_PREFIX_MAPPING;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_TEXT = _WSDXML_TEXT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_TYPE = _WSDXML_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_APP_SEQUENCE = _WSD_APP_SEQUENCE;
#[cfg(feature = "shared")]
pub const WSD_Actions: super::DWORD = 327682;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_BYE = _WSD_BYE;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut *mut core::ffi::c_void,
    pub dwAddressCount: super::DWORD,
}
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 10;
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 9;
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 1;
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: super::PVOID,
    pub dwConfigDataSize: super::DWORD,
}
pub type WSD_CONFIG_PARAM_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WSD_DATETIME {
    pub isPositive: windows_sys::core::BOOL,
    pub year: super::ULONG,
    pub month: super::UCHAR,
    pub day: super::UCHAR,
    pub hour: super::UCHAR,
    pub minute: super::UCHAR,
    pub second: super::UCHAR,
    pub millisecond: super::UINT,
    pub TZIsLocal: windows_sys::core::BOOL,
    pub TZIsPositive: windows_sys::core::BOOL,
    pub TZHour: super::UCHAR,
    pub TZMinute: super::UCHAR,
}
pub const WSD_DEFAULT_EVENTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("http://*:5357/");
pub const WSD_DEFAULT_HOSTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("http://*:5357/");
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: windows_sys::core::PCWSTR = windows_sys::core::w!("https://*:5358/");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WSD_DURATION {
    pub isPositive: windows_sys::core::BOOL,
    pub year: super::ULONG,
    pub month: super::ULONG,
    pub day: super::ULONG,
    pub hour: super::ULONG,
    pub minute: super::ULONG,
    pub second: super::ULONG,
    pub millisecond: super::ULONG,
}
#[cfg(feature = "shared")]
pub const WSD_Data: super::DWORD = 327701;
#[cfg(feature = "shared")]
pub const WSD_Device: super::DWORD = 327680;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_ENDPOINT_REFERENCE = _WSD_ENDPOINT_REFERENCE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_ENDPOINT_REFERENCE_LIST = _WSD_ENDPOINT_REFERENCE_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_EVENT = _WSD_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_EVENTING_DELIVERY_MODE = _WSD_EVENTING_DELIVERY_MODE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_EVENTING_DELIVERY_MODE_PUSH = _WSD_EVENTING_DELIVERY_MODE_PUSH;
#[cfg(feature = "shared")]
pub type WSD_EVENTING_EXPIRES = _WSD_EVENTING_EXPIRES;
#[cfg(feature = "winnt")]
pub type WSD_EVENTING_FILTER = _WSD_EVENTING_FILTER;
#[cfg(feature = "winnt")]
pub type WSD_EVENTING_FILTER_ACTION = _WSD_EVENTING_FILTER_ACTION;
#[cfg(feature = "shared")]
pub const WSD_Element: super::DWORD = 327683;
#[cfg(feature = "shared")]
pub const WSD_FilterActionNotSupported: super::DWORD = 327681;
#[cfg(feature = "shared")]
pub const WSD_FirmwareVersion: super::DWORD = 327693;
#[cfg(feature = "shared")]
pub const WSD_FriendlyName: super::DWORD = 327692;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HANDLER_CONTEXT = _WSD_HANDLER_CONTEXT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HEADER_RELATESTO = _WSD_HEADER_RELATESTO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HELLO = _WSD_HELLO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HOST_METADATA = _WSD_HOST_METADATA;
#[cfg(feature = "shared")]
pub const WSD_Host: super::DWORD = 327697;
#[cfg(feature = "shared")]
pub const WSD_Hosted: super::DWORD = 327698;
#[cfg(feature = "winnt")]
pub type WSD_LOCALIZED_STRING = _WSD_LOCALIZED_STRING;
#[cfg(feature = "winnt")]
pub type WSD_LOCALIZED_STRING_LIST = _WSD_LOCALIZED_STRING_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_METADATA_SECTION = _WSD_METADATA_SECTION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_METADATA_SECTION_LIST = _WSD_METADATA_SECTION_LIST;
#[cfg(feature = "shared")]
pub const WSD_Manufacturer: super::DWORD = 327685;
#[cfg(feature = "shared")]
pub const WSD_ManufacturerUrl: super::DWORD = 327686;
#[cfg(feature = "shared")]
pub const WSD_ModelName: super::DWORD = 327687;
#[cfg(feature = "shared")]
pub const WSD_ModelNumber: super::DWORD = 327688;
#[cfg(feature = "shared")]
pub const WSD_ModelUrl: super::DWORD = 327689;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_NAME_LIST = _WSD_NAME_LIST;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WSD_OPERATION {
    pub RequestType: *mut WSDXML_TYPE,
    pub ResponseType: *mut WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WSD_PORT_TYPE {
    pub EncodedName: super::DWORD,
    pub OperationCount: super::DWORD,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_PROBE = _WSD_PROBE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_PROBE_MATCH = _WSD_PROBE_MATCH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_PROBE_MATCHES = _WSD_PROBE_MATCHES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_PROBE_MATCH_LIST = _WSD_PROBE_MATCH_LIST;
pub type WSD_PROTOCOL_TYPE = i32;
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = 255;
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = 2;
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = 4;
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = 0;
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = 1;
#[cfg(feature = "shared")]
pub const WSD_PresentationUrl: super::DWORD = 327690;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_REFERENCE_PARAMETERS = _WSD_REFERENCE_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_REFERENCE_PROPERTIES = _WSD_REFERENCE_PROPERTIES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_RELATIONSHIP_METADATA = _WSD_RELATIONSHIP_METADATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_RESOLVE = _WSD_RESOLVE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_RESOLVE_MATCH = _WSD_RESOLVE_MATCH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_RESOLVE_MATCHES = _WSD_RESOLVE_MATCHES;
#[cfg(feature = "shared")]
pub const WSD_Relationship: super::DWORD = 327699;
#[cfg(feature = "winnt")]
pub type WSD_SCOPES = _WSD_SCOPES;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Default)]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut super::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: super::DWORD,
    pub hCertMatchStore: super::HCERTSTORE,
    pub hCertIssuerStore: super::HCERTSTORE,
    pub dwCertCheckOptions: super::DWORD,
    pub pszCNGHashAlgId: windows_sys::core::PCWSTR,
    pub pbCertHash: *mut super::BYTE,
    pub dwCertHashSize: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Default)]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut super::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: super::DWORD,
    pub hCertMatchStore: super::HCERTSTORE,
    pub hCertIssuerStore: super::HCERTSTORE,
    pub dwCertCheckOptions: super::DWORD,
}
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = 7;
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = 8;
pub type WSD_SECURITY_HTTP_AUTH_SCHEMES = u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: i32 = 1;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: i32 = 2;
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 12;
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 11;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Default)]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut super::PCCERT_CONTEXT,
    pub dwSigningCertArrayCount: super::DWORD,
    pub hSigningCertStore: super::HCERTSTORE,
    pub dwFlags: super::DWORD,
}
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 3;
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 5;
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = 6;
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 4;
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 13;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SERVICE_METADATA = _WSD_SERVICE_METADATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SERVICE_METADATA_LIST = _WSD_SERVICE_METADATA_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_FAULT = _WSD_SOAP_FAULT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_FAULT_CODE = _WSD_SOAP_FAULT_CODE;
#[cfg(feature = "winnt")]
pub type WSD_SOAP_FAULT_REASON = _WSD_SOAP_FAULT_REASON;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_FAULT_SUBCODE = _WSD_SOAP_FAULT_SUBCODE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_HEADER = _WSD_SOAP_HEADER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_MESSAGE = _WSD_SOAP_MESSAGE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_STUB_FUNCTION = Option<unsafe extern "C" fn(server: *mut core::ffi::c_void, session: *mut core::ffi::c_void, event: *mut WSD_EVENT) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: windows_sys::core::HRESULT,
    pub eventHandle: super::HANDLE,
    pub messageParameters: *mut core::ffi::c_void,
    pub results: *mut core::ffi::c_void,
}
#[cfg(feature = "shared")]
pub const WSD_SerialNumber: super::DWORD = 327694;
#[cfg(feature = "shared")]
pub const WSD_ServiceId: super::DWORD = 327696;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_THIS_DEVICE_METADATA = _WSD_THIS_DEVICE_METADATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_THIS_MODEL_METADATA = _WSD_THIS_MODEL_METADATA;
#[cfg(feature = "shared")]
pub const WSD_ThisDevice: super::DWORD = 327691;
#[cfg(feature = "shared")]
pub const WSD_ThisModel: super::DWORD = 327684;
#[cfg(feature = "shared")]
pub const WSD_Type: super::DWORD = 327700;
#[cfg(feature = "shared")]
pub const WSD_Types: super::DWORD = 327695;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut WSDXML_ELEMENT,
}
#[cfg(feature = "winnt")]
pub type WSD_URI_LIST = _WSD_URI_LIST;
#[cfg(feature = "shared")]
pub const WSE_Any: super::DWORD = 27;
#[cfg(feature = "shared")]
pub const WSE_Any_1: super::DWORD = 29;
#[cfg(feature = "shared")]
pub const WSE_Data: super::DWORD = 19;
#[cfg(feature = "shared")]
pub const WSE_DateTime: super::DWORD = 22;
#[cfg(feature = "shared")]
pub const WSE_Delivery: super::DWORD = 20;
#[cfg(feature = "shared")]
pub const WSE_DeliveryModeRequestedUnavailable: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const WSE_Dialect: super::DWORD = 24;
#[cfg(feature = "shared")]
pub const WSE_Duration: super::DWORD = 21;
#[cfg(feature = "shared")]
pub const WSE_EndTo: super::DWORD = 15;
#[cfg(feature = "shared")]
pub const WSE_EventSink: super::DWORD = 36;
#[cfg(feature = "shared")]
pub const WSE_EventSourceUnableToProcess: super::DWORD = 6;
#[cfg(feature = "shared")]
pub const WSE_Eventing: super::DWORD = 11;
#[cfg(feature = "shared")]
pub const WSE_Expires: super::DWORD = 23;
#[cfg(feature = "shared")]
pub const WSE_Filter: super::DWORD = 26;
#[cfg(feature = "shared")]
pub const WSE_FilterAction: super::DWORD = 25;
#[cfg(feature = "shared")]
pub const WSE_FilteringNotSupported: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const WSE_FilteringRequestedUnavailable: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const WSE_GetStatus: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const WSE_GetStatusResponse: super::DWORD = 33;
#[cfg(feature = "shared")]
pub const WSE_Identifier: super::DWORD = 0;
#[cfg(feature = "shared")]
pub const WSE_InvalidExpirationTime: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const WSE_InvalidMessage: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const WSE_Mode: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const WSE_NotifyTo: super::DWORD = 18;
#[cfg(feature = "shared")]
pub const WSE_Push: super::DWORD = 17;
#[cfg(feature = "shared")]
pub const WSE_Reason: super::DWORD = 40;
#[cfg(feature = "shared")]
pub const WSE_Renew: super::DWORD = 30;
#[cfg(feature = "shared")]
pub const WSE_RenewResponse: super::DWORD = 31;
#[cfg(feature = "shared")]
pub const WSE_ReturnValue: super::DWORD = 14;
#[cfg(feature = "shared")]
pub const WSE_Status: super::DWORD = 38;
#[cfg(feature = "shared")]
pub const WSE_String: super::DWORD = 39;
#[cfg(feature = "shared")]
pub const WSE_Subscribe: super::DWORD = 12;
#[cfg(feature = "shared")]
pub const WSE_SubscribeResponse: super::DWORD = 13;
#[cfg(feature = "shared")]
pub const WSE_SubscriptionEnd: super::DWORD = 37;
#[cfg(feature = "shared")]
pub const WSE_SubscriptionManager: super::DWORD = 28;
#[cfg(feature = "shared")]
pub const WSE_SupportedDeliveryMode: super::DWORD = 10;
#[cfg(feature = "shared")]
pub const WSE_SupportedDialect: super::DWORD = 9;
#[cfg(feature = "shared")]
pub const WSE_UnableToRenew: super::DWORD = 7;
#[cfg(feature = "shared")]
pub const WSE_Unsubscribe: super::DWORD = 34;
#[cfg(feature = "shared")]
pub const WSE_UnsubscribeResponse: super::DWORD = 35;
#[cfg(feature = "shared")]
pub const WSE_UnsupportedExpirationTime: super::DWORD = 3;
#[cfg(feature = "shared")]
pub const WSX_Data: super::DWORD = 393223;
#[cfg(feature = "shared")]
pub const WSX_Dialect: super::DWORD = 393221;
#[cfg(feature = "shared")]
pub const WSX_Element: super::DWORD = 393226;
#[cfg(feature = "shared")]
pub const WSX_GetMetadata: super::DWORD = 393217;
#[cfg(feature = "shared")]
pub const WSX_GetMetadataResponse: super::DWORD = 393218;
#[cfg(feature = "shared")]
pub const WSX_Identifier: super::DWORD = 393222;
#[cfg(feature = "shared")]
pub const WSX_Location: super::DWORD = 393225;
#[cfg(feature = "shared")]
pub const WSX_Metadata: super::DWORD = 393227;
#[cfg(feature = "shared")]
pub const WSX_MetadataReference: super::DWORD = 393224;
#[cfg(feature = "shared")]
pub const WSX_MetadataSection: super::DWORD = 393220;
#[cfg(feature = "shared")]
pub const WSX_Mex: super::DWORD = 393216;
#[cfg(feature = "shared")]
pub const WSX_ReturnValue: super::DWORD = 393219;
#[cfg(feature = "shared")]
pub const XML_Lang: super::DWORD = 262144;
#[cfg(feature = "shared")]
pub const XML_Space: super::DWORD = 262145;
#[cfg(feature = "shared")]
pub const XOP_Href: super::DWORD = 196609;
#[cfg(feature = "shared")]
pub const XOP_Include: super::DWORD = 196608;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_ATTRIBUTE,
    pub Name: *mut WSDXML_NAME,
    pub Value: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_ELEMENT_LIST {
    pub Next: *mut WSDXML_ELEMENT_LIST,
    pub Element: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_NAMESPACE {
    pub Uri: *const super::WCHAR,
    pub PreferredPrefix: *const super::WCHAR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: super::WORD,
    pub Encoding: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut WSDXML_NODE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl _WSDXML_NODE {
    pub const ElementType: i32 = 0;
    pub const TextType: i32 = 1;
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_PREFIX_MAPPING {
    pub Refs: super::DWORD,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSDXML_TYPE {
    pub Uri: *const super::WCHAR,
    pub Table: *const super::BYTE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_APP_SEQUENCE {
    pub InstanceId: super::ULONGLONG,
    pub SequenceId: *const super::WCHAR,
    pub MessageNumber: super::ULONGLONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_ENDPOINT_REFERENCE {
    pub Address: *const super::WCHAR,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut WSDXML_NAME,
    pub ServiceName: *mut WSDXML_NAME,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_EVENT {
    pub Hr: windows_sys::core::HRESULT,
    pub EventType: super::DWORD,
    pub DispatchTag: *mut super::WCHAR,
    pub HandlerContext: WSD_HANDLER_CONTEXT,
    pub Soap: *mut WSD_SOAP_MESSAGE,
    pub Operation: *mut WSD_OPERATION,
    pub MessageParameters: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_EVENTING_DELIVERY_MODE {
    pub Mode: *const super::WCHAR,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct _WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct _WSD_EVENTING_FILTER {
    pub Dialect: *const super::WCHAR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct _WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut core::ffi::c_void,
    pub Unknown: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: *const super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: super::ULONGLONG,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct _WSD_LOCALIZED_STRING {
    pub lang: *const super::WCHAR,
    pub String: *const super::WCHAR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct _WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_METADATA_SECTION {
    pub Dialect: *const super::WCHAR,
    pub Identifier: *const super::WCHAR,
    pub Data: *mut core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: *const super::WCHAR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_METADATA_SECTION_LIST {
    pub Next: *mut WSD_METADATA_SECTION_LIST,
    pub Element: *mut WSD_METADATA_SECTION,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: super::ULONGLONG,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_PROBE_MATCH_LIST {
    pub Next: *mut WSD_PROBE_MATCH_LIST,
    pub Element: *mut WSD_PROBE_MATCH,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_REFERENCE_PARAMETERS {
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_REFERENCE_PROPERTIES {
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_RELATIONSHIP_METADATA {
    pub Type: *const super::WCHAR,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: super::ULONGLONG,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SCOPES {
    pub MatchBy: *const super::WCHAR,
    pub Scopes: *mut WSD_URI_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: *const super::WCHAR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SERVICE_METADATA_LIST {
    pub Next: *mut WSD_SERVICE_METADATA_LIST,
    pub Element: *mut WSD_SERVICE_METADATA,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: *const super::WCHAR,
    pub Role: *const super::WCHAR,
    pub Detail: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SOAP_HEADER {
    pub To: *const super::WCHAR,
    pub Action: *const super::WCHAR,
    pub MessageID: *const super::WCHAR,
    pub RelatesTo: WSD_HEADER_RELATESTO,
    pub ReplyTo: *mut WSD_ENDPOINT_REFERENCE,
    pub From: *mut WSD_ENDPOINT_REFERENCE,
    pub FaultTo: *mut WSD_ENDPOINT_REFERENCE,
    pub AppSequence: *mut WSD_APP_SEQUENCE,
    pub AnyHeaders: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut core::ffi::c_void,
    pub BodyType: *mut WSDXML_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: *const super::WCHAR,
    pub SerialNumber: *const super::WCHAR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct _WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: *const super::WCHAR,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: *const super::WCHAR,
    pub ModelUrl: *const super::WCHAR,
    pub PresentationUrl: *const super::WCHAR,
    pub Any: *mut WSDXML_ELEMENT,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct _WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: *const super::WCHAR,
}
