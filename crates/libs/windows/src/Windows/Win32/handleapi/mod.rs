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
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DuplicateHandle(hsourceprocesshandle: super::HANDLE, hsourcehandle: super::HANDLE, htargetprocesshandle: super::HANDLE, lptargethandle: *mut super::HANDLE, dwdesiredaccess: u32, binherithandle: bool, dwoptions: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DuplicateHandle(hsourceprocesshandle : super::HANDLE, hsourcehandle : super::HANDLE, htargetprocesshandle : super::HANDLE, lptargethandle : *mut super::HANDLE, dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwoptions : u32) -> windows_core::BOOL);
    unsafe { DuplicateHandle(hsourceprocesshandle, hsourcehandle, htargetprocesshandle, lptargethandle as _, dwdesiredaccess, binherithandle.into(), dwoptions) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetHandleInformation(hobject: super::HANDLE, lpdwflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetHandleInformation(hobject : super::HANDLE, lpdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetHandleInformation(hobject, lpdwflags as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetHandleInformation(hobject: super::HANDLE, dwmask: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetHandleInformation(hobject : super::HANDLE, dwmask : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { SetHandleInformation(hobject, dwmask, dwflags) }
}
#[cfg(feature = "winnt")]
pub const INVALID_HANDLE_VALUE: super::HANDLE = super::HANDLE(-1 as _);
