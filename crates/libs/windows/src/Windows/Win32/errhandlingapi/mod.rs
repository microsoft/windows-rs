#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddVectoredContinueHandler(first: super::ULONG, handler: super::PVECTORED_EXCEPTION_HANDLER) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn AddVectoredContinueHandler(first : super::ULONG, handler : super::PVECTORED_EXCEPTION_HANDLER) -> super::PVOID);
    unsafe { AddVectoredContinueHandler(first, handler) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddVectoredExceptionHandler(first: super::ULONG, handler: super::PVECTORED_EXCEPTION_HANDLER) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn AddVectoredExceptionHandler(first : super::ULONG, handler : super::PVECTORED_EXCEPTION_HANDLER) -> super::PVOID);
    unsafe { AddVectoredExceptionHandler(first, handler) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FatalAppExitA<P1>(uaction: super::UINT, lpmessagetext: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FatalAppExitA(uaction : super::UINT, lpmessagetext : windows_core::PCSTR));
    unsafe { FatalAppExitA(uaction, lpmessagetext.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FatalAppExitW<P1>(uaction: super::UINT, lpmessagetext: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FatalAppExitW(uaction : super::UINT, lpmessagetext : windows_core::PCWSTR));
    unsafe { FatalAppExitW(uaction, lpmessagetext.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetErrorMode() -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetErrorMode() -> super::UINT);
    unsafe { GetErrorMode() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetLastError() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetLastError() -> super::DWORD);
    unsafe { GetLastError() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetThreadErrorMode() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetThreadErrorMode() -> super::DWORD);
    unsafe { GetThreadErrorMode() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RaiseException(dwexceptioncode: super::DWORD, dwexceptionflags: super::DWORD, lparguments: Option<&[super::ULONG_PTR]>) {
    windows_core::link!("kernel32.dll" "system" fn RaiseException(dwexceptioncode : super::DWORD, dwexceptionflags : super::DWORD, nnumberofarguments : super::DWORD, lparguments : *const super::ULONG_PTR));
    unsafe { RaiseException(dwexceptioncode, dwexceptionflags, lparguments.map_or(0, |slice| slice.len().try_into().unwrap()), lparguments.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RaiseFailFastException(pexceptionrecord: Option<super::PEXCEPTION_RECORD>, pcontextrecord: Option<super::PCONTEXT>, dwflags: super::DWORD) {
    windows_core::link!("kernel32.dll" "system" fn RaiseFailFastException(pexceptionrecord : super::PEXCEPTION_RECORD, pcontextrecord : super::PCONTEXT, dwflags : super::DWORD));
    unsafe { RaiseFailFastException(pexceptionrecord.unwrap_or(core::mem::zeroed()) as _, pcontextrecord.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RemoveVectoredContinueHandler(handle: super::PVOID) -> super::ULONG {
    windows_core::link!("kernel32.dll" "system" fn RemoveVectoredContinueHandler(handle : super::PVOID) -> super::ULONG);
    unsafe { RemoveVectoredContinueHandler(handle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RemoveVectoredExceptionHandler(handle: super::PVOID) -> super::ULONG {
    windows_core::link!("kernel32.dll" "system" fn RemoveVectoredExceptionHandler(handle : super::PVOID) -> super::ULONG);
    unsafe { RemoveVectoredExceptionHandler(handle) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetErrorMode(umode: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn SetErrorMode(umode : super::UINT) -> super::UINT);
    unsafe { SetErrorMode(umode) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetLastError(dwerrcode: super::DWORD) {
    windows_core::link!("kernel32.dll" "system" fn SetLastError(dwerrcode : super::DWORD));
    unsafe { SetLastError(dwerrcode) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn SetThreadErrorMode(dwnewmode: super::DWORD, lpoldmode: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadErrorMode(dwnewmode : super::DWORD, lpoldmode : super::LPDWORD) -> windows_core::BOOL);
    unsafe { SetThreadErrorMode(dwnewmode, lpoldmode.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter: LPTOP_LEVEL_EXCEPTION_FILTER) -> LPTOP_LEVEL_EXCEPTION_FILTER {
    windows_core::link!("kernel32.dll" "system" fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter : LPTOP_LEVEL_EXCEPTION_FILTER) -> LPTOP_LEVEL_EXCEPTION_FILTER);
    unsafe { SetUnhandledExceptionFilter(lptoplevelexceptionfilter) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn TerminateProcessOnMemoryExhaustion(failedallocationsize: super::SIZE_T) {
    windows_core::link!("api-ms-win-core-errorhandling-l1-1-3.dll" "system" fn TerminateProcessOnMemoryExhaustion(failedallocationsize : super::SIZE_T));
    unsafe { TerminateProcessOnMemoryExhaustion(failedallocationsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UnhandledExceptionFilter(exceptioninfo: *const super::EXCEPTION_POINTERS) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn UnhandledExceptionFilter(exceptioninfo : *const super::EXCEPTION_POINTERS) -> super::LONG);
    unsafe { UnhandledExceptionFilter(exceptioninfo) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPTOP_LEVEL_EXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptioninfo: *const super::EXCEPTION_POINTERS) -> super::LONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTOP_LEVEL_EXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptioninfo: *const super::EXCEPTION_POINTERS) -> super::LONG>;
