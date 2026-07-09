#[inline]
pub unsafe fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryPerformanceCounter(lpperformancecount : *mut i64) -> windows_core::BOOL);
    unsafe { QueryPerformanceCounter(lpperformancecount as _) }
}
#[inline]
pub unsafe fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryPerformanceFrequency(lpfrequency : *mut i64) -> windows_core::BOOL);
    unsafe { QueryPerformanceFrequency(lpfrequency as _) }
}
