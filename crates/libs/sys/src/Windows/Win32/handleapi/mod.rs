#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseHandle(hobject : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-handle-l1-1-0.dll" "system" fn CompareObjectHandles(hfirstobjecthandle : super::HANDLE, hsecondobjecthandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DuplicateHandle(hsourceprocesshandle : super::HANDLE, hsourcehandle : super::HANDLE, htargetprocesshandle : super::HANDLE, lptargethandle : *mut super::HANDLE, dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, dwoptions : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetHandleInformation(hobject : super::HANDLE, lpdwflags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetHandleInformation(hobject : super::HANDLE, dwmask : u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
pub const INVALID_HANDLE_VALUE: super::HANDLE = -1 as _;
