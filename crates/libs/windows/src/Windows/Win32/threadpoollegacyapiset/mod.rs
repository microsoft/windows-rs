#[must_use]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ChangeTimerQueueTimer(timerqueue: Option<super::HANDLE>, timer: super::HANDLE, duetime: super::ULONG, period: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ChangeTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE, duetime : super::ULONG, period : super::ULONG) -> windows_core::BOOL);
    unsafe { ChangeTimerQueueTimer(timerqueue.unwrap_or(core::mem::zeroed()) as _, timer as _, duetime, period) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateTimerQueue() -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateTimerQueue() -> super::HANDLE);
    unsafe { CreateTimerQueue() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateTimerQueueTimer(phnewtimer: super::PHANDLE, timerqueue: Option<super::HANDLE>, callback: super::WAITORTIMERCALLBACK, parameter: Option<super::PVOID>, duetime: super::DWORD, period: super::DWORD, flags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CreateTimerQueueTimer(phnewtimer : super::PHANDLE, timerqueue : super::HANDLE, callback : super::WAITORTIMERCALLBACK, parameter : super::PVOID, duetime : super::DWORD, period : super::DWORD, flags : super::ULONG) -> windows_core::BOOL);
    unsafe { CreateTimerQueueTimer(phnewtimer as _, timerqueue.unwrap_or(core::mem::zeroed()) as _, callback, parameter.unwrap_or(core::mem::zeroed()) as _, duetime, period, flags) }
}
#[must_use]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteTimerQueue(timerqueue: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteTimerQueue(timerqueue : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteTimerQueue(timerqueue) }
}
#[must_use]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteTimerQueueEx(timerqueue: super::HANDLE, completionevent: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteTimerQueueEx(timerqueue : super::HANDLE, completionevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteTimerQueueEx(timerqueue, completionevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[must_use]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteTimerQueueTimer(timerqueue: Option<super::HANDLE>, timer: super::HANDLE, completionevent: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE, completionevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteTimerQueueTimer(timerqueue.unwrap_or(core::mem::zeroed()) as _, timer, completionevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueueUserWorkItem(function: super::LPTHREAD_START_ROUTINE, context: Option<super::PVOID>, flags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueueUserWorkItem(function : super::LPTHREAD_START_ROUTINE, context : super::PVOID, flags : super::ULONG) -> windows_core::BOOL);
    unsafe { QueueUserWorkItem(function, context.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[must_use]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnregisterWaitEx(waithandle: super::HANDLE, completionevent: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnregisterWaitEx(waithandle : super::HANDLE, completionevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { UnregisterWaitEx(waithandle, completionevent.unwrap_or(core::mem::zeroed()) as _) }
}
