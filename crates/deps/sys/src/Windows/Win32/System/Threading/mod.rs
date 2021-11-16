#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn AcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK);
    pub fn AcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK);
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor: *mut super::super::Foundation::HANDLE, integritylabel: super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSIDToBoundaryDescriptor(boundarydescriptor: *mut super::super::Foundation::HANDLE, requiredsid: super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AttachThreadInput(idattach: u32, idattachto: u32, fattach: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallbackMayRunLong(pci: *mut TP_CALLBACK_INSTANCE) -> super::super::Foundation::BOOL;
    pub fn CancelThreadpoolIo(pio: *mut TP_IO);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelWaitableTimer(htimer: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClosePrivateNamespace(handle: NamespaceHandle, flags: u32) -> super::super::Foundation::BOOLEAN;
    pub fn CloseThreadpool(ptpp: PTP_POOL);
    pub fn CloseThreadpoolCleanupGroup(ptpcg: isize);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseThreadpoolCleanupGroupMembers(ptpcg: isize, fcancelpendingcallbacks: super::super::Foundation::BOOL, pvcleanupcontext: *mut ::core::ffi::c_void);
    pub fn CloseThreadpoolIo(pio: *mut TP_IO);
    pub fn CloseThreadpoolTimer(pti: *mut TP_TIMER);
    pub fn CloseThreadpoolWait(pwa: *mut TP_WAIT);
    pub fn CloseThreadpoolWork(pwk: *mut TP_WORK);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertFiberToThread() -> super::super::Foundation::BOOL;
    pub fn ConvertThreadToFiber(lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn ConvertThreadToFiberEx(lpparameter: *const ::core::ffi::c_void, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateBoundaryDescriptorA(name: super::super::Foundation::PSTR, flags: u32) -> BoundaryDescriptorHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateBoundaryDescriptorW(name: super::super::Foundation::PWSTR, flags: u32) -> BoundaryDescriptorHandle;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventA(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventExA(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventExW(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventW(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    pub fn CreateFiber(dwstacksize: usize, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn CreateFiberEx(dwstackcommitsize: usize, dwstackreservesize: usize, dwflags: u32, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexA(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexExA(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexExW(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexW(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreatePrivateNamespaceA(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PSTR) -> NamespaceHandle;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreatePrivateNamespaceW(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PWSTR) -> NamespaceHandle;
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
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateProcessWithTokenW(htoken: super::super::Foundation::HANDLE, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: super::super::Foundation::PWSTR, lpcommandline: super::super::Foundation::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: super::super::Foundation::PWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateRemoteThread(hprocess: super::super::Foundation::HANDLE, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateRemoteThreadEx(hprocess: super::super::Foundation::HANDLE, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreA(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreExA(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreExW(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreW(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateThread(lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: THREAD_CREATION_FLAGS, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
    pub fn CreateThreadpool(reserved: *mut ::core::ffi::c_void) -> PTP_POOL;
    pub fn CreateThreadpoolCleanupGroup() -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateThreadpoolIo(fl: super::super::Foundation::HANDLE, pfnio: PTP_WIN32_IO_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_IO;
    pub fn CreateThreadpoolTimer(pfnti: PTP_TIMER_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_TIMER;
    pub fn CreateThreadpoolWait(pfnwa: PTP_WAIT_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WAIT;
    pub fn CreateThreadpoolWork(pfnwk: PTP_WORK_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WORK;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTimerQueue() -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTimerQueueTimer(phnewtimer: *mut super::super::Foundation::HANDLE, timerqueue: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUmsCompletionList(umscompletionlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUmsThreadContext(lpumsthread: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerExW(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerW(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    pub fn DeleteBoundaryDescriptor(boundarydescriptor: BoundaryDescriptorHandle);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn DeleteCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    pub fn DeleteFiber(lpfiber: *const ::core::ffi::c_void);
    pub fn DeleteProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueue(timerqueue: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueueEx(timerqueue: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUmsCompletionList(umscompletionlist: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUmsThreadContext(umsthread: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DequeueUmsCompletionListItems(umscompletionlist: *const ::core::ffi::c_void, waittimeout: u32, umsthreadlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn DisassociateCurrentThreadFromCallback(pci: *mut TP_CALLBACK_INSTANCE);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn EnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnterSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub fn EnterUmsSchedulingMode(schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteUmsThread(umsthread: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn ExitProcess(uexitcode: u32);
    pub fn ExitThread(dwexitcode: u32);
    pub fn FlsAlloc(lpcallback: PFLS_CALLBACK_FUNCTION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlsFree(dwflsindex: u32) -> super::super::Foundation::BOOL;
    pub fn FlsGetValue(dwflsindex: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlsSetValue(dwflsindex: u32, lpflsdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn FlushProcessWriteBuffers();
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibraryWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, r#mod: super::super::Foundation::HINSTANCE);
    pub fn GetActiveProcessorCount(groupnumber: u16) -> u32;
    pub fn GetActiveProcessorGroupCount() -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentProcess() -> super::super::Foundation::HANDLE;
    pub fn GetCurrentProcessId() -> u32;
    pub fn GetCurrentProcessorNumber() -> u32;
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn GetCurrentProcessorNumberEx(procnumber: *mut super::Kernel::PROCESSOR_NUMBER);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentThread() -> super::super::Foundation::HANDLE;
    pub fn GetCurrentThreadId() -> u32;
    pub fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize);
    pub fn GetCurrentUmsThread() -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExitCodeProcess(hprocess: super::super::Foundation::HANDLE, lpexitcode: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExitCodeThread(hthread: super::super::Foundation::HANDLE, lpexitcode: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGuiResources(hprocess: super::super::Foundation::HANDLE, uiflags: GET_GUI_RESOURCES_FLAGS) -> u32;
    pub fn GetMachineTypeAttributes(machine: u16, machinetypeattributes: *mut MACHINE_ATTRIBUTES) -> ::windows_sys::core::HRESULT;
    pub fn GetMaximumProcessorCount(groupnumber: u16) -> u32;
    pub fn GetMaximumProcessorGroupCount() -> u16;
    pub fn GetNextUmsListItem(umscontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaAvailableMemoryNode(node: u8, availablebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaAvailableMemoryNodeEx(node: u16, availablebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaHighestNodeNumber(highestnodenumber: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaNodeNumberFromHandle(hfile: super::super::Foundation::HANDLE, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaNodeProcessorMask(node: u8, processormask: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: *mut super::SystemInformation::GROUP_AFFINITY, processormaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetNumaNodeProcessorMaskEx(node: u16, processormask: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProcessorNode(processor: u8, nodenumber: *mut u8) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetNumaProcessorNodeEx(processor: *const super::Kernel::PROCESSOR_NUMBER, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProximityNode(proximityid: u32, nodenumber: *mut u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPriorityClass(hprocess: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessAffinityMask(hprocess: super::super::Foundation::HANDLE, lpprocessaffinitymask: *mut usize, lpsystemaffinitymask: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDEPPolicy(hprocess: super::super::Foundation::HANDLE, lpflags: *mut u32, lppermanent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetProcessDefaultCpuSetMasks(process: super::super::Foundation::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDefaultCpuSets(process: super::super::Foundation::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessGroupAffinity(hprocess: super::super::Foundation::HANDLE, groupcount: *mut u16, grouparray: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessHandleCount(hprocess: super::super::Foundation::HANDLE, pdwhandlecount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessId(process: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessIdOfThread(thread: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessInformation(hprocess: super::super::Foundation::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessIoCounters(hprocess: super::super::Foundation::HANDLE, lpiocounters: *mut IO_COUNTERS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessMitigationPolicy(hprocess: super::super::Foundation::HANDLE, mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessPriorityBoost(hprocess: super::super::Foundation::HANDLE, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessShutdownParameters(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessTimes(hprocess: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    pub fn GetProcessVersion(processid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessWorkingSetSize(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStartupInfoA(lpstartupinfo: *mut STARTUPINFOA);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimes(lpidletime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadDescription(hthread: super::super::Foundation::HANDLE, ppszthreaddescription: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetThreadGroupAffinity(hthread: super::super::Foundation::HANDLE, groupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadIOPendingFlag(hthread: super::super::Foundation::HANDLE, lpioispending: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadId(thread: super::super::Foundation::HANDLE) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetThreadIdealProcessorEx(hthread: super::super::Foundation::HANDLE, lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadInformation(hthread: super::super::Foundation::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPriority(hthread: super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPriorityBoost(hthread: super::super::Foundation::HANDLE, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetThreadSelectedCpuSetMasks(thread: super::super::Foundation::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadSelectedCpuSets(thread: super::super::Foundation::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadTimes(hthread: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUmsCompletionListEvent(umscompletionlist: *const ::core::ffi::c_void, umscompletionevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUmsSystemThreadInformation(threadhandle: super::super::Foundation::HANDLE, systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceBeginInitialize(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, fpending: *mut super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceComplete(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, lpcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceExecuteOnce(initonce: *mut RTL_RUN_ONCE, initfn: PINIT_ONCE_FN, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    pub fn InitOnceInitialize(initonce: *mut RTL_RUN_ONCE);
    pub fn InitializeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSectionEx(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount: u32, dwflags: u32, lpsize: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InitializeSListHead(listhead: *mut super::Kernel::SLIST_HEADER);
    pub fn InitializeSRWLock(srwlock: *mut RTL_SRWLOCK);
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, ltotalthreads: i32, lspincount: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedFlushSList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPopEntrySList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPushEntrySList(listhead: *mut super::Kernel::SLIST_HEADER, listentry: *mut super::Kernel::SLIST_ENTRY) -> *mut super::Kernel::SLIST_ENTRY;
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPushListSListEx(listhead: *mut super::Kernel::SLIST_HEADER, list: *mut super::Kernel::SLIST_ENTRY, listend: *mut super::Kernel::SLIST_ENTRY, count: u32) -> *mut super::Kernel::SLIST_ENTRY;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsImmersiveProcess(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessCritical(hprocess: super::super::Foundation::HANDLE, critical: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessorFeaturePresent(processorfeature: PROCESSOR_FEATURE_ID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThreadAFiber() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThreadpoolTimerSet(pti: *mut TP_TIMER) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64Process(hprocess: super::super::Foundation::HANDLE, wow64process: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64Process2(hprocess: super::super::Foundation::HANDLE, pprocessmachine: *mut u16, pnativemachine: *mut u16) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LeaveCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LeaveCriticalSectionWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, pcs: *mut RTL_CRITICAL_SECTION);
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryInformationProcess(processhandle: super::super::Foundation::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut ::core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryInformationThread(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtSetInformationThread(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> super::super::Foundation::NTSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenMutexW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPrivateNamespaceA(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PSTR) -> NamespaceHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPrivateNamespaceW(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: super::super::Foundation::PWSTR) -> NamespaceHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenProcess(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: super::super::Foundation::BOOL, dwprocessid: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenProcessToken(processhandle: super::super::Foundation::HANDLE, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenSemaphoreW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThread(dwdesiredaccess: THREAD_ACCESS_RIGHTS, binherithandle: super::super::Foundation::BOOL, dwthreadid: u32) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenThreadToken(threadhandle: super::super::Foundation::HANDLE, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, openasself: super::super::Foundation::BOOL, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWaitableTimerW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PulseEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryFullProcessImageNameA(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: super::super::Foundation::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryFullProcessImageNameW(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: super::super::Foundation::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProcessAffinityUpdateMode(hprocess: super::super::Foundation::HANDLE, lpdwflags: *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProtectedPolicy(policyguid: *const ::windows_sys::core::GUID, policyvalue: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadpoolStackInformation(ptpp: PTP_POOL, ptpsi: *mut TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *mut ::core::ffi::c_void, umsthreadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserAPC(pfnapc: super::super::Foundation::PAPCFUNC, hthread: super::super::Foundation::HANDLE, dwdata: usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserAPC2(apcroutine: super::super::Foundation::PAPCFUNC, thread: super::super::Foundation::HANDLE, data: usize, flags: QUEUE_USER_APC_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserWorkItem(function: LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterWaitForSingleObject(phnewwaitobject: *mut super::super::Foundation::HANDLE, hobject: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, dwmilliseconds: u32, dwflags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseMutex(hmutex: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseMutexWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, r#mut: super::super::Foundation::HANDLE);
    pub fn ReleaseSRWLockExclusive(srwlock: *mut RTL_SRWLOCK);
    pub fn ReleaseSRWLockShared(srwlock: *mut RTL_SRWLOCK);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseSemaphore(hsemaphore: super::super::Foundation::HANDLE, lreleasecount: i32, lppreviouscount: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseSemaphoreWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, sem: super::super::Foundation::HANDLE, crel: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResumeThread(hthread: super::super::Foundation::HANDLE) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetCriticalSectionSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEventWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, evt: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPriorityClass(hprocess: super::super::Foundation::HANDLE, dwpriorityclass: PROCESS_CREATION_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessAffinityMask(hprocess: super::super::Foundation::HANDLE, dwprocessaffinitymask: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessAffinityUpdateMode(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetProcessDefaultCpuSetMasks(process: super::super::Foundation::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDefaultCpuSets(process: super::super::Foundation::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDynamicEHContinuationTargets(process: super::super::Foundation::HANDLE, numberoftargets: u16, targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDynamicEnforcedCetCompatibleRanges(process: super::super::Foundation::HANDLE, numberofranges: u16, ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessInformation(hprocess: super::super::Foundation::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessMitigationPolicy(mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *const ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessPriorityBoost(hprocess: super::super::Foundation::HANDLE, bdisablepriorityboost: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessRestrictionExemption(fenableexemption: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessShutdownParameters(dwlevel: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWorkingSetSize(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProtectedPolicy(policyguid: *const ::windows_sys::core::GUID, policyvalue: usize, oldpolicyvalue: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadAffinityMask(hthread: super::super::Foundation::HANDLE, dwthreadaffinitymask: usize) -> usize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadDescription(hthread: super::super::Foundation::HANDLE, lpthreaddescription: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetThreadGroupAffinity(hthread: super::super::Foundation::HANDLE, groupaffinity: *const super::SystemInformation::GROUP_AFFINITY, previousgroupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadIdealProcessor(hthread: super::super::Foundation::HANDLE, dwidealprocessor: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetThreadIdealProcessorEx(hthread: super::super::Foundation::HANDLE, lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER, lppreviousidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadInformation(hthread: super::super::Foundation::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPriority(hthread: super::super::Foundation::HANDLE, npriority: THREAD_PRIORITY) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPriorityBoost(hthread: super::super::Foundation::HANDLE, bdisablepriorityboost: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetThreadSelectedCpuSetMasks(thread: super::super::Foundation::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadSelectedCpuSets(thread: super::super::Foundation::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadStackGuarantee(stacksizeinbytes: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadToken(thread: *const super::super::Foundation::HANDLE, token: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolStackInformation(ptpp: PTP_POOL, ptpsi: *const TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL;
    pub fn SetThreadpoolThreadMaximum(ptpp: PTP_POOL, cthrdmost: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolThreadMinimum(ptpp: PTP_POOL, cthrdmic: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolTimer(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolTimerEx(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolWait(pwa: *mut TP_WAIT, h: super::super::Foundation::HANDLE, pfttimeout: *const super::super::Foundation::FILETIME);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolWaitEx(pwa: *mut TP_WAIT, h: super::super::Foundation::HANDLE, pfttimeout: *const super::super::Foundation::FILETIME, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, preferio: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *const ::core::ffi::c_void, umsthreadinformationlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWaitableTimer(htimer: super::super::Foundation::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, fresume: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWaitableTimerEx(htimer: super::super::Foundation::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, wakecontext: *const REASON_CONTEXT, tolerabledelay: u32) -> super::super::Foundation::BOOL;
    pub fn Sleep(dwmilliseconds: u32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SleepConditionVariableCS(conditionvariable: *mut RTL_CONDITION_VARIABLE, criticalsection: *mut RTL_CRITICAL_SECTION, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SleepConditionVariableSRW(conditionvariable: *mut RTL_CONDITION_VARIABLE, srwlock: *mut RTL_SRWLOCK, dwmilliseconds: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SleepEx(dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
    pub fn StartThreadpoolIo(pio: *mut TP_IO);
    pub fn SubmitThreadpoolWork(pwk: *mut TP_WORK);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SuspendThread(hthread: super::super::Foundation::HANDLE) -> u32;
    pub fn SwitchToFiber(lpfiber: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwitchToThread() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateProcess(hprocess: super::super::Foundation::HANDLE, uexitcode: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateThread(hthread: super::super::Foundation::HANDLE, dwexitcode: u32) -> super::super::Foundation::BOOL;
    pub fn TlsAlloc() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TlsFree(dwtlsindex: u32) -> super::super::Foundation::BOOL;
    pub fn TlsGetValue(dwtlsindex: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TlsSetValue(dwtlsindex: u32, lptlsvalue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryAcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryAcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn TryEnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrySubmitThreadpoolCallback(pfns: PTP_SIMPLE_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UmsThreadYield(schedulerparam: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterWait(waithandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterWaitEx(waithandle: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateProcThreadAttribute(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwflags: u32, attribute: usize, lpvalue: *const ::core::ffi::c_void, cbsize: usize, lppreviousvalue: *mut ::core::ffi::c_void, lpreturnsize: *const usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForInputIdle(hprocess: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForMultipleObjects(ncount: u32, lphandles: *const super::super::Foundation::HANDLE, bwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForMultipleObjectsEx(ncount: u32, lphandles: *const super::super::Foundation::HANDLE, bwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForSingleObject(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForSingleObjectEx(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolIoCallbacks(pio: *mut TP_IO, fcancelpendingcallbacks: super::super::Foundation::BOOL);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolTimerCallbacks(pti: *mut TP_TIMER, fcancelpendingcallbacks: super::super::Foundation::BOOL);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolWaitCallbacks(pwa: *mut TP_WAIT, fcancelpendingcallbacks: super::super::Foundation::BOOL);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolWorkCallbacks(pwk: *mut TP_WORK, fcancelpendingcallbacks: super::super::Foundation::BOOL);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitOnAddress(address: *const ::core::ffi::c_void, compareaddress: *const ::core::ffi::c_void, addresssize: usize, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
    pub fn WakeAllConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    pub fn WakeByAddressAll(address: *const ::core::ffi::c_void);
    pub fn WakeByAddressSingle(address: *const ::core::ffi::c_void);
    pub fn WakeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinExec(lpcmdline: super::super::Foundation::PSTR, ucmdshow: u32) -> u32;
    pub fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64SuspendThread(hthread: super::super::Foundation::HANDLE) -> u32;
}
#[repr(C)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
impl ::core::marker::Copy for APP_MEMORY_INFORMATION {}
impl ::core::clone::Clone for APP_MEMORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub type BoundaryDescriptorHandle = isize;
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
pub const CREATE_EVENT_INITIAL_SET: u32 = 2u32;
pub const CREATE_EVENT_MANUAL_RESET: u32 = 1u32;
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
pub const LOGON_WITH_PROFILE: u32 = 1u32;
pub const LOGON_NETCREDENTIALS_ONLY: u32 = 2u32;
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
pub const GR_GDIOBJECTS: u32 = 0u32;
pub const GR_GDIOBJECTS_PEAK: u32 = 2u32;
pub const GR_USEROBJECTS: u32 = 1u32;
pub const GR_USEROBJECTS_PEAK: u32 = 4u32;
pub const INIT_ONCE_ASYNC: u32 = 2u32;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
#[repr(C)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
impl ::core::marker::Copy for IO_COUNTERS {}
impl ::core::clone::Clone for IO_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LPFIBER_START_ROUTINE = unsafe extern "system" fn(lpfiberparameter: *mut ::core::ffi::c_void);
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut ::core::ffi::c_void;
pub type LPTHREAD_START_ROUTINE = unsafe extern "system" fn(lpthreadparameter: *mut ::core::ffi::c_void) -> u32;
pub const UserEnabled: u32 = 1u32;
pub const KernelEnabled: u32 = 2u32;
pub const Wow64Container: u32 = 4u32;
pub const MEMORY_PRIORITY_VERY_LOW: u32 = 1u32;
pub const MEMORY_PRIORITY_LOW: u32 = 2u32;
pub const MEMORY_PRIORITY_MEDIUM: u32 = 3u32;
pub const MEMORY_PRIORITY_BELOW_NORMAL: u32 = 4u32;
pub const MEMORY_PRIORITY_NORMAL: u32 = 5u32;
#[repr(C)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
impl ::core::marker::Copy for MEMORY_PRIORITY_INFORMATION {}
impl ::core::clone::Clone for MEMORY_PRIORITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MUTEX_MODIFY_STATE: u32 = 1u32;
pub type NamespaceHandle = isize;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut ::core::ffi::c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut ::core::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut ::core::ffi::c_void,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut ::core::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut ::core::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut ::core::ffi::c_void; 1],
    pub SessionId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PEB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PEB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut ::core::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for PEB_LDR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PFLS_CALLBACK_FUNCTION = unsafe extern "system" fn(lpflsdata: *const ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PINIT_ONCE_FN = unsafe extern "system" fn(initonce: *mut RTL_RUN_ONCE, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub const PME_CURRENT_VERSION: u32 = 1u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: u32 = 2u32;
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: u32 = 1u32;
pub type PPS_POST_PROCESS_INIT_ROUTINE = unsafe extern "system" fn();
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
pub const ProcessBasicInformation: i32 = 0i32;
pub const ProcessDebugPort: i32 = 7i32;
pub const ProcessWow64Information: i32 = 26i32;
pub const ProcessImageFileName: i32 = 27i32;
pub const ProcessBreakOnTermination: i32 = 29i32;
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: u32 = 25u32;
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: u32 = 24u32;
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: u32 = 26u32;
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: u32 = 27u32;
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: u32 = 18u32;
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: u32 = 7u32;
pub const PF_CHANNELS_ENABLED: u32 = 16u32;
pub const PF_COMPARE_EXCHANGE_DOUBLE: u32 = 2u32;
pub const PF_COMPARE_EXCHANGE128: u32 = 14u32;
pub const PF_COMPARE64_EXCHANGE128: u32 = 15u32;
pub const PF_FASTFAIL_AVAILABLE: u32 = 23u32;
pub const PF_FLOATING_POINT_EMULATED: u32 = 1u32;
pub const PF_FLOATING_POINT_PRECISION_ERRATA: u32 = 0u32;
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: u32 = 3u32;
pub const PF_NX_ENABLED: u32 = 12u32;
pub const PF_PAE_ENABLED: u32 = 9u32;
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: u32 = 8u32;
pub const PF_RDWRFSGSBASE_AVAILABLE: u32 = 22u32;
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: u32 = 20u32;
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: u32 = 13u32;
pub const PF_VIRT_FIRMWARE_ENABLED: u32 = 21u32;
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: u32 = 6u32;
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: u32 = 10u32;
pub const PF_XSAVE_ENABLED: u32 = 17u32;
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: u32 = 29u32;
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: u32 = 30u32;
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: u32 = 31u32;
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: u32 = 34u32;
pub const PROCESS_TERMINATE: u32 = 1u32;
pub const PROCESS_CREATE_THREAD: u32 = 2u32;
pub const PROCESS_SET_SESSIONID: u32 = 4u32;
pub const PROCESS_VM_OPERATION: u32 = 8u32;
pub const PROCESS_VM_READ: u32 = 16u32;
pub const PROCESS_VM_WRITE: u32 = 32u32;
pub const PROCESS_DUP_HANDLE: u32 = 64u32;
pub const PROCESS_CREATE_PROCESS: u32 = 128u32;
pub const PROCESS_SET_QUOTA: u32 = 256u32;
pub const PROCESS_SET_INFORMATION: u32 = 512u32;
pub const PROCESS_QUERY_INFORMATION: u32 = 1024u32;
pub const PROCESS_SUSPEND_RESUME: u32 = 2048u32;
pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 4096u32;
pub const PROCESS_SET_LIMITED_INFORMATION: u32 = 8192u32;
pub const PROCESS_ALL_ACCESS: u32 = 2097151u32;
pub const PROCESS_DELETE: u32 = 65536u32;
pub const PROCESS_READ_CONTROL: u32 = 131072u32;
pub const PROCESS_WRITE_DAC: u32 = 262144u32;
pub const PROCESS_WRITE_OWNER: u32 = 524288u32;
pub const PROCESS_SYNCHRONIZE: u32 = 1048576u32;
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: u32 = 983040u32;
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: u32 = 0u32;
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PROCESS_BASIC_INFORMATION {
    pub Reserved1: *mut ::core::ffi::c_void,
    pub PebBaseAddress: *mut PEB,
    pub Reserved2: [*mut ::core::ffi::c_void; 2],
    pub UniqueProcessId: usize,
    pub Reserved3: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PROCESS_BASIC_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PROCESS_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DEBUG_PROCESS: u32 = 1u32;
pub const DEBUG_ONLY_THIS_PROCESS: u32 = 2u32;
pub const CREATE_SUSPENDED: u32 = 4u32;
pub const DETACHED_PROCESS: u32 = 8u32;
pub const CREATE_NEW_CONSOLE: u32 = 16u32;
pub const NORMAL_PRIORITY_CLASS: u32 = 32u32;
pub const IDLE_PRIORITY_CLASS: u32 = 64u32;
pub const HIGH_PRIORITY_CLASS: u32 = 128u32;
pub const REALTIME_PRIORITY_CLASS: u32 = 256u32;
pub const CREATE_NEW_PROCESS_GROUP: u32 = 512u32;
pub const CREATE_UNICODE_ENVIRONMENT: u32 = 1024u32;
pub const CREATE_SEPARATE_WOW_VDM: u32 = 2048u32;
pub const CREATE_SHARED_WOW_VDM: u32 = 4096u32;
pub const CREATE_FORCEDOS: u32 = 8192u32;
pub const BELOW_NORMAL_PRIORITY_CLASS: u32 = 16384u32;
pub const ABOVE_NORMAL_PRIORITY_CLASS: u32 = 32768u32;
pub const INHERIT_PARENT_AFFINITY: u32 = 65536u32;
pub const INHERIT_CALLER_PRIORITY: u32 = 131072u32;
pub const CREATE_PROTECTED_PROCESS: u32 = 262144u32;
pub const EXTENDED_STARTUPINFO_PRESENT: u32 = 524288u32;
pub const PROCESS_MODE_BACKGROUND_BEGIN: u32 = 1048576u32;
pub const PROCESS_MODE_BACKGROUND_END: u32 = 2097152u32;
pub const CREATE_SECURE_PROCESS: u32 = 4194304u32;
pub const CREATE_BREAKAWAY_FROM_JOB: u32 = 16777216u32;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: u32 = 33554432u32;
pub const CREATE_DEFAULT_ERROR_MODE: u32 = 67108864u32;
pub const CREATE_NO_WINDOW: u32 = 134217728u32;
pub const PROFILE_USER: u32 = 268435456u32;
pub const PROFILE_KERNEL: u32 = 536870912u32;
pub const PROFILE_SERVER: u32 = 1073741824u32;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: u32 = 2147483648u32;
pub const PROCESS_DEP_ENABLE: u32 = 1u32;
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: u32 = 2u32;
pub const PROCESS_DEP_NONE: u32 = 0u32;
#[repr(C)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: usize,
    pub Size: usize,
    pub Flags: u32,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ProcessMemoryPriority: i32 = 0i32;
pub const ProcessMemoryExhaustionInfo: i32 = 1i32;
pub const ProcessAppMemoryInfo: i32 = 2i32;
pub const ProcessInPrivateInfo: i32 = 3i32;
pub const ProcessPowerThrottling: i32 = 4i32;
pub const ProcessReservedValue1: i32 = 5i32;
pub const ProcessTelemetryCoverageInfo: i32 = 6i32;
pub const ProcessProtectionLevelInfo: i32 = 7i32;
pub const ProcessLeapSecondInfo: i32 = 8i32;
pub const ProcessMachineTypeInfo: i32 = 9i32;
pub const ProcessInformationClassMax: i32 = 10i32;
#[repr(C)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PROCESS_LEAP_SECOND_INFO {}
impl ::core::clone::Clone for PROCESS_LEAP_SECOND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
#[repr(C)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: u16,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
impl ::core::marker::Copy for PROCESS_MACHINE_INFORMATION {}
impl ::core::clone::Clone for PROCESS_MACHINE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PMETypeFailFastOnCommitFailure: i32 = 0i32;
pub const PMETypeMax: i32 = 1i32;
pub const ProcessDEPPolicy: i32 = 0i32;
pub const ProcessASLRPolicy: i32 = 1i32;
pub const ProcessDynamicCodePolicy: i32 = 2i32;
pub const ProcessStrictHandleCheckPolicy: i32 = 3i32;
pub const ProcessSystemCallDisablePolicy: i32 = 4i32;
pub const ProcessMitigationOptionsMask: i32 = 5i32;
pub const ProcessExtensionPointDisablePolicy: i32 = 6i32;
pub const ProcessControlFlowGuardPolicy: i32 = 7i32;
pub const ProcessSignaturePolicy: i32 = 8i32;
pub const ProcessFontDisablePolicy: i32 = 9i32;
pub const ProcessImageLoadPolicy: i32 = 10i32;
pub const ProcessSystemCallFilterPolicy: i32 = 11i32;
pub const ProcessPayloadRestrictionPolicy: i32 = 12i32;
pub const ProcessChildProcessPolicy: i32 = 13i32;
pub const ProcessSideChannelIsolationPolicy: i32 = 14i32;
pub const ProcessUserShadowStackPolicy: i32 = 15i32;
pub const ProcessRedirectionTrustPolicy: i32 = 16i32;
pub const MaxProcessMitigationPolicy: i32 = 17i32;
pub const PROCESS_NAME_WIN32: u32 = 0u32;
pub const PROCESS_NAME_NATIVE: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
#[repr(C)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for PROCESS_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROTECTION_LEVEL_WINTCB_LIGHT: u32 = 0u32;
pub const PROTECTION_LEVEL_WINDOWS: u32 = 1u32;
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: u32 = 2u32;
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: u32 = 3u32;
pub const PROTECTION_LEVEL_LSA_LIGHT: u32 = 4u32;
pub const PROTECTION_LEVEL_WINTCB: u32 = 5u32;
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: u32 = 6u32;
pub const PROTECTION_LEVEL_AUTHENTICODE: u32 = 7u32;
pub const PROTECTION_LEVEL_PPL_APP: u32 = 8u32;
pub const PROTECTION_LEVEL_NONE: u32 = 4294967294u32;
#[repr(C)]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
#[cfg(feature = "Win32_System_SystemServices")]
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = unsafe extern "system" fn(reason: super::SystemServices::RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const ::core::ffi::c_void);
pub type PTIMERAPCROUTINE = unsafe extern "system" fn(lpargtocompletionroutine: *const ::core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32);
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = unsafe extern "system" fn(objectcontext: *mut ::core::ffi::c_void, cleanupcontext: *mut ::core::ffi::c_void);
pub type PTP_POOL = isize;
pub type PTP_SIMPLE_CALLBACK = unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void);
pub type PTP_TIMER_CALLBACK = unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, timer: *mut TP_TIMER);
pub type PTP_WAIT_CALLBACK = unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, wait: *mut TP_WAIT, waitresult: u32);
pub type PTP_WIN32_IO_CALLBACK = unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, overlapped: *mut ::core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: *mut TP_IO);
pub type PTP_WORK_CALLBACK = unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, work: *mut TP_WORK);
pub const QUEUE_USER_APC_FLAGS_NONE: i32 = 0i32;
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: POWER_REQUEST_CONTEXT_FLAGS,
    pub Reason: REASON_CONTEXT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::super::Foundation::HINSTANCE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl ::core::marker::Copy for RTL_BARRIER {}
impl ::core::clone::Clone for RTL_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_CONDITION_VARIABLE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_CONDITION_VARIABLE {}
impl ::core::clone::Clone for RTL_CONDITION_VARIABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: *mut RTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: super::super::Foundation::HANDLE,
    pub LockSemaphore: super::super::Foundation::HANDLE,
    pub SpinCount: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut RTL_CRITICAL_SECTION,
    pub ProcessLocksList: super::Kernel::LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub SpareWORD: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION_DEBUG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RTL_RUN_ONCE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_RUN_ONCE {}
impl ::core::clone::Clone for RTL_RUN_ONCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_SRWLOCK {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_SRWLOCK {}
impl ::core::clone::Clone for RTL_SRWLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
pub const UmsThreadInvalidInfoClass: i32 = 0i32;
pub const UmsThreadUserContext: i32 = 1i32;
pub const UmsThreadPriority: i32 = 2i32;
pub const UmsThreadAffinity: i32 = 3i32;
pub const UmsThreadTeb: i32 = 4i32;
pub const UmsThreadIsSuspended: i32 = 5i32;
pub const UmsThreadIsTerminated: i32 = 6i32;
pub const UmsThreadMaxInfoClass: i32 = 7i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut ::core::ffi::c_void; 10],
    pub ImagePathName: super::super::Foundation::UNICODE_STRING,
    pub CommandLine: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_USER_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_USER_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: super::super::Foundation::PSTR,
    pub lpDesktop: super::super::Foundation::PSTR,
    pub lpTitle: super::super::Foundation::PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: super::super::Foundation::PWSTR,
    pub lpDesktop: super::super::Foundation::PWSTR,
    pub lpTitle: super::super::Foundation::PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const STARTF_FORCEONFEEDBACK: u32 = 64u32;
pub const STARTF_FORCEOFFFEEDBACK: u32 = 128u32;
pub const STARTF_PREVENTPINNING: u32 = 8192u32;
pub const STARTF_RUNFULLSCREEN: u32 = 32u32;
pub const STARTF_TITLEISAPPID: u32 = 4096u32;
pub const STARTF_TITLEISLINKNAME: u32 = 2048u32;
pub const STARTF_UNTRUSTEDSOURCE: u32 = 32768u32;
pub const STARTF_USECOUNTCHARS: u32 = 8u32;
pub const STARTF_USEFILLATTRIBUTE: u32 = 16u32;
pub const STARTF_USEHOTKEY: u32 = 512u32;
pub const STARTF_USEPOSITION: u32 = 4u32;
pub const STARTF_USESHOWWINDOW: u32 = 1u32;
pub const STARTF_USESIZE: u32 = 2u32;
pub const STARTF_USESTDHANDLES: u32 = 256u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
pub const ThreadIsIoPending: i32 = 16i32;
pub const ThreadNameInformation: i32 = 38i32;
pub const THREAD_TERMINATE: u32 = 1u32;
pub const THREAD_SUSPEND_RESUME: u32 = 2u32;
pub const THREAD_GET_CONTEXT: u32 = 8u32;
pub const THREAD_SET_CONTEXT: u32 = 16u32;
pub const THREAD_SET_INFORMATION: u32 = 32u32;
pub const THREAD_QUERY_INFORMATION: u32 = 64u32;
pub const THREAD_SET_THREAD_TOKEN: u32 = 128u32;
pub const THREAD_IMPERSONATE: u32 = 256u32;
pub const THREAD_DIRECT_IMPERSONATION: u32 = 512u32;
pub const THREAD_SET_LIMITED_INFORMATION: u32 = 1024u32;
pub const THREAD_QUERY_LIMITED_INFORMATION: u32 = 2048u32;
pub const THREAD_RESUME: u32 = 4096u32;
pub const THREAD_ALL_ACCESS: u32 = 2097151u32;
pub const THREAD_DELETE: u32 = 65536u32;
pub const THREAD_READ_CONTROL: u32 = 131072u32;
pub const THREAD_WRITE_DAC: u32 = 262144u32;
pub const THREAD_WRITE_OWNER: u32 = 524288u32;
pub const THREAD_SYNCHRONIZE: u32 = 1048576u32;
pub const THREAD_STANDARD_RIGHTS_REQUIRED: u32 = 983040u32;
pub const THREAD_CREATE_RUN_IMMEDIATELY: u32 = 0u32;
pub const THREAD_CREATE_SUSPENDED: u32 = 4u32;
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: u32 = 65536u32;
pub const ThreadMemoryPriority: i32 = 0i32;
pub const ThreadAbsoluteCpuPriority: i32 = 1i32;
pub const ThreadDynamicCodePolicy: i32 = 2i32;
pub const ThreadPowerThrottling: i32 = 3i32;
pub const ThreadInformationClassMax: i32 = 4i32;
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[repr(C)]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for THREAD_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for THREAD_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
pub const THREAD_MODE_BACKGROUND_BEGIN: i32 = 65536i32;
pub const THREAD_MODE_BACKGROUND_END: i32 = 131072i32;
pub const THREAD_PRIORITY_ABOVE_NORMAL: i32 = 1i32;
pub const THREAD_PRIORITY_BELOW_NORMAL: i32 = -1i32;
pub const THREAD_PRIORITY_HIGHEST: i32 = 2i32;
pub const THREAD_PRIORITY_IDLE: i32 = -15i32;
pub const THREAD_PRIORITY_MIN: i32 = -2i32;
pub const THREAD_PRIORITY_LOWEST: i32 = -2i32;
pub const THREAD_PRIORITY_NORMAL: i32 = 0i32;
pub const THREAD_PRIORITY_TIME_CRITICAL: i32 = 15i32;
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: isize,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut ::core::ffi::c_void,
    pub ActivationContext: isize,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_1,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_0(pub u8);
#[repr(C)]
pub union TP_CALLBACK_ENVIRON_V3_1 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_1_0,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_CALLBACK_INSTANCE(pub u8);
pub const TP_CALLBACK_PRIORITY_HIGH: i32 = 0i32;
pub const TP_CALLBACK_PRIORITY_NORMAL: i32 = 1i32;
pub const TP_CALLBACK_PRIORITY_LOW: i32 = 2i32;
pub const TP_CALLBACK_PRIORITY_INVALID: i32 = 3i32;
pub const TP_CALLBACK_PRIORITY_COUNT: i32 = 3i32;
#[repr(C)]
pub struct TP_IO(pub u8);
#[repr(C)]
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: usize,
    pub StackCommit: usize,
}
impl ::core::marker::Copy for TP_POOL_STACK_INFORMATION {}
impl ::core::clone::Clone for TP_POOL_STACK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TP_TIMER(pub u8);
#[repr(C)]
pub struct TP_WAIT(pub u8);
#[repr(C)]
pub struct TP_WORK(pub u8);
pub type TimerQueueHandle = isize;
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: *mut ::core::ffi::c_void,
    pub SchedulerProc: PRTL_UMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::marker::Copy for UMS_SCHEDULER_STARTUP_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::clone::Clone for UMS_SCHEDULER_STARTUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type WAITORTIMERCALLBACK = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::BOOLEAN);
pub const WAIT_ABANDONED: u32 = 128u32;
pub const WAIT_ABANDONED_0: u32 = 128u32;
pub const WAIT_IO_COMPLETION: u32 = 192u32;
pub const WAIT_OBJECT_0: u32 = 0u32;
pub const WT_EXECUTEDEFAULT: u32 = 0u32;
pub const WT_EXECUTEINIOTHREAD: u32 = 1u32;
pub const WT_EXECUTEINPERSISTENTTHREAD: u32 = 128u32;
pub const WT_EXECUTEINWAITTHREAD: u32 = 4u32;
pub const WT_EXECUTELONGFUNCTION: u32 = 16u32;
pub const WT_EXECUTEONLYONCE: u32 = 8u32;
pub const WT_TRANSFER_IMPERSONATION: u32 = 256u32;
pub const WT_EXECUTEINTIMERTHREAD: u32 = 32u32;
