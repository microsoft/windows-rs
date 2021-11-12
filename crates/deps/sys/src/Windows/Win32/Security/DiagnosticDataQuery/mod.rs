#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqCancelDiagnosticRecordOperation(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqCloseSession(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqCreateSession(accesslevel: DdqAccessLevel, hsession: *mut super::HDIAGNOSTIC_DATA_QUERY_SESSION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqExtractDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportkey: super::super::Foundation::PWSTR, destinationpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticRecordLocaleTags(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticRecordPage(hrecord: super::HDIAGNOSTIC_RECORD) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticRecordProducerCategories(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticRecordProducers(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticReport(hreport: super::HDIAGNOSTIC_REPORT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticDataAccessLevelAllowed(accesslevel: *mut DdqAccessLevel) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordAtIndex(hrecord: super::HDIAGNOSTIC_RECORD, index: u32, record: *mut DIAGNOSTIC_DATA_RECORD) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordBinaryDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, topnbinaries: u32, binarystats: *mut *mut DIAGNOSTIC_DATA_EVENT_BINARY_STATS, statcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordCategoryAtIndex(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, index: u32, categorydescription: *mut DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordCategoryCount(hcategorydescription: super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION, categorydescriptioncount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordCount(hrecord: super::HDIAGNOSTIC_RECORD, recordcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordLocaleTagAtIndex(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, index: u32, tagdescription: *mut DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordLocaleTagCount(htagdescription: super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION, tagdescriptioncount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordLocaleTags(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, locale: super::super::Foundation::PWSTR, htagdescription: *mut super::HDIAGNOSTIC_EVENT_TAG_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordPage(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, offset: u32, pagerecordcount: u32, baserowid: i64, hrecord: *mut super::HDIAGNOSTIC_RECORD) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordPayload(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, rowid: i64, payload: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordProducerAtIndex(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, index: u32, producerdescription: *mut DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordProducerCategories(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producername: super::super::Foundation::PWSTR, hcategorydescription: *mut super::HDIAGNOSTIC_EVENT_CATEGORY_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordProducerCount(hproducerdescription: super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION, producerdescriptioncount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordProducers(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, hproducerdescription: *mut super::HDIAGNOSTIC_EVENT_PRODUCER_DESCRIPTION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordStats(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, searchcriteria: *const DIAGNOSTIC_DATA_SEARCH_CRITERIA, recordcount: *mut u32, minrowid: *mut i64, maxrowid: *mut i64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordSummary(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, generalstats: *mut DIAGNOSTIC_DATA_GENERAL_STATS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordTagDistribution(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, producernames: *const super::super::Foundation::PWSTR, producernamecount: u32, tagstats: *mut *mut DIAGNOSTIC_DATA_EVENT_TAG_STATS, statcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticReport(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, hreport: *mut super::HDIAGNOSTIC_REPORT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticReportAtIndex(hreport: super::HDIAGNOSTIC_REPORT, index: u32, report: *mut DIAGNOSTIC_REPORT_DATA) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticReportCount(hreport: super::HDIAGNOSTIC_REPORT, reportcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticReportStoreReportCount(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, reportstoretype: u32, reportcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetSessionAccessLevel(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, accesslevel: *mut DdqAccessLevel) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, currentconfig: *mut DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqIsDiagnosticRecordSampledIn(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, providergroup: *const ::windows_sys::core::GUID, providerid: *const ::windows_sys::core::GUID, providername: super::super::Foundation::PWSTR, eventid: *const u32, eventname: super::super::Foundation::PWSTR, eventversion: *const u32, eventkeywords: *const u64, issampledin: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqSetTranscriptConfiguration(hsession: super::HDIAGNOSTIC_DATA_QUERY_SESSION, desiredconfig: *const DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION) -> ::windows_sys::core::HRESULT;
}
pub struct DIAGNOSTIC_DATA_EVENT_BINARY_STATS(i32);
pub struct DIAGNOSTIC_DATA_EVENT_CATEGORY_DESCRIPTION(i32);
pub struct DIAGNOSTIC_DATA_EVENT_PRODUCER_DESCRIPTION(i32);
pub struct DIAGNOSTIC_DATA_EVENT_TAG_DESCRIPTION(i32);
pub struct DIAGNOSTIC_DATA_EVENT_TAG_STATS(i32);
pub struct DIAGNOSTIC_DATA_EVENT_TRANSCRIPT_CONFIGURATION(i32);
pub struct DIAGNOSTIC_DATA_GENERAL_STATS(i32);
pub struct DIAGNOSTIC_DATA_RECORD(i32);
pub struct DIAGNOSTIC_DATA_SEARCH_CRITERIA(i32);
pub struct DIAGNOSTIC_REPORT_DATA(i32);
pub struct DIAGNOSTIC_REPORT_PARAMETER(i32);
pub struct DIAGNOSTIC_REPORT_SIGNATURE(i32);
pub struct DdqAccessLevel(i32);
