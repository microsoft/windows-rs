#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsProcessInJob(processhandle: super::HANDLE, jobhandle: Option<super::HANDLE>, result: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsProcessInJob(processhandle : super::HANDLE, jobhandle : super::HANDLE, result : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsProcessInJob(processhandle, jobhandle.unwrap_or(core::mem::zeroed()) as _, result as _) }
}
