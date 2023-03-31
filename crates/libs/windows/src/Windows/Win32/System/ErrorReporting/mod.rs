#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddERExcludedApplicationA<P0>(szapplication: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "faultrep.dll""system" fn AddERExcludedApplicationA ( szapplication : ::windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    AddERExcludedApplicationA(szapplication.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddERExcludedApplicationW<P0>(wszapplication: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "faultrep.dll""system" fn AddERExcludedApplicationW ( wszapplication : ::windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    AddERExcludedApplicationW(wszapplication.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn ReportFault(pep: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, dwopt: u32) -> EFaultRepRetVal {
    ::windows_targets::link ! ( "faultrep.dll""system" fn ReportFault ( pep : *const super::Diagnostics::Debug:: EXCEPTION_POINTERS , dwopt : u32 ) -> EFaultRepRetVal );
    ReportFault(pep, dwopt)
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerAddExcludedApplication<P0, P1>(pwzexename: P0, ballusers: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerAddExcludedApplication ( pwzexename : ::windows::core::PCWSTR , ballusers : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    WerAddExcludedApplication(pwzexename.into_param().abi(), ballusers.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerFreeString<P0>(pwszstr: P0)
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerFreeString ( pwszstr : ::windows::core::PCWSTR ) -> ( ) );
    WerFreeString(pwszstr.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerGetFlags<P0>(hprocess: P0) -> ::windows::core::Result<WER_FAULT_REPORTING>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerGetFlags ( hprocess : super::super::Foundation:: HANDLE , pdwflags : *mut WER_FAULT_REPORTING ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<WER_FAULT_REPORTING>();
    WerGetFlags(hprocess.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerRegisterAdditionalProcess(processid: u32, captureextrainfoforthreadid: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerRegisterAdditionalProcess ( processid : u32 , captureextrainfoforthreadid : u32 ) -> ::windows::core::HRESULT );
    WerRegisterAdditionalProcess(processid, captureextrainfoforthreadid).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerRegisterAppLocalDump<P0>(localappdatarelativepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerRegisterAppLocalDump ( localappdatarelativepath : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerRegisterAppLocalDump(localappdatarelativepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerRegisterCustomMetadata<P0, P1>(key: P0, value: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerRegisterCustomMetadata ( key : ::windows::core::PCWSTR , value : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerRegisterCustomMetadata(key.into_param().abi(), value.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerRegisterExcludedMemoryBlock(address: *const ::core::ffi::c_void, size: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerRegisterExcludedMemoryBlock ( address : *const ::core::ffi::c_void , size : u32 ) -> ::windows::core::HRESULT );
    WerRegisterExcludedMemoryBlock(address, size).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerRegisterFile<P0>(pwzfile: P0, regfiletype: WER_REGISTER_FILE_TYPE, dwflags: WER_FILE) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerRegisterFile ( pwzfile : ::windows::core::PCWSTR , regfiletype : WER_REGISTER_FILE_TYPE , dwflags : WER_FILE ) -> ::windows::core::HRESULT );
    WerRegisterFile(pwzfile.into_param().abi(), regfiletype, dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerRegisterMemoryBlock(pvaddress: *const ::core::ffi::c_void, dwsize: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerRegisterMemoryBlock ( pvaddress : *const ::core::ffi::c_void , dwsize : u32 ) -> ::windows::core::HRESULT );
    WerRegisterMemoryBlock(pvaddress, dwsize).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerRegisterRuntimeExceptionModule<P0>(pwszoutofprocesscallbackdll: P0, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerRegisterRuntimeExceptionModule ( pwszoutofprocesscallbackdll : ::windows::core::PCWSTR , pcontext : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    WerRegisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.into_param().abi(), pcontext).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerRemoveExcludedApplication<P0, P1>(pwzexename: P0, ballusers: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerRemoveExcludedApplication ( pwzexename : ::windows::core::PCWSTR , ballusers : super::super::Foundation:: BOOL ) -> ::windows::core::HRESULT );
    WerRemoveExcludedApplication(pwzexename.into_param().abi(), ballusers.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn WerReportAddDump<P0, P1, P2>(hreporthandle: P0, hprocess: P1, hthread: P2, dumptype: WER_DUMP_TYPE, pexceptionparam: ::core::option::Option<*const WER_EXCEPTION_INFORMATION>, pdumpcustomoptions: ::core::option::Option<*const WER_DUMP_CUSTOM_OPTIONS>, dwflags: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORT>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerReportAddDump ( hreporthandle : HREPORT , hprocess : super::super::Foundation:: HANDLE , hthread : super::super::Foundation:: HANDLE , dumptype : WER_DUMP_TYPE , pexceptionparam : *const WER_EXCEPTION_INFORMATION , pdumpcustomoptions : *const WER_DUMP_CUSTOM_OPTIONS , dwflags : u32 ) -> ::windows::core::HRESULT );
    WerReportAddDump(hreporthandle.into_param().abi(), hprocess.into_param().abi(), hthread.into_param().abi(), dumptype, ::core::mem::transmute(pexceptionparam.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdumpcustomoptions.unwrap_or(::std::ptr::null())), dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerReportAddFile<P0, P1>(hreporthandle: P0, pwzpath: P1, repfiletype: WER_FILE_TYPE, dwfileflags: WER_FILE) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORT>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerReportAddFile ( hreporthandle : HREPORT , pwzpath : ::windows::core::PCWSTR , repfiletype : WER_FILE_TYPE , dwfileflags : WER_FILE ) -> ::windows::core::HRESULT );
    WerReportAddFile(hreporthandle.into_param().abi(), pwzpath.into_param().abi(), repfiletype, dwfileflags).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerReportCloseHandle<P0>(hreporthandle: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORT>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerReportCloseHandle ( hreporthandle : HREPORT ) -> ::windows::core::HRESULT );
    WerReportCloseHandle(hreporthandle.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerReportCreate<P0>(pwzeventtype: P0, reptype: WER_REPORT_TYPE, preportinformation: ::core::option::Option<*const WER_REPORT_INFORMATION>) -> ::windows::core::Result<HREPORT>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerReportCreate ( pwzeventtype : ::windows::core::PCWSTR , reptype : WER_REPORT_TYPE , preportinformation : *const WER_REPORT_INFORMATION , phreporthandle : *mut HREPORT ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<HREPORT>();
    WerReportCreate(pwzeventtype.into_param().abi(), reptype, ::core::mem::transmute(preportinformation.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerReportHang<P0, P1>(hwndhungapp: P0, pwzhungapplicationname: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "faultrep.dll""system" fn WerReportHang ( hwndhungapp : super::super::Foundation:: HWND , pwzhungapplicationname : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerReportHang(hwndhungapp.into_param().abi(), pwzhungapplicationname.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerReportSetParameter<P0, P1, P2>(hreporthandle: P0, dwparamid: u32, pwzname: P1, pwzvalue: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORT>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerReportSetParameter ( hreporthandle : HREPORT , dwparamid : u32 , pwzname : ::windows::core::PCWSTR , pwzvalue : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerReportSetParameter(hreporthandle.into_param().abi(), dwparamid, pwzname.into_param().abi(), pwzvalue.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerReportSetUIOption<P0, P1>(hreporthandle: P0, repuitypeid: WER_REPORT_UI, pwzvalue: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORT>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerReportSetUIOption ( hreporthandle : HREPORT , repuitypeid : WER_REPORT_UI , pwzvalue : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerReportSetUIOption(hreporthandle.into_param().abi(), repuitypeid, pwzvalue.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerReportSubmit<P0>(hreporthandle: P0, consent: WER_CONSENT, dwflags: WER_SUBMIT_FLAGS, psubmitresult: ::core::option::Option<*mut WER_SUBMIT_RESULT>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORT>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerReportSubmit ( hreporthandle : HREPORT , consent : WER_CONSENT , dwflags : WER_SUBMIT_FLAGS , psubmitresult : *mut WER_SUBMIT_RESULT ) -> ::windows::core::HRESULT );
    WerReportSubmit(hreporthandle.into_param().abi(), consent, dwflags, ::core::mem::transmute(psubmitresult.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerSetFlags(dwflags: WER_FAULT_REPORTING) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerSetFlags ( dwflags : WER_FAULT_REPORTING ) -> ::windows::core::HRESULT );
    WerSetFlags(dwflags).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerStoreClose<P0>(hreportstore: P0)
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreClose ( hreportstore : HREPORTSTORE ) -> ( ) );
    WerStoreClose(hreportstore.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerStoreGetFirstReportKey<P0>(hreportstore: P0, ppszreportkey: ::core::option::Option<*mut ::windows::core::PCWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreGetFirstReportKey ( hreportstore : HREPORTSTORE , ppszreportkey : *mut ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerStoreGetFirstReportKey(hreportstore.into_param().abi(), ::core::mem::transmute(ppszreportkey.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerStoreGetNextReportKey<P0>(hreportstore: P0, ppszreportkey: ::core::option::Option<*mut ::windows::core::PCWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreGetNextReportKey ( hreportstore : HREPORTSTORE , ppszreportkey : *mut ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerStoreGetNextReportKey(hreportstore.into_param().abi(), ::core::mem::transmute(ppszreportkey.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerStoreGetReportCount<P0>(hreportstore: P0) -> ::windows::core::Result<u32>
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreGetReportCount ( hreportstore : HREPORTSTORE , pdwreportcount : *mut u32 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u32>();
    WerStoreGetReportCount(hreportstore.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerStoreGetSizeOnDisk<P0>(hreportstore: P0) -> ::windows::core::Result<u64>
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreGetSizeOnDisk ( hreportstore : HREPORTSTORE , pqwsizeinbytes : *mut u64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    WerStoreGetSizeOnDisk(hreportstore.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerStoreOpen(repstoretype: REPORT_STORE_TYPES) -> ::windows::core::Result<HREPORTSTORE> {
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreOpen ( repstoretype : REPORT_STORE_TYPES , phreportstore : *mut HREPORTSTORE ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<HREPORTSTORE>();
    WerStoreOpen(repstoretype, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerStorePurge() -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "wer.dll""system" fn WerStorePurge ( ) -> ::windows::core::HRESULT );
    WerStorePurge().ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV1<P0, P1>(hreportstore: P0, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreQueryReportMetadataV1 ( hreportstore : HREPORTSTORE , pszreportkey : ::windows::core::PCWSTR , preportmetadata : *mut WER_REPORT_METADATA_V1 ) -> ::windows::core::HRESULT );
    WerStoreQueryReportMetadataV1(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), preportmetadata).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV2<P0, P1>(hreportstore: P0, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreQueryReportMetadataV2 ( hreportstore : HREPORTSTORE , pszreportkey : ::windows::core::PCWSTR , preportmetadata : *mut WER_REPORT_METADATA_V2 ) -> ::windows::core::HRESULT );
    WerStoreQueryReportMetadataV2(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), preportmetadata).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WerStoreQueryReportMetadataV3<P0, P1>(hreportstore: P0, pszreportkey: P1, preportmetadata: *mut WER_REPORT_METADATA_V3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreQueryReportMetadataV3 ( hreportstore : HREPORTSTORE , pszreportkey : ::windows::core::PCWSTR , preportmetadata : *mut WER_REPORT_METADATA_V3 ) -> ::windows::core::HRESULT );
    WerStoreQueryReportMetadataV3(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), preportmetadata).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerStoreUploadReport<P0, P1>(hreportstore: P0, pszreportkey: P1, dwflags: u32, psubmitresult: ::core::option::Option<*mut WER_SUBMIT_RESULT>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HREPORTSTORE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wer.dll""system" fn WerStoreUploadReport ( hreportstore : HREPORTSTORE , pszreportkey : ::windows::core::PCWSTR , dwflags : u32 , psubmitresult : *mut WER_SUBMIT_RESULT ) -> ::windows::core::HRESULT );
    WerStoreUploadReport(hreportstore.into_param().abi(), pszreportkey.into_param().abi(), dwflags, ::core::mem::transmute(psubmitresult.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerUnregisterAdditionalProcess(processid: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerUnregisterAdditionalProcess ( processid : u32 ) -> ::windows::core::HRESULT );
    WerUnregisterAdditionalProcess(processid).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerUnregisterAppLocalDump() -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerUnregisterAppLocalDump ( ) -> ::windows::core::HRESULT );
    WerUnregisterAppLocalDump().ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerUnregisterCustomMetadata<P0>(key: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerUnregisterCustomMetadata ( key : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerUnregisterCustomMetadata(key.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerUnregisterExcludedMemoryBlock(address: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerUnregisterExcludedMemoryBlock ( address : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    WerUnregisterExcludedMemoryBlock(address).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerUnregisterFile<P0>(pwzfilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerUnregisterFile ( pwzfilepath : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    WerUnregisterFile(pwzfilepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerUnregisterMemoryBlock(pvaddress: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerUnregisterMemoryBlock ( pvaddress : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    WerUnregisterMemoryBlock(pvaddress).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[inline]
pub unsafe fn WerUnregisterRuntimeExceptionModule<P0>(pwszoutofprocesscallbackdll: P0, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn WerUnregisterRuntimeExceptionModule ( pwszoutofprocesscallbackdll : ::windows::core::PCWSTR , pcontext : *const ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    WerUnregisterRuntimeExceptionModule(pwszoutofprocesscallbackdll.into_param().abi(), pcontext).ok()
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const APPCRASH_EVENT: ::windows::core::PCWSTR = ::windows::core::w!("APPCRASH");
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const PACKAGED_APPCRASH_EVENT: ::windows::core::PCWSTR = ::windows::core::w!("MoAppCrash");
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_DUMP_AUXILIARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_DUMP_MASK_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_DUMP_NOHEAP_ONQUEUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_CRITICAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_CRASH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_DISABLE_SNAPSHOT_HANG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_DURABLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_FLAG_NO_HEAP_ON_QUEUE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_NO_UI: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FILE_COMPRESSED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_APPLICATION_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_BUCKET_ID_STRING_LENGTH: u32 = 260u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_DESCRIPTION_LENGTH: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_EVENT_NAME_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_FRIENDLY_EVENT_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_LOCAL_DUMP_SUBPATH_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_PARAM_COUNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_PARAM_LENGTH: u32 = 260u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_PREFERRED_MODULES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_PREFERRED_MODULES_BUFFER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_REGISTERED_DUMPCOLLECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_REGISTERED_ENTRIES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_REGISTERED_METADATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_REGISTERED_RUNTIME_EXCEPTION_MODULES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_SIGNATURE_NAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_MAX_TOTAL_PARAM_LENGTH: u32 = 1720u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_METADATA_KEY_MAX_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_METADATA_VALUE_MAX_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P6: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_P9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH: ::windows::core::PCSTR = ::windows::core::s!("OutOfProcessExceptionEventDebuggerLaunchCallback");
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_RUNTIME_EXCEPTION_EVENT_FUNCTION: ::windows::core::PCSTR = ::windows::core::s!("OutOfProcessExceptionEventCallback");
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE_FUNCTION: ::windows::core::PCSTR = ::windows::core::s!("OutOfProcessExceptionEventSignatureCallback");
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_BYPASS_NETWORK_COST_THROTTLING: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_BYPASS_POWER_THROTTLING: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EFaultRepRetVal(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvOk: EFaultRepRetVal = EFaultRepRetVal(0i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvOkManifest: EFaultRepRetVal = EFaultRepRetVal(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvOkQueued: EFaultRepRetVal = EFaultRepRetVal(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvErr: EFaultRepRetVal = EFaultRepRetVal(3i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvErrNoDW: EFaultRepRetVal = EFaultRepRetVal(4i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvErrTimeout: EFaultRepRetVal = EFaultRepRetVal(5i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvLaunchDebugger: EFaultRepRetVal = EFaultRepRetVal(6i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvOkHeadless: EFaultRepRetVal = EFaultRepRetVal(7i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvErrAnotherInstance: EFaultRepRetVal = EFaultRepRetVal(8i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvErrNoMemory: EFaultRepRetVal = EFaultRepRetVal(9i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const frrvErrDoubleFault: EFaultRepRetVal = EFaultRepRetVal(10i32);
impl ::core::marker::Copy for EFaultRepRetVal {}
impl ::core::clone::Clone for EFaultRepRetVal {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EFaultRepRetVal {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EFaultRepRetVal {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EFaultRepRetVal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EFaultRepRetVal").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REPORT_STORE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const E_STORE_USER_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(0i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const E_STORE_USER_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const E_STORE_MACHINE_ARCHIVE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const E_STORE_MACHINE_QUEUE: REPORT_STORE_TYPES = REPORT_STORE_TYPES(3i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const E_STORE_INVALID: REPORT_STORE_TYPES = REPORT_STORE_TYPES(4i32);
impl ::core::marker::Copy for REPORT_STORE_TYPES {}
impl ::core::clone::Clone for REPORT_STORE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPORT_STORE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REPORT_STORE_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REPORT_STORE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPORT_STORE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_CONSENT(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerConsentNotAsked: WER_CONSENT = WER_CONSENT(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerConsentApproved: WER_CONSENT = WER_CONSENT(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerConsentDenied: WER_CONSENT = WER_CONSENT(3i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerConsentAlwaysPrompt: WER_CONSENT = WER_CONSENT(4i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerConsentMax: WER_CONSENT = WER_CONSENT(5i32);
impl ::core::marker::Copy for WER_CONSENT {}
impl ::core::clone::Clone for WER_CONSENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_CONSENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_CONSENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_CONSENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_CONSENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_DUMP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerDumpTypeNone: WER_DUMP_TYPE = WER_DUMP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerDumpTypeMicroDump: WER_DUMP_TYPE = WER_DUMP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerDumpTypeMiniDump: WER_DUMP_TYPE = WER_DUMP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerDumpTypeHeapDump: WER_DUMP_TYPE = WER_DUMP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerDumpTypeTriageDump: WER_DUMP_TYPE = WER_DUMP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerDumpTypeMax: WER_DUMP_TYPE = WER_DUMP_TYPE(5i32);
impl ::core::marker::Copy for WER_DUMP_TYPE {}
impl ::core::clone::Clone for WER_DUMP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_DUMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_DUMP_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_DUMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_DUMP_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_FAULT_REPORTING(pub u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_FLAG_DISABLE_THREAD_SUSPENSION: WER_FAULT_REPORTING = WER_FAULT_REPORTING(4u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_FLAG_NOHEAP: WER_FAULT_REPORTING = WER_FAULT_REPORTING(1u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_FLAG_QUEUE: WER_FAULT_REPORTING = WER_FAULT_REPORTING(2u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_FLAG_QUEUE_UPLOAD: WER_FAULT_REPORTING = WER_FAULT_REPORTING(8u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FAULT_REPORTING_ALWAYS_SHOW_UI: WER_FAULT_REPORTING = WER_FAULT_REPORTING(16u32);
impl ::core::marker::Copy for WER_FAULT_REPORTING {}
impl ::core::clone::Clone for WER_FAULT_REPORTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_FAULT_REPORTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_FAULT_REPORTING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_FAULT_REPORTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FAULT_REPORTING").field(&self.0).finish()
    }
}
impl WER_FAULT_REPORTING {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_FAULT_REPORTING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_FAULT_REPORTING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_FAULT_REPORTING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_FAULT_REPORTING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_FILE(pub u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FILE_ANONYMOUS_DATA: WER_FILE = WER_FILE(2u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_FILE_DELETE_WHEN_DONE: WER_FILE = WER_FILE(1u32);
impl ::core::marker::Copy for WER_FILE {}
impl ::core::clone::Clone for WER_FILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_FILE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_FILE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_FILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FILE").field(&self.0).finish()
    }
}
impl WER_FILE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WER_FILE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_FILE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_FILE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_FILE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_FILE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_FILE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeMicrodump: WER_FILE_TYPE = WER_FILE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeMinidump: WER_FILE_TYPE = WER_FILE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeHeapdump: WER_FILE_TYPE = WER_FILE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeUserDocument: WER_FILE_TYPE = WER_FILE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeOther: WER_FILE_TYPE = WER_FILE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeTriagedump: WER_FILE_TYPE = WER_FILE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeCustomDump: WER_FILE_TYPE = WER_FILE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeAuxiliaryDump: WER_FILE_TYPE = WER_FILE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeEtlTrace: WER_FILE_TYPE = WER_FILE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerFileTypeMax: WER_FILE_TYPE = WER_FILE_TYPE(10i32);
impl ::core::marker::Copy for WER_FILE_TYPE {}
impl ::core::clone::Clone for WER_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_FILE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_FILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_REGISTER_FILE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerRegFileTypeUserDocument: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerRegFileTypeOther: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerRegFileTypeMax: WER_REGISTER_FILE_TYPE = WER_REGISTER_FILE_TYPE(3i32);
impl ::core::marker::Copy for WER_REGISTER_FILE_TYPE {}
impl ::core::clone::Clone for WER_REGISTER_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_REGISTER_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_REGISTER_FILE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_REGISTER_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REGISTER_FILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_REPORT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportNonCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportCritical: WER_REPORT_TYPE = WER_REPORT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportApplicationCrash: WER_REPORT_TYPE = WER_REPORT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportApplicationHang: WER_REPORT_TYPE = WER_REPORT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportKernel: WER_REPORT_TYPE = WER_REPORT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportInvalid: WER_REPORT_TYPE = WER_REPORT_TYPE(5i32);
impl ::core::marker::Copy for WER_REPORT_TYPE {}
impl ::core::clone::Clone for WER_REPORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_REPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_REPORT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_REPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REPORT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_REPORT_UI(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUIAdditionalDataDlgHeader: WER_REPORT_UI = WER_REPORT_UI(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUIIconFilePath: WER_REPORT_UI = WER_REPORT_UI(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUIConsentDlgHeader: WER_REPORT_UI = WER_REPORT_UI(3i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUIConsentDlgBody: WER_REPORT_UI = WER_REPORT_UI(4i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUIOnlineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(5i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUIOfflineSolutionCheckText: WER_REPORT_UI = WER_REPORT_UI(6i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUICloseText: WER_REPORT_UI = WER_REPORT_UI(7i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUICloseDlgHeader: WER_REPORT_UI = WER_REPORT_UI(8i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUICloseDlgBody: WER_REPORT_UI = WER_REPORT_UI(9i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUICloseDlgButtonText: WER_REPORT_UI = WER_REPORT_UI(10i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerUIMax: WER_REPORT_UI = WER_REPORT_UI(11i32);
impl ::core::marker::Copy for WER_REPORT_UI {}
impl ::core::clone::Clone for WER_REPORT_UI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_REPORT_UI {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_REPORT_UI {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_REPORT_UI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_REPORT_UI").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_SUBMIT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_ADD_REGISTERED_DATA: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_HONOR_RECOVERY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_HONOR_RESTART: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_NO_ARCHIVE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_NO_CLOSE_UI: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_NO_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_OUTOFPROCESS: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_OUTOFPROCESS_ASYNC: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_QUEUE: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_SHOW_DEBUG: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_START_MINIMIZED: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_BYPASS_DATA_THROTTLING: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_ARCHIVE_PARAMETERS_ONLY: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WER_SUBMIT_REPORT_MACHINE_ID: WER_SUBMIT_FLAGS = WER_SUBMIT_FLAGS(8192u32);
impl ::core::marker::Copy for WER_SUBMIT_FLAGS {}
impl ::core::clone::Clone for WER_SUBMIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_SUBMIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_SUBMIT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_SUBMIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_SUBMIT_FLAGS").field(&self.0).finish()
    }
}
impl WER_SUBMIT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WER_SUBMIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WER_SUBMIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WER_SUBMIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WER_SUBMIT_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportQueued: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(1i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportUploaded: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(2i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportDebug: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(3i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportFailed: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(4i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerDisabled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(5i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportCancelled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(6i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerDisabledQueue: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(7i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportAsync: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(8i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerCustomAction: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(9i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerThrottled: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(10i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerReportUploadedCab: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(11i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerStorageLocationNotFound: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(12i32);
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub const WerSubmitResultMax: WER_SUBMIT_RESULT = WER_SUBMIT_RESULT(13i32);
impl ::core::marker::Copy for WER_SUBMIT_RESULT {}
impl ::core::clone::Clone for WER_SUBMIT_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WER_SUBMIT_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WER_SUBMIT_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WER_SUBMIT_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WER_SUBMIT_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HREPORT(pub isize);
impl HREPORT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HREPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HREPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HREPORT {}
impl ::core::fmt::Debug for HREPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HREPORT").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HREPORT {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HREPORTSTORE(pub isize);
impl HREPORTSTORE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HREPORTSTORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HREPORTSTORE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HREPORTSTORE {}
impl ::core::fmt::Debug for HREPORTSTORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HREPORTSTORE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HREPORTSTORE {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_DUMP_CUSTOM_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.dwDumpFlags == other.dwDumpFlags && self.bOnlyThisThread == other.bOnlyThisThread && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags && self.dwOtherThreadFlags == other.dwOtherThreadFlags && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags && self.dwOtherModuleFlags == other.dwOtherModuleFlags && self.wzPreferredModuleList == other.wzPreferredModuleList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS_V2")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .field("dwPreferredModuleResetFlags", &self.dwPreferredModuleResetFlags)
            .field("dwOtherModuleResetFlags", &self.dwOtherModuleResetFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_DUMP_CUSTOM_OPTIONS_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.dwDumpFlags == other.dwDumpFlags && self.bOnlyThisThread == other.bOnlyThisThread && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags && self.dwOtherThreadFlags == other.dwOtherThreadFlags && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags && self.dwOtherModuleFlags == other.dwOtherModuleFlags && self.wzPreferredModuleList == other.wzPreferredModuleList && self.dwPreferredModuleResetFlags == other.dwPreferredModuleResetFlags && self.dwOtherModuleResetFlags == other.dwOtherModuleResetFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_DUMP_CUSTOM_OPTIONS_V3")
            .field("dwSize", &self.dwSize)
            .field("dwMask", &self.dwMask)
            .field("dwDumpFlags", &self.dwDumpFlags)
            .field("bOnlyThisThread", &self.bOnlyThisThread)
            .field("dwExceptionThreadFlags", &self.dwExceptionThreadFlags)
            .field("dwOtherThreadFlags", &self.dwOtherThreadFlags)
            .field("dwExceptionThreadExFlags", &self.dwExceptionThreadExFlags)
            .field("dwOtherThreadExFlags", &self.dwOtherThreadExFlags)
            .field("dwPreferredModuleFlags", &self.dwPreferredModuleFlags)
            .field("dwOtherModuleFlags", &self.dwOtherModuleFlags)
            .field("wzPreferredModuleList", &self.wzPreferredModuleList)
            .field("dwPreferredModuleResetFlags", &self.dwPreferredModuleResetFlags)
            .field("dwOtherModuleResetFlags", &self.dwOtherModuleResetFlags)
            .field("pvDumpKey", &self.pvDumpKey)
            .field("hSnapshot", &self.hSnapshot)
            .field("dwThreadID", &self.dwThreadID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_DUMP_CUSTOM_OPTIONS_V3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwMask == other.dwMask
            && self.dwDumpFlags == other.dwDumpFlags
            && self.bOnlyThisThread == other.bOnlyThisThread
            && self.dwExceptionThreadFlags == other.dwExceptionThreadFlags
            && self.dwOtherThreadFlags == other.dwOtherThreadFlags
            && self.dwExceptionThreadExFlags == other.dwExceptionThreadExFlags
            && self.dwOtherThreadExFlags == other.dwOtherThreadExFlags
            && self.dwPreferredModuleFlags == other.dwPreferredModuleFlags
            && self.dwOtherModuleFlags == other.dwOtherModuleFlags
            && self.wzPreferredModuleList == other.wzPreferredModuleList
            && self.dwPreferredModuleResetFlags == other.dwPreferredModuleResetFlags
            && self.dwOtherModuleResetFlags == other.dwOtherModuleResetFlags
            && self.pvDumpKey == other.pvDumpKey
            && self.hSnapshot == other.hSnapshot
            && self.dwThreadID == other.dwThreadID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_DUMP_CUSTOM_OPTIONS_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_DUMP_CUSTOM_OPTIONS_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for WER_EXCEPTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_EXCEPTION_INFORMATION").field("pExceptionPointers", &self.pExceptionPointers).field("bClientPointers", &self.bClientPointers).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for WER_EXCEPTION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for WER_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.pExceptionPointers == other.pExceptionPointers && self.bClientPointers == other.bClientPointers
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for WER_EXCEPTION_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for WER_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION").field("dwSize", &self.dwSize).field("hProcess", &self.hProcess).field("wzConsentKey", &self.wzConsentKey).field("wzFriendlyEventName", &self.wzFriendlyEventName).field("wzApplicationName", &self.wzApplicationName).field("wzApplicationPath", &self.wzApplicationPath).field("wzDescription", &self.wzDescription).field("hwndParent", &self.hwndParent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_REPORT_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V3")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_REPORT_INFORMATION_V3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent && self.wzNamespacePartner == other.wzNamespacePartner && self.wzNamespaceGroup == other.wzNamespaceGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_INFORMATION_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V4")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .field("rgbApplicationIdentity", &self.rgbApplicationIdentity)
            .field("hSnapshot", &self.hSnapshot)
            .field("hDeleteFilesImpersonationToken", &self.hDeleteFilesImpersonationToken)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_REPORT_INFORMATION_V4 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent && self.wzNamespacePartner == other.wzNamespacePartner && self.wzNamespaceGroup == other.wzNamespaceGroup && self.rgbApplicationIdentity == other.rgbApplicationIdentity && self.hSnapshot == other.hSnapshot && self.hDeleteFilesImpersonationToken == other.hDeleteFilesImpersonationToken
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_INFORMATION_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_INFORMATION_V5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_INFORMATION_V5")
            .field("dwSize", &self.dwSize)
            .field("hProcess", &self.hProcess)
            .field("wzConsentKey", &self.wzConsentKey)
            .field("wzFriendlyEventName", &self.wzFriendlyEventName)
            .field("wzApplicationName", &self.wzApplicationName)
            .field("wzApplicationPath", &self.wzApplicationPath)
            .field("wzDescription", &self.wzDescription)
            .field("hwndParent", &self.hwndParent)
            .field("wzNamespacePartner", &self.wzNamespacePartner)
            .field("wzNamespaceGroup", &self.wzNamespaceGroup)
            .field("rgbApplicationIdentity", &self.rgbApplicationIdentity)
            .field("hSnapshot", &self.hSnapshot)
            .field("hDeleteFilesImpersonationToken", &self.hDeleteFilesImpersonationToken)
            .field("submitResultMax", &self.submitResultMax)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_REPORT_INFORMATION_V5 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_INFORMATION_V5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hProcess == other.hProcess && self.wzConsentKey == other.wzConsentKey && self.wzFriendlyEventName == other.wzFriendlyEventName && self.wzApplicationName == other.wzApplicationName && self.wzApplicationPath == other.wzApplicationPath && self.wzDescription == other.wzDescription && self.hwndParent == other.hwndParent && self.wzNamespacePartner == other.wzNamespacePartner && self.wzNamespaceGroup == other.wzNamespaceGroup && self.rgbApplicationIdentity == other.rgbApplicationIdentity && self.hSnapshot == other.hSnapshot && self.hDeleteFilesImpersonationToken == other.hDeleteFilesImpersonationToken && self.submitResultMax == other.submitResultMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_INFORMATION_V5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_INFORMATION_V5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_METADATA_V1 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows::core::GUID,
    pub ReportId: ::windows::core::GUID,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_METADATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V1").field("Signature", &self.Signature).field("BucketId", &self.BucketId).field("ReportId", &self.ReportId).field("CreationTime", &self.CreationTime).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_REPORT_METADATA_V1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.BucketId == other.BucketId && self.ReportId == other.ReportId && self.CreationTime == other.CreationTime && self.SizeInBytes == other.SizeInBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_METADATA_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_METADATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_METADATA_V2 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows::core::GUID,
    pub ReportId: ::windows::core::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: ::windows::core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WER_REPORT_METADATA_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WER_REPORT_METADATA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_METADATA_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V2")
            .field("Signature", &self.Signature)
            .field("BucketId", &self.BucketId)
            .field("ReportId", &self.ReportId)
            .field("CreationTime", &self.CreationTime)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("CabId", &self.CabId)
            .field("ReportStatus", &self.ReportStatus)
            .field("ReportIntegratorId", &self.ReportIntegratorId)
            .field("NumberOfFiles", &self.NumberOfFiles)
            .field("SizeOfFileNames", &self.SizeOfFileNames)
            .field("FileNames", &self.FileNames)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_REPORT_METADATA_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.BucketId == other.BucketId && self.ReportId == other.ReportId && self.CreationTime == other.CreationTime && self.SizeInBytes == other.SizeInBytes && self.CabId == other.CabId && self.ReportStatus == other.ReportStatus && self.ReportIntegratorId == other.ReportIntegratorId && self.NumberOfFiles == other.NumberOfFiles && self.SizeOfFileNames == other.SizeOfFileNames && self.FileNames == other.FileNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_METADATA_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_METADATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WER_REPORT_METADATA_V3 {
    pub Signature: WER_REPORT_SIGNATURE,
    pub BucketId: ::windows::core::GUID,
    pub ReportId: ::windows::core::GUID,
    pub CreationTime: super::super::Foundation::FILETIME,
    pub SizeInBytes: u64,
    pub CabId: [u16; 260],
    pub ReportStatus: u32,
    pub ReportIntegratorId: ::windows::core::GUID,
    pub NumberOfFiles: u32,
    pub SizeOfFileNames: u32,
    pub FileNames: ::windows::core::PWSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WER_REPORT_METADATA_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_METADATA_V3")
            .field("Signature", &self.Signature)
            .field("BucketId", &self.BucketId)
            .field("ReportId", &self.ReportId)
            .field("CreationTime", &self.CreationTime)
            .field("SizeInBytes", &self.SizeInBytes)
            .field("CabId", &self.CabId)
            .field("ReportStatus", &self.ReportStatus)
            .field("ReportIntegratorId", &self.ReportIntegratorId)
            .field("NumberOfFiles", &self.NumberOfFiles)
            .field("SizeOfFileNames", &self.SizeOfFileNames)
            .field("FileNames", &self.FileNames)
            .field("FriendlyEventName", &self.FriendlyEventName)
            .field("ApplicationName", &self.ApplicationName)
            .field("ApplicationPath", &self.ApplicationPath)
            .field("Description", &self.Description)
            .field("BucketIdString", &self.BucketIdString)
            .field("LegacyBucketId", &self.LegacyBucketId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WER_REPORT_METADATA_V3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WER_REPORT_METADATA_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.BucketId == other.BucketId && self.ReportId == other.ReportId && self.CreationTime == other.CreationTime && self.SizeInBytes == other.SizeInBytes && self.CabId == other.CabId && self.ReportStatus == other.ReportStatus && self.ReportIntegratorId == other.ReportIntegratorId && self.NumberOfFiles == other.NumberOfFiles && self.SizeOfFileNames == other.SizeOfFileNames && self.FileNames == other.FileNames && self.FriendlyEventName == other.FriendlyEventName && self.ApplicationName == other.ApplicationName && self.ApplicationPath == other.ApplicationPath && self.Description == other.Description && self.BucketIdString == other.BucketIdString && self.LegacyBucketId == other.LegacyBucketId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WER_REPORT_METADATA_V3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WER_REPORT_METADATA_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
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
impl ::core::fmt::Debug for WER_REPORT_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_PARAMETER").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for WER_REPORT_PARAMETER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WER_REPORT_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for WER_REPORT_PARAMETER {}
impl ::core::default::Default for WER_REPORT_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
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
impl ::core::fmt::Debug for WER_REPORT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WER_REPORT_SIGNATURE").field("EventName", &self.EventName).field("Parameters", &self.Parameters).finish()
    }
}
impl ::windows::core::TypeKind for WER_REPORT_SIGNATURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WER_REPORT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.EventName == other.EventName && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for WER_REPORT_SIGNATURE {}
impl ::core::default::Default for WER_REPORT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub struct WER_RUNTIME_EXCEPTION_INFORMATION {
    pub dwSize: u32,
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub exceptionRecord: super::Diagnostics::Debug::EXCEPTION_RECORD,
    pub context: super::Diagnostics::Debug::CONTEXT,
    pub pwszReportId: ::windows::core::PCWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::windows::core::TypeKind for WER_RUNTIME_EXCEPTION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for WER_RUNTIME_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_DEBUGGER_LAUNCH = ::core::option::Option<unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbiscustomdebugger: *mut super::super::Foundation::BOOL, pwszdebuggerlaunch: ::windows::core::PWSTR, pchdebuggerlaunch: *mut u32, pbisdebuggerautolaunch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT = ::core::option::Option<unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, pbownershipclaimed: *mut super::super::Foundation::BOOL, pwszeventname: ::windows::core::PWSTR, pchsize: *mut u32, pdwsignaturecount: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PFN_WER_RUNTIME_EXCEPTION_EVENT_SIGNATURE = ::core::option::Option<unsafe extern "system" fn(pcontext: *const ::core::ffi::c_void, pexceptioninformation: *const WER_RUNTIME_EXCEPTION_INFORMATION, dwindex: u32, pwszname: ::windows::core::PWSTR, pchname: *mut u32, pwszvalue: ::windows::core::PWSTR, pchvalue: *mut u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub type pfn_ADDEREXCLUDEDAPPLICATIONA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR) -> EFaultRepRetVal>;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`*"]
pub type pfn_ADDEREXCLUDEDAPPLICATIONW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR) -> EFaultRepRetVal>;
#[doc = "*Required features: `\"Win32_System_ErrorReporting\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type pfn_REPORTFAULT = ::core::option::Option<unsafe extern "system" fn(param0: *const super::Diagnostics::Debug::EXCEPTION_POINTERS, param1: u32) -> EFaultRepRetVal>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
