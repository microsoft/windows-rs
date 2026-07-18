#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ChangeTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE, duetime : u32, period : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateTimerQueue() -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateTimerQueueTimer(phnewtimer : *mut super::HANDLE, timerqueue : super::HANDLE, callback : super::WAITORTIMERCALLBACK, parameter : *const core::ffi::c_void, duetime : u32, period : u32, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueue(timerqueue : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueueEx(timerqueue : super::HANDLE, completionevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE, completionevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn QueueUserWorkItem(function : super::LPTHREAD_START_ROUTINE, context : *const core::ffi::c_void, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn UnregisterWaitEx(waithandle : super::HANDLE, completionevent : super::HANDLE) -> windows_sys::core::BOOL);
