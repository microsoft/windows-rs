#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    pub moduleName: ::windows::core::PWSTR,
    pub friendlyModuleName: ::windows::core::PWSTR,
    pub eventCount: u32,
    pub uploadSizeBytes: u64,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_BINARY_STATS").field("moduleName", &self.moduleName).field("friendlyModuleName", &self.friendlyModuleName).field("eventCount", &self.eventCount).field("uploadSizeBytes", &self.uploadSizeBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_BINARY_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    pub id: i32,
    pub name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION").field("id", &self.id).field("name", &self.name).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    pub name: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION").field("name", &self.name).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    pub privacyTag: i32,
    pub name: ::windows::core::PWSTR,
    pub description: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION").field("privacyTag", &self.privacyTag).field("name", &self.name).field("description", &self.description).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    pub privacyTag: i32,
    pub eventCount: u32,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_TAG_STATS").field("privacyTag", &self.privacyTag).field("eventCount", &self.eventCount).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_TAG_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    pub hoursOfHistoryToKeep: u32,
    pub maxStoreMegabytes: u32,
    pub requestedMaxStoreMegabytes: u32,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION").field("hoursOfHistoryToKeep", &self.hoursOfHistoryToKeep).field("maxStoreMegabytes", &self.maxStoreMegabytes).field("requestedMaxStoreMegabytes", &self.requestedMaxStoreMegabytes).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
