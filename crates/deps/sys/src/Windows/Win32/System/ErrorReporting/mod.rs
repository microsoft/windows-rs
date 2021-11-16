#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const frrvOk: i32 = 0i32;
pub const frrvOkManifest: i32 = 1i32;
pub const frrvOkQueued: i32 = 2i32;
pub const frrvErr: i32 = 3i32;
pub const frrvErrNoDW: i32 = 4i32;
pub const frrvErrTimeout: i32 = 5i32;
pub const frrvLaunchDebugger: i32 = 6i32;
pub const frrvOkHeadless: i32 = 7i32;
pub const frrvErrAnotherInstance: i32 = 8i32;
pub const frrvErrNoMemory: i32 = 9i32;
pub const frrvErrDoubleFault: i32 = 10i32;
pub type HREPORT = isize;
pub type HREPORTSTORE = isize;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH = unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbiscustomdebugger: *mut super::super::Foundation::BOOL, pwszdebuggerlaunch: super::super::Foundation::PWSTR, pchdebuggerlaunch: *mut u32, pbisdebuggerautolaunch: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT = unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbownershipclaimed: *mut super::super::Foundation::BOOL, pwszeventname: super::super::Foundation::PWSTR, pchsize: *mut u32, pdwsignaturecount: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE = unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, dwindex: u32, pwszname: super::super::Foundation::PWSTR, pchname: *mut u32, pwszvalue: super::super::Foundation::PWSTR, pchvalue: *mut u32) -> ::windows_sys::core::HRESULT;
pub const E_STORE_USER_ARCHIVE: i32 = 0i32;
pub const E_STORE_USER_QUEUE: i32 = 1i32;
pub const E_STORE_MACHINE_ARCHIVE: i32 = 2i32;
pub const E_STORE_MACHINE_QUEUE: i32 = 3i32;
pub const E_STORE_INVALID: i32 = 4i32;
pub const WerConsentNotAsked: i32 = 1i32;
pub const WerConsentApproved: i32 = 2i32;
pub const WerConsentDenied: i32 = 3i32;
pub const WerConsentAlwaysPrompt: i32 = 4i32;
pub const WerConsentMax: i32 = 5i32;
pub const WER_DUMP_AUXILIARY: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_DUMP_CUSTOM_OPTIONS {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_DUMP_CUSTOM_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_DUMP_CUSTOM_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_DUMP_CUSTOM_OPTIONS_V2 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_DUMP_CUSTOM_OPTIONS_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_DUMP_CUSTOM_OPTIONS_V3 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: super::super::Foundation::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
    pub pvDumpKey: *mut ::core::ffi::c_void,
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub dwThreadID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_DUMP_CUSTOM_OPTIONS_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WER_DUMP_MASK_START: u32 = 1u32;
pub const WER_DUMP_NOHEAP_ONQUEUE: u32 = 1u32;
pub const WerDumpTypeNone: i32 = 0i32;
pub const WerDumpTypeMicroDump: i32 = 1i32;
pub const WerDumpTypeMiniDump: i32 = 2i32;
pub const WerDumpTypeHeapDump: i32 = 3i32;
pub const WerDumpTypeTriageDump: i32 = 4i32;
pub const WerDumpTypeMax: i32 = 5i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub struct WER_EXCEPTION_INFORMATION {
    pub pExceptionPointers: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS,
    pub bClientPointers: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for WER_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for WER_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: u32 = 4u32;
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: u32 = 1u32;
pub const WER_FAULT_REPORTING_FLAG_QUEUE: u32 = 2u32;
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: u32 = 8u32;
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: u32 = 16u32;
pub const WER_FAULT_REPORTING_CRITICAL: u32 = 512u32;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: u32 = 128u32;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: u32 = 256u32;
pub const WER_FAULT_REPORTING_DURABLE: u32 = 1024u32;
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: u32 = 64u32;
pub const WER_FAULT_REPORTING_NO_UI: u32 = 32u32;
pub const WER_FILE_ANONYMOUS_DATA: u32 = 2u32;
pub const WER_FILE_DELETE_WHEN_DONE: u32 = 1u32;
pub const WER_FILE_COMPRESSED: u32 = 4u32;
pub const WerFileTypeMicrodump: i32 = 1i32;
pub const WerFileTypeMinidump: i32 = 2i32;
pub const WerFileTypeHeapdump: i32 = 3i32;
pub const WerFileTypeUserDocument: i32 = 4i32;
pub const WerFileTypeOther: i32 = 5i32;
pub const WerFileTypeTriagedump: i32 = 6i32;
pub const WerFileTypeCustomDump: i32 = 7i32;
pub const WerFileTypeAuxiliaryDump: i32 = 8i32;
pub const WerFileTypeEtlTrace: i32 = 9i32;
pub const WerFileTypeMax: i32 = 10i32;
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
pub const WerRegFileTypeUserDocument: i32 = 1i32;
pub const WerRegFileTypeOther: i32 = 2i32;
pub const WerRegFileTypeMax: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_REPORT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_REPORT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_INFORMATION_V3 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_REPORT_INFORMATION_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_REPORT_INFORMATION_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_INFORMATION_V4 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub hDeleteFilesImpersonationToken: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_REPORT_INFORMATION_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_REPORT_INFORMATION_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_INFORMATION_V5 {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::super::Foundation::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: super::super::Foundation::HANDLE,
    pub hDeleteFilesImpersonationToken: super::super::Foundation::HANDLE,
    pub submitResultMax: WER_SUBMIT_RESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_REPORT_INFORMATION_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_REPORT_INFORMATION_V5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_METADATA_V1 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows_sys::core::GUID,
    pub ReportId: ::windows_sys::core::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_REPORT_METADATA_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_REPORT_METADATA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_METADATA_V2 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows_sys::core::GUID,
    pub ReportId: ::windows_sys::core::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: ::windows_sys::core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_REPORT_METADATA_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_REPORT_METADATA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_METADATA_V3 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows_sys::core::GUID,
    pub ReportId: ::windows_sys::core::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: ::windows_sys::core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: super::super::Foundation::PWSTR,
    pub FriendlyEventName: [u16; 128],
    pub ApplicationName: [u16; 128],
    pub ApplicationPath: [u16; 260],
    pub Description: [u16; 512],
    pub BucketIdString: [u16; 260],
    pub LegacyBucketId: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_REPORT_METADATA_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_REPORT_METADATA_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WER_REPORT_PARAMETER {
    pub Name: [u16; 129],
    pub Value: [u16; 260],
}
impl ::core::marker::Copy for WER_REPORT_PARAMETER {}
impl ::core::clone::Clone for WER_REPORT_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WER_REPORT_SIGNATURE {
    pub EventName: [u16; 65],
    pub Parameters: [WER_REPORT_PARAMETER; 10],
}
impl ::core::marker::Copy for WER_REPORT_SIGNATURE {}
impl ::core::clone::Clone for WER_REPORT_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WerReportNonCritical: i32 = 0i32;
pub const WerReportCritical: i32 = 1i32;
pub const WerReportApplicationCrash: i32 = 2i32;
pub const WerReportApplicationHang: i32 = 3i32;
pub const WerReportKernel: i32 = 4i32;
pub const WerReportInvalid: i32 = 5i32;
pub const WerUIAdditionalDataDlgHeader: i32 = 1i32;
pub const WerUIIconFilePath: i32 = 2i32;
pub const WerUIConsentDlgHeader: i32 = 3i32;
pub const WerUIConsentDlgBody: i32 = 4i32;
pub const WerUIOnlineSolutionCheckText: i32 = 5i32;
pub const WerUIOfflineSolutionCheckText: i32 = 6i32;
pub const WerUICloseText: i32 = 7i32;
pub const WerUICloseDlgHeader: i32 = 8i32;
pub const WerUICloseDlgBody: i32 = 9i32;
pub const WerUICloseDlgButtonText: i32 = 10i32;
pub const WerUIMax: i32 = 11i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub struct WER_RUNTIME_EXCEPTION_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub exceptionRecord: super::Diagnostics::Debug::EXCEPTION_RECORD,
    pub context: super::Diagnostics::Debug::CONTEXT,
    pub pwszReportId: super::super::Foundation::PWSTR,
    pub bIsFatal: super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for WER_RUNTIME_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WER_SUBMIT_BYPASS_NETWORK_COST_THROTTLING: u32 = 32768u32;
pub const WER_SUBMIT_BYPASS_POWER_THROTTLING: u32 = 16384u32;
pub const WER_SUBMIT_ADD_REGISTERED_DATA: u32 = 16u32;
pub const WER_SUBMIT_HONOR_RECOVERY: u32 = 1u32;
pub const WER_SUBMIT_HONOR_RESTART: u32 = 2u32;
pub const WER_SUBMIT_NO_ARCHIVE: u32 = 256u32;
pub const WER_SUBMIT_NO_CLOSE_UI: u32 = 64u32;
pub const WER_SUBMIT_NO_QUEUE: u32 = 128u32;
pub const WER_SUBMIT_OUTOFPROCESS: u32 = 32u32;
pub const WER_SUBMIT_OUTOFPROCESS_ASYNC: u32 = 1024u32;
pub const WER_SUBMIT_QUEUE: u32 = 4u32;
pub const WER_SUBMIT_SHOW_DEBUG: u32 = 8u32;
pub const WER_SUBMIT_START_MINIMIZED: u32 = 512u32;
pub const WER_SUBMIT_BYPASS_DATA_THROTTLING: u32 = 2048u32;
pub const WER_SUBMIT_ARCHIVE_PARAMETERS_ONLY: u32 = 4096u32;
pub const WER_SUBMIT_REPORT_MACHINE_ID: u32 = 8192u32;
pub const WerReportQueued: i32 = 1i32;
pub const WerReportUploaded: i32 = 2i32;
pub const WerReportDebug: i32 = 3i32;
pub const WerReportFailed: i32 = 4i32;
pub const WerDisabled: i32 = 5i32;
pub const WerReportCancelled: i32 = 6i32;
pub const WerDisabledQueue: i32 = 7i32;
pub const WerReportAsync: i32 = 8i32;
pub const WerCustomAction: i32 = 9i32;
pub const WerThrottled: i32 = 10i32;
pub const WerReportUploadedCab: i32 = 11i32;
pub const WerStorageLocationNotFound: i32 = 12i32;
pub const WerSubmitResultMax: i32 = 13i32;
#[cfg(feature = "Win32_Foundation")]
pub type pfn_ADDEREXCLUDEDAPPLICATIONA = unsafe extern "system" fn(param0: super::super::Foundation::PSTR) -> EFaultRepRetVal;
#[cfg(feature = "Win32_Foundation")]
pub type pfn_ADDEREXCLUDEDAPPLICATIONW = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR) -> EFaultRepRetVal;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type pfn_REPORTFAULT = unsafe extern "system" fn(param0: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, param1: u32) -> EFaultRepRetVal;
