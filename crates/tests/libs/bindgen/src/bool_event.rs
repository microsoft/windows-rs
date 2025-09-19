#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn CreateEventW<P3>(
    lpeventattributes: Option<*const windows::Win32::Security::SECURITY_ATTRIBUTES>,
    bmanualreset: bool,
    binitialstate: bool,
    lpname: P3,
) -> windows_core::Result<windows::Win32::Foundation::HANDLE>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const windows::Win32::Security:: SECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, binitialstate : windows_core::BOOL, lpname : windows_core::PCWSTR) -> windows::Win32::Foundation:: HANDLE);
    let result__ = unsafe {
        CreateEventW(
            lpeventattributes.unwrap_or(core::mem::zeroed()) as _,
            bmanualreset.into(),
            binitialstate.into(),
            lpname.param().abi(),
        )
    };
    (!result__.is_invalid())
        .then_some(result__)
        .ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn NtWaitForSingleObject(
    handle: windows::Win32::Foundation::HANDLE,
    alertable: bool,
    timeout: *mut i64,
) -> windows::Win32::Foundation::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : windows::Win32::Foundation:: HANDLE, alertable : bool, timeout : *mut i64) -> windows::Win32::Foundation:: NTSTATUS);
    unsafe { NtWaitForSingleObject(handle, alertable, timeout as _) }
}
#[inline]
pub unsafe fn SetEvent(hevent: windows::Win32::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetEvent(hevent : windows::Win32::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { SetEvent(hevent).ok() }
}
#[inline]
pub unsafe fn WaitForSingleObjectEx(
    hhandle: windows::Win32::Foundation::HANDLE,
    dwmilliseconds: u32,
    balertable: bool,
) -> windows::Win32::Foundation::WAIT_EVENT {
    windows_core::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : windows::Win32::Foundation:: HANDLE, dwmilliseconds : u32, balertable : windows_core::BOOL) -> windows::Win32::Foundation:: WAIT_EVENT);
    unsafe { WaitForSingleObjectEx(hhandle, dwmilliseconds, balertable.into()) }
}
