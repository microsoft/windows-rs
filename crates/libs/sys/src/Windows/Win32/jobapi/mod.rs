#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn IsProcessInJob(processhandle : super::HANDLE, jobhandle : super::HANDLE, result : super::PBOOL) -> windows_sys::core::BOOL);
