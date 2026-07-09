#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn ChangeTimerQueueTimer(timerqueue : super::winnt::HANDLE, timer : super::winnt::HANDLE, duetime : u32, period : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateTimerQueue() -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateTimerQueueTimer(phnewtimer : *mut super::winnt::HANDLE, timerqueue : super::winnt::HANDLE, callback : super::winnt::WAITORTIMERCALLBACK, parameter : *const core::ffi::c_void, duetime : u32, period : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueue(timerqueue : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueueEx(timerqueue : super::winnt::HANDLE, completionevent : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueueTimer(timerqueue : super::winnt::HANDLE, timer : super::winnt::HANDLE, completionevent : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("kernel32.dll" "system" fn QueueUserWorkItem(function : super::minwinbase::LPTHREAD_START_ROUTINE, context : *const core::ffi::c_void, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn UnregisterWaitEx(waithandle : super::winnt::HANDLE, completionevent : super::winnt::HANDLE) -> windows_sys::core::BOOL);
