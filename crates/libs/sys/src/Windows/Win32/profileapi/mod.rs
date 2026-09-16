#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryPerformanceCounter(lpperformancecount : *mut super::LARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryPerformanceFrequency(lpfrequency : *mut super::LARGE_INTEGER) -> windows_sys::core::BOOL);
