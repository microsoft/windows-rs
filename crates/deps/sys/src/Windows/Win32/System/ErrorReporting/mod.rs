#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_DUMP_AUXILIARY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_DUMP_MASK_START: u32 = 1u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_DUMP_NOHEAP_ONQUEUE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_CRITICAL: u32 = 512u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: u32 = 256u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_DURABLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FAULT_REPORTING_NO_UI: u32 = 32u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_FILE_COMPRESSED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_APPLICATION_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_BUCKET_ID_STRING_LENGTH: u32 = 260u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_DESCRIPTION_LENGTH: u32 = 512u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_EVENT_NAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_FRIENDLY_EVENT_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_LOCAL_DUMP_SUBPATH_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_PARAM_COUNT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_PARAM_LENGTH: u32 = 260u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_PREFERRED_MODULES: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_PREFERRED_MODULES_BUFFER: u32 = 256u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_REGISTERED_DUMPCOLLECTION: u32 = 4u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_REGISTERED_ENTRIES: u32 = 512u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_REGISTERED_METADATA: u32 = 8u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_REGISTERED_RUNTIME_EXCEPTION_MODULES: u32 = 16u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_SIGNATURE_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_MAX_TOTAL_PARAM_LENGTH: u32 = 1720u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_METADATA_KEY_MAX_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_METADATA_VALUE_MAX_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P0: u32 = 0u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P2: u32 = 2u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P3: u32 = 3u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P4: u32 = 4u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P5: u32 = 5u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P6: u32 = 6u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P7: u32 = 7u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P8: u32 = 8u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_P9: u32 = 9u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_SUBMIT_BYPASS_NETWORK_COST_THROTTLING: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_ErrorReporting`*"]
pub const WER_SUBMIT_BYPASS_POWER_THROTTLING: u32 = 16384u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddERExcludedApplicationA(szapplication: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddERExcludedApplicationW(wszapplication: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
    pub fn ReportFault(pep: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, dwopt: u32) -> EFaultRepRetVal;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerAddExcludedApplication(pwzexename: super::super::Foundation::PWSTR, ballusers: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerFreeString(pwszstr: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerGetFlags(hprocess: super::super::Foundation::HANDLE, pdwflags: *mut WER_FAULT_REPORTING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterAppLocalDump(localappdatarelativepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterCustomMetadata(key: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerRegisterExcludedMemoryBlock(address: *const ::core::ffi::c_void, size: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterFile(pwzfile: super::super::Foundation::PWSTR, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: WER_FILE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerRegisterMemoryBlock(pvaddress: *const ::core::ffi::c_void, dwsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll: super::super::Foundation::PWSTR, pcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRemoveExcludedApplication(pwzexename: super::super::Foundation::PWSTR, ballusers: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
    pub fn WerReportAddDump(hreporthandle: HREPORT, hprocess: super::super::Foundation::HANDLE, hthread: super::super::Foundation::HANDLE, dumptype: WER_DUMP_TYPE, pexceptionparam: *const WER_EXCEPTION_INFORMATION, pdumpcustomoptions: *const WER_DUMP_CUSTOM_OPTIONS, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportAddFile(hreporthandle: HREPORT, pwzpath: super::super::Foundation::PWSTR, repfiletype: WER_FILE_TYPE, dwfileflags: WER_FILE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerReportCloseHandle(hreporthandle: HREPORT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportCreate(pwzeventtype: super::super::Foundation::PWSTR, reptype: WER_REPORT_TYPE, preportinformation: *const WER_REPORT_INFORMATION, phreporthandle: *mut HREPORT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportHang(hwndhungapp: super::super::Foundation::HWND, pwzhungapplicationname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportSetParameter(hreporthandle: HREPORT, dwparamid: u32, pwzname: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportSetUIOption(hreporthandle: HREPORT, repuitypeid: WER_REPORT_UI, pwzvalue: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerReportSubmit(hreporthandle: HREPORT, consent: WER_CONSENT, dwflags: WER_SUBMIT_FLAGS, psubmitresult: *mut WER_SUBMIT_RESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerSetFlags(dwflags: WER_FAULT_REPORTING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStoreClose(hreportstore: HREPORTSTORE);
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreGetFirstReportKey(hreportstore: HREPORTSTORE, ppszreportkey: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreGetNextReportKey(hreportstore: HREPORTSTORE, ppszreportkey: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStoreGetReportCount(hreportstore: HREPORTSTORE, pdwreportcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStoreGetSizeOnDisk(hreportstore: HREPORTSTORE, pqwsizeinbytes: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES, phreportstore: *mut HREPORTSTORE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerStorePurge() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV1(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V1) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV2(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V2) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV3(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V3) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreUploadReport(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, dwflags: u32, psubmitresult: *mut WER_SUBMIT_RESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerUnregisterAdditionalProcess(processid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerUnregisterAppLocalDump() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterCustomMetadata(key: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerUnregisterExcludedMemoryBlock(address: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterFile(pwzfilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`*"]
    pub fn WerUnregisterMemoryBlock(pvaddress: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ErrorReporting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll: super::super::Foundation::PWSTR, pcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
