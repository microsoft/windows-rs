#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddERExcludedApplicationA();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddERExcludedApplicationW();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
    pub fn ReportFault();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerAddExcludedApplication();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerFreeString();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerGetFlags();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerRegisterAdditionalProcess();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterAppLocalDump();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterCustomMetadata();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerRegisterExcludedMemoryBlock();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterFile();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerRegisterMemoryBlock();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterRuntimeExceptionModule();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRemoveExcludedApplication();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
    pub fn WerReportAddDump();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportAddFile();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerReportCloseHandle();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportCreate();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportHang();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportSetParameter();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportSetUIOption();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerReportSubmit();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerSetFlags();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStoreClose();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreGetFirstReportKey();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreGetNextReportKey();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStoreGetReportCount();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStoreGetSizeOnDisk();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStoreOpen();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStorePurge();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV1();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV2();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV3();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreUploadReport();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerUnregisterAdditionalProcess();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerUnregisterAppLocalDump();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterCustomMetadata();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerUnregisterExcludedMemoryBlock();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterFile();
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerUnregisterMemoryBlock();
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterRuntimeExceptionModule();
}
