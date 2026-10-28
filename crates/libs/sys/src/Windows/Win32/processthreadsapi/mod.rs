#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateProcessA(lpapplicationname : windows_sys::core::PCSTR, lpcommandline : windows_sys::core::PSTR, lpprocessattributes : super::LPSECURITY_ATTRIBUTES, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, binherithandles : windows_sys::core::BOOL, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_sys::core::PCSTR, lpstartupinfo : LPSTARTUPINFOA, lpprocessinformation : LPPROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn CreateProcessAsUserA(htoken : super::HANDLE, lpapplicationname : windows_sys::core::PCSTR, lpcommandline : windows_sys::core::PSTR, lpprocessattributes : super::LPSECURITY_ATTRIBUTES, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, binherithandles : windows_sys::core::BOOL, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_sys::core::PCSTR, lpstartupinfo : LPSTARTUPINFOA, lpprocessinformation : LPPROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn CreateProcessAsUserW(htoken : super::HANDLE, lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, lpprocessattributes : super::LPSECURITY_ATTRIBUTES, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, binherithandles : windows_sys::core::BOOL, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : LPSTARTUPINFOW, lpprocessinformation : LPPROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateProcessW(lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, lpprocessattributes : super::LPSECURITY_ATTRIBUTES, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, binherithandles : windows_sys::core::BOOL, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : LPSTARTUPINFOW, lpprocessinformation : LPPROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateRemoteThread(hprocess : super::HANDLE, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, dwstacksize : super::SIZE_T, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : super::LPVOID, dwcreationflags : super::DWORD, lpthreadid : super::LPDWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateRemoteThreadEx(hprocess : super::HANDLE, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, dwstacksize : super::SIZE_T, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : super::LPVOID, dwcreationflags : super::DWORD, lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid : super::LPDWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateThread(lpthreadattributes : super::LPSECURITY_ATTRIBUTES, dwstacksize : super::SIZE_T, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : super::LPVOID, dwcreationflags : super::DWORD, lpthreadid : super::LPDWORD) -> super::HANDLE);
windows_link::link!("kernel32.dll" "system" fn DeleteProcThreadAttributeList(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST));
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn ExitProcess(uexitcode : super::UINT) -> !);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn ExitThread(dwexitcode : super::DWORD) -> !);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FlushInstructionCache(hprocess : super::HANDLE, lpbaseaddress : super::LPCVOID, dwsize : super::SIZE_T) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FlushProcessWriteBuffers());
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentProcess() -> super::HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentProcessorNumber() -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetCurrentProcessorNumberEx(procnumber : super::PPROCESSOR_NUMBER));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentThread() -> super::HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentThreadStackLimits(lowlimit : super::PULONG_PTR, highlimit : super::PULONG_PTR));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetExitCodeProcess(hprocess : super::HANDLE, lpexitcode : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetExitCodeThread(hthread : super::HANDLE, lpexitcode : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetMachineTypeAttributes(machine : super::USHORT, machinetypeattributes : *mut MACHINE_ATTRIBUTES) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetPriorityClass(hprocess : super::HANDLE) -> super::DWORD);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSetMasks(process : super::HANDLE, cpusetmasks : super::PGROUP_AFFINITY, cpusetmaskcount : super::USHORT, requiredmaskcount : super::PUSHORT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSets(process : super::HANDLE, cpusetids : super::PULONG, cpusetidcount : super::ULONG, requiredidcount : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessHandleCount(hprocess : super::HANDLE, pdwhandlecount : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessId(process : super::HANDLE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessIdOfThread(thread : super::HANDLE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessInformation(hprocess : super::HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : super::LPVOID, processinformationsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessMitigationPolicy(hprocess : super::HANDLE, mitigationpolicy : super::PROCESS_MITIGATION_POLICY, lpbuffer : super::PVOID, dwlength : super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessPriorityBoost(hprocess : super::HANDLE, pdisablepriorityboost : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetProcessShutdownParameters(lpdwlevel : super::LPDWORD, lpdwflags : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessTimes(hprocess : super::HANDLE, lpcreationtime : super::LPFILETIME, lpexittime : super::LPFILETIME, lpkerneltime : super::LPFILETIME, lpusertime : super::LPFILETIME) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetProcessVersion(processid : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetStartupInfoW(lpstartupinfo : LPSTARTUPINFOW));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetSystemCpuSetInformation(information : super::PSYSTEM_CPU_SET_INFORMATION, bufferlength : super::ULONG, returnedlength : super::PULONG, process : super::HANDLE, flags : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetSystemTimes(lpidletime : super::PFILETIME, lpkerneltime : super::PFILETIME, lpusertime : super::PFILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadContext(hthread : super::HANDLE, lpcontext : super::LPCONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadDescription(hthread : super::HANDLE, ppszthreaddescription : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadIOPendingFlag(hthread : super::HANDLE, lpioispending : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadId(thread : super::HANDLE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadIdealProcessorEx(hthread : super::HANDLE, lpidealprocessor : super::PPROCESSOR_NUMBER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadInformation(hthread : super::HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : super::LPVOID, threadinformationsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadPriority(hthread : super::HANDLE) -> i32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadPriorityBoost(hthread : super::HANDLE, pdisablepriorityboost : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSetMasks(thread : super::HANDLE, cpusetmasks : super::PGROUP_AFFINITY, cpusetmaskcount : super::USHORT, requiredmaskcount : super::PUSHORT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSets(thread : super::HANDLE, cpusetids : super::PULONG, cpusetidcount : super::ULONG, requiredidcount : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadTimes(hthread : super::HANDLE, lpcreationtime : super::LPFILETIME, lpexittime : super::LPFILETIME, lpkerneltime : super::LPFILETIME, lpusertime : super::LPFILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn InitializeProcThreadAttributeList(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount : super::DWORD, dwflags : super::DWORD, lpsize : super::PSIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn IsProcessCritical(hprocess : super::HANDLE, critical : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn IsProcessorFeaturePresent(processorfeature : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, dwprocessid : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : super::HANDLE, desiredaccess : super::DWORD, tokenhandle : super::PHANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenThread(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, dwthreadid : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn OpenThreadToken(threadhandle : super::HANDLE, desiredaccess : super::DWORD, openasself : windows_sys::core::BOOL, tokenhandle : super::PHANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn ProcessIdToSessionId(dwprocessid : super::DWORD, psessionid : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueryProcessAffinityUpdateMode(hprocess : super::HANDLE, lpdwflags : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn QueryProtectedPolicy(policyguid : super::LPCGUID, policyvalue : super::PULONG_PTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueueUserAPC(pfnapc : super::PAPCFUNC, hthread : super::HANDLE, dwdata : super::ULONG_PTR) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueueUserAPC2(apcroutine : super::PAPCFUNC, thread : super::HANDLE, data : super::ULONG_PTR, flags : QUEUE_USER_APC_FLAGS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ResumeThread(hthread : super::HANDLE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetPriorityClass(hprocess : super::HANDLE, dwpriorityclass : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessAffinityUpdateMode(hprocess : super::HANDLE, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSetMasks(process : super::HANDLE, cpusetmasks : super::PGROUP_AFFINITY, cpusetmaskcount : super::USHORT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSets(process : super::HANDLE, cpusetids : *const super::ULONG, cpusetidcount : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessDynamicEHContinuationTargets(process : super::HANDLE, numberoftargets : super::USHORT, targets : super::PPROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessDynamicEnforcedCetCompatibleRanges(process : super::HANDLE, numberofranges : super::USHORT, ranges : super::PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessInformation(hprocess : super::HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : super::LPVOID, processinformationsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessMitigationPolicy(mitigationpolicy : super::PROCESS_MITIGATION_POLICY, lpbuffer : super::PVOID, dwlength : super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessPriorityBoost(hprocess : super::HANDLE, bdisablepriorityboost : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetProcessShutdownParameters(dwlevel : super::DWORD, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn SetProtectedPolicy(policyguid : super::LPCGUID, policyvalue : super::ULONG_PTR, oldpolicyvalue : super::PULONG_PTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadContext(hthread : super::HANDLE, lpcontext : *const super::CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadDescription(hthread : super::HANDLE, lpthreaddescription : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadIdealProcessor(hthread : super::HANDLE, dwidealprocessor : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadIdealProcessorEx(hthread : super::HANDLE, lpidealprocessor : super::PPROCESSOR_NUMBER, lppreviousidealprocessor : super::PPROCESSOR_NUMBER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadInformation(hthread : super::HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : super::LPVOID, threadinformationsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadPriority(hthread : super::HANDLE, npriority : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadPriorityBoost(hthread : super::HANDLE, bdisablepriorityboost : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSetMasks(thread : super::HANDLE, cpusetmasks : super::PGROUP_AFFINITY, cpusetmaskcount : super::USHORT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSets(thread : super::HANDLE, cpusetids : *const super::ULONG, cpusetidcount : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn SetThreadStackGuarantee(stacksizeinbytes : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetThreadToken(thread : super::PHANDLE, token : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SuspendThread(hthread : super::HANDLE) -> super::DWORD);
windows_link::link!("kernel32.dll" "system" fn SwitchToThread() -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TerminateProcess(hprocess : super::HANDLE, uexitcode : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn TerminateThread(hthread : super::HANDLE, dwexitcode : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn TlsAlloc() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn TlsFree(dwtlsindex : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn TlsGetValue(dwtlsindex : super::DWORD) -> super::LPVOID);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn TlsGetValue2(dwtlsindex : super::DWORD) -> super::LPVOID);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn TlsSetValue(dwtlsindex : super::DWORD, lptlsvalue : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn UpdateProcThreadAttribute(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, dwflags : super::DWORD, attribute : super::DWORD_PTR, lpvalue : super::PVOID, cbsize : super::SIZE_T, lppreviousvalue : super::PVOID, lpreturnsize : super::PSIZE_T) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct APC_CALLBACK_DATA {
    pub Parameter: super::ULONG_PTR,
    pub ContextRecord: super::PCONTEXT,
    pub Reserved0: super::ULONG_PTR,
    pub Reserved1: super::ULONG_PTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: super::ULONG64,
    pub PrivateCommitUsage: super::ULONG64,
    pub PeakPrivateCommitUsage: super::ULONG64,
    pub TotalCommitUsage: super::ULONG64,
}
pub const KernelEnabled: MACHINE_ATTRIBUTES = 2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut _PROC_THREAD_ATTRIBUTE_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTARTUPINFO = *mut STARTUPINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTARTUPINFOA = *mut STARTUPINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPSTARTUPINFOW = *mut STARTUPINFOW;
pub type MACHINE_ATTRIBUTES = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: super::ULONG,
}
#[cfg(feature = "shared")]
pub type OVERRIDE_PREFETCH_PARAMETER = super::UINT32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PAPC_CALLBACK_DATA = *mut APC_CALLBACK_DATA;
#[cfg(feature = "shared")]
pub type PAPP_MEMORY_INFORMATION = *mut APP_MEMORY_INFORMATION;
#[cfg(feature = "shared")]
pub type PMEMORY_PRIORITY_INFORMATION = *mut MEMORY_PRIORITY_INFORMATION;
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = 0;
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = 1;
pub const PME_CURRENT_VERSION: i32 = 1;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: i32 = 0;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: i32 = 1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
#[cfg(feature = "shared")]
pub type PPROCESS_LEAP_SECOND_INFO = *mut PROCESS_LEAP_SECOND_INFO;
#[cfg(feature = "shared")]
pub type PPROCESS_MEMORY_EXHAUSTION_INFO = *mut PROCESS_MEMORY_EXHAUSTION_INFO;
pub type PPROCESS_MEMORY_EXHAUSTION_TYPE = *mut PROCESS_MEMORY_EXHAUSTION_TYPE;
#[cfg(feature = "shared")]
pub type PPROCESS_POWER_THROTTLING_STATE = *mut PROCESS_POWER_THROTTLING_STATE;
pub type PPROC_THREAD_ATTRIBUTE_LIST = *mut _PROC_THREAD_ATTRIBUTE_LIST;
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::HANDLE,
    pub hThread: super::HANDLE,
    pub dwProcessId: super::DWORD,
    pub dwThreadId: super::DWORD,
}
pub type PROCESS_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: super::ULONG,
    pub Reserved: super::ULONG,
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: i32 = 1;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: super::USHORT,
    pub Res0: super::USHORT,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: super::USHORT,
    pub Reserved: super::USHORT,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: super::ULONG_PTR,
}
pub type PROCESS_MEMORY_EXHAUSTION_TYPE = i32;
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: i32 = 1;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: i32 = 1;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: super::ULONG,
    pub ControlMask: super::ULONG,
    pub StateMask: super::ULONG,
}
pub const PROCESS_POWER_THROTTLING_VALID_FLAGS: i32 = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: super::DWORD,
}
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: i32 = 1;
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = 2;
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = 3;
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = 12;
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = 8;
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = 9;
pub const ProcessMaxOverridePrefetchParameter: PROCESS_INFORMATION_CLASS = 11;
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = 1;
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = 0;
pub const ProcessOverrideSubsequentPrefetchParameter: PROCESS_INFORMATION_CLASS = 10;
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = 4;
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = 7;
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = 5;
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = 6;
pub const QUEUE_USER_APC_CALLBACK_DATA_CONTEXT: QUEUE_USER_APC_FLAGS = 65536;
pub type QUEUE_USER_APC_FLAGS = i32;
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = 0;
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = 1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type STARTUPINFO = STARTUPINFOA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct STARTUPINFOA {
    pub cb: super::DWORD,
    pub lpReserved: windows_sys::core::PSTR,
    pub lpDesktop: windows_sys::core::PSTR,
    pub lpTitle: windows_sys::core::PSTR,
    pub dwX: super::DWORD,
    pub dwY: super::DWORD,
    pub dwXSize: super::DWORD,
    pub dwYSize: super::DWORD,
    pub dwXCountChars: super::DWORD,
    pub dwYCountChars: super::DWORD,
    pub dwFillAttribute: super::DWORD,
    pub dwFlags: super::DWORD,
    pub wShowWindow: super::WORD,
    pub cbReserved2: super::WORD,
    pub lpReserved2: super::LPBYTE,
    pub hStdInput: super::HANDLE,
    pub hStdOutput: super::HANDLE,
    pub hStdError: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct STARTUPINFOW {
    pub cb: super::DWORD,
    pub lpReserved: windows_sys::core::PWSTR,
    pub lpDesktop: windows_sys::core::PWSTR,
    pub lpTitle: windows_sys::core::PWSTR,
    pub dwX: super::DWORD,
    pub dwY: super::DWORD,
    pub dwXSize: super::DWORD,
    pub dwYSize: super::DWORD,
    pub dwXCountChars: super::DWORD,
    pub dwYCountChars: super::DWORD,
    pub dwFillAttribute: super::DWORD,
    pub dwFlags: super::DWORD,
    pub wShowWindow: super::WORD,
    pub cbReserved2: super::WORD,
    pub lpReserved2: super::LPBYTE,
    pub hStdInput: super::HANDLE,
    pub hStdOutput: super::HANDLE,
    pub hStdError: super::HANDLE,
}
pub type THREAD_INFORMATION_CLASS = i32;
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: i32 = 1;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: super::ULONG,
    pub ControlMask: super::ULONG,
    pub StateMask: super::ULONG,
}
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: i32 = 1;
#[cfg(feature = "shared")]
pub const TLS_OUT_OF_INDEXES: super::DWORD = 4294967295;
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = 1;
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = 2;
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = 4;
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = 0;
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = 3;
pub const UserEnabled: MACHINE_ATTRIBUTES = 1;
pub const Wow64Container: MACHINE_ATTRIBUTES = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _PROC_THREAD_ATTRIBUTE_LIST(pub u8);
