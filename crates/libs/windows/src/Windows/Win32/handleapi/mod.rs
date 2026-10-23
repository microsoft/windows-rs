#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseHandle(hobject: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : super::HANDLE) -> windows_core::BOOL);
    unsafe { CloseHandle(hobject) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CompareObjectHandles(hfirstobjecthandle: super::HANDLE, hsecondobjecthandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-handle-l1-1-0.dll" "system" fn CompareObjectHandles(hfirstobjecthandle : super::HANDLE, hsecondobjecthandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { CompareObjectHandles(hfirstobjecthandle, hsecondobjecthandle) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DuplicateHandle(hsourceprocesshandle: super::HANDLE, hsourcehandle: super::HANDLE, htargetprocesshandle: super::HANDLE, lptargethandle: super::LPHANDLE, dwdesiredaccess: super::DWORD, binherithandle: bool, dwoptions: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DuplicateHandle(hsourceprocesshandle : super::HANDLE, hsourcehandle : super::HANDLE, htargetprocesshandle : super::HANDLE, lptargethandle : super::LPHANDLE, dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, dwoptions : super::DWORD) -> windows_core::BOOL);
    unsafe { DuplicateHandle(hsourceprocesshandle, hsourcehandle, htargetprocesshandle, lptargethandle as _, dwdesiredaccess, binherithandle.into(), dwoptions) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetHandleInformation(hobject: super::HANDLE, lpdwflags: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetHandleInformation(hobject : super::HANDLE, lpdwflags : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetHandleInformation(hobject, lpdwflags as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetHandleInformation(hobject: super::HANDLE, dwmask: super::DWORD, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetHandleInformation(hobject : super::HANDLE, dwmask : super::DWORD, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { SetHandleInformation(hobject, dwmask, dwflags) }
}
