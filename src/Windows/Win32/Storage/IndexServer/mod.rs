#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_System_Com_StructuredStorage`*"]
#[inline]
pub unsafe fn BindIFilterFromStorage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::StructuredStorage::IStorage>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pstg: Param0, punkouter: Param1, ppiunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindIFilterFromStorage(pstg: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, ppiunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        BindIFilterFromStorage(pstg.into_param().abi(), punkouter.into_param().abi(), ::std::mem::transmute(ppiunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn BindIFilterFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pstm: Param0, punkouter: Param1, ppiunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindIFilterFromStream(pstm: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, ppiunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        BindIFilterFromStream(pstm.into_param().abi(), punkouter.into_param().abi(), ::std::mem::transmute(ppiunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHUNKSTATE(pub i32);
pub const CHUNK_TEXT: CHUNKSTATE = CHUNKSTATE(1i32);
pub const CHUNK_VALUE: CHUNKSTATE = CHUNKSTATE(2i32);
pub const CHUNK_FILTER_OWNED_VALUE: CHUNKSTATE = CHUNKSTATE(4i32);
impl ::std::convert::From<i32> for CHUNKSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CHUNKSTATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHUNK_BREAKTYPE(pub i32);
pub const CHUNK_NO_BREAK: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(0i32);
pub const CHUNK_EOW: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(1i32);
pub const CHUNK_EOS: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(2i32);
pub const CHUNK_EOP: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(3i32);
pub const CHUNK_EOC: CHUNK_BREAKTYPE = CHUNK_BREAKTYPE(4i32);
impl ::std::convert::From<i32> for CHUNK_BREAKTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CHUNK_BREAKTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CICAT_ALL_OPENED: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CICAT_GET_STATE: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CICAT_NO_QUERY: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CICAT_READONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CICAT_STOPPED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CICAT_WRITABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_PROVIDER_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_PROVIDER_INDEXING_SERVICE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_PROVIDER_MSSEARCH: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
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
impl CI_STATE {}
impl ::std::default::Default for CI_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CI_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CI_STATE")
            .field("cbStruct", &self.cbStruct)
            .field("cWordList", &self.cWordList)
            .field("cPersistentIndex", &self.cPersistentIndex)
            .field("cQueries", &self.cQueries)
            .field("cDocuments", &self.cDocuments)
            .field("cFreshTest", &self.cFreshTest)
            .field("dwMergeProgress", &self.dwMergeProgress)
            .field("eState", &self.eState)
            .field("cFilteredDocuments", &self.cFilteredDocuments)
            .field("cTotalDocuments", &self.cTotalDocuments)
            .field("cPendingScans", &self.cPendingScans)
            .field("dwIndexSize", &self.dwIndexSize)
            .field("cUniqueKeys", &self.cUniqueKeys)
            .field("cSecQDocuments", &self.cSecQDocuments)
            .field("dwPropCacheSize", &self.dwPropCacheSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CI_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.cWordList == other.cWordList
            && self.cPersistentIndex == other.cPersistentIndex
            && self.cQueries == other.cQueries
            && self.cDocuments == other.cDocuments
            && self.cFreshTest == other.cFreshTest
            && self.dwMergeProgress == other.dwMergeProgress
            && self.eState == other.eState
            && self.cFilteredDocuments == other.cFilteredDocuments
            && self.cTotalDocuments == other.cTotalDocuments
            && self.cPendingScans == other.cPendingScans
            && self.dwIndexSize == other.dwIndexSize
            && self.cUniqueKeys == other.cUniqueKeys
            && self.cSecQDocuments == other.cSecQDocuments
            && self.dwPropCacheSize == other.dwPropCacheSize
    }
}
impl ::std::cmp::Eq for CI_STATE {}
unsafe impl ::windows::runtime::Abi for CI_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_ANNEALING_MERGE: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_BATTERY_POLICY: u32 = 262144u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_BATTERY_POWER: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_CONTENT_SCAN_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_DELETION_MERGE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_HIGH_CPU: u32 = 131072u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_HIGH_IO: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_INDEX_MIGRATION_MERGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_LOW_DISK: u32 = 65536u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_LOW_MEMORY: u32 = 128u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_MASTER_MERGE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_MASTER_MERGE_PAUSED: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_READING_USNS: u32 = 16384u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_READ_ONLY: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_RECOVERING: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_SCANNING: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_SHADOW_MERGE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_STARTING: u32 = 8192u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_STATE_USER_ACTIVE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_VERSION_WDS30: u32 = 258u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_VERSION_WDS40: u32 = 265u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const CI_VERSION_WIN70: u32 = 1792u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
pub struct DBID {
    pub uGuid: DBID_0,
    pub eKind: u32,
    pub uName: DBID_1,
}
#[cfg(feature = "Win32_Foundation")]
impl DBID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DBID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DBID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DBID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DBID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub union DBID_0 {
    pub guid: ::windows::runtime::GUID,
    pub pguid: *mut ::windows::runtime::GUID,
}
impl DBID_0 {}
impl ::std::default::Default for DBID_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DBID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DBID_0 {}
unsafe impl ::windows::runtime::Abi for DBID_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
pub union DBID_1 {
    pub pwszName: super::super::Foundation::PWSTR,
    pub ulPropid: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DBID_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DBID_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DBID_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DBID_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DBID_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DBKINDENUM(pub i32);
pub const DBKIND_GUID_NAME: DBKINDENUM = DBKINDENUM(0i32);
pub const DBKIND_GUID_PROPID: DBKINDENUM = DBKINDENUM(1i32);
pub const DBKIND_NAME: DBKINDENUM = DBKINDENUM(2i32);
pub const DBKIND_PGUID_NAME: DBKINDENUM = DBKINDENUM(3i32);
pub const DBKIND_PGUID_PROPID: DBKINDENUM = DBKINDENUM(4i32);
pub const DBKIND_PROPID: DBKINDENUM = DBKINDENUM(5i32);
pub const DBKIND_GUID: DBKINDENUM = DBKINDENUM(6i32);
impl ::std::convert::From<i32> for DBKINDENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DBKINDENUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_APPLICATION_NAME: u32 = 11u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CATALOGLISTID: u32 = 9u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CI_CATALOG_NAME: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CI_DEPTHS: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CI_EXCLUDE_SCOPES: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CI_INCLUDE_SCOPES: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CI_PROVIDER: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CI_QUERY_TYPE: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CI_SCOPE_FLAGS: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CI_SECURITY_ID: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_CLIENT_CLSID: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_DEFAULT_EQUALS_BEHAVIOR: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_DEFERCATALOGVERIFICATION: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_DEFERNONINDEXEDTRIMMING: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_DONOTCOMPUTEEXPENSIVEPROPS: u32 = 15u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_ENABLEROWSETEVENTS: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_FIRSTROWS: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_FREETEXTANYTERM: u32 = 12u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_FREETEXTUSESTEMMING: u32 = 13u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_GENERATEPARSETREE: u32 = 10u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_GENERICOPTIONS_STRING: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_IGNORENOISEONLYCLAUSES: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_IGNORESBRI: u32 = 14u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_MACHINE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_USECONTENTINDEX: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBPROP_USEEXTENDEDDBTYPES: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBSETFUNC_ALL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBSETFUNC_DISTINCT: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const DBSETFUNC_NONE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub struct FILTERREGION {
    pub idChunk: u32,
    pub cwcStart: u32,
    pub cwcExtent: u32,
}
impl FILTERREGION {}
impl ::std::default::Default for FILTERREGION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILTERREGION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILTERREGION").field("idChunk", &self.idChunk).field("cwcStart", &self.cwcStart).field("cwcExtent", &self.cwcExtent).finish()
    }
}
impl ::std::cmp::PartialEq for FILTERREGION {
    fn eq(&self, other: &Self) -> bool {
        self.idChunk == other.idChunk && self.cwcStart == other.cwcStart && self.cwcExtent == other.cwcExtent
    }
}
impl ::std::cmp::Eq for FILTERREGION {}
unsafe impl ::windows::runtime::Abi for FILTERREGION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_ACCESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215613i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_EMBEDDING_UNAVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215609i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_END_OF_CHUNKS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215616i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_LINK_UNAVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215608i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_NO_MORE_TEXT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215615i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_NO_MORE_VALUES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215614i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_NO_TEXT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215611i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_NO_VALUES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215610i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_PASSWORD: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215605i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_E_UNKNOWNFORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147215604i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_S_LAST_TEXT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(268041i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_S_LAST_VALUES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(268042i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const FILTER_W_MONIKER_CLIPPED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(268036i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
pub struct FULLPROPSPEC {
    pub guidPropSet: ::windows::runtime::GUID,
    pub psProperty: super::super::System::Com::StructuredStorage::PROPSPEC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl FULLPROPSPEC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::default::Default for FULLPROPSPEC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::PartialEq for FULLPROPSPEC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::Eq for FULLPROPSPEC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for FULLPROPSPEC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const GENERATE_METHOD_EXACT: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const GENERATE_METHOD_INFLECT: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const GENERATE_METHOD_PREFIX: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IFILTER_FLAGS(pub i32);
pub const IFILTER_FLAGS_OLE_PROPERTIES: IFILTER_FLAGS = IFILTER_FLAGS(1i32);
impl ::std::convert::From<i32> for IFILTER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IFILTER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IFILTER_INIT(pub i32);
pub const IFILTER_INIT_CANON_PARAGRAPHS: IFILTER_INIT = IFILTER_INIT(1i32);
pub const IFILTER_INIT_HARD_LINE_BREAKS: IFILTER_INIT = IFILTER_INIT(2i32);
pub const IFILTER_INIT_CANON_HYPHENS: IFILTER_INIT = IFILTER_INIT(4i32);
pub const IFILTER_INIT_CANON_SPACES: IFILTER_INIT = IFILTER_INIT(8i32);
pub const IFILTER_INIT_APPLY_INDEX_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(16i32);
pub const IFILTER_INIT_APPLY_OTHER_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(32i32);
pub const IFILTER_INIT_APPLY_CRAWL_ATTRIBUTES: IFILTER_INIT = IFILTER_INIT(256i32);
pub const IFILTER_INIT_INDEXING_ONLY: IFILTER_INIT = IFILTER_INIT(64i32);
pub const IFILTER_INIT_SEARCH_LINKS: IFILTER_INIT = IFILTER_INIT(128i32);
pub const IFILTER_INIT_FILTER_OWNED_VALUE_OK: IFILTER_INIT = IFILTER_INIT(512i32);
pub const IFILTER_INIT_FILTER_AGGRESSIVE_BREAK: IFILTER_INIT = IFILTER_INIT(1024i32);
pub const IFILTER_INIT_DISABLE_EMBEDDED: IFILTER_INIT = IFILTER_INIT(2048i32);
pub const IFILTER_INIT_EMIT_FORMATTING: IFILTER_INIT = IFILTER_INIT(4096i32);
impl ::std::convert::From<i32> for IFILTER_INIT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IFILTER_INIT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IFilter(::windows::runtime::IUnknown);
impl IFilter {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Init(&self, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfflags), ::std::mem::transmute(cattributes), ::std::mem::transmute(aattributes), ::std::mem::transmute(pflags)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstat)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
    pub unsafe fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: super::super::Foundation::PWSTR) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcwcbuffer), ::std::mem::transmute(awcbuffer)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetValue(&self, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pppropvalue)))
    }
    #[doc = "*Required features: `Win32_Storage_IndexServer`*"]
    pub unsafe fn BindRegion<'a, Param0: ::windows::runtime::IntoParam<'a, FILTERREGION>>(&self, origpos: Param0, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), origpos.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)))
    }
}
unsafe impl ::windows::runtime::Interface for IFilter {
    type Vtable = IFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2310846272, 24857, 4122, [188, 183, 0, 221, 1, 6, 85, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstat: *mut STAT_CHUNK) -> i32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcwcbuffer: *mut u32, awcbuffer: super::super::Foundation::PWSTR) -> i32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, origpos: FILTERREGION, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> i32,
);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPhraseSink(::windows::runtime::IUnknown);
impl IPhraseSink {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
    pub unsafe fn PutSmallPhrase<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwcnoun: Param0, cwcnoun: u32, pwcmodifier: Param2, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwcnoun.into_param().abi(), ::std::mem::transmute(cwcnoun), pwcmodifier.into_param().abi(), ::std::mem::transmute(cwcmodifier), ::std::mem::transmute(ulattachmenttype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
    pub unsafe fn PutPhrase<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwcphrase: Param0, cwcphrase: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pwcphrase.into_param().abi(), ::std::mem::transmute(cwcphrase)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPhraseSink {
    type Vtable = IPhraseSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3432017904, 49240, 4122, [181, 84, 8, 0, 43, 51, 176, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhraseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcphrase: super::super::Foundation::PWSTR, cwcphrase: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const LIFF_FORCE_TEXT_FILTER_FALLBACK: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const LIFF_IMPLEMENT_TEXT_FILTER_FALLBACK_POLICY: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const LIFF_LOAD_DEFINED_FILTER: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn LoadIFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pwcspath: Param0, punkouter: Param1, ppiunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadIFilter(pwcspath: super::super::Foundation::PWSTR, punkouter: ::windows::runtime::RawPtr, ppiunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        LoadIFilter(pwcspath.into_param().abi(), punkouter.into_param().abi(), ::std::mem::transmute(ppiunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn LoadIFilterEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwcspath: Param0, dwflags: u32, riid: *const ::windows::runtime::GUID, ppiunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadIFilterEx(pwcspath: super::super::Foundation::PWSTR, dwflags: u32, riid: *const ::windows::runtime::GUID, ppiunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        LoadIFilterEx(pwcspath.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(riid), ::std::mem::transmute(ppiunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_COMMAND_LOCALE_STRING: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_MAX_RANK: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_PARSE_TREE: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_QUERY_RESTRICTION: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_RESULTS_FOUND: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_ROWSETQUERYSTATUS: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_SAME_SORTORDER_USED: u32 = 14u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_SERVER_NLSVERSION: u32 = 12u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_SERVER_NLSVER_DEFINED: u32 = 13u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_SERVER_VERSION: u32 = 9u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_SERVER_WINVER_MAJOR: u32 = 10u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_SERVER_WINVER_MINOR: u32 = 11u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const MSIDXSPROP_WHEREID: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const NOT_AN_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(524288i32 as _);
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PID_FILENAME: u32 = 100u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_QUERY_ALL: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_QUERY_HITCOUNT: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_QUERY_LASTSEENTIME: u32 = 10u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_QUERY_RANK: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_QUERY_RANKVECTOR: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_QUERY_UNFILTERED: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_QUERY_VIRTUALPATH: u32 = 9u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_QUERY_WORKID: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROPID_STG_CONTENTS: u32 = 19u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROXIMITY_UNIT_CHAPTER: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROXIMITY_UNIT_PARAGRAPH: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROXIMITY_UNIT_SENTENCE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const PROXIMITY_UNIT_WORD: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const QUERY_DEEP: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const QUERY_PHYSICAL_PATH: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const QUERY_SHALLOW: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const QUERY_VIRTUAL_PATH: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const SCOPE_FLAG_DEEP: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const SCOPE_FLAG_INCLUDE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const SCOPE_FLAG_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const SCOPE_TYPE_MASK: u32 = 4294967040u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const SCOPE_TYPE_VPATH: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const SCOPE_TYPE_WINPATH: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_BUSY: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
pub struct STAT_CHUNK {
    pub idChunk: u32,
    pub breakType: CHUNK_BREAKTYPE,
    pub flags: CHUNKSTATE,
    pub locale: u32,
    pub attribute: FULLPROPSPEC,
    pub idChunkSource: u32,
    pub cwcStartSource: u32,
    pub cwcLenSource: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl STAT_CHUNK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::default::Default for STAT_CHUNK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::PartialEq for STAT_CHUNK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::Eq for STAT_CHUNK {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for STAT_CHUNK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_COALESCE_COMP_ALL_NOISE: u32 = 8192u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_CONTENT_OUT_OF_DATE: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_CONTENT_QUERY_INCOMPLETE: u32 = 128u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_DONE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_ERROR: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_MISSING_PROP_IN_RELDOC: u32 = 2048u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_MISSING_RELDOC: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_NOISE_WORDS: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_PARTIAL_SCOPE: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_REFRESH: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_REFRESH_INCOMPLETE: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_RELDOC_ACCESS_DENIED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_SHARING_VIOLATION: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const STAT_TIME_LIMIT_EXCEEDED: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const VECTOR_RANK_DICE: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const VECTOR_RANK_INNER: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const VECTOR_RANK_JACCARD: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const VECTOR_RANK_MAX: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
pub const VECTOR_RANK_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_IndexServer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WORDREP_BREAK_TYPE(pub i32);
pub const WORDREP_BREAK_EOW: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(0i32);
pub const WORDREP_BREAK_EOS: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(1i32);
pub const WORDREP_BREAK_EOP: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(2i32);
pub const WORDREP_BREAK_EOC: WORDREP_BREAK_TYPE = WORDREP_BREAK_TYPE(3i32);
impl ::std::convert::From<i32> for WORDREP_BREAK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WORDREP_BREAK_TYPE {
    type Abi = Self;
}
