#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUTHENTICATION_INFO {
    pub dwSize: u32,
    pub atAuthenticationType: AUTH_TYPE,
    pub pcwszUser: windows_sys::core::PCWSTR,
    pub pcwszPassword: windows_sys::core::PCWSTR,
}
impl Default for AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AUTH_TYPE = i32;
pub const CATALOG_PAUSED_REASON_DELAYED_RECOVERY: CatalogPausedReason = 7;
pub const CATALOG_PAUSED_REASON_EXTERNAL: CatalogPausedReason = 9;
pub const CATALOG_PAUSED_REASON_HIGH_CPU: CatalogPausedReason = 2;
pub const CATALOG_PAUSED_REASON_HIGH_IO: CatalogPausedReason = 1;
pub const CATALOG_PAUSED_REASON_HIGH_NTF_RATE: CatalogPausedReason = 3;
pub const CATALOG_PAUSED_REASON_LOW_BATTERY: CatalogPausedReason = 4;
pub const CATALOG_PAUSED_REASON_LOW_DISK: CatalogPausedReason = 6;
pub const CATALOG_PAUSED_REASON_LOW_MEMORY: CatalogPausedReason = 5;
pub const CATALOG_PAUSED_REASON_NONE: CatalogPausedReason = 0;
pub const CATALOG_PAUSED_REASON_UPGRADING: CatalogPausedReason = 10;
pub const CATALOG_PAUSED_REASON_USER_ACTIVE: CatalogPausedReason = 8;
pub const CATALOG_STATUS_FULL_CRAWL: CatalogStatus = 3;
pub const CATALOG_STATUS_IDLE: CatalogStatus = 0;
pub const CATALOG_STATUS_INCREMENTAL_CRAWL: CatalogStatus = 4;
pub const CATALOG_STATUS_PAUSED: CatalogStatus = 1;
pub const CATALOG_STATUS_PROCESSING_NOTIFICATIONS: CatalogStatus = 5;
pub const CATALOG_STATUS_RECOVERING: CatalogStatus = 2;
pub const CATALOG_STATUS_SHUTTING_DOWN: CatalogStatus = 6;
pub const CLUSIONREASON_DEFAULT: CLUSION_REASON = 1;
pub const CLUSIONREASON_GROUPPOLICY: CLUSION_REASON = 3;
pub const CLUSIONREASON_UNKNOWNSCOPE: CLUSION_REASON = 0;
pub const CLUSIONREASON_USER: CLUSION_REASON = 2;
pub type CLUSION_REASON = i32;
pub const CSearchLanguageSupport: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a68cc80_4337_4dbc_bd27_fbfb1053820b);
pub const CSearchManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7d096c5f_ac08_4f1f_beb7_5c22c517ce39);
pub const CSearchRoot: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x30766bd2_ea1c_4f28_bf27_0b44e2f68db7);
pub const CSearchScopeRule: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe63de750_3bd7_4be5_9c84_6b4281988c44);
pub type CatalogPausedReason = i32;
pub type CatalogStatus = i32;
pub const FF_INDEXCOMPLEXURLS: FOLLOW_FLAGS = 1;
pub const FF_SUPPRESSINDEXING: FOLLOW_FLAGS = 2;
pub type FOLLOW_FLAGS = i32;
pub const FilterRegistration: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9e175b8d_f52a_11d8_b9a5_505054503030);
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct INCREMENTAL_ACCESS_INFO {
    pub dwSize: u32,
    pub ftLastModifiedTime: super::minwindef::FILETIME,
}
pub type ITEMID = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ITEM_INFO {
    pub dwSize: u32,
    pub pcwszFromEMail: windows_sys::core::PCWSTR,
    pub pcwszApplicationName: windows_sys::core::PCWSTR,
    pub pcwszCatalogName: windows_sys::core::PCWSTR,
    pub pcwszContentClass: windows_sys::core::PCWSTR,
}
impl Default for ITEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRIORITIZE_FLAGS = i32;
pub const PRIORITIZE_FLAG_IGNOREFAILURECOUNT: tagPRIORITIZE_FLAGS = 2;
pub const PRIORITIZE_FLAG_RETRYFAILEDITEMS: tagPRIORITIZE_FLAGS = 1;
pub type PRIORITY_LEVEL = i32;
pub const PRIORITY_LEVEL_DEFAULT: PRIORITY_LEVEL = 3;
pub const PRIORITY_LEVEL_FOREGROUND: PRIORITY_LEVEL = 0;
pub const PRIORITY_LEVEL_HIGH: PRIORITY_LEVEL = 1;
pub const PRIORITY_LEVEL_LOW: PRIORITY_LEVEL = 2;
pub type PROXY_ACCESS = i32;
pub const PROXY_ACCESS_DIRECT: PROXY_ACCESS = 1;
pub const PROXY_ACCESS_PRECONFIG: PROXY_ACCESS = 0;
pub const PROXY_ACCESS_PROXY: PROXY_ACCESS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROXY_INFO {
    pub dwSize: u32,
    pub pcwszUserAgent: windows_sys::core::PCWSTR,
    pub paUseProxy: PROXY_ACCESS,
    pub fLocalBypass: windows_sys::core::BOOL,
    pub dwPortNumber: u32,
    pub pcwszProxyName: windows_sys::core::PCWSTR,
    pub pcwszBypassList: windows_sys::core::PCWSTR,
}
impl Default for PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ROWSETEVENT_ITEMSTATE = i32;
pub const ROWSETEVENT_ITEMSTATE_INROWSET: ROWSETEVENT_ITEMSTATE = 1;
pub const ROWSETEVENT_ITEMSTATE_NOTINROWSET: ROWSETEVENT_ITEMSTATE = 0;
pub const ROWSETEVENT_ITEMSTATE_UNKNOWN: ROWSETEVENT_ITEMSTATE = 2;
pub type ROWSETEVENT_TYPE = i32;
pub const ROWSETEVENT_TYPE_DATAEXPIRED: ROWSETEVENT_TYPE = 0;
pub const ROWSETEVENT_TYPE_FOREGROUNDLOST: ROWSETEVENT_TYPE = 1;
pub const ROWSETEVENT_TYPE_SCOPESTATISTICS: ROWSETEVENT_TYPE = 2;
pub const SEARCH_ADVANCED_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 1;
pub const SEARCH_CHANGE_ADD: SEARCH_KIND_OF_CHANGE = 0;
pub const SEARCH_CHANGE_DELETE: SEARCH_KIND_OF_CHANGE = 1;
pub const SEARCH_CHANGE_MODIFY: SEARCH_KIND_OF_CHANGE = 2;
pub const SEARCH_CHANGE_MOVE_RENAME: SEARCH_KIND_OF_CHANGE = 3;
pub const SEARCH_CHANGE_SEMANTICS_DIRECTORY: SEARCH_KIND_OF_CHANGE = 262144;
pub const SEARCH_CHANGE_SEMANTICS_SHALLOW: SEARCH_KIND_OF_CHANGE = 524288;
pub const SEARCH_CHANGE_SEMANTICS_UPDATE_SECURITY: SEARCH_KIND_OF_CHANGE = 4194304;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SEARCH_COLUMN_PROPERTIES {
    pub Value: super::propidlbase::PROPVARIANT,
    pub lcid: super::winnt::LCID,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SEARCH_COLUMN_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEARCH_HIGH_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = 1;
pub type SEARCH_INDEXING_PHASE = i32;
pub const SEARCH_INDEXING_PHASE_GATHERER: SEARCH_INDEXING_PHASE = 0;
pub const SEARCH_INDEXING_PHASE_PERSISTED: SEARCH_INDEXING_PHASE = 2;
pub const SEARCH_INDEXING_PHASE_QUERYABLE: SEARCH_INDEXING_PHASE = 1;
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy)]
pub struct SEARCH_ITEM_CHANGE {
    pub Change: SEARCH_KIND_OF_CHANGE,
    pub Priority: SEARCH_NOTIFICATION_PRIORITY,
    pub pUserData: *mut super::wtypesbase::BLOB,
    pub lpwszURL: windows_sys::core::PWSTR,
    pub lpwszOldURL: windows_sys::core::PWSTR,
}
#[cfg(feature = "wtypesbase")]
impl Default for SEARCH_ITEM_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SEARCH_ITEM_INDEXING_STATUS {
    pub dwDocID: u32,
    pub hrIndexingStatus: windows_sys::core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SEARCH_ITEM_PERSISTENT_CHANGE {
    pub Change: SEARCH_KIND_OF_CHANGE,
    pub URL: windows_sys::core::PWSTR,
    pub OldURL: windows_sys::core::PWSTR,
    pub Priority: SEARCH_NOTIFICATION_PRIORITY,
}
impl Default for SEARCH_ITEM_PERSISTENT_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SEARCH_KIND_OF_CHANGE = i32;
pub const SEARCH_NATURAL_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 2;
pub const SEARCH_NORMAL_PRIORITY: SEARCH_NOTIFICATION_PRIORITY = 0;
pub type SEARCH_NOTIFICATION_PRIORITY = i32;
pub const SEARCH_NO_QUERY_SYNTAX: SEARCH_QUERY_SYNTAX = 0;
pub type SEARCH_QUERY_SYNTAX = i32;
pub type SEARCH_TERM_EXPANSION = i32;
pub const SEARCH_TERM_NO_EXPANSION: SEARCH_TERM_EXPANSION = 0;
pub const SEARCH_TERM_PREFIX_ALL: SEARCH_TERM_EXPANSION = 1;
pub const SEARCH_TERM_STEM_ALL: SEARCH_TERM_EXPANSION = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TIMEOUT_INFO {
    pub dwSize: u32,
    pub dwConnectTimeout: u32,
    pub dwDataTimeout: u32,
}
pub const eAUTH_TYPE_ANONYMOUS: AUTH_TYPE = 0;
pub const eAUTH_TYPE_BASIC: AUTH_TYPE = 2;
pub const eAUTH_TYPE_NTLM: AUTH_TYPE = 1;
pub type tagPRIORITIZE_FLAGS = i32;