impl ::core::default::Default for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_DATA_GENERAL_STATS {
    pub optInLevel: u32,
    pub transcriptSizeBytes: u64,
    pub oldestEventTimestamp: u64,
    pub totalEventCountLast24Hours: u32,
    pub averageDailyEvents: f32,
}
impl ::core::marker::Copy for DIAGNOSTIC_DATA_GENERAL_STATS {}
impl ::core::clone::Clone for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_GENERAL_STATS").field("optInLevel", &self.optInLevel).field("transcriptSizeBytes", &self.transcriptSizeBytes).field("oldestEventTimestamp", &self.oldestEventTimestamp).field("totalEventCountLast24Hours", &self.totalEventCountLast24Hours).field("averageDailyEvents", &self.averageDailyEvents).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_GENERAL_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_GENERAL_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_GENERAL_STATS {}
impl ::core::default::Default for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_RECORD {
    pub rowId: i64,
    pub timestamp: u64,
    pub eventKeywords: u64,
    pub fullEventName: ::windows::core::PWSTR,
    pub providerGroupGuid: ::windows::core::PWSTR,
    pub producerName: ::windows::core::PWSTR,
    pub privacyTags: *mut i32,
    pub privacyTagCount: u32,
    pub categoryIds: *mut i32,
    pub categoryIdCount: u32,
    pub isCoreData: super::super::Foundation::BOOL,
    pub extra1: ::windows::core::PWSTR,
    pub extra2: ::windows::core::PWSTR,
    pub extra3: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_RECORD")
            .field("rowId", &self.rowId)
            .field("timestamp", &self.timestamp)
            .field("eventKeywords", &self.eventKeywords)
            .field("fullEventName", &self.fullEventName)
            .field("providerGroupGuid", &self.providerGroupGuid)
            .field("producerName", &self.producerName)
            .field("privacyTags", &self.privacyTags)
            .field("privacyTagCount", &self.privacyTagCount)
            .field("categoryIds", &self.categoryIds)
            .field("categoryIdCount", &self.categoryIdCount)
            .field("isCoreData", &self.isCoreData)
            .field("extra1", &self.extra1)
            .field("extra2", &self.extra2)
            .field("extra3", &self.extra3)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    pub producerNames: *mut ::windows::core::PWSTR,
    pub producerNameCount: u32,
    pub textToMatch: ::windows::core::PCWSTR,
    pub categoryIds: *const i32,
    pub categoryIdCount: u32,
    pub privacyTags: *const i32,
    pub privacyTagCount: u32,
    pub coreDataOnly: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_DATA_SEARCH_CRITERIA").field("producerNames", &self.producerNames).field("producerNameCount", &self.producerNameCount).field("textToMatch", &self.textToMatch).field("categoryIds", &self.categoryIds).field("categoryIdCount", &self.categoryIdCount).field("privacyTags", &self.privacyTags).field("privacyTagCount", &self.privacyTagCount).field("coreDataOnly", &self.coreDataOnly).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_DATA_SEARCH_CRITERIA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_REPORT_DATA {
    pub signature: DIAGNOSTIC_REPORT_SIGNATURE,
    pub bucketId: ::windows::core::GUID,
    pub reportId: ::windows::core::GUID,
    pub creationTime: super::super::Foundation::FILETIME,
    pub sizeInBytes: u64,
    pub cabId: ::windows::core::PWSTR,
    pub reportStatus: u32,
    pub reportIntegratorId: ::windows::core::GUID,
    pub fileNames: *mut ::windows::core::PWSTR,
    pub fileCount: u32,
    pub friendlyEventName: ::windows::core::PWSTR,
    pub applicationName: ::windows::core::PWSTR,
    pub applicationPath: ::windows::core::PWSTR,
    pub description: ::windows::core::PWSTR,
    pub bucketIdString: ::windows::core::PWSTR,
    pub legacyBucketId: u64,
    pub reportKey: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_REPORT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIAGNOSTIC_REPORT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_REPORT_DATA")
            .field("signature", &self.signature)
            .field("bucketId", &self.bucketId)
            .field("reportId", &self.reportId)
            .field("creationTime", &self.creationTime)
            .field("sizeInBytes", &self.sizeInBytes)
            .field("cabId", &self.cabId)
            .field("reportStatus", &self.reportStatus)
            .field("reportIntegratorId", &self.reportIntegratorId)
            .field("fileNames", &self.fileNames)
            .field("fileCount", &self.fileCount)
            .field("friendlyEventName", &self.friendlyEventName)
            .field("applicationName", &self.applicationName)
            .field("applicationPath", &self.applicationPath)
            .field("description", &self.description)
            .field("bucketIdString", &self.bucketIdString)
            .field("legacyBucketId", &self.legacyBucketId)
            .field("reportKey", &self.reportKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAGNOSTIC_REPORT_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_REPORT_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAGNOSTIC_REPORT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_REPORT_PARAMETER {
    pub name: [u16; 129],
    pub value: [u16; 260],
}
impl ::core::marker::Copy for DIAGNOSTIC_REPORT_PARAMETER {}
impl ::core::clone::Clone for DIAGNOSTIC_REPORT_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_REPORT_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_REPORT_PARAMETER").field("name", &self.name).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_REPORT_PARAMETER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_REPORT_PARAMETER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_PARAMETER {}
impl ::core::default::Default for DIAGNOSTIC_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub struct DIAGNOSTIC_REPORT_SIGNATURE {
    pub eventName: [u16; 65],
    pub parameters: [DIAGNOSTIC_REPORT_PARAMETER; 10],
}
impl ::core::marker::Copy for DIAGNOSTIC_REPORT_SIGNATURE {}
impl ::core::clone::Clone for DIAGNOSTIC_REPORT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAGNOSTIC_REPORT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAGNOSTIC_REPORT_SIGNATURE").field("eventName", &self.eventName).field("parameters", &self.parameters).finish()
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSTIC_REPORT_SIGNATURE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIAGNOSTIC_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIAGNOSTIC_REPORT_SIGNATURE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIAGNOSTIC_REPORT_SIGNATURE {}
impl ::core::default::Default for DIAGNOSTIC_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DdqAccessLevel(pub i32);
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub const NoData: DdqAccessLevel = DdqAccessLevel(0i32);
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub const CurrentUserData: DdqAccessLevel = DdqAccessLevel(1i32);
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
pub const AllUserData: DdqAccessLevel = DdqAccessLevel(2i32);
impl ::core::marker::Copy for DdqAccessLevel {}
impl ::core::clone::Clone for DdqAccessLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DdqAccessLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DdqAccessLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for DdqAccessLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DdqAccessLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqCancelDiagnosticRecordOperation<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCancelDiagnosticRecordOperation(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
        }
        DdqCancelDiagnosticRecordOperation(hsession.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqCloseSession<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCloseSession(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
        }
        DdqCloseSession(hsession.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqCreateSession(accesslevel: DdqAccessLevel) -> ::windows::core::Result<super::HDIAGNOSTIC_DATA_QUERY_SESSION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCreateSession(accesslevel: DdqAccessLevel, hsession: *mut super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_DATA_QUERY_SESSION = ::core::mem::zeroed();
        DdqCreateSession(::core::mem::transmute(accesslevel), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_DATA_QUERY_SESSION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqExtractDiagnosticReport<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hsession: Param0, reportstoretype: u32, reportkey: Param2, destinationpath: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqExtractDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportkey: ::windows::core::PCWSTR, destinationpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DdqExtractDiagnosticReport(hsession.into_param().abi(), ::core::mem::transmute(reportstoretype), reportkey.into_param().abi(), destinationpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordLocaleTags<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordLocaleTags(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticRecordLocaleTags(htagdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordPage<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordPage(hrecord: super::HDIAGNOSTIC_RECORD) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticRecordPage(hrecord.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducerCategories<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticRecordProducerCategories(hcategorydescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducers<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordProducers(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticRecordProducers(hproducerdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticReport<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticReport(hreport: super::HDIAGNOSTIC_REPORT) -> ::windows::core::HRESULT;
        }
        DdqFreeDiagnosticReport(hreport.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticDataAccessLevelAllowed() -> ::windows::core::Result<DdqAccessLevel> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticDataAccessLevelAllowed(accesslevel: *mut DdqAccessLevel) -> ::windows::core::HRESULT;
        }
        let mut result__: DdqAccessLevel = ::core::mem::zeroed();
        DdqGetDiagnosticDataAccessLevelAllowed(::core::mem::transmute(&mut result__)).from_abi::<DdqAccessLevel>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordAtIndex(hrecord: super::HDIAGNOSTIC_RECORD, index: u32, record: *mut DIAGNOSTIC_DATA_RECORD) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_RECORD = ::core::mem::zeroed();
        DdqGetDiagnosticRecordAtIndex(hrecord.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordBinaryDistribution<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: &[::windows::core::PWSTR], topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordBinaryDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const ::windows::core::PWSTR, producernamecount: u32, topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows::core::HRESULT;
        }
        DdqGetDiagnosticRecordBinaryDistribution(hsession.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(producernames)), producernames.len() as _, ::core::mem::transmute(topnbinaries), ::core::mem::transmute(binarystats), ::core::mem::transmute(statcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index: u32, categorydescription: *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, categorydescriptioncount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticRecordCategoryCount(hcategorydescription.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCount(hrecord: super::HDIAGNOSTIC_RECORD, recordcount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticRecordCount(hrecord.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index: u32, tagdescription: *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, tagdescriptioncount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagCount(htagdescription.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTags<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hsession: Param0, locale: Param1) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTags(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, locale: ::windows::core::PCWSTR, htagdescription: *mut super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTags(hsession.into_param().abi(), locale.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPage<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64) -> ::windows::core::Result<super::HDIAGNOSTIC_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordPage(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64, hrecord: *mut super::HDIAGNOSTIC_RECORD) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_RECORD = ::core::mem::zeroed();
        DdqGetDiagnosticRecordPage(hsession.into_param().abi(), ::core::mem::transmute(searchcriteria), ::core::mem::transmute(offset), ::core::mem::transmute(pagerecordcount), ::core::mem::transmute(baserowid), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPayload<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, rowid: i64) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordPayload(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, rowid: i64, payload: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        DdqGetDiagnosticRecordPayload(hsession.into_param().abi(), ::core::mem::transmute(rowid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index: u32, producerdescription: *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCategories<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hsession: Param0, producername: Param1) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerCategories(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producername: ::windows::core::PCWSTR, hcategorydescription: *mut super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordProducerCategories(hsession.into_param().abi(), producername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerCount(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, producerdescriptioncount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticRecordProducerCount(hproducerdescription.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducers<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducers(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, hproducerdescription: *mut super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION = ::core::mem::zeroed();
        DdqGetDiagnosticRecordProducers(hsession.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordStats<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordStats(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows::core::HRESULT;
        }
        DdqGetDiagnosticRecordStats(hsession.into_param().abi(), ::core::mem::transmute(searchcriteria), ::core::mem::transmute(recordcount), ::core::mem::transmute(minrowid), ::core::mem::transmute(maxrowid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordSummary<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: &[::windows::core::PWSTR]) -> ::windows::core::Result<DIAGNOSTIC_DATA_GENERAL_STATS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordSummary(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const ::windows::core::PWSTR, producernamecount: u32, generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_GENERAL_STATS = ::core::mem::zeroed();
        DdqGetDiagnosticRecordSummary(hsession.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(producernames)), producernames.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_GENERAL_STATS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordTagDistribution<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: &[::windows::core::PWSTR], tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordTagDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const ::windows::core::PWSTR, producernamecount: u32, tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows::core::HRESULT;
        }
        DdqGetDiagnosticRecordTagDistribution(hsession.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(producernames)), producernames.len() as _, ::core::mem::transmute(tagstats), ::core::mem::transmute(statcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReport<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, reportstoretype: u32) -> ::windows::core::Result<super::HDIAGNOSTIC_REPORT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, hreport: *mut super::HDIAGNOSTIC_REPORT) -> ::windows::core::HRESULT;
        }
        let mut result__: super::HDIAGNOSTIC_REPORT = ::core::mem::zeroed();
        DdqGetDiagnosticReport(hsession.into_param().abi(), ::core::mem::transmute(reportstoretype), ::core::mem::transmute(&mut result__)).from_abi::<super::HDIAGNOSTIC_REPORT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticReportAtIndex<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_REPORT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportAtIndex(hreport: super::HDIAGNOSTIC_REPORT, index: u32, report: *mut DIAGNOSTIC_REPORT_DATA) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_REPORT_DATA = ::core::mem::zeroed();
        DdqGetDiagnosticReportAtIndex(hreport.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_REPORT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReportCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportCount(hreport: super::HDIAGNOSTIC_REPORT, reportcount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticReportCount(hreport.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReportStoreReportCount<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, reportstoretype: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportStoreReportCount(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportcount: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        DdqGetDiagnosticReportStoreReportCount(hsession.into_param().abi(), ::core::mem::transmute(reportstoretype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetSessionAccessLevel<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<DdqAccessLevel> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetSessionAccessLevel(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, accesslevel: *mut DdqAccessLevel) -> ::windows::core::HRESULT;
        }
        let mut result__: DdqAccessLevel = ::core::mem::zeroed();
        DdqGetSessionAccessLevel(hsession.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DdqAccessLevel>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetTranscriptConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, currentconfig: *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::HRESULT;
        }
        let mut result__: DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION = ::core::mem::zeroed();
        DdqGetTranscriptConfiguration(hsession.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqIsDiagnosticRecordSampledIn<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hsession: Param0, providergroup: *const ::windows::core::GUID, providerid: *const ::windows::core::GUID, providername: Param3, eventid: *const u32, eventname: Param5, eventversion: *const u32, eventkeywords: *const u64) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqIsDiagnosticRecordSampledIn(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup: *const ::windows::core::GUID, providerid: *const ::windows::core::GUID, providername: ::windows::core::PCWSTR, eventid: *const u32, eventname: ::windows::core::PCWSTR, eventversion: *const u32, eventkeywords: *const u64, issampledin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        DdqIsDiagnosticRecordSampledIn(hsession.into_param().abi(), ::core::mem::transmute(providergroup), ::core::mem::transmute(providerid), providername.into_param().abi(), ::core::mem::transmute(eventid), eventname.into_param().abi(), ::core::mem::transmute(eventversion), ::core::mem::transmute(eventkeywords), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqSetTranscriptConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqSetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::HRESULT;
        }
        DdqSetTranscriptConfiguration(hsession.into_param().abi(), ::core::mem::transmute(desiredconfig)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
