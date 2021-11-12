#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUTHENTICATEF(pub i32);
pub const AUTHENTICATEF_PROXY: AUTHENTICATEF = AUTHENTICATEF(1i32);
pub const AUTHENTICATEF_BASIC: AUTHENTICATEF = AUTHENTICATEF(2i32);
pub const AUTHENTICATEF_HTTP: AUTHENTICATEF = AUTHENTICATEF(4i32);
impl ::core::convert::From<i32> for AUTHENTICATEF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AUTHENTICATEF {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDF(pub i32);
pub const BINDF_ASYNCHRONOUS: BINDF = BINDF(1i32);
pub const BINDF_ASYNCSTORAGE: BINDF = BINDF(2i32);
pub const BINDF_NOPROGRESSIVERENDERING: BINDF = BINDF(4i32);
pub const BINDF_OFFLINEOPERATION: BINDF = BINDF(8i32);
pub const BINDF_GETNEWESTVERSION: BINDF = BINDF(16i32);
pub const BINDF_NOWRITECACHE: BINDF = BINDF(32i32);
pub const BINDF_NEEDFILE: BINDF = BINDF(64i32);
pub const BINDF_PULLDATA: BINDF = BINDF(128i32);
pub const BINDF_IGNORESECURITYPROBLEM: BINDF = BINDF(256i32);
pub const BINDF_RESYNCHRONIZE: BINDF = BINDF(512i32);
pub const BINDF_HYPERLINK: BINDF = BINDF(1024i32);
pub const BINDF_NO_UI: BINDF = BINDF(2048i32);
pub const BINDF_SILENTOPERATION: BINDF = BINDF(4096i32);
pub const BINDF_PRAGMA_NO_CACHE: BINDF = BINDF(8192i32);
pub const BINDF_GETCLASSOBJECT: BINDF = BINDF(16384i32);
pub const BINDF_RESERVED_1: BINDF = BINDF(32768i32);
pub const BINDF_FREE_THREADED: BINDF = BINDF(65536i32);
pub const BINDF_DIRECT_READ: BINDF = BINDF(131072i32);
pub const BINDF_FORMS_SUBMIT: BINDF = BINDF(262144i32);
pub const BINDF_GETFROMCACHE_IF_NET_FAIL: BINDF = BINDF(524288i32);
pub const BINDF_FROMURLMON: BINDF = BINDF(1048576i32);
pub const BINDF_FWD_BACK: BINDF = BINDF(2097152i32);
pub const BINDF_PREFERDEFAULTHANDLER: BINDF = BINDF(4194304i32);
pub const BINDF_ENFORCERESTRICTED: BINDF = BINDF(8388608i32);
pub const BINDF_RESERVED_2: BINDF = BINDF(-2147483648i32);
pub const BINDF_RESERVED_3: BINDF = BINDF(16777216i32);
pub const BINDF_RESERVED_4: BINDF = BINDF(33554432i32);
pub const BINDF_RESERVED_5: BINDF = BINDF(67108864i32);
pub const BINDF_RESERVED_6: BINDF = BINDF(134217728i32);
pub const BINDF_RESERVED_7: BINDF = BINDF(1073741824i32);
pub const BINDF_RESERVED_8: BINDF = BINDF(536870912i32);
impl ::core::convert::From<i32> for BINDF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BINDF {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDF2(pub i32);
pub const BINDF2_DISABLEBASICOVERHTTP: BINDF2 = BINDF2(1i32);
pub const BINDF2_DISABLEAUTOCOOKIEHANDLING: BINDF2 = BINDF2(2i32);
pub const BINDF2_READ_DATA_GREATER_THAN_4GB: BINDF2 = BINDF2(4i32);
pub const BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID: BINDF2 = BINDF2(8i32);
pub const BINDF2_SETDOWNLOADMODE: BINDF2 = BINDF2(32i32);
pub const BINDF2_DISABLE_HTTP_REDIRECT_CACHING: BINDF2 = BINDF2(64i32);
pub const BINDF2_KEEP_CALLBACK_MODULE_LOADED: BINDF2 = BINDF2(128i32);
pub const BINDF2_ALLOW_PROXY_CRED_PROMPT: BINDF2 = BINDF2(256i32);
pub const BINDF2_RESERVED_17: BINDF2 = BINDF2(512i32);
pub const BINDF2_RESERVED_16: BINDF2 = BINDF2(1024i32);
pub const BINDF2_RESERVED_15: BINDF2 = BINDF2(2048i32);
pub const BINDF2_RESERVED_14: BINDF2 = BINDF2(4096i32);
pub const BINDF2_RESERVED_13: BINDF2 = BINDF2(8192i32);
pub const BINDF2_RESERVED_12: BINDF2 = BINDF2(16384i32);
pub const BINDF2_RESERVED_11: BINDF2 = BINDF2(32768i32);
pub const BINDF2_RESERVED_10: BINDF2 = BINDF2(65536i32);
pub const BINDF2_RESERVED_F: BINDF2 = BINDF2(131072i32);
pub const BINDF2_RESERVED_E: BINDF2 = BINDF2(262144i32);
pub const BINDF2_RESERVED_D: BINDF2 = BINDF2(524288i32);
pub const BINDF2_RESERVED_C: BINDF2 = BINDF2(1048576i32);
pub const BINDF2_RESERVED_B: BINDF2 = BINDF2(2097152i32);
pub const BINDF2_RESERVED_A: BINDF2 = BINDF2(4194304i32);
pub const BINDF2_RESERVED_9: BINDF2 = BINDF2(8388608i32);
pub const BINDF2_RESERVED_8: BINDF2 = BINDF2(16777216i32);
pub const BINDF2_RESERVED_7: BINDF2 = BINDF2(33554432i32);
pub const BINDF2_RESERVED_6: BINDF2 = BINDF2(67108864i32);
pub const BINDF2_RESERVED_5: BINDF2 = BINDF2(134217728i32);
pub const BINDF2_RESERVED_4: BINDF2 = BINDF2(268435456i32);
pub const BINDF2_RESERVED_3: BINDF2 = BINDF2(536870912i32);
pub const BINDF2_RESERVED_2: BINDF2 = BINDF2(1073741824i32);
pub const BINDF2_RESERVED_1: BINDF2 = BINDF2(-2147483648i32);
impl ::core::convert::From<i32> for BINDF2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BINDF2 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDHANDLETYPES(pub i32);
pub const BINDHANDLETYPES_APPCACHE: BINDHANDLETYPES = BINDHANDLETYPES(0i32);
pub const BINDHANDLETYPES_DEPENDENCY: BINDHANDLETYPES = BINDHANDLETYPES(1i32);
pub const BINDHANDLETYPES_COUNT: BINDHANDLETYPES = BINDHANDLETYPES(2i32);
impl ::core::convert::From<i32> for BINDHANDLETYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BINDHANDLETYPES {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDINFO_OPTIONS(pub i32);
pub const BINDINFO_OPTIONS_WININETFLAG: BINDINFO_OPTIONS = BINDINFO_OPTIONS(65536i32);
pub const BINDINFO_OPTIONS_ENABLE_UTF8: BINDINFO_OPTIONS = BINDINFO_OPTIONS(131072i32);
pub const BINDINFO_OPTIONS_DISABLE_UTF8: BINDINFO_OPTIONS = BINDINFO_OPTIONS(262144i32);
pub const BINDINFO_OPTIONS_USE_IE_ENCODING: BINDINFO_OPTIONS = BINDINFO_OPTIONS(524288i32);
pub const BINDINFO_OPTIONS_BINDTOOBJECT: BINDINFO_OPTIONS = BINDINFO_OPTIONS(1048576i32);
pub const BINDINFO_OPTIONS_SECURITYOPTOUT: BINDINFO_OPTIONS = BINDINFO_OPTIONS(2097152i32);
pub const BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN: BINDINFO_OPTIONS = BINDINFO_OPTIONS(4194304i32);
pub const BINDINFO_OPTIONS_USEBINDSTRINGCREDS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(8388608i32);
pub const BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(16777216i32);
pub const BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE: BINDINFO_OPTIONS = BINDINFO_OPTIONS(33554432i32);
pub const BINDINFO_WPC_DOWNLOADBLOCKED: BINDINFO_OPTIONS = BINDINFO_OPTIONS(134217728i32);
pub const BINDINFO_WPC_LOGGING_ENABLED: BINDINFO_OPTIONS = BINDINFO_OPTIONS(268435456i32);
pub const BINDINFO_OPTIONS_ALLOWCONNECTDATA: BINDINFO_OPTIONS = BINDINFO_OPTIONS(536870912i32);
pub const BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(1073741824i32);
pub const BINDINFO_OPTIONS_SHDOCVW_NAVIGATE: BINDINFO_OPTIONS = BINDINFO_OPTIONS(-2147483648i32);
impl ::core::convert::From<i32> for BINDINFO_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BINDINFO_OPTIONS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDSTATUS(pub i32);
pub const BINDSTATUS_FINDINGRESOURCE: BINDSTATUS = BINDSTATUS(1i32);
pub const BINDSTATUS_CONNECTING: BINDSTATUS = BINDSTATUS(2i32);
pub const BINDSTATUS_REDIRECTING: BINDSTATUS = BINDSTATUS(3i32);
pub const BINDSTATUS_BEGINDOWNLOADDATA: BINDSTATUS = BINDSTATUS(4i32);
pub const BINDSTATUS_DOWNLOADINGDATA: BINDSTATUS = BINDSTATUS(5i32);
pub const BINDSTATUS_ENDDOWNLOADDATA: BINDSTATUS = BINDSTATUS(6i32);
pub const BINDSTATUS_BEGINDOWNLOADCOMPONENTS: BINDSTATUS = BINDSTATUS(7i32);
pub const BINDSTATUS_INSTALLINGCOMPONENTS: BINDSTATUS = BINDSTATUS(8i32);
pub const BINDSTATUS_ENDDOWNLOADCOMPONENTS: BINDSTATUS = BINDSTATUS(9i32);
pub const BINDSTATUS_USINGCACHEDCOPY: BINDSTATUS = BINDSTATUS(10i32);
pub const BINDSTATUS_SENDINGREQUEST: BINDSTATUS = BINDSTATUS(11i32);
pub const BINDSTATUS_CLASSIDAVAILABLE: BINDSTATUS = BINDSTATUS(12i32);
pub const BINDSTATUS_MIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(13i32);
pub const BINDSTATUS_CACHEFILENAMEAVAILABLE: BINDSTATUS = BINDSTATUS(14i32);
pub const BINDSTATUS_BEGINSYNCOPERATION: BINDSTATUS = BINDSTATUS(15i32);
pub const BINDSTATUS_ENDSYNCOPERATION: BINDSTATUS = BINDSTATUS(16i32);
pub const BINDSTATUS_BEGINUPLOADDATA: BINDSTATUS = BINDSTATUS(17i32);
pub const BINDSTATUS_UPLOADINGDATA: BINDSTATUS = BINDSTATUS(18i32);
pub const BINDSTATUS_ENDUPLOADDATA: BINDSTATUS = BINDSTATUS(19i32);
pub const BINDSTATUS_PROTOCOLCLASSID: BINDSTATUS = BINDSTATUS(20i32);
pub const BINDSTATUS_ENCODING: BINDSTATUS = BINDSTATUS(21i32);
pub const BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(22i32);
pub const BINDSTATUS_CLASSINSTALLLOCATION: BINDSTATUS = BINDSTATUS(23i32);
pub const BINDSTATUS_DECODING: BINDSTATUS = BINDSTATUS(24i32);
pub const BINDSTATUS_LOADINGMIMEHANDLER: BINDSTATUS = BINDSTATUS(25i32);
pub const BINDSTATUS_CONTENTDISPOSITIONATTACH: BINDSTATUS = BINDSTATUS(26i32);
pub const BINDSTATUS_FILTERREPORTMIMETYPE: BINDSTATUS = BINDSTATUS(27i32);
pub const BINDSTATUS_CLSIDCANINSTANTIATE: BINDSTATUS = BINDSTATUS(28i32);
pub const BINDSTATUS_IUNKNOWNAVAILABLE: BINDSTATUS = BINDSTATUS(29i32);
pub const BINDSTATUS_DIRECTBIND: BINDSTATUS = BINDSTATUS(30i32);
pub const BINDSTATUS_RAWMIMETYPE: BINDSTATUS = BINDSTATUS(31i32);
pub const BINDSTATUS_PROXYDETECTING: BINDSTATUS = BINDSTATUS(32i32);
pub const BINDSTATUS_ACCEPTRANGES: BINDSTATUS = BINDSTATUS(33i32);
pub const BINDSTATUS_COOKIE_SENT: BINDSTATUS = BINDSTATUS(34i32);
pub const BINDSTATUS_COMPACT_POLICY_RECEIVED: BINDSTATUS = BINDSTATUS(35i32);
pub const BINDSTATUS_COOKIE_SUPPRESSED: BINDSTATUS = BINDSTATUS(36i32);
pub const BINDSTATUS_COOKIE_STATE_UNKNOWN: BINDSTATUS = BINDSTATUS(37i32);
pub const BINDSTATUS_COOKIE_STATE_ACCEPT: BINDSTATUS = BINDSTATUS(38i32);
pub const BINDSTATUS_COOKIE_STATE_REJECT: BINDSTATUS = BINDSTATUS(39i32);
pub const BINDSTATUS_COOKIE_STATE_PROMPT: BINDSTATUS = BINDSTATUS(40i32);
pub const BINDSTATUS_COOKIE_STATE_LEASH: BINDSTATUS = BINDSTATUS(41i32);
pub const BINDSTATUS_COOKIE_STATE_DOWNGRADE: BINDSTATUS = BINDSTATUS(42i32);
pub const BINDSTATUS_POLICY_HREF: BINDSTATUS = BINDSTATUS(43i32);
pub const BINDSTATUS_P3P_HEADER: BINDSTATUS = BINDSTATUS(44i32);
pub const BINDSTATUS_SESSION_COOKIE_RECEIVED: BINDSTATUS = BINDSTATUS(45i32);
pub const BINDSTATUS_PERSISTENT_COOKIE_RECEIVED: BINDSTATUS = BINDSTATUS(46i32);
pub const BINDSTATUS_SESSION_COOKIES_ALLOWED: BINDSTATUS = BINDSTATUS(47i32);
pub const BINDSTATUS_CACHECONTROL: BINDSTATUS = BINDSTATUS(48i32);
pub const BINDSTATUS_CONTENTDISPOSITIONFILENAME: BINDSTATUS = BINDSTATUS(49i32);
pub const BINDSTATUS_MIMETEXTPLAINMISMATCH: BINDSTATUS = BINDSTATUS(50i32);
pub const BINDSTATUS_PUBLISHERAVAILABLE: BINDSTATUS = BINDSTATUS(51i32);
pub const BINDSTATUS_DISPLAYNAMEAVAILABLE: BINDSTATUS = BINDSTATUS(52i32);
pub const BINDSTATUS_SSLUX_NAVBLOCKED: BINDSTATUS = BINDSTATUS(53i32);
pub const BINDSTATUS_SERVER_MIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(54i32);
pub const BINDSTATUS_SNIFFED_CLASSIDAVAILABLE: BINDSTATUS = BINDSTATUS(55i32);
pub const BINDSTATUS_64BIT_PROGRESS: BINDSTATUS = BINDSTATUS(56i32);
pub const BINDSTATUS_LAST: BINDSTATUS = BINDSTATUS(56i32);
pub const BINDSTATUS_RESERVED_0: BINDSTATUS = BINDSTATUS(57i32);
pub const BINDSTATUS_RESERVED_1: BINDSTATUS = BINDSTATUS(58i32);
pub const BINDSTATUS_RESERVED_2: BINDSTATUS = BINDSTATUS(59i32);
pub const BINDSTATUS_RESERVED_3: BINDSTATUS = BINDSTATUS(60i32);
pub const BINDSTATUS_RESERVED_4: BINDSTATUS = BINDSTATUS(61i32);
pub const BINDSTATUS_RESERVED_5: BINDSTATUS = BINDSTATUS(62i32);
pub const BINDSTATUS_RESERVED_6: BINDSTATUS = BINDSTATUS(63i32);
pub const BINDSTATUS_RESERVED_7: BINDSTATUS = BINDSTATUS(64i32);
pub const BINDSTATUS_RESERVED_8: BINDSTATUS = BINDSTATUS(65i32);
pub const BINDSTATUS_RESERVED_9: BINDSTATUS = BINDSTATUS(66i32);
pub const BINDSTATUS_RESERVED_A: BINDSTATUS = BINDSTATUS(67i32);
pub const BINDSTATUS_RESERVED_B: BINDSTATUS = BINDSTATUS(68i32);
pub const BINDSTATUS_RESERVED_C: BINDSTATUS = BINDSTATUS(69i32);
pub const BINDSTATUS_RESERVED_D: BINDSTATUS = BINDSTATUS(70i32);
pub const BINDSTATUS_RESERVED_E: BINDSTATUS = BINDSTATUS(71i32);
pub const BINDSTATUS_RESERVED_F: BINDSTATUS = BINDSTATUS(72i32);
pub const BINDSTATUS_RESERVED_10: BINDSTATUS = BINDSTATUS(73i32);
pub const BINDSTATUS_RESERVED_11: BINDSTATUS = BINDSTATUS(74i32);
pub const BINDSTATUS_RESERVED_12: BINDSTATUS = BINDSTATUS(75i32);
pub const BINDSTATUS_RESERVED_13: BINDSTATUS = BINDSTATUS(76i32);
pub const BINDSTATUS_RESERVED_14: BINDSTATUS = BINDSTATUS(77i32);
pub const BINDSTATUS_LAST_PRIVATE: BINDSTATUS = BINDSTATUS(77i32);
impl ::core::convert::From<i32> for BINDSTATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BINDSTATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDSTRING(pub i32);
pub const BINDSTRING_HEADERS: BINDSTRING = BINDSTRING(1i32);
pub const BINDSTRING_ACCEPT_MIMES: BINDSTRING = BINDSTRING(2i32);
pub const BINDSTRING_EXTRA_URL: BINDSTRING = BINDSTRING(3i32);
pub const BINDSTRING_LANGUAGE: BINDSTRING = BINDSTRING(4i32);
pub const BINDSTRING_USERNAME: BINDSTRING = BINDSTRING(5i32);
pub const BINDSTRING_PASSWORD: BINDSTRING = BINDSTRING(6i32);
pub const BINDSTRING_UA_PIXELS: BINDSTRING = BINDSTRING(7i32);
pub const BINDSTRING_UA_COLOR: BINDSTRING = BINDSTRING(8i32);
pub const BINDSTRING_OS: BINDSTRING = BINDSTRING(9i32);
pub const BINDSTRING_USER_AGENT: BINDSTRING = BINDSTRING(10i32);
pub const BINDSTRING_ACCEPT_ENCODINGS: BINDSTRING = BINDSTRING(11i32);
pub const BINDSTRING_POST_COOKIE: BINDSTRING = BINDSTRING(12i32);
pub const BINDSTRING_POST_DATA_MIME: BINDSTRING = BINDSTRING(13i32);
pub const BINDSTRING_URL: BINDSTRING = BINDSTRING(14i32);
pub const BINDSTRING_IID: BINDSTRING = BINDSTRING(15i32);
pub const BINDSTRING_FLAG_BIND_TO_OBJECT: BINDSTRING = BINDSTRING(16i32);
pub const BINDSTRING_PTR_BIND_CONTEXT: BINDSTRING = BINDSTRING(17i32);
pub const BINDSTRING_XDR_ORIGIN: BINDSTRING = BINDSTRING(18i32);
pub const BINDSTRING_DOWNLOADPATH: BINDSTRING = BINDSTRING(19i32);
pub const BINDSTRING_ROOTDOC_URL: BINDSTRING = BINDSTRING(20i32);
pub const BINDSTRING_INITIAL_FILENAME: BINDSTRING = BINDSTRING(21i32);
pub const BINDSTRING_PROXY_USERNAME: BINDSTRING = BINDSTRING(22i32);
pub const BINDSTRING_PROXY_PASSWORD: BINDSTRING = BINDSTRING(23i32);
pub const BINDSTRING_ENTERPRISE_ID: BINDSTRING = BINDSTRING(24i32);
pub const BINDSTRING_DOC_URL: BINDSTRING = BINDSTRING(25i32);
pub const BINDSTRING_SAMESITE_COOKIE_LEVEL: BINDSTRING = BINDSTRING(26i32);
impl ::core::convert::From<i32> for BINDSTRING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BINDSTRING {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDVERB(pub i32);
pub const BINDVERB_GET: BINDVERB = BINDVERB(0i32);
pub const BINDVERB_POST: BINDVERB = BINDVERB(1i32);
pub const BINDVERB_PUT: BINDVERB = BINDVERB(2i32);
pub const BINDVERB_CUSTOM: BINDVERB = BINDVERB(3i32);
pub const BINDVERB_RESERVED1: BINDVERB = BINDVERB(4i32);
impl ::core::convert::From<i32> for BINDVERB {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BINDVERB {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BSCF(pub i32);
pub const BSCF_FIRSTDATANOTIFICATION: BSCF = BSCF(1i32);
pub const BSCF_INTERMEDIATEDATANOTIFICATION: BSCF = BSCF(2i32);
pub const BSCF_LASTDATANOTIFICATION: BSCF = BSCF(4i32);
pub const BSCF_DATAFULLYAVAILABLE: BSCF = BSCF(8i32);
pub const BSCF_AVAILABLEDATASIZEUNKNOWN: BSCF = BSCF(16i32);
pub const BSCF_SKIPDRAINDATAFORFILEURLS: BSCF = BSCF(32i32);
pub const BSCF_64BITLENGTHDOWNLOAD: BSCF = BSCF(64i32);
impl ::core::convert::From<i32> for BSCF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BSCF {
    type Abi = Self;
}
pub const CF_NULL: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CIP_STATUS(pub i32);
pub const CIP_DISK_FULL: CIP_STATUS = CIP_STATUS(0i32);
pub const CIP_ACCESS_DENIED: CIP_STATUS = CIP_STATUS(1i32);
pub const CIP_NEWER_VERSION_EXISTS: CIP_STATUS = CIP_STATUS(2i32);
pub const CIP_OLDER_VERSION_EXISTS: CIP_STATUS = CIP_STATUS(3i32);
pub const CIP_NAME_CONFLICT: CIP_STATUS = CIP_STATUS(4i32);
pub const CIP_TRUST_VERIFICATION_COMPONENT_MISSING: CIP_STATUS = CIP_STATUS(5i32);
pub const CIP_EXE_SELF_REGISTERATION_TIMEOUT: CIP_STATUS = CIP_STATUS(6i32);
pub const CIP_UNSAFE_TO_ABORT: CIP_STATUS = CIP_STATUS(7i32);
pub const CIP_NEED_REBOOT: CIP_STATUS = CIP_STATUS(8i32);
pub const CIP_NEED_REBOOT_UI_PERMISSION: CIP_STATUS = CIP_STATUS(9i32);
impl ::core::convert::From<i32> for CIP_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CIP_STATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CODEBASEHOLD {
    pub cbSize: u32,
    pub szDistUnit: super::super::super::Foundation::PWSTR,
    pub szCodeBase: super::super::super::Foundation::PWSTR,
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
    pub dwStyle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CODEBASEHOLD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CODEBASEHOLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CODEBASEHOLD {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CODEBASEHOLD").field("cbSize", &self.cbSize).field("szDistUnit", &self.szDistUnit).field("szCodeBase", &self.szCodeBase).field("dwVersionMS", &self.dwVersionMS).field("dwVersionLS", &self.dwVersionLS).field("dwStyle", &self.dwStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CODEBASEHOLD {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szDistUnit == other.szDistUnit && self.szCodeBase == other.szCodeBase && self.dwVersionMS == other.dwVersionMS && self.dwVersionLS == other.dwVersionLS && self.dwStyle == other.dwStyle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CODEBASEHOLD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CODEBASEHOLD {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct CONFIRMSAFETY {
    pub clsid: ::windows::core::GUID,
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub dwFlags: u32,
}
impl CONFIRMSAFETY {}
impl ::core::default::Default for CONFIRMSAFETY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CONFIRMSAFETY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CONFIRMSAFETY").field("clsid", &self.clsid).field("pUnk", &self.pUnk).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::cmp::PartialEq for CONFIRMSAFETY {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid && self.pUnk == other.pUnk && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CONFIRMSAFETY {}
unsafe impl ::windows::core::Abi for CONFIRMSAFETY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const CONFIRMSAFETYACTION_LOADOBJECT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoGetClassObjectFromURL<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::IBindCtx>>(
    rclassid: *const ::windows::core::GUID,
    szcode: Param1,
    dwfileversionms: u32,
    dwfileversionls: u32,
    sztype: Param4,
    pbindctx: Param5,
    dwclscontext: super::CLSCTX,
    pvreserved: *mut ::core::ffi::c_void,
    riid: *const ::windows::core::GUID,
    ppv: *mut *mut ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetClassObjectFromURL(rclassid: *const ::windows::core::GUID, szcode: super::super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, sztype: super::super::super::Foundation::PWSTR, pbindctx: ::windows::core::RawPtr, dwclscontext: super::CLSCTX, pvreserved: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetClassObjectFromURL(
            ::core::mem::transmute(rclassid),
            szcode.into_param().abi(),
            ::core::mem::transmute(dwfileversionms),
            ::core::mem::transmute(dwfileversionls),
            sztype.into_param().abi(),
            pbindctx.into_param().abi(),
            ::core::mem::transmute(dwclscontext),
            ::core::mem::transmute(pvreserved),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(ppv),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInternetCombineIUri<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>, Param1: ::windows::core::IntoParam<'a, super::IUri>>(pbaseuri: Param0, prelativeuri: Param1, dwcombineflags: u32, ppcombineduri: *mut ::core::option::Option<super::IUri>, dwreserved: usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetCombineIUri(pbaseuri: ::windows::core::RawPtr, prelativeuri: ::windows::core::RawPtr, dwcombineflags: u32, ppcombineduri: *mut ::windows::core::RawPtr, dwreserved: usize) -> ::windows::core::HRESULT;
        }
        CoInternetCombineIUri(pbaseuri.into_param().abi(), prelativeuri.into_param().abi(), ::core::mem::transmute(dwcombineflags), ::core::mem::transmute(ppcombineduri), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetCombineUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwzbaseurl: Param0, pwzrelativeurl: Param1, dwcombineflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetCombineUrl(pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoInternetCombineUrl(pwzbaseurl.into_param().abi(), pwzrelativeurl.into_param().abi(), ::core::mem::transmute(dwcombineflags), ::core::mem::transmute(pszresult), ::core::mem::transmute(cchresult), ::core::mem::transmute(pcchresult), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetCombineUrlEx<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pbaseuri: Param0, pwzrelativeurl: Param1, dwcombineflags: u32, ppcombineduri: *mut ::core::option::Option<super::IUri>, dwreserved: usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetCombineUrlEx(pbaseuri: ::windows::core::RawPtr, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, ppcombineduri: *mut ::windows::core::RawPtr, dwreserved: usize) -> ::windows::core::HRESULT;
        }
        CoInternetCombineUrlEx(pbaseuri.into_param().abi(), pwzrelativeurl.into_param().abi(), ::core::mem::transmute(dwcombineflags), ::core::mem::transmute(ppcombineduri), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetCompareUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwzurl1: Param0, pwzurl2: Param1, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetCompareUrl(pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT;
        }
        CoInternetCompareUrl(pwzurl1.into_param().abi(), pwzurl2.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInternetCreateSecurityManager<'a, Param0: ::windows::core::IntoParam<'a, super::IServiceProvider>>(psp: Param0, ppsm: *mut ::core::option::Option<IInternetSecurityManager>, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetCreateSecurityManager(psp: ::windows::core::RawPtr, ppsm: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoInternetCreateSecurityManager(psp.into_param().abi(), ::core::mem::transmute(ppsm), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInternetCreateZoneManager<'a, Param0: ::windows::core::IntoParam<'a, super::IServiceProvider>>(psp: Param0, ppzm: *mut ::core::option::Option<IInternetZoneManager>, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetCreateZoneManager(psp: ::windows::core::RawPtr, ppzm: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoInternetCreateZoneManager(psp.into_param().abi(), ::core::mem::transmute(ppzm), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetGetProtocolFlags<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwzurl: Param0, pdwflags: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetGetProtocolFlags(pwzurl: super::super::super::Foundation::PWSTR, pdwflags: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoInternetGetProtocolFlags(pwzurl.into_param().abi(), ::core::mem::transmute(pdwflags), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetGetSecurityUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwszurl: Param0, ppwszsecurl: *mut super::super::super::Foundation::PWSTR, psuaction: PSUACTION, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetGetSecurityUrl(pwszurl: super::super::super::Foundation::PWSTR, ppwszsecurl: *mut super::super::super::Foundation::PWSTR, psuaction: PSUACTION, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoInternetGetSecurityUrl(pwszurl.into_param().abi(), ::core::mem::transmute(ppwszsecurl), ::core::mem::transmute(psuaction), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInternetGetSecurityUrlEx<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>>(puri: Param0, ppsecuri: *mut ::core::option::Option<super::IUri>, psuaction: PSUACTION, dwreserved: usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetGetSecurityUrlEx(puri: ::windows::core::RawPtr, ppsecuri: *mut ::windows::core::RawPtr, psuaction: PSUACTION, dwreserved: usize) -> ::windows::core::HRESULT;
        }
        CoInternetGetSecurityUrlEx(puri.into_param().abi(), ::core::mem::transmute(ppsecuri), ::core::mem::transmute(psuaction), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInternetGetSession(dwsessionmode: u32, ppiinternetsession: *mut ::core::option::Option<IInternetSession>, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetGetSession(dwsessionmode: u32, ppiinternetsession: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoInternetGetSession(::core::mem::transmute(dwsessionmode), ::core::mem::transmute(ppiinternetsession), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInternetIsFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetIsFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32) -> ::windows::core::HRESULT;
        }
        CoInternetIsFeatureEnabled(::core::mem::transmute(featureentry), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInternetIsFeatureEnabledForIUri<'a, Param2: ::windows::core::IntoParam<'a, super::IUri>, Param3: ::windows::core::IntoParam<'a, IInternetSecurityManagerEx2>>(featureentry: INTERNETFEATURELIST, dwflags: u32, piuri: Param2, psecmgr: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetIsFeatureEnabledForIUri(featureentry: INTERNETFEATURELIST, dwflags: u32, piuri: ::windows::core::RawPtr, psecmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoInternetIsFeatureEnabledForIUri(::core::mem::transmute(featureentry), ::core::mem::transmute(dwflags), piuri.into_param().abi(), psecmgr.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetIsFeatureEnabledForUrl<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IInternetSecurityManager>>(featureentry: INTERNETFEATURELIST, dwflags: u32, szurl: Param2, psecmgr: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetIsFeatureEnabledForUrl(featureentry: INTERNETFEATURELIST, dwflags: u32, szurl: super::super::super::Foundation::PWSTR, psecmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoInternetIsFeatureEnabledForUrl(::core::mem::transmute(featureentry), ::core::mem::transmute(dwflags), szurl.into_param().abi(), psecmgr.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetIsFeatureZoneElevationEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IInternetSecurityManager>>(szfromurl: Param0, sztourl: Param1, psecmgr: Param2, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetIsFeatureZoneElevationEnabled(szfromurl: super::super::super::Foundation::PWSTR, sztourl: super::super::super::Foundation::PWSTR, psecmgr: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT;
        }
        CoInternetIsFeatureZoneElevationEnabled(szfromurl.into_param().abi(), sztourl.into_param().abi(), psecmgr.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetParseIUri<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>>(piuri: Param0, parseaction: PARSEACTION, dwflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetParseIUri(piuri: ::windows::core::RawPtr, parseaction: PARSEACTION, dwflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT;
        }
        CoInternetParseIUri(piuri.into_param().abi(), ::core::mem::transmute(parseaction), ::core::mem::transmute(dwflags), ::core::mem::transmute(pwzresult), ::core::mem::transmute(cchresult), ::core::mem::transmute(pcchresult), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetParseUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwzurl: Param0, parseaction: PARSEACTION, dwflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetParseUrl(pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwflags: u32, pszresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoInternetParseUrl(pwzurl.into_param().abi(), ::core::mem::transmute(parseaction), ::core::mem::transmute(dwflags), ::core::mem::transmute(pszresult), ::core::mem::transmute(cchresult), ::core::mem::transmute(pcchresult), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetQueryInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwzurl: Param0, queryoptions: QUERYOPTION, dwqueryflags: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuffer: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetQueryInfo(pwzurl: super::super::super::Foundation::PWSTR, queryoptions: QUERYOPTION, dwqueryflags: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuffer: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoInternetQueryInfo(pwzurl.into_param().abi(), ::core::mem::transmute(queryoptions), ::core::mem::transmute(dwqueryflags), ::core::mem::transmute(pvbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcbbuffer), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInternetSetFeatureEnabled<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(featureentry: INTERNETFEATURELIST, dwflags: u32, fenable: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInternetSetFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        CoInternetSetFeatureEnabled(::core::mem::transmute(featureentry), ::core::mem::transmute(dwflags), fenable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CompareSecurityIds(pbsecurityid1: *const u8, dwlen1: u32, pbsecurityid2: *const u8, dwlen2: u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CompareSecurityIds(pbsecurityid1: *const u8, dwlen1: u32, pbsecurityid2: *const u8, dwlen2: u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CompareSecurityIds(::core::mem::transmute(pbsecurityid1), ::core::mem::transmute(dwlen1), ::core::mem::transmute(pbsecurityid2), ::core::mem::transmute(dwlen2), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CompatFlagsFromClsid(pclsid: *const ::windows::core::GUID, pdwcompatflags: *mut u32, pdwmiscstatusflags: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CompatFlagsFromClsid(pclsid: *const ::windows::core::GUID, pdwcompatflags: *mut u32, pdwmiscstatusflags: *mut u32) -> ::windows::core::HRESULT;
        }
        CompatFlagsFromClsid(::core::mem::transmute(pclsid), ::core::mem::transmute(pdwcompatflags), ::core::mem::transmute(pdwmiscstatusflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn CopyBindInfo(pcbisrc: *const super::BINDINFO) -> ::windows::core::Result<super::BINDINFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyBindInfo(pcbisrc: *const ::core::mem::ManuallyDrop<super::BINDINFO>, pbidest: *mut ::core::mem::ManuallyDrop<super::BINDINFO>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::BINDINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CopyBindInfo(::core::mem::transmute(pcbisrc), &mut result__).from_abi::<super::BINDINFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn CopyStgMedium(pcstgmedsrc: *const super::STGMEDIUM) -> ::windows::core::Result<super::STGMEDIUM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CopyStgMedium(pcstgmedsrc: *const ::core::mem::ManuallyDrop<super::STGMEDIUM>, pstgmeddest: *mut ::core::mem::ManuallyDrop<super::STGMEDIUM>) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::STGMEDIUM as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CopyStgMedium(::core::mem::transmute(pcstgmedsrc), &mut result__).from_abi::<super::STGMEDIUM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateAsyncBindCtx<'a, Param1: ::windows::core::IntoParam<'a, super::IBindStatusCallback>, Param2: ::windows::core::IntoParam<'a, super::IEnumFORMATETC>>(reserved: u32, pbscb: Param1, pefetc: Param2) -> ::windows::core::Result<super::IBindCtx> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAsyncBindCtx(reserved: u32, pbscb: ::windows::core::RawPtr, pefetc: ::windows::core::RawPtr, ppbc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::IBindCtx as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateAsyncBindCtx(::core::mem::transmute(reserved), pbscb.into_param().abi(), pefetc.into_param().abi(), &mut result__).from_abi::<super::IBindCtx>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateAsyncBindCtxEx<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param2: ::windows::core::IntoParam<'a, super::IBindStatusCallback>, Param3: ::windows::core::IntoParam<'a, super::IEnumFORMATETC>>(pbc: Param0, dwoptions: u32, pbscb: Param2, penum: Param3, ppbc: *mut ::core::option::Option<super::IBindCtx>, reserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAsyncBindCtxEx(pbc: ::windows::core::RawPtr, dwoptions: u32, pbscb: ::windows::core::RawPtr, penum: ::windows::core::RawPtr, ppbc: *mut ::windows::core::RawPtr, reserved: u32) -> ::windows::core::HRESULT;
        }
        CreateAsyncBindCtxEx(pbc.into_param().abi(), ::core::mem::transmute(dwoptions), pbscb.into_param().abi(), penum.into_param().abi(), ::core::mem::transmute(ppbc), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateFormatEnumerator(cfmtetc: u32, rgfmtetc: *const super::FORMATETC) -> ::windows::core::Result<super::IEnumFORMATETC> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFormatEnumerator(cfmtetc: u32, rgfmtetc: *const super::FORMATETC, ppenumfmtetc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::IEnumFORMATETC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateFormatEnumerator(::core::mem::transmute(cfmtetc), ::core::mem::transmute(rgfmtetc), &mut result__).from_abi::<super::IEnumFORMATETC>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateURLMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::IMoniker>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pmkctx: Param0, szurl: Param1) -> ::windows::core::Result<super::IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateURLMoniker(pmkctx: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateURLMoniker(pmkctx.into_param().abi(), szurl.into_param().abi(), &mut result__).from_abi::<super::IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateURLMonikerEx<'a, Param0: ::windows::core::IntoParam<'a, super::IMoniker>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pmkctx: Param0, szurl: Param1, ppmk: *mut ::core::option::Option<super::IMoniker>, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateURLMonikerEx(pmkctx: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, ppmk: *mut ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT;
        }
        CreateURLMonikerEx(pmkctx.into_param().abi(), szurl.into_param().abi(), ::core::mem::transmute(ppmk), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateURLMonikerEx2<'a, Param0: ::windows::core::IntoParam<'a, super::IMoniker>, Param1: ::windows::core::IntoParam<'a, super::IUri>>(pmkctx: Param0, puri: Param1, ppmk: *mut ::core::option::Option<super::IMoniker>, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateURLMonikerEx2(pmkctx: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT;
        }
        CreateURLMonikerEx2(pmkctx.into_param().abi(), puri.into_param().abi(), ::core::mem::transmute(ppmk), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DATAINFO {
    pub ulTotalSize: u32,
    pub ulavrPacketSize: u32,
    pub ulConnectSpeed: u32,
    pub ulProcessorSpeed: u32,
}
impl DATAINFO {}
impl ::core::default::Default for DATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DATAINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DATAINFO").field("ulTotalSize", &self.ulTotalSize).field("ulavrPacketSize", &self.ulavrPacketSize).field("ulConnectSpeed", &self.ulConnectSpeed).field("ulProcessorSpeed", &self.ulProcessorSpeed).finish()
    }
}
impl ::core::cmp::PartialEq for DATAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulTotalSize == other.ulTotalSize && self.ulavrPacketSize == other.ulavrPacketSize && self.ulConnectSpeed == other.ulConnectSpeed && self.ulProcessorSpeed == other.ulProcessorSpeed
    }
}
impl ::core::cmp::Eq for DATAINFO {}
unsafe impl ::windows::core::Abi for DATAINFO {
    type Abi = Self;
}
pub const E_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147483638i32 as _);
pub const FIEF_FLAG_FORCE_JITUI: u32 = 1u32;
pub const FIEF_FLAG_PEEK: u32 = 2u32;
pub const FIEF_FLAG_RESERVED_0: u32 = 8u32;
pub const FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK: u32 = 4u32;
pub const FMFD_DEFAULT: u32 = 0u32;
pub const FMFD_ENABLEMIMESNIFFING: u32 = 2u32;
pub const FMFD_IGNOREMIMETEXTPLAIN: u32 = 4u32;
pub const FMFD_RESERVED_1: u32 = 64u32;
pub const FMFD_RESERVED_2: u32 = 128u32;
pub const FMFD_RESPECTTEXTPLAIN: u32 = 16u32;
pub const FMFD_RETURNUPDATEDIMGMIMES: u32 = 32u32;
pub const FMFD_SERVERMIME: u32 = 8u32;
pub const FMFD_URLASFILENAME: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FaultInIEFeature<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(hwnd: Param0, pclassspec: *const super::uCLSSPEC, pquery: *mut super::QUERYCONTEXT, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FaultInIEFeature(hwnd: super::super::super::Foundation::HWND, pclassspec: *const super::uCLSSPEC, pquery: *mut super::QUERYCONTEXT, dwflags: u32) -> ::windows::core::HRESULT;
        }
        FaultInIEFeature(hwnd.into_param().abi(), ::core::mem::transmute(pclassspec), ::core::mem::transmute(pquery), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindMediaType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(rgsztypes: Param0) -> ::windows::core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindMediaType(rgsztypes: super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows::core::HRESULT;
        }
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        FindMediaType(rgsztypes.into_param().abi(), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindMediaTypeClass<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(pbc: Param0, sztype: Param1, pclsid: *mut ::windows::core::GUID, reserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindMediaTypeClass(pbc: ::windows::core::RawPtr, sztype: super::super::super::Foundation::PSTR, pclsid: *mut ::windows::core::GUID, reserved: u32) -> ::windows::core::HRESULT;
        }
        FindMediaTypeClass(pbc.into_param().abi(), sztype.into_param().abi(), ::core::mem::transmute(pclsid), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindMimeFromData<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pbc: Param0, pwzurl: Param1, pbuffer: *const ::core::ffi::c_void, cbsize: u32, pwzmimeproposed: Param4, dwmimeflags: u32, ppwzmimeout: *mut super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindMimeFromData(pbc: ::windows::core::RawPtr, pwzurl: super::super::super::Foundation::PWSTR, pbuffer: *const ::core::ffi::c_void, cbsize: u32, pwzmimeproposed: super::super::super::Foundation::PWSTR, dwmimeflags: u32, ppwzmimeout: *mut super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        FindMimeFromData(pbc.into_param().abi(), pwzurl.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbsize), pwzmimeproposed.into_param().abi(), ::core::mem::transmute(dwmimeflags), ::core::mem::transmute(ppwzmimeout), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GET_FEATURE_FROM_PROCESS: u32 = 2u32;
pub const GET_FEATURE_FROM_REGISTRY: u32 = 4u32;
pub const GET_FEATURE_FROM_THREAD: u32 = 1u32;
pub const GET_FEATURE_FROM_THREAD_INTERNET: u32 = 64u32;
pub const GET_FEATURE_FROM_THREAD_INTRANET: u32 = 16u32;
pub const GET_FEATURE_FROM_THREAD_LOCALMACHINE: u32 = 8u32;
pub const GET_FEATURE_FROM_THREAD_RESTRICTED: u32 = 128u32;
pub const GET_FEATURE_FROM_THREAD_TRUSTED: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassFileOrMime<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pbc: Param0, szfilename: Param1, pbuffer: *const ::core::ffi::c_void, cbsize: u32, szmime: Param4, dwreserved: u32) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClassFileOrMime(pbc: ::windows::core::RawPtr, szfilename: super::super::super::Foundation::PWSTR, pbuffer: *const ::core::ffi::c_void, cbsize: u32, szmime: super::super::super::Foundation::PWSTR, dwreserved: u32, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetClassFileOrMime(pbc.into_param().abi(), szfilename.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbsize), szmime.into_param().abi(), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassURL<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(szurl: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClassURL(szurl: super::super::super::Foundation::PWSTR, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetClassURL(szurl.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComponentIDFromCLSSPEC(pclassspec: *const super::uCLSSPEC) -> ::windows::core::Result<super::super::super::Foundation::PSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComponentIDFromCLSSPEC(pclassspec: *const super::uCLSSPEC, ppszcomponentid: *mut super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetComponentIDFromCLSSPEC(::core::mem::transmute(pclassspec), &mut result__).from_abi::<super::super::super::Foundation::PSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSoftwareUpdateInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(szdistunit: Param0) -> ::windows::core::Result<SOFTDISTINFO> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSoftwareUpdateInfo(szdistunit: super::super::super::Foundation::PWSTR, psdi: *mut SOFTDISTINFO) -> ::windows::core::HRESULT;
        }
        let mut result__: <SOFTDISTINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetSoftwareUpdateInfo(szdistunit.into_param().abi(), &mut result__).from_abi::<SOFTDISTINFO>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HIT_LOGGING_INFO {
    pub dwStructSize: u32,
    pub lpszLoggedUrlName: super::super::super::Foundation::PSTR,
    pub StartTime: super::super::super::Foundation::SYSTEMTIME,
    pub EndTime: super::super::super::Foundation::SYSTEMTIME,
    pub lpszExtendedInfo: super::super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl HIT_LOGGING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIT_LOGGING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIT_LOGGING_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HIT_LOGGING_INFO").field("dwStructSize", &self.dwStructSize).field("lpszLoggedUrlName", &self.lpszLoggedUrlName).field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("lpszExtendedInfo", &self.lpszExtendedInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIT_LOGGING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszLoggedUrlName == other.lpszLoggedUrlName && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.lpszExtendedInfo == other.lpszExtendedInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIT_LOGGING_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HIT_LOGGING_INFO {
    type Abi = Self;
}
#[inline]
pub unsafe fn HlinkGoBack<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HlinkGoBack(punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HlinkGoBack(punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HlinkGoForward<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HlinkGoForward(punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HlinkGoForward(punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HlinkNavigateMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::IMoniker>>(punk: Param0, pmktarget: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HlinkNavigateMoniker(punk: ::windows::core::RawPtr, pmktarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HlinkNavigateMoniker(punk.into_param().abi(), pmktarget.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HlinkNavigateString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(punk: Param0, sztarget: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HlinkNavigateString(punk: ::windows::core::RawPtr, sztarget: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HlinkNavigateString(punk.into_param().abi(), sztarget.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HlinkSimpleNavigateToMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::IMoniker>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, super::IBindCtx>, Param5: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(
    pmktarget: Param0,
    szlocation: Param1,
    sztargetframename: Param2,
    punk: Param3,
    pbc: Param4,
    param5: Param5,
    grfhlnf: u32,
    dwreserved: u32,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HlinkSimpleNavigateToMoniker(pmktarget: ::windows::core::RawPtr, szlocation: super::super::super::Foundation::PWSTR, sztargetframename: super::super::super::Foundation::PWSTR, punk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, param5: ::windows::core::RawPtr, grfhlnf: u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        HlinkSimpleNavigateToMoniker(pmktarget.into_param().abi(), szlocation.into_param().abi(), sztargetframename.into_param().abi(), punk.into_param().abi(), pbc.into_param().abi(), param5.into_param().abi(), ::core::mem::transmute(grfhlnf), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HlinkSimpleNavigateToString<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, super::IBindCtx>, Param5: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(
    sztarget: Param0,
    szlocation: Param1,
    sztargetframename: Param2,
    punk: Param3,
    pbc: Param4,
    param5: Param5,
    grfhlnf: u32,
    dwreserved: u32,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HlinkSimpleNavigateToString(sztarget: super::super::super::Foundation::PWSTR, szlocation: super::super::super::Foundation::PWSTR, sztargetframename: super::super::super::Foundation::PWSTR, punk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, param5: ::windows::core::RawPtr, grfhlnf: u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        HlinkSimpleNavigateToString(sztarget.into_param().abi(), szlocation.into_param().abi(), sztargetframename.into_param().abi(), punk.into_param().abi(), pbc.into_param().abi(), param5.into_param().abi(), ::core::mem::transmute(grfhlnf), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindCallbackRedirect(pub ::windows::core::IUnknown);
impl IBindCallbackRedirect {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Redirect<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, lpcurl: Param0) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), lpcurl.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBindCallbackRedirect {
    type Vtable = IBindCallbackRedirect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c81bc2_121e_4ed5_b9c4_b430bd54f2c0);
}
impl ::core::convert::From<IBindCallbackRedirect> for ::windows::core::IUnknown {
    fn from(value: IBindCallbackRedirect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindCallbackRedirect> for ::windows::core::IUnknown {
    fn from(value: &IBindCallbackRedirect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindCallbackRedirect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindCallbackRedirect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCallbackRedirect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpcurl: super::super::super::Foundation::PWSTR, vbcancel: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindHttpSecurity(pub ::windows::core::IUnknown);
impl IBindHttpSecurity {
    pub unsafe fn GetIgnoreCertMask(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBindHttpSecurity {
    type Vtable = IBindHttpSecurity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9eda967_f50e_4a33_b358_206f6ef3086d);
}
impl ::core::convert::From<IBindHttpSecurity> for ::windows::core::IUnknown {
    fn from(value: IBindHttpSecurity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindHttpSecurity> for ::windows::core::IUnknown {
    fn from(value: &IBindHttpSecurity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindHttpSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindHttpSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHttpSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwignorecertmask: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindProtocol(pub ::windows::core::IUnknown);
impl IBindProtocol {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBinding<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::IBindCtx>>(&self, szurl: Param0, pbc: Param1) -> ::windows::core::Result<super::IBinding> {
        let mut result__: <super::IBinding as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szurl.into_param().abi(), pbc.into_param().abi(), &mut result__).from_abi::<super::IBinding>(result__)
    }
}
unsafe impl ::windows::core::Interface for IBindProtocol {
    type Vtable = IBindProtocol_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9cd_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IBindProtocol> for ::windows::core::IUnknown {
    fn from(value: IBindProtocol) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindProtocol> for ::windows::core::IUnknown {
    fn from(value: &IBindProtocol) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindProtocol {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindProtocol {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindProtocol_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, ppb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICatalogFileInfo(pub ::windows::core::IUnknown);
impl ICatalogFileInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCatalogFile(&self) -> ::windows::core::Result<super::super::super::Foundation::PSTR> {
        let mut result__: <super::super::super::Foundation::PSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PSTR>(result__)
    }
    pub unsafe fn GetJavaTrust(&self, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppjavatrust)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICatalogFileInfo {
    type Vtable = ICatalogFileInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x711c7600_6b48_11d1_b403_00aa00b92af1);
}
impl ::core::convert::From<ICatalogFileInfo> for ::windows::core::IUnknown {
    fn from(value: ICatalogFileInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICatalogFileInfo> for ::windows::core::IUnknown {
    fn from(value: &ICatalogFileInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatalogFileInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICatalogFileInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatalogFileInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszcatalogfile: *mut super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICodeInstall(pub ::windows::core::IUnknown);
impl ICodeInstall {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidreason), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnCodeInstallProblem<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, ulstatuscode: u32, szdestination: Param1, szsource: Param2, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulstatuscode), szdestination.into_param().abi(), szsource.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICodeInstall {
    type Vtable = ICodeInstall_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d1_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<ICodeInstall> for ::windows::core::IUnknown {
    fn from(value: ICodeInstall) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICodeInstall> for ::windows::core::IUnknown {
    fn from(value: &ICodeInstall) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICodeInstall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICodeInstall {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ICodeInstall> for IWindowForBindingUI {
    fn from(value: ICodeInstall) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICodeInstall> for IWindowForBindingUI {
    fn from(value: &ICodeInstall) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWindowForBindingUI> for ICodeInstall {
    fn into_param(self) -> ::windows::core::Param<'a, IWindowForBindingUI> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWindowForBindingUI> for &ICodeInstall {
    fn into_param(self) -> ::windows::core::Param<'a, IWindowForBindingUI> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodeInstall_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguidreason: *const ::windows::core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulstatuscode: u32, szdestination: super::super::super::Foundation::PWSTR, szsource: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataFilter(pub ::windows::core::IUnknown);
impl IDataFilter {
    pub unsafe fn DoEncode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(linbuffersize),
            ::core::mem::transmute(pbinbuffer),
            ::core::mem::transmute(loutbuffersize),
            ::core::mem::transmute(pboutbuffer),
            ::core::mem::transmute(linbytesavailable),
            ::core::mem::transmute(plinbytesread),
            ::core::mem::transmute(ploutbyteswritten),
            ::core::mem::transmute(dwreserved),
        )
        .ok()
    }
    pub unsafe fn DoDecode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(linbuffersize),
            ::core::mem::transmute(pbinbuffer),
            ::core::mem::transmute(loutbuffersize),
            ::core::mem::transmute(pboutbuffer),
            ::core::mem::transmute(linbytesavailable),
            ::core::mem::transmute(plinbytesread),
            ::core::mem::transmute(ploutbyteswritten),
            ::core::mem::transmute(dwreserved),
        )
        .ok()
    }
    pub unsafe fn SetEncodingLevel(&self, dwenclevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenclevel)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDataFilter {
    type Vtable = IDataFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69d14c80_c18e_11d0_a9ce_006097942311);
}
impl ::core::convert::From<IDataFilter> for ::windows::core::IUnknown {
    fn from(value: IDataFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataFilter> for ::windows::core::IUnknown {
    fn from(value: &IDataFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwenclevel: u32) -> ::windows::core::HRESULT,
);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IEGetUserPrivateNamespaceName() -> super::super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IEGetUserPrivateNamespaceName() -> super::super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(IEGetUserPrivateNamespaceName())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IEInstallScope() -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IEInstallScope(pdwscope: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        IEInstallScope(&mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct IEObjectType(pub i32);
pub const IE_EPM_OBJECT_EVENT: IEObjectType = IEObjectType(0i32);
pub const IE_EPM_OBJECT_MUTEX: IEObjectType = IEObjectType(1i32);
pub const IE_EPM_OBJECT_SEMAPHORE: IEObjectType = IEObjectType(2i32);
pub const IE_EPM_OBJECT_SHARED_MEMORY: IEObjectType = IEObjectType(3i32);
pub const IE_EPM_OBJECT_WAITABLE_TIMER: IEObjectType = IEObjectType(4i32);
pub const IE_EPM_OBJECT_FILE: IEObjectType = IEObjectType(5i32);
pub const IE_EPM_OBJECT_NAMED_PIPE: IEObjectType = IEObjectType(6i32);
pub const IE_EPM_OBJECT_REGISTRY: IEObjectType = IEObjectType(7i32);
impl ::core::convert::From<i32> for IEObjectType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for IEObjectType {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEncodingFilterFactory(pub ::windows::core::IUnknown);
impl IEncodingFilterFactory {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindBestFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, DATAINFO>>(&self, pwzcodein: Param0, pwzcodeout: Param1, info: Param2) -> ::windows::core::Result<IDataFilter> {
        let mut result__: <IDataFilter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwzcodein.into_param().abi(), pwzcodeout.into_param().abi(), info.into_param().abi(), &mut result__).from_abi::<IDataFilter>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzcodein: Param0, pwzcodeout: Param1) -> ::windows::core::Result<IDataFilter> {
        let mut result__: <IDataFilter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwzcodein.into_param().abi(), pwzcodeout.into_param().abi(), &mut result__).from_abi::<IDataFilter>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEncodingFilterFactory {
    type Vtable = IEncodingFilterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70bdde00_c18e_11d0_a9ce_006097942311);
}
impl ::core::convert::From<IEncodingFilterFactory> for ::windows::core::IUnknown {
    fn from(value: IEncodingFilterFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEncodingFilterFactory> for ::windows::core::IUnknown {
    fn from(value: &IEncodingFilterFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEncodingFilterFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEncodingFilterFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEncodingFilterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, info: DATAINFO, ppdf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, ppdf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGetBindHandle(pub ::windows::core::IUnknown);
impl IGetBindHandle {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBindHandle(&self, enumrequestedhandle: BINDHANDLETYPES) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(enumrequestedhandle), &mut result__).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::core::Interface for IGetBindHandle {
    type Vtable = IGetBindHandle_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf0ff408_129d_4b20_91f0_02bd23d88352);
}
impl ::core::convert::From<IGetBindHandle> for ::windows::core::IUnknown {
    fn from(value: IGetBindHandle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGetBindHandle> for ::windows::core::IUnknown {
    fn from(value: &IGetBindHandle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGetBindHandle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGetBindHandle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetBindHandle_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHttpNegotiate(pub ::windows::core::IUnknown);
impl IHttpNegotiate {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginningTransaction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szurl: Param0, szheaders: Param1, dwreserved: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szurl.into_param().abi(), szheaders.into_param().abi(), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnResponse<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwresponsecode: u32, szresponseheaders: Param1, szrequestheaders: Param2) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwresponsecode), szresponseheaders.into_param().abi(), szrequestheaders.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IHttpNegotiate {
    type Vtable = IHttpNegotiate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d2_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IHttpNegotiate> for ::windows::core::IUnknown {
    fn from(value: IHttpNegotiate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHttpNegotiate> for ::windows::core::IUnknown {
    fn from(value: &IHttpNegotiate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHttpNegotiate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHttpNegotiate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, szheaders: super::super::super::Foundation::PWSTR, dwreserved: u32, pszadditionalheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwresponsecode: u32, szresponseheaders: super::super::super::Foundation::PWSTR, szrequestheaders: super::super::super::Foundation::PWSTR, pszadditionalrequestheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHttpNegotiate2(pub ::windows::core::IUnknown);
impl IHttpNegotiate2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginningTransaction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szurl: Param0, szheaders: Param1, dwreserved: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szurl.into_param().abi(), szheaders.into_param().abi(), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnResponse<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwresponsecode: u32, szresponseheaders: Param1, szrequestheaders: Param2) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwresponsecode), szresponseheaders.into_param().abi(), szrequestheaders.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsecurityid), ::core::mem::transmute(pcbsecurityid), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IHttpNegotiate2 {
    type Vtable = IHttpNegotiate2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f9f9fcb_e0f4_48eb_b7ab_fa2ea9365cb4);
}
impl ::core::convert::From<IHttpNegotiate2> for ::windows::core::IUnknown {
    fn from(value: IHttpNegotiate2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHttpNegotiate2> for ::windows::core::IUnknown {
    fn from(value: &IHttpNegotiate2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHttpNegotiate2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHttpNegotiate2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IHttpNegotiate2> for IHttpNegotiate {
    fn from(value: IHttpNegotiate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpNegotiate2> for IHttpNegotiate {
    fn from(value: &IHttpNegotiate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpNegotiate> for IHttpNegotiate2 {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpNegotiate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpNegotiate> for &IHttpNegotiate2 {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpNegotiate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, szheaders: super::super::super::Foundation::PWSTR, dwreserved: u32, pszadditionalheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwresponsecode: u32, szresponseheaders: super::super::super::Foundation::PWSTR, szrequestheaders: super::super::super::Foundation::PWSTR, pszadditionalrequestheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHttpNegotiate3(pub ::windows::core::IUnknown);
impl IHttpNegotiate3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginningTransaction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, szurl: Param0, szheaders: Param1, dwreserved: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szurl.into_param().abi(), szheaders.into_param().abi(), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnResponse<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwresponsecode: u32, szresponseheaders: Param1, szrequestheaders: Param2) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwresponsecode), szresponseheaders.into_param().abi(), szrequestheaders.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsecurityid), ::core::mem::transmute(pcbsecurityid), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetSerializedClientCertContext(&self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbcert), ::core::mem::transmute(pcbcert)).ok()
    }
}
unsafe impl ::windows::core::Interface for IHttpNegotiate3 {
    type Vtable = IHttpNegotiate3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57b6c80a_34c2_4602_bc26_66a02fc57153);
}
impl ::core::convert::From<IHttpNegotiate3> for ::windows::core::IUnknown {
    fn from(value: IHttpNegotiate3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHttpNegotiate3> for ::windows::core::IUnknown {
    fn from(value: &IHttpNegotiate3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHttpNegotiate3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHttpNegotiate3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IHttpNegotiate3> for IHttpNegotiate2 {
    fn from(value: IHttpNegotiate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpNegotiate3> for IHttpNegotiate2 {
    fn from(value: &IHttpNegotiate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpNegotiate2> for IHttpNegotiate3 {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpNegotiate2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpNegotiate2> for &IHttpNegotiate3 {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpNegotiate2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHttpNegotiate3> for IHttpNegotiate {
    fn from(value: IHttpNegotiate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpNegotiate3> for IHttpNegotiate {
    fn from(value: &IHttpNegotiate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpNegotiate> for IHttpNegotiate3 {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpNegotiate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IHttpNegotiate> for &IHttpNegotiate3 {
    fn into_param(self) -> ::windows::core::Param<'a, IHttpNegotiate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpNegotiate3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, szheaders: super::super::super::Foundation::PWSTR, dwreserved: u32, pszadditionalheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwresponsecode: u32, szresponseheaders: super::super::super::Foundation::PWSTR, szrequestheaders: super::super::super::Foundation::PWSTR, pszadditionalrequestheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHttpSecurity(pub ::windows::core::IUnknown);
impl IHttpSecurity {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidreason), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    pub unsafe fn OnSecurityProblem(&self, dwproblem: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproblem)).ok()
    }
}
unsafe impl ::windows::core::Interface for IHttpSecurity {
    type Vtable = IHttpSecurity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d7_bafa_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IHttpSecurity> for ::windows::core::IUnknown {
    fn from(value: IHttpSecurity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHttpSecurity> for ::windows::core::IUnknown {
    fn from(value: &IHttpSecurity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IHttpSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IHttpSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IHttpSecurity> for IWindowForBindingUI {
    fn from(value: IHttpSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHttpSecurity> for IWindowForBindingUI {
    fn from(value: &IHttpSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWindowForBindingUI> for IHttpSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, IWindowForBindingUI> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWindowForBindingUI> for &IHttpSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, IWindowForBindingUI> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHttpSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguidreason: *const ::windows::core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwproblem: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternet(pub ::windows::core::IUnknown);
impl IInternet {}
unsafe impl ::windows::core::Interface for IInternet {
    type Vtable = IInternet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e0_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternet> for ::windows::core::IUnknown {
    fn from(value: IInternet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternet> for ::windows::core::IUnknown {
    fn from(value: &IInternet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternet_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetBindInfo(pub ::windows::core::IUnknown);
impl IInternetBindInfo {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulstringtype), ::core::mem::transmute(ppwzstr), ::core::mem::transmute(cel), ::core::mem::transmute(pcelfetched)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetBindInfo {
    type Vtable = IInternetBindInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e1_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetBindInfo> for ::windows::core::IUnknown {
    fn from(value: IInternetBindInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetBindInfo> for ::windows::core::IUnknown {
    fn from(value: &IInternetBindInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetBindInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetBindInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetBindInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<super::BINDINFO>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetBindInfoEx(pub ::windows::core::IUnknown);
impl IInternetBindInfoEx {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulstringtype), ::core::mem::transmute(ppwzstr), ::core::mem::transmute(cel), ::core::mem::transmute(pcelfetched)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo), ::core::mem::transmute(grfbindf2), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetBindInfoEx {
    type Vtable = IInternetBindInfoEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3e015b7_a82c_4dcd_a150_569aeeed36ab);
}
impl ::core::convert::From<IInternetBindInfoEx> for ::windows::core::IUnknown {
    fn from(value: IInternetBindInfoEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetBindInfoEx> for ::windows::core::IUnknown {
    fn from(value: &IInternetBindInfoEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetBindInfoEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetBindInfoEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IInternetBindInfoEx> for IInternetBindInfo {
    fn from(value: IInternetBindInfoEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetBindInfoEx> for IInternetBindInfo {
    fn from(value: &IInternetBindInfoEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetBindInfo> for IInternetBindInfoEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetBindInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetBindInfo> for &IInternetBindInfoEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetBindInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetBindInfoEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<super::BINDINFO>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<super::BINDINFO>, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetHostSecurityManager(pub ::windows::core::IUnknown);
impl IInternetHostSecurityManager {
    pub unsafe fn GetSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbsecurityid), ::core::mem::transmute(pcbsecurityid), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn ProcessUrlAction(&self, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn QueryCustomPolicy(&self, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidkey), ::core::mem::transmute(pppolicy), ::core::mem::transmute(pcbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetHostSecurityManager {
    type Vtable = IInternetHostSecurityManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3af280b6_cb3f_11d0_891e_00c04fb6bfc4);
}
impl ::core::convert::From<IInternetHostSecurityManager> for ::windows::core::IUnknown {
    fn from(value: IInternetHostSecurityManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetHostSecurityManager> for ::windows::core::IUnknown {
    fn from(value: &IInternetHostSecurityManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetHostSecurityManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetHostSecurityManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetHostSecurityManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetPriority(pub ::windows::core::IUnknown);
impl IInternetPriority {
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(npriority)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IInternetPriority {
    type Vtable = IInternetPriority_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9eb_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetPriority> for ::windows::core::IUnknown {
    fn from(value: IInternetPriority) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetPriority> for ::windows::core::IUnknown {
    fn from(value: &IInternetPriority) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetPriority {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetPriority {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetPriority_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, npriority: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnpriority: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetProtocol(pub ::windows::core::IUnknown);
impl IInternetProtocol {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IInternetProtocolSink>, Param2: ::windows::core::IntoParam<'a, IInternetBindInfo>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE_PTR>>(&self, szurl: Param0, poiprotsink: Param1, poibindinfo: Param2, grfpi: u32, dwreserved: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szurl.into_param().abi(), poiprotsink.into_param().abi(), poibindinfo.into_param().abi(), ::core::mem::transmute(grfpi), dwreserved.into_param().abi()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprotocoldata)).ok()
    }
    pub unsafe fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrreason), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn LockRequest(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn UnlockRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocol {
    type Vtable = IInternetProtocol_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e4_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetProtocol> for ::windows::core::IUnknown {
    fn from(value: IInternetProtocol) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetProtocol> for ::windows::core::IUnknown {
    fn from(value: &IInternetProtocol) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetProtocol {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetProtocol {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IInternetProtocol> for IInternetProtocolRoot {
    fn from(value: IInternetProtocol) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetProtocol> for IInternetProtocolRoot {
    fn from(value: &IInternetProtocol) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetProtocolRoot> for IInternetProtocol {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetProtocolRoot> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetProtocolRoot> for &IInternetProtocol {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetProtocolRoot> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocol_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetProtocolEx(pub ::windows::core::IUnknown);
impl IInternetProtocolEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IInternetProtocolSink>, Param2: ::windows::core::IntoParam<'a, IInternetBindInfo>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE_PTR>>(&self, szurl: Param0, poiprotsink: Param1, poibindinfo: Param2, grfpi: u32, dwreserved: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szurl.into_param().abi(), poiprotsink.into_param().abi(), poibindinfo.into_param().abi(), ::core::mem::transmute(grfpi), dwreserved.into_param().abi()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprotocoldata)).ok()
    }
    pub unsafe fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrreason), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn LockRequest(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn UnlockRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartEx<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>, Param1: ::windows::core::IntoParam<'a, IInternetProtocolSink>, Param2: ::windows::core::IntoParam<'a, IInternetBindInfo>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE_PTR>>(&self, puri: Param0, poiprotsink: Param1, poibindinfo: Param2, grfpi: u32, dwreserved: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), puri.into_param().abi(), poiprotsink.into_param().abi(), poibindinfo.into_param().abi(), ::core::mem::transmute(grfpi), dwreserved.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolEx {
    type Vtable = IInternetProtocolEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7a98e66_1010_492c_a1c8_c809e1f75905);
}
impl ::core::convert::From<IInternetProtocolEx> for ::windows::core::IUnknown {
    fn from(value: IInternetProtocolEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetProtocolEx> for ::windows::core::IUnknown {
    fn from(value: &IInternetProtocolEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetProtocolEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetProtocolEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IInternetProtocolEx> for IInternetProtocol {
    fn from(value: IInternetProtocolEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetProtocolEx> for IInternetProtocol {
    fn from(value: &IInternetProtocolEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetProtocol> for IInternetProtocolEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetProtocol> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetProtocol> for &IInternetProtocolEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetProtocol> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInternetProtocolEx> for IInternetProtocolRoot {
    fn from(value: IInternetProtocolEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetProtocolEx> for IInternetProtocolRoot {
    fn from(value: &IInternetProtocolEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetProtocolRoot> for IInternetProtocolEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetProtocolRoot> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetProtocolRoot> for &IInternetProtocolEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetProtocolRoot> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetProtocolInfo(pub ::windows::core::IUnknown);
impl IInternetProtocolInfo {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParseUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzurl: Param0, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwzurl.into_param().abi(), ::core::mem::transmute(parseaction), ::core::mem::transmute(dwparseflags), ::core::mem::transmute(pwzresult), ::core::mem::transmute(cchresult), ::core::mem::transmute(pcchresult), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CombineUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzbaseurl: Param0, pwzrelativeurl: Param1, dwcombineflags: u32, pwzresult: Param3, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwzbaseurl.into_param().abi(), pwzrelativeurl.into_param().abi(), ::core::mem::transmute(dwcombineflags), pwzresult.into_param().abi(), ::core::mem::transmute(cchresult), ::core::mem::transmute(pcchresult), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CompareUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzurl1: Param0, pwzurl2: Param1, dwcompareflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwzurl1.into_param().abi(), pwzurl2.into_param().abi(), ::core::mem::transmute(dwcompareflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzurl: Param0, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwzurl.into_param().abi(), ::core::mem::transmute(oueryoption), ::core::mem::transmute(dwqueryflags), ::core::mem::transmute(pbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(pcbbuf), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolInfo {
    type Vtable = IInternetProtocolInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9ec_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetProtocolInfo> for ::windows::core::IUnknown {
    fn from(value: IInternetProtocolInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetProtocolInfo> for ::windows::core::IUnknown {
    fn from(value: &IInternetProtocolInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetProtocolInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetProtocolInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwcompareflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzurl: super::super::super::Foundation::PWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetProtocolRoot(pub ::windows::core::IUnknown);
impl IInternetProtocolRoot {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IInternetProtocolSink>, Param2: ::windows::core::IntoParam<'a, IInternetBindInfo>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE_PTR>>(&self, szurl: Param0, poiprotsink: Param1, poibindinfo: Param2, grfpi: u32, dwreserved: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szurl.into_param().abi(), poiprotsink.into_param().abi(), poibindinfo.into_param().abi(), ::core::mem::transmute(grfpi), dwreserved.into_param().abi()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprotocoldata)).ok()
    }
    pub unsafe fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrreason), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolRoot {
    type Vtable = IInternetProtocolRoot_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e3_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetProtocolRoot> for ::windows::core::IUnknown {
    fn from(value: IInternetProtocolRoot) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetProtocolRoot> for ::windows::core::IUnknown {
    fn from(value: &IInternetProtocolRoot) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetProtocolRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetProtocolRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolRoot_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoptions: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetProtocolSink(pub ::windows::core::IUnknown);
impl IInternetProtocolSink {
    pub unsafe fn Switch(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprotocoldata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportProgress<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, ulstatuscode: u32, szstatustext: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    pub unsafe fn ReportData(&self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportResult<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrresult), ::core::mem::transmute(dwerror), szresult.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolSink {
    type Vtable = IInternetProtocolSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e5_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetProtocolSink> for ::windows::core::IUnknown {
    fn from(value: IInternetProtocolSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetProtocolSink> for ::windows::core::IUnknown {
    fn from(value: &IInternetProtocolSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetProtocolSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetProtocolSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulstatuscode: u32, szstatustext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetProtocolSinkStackable(pub ::windows::core::IUnknown);
impl IInternetProtocolSinkStackable {
    pub unsafe fn SwitchSink<'a, Param0: ::windows::core::IntoParam<'a, IInternetProtocolSink>>(&self, poiprotsink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), poiprotsink.into_param().abi()).ok()
    }
    pub unsafe fn CommitSwitch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RollbackSwitch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetProtocolSinkStackable {
    type Vtable = IInternetProtocolSinkStackable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9f0_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetProtocolSinkStackable> for ::windows::core::IUnknown {
    fn from(value: IInternetProtocolSinkStackable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetProtocolSinkStackable> for ::windows::core::IUnknown {
    fn from(value: &IInternetProtocolSinkStackable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetProtocolSinkStackable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetProtocolSinkStackable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetProtocolSinkStackable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, poiprotsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetSecurityManager(pub ::windows::core::IUnknown);
impl IInternetSecurityManager {
    pub unsafe fn SetSecuritySite<'a, Param0: ::windows::core::IntoParam<'a, IInternetSecurityMgrSite>>(&self, psite: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psite.into_param().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite> {
        let mut result__: <IInternetSecurityMgrSite as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IInternetSecurityMgrSite>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapUrlToZone<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(pdwzone), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurityId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(pbsecurityid), ::core::mem::transmute(pcbsecurityid), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessUrlAction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryCustomPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(guidkey), ::core::mem::transmute(pppolicy), ::core::mem::transmute(pcbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetZoneMapping<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwzone: u32, lpszpattern: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), lpszpattern.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(ppenumstring), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetSecurityManager {
    type Vtable = IInternetSecurityManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9ee_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetSecurityManager> for ::windows::core::IUnknown {
    fn from(value: IInternetSecurityManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetSecurityManager> for ::windows::core::IUnknown {
    fn from(value: &IInternetSecurityManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetSecurityManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetSecurityManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psite: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, lpszpattern: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, ppenumstring: *mut ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetSecurityManagerEx(pub ::windows::core::IUnknown);
impl IInternetSecurityManagerEx {
    pub unsafe fn SetSecuritySite<'a, Param0: ::windows::core::IntoParam<'a, IInternetSecurityMgrSite>>(&self, psite: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psite.into_param().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite> {
        let mut result__: <IInternetSecurityMgrSite as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IInternetSecurityMgrSite>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapUrlToZone<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(pdwzone), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurityId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(pbsecurityid), ::core::mem::transmute(pcbsecurityid), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessUrlAction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryCustomPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(guidkey), ::core::mem::transmute(pppolicy), ::core::mem::transmute(pcbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetZoneMapping<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwzone: u32, lpszpattern: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), lpszpattern.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(ppenumstring), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessUrlActionEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(
            ::core::mem::transmute_copy(self),
            pwszurl.into_param().abi(),
            ::core::mem::transmute(dwaction),
            ::core::mem::transmute(ppolicy),
            ::core::mem::transmute(cbpolicy),
            ::core::mem::transmute(pcontext),
            ::core::mem::transmute(cbcontext),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(dwreserved),
            ::core::mem::transmute(pdwoutflags),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetSecurityManagerEx {
    type Vtable = IInternetSecurityManagerEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf164edf1_cc7c_4f0d_9a94_34222625c393);
}
impl ::core::convert::From<IInternetSecurityManagerEx> for ::windows::core::IUnknown {
    fn from(value: IInternetSecurityManagerEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetSecurityManagerEx> for ::windows::core::IUnknown {
    fn from(value: &IInternetSecurityManagerEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetSecurityManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetSecurityManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IInternetSecurityManagerEx> for IInternetSecurityManager {
    fn from(value: IInternetSecurityManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetSecurityManagerEx> for IInternetSecurityManager {
    fn from(value: &IInternetSecurityManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetSecurityManager> for IInternetSecurityManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetSecurityManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetSecurityManager> for &IInternetSecurityManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetSecurityManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManagerEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psite: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, lpszpattern: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, ppenumstring: *mut ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetSecurityManagerEx2(pub ::windows::core::IUnknown);
impl IInternetSecurityManagerEx2 {
    pub unsafe fn SetSecuritySite<'a, Param0: ::windows::core::IntoParam<'a, IInternetSecurityMgrSite>>(&self, psite: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psite.into_param().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite> {
        let mut result__: <IInternetSecurityMgrSite as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IInternetSecurityMgrSite>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapUrlToZone<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(pdwzone), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurityId<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(pbsecurityid), ::core::mem::transmute(pcbsecurityid), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessUrlAction<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryCustomPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), ::core::mem::transmute(guidkey), ::core::mem::transmute(pppolicy), ::core::mem::transmute(pcbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetZoneMapping<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwzone: u32, lpszpattern: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), lpszpattern.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(ppenumstring), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessUrlActionEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwszurl: Param0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(
            ::core::mem::transmute_copy(self),
            pwszurl.into_param().abi(),
            ::core::mem::transmute(dwaction),
            ::core::mem::transmute(ppolicy),
            ::core::mem::transmute(cbpolicy),
            ::core::mem::transmute(pcontext),
            ::core::mem::transmute(cbcontext),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(dwreserved),
            ::core::mem::transmute(pdwoutflags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapUrlToZoneEx2<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>>(&self, puri: Param0, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut super::super::super::Foundation::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), puri.into_param().abi(), ::core::mem::transmute(pdwzone), ::core::mem::transmute(dwflags), ::core::mem::transmute(ppwszmappedurl), ::core::mem::transmute(pdwoutflags)).ok()
    }
    pub unsafe fn ProcessUrlActionEx2<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>>(&self, puri: Param0, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(
            ::core::mem::transmute_copy(self),
            puri.into_param().abi(),
            ::core::mem::transmute(dwaction),
            ::core::mem::transmute(ppolicy),
            ::core::mem::transmute(cbpolicy),
            ::core::mem::transmute(pcontext),
            ::core::mem::transmute(cbcontext),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(dwreserved),
            ::core::mem::transmute(pdwoutflags),
        )
        .ok()
    }
    pub unsafe fn GetSecurityIdEx2<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>>(&self, puri: Param0, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), puri.into_param().abi(), ::core::mem::transmute(pbsecurityid), ::core::mem::transmute(pcbsecurityid), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn QueryCustomPolicyEx2<'a, Param0: ::windows::core::IntoParam<'a, super::IUri>>(&self, puri: Param0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), puri.into_param().abi(), ::core::mem::transmute(guidkey), ::core::mem::transmute(pppolicy), ::core::mem::transmute(pcbpolicy), ::core::mem::transmute(pcontext), ::core::mem::transmute(cbcontext), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetSecurityManagerEx2 {
    type Vtable = IInternetSecurityManagerEx2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1e50292_a795_4117_8e09_2b560a72ac60);
}
impl ::core::convert::From<IInternetSecurityManagerEx2> for ::windows::core::IUnknown {
    fn from(value: IInternetSecurityManagerEx2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetSecurityManagerEx2> for ::windows::core::IUnknown {
    fn from(value: &IInternetSecurityManagerEx2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetSecurityManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetSecurityManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IInternetSecurityManagerEx2> for IInternetSecurityManagerEx {
    fn from(value: IInternetSecurityManagerEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetSecurityManagerEx2> for IInternetSecurityManagerEx {
    fn from(value: &IInternetSecurityManagerEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetSecurityManagerEx> for IInternetSecurityManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetSecurityManagerEx> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetSecurityManagerEx> for &IInternetSecurityManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetSecurityManagerEx> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInternetSecurityManagerEx2> for IInternetSecurityManager {
    fn from(value: IInternetSecurityManagerEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetSecurityManagerEx2> for IInternetSecurityManager {
    fn from(value: &IInternetSecurityManagerEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetSecurityManager> for IInternetSecurityManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetSecurityManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetSecurityManager> for &IInternetSecurityManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetSecurityManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityManagerEx2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psite: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, lpszpattern: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, ppenumstring: *mut ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut super::super::super::Foundation::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetSecurityMgrSite(pub ::windows::core::IUnknown);
impl IInternetSecurityMgrSite {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableModeless<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetSecurityMgrSite {
    type Vtable = IInternetSecurityMgrSite_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9ed_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetSecurityMgrSite> for ::windows::core::IUnknown {
    fn from(value: IInternetSecurityMgrSite) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetSecurityMgrSite> for ::windows::core::IUnknown {
    fn from(value: &IInternetSecurityMgrSite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetSecurityMgrSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetSecurityMgrSite {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSecurityMgrSite_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetSession(pub ::windows::core::IUnknown);
impl IInternetSession {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterNameSpace<'a, Param0: ::windows::core::IntoParam<'a, super::IClassFactory>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pcf: Param0, rclsid: *const ::windows::core::GUID, pwzprotocol: Param2, cpatterns: u32, ppwzpatterns: *const super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcf.into_param().abi(), ::core::mem::transmute(rclsid), pwzprotocol.into_param().abi(), ::core::mem::transmute(cpatterns), ::core::mem::transmute(ppwzpatterns), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterNameSpace<'a, Param0: ::windows::core::IntoParam<'a, super::IClassFactory>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pcf: Param0, pszprotocol: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcf.into_param().abi(), pszprotocol.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterMimeFilter<'a, Param0: ::windows::core::IntoParam<'a, super::IClassFactory>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pcf: Param0, rclsid: *const ::windows::core::GUID, pwztype: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcf.into_param().abi(), ::core::mem::transmute(rclsid), pwztype.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterMimeFilter<'a, Param0: ::windows::core::IntoParam<'a, super::IClassFactory>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pcf: Param0, pwztype: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pcf.into_param().abi(), pwztype.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBinding<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pbc: Param0, szurl: Param1, punkouter: Param2, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>, ppoinetprot: *mut ::core::option::Option<IInternetProtocol>, dwoption: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pbc.into_param().abi(), szurl.into_param().abi(), punkouter.into_param().abi(), ::core::mem::transmute(ppunk), ::core::mem::transmute(ppoinetprot), ::core::mem::transmute(dwoption)).ok()
    }
    pub unsafe fn SetSessionOption(&self, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoption), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwbufferlength), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetSessionOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoption), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwbufferlength), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetSession {
    type Vtable = IInternetSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e7_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetSession> for ::windows::core::IUnknown {
    fn from(value: IInternetSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetSession> for ::windows::core::IUnknown {
    fn from(value: &IInternetSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcf: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, pwzprotocol: super::super::super::Foundation::PWSTR, cpatterns: u32, ppwzpatterns: *const super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcf: ::windows::core::RawPtr, pszprotocol: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcf: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcf: ::windows::core::RawPtr, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, punkouter: ::windows::core::RawPtr, ppunk: *mut ::windows::core::RawPtr, ppoinetprot: *mut ::windows::core::RawPtr, dwoption: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetThreadSwitch(pub ::windows::core::IUnknown);
impl IInternetThreadSwitch {
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Continue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetThreadSwitch {
    type Vtable = IInternetThreadSwitch_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9e8_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetThreadSwitch> for ::windows::core::IUnknown {
    fn from(value: IInternetThreadSwitch) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetThreadSwitch> for ::windows::core::IUnknown {
    fn from(value: &IInternetThreadSwitch) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetThreadSwitch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetThreadSwitch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetThreadSwitch_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetZoneManager(pub ::windows::core::IUnknown);
impl IInternetZoneManager {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(pzoneattributes)).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(pzoneattributes)).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(guidkey), ::core::mem::transmute(pppolicy), ::core::mem::transmute(pcbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(guidkey), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptAction<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwaction: u32, hwndparent: Param1, pwszurl: Param2, pwsztext: Param3, dwpromptflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaction), hwndparent.into_param().abi(), pwszurl.into_param().abi(), pwsztext.into_param().abi(), ::core::mem::transmute(dwpromptflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogAction<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwaction: u32, pwszurl: Param1, pwsztext: Param2, dwlogflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaction), pwszurl.into_param().abi(), pwsztext.into_param().abi(), ::core::mem::transmute(dwlogflags)).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwenum), ::core::mem::transmute(pdwcount), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenum), ::core::mem::transmute(dwindex), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenum)).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtemplate), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetZoneManager {
    type Vtable = IInternetZoneManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9ef_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IInternetZoneManager> for ::windows::core::IUnknown {
    fn from(value: IInternetZoneManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetZoneManager> for ::windows::core::IUnknown {
    fn from(value: &IInternetZoneManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetZoneManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetZoneManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwpromptflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaction: u32, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwlogflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwenum: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetZoneManagerEx(pub ::windows::core::IUnknown);
impl IInternetZoneManagerEx {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(pzoneattributes)).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(pzoneattributes)).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(guidkey), ::core::mem::transmute(pppolicy), ::core::mem::transmute(pcbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(guidkey), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptAction<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwaction: u32, hwndparent: Param1, pwszurl: Param2, pwsztext: Param3, dwpromptflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaction), hwndparent.into_param().abi(), pwszurl.into_param().abi(), pwsztext.into_param().abi(), ::core::mem::transmute(dwpromptflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogAction<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwaction: u32, pwszurl: Param1, pwsztext: Param2, dwlogflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaction), pwszurl.into_param().abi(), pwsztext.into_param().abi(), ::core::mem::transmute(dwlogflags)).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwenum), ::core::mem::transmute(pdwcount), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenum), ::core::mem::transmute(dwindex), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenum)).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtemplate), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetZoneManagerEx {
    type Vtable = IInternetZoneManagerEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4c23339_8e06_431e_9bf4_7e711c085648);
}
impl ::core::convert::From<IInternetZoneManagerEx> for ::windows::core::IUnknown {
    fn from(value: IInternetZoneManagerEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetZoneManagerEx> for ::windows::core::IUnknown {
    fn from(value: &IInternetZoneManagerEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetZoneManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetZoneManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IInternetZoneManagerEx> for IInternetZoneManager {
    fn from(value: IInternetZoneManagerEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetZoneManagerEx> for IInternetZoneManager {
    fn from(value: &IInternetZoneManagerEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetZoneManager> for IInternetZoneManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetZoneManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetZoneManager> for &IInternetZoneManagerEx {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetZoneManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManagerEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwpromptflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaction: u32, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwlogflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwenum: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternetZoneManagerEx2(pub ::windows::core::IUnknown);
impl IInternetZoneManagerEx2 {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(pzoneattributes)).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(pzoneattributes)).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(guidkey), ::core::mem::transmute(pppolicy), ::core::mem::transmute(pcbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(guidkey), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptAction<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwaction: u32, hwndparent: Param1, pwszurl: Param2, pwsztext: Param3, dwpromptflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaction), hwndparent.into_param().abi(), pwszurl.into_param().abi(), pwsztext.into_param().abi(), ::core::mem::transmute(dwpromptflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogAction<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwaction: u32, pwszurl: Param1, pwsztext: Param2, dwlogflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaction), pwszurl.into_param().abi(), pwsztext.into_param().abi(), ::core::mem::transmute(dwlogflags)).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwenum), ::core::mem::transmute(pdwcount), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenum), ::core::mem::transmute(dwindex), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwenum)).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtemplate), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(dwaction), ::core::mem::transmute(ppolicy), ::core::mem::transmute(cbpolicy), ::core::mem::transmute(urlzonereg), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetZoneAttributesEx(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone), ::core::mem::transmute(pzoneattributes), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetZoneSecurityState<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, dwzoneindex: u32, frespectpolicy: Param1, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzoneindex), frespectpolicy.into_param().abi(), ::core::mem::transmute(pdwstate), ::core::mem::transmute(pfpolicyencountered)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIESecurityState<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, frespectpolicy: Param0, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), frespectpolicy.into_param().abi(), ::core::mem::transmute(pdwstate), ::core::mem::transmute(pfpolicyencountered), fnocache.into_param().abi()).ok()
    }
    pub unsafe fn FixUnsecureSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternetZoneManagerEx2 {
    type Vtable = IInternetZoneManagerEx2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedc17559_dd5d_4846_8eef_8becba5a4abf);
}
impl ::core::convert::From<IInternetZoneManagerEx2> for ::windows::core::IUnknown {
    fn from(value: IInternetZoneManagerEx2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternetZoneManagerEx2> for ::windows::core::IUnknown {
    fn from(value: &IInternetZoneManagerEx2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternetZoneManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternetZoneManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IInternetZoneManagerEx2> for IInternetZoneManagerEx {
    fn from(value: IInternetZoneManagerEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetZoneManagerEx2> for IInternetZoneManagerEx {
    fn from(value: &IInternetZoneManagerEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetZoneManagerEx> for IInternetZoneManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetZoneManagerEx> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetZoneManagerEx> for &IInternetZoneManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetZoneManagerEx> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInternetZoneManagerEx2> for IInternetZoneManager {
    fn from(value: IInternetZoneManagerEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternetZoneManagerEx2> for IInternetZoneManager {
    fn from(value: &IInternetZoneManagerEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetZoneManager> for IInternetZoneManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetZoneManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInternetZoneManager> for &IInternetZoneManagerEx2 {
    fn into_param(self) -> ::windows::core::Param<'a, IInternetZoneManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetZoneManagerEx2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwpromptflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaction: u32, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwlogflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwenum: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMonikerProp(pub ::windows::core::IUnknown);
impl IMonikerProp {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PutProperty<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, mkp: MONIKERPROPERTY, val: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(mkp), val.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMonikerProp {
    type Vtable = IMonikerProp_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ca5f7f_1847_4d87_9c5b_918509f7511d);
}
impl ::core::convert::From<IMonikerProp> for ::windows::core::IUnknown {
    fn from(value: IMonikerProp) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMonikerProp> for ::windows::core::IUnknown {
    fn from(value: &IMonikerProp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMonikerProp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMonikerProp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMonikerProp_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mkp: MONIKERPROPERTY, val: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const INET_E_AUTHENTICATION_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697207i32 as _);
pub const INET_E_BLOCKED_ENHANCEDPROTECTEDMODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695930i32 as _);
pub const INET_E_BLOCKED_PLUGGABLE_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695931i32 as _);
pub const INET_E_BLOCKED_REDIRECT_XSECURITYID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697189i32 as _);
pub const INET_E_CANNOT_CONNECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697212i32 as _);
pub const INET_E_CANNOT_INSTANTIATE_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697200i32 as _);
pub const INET_E_CANNOT_LOAD_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697201i32 as _);
pub const INET_E_CANNOT_LOCK_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697194i32 as _);
pub const INET_E_CANNOT_REPLACE_SFP_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146696448i32 as _);
pub const INET_E_CODE_DOWNLOAD_DECLINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146696960i32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_ARM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695932i32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_BITNESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695929i32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695936i32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695934i32 as _);
pub const INET_E_CODE_INSTALL_SUPPRESSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146696192i32 as _);
pub const INET_E_CONNECTION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697205i32 as _);
pub const INET_E_DATA_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697209i32 as _);
pub const INET_E_DEFAULT_ACTION: i32 = -2146697199i32;
pub const INET_E_DOMINJECTIONVALIDATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697188i32 as _);
pub const INET_E_DOWNLOAD_BLOCKED_BY_CSP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695928i32 as _);
pub const INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695935i32 as _);
pub const INET_E_DOWNLOAD_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697208i32 as _);
pub const INET_E_ERROR_FIRST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697214i32 as _);
pub const INET_E_ERROR_LAST: i32 = -2146695928i32;
pub const INET_E_FORBIDFRAMING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146695933i32 as _);
pub const INET_E_HSTS_CERTIFICATE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697186i32 as _);
pub const INET_E_INVALID_CERTIFICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697191i32 as _);
pub const INET_E_INVALID_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697204i32 as _);
pub const INET_E_INVALID_URL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697214i32 as _);
pub const INET_E_NO_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697213i32 as _);
pub const INET_E_NO_VALID_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697206i32 as _);
pub const INET_E_OBJECT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697210i32 as _);
pub const INET_E_QUERYOPTION_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697197i32 as _);
pub const INET_E_REDIRECTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697196i32 as _);
pub const INET_E_REDIRECT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697196i32 as _);
pub const INET_E_REDIRECT_TO_DIR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697195i32 as _);
pub const INET_E_RESERVED_1: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697190i32 as _);
pub const INET_E_RESERVED_2: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697185i32 as _);
pub const INET_E_RESERVED_3: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697184i32 as _);
pub const INET_E_RESERVED_4: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697183i32 as _);
pub const INET_E_RESERVED_5: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697182i32 as _);
pub const INET_E_RESOURCE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697211i32 as _);
pub const INET_E_RESULT_DISPATCHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146696704i32 as _);
pub const INET_E_SECURITY_PROBLEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697202i32 as _);
pub const INET_E_TERMINATED_BIND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697192i32 as _);
pub const INET_E_UNKNOWN_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697203i32 as _);
pub const INET_E_USE_DEFAULT_PROTOCOLHANDLER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697199i32 as _);
pub const INET_E_USE_DEFAULT_SETTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697198i32 as _);
pub const INET_E_USE_EXTEND_BINDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697193i32 as _);
pub const INET_E_VTAB_SWITCH_FORCE_ENGINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146697187i32 as _);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INET_ZONE_MANAGER_CONSTANTS(pub i32);
pub const MAX_ZONE_PATH: INET_ZONE_MANAGER_CONSTANTS = INET_ZONE_MANAGER_CONSTANTS(260i32);
pub const MAX_ZONE_DESCRIPTION: INET_ZONE_MANAGER_CONSTANTS = INET_ZONE_MANAGER_CONSTANTS(200i32);
impl ::core::convert::From<i32> for INET_ZONE_MANAGER_CONSTANTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INET_ZONE_MANAGER_CONSTANTS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERNETFEATURELIST(pub i32);
pub const FEATURE_OBJECT_CACHING: INTERNETFEATURELIST = INTERNETFEATURELIST(0i32);
pub const FEATURE_ZONE_ELEVATION: INTERNETFEATURELIST = INTERNETFEATURELIST(1i32);
pub const FEATURE_MIME_HANDLING: INTERNETFEATURELIST = INTERNETFEATURELIST(2i32);
pub const FEATURE_MIME_SNIFFING: INTERNETFEATURELIST = INTERNETFEATURELIST(3i32);
pub const FEATURE_WINDOW_RESTRICTIONS: INTERNETFEATURELIST = INTERNETFEATURELIST(4i32);
pub const FEATURE_WEBOC_POPUPMANAGEMENT: INTERNETFEATURELIST = INTERNETFEATURELIST(5i32);
pub const FEATURE_BEHAVIORS: INTERNETFEATURELIST = INTERNETFEATURELIST(6i32);
pub const FEATURE_DISABLE_MK_PROTOCOL: INTERNETFEATURELIST = INTERNETFEATURELIST(7i32);
pub const FEATURE_LOCALMACHINE_LOCKDOWN: INTERNETFEATURELIST = INTERNETFEATURELIST(8i32);
pub const FEATURE_SECURITYBAND: INTERNETFEATURELIST = INTERNETFEATURELIST(9i32);
pub const FEATURE_RESTRICT_ACTIVEXINSTALL: INTERNETFEATURELIST = INTERNETFEATURELIST(10i32);
pub const FEATURE_VALIDATE_NAVIGATE_URL: INTERNETFEATURELIST = INTERNETFEATURELIST(11i32);
pub const FEATURE_RESTRICT_FILEDOWNLOAD: INTERNETFEATURELIST = INTERNETFEATURELIST(12i32);
pub const FEATURE_ADDON_MANAGEMENT: INTERNETFEATURELIST = INTERNETFEATURELIST(13i32);
pub const FEATURE_PROTOCOL_LOCKDOWN: INTERNETFEATURELIST = INTERNETFEATURELIST(14i32);
pub const FEATURE_HTTP_USERNAME_PASSWORD_DISABLE: INTERNETFEATURELIST = INTERNETFEATURELIST(15i32);
pub const FEATURE_SAFE_BINDTOOBJECT: INTERNETFEATURELIST = INTERNETFEATURELIST(16i32);
pub const FEATURE_UNC_SAVEDFILECHECK: INTERNETFEATURELIST = INTERNETFEATURELIST(17i32);
pub const FEATURE_GET_URL_DOM_FILEPATH_UNENCODED: INTERNETFEATURELIST = INTERNETFEATURELIST(18i32);
pub const FEATURE_TABBED_BROWSING: INTERNETFEATURELIST = INTERNETFEATURELIST(19i32);
pub const FEATURE_SSLUX: INTERNETFEATURELIST = INTERNETFEATURELIST(20i32);
pub const FEATURE_DISABLE_NAVIGATION_SOUNDS: INTERNETFEATURELIST = INTERNETFEATURELIST(21i32);
pub const FEATURE_DISABLE_LEGACY_COMPRESSION: INTERNETFEATURELIST = INTERNETFEATURELIST(22i32);
pub const FEATURE_FORCE_ADDR_AND_STATUS: INTERNETFEATURELIST = INTERNETFEATURELIST(23i32);
pub const FEATURE_XMLHTTP: INTERNETFEATURELIST = INTERNETFEATURELIST(24i32);
pub const FEATURE_DISABLE_TELNET_PROTOCOL: INTERNETFEATURELIST = INTERNETFEATURELIST(25i32);
pub const FEATURE_FEEDS: INTERNETFEATURELIST = INTERNETFEATURELIST(26i32);
pub const FEATURE_BLOCK_INPUT_PROMPTS: INTERNETFEATURELIST = INTERNETFEATURELIST(27i32);
pub const FEATURE_ENTRY_COUNT: INTERNETFEATURELIST = INTERNETFEATURELIST(28i32);
impl ::core::convert::From<i32> for INTERNETFEATURELIST {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INTERNETFEATURELIST {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistMoniker(pub ::windows::core::IUnknown);
impl IPersistMoniker {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::IMoniker>, Param2: ::windows::core::IntoParam<'a, super::IBindCtx>>(&self, ffullyavailable: Param0, pimkname: Param1, pibc: Param2, grfmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ffullyavailable.into_param().abi(), pimkname.into_param().abi(), pibc.into_param().abi(), ::core::mem::transmute(grfmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::IMoniker>, Param1: ::windows::core::IntoParam<'a, super::IBindCtx>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, pimkname: Param0, pbc: Param1, fremember: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pimkname.into_param().abi(), pbc.into_param().abi(), fremember.into_param().abi()).ok()
    }
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::IMoniker>, Param1: ::windows::core::IntoParam<'a, super::IBindCtx>>(&self, pimkname: Param0, pibc: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pimkname.into_param().abi(), pibc.into_param().abi()).ok()
    }
    pub unsafe fn GetCurMoniker(&self) -> ::windows::core::Result<super::IMoniker> {
        let mut result__: <super::IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IMoniker>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPersistMoniker {
    type Vtable = IPersistMoniker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c9_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IPersistMoniker> for ::windows::core::IUnknown {
    fn from(value: IPersistMoniker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistMoniker> for ::windows::core::IUnknown {
    fn from(value: &IPersistMoniker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMoniker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: ::windows::core::RawPtr, pibc: ::windows::core::RawPtr, grfmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pimkname: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pimkname: ::windows::core::RawPtr, pibc: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppimkname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISoftDistExt(pub ::windows::core::IUnknown);
impl ISoftDistExt {
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub unsafe fn ProcessSoftDist<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Data::Xml::MsXml::IXMLElement>>(&self, szcdfurl: Param0, psoftdistelement: Param1, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szcdfurl.into_param().abi(), psoftdistelement.into_param().abi(), ::core::mem::transmute(lpsdi)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFirstCodeBase(&self, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(szcodebase), ::core::mem::transmute(dwmaxsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextCodeBase(&self, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(szcodebase), ::core::mem::transmute(dwmaxsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AsyncInstallDistributionUnit<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>>(&self, pbc: Param0, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pbc.into_param().abi(), ::core::mem::transmute(pvreserved), ::core::mem::transmute(flags), ::core::mem::transmute(lpcbh)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISoftDistExt {
    type Vtable = ISoftDistExt_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb15b8dc1_c7e1_11d0_8680_00aa00bdcb71);
}
impl ::core::convert::From<ISoftDistExt> for ::windows::core::IUnknown {
    fn from(value: ISoftDistExt) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISoftDistExt> for ::windows::core::IUnknown {
    fn from(value: &ISoftDistExt) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISoftDistExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISoftDistExt {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftDistExt_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szcdfurl: super::super::super::Foundation::PWSTR, psoftdistelement: ::windows::core::RawPtr, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUriBuilderFactory(pub ::windows::core::IUnknown);
impl IUriBuilderFactory {
    pub unsafe fn CreateIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder> {
        let mut result__: <super::IUriBuilder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<super::IUriBuilder>(result__)
    }
    pub unsafe fn CreateInitializedIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder> {
        let mut result__: <super::IUriBuilder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<super::IUriBuilder>(result__)
    }
}
unsafe impl ::windows::core::Interface for IUriBuilderFactory {
    type Vtable = IUriBuilderFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe982ce48_0b96_440c_bc37_0c869b27a29e);
}
impl ::core::convert::From<IUriBuilderFactory> for ::windows::core::IUnknown {
    fn from(value: IUriBuilderFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUriBuilderFactory> for ::windows::core::IUnknown {
    fn from(value: &IUriBuilderFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUriBuilderFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUriBuilderFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUriContainer(pub ::windows::core::IUnknown);
impl IUriContainer {
    pub unsafe fn GetIUri(&self) -> ::windows::core::Result<super::IUri> {
        let mut result__: <super::IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IUri>(result__)
    }
}
unsafe impl ::windows::core::Interface for IUriContainer {
    type Vtable = IUriContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa158a630_ed6f_45fb_b987_f68676f57752);
}
impl ::core::convert::From<IUriContainer> for ::windows::core::IUnknown {
    fn from(value: IUriContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUriContainer> for ::windows::core::IUnknown {
    fn from(value: &IUriContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUriContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUriContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinInetCacheHints(pub ::windows::core::IUnknown);
impl IWinInetCacheHints {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCacheExtension<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzext: Param0, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwzext.into_param().abi(), ::core::mem::transmute(pszcachefile), ::core::mem::transmute(pcbcachefile), ::core::mem::transmute(pdwwinineterror), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWinInetCacheHints {
    type Vtable = IWinInetCacheHints_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd1ec3b3_8391_4fdb_a9e6_347c3caaa7dd);
}
impl ::core::convert::From<IWinInetCacheHints> for ::windows::core::IUnknown {
    fn from(value: IWinInetCacheHints) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinInetCacheHints> for ::windows::core::IUnknown {
    fn from(value: &IWinInetCacheHints) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinInetCacheHints {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinInetCacheHints {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetCacheHints_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzext: super::super::super::Foundation::PWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinInetCacheHints2(pub ::windows::core::IUnknown);
impl IWinInetCacheHints2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCacheExtension<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzext: Param0, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwzext.into_param().abi(), ::core::mem::transmute(pszcachefile), ::core::mem::transmute(pcbcachefile), ::core::mem::transmute(pdwwinineterror), ::core::mem::transmute(pdwreserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCacheExtension2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, pwzext: Param0, pwzcachefile: super::super::super::Foundation::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwzext.into_param().abi(), ::core::mem::transmute(pwzcachefile), ::core::mem::transmute(pcchcachefile), ::core::mem::transmute(pdwwinineterror), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWinInetCacheHints2 {
    type Vtable = IWinInetCacheHints2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7857aeac_d31f_49bf_884e_dd46df36780a);
}
impl ::core::convert::From<IWinInetCacheHints2> for ::windows::core::IUnknown {
    fn from(value: IWinInetCacheHints2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinInetCacheHints2> for ::windows::core::IUnknown {
    fn from(value: &IWinInetCacheHints2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinInetCacheHints2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinInetCacheHints2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWinInetCacheHints2> for IWinInetCacheHints {
    fn from(value: IWinInetCacheHints2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWinInetCacheHints2> for IWinInetCacheHints {
    fn from(value: &IWinInetCacheHints2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWinInetCacheHints> for IWinInetCacheHints2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWinInetCacheHints> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWinInetCacheHints> for &IWinInetCacheHints2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWinInetCacheHints> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetCacheHints2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzext: super::super::super::Foundation::PWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzext: super::super::super::Foundation::PWSTR, pwzcachefile: super::super::super::Foundation::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinInetFileStream(pub ::windows::core::IUnknown);
impl IWinInetFileStream {
    pub unsafe fn SetHandleForUnlock(&self, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwininetlockhandle), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn SetDeleteFile(&self, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWinInetFileStream {
    type Vtable = IWinInetFileStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf134c4b7_b1f8_4e75_b886_74b90943becb);
}
impl ::core::convert::From<IWinInetFileStream> for ::windows::core::IUnknown {
    fn from(value: IWinInetFileStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinInetFileStream> for ::windows::core::IUnknown {
    fn from(value: &IWinInetFileStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinInetFileStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinInetFileStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetFileStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwreserved: usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinInetHttpInfo(pub ::windows::core::IUnknown);
impl IWinInetHttpInfo {
    pub unsafe fn QueryOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoption), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pcbbuf)).ok()
    }
    pub unsafe fn QueryInfo(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoption), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pcbbuf), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWinInetHttpInfo {
    type Vtable = IWinInetHttpInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d8_bafa_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IWinInetHttpInfo> for ::windows::core::IUnknown {
    fn from(value: IWinInetHttpInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinInetHttpInfo> for ::windows::core::IUnknown {
    fn from(value: &IWinInetHttpInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinInetHttpInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinInetHttpInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IWinInetHttpInfo> for IWinInetInfo {
    fn from(value: IWinInetHttpInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWinInetHttpInfo> for IWinInetInfo {
    fn from(value: &IWinInetHttpInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWinInetInfo> for IWinInetHttpInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWinInetInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWinInetInfo> for &IWinInetHttpInfo {
    fn into_param(self) -> ::windows::core::Param<'a, IWinInetInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetHttpInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinInetHttpTimeouts(pub ::windows::core::IUnknown);
impl IWinInetHttpTimeouts {
    pub unsafe fn GetRequestTimeouts(&self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwconnecttimeout), ::core::mem::transmute(pdwsendtimeout), ::core::mem::transmute(pdwreceivetimeout)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWinInetHttpTimeouts {
    type Vtable = IWinInetHttpTimeouts_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf286fa56_c1fd_4270_8e67_b3eb790a81e8);
}
impl ::core::convert::From<IWinInetHttpTimeouts> for ::windows::core::IUnknown {
    fn from(value: IWinInetHttpTimeouts) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinInetHttpTimeouts> for ::windows::core::IUnknown {
    fn from(value: &IWinInetHttpTimeouts) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinInetHttpTimeouts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinInetHttpTimeouts {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetHttpTimeouts_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWinInetInfo(pub ::windows::core::IUnknown);
impl IWinInetInfo {
    pub unsafe fn QueryOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoption), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pcbbuf)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWinInetInfo {
    type Vtable = IWinInetInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d6_bafa_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IWinInetInfo> for ::windows::core::IUnknown {
    fn from(value: IWinInetInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWinInetInfo> for ::windows::core::IUnknown {
    fn from(value: &IWinInetInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWinInetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWinInetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWinInetInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWindowForBindingUI(pub ::windows::core::IUnknown);
impl IWindowForBindingUI {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidreason), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWindowForBindingUI {
    type Vtable = IWindowForBindingUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d5_bafa_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IWindowForBindingUI> for ::windows::core::IUnknown {
    fn from(value: IWindowForBindingUI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWindowForBindingUI> for ::windows::core::IUnknown {
    fn from(value: &IWindowForBindingUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowForBindingUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowForBindingUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowForBindingUI_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguidreason: *const ::windows::core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWrappedProtocol(pub ::windows::core::IUnknown);
impl IWrappedProtocol {
    pub unsafe fn GetWrapperCode(&self, pncode: *mut i32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pncode), ::core::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWrappedProtocol {
    type Vtable = IWrappedProtocol_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53c84785_8425_4dc5_971b_e58d9c19f9b6);
}
impl ::core::convert::From<IWrappedProtocol> for ::windows::core::IUnknown {
    fn from(value: IWrappedProtocol) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWrappedProtocol> for ::windows::core::IUnknown {
    fn from(value: &IWrappedProtocol) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWrappedProtocol {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWrappedProtocol {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWrappedProtocol_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pncode: *mut i32, dwreserved: usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IZoneIdentifier(pub ::windows::core::IUnknown);
impl IZoneIdentifier {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetId(&self, dwzone: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone)).ok()
    }
    pub unsafe fn Remove(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IZoneIdentifier {
    type Vtable = IZoneIdentifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd45f185_1b21_48e2_967b_ead743a8914e);
}
impl ::core::convert::From<IZoneIdentifier> for ::windows::core::IUnknown {
    fn from(value: IZoneIdentifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IZoneIdentifier> for ::windows::core::IUnknown {
    fn from(value: &IZoneIdentifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IZoneIdentifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IZoneIdentifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoneIdentifier_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IZoneIdentifier2(pub ::windows::core::IUnknown);
impl IZoneIdentifier2 {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetId(&self, dwzone: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwzone)).ok()
    }
    pub unsafe fn Remove(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastWriterPackageFamilyName(&self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastWriterPackageFamilyName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, packagefamilyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), packagefamilyname.into_param().abi()).ok()
    }
    pub unsafe fn RemoveLastWriterPackageFamilyName(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetAppZoneId(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAppZoneId(&self, zone: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(zone)).ok()
    }
    pub unsafe fn RemoveAppZoneId(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IZoneIdentifier2 {
    type Vtable = IZoneIdentifier2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb5e760c_09ef_45c0_b510_70830ce31e6a);
}
impl ::core::convert::From<IZoneIdentifier2> for ::windows::core::IUnknown {
    fn from(value: IZoneIdentifier2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IZoneIdentifier2> for ::windows::core::IUnknown {
    fn from(value: &IZoneIdentifier2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IZoneIdentifier2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IZoneIdentifier2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IZoneIdentifier2> for IZoneIdentifier {
    fn from(value: IZoneIdentifier2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IZoneIdentifier2> for IZoneIdentifier {
    fn from(value: &IZoneIdentifier2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IZoneIdentifier> for IZoneIdentifier2 {
    fn into_param(self) -> ::windows::core::Param<'a, IZoneIdentifier> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IZoneIdentifier> for &IZoneIdentifier2 {
    fn into_param(self) -> ::windows::core::Param<'a, IZoneIdentifier> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IZoneIdentifier2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwzone: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, packagefamilyname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, zone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, zone: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[inline]
pub unsafe fn IsAsyncMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::IMoniker>>(pmk: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsAsyncMoniker(pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        IsAsyncMoniker(pmk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsLoggingEnabledA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(pszurl: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsLoggingEnabledA(pszurl: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsLoggingEnabledA(pszurl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsLoggingEnabledW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pwszurl: Param0) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsLoggingEnabledW(pwszurl: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsLoggingEnabledW(pwszurl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidURL<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pbc: Param0, szurl: Param1, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsValidURL(pbc: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        IsValidURL(pbc.into_param().abi(), szurl.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MAX_SIZE_SECURITY_ID: u32 = 512u32;
pub const MKSYS_URLMONIKER: u32 = 6u32;
pub const MK_S_ASYNCHRONOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(262632i32 as _);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MONIKERPROPERTY(pub i32);
pub const MIMETYPEPROP: MONIKERPROPERTY = MONIKERPROPERTY(0i32);
pub const USE_SRC_URL: MONIKERPROPERTY = MONIKERPROPERTY(1i32);
pub const CLASSIDPROP: MONIKERPROPERTY = MONIKERPROPERTY(2i32);
pub const TRUSTEDDOWNLOADPROP: MONIKERPROPERTY = MONIKERPROPERTY(3i32);
pub const POPUPLEVELPROP: MONIKERPROPERTY = MONIKERPROPERTY(4i32);
impl ::core::convert::From<i32> for MONIKERPROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MONIKERPROPERTY {
    type Abi = Self;
}
pub const MUTZ_ACCEPT_WILDCARD_SCHEME: u32 = 128u32;
pub const MUTZ_DONT_UNESCAPE: u32 = 2048u32;
pub const MUTZ_DONT_USE_CACHE: u32 = 4096u32;
pub const MUTZ_ENFORCERESTRICTED: u32 = 256u32;
pub const MUTZ_FORCE_INTRANET_FLAGS: u32 = 8192u32;
pub const MUTZ_IGNORE_ZONE_MAPPINGS: u32 = 16384u32;
pub const MUTZ_ISFILE: u32 = 2u32;
pub const MUTZ_NOSAVEDFILECHECK: u32 = 1u32;
pub const MUTZ_REQUIRESAVEDFILECHECK: u32 = 1024u32;
pub const MUTZ_RESERVED: u32 = 512u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MkParseDisplayNameEx<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pbc: Param0, szdisplayname: Param1, pcheaten: *mut u32, ppmk: *mut ::core::option::Option<super::IMoniker>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MkParseDisplayNameEx(pbc: ::windows::core::RawPtr, szdisplayname: super::super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        MkParseDisplayNameEx(pbc.into_param().abi(), szdisplayname.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OIBDG_FLAGS(pub i32);
pub const OIBDG_APARTMENTTHREADED: OIBDG_FLAGS = OIBDG_FLAGS(256i32);
pub const OIBDG_DATAONLY: OIBDG_FLAGS = OIBDG_FLAGS(4096i32);
impl ::core::convert::From<i32> for OIBDG_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OIBDG_FLAGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ObtainUserAgentString(dwoption: u32, pszuaout: super::super::super::Foundation::PSTR, cbsize: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ObtainUserAgentString(dwoption: u32, pszuaout: super::super::super::Foundation::PSTR, cbsize: *mut u32) -> ::windows::core::HRESULT;
        }
        ObtainUserAgentString(::core::mem::transmute(dwoption), ::core::mem::transmute(pszuaout), ::core::mem::transmute(cbsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PARSEACTION(pub i32);
pub const PARSE_CANONICALIZE: PARSEACTION = PARSEACTION(1i32);
pub const PARSE_FRIENDLY: PARSEACTION = PARSEACTION(2i32);
pub const PARSE_SECURITY_URL: PARSEACTION = PARSEACTION(3i32);
pub const PARSE_ROOTDOCUMENT: PARSEACTION = PARSEACTION(4i32);
pub const PARSE_DOCUMENT: PARSEACTION = PARSEACTION(5i32);
pub const PARSE_ANCHOR: PARSEACTION = PARSEACTION(6i32);
pub const PARSE_ENCODE_IS_UNESCAPE: PARSEACTION = PARSEACTION(7i32);
pub const PARSE_DECODE_IS_ESCAPE: PARSEACTION = PARSEACTION(8i32);
pub const PARSE_PATH_FROM_URL: PARSEACTION = PARSEACTION(9i32);
pub const PARSE_URL_FROM_PATH: PARSEACTION = PARSEACTION(10i32);
pub const PARSE_MIME: PARSEACTION = PARSEACTION(11i32);
pub const PARSE_SERVER: PARSEACTION = PARSEACTION(12i32);
pub const PARSE_SCHEMA: PARSEACTION = PARSEACTION(13i32);
pub const PARSE_SITE: PARSEACTION = PARSEACTION(14i32);
pub const PARSE_DOMAIN: PARSEACTION = PARSEACTION(15i32);
pub const PARSE_LOCATION: PARSEACTION = PARSEACTION(16i32);
pub const PARSE_SECURITY_DOMAIN: PARSEACTION = PARSEACTION(17i32);
pub const PARSE_ESCAPE: PARSEACTION = PARSEACTION(18i32);
pub const PARSE_UNESCAPE: PARSEACTION = PARSEACTION(19i32);
impl ::core::convert::From<i32> for PARSEACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PARSEACTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PI_FLAGS(pub i32);
pub const PI_PARSE_URL: PI_FLAGS = PI_FLAGS(1i32);
pub const PI_FILTER_MODE: PI_FLAGS = PI_FLAGS(2i32);
pub const PI_FORCE_ASYNC: PI_FLAGS = PI_FLAGS(4i32);
pub const PI_USE_WORKERTHREAD: PI_FLAGS = PI_FLAGS(8i32);
pub const PI_MIMEVERIFICATION: PI_FLAGS = PI_FLAGS(16i32);
pub const PI_CLSIDLOOKUP: PI_FLAGS = PI_FLAGS(32i32);
pub const PI_DATAPROGRESS: PI_FLAGS = PI_FLAGS(64i32);
pub const PI_SYNCHRONOUS: PI_FLAGS = PI_FLAGS(128i32);
pub const PI_APARTMENTTHREADED: PI_FLAGS = PI_FLAGS(256i32);
pub const PI_CLASSINSTALL: PI_FLAGS = PI_FLAGS(512i32);
pub const PI_PASSONBINDCTX: PI_FLAGS = PI_FLAGS(8192i32);
pub const PI_NOMIMEHANDLER: PI_FLAGS = PI_FLAGS(32768i32);
pub const PI_LOADAPPDIRECT: PI_FLAGS = PI_FLAGS(16384i32);
pub const PD_FORCE_SWITCH: PI_FLAGS = PI_FLAGS(65536i32);
pub const PI_PREFERDEFAULTHANDLER: PI_FLAGS = PI_FLAGS(131072i32);
impl ::core::convert::From<i32> for PI_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PI_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PROTOCOLDATA {
    pub grfFlags: u32,
    pub dwState: u32,
    pub pData: *mut ::core::ffi::c_void,
    pub cbData: u32,
}
impl PROTOCOLDATA {}
impl ::core::default::Default for PROTOCOLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROTOCOLDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROTOCOLDATA").field("grfFlags", &self.grfFlags).field("dwState", &self.dwState).field("pData", &self.pData).field("cbData", &self.cbData).finish()
    }
}
impl ::core::cmp::PartialEq for PROTOCOLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.grfFlags == other.grfFlags && self.dwState == other.dwState && self.pData == other.pData && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for PROTOCOLDATA {}
unsafe impl ::windows::core::Abi for PROTOCOLDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct PROTOCOLFILTERDATA {
    pub cbSize: u32,
    pub pProtocolSink: ::core::option::Option<IInternetProtocolSink>,
    pub pProtocol: ::core::option::Option<IInternetProtocol>,
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub dwFilterFlags: u32,
}
impl PROTOCOLFILTERDATA {}
impl ::core::default::Default for PROTOCOLFILTERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PROTOCOLFILTERDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROTOCOLFILTERDATA").field("cbSize", &self.cbSize).field("pProtocolSink", &self.pProtocolSink).field("pProtocol", &self.pProtocol).field("pUnk", &self.pUnk).field("dwFilterFlags", &self.dwFilterFlags).finish()
    }
}
impl ::core::cmp::PartialEq for PROTOCOLFILTERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pProtocolSink == other.pProtocolSink && self.pProtocol == other.pProtocol && self.pUnk == other.pUnk && self.dwFilterFlags == other.dwFilterFlags
    }
}
impl ::core::cmp::Eq for PROTOCOLFILTERDATA {}
unsafe impl ::windows::core::Abi for PROTOCOLFILTERDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const PROTOCOLFLAG_NO_PICS_CHECK: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROTOCOL_ARGUMENT {
    pub szMethod: super::super::super::Foundation::PWSTR,
    pub szTargetUrl: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PROTOCOL_ARGUMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROTOCOL_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROTOCOL_ARGUMENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PROTOCOL_ARGUMENT").field("szMethod", &self.szMethod).field("szTargetUrl", &self.szTargetUrl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROTOCOL_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.szMethod == other.szMethod && self.szTargetUrl == other.szTargetUrl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROTOCOL_ARGUMENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROTOCOL_ARGUMENT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSUACTION(pub i32);
pub const PSU_DEFAULT: PSUACTION = PSUACTION(1i32);
pub const PSU_SECURITY_URL_ONLY: PSUACTION = PSUACTION(2i32);
impl ::core::convert::From<i32> for PSUACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PSUACTION {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PUAF(pub i32);
pub const PUAF_DEFAULT: PUAF = PUAF(0i32);
pub const PUAF_NOUI: PUAF = PUAF(1i32);
pub const PUAF_ISFILE: PUAF = PUAF(2i32);
pub const PUAF_WARN_IF_DENIED: PUAF = PUAF(4i32);
pub const PUAF_FORCEUI_FOREGROUND: PUAF = PUAF(8i32);
pub const PUAF_CHECK_TIFS: PUAF = PUAF(16i32);
pub const PUAF_DONTCHECKBOXINDIALOG: PUAF = PUAF(32i32);
pub const PUAF_TRUSTED: PUAF = PUAF(64i32);
pub const PUAF_ACCEPT_WILDCARD_SCHEME: PUAF = PUAF(128i32);
pub const PUAF_ENFORCERESTRICTED: PUAF = PUAF(256i32);
pub const PUAF_NOSAVEDFILECHECK: PUAF = PUAF(512i32);
pub const PUAF_REQUIRESAVEDFILECHECK: PUAF = PUAF(1024i32);
pub const PUAF_DONT_USE_CACHE: PUAF = PUAF(4096i32);
pub const PUAF_RESERVED1: PUAF = PUAF(8192i32);
pub const PUAF_RESERVED2: PUAF = PUAF(16384i32);
pub const PUAF_LMZ_UNLOCKED: PUAF = PUAF(65536i32);
pub const PUAF_LMZ_LOCKED: PUAF = PUAF(131072i32);
pub const PUAF_DEFAULTZONEPOL: PUAF = PUAF(262144i32);
pub const PUAF_NPL_USE_LOCKED_IF_RESTRICTED: PUAF = PUAF(524288i32);
pub const PUAF_NOUIIFLOCKED: PUAF = PUAF(1048576i32);
pub const PUAF_DRAGPROTOCOLCHECK: PUAF = PUAF(2097152i32);
impl ::core::convert::From<i32> for PUAF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PUAF {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PUAFOUT(pub i32);
pub const PUAFOUT_DEFAULT: PUAFOUT = PUAFOUT(0i32);
pub const PUAFOUT_ISLOCKZONEPOLICY: PUAFOUT = PUAFOUT(1i32);
impl ::core::convert::From<i32> for PUAFOUT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PUAFOUT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct QUERYOPTION(pub i32);
pub const QUERY_EXPIRATION_DATE: QUERYOPTION = QUERYOPTION(1i32);
pub const QUERY_TIME_OF_LAST_CHANGE: QUERYOPTION = QUERYOPTION(2i32);
pub const QUERY_CONTENT_ENCODING: QUERYOPTION = QUERYOPTION(3i32);
pub const QUERY_CONTENT_TYPE: QUERYOPTION = QUERYOPTION(4i32);
pub const QUERY_REFRESH: QUERYOPTION = QUERYOPTION(5i32);
pub const QUERY_RECOMBINE: QUERYOPTION = QUERYOPTION(6i32);
pub const QUERY_CAN_NAVIGATE: QUERYOPTION = QUERYOPTION(7i32);
pub const QUERY_USES_NETWORK: QUERYOPTION = QUERYOPTION(8i32);
pub const QUERY_IS_CACHED: QUERYOPTION = QUERYOPTION(9i32);
pub const QUERY_IS_INSTALLEDENTRY: QUERYOPTION = QUERYOPTION(10i32);
pub const QUERY_IS_CACHED_OR_MAPPED: QUERYOPTION = QUERYOPTION(11i32);
pub const QUERY_USES_CACHE: QUERYOPTION = QUERYOPTION(12i32);
pub const QUERY_IS_SECURE: QUERYOPTION = QUERYOPTION(13i32);
pub const QUERY_IS_SAFE: QUERYOPTION = QUERYOPTION(14i32);
pub const QUERY_USES_HISTORYFOLDER: QUERYOPTION = QUERYOPTION(15i32);
pub const QUERY_IS_CACHED_AND_USABLE_OFFLINE: QUERYOPTION = QUERYOPTION(16i32);
impl ::core::convert::From<i32> for QUERYOPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for QUERYOPTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REMSECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: u32,
    pub bInheritHandle: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl REMSECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REMSECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REMSECURITY_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("REMSECURITY_ATTRIBUTES").field("nLength", &self.nLength).field("lpSecurityDescriptor", &self.lpSecurityDescriptor).field("bInheritHandle", &self.bInheritHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REMSECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength && self.lpSecurityDescriptor == other.lpSecurityDescriptor && self.bInheritHandle == other.bInheritHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REMSECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REMSECURITY_ATTRIBUTES {
    type Abi = Self;
}
#[inline]
pub unsafe fn RegisterBindStatusCallback<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(pbc: Param0, pbscb: Param1, ppbscbprev: *mut ::core::option::Option<super::IBindStatusCallback>, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterBindStatusCallback(pbc: ::windows::core::RawPtr, pbscb: ::windows::core::RawPtr, ppbscbprev: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        RegisterBindStatusCallback(pbc.into_param().abi(), pbscb.into_param().abi(), ::core::mem::transmute(ppbscbprev), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterFormatEnumerator<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::IEnumFORMATETC>>(pbc: Param0, pefetc: Param1, reserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterFormatEnumerator(pbc: ::windows::core::RawPtr, pefetc: ::windows::core::RawPtr, reserved: u32) -> ::windows::core::HRESULT;
        }
        RegisterFormatEnumerator(pbc.into_param().abi(), pefetc.into_param().abi(), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterMediaTypeClass<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>>(pbc: Param0, ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgclsid: *const ::windows::core::GUID, reserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterMediaTypeClass(pbc: ::windows::core::RawPtr, ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgclsid: *const ::windows::core::GUID, reserved: u32) -> ::windows::core::HRESULT;
        }
        RegisterMediaTypeClass(pbc.into_param().abi(), ::core::mem::transmute(ctypes), ::core::mem::transmute(rgsztypes), ::core::mem::transmute(rgclsid), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterMediaTypes(ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterMediaTypes(ctypes: u32, rgsztypes: *const super::super::super::Foundation::PSTR, rgcftypes: *mut u16) -> ::windows::core::HRESULT;
        }
        RegisterMediaTypes(::core::mem::transmute(ctypes), ::core::mem::transmute(rgsztypes), ::core::mem::transmute(rgcftypes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ReleaseBindInfo(pbindinfo: *mut super::BINDINFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseBindInfo(pbindinfo: *mut ::core::mem::ManuallyDrop<super::BINDINFO>);
        }
        ::core::mem::transmute(ReleaseBindInfo(::core::mem::transmute(pbindinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RemBINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: super::super::super::Foundation::PWSTR,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: super::super::super::Foundation::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: REMSECURITY_ATTRIBUTES,
    pub iid: ::windows::core::GUID,
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl RemBINDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RemBINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RemBINDINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RemBINDINFO")
            .field("cbSize", &self.cbSize)
            .field("szExtraInfo", &self.szExtraInfo)
            .field("grfBindInfoF", &self.grfBindInfoF)
            .field("dwBindVerb", &self.dwBindVerb)
            .field("szCustomVerb", &self.szCustomVerb)
            .field("cbstgmedData", &self.cbstgmedData)
            .field("dwOptions", &self.dwOptions)
            .field("dwOptionsFlags", &self.dwOptionsFlags)
            .field("dwCodePage", &self.dwCodePage)
            .field("securityAttributes", &self.securityAttributes)
            .field("iid", &self.iid)
            .field("pUnk", &self.pUnk)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RemBINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.szExtraInfo == other.szExtraInfo
            && self.grfBindInfoF == other.grfBindInfoF
            && self.dwBindVerb == other.dwBindVerb
            && self.szCustomVerb == other.szCustomVerb
            && self.cbstgmedData == other.cbstgmedData
            && self.dwOptions == other.dwOptions
            && self.dwOptionsFlags == other.dwOptionsFlags
            && self.dwCodePage == other.dwCodePage
            && self.securityAttributes == other.securityAttributes
            && self.iid == other.iid
            && self.pUnk == other.pUnk
            && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RemBINDINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RemBINDINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct RemFORMATETC {
    pub cfFormat: u32,
    pub ptd: u32,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl RemFORMATETC {}
impl ::core::default::Default for RemFORMATETC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RemFORMATETC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RemFORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
impl ::core::cmp::PartialEq for RemFORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.cfFormat == other.cfFormat && self.ptd == other.ptd && self.dwAspect == other.dwAspect && self.lindex == other.lindex && self.tymed == other.tymed
    }
}
impl ::core::cmp::Eq for RemFORMATETC {}
unsafe impl ::windows::core::Abi for RemFORMATETC {
    type Abi = Self;
}
#[inline]
pub unsafe fn RevokeBindStatusCallback<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(pbc: Param0, pbscb: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RevokeBindStatusCallback(pbc: ::windows::core::RawPtr, pbscb: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        RevokeBindStatusCallback(pbc.into_param().abi(), pbscb.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RevokeFormatEnumerator<'a, Param0: ::windows::core::IntoParam<'a, super::IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::IEnumFORMATETC>>(pbc: Param0, pefetc: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RevokeFormatEnumerator(pbc: ::windows::core::RawPtr, pefetc: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        RevokeFormatEnumerator(pbc.into_param().abi(), pefetc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SECURITY_IE_STATE_GREEN: u32 = 0u32;
pub const SECURITY_IE_STATE_RED: u32 = 1u32;
pub const SET_FEATURE_IN_REGISTRY: u32 = 4u32;
pub const SET_FEATURE_ON_PROCESS: u32 = 2u32;
pub const SET_FEATURE_ON_THREAD: u32 = 1u32;
pub const SET_FEATURE_ON_THREAD_INTERNET: u32 = 64u32;
pub const SET_FEATURE_ON_THREAD_INTRANET: u32 = 16u32;
pub const SET_FEATURE_ON_THREAD_LOCALMACHINE: u32 = 8u32;
pub const SET_FEATURE_ON_THREAD_RESTRICTED: u32 = 128u32;
pub const SET_FEATURE_ON_THREAD_TRUSTED: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SOFTDISTINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwAdState: u32,
    pub szTitle: super::super::super::Foundation::PWSTR,
    pub szAbstract: super::super::super::Foundation::PWSTR,
    pub szHREF: super::super::super::Foundation::PWSTR,
    pub dwInstalledVersionMS: u32,
    pub dwInstalledVersionLS: u32,
    pub dwUpdateVersionMS: u32,
    pub dwUpdateVersionLS: u32,
    pub dwAdvertisedVersionMS: u32,
    pub dwAdvertisedVersionLS: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SOFTDISTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOFTDISTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOFTDISTINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SOFTDISTINFO")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwAdState", &self.dwAdState)
            .field("szTitle", &self.szTitle)
            .field("szAbstract", &self.szAbstract)
            .field("szHREF", &self.szHREF)
            .field("dwInstalledVersionMS", &self.dwInstalledVersionMS)
            .field("dwInstalledVersionLS", &self.dwInstalledVersionLS)
            .field("dwUpdateVersionMS", &self.dwUpdateVersionMS)
            .field("dwUpdateVersionLS", &self.dwUpdateVersionLS)
            .field("dwAdvertisedVersionMS", &self.dwAdvertisedVersionMS)
            .field("dwAdvertisedVersionLS", &self.dwAdvertisedVersionLS)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOFTDISTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.dwAdState == other.dwAdState
            && self.szTitle == other.szTitle
            && self.szAbstract == other.szAbstract
            && self.szHREF == other.szHREF
            && self.dwInstalledVersionMS == other.dwInstalledVersionMS
            && self.dwInstalledVersionLS == other.dwInstalledVersionLS
            && self.dwUpdateVersionMS == other.dwUpdateVersionMS
            && self.dwUpdateVersionLS == other.dwUpdateVersionLS
            && self.dwAdvertisedVersionMS == other.dwAdvertisedVersionMS
            && self.dwAdvertisedVersionLS == other.dwAdvertisedVersionLS
            && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOFTDISTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SOFTDISTINFO {
    type Abi = Self;
}
pub const SOFTDIST_ADSTATE_AVAILABLE: u32 = 1u32;
pub const SOFTDIST_ADSTATE_DOWNLOADED: u32 = 2u32;
pub const SOFTDIST_ADSTATE_INSTALLED: u32 = 3u32;
pub const SOFTDIST_ADSTATE_NONE: u32 = 0u32;
pub const SOFTDIST_FLAG_DELETE_SUBSCRIPTION: u32 = 8u32;
pub const SOFTDIST_FLAG_USAGE_AUTOINSTALL: u32 = 4u32;
pub const SOFTDIST_FLAG_USAGE_EMAIL: u32 = 1u32;
pub const SOFTDIST_FLAG_USAGE_PRECACHE: u32 = 2u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SZM_FLAGS(pub i32);
pub const SZM_CREATE: SZM_FLAGS = SZM_FLAGS(0i32);
pub const SZM_DELETE: SZM_FLAGS = SZM_FLAGS(1i32);
impl ::core::convert::From<i32> for SZM_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SZM_FLAGS {
    type Abi = Self;
}
pub const S_ASYNCHRONOUS: i32 = 262632i32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetAccessForIEAppContainer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hobject: Param0, ieobjecttype: IEObjectType, dwaccessmask: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetAccessForIEAppContainer(hobject: super::super::super::Foundation::HANDLE, ieobjecttype: IEObjectType, dwaccessmask: u32) -> ::windows::core::HRESULT;
        }
        SetAccessForIEAppContainer(hobject.into_param().abi(), ::core::mem::transmute(ieobjecttype), ::core::mem::transmute(dwaccessmask)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSoftwareUpdateAdvertisementState<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(szdistunit: Param0, dwadstate: u32, dwadvertisedversionms: u32, dwadvertisedversionls: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSoftwareUpdateAdvertisementState(szdistunit: super::super::super::Foundation::PWSTR, dwadstate: u32, dwadvertisedversionms: u32, dwadvertisedversionls: u32) -> ::windows::core::HRESULT;
        }
        SetSoftwareUpdateAdvertisementState(szdistunit.into_param().abi(), ::core::mem::transmute(dwadstate), ::core::mem::transmute(dwadvertisedversionms), ::core::mem::transmute(dwadvertisedversionls)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct StartParam {
    pub iid: ::windows::core::GUID,
    pub pIBindCtx: ::core::option::Option<super::IBindCtx>,
    pub pItf: ::core::option::Option<::windows::core::IUnknown>,
}
impl StartParam {}
impl ::core::default::Default for StartParam {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for StartParam {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("StartParam").field("iid", &self.iid).field("pIBindCtx", &self.pIBindCtx).field("pItf", &self.pItf).finish()
    }
}
impl ::core::cmp::PartialEq for StartParam {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid && self.pIBindCtx == other.pIBindCtx && self.pItf == other.pItf
    }
}
impl ::core::cmp::Eq for StartParam {}
unsafe impl ::windows::core::Abi for StartParam {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const UAS_EXACTLEGACY: u32 = 4096u32;
pub const URLACTION_ACTIVEX_ALLOW_TDC: u32 = 4620u32;
pub const URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY: u32 = 4612u32;
pub const URLACTION_ACTIVEX_CURR_MAX: u32 = 4620u32;
pub const URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION: u32 = 4618u32;
pub const URLACTION_ACTIVEX_MAX: u32 = 5119u32;
pub const URLACTION_ACTIVEX_MIN: u32 = 4608u32;
pub const URLACTION_ACTIVEX_NO_WEBOC_SCRIPT: u32 = 4614u32;
pub const URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY: u32 = 4610u32;
pub const URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST: u32 = 4619u32;
pub const URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY: u32 = 4609u32;
pub const URLACTION_ACTIVEX_OVERRIDE_OPTIN: u32 = 4616u32;
pub const URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION: u32 = 4615u32;
pub const URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY: u32 = 4611u32;
pub const URLACTION_ACTIVEX_RUN: u32 = 4608u32;
pub const URLACTION_ACTIVEX_SCRIPTLET_RUN: u32 = 4617u32;
pub const URLACTION_ACTIVEX_TREATASUNTRUSTED: u32 = 4613u32;
pub const URLACTION_ALLOW_ACTIVEX_FILTERING: u32 = 9986u32;
pub const URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX: u32 = 9996u32;
pub const URLACTION_ALLOW_APEVALUATION: u32 = 8961u32;
pub const URLACTION_ALLOW_AUDIO_VIDEO: u32 = 9985u32;
pub const URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS: u32 = 9988u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST: u32 = 9994u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS: u32 = 9993u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW: u32 = 9992u32;
pub const URLACTION_ALLOW_CSS_EXPRESSIONS: u32 = 9997u32;
pub const URLACTION_ALLOW_JSCRIPT_IE: u32 = 5133u32;
pub const URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS: u32 = 9995u32;
pub const URLACTION_ALLOW_RESTRICTEDPROTOCOLS: u32 = 8960u32;
pub const URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING: u32 = 9987u32;
pub const URLACTION_ALLOW_VBSCRIPT_IE: u32 = 5132u32;
pub const URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE: u32 = 5128u32;
pub const URLACTION_ALLOW_XHR_EVALUATION: u32 = 8962u32;
pub const URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION: u32 = 9990u32;
pub const URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT: u32 = 9989u32;
pub const URLACTION_AUTHENTICATE_CLIENT: u32 = 6657u32;
pub const URLACTION_AUTOMATIC_ACTIVEX_UI: u32 = 8705u32;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI: u32 = 8704u32;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN: u32 = 8704u32;
pub const URLACTION_BEHAVIOR_MIN: u32 = 8192u32;
pub const URLACTION_BEHAVIOR_RUN: u32 = 8192u32;
pub const URLACTION_CHANNEL_SOFTDIST_MAX: u32 = 7935u32;
pub const URLACTION_CHANNEL_SOFTDIST_MIN: u32 = 7680u32;
pub const URLACTION_CHANNEL_SOFTDIST_PERMISSIONS: u32 = 7685u32;
pub const URLACTION_CLIENT_CERT_PROMPT: u32 = 6660u32;
pub const URLACTION_COOKIES: u32 = 6658u32;
pub const URLACTION_COOKIES_ENABLED: u32 = 6672u32;
pub const URLACTION_COOKIES_SESSION: u32 = 6659u32;
pub const URLACTION_COOKIES_SESSION_THIRD_PARTY: u32 = 6662u32;
pub const URLACTION_COOKIES_THIRD_PARTY: u32 = 6661u32;
pub const URLACTION_CREDENTIALS_USE: u32 = 6656u32;
pub const URLACTION_CROSS_DOMAIN_DATA: u32 = 5126u32;
pub const URLACTION_DOTNET_USERCONTROLS: u32 = 8197u32;
pub const URLACTION_DOWNLOAD_CURR_MAX: u32 = 4100u32;
pub const URLACTION_DOWNLOAD_MAX: u32 = 4607u32;
pub const URLACTION_DOWNLOAD_MIN: u32 = 4096u32;
pub const URLACTION_DOWNLOAD_SIGNED_ACTIVEX: u32 = 4097u32;
pub const URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX: u32 = 4100u32;
pub const URLACTION_FEATURE_BLOCK_INPUT_PROMPTS: u32 = 8453u32;
pub const URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE: u32 = 8455u32;
pub const URLACTION_FEATURE_DATA_BINDING: u32 = 8454u32;
pub const URLACTION_FEATURE_FORCE_ADDR_AND_STATUS: u32 = 8452u32;
pub const URLACTION_FEATURE_MIME_SNIFFING: u32 = 8448u32;
pub const URLACTION_FEATURE_MIN: u32 = 8448u32;
pub const URLACTION_FEATURE_SCRIPT_STATUS_BAR: u32 = 8451u32;
pub const URLACTION_FEATURE_WINDOW_RESTRICTIONS: u32 = 8450u32;
pub const URLACTION_FEATURE_ZONE_ELEVATION: u32 = 8449u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS: u32 = 5645u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK: u32 = 5648u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER: u32 = 5647u32;
pub const URLACTION_HTML_ALLOW_INDEXEDDB: u32 = 5649u32;
pub const URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML: u32 = 5643u32;
pub const URLACTION_HTML_ALLOW_WINDOW_CLOSE: u32 = 5646u32;
pub const URLACTION_HTML_FONT_DOWNLOAD: u32 = 5636u32;
pub const URLACTION_HTML_INCLUDE_FILE_PATH: u32 = 5642u32;
pub const URLACTION_HTML_JAVA_RUN: u32 = 5637u32;
pub const URLACTION_HTML_MAX: u32 = 6143u32;
pub const URLACTION_HTML_META_REFRESH: u32 = 5640u32;
pub const URLACTION_HTML_MIN: u32 = 5632u32;
pub const URLACTION_HTML_MIXED_CONTENT: u32 = 5641u32;
pub const URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE: u32 = 5644u32;
pub const URLACTION_HTML_SUBFRAME_NAVIGATE: u32 = 5639u32;
pub const URLACTION_HTML_SUBMIT_FORMS: u32 = 5633u32;
pub const URLACTION_HTML_SUBMIT_FORMS_FROM: u32 = 5634u32;
pub const URLACTION_HTML_SUBMIT_FORMS_TO: u32 = 5635u32;
pub const URLACTION_HTML_USERDATA_SAVE: u32 = 5638u32;
pub const URLACTION_INFODELIVERY_CURR_MAX: u32 = 7430u32;
pub const URLACTION_INFODELIVERY_MAX: u32 = 7679u32;
pub const URLACTION_INFODELIVERY_MIN: u32 = 7424u32;
pub const URLACTION_INFODELIVERY_NO_ADDING_CHANNELS: u32 = 7424u32;
pub const URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS: u32 = 7427u32;
pub const URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING: u32 = 7430u32;
pub const URLACTION_INFODELIVERY_NO_EDITING_CHANNELS: u32 = 7425u32;
pub const URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS: u32 = 7428u32;
pub const URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS: u32 = 7426u32;
pub const URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS: u32 = 7429u32;
pub const URLACTION_INPRIVATE_BLOCKING: u32 = 9984u32;
pub const URLACTION_JAVA_CURR_MAX: u32 = 7168u32;
pub const URLACTION_JAVA_MAX: u32 = 7423u32;
pub const URLACTION_JAVA_MIN: u32 = 7168u32;
pub const URLACTION_JAVA_PERMISSIONS: u32 = 7168u32;
pub const URLACTION_LOOSE_XAML: u32 = 9218u32;
pub const URLACTION_LOWRIGHTS: u32 = 9472u32;
pub const URLACTION_MIN: u32 = 4096u32;
pub const URLACTION_NETWORK_CURR_MAX: u32 = 6672u32;
pub const URLACTION_NETWORK_MAX: u32 = 7167u32;
pub const URLACTION_NETWORK_MIN: u32 = 6656u32;
pub const URLACTION_PLUGGABLE_PROTOCOL_XHR: u32 = 5131u32;
pub const URLACTION_SCRIPT_CURR_MAX: u32 = 5133u32;
pub const URLACTION_SCRIPT_JAVA_USE: u32 = 5122u32;
pub const URLACTION_SCRIPT_MAX: u32 = 5631u32;
pub const URLACTION_SCRIPT_MIN: u32 = 5120u32;
pub const URLACTION_SCRIPT_NAVIGATE: u32 = 5130u32;
pub const URLACTION_SCRIPT_OVERRIDE_SAFETY: u32 = 5121u32;
pub const URLACTION_SCRIPT_PASTE: u32 = 5127u32;
pub const URLACTION_SCRIPT_RUN: u32 = 5120u32;
pub const URLACTION_SCRIPT_SAFE_ACTIVEX: u32 = 5125u32;
pub const URLACTION_SCRIPT_XSSFILTER: u32 = 5129u32;
pub const URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE: u32 = 6161u32;
pub const URLACTION_SHELL_CURR_MAX: u32 = 6162u32;
pub const URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY: u32 = 6155u32;
pub const URLACTION_SHELL_EXECUTE_HIGHRISK: u32 = 6150u32;
pub const URLACTION_SHELL_EXECUTE_LOWRISK: u32 = 6152u32;
pub const URLACTION_SHELL_EXECUTE_MODRISK: u32 = 6151u32;
pub const URLACTION_SHELL_EXTENSIONSECURITY: u32 = 6156u32;
pub const URLACTION_SHELL_FILE_DOWNLOAD: u32 = 6147u32;
pub const URLACTION_SHELL_INSTALL_DTITEMS: u32 = 6144u32;
pub const URLACTION_SHELL_MAX: u32 = 6655u32;
pub const URLACTION_SHELL_MIN: u32 = 6144u32;
pub const URLACTION_SHELL_MOVE_OR_COPY: u32 = 6146u32;
pub const URLACTION_SHELL_POPUPMGR: u32 = 6153u32;
pub const URLACTION_SHELL_PREVIEW: u32 = 6159u32;
pub const URLACTION_SHELL_REMOTEQUERY: u32 = 6158u32;
pub const URLACTION_SHELL_RTF_OBJECTS_LOAD: u32 = 6154u32;
pub const URLACTION_SHELL_SECURE_DRAGSOURCE: u32 = 6157u32;
pub const URLACTION_SHELL_SHARE: u32 = 6160u32;
pub const URLACTION_SHELL_SHELLEXECUTE: u32 = 6150u32;
pub const URLACTION_SHELL_TOCTOU_RISK: u32 = 6162u32;
pub const URLACTION_SHELL_VERB: u32 = 6148u32;
pub const URLACTION_SHELL_WEBVIEW_VERB: u32 = 6149u32;
pub const URLACTION_WINDOWS_BROWSER_APPLICATIONS: u32 = 9216u32;
pub const URLACTION_WINFX_SETUP: u32 = 9728u32;
pub const URLACTION_XPS_DOCUMENTS: u32 = 9217u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLDownloadToCacheFileA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: super::super::super::Foundation::PSTR, cchfilename: u32, param4: u32, param5: Param5) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLDownloadToCacheFileA(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, cchfilename: u32, param4: u32, param5: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLDownloadToCacheFileA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(cchfilename), ::core::mem::transmute(param4), param5.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLDownloadToCacheFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: super::super::super::Foundation::PWSTR, cchfilename: u32, param4: u32, param5: Param5) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLDownloadToCacheFileW(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, cchfilename: u32, param4: u32, param5: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLDownloadToCacheFileW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(cchfilename), ::core::mem::transmute(param4), param5.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLDownloadToFileA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: Param2, param3: u32, param4: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLDownloadToFileA(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: u32, param4: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLDownloadToFileA(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi(), ::core::mem::transmute(param3), param4.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLDownloadToFileW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: Param2, param3: u32, param4: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLDownloadToFileW(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: u32, param4: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLDownloadToFileW(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi(), ::core::mem::transmute(param3), param4.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const URLMON_OPTION_URL_ENCODING: u32 = 268435460u32;
pub const URLMON_OPTION_USERAGENT: u32 = 268435457u32;
pub const URLMON_OPTION_USERAGENT_REFRESH: u32 = 268435458u32;
pub const URLMON_OPTION_USE_BINDSTRINGCREDS: u32 = 268435464u32;
pub const URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS: u32 = 268435472u32;
pub const URLOSTRM_GETNEWESTVERSION: u32 = 3u32;
pub const URLOSTRM_USECACHEDCOPY: u32 = 2u32;
pub const URLOSTRM_USECACHEDCOPY_ONLY: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLOpenBlockingStreamA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: *mut ::core::option::Option<super::IStream>, param3: u32, param4: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLOpenBlockingStreamA(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PSTR, param2: *mut ::windows::core::RawPtr, param3: u32, param4: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLOpenBlockingStreamA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3), param4.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLOpenBlockingStreamW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: *mut ::core::option::Option<super::IStream>, param3: u32, param4: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLOpenBlockingStreamW(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: *mut ::windows::core::RawPtr, param3: u32, param4: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLOpenBlockingStreamW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), ::core::mem::transmute(param3), param4.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLOpenPullStreamA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: u32, param3: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLOpenPullStreamA(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PSTR, param2: u32, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLOpenPullStreamA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLOpenPullStreamW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: u32, param3: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLOpenPullStreamW(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: u32, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLOpenPullStreamW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLOpenStreamA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: u32, param3: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLOpenStreamA(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PSTR, param2: u32, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLOpenStreamA(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn URLOpenStreamW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::IBindStatusCallback>>(param0: Param0, param1: Param1, param2: u32, param3: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn URLOpenStreamW(param0: ::windows::core::RawPtr, param1: super::super::super::Foundation::PWSTR, param2: u32, param3: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        URLOpenStreamW(param0.into_param().abi(), param1.into_param().abi(), ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const URLPOLICY_ACTIVEX_CHECK_LIST: u32 = 65536u32;
pub const URLPOLICY_ALLOW: u32 = 0u32;
pub const URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE: u32 = 65536u32;
pub const URLPOLICY_AUTHENTICATE_CLEARTEXT_OK: u32 = 0u32;
pub const URLPOLICY_AUTHENTICATE_MUTUAL_ONLY: u32 = 196608u32;
pub const URLPOLICY_BEHAVIOR_CHECK_LIST: u32 = 65536u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL: u32 = 196608u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_PRECACHE: u32 = 131072u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT: u32 = 65536u32;
pub const URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY: u32 = 196608u32;
pub const URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT: u32 = 131072u32;
pub const URLPOLICY_CREDENTIALS_MUST_PROMPT_USER: u32 = 65536u32;
pub const URLPOLICY_CREDENTIALS_SILENT_LOGON_OK: u32 = 0u32;
pub const URLPOLICY_DISALLOW: u32 = 3u32;
pub const URLPOLICY_DONTCHECKDLGBOX: u32 = 256u32;
pub const URLPOLICY_JAVA_CUSTOM: u32 = 8388608u32;
pub const URLPOLICY_JAVA_HIGH: u32 = 65536u32;
pub const URLPOLICY_JAVA_LOW: u32 = 196608u32;
pub const URLPOLICY_JAVA_MEDIUM: u32 = 131072u32;
pub const URLPOLICY_JAVA_PROHIBIT: u32 = 0u32;
pub const URLPOLICY_LOG_ON_ALLOW: u32 = 64u32;
pub const URLPOLICY_LOG_ON_DISALLOW: u32 = 128u32;
pub const URLPOLICY_MASK_PERMISSIONS: u32 = 15u32;
pub const URLPOLICY_NOTIFY_ON_ALLOW: u32 = 16u32;
pub const URLPOLICY_NOTIFY_ON_DISALLOW: u32 = 32u32;
pub const URLPOLICY_QUERY: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct URLTEMPLATE(pub i32);
pub const URLTEMPLATE_CUSTOM: URLTEMPLATE = URLTEMPLATE(0i32);
pub const URLTEMPLATE_PREDEFINED_MIN: URLTEMPLATE = URLTEMPLATE(65536i32);
pub const URLTEMPLATE_LOW: URLTEMPLATE = URLTEMPLATE(65536i32);
pub const URLTEMPLATE_MEDLOW: URLTEMPLATE = URLTEMPLATE(66816i32);
pub const URLTEMPLATE_MEDIUM: URLTEMPLATE = URLTEMPLATE(69632i32);
pub const URLTEMPLATE_MEDHIGH: URLTEMPLATE = URLTEMPLATE(70912i32);
pub const URLTEMPLATE_HIGH: URLTEMPLATE = URLTEMPLATE(73728i32);
pub const URLTEMPLATE_PREDEFINED_MAX: URLTEMPLATE = URLTEMPLATE(131072i32);
impl ::core::convert::From<i32> for URLTEMPLATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for URLTEMPLATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct URLZONE(pub i32);
pub const URLZONE_INVALID: URLZONE = URLZONE(-1i32);
pub const URLZONE_PREDEFINED_MIN: URLZONE = URLZONE(0i32);
pub const URLZONE_LOCAL_MACHINE: URLZONE = URLZONE(0i32);
pub const URLZONE_INTRANET: URLZONE = URLZONE(1i32);
pub const URLZONE_TRUSTED: URLZONE = URLZONE(2i32);
pub const URLZONE_INTERNET: URLZONE = URLZONE(3i32);
pub const URLZONE_UNTRUSTED: URLZONE = URLZONE(4i32);
pub const URLZONE_PREDEFINED_MAX: URLZONE = URLZONE(999i32);
pub const URLZONE_USER_MIN: URLZONE = URLZONE(1000i32);
pub const URLZONE_USER_MAX: URLZONE = URLZONE(10000i32);
impl ::core::convert::From<i32> for URLZONE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for URLZONE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct URLZONEREG(pub i32);
pub const URLZONEREG_DEFAULT: URLZONEREG = URLZONEREG(0i32);
pub const URLZONEREG_HKLM: URLZONEREG = URLZONEREG(1i32);
pub const URLZONEREG_HKCU: URLZONEREG = URLZONEREG(2i32);
impl ::core::convert::From<i32> for URLZONEREG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for URLZONEREG {
    type Abi = Self;
}
pub const URLZONE_ESC_FLAG: u32 = 256u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct URL_ENCODING(pub i32);
pub const URL_ENCODING_NONE: URL_ENCODING = URL_ENCODING(0i32);
pub const URL_ENCODING_ENABLE_UTF8: URL_ENCODING = URL_ENCODING(268435456i32);
pub const URL_ENCODING_DISABLE_UTF8: URL_ENCODING = URL_ENCODING(536870912i32);
impl ::core::convert::From<i32> for URL_ENCODING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for URL_ENCODING {
    type Abi = Self;
}
pub const URL_MK_LEGACY: u32 = 0u32;
pub const URL_MK_NO_CANONICALIZE: u32 = 2u32;
pub const URL_MK_UNIFORM: u32 = 1u32;
pub const UriBuilder_USE_ORIGINAL_FLAGS: u32 = 1u32;
pub const Uri_DISPLAY_IDN_HOST: u32 = 4u32;
pub const Uri_DISPLAY_NO_FRAGMENT: u32 = 1u32;
pub const Uri_DISPLAY_NO_PUNYCODE: u32 = 8u32;
pub const Uri_ENCODING_HOST_IS_IDN: u32 = 4u32;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP: u32 = 16u32;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8: u32 = 8u32;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP: u32 = 64u32;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8: u32 = 32u32;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_CP: u32 = 2u32;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Uri_HOST_TYPE(pub i32);
pub const Uri_HOST_UNKNOWN: Uri_HOST_TYPE = Uri_HOST_TYPE(0i32);
pub const Uri_HOST_DNS: Uri_HOST_TYPE = Uri_HOST_TYPE(1i32);
pub const Uri_HOST_IPV4: Uri_HOST_TYPE = Uri_HOST_TYPE(2i32);
pub const Uri_HOST_IPV6: Uri_HOST_TYPE = Uri_HOST_TYPE(3i32);
pub const Uri_HOST_IDN: Uri_HOST_TYPE = Uri_HOST_TYPE(4i32);
impl ::core::convert::From<i32> for Uri_HOST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for Uri_HOST_TYPE {
    type Abi = Self;
}
pub const Uri_PUNYCODE_IDN_HOST: u32 = 2u32;
#[inline]
pub unsafe fn UrlMkGetSessionOption(dwoption: u32, pbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbufferlengthout: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlMkGetSessionOption(dwoption: u32, pbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbufferlengthout: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        UrlMkGetSessionOption(::core::mem::transmute(dwoption), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwbufferlength), ::core::mem::transmute(pdwbufferlengthout), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlMkSetSessionOption(dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlMkSetSessionOption(dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        UrlMkSetSessionOption(::core::mem::transmute(dwoption), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwbufferlength), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WININETINFO_OPTION_LOCK_HANDLE: u32 = 65534u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteHitLogging(lplogginginfo: *const HIT_LOGGING_INFO) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteHitLogging(lplogginginfo: *const HIT_LOGGING_INFO) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WriteHitLogging(::core::mem::transmute(lplogginginfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ZAFLAGS(pub i32);
pub const ZAFLAGS_CUSTOM_EDIT: ZAFLAGS = ZAFLAGS(1i32);
pub const ZAFLAGS_ADD_SITES: ZAFLAGS = ZAFLAGS(2i32);
pub const ZAFLAGS_REQUIRE_VERIFICATION: ZAFLAGS = ZAFLAGS(4i32);
pub const ZAFLAGS_INCLUDE_PROXY_OVERRIDE: ZAFLAGS = ZAFLAGS(8i32);
pub const ZAFLAGS_INCLUDE_INTRANET_SITES: ZAFLAGS = ZAFLAGS(16i32);
pub const ZAFLAGS_NO_UI: ZAFLAGS = ZAFLAGS(32i32);
pub const ZAFLAGS_SUPPORTS_VERIFICATION: ZAFLAGS = ZAFLAGS(64i32);
pub const ZAFLAGS_UNC_AS_INTRANET: ZAFLAGS = ZAFLAGS(128i32);
pub const ZAFLAGS_DETECT_INTRANET: ZAFLAGS = ZAFLAGS(256i32);
pub const ZAFLAGS_USE_LOCKED_ZONES: ZAFLAGS = ZAFLAGS(65536i32);
pub const ZAFLAGS_VERIFY_TEMPLATE_SETTINGS: ZAFLAGS = ZAFLAGS(131072i32);
pub const ZAFLAGS_NO_CACHE: ZAFLAGS = ZAFLAGS(262144i32);
impl ::core::convert::From<i32> for ZAFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ZAFLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct ZONEATTRIBUTES {
    pub cbSize: u32,
    pub szDisplayName: [u16; 260],
    pub szDescription: [u16; 200],
    pub szIconPath: [u16; 260],
    pub dwTemplateMinLevel: u32,
    pub dwTemplateRecommended: u32,
    pub dwTemplateCurrentLevel: u32,
    pub dwFlags: u32,
}
impl ZONEATTRIBUTES {}
impl ::core::default::Default for ZONEATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ZONEATTRIBUTES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ZONEATTRIBUTES")
            .field("cbSize", &self.cbSize)
            .field("szDisplayName", &self.szDisplayName)
            .field("szDescription", &self.szDescription)
            .field("szIconPath", &self.szIconPath)
            .field("dwTemplateMinLevel", &self.dwTemplateMinLevel)
            .field("dwTemplateRecommended", &self.dwTemplateRecommended)
            .field("dwTemplateCurrentLevel", &self.dwTemplateCurrentLevel)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ZONEATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szDisplayName == other.szDisplayName && self.szDescription == other.szDescription && self.szIconPath == other.szIconPath && self.dwTemplateMinLevel == other.dwTemplateMinLevel && self.dwTemplateRecommended == other.dwTemplateRecommended && self.dwTemplateCurrentLevel == other.dwTemplateCurrentLevel && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for ZONEATTRIBUTES {}
unsafe impl ::windows::core::Abi for ZONEATTRIBUTES {
    type Abi = Self;
}
