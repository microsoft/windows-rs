#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ChangeTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE, duetime : super::ULONG, period : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateTimerQueue() -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateTimerQueueTimer(phnewtimer : super::PHANDLE, timerqueue : super::HANDLE, callback : super::WAITORTIMERCALLBACK, parameter : super::PVOID, duetime : super::DWORD, period : super::DWORD, flags : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueue(timerqueue : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueueEx(timerqueue : super::HANDLE, completionevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE, completionevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueueUserWorkItem(function : super::LPTHREAD_START_ROUTINE, context : super::PVOID, flags : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn UnregisterWaitEx(waithandle : super::HANDLE, completionevent : super::HANDLE) -> windows_sys::core::BOOL);
