#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DBGPRINT = unsafe extern "system" fn(format: super::super::Foundation::PSTR) -> u32;
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DEREFERENCECONNECTION = unsafe extern "system" fn(primaryconnection: *mut ldap, connectiontodereference: *mut ldap) -> u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LAPI_MAJOR_VER1: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LAPI_MINOR_VER1: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LBER_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LBER_ERROR: i32 = -1i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LBER_TRANSLATE_STRINGS: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LBER_USE_DER: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LBER_USE_INDEFINITE_LEN: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct LDAPAPIFeatureInfoA {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: super::super::Foundation::PSTR,
    pub ldapaif_version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl LDAPAPIFeatureInfoA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPAPIFeatureInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPAPIFeatureInfoA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LDAPAPIFeatureInfoA").field("ldapaif_info_version", &self.ldapaif_info_version).field("ldapaif_name", &self.ldapaif_name).field("ldapaif_version", &self.ldapaif_version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPAPIFeatureInfoA {
    fn eq(&self, other: &Self) -> bool {
        self.ldapaif_info_version == other.ldapaif_info_version && self.ldapaif_name == other.ldapaif_name && self.ldapaif_version == other.ldapaif_version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPAPIFeatureInfoA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LDAPAPIFeatureInfoA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct LDAPAPIFeatureInfoW {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: super::super::Foundation::PWSTR,
    pub ldapaif_version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl LDAPAPIFeatureInfoW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPAPIFeatureInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPAPIFeatureInfoW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LDAPAPIFeatureInfoW").field("ldapaif_info_version", &self.ldapaif_info_version).field("ldapaif_name", &self.ldapaif_name).field("ldapaif_version", &self.ldapaif_version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPAPIFeatureInfoW {
    fn eq(&self, other: &Self) -> bool {
        self.ldapaif_info_version == other.ldapaif_info_version && self.ldapaif_name == other.ldapaif_name && self.ldapaif_version == other.ldapaif_version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPAPIFeatureInfoW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LDAPAPIFeatureInfoW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct LDAPMessage {
    pub lm_msgid: u32,
    pub lm_msgtype: u32,
    pub lm_ber: *mut ::core::ffi::c_void,
    pub lm_chain: *mut LDAPMessage,
    pub lm_next: *mut LDAPMessage,
    pub lm_time: u32,
    pub Connection: *mut ldap,
    pub Request: *mut ::core::ffi::c_void,
    pub lm_returncode: u32,
    pub lm_referral: u16,
    pub lm_chased: super::super::Foundation::BOOLEAN,
    pub lm_eom: super::super::Foundation::BOOLEAN,
    pub ConnectionReferenced: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl LDAPMessage {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPMessage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPMessage {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LDAPMessage")
            .field("lm_msgid", &self.lm_msgid)
            .field("lm_msgtype", &self.lm_msgtype)
            .field("lm_ber", &self.lm_ber)
            .field("lm_chain", &self.lm_chain)
            .field("lm_next", &self.lm_next)
            .field("lm_time", &self.lm_time)
            .field("Connection", &self.Connection)
            .field("Request", &self.Request)
            .field("lm_returncode", &self.lm_returncode)
            .field("lm_referral", &self.lm_referral)
            .field("lm_chased", &self.lm_chased)
            .field("lm_eom", &self.lm_eom)
            .field("ConnectionReferenced", &self.ConnectionReferenced)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPMessage {
    fn eq(&self, other: &Self) -> bool {
        self.lm_msgid == other.lm_msgid && self.lm_msgtype == other.lm_msgtype && self.lm_ber == other.lm_ber && self.lm_chain == other.lm_chain && self.lm_next == other.lm_next && self.lm_time == other.lm_time && self.Connection == other.Connection && self.Request == other.Request && self.lm_returncode == other.lm_returncode && self.lm_referral == other.lm_referral && self.lm_chased == other.lm_chased && self.lm_eom == other.lm_eom && self.ConnectionReferenced == other.ConnectionReferenced
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPMessage {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LDAPMessage {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_ABANDON_CMD: i32 = 80i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_ADD_CMD: i32 = 104i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_API_FEATURE_VIRTUAL_LIST_VIEW: u32 = 1001u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_API_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_API_VERSION: u32 = 2004u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_AUTH_OTHERKIND: i32 = 134i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_AUTH_SASL: i32 = 131i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_AUTH_SIMPLE: i32 = 128i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct LDAP_BERVAL {
    pub bv_len: u32,
    pub bv_val: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl LDAP_BERVAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAP_BERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAP_BERVAL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LDAP_BERVAL").field("bv_len", &self.bv_len).field("bv_val", &self.bv_val).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAP_BERVAL {
    fn eq(&self, other: &Self) -> bool {
        self.bv_len == other.bv_len && self.bv_val == other.bv_val
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAP_BERVAL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LDAP_BERVAL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_BIND_CMD: i32 = 96i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_CHASE_EXTERNAL_REFERRALS: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_CHASE_SUBORDINATE_REFERRALS: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_COMPARE_CMD: i32 = 110i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DELETE_CMD: i32 = 74i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DEREF_ALWAYS: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DEREF_FINDING: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DEREF_NEVER: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DEREF_SEARCHING: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DIRSYNC_ANCESTORS_FIRST_ORDER: u32 = 2048u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DIRSYNC_INCREMENTAL_VALUES: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DIRSYNC_OBJECT_SECURITY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DIRSYNC_PUBLIC_DATA_ONLY: u32 = 8192u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_DIRSYNC_ROPAS_DATA_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_EXTENDED_CMD: i32 = 119i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FEATURE_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_AND: u32 = 160u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_APPROX: u32 = 168u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_EQUALITY: u32 = 163u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_EXTENSIBLE: u32 = 169u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_GE: u32 = 165u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_LE: u32 = 166u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_NOT: u32 = 162u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_OR: u32 = 161u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_PRESENT: u32 = 135u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_FILTER_SUBSTRINGS: u32 = 164u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_GC_PORT: u32 = 3268u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_INVALID_CMD: u32 = 255u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_INVALID_RES: u32 = 255u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MODIFY_CMD: i32 = 102i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MODRDN_CMD: i32 = 108i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MOD_ADD: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MOD_BVALUES: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MOD_DELETE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MOD_REPLACE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MSG_ALL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MSG_ONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_MSG_RECEIVED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_NO_LIMIT: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_API_FEATURE_INFO: u32 = 21u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_API_INFO: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_AREC_EXCLUSIVE: u32 = 152u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_AUTO_RECONNECT: u32 = 145u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_CACHE_ENABLE: u32 = 15u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_CACHE_FN_PTRS: u32 = 13u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_CACHE_STRATEGY: u32 = 14u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_CHASE_REFERRALS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_CLIENT_CERTIFICATE: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_DEREF: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_DESC: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_DNS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_DNSDOMAIN_NAME: u32 = 59u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_ENCRYPT: u32 = 150u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_ERROR_NUMBER: u32 = 49u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_ERROR_STRING: u32 = 50u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_FAST_CONCURRENT_BIND: u32 = 65u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_GETDSNAME_FLAGS: u32 = 61u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_HOST_NAME: u32 = 48u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_HOST_REACHABLE: u32 = 62u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_IO_FN_PTRS: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_PING_KEEP_ALIVE: u32 = 54u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_PING_LIMIT: u32 = 56u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_PING_WAIT_TIME: u32 = 55u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_PROMPT_CREDENTIALS: u32 = 63u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_PROTOCOL_VERSION: u32 = 17u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_REBIND_ARG: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_REBIND_FN: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_REFERRALS: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_REFERRAL_CALLBACK: u32 = 112u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_REFERRAL_HOP_LIMIT: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_REF_DEREF_CONN_PER_MSG: u32 = 148u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_RESTART: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_RETURN_REFS: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_ROOTDSE_CACHE: u32 = 154u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SASL_METHOD: u32 = 151u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SCH_FLAGS: u32 = 67u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SECURITY_CONTEXT: u32 = 153u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SEND_TIMEOUT: u32 = 66u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SERVER_CERTIFICATE: u32 = 129u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SERVER_ERROR: u32 = 51u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SERVER_EXT_ERROR: u32 = 52u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SIGN: u32 = 149u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SIZELIMIT: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SOCKET_BIND_ADDRESSES: u32 = 68u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SSL: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SSL_INFO: u32 = 147u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_SSPI_FLAGS: u32 = 146u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_TCP_KEEPALIVE: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_THREAD_FN_PTRS: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_TIMELIMIT: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_TLS: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_TLS_INFO: u32 = 147u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_OPT_VERSION: u32 = 17u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_POLICYHINT_APPLY_FULLPWDPOLICY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_PORT: u32 = 389u32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct LDAP_REFERRAL_CALLBACK {
    pub SizeOfCallbacks: u32,
    pub QueryForConnection: ::core::option::Option<QUERYFORCONNECTION>,
    pub NotifyRoutine: ::core::option::Option<NOTIFYOFNEWCONNECTION>,
    pub DereferenceRoutine: ::core::option::Option<DEREFERENCECONNECTION>,
}
#[cfg(feature = "Win32_Foundation")]
impl LDAP_REFERRAL_CALLBACK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAP_REFERRAL_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAP_REFERRAL_CALLBACK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LDAP_REFERRAL_CALLBACK").field("SizeOfCallbacks", &self.SizeOfCallbacks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAP_REFERRAL_CALLBACK {
    fn eq(&self, other: &Self) -> bool {
        self.SizeOfCallbacks == other.SizeOfCallbacks && self.QueryForConnection.map(|f| f as usize) == other.QueryForConnection.map(|f| f as usize) && self.NotifyRoutine.map(|f| f as usize) == other.NotifyRoutine.map(|f| f as usize) && self.DereferenceRoutine.map(|f| f as usize) == other.DereferenceRoutine.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAP_REFERRAL_CALLBACK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LDAP_REFERRAL_CALLBACK {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_ADD: i32 = 105i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_ANY: i32 = -1i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_BIND: i32 = 97i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_COMPARE: i32 = 111i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_DELETE: i32 = 107i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_EXTENDED: i32 = 120i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_MODIFY: i32 = 103i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_MODRDN: i32 = 109i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_REFERRAL: i32 = 115i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_SEARCH_ENTRY: i32 = 100i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_SEARCH_RESULT: i32 = 101i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_RES_SESSION: i32 = 114i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LDAP_RETCODE(pub i32);
pub const LDAP_SUCCESS: LDAP_RETCODE = LDAP_RETCODE(0i32);
pub const LDAP_OPERATIONS_ERROR: LDAP_RETCODE = LDAP_RETCODE(1i32);
pub const LDAP_PROTOCOL_ERROR: LDAP_RETCODE = LDAP_RETCODE(2i32);
pub const LDAP_TIMELIMIT_EXCEEDED: LDAP_RETCODE = LDAP_RETCODE(3i32);
pub const LDAP_SIZELIMIT_EXCEEDED: LDAP_RETCODE = LDAP_RETCODE(4i32);
pub const LDAP_COMPARE_FALSE: LDAP_RETCODE = LDAP_RETCODE(5i32);
pub const LDAP_COMPARE_TRUE: LDAP_RETCODE = LDAP_RETCODE(6i32);
pub const LDAP_AUTH_METHOD_NOT_SUPPORTED: LDAP_RETCODE = LDAP_RETCODE(7i32);
pub const LDAP_STRONG_AUTH_REQUIRED: LDAP_RETCODE = LDAP_RETCODE(8i32);
pub const LDAP_REFERRAL_V2: LDAP_RETCODE = LDAP_RETCODE(9i32);
pub const LDAP_PARTIAL_RESULTS: LDAP_RETCODE = LDAP_RETCODE(9i32);
pub const LDAP_REFERRAL: LDAP_RETCODE = LDAP_RETCODE(10i32);
pub const LDAP_ADMIN_LIMIT_EXCEEDED: LDAP_RETCODE = LDAP_RETCODE(11i32);
pub const LDAP_UNAVAILABLE_CRIT_EXTENSION: LDAP_RETCODE = LDAP_RETCODE(12i32);
pub const LDAP_CONFIDENTIALITY_REQUIRED: LDAP_RETCODE = LDAP_RETCODE(13i32);
pub const LDAP_SASL_BIND_IN_PROGRESS: LDAP_RETCODE = LDAP_RETCODE(14i32);
pub const LDAP_NO_SUCH_ATTRIBUTE: LDAP_RETCODE = LDAP_RETCODE(16i32);
pub const LDAP_UNDEFINED_TYPE: LDAP_RETCODE = LDAP_RETCODE(17i32);
pub const LDAP_INAPPROPRIATE_MATCHING: LDAP_RETCODE = LDAP_RETCODE(18i32);
pub const LDAP_CONSTRAINT_VIOLATION: LDAP_RETCODE = LDAP_RETCODE(19i32);
pub const LDAP_ATTRIBUTE_OR_VALUE_EXISTS: LDAP_RETCODE = LDAP_RETCODE(20i32);
pub const LDAP_INVALID_SYNTAX: LDAP_RETCODE = LDAP_RETCODE(21i32);
pub const LDAP_NO_SUCH_OBJECT: LDAP_RETCODE = LDAP_RETCODE(32i32);
pub const LDAP_ALIAS_PROBLEM: LDAP_RETCODE = LDAP_RETCODE(33i32);
pub const LDAP_INVALID_DN_SYNTAX: LDAP_RETCODE = LDAP_RETCODE(34i32);
pub const LDAP_IS_LEAF: LDAP_RETCODE = LDAP_RETCODE(35i32);
pub const LDAP_ALIAS_DEREF_PROBLEM: LDAP_RETCODE = LDAP_RETCODE(36i32);
pub const LDAP_INAPPROPRIATE_AUTH: LDAP_RETCODE = LDAP_RETCODE(48i32);
pub const LDAP_INVALID_CREDENTIALS: LDAP_RETCODE = LDAP_RETCODE(49i32);
pub const LDAP_INSUFFICIENT_RIGHTS: LDAP_RETCODE = LDAP_RETCODE(50i32);
pub const LDAP_BUSY: LDAP_RETCODE = LDAP_RETCODE(51i32);
pub const LDAP_UNAVAILABLE: LDAP_RETCODE = LDAP_RETCODE(52i32);
pub const LDAP_UNWILLING_TO_PERFORM: LDAP_RETCODE = LDAP_RETCODE(53i32);
pub const LDAP_LOOP_DETECT: LDAP_RETCODE = LDAP_RETCODE(54i32);
pub const LDAP_SORT_CONTROL_MISSING: LDAP_RETCODE = LDAP_RETCODE(60i32);
pub const LDAP_OFFSET_RANGE_ERROR: LDAP_RETCODE = LDAP_RETCODE(61i32);
pub const LDAP_NAMING_VIOLATION: LDAP_RETCODE = LDAP_RETCODE(64i32);
pub const LDAP_OBJECT_CLASS_VIOLATION: LDAP_RETCODE = LDAP_RETCODE(65i32);
pub const LDAP_NOT_ALLOWED_ON_NONLEAF: LDAP_RETCODE = LDAP_RETCODE(66i32);
pub const LDAP_NOT_ALLOWED_ON_RDN: LDAP_RETCODE = LDAP_RETCODE(67i32);
pub const LDAP_ALREADY_EXISTS: LDAP_RETCODE = LDAP_RETCODE(68i32);
pub const LDAP_NO_OBJECT_CLASS_MODS: LDAP_RETCODE = LDAP_RETCODE(69i32);
pub const LDAP_RESULTS_TOO_LARGE: LDAP_RETCODE = LDAP_RETCODE(70i32);
pub const LDAP_AFFECTS_MULTIPLE_DSAS: LDAP_RETCODE = LDAP_RETCODE(71i32);
pub const LDAP_VIRTUAL_LIST_VIEW_ERROR: LDAP_RETCODE = LDAP_RETCODE(76i32);
pub const LDAP_OTHER: LDAP_RETCODE = LDAP_RETCODE(80i32);
pub const LDAP_SERVER_DOWN: LDAP_RETCODE = LDAP_RETCODE(81i32);
pub const LDAP_LOCAL_ERROR: LDAP_RETCODE = LDAP_RETCODE(82i32);
pub const LDAP_ENCODING_ERROR: LDAP_RETCODE = LDAP_RETCODE(83i32);
pub const LDAP_DECODING_ERROR: LDAP_RETCODE = LDAP_RETCODE(84i32);
pub const LDAP_TIMEOUT: LDAP_RETCODE = LDAP_RETCODE(85i32);
pub const LDAP_AUTH_UNKNOWN: LDAP_RETCODE = LDAP_RETCODE(86i32);
pub const LDAP_FILTER_ERROR: LDAP_RETCODE = LDAP_RETCODE(87i32);
pub const LDAP_USER_CANCELLED: LDAP_RETCODE = LDAP_RETCODE(88i32);
pub const LDAP_PARAM_ERROR: LDAP_RETCODE = LDAP_RETCODE(89i32);
pub const LDAP_NO_MEMORY: LDAP_RETCODE = LDAP_RETCODE(90i32);
pub const LDAP_CONNECT_ERROR: LDAP_RETCODE = LDAP_RETCODE(91i32);
pub const LDAP_NOT_SUPPORTED: LDAP_RETCODE = LDAP_RETCODE(92i32);
pub const LDAP_NO_RESULTS_RETURNED: LDAP_RETCODE = LDAP_RETCODE(94i32);
pub const LDAP_CONTROL_NOT_FOUND: LDAP_RETCODE = LDAP_RETCODE(93i32);
pub const LDAP_MORE_RESULTS_TO_RETURN: LDAP_RETCODE = LDAP_RETCODE(95i32);
pub const LDAP_CLIENT_LOOP: LDAP_RETCODE = LDAP_RETCODE(96i32);
pub const LDAP_REFERRAL_LIMIT_EXCEEDED: LDAP_RETCODE = LDAP_RETCODE(97i32);
impl ::core::convert::From<i32> for LDAP_RETCODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LDAP_RETCODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SCOPE_BASE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SCOPE_ONELEVEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SCOPE_SUBTREE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SEARCH_CMD: i32 = 99i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SESSION_CMD: i32 = 113i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SSL_GC_PORT: u32 = 3269u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SSL_PORT: u32 = 636u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SUBSTRING_ANY: i32 = 129i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SUBSTRING_FINAL: i32 = 130i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_SUBSTRING_INITIAL: i32 = 128i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub struct LDAP_TIMEVAL {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
impl LDAP_TIMEVAL {}
impl ::core::default::Default for LDAP_TIMEVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for LDAP_TIMEVAL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LDAP_TIMEVAL").field("tv_sec", &self.tv_sec).field("tv_usec", &self.tv_usec).finish()
    }
}
impl ::core::cmp::PartialEq for LDAP_TIMEVAL {
    fn eq(&self, other: &Self) -> bool {
        self.tv_sec == other.tv_sec && self.tv_usec == other.tv_usec
    }
}
impl ::core::cmp::Eq for LDAP_TIMEVAL {}
unsafe impl ::windows::runtime::Abi for LDAP_TIMEVAL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_UNBIND_CMD: i32 = 66i32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_UNICODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_VENDOR_VERSION: u32 = 510u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_VERSION1: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_VERSION2: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_VERSION3: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_VERSION_MAX: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_VERSION_MIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const LDAP_VLVINFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
#[inline]
pub unsafe fn LdapGetLastError() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LdapGetLastError() -> u32;
        }
        ::core::mem::transmute(LdapGetLastError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
#[inline]
pub unsafe fn LdapMapErrorToWin32(ldaperror: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LdapMapErrorToWin32(ldaperror: u32) -> u32;
        }
        ::core::mem::transmute(LdapMapErrorToWin32(::core::mem::transmute(ldaperror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LdapUTF8ToUnicode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpsrcstr: Param0, cchsrc: i32, lpdeststr: super::super::Foundation::PWSTR, cchdest: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LdapUTF8ToUnicode(lpsrcstr: super::super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::super::Foundation::PWSTR, cchdest: i32) -> i32;
        }
        ::core::mem::transmute(LdapUTF8ToUnicode(lpsrcstr.into_param().abi(), ::core::mem::transmute(cchsrc), ::core::mem::transmute(lpdeststr), ::core::mem::transmute(cchdest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LdapUnicodeToUTF8<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsrcstr: Param0, cchsrc: i32, lpdeststr: super::super::Foundation::PSTR, cchdest: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LdapUnicodeToUTF8(lpsrcstr: super::super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::super::Foundation::PSTR, cchdest: i32) -> i32;
        }
        ::core::mem::transmute(LdapUnicodeToUTF8(lpsrcstr.into_param().abi(), ::core::mem::transmute(cchsrc), ::core::mem::transmute(lpdeststr), ::core::mem::transmute(cchdest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type NOTIFYOFNEWCONNECTION = unsafe extern "system" fn(primaryconnection: *mut ldap, referralfromconnection: *mut ldap, newdn: super::super::Foundation::PWSTR, hostname: super::super::Foundation::PSTR, newconnection: *mut ldap, portnumber: u32, secauthidentity: *mut ::core::ffi::c_void, currentuser: *mut ::core::ffi::c_void, errorcodefrombind: u32) -> super::super::Foundation::BOOLEAN;
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`, `Win32_Security_Authentication_Identity`, `Win32_Security_Cryptography`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub type QUERYCLIENTCERT = unsafe extern "system" fn(connection: *mut ldap, trusted_cas: *mut super::super::Security::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertificate: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN;
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type QUERYFORCONNECTION = unsafe extern "system" fn(primaryconnection: *mut ldap, referralfromconnection: *mut ldap, newdn: super::super::Foundation::PWSTR, hostname: super::super::Foundation::PSTR, portnumber: u32, secauthidentity: *mut ::core::ffi::c_void, currentusertoken: *mut ::core::ffi::c_void, connectiontouse: *mut *mut ldap) -> u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const SERVER_SEARCH_FLAG_DOMAIN_SCOPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub const SERVER_SEARCH_FLAG_PHANTOM_ROOT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type VERIFYSERVERCERT = unsafe extern "system" fn(connection: *mut ldap, pservercert: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN;
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_alloc_t(options: i32) -> *mut berelement {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_alloc_t(options: i32) -> *mut berelement;
        }
        ::core::mem::transmute(ber_alloc_t(::core::mem::transmute(options)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_bvdup(pberval: *mut LDAP_BERVAL) -> *mut LDAP_BERVAL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_bvdup(pberval: *mut LDAP_BERVAL) -> *mut LDAP_BERVAL;
        }
        ::core::mem::transmute(ber_bvdup(::core::mem::transmute(pberval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_bvecfree(pberval: *mut *mut LDAP_BERVAL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_bvecfree(pberval: *mut *mut LDAP_BERVAL);
        }
        ::core::mem::transmute(ber_bvecfree(::core::mem::transmute(pberval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_bvfree(bv: *mut LDAP_BERVAL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_bvfree(bv: *mut LDAP_BERVAL);
        }
        ::core::mem::transmute(ber_bvfree(::core::mem::transmute(bv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_first_element(pberelement: *mut berelement, plen: *mut u32, ppopaque: *mut *mut super::super::Foundation::CHAR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_first_element(pberelement: *mut berelement, plen: *mut u32, ppopaque: *mut *mut super::super::Foundation::CHAR) -> u32;
        }
        ::core::mem::transmute(ber_first_element(::core::mem::transmute(pberelement), ::core::mem::transmute(plen), ::core::mem::transmute(ppopaque)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_flatten(pberelement: *mut berelement, pberval: *mut *mut LDAP_BERVAL) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_flatten(pberelement: *mut berelement, pberval: *mut *mut LDAP_BERVAL) -> i32;
        }
        ::core::mem::transmute(ber_flatten(::core::mem::transmute(pberelement), ::core::mem::transmute(pberval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_free(pberelement: *mut berelement, fbuf: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_free(pberelement: *mut berelement, fbuf: i32);
        }
        ::core::mem::transmute(ber_free(::core::mem::transmute(pberelement), ::core::mem::transmute(fbuf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_init(pberval: *mut LDAP_BERVAL) -> *mut berelement {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_init(pberval: *mut LDAP_BERVAL) -> *mut berelement;
        }
        ::core::mem::transmute(ber_init(::core::mem::transmute(pberval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_next_element<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pberelement: *mut berelement, plen: *mut u32, opaque: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_next_element(pberelement: *mut berelement, plen: *mut u32, opaque: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ber_next_element(::core::mem::transmute(pberelement), ::core::mem::transmute(plen), opaque.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_peek_tag(pberelement: *mut berelement, plen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_peek_tag(pberelement: *mut berelement, plen: *mut u32) -> u32;
        }
        ::core::mem::transmute(ber_peek_tag(::core::mem::transmute(pberelement), ::core::mem::transmute(plen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_printf<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pberelement: *mut berelement, fmt: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_printf(pberelement: *mut berelement, fmt: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(ber_printf(::core::mem::transmute(pberelement), fmt.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_scanf<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pberelement: *mut berelement, fmt: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_scanf(pberelement: *mut berelement, fmt: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ber_scanf(::core::mem::transmute(pberelement), fmt.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_skip_tag(pberelement: *mut berelement, plen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ber_skip_tag(pberelement: *mut berelement, plen: *mut u32) -> u32;
        }
        ::core::mem::transmute(ber_skip_tag(::core::mem::transmute(pberelement), ::core::mem::transmute(plen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct berelement {
    pub opaque: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl berelement {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for berelement {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for berelement {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("berelement").field("opaque", &self.opaque).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for berelement {
    fn eq(&self, other: &Self) -> bool {
        self.opaque == other.opaque
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for berelement {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for berelement {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn cldap_open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn cldap_open(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(cldap_open(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn cldap_openA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn cldap_openA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(cldap_openA(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn cldap_openW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn cldap_openW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(cldap_openW(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldap {
    pub ld_sb: ldap_0,
    pub ld_host: super::super::Foundation::PSTR,
    pub ld_version: u32,
    pub ld_lberoptions: u8,
    pub ld_deref: u32,
    pub ld_timelimit: u32,
    pub ld_sizelimit: u32,
    pub ld_errno: u32,
    pub ld_matched: super::super::Foundation::PSTR,
    pub ld_error: super::super::Foundation::PSTR,
    pub ld_msgid: u32,
    pub Reserved3: [u8; 25],
    pub ld_cldaptries: u32,
    pub ld_cldaptimeout: u32,
    pub ld_refhoplimit: u32,
    pub ld_options: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ldap {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldap {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldap {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldap")
            .field("ld_sb", &self.ld_sb)
            .field("ld_host", &self.ld_host)
            .field("ld_version", &self.ld_version)
            .field("ld_lberoptions", &self.ld_lberoptions)
            .field("ld_deref", &self.ld_deref)
            .field("ld_timelimit", &self.ld_timelimit)
            .field("ld_sizelimit", &self.ld_sizelimit)
            .field("ld_errno", &self.ld_errno)
            .field("ld_matched", &self.ld_matched)
            .field("ld_error", &self.ld_error)
            .field("ld_msgid", &self.ld_msgid)
            .field("Reserved3", &self.Reserved3)
            .field("ld_cldaptries", &self.ld_cldaptries)
            .field("ld_cldaptimeout", &self.ld_cldaptimeout)
            .field("ld_refhoplimit", &self.ld_refhoplimit)
            .field("ld_options", &self.ld_options)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldap {
    fn eq(&self, other: &Self) -> bool {
        self.ld_sb == other.ld_sb
            && self.ld_host == other.ld_host
            && self.ld_version == other.ld_version
            && self.ld_lberoptions == other.ld_lberoptions
            && self.ld_deref == other.ld_deref
            && self.ld_timelimit == other.ld_timelimit
            && self.ld_sizelimit == other.ld_sizelimit
            && self.ld_errno == other.ld_errno
            && self.ld_matched == other.ld_matched
            && self.ld_error == other.ld_error
            && self.ld_msgid == other.ld_msgid
            && self.Reserved3 == other.Reserved3
            && self.ld_cldaptries == other.ld_cldaptries
            && self.ld_cldaptimeout == other.ld_cldaptimeout
            && self.ld_refhoplimit == other.ld_refhoplimit
            && self.ld_options == other.ld_options
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldap {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldap {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ldap_0 {
    pub sb_sd: usize,
    pub Reserved1: [u8; 41],
    pub sb_naddr: usize,
    pub Reserved2: [u8; 24],
}
#[cfg(feature = "Win32_Foundation")]
impl ldap_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldap_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldap_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_ld_sb_e__Struct").field("sb_sd", &self.sb_sd).field("Reserved1", &self.Reserved1).field("sb_naddr", &self.sb_naddr).field("Reserved2", &self.Reserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldap_0 {
    fn eq(&self, other: &Self) -> bool {
        self.sb_sd == other.sb_sd && self.Reserved1 == other.Reserved1 && self.sb_naddr == other.sb_naddr && self.Reserved2 == other.Reserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldap_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldap_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_abandon(ld: *mut ldap, msgid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_abandon(ld: *mut ldap, msgid: u32) -> u32;
        }
        ::core::mem::transmute(ldap_abandon(::core::mem::transmute(ld), ::core::mem::transmute(msgid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
        }
        ::core::mem::transmute(ldap_add(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_addA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_addA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
        }
        ::core::mem::transmute(ldap_addA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_addW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_addW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW) -> u32;
        }
        ::core::mem::transmute(ldap_addW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_ext<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_add_ext(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_extA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_add_extA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_extW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_add_extW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_ext_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_add_ext_s(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_ext_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_add_ext_sA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_ext_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_add_ext_sW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
        }
        ::core::mem::transmute(ldap_add_s(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
        }
        ::core::mem::transmute(ldap_add_sA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, attrs: *mut *mut ldapmodW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_add_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW) -> u32;
        }
        ::core::mem::transmute(ldap_add_sW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(attrs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_bind<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, cred: Param2, method: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_bind(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
        }
        ::core::mem::transmute(ldap_bind(::core::mem::transmute(ld), dn.into_param().abi(), cred.into_param().abi(), ::core::mem::transmute(method)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_bindA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, cred: Param2, method: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_bindA(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
        }
        ::core::mem::transmute(ldap_bindA(::core::mem::transmute(ld), dn.into_param().abi(), cred.into_param().abi(), ::core::mem::transmute(method)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_bindW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, cred: Param2, method: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_bindW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, cred: super::super::Foundation::PWSTR, method: u32) -> u32;
        }
        ::core::mem::transmute(ldap_bindW(::core::mem::transmute(ld), dn.into_param().abi(), cred.into_param().abi(), ::core::mem::transmute(method)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_bind_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, cred: Param2, method: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_bind_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
        }
        ::core::mem::transmute(ldap_bind_s(::core::mem::transmute(ld), dn.into_param().abi(), cred.into_param().abi(), ::core::mem::transmute(method)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_bind_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, cred: Param2, method: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_bind_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
        }
        ::core::mem::transmute(ldap_bind_sA(::core::mem::transmute(ld), dn.into_param().abi(), cred.into_param().abi(), ::core::mem::transmute(method)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_bind_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, cred: Param2, method: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_bind_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, cred: super::super::Foundation::PWSTR, method: u32) -> u32;
        }
        ::core::mem::transmute(ldap_bind_sW(::core::mem::transmute(ld), dn.into_param().abi(), cred.into_param().abi(), ::core::mem::transmute(method)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_check_filterA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, searchfilter: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_check_filterA(ld: *mut ldap, searchfilter: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_check_filterA(::core::mem::transmute(ld), searchfilter.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_check_filterW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, searchfilter: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_check_filterW(ld: *mut ldap, searchfilter: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_check_filterW(::core::mem::transmute(ld), searchfilter.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_cleanup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hinstance: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_cleanup(hinstance: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(ldap_cleanup(hinstance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_close_extended_op(ld: *mut ldap, messagenumber: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_close_extended_op(ld: *mut ldap, messagenumber: u32) -> u32;
        }
        ::core::mem::transmute(ldap_close_extended_op(::core::mem::transmute(ld), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_compare(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compareA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compareA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_compareA(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compareW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compareW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_compareW(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_ext<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_compare_ext(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_extA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_compare_extA(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_extW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_compare_extW(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_ext_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_compare_ext_s(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_ext_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_compare_ext_sA(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_ext_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_compare_ext_sW(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_compare_s(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_compare_sA(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, attr: Param2, value: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_compare_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_compare_sW(::core::mem::transmute(ld), dn.into_param().abi(), attr.into_param().abi(), value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_conn_from_msg(primaryconn: *mut ldap, res: *mut LDAPMessage) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_conn_from_msg(primaryconn: *mut ldap, res: *mut LDAPMessage) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_conn_from_msg(::core::mem::transmute(primaryconn), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_connect(ld: *mut ldap, timeout: *mut LDAP_TIMEVAL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_connect(ld: *mut ldap, timeout: *mut LDAP_TIMEVAL) -> u32;
        }
        ::core::mem::transmute(ldap_connect(::core::mem::transmute(ld), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_control_free(control: *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_control_free(control: *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_control_free(::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_control_freeA(controls: *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_control_freeA(controls: *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_control_freeA(::core::mem::transmute(controls)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_control_freeW(control: *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_control_freeW(control: *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_control_freeW(::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_controls_free(controls: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_controls_free(controls: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_controls_free(::core::mem::transmute(controls)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_controls_freeA(controls: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_controls_freeA(controls: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_controls_freeA(::core::mem::transmute(controls)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_controls_freeW(control: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_controls_freeW(control: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_controls_freeW(::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_count_entries(ld: *mut ldap, res: *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_count_entries(ld: *mut ldap, res: *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_count_entries(::core::mem::transmute(ld), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_count_references(ld: *mut ldap, res: *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_count_references(ld: *mut ldap, res: *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_count_references(::core::mem::transmute(ld), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_count_values(vals: *const super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_count_values(vals: *const super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_count_values(::core::mem::transmute(vals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_count_valuesA(vals: *const super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_count_valuesA(vals: *const super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_count_valuesA(::core::mem::transmute(vals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_count_valuesW(vals: *const super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_count_valuesW(vals: *const super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_count_valuesW(::core::mem::transmute(vals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_count_values_len(vals: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_count_values_len(vals: *mut *mut LDAP_BERVAL) -> u32;
        }
        ::core::mem::transmute(ldap_count_values_len(::core::mem::transmute(vals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_page_control(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_create_page_control(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_create_page_control(::core::mem::transmute(externalhandle), ::core::mem::transmute(pagesize), ::core::mem::transmute(cookie), ::core::mem::transmute(iscritical), ::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_page_controlA(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_create_page_controlA(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_create_page_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(pagesize), ::core::mem::transmute(cookie), ::core::mem::transmute(iscritical), ::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_page_controlW(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_create_page_controlW(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_create_page_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(pagesize), ::core::mem::transmute(cookie), ::core::mem::transmute(iscritical), ::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_sort_control(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_create_sort_control(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_create_sort_control(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), ::core::mem::transmute(iscritical), ::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_create_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_create_sort_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), ::core::mem::transmute(iscritical), ::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_create_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_create_sort_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), ::core::mem::transmute(iscritical), ::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_vlv_controlA(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_create_vlv_controlA(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolA) -> i32;
        }
        ::core::mem::transmute(ldap_create_vlv_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(vlvinfo), ::core::mem::transmute(iscritical), ::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_vlv_controlW(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolW) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_create_vlv_controlW(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolW) -> i32;
        }
        ::core::mem::transmute(ldap_create_vlv_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(vlvinfo), ::core::mem::transmute(iscritical), ::core::mem::transmute(control)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_delete(::core::mem::transmute(ld), dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_deleteA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_deleteA(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_deleteA(::core::mem::transmute(ld), dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_deleteW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_deleteW(ld: *mut ldap, dn: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_deleteW(::core::mem::transmute(ld), dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_ext<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_delete_ext(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_extA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_delete_extA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_extW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_delete_extW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_ext_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_delete_ext_s(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_ext_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_delete_ext_sA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_ext_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_delete_ext_sW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_s(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_delete_s(::core::mem::transmute(ld), dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_delete_sA(::core::mem::transmute(ld), dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_delete_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_delete_sW(::core::mem::transmute(ld), dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_dn2ufn<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dn: Param0) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_dn2ufn(dn: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_dn2ufn(dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_dn2ufnA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dn: Param0) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_dn2ufnA(dn: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_dn2ufnA(dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_dn2ufnW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(dn: Param0) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_dn2ufnW(dn: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(ldap_dn2ufnW(dn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_encode_sort_controlA<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, control: *mut ldapcontrolA, criticality: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_encode_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, control: *mut ldapcontrolA, criticality: super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(ldap_encode_sort_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), ::core::mem::transmute(control), criticality.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_encode_sort_controlW<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, control: *mut ldapcontrolW, criticality: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_encode_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, control: *mut ldapcontrolW, criticality: super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(ldap_encode_sort_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), ::core::mem::transmute(control), criticality.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_err2string(err: u32) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_err2string(err: u32) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_err2string(::core::mem::transmute(err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_err2stringA(err: u32) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_err2stringA(err: u32) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_err2stringA(::core::mem::transmute(err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_err2stringW(err: u32) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_err2stringW(err: u32) -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(ldap_err2stringW(::core::mem::transmute(err)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_escape_filter_element<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(sourcefilterelement: Param0, sourcelength: u32, destfilterelement: super::super::Foundation::PSTR, destlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_escape_filter_element(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PSTR, destlength: u32) -> u32;
        }
        ::core::mem::transmute(ldap_escape_filter_element(sourcefilterelement.into_param().abi(), ::core::mem::transmute(sourcelength), ::core::mem::transmute(destfilterelement), ::core::mem::transmute(destlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_escape_filter_elementA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(sourcefilterelement: Param0, sourcelength: u32, destfilterelement: super::super::Foundation::PSTR, destlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_escape_filter_elementA(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PSTR, destlength: u32) -> u32;
        }
        ::core::mem::transmute(ldap_escape_filter_elementA(sourcefilterelement.into_param().abi(), ::core::mem::transmute(sourcelength), ::core::mem::transmute(destfilterelement), ::core::mem::transmute(destlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_escape_filter_elementW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(sourcefilterelement: Param0, sourcelength: u32, destfilterelement: super::super::Foundation::PWSTR, destlength: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_escape_filter_elementW(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PWSTR, destlength: u32) -> u32;
        }
        ::core::mem::transmute(ldap_escape_filter_elementW(sourcefilterelement.into_param().abi(), ::core::mem::transmute(sourcelength), ::core::mem::transmute(destfilterelement), ::core::mem::transmute(destlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_explode_dn<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dn: Param0, notypes: u32) -> *mut super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_explode_dn(dn: super::super::Foundation::PSTR, notypes: u32) -> *mut super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_explode_dn(dn.into_param().abi(), ::core::mem::transmute(notypes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_explode_dnA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dn: Param0, notypes: u32) -> *mut super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_explode_dnA(dn: super::super::Foundation::PSTR, notypes: u32) -> *mut super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_explode_dnA(dn.into_param().abi(), ::core::mem::transmute(notypes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_explode_dnW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(dn: Param0, notypes: u32) -> *mut super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_explode_dnW(dn: super::super::Foundation::PWSTR, notypes: u32) -> *mut super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(ldap_explode_dnW(dn.into_param().abi(), ::core::mem::transmute(notypes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operation<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, oid: Param1, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_extended_operation(ld: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_extended_operation(::core::mem::transmute(ld), oid.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operationA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, oid: Param1, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_extended_operationA(ld: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_extended_operationA(::core::mem::transmute(ld), oid.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operationW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, oid: Param1, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_extended_operationW(ld: *mut ldap, oid: super::super::Foundation::PWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_extended_operationW(::core::mem::transmute(ld), oid.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operation_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, oid: Param1, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, returnedoid: *mut super::super::Foundation::PSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_extended_operation_sA(externalhandle: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, returnedoid: *mut super::super::Foundation::PSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
        }
        ::core::mem::transmute(ldap_extended_operation_sA(::core::mem::transmute(externalhandle), oid.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(returnedoid), ::core::mem::transmute(returneddata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operation_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(externalhandle: *mut ldap, oid: Param1, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, returnedoid: *mut super::super::Foundation::PWSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_extended_operation_sW(externalhandle: *mut ldap, oid: super::super::Foundation::PWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, returnedoid: *mut super::super::Foundation::PWSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
        }
        ::core::mem::transmute(ldap_extended_operation_sW(::core::mem::transmute(externalhandle), oid.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(returnedoid), ::core::mem::transmute(returneddata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_first_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_first_attribute(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_first_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_first_attributeA(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_first_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(ldap_first_attributeW(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_entry(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_first_entry(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage;
        }
        ::core::mem::transmute(ldap_first_entry(::core::mem::transmute(ld), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_reference(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_first_reference(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage;
        }
        ::core::mem::transmute(ldap_first_reference(::core::mem::transmute(ld), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_free_controls(controls: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_free_controls(controls: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_free_controls(::core::mem::transmute(controls)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_free_controlsA(controls: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_free_controlsA(controls: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_free_controlsA(::core::mem::transmute(controls)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_free_controlsW(controls: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_free_controlsW(controls: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_free_controlsW(::core::mem::transmute(controls)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_dn(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_dn(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_get_dn(::core::mem::transmute(ld), ::core::mem::transmute(entry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_dnA(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_dnA(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_get_dnA(::core::mem::transmute(ld), ::core::mem::transmute(entry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_dnW(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_dnW(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(ldap_get_dnW(::core::mem::transmute(ld), ::core::mem::transmute(entry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_next_page(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, pagesize: u32, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_next_page(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, pagesize: u32, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_get_next_page(::core::mem::transmute(externalhandle), ::core::mem::transmute(searchhandle), ::core::mem::transmute(pagesize), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_next_page_s(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, timeout: *mut LDAP_TIMEVAL, pagesize: u32, totalcount: *mut u32, results: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_next_page_s(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, timeout: *mut LDAP_TIMEVAL, pagesize: u32, totalcount: *mut u32, results: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_get_next_page_s(::core::mem::transmute(externalhandle), ::core::mem::transmute(searchhandle), ::core::mem::transmute(timeout), ::core::mem::transmute(pagesize), ::core::mem::transmute(totalcount), ::core::mem::transmute(results)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_option(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_option(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ldap_get_option(::core::mem::transmute(ld), ::core::mem::transmute(option), ::core::mem::transmute(outvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_optionW(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_optionW(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ldap_get_optionW(::core::mem::transmute(ld), ::core::mem::transmute(option), ::core::mem::transmute(outvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_paged_count(externalhandle: *mut ldap, searchblock: *mut ldapsearch, totalcount: *mut u32, results: *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_paged_count(externalhandle: *mut ldap, searchblock: *mut ldapsearch, totalcount: *mut u32, results: *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_get_paged_count(::core::mem::transmute(externalhandle), ::core::mem::transmute(searchblock), ::core::mem::transmute(totalcount), ::core::mem::transmute(results)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_values<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, entry: *mut LDAPMessage, attr: Param2) -> *mut super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_values(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_get_values(::core::mem::transmute(ld), ::core::mem::transmute(entry), attr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_valuesA<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, entry: *mut LDAPMessage, attr: Param2) -> *mut super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_valuesA(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_get_valuesA(::core::mem::transmute(ld), ::core::mem::transmute(entry), attr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_valuesW<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, entry: *mut LDAPMessage, attr: Param2) -> *mut super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_valuesW(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PWSTR) -> *mut super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(ldap_get_valuesW(::core::mem::transmute(ld), ::core::mem::transmute(entry), attr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_values_len<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: Param2) -> *mut *mut LDAP_BERVAL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_values_len(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut *mut LDAP_BERVAL;
        }
        ::core::mem::transmute(ldap_get_values_len(::core::mem::transmute(externalhandle), ::core::mem::transmute(message), attr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_values_lenA<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: Param2) -> *mut *mut LDAP_BERVAL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_values_lenA(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut *mut LDAP_BERVAL;
        }
        ::core::mem::transmute(ldap_get_values_lenA(::core::mem::transmute(externalhandle), ::core::mem::transmute(message), attr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_values_lenW<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: Param2) -> *mut *mut LDAP_BERVAL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_get_values_lenW(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PWSTR) -> *mut *mut LDAP_BERVAL;
        }
        ::core::mem::transmute(ldap_get_values_lenW(::core::mem::transmute(externalhandle), ::core::mem::transmute(message), attr.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_init<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_init(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_init(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_initA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_initA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_initA(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_initW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_initW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_initW(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_memfree<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(block: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_memfree(block: super::super::Foundation::PSTR);
        }
        ::core::mem::transmute(ldap_memfree(block.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_memfreeA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(block: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_memfreeA(block: super::super::Foundation::PSTR);
        }
        ::core::mem::transmute(ldap_memfreeA(block.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_memfreeW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(block: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_memfreeW(block: super::super::Foundation::PWSTR);
        }
        ::core::mem::transmute(ldap_memfreeW(block.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
        }
        ::core::mem::transmute(ldap_modify(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modifyA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modifyA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
        }
        ::core::mem::transmute(ldap_modifyA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modifyW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modifyW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW) -> u32;
        }
        ::core::mem::transmute(ldap_modifyW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_ext<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_modify_ext(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_extA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_modify_extA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_extW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_modify_extW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_ext_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_modify_ext_s(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_ext_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_modify_ext_sA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_ext_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_modify_ext_sW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
        }
        ::core::mem::transmute(ldap_modify_s(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
        }
        ::core::mem::transmute(ldap_modify_sA(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, mods: *mut *mut ldapmodW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modify_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW) -> u32;
        }
        ::core::mem::transmute(ldap_modify_sW(::core::mem::transmute(ld), dn.into_param().abi(), ::core::mem::transmute(mods)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2, deleteoldrdn: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn2(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn2(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi(), ::core::mem::transmute(deleteoldrdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn2A<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2, deleteoldrdn: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn2A(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn2A(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi(), ::core::mem::transmute(deleteoldrdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn2W<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2, deleteoldrdn: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn2W(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR, deleteoldrdn: i32) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn2W(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi(), ::core::mem::transmute(deleteoldrdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn2_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2, deleteoldrdn: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn2_s(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn2_s(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi(), ::core::mem::transmute(deleteoldrdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn2_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2, deleteoldrdn: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn2_sA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn2_sA(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi(), ::core::mem::transmute(deleteoldrdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn2_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2, deleteoldrdn: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn2_sW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR, deleteoldrdn: i32) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn2_sW(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi(), ::core::mem::transmute(deleteoldrdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdnA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdnA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_modrdnA(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdnW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdnW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_modrdnW(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn_s(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn_s(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn_sA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn_sA(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modrdn_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(externalhandle: *mut ldap, distinguishedname: Param1, newdistinguishedname: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_modrdn_sW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_modrdn_sW(::core::mem::transmute(externalhandle), distinguishedname.into_param().abi(), newdistinguishedname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_msgfree(res: *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_msgfree(res: *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_msgfree(::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_next_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_next_attribute(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_next_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PSTR;
        }
        ::core::mem::transmute(ldap_next_attributeA(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_next_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(ldap_next_attributeW(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_entry(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_next_entry(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage;
        }
        ::core::mem::transmute(ldap_next_entry(::core::mem::transmute(ld), ::core::mem::transmute(entry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_reference(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_next_reference(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage;
        }
        ::core::mem::transmute(ldap_next_reference(::core::mem::transmute(ld), ::core::mem::transmute(entry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_open(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_open(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_openA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_openA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_openA(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_openW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hostname: Param0, portnumber: u32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_openW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_openW(hostname.into_param().abi(), ::core::mem::transmute(portnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_extended_resultA<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut super::super::Foundation::PSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: Param4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_extended_resultA(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut super::super::Foundation::PSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(ldap_parse_extended_resultA(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(resultoid), ::core::mem::transmute(resultdata), freeit.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_extended_resultW<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut super::super::Foundation::PWSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: Param4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_extended_resultW(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut super::super::Foundation::PWSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(ldap_parse_extended_resultW(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(resultoid), ::core::mem::transmute(resultdata), freeit.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_page_control(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_page_control(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
        }
        ::core::mem::transmute(ldap_parse_page_control(::core::mem::transmute(externalhandle), ::core::mem::transmute(servercontrols), ::core::mem::transmute(totalcount), ::core::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_page_controlA(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_page_controlA(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
        }
        ::core::mem::transmute(ldap_parse_page_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(servercontrols), ::core::mem::transmute(totalcount), ::core::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_page_controlW(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolW, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_page_controlW(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolW, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
        }
        ::core::mem::transmute(ldap_parse_page_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(servercontrols), ::core::mem::transmute(totalcount), ::core::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_reference(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_reference(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_parse_reference(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(referrals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_referenceA(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_referenceA(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_parse_referenceA(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(referrals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_referenceW(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_referenceW(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_parse_referenceW(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(referrals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_result<'a, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PSTR, errormessage: *mut super::super::Foundation::PSTR, referrals: *mut *mut super::super::Foundation::PSTR, servercontrols: *mut *mut *mut ldapcontrolA, freeit: Param7) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_result(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PSTR, errormessage: *mut super::super::Foundation::PSTR, referrals: *mut *mut super::super::Foundation::PSTR, servercontrols: *mut *mut *mut ldapcontrolA, freeit: super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(ldap_parse_result(
            ::core::mem::transmute(connection),
            ::core::mem::transmute(resultmessage),
            ::core::mem::transmute(returncode),
            ::core::mem::transmute(matcheddns),
            ::core::mem::transmute(errormessage),
            ::core::mem::transmute(referrals),
            ::core::mem::transmute(servercontrols),
            freeit.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_resultA<'a, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PSTR, errormessage: *mut super::super::Foundation::PSTR, referrals: *mut *mut *mut i8, servercontrols: *mut *mut *mut ldapcontrolA, freeit: Param7) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_resultA(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PSTR, errormessage: *mut super::super::Foundation::PSTR, referrals: *mut *mut *mut i8, servercontrols: *mut *mut *mut ldapcontrolA, freeit: super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(ldap_parse_resultA(
            ::core::mem::transmute(connection),
            ::core::mem::transmute(resultmessage),
            ::core::mem::transmute(returncode),
            ::core::mem::transmute(matcheddns),
            ::core::mem::transmute(errormessage),
            ::core::mem::transmute(referrals),
            ::core::mem::transmute(servercontrols),
            freeit.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_resultW<'a, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PWSTR, errormessage: *mut super::super::Foundation::PWSTR, referrals: *mut *mut *mut u16, servercontrols: *mut *mut *mut ldapcontrolW, freeit: Param7) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_resultW(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PWSTR, errormessage: *mut super::super::Foundation::PWSTR, referrals: *mut *mut *mut u16, servercontrols: *mut *mut *mut ldapcontrolW, freeit: super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(ldap_parse_resultW(
            ::core::mem::transmute(connection),
            ::core::mem::transmute(resultmessage),
            ::core::mem::transmute(returncode),
            ::core::mem::transmute(matcheddns),
            ::core::mem::transmute(errormessage),
            ::core::mem::transmute(referrals),
            ::core::mem::transmute(servercontrols),
            freeit.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_sort_control(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_sort_control(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_parse_sort_control(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(result), ::core::mem::transmute(attribute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_sort_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_sort_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_parse_sort_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(result), ::core::mem::transmute(attribute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_sort_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, result: *mut u32, attribute: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_sort_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, result: *mut u32, attribute: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_parse_sort_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(result), ::core::mem::transmute(attribute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_vlv_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_vlv_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
        }
        ::core::mem::transmute(ldap_parse_vlv_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(targetpos), ::core::mem::transmute(listcount), ::core::mem::transmute(context), ::core::mem::transmute(errcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_vlv_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_parse_vlv_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
        }
        ::core::mem::transmute(ldap_parse_vlv_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(targetpos), ::core::mem::transmute(listcount), ::core::mem::transmute(context), ::core::mem::transmute(errcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_perror<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, msg: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_perror(ld: *mut ldap, msg: super::super::Foundation::PSTR);
        }
        ::core::mem::transmute(ldap_perror(::core::mem::transmute(ld), msg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_ext<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, newrdn: Param2, newparent: Param3, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_rename_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_rename_ext(::core::mem::transmute(ld), dn.into_param().abi(), newrdn.into_param().abi(), newparent.into_param().abi(), ::core::mem::transmute(deleteoldrdn), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_extA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, newrdn: Param2, newparent: Param3, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_rename_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_rename_extA(::core::mem::transmute(ld), dn.into_param().abi(), newrdn.into_param().abi(), newparent.into_param().abi(), ::core::mem::transmute(deleteoldrdn), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_extW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, newrdn: Param2, newparent: Param3, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_rename_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, newrdn: super::super::Foundation::PWSTR, newparent: super::super::Foundation::PWSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_rename_extW(::core::mem::transmute(ld), dn.into_param().abi(), newrdn.into_param().abi(), newparent.into_param().abi(), ::core::mem::transmute(deleteoldrdn), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_ext_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, newrdn: Param2, newparent: Param3, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_rename_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_rename_ext_s(::core::mem::transmute(ld), dn.into_param().abi(), newrdn.into_param().abi(), newparent.into_param().abi(), ::core::mem::transmute(deleteoldrdn), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_ext_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, newrdn: Param2, newparent: Param3, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_rename_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_rename_ext_sA(::core::mem::transmute(ld), dn.into_param().abi(), newrdn.into_param().abi(), newparent.into_param().abi(), ::core::mem::transmute(deleteoldrdn), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_ext_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, newrdn: Param2, newparent: Param3, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_rename_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, newrdn: super::super::Foundation::PWSTR, newparent: super::super::Foundation::PWSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_rename_ext_sW(::core::mem::transmute(ld), dn.into_param().abi(), newrdn.into_param().abi(), newparent.into_param().abi(), ::core::mem::transmute(deleteoldrdn), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_result(ld: *mut ldap, msgid: u32, all: u32, timeout: *const LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_result(ld: *mut ldap, msgid: u32, all: u32, timeout: *const LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_result(::core::mem::transmute(ld), ::core::mem::transmute(msgid), ::core::mem::transmute(all), ::core::mem::transmute(timeout), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_result2error(ld: *mut ldap, res: *mut LDAPMessage, freeit: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_result2error(ld: *mut ldap, res: *mut LDAPMessage, freeit: u32) -> u32;
        }
        ::core::mem::transmute(ldap_result2error(::core::mem::transmute(ld), ::core::mem::transmute(res), ::core::mem::transmute(freeit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sasl_bindA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distname: Param1, authmechanism: Param2, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, messagenumber: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_sasl_bindA(externalhandle: *mut ldap, distname: super::super::Foundation::PSTR, authmechanism: super::super::Foundation::PSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, messagenumber: *mut i32) -> i32;
        }
        ::core::mem::transmute(ldap_sasl_bindA(::core::mem::transmute(externalhandle), distname.into_param().abi(), authmechanism.into_param().abi(), ::core::mem::transmute(cred), ::core::mem::transmute(serverctrls), ::core::mem::transmute(clientctrls), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sasl_bindW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(externalhandle: *mut ldap, distname: Param1, authmechanism: Param2, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, messagenumber: *mut i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_sasl_bindW(externalhandle: *mut ldap, distname: super::super::Foundation::PWSTR, authmechanism: super::super::Foundation::PWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, messagenumber: *mut i32) -> i32;
        }
        ::core::mem::transmute(ldap_sasl_bindW(::core::mem::transmute(externalhandle), distname.into_param().abi(), authmechanism.into_param().abi(), ::core::mem::transmute(cred), ::core::mem::transmute(serverctrls), ::core::mem::transmute(clientctrls), ::core::mem::transmute(messagenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sasl_bind_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(externalhandle: *mut ldap, distname: Param1, authmechanism: Param2, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, serverdata: *mut *mut LDAP_BERVAL) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_sasl_bind_sA(externalhandle: *mut ldap, distname: super::super::Foundation::PSTR, authmechanism: super::super::Foundation::PSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, serverdata: *mut *mut LDAP_BERVAL) -> i32;
        }
        ::core::mem::transmute(ldap_sasl_bind_sA(::core::mem::transmute(externalhandle), distname.into_param().abi(), authmechanism.into_param().abi(), ::core::mem::transmute(cred), ::core::mem::transmute(serverctrls), ::core::mem::transmute(clientctrls), ::core::mem::transmute(serverdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sasl_bind_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(externalhandle: *mut ldap, distname: Param1, authmechanism: Param2, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, serverdata: *mut *mut LDAP_BERVAL) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_sasl_bind_sW(externalhandle: *mut ldap, distname: super::super::Foundation::PWSTR, authmechanism: super::super::Foundation::PWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, serverdata: *mut *mut LDAP_BERVAL) -> i32;
        }
        ::core::mem::transmute(ldap_sasl_bind_sW(::core::mem::transmute(externalhandle), distname.into_param().abi(), authmechanism.into_param().abi(), ::core::mem::transmute(cred), ::core::mem::transmute(serverctrls), ::core::mem::transmute(clientctrls), ::core::mem::transmute(serverdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
        }
        ::core::mem::transmute(ldap_search(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_searchA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_searchA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
        }
        ::core::mem::transmute(ldap_searchA(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_searchW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const u16, attrsonly: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_searchW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32) -> u32;
        }
        ::core::mem::transmute(ldap_searchW(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_abandon_page(externalhandle: *mut ldap, searchblock: *mut ldapsearch) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_abandon_page(externalhandle: *mut ldap, searchblock: *mut ldapsearch) -> u32;
        }
        ::core::mem::transmute(ldap_search_abandon_page(::core::mem::transmute(externalhandle), ::core::mem::transmute(searchblock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_ext<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_ext(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_search_ext(
            ::core::mem::transmute(ld),
            base.into_param().abi(),
            ::core::mem::transmute(scope),
            filter.into_param().abi(),
            ::core::mem::transmute(attrs),
            ::core::mem::transmute(attrsonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(timelimit),
            ::core::mem::transmute(sizelimit),
            ::core::mem::transmute(messagenumber),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_extA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_extA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_search_extA(
            ::core::mem::transmute(ld),
            base.into_param().abi(),
            ::core::mem::transmute(scope),
            filter.into_param().abi(),
            ::core::mem::transmute(attrs),
            ::core::mem::transmute(attrsonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(timelimit),
            ::core::mem::transmute(sizelimit),
            ::core::mem::transmute(messagenumber),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_extW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_extW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
        }
        ::core::mem::transmute(ldap_search_extW(
            ::core::mem::transmute(ld),
            base.into_param().abi(),
            ::core::mem::transmute(scope),
            filter.into_param().abi(),
            ::core::mem::transmute(attrs),
            ::core::mem::transmute(attrsonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(timelimit),
            ::core::mem::transmute(sizelimit),
            ::core::mem::transmute(messagenumber),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_ext_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_ext_s(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_ext_s(
            ::core::mem::transmute(ld),
            base.into_param().abi(),
            ::core::mem::transmute(scope),
            filter.into_param().abi(),
            ::core::mem::transmute(attrs),
            ::core::mem::transmute(attrsonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(timeout),
            ::core::mem::transmute(sizelimit),
            ::core::mem::transmute(res),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_ext_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_ext_sA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_ext_sA(
            ::core::mem::transmute(ld),
            base.into_param().abi(),
            ::core::mem::transmute(scope),
            filter.into_param().abi(),
            ::core::mem::transmute(attrs),
            ::core::mem::transmute(attrsonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(timeout),
            ::core::mem::transmute(sizelimit),
            ::core::mem::transmute(res),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_ext_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_ext_sW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_ext_sW(
            ::core::mem::transmute(ld),
            base.into_param().abi(),
            ::core::mem::transmute(scope),
            filter.into_param().abi(),
            ::core::mem::transmute(attrs),
            ::core::mem::transmute(attrsonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(timeout),
            ::core::mem::transmute(sizelimit),
            ::core::mem::transmute(res),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_init_page<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    externalhandle: *mut ldap,
    distinguishedname: Param1,
    scopeofsearch: u32,
    searchfilter: Param3,
    attributelist: *mut *mut i8,
    attributesonly: u32,
    servercontrols: *mut *mut ldapcontrolA,
    clientcontrols: *mut *mut ldapcontrolA,
    pagetimelimit: u32,
    totalsizelimit: u32,
    sortkeys: *mut *mut ldapsortkeyA,
) -> *mut ldapsearch {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_init_page(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PSTR, attributelist: *mut *mut i8, attributesonly: u32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyA) -> *mut ldapsearch;
        }
        ::core::mem::transmute(ldap_search_init_page(
            ::core::mem::transmute(externalhandle),
            distinguishedname.into_param().abi(),
            ::core::mem::transmute(scopeofsearch),
            searchfilter.into_param().abi(),
            ::core::mem::transmute(attributelist),
            ::core::mem::transmute(attributesonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(pagetimelimit),
            ::core::mem::transmute(totalsizelimit),
            ::core::mem::transmute(sortkeys),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_init_pageA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    externalhandle: *mut ldap,
    distinguishedname: Param1,
    scopeofsearch: u32,
    searchfilter: Param3,
    attributelist: *const *const i8,
    attributesonly: u32,
    servercontrols: *mut *mut ldapcontrolA,
    clientcontrols: *mut *mut ldapcontrolA,
    pagetimelimit: u32,
    totalsizelimit: u32,
    sortkeys: *mut *mut ldapsortkeyA,
) -> *mut ldapsearch {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_init_pageA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PSTR, attributelist: *const *const i8, attributesonly: u32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyA) -> *mut ldapsearch;
        }
        ::core::mem::transmute(ldap_search_init_pageA(
            ::core::mem::transmute(externalhandle),
            distinguishedname.into_param().abi(),
            ::core::mem::transmute(scopeofsearch),
            searchfilter.into_param().abi(),
            ::core::mem::transmute(attributelist),
            ::core::mem::transmute(attributesonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(pagetimelimit),
            ::core::mem::transmute(totalsizelimit),
            ::core::mem::transmute(sortkeys),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_init_pageW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    externalhandle: *mut ldap,
    distinguishedname: Param1,
    scopeofsearch: u32,
    searchfilter: Param3,
    attributelist: *const *const u16,
    attributesonly: u32,
    servercontrols: *mut *mut ldapcontrolW,
    clientcontrols: *mut *mut ldapcontrolW,
    pagetimelimit: u32,
    totalsizelimit: u32,
    sortkeys: *mut *mut ldapsortkeyW,
) -> *mut ldapsearch {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_init_pageW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PWSTR, attributelist: *const *const u16, attributesonly: u32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyW) -> *mut ldapsearch;
        }
        ::core::mem::transmute(ldap_search_init_pageW(
            ::core::mem::transmute(externalhandle),
            distinguishedname.into_param().abi(),
            ::core::mem::transmute(scopeofsearch),
            searchfilter.into_param().abi(),
            ::core::mem::transmute(attributelist),
            ::core::mem::transmute(attributesonly),
            ::core::mem::transmute(servercontrols),
            ::core::mem::transmute(clientcontrols),
            ::core::mem::transmute(pagetimelimit),
            ::core::mem::transmute(totalsizelimit),
            ::core::mem::transmute(sortkeys),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_s(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_s(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_sA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_sA(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const u16, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_sW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_sW(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_st<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_st(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_st(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly), ::core::mem::transmute(timeout), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_stA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_stA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_stA(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly), ::core::mem::transmute(timeout), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_stW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, base: Param1, scope: u32, filter: Param3, attrs: *const *const u16, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_search_stW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
        }
        ::core::mem::transmute(ldap_search_stW(::core::mem::transmute(ld), base.into_param().abi(), ::core::mem::transmute(scope), filter.into_param().abi(), ::core::mem::transmute(attrs), ::core::mem::transmute(attrsonly), ::core::mem::transmute(timeout), ::core::mem::transmute(res)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
#[inline]
pub unsafe fn ldap_set_dbg_flags(newflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_set_dbg_flags(newflags: u32) -> u32;
        }
        ::core::mem::transmute(ldap_set_dbg_flags(::core::mem::transmute(newflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_set_dbg_routine(debugprintroutine: ::core::option::Option<DBGPRINT>) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_set_dbg_routine(debugprintroutine: ::windows::runtime::RawPtr);
        }
        ::core::mem::transmute(ldap_set_dbg_routine(::core::mem::transmute(debugprintroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_set_option(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_set_option(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ldap_set_option(::core::mem::transmute(ld), ::core::mem::transmute(option), ::core::mem::transmute(invalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_set_optionW(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_set_optionW(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ldap_set_optionW(::core::mem::transmute(ld), ::core::mem::transmute(option), ::core::mem::transmute(invalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_simple_bind<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, passwd: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_simple_bind(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_simple_bind(::core::mem::transmute(ld), dn.into_param().abi(), passwd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_simple_bindA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, passwd: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_simple_bindA(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_simple_bindA(::core::mem::transmute(ld), dn.into_param().abi(), passwd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_simple_bindW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, passwd: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_simple_bindW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_simple_bindW(::core::mem::transmute(ld), dn.into_param().abi(), passwd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_simple_bind_s<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, passwd: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_simple_bind_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_simple_bind_s(::core::mem::transmute(ld), dn.into_param().abi(), passwd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_simple_bind_sA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ld: *mut ldap, dn: Param1, passwd: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_simple_bind_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_simple_bind_sA(::core::mem::transmute(ld), dn.into_param().abi(), passwd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_simple_bind_sW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ld: *mut ldap, dn: Param1, passwd: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_simple_bind_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_simple_bind_sW(::core::mem::transmute(ld), dn.into_param().abi(), passwd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sslinit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, portnumber: u32, secure: i32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_sslinit(hostname: super::super::Foundation::PSTR, portnumber: u32, secure: i32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_sslinit(hostname.into_param().abi(), ::core::mem::transmute(portnumber), ::core::mem::transmute(secure)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sslinitA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, portnumber: u32, secure: i32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_sslinitA(hostname: super::super::Foundation::PSTR, portnumber: u32, secure: i32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_sslinitA(hostname.into_param().abi(), ::core::mem::transmute(portnumber), ::core::mem::transmute(secure)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sslinitW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hostname: Param0, portnumber: u32, secure: i32) -> *mut ldap {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_sslinitW(hostname: super::super::Foundation::PWSTR, portnumber: u32, secure: i32) -> *mut ldap;
        }
        ::core::mem::transmute(ldap_sslinitW(hostname.into_param().abi(), ::core::mem::transmute(portnumber), ::core::mem::transmute(secure)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_start_tls_sA(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_start_tls_sA(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
        }
        ::core::mem::transmute(ldap_start_tls_sA(::core::mem::transmute(externalhandle), ::core::mem::transmute(serverreturnvalue), ::core::mem::transmute(result), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_start_tls_sW(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_start_tls_sW(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
        }
        ::core::mem::transmute(ldap_start_tls_sW(::core::mem::transmute(externalhandle), ::core::mem::transmute(serverreturnvalue), ::core::mem::transmute(result), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_startup(version: *mut ldap_version_info, instance: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_startup(version: *mut ldap_version_info, instance: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(ldap_startup(::core::mem::transmute(version), ::core::mem::transmute(instance)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_stop_tls_s(externalhandle: *mut ldap) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_stop_tls_s(externalhandle: *mut ldap) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(ldap_stop_tls_s(::core::mem::transmute(externalhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_ufn2dn<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ufn: Param0, pdn: *mut super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_ufn2dn(ufn: super::super::Foundation::PSTR, pdn: *mut super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_ufn2dn(ufn.into_param().abi(), ::core::mem::transmute(pdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_ufn2dnA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(ufn: Param0, pdn: *mut super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_ufn2dnA(ufn: super::super::Foundation::PSTR, pdn: *mut super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_ufn2dnA(ufn.into_param().abi(), ::core::mem::transmute(pdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_ufn2dnW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ufn: Param0, pdn: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_ufn2dnW(ufn: super::super::Foundation::PWSTR, pdn: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_ufn2dnW(ufn.into_param().abi(), ::core::mem::transmute(pdn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_unbind(ld: *mut ldap) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_unbind(ld: *mut ldap) -> u32;
        }
        ::core::mem::transmute(ldap_unbind(::core::mem::transmute(ld)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_unbind_s(ld: *mut ldap) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_unbind_s(ld: *mut ldap) -> u32;
        }
        ::core::mem::transmute(ldap_unbind_s(::core::mem::transmute(ld)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_value_free(vals: *const super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_value_free(vals: *const super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_value_free(::core::mem::transmute(vals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_value_freeA(vals: *const super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_value_freeA(vals: *const super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(ldap_value_freeA(::core::mem::transmute(vals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_value_freeW(vals: *const super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_value_freeW(vals: *const super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(ldap_value_freeW(::core::mem::transmute(vals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_value_free_len(vals: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ldap_value_free_len(vals: *mut *mut LDAP_BERVAL) -> u32;
        }
        ::core::mem::transmute(ldap_value_free_len(::core::mem::transmute(vals)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Networking_Ldap`*"]
pub struct ldap_version_info {
    pub lv_size: u32,
    pub lv_major: u32,
    pub lv_minor: u32,
}
impl ldap_version_info {}
impl ::core::default::Default for ldap_version_info {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ldap_version_info {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldap_version_info").field("lv_size", &self.lv_size).field("lv_major", &self.lv_major).field("lv_minor", &self.lv_minor).finish()
    }
}
impl ::core::cmp::PartialEq for ldap_version_info {
    fn eq(&self, other: &Self) -> bool {
        self.lv_size == other.lv_size && self.lv_major == other.lv_major && self.lv_minor == other.lv_minor
    }
}
impl ::core::cmp::Eq for ldap_version_info {}
unsafe impl ::windows::runtime::Abi for ldap_version_info {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapapiinfoA {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut *mut i8,
    pub ldapai_vendor_name: super::super::Foundation::PSTR,
    pub ldapai_vendor_version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapapiinfoA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapapiinfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldapapiinfoA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldapapiinfoA")
            .field("ldapai_info_version", &self.ldapai_info_version)
            .field("ldapai_api_version", &self.ldapai_api_version)
            .field("ldapai_protocol_version", &self.ldapai_protocol_version)
            .field("ldapai_extensions", &self.ldapai_extensions)
            .field("ldapai_vendor_name", &self.ldapai_vendor_name)
            .field("ldapai_vendor_version", &self.ldapai_vendor_version)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapapiinfoA {
    fn eq(&self, other: &Self) -> bool {
        self.ldapai_info_version == other.ldapai_info_version && self.ldapai_api_version == other.ldapai_api_version && self.ldapai_protocol_version == other.ldapai_protocol_version && self.ldapai_extensions == other.ldapai_extensions && self.ldapai_vendor_name == other.ldapai_vendor_name && self.ldapai_vendor_version == other.ldapai_vendor_version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapapiinfoA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapapiinfoA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapapiinfoW {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut super::super::Foundation::PWSTR,
    pub ldapai_vendor_name: super::super::Foundation::PWSTR,
    pub ldapai_vendor_version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapapiinfoW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapapiinfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldapapiinfoW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldapapiinfoW")
            .field("ldapai_info_version", &self.ldapai_info_version)
            .field("ldapai_api_version", &self.ldapai_api_version)
            .field("ldapai_protocol_version", &self.ldapai_protocol_version)
            .field("ldapai_extensions", &self.ldapai_extensions)
            .field("ldapai_vendor_name", &self.ldapai_vendor_name)
            .field("ldapai_vendor_version", &self.ldapai_vendor_version)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapapiinfoW {
    fn eq(&self, other: &Self) -> bool {
        self.ldapai_info_version == other.ldapai_info_version && self.ldapai_api_version == other.ldapai_api_version && self.ldapai_protocol_version == other.ldapai_protocol_version && self.ldapai_extensions == other.ldapai_extensions && self.ldapai_vendor_name == other.ldapai_vendor_name && self.ldapai_vendor_version == other.ldapai_vendor_version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapapiinfoW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapapiinfoW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapcontrolA {
    pub ldctl_oid: super::super::Foundation::PSTR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapcontrolA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapcontrolA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldapcontrolA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldapcontrolA").field("ldctl_oid", &self.ldctl_oid).field("ldctl_value", &self.ldctl_value).field("ldctl_iscritical", &self.ldctl_iscritical).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapcontrolA {
    fn eq(&self, other: &Self) -> bool {
        self.ldctl_oid == other.ldctl_oid && self.ldctl_value == other.ldctl_value && self.ldctl_iscritical == other.ldctl_iscritical
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapcontrolA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapcontrolA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapcontrolW {
    pub ldctl_oid: super::super::Foundation::PWSTR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapcontrolW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapcontrolW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldapcontrolW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldapcontrolW").field("ldctl_oid", &self.ldctl_oid).field("ldctl_value", &self.ldctl_value).field("ldctl_iscritical", &self.ldctl_iscritical).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapcontrolW {
    fn eq(&self, other: &Self) -> bool {
        self.ldctl_oid == other.ldctl_oid && self.ldctl_value == other.ldctl_value && self.ldctl_iscritical == other.ldctl_iscritical
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapcontrolW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapcontrolW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapmodA {
    pub mod_op: u32,
    pub mod_type: super::super::Foundation::PSTR,
    pub mod_vals: ldapmodA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapmodA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapmodA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapmodA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapmodA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapmodA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union ldapmodA_0 {
    pub modv_strvals: *mut super::super::Foundation::PSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapmodA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapmodA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapmodA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapmodA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapmodA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapmodW {
    pub mod_op: u32,
    pub mod_type: super::super::Foundation::PWSTR,
    pub mod_vals: ldapmodW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapmodW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapmodW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapmodW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapmodW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapmodW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union ldapmodW_0 {
    pub modv_strvals: *mut super::super::Foundation::PWSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapmodW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapmodW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapmodW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapmodW_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapmodW_0 {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct ldapsearch(pub u8);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapsortkeyA {
    pub sk_attrtype: super::super::Foundation::PSTR,
    pub sk_matchruleoid: super::super::Foundation::PSTR,
    pub sk_reverseorder: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapsortkeyA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapsortkeyA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldapsortkeyA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldapsortkeyA").field("sk_attrtype", &self.sk_attrtype).field("sk_matchruleoid", &self.sk_matchruleoid).field("sk_reverseorder", &self.sk_reverseorder).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapsortkeyA {
    fn eq(&self, other: &Self) -> bool {
        self.sk_attrtype == other.sk_attrtype && self.sk_matchruleoid == other.sk_matchruleoid && self.sk_reverseorder == other.sk_reverseorder
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapsortkeyA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapsortkeyA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapsortkeyW {
    pub sk_attrtype: super::super::Foundation::PWSTR,
    pub sk_matchruleoid: super::super::Foundation::PWSTR,
    pub sk_reverseorder: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapsortkeyW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapsortkeyW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldapsortkeyW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldapsortkeyW").field("sk_attrtype", &self.sk_attrtype).field("sk_matchruleoid", &self.sk_matchruleoid).field("sk_reverseorder", &self.sk_reverseorder).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapsortkeyW {
    fn eq(&self, other: &Self) -> bool {
        self.sk_attrtype == other.sk_attrtype && self.sk_matchruleoid == other.sk_matchruleoid && self.sk_reverseorder == other.sk_reverseorder
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapsortkeyW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapsortkeyW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
pub struct ldapvlvinfo {
    pub ldvlv_version: i32,
    pub ldvlv_before_count: u32,
    pub ldvlv_after_count: u32,
    pub ldvlv_offset: u32,
    pub ldvlv_count: u32,
    pub ldvlv_attrvalue: *mut LDAP_BERVAL,
    pub ldvlv_context: *mut LDAP_BERVAL,
    pub ldvlv_extradata: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ldapvlvinfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ldapvlvinfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ldapvlvinfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ldapvlvinfo")
            .field("ldvlv_version", &self.ldvlv_version)
            .field("ldvlv_before_count", &self.ldvlv_before_count)
            .field("ldvlv_after_count", &self.ldvlv_after_count)
            .field("ldvlv_offset", &self.ldvlv_offset)
            .field("ldvlv_count", &self.ldvlv_count)
            .field("ldvlv_attrvalue", &self.ldvlv_attrvalue)
            .field("ldvlv_context", &self.ldvlv_context)
            .field("ldvlv_extradata", &self.ldvlv_extradata)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ldapvlvinfo {
    fn eq(&self, other: &Self) -> bool {
        self.ldvlv_version == other.ldvlv_version && self.ldvlv_before_count == other.ldvlv_before_count && self.ldvlv_after_count == other.ldvlv_after_count && self.ldvlv_offset == other.ldvlv_offset && self.ldvlv_count == other.ldvlv_count && self.ldvlv_attrvalue == other.ldvlv_attrvalue && self.ldvlv_context == other.ldvlv_context && self.ldvlv_extradata == other.ldvlv_extradata
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ldapvlvinfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ldapvlvinfo {
    type Abi = Self;
}
