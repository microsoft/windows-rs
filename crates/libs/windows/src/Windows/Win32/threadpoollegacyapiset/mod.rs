#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ChangeTimerQueueTimer(timerqueue: Option<super::HANDLE>, timer: super::HANDLE, duetime: u32, period: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ChangeTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE, duetime : u32, period : u32) -> windows_core::BOOL);
    unsafe { ChangeTimerQueueTimer(timerqueue.unwrap_or(core::mem::zeroed()) as _, timer as _, duetime, period) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateTimerQueue() -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateTimerQueue() -> super::HANDLE);
    unsafe { CreateTimerQueue() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateTimerQueueTimer(phnewtimer: *mut super::HANDLE, timerqueue: Option<super::HANDLE>, callback: super::WAITORTIMERCALLBACK, parameter: Option<*const core::ffi::c_void>, duetime: u32, period: u32, flags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CreateTimerQueueTimer(phnewtimer : *mut super::HANDLE, timerqueue : super::HANDLE, callback : super::WAITORTIMERCALLBACK, parameter : *const core::ffi::c_void, duetime : u32, period : u32, flags : u32) -> windows_core::BOOL);
    unsafe { CreateTimerQueueTimer(phnewtimer as _, timerqueue.unwrap_or(core::mem::zeroed()) as _, callback, parameter.unwrap_or(core::mem::zeroed()) as _, duetime, period, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteTimerQueue(timerqueue: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteTimerQueue(timerqueue : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteTimerQueue(timerqueue) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteTimerQueueEx(timerqueue: super::HANDLE, completionevent: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteTimerQueueEx(timerqueue : super::HANDLE, completionevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteTimerQueueEx(timerqueue, completionevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteTimerQueueTimer(timerqueue: Option<super::HANDLE>, timer: super::HANDLE, completionevent: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE, completionevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteTimerQueueTimer(timerqueue.unwrap_or(core::mem::zeroed()) as _, timer, completionevent.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn QueueUserWorkItem(function: super::LPTHREAD_START_ROUTINE, context: Option<*const core::ffi::c_void>, flags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueueUserWorkItem(function : super::LPTHREAD_START_ROUTINE, context : *const core::ffi::c_void, flags : u32) -> windows_core::BOOL);
    unsafe { QueueUserWorkItem(function, context.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnregisterWaitEx(waithandle: super::HANDLE, completionevent: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnregisterWaitEx(waithandle : super::HANDLE, completionevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { UnregisterWaitEx(waithandle, completionevent.unwrap_or(core::mem::zeroed()) as _) }
}
