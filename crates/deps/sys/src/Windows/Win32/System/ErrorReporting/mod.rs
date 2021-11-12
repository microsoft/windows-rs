#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddERExcludedApplicationA(szapplication: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddERExcludedApplicationW(wszapplication: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
    pub fn ReportFault(pep: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, dwopt: u32) -> EFaultRepRetVal;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerAddExcludedApplication(pwzexename: super::super::Foundation::PWSTR, ballusers: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerFreeString(pwszstr: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerGetFlags(hprocess: super::super::Foundation::HANDLE, pdwflags: *mut WER_FAULT_REPORTING) -> ::windows_sys::core::HRESULT;
    pub fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterAppLocalDump(localappdatarelativepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterCustomMetadata(key: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn WerRegisterExcludedMemoryBlock(address: *const ::core::ffi::c_void, size: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterFile(pwzfile: super::super::Foundation::PWSTR, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: WER_FILE) -> ::windows_sys::core::HRESULT;
    pub fn WerRegisterMemoryBlock(pvaddress: *const ::core::ffi::c_void, dwsize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll: super::super::Foundation::PWSTR, pcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerRemoveExcludedApplication(pwzexename: super::super::Foundation::PWSTR, ballusers: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
    pub fn WerReportAddDump(hreporthandle: HREPORT, hprocess: super::super::Foundation::HANDLE, hthread: super::super::Foundation::HANDLE, dumptype: WER_DUMP_TYPE, pexceptionparam: *const WER_EXCEPTION_INFORMATION, pdumpcustomoptions: *const WER_DUMP_CUSTOM_OPTIONS, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportAddFile(hreporthandle: HREPORT, pwzpath: super::super::Foundation::PWSTR, repfiletype: WER_FILE_TYPE, dwfileflags: WER_FILE) -> ::windows_sys::core::HRESULT;
    pub fn WerReportCloseHandle(hreporthandle: HREPORT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportCreate(pwzeventtype: super::super::Foundation::PWSTR, reptype: WER_REPORT_TYPE, preportinformation: *const WER_REPORT_INFORMATION, phreporthandle: *mut HREPORT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportHang(hwndhungapp: super::super::Foundation::HWND, pwzhungapplicationname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportSetParameter(hreporthandle: HREPORT, dwparamid: u32, pwzname: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerReportSetUIOption(hreporthandle: HREPORT, repuitypeid: WER_REPORT_UI, pwzvalue: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn WerReportSubmit(hreporthandle: HREPORT, consent: WER_CONSENT, dwflags: WER_SUBMIT_FLAGS, psubmitresult: *mut WER_SUBMIT_RESULT) -> ::windows_sys::core::HRESULT;
    pub fn WerSetFlags(dwflags: WER_FAULT_REPORTING) -> ::windows_sys::core::HRESULT;
    pub fn WerStoreClose(hreportstore: HREPORTSTORE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreGetFirstReportKey(hreportstore: HREPORTSTORE, ppszreportkey: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreGetNextReportKey(hreportstore: HREPORTSTORE, ppszreportkey: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn WerStoreGetReportCount(hreportstore: HREPORTSTORE, pdwreportcount: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WerStoreGetSizeOnDisk(hreportstore: HREPORTSTORE, pqwsizeinbytes: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES, phreportstore: *mut HREPORTSTORE) -> ::windows_sys::core::HRESULT;
    pub fn WerStorePurge() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV1(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V1) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV2(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V2) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreQueryReportMetadataV3(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, preportmetadata: *mut WER_REPORT_METADATA_V3) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerStoreUploadReport(hreportstore: HREPORTSTORE, pszreportkey: super::super::Foundation::PWSTR, dwflags: u32, psubmitresult: *mut WER_SUBMIT_RESULT) -> ::windows_sys::core::HRESULT;
    pub fn WerUnregisterAdditionalProcess(processid: u32) -> ::windows_sys::core::HRESULT;
    pub fn WerUnregisterAppLocalDump() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterCustomMetadata(key: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn WerUnregisterExcludedMemoryBlock(address: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterFile(pwzfilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn WerUnregisterMemoryBlock(pvaddress: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll: super::super::Foundation::PWSTR, pcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct EFaultRepRetVal(pub i32);
pub const frrvOk: EFaultRepRetVal = EFaultRepRetVal(0i32);
pub const frrvOkManifest: EFaultRepRetVal = EFaultRepRetVal(1i32);
pub const frrvOkQueued: EFaultRepRetVal = EFaultRepRetVal(2i32);
pub const frrvErr: EFaultRepRetVal = EFaultRepRetVal(3i32);
pub const frrvErrNoDW: EFaultRepRetVal = EFaultRepRetVal(4i32);
pub const frrvErrTimeout: EFaultRepRetVal = EFaultRepRetVal(5i32);
pub const frrvLaunchDebugger: EFaultRepRetVal = EFaultRepRetVal(6i32);
pub const frrvOkHeadless: EFaultRepRetVal = EFaultRepRetVal(7i32);
pub const frrvErrAnotherInstance: EFaultRepRetVal = EFaultRepRetVal(8i32);
pub const frrvErrNoMemory: EFaultRepRetVal = EFaultRepRetVal(9i32);
pub const frrvErrDoubleFault: EFaultRepRetVal = EFaultRepRetVal(10i32);
#[repr(C)]
pub struct HREPORT(i32);
#[repr(C)]
pub struct HREPORTSTORE(i32);
#[repr(C)]
pub struct PFN_WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH(i32);
#[repr(C)]
pub struct PFN_WER_RUNTIME_EXCEPTION_EVENT(i32);
#[repr(C)]
pub struct PFN_WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE(i32);
#[repr(transparent)]
pub struct REPORT_STORE_TYPES(pub i32);
pub const E_STORE_USER_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(0i32);
pub const E_STORE_USER_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(1i32);
pub const E_STORE_MACHINE_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(2i32);
pub const E_STORE_MACHINE_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(3i32);
pub const E_STORE_INVALID: REPORT_STORE_TYPES = REPORT_STORE_TYPES(4i32);
#[repr(transparent)]
pub struct WER_CONSENT(pub i32);
pub const WerConsentNotAsked: WER_CONSENT = WER_CONSENT(1i32);
pub const WerConsentApproved: WER_CONSENT = WER_CONSENT(2i32);
pub const WerConsentDenied: WER_CONSENT = WER_CONSENT(3i32);
pub const WerConsentAlwaysPrompt: WER_CONSENT = WER_CONSENT(4i32);
pub const WerConsentMax: WER_CONSENT = WER_CONSENT(5i32);
pub const WER_DUMP_AUXILIARY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_DUMP_CUSTOM_OPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_DUMP_CUSTOM_OPTIONS_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_DUMP_CUSTOM_OPTIONS_V3(i32);
pub const WER_DUMP_MASK_START: u32 = 1u32;
pub const WER_DUMP_NOHEAP_ONQUEUE: u32 = 1u32;
#[repr(transparent)]
pub struct WER_DUMP_TYPE(pub i32);
pub const WerDumpTypeNone: WER_DUMP_TYPE = WER_DUMP_TYPE(0i32);
pub const WerDumpTypeMicroDump: WER_DUMP_TYPE = WER_DUMP_TYPE(1i32);
pub const WerDumpTypeMiniDump: WER_DUMP_TYPE = WER_DUMP_TYPE(2i32);
pub const WerDumpTypeHeapDump: WER_DUMP_TYPE = WER_DUMP_TYPE(3i32);
pub const WerDumpTypeTriageDump: WER_DUMP_TYPE = WER_DUMP_TYPE(4i32);
pub const WerDumpTypeMax: WER_DUMP_TYPE = WER_DUMP_TYPE(5i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct WER_EXCEPTION_INFORMATION(i32);
#[repr(transparent)]
pub struct WER_FAULT_REPORTING(pub u32);
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: WER_FAULT_REPORTING = WER_FAULT_REPORTING(4u32);
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: WER_FAULT_REPORTING = WER_FAULT_REPORTING(1u32);
pub const WER_FAULT_REPORTING_FLAG_QUEUE: WER_FAULT_REPORTING = WER_FAULT_REPORTING(2u32);
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: WER_FAULT_REPORTING = WER_FAULT_REPORTING(8u32);
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: WER_FAULT_REPORTING = WER_FAULT_REPORTING(16u32);
pub const WER_FAULT_REPORTING_CRITICAL: u32 = 512u32;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: u32 = 128u32;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: u32 = 256u32;
pub const WER_FAULT_REPORTING_DURABLE: u32 = 1024u32;
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: u32 = 64u32;
pub const WER_FAULT_REPORTING_NO_UI: u32 = 32u32;
#[repr(transparent)]
pub struct WER_FILE(pub u32);
pub const WER_FILE_ANONYMOUS_DATA: WER_FILE = WER_FILE(2u32);
pub const WER_FILE_DELETE_WHEN_DONE: WER_FILE = WER_FILE(1u32);
pub const WER_FILE_COMPRESSED: u32 = 4u32;
#[repr(transparent)]
pub struct WER_FILE_TYPE(pub i32);
pub const WerFileTypeMicrodump: WER_FILE_TYPE = WER_FILE_TYPE(1i32);
pub const WerFileTypeMinidump: WER_FILE_TYPE = WER_FILE_TYPE(2i32);
pub const WerFileTypeHeapdump: WER_FILE_TYPE = WER_FILE_TYPE(3i32);
pub const WerFileTypeUserDocument: WER_FILE_TYPE = WER_FILE_TYPE(4i32);
pub const WerFileTypeOther: WER_FILE_TYPE = WER_FILE_TYPE(5i32);
pub const WerFileTypeTriagedump: WER_FILE_TYPE = WER_FILE_TYPE(6i32);
pub const WerFileTypeCustomDump: WER_FILE_TYPE = WER_FILE_TYPE(7i32);
pub const WerFileTypeAuxiliaryDump: WER_FILE_TYPE = WER_FILE_TYPE(8i32);
pub const WerFileTypeEtlTrace: WER_FILE_TYPE = WER_FILE_TYPE(9i32);
pub const WerFileTypeMax: WER_FILE_TYPE = WER_FILE_TYPE(10i32);
pub const WER_MAX_APPLICATION_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_BUCKET_ID_STRING_LENGTH: u32 = 260u32;
pub const WER_MAX_DESCRIPTION_LENGTH: u32 = 512u32;
pub const WER_MAX_EVENT_NAME_LENGTH: u32 = 64u32;
pub const WER_MAX_FRIENDLY_EVENT_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_LOCAL_DUMP_SUBPATH_LENGTH: u32 = 64u32;
pub const WER_MAX_PARAM_COUNT: u32 = 10u32;
pub const WER_MAX_PARAM_LENGTH: u32 = 260u32;
pub const WER_MAX_PREFERRED_MODULES: u32 = 128u32;
pub const WER_MAX_PREFERRED_MODULES_BUFFER: u32 = 256u32;
pub const WER_MAX_REGISTERED_DUMPCOLLECTION: u32 = 4u32;
pub const WER_MAX_REGISTERED_ENTRIES: u32 = 512u32;
pub const WER_MAX_REGISTERED_METADATA: u32 = 8u32;
pub const WER_MAX_REGISTERED_RUNTIME_EXCEPTION_MODULES: u32 = 16u32;
pub const WER_MAX_SIGNATURE_NAME_LENGTH: u32 = 128u32;
pub const WER_MAX_TOTAL_PARAM_LENGTH: u32 = 1720u32;
pub const WER_METADATA_KEY_MAX_LENGTH: u32 = 64u32;
pub const WER_METADATA_VALUE_MAX_LENGTH: u32 = 128u32;
pub const WER_P0: u32 = 0u32;
pub const WER_P1: u32 = 1u32;
pub const WER_P2: u32 = 2u32;
pub const WER_P3: u32 = 3u32;
pub const WER_P4: u32 = 4u32;
pub const WER_P5: u32 = 5u32;
pub const WER_P6: u32 = 6u32;
pub const WER_P7: u32 = 7u32;
pub const WER_P8: u32 = 8u32;
pub const WER_P9: u32 = 9u32;
#[repr(transparent)]
pub struct WER_REGISTER_FILE_TYPE(pub i32);
pub const WerRegFileTypeUserDocument: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(1i32);
pub const WerRegFileTypeOther: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(2i32);
pub const WerRegFileTypeMax: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_REPORT_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_REPORT_INFORMATION_V3(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_REPORT_INFORMATION_V4(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_REPORT_INFORMATION_V5(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_REPORT_METADATA_V1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_REPORT_METADATA_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WER_REPORT_METADATA_V3(i32);
#[repr(C)]
pub struct WER_REPORT_PARAMETER(i32);
#[repr(C)]
pub struct WER_REPORT_SIGNATURE(i32);
#[repr(transparent)]
pub struct WER_REPORT_TYPE(pub i32);
pub const WerReportNonCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(0i32);
pub const WerReportCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(1i32);
pub const WerReportApplicationCrash: WER_REPORT_TYPE = WER_REPORT_TYPE(2i32);
pub const WerReportApplicationHang: WER_REPORT_TYPE = WER_REPORT_TYPE(3i32);
pub const WerReportKernel: WER_REPORT_TYPE = WER_REPORT_TYPE(4i32);
pub const WerReportInvalid: WER_REPORT_TYPE = WER_REPORT_TYPE(5i32);
#[repr(transparent)]
pub struct WER_REPORT_UI(pub i32);
pub const WerUIAdditionalDataDlgHeader: WER_REPORT_UI = WER_REPORT_UI(1i32);
pub const WerUIIconFilePath: WER_REPORT_UI = WER_REPORT_UI(2i32);
pub const WerUIConsentDlgHeader: WER_REPORT_UI = WER_REPORT_UI(3i32);
pub const WerUIConsentDlgBody: WER_REPORT_UI = WER_REPORT_UI(4i32);
pub const WerUIOnlineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(5i32);
pub const WerUIOfflineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(6i32);
pub const WerUICloseText: WER_REPORT_UI = WER_REPORT_UI(7i32);
pub const WerUICloseDlgHeader: WER_REPORT_UI = WER_REPORT_UI(8i32);
pub const WerUICloseDlgBody: WER_REPORT_UI = WER_REPORT_UI(9i32);
pub const WerUICloseDlgButtonText: WER_REPORT_UI = WER_REPORT_UI(10i32);
pub const WerUIMax: WER_REPORT_UI = WER_REPORT_UI(11i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct WER_RUNTIME_EXCEPTION_INFORMATION(i32);
pub const WER_SUBMIT_BYPASS_NETWORK_COST_THROTTLING: u32 = 32768u32;
pub const WER_SUBMIT_BYPASS_POWER_THROTTLING: u32 = 16384u32;
#[repr(transparent)]
pub struct WER_SUBMIT_FLAGS(pub u32);
pub const WER_SUBMIT_ADD_REGISTERED_DATA: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(16u32);
pub const WER_SUBMIT_HONOR_RECOVERY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1u32);
pub const WER_SUBMIT_HONOR_RESTART: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2u32);
pub const WER_SUBMIT_NO_ARCHIVE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(256u32);
pub const WER_SUBMIT_NO_CLOSE_UI: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(64u32);
pub const WER_SUBMIT_NO_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(128u32);
pub const WER_SUBMIT_OUTOFPROCESS: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(32u32);
pub const WER_SUBMIT_OUTOFPROCESS_ASYNC: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1024u32);
pub const WER_SUBMIT_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4u32);
pub const WER_SUBMIT_SHOW_DEBUG: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8u32);
pub const WER_SUBMIT_START_MINIMIZED: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(512u32);
pub const WER_SUBMIT_BYPASS_DATA_THROTTLING: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2048u32);
pub const WER_SUBMIT_ARCHIVE_PARAMETERS_ONLY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4096u32);
pub const WER_SUBMIT_REPORT_MACHINE_ID: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8192u32);
#[repr(transparent)]
pub struct WER_SUBMIT_RESULT(pub i32);
pub const WerReportQueued: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(1i32);
pub const WerReportUploaded: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(2i32);
pub const WerReportDebug: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(3i32);
pub const WerReportFailed: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(4i32);
pub const WerDisabled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(5i32);
pub const WerReportCancelled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(6i32);
pub const WerDisabledQueue: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(7i32);
pub const WerReportAsync: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(8i32);
pub const WerCustomAction: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(9i32);
pub const WerThrottled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(10i32);
pub const WerReportUploadedCab: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(11i32);
pub const WerStorageLocationNotFound: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(12i32);
pub const WerSubmitResultMax: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(13i32);
#[repr(C)]
pub struct pfn_ADDEREXCLUDEDAPPLICATIONA(i32);
#[repr(C)]
pub struct pfn_ADDEREXCLUDEDAPPLICATIONW(i32);
#[repr(C)]
pub struct pfn_REPORTFAULT(i32);
