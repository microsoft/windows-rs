#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
pub struct DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    pub moduleName: super::super::Foundation::PWSTR,
    pub friendlyModuleName: super::super::Foundation::PWSTR,
    pub eventCount: u32,
    pub uploadSizeBytes: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_BINARY_STATS").field("moduleName", &self.moduleName).field("friendlyModuleName", &self.friendlyModuleName).field("eventCount", &self.eventCount).field("uploadSizeBytes", &self.uploadSizeBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.moduleName == other.moduleName && self.friendlyModuleName == other.friendlyModuleName && self.eventCount == other.eventCount && self.uploadSizeBytes == other.uploadSizeBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
pub struct DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    pub id: i32,
    pub name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION").field("id", &self.id).field("name", &self.name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
pub struct DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    pub name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION").field("name", &self.name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    pub privacyTag: i32,
    pub name: super::super::Foundation::PWSTR,
    pub description: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION").field("privacyTag", &self.privacyTag).field("name", &self.name).field("description", &self.description).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.privacyTag == other.privacyTag && self.name == other.name && self.description == other.description
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    pub privacyTag: i32,
    pub eventCount: u32,
}
impl DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_TAG_STATS").field("privacyTag", &self.privacyTag).field("eventCount", &self.eventCount).finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.privacyTag == other.privacyTag && self.eventCount == other.eventCount
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TAG_STATS {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_TAG_STATS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
pub struct DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    pub hoursOfHistoryToKeep: u32,
    pub maxStoreMegabytes: u32,
    pub requestedMaxStoreMegabytes: u32,
}
impl DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
impl ::std::default::Default for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION").field("hoursOfHistoryToKeep", &self.hoursOfHistoryToKeep).field("maxStoreMegabytes", &self.maxStoreMegabytes).field("requestedMaxStoreMegabytes", &self.requestedMaxStoreMegabytes).finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.hoursOfHistoryToKeep == other.hoursOfHistoryToKeep && self.maxStoreMegabytes == other.maxStoreMegabytes && self.requestedMaxStoreMegabytes == other.requestedMaxStoreMegabytes
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
pub struct DIAGNOSTIC_DATA_GENERAL_STATS {
    pub optInLevel: u32,
    pub transcriptSizeBytes: u64,
    pub oldestEventTimestamp: u64,
    pub totalEventCountLast24Hours: u32,
    pub averageDailyEvents: f32,
}
impl DIAGNOSTIC_DATA_GENERAL_STATS {}
impl ::std::default::Default for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_GENERAL_STATS")
            .field("optInLevel", &self.optInLevel)
            .field("transcriptSizeBytes", &self.transcriptSizeBytes)
            .field("oldestEventTimestamp", &self.oldestEventTimestamp)
            .field("totalEventCountLast24Hours", &self.totalEventCountLast24Hours)
            .field("averageDailyEvents", &self.averageDailyEvents)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_GENERAL_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.optInLevel == other.optInLevel && self.transcriptSizeBytes == other.transcriptSizeBytes && self.oldestEventTimestamp == other.oldestEventTimestamp && self.totalEventCountLast24Hours == other.totalEventCountLast24Hours && self.averageDailyEvents == other.averageDailyEvents
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_GENERAL_STATS {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_GENERAL_STATS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
pub struct DIAGNOSTIC_DATA_RECORD {
    pub rowId: i64,
    pub timestamp: u64,
    pub eventKeywords: u64,
    pub fullEventName: super::super::Foundation::PWSTR,
    pub providerGroupGuid: super::super::Foundation::PWSTR,
    pub producerName: super::super::Foundation::PWSTR,
    pub privacyTags: *mut i32,
    pub privacyTagCount: u32,
    pub categoryIds: *mut i32,
    pub categoryIdCount: u32,
    pub isCoreData: super::super::Foundation::BOOL,
    pub extra1: super::super::Foundation::PWSTR,
    pub extra2: super::super::Foundation::PWSTR,
    pub extra3: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_RECORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_RECORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_RECORD")
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
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.rowId == other.rowId
            && self.timestamp == other.timestamp
            && self.eventKeywords == other.eventKeywords
            && self.fullEventName == other.fullEventName
            && self.providerGroupGuid == other.providerGroupGuid
            && self.producerName == other.producerName
            && self.privacyTags == other.privacyTags
            && self.privacyTagCount == other.privacyTagCount
            && self.categoryIds == other.categoryIds
            && self.categoryIdCount == other.categoryIdCount
            && self.isCoreData == other.isCoreData
            && self.extra1 == other.extra1
            && self.extra2 == other.extra2
            && self.extra3 == other.extra3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_RECORD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
pub struct DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    pub producerNames: *mut super::super::Foundation::PWSTR,
    pub producerNameCount: u32,
    pub textToMatch: super::super::Foundation::PWSTR,
    pub categoryIds: *mut i32,
    pub categoryIdCount: u32,
    pub privacyTags: *mut i32,
    pub privacyTagCount: u32,
    pub coreDataOnly: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_DATA_SEARCH_CRITERIA")
            .field("producerNames", &self.producerNames)
            .field("producerNameCount", &self.producerNameCount)
            .field("textToMatch", &self.textToMatch)
            .field("categoryIds", &self.categoryIds)
            .field("categoryIdCount", &self.categoryIdCount)
            .field("privacyTags", &self.privacyTags)
            .field("privacyTagCount", &self.privacyTagCount)
            .field("coreDataOnly", &self.coreDataOnly)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn eq(&self, other: &Self) -> bool {
        self.producerNames == other.producerNames && self.producerNameCount == other.producerNameCount && self.textToMatch == other.textToMatch && self.categoryIds == other.categoryIds && self.categoryIdCount == other.categoryIdCount && self.privacyTags == other.privacyTags && self.privacyTagCount == other.privacyTagCount && self.coreDataOnly == other.coreDataOnly
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
pub struct DIAGNOSTIC_REPORT_DATA {
    pub signature: DIAGNOSTIC_REPORT_SIGNATURE,
    pub bucketId: ::windows::runtime::GUID,
    pub reportId: ::windows::runtime::GUID,
    pub creationTime: super::super::Foundation::FILETIME,
    pub sizeInBytes: u64,
    pub cabId: super::super::Foundation::PWSTR,
    pub reportStatus: u32,
    pub reportIntegratorId: ::windows::runtime::GUID,
    pub fileNames: *mut super::super::Foundation::PWSTR,
    pub fileCount: u32,
    pub friendlyEventName: super::super::Foundation::PWSTR,
    pub applicationName: super::super::Foundation::PWSTR,
    pub applicationPath: super::super::Foundation::PWSTR,
    pub description: super::super::Foundation::PWSTR,
    pub bucketIdString: super::super::Foundation::PWSTR,
    pub legacyBucketId: u64,
    pub reportKey: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DIAGNOSTIC_REPORT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DIAGNOSTIC_REPORT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_REPORT_DATA")
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
impl ::std::cmp::PartialEq for DIAGNOSTIC_REPORT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
            && self.bucketId == other.bucketId
            && self.reportId == other.reportId
            && self.creationTime == other.creationTime
            && self.sizeInBytes == other.sizeInBytes
            && self.cabId == other.cabId
            && self.reportStatus == other.reportStatus
            && self.reportIntegratorId == other.reportIntegratorId
            && self.fileNames == other.fileNames
            && self.fileCount == other.fileCount
            && self.friendlyEventName == other.friendlyEventName
            && self.applicationName == other.applicationName
            && self.applicationPath == other.applicationPath
            && self.description == other.description
            && self.bucketIdString == other.bucketIdString
            && self.legacyBucketId == other.legacyBucketId
            && self.reportKey == other.reportKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_REPORT_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
pub struct DIAGNOSTIC_REPORT_PARAMETER {
    pub name: [u16; 129],
    pub value: [u16; 260],
}
impl DIAGNOSTIC_REPORT_PARAMETER {}
impl ::std::default::Default for DIAGNOSTIC_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_REPORT_PARAMETER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_REPORT_PARAMETER").field("name", &self.name).field("value", &self.value).finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_REPORT_PARAMETER {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_REPORT_PARAMETER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
pub struct DIAGNOSTIC_REPORT_SIGNATURE {
    pub eventName: [u16; 65],
    pub parameters: [DIAGNOSTIC_REPORT_PARAMETER; 10],
}
impl DIAGNOSTIC_REPORT_SIGNATURE {}
impl ::std::default::Default for DIAGNOSTIC_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DIAGNOSTIC_REPORT_SIGNATURE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DIAGNOSTIC_REPORT_SIGNATURE").field("eventName", &self.eventName).field("parameters", &self.parameters).finish()
    }
}
impl ::std::cmp::PartialEq for DIAGNOSTIC_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.eventName == other.eventName && self.parameters == other.parameters
    }
}
impl ::std::cmp::Eq for DIAGNOSTIC_REPORT_SIGNATURE {}
unsafe impl ::windows::runtime::Abi for DIAGNOSTIC_REPORT_SIGNATURE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DdqAccessLevel(pub i32);
pub const NoData: DdqAccessLevel = DdqAccessLevel(0i32);
pub const CurrentUserData: DdqAccessLevel = DdqAccessLevel(1i32);
pub const AllUserData: DdqAccessLevel = DdqAccessLevel(2i32);
impl ::std::convert::From<i32> for DdqAccessLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DdqAccessLevel {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqCancelDiagnosticRecordOperation<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCancelDiagnosticRecordOperation(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::runtime::HRESULT;
        }
        DdqCancelDiagnosticRecordOperation(hsession.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqCloseSession<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCloseSession(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::runtime::HRESULT;
        }
        DdqCloseSession(hsession.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqCreateSession(accesslevel: DdqAccessLevel) -> ::windows::runtime::Result<super::HDIAGNOSTIC_DATA_QUERY_SESSION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqCreateSession(accesslevel: DdqAccessLevel, hsession: *mut super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::HDIAGNOSTIC_DATA_QUERY_SESSION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqCreateSession(::std::mem::transmute(accesslevel), &mut result__).from_abi::<super::HDIAGNOSTIC_DATA_QUERY_SESSION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqExtractDiagnosticReport<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, reportstoretype: u32, reportkey: Param2, destinationpath: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqExtractDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportkey: super::super::Foundation::PWSTR, destinationpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        DdqExtractDiagnosticReport(hsession.into_param().abi(), ::std::mem::transmute(reportstoretype), reportkey.into_param().abi(), destinationpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordLocaleTags<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordLocaleTags(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticRecordLocaleTags(htagdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordPage<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordPage(hrecord: super::HDIAGNOSTIC_RECORD) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticRecordPage(hrecord.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducerCategories<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticRecordProducerCategories(hcategorydescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducers<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticRecordProducers(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticRecordProducers(hproducerdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticReport<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqFreeDiagnosticReport(hreport: super::HDIAGNOSTIC_REPORT) -> ::windows::runtime::HRESULT;
        }
        DdqFreeDiagnosticReport(hreport.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticDataAccessLevelAllowed() -> ::windows::runtime::Result<DdqAccessLevel> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticDataAccessLevelAllowed(accesslevel: *mut DdqAccessLevel) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DdqAccessLevel as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticDataAccessLevelAllowed(&mut result__).from_abi::<DdqAccessLevel>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordAtIndex<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0, index: u32) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordAtIndex(hrecord: super::HDIAGNOSTIC_RECORD, index: u32, record: *mut DIAGNOSTIC_DATA_RECORD) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_RECORD as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordAtIndex(hrecord.into_param().abi(), ::std::mem::transmute(index), &mut result__).from_abi::<DIAGNOSTIC_DATA_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordBinaryDistribution<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordBinaryDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        DdqGetDiagnosticRecordBinaryDistribution(hsession.into_param().abi(), ::std::mem::transmute(producernames), ::std::mem::transmute(producernamecount), ::std::mem::transmute(topnbinaries), ::std::mem::transmute(binarystats), ::std::mem::transmute(statcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryAtIndex<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0, index: u32) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index: u32, categorydescription: *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription.into_param().abi(), ::std::mem::transmute(index), &mut result__).from_abi::<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>>(hcategorydescription: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, categorydescriptioncount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordCategoryCount(hcategorydescription.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_RECORD>>(hrecord: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordCount(hrecord: super::HDIAGNOSTIC_RECORD, recordcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordCount(hrecord.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagAtIndex<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0, index: u32) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index: u32, tagdescription: *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription.into_param().abi(), ::std::mem::transmute(index), &mut result__).from_abi::<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>>(htagdescription: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, tagdescriptioncount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTagCount(htagdescription.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTags<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, locale: Param1) -> ::windows::runtime::Result<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordLocaleTags(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, locale: super::super::Foundation::PWSTR, htagdescription: *mut super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordLocaleTags(hsession.into_param().abi(), locale.into_param().abi(), &mut result__).from_abi::<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPage<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64) -> ::windows::runtime::Result<super::HDIAGNOSTIC_RECORD> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordPage(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64, hrecord: *mut super::HDIAGNOSTIC_RECORD) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::HDIAGNOSTIC_RECORD as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordPage(hsession.into_param().abi(), ::std::mem::transmute(searchcriteria), ::std::mem::transmute(offset), ::std::mem::transmute(pagerecordcount), ::std::mem::transmute(baserowid), &mut result__).from_abi::<super::HDIAGNOSTIC_RECORD>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPayload<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, rowid: i64) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordPayload(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, rowid: i64, payload: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordPayload(hsession.into_param().abi(), ::std::mem::transmute(rowid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerAtIndex<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0, index: u32) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index: u32, producerdescription: *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription.into_param().abi(), ::std::mem::transmute(index), &mut result__).from_abi::<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCategories<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hsession: Param0, producername: Param1) -> ::windows::runtime::Result<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerCategories(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producername: super::super::Foundation::PWSTR, hcategorydescription: *mut super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordProducerCategories(hsession.into_param().abi(), producername.into_param().abi(), &mut result__).from_abi::<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>>(hproducerdescription: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducerCount(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, producerdescriptioncount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordProducerCount(hproducerdescription.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducers<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::runtime::Result<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordProducers(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, hproducerdescription: *mut super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordProducers(hsession.into_param().abi(), &mut result__).from_abi::<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordStats<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordStats(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows::runtime::HRESULT;
        }
        DdqGetDiagnosticRecordStats(hsession.into_param().abi(), ::std::mem::transmute(searchcriteria), ::std::mem::transmute(recordcount), ::std::mem::transmute(minrowid), ::std::mem::transmute(maxrowid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordSummary<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_GENERAL_STATS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordSummary(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_GENERAL_STATS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticRecordSummary(hsession.into_param().abi(), ::std::mem::transmute(producernames), ::std::mem::transmute(producernamecount), &mut result__).from_abi::<DIAGNOSTIC_DATA_GENERAL_STATS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordTagDistribution<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticRecordTagDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        DdqGetDiagnosticRecordTagDistribution(hsession.into_param().abi(), ::std::mem::transmute(producernames), ::std::mem::transmute(producernamecount), ::std::mem::transmute(tagstats), ::std::mem::transmute(statcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReport<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, reportstoretype: u32) -> ::windows::runtime::Result<super::HDIAGNOSTIC_REPORT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, hreport: *mut super::HDIAGNOSTIC_REPORT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::HDIAGNOSTIC_REPORT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticReport(hsession.into_param().abi(), ::std::mem::transmute(reportstoretype), &mut result__).from_abi::<super::HDIAGNOSTIC_REPORT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticReportAtIndex<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0, index: u32) -> ::windows::runtime::Result<DIAGNOSTIC_REPORT_DATA> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportAtIndex(hreport: super::HDIAGNOSTIC_REPORT, index: u32, report: *mut DIAGNOSTIC_REPORT_DATA) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_REPORT_DATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticReportAtIndex(hreport.into_param().abi(), ::std::mem::transmute(index), &mut result__).from_abi::<DIAGNOSTIC_REPORT_DATA>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReportCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_REPORT>>(hreport: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportCount(hreport: super::HDIAGNOSTIC_REPORT, reportcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticReportCount(hreport.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReportStoreReportCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, reportstoretype: u32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetDiagnosticReportStoreReportCount(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportcount: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetDiagnosticReportStoreReportCount(hsession.into_param().abi(), ::std::mem::transmute(reportstoretype), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetSessionAccessLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::runtime::Result<DdqAccessLevel> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetSessionAccessLevel(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, accesslevel: *mut DdqAccessLevel) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DdqAccessLevel as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetSessionAccessLevel(hsession.into_param().abi(), &mut result__).from_abi::<DdqAccessLevel>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqGetTranscriptConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0) -> ::windows::runtime::Result<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqGetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, currentconfig: *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqGetTranscriptConfiguration(hsession.into_param().abi(), &mut result__).from_abi::<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqIsDiagnosticRecordSampledIn<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hsession: Param0,
    providergroup: *const ::windows::runtime::GUID,
    providerid: *const ::windows::runtime::GUID,
    providername: Param3,
    eventid: *const u32,
    eventname: Param5,
    eventversion: *const u32,
    eventkeywords: *const u64,
) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqIsDiagnosticRecordSampledIn(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup: *const ::windows::runtime::GUID, providerid: *const ::windows::runtime::GUID, providername: super::super::Foundation::PWSTR, eventid: *const u32, eventname: super::super::Foundation::PWSTR, eventversion: *const u32, eventkeywords: *const u64, issampledin: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DdqIsDiagnosticRecordSampledIn(hsession.into_param().abi(), ::std::mem::transmute(providergroup), ::std::mem::transmute(providerid), providername.into_param().abi(), ::std::mem::transmute(eventid), eventname.into_param().abi(), ::std::mem::transmute(eventversion), ::std::mem::transmute(eventkeywords), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
#[inline]
pub unsafe fn DdqSetTranscriptConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::HDIAGNOSTIC_DATA_QUERY_SESSION>>(hsession: Param0, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DdqSetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::runtime::HRESULT;
        }
        DdqSetTranscriptConfiguration(hsession.into_param().abi(), ::std::mem::transmute(desiredconfig)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
