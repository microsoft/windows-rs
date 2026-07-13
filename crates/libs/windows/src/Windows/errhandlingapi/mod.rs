#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddVectoredContinueHandler(first: u32, handler: super::winnt::PVECTORED_EXCEPTION_HANDLER) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn AddVectoredContinueHandler(first : u32, handler : super::winnt::PVECTORED_EXCEPTION_HANDLER) -> *mut core::ffi::c_void);
    unsafe { AddVectoredContinueHandler(first, handler) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddVectoredExceptionHandler(first: u32, handler: super::winnt::PVECTORED_EXCEPTION_HANDLER) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn AddVectoredExceptionHandler(first : u32, handler : super::winnt::PVECTORED_EXCEPTION_HANDLER) -> *mut core::ffi::c_void);
    unsafe { AddVectoredExceptionHandler(first, handler) }
}
#[inline]
pub unsafe fn FatalAppExitA<P1>(uaction: u32, lpmessagetext: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FatalAppExitA(uaction : u32, lpmessagetext : windows_core::PCSTR));
    unsafe { FatalAppExitA(uaction, lpmessagetext.param().abi()) }
}
#[inline]
pub unsafe fn FatalAppExitW<P1>(uaction: u32, lpmessagetext: P1)
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FatalAppExitW(uaction : u32, lpmessagetext : windows_core::PCWSTR));
    unsafe { FatalAppExitW(uaction, lpmessagetext.param().abi()) }
}
#[inline]
pub unsafe fn GetErrorMode() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetErrorMode() -> u32);
    unsafe { GetErrorMode() }
}
#[inline]
pub unsafe fn GetLastError() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetLastError() -> u32);
    unsafe { GetLastError() }
}
#[inline]
pub unsafe fn GetThreadErrorMode() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetThreadErrorMode() -> u32);
    unsafe { GetThreadErrorMode() }
}
#[inline]
pub unsafe fn RaiseException(dwexceptioncode: u32, dwexceptionflags: u32, lparguments: Option<&[usize]>) {
    windows_core::link!("kernel32.dll" "system" fn RaiseException(dwexceptioncode : u32, dwexceptionflags : u32, nnumberofarguments : u32, lparguments : *const usize));
    unsafe { RaiseException(dwexceptioncode, dwexceptionflags, lparguments.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lparguments.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RaiseFailFastException(pexceptionrecord: Option<*const super::winnt::EXCEPTION_RECORD>, pcontextrecord: Option<*const super::winnt::CONTEXT>, dwflags: u32) {
    windows_core::link!("kernel32.dll" "system" fn RaiseFailFastException(pexceptionrecord : *const super::winnt::EXCEPTION_RECORD, pcontextrecord : *const super::winnt::CONTEXT, dwflags : u32));
    unsafe { RaiseFailFastException(pexceptionrecord.unwrap_or(core::mem::zeroed()) as _, pcontextrecord.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RaiseFailFastException(pexceptionrecord: Option<*const super::winnt::EXCEPTION_RECORD>, pcontextrecord: Option<*const super::winnt::ARM64_NT_CONTEXT>, dwflags: u32) {
    windows_core::link!("kernel32.dll" "system" fn RaiseFailFastException(pexceptionrecord : *const super::winnt::EXCEPTION_RECORD, pcontextrecord : *const super::winnt::ARM64_NT_CONTEXT, dwflags : u32));
    unsafe { RaiseFailFastException(pexceptionrecord.unwrap_or(core::mem::zeroed()) as _, pcontextrecord.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn RemoveVectoredContinueHandler(handle: *const core::ffi::c_void) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn RemoveVectoredContinueHandler(handle : *const core::ffi::c_void) -> u32);
    unsafe { RemoveVectoredContinueHandler(handle) }
}
#[inline]
pub unsafe fn RemoveVectoredExceptionHandler(handle: *const core::ffi::c_void) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn RemoveVectoredExceptionHandler(handle : *const core::ffi::c_void) -> u32);
    unsafe { RemoveVectoredExceptionHandler(handle) }
}
#[inline]
pub unsafe fn SetErrorMode(umode: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SetErrorMode(umode : u32) -> u32);
    unsafe { SetErrorMode(umode) }
}
#[inline]
pub unsafe fn SetLastError(dwerrcode: u32) {
    windows_core::link!("kernel32.dll" "system" fn SetLastError(dwerrcode : u32));
    unsafe { SetLastError(dwerrcode) }
}
#[inline]
pub unsafe fn SetThreadErrorMode(dwnewmode: u32, lpoldmode: Option<*const u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadErrorMode(dwnewmode : u32, lpoldmode : *const u32) -> windows_core::BOOL);
    unsafe { SetThreadErrorMode(dwnewmode, lpoldmode.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter: Option<LPTOP_LEVEL_EXCEPTION_FILTER>) -> LPTOP_LEVEL_EXCEPTION_FILTER {
    windows_core::link!("kernel32.dll" "system" fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter : LPTOP_LEVEL_EXCEPTION_FILTER) -> LPTOP_LEVEL_EXCEPTION_FILTER);
    unsafe { SetUnhandledExceptionFilter(lptoplevelexceptionfilter.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TerminateProcessOnMemoryExhaustion(failedallocationsize: usize) {
    windows_core::link!("api-ms-win-core-errorhandling-l1-1-3.dll" "system" fn TerminateProcessOnMemoryExhaustion(failedallocationsize : usize));
    unsafe { TerminateProcessOnMemoryExhaustion(failedallocationsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnhandledExceptionFilter(exceptioninfo: *const super::winnt::EXCEPTION_POINTERS) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn UnhandledExceptionFilter(exceptioninfo : *const super::winnt::EXCEPTION_POINTERS) -> i32);
    unsafe { UnhandledExceptionFilter(exceptioninfo) }
}
#[cfg(feature = "winnt")]
pub type LPTOP_LEVEL_EXCEPTION_FILTER = PTOP_LEVEL_EXCEPTION_FILTER;
#[cfg(feature = "winnt")]
pub type PTOP_LEVEL_EXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptioninfo: *const super::winnt::EXCEPTION_POINTERS) -> i32>;
