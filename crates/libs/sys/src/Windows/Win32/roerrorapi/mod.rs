#[cfg(feature = "Win32_restrictederrorinfo")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn GetRestrictedErrorInfo(pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn IsErrorPropagationEnabled() -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoCaptureErrorContext(hr : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoClearError());
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoFailFastWithErrorContext(hrerror : windows_sys::core::HRESULT));
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoGetErrorReportingFlags(pflags : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_restrictederrorinfo")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoGetMatchingRestrictedErrorInfo(hrin : windows_sys::core::HRESULT, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, framecount : *mut u32, targetbacktraceaddress : *mut usize) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectThreadErrorInfo(targettebaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, targeterrorinfoaddress : *mut usize) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateError(error : windows_sys::core::HRESULT, message : windows_sys::core::HSTRING) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : windows_sys::core::HRESULT, cchmax : u32, message : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoOriginateLanguageException(error : windows_sys::core::HRESULT, message : windows_sys::core::HSTRING, languageexception : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_restrictederrorinfo")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportFailedDelegate(punkdelegate : *mut core::ffi::c_void, prestrictederrorinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_restrictederrorinfo")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportUnhandledError(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_restrictederrorinfo")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoResolveRestrictedErrorInfoReference(reference : windows_sys::core::PCWSTR, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoSetErrorReportingFlags(flags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformError(olderror : windows_sys::core::HRESULT, newerror : windows_sys::core::HRESULT, message : windows_sys::core::HSTRING) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformErrorW(olderror : windows_sys::core::HRESULT, newerror : windows_sys::core::HRESULT, cchmax : u32, message : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_restrictederrorinfo")]
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn SetRestrictedErrorInfo(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const EXCEPTION_RO_ORIGINATEERROR: u32 = 1074266625;
pub const EXCEPTION_RO_TRANSFORMERROR: u32 = 1074266626;
pub const ForceExceptions: RoErrorReportingFlags = 2;
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512;
pub const None: RoErrorReportingFlags = 0;
pub type PINSPECT_MEMORY_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> windows_sys::core::HRESULT>;
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
