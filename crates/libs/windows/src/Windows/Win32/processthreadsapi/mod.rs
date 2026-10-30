#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateProcessA<P0, P7>(lpapplicationname: P0, lpcommandline: Option<windows_core::PSTR>, lpprocessattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpthreadattributes: Option<super::LPSECURITY_ATTRIBUTES>, binherithandles: bool, dwcreationflags: super::DWORD, lpenvironment: Option<super::LPVOID>, lpcurrentdirectory: P7, lpstartupinfo: LPSTARTUPINFOA, lpprocessinformation: LPPROCESS_INFORMATION) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateProcessA(lpapplicationname : windows_core::PCSTR, lpcommandline : windows_core::PSTR, lpprocessattributes : super::LPSECURITY_ATTRIBUTES, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, binherithandles : windows_core::BOOL, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_core::PCSTR, lpstartupinfo : LPSTARTUPINFOA, lpprocessinformation : LPPROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessA(lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, lpprocessattributes.unwrap_or(core::mem::zeroed()) as _, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, binherithandles.into(), dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateProcessAsUserA<P1, P8>(htoken: Option<super::HANDLE>, lpapplicationname: P1, lpcommandline: Option<windows_core::PSTR>, lpprocessattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpthreadattributes: Option<super::LPSECURITY_ATTRIBUTES>, binherithandles: bool, dwcreationflags: super::DWORD, lpenvironment: Option<super::LPVOID>, lpcurrentdirectory: P8, lpstartupinfo: LPSTARTUPINFOA, lpprocessinformation: LPPROCESS_INFORMATION) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P8: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CreateProcessAsUserA(htoken : super::HANDLE, lpapplicationname : windows_core::PCSTR, lpcommandline : windows_core::PSTR, lpprocessattributes : super::LPSECURITY_ATTRIBUTES, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, binherithandles : windows_core::BOOL, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_core::PCSTR, lpstartupinfo : LPSTARTUPINFOA, lpprocessinformation : LPPROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessAsUserA(htoken.unwrap_or(core::mem::zeroed()) as _, lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, lpprocessattributes.unwrap_or(core::mem::zeroed()) as _, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, binherithandles.into(), dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateProcessAsUserW<P1, P8>(htoken: Option<super::HANDLE>, lpapplicationname: P1, lpcommandline: Option<windows_core::PWSTR>, lpprocessattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpthreadattributes: Option<super::LPSECURITY_ATTRIBUTES>, binherithandles: bool, dwcreationflags: super::DWORD, lpenvironment: Option<super::LPVOID>, lpcurrentdirectory: P8, lpstartupinfo: LPSTARTUPINFOW, lpprocessinformation: LPPROCESS_INFORMATION) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CreateProcessAsUserW(htoken : super::HANDLE, lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, lpprocessattributes : super::LPSECURITY_ATTRIBUTES, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, binherithandles : windows_core::BOOL, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : LPSTARTUPINFOW, lpprocessinformation : LPPROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessAsUserW(htoken.unwrap_or(core::mem::zeroed()) as _, lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, lpprocessattributes.unwrap_or(core::mem::zeroed()) as _, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, binherithandles.into(), dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateProcessW<P0, P7>(lpapplicationname: P0, lpcommandline: Option<windows_core::PWSTR>, lpprocessattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpthreadattributes: Option<super::LPSECURITY_ATTRIBUTES>, binherithandles: bool, dwcreationflags: super::DWORD, lpenvironment: Option<super::LPVOID>, lpcurrentdirectory: P7, lpstartupinfo: LPSTARTUPINFOW, lpprocessinformation: LPPROCESS_INFORMATION) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateProcessW(lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, lpprocessattributes : super::LPSECURITY_ATTRIBUTES, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, binherithandles : windows_core::BOOL, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : LPSTARTUPINFOW, lpprocessinformation : LPPROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessW(lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, lpprocessattributes.unwrap_or(core::mem::zeroed()) as _, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, binherithandles.into(), dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateRemoteThread(hprocess: super::HANDLE, lpthreadattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwstacksize: super::SIZE_T, lpstartaddress: super::LPTHREAD_START_ROUTINE, lpparameter: Option<super::LPVOID>, dwcreationflags: super::DWORD, lpthreadid: Option<super::LPDWORD>) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateRemoteThread(hprocess : super::HANDLE, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, dwstacksize : super::SIZE_T, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : super::LPVOID, dwcreationflags : super::DWORD, lpthreadid : super::LPDWORD) -> super::HANDLE);
    unsafe { CreateRemoteThread(hprocess, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, dwstacksize, lpstartaddress, lpparameter.unwrap_or(core::mem::zeroed()) as _, dwcreationflags, lpthreadid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateRemoteThreadEx(hprocess: super::HANDLE, lpthreadattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwstacksize: super::SIZE_T, lpstartaddress: super::LPTHREAD_START_ROUTINE, lpparameter: Option<super::LPVOID>, dwcreationflags: super::DWORD, lpattributelist: Option<LPPROC_THREAD_ATTRIBUTE_LIST>, lpthreadid: Option<super::LPDWORD>) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateRemoteThreadEx(hprocess : super::HANDLE, lpthreadattributes : super::LPSECURITY_ATTRIBUTES, dwstacksize : super::SIZE_T, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : super::LPVOID, dwcreationflags : super::DWORD, lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid : super::LPDWORD) -> super::HANDLE);
    unsafe { CreateRemoteThreadEx(hprocess, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, dwstacksize, lpstartaddress, lpparameter.unwrap_or(core::mem::zeroed()) as _, dwcreationflags, lpattributelist.unwrap_or(core::mem::zeroed()) as _, lpthreadid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateThread(lpthreadattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwstacksize: super::SIZE_T, lpstartaddress: super::LPTHREAD_START_ROUTINE, lpparameter: Option<super::LPVOID>, dwcreationflags: super::DWORD, lpthreadid: Option<super::LPDWORD>) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateThread(lpthreadattributes : super::LPSECURITY_ATTRIBUTES, dwstacksize : super::SIZE_T, lpstartaddress : super::LPTHREAD_START_ROUTINE, lpparameter : super::LPVOID, dwcreationflags : super::DWORD, lpthreadid : super::LPDWORD) -> super::HANDLE);
    unsafe { CreateThread(lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, dwstacksize, lpstartaddress, lpparameter.unwrap_or(core::mem::zeroed()) as _, dwcreationflags, lpthreadid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST) {
    windows_core::link!("kernel32.dll" "system" fn DeleteProcThreadAttributeList(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST));
    unsafe { DeleteProcThreadAttributeList(lpattributelist as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ExitProcess(uexitcode: super::UINT) -> ! {
    windows_core::link!("kernel32.dll" "system" fn ExitProcess(uexitcode : super::UINT) -> !);
    unsafe { ExitProcess(uexitcode) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ExitThread(dwexitcode: super::DWORD) -> ! {
    windows_core::link!("kernel32.dll" "system" fn ExitThread(dwexitcode : super::DWORD) -> !);
    unsafe { ExitThread(dwexitcode) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FlushInstructionCache(hprocess: super::HANDLE, lpbaseaddress: Option<super::LPCVOID>, dwsize: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlushInstructionCache(hprocess : super::HANDLE, lpbaseaddress : super::LPCVOID, dwsize : super::SIZE_T) -> windows_core::BOOL);
    unsafe { FlushInstructionCache(hprocess, lpbaseaddress.unwrap_or(core::mem::zeroed()) as _, dwsize) }
}
#[inline]
pub unsafe fn FlushProcessWriteBuffers() {
    windows_core::link!("kernel32.dll" "system" fn FlushProcessWriteBuffers());
    unsafe { FlushProcessWriteBuffers() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetCurrentProcess() -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcess() -> super::HANDLE);
    unsafe { GetCurrentProcess() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentProcessId() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> super::DWORD);
    unsafe { GetCurrentProcessId() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentProcessorNumber() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcessorNumber() -> super::DWORD);
    unsafe { GetCurrentProcessorNumber() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCurrentProcessorNumberEx(procnumber: super::PPROCESSOR_NUMBER) {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcessorNumberEx(procnumber : super::PPROCESSOR_NUMBER));
    unsafe { GetCurrentProcessorNumberEx(procnumber as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetCurrentThread() -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentThread() -> super::HANDLE);
    unsafe { GetCurrentThread() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentThreadId() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> super::DWORD);
    unsafe { GetCurrentThreadId() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentThreadStackLimits(lowlimit: super::PULONG_PTR, highlimit: super::PULONG_PTR) {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentThreadStackLimits(lowlimit : super::PULONG_PTR, highlimit : super::PULONG_PTR));
    unsafe { GetCurrentThreadStackLimits(lowlimit as _, highlimit as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetExitCodeProcess(hprocess: super::HANDLE, lpexitcode: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetExitCodeProcess(hprocess : super::HANDLE, lpexitcode : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetExitCodeProcess(hprocess, lpexitcode as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetExitCodeThread(hthread: super::HANDLE, lpexitcode: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetExitCodeThread(hthread : super::HANDLE, lpexitcode : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetExitCodeThread(hthread, lpexitcode as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetMachineTypeAttributes(machine: super::USHORT, machinetypeattributes: *mut MACHINE_ATTRIBUTES) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn GetMachineTypeAttributes(machine : super::USHORT, machinetypeattributes : *mut MACHINE_ATTRIBUTES) -> windows_core::HRESULT);
    unsafe { GetMachineTypeAttributes(machine, machinetypeattributes as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPriorityClass(hprocess: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetPriorityClass(hprocess : super::HANDLE) -> super::DWORD);
    unsafe { GetPriorityClass(hprocess) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessDefaultCpuSetMasks(process: super::HANDLE, cpusetmasks: Option<super::PGROUP_AFFINITY>, cpusetmaskcount: super::USHORT, requiredmaskcount: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSetMasks(process : super::HANDLE, cpusetmasks : super::PGROUP_AFFINITY, cpusetmaskcount : super::USHORT, requiredmaskcount : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetProcessDefaultCpuSetMasks(process, cpusetmasks.unwrap_or(core::mem::zeroed()) as _, cpusetmaskcount, requiredmaskcount as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessDefaultCpuSets(process: super::HANDLE, cpusetids: Option<super::PULONG>, cpusetidcount: super::ULONG, requiredidcount: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSets(process : super::HANDLE, cpusetids : super::PULONG, cpusetidcount : super::ULONG, requiredidcount : super::PULONG) -> windows_core::BOOL);
    unsafe { GetProcessDefaultCpuSets(process, cpusetids.unwrap_or(core::mem::zeroed()) as _, cpusetidcount, requiredidcount as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessHandleCount(hprocess: super::HANDLE, pdwhandlecount: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHandleCount(hprocess : super::HANDLE, pdwhandlecount : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetProcessHandleCount(hprocess, pdwhandlecount as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessId(process: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetProcessId(process : super::HANDLE) -> super::DWORD);
    unsafe { GetProcessId(process) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessIdOfThread(thread: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetProcessIdOfThread(thread : super::HANDLE) -> super::DWORD);
    unsafe { GetProcessIdOfThread(thread) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessInformation(hprocess: super::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: super::LPVOID, processinformationsize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessInformation(hprocess : super::HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : super::LPVOID, processinformationsize : super::DWORD) -> windows_core::BOOL);
    unsafe { GetProcessInformation(hprocess, processinformationclass, processinformation as _, processinformationsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessMitigationPolicy(hprocess: super::HANDLE, mitigationpolicy: super::PROCESS_MITIGATION_POLICY, lpbuffer: super::PVOID, dwlength: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessMitigationPolicy(hprocess : super::HANDLE, mitigationpolicy : super::PROCESS_MITIGATION_POLICY, lpbuffer : super::PVOID, dwlength : super::SIZE_T) -> windows_core::BOOL);
    unsafe { GetProcessMitigationPolicy(hprocess, mitigationpolicy, lpbuffer as _, dwlength) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessPriorityBoost(hprocess: super::HANDLE, pdisablepriorityboost: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessPriorityBoost(hprocess : super::HANDLE, pdisablepriorityboost : super::PBOOL) -> windows_core::BOOL);
    unsafe { GetProcessPriorityBoost(hprocess, pdisablepriorityboost as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetProcessShutdownParameters(lpdwlevel: super::LPDWORD, lpdwflags: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessShutdownParameters(lpdwlevel : super::LPDWORD, lpdwflags : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetProcessShutdownParameters(lpdwlevel as _, lpdwflags as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessTimes(hprocess: super::HANDLE, lpcreationtime: super::LPFILETIME, lpexittime: super::LPFILETIME, lpkerneltime: super::LPFILETIME, lpusertime: super::LPFILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessTimes(hprocess : super::HANDLE, lpcreationtime : super::LPFILETIME, lpexittime : super::LPFILETIME, lpkerneltime : super::LPFILETIME, lpusertime : super::LPFILETIME) -> windows_core::BOOL);
    unsafe { GetProcessTimes(hprocess, lpcreationtime as _, lpexittime as _, lpkerneltime as _, lpusertime as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetProcessVersion(processid: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetProcessVersion(processid : super::DWORD) -> super::DWORD);
    unsafe { GetProcessVersion(processid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetStartupInfoW(lpstartupinfo: LPSTARTUPINFOW) {
    windows_core::link!("kernel32.dll" "system" fn GetStartupInfoW(lpstartupinfo : LPSTARTUPINFOW));
    unsafe { GetStartupInfoW(lpstartupinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetSystemCpuSetInformation(information: Option<super::PSYSTEM_CPU_SET_INFORMATION>, bufferlength: super::ULONG, returnedlength: super::PULONG, process: Option<super::HANDLE>, flags: Option<super::ULONG>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemCpuSetInformation(information : super::PSYSTEM_CPU_SET_INFORMATION, bufferlength : super::ULONG, returnedlength : super::PULONG, process : super::HANDLE, flags : super::ULONG) -> windows_core::BOOL);
    unsafe { GetSystemCpuSetInformation(information.unwrap_or(core::mem::zeroed()) as _, bufferlength, returnedlength as _, process.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetSystemTimes(lpidletime: Option<super::PFILETIME>, lpkerneltime: Option<super::PFILETIME>, lpusertime: Option<super::PFILETIME>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimes(lpidletime : super::PFILETIME, lpkerneltime : super::PFILETIME, lpusertime : super::PFILETIME) -> windows_core::BOOL);
    unsafe { GetSystemTimes(lpidletime.unwrap_or(core::mem::zeroed()) as _, lpkerneltime.unwrap_or(core::mem::zeroed()) as _, lpusertime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadContext(hthread: super::HANDLE, lpcontext: super::LPCONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadContext(hthread : super::HANDLE, lpcontext : super::LPCONTEXT) -> windows_core::BOOL);
    unsafe { GetThreadContext(hthread, lpcontext as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetThreadDescription(hthread: super::HANDLE, ppszthreaddescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn GetThreadDescription(hthread : super::HANDLE, ppszthreaddescription : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { GetThreadDescription(hthread, ppszthreaddescription as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadIOPendingFlag(hthread: super::HANDLE, lpioispending: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadIOPendingFlag(hthread : super::HANDLE, lpioispending : super::PBOOL) -> windows_core::BOOL);
    unsafe { GetThreadIOPendingFlag(hthread, lpioispending as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadId(thread: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetThreadId(thread : super::HANDLE) -> super::DWORD);
    unsafe { GetThreadId(thread) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadIdealProcessorEx(hthread: super::HANDLE, lpidealprocessor: super::PPROCESSOR_NUMBER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadIdealProcessorEx(hthread : super::HANDLE, lpidealprocessor : super::PPROCESSOR_NUMBER) -> windows_core::BOOL);
    unsafe { GetThreadIdealProcessorEx(hthread, lpidealprocessor as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadInformation(hthread: super::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: super::LPVOID, threadinformationsize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadInformation(hthread : super::HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : super::LPVOID, threadinformationsize : super::DWORD) -> windows_core::BOOL);
    unsafe { GetThreadInformation(hthread, threadinformationclass, threadinformation as _, threadinformationsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetThreadPriority(hthread: super::HANDLE) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetThreadPriority(hthread : super::HANDLE) -> i32);
    unsafe { GetThreadPriority(hthread) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadPriorityBoost(hthread: super::HANDLE, pdisablepriorityboost: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadPriorityBoost(hthread : super::HANDLE, pdisablepriorityboost : super::PBOOL) -> windows_core::BOOL);
    unsafe { GetThreadPriorityBoost(hthread, pdisablepriorityboost as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadSelectedCpuSetMasks(thread: super::HANDLE, cpusetmasks: Option<super::PGROUP_AFFINITY>, cpusetmaskcount: super::USHORT, requiredmaskcount: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSetMasks(thread : super::HANDLE, cpusetmasks : super::PGROUP_AFFINITY, cpusetmaskcount : super::USHORT, requiredmaskcount : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetThreadSelectedCpuSetMasks(thread, cpusetmasks.unwrap_or(core::mem::zeroed()) as _, cpusetmaskcount, requiredmaskcount as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadSelectedCpuSets(thread: super::HANDLE, cpusetids: Option<super::PULONG>, cpusetidcount: super::ULONG, requiredidcount: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSets(thread : super::HANDLE, cpusetids : super::PULONG, cpusetidcount : super::ULONG, requiredidcount : super::PULONG) -> windows_core::BOOL);
    unsafe { GetThreadSelectedCpuSets(thread, cpusetids.unwrap_or(core::mem::zeroed()) as _, cpusetidcount, requiredidcount as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadTimes(hthread: super::HANDLE, lpcreationtime: super::LPFILETIME, lpexittime: super::LPFILETIME, lpkerneltime: super::LPFILETIME, lpusertime: super::LPFILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadTimes(hthread : super::HANDLE, lpcreationtime : super::LPFILETIME, lpexittime : super::LPFILETIME, lpkerneltime : super::LPFILETIME, lpusertime : super::LPFILETIME) -> windows_core::BOOL);
    unsafe { GetThreadTimes(hthread, lpcreationtime as _, lpexittime as _, lpkerneltime as _, lpusertime as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[inline]
pub unsafe fn InitializeProcThreadAttributeList(lpattributelist: Option<LPPROC_THREAD_ATTRIBUTE_LIST>, dwattributecount: super::DWORD, dwflags: Option<super::DWORD>, lpsize: super::PSIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeProcThreadAttributeList(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount : super::DWORD, dwflags : super::DWORD, lpsize : super::PSIZE_T) -> windows_core::BOOL);
    unsafe { InitializeProcThreadAttributeList(lpattributelist.unwrap_or(core::mem::zeroed()) as _, dwattributecount, dwflags.unwrap_or(core::mem::zeroed()) as _, lpsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn IsProcessCritical(hprocess: super::HANDLE, critical: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsProcessCritical(hprocess : super::HANDLE, critical : super::PBOOL) -> windows_core::BOOL);
    unsafe { IsProcessCritical(hprocess, critical as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsProcessorFeaturePresent(processorfeature: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsProcessorFeaturePresent(processorfeature : super::DWORD) -> windows_core::BOOL);
    unsafe { IsProcessorFeaturePresent(processorfeature) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenProcess(dwdesiredaccess: super::DWORD, binherithandle: bool, dwprocessid: super::DWORD) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, dwprocessid : super::DWORD) -> super::HANDLE);
    unsafe { OpenProcess(dwdesiredaccess, binherithandle.into(), dwprocessid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenProcessToken(processhandle: super::HANDLE, desiredaccess: super::DWORD, tokenhandle: super::PHANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : super::HANDLE, desiredaccess : super::DWORD, tokenhandle : super::PHANDLE) -> windows_core::BOOL);
    unsafe { OpenProcessToken(processhandle, desiredaccess, tokenhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenThread(dwdesiredaccess: super::DWORD, binherithandle: bool, dwthreadid: super::DWORD) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn OpenThread(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, dwthreadid : super::DWORD) -> super::HANDLE);
    unsafe { OpenThread(dwdesiredaccess, binherithandle.into(), dwthreadid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenThreadToken(threadhandle: super::HANDLE, desiredaccess: super::DWORD, openasself: bool, tokenhandle: super::PHANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OpenThreadToken(threadhandle : super::HANDLE, desiredaccess : super::DWORD, openasself : windows_core::BOOL, tokenhandle : super::PHANDLE) -> windows_core::BOOL);
    unsafe { OpenThreadToken(threadhandle, desiredaccess, openasself.into(), tokenhandle as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ProcessIdToSessionId(dwprocessid: super::DWORD, psessionid: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ProcessIdToSessionId(dwprocessid : super::DWORD, psessionid : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { ProcessIdToSessionId(dwprocessid, psessionid as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn QueryProcessAffinityUpdateMode(hprocess: super::HANDLE, lpdwflags: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryProcessAffinityUpdateMode(hprocess : super::HANDLE, lpdwflags : super::LPDWORD) -> windows_core::BOOL);
    unsafe { QueryProcessAffinityUpdateMode(hprocess, lpdwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn QueryProtectedPolicy(policyguid: super::LPCGUID, policyvalue: super::PULONG_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryProtectedPolicy(policyguid : super::LPCGUID, policyvalue : super::PULONG_PTR) -> windows_core::BOOL);
    unsafe { QueryProtectedPolicy(policyguid, policyvalue as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueueUserAPC(pfnapc: super::PAPCFUNC, hthread: super::HANDLE, dwdata: super::ULONG_PTR) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn QueueUserAPC(pfnapc : super::PAPCFUNC, hthread : super::HANDLE, dwdata : super::ULONG_PTR) -> super::DWORD);
    unsafe { QueueUserAPC(pfnapc, hthread, dwdata) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueueUserAPC2(apcroutine: super::PAPCFUNC, thread: super::HANDLE, data: super::ULONG_PTR, flags: QUEUE_USER_APC_FLAGS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueueUserAPC2(apcroutine : super::PAPCFUNC, thread : super::HANDLE, data : super::ULONG_PTR, flags : QUEUE_USER_APC_FLAGS) -> windows_core::BOOL);
    unsafe { QueueUserAPC2(apcroutine, thread, data, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ResumeThread(hthread: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn ResumeThread(hthread : super::HANDLE) -> super::DWORD);
    unsafe { ResumeThread(hthread) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetPriorityClass(hprocess: super::HANDLE, dwpriorityclass: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetPriorityClass(hprocess : super::HANDLE, dwpriorityclass : super::DWORD) -> windows_core::BOOL);
    unsafe { SetPriorityClass(hprocess, dwpriorityclass) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessAffinityUpdateMode(hprocess: super::HANDLE, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessAffinityUpdateMode(hprocess : super::HANDLE, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { SetProcessAffinityUpdateMode(hprocess, dwflags) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessDefaultCpuSetMasks(process: super::HANDLE, cpusetmasks: Option<&[super::GROUP_AFFINITY]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSetMasks(process : super::HANDLE, cpusetmasks : super::PGROUP_AFFINITY, cpusetmaskcount : super::USHORT) -> windows_core::BOOL);
    unsafe { SetProcessDefaultCpuSetMasks(process, core::mem::transmute(cpusetmasks.map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessDefaultCpuSets(process: super::HANDLE, cpusetids: Option<&[super::ULONG]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSets(process : super::HANDLE, cpusetids : *const super::ULONG, cpusetidcount : super::ULONG) -> windows_core::BOOL);
    unsafe { SetProcessDefaultCpuSets(process, cpusetids.map_or(core::ptr::null(), |slice| slice.as_ptr()), cpusetids.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessDynamicEHContinuationTargets(process: super::HANDLE, targets: &mut [super::PROCESS_DYNAMIC_EH_CONTINUATION_TARGET]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDynamicEHContinuationTargets(process : super::HANDLE, numberoftargets : super::USHORT, targets : super::PPROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> windows_core::BOOL);
    unsafe { SetProcessDynamicEHContinuationTargets(process, targets.len().try_into().unwrap(), core::mem::transmute(targets.as_mut_ptr())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessDynamicEnforcedCetCompatibleRanges(process: super::HANDLE, ranges: &mut [super::PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDynamicEnforcedCetCompatibleRanges(process : super::HANDLE, numberofranges : super::USHORT, ranges : super::PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> windows_core::BOOL);
    unsafe { SetProcessDynamicEnforcedCetCompatibleRanges(process, ranges.len().try_into().unwrap(), core::mem::transmute(ranges.as_mut_ptr())) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessInformation(hprocess: super::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: super::LPVOID, processinformationsize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessInformation(hprocess : super::HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : super::LPVOID, processinformationsize : super::DWORD) -> windows_core::BOOL);
    unsafe { SetProcessInformation(hprocess, processinformationclass, processinformation, processinformationsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessMitigationPolicy(mitigationpolicy: super::PROCESS_MITIGATION_POLICY, lpbuffer: super::PVOID, dwlength: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessMitigationPolicy(mitigationpolicy : super::PROCESS_MITIGATION_POLICY, lpbuffer : super::PVOID, dwlength : super::SIZE_T) -> windows_core::BOOL);
    unsafe { SetProcessMitigationPolicy(mitigationpolicy, lpbuffer, dwlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetProcessPriorityBoost(hprocess: super::HANDLE, bdisablepriorityboost: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessPriorityBoost(hprocess : super::HANDLE, bdisablepriorityboost : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetProcessPriorityBoost(hprocess, bdisablepriorityboost.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetProcessShutdownParameters(dwlevel: super::DWORD, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessShutdownParameters(dwlevel : super::DWORD, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { SetProcessShutdownParameters(dwlevel, dwflags) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn SetProtectedPolicy(policyguid: super::LPCGUID, policyvalue: super::ULONG_PTR, oldpolicyvalue: Option<super::PULONG_PTR>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProtectedPolicy(policyguid : super::LPCGUID, policyvalue : super::ULONG_PTR, oldpolicyvalue : super::PULONG_PTR) -> windows_core::BOOL);
    unsafe { SetProtectedPolicy(policyguid, policyvalue, oldpolicyvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadContext(hthread: super::HANDLE, lpcontext: *const super::CONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadContext(hthread : super::HANDLE, lpcontext : *const super::CONTEXT) -> windows_core::BOOL);
    unsafe { SetThreadContext(hthread, lpcontext) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetThreadDescription<P1>(hthread: super::HANDLE, lpthreaddescription: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetThreadDescription(hthread : super::HANDLE, lpthreaddescription : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SetThreadDescription(hthread, lpthreaddescription.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadIdealProcessor(hthread: super::HANDLE, dwidealprocessor: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn SetThreadIdealProcessor(hthread : super::HANDLE, dwidealprocessor : super::DWORD) -> super::DWORD);
    unsafe { SetThreadIdealProcessor(hthread, dwidealprocessor) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadIdealProcessorEx(hthread: super::HANDLE, lpidealprocessor: super::PPROCESSOR_NUMBER, lppreviousidealprocessor: Option<super::PPROCESSOR_NUMBER>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadIdealProcessorEx(hthread : super::HANDLE, lpidealprocessor : super::PPROCESSOR_NUMBER, lppreviousidealprocessor : super::PPROCESSOR_NUMBER) -> windows_core::BOOL);
    unsafe { SetThreadIdealProcessorEx(hthread, lpidealprocessor, lppreviousidealprocessor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadInformation(hthread: super::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: super::LPVOID, threadinformationsize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadInformation(hthread : super::HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : super::LPVOID, threadinformationsize : super::DWORD) -> windows_core::BOOL);
    unsafe { SetThreadInformation(hthread, threadinformationclass, threadinformation, threadinformationsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetThreadPriority(hthread: super::HANDLE, npriority: i32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadPriority(hthread : super::HANDLE, npriority : i32) -> windows_core::BOOL);
    unsafe { SetThreadPriority(hthread, npriority) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetThreadPriorityBoost(hthread: super::HANDLE, bdisablepriorityboost: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadPriorityBoost(hthread : super::HANDLE, bdisablepriorityboost : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetThreadPriorityBoost(hthread, bdisablepriorityboost.into()) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadSelectedCpuSetMasks(thread: super::HANDLE, cpusetmasks: Option<&[super::GROUP_AFFINITY]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSetMasks(thread : super::HANDLE, cpusetmasks : super::PGROUP_AFFINITY, cpusetmaskcount : super::USHORT) -> windows_core::BOOL);
    unsafe { SetThreadSelectedCpuSetMasks(thread, core::mem::transmute(cpusetmasks.map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadSelectedCpuSets(thread: super::HANDLE, cpusetids: &[super::ULONG]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSets(thread : super::HANDLE, cpusetids : *const super::ULONG, cpusetidcount : super::ULONG) -> windows_core::BOOL);
    unsafe { SetThreadSelectedCpuSets(thread, cpusetids.as_ptr(), cpusetids.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SetThreadStackGuarantee(stacksizeinbytes: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadStackGuarantee(stacksizeinbytes : super::PULONG) -> windows_core::BOOL);
    unsafe { SetThreadStackGuarantee(stacksizeinbytes as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetThreadToken(thread: Option<super::PHANDLE>, token: Option<super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetThreadToken(thread : super::PHANDLE, token : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetThreadToken(thread.unwrap_or(core::mem::zeroed()) as _, token.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SuspendThread(hthread: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn SuspendThread(hthread : super::HANDLE) -> super::DWORD);
    unsafe { SuspendThread(hthread) }
}
#[inline]
pub unsafe fn SwitchToThread() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SwitchToThread() -> windows_core::BOOL);
    unsafe { SwitchToThread() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TerminateProcess(hprocess: super::HANDLE, uexitcode: super::UINT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TerminateProcess(hprocess : super::HANDLE, uexitcode : super::UINT) -> windows_core::BOOL);
    unsafe { TerminateProcess(hprocess, uexitcode) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TerminateThread(hthread: super::HANDLE, dwexitcode: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TerminateThread(hthread : super::HANDLE, dwexitcode : super::DWORD) -> windows_core::BOOL);
    unsafe { TerminateThread(hthread, dwexitcode) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn TlsAlloc() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn TlsAlloc() -> super::DWORD);
    unsafe { TlsAlloc() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn TlsFree(dwtlsindex: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TlsFree(dwtlsindex : super::DWORD) -> windows_core::BOOL);
    unsafe { TlsFree(dwtlsindex) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn TlsGetValue(dwtlsindex: super::DWORD) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn TlsGetValue(dwtlsindex : super::DWORD) -> super::LPVOID);
    unsafe { TlsGetValue(dwtlsindex) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn TlsGetValue2(dwtlsindex: super::DWORD) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn TlsGetValue2(dwtlsindex : super::DWORD) -> super::LPVOID);
    unsafe { TlsGetValue2(dwtlsindex) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn TlsSetValue(dwtlsindex: super::DWORD, lptlsvalue: Option<super::LPVOID>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TlsSetValue(dwtlsindex : super::DWORD, lptlsvalue : super::LPVOID) -> windows_core::BOOL);
    unsafe { TlsSetValue(dwtlsindex, lptlsvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UpdateProcThreadAttribute(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwflags: super::DWORD, attribute: super::DWORD_PTR, lpvalue: Option<super::PVOID>, cbsize: super::SIZE_T, lppreviousvalue: Option<super::PVOID>, lpreturnsize: Option<super::PSIZE_T>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UpdateProcThreadAttribute(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, dwflags : super::DWORD, attribute : super::DWORD_PTR, lpvalue : super::PVOID, cbsize : super::SIZE_T, lppreviousvalue : super::PVOID, lpreturnsize : super::PSIZE_T) -> windows_core::BOOL);
    unsafe { UpdateProcThreadAttribute(lpattributelist as _, dwflags, attribute, lpvalue.unwrap_or(core::mem::zeroed()) as _, cbsize, lppreviousvalue.unwrap_or(core::mem::zeroed()) as _, lpreturnsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APC_CALLBACK_DATA {
    pub Parameter: super::ULONG_PTR,
    pub ContextRecord: super::PCONTEXT,
    pub Reserved0: super::ULONG_PTR,
    pub Reserved1: super::ULONG_PTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::HANDLE,
    pub hThread: super::HANDLE,
    pub dwProcessId: super::DWORD,
    pub dwThreadId: super::DWORD,
}
pub type PROCESS_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: super::ULONG,
    pub Reserved: super::ULONG,
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: i32 = 1;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: super::USHORT,
    pub Res0: super::USHORT,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: super::ULONG,
    pub ControlMask: super::ULONG,
    pub StateMask: super::ULONG,
}
pub const PROCESS_POWER_THROTTLING_VALID_FLAGS: i32 = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STARTUPINFOA {
    pub cb: super::DWORD,
    pub lpReserved: windows_core::PSTR,
    pub lpDesktop: windows_core::PSTR,
    pub lpTitle: windows_core::PSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STARTUPINFOW {
    pub cb: super::DWORD,
    pub lpReserved: windows_core::PWSTR,
    pub lpDesktop: windows_core::PWSTR,
    pub lpTitle: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _PROC_THREAD_ATTRIBUTE_LIST(pub u8);
