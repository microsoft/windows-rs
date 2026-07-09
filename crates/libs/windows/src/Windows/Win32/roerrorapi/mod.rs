#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn GetRestrictedErrorInfo() -> windows_core::Result<super::restrictederrorinfo::IRestrictedErrorInfo> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn GetRestrictedErrorInfo(pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetRestrictedErrorInfo(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn IsErrorPropagationEnabled() -> windows_core::BOOL);
    unsafe { IsErrorPropagationEnabled() }
}
#[inline]
pub unsafe fn RoCaptureErrorContext(hr: windows_core::HRESULT) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoCaptureErrorContext(hr : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { RoCaptureErrorContext(hr) }
}
#[inline]
pub unsafe fn RoClearError() {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoClearError());
    unsafe { RoClearError() }
}
#[inline]
pub unsafe fn RoFailFastWithErrorContext(hrerror: windows_core::HRESULT) {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoFailFastWithErrorContext(hrerror : windows_core::HRESULT));
    unsafe { RoFailFastWithErrorContext(hrerror) }
}
#[inline]
pub unsafe fn RoGetErrorReportingFlags() -> windows_core::Result<u32> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoGetErrorReportingFlags(pflags : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetErrorReportingFlags(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: windows_core::HRESULT) -> windows_core::Result<super::restrictederrorinfo::IRestrictedErrorInfo> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoGetMatchingRestrictedErrorInfo(hrin : windows_core::HRESULT, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetMatchingRestrictedErrorInfo(hrin, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<*const core::ffi::c_void>, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, framecount : *mut u32, targetbacktraceaddress : *mut usize) -> windows_core::HRESULT);
    unsafe { RoInspectCapturedStackBackTrace(targeterrorinfoaddress, machine, readmemorycallback, context.unwrap_or(core::mem::zeroed()) as _, framecount as _, targetbacktraceaddress as _) }
}
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<*const core::ffi::c_void>) -> windows_core::Result<usize> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectThreadErrorInfo(targettebaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, targeterrorinfoaddress : *mut usize) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoInspectThreadErrorInfo(targettebaddress, machine, readmemorycallback, context.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RoOriginateError(error: windows_core::HRESULT, message: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateError(error : windows_core::HRESULT, message : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoOriginateError(error, core::mem::transmute_copy(message)) }
}
#[inline]
pub unsafe fn RoOriginateErrorW<P2>(error: windows_core::HRESULT, cchmax: u32, message: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : windows_core::HRESULT, cchmax : u32, message : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RoOriginateErrorW(error, cchmax, message.param().abi()) }
}
#[inline]
pub unsafe fn RoOriginateLanguageException<P2>(error: windows_core::HRESULT, message: &windows_core::HSTRING, languageexception: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoOriginateLanguageException(error : windows_core::HRESULT, message : *mut core::ffi::c_void, languageexception : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoOriginateLanguageException(error, core::mem::transmute_copy(message), languageexception.param().abi()) }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn RoReportFailedDelegate<P0, P1>(punkdelegate: P0, prestrictederrorinfo: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::restrictederrorinfo::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportFailedDelegate(punkdelegate : *mut core::ffi::c_void, prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoReportFailedDelegate(punkdelegate.param().abi(), prestrictederrorinfo.param().abi()) }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn RoReportUnhandledError<P0>(prestrictederrorinfo: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::restrictederrorinfo::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportUnhandledError(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoReportUnhandledError(prestrictederrorinfo.param().abi()) }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<P0>(reference: P0) -> windows_core::Result<super::restrictederrorinfo::IRestrictedErrorInfo>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoResolveRestrictedErrorInfoReference(reference : windows_core::PCWSTR, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoResolveRestrictedErrorInfoReference(reference.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: u32) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoSetErrorReportingFlags(flags : u32) -> windows_core::HRESULT);
    unsafe { RoSetErrorReportingFlags(flags) }
}
#[inline]
pub unsafe fn RoTransformError(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, message: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformError(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, message : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoTransformError(olderror, newerror, core::mem::transmute_copy(message)) }
}
#[inline]
pub unsafe fn RoTransformErrorW<P3>(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, cchmax: u32, message: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformErrorW(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, cchmax : u32, message : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RoTransformErrorW(olderror, newerror, cchmax, message.param().abi()) }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn SetRestrictedErrorInfo<P0>(prestrictederrorinfo: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::restrictederrorinfo::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn SetRestrictedErrorInfo(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SetRestrictedErrorInfo(prestrictederrorinfo.param().abi()) }
}
pub const EXCEPTION_RO_ORIGINATEERROR: u32 = 1074266625;
pub const EXCEPTION_RO_TRANSFORMERROR: u32 = 1074266626;
pub const ForceExceptions: RoErrorReportingFlags = 2;
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512;
pub const None: RoErrorReportingFlags = 0;
pub type PINSPECT_MEMORY_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> windows_core::HRESULT>;
pub type RO_ERROR_REPORTING_FLAGS = u32;
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 2;
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = 0;
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 1;
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = 8;
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = 4;
pub type RoErrorReportingFlags = u32;
pub const SuppressExceptions: RoErrorReportingFlags = 1;
pub const SuppressSetErrorInfo: RoErrorReportingFlags = 8;
pub const UseSetErrorInfo: RoErrorReportingFlags = 4;
