windows_link::link!("rtworkq.dll" "system" fn RtwqAddPeriodicCallback(callback : RTWQPERIODICCALLBACK, context : *mut core::ffi::c_void, key : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqAllocateSerialWorkQueue(workqueueidin : u32, workqueueidout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqAllocateWorkQueue(workqueuetype : RTWQ_WORKQUEUE_TYPE, workqueueid : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqBeginRegisterWorkQueueWithMMCSS(workqueueid : u32, usageclass : windows_sys::core::PCWSTR, dwtaskid : u32, lpriority : i32, donecallback : *mut core::ffi::c_void, donestate : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqBeginUnregisterWorkQueueWithMMCSS(workqueueid : u32, donecallback : *mut core::ffi::c_void, donestate : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("rtworkq.dll" "system" fn RtwqCancelDeadline(prequest : super::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqCancelWorkItem(key : RTWQWORKITEM_KEY) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqCreateAsyncResult(appobject : *mut core::ffi::c_void, callback : *mut core::ffi::c_void, appstate : *mut core::ffi::c_void, asyncresult : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqEndRegisterWorkQueueWithMMCSS(result : *mut core::ffi::c_void, taskid : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSClass(workqueueid : u32, usageclass : windows_sys::core::PWSTR, usageclasslength : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSPriority(workqueueid : u32, priority : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSTaskId(workqueueid : u32, taskid : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqInvokeCallback(result : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("rtworkq.dll" "system" fn RtwqJoinWorkQueue(workqueueid : u32, hfile : super::HANDLE, out : *mut super::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqLockPlatform() -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqLockSharedWorkQueue(usageclass : windows_sys::core::PCWSTR, basepriority : i32, taskid : *mut u32, id : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqLockWorkQueue(workqueueid : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("rtworkq.dll" "system" fn RtwqPutWaitingWorkItem(hevent : super::HANDLE, lpriority : i32, result : *mut core::ffi::c_void, key : *mut RTWQWORKITEM_KEY) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqPutWorkItem(dwqueue : u32, lpriority : i32, result : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformEvents(platformevents : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformWithMMCSS(usageclass : windows_sys::core::PCWSTR, taskid : *mut u32, lpriority : i32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqRemovePeriodicCallback(dwkey : u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqScheduleWorkItem(result : *mut core::ffi::c_void, timeout : i64, key : *mut RTWQWORKITEM_KEY) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("rtworkq.dll" "system" fn RtwqSetDeadline(workqueueid : u32, deadlineinhns : i64, prequest : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("rtworkq.dll" "system" fn RtwqSetDeadline2(workqueueid : u32, deadlineinhns : i64, predeadlineinhns : i64, prequest : *mut super::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqSetLongRunning(workqueueid : u32, enable : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqShutdown() -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqStartup() -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("rtworkq.dll" "system" fn RtwqUnjoinWorkQueue(workqueueid : u32, hfile : super::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqUnlockPlatform() -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqUnlockWorkQueue(workqueueid : u32) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformEvents(platformevents : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformFromMMCSS() -> windows_sys::core::HRESULT);
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RTWQASYNCRESULT {
    pub Base: *mut core::ffi::c_void,
    pub overlapped: super::OVERLAPPED,
    pub pCallback: *mut core::ffi::c_void,
    pub hrStatusResult: windows_sys::core::HRESULT,
    pub dwBytesTransferred: u32,
    pub hEvent: super::HANDLE,
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
impl Default for RTWQASYNCRESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RTWQPERIODICCALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type RTWQWORKITEM_KEY = u64;
pub const RTWQ_E_BUFFERTOOSMALL: i32 = -1072875855;
pub const RTWQ_E_INVALID_WORKQUEUE: i32 = -1072875777;
pub const RTWQ_E_NOT_FOUND: i32 = -1072875819;
pub const RTWQ_E_NOT_INITIALIZED: i32 = -1072875850;
pub const RTWQ_E_OPERATION_CANCELLED: i32 = -1072875795;
pub const RTWQ_E_SHUTDOWN: i32 = -1072873851;
pub const RTWQ_E_UNEXPECTED: i32 = -1072875845;
pub const RTWQ_MULTITHREADED_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 2;
pub const RTWQ_STANDARD_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 0;
pub const RTWQ_WINDOW_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = 1;
pub type RTWQ_WORKQUEUE_TYPE = i32;
