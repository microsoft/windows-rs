#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateProcessA(lpapplicationname : windows_sys::core::PCSTR, lpcommandline : windows_sys::core::PSTR, lpprocessattributes : *const super::SECURITY_ATTRIBUTES, lpthreadattributes : *const super::SECURITY_ATTRIBUTES, binherithandles : windows_sys::core::BOOL, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCSTR, lpstartupinfo : *const STARTUPINFOA, lpprocessinformation : *mut PROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn CreateProcessAsUserA(htoken : super::HANDLE, lpapplicationname : windows_sys::core::PCSTR, lpcommandline : windows_sys::core::PSTR, lpprocessattributes : *const super::SECURITY_ATTRIBUTES, lpthreadattributes : *const super::SECURITY_ATTRIBUTES, binherithandles : windows_sys::core::BOOL, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCSTR, lpstartupinfo : *const STARTUPINFOA, lpprocessinformation : *mut PROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn CreateProcessAsUserW(htoken : super::HANDLE, lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, lpprocessattributes : *const super::SECURITY_ATTRIBUTES, lpthreadattributes : *const super::SECURITY_ATTRIBUTES, binherithandles : windows_sys::core::BOOL, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateProcessW(lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, lpprocessattributes : *const super::SECURITY_ATTRIBUTES, lpthreadattributes : *const super::SECURITY_ATTRIBUTES, binherithandles : windows_sys::core::BOOL, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateRemoteThread(hprocess : super::HANDLE, lpthreadattributes : *const super::SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpthreadid : *mut u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateRemoteThreadEx(hprocess : super::HANDLE, lpthreadattributes : *const super::SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpattributelist : *const _PROC_THREAD_ATTRIBUTE_LIST, lpthreadid : *mut u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateThread(lpthreadattributes : *const super::SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpthreadid : *mut u32) -> super::HANDLE);
windows_link::link!("kernel32.dll" "system" fn DeleteProcThreadAttributeList(lpattributelist : *mut _PROC_THREAD_ATTRIBUTE_LIST));
windows_link::link!("kernel32.dll" "system" fn ExitProcess(uexitcode : u32) -> !);
windows_link::link!("kernel32.dll" "system" fn ExitThread(dwexitcode : u32) -> !);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FlushInstructionCache(hprocess : super::HANDLE, lpbaseaddress : *const core::ffi::c_void, dwsize : usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FlushProcessWriteBuffers());
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentProcess() -> super::HANDLE);
windows_link::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetCurrentProcessorNumber() -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentProcessorNumberEx(procnumber : *mut super::PROCESSOR_NUMBER));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentThread() -> super::HANDLE);
windows_link::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> u32);
#[cfg(target_arch = "x86")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentThreadStackLimits(lowlimit : *mut u32, highlimit : *mut u32));
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "system" fn GetCurrentThreadStackLimits(lowlimit : *mut u64, highlimit : *mut u64));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetExitCodeProcess(hprocess : super::HANDLE, lpexitcode : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetExitCodeThread(hthread : super::HANDLE, lpexitcode : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetMachineTypeAttributes(machine : u16, machinetypeattributes : *mut MACHINE_ATTRIBUTES) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetPriorityClass(hprocess : super::HANDLE) -> u32);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSetMasks(process : super::HANDLE, cpusetmasks : *mut super::GROUP_AFFINITY, cpusetmaskcount : u16, requiredmaskcount : *mut u16) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSets(process : super::HANDLE, cpusetids : *mut u32, cpusetidcount : u32, requiredidcount : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessHandleCount(hprocess : super::HANDLE, pdwhandlecount : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessId(process : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessIdOfThread(thread : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessInformation(hprocess : super::HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : *mut core::ffi::c_void, processinformationsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessMitigationPolicy(hprocess : super::HANDLE, mitigationpolicy : super::PROCESS_MITIGATION_POLICY, lpbuffer : *mut core::ffi::c_void, dwlength : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessPriorityBoost(hprocess : super::HANDLE, pdisablepriorityboost : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetProcessShutdownParameters(lpdwlevel : *mut u32, lpdwflags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessTimes(hprocess : super::HANDLE, lpcreationtime : *mut super::FILETIME, lpexittime : *mut super::FILETIME, lpkerneltime : *mut super::FILETIME, lpusertime : *mut super::FILETIME) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetProcessVersion(processid : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetStartupInfoW(lpstartupinfo : *mut STARTUPINFOW));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetSystemCpuSetInformation(information : *mut super::SYSTEM_CPU_SET_INFORMATION, bufferlength : u32, returnedlength : *mut u32, process : super::HANDLE, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetSystemTimes(lpidletime : *mut super::FILETIME, lpkerneltime : *mut super::FILETIME, lpusertime : *mut super::FILETIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadContext(hthread : super::HANDLE, lpcontext : super::LPCONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadDescription(hthread : super::HANDLE, ppszthreaddescription : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadIOPendingFlag(hthread : super::HANDLE, lpioispending : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadId(thread : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadIdealProcessorEx(hthread : super::HANDLE, lpidealprocessor : *mut super::PROCESSOR_NUMBER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadInformation(hthread : super::HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : *mut core::ffi::c_void, threadinformationsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadPriority(hthread : super::HANDLE) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadPriorityBoost(hthread : super::HANDLE, pdisablepriorityboost : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSetMasks(thread : super::HANDLE, cpusetmasks : *mut super::GROUP_AFFINITY, cpusetmaskcount : u16, requiredmaskcount : *mut u16) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSets(thread : super::HANDLE, cpusetids : *mut u32, cpusetidcount : u32, requiredidcount : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadTimes(hthread : super::HANDLE, lpcreationtime : *mut super::FILETIME, lpexittime : *mut super::FILETIME, lpkerneltime : *mut super::FILETIME, lpusertime : *mut super::FILETIME) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn InitializeProcThreadAttributeList(lpattributelist : *mut _PROC_THREAD_ATTRIBUTE_LIST, dwattributecount : u32, dwflags : u32, lpsize : *mut usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn IsProcessCritical(hprocess : super::HANDLE, critical : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsProcessorFeaturePresent(processorfeature : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, dwprocessid : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : super::HANDLE, desiredaccess : u32, tokenhandle : *mut super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenThread(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, dwthreadid : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn OpenThreadToken(threadhandle : super::HANDLE, desiredaccess : u32, openasself : windows_sys::core::BOOL, tokenhandle : *mut super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ProcessIdToSessionId(dwprocessid : u32, psessionid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryProcessAffinityUpdateMode(hprocess : super::HANDLE, lpdwflags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
windows_link::link!("kernel32.dll" "system" fn QueryProtectedPolicy(policyguid : *const windows_sys::core::GUID, policyvalue : *mut u32) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "system" fn QueryProtectedPolicy(policyguid : *const windows_sys::core::GUID, policyvalue : *mut u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueueUserAPC(pfnapc : super::PAPCFUNC, hthread : super::HANDLE, dwdata : usize) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueueUserAPC2(apcroutine : super::PAPCFUNC, thread : super::HANDLE, data : usize, flags : QUEUE_USER_APC_FLAGS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ResumeThread(hthread : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetPriorityClass(hprocess : super::HANDLE, dwpriorityclass : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessAffinityUpdateMode(hprocess : super::HANDLE, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSetMasks(process : super::HANDLE, cpusetmasks : *const super::GROUP_AFFINITY, cpusetmaskcount : u16) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSets(process : super::HANDLE, cpusetids : *const u32, cpusetidcount : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessDynamicEHContinuationTargets(process : super::HANDLE, numberoftargets : u16, targets : *mut super::PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessDynamicEnforcedCetCompatibleRanges(process : super::HANDLE, numberofranges : u16, ranges : *mut super::PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessInformation(hprocess : super::HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : *const core::ffi::c_void, processinformationsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessMitigationPolicy(mitigationpolicy : super::PROCESS_MITIGATION_POLICY, lpbuffer : *const core::ffi::c_void, dwlength : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessPriorityBoost(hprocess : super::HANDLE, bdisablepriorityboost : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetProcessShutdownParameters(dwlevel : u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
windows_link::link!("kernel32.dll" "system" fn SetProtectedPolicy(policyguid : *const windows_sys::core::GUID, policyvalue : usize, oldpolicyvalue : *mut u32) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "system" fn SetProtectedPolicy(policyguid : *const windows_sys::core::GUID, policyvalue : usize, oldpolicyvalue : *mut u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadContext(hthread : super::HANDLE, lpcontext : *const super::CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadDescription(hthread : super::HANDLE, lpthreaddescription : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadIdealProcessor(hthread : super::HANDLE, dwidealprocessor : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadIdealProcessorEx(hthread : super::HANDLE, lpidealprocessor : *const super::PROCESSOR_NUMBER, lppreviousidealprocessor : *mut super::PROCESSOR_NUMBER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadInformation(hthread : super::HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : *const core::ffi::c_void, threadinformationsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadPriority(hthread : super::HANDLE, npriority : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadPriorityBoost(hthread : super::HANDLE, bdisablepriorityboost : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSetMasks(thread : super::HANDLE, cpusetmasks : *const super::GROUP_AFFINITY, cpusetmaskcount : u16) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSets(thread : super::HANDLE, cpusetids : *const u32, cpusetidcount : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetThreadStackGuarantee(stacksizeinbytes : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetThreadToken(thread : *const super::HANDLE, token : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SuspendThread(hthread : super::HANDLE) -> u32);
windows_link::link!("kernel32.dll" "system" fn SwitchToThread() -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn TerminateProcess(hprocess : super::HANDLE, uexitcode : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn TerminateThread(hthread : super::HANDLE, dwexitcode : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn TlsAlloc() -> u32);
windows_link::link!("kernel32.dll" "system" fn TlsFree(dwtlsindex : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn TlsGetValue(dwtlsindex : u32) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn TlsGetValue2(dwtlsindex : u32) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn TlsSetValue(dwtlsindex : u32, lptlsvalue : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn UpdateProcThreadAttribute(lpattributelist : *mut _PROC_THREAD_ATTRIBUTE_LIST, dwflags : u32, attribute : usize, lpvalue : *const core::ffi::c_void, cbsize : usize, lppreviousvalue : *mut core::ffi::c_void, lpreturnsize : *const usize) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct APC_CALLBACK_DATA {
    pub Parameter: usize,
    pub ContextRecord: super::PCONTEXT,
    pub Reserved0: usize,
    pub Reserved1: usize,
}
#[cfg(feature = "winnt")]
impl Default for APC_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
pub const KernelEnabled: MACHINE_ATTRIBUTES = 2;
#[cfg(feature = "winnt")]
pub type LPPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
pub type LPPROC_THREAD_ATTRIBUTE_LIST = *mut _PROC_THREAD_ATTRIBUTE_LIST;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPSTARTUPINFO = LPSTARTUPINFOA;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPSTARTUPINFOA = *mut STARTUPINFOA;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type LPSTARTUPINFOW = *mut STARTUPINFOW;
pub type MACHINE_ATTRIBUTES = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: u32,
}
pub type OVERRIDE_PREFETCH_PARAMETER = u32;
#[cfg(feature = "winnt")]
pub type PAPC_CALLBACK_DATA = *mut APC_CALLBACK_DATA;
pub type PAPP_MEMORY_INFORMATION = *mut APP_MEMORY_INFORMATION;
pub type PMEMORY_PRIORITY_INFORMATION = *mut MEMORY_PRIORITY_INFORMATION;
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = 0;
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = 1;
pub const PME_CURRENT_VERSION: u32 = 1;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1;
#[cfg(feature = "winnt")]
pub type PPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
pub type PPROCESS_LEAP_SECOND_INFO = *mut PROCESS_LEAP_SECOND_INFO;
pub type PPROCESS_MEMORY_EXHAUSTION_INFO = *mut PROCESS_MEMORY_EXHAUSTION_INFO;
pub type PPROCESS_MEMORY_EXHAUSTION_TYPE = *mut PROCESS_MEMORY_EXHAUSTION_TYPE;
pub type PPROCESS_POWER_THROTTLING_STATE = *mut PROCESS_POWER_THROTTLING_STATE;
pub type PPROC_THREAD_ATTRIBUTE_LIST = *mut _PROC_THREAD_ATTRIBUTE_LIST;
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::HANDLE,
    pub hThread: super::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
#[cfg(feature = "winnt")]
impl Default for PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROCESS_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: u16,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
pub type PROCESS_MEMORY_EXHAUSTION_TYPE = i32;
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
pub const PROCESS_POWER_THROTTLING_VALID_FLAGS: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: u32,
}
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1;
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
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type STARTUPINFO = STARTUPINFOA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: windows_sys::core::PSTR,
    pub lpDesktop: windows_sys::core::PSTR,
    pub lpTitle: windows_sys::core::PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: u32,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: super::LPBYTE,
    pub hStdInput: super::HANDLE,
    pub hStdOutput: super::HANDLE,
    pub hStdError: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for STARTUPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: windows_sys::core::PWSTR,
    pub lpDesktop: windows_sys::core::PWSTR,
    pub lpTitle: windows_sys::core::PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: u32,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: super::LPBYTE,
    pub hStdInput: super::HANDLE,
    pub hStdOutput: super::HANDLE,
    pub hStdError: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for STARTUPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type THREAD_INFORMATION_CLASS = i32;
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1;
pub const TLS_OUT_OF_INDEXES: u32 = 4294967295;
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
