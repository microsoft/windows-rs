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
    windows_targets::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const windows::Win32::Security:: SECURITY_ATTRIBUTES, bmanualreset : windows::Win32::Foundation:: BOOL, binitialstate : windows::Win32::Foundation:: BOOL, lpname : windows_core::PCWSTR) -> windows::Win32::Foundation:: HANDLE);
    let result__ = CreateEventW(
        core::mem::transmute(lpeventattributes.unwrap_or(core::mem::zeroed())),
        bmanualreset.into(),
        binitialstate.into(),
        lpname.param().abi(),
    );
    (!result__.is_invalid())
        .then_some(result__)
        .ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn NtWaitForSingleObject(
    handle: windows::Win32::Foundation::HANDLE,
    alertable: bool,
    timeout: *mut i64,
) -> windows::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : windows::Win32::Foundation:: HANDLE, alertable : bool, timeout : *mut i64) -> windows::Win32::Foundation:: NTSTATUS);
    NtWaitForSingleObject(
        core::mem::transmute(handle),
        core::mem::transmute(alertable),
        core::mem::transmute(timeout),
    )
}
#[inline]
pub unsafe fn SetEvent(hevent: windows::Win32::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetEvent(hevent : windows::Win32::Foundation:: HANDLE) -> windows::Win32::Foundation:: BOOL);
    SetEvent(core::mem::transmute(hevent)).ok()
}
#[inline]
pub unsafe fn WaitForSingleObjectEx(
    hhandle: windows::Win32::Foundation::HANDLE,
    dwmilliseconds: u32,
    balertable: bool,
) -> windows::Win32::Foundation::WAIT_EVENT {
    windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : windows::Win32::Foundation:: HANDLE, dwmilliseconds : u32, balertable : windows::Win32::Foundation:: BOOL) -> windows::Win32::Foundation:: WAIT_EVENT);
    WaitForSingleObjectEx(
        core::mem::transmute(hhandle),
        core::mem::transmute(dwmilliseconds),
        balertable.into(),
    )
}
