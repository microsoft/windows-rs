#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn AddVectoredContinueHandler(first : u32, handler : super::winnt::PVECTORED_EXCEPTION_HANDLER) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn AddVectoredExceptionHandler(first : u32, handler : super::winnt::PVECTORED_EXCEPTION_HANDLER) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn FatalAppExitA(uaction : u32, lpmessagetext : windows_sys::core::PCSTR));
windows_link::link!("kernel32.dll" "system" fn FatalAppExitW(uaction : u32, lpmessagetext : windows_sys::core::PCWSTR));
windows_link::link!("kernel32.dll" "system" fn GetErrorMode() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetLastError() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetThreadErrorMode() -> u32);
windows_link::link!("kernel32.dll" "system" fn RaiseException(dwexceptioncode : u32, dwexceptionflags : u32, nnumberofarguments : u32, lparguments : *const usize));
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn RaiseFailFastException(pexceptionrecord : *const super::winnt::EXCEPTION_RECORD, pcontextrecord : *const super::winnt::CONTEXT, dwflags : u32));
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn RaiseFailFastException(pexceptionrecord : *const super::winnt::EXCEPTION_RECORD, pcontextrecord : *const super::winnt::ARM64_NT_CONTEXT, dwflags : u32));
windows_link::link!("kernel32.dll" "system" fn RemoveVectoredContinueHandler(handle : *const core::ffi::c_void) -> u32);
windows_link::link!("kernel32.dll" "system" fn RemoveVectoredExceptionHandler(handle : *const core::ffi::c_void) -> u32);
windows_link::link!("kernel32.dll" "system" fn SetErrorMode(umode : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn SetLastError(dwerrcode : u32));
windows_link::link!("kernel32.dll" "system" fn SetThreadErrorMode(dwnewmode : u32, lpoldmode : *const u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter : LPTOP_LEVEL_EXCEPTION_FILTER) -> LPTOP_LEVEL_EXCEPTION_FILTER);
windows_link::link!("api-ms-win-core-errorhandling-l1-1-3.dll" "system" fn TerminateProcessOnMemoryExhaustion(failedallocationsize : usize));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn UnhandledExceptionFilter(exceptioninfo : *const super::winnt::EXCEPTION_POINTERS) -> i32);
#[cfg(feature = "Win32_winnt")]
pub type LPTOP_LEVEL_EXCEPTION_FILTER = PTOP_LEVEL_EXCEPTION_FILTER;
#[cfg(feature = "Win32_winnt")]
pub type PTOP_LEVEL_EXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptioninfo: *const super::winnt::EXCEPTION_POINTERS) -> i32>;
