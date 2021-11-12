#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn AcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn AcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor: *mut super::super::Foundation::HANDLE, integritylabel: super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSIDToBoundaryDescriptor(boundarydescriptor: *mut super::super::Foundation::HANDLE, requiredsid: super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AttachThreadInput(idattach: u32, idattachto: u32, fattach: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallbackMayRunLong(pci: *mut TP_CALLBACK_INSTANCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CancelThreadpoolIo(pio: *mut TP_IO);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelWaitableTimer(htimer: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClosePrivateNamespace(handle: NamespaceHandle, flags: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpool(ptpp: PTP_POOL);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolCleanupGroup(ptpcg: isize);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseThreadpoolCleanupGroupMembers(ptpcg: isize, fcancelpendingcallbacks: super::super::Foundation::BOOL, pvcleanupcontext: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolIo(pio: *mut TP_IO);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolTimer(pti: *mut TP_TIMER);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolWait(pwa: *mut TP_WAIT);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolWork(pwk: *mut TP_WORK);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertFiberToThread() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ConvertThreadToFiber(lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ConvertThreadToFiberEx(lpparameter: *const ::core::ffi::c_void, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateBoundaryDescriptorA(name: super::super::Foundation::PSTR, flags: u32) -> BoundaryDescriptorHandle;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateBoundaryDescriptorW(name: super::super::Foundation::PWSTR, flags: u32) -> BoundaryDescriptorHandle;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventA(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventExA(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventExW(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventW(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateFiber(dwstacksize: usize, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateFiberEx(dwstackcommitsize: usize, dwstackreservesize: usize, dwflags: u32, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexA(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexExA(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexExW(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexW(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreatePrivateNamespaceA(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PSTR) -> NamespaceHandle;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreatePrivateNamespaceW(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PWSTR) -> NamespaceHandle;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateProcessA(
        lpapplicationname: super::super::Foundation::PSTR,
        lpcommandline: super::super::Foundation::PSTR,
        lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        binherithandles: super::super::Foundation::BOOL,
        dwcreationflags: PROCESS_CREATION_FLAGS,
        lpenvironment: *const ::core::ffi::c_void,
        lpcurrentdirectory: super::super::Foundation::PSTR,
        lpstartupinfo: *const STARTUPINFOA,
        lpprocessinformation: *mut PROCESS_INFORMATION,
    ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateProcessAsUserA(
        htoken: super::super::Foundation::HANDLE,
        lpapplicationname: super::super::Foundation::PSTR,
        lpcommandline: super::super::Foundation::PSTR,
        lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        binherithandles: super::super::Foundation::BOOL,
        dwcreationflags: u32,
        lpenvironment: *const ::core::ffi::c_void,
        lpcurrentdirectory: super::super::Foundation::PSTR,
        lpstartupinfo: *const STARTUPINFOA,
        lpprocessinformation: *mut PROCESS_INFORMATION,
    ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateProcessAsUserW(
        htoken: super::super::Foundation::HANDLE,
        lpapplicationname: super::super::Foundation::PWSTR,
        lpcommandline: super::super::Foundation::PWSTR,
        lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        binherithandles: super::super::Foundation::BOOL,
        dwcreationflags: u32,
        lpenvironment: *const ::core::ffi::c_void,
        lpcurrentdirectory: super::super::Foundation::PWSTR,
        lpstartupinfo: *const STARTUPINFOW,
        lpprocessinformation: *mut PROCESS_INFORMATION,
    ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateProcessW(
        lpapplicationname: super::super::Foundation::PWSTR,
        lpcommandline: super::super::Foundation::PWSTR,
        lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
        binherithandles: super::super::Foundation::BOOL,
        dwcreationflags: PROCESS_CREATION_FLAGS,
        lpenvironment: *const ::core::ffi::c_void,
        lpcurrentdirectory: super::super::Foundation::PWSTR,
        lpstartupinfo: *const STARTUPINFOW,
        lpprocessinformation: *mut PROCESS_INFORMATION,
    ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateProcessWithLogonW(
        lpusername: super::super::Foundation::PWSTR,
        lpdomain: super::super::Foundation::PWSTR,
        lppassword: super::super::Foundation::PWSTR,
        dwlogonflags: CREATE_PROCESS_LOGON_FLAGS,
        lpapplicationname: super::super::Foundation::PWSTR,
        lpcommandline: super::super::Foundation::PWSTR,
        dwcreationflags: u32,
        lpenvironment: *const ::core::ffi::c_void,
        lpcurrentdirectory: super::super::Foundation::PWSTR,
        lpstartupinfo: *const STARTUPINFOW,
        lpprocessinformation: *mut PROCESS_INFORMATION,
    ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateProcessWithTokenW(htoken: super::super::Foundation::HANDLE, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: super::super::Foundation::PWSTR, lpcommandline: super::super::Foundation::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: super::super::Foundation::PWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateRemoteThread(hprocess: super::super::Foundation::HANDLE, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateRemoteThreadEx(hprocess: super::super::Foundation::HANDLE, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreA(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreExA(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreExW(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreW(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateThread(lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: THREAD_CREATION_FLAGS, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpool(reserved: *mut ::core::ffi::c_void) -> PTP_POOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpoolCleanupGroup() -> isize;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateThreadpoolIo(fl: super::super::Foundation::HANDLE, pfnio: PTP_WIN32_IO_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_IO;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpoolTimer(pfnti: PTP_TIMER_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_TIMER;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpoolWait(pfnwa: PTP_WAIT_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WAIT;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpoolWork(pfnwk: PTP_WORK_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WORK;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTimerQueue() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTimerQueueTimer(phnewtimer: *mut super::super::Foundation::HANDLE, timerqueue: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUmsCompletionList(umscompletionlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUmsThreadContext(lpumsthread: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerExW(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerW(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn DeleteBoundaryDescriptor(boundarydescriptor: BoundaryDescriptorHandle);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn DeleteCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn DeleteFiber(lpfiber: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn DeleteProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueue(timerqueue: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueueEx(timerqueue: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUmsCompletionList(umscompletionlist: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUmsThreadContext(umsthread: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DequeueUmsCompletionListItems(umscompletionlist: *const ::core::ffi::c_void, waittimeout: u32, umsthreadlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn DisassociateCurrentThreadFromCallback(pci: *mut TP_CALLBACK_INSTANCE);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn EnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnterSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub fn EnterUmsSchedulingMode(schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteUmsThread(umsthread: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ExitProcess(uexitcode: u32);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ExitThread(dwexitcode: u32);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn FlsAlloc(lpcallback: PFLS_CALLBACK_FUNCTION) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlsFree(dwflsindex: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn FlsGetValue(dwflsindex: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlsSetValue(dwflsindex: u32, lpflsdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn FlushProcessWriteBuffers();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibraryWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, r#mod: super::super::Foundation::HINSTANCE);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetActiveProcessorCount(groupnumber: u16) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetActiveProcessorGroupCount() -> u16;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentProcess() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentProcessId() -> u32;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentProcessorNumber() -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn GetCurrentProcessorNumberEx(procnumber: *mut super::Kernel::PROCESSOR_NUMBER);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentThread() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentThreadId() -> u32;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentUmsThread() -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExitCodeProcess(hprocess: super::super::Foundation::HANDLE, lpexitcode: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExitCodeThread(hthread: super::super::Foundation::HANDLE, lpexitcode: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGuiResources(hprocess: super::super::Foundation::HANDLE, uiflags: GET_GUI_RESOURCES_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetMachineTypeAttributes(machine: u16, machinetypeattributes: *mut MACHINE_ATTRIBUTES) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetMaximumProcessorCount(groupnumber: u16) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetMaximumProcessorGroupCount() -> u16;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetNextUmsListItem(umscontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaAvailableMemoryNode(node: u8, availablebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaAvailableMemoryNodeEx(node: u16, availablebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaHighestNodeNumber(highestnodenumber: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaNodeNumberFromHandle(hfile: super::super::Foundation::HANDLE, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaNodeProcessorMask(node: u8, processormask: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: *mut super::SystemInformation::GROUP_AFFINITY, processormaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetNumaNodeProcessorMaskEx(node: u16, processormask: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProcessorNode(processor: u8, nodenumber: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetNumaProcessorNodeEx(processor: *const super::Kernel::PROCESSOR_NUMBER, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProximityNode(proximityid: u32, nodenumber: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPriorityClass(hprocess: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessAffinityMask(hprocess: super::super::Foundation::HANDLE, lpprocessaffinitymask: *mut usize, lpsystemaffinitymask: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDEPPolicy(hprocess: super::super::Foundation::HANDLE, lpflags: *mut u32, lppermanent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetProcessDefaultCpuSetMasks(process: super::super::Foundation::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDefaultCpuSets(process: super::super::Foundation::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessGroupAffinity(hprocess: super::super::Foundation::HANDLE, groupcount: *mut u16, grouparray: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessHandleCount(hprocess: super::super::Foundation::HANDLE, pdwhandlecount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessId(process: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessIdOfThread(thread: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessInformation(hprocess: super::super::Foundation::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessIoCounters(hprocess: super::super::Foundation::HANDLE, lpiocounters: *mut IO_COUNTERS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessMitigationPolicy(hprocess: super::super::Foundation::HANDLE, mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessPriorityBoost(hprocess: super::super::Foundation::HANDLE, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessShutdownParameters(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessTimes(hprocess: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetProcessVersion(processid: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessWorkingSetSize(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStartupInfoA(lpstartupinfo: *mut STARTUPINFOA);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimes(lpidletime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadDescription(hthread: super::super::Foundation::HANDLE, ppszthreaddescription: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetThreadGroupAffinity(hthread: super::super::Foundation::HANDLE, groupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadIOPendingFlag(hthread: super::super::Foundation::HANDLE, lpioispending: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadId(thread: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetThreadIdealProcessorEx(hthread: super::super::Foundation::HANDLE, lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadInformation(hthread: super::super::Foundation::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPriority(hthread: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPriorityBoost(hthread: super::super::Foundation::HANDLE, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetThreadSelectedCpuSetMasks(thread: super::super::Foundation::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadSelectedCpuSets(thread: super::super::Foundation::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadTimes(hthread: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUmsCompletionListEvent(umscompletionlist: *const ::core::ffi::c_void, umscompletionevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUmsSystemThreadInformation(threadhandle: super::super::Foundation::HANDLE, systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceBeginInitialize(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, fpending: *mut super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceComplete(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, lpcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceExecuteOnce(initonce: *mut RTL_RUN_ONCE, initfn: PINIT_ONCE_FN, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn InitOnceInitialize(initonce: *mut RTL_RUN_ONCE);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn InitializeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSectionEx(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount: u32, dwflags: u32, lpsize: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InitializeSListHead(listhead: *mut super::Kernel::SLIST_HEADER);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn InitializeSRWLock(srwlock: *mut RTL_SRWLOCK);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, ltotalthreads: i32, lspincount: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedFlushSList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPopEntrySList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPushEntrySList(listhead: *mut super::Kernel::SLIST_HEADER, listentry: *mut super::Kernel::SLIST_ENTRY) -> *mut super::Kernel::SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPushListSListEx(listhead: *mut super::Kernel::SLIST_HEADER, list: *mut super::Kernel::SLIST_ENTRY, listend: *mut super::Kernel::SLIST_ENTRY, count: u32) -> *mut super::Kernel::SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsImmersiveProcess(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessCritical(hprocess: super::super::Foundation::HANDLE, critical: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessorFeaturePresent(processorfeature: PROCESSOR_FEATURE_ID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThreadAFiber() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThreadpoolTimerSet(pti: *mut TP_TIMER) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64Process(hprocess: super::super::Foundation::HANDLE, wow64process: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64Process2(hprocess: super::super::Foundation::HANDLE, pprocessmachine: *mut u16, pnativemachine: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LeaveCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LeaveCriticalSectionWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, pcs: *mut RTL_CRITICAL_SECTION);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryInformationProcess(processhandle: super::super::Foundation::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut ::core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryInformationThread(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtSetInformationThread(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenMutexW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPrivateNamespaceA(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PSTR) -> NamespaceHandle;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPrivateNamespaceW(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PWSTR) -> NamespaceHandle;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenProcess(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: super::super::Foundation::BOOL, dwprocessid: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenProcessToken(processhandle: super::super::Foundation::HANDLE, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenSemaphoreW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThread(dwdesiredaccess: THREAD_ACCESS_RIGHTS, binherithandle: super::super::Foundation::BOOL, dwthreadid: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenThreadToken(threadhandle: super::super::Foundation::HANDLE, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, openasself: super::super::Foundation::BOOL, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWaitableTimerW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PulseEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryFullProcessImageNameA(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: super::super::Foundation::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryFullProcessImageNameW(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: super::super::Foundation::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProcessAffinityUpdateMode(hprocess: super::super::Foundation::HANDLE, lpdwflags: *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProtectedPolicy(policyguid: *const ::windows_sys::core::GUID, policyvalue: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadpoolStackInformation(ptpp: PTP_POOL, ptpsi: *mut TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *mut ::core::ffi::c_void, umsthreadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserAPC(pfnapc: super::super::Foundation::PAPCFUNC, hthread: super::super::Foundation::HANDLE, dwdata: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserAPC2(apcroutine: super::super::Foundation::PAPCFUNC, thread: super::super::Foundation::HANDLE, data: usize, flags: QUEUE_USER_APC_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserWorkItem(function: LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterWaitForSingleObject(phnewwaitobject: *mut super::super::Foundation::HANDLE, hobject: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, dwmilliseconds: u32, dwflags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseMutex(hmutex: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseMutexWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, r#mut: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ReleaseSRWLockExclusive(srwlock: *mut RTL_SRWLOCK);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ReleaseSRWLockShared(srwlock: *mut RTL_SRWLOCK);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseSemaphore(hsemaphore: super::super::Foundation::HANDLE, lreleasecount: i32, lppreviouscount: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseSemaphoreWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, sem: super::super::Foundation::HANDLE, crel: u32);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResumeThread(hthread: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetCriticalSectionSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEventWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, evt: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPriorityClass(hprocess: super::super::Foundation::HANDLE, dwpriorityclass: PROCESS_CREATION_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessAffinityMask(hprocess: super::super::Foundation::HANDLE, dwprocessaffinitymask: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessAffinityUpdateMode(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetProcessDefaultCpuSetMasks(process: super::super::Foundation::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDefaultCpuSets(process: super::super::Foundation::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDynamicEHContinuationTargets(process: super::super::Foundation::HANDLE, numberoftargets: u16, targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDynamicEnforcedCetCompatibleRanges(process: super::super::Foundation::HANDLE, numberofranges: u16, ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessInformation(hprocess: super::super::Foundation::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessMitigationPolicy(mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *const ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessPriorityBoost(hprocess: super::super::Foundation::HANDLE, bdisablepriorityboost: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessRestrictionExemption(fenableexemption: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessShutdownParameters(dwlevel: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWorkingSetSize(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProtectedPolicy(policyguid: *const ::windows_sys::core::GUID, policyvalue: usize, oldpolicyvalue: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadAffinityMask(hthread: super::super::Foundation::HANDLE, dwthreadaffinitymask: usize) -> usize;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadDescription(hthread: super::super::Foundation::HANDLE, lpthreaddescription: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetThreadGroupAffinity(hthread: super::super::Foundation::HANDLE, groupaffinity: *const super::SystemInformation::GROUP_AFFINITY, previousgroupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadIdealProcessor(hthread: super::super::Foundation::HANDLE, dwidealprocessor: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetThreadIdealProcessorEx(hthread: super::super::Foundation::HANDLE, lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER, lppreviousidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadInformation(hthread: super::super::Foundation::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPriority(hthread: super::super::Foundation::HANDLE, npriority: THREAD_PRIORITY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPriorityBoost(hthread: super::super::Foundation::HANDLE, bdisablepriorityboost: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetThreadSelectedCpuSetMasks(thread: super::super::Foundation::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadSelectedCpuSets(thread: super::super::Foundation::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadStackGuarantee(stacksizeinbytes: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadToken(thread: *const super::super::Foundation::HANDLE, token: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolStackInformation(ptpp: PTP_POOL, ptpsi: *const TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn SetThreadpoolThreadMaximum(ptpp: PTP_POOL, cthrdmost: u32);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolThreadMinimum(ptpp: PTP_POOL, cthrdmic: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolTimer(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolTimerEx(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolWait(pwa: *mut TP_WAIT, h: super::super::Foundation::HANDLE, pfttimeout: *const super::super::Foundation::FILETIME);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolWaitEx(pwa: *mut TP_WAIT, h: super::super::Foundation::HANDLE, pfttimeout: *const super::super::Foundation::FILETIME, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, preferio: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *const ::core::ffi::c_void, umsthreadinformationlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWaitableTimer(htimer: super::super::Foundation::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, fresume: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWaitableTimerEx(htimer: super::super::Foundation::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, wakecontext: *const REASON_CONTEXT, tolerabledelay: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn Sleep(dwmilliseconds: u32);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SleepConditionVariableCS(conditionvariable: *mut RTL_CONDITION_VARIABLE, criticalsection: *mut RTL_CRITICAL_SECTION, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SleepConditionVariableSRW(conditionvariable: *mut RTL_CONDITION_VARIABLE, srwlock: *mut RTL_SRWLOCK, dwmilliseconds: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SleepEx(dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn StartThreadpoolIo(pio: *mut TP_IO);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn SubmitThreadpoolWork(pwk: *mut TP_WORK);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SuspendThread(hthread: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn SwitchToFiber(lpfiber: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwitchToThread() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateProcess(hprocess: super::super::Foundation::HANDLE, uexitcode: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateThread(hthread: super::super::Foundation::HANDLE, dwexitcode: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn TlsAlloc() -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TlsFree(dwtlsindex: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn TlsGetValue(dwtlsindex: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TlsSetValue(dwtlsindex: u32, lptlsvalue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryAcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryAcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn TryEnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrySubmitThreadpoolCallback(pfns: PTP_SIMPLE_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UmsThreadYield(schedulerparam: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterWait(waithandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterWaitEx(waithandle: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateProcThreadAttribute(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwflags: u32, attribute: usize, lpvalue: *const ::core::ffi::c_void, cbsize: usize, lppreviousvalue: *mut ::core::ffi::c_void, lpreturnsize: *const usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForInputIdle(hprocess: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForMultipleObjects(ncount: u32, lphandles: *const super::super::Foundation::HANDLE, bwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForMultipleObjectsEx(ncount: u32, lphandles: *const super::super::Foundation::HANDLE, bwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForSingleObject(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForSingleObjectEx(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolIoCallbacks(pio: *mut TP_IO, fcancelpendingcallbacks: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolTimerCallbacks(pti: *mut TP_TIMER, fcancelpendingcallbacks: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolWaitCallbacks(pwa: *mut TP_WAIT, fcancelpendingcallbacks: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolWorkCallbacks(pwk: *mut TP_WORK, fcancelpendingcallbacks: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitOnAddress(address: *const ::core::ffi::c_void, compareaddress: *const ::core::ffi::c_void, addresssize: usize, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn WakeAllConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn WakeByAddressAll(address: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn WakeByAddressSingle(address: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn WakeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinExec(lpcmdline: super::super::Foundation::PSTR, ucmdshow: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16;
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64SuspendThread(hthread: super::super::Foundation::HANDLE) -> u32;
}
pub struct APP_MEMORY_INFORMATION(i32);
pub struct BoundaryDescriptorHandle(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
pub struct CREATE_EVENT(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
pub struct CREATE_PROCESS_LOGON_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
pub struct GET_GUI_RESOURCES_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const INIT_ONCE_ASYNC: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
pub struct IO_COUNTERS(i32);
pub struct LPFIBER_START_ROUTINE(i32);
pub struct LPPROC_THREAD_ATTRIBUTE_LIST(i32);
pub struct LPTHREAD_START_ROUTINE(i32);
pub struct MACHINE_ATTRIBUTES(i32);
pub struct MEMORY_PRIORITY(i32);
pub struct MEMORY_PRIORITY_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const MUTEX_MODIFY_STATE: u32 = 1u32;
pub struct NamespaceHandle(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PEB(i32);
#[cfg(feature = "Win32_System_Kernel")]
pub struct PEB_LDR_DATA(i32);
pub struct PFLS_CALLBACK_FUNCTION(i32);
pub struct PINIT_ONCE_FN(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PME_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
pub struct POWER_REQUEST_CONTEXT_FLAGS(i32);
pub struct PPS_POST_PROCESS_INIT_ROUTINE(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
pub struct PROCESSINFOCLASS(i32);
pub struct PROCESSOR_FEATURE_ID(i32);
pub struct PROCESS_ACCESS_RIGHTS(i32);
pub struct PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PROCESS_BASIC_INFORMATION(i32);
pub struct PROCESS_CREATION_FLAGS(i32);
pub struct PROCESS_DEP_FLAGS(i32);
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET(i32);
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION(i32);
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE(i32);
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_INFORMATION(i32);
pub struct PROCESS_INFORMATION_CLASS(i32);
pub struct PROCESS_LEAP_SECOND_INFO(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
pub struct PROCESS_MACHINE_INFORMATION(i32);
pub struct PROCESS_MEMORY_EXHAUSTION_INFO(i32);
pub struct PROCESS_MEMORY_EXHAUSTION_TYPE(i32);
pub struct PROCESS_MITIGATION_POLICY(i32);
pub struct PROCESS_NAME_FORMAT(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
pub struct PROCESS_POWER_THROTTLING_STATE(i32);
pub struct PROCESS_PROTECTION_LEVEL(i32);
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
pub struct PRTL_UMS_SCHEDULER_ENTRY_POINT(i32);
pub struct PTIMERAPCROUTINE(i32);
pub struct PTP_CLEANUP_GROUP_CANCEL_CALLBACK(i32);
pub struct PTP_POOL(i32);
pub struct PTP_SIMPLE_CALLBACK(i32);
pub struct PTP_TIMER_CALLBACK(i32);
pub struct PTP_WAIT_CALLBACK(i32);
pub struct PTP_WIN32_IO_CALLBACK(i32);
pub struct PTP_WORK_CALLBACK(i32);
pub struct QUEUE_USER_APC_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT(i32);
pub struct RTL_BARRIER(i32);
pub struct RTL_CONDITION_VARIABLE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION_DEBUG(i32);
pub struct RTL_RUN_ONCE(i32);
pub struct RTL_SRWLOCK(i32);
pub struct RTL_UMS_THREAD_INFO_CLASS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_USER_PROCESS_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXW(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOW(i32);
pub struct STARTUPINFOW_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
pub struct THREADINFOCLASS(i32);
pub struct THREAD_ACCESS_RIGHTS(i32);
pub struct THREAD_CREATION_FLAGS(i32);
pub struct THREAD_INFORMATION_CLASS(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub struct THREAD_POWER_THROTTLING_STATE(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
pub struct THREAD_PRIORITY(i32);
pub struct TP_CALLBACK_ENVIRON_V3(i32);
pub struct TP_CALLBACK_INSTANCE(i32);
pub struct TP_CALLBACK_PRIORITY(i32);
pub struct TP_IO(i32);
pub struct TP_POOL_STACK_INFORMATION(i32);
pub struct TP_TIMER(i32);
pub struct TP_WAIT(i32);
pub struct TP_WORK(i32);
pub struct TimerQueueHandle(i32);
#[cfg(feature = "Win32_System_SystemServices")]
pub struct UMS_SCHEDULER_STARTUP_INFO(i32);
pub struct UMS_SYSTEM_THREAD_INFORMATION(i32);
pub struct WAITORTIMERCALLBACK(i32);
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const WAIT_ABANDONED: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const WAIT_ABANDONED_0: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const WAIT_IO_COMPLETION: u32 = 192u32;
#[doc = "*Required features: `Win32_System_Threading`*"]
pub const WAIT_OBJECT_0: u32 = 0u32;
pub struct WORKER_THREAD_FLAGS(i32);
