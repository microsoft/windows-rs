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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AsnAny {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AsnAny {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AsnAny>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AsnAny {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AsnAny {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AsnAny_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AsnAny_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AsnAny_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AsnAny_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AsnAny_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for AsnObjectIdentifier {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AsnObjectIdentifier {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AsnObjectIdentifier>()) == 0 }
    }
}
impl ::core::cmp::Eq for AsnObjectIdentifier {}
impl ::core::default::Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AsnOctetString {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AsnOctetString {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AsnOctetString>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AsnOctetString {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AsnOctetString {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_API_TRANSLATE_MODE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_TRANSLATED: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_UNTRANSLATED_V1: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_UNTRANSLATED_V2: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(2u32);
impl ::core::marker::Copy for SNMP_API_TRANSLATE_MODE {}
impl ::core::clone::Clone for SNMP_API_TRANSLATE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_API_TRANSLATE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_API_TRANSLATE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_API_TRANSLATE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_API_TRANSLATE_MODE").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_ERROR(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOERROR: SNMP_ERROR = SNMP_ERROR(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_TOOBIG: SNMP_ERROR = SNMP_ERROR(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOSUCHNAME: SNMP_ERROR = SNMP_ERROR(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_BADVALUE: SNMP_ERROR = SNMP_ERROR(3u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_READONLY: SNMP_ERROR = SNMP_ERROR(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_GENERR: SNMP_ERROR = SNMP_ERROR(5u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOACCESS: SNMP_ERROR = SNMP_ERROR(6u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_WRONGTYPE: SNMP_ERROR = SNMP_ERROR(7u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_WRONGLENGTH: SNMP_ERROR = SNMP_ERROR(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_WRONGENCODING: SNMP_ERROR = SNMP_ERROR(9u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_WRONGVALUE: SNMP_ERROR = SNMP_ERROR(10u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOCREATION: SNMP_ERROR = SNMP_ERROR(11u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_INCONSISTENTVALUE: SNMP_ERROR = SNMP_ERROR(12u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_RESOURCEUNAVAILABLE: SNMP_ERROR = SNMP_ERROR(13u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_COMMITFAILED: SNMP_ERROR = SNMP_ERROR(14u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_UNDOFAILED: SNMP_ERROR = SNMP_ERROR(15u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_AUTHORIZATIONERROR: SNMP_ERROR = SNMP_ERROR(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_NOTWRITABLE: SNMP_ERROR = SNMP_ERROR(17u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERROR_INCONSISTENTNAME: SNMP_ERROR = SNMP_ERROR(18u32);
impl ::core::marker::Copy for SNMP_ERROR {}
impl ::core::clone::Clone for SNMP_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_ERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_ERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_ERROR_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOERROR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_TOOBIG: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOSUCHNAME: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_BADVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(3u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_READONLY: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_GENERR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(5u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOACCESS: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(6u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_WRONGTYPE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(7u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_WRONGLENGTH: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_WRONGENCODING: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(9u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_WRONGVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(10u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOCREATION: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(11u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_INCONSISTENTVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(12u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_RESOURCEUNAVAILABLE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(13u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_COMMITFAILED: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(14u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_UNDOFAILED: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(15u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_AUTHORIZATIONERROR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_NOTWRITABLE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(17u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_ERRORSTATUS_INCONSISTENTNAME: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(18u32);
impl ::core::marker::Copy for SNMP_ERROR_STATUS {}
impl ::core::clone::Clone for SNMP_ERROR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_ERROR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_ERROR_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_ERROR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_ERROR_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_EXTENSION_REQUEST_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_GET: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(160u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_GET_NEXT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(161u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_SET_TEST: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(224u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_SET_COMMIT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(163u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_SET_UNDO: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(225u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_EXTENSION_SET_CLEANUP: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(226u32);
impl ::core::marker::Copy for SNMP_EXTENSION_REQUEST_TYPE {}
impl ::core::clone::Clone for SNMP_EXTENSION_REQUEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_EXTENSION_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_EXTENSION_REQUEST_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_EXTENSION_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_EXTENSION_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_GENERICTRAP(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_COLDSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_WARMSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_LINKDOWN: SNMP_GENERICTRAP = SNMP_GENERICTRAP(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_LINKUP: SNMP_GENERICTRAP = SNMP_GENERICTRAP(3u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_AUTHFAILURE: SNMP_GENERICTRAP = SNMP_GENERICTRAP(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_EGPNEIGHLOSS: SNMP_GENERICTRAP = SNMP_GENERICTRAP(5u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_GENERICTRAP_ENTERSPECIFIC: SNMP_GENERICTRAP = SNMP_GENERICTRAP(6u32);
impl ::core::marker::Copy for SNMP_GENERICTRAP {}
impl ::core::clone::Clone for SNMP_GENERICTRAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_GENERICTRAP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_GENERICTRAP {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_GENERICTRAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_GENERICTRAP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_LOG(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_SILENT: SNMP_LOG = SNMP_LOG(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_FATAL: SNMP_LOG = SNMP_LOG(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_ERROR: SNMP_LOG = SNMP_LOG(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_WARNING: SNMP_LOG = SNMP_LOG(3u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_TRACE: SNMP_LOG = SNMP_LOG(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_LOG_VERBOSE: SNMP_LOG = SNMP_LOG(5u32);
impl ::core::marker::Copy for SNMP_LOG {}
impl ::core::clone::Clone for SNMP_LOG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_LOG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_LOG {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_LOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_LOG").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_OUTPUT_LOG_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_OUTPUT_TO_CONSOLE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_OUTPUT_TO_LOGFILE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_OUTPUT_TO_DEBUGGER: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(8u32);
impl ::core::marker::Copy for SNMP_OUTPUT_LOG_TYPE {}
impl ::core::clone::Clone for SNMP_OUTPUT_LOG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_OUTPUT_LOG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_OUTPUT_LOG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_OUTPUT_LOG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_OUTPUT_LOG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_OUTPUT_TO_EVENTLOG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDUAPI_INVALID_ES: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDUAPI_INVALID_GT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDUAPI_UNRECOGNIZED_PDU: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_PDU_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_GET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(160u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_GETNEXT: SNMP_PDU_TYPE = SNMP_PDU_TYPE(161u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_RESPONSE: SNMP_PDU_TYPE = SNMP_PDU_TYPE(162u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_SET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(163u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_GETBULK: SNMP_PDU_TYPE = SNMP_PDU_TYPE(165u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMP_PDU_TRAP: SNMP_PDU_TYPE = SNMP_PDU_TYPE(167u32);
impl ::core::marker::Copy for SNMP_PDU_TYPE {}
impl ::core::clone::Clone for SNMP_PDU_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_PDU_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_PDU_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_PDU_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_PDU_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SNMP_STATUS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_ON: SNMP_STATUS = SNMP_STATUS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
pub const SNMPAPI_OFF: SNMP_STATUS = SNMP_STATUS(0u32);
impl ::core::marker::Copy for SNMP_STATUS {}
impl ::core::clone::Clone for SNMP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SNMP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SNMP_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SNMP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SNMP_STATUS").field(&self.0).finish()
    }
}
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
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpCancelMsg(session: isize, reqid: i32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpCancelMsg(session: isize, reqid: i32) -> u32;
    }
    SnmpCancelMsg(session, reqid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpCleanup() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpCleanup() -> u32;
    }
    SnmpCleanup()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpCleanupEx() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpCleanupEx() -> u32;
    }
    SnmpCleanupEx()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpClose(session: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpClose(session: isize) -> u32;
    }
    SnmpClose(session)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpContextToStr(context: isize, string: *mut smiOCTETS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpContextToStr(context: isize, string: *mut smiOCTETS) -> u32;
    }
    SnmpContextToStr(context, ::core::mem::transmute(string))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpCountVbl(vbl: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpCountVbl(vbl: isize) -> u32;
    }
    SnmpCountVbl(vbl)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpCreatePdu(session: isize, pdu_type: SNMP_PDU_TYPE, request_id: i32, error_status: i32, error_index: i32, varbindlist: isize) -> isize {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpCreatePdu(session: isize, pdu_type: SNMP_PDU_TYPE, request_id: i32, error_status: i32, error_index: i32, varbindlist: isize) -> isize;
    }
    SnmpCreatePdu(session, pdu_type, request_id, error_status, error_index, varbindlist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpCreateSession<'a, P0>(hwnd: P0, wmsg: u32, fcallback: SNMPAPI_CALLBACK, lpclientdata: *mut ::core::ffi::c_void) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpCreateSession(hwnd: super::super::Foundation::HWND, wmsg: u32, fcallback: *mut ::core::ffi::c_void, lpclientdata: *mut ::core::ffi::c_void) -> isize;
    }
    SnmpCreateSession(hwnd.into(), wmsg, ::core::mem::transmute(fcallback), ::core::mem::transmute(lpclientdata))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpCreateVbl(session: isize, name: *mut smiOID, value: *mut smiVALUE) -> isize {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpCreateVbl(session: isize, name: *mut smiOID, value: *mut smiVALUE) -> isize;
    }
    SnmpCreateVbl(session, ::core::mem::transmute(name), ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpDecodeMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize, msgbufdesc: *mut smiOCTETS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpDecodeMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize, msgbufdesc: *mut smiOCTETS) -> u32;
    }
    SnmpDecodeMsg(session, ::core::mem::transmute(srcentity), ::core::mem::transmute(dstentity), ::core::mem::transmute(context), ::core::mem::transmute(pdu), ::core::mem::transmute(msgbufdesc))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpDeleteVb(vbl: isize, index: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpDeleteVb(vbl: isize, index: u32) -> u32;
    }
    SnmpDeleteVb(vbl, index)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpDuplicatePdu(session: isize, pdu: isize) -> isize {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpDuplicatePdu(session: isize, pdu: isize) -> isize;
    }
    SnmpDuplicatePdu(session, pdu)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpDuplicateVbl(session: isize, vbl: isize) -> isize {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpDuplicateVbl(session: isize, vbl: isize) -> isize;
    }
    SnmpDuplicateVbl(session, vbl)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpEncodeMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize, msgbufdesc: *mut smiOCTETS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpEncodeMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize, msgbufdesc: *mut smiOCTETS) -> u32;
    }
    SnmpEncodeMsg(session, srcentity, dstentity, context, pdu, ::core::mem::transmute(msgbufdesc))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpEntityToStr(entity: isize, string: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpEntityToStr(entity: isize, size: u32, string: ::windows::core::PSTR) -> u32;
    }
    SnmpEntityToStr(entity, string.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(string)))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpFreeContext(context: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpFreeContext(context: isize) -> u32;
    }
    SnmpFreeContext(context)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpFreeDescriptor(syntax: u32, descriptor: *mut smiOCTETS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpFreeDescriptor(syntax: u32, descriptor: *mut smiOCTETS) -> u32;
    }
    SnmpFreeDescriptor(syntax, ::core::mem::transmute(descriptor))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpFreeEntity(entity: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpFreeEntity(entity: isize) -> u32;
    }
    SnmpFreeEntity(entity)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpFreePdu(pdu: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpFreePdu(pdu: isize) -> u32;
    }
    SnmpFreePdu(pdu)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpFreeVbl(vbl: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpFreeVbl(vbl: isize) -> u32;
    }
    SnmpFreeVbl(vbl)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpGetLastError(session: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpGetLastError(session: isize) -> u32;
    }
    SnmpGetLastError(session)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpGetPduData(pdu: isize, pdu_type: *mut SNMP_PDU_TYPE, request_id: *mut i32, error_status: *mut SNMP_ERROR, error_index: *mut i32, varbindlist: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpGetPduData(pdu: isize, pdu_type: *mut SNMP_PDU_TYPE, request_id: *mut i32, error_status: *mut SNMP_ERROR, error_index: *mut i32, varbindlist: *mut isize) -> u32;
    }
    SnmpGetPduData(pdu, ::core::mem::transmute(pdu_type), ::core::mem::transmute(request_id), ::core::mem::transmute(error_status), ::core::mem::transmute(error_index), ::core::mem::transmute(varbindlist))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpGetRetransmitMode(nretransmitmode: *mut SNMP_STATUS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpGetRetransmitMode(nretransmitmode: *mut SNMP_STATUS) -> u32;
    }
    SnmpGetRetransmitMode(::core::mem::transmute(nretransmitmode))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpGetRetry(hentity: isize, npolicyretry: *mut u32, nactualretry: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpGetRetry(hentity: isize, npolicyretry: *mut u32, nactualretry: *mut u32) -> u32;
    }
    SnmpGetRetry(hentity, ::core::mem::transmute(npolicyretry), ::core::mem::transmute(nactualretry))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpGetTimeout(hentity: isize, npolicytimeout: *mut u32, nactualtimeout: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpGetTimeout(hentity: isize, npolicytimeout: *mut u32, nactualtimeout: *mut u32) -> u32;
    }
    SnmpGetTimeout(hentity, ::core::mem::transmute(npolicytimeout), ::core::mem::transmute(nactualtimeout))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpGetTranslateMode(ntranslatemode: *mut SNMP_API_TRANSLATE_MODE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpGetTranslateMode(ntranslatemode: *mut SNMP_API_TRANSLATE_MODE) -> u32;
    }
    SnmpGetTranslateMode(::core::mem::transmute(ntranslatemode))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpGetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpGetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32;
    }
    SnmpGetVb(vbl, index, ::core::mem::transmute(name), ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpGetVendorInfo(vendorinfo: *mut smiVENDORINFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpGetVendorInfo(vendorinfo: *mut smiVENDORINFO) -> u32;
    }
    SnmpGetVendorInfo(::core::mem::transmute(vendorinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpListen(hentity: isize, lstatus: SNMP_STATUS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpListen(hentity: isize, lstatus: SNMP_STATUS) -> u32;
    }
    SnmpListen(hentity, lstatus)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpListenEx(hentity: isize, lstatus: u32, nuseentityaddr: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpListenEx(hentity: isize, lstatus: u32, nuseentityaddr: u32) -> u32;
    }
    SnmpListenEx(hentity, lstatus, nuseentityaddr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrClose(session: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrClose(session: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    SnmpMgrClose(::core::mem::transmute(session))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrCtl(session: *mut ::core::ffi::c_void, dwctlcode: u32, lpvinbuffer: *mut ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrCtl(session: *mut ::core::ffi::c_void, dwctlcode: u32, lpvinbuffer: *mut ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32) -> super::super::Foundation::BOOL;
    }
    SnmpMgrCtl(::core::mem::transmute(session), dwctlcode, ::core::mem::transmute(lpvinbuffer), cbinbuffer, ::core::mem::transmute(lpvoutbuffer), cboutbuffer, ::core::mem::transmute(lpcbbytesreturned))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrGetTrap(enterprise: *mut AsnObjectIdentifier, ipaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrGetTrap(enterprise: *mut AsnObjectIdentifier, ipaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
    }
    SnmpMgrGetTrap(::core::mem::transmute(enterprise), ::core::mem::transmute(ipaddress), ::core::mem::transmute(generictrap), ::core::mem::transmute(specifictrap), ::core::mem::transmute(timestamp), ::core::mem::transmute(variablebindings))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrGetTrapEx(enterprise: *mut AsnObjectIdentifier, agentaddress: *mut AsnOctetString, sourceaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, community: *mut AsnOctetString, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrGetTrapEx(enterprise: *mut AsnObjectIdentifier, agentaddress: *mut AsnOctetString, sourceaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, community: *mut AsnOctetString, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL;
    }
    SnmpMgrGetTrapEx(::core::mem::transmute(enterprise), ::core::mem::transmute(agentaddress), ::core::mem::transmute(sourceaddress), ::core::mem::transmute(generictrap), ::core::mem::transmute(specifictrap), ::core::mem::transmute(community), ::core::mem::transmute(timestamp), ::core::mem::transmute(variablebindings))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrOidToStr(oid: *mut AsnObjectIdentifier, string: *mut ::windows::core::PSTR) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrOidToStr(oid: *mut AsnObjectIdentifier, string: *mut ::windows::core::PSTR) -> super::super::Foundation::BOOL;
    }
    SnmpMgrOidToStr(::core::mem::transmute(oid), ::core::mem::transmute(string))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpMgrOpen<'a, P0, P1>(lpagentaddress: P0, lpagentcommunity: P1, ntimeout: i32, nretries: i32) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrOpen(lpagentaddress: ::windows::core::PCSTR, lpagentcommunity: ::windows::core::PCSTR, ntimeout: i32, nretries: i32) -> *mut ::core::ffi::c_void;
    }
    SnmpMgrOpen(lpagentaddress.into(), lpagentcommunity.into(), ntimeout, nretries)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrRequest(session: *mut ::core::ffi::c_void, requesttype: u8, variablebindings: *mut SnmpVarBindList, errorstatus: *mut SNMP_ERROR_STATUS, errorindex: *mut i32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrRequest(session: *mut ::core::ffi::c_void, requesttype: u8, variablebindings: *mut SnmpVarBindList, errorstatus: *mut SNMP_ERROR_STATUS, errorindex: *mut i32) -> i32;
    }
    SnmpMgrRequest(::core::mem::transmute(session), requesttype, ::core::mem::transmute(variablebindings), ::core::mem::transmute(errorstatus), ::core::mem::transmute(errorindex))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrStrToOid<'a, P0>(string: P0, oid: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrStrToOid(string: ::windows::core::PCSTR, oid: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL;
    }
    SnmpMgrStrToOid(string.into(), ::core::mem::transmute(oid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpMgrTrapListen(phtrapavailable: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpMgrTrapListen(phtrapavailable: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    SnmpMgrTrapListen(::core::mem::transmute(phtrapavailable))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpOidCompare(xoid: *mut smiOID, yoid: *mut smiOID, maxlen: u32, result: *mut i32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpOidCompare(xoid: *mut smiOID, yoid: *mut smiOID, maxlen: u32, result: *mut i32) -> u32;
    }
    SnmpOidCompare(::core::mem::transmute(xoid), ::core::mem::transmute(yoid), maxlen, ::core::mem::transmute(result))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpOidCopy(srcoid: *mut smiOID, dstoid: *mut smiOID) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpOidCopy(srcoid: *mut smiOID, dstoid: *mut smiOID) -> u32;
    }
    SnmpOidCopy(::core::mem::transmute(srcoid), ::core::mem::transmute(dstoid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpOidToStr(srcoid: *const smiOID, string: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpOidToStr(srcoid: *const smiOID, size: u32, string: ::windows::core::PSTR) -> u32;
    }
    SnmpOidToStr(::core::mem::transmute(srcoid), string.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(string)))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpOpen<'a, P0>(hwnd: P0, wmsg: u32) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpOpen(hwnd: super::super::Foundation::HWND, wmsg: u32) -> isize;
    }
    SnmpOpen(hwnd.into(), wmsg)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpRecvMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpRecvMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize) -> u32;
    }
    SnmpRecvMsg(session, ::core::mem::transmute(srcentity), ::core::mem::transmute(dstentity), ::core::mem::transmute(context), ::core::mem::transmute(pdu))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpRegister(session: isize, srcentity: isize, dstentity: isize, context: isize, notification: *mut smiOID, state: SNMP_STATUS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpRegister(session: isize, srcentity: isize, dstentity: isize, context: isize, notification: *mut smiOID, state: SNMP_STATUS) -> u32;
    }
    SnmpRegister(session, srcentity, dstentity, context, ::core::mem::transmute(notification), state)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSendMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSendMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize) -> u32;
    }
    SnmpSendMsg(session, srcentity, dstentity, context, pdu)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSetPduData(pdu: isize, pdu_type: *const i32, request_id: *const i32, non_repeaters: *const i32, max_repetitions: *const i32, varbindlist: *const isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSetPduData(pdu: isize, pdu_type: *const i32, request_id: *const i32, non_repeaters: *const i32, max_repetitions: *const i32, varbindlist: *const isize) -> u32;
    }
    SnmpSetPduData(pdu, ::core::mem::transmute(pdu_type), ::core::mem::transmute(request_id), ::core::mem::transmute(non_repeaters), ::core::mem::transmute(max_repetitions), ::core::mem::transmute(varbindlist))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSetPort(hentity: isize, nport: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSetPort(hentity: isize, nport: u32) -> u32;
    }
    SnmpSetPort(hentity, nport)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSetRetransmitMode(nretransmitmode: SNMP_STATUS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSetRetransmitMode(nretransmitmode: SNMP_STATUS) -> u32;
    }
    SnmpSetRetransmitMode(nretransmitmode)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSetRetry(hentity: isize, npolicyretry: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSetRetry(hentity: isize, npolicyretry: u32) -> u32;
    }
    SnmpSetRetry(hentity, npolicyretry)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSetTimeout(hentity: isize, npolicytimeout: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSetTimeout(hentity: isize, npolicytimeout: u32) -> u32;
    }
    SnmpSetTimeout(hentity, npolicytimeout)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSetTranslateMode(ntranslatemode: SNMP_API_TRANSLATE_MODE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSetTranslateMode(ntranslatemode: SNMP_API_TRANSLATE_MODE) -> u32;
    }
    SnmpSetTranslateMode(ntranslatemode)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32;
    }
    SnmpSetVb(vbl, index, ::core::mem::transmute(name), ::core::mem::transmute(value))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpStartup(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpStartup(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32;
    }
    SnmpStartup(::core::mem::transmute(nmajorversion), ::core::mem::transmute(nminorversion), ::core::mem::transmute(nlevel), ::core::mem::transmute(ntranslatemode), ::core::mem::transmute(nretransmitmode))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpStartupEx(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpStartupEx(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32;
    }
    SnmpStartupEx(::core::mem::transmute(nmajorversion), ::core::mem::transmute(nminorversion), ::core::mem::transmute(nlevel), ::core::mem::transmute(ntranslatemode), ::core::mem::transmute(nretransmitmode))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpStrToContext(session: isize, string: *mut smiOCTETS) -> isize {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpStrToContext(session: isize, string: *mut smiOCTETS) -> isize;
    }
    SnmpStrToContext(session, ::core::mem::transmute(string))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpStrToEntity<'a, P0>(session: isize, string: P0) -> isize
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpStrToEntity(session: isize, string: ::windows::core::PCSTR) -> isize;
    }
    SnmpStrToEntity(session, string.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpStrToOid<'a, P0>(string: P0, dstoid: *mut smiOID) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpStrToOid(string: ::windows::core::PCSTR, dstoid: *mut smiOID) -> u32;
    }
    SnmpStrToOid(string.into(), ::core::mem::transmute(dstoid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSvcGetUptime() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSvcGetUptime() -> u32;
    }
    SnmpSvcGetUptime()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSvcSetLogLevel(nloglevel: SNMP_LOG) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSvcSetLogLevel(nloglevel: SNMP_LOG);
    }
    SnmpSvcSetLogLevel(nloglevel)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpSvcSetLogType(nlogtype: SNMP_OUTPUT_LOG_TYPE) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpSvcSetLogType(nlogtype: SNMP_OUTPUT_LOG_TYPE);
    }
    SnmpSvcSetLogType(nlogtype)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilAsnAnyCpy(panydst: *mut AsnAny, panysrc: *mut AsnAny) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilAsnAnyCpy(panydst: *mut AsnAny, panysrc: *mut AsnAny) -> i32;
    }
    SnmpUtilAsnAnyCpy(::core::mem::transmute(panydst), ::core::mem::transmute(panysrc))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilAsnAnyFree(pany: *mut AsnAny) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilAsnAnyFree(pany: *mut AsnAny);
    }
    SnmpUtilAsnAnyFree(::core::mem::transmute(pany))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilDbgPrint<'a, P0>(nloglevel: SNMP_LOG, szformat: P0)
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilDbgPrint(nloglevel: SNMP_LOG, szformat: ::windows::core::PCSTR);
    }
    SnmpUtilDbgPrint(nloglevel, szformat.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilIdsToA(ids: *mut u32, idlength: u32) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilIdsToA(ids: *mut u32, idlength: u32) -> ::windows::core::PSTR;
    }
    SnmpUtilIdsToA(::core::mem::transmute(ids), idlength)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilMemAlloc(nbytes: u32) -> *mut ::core::ffi::c_void {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilMemAlloc(nbytes: u32) -> *mut ::core::ffi::c_void;
    }
    SnmpUtilMemAlloc(nbytes)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilMemFree(pmem: *mut ::core::ffi::c_void) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilMemFree(pmem: *mut ::core::ffi::c_void);
    }
    SnmpUtilMemFree(::core::mem::transmute(pmem))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilMemReAlloc(pmem: *mut ::core::ffi::c_void, nbytes: u32) -> *mut ::core::ffi::c_void {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilMemReAlloc(pmem: *mut ::core::ffi::c_void, nbytes: u32) -> *mut ::core::ffi::c_void;
    }
    SnmpUtilMemReAlloc(::core::mem::transmute(pmem), nbytes)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOctetsCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOctetsCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString) -> i32;
    }
    SnmpUtilOctetsCmp(::core::mem::transmute(poctets1), ::core::mem::transmute(poctets2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOctetsCpy(poctetsdst: *mut AsnOctetString, poctetssrc: *mut AsnOctetString) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOctetsCpy(poctetsdst: *mut AsnOctetString, poctetssrc: *mut AsnOctetString) -> i32;
    }
    SnmpUtilOctetsCpy(::core::mem::transmute(poctetsdst), ::core::mem::transmute(poctetssrc))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOctetsFree(poctets: *mut AsnOctetString) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOctetsFree(poctets: *mut AsnOctetString);
    }
    SnmpUtilOctetsFree(::core::mem::transmute(poctets))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilOctetsNCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString, nchars: u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOctetsNCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString, nchars: u32) -> i32;
    }
    SnmpUtilOctetsNCmp(::core::mem::transmute(poctets1), ::core::mem::transmute(poctets2), nchars)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilOidAppend(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOidAppend(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32;
    }
    SnmpUtilOidAppend(::core::mem::transmute(poiddst), ::core::mem::transmute(poidsrc))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilOidCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOidCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier) -> i32;
    }
    SnmpUtilOidCmp(::core::mem::transmute(poid1), ::core::mem::transmute(poid2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilOidCpy(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOidCpy(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32;
    }
    SnmpUtilOidCpy(::core::mem::transmute(poiddst), ::core::mem::transmute(poidsrc))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilOidFree(poid: *mut AsnObjectIdentifier) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOidFree(poid: *mut AsnObjectIdentifier);
    }
    SnmpUtilOidFree(::core::mem::transmute(poid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilOidNCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier, nsubids: u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOidNCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier, nsubids: u32) -> i32;
    }
    SnmpUtilOidNCmp(::core::mem::transmute(poid1), ::core::mem::transmute(poid2), nsubids)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilOidToA(oid: *mut AsnObjectIdentifier) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilOidToA(oid: *mut AsnObjectIdentifier) -> ::windows::core::PSTR;
    }
    SnmpUtilOidToA(::core::mem::transmute(oid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilPrintAsnAny(pany: *mut AsnAny) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilPrintAsnAny(pany: *mut AsnAny);
    }
    SnmpUtilPrintAsnAny(::core::mem::transmute(pany))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`*"]
#[inline]
pub unsafe fn SnmpUtilPrintOid(oid: *mut AsnObjectIdentifier) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilPrintOid(oid: *mut AsnObjectIdentifier);
    }
    SnmpUtilPrintOid(::core::mem::transmute(oid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilVarBindCpy(pvbdst: *mut SnmpVarBind, pvbsrc: *mut SnmpVarBind) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilVarBindCpy(pvbdst: *mut SnmpVarBind, pvbsrc: *mut SnmpVarBind) -> i32;
    }
    SnmpUtilVarBindCpy(::core::mem::transmute(pvbdst), ::core::mem::transmute(pvbsrc))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilVarBindFree(pvb: *mut SnmpVarBind) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilVarBindFree(pvb: *mut SnmpVarBind);
    }
    SnmpUtilVarBindFree(::core::mem::transmute(pvb))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilVarBindListCpy(pvbldst: *mut SnmpVarBindList, pvblsrc: *mut SnmpVarBindList) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilVarBindListCpy(pvbldst: *mut SnmpVarBindList, pvblsrc: *mut SnmpVarBindList) -> i32;
    }
    SnmpUtilVarBindListCpy(::core::mem::transmute(pvbldst), ::core::mem::transmute(pvblsrc))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Snmp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SnmpUtilVarBindListFree(pvbl: *mut SnmpVarBindList) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SnmpUtilVarBindListFree(pvbl: *mut SnmpVarBindList);
    }
    SnmpUtilVarBindListFree(::core::mem::transmute(pvbl))
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SnmpVarBind {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SnmpVarBind {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SnmpVarBind>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SnmpVarBind {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SnmpVarBind {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SnmpVarBindList {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SnmpVarBindList {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SnmpVarBindList>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SnmpVarBindList {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for smiCNTR64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("smiCNTR64").field("hipart", &self.hipart).field("lopart", &self.lopart).finish()
    }
}
unsafe impl ::windows::core::Abi for smiCNTR64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for smiCNTR64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<smiCNTR64>()) == 0 }
    }
}
impl ::core::cmp::Eq for smiCNTR64 {}
impl ::core::default::Default for smiCNTR64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for smiOCTETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("smiOCTETS").field("len", &self.len).field("ptr", &self.ptr).finish()
    }
}
unsafe impl ::windows::core::Abi for smiOCTETS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for smiOCTETS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<smiOCTETS>()) == 0 }
    }
}
impl ::core::cmp::Eq for smiOCTETS {}
impl ::core::default::Default for smiOCTETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for smiOID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("smiOID").field("len", &self.len).field("ptr", &self.ptr).finish()
    }
}
unsafe impl ::windows::core::Abi for smiOID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for smiOID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<smiOID>()) == 0 }
    }
}
impl ::core::cmp::Eq for smiOID {}
impl ::core::default::Default for smiOID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for smiVALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for smiVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<smiVALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for smiVALUE {}
impl ::core::default::Default for smiVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for smiVALUE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for smiVALUE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<smiVALUE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for smiVALUE_0 {}
impl ::core::default::Default for smiVALUE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for smiVENDORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("smiVENDORINFO").field("vendorName", &self.vendorName).field("vendorContact", &self.vendorContact).field("vendorVersionId", &self.vendorVersionId).field("vendorVersionDate", &self.vendorVersionDate).field("vendorEnterprise", &self.vendorEnterprise).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for smiVENDORINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for smiVENDORINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<smiVENDORINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for smiVENDORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for smiVENDORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
