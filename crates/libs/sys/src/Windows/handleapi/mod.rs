#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CloseHandle(hobject : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-handle-l1-1-0.dll" "system" fn CompareObjectHandles(hfirstobjecthandle : super::winnt::HANDLE, hsecondobjecthandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DuplicateHandle(hsourceprocesshandle : super::winnt::HANDLE, hsourcehandle : super::winnt::HANDLE, htargetprocesshandle : super::winnt::HANDLE, lptargethandle : *mut super::winnt::HANDLE, dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, dwoptions : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetHandleInformation(hobject : super::winnt::HANDLE, lpdwflags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetHandleInformation(hobject : super::winnt::HANDLE, dwmask : u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
pub const INVALID_HANDLE_VALUE: super::winnt::HANDLE = -1 as _;
