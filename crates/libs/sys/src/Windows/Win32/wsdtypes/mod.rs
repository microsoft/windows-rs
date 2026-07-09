pub type DeviceDiscoveryMechanism = i32;
pub const DirectedDiscovery: DeviceDiscoveryMechanism = 1;
pub const MulticastDiscovery: DeviceDiscoveryMechanism = 0;
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
pub type PWSD_SOAP_MESSAGE_HANDLER = Option<unsafe extern "system" fn(thisunknown: *mut core::ffi::c_void, event: *mut WSD_EVENT) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_GetStatus {
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Renew {
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Subscribe {
    pub EndTo: *mut WSD_ENDPOINT_REFERENCE,
    pub Delivery: *mut WSD_EVENTING_DELIVERY_MODE,
    pub Expires: *mut WSD_EVENTING_EXPIRES,
    pub Filter: *mut WSD_EVENTING_FILTER,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct REQUESTBODY_Unsubscribe {
    pub any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_GetMetadata {
    pub Metadata: *mut WSD_METADATA_SECTION_LIST,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_GetStatus {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_Renew {
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_Subscribe {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub expires: *mut WSD_EVENTING_EXPIRES,
    pub any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(all(feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct RESPONSEBODY_SubscriptionEnd {
    pub SubscriptionManager: *mut WSD_ENDPOINT_REFERENCE,
    pub Status: *const u16,
    pub Reason: *mut WSD_LOCALIZED_STRING,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = 2;
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
pub const WSDET_INCOMING_FAULT: WSDEventType = 2;
pub const WSDET_INCOMING_MESSAGE: WSDEventType = 1;
pub const WSDET_NONE: WSDEventType = 0;
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = 4;
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = 3;
pub type WSDEventType = i32;
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
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_BYE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_BYE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_ENDPOINT_REFERENCE {
    pub Address: *const u16,
    pub ReferenceProperties: WSD_REFERENCE_PROPERTIES,
    pub ReferenceParameters: WSD_REFERENCE_PARAMETERS,
    pub PortType: *mut super::wsdxmldom::WSDXML_NAME,
    pub ServiceName: *mut super::wsdxmldom::WSDXML_NAME,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_ENDPOINT_REFERENCE_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_ENDPOINT_REFERENCE,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
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
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
impl Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_DELIVERY_MODE {
    pub Mode: *const u16,
    pub Push: *mut WSD_EVENTING_DELIVERY_MODE_PUSH,
    pub Data: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_DELIVERY_MODE_PUSH {
    pub NotifyTo: *mut WSD_ENDPOINT_REFERENCE,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxml")]
#[derive(Clone, Copy)]
pub struct WSD_EVENTING_EXPIRES {
    pub Duration: *mut super::wsdxml::WSD_DURATION,
    pub DateTime: *mut super::wsdxml::WSD_DATETIME,
}
#[cfg(feature = "Win32_wsdxml")]
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
#[repr(C)]
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
#[derive(Clone, Copy)]
pub struct WSD_HANDLER_CONTEXT {
    pub Handler: PWSD_SOAP_MESSAGE_HANDLER,
    pub PVoid: *mut core::ffi::c_void,
    pub Unknown: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
impl Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut super::wsdxmldom::WSDXML_NAME,
    pub MessageID: *const u16,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_HELLO {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_HOST_METADATA {
    pub Host: *mut WSD_SERVICE_METADATA,
    pub Hosted: *mut WSD_SERVICE_METADATA_LIST,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_METADATA_SECTION {
    pub Dialect: *const u16,
    pub Identifier: *const u16,
    pub Data: *mut core::ffi::c_void,
    pub MetadataReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Location: *const u16,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_METADATA_SECTION_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_METADATA_SECTION,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_NAME_LIST {
    pub Next: *mut Self,
    pub Element: *mut super::wsdxmldom::WSDXML_NAME,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
#[derive(Clone, Copy)]
pub struct WSD_OPERATION {
    pub RequestType: *mut super::wsdxmldom::WSDXML_TYPE,
    pub ResponseType: *mut super::wsdxmldom::WSDXML_TYPE,
    pub RequestStubFunction: WSD_STUB_FUNCTION,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
impl Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
#[derive(Clone, Copy)]
pub struct WSD_PORT_TYPE {
    pub EncodedName: u32,
    pub OperationCount: u32,
    pub Operations: *mut WSD_OPERATION,
    pub ProtocolType: WSD_PROTOCOL_TYPE,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
impl Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_PROBE {
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCHES {
    pub ProbeMatch: *mut WSD_PROBE_MATCH_LIST,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_PROBE_MATCH_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_PROBE_MATCH,
}
#[cfg(feature = "Win32_wsdxmldom")]
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
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_REFERENCE_PARAMETERS {
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_REFERENCE_PROPERTIES {
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_RELATIONSHIP_METADATA {
    pub Type: *const u16,
    pub Data: *mut WSD_HOST_METADATA,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE_MATCH {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE,
    pub Types: *mut WSD_NAME_LIST,
    pub Scopes: *mut WSD_SCOPES,
    pub XAddrs: *mut WSD_URI_LIST,
    pub MetadataVersion: u64,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_RESOLVE_MATCHES {
    pub ResolveMatch: *mut WSD_RESOLVE_MATCH,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_SERVICE_METADATA {
    pub EndpointReference: *mut WSD_ENDPOINT_REFERENCE_LIST,
    pub Types: *mut WSD_NAME_LIST,
    pub ServiceId: *const u16,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_SERVICE_METADATA_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSD_SERVICE_METADATA,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT {
    pub Code: *mut WSD_SOAP_FAULT_CODE,
    pub Reason: *mut WSD_SOAP_FAULT_REASON,
    pub Node: *const u16,
    pub Role: *const u16,
    pub Detail: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT_CODE {
    pub Value: *mut super::wsdxmldom::WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[cfg(feature = "Win32_wsdxmldom")]
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
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut super::wsdxmldom::WSDXML_NAME,
    pub Subcode: *mut Self,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
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
    pub AnyHeaders: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_SOAP_MESSAGE {
    pub Header: WSD_SOAP_HEADER,
    pub Body: *mut core::ffi::c_void,
    pub BodyType: *mut super::wsdxmldom::WSDXML_TYPE,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdxmldom"))]
pub type WSD_STUB_FUNCTION = Option<unsafe extern "system" fn(server: *mut core::ffi::c_void, session: *mut core::ffi::c_void, event: *mut WSD_EVENT) -> windows_sys::core::HRESULT>;
#[repr(C)]
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase"))]
#[derive(Clone, Copy)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: windows_sys::core::HRESULT,
    pub eventHandle: super::winnt::HANDLE,
    pub messageParameters: *mut core::ffi::c_void,
    pub results: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase"))]
impl Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_THIS_DEVICE_METADATA {
    pub FriendlyName: *mut WSD_LOCALIZED_STRING_LIST,
    pub FirmwareVersion: *const u16,
    pub SerialNumber: *const u16,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_THIS_MODEL_METADATA {
    pub Manufacturer: *mut WSD_LOCALIZED_STRING_LIST,
    pub ManufacturerUrl: *const u16,
    pub ModelName: *mut WSD_LOCALIZED_STRING_LIST,
    pub ModelNumber: *const u16,
    pub ModelUrl: *const u16,
    pub PresentationUrl: *const u16,
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
impl Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_wsdxmldom")]
#[derive(Clone, Copy)]
pub struct WSD_UNKNOWN_LOOKUP {
    pub Any: *mut super::wsdxmldom::WSDXML_ELEMENT,
}
#[cfg(feature = "Win32_wsdxmldom")]
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
