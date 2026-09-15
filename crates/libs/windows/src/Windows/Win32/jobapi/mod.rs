#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn IsProcessInJob(processhandle: super::HANDLE, jobhandle: Option<super::HANDLE>, result: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsProcessInJob(processhandle : super::HANDLE, jobhandle : super::HANDLE, result : super::PBOOL) -> windows_core::BOOL);
    unsafe { IsProcessInJob(processhandle, jobhandle.unwrap_or(core::mem::zeroed()) as _, result as _) }
}
