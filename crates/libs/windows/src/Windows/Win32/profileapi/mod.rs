#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryPerformanceCounter(lpperformancecount: *mut super::LARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryPerformanceCounter(lpperformancecount : *mut super::LARGE_INTEGER) -> windows_core::BOOL);
    unsafe { QueryPerformanceCounter(lpperformancecount as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryPerformanceFrequency(lpfrequency: *mut super::LARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryPerformanceFrequency(lpfrequency : *mut super::LARGE_INTEGER) -> windows_core::BOOL);
    unsafe { QueryPerformanceFrequency(lpfrequency as _) }
}
