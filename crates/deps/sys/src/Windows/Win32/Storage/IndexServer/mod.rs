#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn BindIFilterFromStorage(pstg: super::super::System::Com::StructuredStorage::IStorage, punkouter: ::windows_sys::core::IUnknown, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn BindIFilterFromStream(pstm: super::super::System::Com::IStream, punkouter: ::windows_sys::core::IUnknown, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIFilter(pwcspath: super::super::Foundation::PWSTR, punkouter: ::windows_sys::core::IUnknown, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIFilterEx(pwcspath: super::super::Foundation::PWSTR, dwflags: u32, riid: *const ::windows_sys::core::GUID, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct CHUNKSTATE(i32);
#[repr(C)]
pub struct CHUNK_BREAKTYPE(i32);
pub const CICAT_ALL_OPENED: u32 = 32u32;
pub const CICAT_GET_STATE: u32 = 16u32;
pub const CICAT_NO_QUERY: u32 = 8u32;
pub const CICAT_READONLY: u32 = 2u32;
pub const CICAT_STOPPED: u32 = 1u32;
pub const CICAT_WRITABLE: u32 = 4u32;
pub const CI_PROVIDER_ALL: u32 = 4294967295u32;
pub const CI_PROVIDER_INDEXING_SERVICE: u32 = 2u32;
pub const CI_PROVIDER_MSSEARCH: u32 = 1u32;
#[repr(C)]
pub struct CI_STATE(i32);
pub const CI_STATE_ANNEALING_MERGE: u32 = 8u32;
pub const CI_STATE_BATTERY_POLICY: u32 = 262144u32;
pub const CI_STATE_BATTERY_POWER: u32 = 2048u32;
pub const CI_STATE_CONTENT_SCAN_REQUIRED: u32 = 4u32;
pub const CI_STATE_DELETION_MERGE: u32 = 32768u32;
pub const CI_STATE_HIGH_CPU: u32 = 131072u32;
pub const CI_STATE_HIGH_IO: u32 = 256u32;
pub const CI_STATE_INDEX_MIGRATION_MERGE: u32 = 64u32;
pub const CI_STATE_LOW_DISK: u32 = 65536u32;
pub const CI_STATE_LOW_MEMORY: u32 = 128u32;
pub const CI_STATE_MASTER_MERGE: u32 = 2u32;
pub const CI_STATE_MASTER_MERGE_PAUSED: u32 = 512u32;
pub const CI_STATE_READING_USNS: u32 = 16384u32;
pub const CI_STATE_READ_ONLY: u32 = 1024u32;
pub const CI_STATE_RECOVERING: u32 = 32u32;
pub const CI_STATE_SCANNING: u32 = 16u32;
pub const CI_STATE_SHADOW_MERGE: u32 = 1u32;
pub const CI_STATE_STARTING: u32 = 8192u32;
pub const CI_STATE_USER_ACTIVE: u32 = 4096u32;
pub const CI_VERSION_WDS30: u32 = 258u32;
pub const CI_VERSION_WDS40: u32 = 265u32;
pub const CI_VERSION_WIN70: u32 = 1792u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DBID(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DBID(i32);
#[repr(C)]
pub struct DBKINDENUM(i32);
pub const DBPROP_APPLICATION_NAME: u32 = 11u32;
pub const DBPROP_CATALOGLISTID: u32 = 9u32;
pub const DBPROP_CI_CATALOG_NAME: u32 = 2u32;
pub const DBPROP_CI_DEPTHS: u32 = 4u32;
pub const DBPROP_CI_EXCLUDE_SCOPES: u32 = 5u32;
pub const DBPROP_CI_INCLUDE_SCOPES: u32 = 3u32;
pub const DBPROP_CI_PROVIDER: u32 = 8u32;
pub const DBPROP_CI_QUERY_TYPE: u32 = 7u32;
pub const DBPROP_CI_SCOPE_FLAGS: u32 = 4u32;
pub const DBPROP_CI_SECURITY_ID: u32 = 6u32;
pub const DBPROP_CLIENT_CLSID: u32 = 3u32;
pub const DBPROP_DEFAULT_EQUALS_BEHAVIOR: u32 = 2u32;
pub const DBPROP_DEFERCATALOGVERIFICATION: u32 = 8u32;
pub const DBPROP_DEFERNONINDEXEDTRIMMING: u32 = 3u32;
pub const DBPROP_DONOTCOMPUTEEXPENSIVEPROPS: u32 = 15u32;
pub const DBPROP_ENABLEROWSETEVENTS: u32 = 16u32;
pub const DBPROP_FIRSTROWS: u32 = 7u32;
pub const DBPROP_FREETEXTANYTERM: u32 = 12u32;
pub const DBPROP_FREETEXTUSESTEMMING: u32 = 13u32;
pub const DBPROP_GENERATEPARSETREE: u32 = 10u32;
pub const DBPROP_GENERICOPTIONS_STRING: u32 = 6u32;
pub const DBPROP_IGNORENOISEONLYCLAUSES: u32 = 5u32;
pub const DBPROP_IGNORESBRI: u32 = 14u32;
pub const DBPROP_MACHINE: u32 = 2u32;
pub const DBPROP_USECONTENTINDEX: u32 = 2u32;
pub const DBPROP_USEEXTENDEDDBTYPES: u32 = 4u32;
pub const DBSETFUNC_ALL: u32 = 1u32;
pub const DBSETFUNC_DISTINCT: u32 = 2u32;
pub const DBSETFUNC_NONE: u32 = 0u32;
#[repr(C)]
pub struct FILTERREGION(i32);
pub const FILTER_E_ACCESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215613i32 as _);
pub const FILTER_E_EMBEDDING_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215609i32 as _);
pub const FILTER_E_END_OF_CHUNKS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215616i32 as _);
pub const FILTER_E_LINK_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215608i32 as _);
pub const FILTER_E_NO_MORE_TEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215615i32 as _);
pub const FILTER_E_NO_MORE_VALUES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215614i32 as _);
pub const FILTER_E_NO_TEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215611i32 as _);
pub const FILTER_E_NO_VALUES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215610i32 as _);
pub const FILTER_E_PASSWORD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215605i32 as _);
pub const FILTER_E_UNKNOWNFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147215604i32 as _);
pub const FILTER_S_LAST_TEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(268041i32 as _);
pub const FILTER_S_LAST_VALUES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(268042i32 as _);
pub const FILTER_W_MONIKER_CLIPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(268036i32 as _);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct FULLPROPSPEC(i32);
pub const GENERATE_METHOD_EXACT: u32 = 0u32;
pub const GENERATE_METHOD_INFLECT: u32 = 2u32;
pub const GENERATE_METHOD_PREFIX: u32 = 1u32;
#[repr(C)]
pub struct IFILTER_FLAGS(i32);
#[repr(C)]
pub struct IFILTER_INIT(i32);
#[repr(transparent)]
pub struct IFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhraseSink(pub *mut ::core::ffi::c_void);
pub const LIFF_FORCE_TEXT_FILTER_FALLBACK: u32 = 3u32;
pub const LIFF_IMPLEMENT_TEXT_FILTER_FALLBACK_POLICY: u32 = 2u32;
pub const LIFF_LOAD_DEFINED_FILTER: u32 = 1u32;
pub const MSIDXSPROP_COMMAND_LOCALE_STRING: u32 = 3u32;
pub const MSIDXSPROP_MAX_RANK: u32 = 6u32;
pub const MSIDXSPROP_PARSE_TREE: u32 = 5u32;
pub const MSIDXSPROP_QUERY_RESTRICTION: u32 = 4u32;
pub const MSIDXSPROP_RESULTS_FOUND: u32 = 7u32;
pub const MSIDXSPROP_ROWSETQUERYSTATUS: u32 = 2u32;
pub const MSIDXSPROP_SAME_SORTORDER_USED: u32 = 14u32;
pub const MSIDXSPROP_SERVER_NLSVERSION: u32 = 12u32;
pub const MSIDXSPROP_SERVER_NLSVER_DEFINED: u32 = 13u32;
pub const MSIDXSPROP_SERVER_VERSION: u32 = 9u32;
pub const MSIDXSPROP_SERVER_WINVER_MAJOR: u32 = 10u32;
pub const MSIDXSPROP_SERVER_WINVER_MINOR: u32 = 11u32;
pub const MSIDXSPROP_WHEREID: u32 = 8u32;
pub const NOT_AN_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(524288i32 as _);
pub const PID_FILENAME: u32 = 100u32;
pub const PROPID_QUERY_ALL: u32 = 6u32;
pub const PROPID_QUERY_HITCOUNT: u32 = 4u32;
pub const PROPID_QUERY_LASTSEENTIME: u32 = 10u32;
pub const PROPID_QUERY_RANK: u32 = 3u32;
pub const PROPID_QUERY_RANKVECTOR: u32 = 2u32;
pub const PROPID_QUERY_UNFILTERED: u32 = 7u32;
pub const PROPID_QUERY_VIRTUALPATH: u32 = 9u32;
pub const PROPID_QUERY_WORKID: u32 = 5u32;
pub const PROPID_STG_CONTENTS: u32 = 19u32;
pub const PROXIMITY_UNIT_CHAPTER: u32 = 3u32;
pub const PROXIMITY_UNIT_PARAGRAPH: u32 = 2u32;
pub const PROXIMITY_UNIT_SENTENCE: u32 = 1u32;
pub const PROXIMITY_UNIT_WORD: u32 = 0u32;
pub const QUERY_DEEP: u32 = 1u32;
pub const QUERY_PHYSICAL_PATH: u32 = 0u32;
pub const QUERY_SHALLOW: u32 = 0u32;
pub const QUERY_VIRTUAL_PATH: u32 = 2u32;
pub const SCOPE_FLAG_DEEP: u32 = 2u32;
pub const SCOPE_FLAG_INCLUDE: u32 = 1u32;
pub const SCOPE_FLAG_MASK: u32 = 255u32;
pub const SCOPE_TYPE_MASK: u32 = 4294967040u32;
pub const SCOPE_TYPE_VPATH: u32 = 512u32;
pub const SCOPE_TYPE_WINPATH: u32 = 256u32;
pub const STAT_BUSY: u32 = 0u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[repr(C)]
pub struct STAT_CHUNK(i32);
pub const STAT_COALESCE_COMP_ALL_NOISE: u32 = 8192u32;
pub const STAT_CONTENT_OUT_OF_DATE: u32 = 32u32;
pub const STAT_CONTENT_QUERY_INCOMPLETE: u32 = 128u32;
pub const STAT_DONE: u32 = 2u32;
pub const STAT_ERROR: u32 = 1u32;
pub const STAT_MISSING_PROP_IN_RELDOC: u32 = 2048u32;
pub const STAT_MISSING_RELDOC: u32 = 1024u32;
pub const STAT_NOISE_WORDS: u32 = 16u32;
pub const STAT_PARTIAL_SCOPE: u32 = 8u32;
pub const STAT_REFRESH: u32 = 3u32;
pub const STAT_REFRESH_INCOMPLETE: u32 = 64u32;
pub const STAT_RELDOC_ACCESS_DENIED: u32 = 4096u32;
pub const STAT_SHARING_VIOLATION: u32 = 512u32;
pub const STAT_TIME_LIMIT_EXCEEDED: u32 = 256u32;
pub const VECTOR_RANK_DICE: u32 = 3u32;
pub const VECTOR_RANK_INNER: u32 = 2u32;
pub const VECTOR_RANK_JACCARD: u32 = 4u32;
pub const VECTOR_RANK_MAX: u32 = 1u32;
pub const VECTOR_RANK_MIN: u32 = 0u32;
#[repr(C)]
pub struct WORDREP_BREAK_TYPE(i32);
