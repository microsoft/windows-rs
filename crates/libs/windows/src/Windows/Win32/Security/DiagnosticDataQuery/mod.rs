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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
pub unsafe fn DdqCancelDiagnosticRecordOperation<'a, P0>(hsession: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqCancelDiagnosticRecordOperation(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
    }
    DdqCancelDiagnosticRecordOperation(hsession.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqCloseSession<'a, P0>(hsession: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqCloseSession(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
    }
    DdqCloseSession(hsession.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqCreateSession(accesslevel: DdqAccessLevel) -> ::windows::core::Result<super::HDIAGNOSTIC_DATA_QUERY_SESSION> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqCreateSession(accesslevel: DdqAccessLevel, hsession: *mut super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqCreateSession(accesslevel, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::HDIAGNOSTIC_DATA_QUERY_SESSION>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqExtractDiagnosticReport<'a, P0, P1, P2>(hsession: P0, reportstoretype: u32, reportkey: P1, destinationpath: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqExtractDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportkey: ::windows::core::PCWSTR, destinationpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DdqExtractDiagnosticReport(hsession.into(), reportstoretype, reportkey.into(), destinationpath.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordLocaleTags<'a, P0>(htagdescription: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqFreeDiagnosticRecordLocaleTags(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    DdqFreeDiagnosticRecordLocaleTags(htagdescription.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordPage<'a, P0>(hrecord: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_RECORD>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqFreeDiagnosticRecordPage(hrecord: super::HDIAGNOSTIC_RECORD) -> ::windows::core::HRESULT;
    }
    DdqFreeDiagnosticRecordPage(hrecord.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducerCategories<'a, P0>(hcategorydescription: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    DdqFreeDiagnosticRecordProducerCategories(hcategorydescription.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticRecordProducers<'a, P0>(hproducerdescription: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqFreeDiagnosticRecordProducers(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    DdqFreeDiagnosticRecordProducers(hproducerdescription.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqFreeDiagnosticReport<'a, P0>(hreport: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_REPORT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqFreeDiagnosticReport(hreport: super::HDIAGNOSTIC_REPORT) -> ::windows::core::HRESULT;
    }
    DdqFreeDiagnosticReport(hreport.into()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticDataAccessLevelAllowed() -> ::windows::core::Result<DdqAccessLevel> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticDataAccessLevelAllowed(accesslevel: *mut DdqAccessLevel) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticDataAccessLevelAllowed(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DdqAccessLevel>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordAtIndex<'a, P0>(hrecord: P0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_RECORD>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_RECORD>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordAtIndex(hrecord: super::HDIAGNOSTIC_RECORD, index: u32, record: *mut DIAGNOSTIC_DATA_RECORD) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordAtIndex(hrecord.into(), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DIAGNOSTIC_DATA_RECORD>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordBinaryDistribution<'a, P0>(hsession: P0, producernames: &[::windows::core::PWSTR], topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordBinaryDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const ::windows::core::PWSTR, producernamecount: u32, topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows::core::HRESULT;
    }
    DdqGetDiagnosticRecordBinaryDistribution(hsession.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(producernames)), producernames.len() as _, topnbinaries, ::core::mem::transmute(binarystats), ::core::mem::transmute(statcount)).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryAtIndex<'a, P0>(hcategorydescription: P0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index: u32, categorydescription: *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription.into(), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCategoryCount<'a, P0>(hcategorydescription: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, categorydescriptioncount: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordCategoryCount(hcategorydescription.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordCount<'a, P0>(hrecord: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_RECORD>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordCount(hrecord: super::HDIAGNOSTIC_RECORD, recordcount: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordCount(hrecord.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagAtIndex<'a, P0>(htagdescription: P0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index: u32, tagdescription: *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription.into(), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTagCount<'a, P0>(htagdescription: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, tagdescriptioncount: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordLocaleTagCount(htagdescription.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordLocaleTags<'a, P0, P1>(hsession: P0, locale: P1) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordLocaleTags(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, locale: ::windows::core::PCWSTR, htagdescription: *mut super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordLocaleTags(hsession.into(), locale.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPage<'a, P0>(hsession: P0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64) -> ::windows::core::Result<super::HDIAGNOSTIC_RECORD>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordPage(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64, hrecord: *mut super::HDIAGNOSTIC_RECORD) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordPage(hsession.into(), ::core::mem::transmute(searchcriteria), offset, pagerecordcount, baserowid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::HDIAGNOSTIC_RECORD>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordPayload<'a, P0>(hsession: P0, rowid: i64) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordPayload(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, rowid: i64, payload: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordPayload(hsession.into(), rowid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerAtIndex<'a, P0>(hproducerdescription: P0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index: u32, producerdescription: *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription.into(), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCategories<'a, P0, P1>(hsession: P0, producername: P1) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordProducerCategories(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producername: ::windows::core::PCWSTR, hcategorydescription: *mut super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordProducerCategories(hsession.into(), producername.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducerCount<'a, P0>(hproducerdescription: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordProducerCount(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, producerdescriptioncount: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordProducerCount(hproducerdescription.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordProducers<'a, P0>(hsession: P0) -> ::windows::core::Result<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordProducers(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, hproducerdescription: *mut super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordProducers(hsession.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordStats<'a, P0>(hsession: P0, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordStats(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows::core::HRESULT;
    }
    DdqGetDiagnosticRecordStats(hsession.into(), ::core::mem::transmute(searchcriteria), ::core::mem::transmute(recordcount), ::core::mem::transmute(minrowid), ::core::mem::transmute(maxrowid)).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordSummary<'a, P0>(hsession: P0, producernames: &[::windows::core::PWSTR]) -> ::windows::core::Result<DIAGNOSTIC_DATA_GENERAL_STATS>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordSummary(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const ::windows::core::PWSTR, producernamecount: u32, generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticRecordSummary(hsession.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(producernames)), producernames.len() as _, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DIAGNOSTIC_DATA_GENERAL_STATS>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticRecordTagDistribution<'a, P0>(hsession: P0, producernames: &[::windows::core::PWSTR], tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticRecordTagDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const ::windows::core::PWSTR, producernamecount: u32, tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows::core::HRESULT;
    }
    DdqGetDiagnosticRecordTagDistribution(hsession.into(), ::core::mem::transmute(::windows::core::as_ptr_or_null(producernames)), producernames.len() as _, ::core::mem::transmute(tagstats), ::core::mem::transmute(statcount)).ok()
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReport<'a, P0>(hsession: P0, reportstoretype: u32) -> ::windows::core::Result<super::HDIAGNOSTIC_REPORT>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, hreport: *mut super::HDIAGNOSTIC_REPORT) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticReport(hsession.into(), reportstoretype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::HDIAGNOSTIC_REPORT>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqGetDiagnosticReportAtIndex<'a, P0>(hreport: P0, index: u32) -> ::windows::core::Result<DIAGNOSTIC_REPORT_DATA>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_REPORT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticReportAtIndex(hreport: super::HDIAGNOSTIC_REPORT, index: u32, report: *mut DIAGNOSTIC_REPORT_DATA) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticReportAtIndex(hreport.into(), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DIAGNOSTIC_REPORT_DATA>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReportCount<'a, P0>(hreport: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_REPORT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticReportCount(hreport: super::HDIAGNOSTIC_REPORT, reportcount: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticReportCount(hreport.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetDiagnosticReportStoreReportCount<'a, P0>(hsession: P0, reportstoretype: u32) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetDiagnosticReportStoreReportCount(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportcount: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetDiagnosticReportStoreReportCount(hsession.into(), reportstoretype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetSessionAccessLevel<'a, P0>(hsession: P0) -> ::windows::core::Result<DdqAccessLevel>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetSessionAccessLevel(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, accesslevel: *mut DdqAccessLevel) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetSessionAccessLevel(hsession.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DdqAccessLevel>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqGetTranscriptConfiguration<'a, P0>(hsession: P0) -> ::windows::core::Result<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqGetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, currentconfig: *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqGetTranscriptConfiguration(hsession.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DdqIsDiagnosticRecordSampledIn<'a, P0, P1, P2>(hsession: P0, providergroup: *const ::windows::core::GUID, providerid: *const ::windows::core::GUID, providername: P1, eventid: *const u32, eventname: P2, eventversion: *const u32, eventkeywords: *const u64) -> ::windows::core::Result<super::super::Foundation::BOOL>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqIsDiagnosticRecordSampledIn(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup: *const ::windows::core::GUID, providerid: *const ::windows::core::GUID, providername: ::windows::core::PCWSTR, eventid: *const u32, eventname: ::windows::core::PCWSTR, eventversion: *const u32, eventkeywords: *const u64, issampledin: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DdqIsDiagnosticRecordSampledIn(hsession.into(), ::core::mem::transmute(providergroup), ::core::mem::transmute(providerid), providername.into(), ::core::mem::transmute(eventid), eventname.into(), ::core::mem::transmute(eventversion), ::core::mem::transmute(eventkeywords), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
}
#[doc = "*Required features: `\"Win32_Security_DiagnosticDataQuery\"`*"]
#[inline]
pub unsafe fn DdqSetTranscriptConfiguration<'a, P0>(hsession: P0, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::HDIAGNOSTIC_DATA_QUERY_SESSION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DdqSetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows::core::HRESULT;
    }
    DdqSetTranscriptConfiguration(hsession.into(), ::core::mem::transmute(desiredconfig)).ok()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
