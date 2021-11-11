#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn AcquireSRWLockExclusive();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn AcquireSRWLockShared();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddIntegrityLabelToBoundaryDescriptor();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSIDToBoundaryDescriptor();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AttachThreadInput();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallbackMayRunLong();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CancelThreadpoolIo();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelWaitableTimer();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeTimerQueueTimer();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClosePrivateNamespace();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpool();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolCleanupGroup();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseThreadpoolCleanupGroupMembers();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolIo();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolTimer();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolWait();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CloseThreadpoolWork();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertFiberToThread();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ConvertThreadToFiber();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ConvertThreadToFiberEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateBoundaryDescriptorA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateBoundaryDescriptorW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventExA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventExW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateEventW();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateFiber();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateFiberEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexExA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexExW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateMutexW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreatePrivateNamespaceA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreatePrivateNamespaceW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateProcessA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateProcessAsUserA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateProcessAsUserW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateProcessW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateProcessWithLogonW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateProcessWithTokenW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateRemoteThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateRemoteThreadEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreExA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreExW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateSemaphoreW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateThread();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpool();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpoolCleanupGroup();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateThreadpoolIo();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpoolTimer();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpoolWait();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn CreateThreadpoolWork();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTimerQueue();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTimerQueueTimer();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUmsCompletionList();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUmsThreadContext();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerExW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerW();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn DeleteBoundaryDescriptor();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn DeleteCriticalSection();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn DeleteFiber();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn DeleteProcThreadAttributeList();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSynchronizationBarrier();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueue();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueueEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteTimerQueueTimer();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUmsCompletionList();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteUmsThreadContext();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DequeueUmsCompletionListItems();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn DisassociateCurrentThreadFromCallback();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn EnterCriticalSection();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnterSynchronizationBarrier();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
    pub fn EnterUmsSchedulingMode();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteUmsThread();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ExitProcess();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ExitThread();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn FlsAlloc();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlsFree();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn FlsGetValue();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlsSetValue();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn FlushProcessWriteBuffers();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeLibraryWhenCallbackReturns();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetActiveProcessorCount();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetActiveProcessorGroupCount();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentProcess();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentProcessId();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentProcessorNumber();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn GetCurrentProcessorNumberEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentThread();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentThreadId();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentThreadStackLimits();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetCurrentUmsThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExitCodeProcess();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExitCodeThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGuiResources();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetMachineTypeAttributes();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetMaximumProcessorCount();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetMaximumProcessorGroupCount();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetNextUmsListItem();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaAvailableMemoryNode();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaAvailableMemoryNodeEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaHighestNodeNumber();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaNodeNumberFromHandle();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaNodeProcessorMask();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetNumaNodeProcessorMask2();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetNumaNodeProcessorMaskEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProcessorNode();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetNumaProcessorNodeEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProximityNode();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNumaProximityNodeEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPriorityClass();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessAffinityMask();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDEPPolicy();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetProcessDefaultCpuSetMasks();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDefaultCpuSets();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessGroupAffinity();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessHandleCount();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessId();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessIdOfThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessInformation();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessIoCounters();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessMitigationPolicy();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessPriorityBoost();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessShutdownParameters();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessTimes();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn GetProcessVersion();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessWorkingSetSize();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStartupInfoA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStartupInfoW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimes();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadDescription();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetThreadGroupAffinity();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadIOPendingFlag();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadId();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetThreadIdealProcessorEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadInformation();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPriority();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadPriorityBoost();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn GetThreadSelectedCpuSetMasks();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadSelectedCpuSets();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadTimes();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUmsCompletionListEvent();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUmsSystemThreadInformation();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceBeginInitialize();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceComplete();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitOnceExecuteOnce();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn InitOnceInitialize();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn InitializeConditionVariable();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSection();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSectionAndSpinCount();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeCriticalSectionEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeProcThreadAttributeList();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InitializeSListHead();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn InitializeSRWLock();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeSynchronizationBarrier();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedFlushSList();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPopEntrySList();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPushEntrySList();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn InterlockedPushListSListEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsImmersiveProcess();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessCritical();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessorFeaturePresent();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThreadAFiber();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThreadpoolTimerSet();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64Process();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64Process2();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LeaveCriticalSection();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn LeaveCriticalSectionWhenCallbackReturns();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryInformationProcess();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryInformationThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtSetInformationThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenEventW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenMutexW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPrivateNamespaceA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPrivateNamespaceW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenProcess();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenProcessToken();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenSemaphoreW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenThreadToken();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWaitableTimerW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PulseEvent();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn QueryDepthSList();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryFullProcessImageNameA();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryFullProcessImageNameW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProcessAffinityUpdateMode();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProtectedPolicy();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadpoolStackInformation();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryUmsThreadInformation();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserAPC();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserAPC2();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueueUserWorkItem();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterWaitForSingleObject();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseMutex();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseMutexWhenCallbackReturns();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ReleaseSRWLockExclusive();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn ReleaseSRWLockShared();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseSemaphore();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseSemaphoreWhenCallbackReturns();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetEvent();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResumeThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetCriticalSectionSpinCount();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEvent();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEventWhenCallbackReturns();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPriorityClass();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessAffinityMask();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessAffinityUpdateMode();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDEPPolicy();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetProcessDefaultCpuSetMasks();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDefaultCpuSets();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDynamicEHContinuationTargets();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDynamicEnforcedCetCompatibleRanges();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessInformation();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessMitigationPolicy();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessPriorityBoost();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessRestrictionExemption();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessShutdownParameters();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWorkingSetSize();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProtectedPolicy();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadAffinityMask();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadDescription();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetThreadGroupAffinity();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadIdealProcessor();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetThreadIdealProcessorEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadInformation();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPriority();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadPriorityBoost();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_SystemInformation`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
    pub fn SetThreadSelectedCpuSetMasks();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadSelectedCpuSets();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadStackGuarantee();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadToken();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolStackInformation();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn SetThreadpoolThreadMaximum();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolThreadMinimum();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolTimer();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolTimerEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolWait();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadpoolWaitEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTimerQueueTimer();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUmsThreadInformation();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWaitableTimer();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWaitableTimerEx();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn Sleep();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SleepConditionVariableCS();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SleepConditionVariableSRW();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SleepEx();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn StartThreadpoolIo();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn SubmitThreadpoolWork();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SuspendThread();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn SwitchToFiber();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwitchToThread();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateProcess();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateThread();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn TlsAlloc();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TlsFree();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn TlsGetValue();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TlsSetValue();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryAcquireSRWLockExclusive();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryAcquireSRWLockShared();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn TryEnterCriticalSection();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrySubmitThreadpoolCallback();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UmsThreadYield();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterWait();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterWaitEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateProcThreadAttribute();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForInputIdle();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForMultipleObjects();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForMultipleObjectsEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForSingleObject();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForSingleObjectEx();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolIoCallbacks();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolTimerCallbacks();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolWaitCallbacks();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitForThreadpoolWorkCallbacks();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitOnAddress();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn WakeAllConditionVariable();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn WakeByAddressAll();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn WakeByAddressSingle();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn WakeConditionVariable();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinExec();
    #[doc = "*Required features: `Win32_System_Threading`*"]
    pub fn Wow64SetThreadDefaultGuestMachine();
    #[doc = "*Required features: `Win32_System_Threading`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64SuspendThread();
}
