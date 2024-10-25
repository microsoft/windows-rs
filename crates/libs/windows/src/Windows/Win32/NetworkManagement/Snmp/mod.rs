pub const ASN_APPLICATION: u32 = 64u32;
pub const ASN_CONSTRUCTOR: u32 = 32u32;
pub const ASN_CONTEXT: u32 = 128u32;
pub const ASN_CONTEXTSPECIFIC: u32 = 128u32;
pub const ASN_PRIMATIVE: u32 = 0u32;
pub const ASN_PRIMITIVE: u32 = 0u32;
pub const ASN_PRIVATE: u32 = 192u32;
pub const ASN_UNIVERSAL: u32 = 0u32;
pub const DEFAULT_SNMPTRAP_PORT_IPX: u32 = 36880u32;
pub const DEFAULT_SNMPTRAP_PORT_UDP: u32 = 162u32;
pub const DEFAULT_SNMP_PORT_IPX: u32 = 36879u32;
pub const DEFAULT_SNMP_PORT_UDP: u32 = 161u32;
pub const MAXOBJIDSIZE: u32 = 128u32;
pub const MAXOBJIDSTRSIZE: u32 = 1408u32;
pub const MAXVENDORINFO: u32 = 32u32;
pub const MGMCTL_SETAGENTPORT: u32 = 1u32;
pub const SNMPAPI_ALLOC_ERROR: u32 = 2u32;
pub const SNMPAPI_CONTEXT_INVALID: u32 = 3u32;
pub const SNMPAPI_CONTEXT_UNKNOWN: u32 = 4u32;
pub const SNMPAPI_ENTITY_INVALID: u32 = 5u32;
pub const SNMPAPI_ENTITY_UNKNOWN: u32 = 6u32;
pub const SNMPAPI_ERROR: u32 = 0u32;
pub const SNMPAPI_FAILURE: u32 = 0u32;
pub const SNMPAPI_HWND_INVALID: u32 = 20u32;
pub const SNMPAPI_INDEX_INVALID: u32 = 7u32;
pub const SNMPAPI_M2M_SUPPORT: u32 = 3u32;
pub const SNMPAPI_MESSAGE_INVALID: u32 = 19u32;
pub const SNMPAPI_MODE_INVALID: u32 = 16u32;
pub const SNMPAPI_NOERROR: u32 = 1u32;
pub const SNMPAPI_NOOP: u32 = 8u32;
pub const SNMPAPI_NOT_INITIALIZED: u32 = 18u32;
pub const SNMPAPI_NO_SUPPORT: u32 = 0u32;
pub const SNMPAPI_OFF: SNMP_STATUS = 0u32;
pub const SNMPAPI_OID_INVALID: u32 = 9u32;
pub const SNMPAPI_ON: SNMP_STATUS = 1u32;
pub const SNMPAPI_OPERATION_INVALID: u32 = 10u32;
pub const SNMPAPI_OTHER_ERROR: u32 = 99u32;
pub const SNMPAPI_OUTPUT_TRUNCATED: u32 = 11u32;
pub const SNMPAPI_PDU_INVALID: u32 = 12u32;
pub const SNMPAPI_SESSION_INVALID: u32 = 13u32;
pub const SNMPAPI_SIZE_INVALID: u32 = 17u32;
pub const SNMPAPI_SUCCESS: u32 = 1u32;
pub const SNMPAPI_SYNTAX_INVALID: u32 = 14u32;
pub const SNMPAPI_TL_INVALID_PARAM: u32 = 106u32;
pub const SNMPAPI_TL_IN_USE: u32 = 107u32;
pub const SNMPAPI_TL_NOT_AVAILABLE: u32 = 102u32;
pub const SNMPAPI_TL_NOT_INITIALIZED: u32 = 100u32;
pub const SNMPAPI_TL_NOT_SUPPORTED: u32 = 101u32;
pub const SNMPAPI_TL_OTHER: u32 = 199u32;
pub const SNMPAPI_TL_PDU_TOO_BIG: u32 = 109u32;
pub const SNMPAPI_TL_RESOURCE_ERROR: u32 = 103u32;
pub const SNMPAPI_TL_SRC_INVALID: u32 = 105u32;
pub const SNMPAPI_TL_TIMEOUT: u32 = 108u32;
pub const SNMPAPI_TL_UNDELIVERABLE: u32 = 104u32;
pub const SNMPAPI_TRANSLATED: SNMP_API_TRANSLATE_MODE = 0u32;
pub const SNMPAPI_UNTRANSLATED_V1: SNMP_API_TRANSLATE_MODE = 1u32;
pub const SNMPAPI_UNTRANSLATED_V2: SNMP_API_TRANSLATE_MODE = 2u32;
pub const SNMPAPI_V1_SUPPORT: u32 = 1u32;
pub const SNMPAPI_V2_SUPPORT: u32 = 2u32;
pub const SNMPAPI_VBL_INVALID: u32 = 15u32;
pub const SNMPLISTEN_ALL_ADDR: u32 = 1u32;
pub const SNMPLISTEN_USEENTITY_ADDR: u32 = 0u32;
pub const SNMP_ACCESS_NONE: u32 = 0u32;
pub const SNMP_ACCESS_NOTIFY: u32 = 1u32;
pub const SNMP_ACCESS_READ_CREATE: u32 = 4u32;
pub const SNMP_ACCESS_READ_ONLY: u32 = 2u32;
pub const SNMP_ACCESS_READ_WRITE: u32 = 3u32;
pub const SNMP_AUTHAPI_INVALID_MSG_TYPE: u32 = 31u32;
pub const SNMP_AUTHAPI_INVALID_VERSION: u32 = 30u32;
pub const SNMP_AUTHAPI_TRIV_AUTH_FAILED: u32 = 32u32;
pub const SNMP_BERAPI_INVALID_LENGTH: u32 = 10u32;
pub const SNMP_BERAPI_INVALID_OBJELEM: u32 = 14u32;
pub const SNMP_BERAPI_INVALID_TAG: u32 = 11u32;
pub const SNMP_BERAPI_OVERFLOW: u32 = 12u32;
pub const SNMP_BERAPI_SHORT_BUFFER: u32 = 13u32;
pub const SNMP_ERRORSTATUS_AUTHORIZATIONERROR: SNMP_ERROR_STATUS = 16u32;
pub const SNMP_ERRORSTATUS_BADVALUE: SNMP_ERROR_STATUS = 3u32;
pub const SNMP_ERRORSTATUS_COMMITFAILED: SNMP_ERROR_STATUS = 14u32;
pub const SNMP_ERRORSTATUS_GENERR: SNMP_ERROR_STATUS = 5u32;
pub const SNMP_ERRORSTATUS_INCONSISTENTNAME: SNMP_ERROR_STATUS = 18u32;
pub const SNMP_ERRORSTATUS_INCONSISTENTVALUE: SNMP_ERROR_STATUS = 12u32;
pub const SNMP_ERRORSTATUS_NOACCESS: SNMP_ERROR_STATUS = 6u32;
pub const SNMP_ERRORSTATUS_NOCREATION: SNMP_ERROR_STATUS = 11u32;
pub const SNMP_ERRORSTATUS_NOERROR: SNMP_ERROR_STATUS = 0u32;
pub const SNMP_ERRORSTATUS_NOSUCHNAME: SNMP_ERROR_STATUS = 2u32;
pub const SNMP_ERRORSTATUS_NOTWRITABLE: SNMP_ERROR_STATUS = 17u32;
pub const SNMP_ERRORSTATUS_READONLY: SNMP_ERROR_STATUS = 4u32;
pub const SNMP_ERRORSTATUS_RESOURCEUNAVAILABLE: SNMP_ERROR_STATUS = 13u32;
pub const SNMP_ERRORSTATUS_TOOBIG: SNMP_ERROR_STATUS = 1u32;
pub const SNMP_ERRORSTATUS_UNDOFAILED: SNMP_ERROR_STATUS = 15u32;
pub const SNMP_ERRORSTATUS_WRONGENCODING: SNMP_ERROR_STATUS = 9u32;
pub const SNMP_ERRORSTATUS_WRONGLENGTH: SNMP_ERROR_STATUS = 8u32;
pub const SNMP_ERRORSTATUS_WRONGTYPE: SNMP_ERROR_STATUS = 7u32;
pub const SNMP_ERRORSTATUS_WRONGVALUE: SNMP_ERROR_STATUS = 10u32;
pub const SNMP_ERROR_AUTHORIZATIONERROR: SNMP_ERROR = 16u32;
pub const SNMP_ERROR_BADVALUE: SNMP_ERROR = 3u32;
pub const SNMP_ERROR_COMMITFAILED: SNMP_ERROR = 14u32;
pub const SNMP_ERROR_GENERR: SNMP_ERROR = 5u32;
pub const SNMP_ERROR_INCONSISTENTNAME: SNMP_ERROR = 18u32;
pub const SNMP_ERROR_INCONSISTENTVALUE: SNMP_ERROR = 12u32;
pub const SNMP_ERROR_NOACCESS: SNMP_ERROR = 6u32;
pub const SNMP_ERROR_NOCREATION: SNMP_ERROR = 11u32;
pub const SNMP_ERROR_NOERROR: SNMP_ERROR = 0u32;
pub const SNMP_ERROR_NOSUCHNAME: SNMP_ERROR = 2u32;
pub const SNMP_ERROR_NOTWRITABLE: SNMP_ERROR = 17u32;
pub const SNMP_ERROR_READONLY: SNMP_ERROR = 4u32;
pub const SNMP_ERROR_RESOURCEUNAVAILABLE: SNMP_ERROR = 13u32;
pub const SNMP_ERROR_TOOBIG: SNMP_ERROR = 1u32;
pub const SNMP_ERROR_UNDOFAILED: SNMP_ERROR = 15u32;
pub const SNMP_ERROR_WRONGENCODING: SNMP_ERROR = 9u32;
pub const SNMP_ERROR_WRONGLENGTH: SNMP_ERROR = 8u32;
pub const SNMP_ERROR_WRONGTYPE: SNMP_ERROR = 7u32;
pub const SNMP_ERROR_WRONGVALUE: SNMP_ERROR = 10u32;
pub const SNMP_EXTENSION_GET: SNMP_EXTENSION_REQUEST_TYPE = 160u32;
pub const SNMP_EXTENSION_GET_NEXT: SNMP_EXTENSION_REQUEST_TYPE = 161u32;
pub const SNMP_EXTENSION_SET_CLEANUP: SNMP_EXTENSION_REQUEST_TYPE = 226u32;
pub const SNMP_EXTENSION_SET_COMMIT: SNMP_EXTENSION_REQUEST_TYPE = 163u32;
pub const SNMP_EXTENSION_SET_TEST: SNMP_EXTENSION_REQUEST_TYPE = 224u32;
pub const SNMP_EXTENSION_SET_UNDO: SNMP_EXTENSION_REQUEST_TYPE = 225u32;
pub const SNMP_GENERICTRAP_AUTHFAILURE: SNMP_GENERICTRAP = 4u32;
pub const SNMP_GENERICTRAP_COLDSTART: SNMP_GENERICTRAP = 0u32;
pub const SNMP_GENERICTRAP_EGPNEIGHLOSS: SNMP_GENERICTRAP = 5u32;
pub const SNMP_GENERICTRAP_ENTERSPECIFIC: SNMP_GENERICTRAP = 6u32;
pub const SNMP_GENERICTRAP_LINKDOWN: SNMP_GENERICTRAP = 2u32;
pub const SNMP_GENERICTRAP_LINKUP: SNMP_GENERICTRAP = 3u32;
pub const SNMP_GENERICTRAP_WARMSTART: SNMP_GENERICTRAP = 1u32;
pub const SNMP_LOG_ERROR: SNMP_LOG = 2i32;
pub const SNMP_LOG_FATAL: SNMP_LOG = 1i32;
pub const SNMP_LOG_SILENT: SNMP_LOG = 0i32;
pub const SNMP_LOG_TRACE: SNMP_LOG = 4i32;
pub const SNMP_LOG_VERBOSE: SNMP_LOG = 5i32;
pub const SNMP_LOG_WARNING: SNMP_LOG = 3i32;
pub const SNMP_MAX_OID_LEN: u32 = 128u32;
pub const SNMP_MEM_ALLOC_ERROR: u32 = 1u32;
pub const SNMP_MGMTAPI_AGAIN: u32 = 45u32;
pub const SNMP_MGMTAPI_INVALID_BUFFER: u32 = 48u32;
pub const SNMP_MGMTAPI_INVALID_CTL: u32 = 46u32;
pub const SNMP_MGMTAPI_INVALID_SESSION: u32 = 47u32;
pub const SNMP_MGMTAPI_NOTRAPS: u32 = 44u32;
pub const SNMP_MGMTAPI_SELECT_FDERRORS: u32 = 41u32;
pub const SNMP_MGMTAPI_TIMEOUT: u32 = 40u32;
pub const SNMP_MGMTAPI_TRAP_DUPINIT: u32 = 43u32;
pub const SNMP_MGMTAPI_TRAP_ERRORS: u32 = 42u32;
pub const SNMP_OUTPUT_TO_CONSOLE: SNMP_OUTPUT_LOG_TYPE = 1u32;
pub const SNMP_OUTPUT_TO_DEBUGGER: SNMP_OUTPUT_LOG_TYPE = 8u32;
pub const SNMP_OUTPUT_TO_EVENTLOG: u32 = 4u32;
pub const SNMP_OUTPUT_TO_LOGFILE: SNMP_OUTPUT_LOG_TYPE = 2u32;
pub const SNMP_PDUAPI_INVALID_ES: u32 = 21u32;
pub const SNMP_PDUAPI_INVALID_GT: u32 = 22u32;
pub const SNMP_PDUAPI_UNRECOGNIZED_PDU: u32 = 20u32;
pub const SNMP_PDU_GET: SNMP_PDU_TYPE = 160u32;
pub const SNMP_PDU_GETBULK: SNMP_PDU_TYPE = 165u32;
pub const SNMP_PDU_GETNEXT: SNMP_PDU_TYPE = 161u32;
pub const SNMP_PDU_RESPONSE: SNMP_PDU_TYPE = 162u32;
pub const SNMP_PDU_SET: SNMP_PDU_TYPE = 163u32;
pub const SNMP_PDU_TRAP: SNMP_PDU_TYPE = 167u32;
pub const SNMP_TRAP_AUTHFAIL: u32 = 4u32;
pub const SNMP_TRAP_COLDSTART: u32 = 0u32;
pub const SNMP_TRAP_EGPNEIGHBORLOSS: u32 = 5u32;
pub const SNMP_TRAP_ENTERPRISESPECIFIC: u32 = 6u32;
pub const SNMP_TRAP_LINKDOWN: u32 = 2u32;
pub const SNMP_TRAP_LINKUP: u32 = 3u32;
pub const SNMP_TRAP_WARMSTART: u32 = 1u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_API_TRANSLATE_MODE(pub u32);
impl windows_core::TypeKind for SNMP_API_TRANSLATE_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_ERROR(pub u32);
impl windows_core::TypeKind for SNMP_ERROR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_ERROR_STATUS(pub u32);
impl windows_core::TypeKind for SNMP_ERROR_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_EXTENSION_REQUEST_TYPE(pub u32);
impl windows_core::TypeKind for SNMP_EXTENSION_REQUEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_GENERICTRAP(pub u32);
impl windows_core::TypeKind for SNMP_GENERICTRAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_LOG(pub i32);
impl windows_core::TypeKind for SNMP_LOG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_OUTPUT_LOG_TYPE(pub u32);
impl windows_core::TypeKind for SNMP_OUTPUT_LOG_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_PDU_TYPE(pub u32);
impl windows_core::TypeKind for SNMP_PDU_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNMP_STATUS(pub u32);
impl windows_core::TypeKind for SNMP_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsnAny {
    pub asnType: u8,
    pub asnValue: AsnAny_0,
}
impl Default for AsnAny {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AsnAny {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union AsnAny_0 {
    pub number: i32,
    pub unsigned32: u32,
    pub counter64: u64,
    pub string: AsnOctetString,
    pub bits: AsnOctetString,
    pub object: AsnObjectIdentifier,
    pub sequence: AsnOctetString,
    pub address: AsnOctetString,
    pub counter: u32,
    pub gauge: u32,
    pub ticks: u32,
    pub arbitrary: AsnOctetString,
}
impl Default for AsnAny_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AsnAny_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsnObjectIdentifier {
    pub idLength: u32,
    pub ids: *mut u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for AsnObjectIdentifier {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsnObjectIdentifier {
    pub idLength: u32,
    pub ids: *mut u32,
}
#[cfg(target_arch = "x86")]
impl Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for AsnObjectIdentifier {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsnOctetString {
    pub stream: *mut u8,
    pub length: u32,
    pub dynamic: super::super::Foundation::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for AsnOctetString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for AsnOctetString {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsnOctetString {
    pub stream: *mut u8,
    pub length: u32,
    pub dynamic: super::super::Foundation::BOOL,
}
#[cfg(target_arch = "x86")]
impl Default for AsnOctetString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for AsnOctetString {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnmpVarBind {
    pub name: AsnObjectIdentifier,
    pub value: AsnAny,
}
impl Default for SnmpVarBind {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SnmpVarBind {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnmpVarBindList {
    pub list: *mut SnmpVarBind,
    pub len: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for SnmpVarBindList {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnmpVarBindList {
    pub list: *mut SnmpVarBind,
    pub len: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for SnmpVarBindList {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiCNTR64 {
    pub hipart: u32,
    pub lopart: u32,
}
impl Default for smiCNTR64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for smiCNTR64 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiOCTETS {
    pub len: u32,
    pub ptr: *mut u8,
}
impl Default for smiOCTETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for smiOCTETS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiOID {
    pub len: u32,
    pub ptr: *mut u32,
}
impl Default for smiOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for smiOID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiVALUE {
    pub syntax: u32,
    pub value: smiVALUE_0,
}
impl Default for smiVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for smiVALUE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union smiVALUE_0 {
    pub sNumber: i32,
    pub uNumber: u32,
    pub hNumber: smiCNTR64,
    pub string: smiOCTETS,
    pub oid: smiOID,
    pub empty: u8,
}
impl Default for smiVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for smiVALUE_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiVENDORINFO {
    pub vendorName: [i8; 64],
    pub vendorContact: [i8; 64],
    pub vendorVersionId: [i8; 32],
    pub vendorVersionDate: [i8; 32],
    pub vendorEnterprise: u32,
}
impl Default for smiVENDORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for smiVENDORINFO {
    type TypeKind = windows_core::CopyType;
}
pub type PFNSNMPCLEANUPEX = Option<unsafe extern "system" fn() -> u32>;
pub type PFNSNMPEXTENSIONCLOSE = Option<unsafe extern "system" fn()>;
pub type PFNSNMPEXTENSIONINIT = Option<unsafe extern "system" fn(dwuptimereference: u32, phsubagenttrapevent: *mut super::super::Foundation::HANDLE, pfirstsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONINITEX = Option<unsafe extern "system" fn(pnextsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONMONITOR = Option<unsafe extern "system" fn(pagentmgmtdata: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONQUERY = Option<unsafe extern "system" fn(bpdutype: u8, pvarbindlist: *mut SnmpVarBindList, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONQUERYEX = Option<unsafe extern "system" fn(nrequesttype: u32, ntransactionid: u32, pvarbindlist: *mut SnmpVarBindList, pcontextinfo: *mut AsnOctetString, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONTRAP = Option<unsafe extern "system" fn(penterpriseoid: *mut AsnObjectIdentifier, pgenerictrapid: *mut i32, pspecifictrapid: *mut i32, ptimestamp: *mut u32, pvarbindlist: *mut SnmpVarBindList) -> super::super::Foundation::BOOL>;
pub type PFNSNMPSTARTUPEX = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u32, param2: *mut u32, param3: *mut u32, param4: *mut u32) -> u32>;
pub type SNMPAPI_CALLBACK = Option<unsafe extern "system" fn(hsession: isize, hwnd: super::super::Foundation::HWND, wmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpclientdata: *mut core::ffi::c_void) -> u32>;
