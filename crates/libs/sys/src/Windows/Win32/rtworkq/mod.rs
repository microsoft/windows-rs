#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqAddPeriodicCallback(callback : RTWQPERIODICCALLBACK, context : *mut core::ffi::c_void, key : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqAllocateSerialWorkQueue(workqueueidin : super::DWORD, workqueueidout : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqAllocateWorkQueue(workqueuetype : RTWQ_WORKQUEUE_TYPE, workqueueid : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqBeginRegisterWorkQueueWithMMCSS(workqueueid : super::DWORD, usageclass : windows_sys::core::PCWSTR, dwtaskid : super::DWORD, lpriority : super::LONG, donecallback : *mut core::ffi::c_void, donestate : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqBeginUnregisterWorkQueueWithMMCSS(workqueueid : super::DWORD, donecallback : *mut core::ffi::c_void, donestate : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("rtworkq.dll" "system" fn RtwqCancelDeadline(prequest : super::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqCancelWorkItem(key : RTWQWORKITEM_KEY) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqCreateAsyncResult(appobject : *mut core::ffi::c_void, callback : *mut core::ffi::c_void, appstate : *mut core::ffi::c_void, asyncresult : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqEndRegisterWorkQueueWithMMCSS(result : *mut core::ffi::c_void, taskid : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSClass(workqueueid : super::DWORD, usageclass : windows_sys::core::PWSTR, usageclasslength : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSPriority(workqueueid : super::DWORD, priority : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSTaskId(workqueueid : super::DWORD, taskid : *mut super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqInvokeCallback(result : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rtworkq.dll" "system" fn RtwqJoinWorkQueue(workqueueid : super::DWORD, hfile : super::HANDLE, out : *mut super::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqLockPlatform() -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqLockSharedWorkQueue(usageclass : windows_sys::core::PCWSTR, basepriority : super::LONG, taskid : *mut super::DWORD, id : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqLockWorkQueue(workqueueid : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rtworkq.dll" "system" fn RtwqPutWaitingWorkItem(hevent : super::HANDLE, lpriority : super::LONG, result : *mut core::ffi::c_void, key : *mut RTWQWORKITEM_KEY) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqPutWorkItem(dwqueue : super::DWORD, lpriority : super::LONG, result : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformEvents(platformevents : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformWithMMCSS(usageclass : windows_sys::core::PCWSTR, taskid : *mut super::DWORD, lpriority : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqRemovePeriodicCallback(dwkey : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqScheduleWorkItem(result : *mut core::ffi::c_void, timeout : super::INT64, key : *mut RTWQWORKITEM_KEY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rtworkq.dll" "system" fn RtwqSetDeadline(workqueueid : super::DWORD, deadlineinhns : super::LONGLONG, prequest : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rtworkq.dll" "system" fn RtwqSetDeadline2(workqueueid : super::DWORD, deadlineinhns : super::LONGLONG, predeadlineinhns : super::LONGLONG, prequest : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqSetLongRunning(workqueueid : super::DWORD, enable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqShutdown() -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqStartup() -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rtworkq.dll" "system" fn RtwqUnjoinWorkQueue(workqueueid : super::DWORD, hfile : super::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqUnlockPlatform() -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("rtworkq.dll" "system" fn RtwqUnlockWorkQueue(workqueueid : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformEvents(platformevents : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformFromMMCSS() -> windows_sys::core::HRESULT);
pub type RTWQPERIODICCALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type RTWQWORKITEM_KEY = u64;
pub const RTWQ_E_BUFFERTOOSMALL: windows_sys::core::HRESULT = 0xC00D36B1_u32 as _;
pub const RTWQ_E_INVALID_WORKQUEUE: windows_sys::core::HRESULT = 0xC00D36FF_u32 as _;
pub const RTWQ_E_NOT_FOUND: windows_sys::core::HRESULT = 0xC00D36D5_u32 as _;
pub const RTWQ_E_NOT_INITIALIZED: windows_sys::core::HRESULT = 0xC00D36B6_u32 as _;
pub const RTWQ_E_OPERATION_CANCELLED: windows_sys::core::HRESULT = 0xC00D36ED_u32 as _;
pub const RTWQ_E_SHUTDOWN: windows_sys::core::HRESULT = 0xC00D3E85_u32 as _;
pub const RTWQ_E_UNEXPECTED: windows_sys::core::HRESULT = 0xC00D36BB_u32 as _;
pub const RTWQ_MULTITHREADED_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 2;
pub const RTWQ_STANDARD_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 0;
pub const RTWQ_WINDOW_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 1;
pub type RTWQ_WORKQUEUE_TYPE = i32;
