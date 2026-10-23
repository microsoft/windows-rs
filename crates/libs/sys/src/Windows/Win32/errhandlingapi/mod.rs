#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AddVectoredContinueHandler(first : super::ULONG, handler : super::PVECTORED_EXCEPTION_HANDLER) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AddVectoredExceptionHandler(first : super::ULONG, handler : super::PVECTORED_EXCEPTION_HANDLER) -> super::PVOID);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn FatalAppExitA(uaction : super::UINT, lpmessagetext : windows_sys::core::PCSTR));
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn FatalAppExitW(uaction : super::UINT, lpmessagetext : windows_sys::core::PCWSTR));
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetErrorMode() -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetLastError() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetThreadErrorMode() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn RaiseException(dwexceptioncode : super::DWORD, dwexceptionflags : super::DWORD, nnumberofarguments : super::DWORD, lparguments : *const super::ULONG_PTR));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn RaiseFailFastException(pexceptionrecord : super::PEXCEPTION_RECORD, pcontextrecord : super::PCONTEXT, dwflags : super::DWORD));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn RemoveVectoredContinueHandler(handle : super::PVOID) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn RemoveVectoredExceptionHandler(handle : super::PVOID) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetErrorMode(umode : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetLastError(dwerrcode : super::DWORD));
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadErrorMode(dwnewmode : super::DWORD, lpoldmode : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter : LPTOP_LEVEL_EXCEPTION_FILTER) -> LPTOP_LEVEL_EXCEPTION_FILTER);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-errorhandling-l1-1-3.dll" "system" fn TerminateProcessOnMemoryExhaustion(failedallocationsize : super::SIZE_T));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn UnhandledExceptionFilter(exceptioninfo : *const super::EXCEPTION_POINTERS) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPTOP_LEVEL_EXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptioninfo: *const super::EXCEPTION_POINTERS) -> super::LONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTOP_LEVEL_EXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptioninfo: *const super::EXCEPTION_POINTERS) -> super::LONG>;
