#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqCancelDiagnosticRecordOperation(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqCloseSession(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqCreateSession(accesslevel: DdqAccessLevel, hsession: *mut super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqExtractDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportkey: super::super::Foundation::PWSTR, destinationpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqFreeDiagnosticRecordLocaleTags(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqFreeDiagnosticRecordPage(hrecord: super::HDIAGNOSTIC_RECORD) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqFreeDiagnosticRecordProducers(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqFreeDiagnosticReport(hreport: super::HDIAGNOSTIC_REPORT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticDataAccessLevelAllowed(accesslevel: *mut DdqAccessLevel) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordAtIndex(hrecord: super::HDIAGNOSTIC_RECORD, index: u32, record: *mut DIAGNOSTIC_DATA_RECORD) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordBinaryDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index: u32, categorydescription: *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, categorydescriptioncount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticRecordCount(hrecord: super::HDIAGNOSTIC_RECORD, recordcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index: u32, tagdescription: *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, tagdescriptioncount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordLocaleTags(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, locale: super::super::Foundation::PWSTR, htagdescription: *mut super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordPage(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64, hrecord: *mut super::HDIAGNOSTIC_RECORD) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordPayload(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, rowid: i64, payload: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index: u32, producerdescription: *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordProducerCategories(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producername: super::super::Foundation::PWSTR, hcategorydescription: *mut super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticRecordProducerCount(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, producerdescriptioncount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticRecordProducers(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, hproducerdescription: *mut super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordStats(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordSummary(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordTagDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, hreport: *mut super::HDIAGNOSTIC_REPORT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticReportAtIndex(hreport: super::HDIAGNOSTIC_REPORT, index: u32, report: *mut DIAGNOSTIC_REPORT_DATA) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticReportCount(hreport: super::HDIAGNOSTIC_REPORT, reportcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetDiagnosticReportStoreReportCount(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetSessionAccessLevel(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, accesslevel: *mut DdqAccessLevel) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqGetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, currentconfig: *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqIsDiagnosticRecordSampledIn(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup: *const ::windows_sys::core::GUID, providerid: *const ::windows_sys::core::GUID, providername: super::super::Foundation::PWSTR, eventid: *const u32, eventname: super::super::Foundation::PWSTR, eventversion: *const u32, eventkeywords: *const u64, issampledin: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
    pub fn DdqSetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    pub moduleName: super::super::Foundation::PWSTR,
    pub friendlyModuleName: super::super::Foundation::PWSTR,
    pub eventCount: u32,
    pub uploadSizeBytes: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_BINARY_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    pub id: i32,
    pub name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    pub name: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    pub privacyTag: i32,
    pub name: super::super::Foundation::PWSTR,
    pub description: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
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
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
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
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
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
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DIAGNOSTIC_DATA_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for DIAGNOSTIC_DATA_SEARCH_CRITERIA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_DATA_SEARCH_CRITERIA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DIAGNOSTIC_REPORT_DATA {
    pub signature: DIAGNOSTIC_REPORT_SIGNATURE,
    pub bucketId: ::windows_sys::core::GUID,
    pub reportId: ::windows_sys::core::GUID,
    pub creationTime: super::super::Foundation::FILETIME,
    pub sizeInBytes: u64,
    pub cabId: super::super::Foundation::PWSTR,
    pub reportStatus: u32,
    pub reportIntegratorId: ::windows_sys::core::GUID,
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
impl ::core::marker::Copy for DIAGNOSTIC_REPORT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DIAGNOSTIC_REPORT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
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
#[repr(C)]
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
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
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
pub type DdqAccessLevel = i32;
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
pub const NoData: DdqAccessLevel = 0i32;
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
pub const CurrentUserData: DdqAccessLevel = 1i32;
#[doc = "*Required features: 'Win32_Security_DiagnosticDataQuery'*"]
pub const AllUserData: DdqAccessLevel = 2i32;
