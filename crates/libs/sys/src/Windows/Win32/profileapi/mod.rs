windows_link::link!("kernel32.dll" "system" fn QueryPerformanceCounter(lpperformancecount : *mut i64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn QueryPerformanceFrequency(lpfrequency : *mut i64) -> windows_sys::core::BOOL);
