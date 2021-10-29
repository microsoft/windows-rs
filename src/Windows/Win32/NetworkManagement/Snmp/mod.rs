#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const ASN_APPLICATION: u32 = 64u32;
pub const ASN_CONSTRUCTOR: u32 = 32u32;
pub const ASN_CONTEXT: u32 = 128u32;
pub const ASN_CONTEXTSPECIFIC: u32 = 128u32;
pub const ASN_PRIMATIVE: u32 = 0u32;
pub const ASN_PRIMITIVE: u32 = 0u32;
pub const ASN_PRIVATE: u32 = 192u32;
pub const ASN_UNIVERSAL: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AsnAny {
    pub asnType: u8,
    pub asnValue: AsnAny_0,
}
#[cfg(feature = "Win32_Foundation")]
impl AsnAny {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AsnAny {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AsnAny {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AsnAny {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AsnAny {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
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
impl AsnAny_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AsnAny_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AsnAny_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AsnAny_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AsnAny_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
pub struct AsnObjectIdentifier {
    pub idLength: u32,
    pub ids: *mut u32,
}
impl AsnObjectIdentifier {}
impl ::std::default::Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for AsnObjectIdentifier {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for AsnObjectIdentifier {}
unsafe impl ::windows::runtime::Abi for AsnObjectIdentifier {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct AsnOctetString {
    pub stream: *mut u8,
    pub length: u32,
    pub dynamic: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl AsnOctetString {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AsnOctetString {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AsnOctetString {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AsnOctetString {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AsnOctetString {
    type Abi = Self;
    type DefaultType = Self;
}
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
pub type PFNSNMPEXTENSIONMONITOR = unsafe extern "system" fn(pagentmgmtdata: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONQUERY = unsafe extern "system" fn(bpdutype: u8, pvarbindlist: *mut SnmpVarBindList, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONQUERYEX = unsafe extern "system" fn(nrequesttype: u32, ntransactionid: u32, pvarbindlist: *mut SnmpVarBindList, pcontextinfo: *mut AsnOctetString, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSNMPEXTENSIONTRAP = unsafe extern "system" fn(penterpriseoid: *mut AsnObjectIdentifier, pgenerictrapid: *mut i32, pspecifictrapid: *mut i32, ptimestamp: *mut u32, pvarbindlist: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
pub type PFNSNMPSTARTUPEX = unsafe extern "system" fn(param0: *mut u32, param1: *mut u32, param2: *mut u32, param3: *mut u32, param4: *mut u32) -> u32;
pub const SNMPAPI_ALLOC_ERROR: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub type SNMPAPI_CALLBACK = unsafe extern "system" fn(hsession: isize, hwnd: super::super::Foundation::HWND, wmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpclientdata: *mut ::std::ffi::c_void) -> u32;
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SNMP_API_TRANSLATE_MODE(pub u32);
pub const SNMPAPI_TRANSLATED: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(0u32);
pub const SNMPAPI_UNTRANSLATED_V1: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(1u32);
pub const SNMPAPI_UNTRANSLATED_V2: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(2u32);
impl ::std::convert::From<u32> for SNMP_API_TRANSLATE_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_API_TRANSLATE_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_API_TRANSLATE_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_API_TRANSLATE_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_API_TRANSLATE_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_API_TRANSLATE_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_API_TRANSLATE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SNMP_AUTHAPI_INVALID_MSG_TYPE: u32 = 31u32;
pub const SNMP_AUTHAPI_INVALID_VERSION: u32 = 30u32;
pub const SNMP_AUTHAPI_TRIV_AUTH_FAILED: u32 = 32u32;
pub const SNMP_BERAPI_INVALID_LENGTH: u32 = 10u32;
pub const SNMP_BERAPI_INVALID_OBJELEM: u32 = 14u32;
pub const SNMP_BERAPI_INVALID_TAG: u32 = 11u32;
pub const SNMP_BERAPI_OVERFLOW: u32 = 12u32;
pub const SNMP_BERAPI_SHORT_BUFFER: u32 = 13u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for SNMP_ERROR {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_ERROR {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_ERROR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_ERROR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_ERROR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_ERROR {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_ERROR {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for SNMP_ERROR_STATUS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_ERROR_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_ERROR_STATUS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_ERROR_STATUS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_ERROR_STATUS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_ERROR_STATUS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_ERROR_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SNMP_EXTENSION_REQUEST_TYPE(pub u32);
pub const SNMP_EXTENSION_GET: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(160u32);
pub const SNMP_EXTENSION_GET_NEXT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(161u32);
pub const SNMP_EXTENSION_SET_TEST: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(224u32);
pub const SNMP_EXTENSION_SET_COMMIT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(163u32);
pub const SNMP_EXTENSION_SET_UNDO: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(225u32);
pub const SNMP_EXTENSION_SET_CLEANUP: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(226u32);
impl ::std::convert::From<u32> for SNMP_EXTENSION_REQUEST_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_EXTENSION_REQUEST_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_EXTENSION_REQUEST_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_EXTENSION_REQUEST_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_EXTENSION_REQUEST_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_EXTENSION_REQUEST_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_EXTENSION_REQUEST_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SNMP_GENERICTRAP(pub u32);
pub const SNMP_GENERICTRAP_COLDSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(0u32);
pub const SNMP_GENERICTRAP_WARMSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(1u32);
pub const SNMP_GENERICTRAP_LINKDOWN: SNMP_GENERICTRAP = SNMP_GENERICTRAP(2u32);
pub const SNMP_GENERICTRAP_LINKUP: SNMP_GENERICTRAP = SNMP_GENERICTRAP(3u32);
pub const SNMP_GENERICTRAP_AUTHFAILURE: SNMP_GENERICTRAP = SNMP_GENERICTRAP(4u32);
pub const SNMP_GENERICTRAP_EGPNEIGHLOSS: SNMP_GENERICTRAP = SNMP_GENERICTRAP(5u32);
pub const SNMP_GENERICTRAP_ENTERSPECIFIC: SNMP_GENERICTRAP = SNMP_GENERICTRAP(6u32);
impl ::std::convert::From<u32> for SNMP_GENERICTRAP {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_GENERICTRAP {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_GENERICTRAP {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_GENERICTRAP {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_GENERICTRAP {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_GENERICTRAP {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_GENERICTRAP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SNMP_LOG(pub u32);
pub const SNMP_LOG_SILENT: SNMP_LOG = SNMP_LOG(0u32);
pub const SNMP_LOG_FATAL: SNMP_LOG = SNMP_LOG(1u32);
pub const SNMP_LOG_ERROR: SNMP_LOG = SNMP_LOG(2u32);
pub const SNMP_LOG_WARNING: SNMP_LOG = SNMP_LOG(3u32);
pub const SNMP_LOG_TRACE: SNMP_LOG = SNMP_LOG(4u32);
pub const SNMP_LOG_VERBOSE: SNMP_LOG = SNMP_LOG(5u32);
impl ::std::convert::From<u32> for SNMP_LOG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_LOG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_LOG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_LOG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_LOG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_LOG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_LOG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SNMP_OUTPUT_LOG_TYPE(pub u32);
pub const SNMP_OUTPUT_TO_CONSOLE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(1u32);
pub const SNMP_OUTPUT_TO_LOGFILE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(2u32);
pub const SNMP_OUTPUT_TO_DEBUGGER: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(8u32);
impl ::std::convert::From<u32> for SNMP_OUTPUT_LOG_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_OUTPUT_LOG_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_OUTPUT_LOG_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_OUTPUT_LOG_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_OUTPUT_LOG_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_OUTPUT_LOG_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_OUTPUT_LOG_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SNMP_OUTPUT_TO_EVENTLOG: u32 = 4u32;
pub const SNMP_PDUAPI_INVALID_ES: u32 = 21u32;
pub const SNMP_PDUAPI_INVALID_GT: u32 = 22u32;
pub const SNMP_PDUAPI_UNRECOGNIZED_PDU: u32 = 20u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SNMP_PDU_TYPE(pub u32);
pub const SNMP_PDU_GET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(160u32);
pub const SNMP_PDU_GETNEXT: SNMP_PDU_TYPE = SNMP_PDU_TYPE(161u32);
pub const SNMP_PDU_RESPONSE: SNMP_PDU_TYPE = SNMP_PDU_TYPE(162u32);
pub const SNMP_PDU_SET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(163u32);
pub const SNMP_PDU_GETBULK: SNMP_PDU_TYPE = SNMP_PDU_TYPE(165u32);
pub const SNMP_PDU_TRAP: SNMP_PDU_TYPE = SNMP_PDU_TYPE(167u32);
impl ::std::convert::From<u32> for SNMP_PDU_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_PDU_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_PDU_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_PDU_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_PDU_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_PDU_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_PDU_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SNMP_STATUS(pub u32);
pub const SNMPAPI_ON: SNMP_STATUS = SNMP_STATUS(1u32);
pub const SNMPAPI_OFF: SNMP_STATUS = SNMP_STATUS(0u32);
impl ::std::convert::From<u32> for SNMP_STATUS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SNMP_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SNMP_STATUS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SNMP_STATUS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SNMP_STATUS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SNMP_STATUS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SNMP_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SNMP_TRAP_AUTHFAIL: u32 = 4u32;
pub const SNMP_TRAP_COLDSTART: u32 = 0u32;
pub const SNMP_TRAP_EGPNEIGHBORLOSS: u32 = 5u32;
pub const SNMP_TRAP_ENTERPRISESPECIFIC: u32 = 6u32;
pub const SNMP_TRAP_LINKDOWN: u32 = 2u32;
pub const SNMP_TRAP_LINKUP: u32 = 3u32;
pub const SNMP_TRAP_WARMSTART: u32 = 1u32;
#[inline]
pub unsafe fn SnmpCancelMsg(session: isize, reqid: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpCancelMsg(session: isize, reqid: i32) -> u32;
        }
        ::std::mem::transmute(SnmpCancelMsg(::std::mem::transmute(session), ::std::mem::transmute(reqid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpCleanup() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpCleanup() -> u32;
        }
        ::std::mem::transmute(SnmpCleanup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpCleanupEx() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpCleanupEx() -> u32;
        }
        ::std::mem::transmute(SnmpCleanupEx())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpClose(session: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpClose(session: isize) -> u32;
        }
        ::std::mem::transmute(SnmpClose(::std::mem::transmute(session)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpContextToStr(context: isize, string: *mut smiOCTETS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpContextToStr(context: isize, string: *mut smiOCTETS) -> u32;
        }
        ::std::mem::transmute(SnmpContextToStr(::std::mem::transmute(context), ::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpCountVbl(vbl: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpCountVbl(vbl: isize) -> u32;
        }
        ::std::mem::transmute(SnmpCountVbl(::std::mem::transmute(vbl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpCreatePdu(session: isize, pdu_type: SNMP_PDU_TYPE, request_id: i32, error_status: i32, error_index: i32, varbindlist: isize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpCreatePdu(session: isize, pdu_type: SNMP_PDU_TYPE, request_id: i32, error_status: i32, error_index: i32, varbindlist: isize) -> isize;
        }
        ::std::mem::transmute(SnmpCreatePdu(::std::mem::transmute(session), ::std::mem::transmute(pdu_type), ::std::mem::transmute(request_id), ::std::mem::transmute(error_status), ::std::mem::transmute(error_index), ::std::mem::transmute(varbindlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpCreateSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, wmsg: u32, fcallback: ::std::option::Option<SNMPAPI_CALLBACK>, lpclientdata: *mut ::std::ffi::c_void) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpCreateSession(hwnd: super::super::Foundation::HWND, wmsg: u32, fcallback: ::windows::runtime::RawPtr, lpclientdata: *mut ::std::ffi::c_void) -> isize;
        }
        ::std::mem::transmute(SnmpCreateSession(hwnd.into_param().abi(), ::std::mem::transmute(wmsg), ::std::mem::transmute(fcallback), ::std::mem::transmute(lpclientdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpCreateVbl(session: isize, name: *mut smiOID, value: *mut smiVALUE) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpCreateVbl(session: isize, name: *mut smiOID, value: *mut smiVALUE) -> isize;
        }
        ::std::mem::transmute(SnmpCreateVbl(::std::mem::transmute(session), ::std::mem::transmute(name), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpDecodeMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize, msgbufdesc: *mut smiOCTETS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpDecodeMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize, msgbufdesc: *mut smiOCTETS) -> u32;
        }
        ::std::mem::transmute(SnmpDecodeMsg(::std::mem::transmute(session), ::std::mem::transmute(srcentity), ::std::mem::transmute(dstentity), ::std::mem::transmute(context), ::std::mem::transmute(pdu), ::std::mem::transmute(msgbufdesc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpDeleteVb(vbl: isize, index: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpDeleteVb(vbl: isize, index: u32) -> u32;
        }
        ::std::mem::transmute(SnmpDeleteVb(::std::mem::transmute(vbl), ::std::mem::transmute(index)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpDuplicatePdu(session: isize, pdu: isize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpDuplicatePdu(session: isize, pdu: isize) -> isize;
        }
        ::std::mem::transmute(SnmpDuplicatePdu(::std::mem::transmute(session), ::std::mem::transmute(pdu)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpDuplicateVbl(session: isize, vbl: isize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpDuplicateVbl(session: isize, vbl: isize) -> isize;
        }
        ::std::mem::transmute(SnmpDuplicateVbl(::std::mem::transmute(session), ::std::mem::transmute(vbl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpEncodeMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize, msgbufdesc: *mut smiOCTETS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpEncodeMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize, msgbufdesc: *mut smiOCTETS) -> u32;
        }
        ::std::mem::transmute(SnmpEncodeMsg(::std::mem::transmute(session), ::std::mem::transmute(srcentity), ::std::mem::transmute(dstentity), ::std::mem::transmute(context), ::std::mem::transmute(pdu), ::std::mem::transmute(msgbufdesc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpEntityToStr(entity: isize, size: u32, string: super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpEntityToStr(entity: isize, size: u32, string: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(SnmpEntityToStr(::std::mem::transmute(entity), ::std::mem::transmute(size), ::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpFreeContext(context: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpFreeContext(context: isize) -> u32;
        }
        ::std::mem::transmute(SnmpFreeContext(::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpFreeDescriptor(syntax: u32, descriptor: *mut smiOCTETS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpFreeDescriptor(syntax: u32, descriptor: *mut smiOCTETS) -> u32;
        }
        ::std::mem::transmute(SnmpFreeDescriptor(::std::mem::transmute(syntax), ::std::mem::transmute(descriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpFreeEntity(entity: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpFreeEntity(entity: isize) -> u32;
        }
        ::std::mem::transmute(SnmpFreeEntity(::std::mem::transmute(entity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpFreePdu(pdu: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpFreePdu(pdu: isize) -> u32;
        }
        ::std::mem::transmute(SnmpFreePdu(::std::mem::transmute(pdu)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpFreeVbl(vbl: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpFreeVbl(vbl: isize) -> u32;
        }
        ::std::mem::transmute(SnmpFreeVbl(::std::mem::transmute(vbl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpGetLastError(session: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpGetLastError(session: isize) -> u32;
        }
        ::std::mem::transmute(SnmpGetLastError(::std::mem::transmute(session)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpGetPduData(pdu: isize, pdu_type: *mut SNMP_PDU_TYPE, request_id: *mut i32, error_status: *mut SNMP_ERROR, error_index: *mut i32, varbindlist: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpGetPduData(pdu: isize, pdu_type: *mut SNMP_PDU_TYPE, request_id: *mut i32, error_status: *mut SNMP_ERROR, error_index: *mut i32, varbindlist: *mut isize) -> u32;
        }
        ::std::mem::transmute(SnmpGetPduData(::std::mem::transmute(pdu), ::std::mem::transmute(pdu_type), ::std::mem::transmute(request_id), ::std::mem::transmute(error_status), ::std::mem::transmute(error_index), ::std::mem::transmute(varbindlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpGetRetransmitMode(nretransmitmode: *mut SNMP_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpGetRetransmitMode(nretransmitmode: *mut SNMP_STATUS) -> u32;
        }
        ::std::mem::transmute(SnmpGetRetransmitMode(::std::mem::transmute(nretransmitmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpGetRetry(hentity: isize, npolicyretry: *mut u32, nactualretry: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpGetRetry(hentity: isize, npolicyretry: *mut u32, nactualretry: *mut u32) -> u32;
        }
        ::std::mem::transmute(SnmpGetRetry(::std::mem::transmute(hentity), ::std::mem::transmute(npolicyretry), ::std::mem::transmute(nactualretry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpGetTimeout(hentity: isize, npolicytimeout: *mut u32, nactualtimeout: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpGetTimeout(hentity: isize, npolicytimeout: *mut u32, nactualtimeout: *mut u32) -> u32;
        }
        ::std::mem::transmute(SnmpGetTimeout(::std::mem::transmute(hentity), ::std::mem::transmute(npolicytimeout), ::std::mem::transmute(nactualtimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpGetTranslateMode(ntranslatemode: *mut SNMP_API_TRANSLATE_MODE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpGetTranslateMode(ntranslatemode: *mut SNMP_API_TRANSLATE_MODE) -> u32;
        }
        ::std::mem::transmute(SnmpGetTranslateMode(::std::mem::transmute(ntranslatemode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpGetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpGetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32;
        }
        ::std::mem::transmute(SnmpGetVb(::std::mem::transmute(vbl), ::std::mem::transmute(index), ::std::mem::transmute(name), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpGetVendorInfo(vendorinfo: *mut smiVENDORINFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpGetVendorInfo(vendorinfo: *mut smiVENDORINFO) -> u32;
        }
        ::std::mem::transmute(SnmpGetVendorInfo(::std::mem::transmute(vendorinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpListen(hentity: isize, lstatus: SNMP_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpListen(hentity: isize, lstatus: SNMP_STATUS) -> u32;
        }
        ::std::mem::transmute(SnmpListen(::std::mem::transmute(hentity), ::std::mem::transmute(lstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpListenEx(hentity: isize, lstatus: u32, nuseentityaddr: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpListenEx(hentity: isize, lstatus: u32, nuseentityaddr: u32) -> u32;
        }
        ::std::mem::transmute(SnmpListenEx(::std::mem::transmute(hentity), ::std::mem::transmute(lstatus), ::std::mem::transmute(nuseentityaddr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrClose(session: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrClose(session: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SnmpMgrClose(::std::mem::transmute(session)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrCtl(session: *mut ::std::ffi::c_void, dwctlcode: u32, lpvinbuffer: *mut ::std::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::std::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrCtl(session: *mut ::std::ffi::c_void, dwctlcode: u32, lpvinbuffer: *mut ::std::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::std::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SnmpMgrCtl(::std::mem::transmute(session), ::std::mem::transmute(dwctlcode), ::std::mem::transmute(lpvinbuffer), ::std::mem::transmute(cbinbuffer), ::std::mem::transmute(lpvoutbuffer), ::std::mem::transmute(cboutbuffer), ::std::mem::transmute(lpcbbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrGetTrap(enterprise: *mut AsnObjectIdentifier, ipaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrGetTrap(enterprise: *mut AsnObjectIdentifier, ipaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SnmpMgrGetTrap(::std::mem::transmute(enterprise), ::std::mem::transmute(ipaddress), ::std::mem::transmute(generictrap), ::std::mem::transmute(specifictrap), ::std::mem::transmute(timestamp), ::std::mem::transmute(variablebindings)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrGetTrapEx(enterprise: *mut AsnObjectIdentifier, agentaddress: *mut AsnOctetString, sourceaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, community: *mut AsnOctetString, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrGetTrapEx(enterprise: *mut AsnObjectIdentifier, agentaddress: *mut AsnOctetString, sourceaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, community: *mut AsnOctetString, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SnmpMgrGetTrapEx(
            ::std::mem::transmute(enterprise),
            ::std::mem::transmute(agentaddress),
            ::std::mem::transmute(sourceaddress),
            ::std::mem::transmute(generictrap),
            ::std::mem::transmute(specifictrap),
            ::std::mem::transmute(community),
            ::std::mem::transmute(timestamp),
            ::std::mem::transmute(variablebindings),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrOidToStr(oid: *mut AsnObjectIdentifier, string: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrOidToStr(oid: *mut AsnObjectIdentifier, string: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SnmpMgrOidToStr(::std::mem::transmute(oid), ::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrOpen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpagentaddress: Param0, lpagentcommunity: Param1, ntimeout: i32, nretries: i32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrOpen(lpagentaddress: super::super::Foundation::PSTR, lpagentcommunity: super::super::Foundation::PSTR, ntimeout: i32, nretries: i32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(SnmpMgrOpen(lpagentaddress.into_param().abi(), lpagentcommunity.into_param().abi(), ::std::mem::transmute(ntimeout), ::std::mem::transmute(nretries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrRequest(session: *mut ::std::ffi::c_void, requesttype: u8, variablebindings: *mut SnmpVarBindList, errorstatus: *mut SNMP_ERROR_STATUS, errorindex: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrRequest(session: *mut ::std::ffi::c_void, requesttype: u8, variablebindings: *mut SnmpVarBindList, errorstatus: *mut SNMP_ERROR_STATUS, errorindex: *mut i32) -> i32;
        }
        ::std::mem::transmute(SnmpMgrRequest(::std::mem::transmute(session), ::std::mem::transmute(requesttype), ::std::mem::transmute(variablebindings), ::std::mem::transmute(errorstatus), ::std::mem::transmute(errorindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrStrToOid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(string: Param0, oid: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrStrToOid(string: super::super::Foundation::PSTR, oid: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SnmpMgrStrToOid(string.into_param().abi(), ::std::mem::transmute(oid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrTrapListen(phtrapavailable: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpMgrTrapListen(phtrapavailable: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SnmpMgrTrapListen(::std::mem::transmute(phtrapavailable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpOidCompare(xoid: *mut smiOID, yoid: *mut smiOID, maxlen: u32, result: *mut i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpOidCompare(xoid: *mut smiOID, yoid: *mut smiOID, maxlen: u32, result: *mut i32) -> u32;
        }
        ::std::mem::transmute(SnmpOidCompare(::std::mem::transmute(xoid), ::std::mem::transmute(yoid), ::std::mem::transmute(maxlen), ::std::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpOidCopy(srcoid: *mut smiOID, dstoid: *mut smiOID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpOidCopy(srcoid: *mut smiOID, dstoid: *mut smiOID) -> u32;
        }
        ::std::mem::transmute(SnmpOidCopy(::std::mem::transmute(srcoid), ::std::mem::transmute(dstoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpOidToStr(srcoid: *const smiOID, size: u32, string: super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpOidToStr(srcoid: *const smiOID, size: u32, string: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(SnmpOidToStr(::std::mem::transmute(srcoid), ::std::mem::transmute(size), ::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpOpen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, wmsg: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpOpen(hwnd: super::super::Foundation::HWND, wmsg: u32) -> isize;
        }
        ::std::mem::transmute(SnmpOpen(hwnd.into_param().abi(), ::std::mem::transmute(wmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpRecvMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpRecvMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize) -> u32;
        }
        ::std::mem::transmute(SnmpRecvMsg(::std::mem::transmute(session), ::std::mem::transmute(srcentity), ::std::mem::transmute(dstentity), ::std::mem::transmute(context), ::std::mem::transmute(pdu)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpRegister(session: isize, srcentity: isize, dstentity: isize, context: isize, notification: *mut smiOID, state: SNMP_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpRegister(session: isize, srcentity: isize, dstentity: isize, context: isize, notification: *mut smiOID, state: SNMP_STATUS) -> u32;
        }
        ::std::mem::transmute(SnmpRegister(::std::mem::transmute(session), ::std::mem::transmute(srcentity), ::std::mem::transmute(dstentity), ::std::mem::transmute(context), ::std::mem::transmute(notification), ::std::mem::transmute(state)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSendMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSendMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize) -> u32;
        }
        ::std::mem::transmute(SnmpSendMsg(::std::mem::transmute(session), ::std::mem::transmute(srcentity), ::std::mem::transmute(dstentity), ::std::mem::transmute(context), ::std::mem::transmute(pdu)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSetPduData(pdu: isize, pdu_type: *const i32, request_id: *const i32, non_repeaters: *const i32, max_repetitions: *const i32, varbindlist: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSetPduData(pdu: isize, pdu_type: *const i32, request_id: *const i32, non_repeaters: *const i32, max_repetitions: *const i32, varbindlist: *const isize) -> u32;
        }
        ::std::mem::transmute(SnmpSetPduData(::std::mem::transmute(pdu), ::std::mem::transmute(pdu_type), ::std::mem::transmute(request_id), ::std::mem::transmute(non_repeaters), ::std::mem::transmute(max_repetitions), ::std::mem::transmute(varbindlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSetPort(hentity: isize, nport: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSetPort(hentity: isize, nport: u32) -> u32;
        }
        ::std::mem::transmute(SnmpSetPort(::std::mem::transmute(hentity), ::std::mem::transmute(nport)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSetRetransmitMode(nretransmitmode: SNMP_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSetRetransmitMode(nretransmitmode: SNMP_STATUS) -> u32;
        }
        ::std::mem::transmute(SnmpSetRetransmitMode(::std::mem::transmute(nretransmitmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSetRetry(hentity: isize, npolicyretry: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSetRetry(hentity: isize, npolicyretry: u32) -> u32;
        }
        ::std::mem::transmute(SnmpSetRetry(::std::mem::transmute(hentity), ::std::mem::transmute(npolicyretry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSetTimeout(hentity: isize, npolicytimeout: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSetTimeout(hentity: isize, npolicytimeout: u32) -> u32;
        }
        ::std::mem::transmute(SnmpSetTimeout(::std::mem::transmute(hentity), ::std::mem::transmute(npolicytimeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSetTranslateMode(ntranslatemode: SNMP_API_TRANSLATE_MODE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSetTranslateMode(ntranslatemode: SNMP_API_TRANSLATE_MODE) -> u32;
        }
        ::std::mem::transmute(SnmpSetTranslateMode(::std::mem::transmute(ntranslatemode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32;
        }
        ::std::mem::transmute(SnmpSetVb(::std::mem::transmute(vbl), ::std::mem::transmute(index), ::std::mem::transmute(name), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpStartup(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpStartup(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32;
        }
        ::std::mem::transmute(SnmpStartup(::std::mem::transmute(nmajorversion), ::std::mem::transmute(nminorversion), ::std::mem::transmute(nlevel), ::std::mem::transmute(ntranslatemode), ::std::mem::transmute(nretransmitmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpStartupEx(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpStartupEx(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32;
        }
        ::std::mem::transmute(SnmpStartupEx(::std::mem::transmute(nmajorversion), ::std::mem::transmute(nminorversion), ::std::mem::transmute(nlevel), ::std::mem::transmute(ntranslatemode), ::std::mem::transmute(nretransmitmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpStrToContext(session: isize, string: *mut smiOCTETS) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpStrToContext(session: isize, string: *mut smiOCTETS) -> isize;
        }
        ::std::mem::transmute(SnmpStrToContext(::std::mem::transmute(session), ::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpStrToEntity<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(session: isize, string: Param1) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpStrToEntity(session: isize, string: super::super::Foundation::PSTR) -> isize;
        }
        ::std::mem::transmute(SnmpStrToEntity(::std::mem::transmute(session), string.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpStrToOid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(string: Param0, dstoid: *mut smiOID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpStrToOid(string: super::super::Foundation::PSTR, dstoid: *mut smiOID) -> u32;
        }
        ::std::mem::transmute(SnmpStrToOid(string.into_param().abi(), ::std::mem::transmute(dstoid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSvcGetUptime() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSvcGetUptime() -> u32;
        }
        ::std::mem::transmute(SnmpSvcGetUptime())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSvcSetLogLevel(nloglevel: SNMP_LOG) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSvcSetLogLevel(nloglevel: SNMP_LOG);
        }
        ::std::mem::transmute(SnmpSvcSetLogLevel(::std::mem::transmute(nloglevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpSvcSetLogType(nlogtype: SNMP_OUTPUT_LOG_TYPE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpSvcSetLogType(nlogtype: SNMP_OUTPUT_LOG_TYPE);
        }
        ::std::mem::transmute(SnmpSvcSetLogType(::std::mem::transmute(nlogtype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilAsnAnyCpy(panydst: *mut AsnAny, panysrc: *mut AsnAny) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilAsnAnyCpy(panydst: *mut AsnAny, panysrc: *mut AsnAny) -> i32;
        }
        ::std::mem::transmute(SnmpUtilAsnAnyCpy(::std::mem::transmute(panydst), ::std::mem::transmute(panysrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilAsnAnyFree(pany: *mut AsnAny) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilAsnAnyFree(pany: *mut AsnAny);
        }
        ::std::mem::transmute(SnmpUtilAsnAnyFree(::std::mem::transmute(pany)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilDbgPrint<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(nloglevel: SNMP_LOG, szformat: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilDbgPrint(nloglevel: SNMP_LOG, szformat: super::super::Foundation::PSTR);
        }
        ::std::mem::transmute(SnmpUtilDbgPrint(::std::mem::transmute(nloglevel), szformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilIdsToA(ids: *mut u32, idlength: u32) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilIdsToA(ids: *mut u32, idlength: u32) -> super::super::Foundation::PSTR;
        }
        ::std::mem::transmute(SnmpUtilIdsToA(::std::mem::transmute(ids), ::std::mem::transmute(idlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilMemAlloc(nbytes: u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilMemAlloc(nbytes: u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(SnmpUtilMemAlloc(::std::mem::transmute(nbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilMemFree(pmem: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilMemFree(pmem: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(SnmpUtilMemFree(::std::mem::transmute(pmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilMemReAlloc(pmem: *mut ::std::ffi::c_void, nbytes: u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilMemReAlloc(pmem: *mut ::std::ffi::c_void, nbytes: u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(SnmpUtilMemReAlloc(::std::mem::transmute(pmem), ::std::mem::transmute(nbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOctetsCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOctetsCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString) -> i32;
        }
        ::std::mem::transmute(SnmpUtilOctetsCmp(::std::mem::transmute(poctets1), ::std::mem::transmute(poctets2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOctetsCpy(poctetsdst: *mut AsnOctetString, poctetssrc: *mut AsnOctetString) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOctetsCpy(poctetsdst: *mut AsnOctetString, poctetssrc: *mut AsnOctetString) -> i32;
        }
        ::std::mem::transmute(SnmpUtilOctetsCpy(::std::mem::transmute(poctetsdst), ::std::mem::transmute(poctetssrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOctetsFree(poctets: *mut AsnOctetString) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOctetsFree(poctets: *mut AsnOctetString);
        }
        ::std::mem::transmute(SnmpUtilOctetsFree(::std::mem::transmute(poctets)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOctetsNCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString, nchars: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOctetsNCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString, nchars: u32) -> i32;
        }
        ::std::mem::transmute(SnmpUtilOctetsNCmp(::std::mem::transmute(poctets1), ::std::mem::transmute(poctets2), ::std::mem::transmute(nchars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilOidAppend(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOidAppend(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32;
        }
        ::std::mem::transmute(SnmpUtilOidAppend(::std::mem::transmute(poiddst), ::std::mem::transmute(poidsrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilOidCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOidCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier) -> i32;
        }
        ::std::mem::transmute(SnmpUtilOidCmp(::std::mem::transmute(poid1), ::std::mem::transmute(poid2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilOidCpy(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOidCpy(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32;
        }
        ::std::mem::transmute(SnmpUtilOidCpy(::std::mem::transmute(poiddst), ::std::mem::transmute(poidsrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilOidFree(poid: *mut AsnObjectIdentifier) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOidFree(poid: *mut AsnObjectIdentifier);
        }
        ::std::mem::transmute(SnmpUtilOidFree(::std::mem::transmute(poid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilOidNCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier, nsubids: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOidNCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier, nsubids: u32) -> i32;
        }
        ::std::mem::transmute(SnmpUtilOidNCmp(::std::mem::transmute(poid1), ::std::mem::transmute(poid2), ::std::mem::transmute(nsubids)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOidToA(oid: *mut AsnObjectIdentifier) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilOidToA(oid: *mut AsnObjectIdentifier) -> super::super::Foundation::PSTR;
        }
        ::std::mem::transmute(SnmpUtilOidToA(::std::mem::transmute(oid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilPrintAsnAny(pany: *mut AsnAny) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilPrintAsnAny(pany: *mut AsnAny);
        }
        ::std::mem::transmute(SnmpUtilPrintAsnAny(::std::mem::transmute(pany)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SnmpUtilPrintOid(oid: *mut AsnObjectIdentifier) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilPrintOid(oid: *mut AsnObjectIdentifier);
        }
        ::std::mem::transmute(SnmpUtilPrintOid(::std::mem::transmute(oid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilVarBindCpy(pvbdst: *mut SnmpVarBind, pvbsrc: *mut SnmpVarBind) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilVarBindCpy(pvbdst: *mut SnmpVarBind, pvbsrc: *mut SnmpVarBind) -> i32;
        }
        ::std::mem::transmute(SnmpUtilVarBindCpy(::std::mem::transmute(pvbdst), ::std::mem::transmute(pvbsrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilVarBindFree(pvb: *mut SnmpVarBind) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilVarBindFree(pvb: *mut SnmpVarBind);
        }
        ::std::mem::transmute(SnmpUtilVarBindFree(::std::mem::transmute(pvb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilVarBindListCpy(pvbldst: *mut SnmpVarBindList, pvblsrc: *mut SnmpVarBindList) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilVarBindListCpy(pvbldst: *mut SnmpVarBindList, pvblsrc: *mut SnmpVarBindList) -> i32;
        }
        ::std::mem::transmute(SnmpUtilVarBindListCpy(::std::mem::transmute(pvbldst), ::std::mem::transmute(pvblsrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilVarBindListFree(pvbl: *mut SnmpVarBindList) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SnmpUtilVarBindListFree(pvbl: *mut SnmpVarBindList);
        }
        ::std::mem::transmute(SnmpUtilVarBindListFree(::std::mem::transmute(pvbl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SnmpVarBind {
    pub name: AsnObjectIdentifier,
    pub value: AsnAny,
}
#[cfg(feature = "Win32_Foundation")]
impl SnmpVarBind {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SnmpVarBind {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SnmpVarBind {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SnmpVarBind {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SnmpVarBind {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct SnmpVarBindList {
    pub list: *mut SnmpVarBind,
    pub len: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SnmpVarBindList {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SnmpVarBindList {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SnmpVarBindList {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SnmpVarBindList {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct smiCNTR64 {
    pub hipart: u32,
    pub lopart: u32,
}
impl smiCNTR64 {}
impl ::std::default::Default for smiCNTR64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for smiCNTR64 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("smiCNTR64").field("hipart", &self.hipart).field("lopart", &self.lopart).finish()
    }
}
impl ::std::cmp::PartialEq for smiCNTR64 {
    fn eq(&self, other: &Self) -> bool {
        self.hipart == other.hipart && self.lopart == other.lopart
    }
}
impl ::std::cmp::Eq for smiCNTR64 {}
unsafe impl ::windows::runtime::Abi for smiCNTR64 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct smiOCTETS {
    pub len: u32,
    pub ptr: *mut u8,
}
impl smiOCTETS {}
impl ::std::default::Default for smiOCTETS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for smiOCTETS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("smiOCTETS").field("len", &self.len).field("ptr", &self.ptr).finish()
    }
}
impl ::std::cmp::PartialEq for smiOCTETS {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.ptr == other.ptr
    }
}
impl ::std::cmp::Eq for smiOCTETS {}
unsafe impl ::windows::runtime::Abi for smiOCTETS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct smiOID {
    pub len: u32,
    pub ptr: *mut u32,
}
impl smiOID {}
impl ::std::default::Default for smiOID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for smiOID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("smiOID").field("len", &self.len).field("ptr", &self.ptr).finish()
    }
}
impl ::std::cmp::PartialEq for smiOID {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.ptr == other.ptr
    }
}
impl ::std::cmp::Eq for smiOID {}
unsafe impl ::windows::runtime::Abi for smiOID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct smiVALUE {
    pub syntax: u32,
    pub value: smiVALUE_0,
}
impl smiVALUE {}
impl ::std::default::Default for smiVALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for smiVALUE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for smiVALUE {}
unsafe impl ::windows::runtime::Abi for smiVALUE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union smiVALUE_0 {
    pub sNumber: i32,
    pub uNumber: u32,
    pub hNumber: smiCNTR64,
    pub string: smiOCTETS,
    pub oid: smiOID,
    pub empty: u8,
}
impl smiVALUE_0 {}
impl ::std::default::Default for smiVALUE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for smiVALUE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for smiVALUE_0 {}
unsafe impl ::windows::runtime::Abi for smiVALUE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct smiVENDORINFO {
    pub vendorName: [super::super::Foundation::CHAR; 64],
    pub vendorContact: [super::super::Foundation::CHAR; 64],
    pub vendorVersionId: [super::super::Foundation::CHAR; 32],
    pub vendorVersionDate: [super::super::Foundation::CHAR; 32],
    pub vendorEnterprise: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl smiVENDORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for smiVENDORINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for smiVENDORINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("smiVENDORINFO").field("vendorName", &self.vendorName).field("vendorContact", &self.vendorContact).field("vendorVersionId", &self.vendorVersionId).field("vendorVersionDate", &self.vendorVersionDate).field("vendorEnterprise", &self.vendorEnterprise).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for smiVENDORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.vendorName == other.vendorName && self.vendorContact == other.vendorContact && self.vendorVersionId == other.vendorVersionId && self.vendorVersionDate == other.vendorVersionDate && self.vendorEnterprise == other.vendorEnterprise
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for smiVENDORINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for smiVENDORINFO {
    type Abi = Self;
    type DefaultType = Self;
}
