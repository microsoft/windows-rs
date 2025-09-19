#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn CreateEventW<P3>(
    lpeventattributes: Option<*const SECURITY_ATTRIBUTES>,
    bmanualreset: bool,
    binitialstate: bool,
    lpname: P3,
) -> windows_core::Result<HANDLE>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, binitialstate : windows_core::BOOL, lpname : windows_core::PCWSTR) -> HANDLE);
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
    handle: HANDLE,
    alertable: bool,
    timeout: *mut i64,
) -> NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : HANDLE, alertable : bool, timeout : *mut i64) -> NTSTATUS);
    unsafe { NtWaitForSingleObject(handle, alertable, timeout as _) }
}
#[inline]
pub unsafe fn SetEvent(hevent: HANDLE) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn SetEvent(hevent : HANDLE) -> windows_core::BOOL);
    unsafe { SetEvent(hevent).ok() }
}
#[inline]
pub unsafe fn WaitForSingleObjectEx(
    hhandle: HANDLE,
    dwmilliseconds: u32,
    balertable: bool,
) -> WAIT_EVENT {
    windows_core::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : HANDLE, dwmilliseconds : u32, balertable : windows_core::BOOL) -> WAIT_EVENT);
    unsafe { WaitForSingleObjectEx(hhandle, dwmilliseconds, balertable.into()) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HANDLE(pub *mut core::ffi::c_void);
impl HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HANDLE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : *mut core::ffi::c_void) -> i32);
            unsafe {
                CloseHandle(self.0);
            }
        }
    }
}
impl Default for HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NTSTATUS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: windows_core::BOOL,
}
impl Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WAIT_EVENT(pub u32);
