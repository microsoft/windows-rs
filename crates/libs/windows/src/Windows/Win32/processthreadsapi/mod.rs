#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateProcessA<P0, P7>(lpapplicationname: P0, lpcommandline: Option<windows_core::PSTR>, lpprocessattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpthreadattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, binherithandles: bool, dwcreationflags: u32, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P7, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateProcessA(lpapplicationname : windows_core::PCSTR, lpcommandline : windows_core::PSTR, lpprocessattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpthreadattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, binherithandles : windows_core::BOOL, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCSTR, lpstartupinfo : *const STARTUPINFOA, lpprocessinformation : *mut PROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessA(lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, lpprocessattributes.unwrap_or(core::mem::zeroed()) as _, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, binherithandles.into(), dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateProcessAsUserA<P1, P8>(htoken: Option<super::winnt::HANDLE>, lpapplicationname: P1, lpcommandline: Option<windows_core::PSTR>, lpprocessattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpthreadattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, binherithandles: bool, dwcreationflags: u32, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P8, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P8: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CreateProcessAsUserA(htoken : super::winnt::HANDLE, lpapplicationname : windows_core::PCSTR, lpcommandline : windows_core::PSTR, lpprocessattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpthreadattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, binherithandles : windows_core::BOOL, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCSTR, lpstartupinfo : *const STARTUPINFOA, lpprocessinformation : *mut PROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessAsUserA(htoken.unwrap_or(core::mem::zeroed()) as _, lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, lpprocessattributes.unwrap_or(core::mem::zeroed()) as _, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, binherithandles.into(), dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateProcessAsUserW<P1, P8>(htoken: Option<super::winnt::HANDLE>, lpapplicationname: P1, lpcommandline: Option<windows_core::PWSTR>, lpprocessattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpthreadattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, binherithandles: bool, dwcreationflags: u32, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P8, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CreateProcessAsUserW(htoken : super::winnt::HANDLE, lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, lpprocessattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpthreadattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, binherithandles : windows_core::BOOL, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessAsUserW(htoken.unwrap_or(core::mem::zeroed()) as _, lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, lpprocessattributes.unwrap_or(core::mem::zeroed()) as _, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, binherithandles.into(), dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateProcessW<P0, P7>(lpapplicationname: P0, lpcommandline: Option<windows_core::PWSTR>, lpprocessattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, lpthreadattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, binherithandles: bool, dwcreationflags: u32, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P7, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateProcessW(lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, lpprocessattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, lpthreadattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, binherithandles : windows_core::BOOL, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessW(lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, lpprocessattributes.unwrap_or(core::mem::zeroed()) as _, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, binherithandles.into(), dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateRemoteThread(hprocess: super::winnt::HANDLE, lpthreadattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: super::minwinbase::LPTHREAD_START_ROUTINE, lpparameter: Option<*const core::ffi::c_void>, dwcreationflags: u32, lpthreadid: Option<*mut u32>) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateRemoteThread(hprocess : super::winnt::HANDLE, lpthreadattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : super::minwinbase::LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpthreadid : *mut u32) -> super::winnt::HANDLE);
    unsafe { CreateRemoteThread(hprocess, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, dwstacksize, lpstartaddress, lpparameter.unwrap_or(core::mem::zeroed()) as _, dwcreationflags, lpthreadid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateRemoteThreadEx(hprocess: super::winnt::HANDLE, lpthreadattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: super::minwinbase::LPTHREAD_START_ROUTINE, lpparameter: Option<*const core::ffi::c_void>, dwcreationflags: u32, lpattributelist: Option<*const _PROC_THREAD_ATTRIBUTE_LIST>, lpthreadid: Option<*mut u32>) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateRemoteThreadEx(hprocess : super::winnt::HANDLE, lpthreadattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : super::minwinbase::LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpattributelist : *const _PROC_THREAD_ATTRIBUTE_LIST, lpthreadid : *mut u32) -> super::winnt::HANDLE);
    unsafe { CreateRemoteThreadEx(hprocess, lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, dwstacksize, lpstartaddress, lpparameter.unwrap_or(core::mem::zeroed()) as _, dwcreationflags, lpattributelist.unwrap_or(core::mem::zeroed()) as _, lpthreadid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateThread(lpthreadattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: super::minwinbase::LPTHREAD_START_ROUTINE, lpparameter: Option<*const core::ffi::c_void>, dwcreationflags: u32, lpthreadid: Option<*mut u32>) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateThread(lpthreadattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : super::minwinbase::LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpthreadid : *mut u32) -> super::winnt::HANDLE);
    unsafe { CreateThread(lpthreadattributes.unwrap_or(core::mem::zeroed()) as _, dwstacksize, lpstartaddress, lpparameter.unwrap_or(core::mem::zeroed()) as _, dwcreationflags, lpthreadid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteProcThreadAttributeList(lpattributelist: *mut _PROC_THREAD_ATTRIBUTE_LIST) {
    windows_core::link!("kernel32.dll" "system" fn DeleteProcThreadAttributeList(lpattributelist : *mut _PROC_THREAD_ATTRIBUTE_LIST));
    unsafe { DeleteProcThreadAttributeList(lpattributelist as _) }
}
#[inline]
pub unsafe fn ExitProcess(uexitcode: u32) -> ! {
    windows_core::link!("kernel32.dll" "system" fn ExitProcess(uexitcode : u32) -> !);
    unsafe { ExitProcess(uexitcode) }
}
#[inline]
pub unsafe fn ExitThread(dwexitcode: u32) -> ! {
    windows_core::link!("kernel32.dll" "system" fn ExitThread(dwexitcode : u32) -> !);
    unsafe { ExitThread(dwexitcode) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn FlushInstructionCache(hprocess: super::winnt::HANDLE, lpbaseaddress: Option<*const core::ffi::c_void>, dwsize: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlushInstructionCache(hprocess : super::winnt::HANDLE, lpbaseaddress : *const core::ffi::c_void, dwsize : usize) -> windows_core::BOOL);
    unsafe { FlushInstructionCache(hprocess, lpbaseaddress.unwrap_or(core::mem::zeroed()) as _, dwsize) }
}
#[inline]
pub unsafe fn FlushProcessWriteBuffers() {
    windows_core::link!("kernel32.dll" "system" fn FlushProcessWriteBuffers());
    unsafe { FlushProcessWriteBuffers() }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetCurrentProcess() -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcess() -> super::winnt::HANDLE);
    unsafe { GetCurrentProcess() }
}
#[inline]
pub unsafe fn GetCurrentProcessId() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);
    unsafe { GetCurrentProcessId() }
}
#[inline]
pub unsafe fn GetCurrentProcessorNumber() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcessorNumber() -> u32);
    unsafe { GetCurrentProcessorNumber() }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetCurrentProcessorNumberEx() -> super::winnt::PROCESSOR_NUMBER {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcessorNumberEx(procnumber : *mut super::winnt::PROCESSOR_NUMBER));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetCurrentProcessorNumberEx(&mut result__);
        result__
    }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetCurrentThread() -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentThread() -> super::winnt::HANDLE);
    unsafe { GetCurrentThread() }
}
#[inline]
pub unsafe fn GetCurrentThreadId() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> u32);
    unsafe { GetCurrentThreadId() }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn GetCurrentThreadStackLimits(lowlimit: *mut u32, highlimit: *mut u32) {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentThreadStackLimits(lowlimit : *mut u32, highlimit : *mut u32));
    unsafe { GetCurrentThreadStackLimits(lowlimit as _, highlimit as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetCurrentThreadStackLimits(lowlimit: *mut u64, highlimit: *mut u64) {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentThreadStackLimits(lowlimit : *mut u64, highlimit : *mut u64));
    unsafe { GetCurrentThreadStackLimits(lowlimit as _, highlimit as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetExitCodeProcess(hprocess: super::winnt::HANDLE, lpexitcode: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetExitCodeProcess(hprocess : super::winnt::HANDLE, lpexitcode : *mut u32) -> windows_core::BOOL);
    unsafe { GetExitCodeProcess(hprocess, lpexitcode as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetExitCodeThread(hthread: super::winnt::HANDLE, lpexitcode: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetExitCodeThread(hthread : super::winnt::HANDLE, lpexitcode : *mut u32) -> windows_core::BOOL);
    unsafe { GetExitCodeThread(hthread, lpexitcode as _) }
}
#[inline]
pub unsafe fn GetMachineTypeAttributes(machine: u16) -> windows_core::Result<MACHINE_ATTRIBUTES> {
    windows_core::link!("kernel32.dll" "system" fn GetMachineTypeAttributes(machine : u16, machinetypeattributes : *mut MACHINE_ATTRIBUTES) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetMachineTypeAttributes(machine, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetPriorityClass(hprocess: super::winnt::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetPriorityClass(hprocess : super::winnt::HANDLE) -> u32);
    unsafe { GetPriorityClass(hprocess) }
}
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetProcessDefaultCpuSetMasks(process: super::winnt::HANDLE, cpusetmasks: Option<&mut [super::winnt::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSetMasks(process : super::winnt::HANDLE, cpusetmasks : *mut super::winnt::GROUP_AFFINITY, cpusetmaskcount : u16, requiredmaskcount : *mut u16) -> windows_core::BOOL);
    unsafe { GetProcessDefaultCpuSetMasks(process, core::mem::transmute(cpusetmasks.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredmaskcount as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessDefaultCpuSets(process: super::winnt::HANDLE, cpusetids: Option<&mut [u32]>, requiredidcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSets(process : super::winnt::HANDLE, cpusetids : *mut u32, cpusetidcount : u32, requiredidcount : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessDefaultCpuSets(process, core::mem::transmute(cpusetids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredidcount as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessHandleCount(hprocess: super::winnt::HANDLE, pdwhandlecount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHandleCount(hprocess : super::winnt::HANDLE, pdwhandlecount : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessHandleCount(hprocess, pdwhandlecount as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessId(process: super::winnt::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetProcessId(process : super::winnt::HANDLE) -> u32);
    unsafe { GetProcessId(process) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessIdOfThread(thread: super::winnt::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetProcessIdOfThread(thread : super::winnt::HANDLE) -> u32);
    unsafe { GetProcessIdOfThread(thread) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessInformation(hprocess: super::winnt::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut core::ffi::c_void, processinformationsize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessInformation(hprocess : super::winnt::HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : *mut core::ffi::c_void, processinformationsize : u32) -> windows_core::BOOL);
    unsafe { GetProcessInformation(hprocess, processinformationclass, processinformation as _, processinformationsize) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessMitigationPolicy(hprocess: super::winnt::HANDLE, mitigationpolicy: super::winnt::PROCESS_MITIGATION_POLICY, lpbuffer: *mut core::ffi::c_void, dwlength: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessMitigationPolicy(hprocess : super::winnt::HANDLE, mitigationpolicy : super::winnt::PROCESS_MITIGATION_POLICY, lpbuffer : *mut core::ffi::c_void, dwlength : usize) -> windows_core::BOOL);
    unsafe { GetProcessMitigationPolicy(hprocess, mitigationpolicy, lpbuffer as _, dwlength) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessPriorityBoost(hprocess: super::winnt::HANDLE, pdisablepriorityboost: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessPriorityBoost(hprocess : super::winnt::HANDLE, pdisablepriorityboost : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetProcessPriorityBoost(hprocess, pdisablepriorityboost as _) }
}
#[inline]
pub unsafe fn GetProcessShutdownParameters(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessShutdownParameters(lpdwlevel : *mut u32, lpdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessShutdownParameters(lpdwlevel as _, lpdwflags as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetProcessTimes(hprocess: super::winnt::HANDLE, lpcreationtime: *mut super::minwindef::FILETIME, lpexittime: *mut super::minwindef::FILETIME, lpkerneltime: *mut super::minwindef::FILETIME, lpusertime: *mut super::minwindef::FILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessTimes(hprocess : super::winnt::HANDLE, lpcreationtime : *mut super::minwindef::FILETIME, lpexittime : *mut super::minwindef::FILETIME, lpkerneltime : *mut super::minwindef::FILETIME, lpusertime : *mut super::minwindef::FILETIME) -> windows_core::BOOL);
    unsafe { GetProcessTimes(hprocess, lpcreationtime as _, lpexittime as _, lpkerneltime as _, lpusertime as _) }
}
#[inline]
pub unsafe fn GetProcessVersion(processid: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetProcessVersion(processid : u32) -> u32);
    unsafe { GetProcessVersion(processid) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW) {
    windows_core::link!("kernel32.dll" "system" fn GetStartupInfoW(lpstartupinfo : *mut STARTUPINFOW));
    unsafe { GetStartupInfoW(lpstartupinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetSystemCpuSetInformation(information: Option<*mut super::winnt::SYSTEM_CPU_SET_INFORMATION>, bufferlength: u32, returnedlength: *mut u32, process: Option<super::winnt::HANDLE>, flags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemCpuSetInformation(information : *mut super::winnt::SYSTEM_CPU_SET_INFORMATION, bufferlength : u32, returnedlength : *mut u32, process : super::winnt::HANDLE, flags : u32) -> windows_core::BOOL);
    unsafe { GetSystemCpuSetInformation(information.unwrap_or(core::mem::zeroed()) as _, bufferlength, returnedlength as _, process.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn GetSystemTimes(lpidletime: Option<*mut super::minwindef::FILETIME>, lpkerneltime: Option<*mut super::minwindef::FILETIME>, lpusertime: Option<*mut super::minwindef::FILETIME>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimes(lpidletime : *mut super::minwindef::FILETIME, lpkerneltime : *mut super::minwindef::FILETIME, lpusertime : *mut super::minwindef::FILETIME) -> windows_core::BOOL);
    unsafe { GetSystemTimes(lpidletime.unwrap_or(core::mem::zeroed()) as _, lpkerneltime.unwrap_or(core::mem::zeroed()) as _, lpusertime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetThreadContext(hthread: super::winnt::HANDLE, lpcontext: super::minwinbase::LPCONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadContext(hthread : super::winnt::HANDLE, lpcontext : super::minwinbase::LPCONTEXT) -> windows_core::BOOL);
    unsafe { GetThreadContext(hthread, lpcontext as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetThreadDescription(hthread: super::winnt::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("kernel32.dll" "system" fn GetThreadDescription(hthread : super::winnt::HANDLE, ppszthreaddescription : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetThreadDescription(hthread, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetThreadIOPendingFlag(hthread: super::winnt::HANDLE, lpioispending: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadIOPendingFlag(hthread : super::winnt::HANDLE, lpioispending : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetThreadIOPendingFlag(hthread, lpioispending as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetThreadId(thread: super::winnt::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetThreadId(thread : super::winnt::HANDLE) -> u32);
    unsafe { GetThreadId(thread) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetThreadIdealProcessorEx(hthread: super::winnt::HANDLE, lpidealprocessor: *mut super::winnt::PROCESSOR_NUMBER) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadIdealProcessorEx(hthread : super::winnt::HANDLE, lpidealprocessor : *mut super::winnt::PROCESSOR_NUMBER) -> windows_core::BOOL);
    unsafe { GetThreadIdealProcessorEx(hthread, lpidealprocessor as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetThreadInformation(hthread: super::winnt::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut core::ffi::c_void, threadinformationsize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadInformation(hthread : super::winnt::HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : *mut core::ffi::c_void, threadinformationsize : u32) -> windows_core::BOOL);
    unsafe { GetThreadInformation(hthread, threadinformationclass, threadinformation as _, threadinformationsize) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetThreadPriority(hthread: super::winnt::HANDLE) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn GetThreadPriority(hthread : super::winnt::HANDLE) -> i32);
    unsafe { GetThreadPriority(hthread) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetThreadPriorityBoost(hthread: super::winnt::HANDLE, pdisablepriorityboost: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadPriorityBoost(hthread : super::winnt::HANDLE, pdisablepriorityboost : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetThreadPriorityBoost(hthread, pdisablepriorityboost as _) }
}
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetThreadSelectedCpuSetMasks(thread: super::winnt::HANDLE, cpusetmasks: Option<&mut [super::winnt::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSetMasks(thread : super::winnt::HANDLE, cpusetmasks : *mut super::winnt::GROUP_AFFINITY, cpusetmaskcount : u16, requiredmaskcount : *mut u16) -> windows_core::BOOL);
    unsafe { GetThreadSelectedCpuSetMasks(thread, core::mem::transmute(cpusetmasks.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredmaskcount as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetThreadSelectedCpuSets(thread: super::winnt::HANDLE, cpusetids: Option<&mut [u32]>, requiredidcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSets(thread : super::winnt::HANDLE, cpusetids : *mut u32, cpusetidcount : u32, requiredidcount : *mut u32) -> windows_core::BOOL);
    unsafe { GetThreadSelectedCpuSets(thread, core::mem::transmute(cpusetids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredidcount as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetThreadTimes(hthread: super::winnt::HANDLE, lpcreationtime: *mut super::minwindef::FILETIME, lpexittime: *mut super::minwindef::FILETIME, lpkerneltime: *mut super::minwindef::FILETIME, lpusertime: *mut super::minwindef::FILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadTimes(hthread : super::winnt::HANDLE, lpcreationtime : *mut super::minwindef::FILETIME, lpexittime : *mut super::minwindef::FILETIME, lpkerneltime : *mut super::minwindef::FILETIME, lpusertime : *mut super::minwindef::FILETIME) -> windows_core::BOOL);
    unsafe { GetThreadTimes(hthread, lpcreationtime as _, lpexittime as _, lpkerneltime as _, lpusertime as _) }
}
#[inline]
pub unsafe fn InitializeProcThreadAttributeList(lpattributelist: Option<*mut _PROC_THREAD_ATTRIBUTE_LIST>, dwattributecount: u32, dwflags: Option<u32>, lpsize: *mut usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeProcThreadAttributeList(lpattributelist : *mut _PROC_THREAD_ATTRIBUTE_LIST, dwattributecount : u32, dwflags : u32, lpsize : *mut usize) -> windows_core::BOOL);
    unsafe { InitializeProcThreadAttributeList(lpattributelist.unwrap_or(core::mem::zeroed()) as _, dwattributecount, dwflags.unwrap_or(core::mem::zeroed()) as _, lpsize as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn IsProcessCritical(hprocess: super::winnt::HANDLE, critical: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsProcessCritical(hprocess : super::winnt::HANDLE, critical : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsProcessCritical(hprocess, critical as _) }
}
#[inline]
pub unsafe fn IsProcessorFeaturePresent(processorfeature: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsProcessorFeaturePresent(processorfeature : u32) -> windows_core::BOOL);
    unsafe { IsProcessorFeaturePresent(processorfeature) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenProcess(dwdesiredaccess: u32, binherithandle: bool, dwprocessid: u32) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwprocessid : u32) -> super::winnt::HANDLE);
    unsafe { OpenProcess(dwdesiredaccess, binherithandle.into(), dwprocessid) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenProcessToken(processhandle: super::winnt::HANDLE, desiredaccess: u32, tokenhandle: *mut super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : super::winnt::HANDLE, desiredaccess : u32, tokenhandle : *mut super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { OpenProcessToken(processhandle, desiredaccess, tokenhandle as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenThread(dwdesiredaccess: u32, binherithandle: bool, dwthreadid: u32) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn OpenThread(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwthreadid : u32) -> super::winnt::HANDLE);
    unsafe { OpenThread(dwdesiredaccess, binherithandle.into(), dwthreadid) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenThreadToken(threadhandle: super::winnt::HANDLE, desiredaccess: u32, openasself: bool, tokenhandle: *mut super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OpenThreadToken(threadhandle : super::winnt::HANDLE, desiredaccess : u32, openasself : windows_core::BOOL, tokenhandle : *mut super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { OpenThreadToken(threadhandle, desiredaccess, openasself.into(), tokenhandle as _) }
}
#[inline]
pub unsafe fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ProcessIdToSessionId(dwprocessid : u32, psessionid : *mut u32) -> windows_core::BOOL);
    unsafe { ProcessIdToSessionId(dwprocessid, psessionid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn QueryProcessAffinityUpdateMode(hprocess: super::winnt::HANDLE, lpdwflags: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryProcessAffinityUpdateMode(hprocess : super::winnt::HANDLE, lpdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { QueryProcessAffinityUpdateMode(hprocess, lpdwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn QueryProtectedPolicy(policyguid: *const windows_core::GUID, policyvalue: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryProtectedPolicy(policyguid : *const windows_core::GUID, policyvalue : *mut u32) -> windows_core::BOOL);
    unsafe { QueryProtectedPolicy(policyguid, policyvalue as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn QueryProtectedPolicy(policyguid: *const windows_core::GUID, policyvalue: *mut u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryProtectedPolicy(policyguid : *const windows_core::GUID, policyvalue : *mut u64) -> windows_core::BOOL);
    unsafe { QueryProtectedPolicy(policyguid, policyvalue as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn QueueUserAPC(pfnapc: super::winnt::PAPCFUNC, hthread: super::winnt::HANDLE, dwdata: usize) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn QueueUserAPC(pfnapc : super::winnt::PAPCFUNC, hthread : super::winnt::HANDLE, dwdata : usize) -> u32);
    unsafe { QueueUserAPC(pfnapc, hthread, dwdata) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn QueueUserAPC2(apcroutine: super::winnt::PAPCFUNC, thread: super::winnt::HANDLE, data: usize, flags: QUEUE_USER_APC_FLAGS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueueUserAPC2(apcroutine : super::winnt::PAPCFUNC, thread : super::winnt::HANDLE, data : usize, flags : QUEUE_USER_APC_FLAGS) -> windows_core::BOOL);
    unsafe { QueueUserAPC2(apcroutine, thread, data, flags) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ResumeThread(hthread: super::winnt::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn ResumeThread(hthread : super::winnt::HANDLE) -> u32);
    unsafe { ResumeThread(hthread) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetPriorityClass(hprocess: super::winnt::HANDLE, dwpriorityclass: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetPriorityClass(hprocess : super::winnt::HANDLE, dwpriorityclass : u32) -> windows_core::BOOL);
    unsafe { SetPriorityClass(hprocess, dwpriorityclass) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetProcessAffinityUpdateMode(hprocess: super::winnt::HANDLE, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessAffinityUpdateMode(hprocess : super::winnt::HANDLE, dwflags : u32) -> windows_core::BOOL);
    unsafe { SetProcessAffinityUpdateMode(hprocess, dwflags) }
}
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SetProcessDefaultCpuSetMasks(process: super::winnt::HANDLE, cpusetmasks: Option<&[super::winnt::GROUP_AFFINITY]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSetMasks(process : super::winnt::HANDLE, cpusetmasks : *const super::winnt::GROUP_AFFINITY, cpusetmaskcount : u16) -> windows_core::BOOL);
    unsafe { SetProcessDefaultCpuSetMasks(process, core::mem::transmute(cpusetmasks.map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetProcessDefaultCpuSets(process: super::winnt::HANDLE, cpusetids: Option<&[u32]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSets(process : super::winnt::HANDLE, cpusetids : *const u32, cpusetidcount : u32) -> windows_core::BOOL);
    unsafe { SetProcessDefaultCpuSets(process, core::mem::transmute(cpusetids.map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetids.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetProcessDynamicEHContinuationTargets(process: super::winnt::HANDLE, targets: &mut [super::winnt::PROCESS_DYNAMIC_EH_CONTINUATION_TARGET]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDynamicEHContinuationTargets(process : super::winnt::HANDLE, numberoftargets : u16, targets : *mut super::winnt::PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> windows_core::BOOL);
    unsafe { SetProcessDynamicEHContinuationTargets(process, targets.len().try_into().unwrap(), core::mem::transmute(targets.as_ptr())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetProcessDynamicEnforcedCetCompatibleRanges(process: super::winnt::HANDLE, ranges: &mut [super::winnt::PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDynamicEnforcedCetCompatibleRanges(process : super::winnt::HANDLE, numberofranges : u16, ranges : *mut super::winnt::PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> windows_core::BOOL);
    unsafe { SetProcessDynamicEnforcedCetCompatibleRanges(process, ranges.len().try_into().unwrap(), core::mem::transmute(ranges.as_ptr())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetProcessInformation(hprocess: super::winnt::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const core::ffi::c_void, processinformationsize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessInformation(hprocess : super::winnt::HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : *const core::ffi::c_void, processinformationsize : u32) -> windows_core::BOOL);
    unsafe { SetProcessInformation(hprocess, processinformationclass, processinformation, processinformationsize) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetProcessMitigationPolicy(mitigationpolicy: super::winnt::PROCESS_MITIGATION_POLICY, lpbuffer: *const core::ffi::c_void, dwlength: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessMitigationPolicy(mitigationpolicy : super::winnt::PROCESS_MITIGATION_POLICY, lpbuffer : *const core::ffi::c_void, dwlength : usize) -> windows_core::BOOL);
    unsafe { SetProcessMitigationPolicy(mitigationpolicy, lpbuffer, dwlength) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetProcessPriorityBoost(hprocess: super::winnt::HANDLE, bdisablepriorityboost: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessPriorityBoost(hprocess : super::winnt::HANDLE, bdisablepriorityboost : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetProcessPriorityBoost(hprocess, bdisablepriorityboost.into()) }
}
#[inline]
pub unsafe fn SetProcessShutdownParameters(dwlevel: u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessShutdownParameters(dwlevel : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { SetProcessShutdownParameters(dwlevel, dwflags) }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn SetProtectedPolicy(policyguid: *const windows_core::GUID, policyvalue: usize, oldpolicyvalue: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProtectedPolicy(policyguid : *const windows_core::GUID, policyvalue : usize, oldpolicyvalue : *mut u32) -> windows_core::BOOL);
    unsafe { SetProtectedPolicy(policyguid, policyvalue, oldpolicyvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn SetProtectedPolicy(policyguid: *const windows_core::GUID, policyvalue: usize, oldpolicyvalue: Option<*mut u64>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProtectedPolicy(policyguid : *const windows_core::GUID, policyvalue : usize, oldpolicyvalue : *mut u64) -> windows_core::BOOL);
    unsafe { SetProtectedPolicy(policyguid, policyvalue, oldpolicyvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadContext(hthread: super::winnt::HANDLE, lpcontext: *const super::winnt::CONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadContext(hthread : super::winnt::HANDLE, lpcontext : *const super::winnt::CONTEXT) -> windows_core::BOOL);
    unsafe { SetThreadContext(hthread, lpcontext) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadDescription<P1>(hthread: super::winnt::HANDLE, lpthreaddescription: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetThreadDescription(hthread : super::winnt::HANDLE, lpthreaddescription : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SetThreadDescription(hthread, lpthreaddescription.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadIdealProcessor(hthread: super::winnt::HANDLE, dwidealprocessor: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SetThreadIdealProcessor(hthread : super::winnt::HANDLE, dwidealprocessor : u32) -> u32);
    unsafe { SetThreadIdealProcessor(hthread, dwidealprocessor) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadIdealProcessorEx(hthread: super::winnt::HANDLE, lpidealprocessor: *const super::winnt::PROCESSOR_NUMBER, lppreviousidealprocessor: Option<*mut super::winnt::PROCESSOR_NUMBER>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadIdealProcessorEx(hthread : super::winnt::HANDLE, lpidealprocessor : *const super::winnt::PROCESSOR_NUMBER, lppreviousidealprocessor : *mut super::winnt::PROCESSOR_NUMBER) -> windows_core::BOOL);
    unsafe { SetThreadIdealProcessorEx(hthread, lpidealprocessor, lppreviousidealprocessor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadInformation(hthread: super::winnt::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const core::ffi::c_void, threadinformationsize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadInformation(hthread : super::winnt::HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : *const core::ffi::c_void, threadinformationsize : u32) -> windows_core::BOOL);
    unsafe { SetThreadInformation(hthread, threadinformationclass, threadinformation, threadinformationsize) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadPriority(hthread: super::winnt::HANDLE, npriority: i32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadPriority(hthread : super::winnt::HANDLE, npriority : i32) -> windows_core::BOOL);
    unsafe { SetThreadPriority(hthread, npriority) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadPriorityBoost(hthread: super::winnt::HANDLE, bdisablepriorityboost: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadPriorityBoost(hthread : super::winnt::HANDLE, bdisablepriorityboost : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetThreadPriorityBoost(hthread, bdisablepriorityboost.into()) }
}
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SetThreadSelectedCpuSetMasks(thread: super::winnt::HANDLE, cpusetmasks: Option<&[super::winnt::GROUP_AFFINITY]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSetMasks(thread : super::winnt::HANDLE, cpusetmasks : *const super::winnt::GROUP_AFFINITY, cpusetmaskcount : u16) -> windows_core::BOOL);
    unsafe { SetThreadSelectedCpuSetMasks(thread, core::mem::transmute(cpusetmasks.map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadSelectedCpuSets(thread: super::winnt::HANDLE, cpusetids: &[u32]) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSets(thread : super::winnt::HANDLE, cpusetids : *const u32, cpusetidcount : u32) -> windows_core::BOOL);
    unsafe { SetThreadSelectedCpuSets(thread, core::mem::transmute(cpusetids.as_ptr()), cpusetids.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SetThreadStackGuarantee(stacksizeinbytes: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadStackGuarantee(stacksizeinbytes : *mut u32) -> windows_core::BOOL);
    unsafe { SetThreadStackGuarantee(stacksizeinbytes as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetThreadToken(thread: Option<*const super::winnt::HANDLE>, token: Option<super::winnt::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetThreadToken(thread : *const super::winnt::HANDLE, token : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { SetThreadToken(thread.unwrap_or(core::mem::zeroed()) as _, token.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SuspendThread(hthread: super::winnt::HANDLE) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn SuspendThread(hthread : super::winnt::HANDLE) -> u32);
    unsafe { SuspendThread(hthread) }
}
#[inline]
pub unsafe fn SwitchToThread() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SwitchToThread() -> windows_core::BOOL);
    unsafe { SwitchToThread() }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn TerminateProcess(hprocess: super::winnt::HANDLE, uexitcode: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TerminateProcess(hprocess : super::winnt::HANDLE, uexitcode : u32) -> windows_core::BOOL);
    unsafe { TerminateProcess(hprocess, uexitcode) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn TerminateThread(hthread: super::winnt::HANDLE, dwexitcode: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TerminateThread(hthread : super::winnt::HANDLE, dwexitcode : u32) -> windows_core::BOOL);
    unsafe { TerminateThread(hthread, dwexitcode) }
}
#[inline]
pub unsafe fn TlsAlloc() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn TlsAlloc() -> u32);
    unsafe { TlsAlloc() }
}
#[inline]
pub unsafe fn TlsFree(dwtlsindex: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TlsFree(dwtlsindex : u32) -> windows_core::BOOL);
    unsafe { TlsFree(dwtlsindex) }
}
#[inline]
pub unsafe fn TlsGetValue(dwtlsindex: u32) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn TlsGetValue(dwtlsindex : u32) -> *mut core::ffi::c_void);
    unsafe { TlsGetValue(dwtlsindex) }
}
#[inline]
pub unsafe fn TlsGetValue2(dwtlsindex: u32) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn TlsGetValue2(dwtlsindex : u32) -> *mut core::ffi::c_void);
    unsafe { TlsGetValue2(dwtlsindex) }
}
#[inline]
pub unsafe fn TlsSetValue(dwtlsindex: u32, lptlsvalue: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TlsSetValue(dwtlsindex : u32, lptlsvalue : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { TlsSetValue(dwtlsindex, lptlsvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn UpdateProcThreadAttribute(lpattributelist: *mut _PROC_THREAD_ATTRIBUTE_LIST, dwflags: u32, attribute: usize, lpvalue: Option<*const core::ffi::c_void>, cbsize: usize, lppreviousvalue: Option<*mut core::ffi::c_void>, lpreturnsize: Option<*const usize>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UpdateProcThreadAttribute(lpattributelist : *mut _PROC_THREAD_ATTRIBUTE_LIST, dwflags : u32, attribute : usize, lpvalue : *const core::ffi::c_void, cbsize : usize, lppreviousvalue : *mut core::ffi::c_void, lpreturnsize : *const usize) -> windows_core::BOOL);
    unsafe { UpdateProcThreadAttribute(lpattributelist as _, dwflags, attribute, lpvalue.unwrap_or(core::mem::zeroed()) as _, cbsize, lppreviousvalue.unwrap_or(core::mem::zeroed()) as _, lpreturnsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APC_CALLBACK_DATA {
    pub Parameter: usize,
    pub ContextRecord: super::winnt::PCONTEXT,
    pub Reserved0: usize,
    pub Reserved1: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
pub const KernelEnabled: MACHINE_ATTRIBUTES = 2;
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPROCESS_INFORMATION(pub *mut PROCESS_INFORMATION);
#[cfg(feature = "Win32_winnt")]
impl LPPROCESS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for LPPROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPROC_THREAD_ATTRIBUTE_LIST(pub *mut _PROC_THREAD_ATTRIBUTE_LIST);
impl LPPROC_THREAD_ATTRIBUTE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPSTARTUPINFO(pub LPSTARTUPINFOA);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSTARTUPINFOA(pub *mut STARTUPINFOA);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl LPSTARTUPINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for LPSTARTUPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSTARTUPINFOW(pub *mut STARTUPINFOW);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl LPSTARTUPINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for LPSTARTUPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MACHINE_ATTRIBUTES = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OVERRIDE_PREFETCH_PARAMETER(pub u32);
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAPC_CALLBACK_DATA(pub *mut APC_CALLBACK_DATA);
#[cfg(feature = "Win32_winnt")]
impl PAPC_CALLBACK_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PAPC_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAPP_MEMORY_INFORMATION(pub *mut APP_MEMORY_INFORMATION);
impl PAPP_MEMORY_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAPP_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEMORY_PRIORITY_INFORMATION(pub *mut MEMORY_PRIORITY_INFORMATION);
impl PMEMORY_PRIORITY_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEMORY_PRIORITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = 0;
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = 1;
pub const PME_CURRENT_VERSION: u32 = 1;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1;
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_INFORMATION(pub *mut PROCESS_INFORMATION);
#[cfg(feature = "Win32_winnt")]
impl PPROCESS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PPROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_LEAP_SECOND_INFO(pub *mut PROCESS_LEAP_SECOND_INFO);
impl PPROCESS_LEAP_SECOND_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_LEAP_SECOND_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MEMORY_EXHAUSTION_INFO(pub *mut PROCESS_MEMORY_EXHAUSTION_INFO);
impl PPROCESS_MEMORY_EXHAUSTION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MEMORY_EXHAUSTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MEMORY_EXHAUSTION_TYPE(pub *mut PROCESS_MEMORY_EXHAUSTION_TYPE);
impl PPROCESS_MEMORY_EXHAUSTION_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MEMORY_EXHAUSTION_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_POWER_THROTTLING_STATE(pub *mut PROCESS_POWER_THROTTLING_STATE);
impl PPROCESS_POWER_THROTTLING_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROC_THREAD_ATTRIBUTE_LIST(pub *mut _PROC_THREAD_ATTRIBUTE_LIST);
impl PPROC_THREAD_ATTRIBUTE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROC_THREAD_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::winnt::HANDLE,
    pub hThread: super::winnt::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
pub type PROCESS_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: u16,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
pub const PROCESS_POWER_THROTTLING_VALID_FLAGS: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type STARTUPINFO = STARTUPINFOA;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: windows_core::PSTR,
    pub lpDesktop: windows_core::PSTR,
    pub lpTitle: windows_core::PSTR,
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
    pub lpReserved2: super::minwindef::LPBYTE,
    pub hStdInput: super::winnt::HANDLE,
    pub hStdOutput: super::winnt::HANDLE,
    pub hStdError: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: windows_core::PWSTR,
    pub lpDesktop: windows_core::PWSTR,
    pub lpTitle: windows_core::PWSTR,
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
    pub lpReserved2: super::minwindef::LPBYTE,
    pub hStdInput: super::winnt::HANDLE,
    pub hStdOutput: super::winnt::HANDLE,
    pub hStdError: super::winnt::HANDLE,
}
pub type THREAD_INFORMATION_CLASS = i32;
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _PROC_THREAD_ATTRIBUTE_LIST(pub u8);
