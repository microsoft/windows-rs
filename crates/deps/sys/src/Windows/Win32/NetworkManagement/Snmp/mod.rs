#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn SnmpCancelMsg(session: isize, reqid: i32) -> u32;
    pub fn SnmpCleanup() -> u32;
    pub fn SnmpCleanupEx() -> u32;
    pub fn SnmpClose(session: isize) -> u32;
    pub fn SnmpContextToStr(context: isize, string: *mut smiOCTETS) -> u32;
    pub fn SnmpCountVbl(vbl: isize) -> u32;
    pub fn SnmpCreatePdu(session: isize, pdu_type: SNMP_PDU_TYPE, request_id: i32, error_status: i32, error_index: i32, varbindlist: isize) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpCreateSession(hwnd: super::super::Foundation::HWND, wmsg: u32, fcallback: SNMPAPI_CALLBACK, lpclientdata: *mut ::core::ffi::c_void) -> isize;
    pub fn SnmpCreateVbl(session: isize, name: *mut smiOID, value: *mut smiVALUE) -> isize;
    pub fn SnmpDecodeMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize, msgbufdesc: *mut smiOCTETS) -> u32;
    pub fn SnmpDeleteVb(vbl: isize, index: u32) -> u32;
    pub fn SnmpDuplicatePdu(session: isize, pdu: isize) -> isize;
    pub fn SnmpDuplicateVbl(session: isize, vbl: isize) -> isize;
    pub fn SnmpEncodeMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize, msgbufdesc: *mut smiOCTETS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpEntityToStr(entity: isize, size: u32, string: super::super::Foundation::PSTR) -> u32;
    pub fn SnmpFreeContext(context: isize) -> u32;
    pub fn SnmpFreeDescriptor(syntax: u32, descriptor: *mut smiOCTETS) -> u32;
    pub fn SnmpFreeEntity(entity: isize) -> u32;
    pub fn SnmpFreePdu(pdu: isize) -> u32;
    pub fn SnmpFreeVbl(vbl: isize) -> u32;
    pub fn SnmpGetLastError(session: isize) -> u32;
    pub fn SnmpGetPduData(pdu: isize, pdu_type: *mut SNMP_PDU_TYPE, request_id: *mut i32, error_status: *mut SNMP_ERROR, error_index: *mut i32, varbindlist: *mut isize) -> u32;
    pub fn SnmpGetRetransmitMode(nretransmitmode: *mut SNMP_STATUS) -> u32;
    pub fn SnmpGetRetry(hentity: isize, npolicyretry: *mut u32, nactualretry: *mut u32) -> u32;
    pub fn SnmpGetTimeout(hentity: isize, npolicytimeout: *mut u32, nactualtimeout: *mut u32) -> u32;
    pub fn SnmpGetTranslateMode(ntranslatemode: *mut SNMP_API_TRANSLATE_MODE) -> u32;
    pub fn SnmpGetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpGetVendorInfo(vendorinfo: *mut smiVENDORINFO) -> u32;
    pub fn SnmpListen(hentity: isize, lstatus: SNMP_STATUS) -> u32;
    pub fn SnmpListenEx(hentity: isize, lstatus: u32, nuseentityaddr: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrClose(session: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrCtl(session: *mut ::core::ffi::c_void, dwctlcode: u32, lpvinbuffer: *mut ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrGetTrap(enterprise: *mut AsnObjectIdentifier, ipaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrGetTrapEx(enterprise: *mut AsnObjectIdentifier, agentaddress: *mut AsnOctetString, sourceaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, community: *mut AsnOctetString, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrOidToStr(oid: *mut AsnObjectIdentifier, string: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrOpen(lpagentaddress: super::super::Foundation::PSTR, lpagentcommunity: super::super::Foundation::PSTR, ntimeout: i32, nretries: i32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrRequest(session: *mut ::core::ffi::c_void, requesttype: u8, variablebindings: *mut SnmpVarBindList, errorstatus: *mut SNMP_ERROR_STATUS, errorindex: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrStrToOid(string: super::super::Foundation::PSTR, oid: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrTrapListen(phtrapavailable: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    pub fn SnmpOidCompare(xoid: *mut smiOID, yoid: *mut smiOID, maxlen: u32, result: *mut i32) -> u32;
    pub fn SnmpOidCopy(srcoid: *mut smiOID, dstoid: *mut smiOID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpOidToStr(srcoid: *const smiOID, size: u32, string: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpOpen(hwnd: super::super::Foundation::HWND, wmsg: u32) -> isize;
    pub fn SnmpRecvMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize) -> u32;
    pub fn SnmpRegister(session: isize, srcentity: isize, dstentity: isize, context: isize, notification: *mut smiOID, state: SNMP_STATUS) -> u32;
    pub fn SnmpSendMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize) -> u32;
    pub fn SnmpSetPduData(pdu: isize, pdu_type: *const i32, request_id: *const i32, non_repeaters: *const i32, max_repetitions: *const i32, varbindlist: *const isize) -> u32;
    pub fn SnmpSetPort(hentity: isize, nport: u32) -> u32;
    pub fn SnmpSetRetransmitMode(nretransmitmode: SNMP_STATUS) -> u32;
    pub fn SnmpSetRetry(hentity: isize, npolicyretry: u32) -> u32;
    pub fn SnmpSetTimeout(hentity: isize, npolicytimeout: u32) -> u32;
    pub fn SnmpSetTranslateMode(ntranslatemode: SNMP_API_TRANSLATE_MODE) -> u32;
    pub fn SnmpSetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32;
    pub fn SnmpStartup(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32;
    pub fn SnmpStartupEx(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32;
    pub fn SnmpStrToContext(session: isize, string: *mut smiOCTETS) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpStrToEntity(session: isize, string: super::super::Foundation::PSTR) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpStrToOid(string: super::super::Foundation::PSTR, dstoid: *mut smiOID) -> u32;
    pub fn SnmpSvcGetUptime() -> u32;
    pub fn SnmpSvcSetLogLevel(nloglevel: SNMP_LOG);
    pub fn SnmpSvcSetLogType(nlogtype: SNMP_OUTPUT_LOG_TYPE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilAsnAnyCpy(panydst: *mut AsnAny, panysrc: *mut AsnAny) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilAsnAnyFree(pany: *mut AsnAny);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilDbgPrint(nloglevel: SNMP_LOG, szformat: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilIdsToA(ids: *mut u32, idlength: u32) -> super::super::Foundation::PSTR;
    pub fn SnmpUtilMemAlloc(nbytes: u32) -> *mut ::core::ffi::c_void;
    pub fn SnmpUtilMemFree(pmem: *mut ::core::ffi::c_void);
    pub fn SnmpUtilMemReAlloc(pmem: *mut ::core::ffi::c_void, nbytes: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsCpy(poctetsdst: *mut AsnOctetString, poctetssrc: *mut AsnOctetString) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsFree(poctets: *mut AsnOctetString);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsNCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString, nchars: u32) -> i32;
    pub fn SnmpUtilOidAppend(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32;
    pub fn SnmpUtilOidCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier) -> i32;
    pub fn SnmpUtilOidCpy(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32;
    pub fn SnmpUtilOidFree(poid: *mut AsnObjectIdentifier);
    pub fn SnmpUtilOidNCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier, nsubids: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOidToA(oid: *mut AsnObjectIdentifier) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilPrintAsnAny(pany: *mut AsnAny);
    pub fn SnmpUtilPrintOid(oid: *mut AsnObjectIdentifier);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindCpy(pvbdst: *mut SnmpVarBind, pvbsrc: *mut SnmpVarBind) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindFree(pvb: *mut SnmpVarBind);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindListCpy(pvbldst: *mut SnmpVarBindList, pvblsrc: *mut SnmpVarBindList) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindListFree(pvbl: *mut SnmpVarBindList);
}
pub const ASN_APPLICATION: u32 = 64u32;
pub const ASN_CONSTRUCTOR: u32 = 32u32;
pub const ASN_CONTEXT: u32 = 128u32;
pub const ASN_CONTEXTSPECIFIC: u32 = 128u32;
pub const ASN_PRIMATIVE: u32 = 0u32;
pub const ASN_PRIMITIVE: u32 = 0u32;
pub const ASN_PRIVATE: u32 = 192u32;
pub const ASN_UNIVERSAL: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AsnAny(i32);
#[repr(C)]
pub struct AsnObjectIdentifier(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AsnOctetString(i32);
pub const DEFAULT_SNMPTRAP_PORT_IPX: u32 = 36880u32;
pub const DEFAULT_SNMPTRAP_PORT_UDP: u32 = 162u32;
pub const DEFAULT_SNMP_PORT_IPX: u32 = 36879u32;
pub const DEFAULT_SNMP_PORT_UDP: u32 = 161u32;
pub const MAXOBJIDSIZE: u32 = 128u32;
pub const MAXOBJIDSTRSIZE: u32 = 1408u32;
pub const MAXVENDORINFO: u32 = 32u32;
pub const MGMCTL_SETAGENTPORT: u32 = 1u32;
pub type PFNSNMPCLEANUPEX = unsafe extern "system" fn() -> u32;
pub type PFNSNMPEXTENSIONCLOSE = unsafe extern "system" fn();
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONINIT = unsafe extern "system" fn(dwuptimereference: u32, phsubagenttrapevent: *mut super::super::Foundation::HANDLE, pfirstsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONINITEX = unsafe extern "system" fn(pnextsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONMONITOR = unsafe extern "system" fn(pagentmgmtdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONQUERY = unsafe extern "system" fn(bpdutype: u8, pvarbindlist: *mut SnmpVarBindList, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONQUERYEX = unsafe extern "system" fn(nrequesttype: u32, ntransactionid: u32, pvarbindlist: *mut SnmpVarBindList, pcontextinfo: *mut AsnOctetString, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONTRAP = unsafe extern "system" fn(penterpriseoid: *mut AsnObjectIdentifier, pgenerictrapid: *mut i32, pspecifictrapid: *mut i32, ptimestamp: *mut u32, pvarbindlist: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
pub type PFNSNMPSTARTUPEX = unsafe extern "system" fn(param0: *mut u32, param1: *mut u32, param2: *mut u32, param3: *mut u32, param4: *mut u32) -> u32;
pub const SNMPAPI_ALLOC_ERROR: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub type SNMPAPI_CALLBACK = unsafe extern "system" fn(hsession: isize, hwnd: super::super::Foundation::HWND, wmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpclientdata: *mut ::core::ffi::c_void) -> u32;
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
pub const SNMPAPI_OID_INVALID: u32 = 9u32;
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
#[repr(transparent)]
pub struct SNMP_API_TRANSLATE_MODE(pub u32);
pub const SNMPAPI_TRANSLATED: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(0u32);
pub const SNMPAPI_UNTRANSLATED_V1: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(1u32);
pub const SNMPAPI_UNTRANSLATED_V2: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(2u32);
pub const SNMP_AUTHAPI_INVALID_MSG_TYPE: u32 = 31u32;
pub const SNMP_AUTHAPI_INVALID_VERSION: u32 = 30u32;
pub const SNMP_AUTHAPI_TRIV_AUTH_FAILED: u32 = 32u32;
pub const SNMP_BERAPI_INVALID_LENGTH: u32 = 10u32;
pub const SNMP_BERAPI_INVALID_OBJELEM: u32 = 14u32;
pub const SNMP_BERAPI_INVALID_TAG: u32 = 11u32;
pub const SNMP_BERAPI_OVERFLOW: u32 = 12u32;
pub const SNMP_BERAPI_SHORT_BUFFER: u32 = 13u32;
#[repr(transparent)]
pub struct SNMP_ERROR(pub u32);
pub const SNMP_ERROR_NOERROR: SNMP_ERROR = SNMP_ERROR(0u32);
pub const SNMP_ERROR_TOOBIG: SNMP_ERROR = SNMP_ERROR(1u32);
pub const SNMP_ERROR_NOSUCHNAME: SNMP_ERROR = SNMP_ERROR(2u32);
pub const SNMP_ERROR_BADVALUE: SNMP_ERROR = SNMP_ERROR(3u32);
pub const SNMP_ERROR_READONLY: SNMP_ERROR = SNMP_ERROR(4u32);
pub const SNMP_ERROR_GENERR: SNMP_ERROR = SNMP_ERROR(5u32);
pub const SNMP_ERROR_NOACCESS: SNMP_ERROR = SNMP_ERROR(6u32);
pub const SNMP_ERROR_WRONGTYPE: SNMP_ERROR = SNMP_ERROR(7u32);
pub const SNMP_ERROR_WRONGLENGTH: SNMP_ERROR = SNMP_ERROR(8u32);
pub const SNMP_ERROR_WRONGENCODING: SNMP_ERROR = SNMP_ERROR(9u32);
pub const SNMP_ERROR_WRONGVALUE: SNMP_ERROR = SNMP_ERROR(10u32);
pub const SNMP_ERROR_NOCREATION: SNMP_ERROR = SNMP_ERROR(11u32);
pub const SNMP_ERROR_INCONSISTENTVALUE: SNMP_ERROR = SNMP_ERROR(12u32);
pub const SNMP_ERROR_RESOURCEUNAVAILABLE: SNMP_ERROR = SNMP_ERROR(13u32);
pub const SNMP_ERROR_COMMITFAILED: SNMP_ERROR = SNMP_ERROR(14u32);
pub const SNMP_ERROR_UNDOFAILED: SNMP_ERROR = SNMP_ERROR(15u32);
pub const SNMP_ERROR_AUTHORIZATIONERROR: SNMP_ERROR = SNMP_ERROR(16u32);
pub const SNMP_ERROR_NOTWRITABLE: SNMP_ERROR = SNMP_ERROR(17u32);
pub const SNMP_ERROR_INCONSISTENTNAME: SNMP_ERROR = SNMP_ERROR(18u32);
#[repr(transparent)]
pub struct SNMP_ERROR_STATUS(pub u32);
pub const SNMP_ERRORSTATUS_NOERROR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(0u32);
pub const SNMP_ERRORSTATUS_TOOBIG: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(1u32);
pub const SNMP_ERRORSTATUS_NOSUCHNAME: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(2u32);
pub const SNMP_ERRORSTATUS_BADVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(3u32);
pub const SNMP_ERRORSTATUS_READONLY: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(4u32);
pub const SNMP_ERRORSTATUS_GENERR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(5u32);
pub const SNMP_ERRORSTATUS_NOACCESS: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(6u32);
pub const SNMP_ERRORSTATUS_WRONGTYPE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(7u32);
pub const SNMP_ERRORSTATUS_WRONGLENGTH: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(8u32);
pub const SNMP_ERRORSTATUS_WRONGENCODING: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(9u32);
pub const SNMP_ERRORSTATUS_WRONGVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(10u32);
pub const SNMP_ERRORSTATUS_NOCREATION: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(11u32);
pub const SNMP_ERRORSTATUS_INCONSISTENTVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(12u32);
pub const SNMP_ERRORSTATUS_RESOURCEUNAVAILABLE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(13u32);
pub const SNMP_ERRORSTATUS_COMMITFAILED: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(14u32);
pub const SNMP_ERRORSTATUS_UNDOFAILED: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(15u32);
pub const SNMP_ERRORSTATUS_AUTHORIZATIONERROR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(16u32);
pub const SNMP_ERRORSTATUS_NOTWRITABLE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(17u32);
pub const SNMP_ERRORSTATUS_INCONSISTENTNAME: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(18u32);
#[repr(transparent)]
pub struct SNMP_EXTENSION_REQUEST_TYPE(pub u32);
pub const SNMP_EXTENSION_GET: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(160u32);
pub const SNMP_EXTENSION_GET_NEXT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(161u32);
pub const SNMP_EXTENSION_SET_TEST: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(224u32);
pub const SNMP_EXTENSION_SET_COMMIT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(163u32);
pub const SNMP_EXTENSION_SET_UNDO: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(225u32);
pub const SNMP_EXTENSION_SET_CLEANUP: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(226u32);
#[repr(transparent)]
pub struct SNMP_GENERICTRAP(pub u32);
pub const SNMP_GENERICTRAP_COLDSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(0u32);
pub const SNMP_GENERICTRAP_WARMSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(1u32);
pub const SNMP_GENERICTRAP_LINKDOWN: SNMP_GENERICTRAP = SNMP_GENERICTRAP(2u32);
pub const SNMP_GENERICTRAP_LINKUP: SNMP_GENERICTRAP = SNMP_GENERICTRAP(3u32);
pub const SNMP_GENERICTRAP_AUTHFAILURE: SNMP_GENERICTRAP = SNMP_GENERICTRAP(4u32);
pub const SNMP_GENERICTRAP_EGPNEIGHLOSS: SNMP_GENERICTRAP = SNMP_GENERICTRAP(5u32);
pub const SNMP_GENERICTRAP_ENTERSPECIFIC: SNMP_GENERICTRAP = SNMP_GENERICTRAP(6u32);
#[repr(transparent)]
pub struct SNMP_LOG(pub u32);
pub const SNMP_LOG_SILENT: SNMP_LOG = SNMP_LOG(0u32);
pub const SNMP_LOG_FATAL: SNMP_LOG = SNMP_LOG(1u32);
pub const SNMP_LOG_ERROR: SNMP_LOG = SNMP_LOG(2u32);
pub const SNMP_LOG_WARNING: SNMP_LOG = SNMP_LOG(3u32);
pub const SNMP_LOG_TRACE: SNMP_LOG = SNMP_LOG(4u32);
pub const SNMP_LOG_VERBOSE: SNMP_LOG = SNMP_LOG(5u32);
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
#[repr(transparent)]
pub struct SNMP_OUTPUT_LOG_TYPE(pub u32);
pub const SNMP_OUTPUT_TO_CONSOLE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(1u32);
pub const SNMP_OUTPUT_TO_LOGFILE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(2u32);
pub const SNMP_OUTPUT_TO_DEBUGGER: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(8u32);
pub const SNMP_OUTPUT_TO_EVENTLOG: u32 = 4u32;
pub const SNMP_PDUAPI_INVALID_ES: u32 = 21u32;
pub const SNMP_PDUAPI_INVALID_GT: u32 = 22u32;
pub const SNMP_PDUAPI_UNRECOGNIZED_PDU: u32 = 20u32;
#[repr(transparent)]
pub struct SNMP_PDU_TYPE(pub u32);
pub const SNMP_PDU_GET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(160u32);
pub const SNMP_PDU_GETNEXT: SNMP_PDU_TYPE = SNMP_PDU_TYPE(161u32);
pub const SNMP_PDU_RESPONSE: SNMP_PDU_TYPE = SNMP_PDU_TYPE(162u32);
pub const SNMP_PDU_SET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(163u32);
pub const SNMP_PDU_GETBULK: SNMP_PDU_TYPE = SNMP_PDU_TYPE(165u32);
pub const SNMP_PDU_TRAP: SNMP_PDU_TYPE = SNMP_PDU_TYPE(167u32);
#[repr(transparent)]
pub struct SNMP_STATUS(pub u32);
pub const SNMPAPI_ON: SNMP_STATUS = SNMP_STATUS(1u32);
pub const SNMPAPI_OFF: SNMP_STATUS = SNMP_STATUS(0u32);
pub const SNMP_TRAP_AUTHFAIL: u32 = 4u32;
pub const SNMP_TRAP_COLDSTART: u32 = 0u32;
pub const SNMP_TRAP_EGPNEIGHBORLOSS: u32 = 5u32;
pub const SNMP_TRAP_ENTERPRISESPECIFIC: u32 = 6u32;
pub const SNMP_TRAP_LINKDOWN: u32 = 2u32;
pub const SNMP_TRAP_LINKUP: u32 = 3u32;
pub const SNMP_TRAP_WARMSTART: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SnmpVarBind(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SnmpVarBindList(i32);
#[repr(C)]
pub struct smiCNTR64(i32);
#[repr(C)]
pub struct smiOCTETS(i32);
#[repr(C)]
pub struct smiOID(i32);
#[repr(C)]
pub struct smiVALUE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct smiVENDORINFO(i32);
