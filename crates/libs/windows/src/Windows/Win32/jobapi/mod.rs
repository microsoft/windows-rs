#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn IsProcessInJob(processhandle: super::winnt::HANDLE, jobhandle: Option<super::winnt::HANDLE>, result: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsProcessInJob(processhandle : super::winnt::HANDLE, jobhandle : super::winnt::HANDLE, result : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsProcessInJob(processhandle, jobhandle.unwrap_or(core::mem::zeroed()) as _, result as _) }
}
