#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpCancelMsg(session: isize, reqid: i32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpCleanup() -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpCleanupEx() -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpClose(session: isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpContextToStr(context: isize, string: *mut smiOCTETS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpCountVbl(vbl: isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpCreatePdu(session: isize, pdu_type: SNMP_PDU_TYPE, request_id: i32, error_status: i32, error_index: i32, varbindlist: isize) -> isize;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpCreateSession(hwnd: super::super::Foundation::HWND, wmsg: u32, fcallback: SNMPAPI_CALLBACK, lpclientdata: *mut ::core::ffi::c_void) -> isize;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpCreateVbl(session: isize, name: *mut smiOID, value: *mut smiVALUE) -> isize;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpDecodeMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize, msgbufdesc: *mut smiOCTETS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpDeleteVb(vbl: isize, index: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpDuplicatePdu(session: isize, pdu: isize) -> isize;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpDuplicateVbl(session: isize, vbl: isize) -> isize;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpEncodeMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize, msgbufdesc: *mut smiOCTETS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpEntityToStr(entity: isize, size: u32, string: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpFreeContext(context: isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpFreeDescriptor(syntax: u32, descriptor: *mut smiOCTETS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpFreeEntity(entity: isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpFreePdu(pdu: isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpFreeVbl(vbl: isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpGetLastError(session: isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpGetPduData(pdu: isize, pdu_type: *mut SNMP_PDU_TYPE, request_id: *mut i32, error_status: *mut SNMP_ERROR, error_index: *mut i32, varbindlist: *mut isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpGetRetransmitMode(nretransmitmode: *mut SNMP_STATUS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpGetRetry(hentity: isize, npolicyretry: *mut u32, nactualretry: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpGetTimeout(hentity: isize, npolicytimeout: *mut u32, nactualtimeout: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpGetTranslateMode(ntranslatemode: *mut SNMP_API_TRANSLATE_MODE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpGetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpGetVendorInfo(vendorinfo: *mut smiVENDORINFO) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpListen(hentity: isize, lstatus: SNMP_STATUS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpListenEx(hentity: isize, lstatus: u32, nuseentityaddr: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrClose(session: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrCtl(session: *mut ::core::ffi::c_void, dwctlcode: u32, lpvinbuffer: *mut ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrGetTrap(enterprise: *mut AsnObjectIdentifier, ipaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrGetTrapEx(enterprise: *mut AsnObjectIdentifier, agentaddress: *mut AsnOctetString, sourceaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, community: *mut AsnOctetString, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrOidToStr(oid: *mut AsnObjectIdentifier, string: *mut ::windows_sys::core::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpMgrOpen(lpagentaddress: ::windows_sys::core::PCSTR, lpagentcommunity: ::windows_sys::core::PCSTR, ntimeout: i32, nretries: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrRequest(session: *mut ::core::ffi::c_void, requesttype: u8, variablebindings: *mut SnmpVarBindList, errorstatus: *mut SNMP_ERROR_STATUS, errorindex: *mut i32) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrStrToOid(string: ::windows_sys::core::PCSTR, oid: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpMgrTrapListen(phtrapavailable: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpOidCompare(xoid: *mut smiOID, yoid: *mut smiOID, maxlen: u32, result: *mut i32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpOidCopy(srcoid: *mut smiOID, dstoid: *mut smiOID) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpOidToStr(srcoid: *const smiOID, size: u32, string: ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpOpen(hwnd: super::super::Foundation::HWND, wmsg: u32) -> isize;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpRecvMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpRegister(session: isize, srcentity: isize, dstentity: isize, context: isize, notification: *mut smiOID, state: SNMP_STATUS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSendMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSetPduData(pdu: isize, pdu_type: *const i32, request_id: *const i32, non_repeaters: *const i32, max_repetitions: *const i32, varbindlist: *const isize) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSetPort(hentity: isize, nport: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSetRetransmitMode(nretransmitmode: SNMP_STATUS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSetRetry(hentity: isize, npolicyretry: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSetTimeout(hentity: isize, npolicytimeout: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSetTranslateMode(ntranslatemode: SNMP_API_TRANSLATE_MODE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpStartup(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpStartupEx(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpStrToContext(session: isize, string: *mut smiOCTETS) -> isize;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpStrToEntity(session: isize, string: ::windows_sys::core::PCSTR) -> isize;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpStrToOid(string: ::windows_sys::core::PCSTR, dstoid: *mut smiOID) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSvcGetUptime() -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSvcSetLogLevel(nloglevel: SNMP_LOG);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpSvcSetLogType(nlogtype: SNMP_OUTPUT_LOG_TYPE);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilAsnAnyCpy(panydst: *mut AsnAny, panysrc: *mut AsnAny) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilAsnAnyFree(pany: *mut AsnAny);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilDbgPrint(nloglevel: SNMP_LOG, szformat: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilIdsToA(ids: *mut u32, idlength: u32) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilMemAlloc(nbytes: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilMemFree(pmem: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilMemReAlloc(pmem: *mut ::core::ffi::c_void, nbytes: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsCpy(poctetsdst: *mut AsnOctetString, poctetssrc: *mut AsnOctetString) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsFree(poctets: *mut AsnOctetString);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilOctetsNCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString, nchars: u32) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilOidAppend(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilOidCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilOidCpy(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilOidFree(poid: *mut AsnObjectIdentifier);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilOidNCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier, nsubids: u32) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilOidToA(oid: *mut AsnObjectIdentifier) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilPrintAsnAny(pany: *mut AsnAny);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
    pub fn SnmpUtilPrintOid(oid: *mut AsnObjectIdentifier);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindCpy(pvbdst: *mut SnmpVarBind, pvbsrc: *mut SnmpVarBind) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindFree(pvb: *mut SnmpVarBind);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindListCpy(pvbldst: *mut SnmpVarBindList, pvblsrc: *mut SnmpVarBindList) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SnmpUtilVarBindListFree(pvbl: *mut SnmpVarBindList);
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const ASN_APPLICATION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const ASN_CONSTRUCTOR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const ASN_CONTEXT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const ASN_CONTEXTSPECIFIC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const ASN_PRIMATIVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const ASN_PRIMITIVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const ASN_PRIVATE: u32 = 192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const ASN_UNIVERSAL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AsnAny {
    pub asnType: u8,
    pub asnValue: AsnAny_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AsnAny {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AsnAny {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AsnAny_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AsnAny_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub struct AsnObjectIdentifier {
    pub idLength: u32,
    pub ids: *mut u32,
}
impl ::core::marker::Copy for AsnObjectIdentifier {}
impl ::core::clone::Clone for AsnObjectIdentifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AsnOctetString {
    pub stream: *mut u8,
    pub length: u32,
    pub dynamic: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AsnOctetString {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AsnOctetString {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const DEFAULT_SNMPTRAP_PORT_IPX: u32 = 36880u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const DEFAULT_SNMPTRAP_PORT_UDP: u32 = 162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const DEFAULT_SNMP_PORT_IPX: u32 = 36879u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const DEFAULT_SNMP_PORT_UDP: u32 = 161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const MAXOBJIDSIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const MAXOBJIDSTRSIZE: u32 = 1408u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const MAXVENDORINFO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const MGMCTL_SETAGENTPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type PFNSNMPCLEANUPEX = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type PFNSNMPEXTENSIONCLOSE = ::core::option::Option<unsafe extern "system" fn()>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONINIT = ::core::option::Option<unsafe extern "system" fn(dwuptimereference: u32, phsubagenttrapevent: *mut super::super::Foundation::HANDLE, pfirstsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONINITEX = ::core::option::Option<unsafe extern "system" fn(pnextsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONMONITOR = ::core::option::Option<unsafe extern "system" fn(pagentmgmtdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONQUERY = ::core::option::Option<unsafe extern "system" fn(bpdutype: u8, pvarbindlist: *mut SnmpVarBindList, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONQUERYEX = ::core::option::Option<unsafe extern "system" fn(nrequesttype: u32, ntransactionid: u32, pvarbindlist: *mut SnmpVarBindList, pcontextinfo: *mut AsnOctetString, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONTRAP = ::core::option::Option<unsafe extern "system" fn(penterpriseoid: *mut AsnObjectIdentifier, pgenerictrapid: *mut i32, pspecifictrapid: *mut i32, ptimestamp: *mut u32, pvarbindlist: *mut SnmpVarBindList) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type PFNSNMPSTARTUPEX = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u32, param2: *mut u32, param3: *mut u32, param4: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_ALLOC_ERROR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SNMPAPI_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hsession: isize, hwnd: super::super::Foundation::HWND, wmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpclientdata: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_CONTEXT_INVALID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_CONTEXT_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_ENTITY_INVALID: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_ENTITY_UNKNOWN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_FAILURE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_HWND_INVALID: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_INDEX_INVALID: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_M2M_SUPPORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_MESSAGE_INVALID: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_MODE_INVALID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_NOERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_NOOP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_NOT_INITIALIZED: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_NO_SUPPORT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_OID_INVALID: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_OPERATION_INVALID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_OTHER_ERROR: u32 = 99u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_OUTPUT_TRUNCATED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_PDU_INVALID: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_SESSION_INVALID: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_SIZE_INVALID: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_SYNTAX_INVALID: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_INVALID_PARAM: u32 = 106u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_IN_USE: u32 = 107u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_NOT_AVAILABLE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_NOT_INITIALIZED: u32 = 100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_NOT_SUPPORTED: u32 = 101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_OTHER: u32 = 199u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_PDU_TOO_BIG: u32 = 109u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_RESOURCE_ERROR: u32 = 103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_SRC_INVALID: u32 = 105u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_TIMEOUT: u32 = 108u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TL_UNDELIVERABLE: u32 = 104u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_V1_SUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_V2_SUPPORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_VBL_INVALID: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPLISTEN_ALL_ADDR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPLISTEN_USEENTITY_ADDR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ACCESS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ACCESS_NOTIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ACCESS_READ_CREATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ACCESS_READ_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ACCESS_READ_WRITE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_API_TRANSLATE_MODE = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TRANSLATED: SNMP_API_TRANSLATE_MODE = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_UNTRANSLATED_V1: SNMP_API_TRANSLATE_MODE = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_UNTRANSLATED_V2: SNMP_API_TRANSLATE_MODE = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_AUTHAPI_INVALID_MSG_TYPE: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_AUTHAPI_INVALID_VERSION: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_AUTHAPI_TRIV_AUTH_FAILED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_BERAPI_INVALID_LENGTH: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_BERAPI_INVALID_OBJELEM: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_BERAPI_INVALID_TAG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_BERAPI_OVERFLOW: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_BERAPI_SHORT_BUFFER: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_ERROR = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOERROR: SNMP_ERROR = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_TOOBIG: SNMP_ERROR = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOSUCHNAME: SNMP_ERROR = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_BADVALUE: SNMP_ERROR = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_READONLY: SNMP_ERROR = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_GENERR: SNMP_ERROR = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOACCESS: SNMP_ERROR = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_WRONGTYPE: SNMP_ERROR = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_WRONGLENGTH: SNMP_ERROR = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_WRONGENCODING: SNMP_ERROR = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_WRONGVALUE: SNMP_ERROR = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOCREATION: SNMP_ERROR = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_INCONSISTENTVALUE: SNMP_ERROR = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_RESOURCEUNAVAILABLE: SNMP_ERROR = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_COMMITFAILED: SNMP_ERROR = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_UNDOFAILED: SNMP_ERROR = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_AUTHORIZATIONERROR: SNMP_ERROR = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOTWRITABLE: SNMP_ERROR = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_INCONSISTENTNAME: SNMP_ERROR = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_ERROR_STATUS = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOERROR: SNMP_ERROR_STATUS = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_TOOBIG: SNMP_ERROR_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOSUCHNAME: SNMP_ERROR_STATUS = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_BADVALUE: SNMP_ERROR_STATUS = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_READONLY: SNMP_ERROR_STATUS = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_GENERR: SNMP_ERROR_STATUS = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOACCESS: SNMP_ERROR_STATUS = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_WRONGTYPE: SNMP_ERROR_STATUS = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_WRONGLENGTH: SNMP_ERROR_STATUS = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_WRONGENCODING: SNMP_ERROR_STATUS = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_WRONGVALUE: SNMP_ERROR_STATUS = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOCREATION: SNMP_ERROR_STATUS = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_INCONSISTENTVALUE: SNMP_ERROR_STATUS = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_RESOURCEUNAVAILABLE: SNMP_ERROR_STATUS = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_COMMITFAILED: SNMP_ERROR_STATUS = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_UNDOFAILED: SNMP_ERROR_STATUS = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_AUTHORIZATIONERROR: SNMP_ERROR_STATUS = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOTWRITABLE: SNMP_ERROR_STATUS = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_INCONSISTENTNAME: SNMP_ERROR_STATUS = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_EXTENSION_REQUEST_TYPE = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_GET: SNMP_EXTENSION_REQUEST_TYPE = 160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_GET_NEXT: SNMP_EXTENSION_REQUEST_TYPE = 161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_SET_TEST: SNMP_EXTENSION_REQUEST_TYPE = 224u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_SET_COMMIT: SNMP_EXTENSION_REQUEST_TYPE = 163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_SET_UNDO: SNMP_EXTENSION_REQUEST_TYPE = 225u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_SET_CLEANUP: SNMP_EXTENSION_REQUEST_TYPE = 226u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_GENERICTRAP = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_COLDSTART: SNMP_GENERICTRAP = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_WARMSTART: SNMP_GENERICTRAP = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_LINKDOWN: SNMP_GENERICTRAP = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_LINKUP: SNMP_GENERICTRAP = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_AUTHFAILURE: SNMP_GENERICTRAP = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_EGPNEIGHLOSS: SNMP_GENERICTRAP = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_ENTERSPECIFIC: SNMP_GENERICTRAP = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_LOG = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_SILENT: SNMP_LOG = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_FATAL: SNMP_LOG = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_ERROR: SNMP_LOG = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_WARNING: SNMP_LOG = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_TRACE: SNMP_LOG = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_VERBOSE: SNMP_LOG = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MAX_OID_LEN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MEM_ALLOC_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_AGAIN: u32 = 45u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_INVALID_BUFFER: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_INVALID_CTL: u32 = 46u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_INVALID_SESSION: u32 = 47u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_NOTRAPS: u32 = 44u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_SELECT_FDERRORS: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_TIMEOUT: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_TRAP_DUPINIT: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_MGMTAPI_TRAP_ERRORS: u32 = 42u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_OUTPUT_LOG_TYPE = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_OUTPUT_TO_CONSOLE: SNMP_OUTPUT_LOG_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_OUTPUT_TO_LOGFILE: SNMP_OUTPUT_LOG_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_OUTPUT_TO_DEBUGGER: SNMP_OUTPUT_LOG_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_OUTPUT_TO_EVENTLOG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDUAPI_INVALID_ES: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDUAPI_INVALID_GT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDUAPI_UNRECOGNIZED_PDU: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_PDU_TYPE = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_GET: SNMP_PDU_TYPE = 160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_GETNEXT: SNMP_PDU_TYPE = 161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_RESPONSE: SNMP_PDU_TYPE = 162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_SET: SNMP_PDU_TYPE = 163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_GETBULK: SNMP_PDU_TYPE = 165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_TRAP: SNMP_PDU_TYPE = 167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub type SNMP_STATUS = u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_ON: SNMP_STATUS = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_OFF: SNMP_STATUS = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_TRAP_AUTHFAIL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_TRAP_COLDSTART: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_TRAP_EGPNEIGHBORLOSS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_TRAP_ENTERPRISESPECIFIC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_TRAP_LINKDOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_TRAP_LINKUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_TRAP_WARMSTART: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SnmpVarBind {
    pub name: AsnObjectIdentifier,
    pub value: AsnAny,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SnmpVarBind {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SnmpVarBind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SnmpVarBindList {
    pub list: *mut SnmpVarBind,
    pub len: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SnmpVarBindList {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SnmpVarBindList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub struct smiCNTR64 {
    pub hipart: u32,
    pub lopart: u32,
}
impl ::core::marker::Copy for smiCNTR64 {}
impl ::core::clone::Clone for smiCNTR64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub struct smiOCTETS {
    pub len: u32,
    pub ptr: *mut u8,
}
impl ::core::marker::Copy for smiOCTETS {}
impl ::core::clone::Clone for smiOCTETS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub struct smiOID {
    pub len: u32,
    pub ptr: *mut u32,
}
impl ::core::marker::Copy for smiOID {}
impl ::core::clone::Clone for smiOID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub struct smiVALUE {
    pub syntax: u32,
    pub value: smiVALUE_0,
}
impl ::core::marker::Copy for smiVALUE {}
impl ::core::clone::Clone for smiVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub union smiVALUE_0 {
    pub sNumber: i32,
    pub uNumber: u32,
    pub hNumber: smiCNTR64,
    pub string: smiOCTETS,
    pub oid: smiOID,
    pub empty: u8,
}
impl ::core::marker::Copy for smiVALUE_0 {}
impl ::core::clone::Clone for smiVALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct smiVENDORINFO {
    pub vendorName: [super::super::Foundation::CHAR; 64],
    pub vendorContact: [super::super::Foundation::CHAR; 64],
    pub vendorVersionId: [super::super::Foundation::CHAR; 32],
    pub vendorVersionDate: [super::super::Foundation::CHAR; 32],
    pub vendorEnterprise: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for smiVENDORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for smiVENDORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
