#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqCancelDiagnosticRecordOperation();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqCloseSession();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqCreateSession();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqExtractDiagnosticReport();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticRecordLocaleTags();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticRecordPage();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticRecordProducerCategories();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticRecordProducers();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqFreeDiagnosticReport();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticDataAccessLevelAllowed();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordAtIndex();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordBinaryDistribution();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordCategoryAtIndex();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordCategoryCount();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordCount();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordLocaleTagAtIndex();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordLocaleTagCount();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordLocaleTags();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordPage();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordPayload();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordProducerAtIndex();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordProducerCategories();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordProducerCount();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticRecordProducers();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordStats();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordSummary();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticRecordTagDistribution();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticReport();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqGetDiagnosticReportAtIndex();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticReportCount();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetDiagnosticReportStoreReportCount();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetSessionAccessLevel();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqGetTranscriptConfiguration();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdqIsDiagnosticRecordSampledIn();
    #[doc = "*Required features: `Win32_Security_DiagnosticDataQuery`*"]
    pub fn DdqSetTranscriptConfiguration();
}
