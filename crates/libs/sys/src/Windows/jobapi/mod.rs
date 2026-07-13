#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn IsProcessInJob(processhandle : super::winnt::HANDLE, jobhandle : super::winnt::HANDLE, result : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
