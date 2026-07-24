#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn BindIFilterFromStorage<P0, P1>(pstg: P0, punkouter: P1, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IStorage>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("query.dll" "system" fn BindIFilterFromStorage(pstg : *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { BindIFilterFromStorage(pstg.param().abi(), punkouter.param().abi(), ppiunk as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn BindIFilterFromStream<P0, P1>(pstm: P0, punkouter: P1, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IStream>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("query.dll" "system" fn BindIFilterFromStream(pstm : *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { BindIFilterFromStream(pstm.param().abi(), punkouter.param().abi(), ppiunk as _) }
}
#[inline]
pub unsafe fn LoadIFilter<P0, P1>(pwcspath: P0, punkouter: P1, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("query.dll" "system" fn LoadIFilter(pwcspath : windows_core::PCWSTR, punkouter : *mut core::ffi::c_void, ppiunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { LoadIFilter(pwcspath.param().abi(), punkouter.param().abi(), ppiunk as _) }
}
#[inline]
pub unsafe fn LoadIFilterEx<P0, T>(pwcspath: P0, dwflags: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_core::link!("query.dll" "system" fn LoadIFilterEx(pwcspath : windows_core::PCWSTR, dwflags : u32, riid : *const windows_core::GUID, ppiunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { LoadIFilterEx(pwcspath.param().abi(), dwflags, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
pub const CIADMIN: windows_core::PCWSTR = windows_core::w!("::_nodocstore_::");
pub const CICAT_ALL_OPENED: i32 = 32;
pub const CICAT_GET_STATE: i32 = 16;
pub const CICAT_NO_QUERY: i32 = 8;
pub const CICAT_READONLY: i32 = 2;
pub const CICAT_STOPPED: i32 = 1;
pub const CICAT_WRITABLE: i32 = 4;
pub const CINULLCATALOG: windows_core::PCWSTR = windows_core::w!("::_noindex_::");
pub const CI_PROVIDER_ALL: u32 = 4294967295;
pub const CI_PROVIDER_INDEXING_SERVICE: i32 = 2;
pub const CI_PROVIDER_MSSEARCH: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const CI_STATE_ANNEALING_MERGE: i32 = 8;
pub const CI_STATE_BATTERY_POLICY: i32 = 262144;
pub const CI_STATE_BATTERY_POWER: i32 = 2048;
pub const CI_STATE_CONTENT_SCAN_REQUIRED: i32 = 4;
pub const CI_STATE_DELETION_MERGE: i32 = 32768;
pub const CI_STATE_HIGH_CPU: i32 = 131072;
pub const CI_STATE_HIGH_IO: i32 = 256;
pub const CI_STATE_INDEX_MIGRATION_MERGE: i32 = 64;
pub const CI_STATE_LOW_DISK: i32 = 65536;
pub const CI_STATE_LOW_MEMORY: i32 = 128;
pub const CI_STATE_MASTER_MERGE: i32 = 2;
pub const CI_STATE_MASTER_MERGE_PAUSED: i32 = 512;
pub const CI_STATE_READING_USNS: i32 = 16384;
pub const CI_STATE_READ_ONLY: i32 = 1024;
pub const CI_STATE_RECOVERING: i32 = 32;
pub const CI_STATE_SCANNING: i32 = 16;
pub const CI_STATE_SHADOW_MERGE: i32 = 1;
pub const CI_STATE_STARTING: i32 = 8192;
pub const CI_STATE_USER_ACTIVE: i32 = 4096;
pub const CI_VERSION_CORRID: i32 = 2048;
pub const CI_VERSION_QUERY_METADATA: i32 = 2304;
pub const CI_VERSION_WDS30: i32 = 258;
pub const CI_VERSION_WDS40: i32 = 265;
pub const CI_VERSION_WIN70: i32 = 1792;
pub const DBPROP_APPLICATION_NAME: i32 = 11;
pub const DBPROP_CATALOGLISTID: i32 = 9;
pub const DBPROP_CI_CATALOG_NAME: i32 = 2;
pub const DBPROP_CI_DEPTHS: i32 = 4;
pub const DBPROP_CI_EXCLUDE_SCOPES: i32 = 5;
pub const DBPROP_CI_INCLUDE_SCOPES: i32 = 3;
pub const DBPROP_CI_PROVIDER: i32 = 8;
pub const DBPROP_CI_QUERY_TYPE: i32 = 7;
pub const DBPROP_CI_SCOPE_FLAGS: i32 = 4;
pub const DBPROP_CI_SECURITY_ID: i32 = 6;
pub const DBPROP_CLIENT_CLSID: i32 = 3;
pub const DBPROP_DEFAULT_EQUALS_BEHAVIOR: i32 = 2;
pub const DBPROP_DEFERCATALOGVERIFICATION: i32 = 8;
pub const DBPROP_DEFERNONINDEXEDTRIMMING: i32 = 3;
pub const DBPROP_DONOTCOMPUTEEXPENSIVEPROPS: i32 = 15;
pub const DBPROP_ENABLEROWSETEVENTS: i32 = 16;
pub const DBPROP_FIRSTROWS: i32 = 7;
pub const DBPROP_FREETEXTANYTERM: i32 = 12;
pub const DBPROP_FREETEXTUSESTEMMING: i32 = 13;
pub const DBPROP_GENERATEPARSETREE: i32 = 10;
pub const DBPROP_GENERICOPTIONS_STRING: i32 = 6;
pub const DBPROP_IGNORENOISEONLYCLAUSES: i32 = 5;
pub const DBPROP_IGNORESBRI: i32 = 14;
pub const DBPROP_MACHINE: i32 = 2;
pub const DBPROP_QUERY_ID: i32 = 18;
pub const DBPROP_SESSION_ID: i32 = 17;
pub const DBPROP_USECONTENTINDEX: i32 = 2;
pub const DBPROP_USEEXTENDEDDBTYPES: i32 = 4;
pub const LIFF_FORCE_TEXT_FILTER_FALLBACK: i32 = 3;
pub const LIFF_IMPLEMENT_TEXT_FILTER_FALLBACK_POLICY: i32 = 2;
pub const LIFF_LOAD_DEFINED_FILTER: i32 = 1;
pub const MSIDXSPROP_COMMAND_LOCALE_STRING: i32 = 3;
pub const MSIDXSPROP_MAX_RANK: i32 = 6;
pub const MSIDXSPROP_PARSE_TREE: i32 = 5;
pub const MSIDXSPROP_QUERY_RESTRICTION: i32 = 4;
pub const MSIDXSPROP_RESULTS_FOUND: i32 = 7;
pub const MSIDXSPROP_ROWSETQUERYSTATUS: i32 = 2;
pub const MSIDXSPROP_SAME_SORTORDER_USED: i32 = 14;
pub const MSIDXSPROP_SERVER_NLSVERSION: i32 = 12;
pub const MSIDXSPROP_SERVER_NLSVER_DEFINED: i32 = 13;
pub const MSIDXSPROP_SERVER_VERSION: i32 = 9;
pub const MSIDXSPROP_SERVER_WINVER_MAJOR: i32 = 10;
pub const MSIDXSPROP_SERVER_WINVER_MINOR: i32 = 11;
pub const MSIDXSPROP_WHEREID: i32 = 8;
pub const PID_FILENAME: i32 = 100;
pub const PROPID_QUERY_LASTSEENTIME: i32 = 10;
pub const PROPID_QUERY_UNFILTERED: i32 = 7;
pub const PROPID_QUERY_VIRTUALPATH: i32 = 9;
pub const PROPID_QUERY_WORKID: i32 = 5;
pub const QUERY_DEEP: i32 = 1;
pub const QUERY_PHYSICAL_PATH: i32 = 0;
pub const QUERY_SHALLOW: i32 = 0;
pub const QUERY_VIRTUAL_PATH: i32 = 2;
pub const STAT_BUSY: i32 = 0;
pub const STAT_COALESCE_COMP_ALL_NOISE: i32 = 8192;
pub const STAT_CONTENT_OUT_OF_DATE: i32 = 32;
pub const STAT_CONTENT_QUERY_INCOMPLETE: i32 = 128;
pub const STAT_DONE: i32 = 2;
pub const STAT_ERROR: i32 = 1;
pub const STAT_MISSING_PROP_IN_RELDOC: i32 = 2048;
pub const STAT_MISSING_RELDOC: i32 = 1024;
pub const STAT_NOISE_WORDS: i32 = 16;
pub const STAT_PARTIAL_SCOPE: i32 = 8;
pub const STAT_REFRESH: i32 = 3;
pub const STAT_REFRESH_INCOMPLETE: i32 = 64;
pub const STAT_RELDOC_ACCESS_DENIED: i32 = 4096;
pub const STAT_SHARING_VIOLATION: i32 = 512;
pub const STAT_TIME_LIMIT_EXCEEDED: i32 = 256;
