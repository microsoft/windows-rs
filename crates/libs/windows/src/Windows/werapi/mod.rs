#[inline]
pub unsafe fn WerAddExcludedApplication<P0>(pwzexename: P0, ballusers: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "system" fn WerAddExcludedApplication(pwzexename : windows_core::PCWSTR, ballusers : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { WerAddExcludedApplication(pwzexename.param().abi(), ballusers.into()) }
}
#[inline]
pub unsafe fn WerFreeString<P0>(pwszstr: P0)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "C" fn WerFreeString(pwszstr : windows_core::PCWSTR));
    unsafe { WerFreeString(pwszstr.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WerGetFlags(hprocess: super::winnt::HANDLE) -> windows_core::Result<u32> {
    windows_core::link!("kernel32.dll" "system" fn WerGetFlags(hprocess : super::winnt::HANDLE, pdwflags : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WerGetFlags(hprocess, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn WerRegisterAdditionalProcess(processid : u32, captureextrainfoforthreadid : u32) -> windows_core::HRESULT);
    unsafe { WerRegisterAdditionalProcess(processid, captureextrainfoforthreadid) }
}
#[inline]
pub unsafe fn WerRegisterAppLocalDump<P0>(localappdatarelativepath: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WerRegisterAppLocalDump(localappdatarelativepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WerRegisterAppLocalDump(localappdatarelativepath.param().abi()) }
}
#[inline]
pub unsafe fn WerRegisterCustomMetadata<P0, P1>(key: P0, value: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WerRegisterCustomMetadata(key : windows_core::PCWSTR, value : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WerRegisterCustomMetadata(key.param().abi(), value.param().abi()) }
}
#[inline]
pub unsafe fn WerRegisterExcludedMemoryBlock(address: *const core::ffi::c_void, size: u32) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn WerRegisterExcludedMemoryBlock(address : *const core::ffi::c_void, size : u32) -> windows_core::HRESULT);
    unsafe { WerRegisterExcludedMemoryBlock(address, size) }
}
#[inline]
pub unsafe fn WerRegisterFile<P0>(pwzfile: P0, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WerRegisterFile(pwzfile : windows_core::PCWSTR, regfiletype : WER_REGISTER_FILE_TYPE, dwflags : u32) -> windows_core::HRESULT);
    unsafe { WerRegisterFile(pwzfile.param().abi(), regfiletype, dwflags) }
}
#[inline]
pub unsafe fn WerRegisterMemoryBlock(pvaddress: *const core::ffi::c_void, dwsize: u32) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn WerRegisterMemoryBlock(pvaddress : *const core::ffi::c_void, dwsize : u32) -> windows_core::HRESULT);
    unsafe { WerRegisterMemoryBlock(pvaddress, dwsize) }
}
#[inline]
pub unsafe fn WerRegisterRuntimeExceptionModule<P0>(pwszoutofprocesscallbackdll: P0, pcontext: *const core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll : windows_core::PCWSTR, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.param().abi(), pcontext) }
}
#[inline]
pub unsafe fn WerRemoveExcludedApplication<P0>(pwzexename: P0, ballusers: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "system" fn WerRemoveExcludedApplication(pwzexename : windows_core::PCWSTR, ballusers : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { WerRemoveExcludedApplication(pwzexename.param().abi(), ballusers.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WerReportAddDump(hreporthandle: HREPORT, hprocess: super::winnt::HANDLE, hthread: Option<super::winnt::HANDLE>, dumptype: WER_DUMP_TYPE, pexceptionparam: Option<*const WER_EXCEPTION_INFORMATION>, pdumpcustomoptions: Option<*const WER_DUMP_CUSTOM_OPTIONS>, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("wer.dll" "system" fn WerReportAddDump(hreporthandle : HREPORT, hprocess : super::winnt::HANDLE, hthread : super::winnt::HANDLE, dumptype : WER_DUMP_TYPE, pexceptionparam : *const WER_EXCEPTION_INFORMATION, pdumpcustomoptions : *const WER_DUMP_CUSTOM_OPTIONS, dwflags : u32) -> windows_core::HRESULT);
    unsafe { WerReportAddDump(hreporthandle, hprocess, hthread.unwrap_or(core::mem::zeroed()) as _, dumptype, pexceptionparam.unwrap_or(core::mem::zeroed()) as _, pdumpcustomoptions.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WerReportAddFile<P1>(hreporthandle: HREPORT, pwzpath: P1, repfiletype: WER_FILE_TYPE, dwfileflags: u32) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "system" fn WerReportAddFile(hreporthandle : HREPORT, pwzpath : windows_core::PCWSTR, repfiletype : WER_FILE_TYPE, dwfileflags : u32) -> windows_core::HRESULT);
    unsafe { WerReportAddFile(hreporthandle, pwzpath.param().abi(), repfiletype, dwfileflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WerReportCloseHandle(hreporthandle: HREPORT) -> windows_core::HRESULT {
    windows_core::link!("wer.dll" "system" fn WerReportCloseHandle(hreporthandle : HREPORT) -> windows_core::HRESULT);
    unsafe { WerReportCloseHandle(hreporthandle) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WerReportCreate<P0>(pwzeventtype: P0, reptype: WER_REPORT_TYPE, preportinformation: Option<*const WER_REPORT_INFORMATION>) -> windows_core::Result<HREPORT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "system" fn WerReportCreate(pwzeventtype : windows_core::PCWSTR, reptype : WER_REPORT_TYPE, preportinformation : *const WER_REPORT_INFORMATION, phreporthandle : *mut HREPORT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WerReportCreate(pwzeventtype.param().abi(), reptype, preportinformation.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WerReportSetParameter<P2, P3>(hreporthandle: HREPORT, dwparamid: u32, pwzname: P2, pwzvalue: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "system" fn WerReportSetParameter(hreporthandle : HREPORT, dwparamid : u32, pwzname : windows_core::PCWSTR, pwzvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WerReportSetParameter(hreporthandle, dwparamid, pwzname.param().abi(), pwzvalue.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WerReportSetUIOption<P2>(hreporthandle: HREPORT, repuitypeid: WER_REPORT_UI, pwzvalue: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "system" fn WerReportSetUIOption(hreporthandle : HREPORT, repuitypeid : WER_REPORT_UI, pwzvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WerReportSetUIOption(hreporthandle, repuitypeid, pwzvalue.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WerReportSubmit(hreporthandle: HREPORT, consent: WER_CONSENT, dwflags: u32, psubmitresult: Option<*mut WER_SUBMIT_RESULT>) -> windows_core::HRESULT {
    windows_core::link!("wer.dll" "system" fn WerReportSubmit(hreporthandle : HREPORT, consent : WER_CONSENT, dwflags : u32, psubmitresult : *mut WER_SUBMIT_RESULT) -> windows_core::HRESULT);
    unsafe { WerReportSubmit(hreporthandle, consent, dwflags, psubmitresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WerSetFlags(dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn WerSetFlags(dwflags : u32) -> windows_core::HRESULT);
    unsafe { WerSetFlags(dwflags) }
}
#[inline]
pub unsafe fn WerStoreClose(hreportstore: Option<HREPORTSTORE>) {
    windows_core::link!("wer.dll" "C" fn WerStoreClose(hreportstore : HREPORTSTORE));
    unsafe { WerStoreClose(hreportstore.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WerStoreGetFirstReportKey(hreportstore: HREPORTSTORE) -> windows_core::Result<windows_core::PCWSTR> {
    windows_core::link!("wer.dll" "C" fn WerStoreGetFirstReportKey(hreportstore : HREPORTSTORE, ppszreportkey : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WerStoreGetFirstReportKey(hreportstore, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WerStoreGetNextReportKey(hreportstore: HREPORTSTORE) -> windows_core::Result<windows_core::PCWSTR> {
    windows_core::link!("wer.dll" "C" fn WerStoreGetNextReportKey(hreportstore : HREPORTSTORE, ppszreportkey : *mut windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WerStoreGetNextReportKey(hreportstore, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WerStoreGetReportCount(hreportstore: HREPORTSTORE) -> windows_core::Result<u32> {
    windows_core::link!("wer.dll" "C" fn WerStoreGetReportCount(hreportstore : HREPORTSTORE, pdwreportcount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WerStoreGetReportCount(hreportstore, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WerStoreGetSizeOnDisk(hreportstore: HREPORTSTORE) -> windows_core::Result<u64> {
    windows_core::link!("wer.dll" "C" fn WerStoreGetSizeOnDisk(hreportstore : HREPORTSTORE, pqwsizeinbytes : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WerStoreGetSizeOnDisk(hreportstore, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES, phreportstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("wer.dll" "C" fn WerStoreOpen(repstoretype : REPORT_STORE_TYPES, phreportstore : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WerStoreOpen(repstoretype, phreportstore as _) }
}
#[inline]
pub unsafe fn WerStorePurge() -> windows_core::HRESULT {
    windows_core::link!("wer.dll" "C" fn WerStorePurge() -> windows_core::HRESULT);
    unsafe { WerStorePurge() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV1<P1>(hreportstore: HREPORTSTORE, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "C" fn WerStoreQueryReportMetadataV1(hreportstore : HREPORTSTORE, pszreportkey : windows_core::PCWSTR, preportmetadata : *mut WER_REPORT_METADATA_V1) -> windows_core::HRESULT);
    unsafe { WerStoreQueryReportMetadataV1(hreportstore, pszreportkey.param().abi(), preportmetadata as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV2<P1>(hreportstore: HREPORTSTORE, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V2) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "C" fn WerStoreQueryReportMetadataV2(hreportstore : HREPORTSTORE, pszreportkey : windows_core::PCWSTR, preportmetadata : *mut WER_REPORT_METADATA_V2) -> windows_core::HRESULT);
    unsafe { WerStoreQueryReportMetadataV2(hreportstore, pszreportkey.param().abi(), preportmetadata as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV3<P1>(hreportstore: HREPORTSTORE, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V3) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "C" fn WerStoreQueryReportMetadataV3(hreportstore : HREPORTSTORE, pszreportkey : windows_core::PCWSTR, preportmetadata : *mut WER_REPORT_METADATA_V3) -> windows_core::HRESULT);
    unsafe { WerStoreQueryReportMetadataV3(hreportstore, pszreportkey.param().abi(), preportmetadata as _) }
}
#[inline]
pub unsafe fn WerStoreUploadReport<P1>(hreportstore: HREPORTSTORE, pszreportkey: P1, dwflags: u32, psubmitresult: Option<*mut WER_SUBMIT_RESULT>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wer.dll" "C" fn WerStoreUploadReport(hreportstore : HREPORTSTORE, pszreportkey : windows_core::PCWSTR, dwflags : u32, psubmitresult : *mut WER_SUBMIT_RESULT) -> windows_core::HRESULT);
    unsafe { WerStoreUploadReport(hreportstore, pszreportkey.param().abi(), dwflags, psubmitresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WerUnregisterAdditionalProcess(processid: u32) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn WerUnregisterAdditionalProcess(processid : u32) -> windows_core::HRESULT);
    unsafe { WerUnregisterAdditionalProcess(processid) }
}
#[inline]
pub unsafe fn WerUnregisterAppLocalDump() -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn WerUnregisterAppLocalDump() -> windows_core::HRESULT);
    unsafe { WerUnregisterAppLocalDump() }
}
#[inline]
pub unsafe fn WerUnregisterCustomMetadata<P0>(key: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WerUnregisterCustomMetadata(key : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WerUnregisterCustomMetadata(key.param().abi()) }
}
#[inline]
pub unsafe fn WerUnregisterExcludedMemoryBlock(address: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn WerUnregisterExcludedMemoryBlock(address : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WerUnregisterExcludedMemoryBlock(address) }
}
#[inline]
pub unsafe fn WerUnregisterFile<P0>(pwzfilepath: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WerUnregisterFile(pwzfilepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WerUnregisterFile(pwzfilepath.param().abi()) }
}
#[inline]
pub unsafe fn WerUnregisterMemoryBlock(pvaddress: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn WerUnregisterMemoryBlock(pvaddress : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WerUnregisterMemoryBlock(pvaddress) }
}
#[inline]
pub unsafe fn WerUnregisterRuntimeExceptionModule<P0>(pwszoutofprocesscallbackdll: P0, pcontext: *const core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll : windows_core::PCWSTR, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.param().abi(), pcontext) }
}
pub const APPCRASH_EVENT: windows_core::PCWSTR = windows_core::w!("APPCRASH");
pub const E_STORE_INVALID: REPORT_STORE_TYPES = 4;
pub const E_STORE_MACHINE_ARCHIVE: REPORT_STORE_TYPES = 2;
pub const E_STORE_MACHINE_QUEUE: REPORT_STORE_TYPES = 3;
pub const E_STORE_USER_ARCHIVE: REPORT_STORE_TYPES = 0;
pub const E_STORE_USER_QUEUE: REPORT_STORE_TYPES = 1;
#[cfg(feature = "winnt")]
pub type HREPORT = super::winnt::HANDLE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREPORTSTORE(pub *mut core::ffi::c_void);
impl HREPORTSTORE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HREPORTSTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PACKAGED_APPCRASH_EVENT: windows_core::PCWSTR = windows_core::w!("MoAppCrash");
#[cfg(feature = "winnt")]
pub type PCWER_DUMP_CUSTOM_OPTIONS_V3 = *const WER_DUMP_CUSTOM_OPTIONS_V3;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PCWER_REPORT_INFORMATION_V4 = *const WER_REPORT_INFORMATION_V4;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PCWER_REPORT_INFORMATION_V5 = *const WER_REPORT_INFORMATION_V5;
#[cfg(feature = "winnt")]
pub type PFN_WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbiscustomdebugger: *mut windows_core::BOOL, pwszdebuggerlaunch: windows_core::PWSTR, pchdebuggerlaunch: *mut u32, pbisdebuggerautolaunch: *mut windows_core::BOOL) -> windows_core::HRESULT>;
#[cfg(feature = "winnt")]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbownershipclaimed: *mut windows_core::BOOL, pwszeventname: windows_core::PWSTR, pchsize: *mut u32, pdwsignaturecount: *mut u32) -> windows_core::HRESULT>;
#[cfg(feature = "winnt")]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE = Option<unsafe extern "system" fn(pcontext: *const core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, dwindex: u32, pwszname: windows_core::PWSTR, pchname: *mut u32, pwszvalue: windows_core::PWSTR, pchvalue: *mut u32) -> windows_core::HRESULT>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHREPORTSTORE(pub *mut *mut core::ffi::c_void);
impl PHREPORTSTORE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHREPORTSTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PWER_DUMP_CUSTOM_OPTIONS = *mut WER_DUMP_CUSTOM_OPTIONS;
pub type PWER_DUMP_CUSTOM_OPTIONS_V2 = *mut WER_DUMP_CUSTOM_OPTIONS_V2;
#[cfg(feature = "winnt")]
pub type PWER_DUMP_CUSTOM_OPTIONS_V3 = *mut WER_DUMP_CUSTOM_OPTIONS_V3;
#[cfg(feature = "winnt")]
pub type PWER_EXCEPTION_INFORMATION = *mut WER_EXCEPTION_INFORMATION;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PWER_REPORT_INFORMATION = *mut WER_REPORT_INFORMATION;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PWER_REPORT_INFORMATION_V3 = *mut WER_REPORT_INFORMATION_V3;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PWER_REPORT_INFORMATION_V4 = *mut WER_REPORT_INFORMATION_V4;
#[cfg(all(feature = "windef", feature = "winnt"))]
pub type PWER_REPORT_INFORMATION_V5 = *mut WER_REPORT_INFORMATION_V5;
#[cfg(feature = "minwindef")]
pub type PWER_REPORT_METADATA_V1 = *mut WER_REPORT_METADATA_V1;
#[cfg(feature = "minwindef")]
pub type PWER_REPORT_METADATA_V2 = *mut WER_REPORT_METADATA_V2;
#[cfg(feature = "minwindef")]
pub type PWER_REPORT_METADATA_V3 = *mut WER_REPORT_METADATA_V3;
pub type PWER_REPORT_PARAMETER = WER_REPORT_PARAMETER;
pub type PWER_REPORT_SIGNATURE = *mut WER_REPORT_SIGNATURE;
#[cfg(feature = "winnt")]
pub type PWER_RUNTIME_EXCEPTION_INFORMATION = *mut WER_RUNTIME_EXCEPTION_INFORMATION;
pub type PWER_SUBMIT_RESULT = *mut WER_SUBMIT_RESULT;
pub type REPORT_STORE_TYPES = i32;
pub type WER_CONSENT = i32;
pub const WER_DUMP_AUXILIARY: u32 = 2;
pub const WER_DUMP_AUX_PROMOTE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_DUMP_CUSTOM_OPTIONS {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: windows_core::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
}
impl Default for WER_DUMP_CUSTOM_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_DUMP_CUSTOM_OPTIONS_V2 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: windows_core::BOOL,
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
impl Default for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_DUMP_CUSTOM_OPTIONS_V3 {
    pub dwSize: u32,
    pub dwMask: u32,
    pub dwDumpFlags: u32,
    pub bOnlyThisThread: windows_core::BOOL,
    pub dwExceptionThreadFlags: u32,
    pub dwOtherThreadFlags: u32,
    pub dwExceptionThreadExFlags: u32,
    pub dwOtherThreadExFlags: u32,
    pub dwPreferredModuleFlags: u32,
    pub dwOtherModuleFlags: u32,
    pub wzPreferredModuleList: [u16; 256],
    pub dwPreferredModuleResetFlags: u32,
    pub dwOtherModuleResetFlags: u32,
    pub pvDumpKey: *mut core::ffi::c_void,
    pub hSnapshot: super::winnt::HANDLE,
    pub dwThreadID: u32,
}
#[cfg(feature = "winnt")]
impl Default for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WER_DUMP_MASK_DUMPTYPE: u32 = 1;
pub const WER_DUMP_MASK_ONLY_THISTHREAD: u32 = 2;
pub const WER_DUMP_MASK_OTHERTHREADFLAGS: u32 = 16;
pub const WER_DUMP_MASK_OTHERTHREADFLAGS_EX: u32 = 32;
pub const WER_DUMP_MASK_OTHER_MODULESFLAGS: u32 = 128;
pub const WER_DUMP_MASK_PREFERRED_MODULESFLAGS: u32 = 64;
pub const WER_DUMP_MASK_PREFERRED_MODULE_LIST: u32 = 256;
pub const WER_DUMP_MASK_START: u32 = 1;
pub const WER_DUMP_MASK_THREADFLAGS: u32 = 4;
pub const WER_DUMP_MASK_THREADFLAGS_EX: u32 = 8;
pub const WER_DUMP_NOHEAP_ONQUEUE: u32 = 1;
pub type WER_DUMP_TYPE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WER_EXCEPTION_INFORMATION {
    pub pExceptionPointers: super::winnt::PEXCEPTION_POINTERS,
    pub bClientPointers: windows_core::BOOL,
}
pub const WER_E_CABBING_FAILURE: i32 = -2147024865;
pub const WER_E_INSUFFICIENT_BUFFER: i32 = -2147024774;
pub const WER_E_INVALID_STATE: i32 = -2147019873;
pub const WER_E_LENGTH_EXCEEDED: i32 = -2147023613;
pub const WER_E_MISSING_DUMP: i32 = -2147024323;
pub const WER_E_NOT_FOUND: i32 = -2147023728;
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: u32 = 16;
pub const WER_FAULT_REPORTING_CRITICAL: u32 = 512;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: u32 = 128;
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: u32 = 256;
pub const WER_FAULT_REPORTING_DURABLE: u32 = 1024;
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: u32 = 4;
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: u32 = 1;
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: u32 = 64;
pub const WER_FAULT_REPORTING_FLAG_QUEUE: u32 = 2;
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: u32 = 8;
pub const WER_FAULT_REPORTING_NO_UI: u32 = 32;
pub const WER_FILE_ANONYMOUS_DATA: u32 = 2;
pub const WER_FILE_COMPRESSED: u32 = 4;
pub const WER_FILE_DELETE_WHEN_DONE: u32 = 1;
pub type WER_FILE_TYPE = i32;
pub const WER_MAX_APPLICATION_NAME_LENGTH: u32 = 128;
pub const WER_MAX_BUCKET_ID_STRING_LENGTH: u32 = 260;
pub const WER_MAX_DESCRIPTION_LENGTH: u32 = 512;
pub const WER_MAX_EVENT_NAME_LENGTH: u32 = 64;
pub const WER_MAX_FRIENDLY_EVENT_NAME_LENGTH: u32 = 128;
pub const WER_MAX_LOCAL_DUMP_SUBPATH_LENGTH: u32 = 64;
pub const WER_MAX_MEM_BLOCK_SIZE: u32 = 65536;
pub const WER_MAX_PARAM_COUNT: u32 = 10;
pub const WER_MAX_PARAM_LENGTH: u32 = 260;
pub const WER_MAX_PREFERRED_MODULES: u32 = 128;
pub const WER_MAX_PREFERRED_MODULES_BUFFER: u32 = 256;
pub const WER_MAX_REGISTERED_DUMPCOLLECTION: u32 = 4;
pub const WER_MAX_REGISTERED_ENTRIES: u32 = 512;
pub const WER_MAX_REGISTERED_METADATA: u32 = 8;
pub const WER_MAX_REGISTERED_RUNTIME_EXCEPTION_MODULES: u32 = 16;
pub const WER_MAX_SIGNATURE_NAME_LENGTH: u32 = 128;
pub const WER_MAX_TOTAL_PARAM_LENGTH: u32 = 1720;
pub const WER_METADATA_KEY_MAX_LENGTH: u32 = 64;
pub const WER_METADATA_VALUE_MAX_LENGTH: u32 = 128;
pub const WER_P0: u32 = 0;
pub const WER_P1: u32 = 1;
pub const WER_P2: u32 = 2;
pub const WER_P3: u32 = 3;
pub const WER_P4: u32 = 4;
pub const WER_P5: u32 = 5;
pub const WER_P6: u32 = 6;
pub const WER_P7: u32 = 7;
pub const WER_P8: u32 = 8;
pub const WER_P9: u32 = 9;
pub type WER_REGISTER_FILE_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_REPORT_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::winnt::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::windef::HWND,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for WER_REPORT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_REPORT_INFORMATION_V3 {
    pub dwSize: u32,
    pub hProcess: super::winnt::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::windef::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for WER_REPORT_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_REPORT_INFORMATION_V4 {
    pub dwSize: u32,
    pub hProcess: super::winnt::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::windef::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: super::winnt::HANDLE,
    pub hDeleteFilesImpersonationToken: super::winnt::HANDLE,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for WER_REPORT_INFORMATION_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_REPORT_INFORMATION_V5 {
    pub dwSize: u32,
    pub hProcess: super::winnt::HANDLE,
    pub wzConsentKey: [u16; 64],
    pub wzFriendlyEventName: [u16; 128],
    pub wzApplicationName: [u16; 128],
    pub wzApplicationPath: [u16; 260],
    pub wzDescription: [u16; 512],
    pub hwndParent: super::windef::HWND,
    pub wzNamespacePartner: [u16; 64],
    pub wzNamespaceGroup: [u16; 64],
    pub rgbApplicationIdentity: [u8; 16],
    pub hSnapshot: super::winnt::HANDLE,
    pub hDeleteFilesImpersonationToken: super::winnt::HANDLE,
    pub submitResultMax: WER_SUBMIT_RESULT,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for WER_REPORT_INFORMATION_V5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WER_REPORT_METADATA_V1 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: windows_core::GUID,
    pub ReportId: windows_core::GUID,
    pub CreationTime: super::minwindef::FILETIME,
    pub SizeInBytes: u64,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_REPORT_METADATA_V2 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: windows_core::GUID,
    pub ReportId: windows_core::GUID,
    pub CreationTime: super::minwindef::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: windows_core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: *mut u16,
}
#[cfg(feature = "minwindef")]
impl Default for WER_REPORT_METADATA_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_REPORT_METADATA_V3 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: windows_core::GUID,
    pub ReportId: windows_core::GUID,
    pub CreationTime: super::minwindef::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: windows_core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: *mut u16,
    pub FriendlyEventName: [u16; 128],
    pub ApplicationName: [u16; 128],
    pub ApplicationPath: [u16; 260],
    pub Description: [u16; 512],
    pub BucketIdString: [u16; 260],
    pub LegacyBucketId: u64,
}
#[cfg(feature = "minwindef")]
impl Default for WER_REPORT_METADATA_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_REPORT_PARAMETER {
    pub Name: [u16; 129],
    pub Value: [u16; 260],
}
impl Default for WER_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WER_REPORT_SIGNATURE {
    pub EventName: [u16; 65],
    pub Parameters: [WER_REPORT_PARAMETER; 10],
}
impl Default for WER_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WER_REPORT_TYPE = i32;
pub type WER_REPORT_UI = i32;
pub const WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH: windows_core::PCSTR = windows_core::s!("OutOfProcessExceptionEventDebuggerLaunchCallback");
pub const WER_RUNTIME_EXCEPTION_EVENT_FUNCTION: windows_core::PCSTR = windows_core::s!("OutOfProcessExceptionEventCallback");
pub const WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE_FUNCTION: windows_core::PCSTR = windows_core::s!("OutOfProcessExceptionEventSignatureCallback");
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WER_RUNTIME_EXCEPTION_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::winnt::HANDLE,
    pub hThread: super::winnt::HANDLE,
    pub exceptionRecord: super::winnt::EXCEPTION_RECORD,
    pub context: super::winnt::CONTEXT,
    pub pwszReportId: windows_core::PCWSTR,
    pub bIsFatal: windows_core::BOOL,
    pub dwReserved: u32,
}
#[cfg(feature = "winnt")]
impl Default for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WER_SUBMIT_ADD_REGISTERED_DATA: u32 = 16;
pub const WER_SUBMIT_ARCHIVE_PARAMETERS_ONLY: u32 = 4096;
pub const WER_SUBMIT_BYPASS_DATA_THROTTLING: u32 = 2048;
pub const WER_SUBMIT_BYPASS_NETWORK_COST_THROTTLING: u32 = 32768;
pub const WER_SUBMIT_BYPASS_POWER_THROTTLING: u32 = 16384;
pub const WER_SUBMIT_DISCARD_IF_QUEUED: u32 = 128;
pub const WER_SUBMIT_HONOR_RECOVERY: u32 = 1;
pub const WER_SUBMIT_HONOR_RESTART: u32 = 2;
pub const WER_SUBMIT_NO_ARCHIVE: u32 = 256;
pub const WER_SUBMIT_NO_CLOSE_UI: u32 = 64;
pub const WER_SUBMIT_NO_QUEUE: u32 = 128;
pub const WER_SUBMIT_OUTOFPROCESS: u32 = 32;
pub const WER_SUBMIT_OUTOFPROCESS_ASYNC: u32 = 1024;
pub const WER_SUBMIT_QUEUE: u32 = 4;
pub const WER_SUBMIT_REPORT_MACHINE_ID: u32 = 8192;
pub type WER_SUBMIT_RESULT = i32;
pub const WER_SUBMIT_SHOW_DEBUG: u32 = 8;
pub const WER_SUBMIT_START_MINIMIZED: u32 = 512;
pub const WerConsentAlwaysPrompt: WER_CONSENT = 4;
pub const WerConsentApproved: WER_CONSENT = 2;
pub const WerConsentDenied: WER_CONSENT = 3;
pub const WerConsentMax: WER_CONSENT = 5;
pub const WerConsentNotAsked: WER_CONSENT = 1;
pub const WerCustomAction: WER_SUBMIT_RESULT = 9;
pub const WerDisabled: WER_SUBMIT_RESULT = 5;
pub const WerDisabledQueue: WER_SUBMIT_RESULT = 7;
pub const WerDumpTypeHeapDump: WER_DUMP_TYPE = 3;
pub const WerDumpTypeMax: WER_DUMP_TYPE = 5;
pub const WerDumpTypeMicroDump: WER_DUMP_TYPE = 1;
pub const WerDumpTypeMiniDump: WER_DUMP_TYPE = 2;
pub const WerDumpTypeNone: WER_DUMP_TYPE = 0;
pub const WerDumpTypeTriageDump: WER_DUMP_TYPE = 4;
pub const WerFileTypeAuxiliaryDump: WER_FILE_TYPE = 8;
pub const WerFileTypeAuxiliaryHeapDump: WER_FILE_TYPE = 10;
pub const WerFileTypeCustomDump: WER_FILE_TYPE = 7;
pub const WerFileTypeEtlTrace: WER_FILE_TYPE = 9;
pub const WerFileTypeHeapdump: WER_FILE_TYPE = 3;
pub const WerFileTypeMax: WER_FILE_TYPE = 11;
pub const WerFileTypeMicrodump: WER_FILE_TYPE = 1;
pub const WerFileTypeMinidump: WER_FILE_TYPE = 2;
pub const WerFileTypeOther: WER_FILE_TYPE = 5;
pub const WerFileTypeTriagedump: WER_FILE_TYPE = 6;
pub const WerFileTypeUserDocument: WER_FILE_TYPE = 4;
pub const WerRegFileTypeMax: WER_REGISTER_FILE_TYPE = 3;
pub const WerRegFileTypeOther: WER_REGISTER_FILE_TYPE = 2;
pub const WerRegFileTypeUserDocument: WER_REGISTER_FILE_TYPE = 1;
pub const WerReportApplicationCrash: WER_REPORT_TYPE = 2;
pub const WerReportApplicationHang: WER_REPORT_TYPE = 3;
pub const WerReportAsync: WER_SUBMIT_RESULT = 8;
pub const WerReportCancelled: WER_SUBMIT_RESULT = 6;
pub const WerReportCritical: WER_REPORT_TYPE = 1;
pub const WerReportDebug: WER_SUBMIT_RESULT = 3;
pub const WerReportFailed: WER_SUBMIT_RESULT = 4;
pub const WerReportInvalid: WER_REPORT_TYPE = 5;
pub const WerReportKernel: WER_REPORT_TYPE = 4;
pub const WerReportNonCritical: WER_REPORT_TYPE = 0;
pub const WerReportQueued: WER_SUBMIT_RESULT = 1;
pub const WerReportUploaded: WER_SUBMIT_RESULT = 2;
pub const WerReportUploadedCab: WER_SUBMIT_RESULT = 11;
pub const WerStorageLocationNotFound: WER_SUBMIT_RESULT = 12;
pub const WerSubmitResultMax: WER_SUBMIT_RESULT = 13;
pub const WerThrottled: WER_SUBMIT_RESULT = 10;
pub const WerUIAdditionalDataDlgHeader: WER_REPORT_UI = 1;
pub const WerUICloseDlgBody: WER_REPORT_UI = 9;
pub const WerUICloseDlgButtonText: WER_REPORT_UI = 10;
pub const WerUICloseDlgHeader: WER_REPORT_UI = 8;
pub const WerUICloseText: WER_REPORT_UI = 7;
pub const WerUIConsentDlgBody: WER_REPORT_UI = 4;
pub const WerUIConsentDlgHeader: WER_REPORT_UI = 3;
pub const WerUIIconFilePath: WER_REPORT_UI = 2;
pub const WerUIMax: WER_REPORT_UI = 11;
pub const WerUIOfflineSolutionCheckText: WER_REPORT_UI = 6;
pub const WerUIOnlineSolutionCheckText: WER_REPORT_UI = 5;
