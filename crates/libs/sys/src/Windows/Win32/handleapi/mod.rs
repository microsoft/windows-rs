#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseHandle(hobject : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-handle-l1-1-0.dll" "system" fn CompareObjectHandles(hfirstobjecthandle : super::HANDLE, hsecondobjecthandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DuplicateHandle(hsourceprocesshandle : super::HANDLE, hsourcehandle : super::HANDLE, htargetprocesshandle : super::HANDLE, lptargethandle : super::LPHANDLE, dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, dwoptions : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetHandleInformation(hobject : super::HANDLE, lpdwflags : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetHandleInformation(hobject : super::HANDLE, dwmask : super::DWORD, dwflags : super::DWORD) -> windows_sys::core::BOOL);
