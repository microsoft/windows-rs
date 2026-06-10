#[inline]
pub unsafe fn NtCancelTimer(timerhandle: super::super::super::Win32::Foundation::HANDLE, currentstate: Option<*mut bool>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCancelTimer(timerhandle : super::super::super::Win32::Foundation::HANDLE, currentstate : *mut bool) -> windows_core::NTSTATUS);
    unsafe { NtCancelTimer(timerhandle, currentstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NtCreateTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, timertype: super::super::super::Win32::System::Kernel::TIMER_TYPE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateTimer(timerhandle : *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation::OBJECT_ATTRIBUTES, timertype : super::super::super::Win32::System::Kernel::TIMER_TYPE) -> windows_core::NTSTATUS);
    unsafe { NtCreateTimer(timerhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, timertype) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenEvent(eventhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenEvent(eventhandle : *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation::OBJECT_ATTRIBUTES) -> windows_core::NTSTATUS);
    unsafe { NtOpenEvent(eventhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn NtOpenProcess(processhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, clientid: Option<*const super::super::super::Win32::System::WindowsProgramming::CLIENT_ID>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenProcess(processhandle : *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation::OBJECT_ATTRIBUTES, clientid : *const super::super::super::Win32::System::WindowsProgramming::CLIENT_ID) -> windows_core::NTSTATUS);
    unsafe { NtOpenProcess(processhandle as _, desiredaccess, objectattributes, clientid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NtOpenTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenTimer(timerhandle : *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation::OBJECT_ATTRIBUTES) -> windows_core::NTSTATUS);
    unsafe { NtOpenTimer(timerhandle as _, desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn NtQueryInformationProcess(processhandle: super::super::super::Win32::Foundation::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationProcess(processhandle : super::super::super::Win32::Foundation::HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> windows_core::NTSTATUS);
    unsafe { NtQueryInformationProcess(processhandle, processinformationclass, processinformation as _, processinformationlength, returnlength as _) }
}
#[inline]
pub unsafe fn NtQueryInformationThread(threadhandle: super::super::super::Win32::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationThread(threadhandle : super::super::super::Win32::Foundation::HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> windows_core::NTSTATUS);
    unsafe { NtQueryInformationThread(threadhandle, threadinformationclass, threadinformation as _, threadinformationlength, returnlength as _) }
}
#[inline]
pub unsafe fn NtSetInformationThread(threadhandle: super::super::super::Win32::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const core::ffi::c_void, threadinformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationThread(threadhandle : super::super::super::Win32::Foundation::HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const core::ffi::c_void, threadinformationlength : u32) -> windows_core::NTSTATUS);
    unsafe { NtSetInformationThread(threadhandle, threadinformationclass, threadinformation, threadinformationlength) }
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn NtSetTimer(timerhandle: super::super::super::Win32::Foundation::HANDLE, duetime: *const i64, timerapcroutine: super::SystemServices::PTIMER_APC_ROUTINE, timercontext: Option<*const core::ffi::c_void>, resumetimer: bool, period: Option<i32>, previousstate: Option<*mut bool>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetTimer(timerhandle : super::super::super::Win32::Foundation::HANDLE, duetime : *const i64, timerapcroutine : super::SystemServices::PTIMER_APC_ROUTINE, timercontext : *const core::ffi::c_void, resumetimer : bool, period : i32, previousstate : *mut bool) -> windows_core::NTSTATUS);
    unsafe { NtSetTimer(timerhandle, duetime, timerapcroutine, timercontext.unwrap_or(core::mem::zeroed()) as _, resumetimer, period.unwrap_or(core::mem::zeroed()) as _, previousstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NtSetTimerEx(timerhandle: super::super::super::Win32::Foundation::HANDLE, timersetinformationclass: TIMER_SET_INFORMATION_CLASS, timersetinformation: Option<*mut core::ffi::c_void>, timersetinformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetTimerEx(timerhandle : super::super::super::Win32::Foundation::HANDLE, timersetinformationclass : TIMER_SET_INFORMATION_CLASS, timersetinformation : *mut core::ffi::c_void, timersetinformationlength : u32) -> windows_core::NTSTATUS);
    unsafe { NtSetTimerEx(timerhandle, timersetinformationclass, timersetinformation.unwrap_or(core::mem::zeroed()) as _, timersetinformationlength) }
}
#[inline]
pub unsafe fn NtTerminateProcess(processhandle: Option<super::super::super::Win32::Foundation::HANDLE>, exitstatus: windows_core::NTSTATUS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtTerminateProcess(processhandle : super::super::super::Win32::Foundation::HANDLE, exitstatus : windows_core::NTSTATUS) -> windows_core::NTSTATUS);
    unsafe { NtTerminateProcess(processhandle.unwrap_or(core::mem::zeroed()) as _, exitstatus) }
}
#[inline]
pub unsafe fn NtWaitForSingleObject(handle: super::super::super::Win32::Foundation::HANDLE, alertable: bool, timeout: *mut i64) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : super::super::super::Win32::Foundation::HANDLE, alertable : bool, timeout : *mut i64) -> windows_core::NTSTATUS);
    unsafe { NtWaitForSingleObject(handle, alertable, timeout as _) }
}
#[inline]
pub unsafe fn ZwCancelTimer(timerhandle: super::super::super::Win32::Foundation::HANDLE, currentstate: Option<*mut bool>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCancelTimer(timerhandle : super::super::super::Win32::Foundation::HANDLE, currentstate : *mut bool) -> windows_core::NTSTATUS);
    unsafe { ZwCancelTimer(timerhandle, currentstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn ZwCreateTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: Option<*const super::super::Foundation::OBJECT_ATTRIBUTES>, timertype: super::super::super::Win32::System::Kernel::TIMER_TYPE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateTimer(timerhandle : *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation::OBJECT_ATTRIBUTES, timertype : super::super::super::Win32::System::Kernel::TIMER_TYPE) -> windows_core::NTSTATUS);
    unsafe { ZwCreateTimer(timerhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, timertype) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenEvent(eventhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenEvent(eventhandle : *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation::OBJECT_ATTRIBUTES) -> windows_core::NTSTATUS);
    unsafe { ZwOpenEvent(eventhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn ZwOpenProcess(processhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, clientid: Option<*const super::super::super::Win32::System::WindowsProgramming::CLIENT_ID>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenProcess(processhandle : *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation::OBJECT_ATTRIBUTES, clientid : *const super::super::super::Win32::System::WindowsProgramming::CLIENT_ID) -> windows_core::NTSTATUS);
    unsafe { ZwOpenProcess(processhandle as _, desiredaccess, objectattributes, clientid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ZwOpenTimer(timerhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenTimer(timerhandle : *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation::OBJECT_ATTRIBUTES) -> windows_core::NTSTATUS);
    unsafe { ZwOpenTimer(timerhandle as _, desiredaccess, objectattributes) }
}
#[inline]
pub unsafe fn ZwQueryInformationProcess(processhandle: super::super::super::Win32::Foundation::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationProcess(processhandle : super::super::super::Win32::Foundation::HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> windows_core::NTSTATUS);
    unsafe { ZwQueryInformationProcess(processhandle, processinformationclass, processinformation as _, processinformationlength, returnlength as _) }
}
#[inline]
pub unsafe fn ZwQueryInformationThread(threadhandle: super::super::super::Win32::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationThread(threadhandle : super::super::super::Win32::Foundation::HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> windows_core::NTSTATUS);
    unsafe { ZwQueryInformationThread(threadhandle, threadinformationclass, threadinformation as _, threadinformationlength, returnlength as _) }
}
#[inline]
pub unsafe fn ZwSetInformationThread(threadhandle: super::super::super::Win32::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const core::ffi::c_void, threadinformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationThread(threadhandle : super::super::super::Win32::Foundation::HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const core::ffi::c_void, threadinformationlength : u32) -> windows_core::NTSTATUS);
    unsafe { ZwSetInformationThread(threadhandle, threadinformationclass, threadinformation, threadinformationlength) }
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn ZwSetTimer(timerhandle: super::super::super::Win32::Foundation::HANDLE, duetime: *const i64, timerapcroutine: super::SystemServices::PTIMER_APC_ROUTINE, timercontext: Option<*const core::ffi::c_void>, resumetimer: bool, period: Option<i32>, previousstate: Option<*mut bool>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetTimer(timerhandle : super::super::super::Win32::Foundation::HANDLE, duetime : *const i64, timerapcroutine : super::SystemServices::PTIMER_APC_ROUTINE, timercontext : *const core::ffi::c_void, resumetimer : bool, period : i32, previousstate : *mut bool) -> windows_core::NTSTATUS);
    unsafe { ZwSetTimer(timerhandle, duetime, timerapcroutine, timercontext.unwrap_or(core::mem::zeroed()) as _, resumetimer, period.unwrap_or(core::mem::zeroed()) as _, previousstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ZwSetTimerEx(timerhandle: super::super::super::Win32::Foundation::HANDLE, timersetinformationclass: TIMER_SET_INFORMATION_CLASS, timersetinformation: Option<*mut core::ffi::c_void>, timersetinformationlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetTimerEx(timerhandle : super::super::super::Win32::Foundation::HANDLE, timersetinformationclass : TIMER_SET_INFORMATION_CLASS, timersetinformation : *mut core::ffi::c_void, timersetinformationlength : u32) -> windows_core::NTSTATUS);
    unsafe { ZwSetTimerEx(timerhandle, timersetinformationclass, timersetinformation.unwrap_or(core::mem::zeroed()) as _, timersetinformationlength) }
}
#[inline]
pub unsafe fn ZwTerminateProcess(processhandle: Option<super::super::super::Win32::Foundation::HANDLE>, exitstatus: windows_core::NTSTATUS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwTerminateProcess(processhandle : super::super::super::Win32::Foundation::HANDLE, exitstatus : windows_core::NTSTATUS) -> windows_core::NTSTATUS);
    unsafe { ZwTerminateProcess(processhandle.unwrap_or(core::mem::zeroed()) as _, exitstatus) }
}
#[inline]
pub unsafe fn ZwWaitForSingleObject(handle: super::super::super::Win32::Foundation::HANDLE, alertable: bool, timeout: Option<*const i64>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwWaitForSingleObject(handle : super::super::super::Win32::Foundation::HANDLE, alertable : bool, timeout : *const i64) -> windows_core::NTSTATUS);
    unsafe { ZwWaitForSingleObject(handle, alertable, timeout.unwrap_or(core::mem::zeroed()) as _) }
}
pub const MaxProcessInfoClass: PROCESSINFOCLASS = PROCESSINFOCLASS(83);
pub const MaxThreadInfoClass: THREADINFOCLASS = THREADINFOCLASS(56);
pub const MaxTimerInfoClass: TIMER_SET_INFORMATION_CLASS = TIMER_SET_INFORMATION_CLASS(1);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESSINFOCLASS(pub i32);
pub const ProcessAccessToken: PROCESSINFOCLASS = PROCESSINFOCLASS(9);
pub const ProcessAffinityMask: PROCESSINFOCLASS = PROCESSINFOCLASS(21);
pub const ProcessAffinityUpdateMode: PROCESSINFOCLASS = PROCESSINFOCLASS(45);
pub const ProcessBasePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(5);
pub const ProcessBasicInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(0);
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = PROCESSINFOCLASS(29);
pub const ProcessCheckStackExtentsMode: PROCESSINFOCLASS = PROCESSINFOCLASS(59);
pub const ProcessCommandLineInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(60);
pub const ProcessCommitReleaseInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(65);
pub const ProcessCookie: PROCESSINFOCLASS = PROCESSINFOCLASS(36);
pub const ProcessCycleTime: PROCESSINFOCLASS = PROCESSINFOCLASS(38);
pub const ProcessDebugFlags: PROCESSINFOCLASS = PROCESSINFOCLASS(31);
pub const ProcessDebugObjectHandle: PROCESSINFOCLASS = PROCESSINFOCLASS(30);
pub const ProcessDebugPort: PROCESSINFOCLASS = PROCESSINFOCLASS(7);
pub const ProcessDefaultHardErrorMode: PROCESSINFOCLASS = PROCESSINFOCLASS(12);
pub const ProcessDeviceMap: PROCESSINFOCLASS = PROCESSINFOCLASS(23);
pub const ProcessDynamicFunctionTableInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(53);
pub const ProcessEnableAlignmentFaultFixup: PROCESSINFOCLASS = PROCESSINFOCLASS(17);
pub const ProcessEnergyTrackingState: PROCESSINFOCLASS = PROCESSINFOCLASS(82);
pub const ProcessExceptionPort: PROCESSINFOCLASS = PROCESSINFOCLASS(8);
pub const ProcessExecuteFlags: PROCESSINFOCLASS = PROCESSINFOCLASS(34);
pub const ProcessFaultInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(63);
pub const ProcessForegroundInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(25);
pub const ProcessGroupInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(47);
pub const ProcessHandleCheckingMode: PROCESSINFOCLASS = PROCESSINFOCLASS(54);
pub const ProcessHandleCount: PROCESSINFOCLASS = PROCESSINFOCLASS(20);
pub const ProcessHandleInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(51);
pub const ProcessHandleTable: PROCESSINFOCLASS = PROCESSINFOCLASS(58);
pub const ProcessHandleTracing: PROCESSINFOCLASS = PROCESSINFOCLASS(32);
pub const ProcessImageFileMapping: PROCESSINFOCLASS = PROCESSINFOCLASS(44);
pub const ProcessImageFileName: PROCESSINFOCLASS = PROCESSINFOCLASS(27);
pub const ProcessImageFileNameWin32: PROCESSINFOCLASS = PROCESSINFOCLASS(43);
pub const ProcessImageInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(37);
pub const ProcessInPrivate: PROCESSINFOCLASS = PROCESSINFOCLASS(70);
pub const ProcessInstrumentationCallback: PROCESSINFOCLASS = PROCESSINFOCLASS(40);
pub const ProcessIoCounters: PROCESSINFOCLASS = PROCESSINFOCLASS(2);
pub const ProcessIoPortHandlers: PROCESSINFOCLASS = PROCESSINFOCLASS(13);
pub const ProcessIoPriority: PROCESSINFOCLASS = PROCESSINFOCLASS(33);
pub const ProcessKeepAliveCount: PROCESSINFOCLASS = PROCESSINFOCLASS(55);
pub const ProcessLUIDDeviceMapsEnabled: PROCESSINFOCLASS = PROCESSINFOCLASS(28);
pub const ProcessLdtInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(10);
pub const ProcessLdtSize: PROCESSINFOCLASS = PROCESSINFOCLASS(11);
pub const ProcessMemoryAllocationMode: PROCESSINFOCLASS = PROCESSINFOCLASS(46);
pub const ProcessMemoryExhaustion: PROCESSINFOCLASS = PROCESSINFOCLASS(62);
pub const ProcessMitigationPolicy: PROCESSINFOCLASS = PROCESSINFOCLASS(52);
pub const ProcessOwnerInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(49);
pub const ProcessPagePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(39);
pub const ProcessPooledUsageAndLimits: PROCESSINFOCLASS = PROCESSINFOCLASS(14);
pub const ProcessPriorityBoost: PROCESSINFOCLASS = PROCESSINFOCLASS(22);
pub const ProcessPriorityClass: PROCESSINFOCLASS = PROCESSINFOCLASS(18);
pub const ProcessProtectionInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(61);
pub const ProcessQuotaLimits: PROCESSINFOCLASS = PROCESSINFOCLASS(1);
pub const ProcessRaisePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(6);
pub const ProcessRaiseUMExceptionOnInvalidHandleClose: PROCESSINFOCLASS = PROCESSINFOCLASS(71);
pub const ProcessReserved1Information: PROCESSINFOCLASS = PROCESSINFOCLASS(66);
pub const ProcessReserved2Information: PROCESSINFOCLASS = PROCESSINFOCLASS(67);
pub const ProcessRevokeFileHandles: PROCESSINFOCLASS = PROCESSINFOCLASS(56);
pub const ProcessSessionInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(24);
pub const ProcessSubsystemInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(75);
pub const ProcessSubsystemProcess: PROCESSINFOCLASS = PROCESSINFOCLASS(68);
pub const ProcessTelemetryIdInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(64);
pub const ProcessThreadStackAllocation: PROCESSINFOCLASS = PROCESSINFOCLASS(41);
pub const ProcessTimes: PROCESSINFOCLASS = PROCESSINFOCLASS(4);
pub const ProcessTlsInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(35);
pub const ProcessTokenVirtualizationEnabled: PROCESSINFOCLASS = PROCESSINFOCLASS(48);
pub const ProcessUserModeIOPL: PROCESSINFOCLASS = PROCESSINFOCLASS(16);
pub const ProcessVmCounters: PROCESSINFOCLASS = PROCESSINFOCLASS(3);
pub const ProcessWin32kSyscallFilterInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(79);
pub const ProcessWindowInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(50);
pub const ProcessWorkingSetControl: PROCESSINFOCLASS = PROCESSINFOCLASS(57);
pub const ProcessWorkingSetWatch: PROCESSINFOCLASS = PROCESSINFOCLASS(15);
pub const ProcessWorkingSetWatchEx: PROCESSINFOCLASS = PROCESSINFOCLASS(42);
pub const ProcessWow64Information: PROCESSINFOCLASS = PROCESSINFOCLASS(26);
pub const ProcessWx86Information: PROCESSINFOCLASS = PROCESSINFOCLASS(19);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREADINFOCLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TIMER_SET_INFORMATION_CLASS(pub i32);
pub const ThreadActualBasePriority: THREADINFOCLASS = THREADINFOCLASS(25);
pub const ThreadActualGroupAffinity: THREADINFOCLASS = THREADINFOCLASS(41);
pub const ThreadAffinityMask: THREADINFOCLASS = THREADINFOCLASS(4);
pub const ThreadAmILastThread: THREADINFOCLASS = THREADINFOCLASS(12);
pub const ThreadBasePriority: THREADINFOCLASS = THREADINFOCLASS(3);
pub const ThreadBasicInformation: THREADINFOCLASS = THREADINFOCLASS(0);
pub const ThreadBreakOnTermination: THREADINFOCLASS = THREADINFOCLASS(18);
pub const ThreadCSwitchMon: THREADINFOCLASS = THREADINFOCLASS(27);
pub const ThreadCSwitchPmu: THREADINFOCLASS = THREADINFOCLASS(28);
pub const ThreadCounterProfiling: THREADINFOCLASS = THREADINFOCLASS(32);
pub const ThreadCpuAccountingInformation: THREADINFOCLASS = THREADINFOCLASS(34);
pub const ThreadCycleTime: THREADINFOCLASS = THREADINFOCLASS(23);
pub const ThreadDescriptorTableEntry: THREADINFOCLASS = THREADINFOCLASS(6);
pub const ThreadDynamicCodePolicyInfo: THREADINFOCLASS = THREADINFOCLASS(42);
pub const ThreadEnableAlignmentFaultFixup: THREADINFOCLASS = THREADINFOCLASS(7);
pub const ThreadEventPair_Reusable: THREADINFOCLASS = THREADINFOCLASS(8);
pub const ThreadGroupInformation: THREADINFOCLASS = THREADINFOCLASS(30);
pub const ThreadHideFromDebugger: THREADINFOCLASS = THREADINFOCLASS(17);
pub const ThreadIdealProcessor: THREADINFOCLASS = THREADINFOCLASS(13);
pub const ThreadIdealProcessorEx: THREADINFOCLASS = THREADINFOCLASS(33);
pub const ThreadImpersonationToken: THREADINFOCLASS = THREADINFOCLASS(5);
pub const ThreadIoPriority: THREADINFOCLASS = THREADINFOCLASS(22);
pub const ThreadIsIoPending: THREADINFOCLASS = THREADINFOCLASS(16);
pub const ThreadIsTerminated: THREADINFOCLASS = THREADINFOCLASS(20);
pub const ThreadLastSystemCall: THREADINFOCLASS = THREADINFOCLASS(21);
pub const ThreadNameInformation: THREADINFOCLASS = THREADINFOCLASS(38);
pub const ThreadPagePriority: THREADINFOCLASS = THREADINFOCLASS(24);
pub const ThreadPerformanceCount: THREADINFOCLASS = THREADINFOCLASS(11);
pub const ThreadPriority: THREADINFOCLASS = THREADINFOCLASS(2);
pub const ThreadPriorityBoost: THREADINFOCLASS = THREADINFOCLASS(14);
pub const ThreadQuerySetWin32StartAddress: THREADINFOCLASS = THREADINFOCLASS(9);
pub const ThreadSetTlsArrayAddress: THREADINFOCLASS = THREADINFOCLASS(15);
pub const ThreadSubsystemInformation: THREADINFOCLASS = THREADINFOCLASS(45);
pub const ThreadSuspendCount: THREADINFOCLASS = THREADINFOCLASS(35);
pub const ThreadSwitchLegacyState: THREADINFOCLASS = THREADINFOCLASS(19);
pub const ThreadTebInformation: THREADINFOCLASS = THREADINFOCLASS(26);
pub const ThreadTimes: THREADINFOCLASS = THREADINFOCLASS(1);
pub const ThreadUmsInformation: THREADINFOCLASS = THREADINFOCLASS(31);
pub const ThreadWow64Context: THREADINFOCLASS = THREADINFOCLASS(29);
pub const ThreadZeroTlsCell: THREADINFOCLASS = THREADINFOCLASS(10);
pub const TimerSetCoalescableTimer: TIMER_SET_INFORMATION_CLASS = TIMER_SET_INFORMATION_CLASS(0);
