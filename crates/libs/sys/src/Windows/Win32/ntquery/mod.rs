#[cfg(feature = "objidl")]
windows_link::link!("query.dll" "system" fn BindIFilterFromStorage(pstg : *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("query.dll" "system" fn BindIFilterFromStream(pstm : *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("query.dll" "system" fn LoadIFilter(pwcspath : windows_sys::core::PCWSTR, punkouter : *mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("query.dll" "system" fn LoadIFilterEx(pwcspath : windows_sys::core::PCWSTR, dwflags : u32, riid : *const windows_sys::core::GUID, ppiunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const CIADMIN: windows_sys::core::PCWSTR = windows_sys::core::w!("::_nodocstore_::");
pub const CICAT_ALL_OPENED: u32 = 32;
pub const CICAT_GET_STATE: u32 = 16;
pub const CICAT_NO_QUERY: u32 = 8;
pub const CICAT_READONLY: u32 = 2;
pub const CICAT_STOPPED: u32 = 1;
pub const CICAT_WRITABLE: u32 = 4;
pub const CINULLCATALOG: windows_sys::core::PCWSTR = windows_sys::core::w!("::_noindex_::");
pub const CI_PROVIDER_ALL: u32 = 4294967295;
pub const CI_PROVIDER_INDEXING_SERVICE: u32 = 2;
pub const CI_PROVIDER_MSSEARCH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CI_STATE {
    pub cbStruct: u32,
    pub cWordList: u32,
    pub cPersistentIndex: u32,
    pub cQueries: u32,
    pub cDocuments: u32,
    pub cFreshTest: u32,
    pub dwMergeProgress: u32,
    pub eState: u32,
    pub cFilteredDocuments: u32,
    pub cTotalDocuments: u32,
    pub cPendingScans: u32,
    pub dwIndexSize: u32,
    pub cUniqueKeys: u32,
    pub cSecQDocuments: u32,
    pub dwPropCacheSize: u32,
}
pub const CI_STATE_ANNEALING_MERGE: u32 = 8;
pub const CI_STATE_BATTERY_POLICY: u32 = 262144;
pub const CI_STATE_BATTERY_POWER: u32 = 2048;
pub const CI_STATE_CONTENT_SCAN_REQUIRED: u32 = 4;
pub const CI_STATE_DELETION_MERGE: u32 = 32768;
pub const CI_STATE_HIGH_CPU: u32 = 131072;
pub const CI_STATE_HIGH_IO: u32 = 256;
pub const CI_STATE_INDEX_MIGRATION_MERGE: u32 = 64;
pub const CI_STATE_LOW_DISK: u32 = 65536;
pub const CI_STATE_LOW_MEMORY: u32 = 128;
pub const CI_STATE_MASTER_MERGE: u32 = 2;
pub const CI_STATE_MASTER_MERGE_PAUSED: u32 = 512;
pub const CI_STATE_READING_USNS: u32 = 16384;
pub const CI_STATE_READ_ONLY: u32 = 1024;
pub const CI_STATE_RECOVERING: u32 = 32;
pub const CI_STATE_SCANNING: u32 = 16;
pub const CI_STATE_SHADOW_MERGE: u32 = 1;
pub const CI_STATE_STARTING: u32 = 8192;
pub const CI_STATE_USER_ACTIVE: u32 = 4096;
pub const CI_VERSION_CORRID: u32 = 2048;
pub const CI_VERSION_QUERY_METADATA: u32 = 2304;
pub const CI_VERSION_WDS30: u32 = 258;
pub const CI_VERSION_WDS40: u32 = 265;
pub const CI_VERSION_WIN70: u32 = 1792;
pub const DBPROP_APPLICATION_NAME: u32 = 11;
pub const DBPROP_CATALOGLISTID: u32 = 9;
pub const DBPROP_CI_CATALOG_NAME: u32 = 2;
pub const DBPROP_CI_DEPTHS: u32 = 4;
pub const DBPROP_CI_EXCLUDE_SCOPES: u32 = 5;
pub const DBPROP_CI_INCLUDE_SCOPES: u32 = 3;
pub const DBPROP_CI_PROVIDER: u32 = 8;
pub const DBPROP_CI_QUERY_TYPE: u32 = 7;
pub const DBPROP_CI_SCOPE_FLAGS: u32 = 4;
pub const DBPROP_CI_SECURITY_ID: u32 = 6;
pub const DBPROP_CLIENT_CLSID: u32 = 3;
pub const DBPROP_DEFAULT_EQUALS_BEHAVIOR: u32 = 2;
pub const DBPROP_DEFERCATALOGVERIFICATION: u32 = 8;
pub const DBPROP_DEFERNONINDEXEDTRIMMING: u32 = 3;
pub const DBPROP_DONOTCOMPUTEEXPENSIVEPROPS: u32 = 15;
pub const DBPROP_ENABLEROWSETEVENTS: u32 = 16;
pub const DBPROP_FIRSTROWS: u32 = 7;
pub const DBPROP_FREETEXTANYTERM: u32 = 12;
pub const DBPROP_FREETEXTUSESTEMMING: u32 = 13;
pub const DBPROP_GENERATEPARSETREE: u32 = 10;
pub const DBPROP_GENERICOPTIONS_STRING: u32 = 6;
pub const DBPROP_IGNORENOISEONLYCLAUSES: u32 = 5;
pub const DBPROP_IGNORESBRI: u32 = 14;
pub const DBPROP_MACHINE: u32 = 2;
pub const DBPROP_QUERY_ID: u32 = 18;
pub const DBPROP_SESSION_ID: u32 = 17;
pub const DBPROP_USECONTENTINDEX: u32 = 2;
pub const DBPROP_USEEXTENDEDDBTYPES: u32 = 4;
pub const LIFF_FORCE_TEXT_FILTER_FALLBACK: u32 = 3;
pub const LIFF_IMPLEMENT_TEXT_FILTER_FALLBACK_POLICY: u32 = 2;
pub const LIFF_LOAD_DEFINED_FILTER: u32 = 1;
pub const MSIDXSPROP_COMMAND_LOCALE_STRING: u32 = 3;
pub const MSIDXSPROP_MAX_RANK: u32 = 6;
pub const MSIDXSPROP_PARSE_TREE: u32 = 5;
pub const MSIDXSPROP_QUERY_RESTRICTION: u32 = 4;
pub const MSIDXSPROP_RESULTS_FOUND: u32 = 7;
pub const MSIDXSPROP_ROWSETQUERYSTATUS: u32 = 2;
pub const MSIDXSPROP_SAME_SORTORDER_USED: u32 = 14;
pub const MSIDXSPROP_SERVER_NLSVERSION: u32 = 12;
pub const MSIDXSPROP_SERVER_NLSVER_DEFINED: u32 = 13;
pub const MSIDXSPROP_SERVER_VERSION: u32 = 9;
pub const MSIDXSPROP_SERVER_WINVER_MAJOR: u32 = 10;
pub const MSIDXSPROP_SERVER_WINVER_MINOR: u32 = 11;
pub const MSIDXSPROP_WHEREID: u32 = 8;
pub const PID_FILENAME: u32 = 100;
pub const PROPID_QUERY_LASTSEENTIME: u32 = 10;
pub const PROPID_QUERY_UNFILTERED: u32 = 7;
pub const PROPID_QUERY_VIRTUALPATH: u32 = 9;
pub const PROPID_QUERY_WORKID: u32 = 5;
pub const QUERY_DEEP: u32 = 1;
pub const QUERY_PHYSICAL_PATH: u32 = 0;
pub const QUERY_SHALLOW: u32 = 0;
pub const QUERY_VIRTUAL_PATH: u32 = 2;
pub const STAT_BUSY: u32 = 0;
pub const STAT_COALESCE_COMP_ALL_NOISE: u32 = 8192;
pub const STAT_CONTENT_OUT_OF_DATE: u32 = 32;
pub const STAT_CONTENT_QUERY_INCOMPLETE: u32 = 128;
pub const STAT_DONE: u32 = 2;
pub const STAT_ERROR: u32 = 1;
pub const STAT_MISSING_PROP_IN_RELDOC: u32 = 2048;
pub const STAT_MISSING_RELDOC: u32 = 1024;
pub const STAT_NOISE_WORDS: u32 = 16;
pub const STAT_PARTIAL_SCOPE: u32 = 8;
pub const STAT_REFRESH: u32 = 3;
pub const STAT_REFRESH_INCOMPLETE: u32 = 64;
pub const STAT_RELDOC_ACCESS_DENIED: u32 = 4096;
pub const STAT_SHARING_VIOLATION: u32 = 512;
pub const STAT_TIME_LIMIT_EXCEEDED: u32 = 256;
