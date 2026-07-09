#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseHandle(hobject: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { CloseHandle(hobject) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CompareObjectHandles(hfirstobjecthandle: super::winnt::HANDLE, hsecondobjecthandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-handle-l1-1-0.dll" "system" fn CompareObjectHandles(hfirstobjecthandle : super::winnt::HANDLE, hsecondobjecthandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { CompareObjectHandles(hfirstobjecthandle, hsecondobjecthandle) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DuplicateHandle(hsourceprocesshandle: super::winnt::HANDLE, hsourcehandle: super::winnt::HANDLE, htargetprocesshandle: super::winnt::HANDLE, lptargethandle: *mut super::winnt::HANDLE, dwdesiredaccess: u32, binherithandle: bool, dwoptions: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DuplicateHandle(hsourceprocesshandle : super::winnt::HANDLE, hsourcehandle : super::winnt::HANDLE, htargetprocesshandle : super::winnt::HANDLE, lptargethandle : *mut super::winnt::HANDLE, dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwoptions : u32) -> windows_core::BOOL);
    unsafe { DuplicateHandle(hsourceprocesshandle, hsourcehandle, htargetprocesshandle, lptargethandle as _, dwdesiredaccess, binherithandle.into(), dwoptions) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetHandleInformation(hobject: super::winnt::HANDLE, lpdwflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetHandleInformation(hobject : super::winnt::HANDLE, lpdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetHandleInformation(hobject, lpdwflags as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetHandleInformation(hobject: super::winnt::HANDLE, dwmask: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetHandleInformation(hobject : super::winnt::HANDLE, dwmask : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { SetHandleInformation(hobject, dwmask, dwflags) }
}
#[cfg(feature = "Win32_winnt")]
pub const INVALID_HANDLE_VALUE: super::winnt::HANDLE = super::winnt::HANDLE(-1 as _);
