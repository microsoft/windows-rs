#[doc = "*Required features: `\"Wdk_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryInformationProcess<P0>(processhandle: P0, processinformationclass: PROCESSINFOCLASS, processinformation: *mut ::core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationProcess(processhandle : super::super::super::Win32::Foundation:: HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut ::core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryInformationProcess(processhandle.into_param().abi(), processinformationclass, processinformation, processinformationlength, returnlength).ok()
}
#[doc = "*Required features: `\"Wdk_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut ::core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryInformationThread(threadhandle.into_param().abi(), threadinformationclass, threadinformation, threadinformationlength, returnlength).ok()
}
#[doc = "*Required features: `\"Wdk_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtSetInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const ::core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetInformationThread(threadhandle.into_param().abi(), threadinformationclass, threadinformation, threadinformationlength).ok()
}
#[doc = "*Required features: `\"Wdk_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtWaitForSingleObject<P0, P1>(handle: P0, alertable: P1, timeout: *mut i64) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : super::super::super::Win32::Foundation:: HANDLE, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtWaitForSingleObject(handle.into_param().abi(), alertable.into_param().abi(), timeout).ok()
}
#[doc = "*Required features: `\"Wdk_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ZwSetInformationThread<P0>(threadhandle: P0, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationThread(threadhandle : super::super::super::Win32::Foundation:: HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const ::core::ffi::c_void, threadinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetInformationThread(threadhandle.into_param().abi(), threadinformationclass, threadinformation, threadinformationlength).ok()
}
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const MaxProcessInfoClass: PROCESSINFOCLASS = PROCESSINFOCLASS(83i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const MaxThreadInfoClass: THREADINFOCLASS = THREADINFOCLASS(56i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessAccessToken: PROCESSINFOCLASS = PROCESSINFOCLASS(9i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessAffinityMask: PROCESSINFOCLASS = PROCESSINFOCLASS(21i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessAffinityUpdateMode: PROCESSINFOCLASS = PROCESSINFOCLASS(45i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessBasePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(5i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessBasicInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(0i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = PROCESSINFOCLASS(29i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessCheckStackExtentsMode: PROCESSINFOCLASS = PROCESSINFOCLASS(59i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessCommandLineInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(60i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessCommitReleaseInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(65i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessCookie: PROCESSINFOCLASS = PROCESSINFOCLASS(36i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessCycleTime: PROCESSINFOCLASS = PROCESSINFOCLASS(38i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessDebugFlags: PROCESSINFOCLASS = PROCESSINFOCLASS(31i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessDebugObjectHandle: PROCESSINFOCLASS = PROCESSINFOCLASS(30i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessDebugPort: PROCESSINFOCLASS = PROCESSINFOCLASS(7i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessDefaultHardErrorMode: PROCESSINFOCLASS = PROCESSINFOCLASS(12i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessDeviceMap: PROCESSINFOCLASS = PROCESSINFOCLASS(23i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessDynamicFunctionTableInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(53i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessEnableAlignmentFaultFixup: PROCESSINFOCLASS = PROCESSINFOCLASS(17i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessEnergyTrackingState: PROCESSINFOCLASS = PROCESSINFOCLASS(82i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessExceptionPort: PROCESSINFOCLASS = PROCESSINFOCLASS(8i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessExecuteFlags: PROCESSINFOCLASS = PROCESSINFOCLASS(34i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessFaultInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(63i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessForegroundInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(25i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessGroupInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(47i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessHandleCheckingMode: PROCESSINFOCLASS = PROCESSINFOCLASS(54i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessHandleCount: PROCESSINFOCLASS = PROCESSINFOCLASS(20i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessHandleInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(51i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessHandleTable: PROCESSINFOCLASS = PROCESSINFOCLASS(58i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessHandleTracing: PROCESSINFOCLASS = PROCESSINFOCLASS(32i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessImageFileMapping: PROCESSINFOCLASS = PROCESSINFOCLASS(44i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessImageFileName: PROCESSINFOCLASS = PROCESSINFOCLASS(27i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessImageFileNameWin32: PROCESSINFOCLASS = PROCESSINFOCLASS(43i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessImageInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(37i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessInPrivate: PROCESSINFOCLASS = PROCESSINFOCLASS(70i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessInstrumentationCallback: PROCESSINFOCLASS = PROCESSINFOCLASS(40i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessIoCounters: PROCESSINFOCLASS = PROCESSINFOCLASS(2i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessIoPortHandlers: PROCESSINFOCLASS = PROCESSINFOCLASS(13i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessIoPriority: PROCESSINFOCLASS = PROCESSINFOCLASS(33i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessKeepAliveCount: PROCESSINFOCLASS = PROCESSINFOCLASS(55i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessLUIDDeviceMapsEnabled: PROCESSINFOCLASS = PROCESSINFOCLASS(28i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessLdtInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(10i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessLdtSize: PROCESSINFOCLASS = PROCESSINFOCLASS(11i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessMemoryAllocationMode: PROCESSINFOCLASS = PROCESSINFOCLASS(46i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessMemoryExhaustion: PROCESSINFOCLASS = PROCESSINFOCLASS(62i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessMitigationPolicy: PROCESSINFOCLASS = PROCESSINFOCLASS(52i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessOwnerInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(49i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessPagePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(39i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessPooledUsageAndLimits: PROCESSINFOCLASS = PROCESSINFOCLASS(14i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessPriorityBoost: PROCESSINFOCLASS = PROCESSINFOCLASS(22i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessPriorityClass: PROCESSINFOCLASS = PROCESSINFOCLASS(18i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessProtectionInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(61i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessQuotaLimits: PROCESSINFOCLASS = PROCESSINFOCLASS(1i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessRaisePriority: PROCESSINFOCLASS = PROCESSINFOCLASS(6i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessRaiseUMExceptionOnInvalidHandleClose: PROCESSINFOCLASS = PROCESSINFOCLASS(71i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessReserved1Information: PROCESSINFOCLASS = PROCESSINFOCLASS(66i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessReserved2Information: PROCESSINFOCLASS = PROCESSINFOCLASS(67i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessRevokeFileHandles: PROCESSINFOCLASS = PROCESSINFOCLASS(56i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessSessionInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(24i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessSubsystemInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(75i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessSubsystemProcess: PROCESSINFOCLASS = PROCESSINFOCLASS(68i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessTelemetryIdInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(64i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessThreadStackAllocation: PROCESSINFOCLASS = PROCESSINFOCLASS(41i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessTimes: PROCESSINFOCLASS = PROCESSINFOCLASS(4i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessTlsInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(35i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessTokenVirtualizationEnabled: PROCESSINFOCLASS = PROCESSINFOCLASS(48i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessUserModeIOPL: PROCESSINFOCLASS = PROCESSINFOCLASS(16i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessVmCounters: PROCESSINFOCLASS = PROCESSINFOCLASS(3i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessWin32kSyscallFilterInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(79i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessWindowInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(50i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessWorkingSetControl: PROCESSINFOCLASS = PROCESSINFOCLASS(57i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessWorkingSetWatch: PROCESSINFOCLASS = PROCESSINFOCLASS(15i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessWorkingSetWatchEx: PROCESSINFOCLASS = PROCESSINFOCLASS(42i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessWow64Information: PROCESSINFOCLASS = PROCESSINFOCLASS(26i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ProcessWx86Information: PROCESSINFOCLASS = PROCESSINFOCLASS(19i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadActualBasePriority: THREADINFOCLASS = THREADINFOCLASS(25i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadActualGroupAffinity: THREADINFOCLASS = THREADINFOCLASS(41i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadAffinityMask: THREADINFOCLASS = THREADINFOCLASS(4i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadAmILastThread: THREADINFOCLASS = THREADINFOCLASS(12i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadBasePriority: THREADINFOCLASS = THREADINFOCLASS(3i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadBasicInformation: THREADINFOCLASS = THREADINFOCLASS(0i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadBreakOnTermination: THREADINFOCLASS = THREADINFOCLASS(18i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadCSwitchMon: THREADINFOCLASS = THREADINFOCLASS(27i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadCSwitchPmu: THREADINFOCLASS = THREADINFOCLASS(28i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadCounterProfiling: THREADINFOCLASS = THREADINFOCLASS(32i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadCpuAccountingInformation: THREADINFOCLASS = THREADINFOCLASS(34i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadCycleTime: THREADINFOCLASS = THREADINFOCLASS(23i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadDescriptorTableEntry: THREADINFOCLASS = THREADINFOCLASS(6i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadDynamicCodePolicyInfo: THREADINFOCLASS = THREADINFOCLASS(42i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadEnableAlignmentFaultFixup: THREADINFOCLASS = THREADINFOCLASS(7i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadEventPair_Reusable: THREADINFOCLASS = THREADINFOCLASS(8i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadGroupInformation: THREADINFOCLASS = THREADINFOCLASS(30i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadHideFromDebugger: THREADINFOCLASS = THREADINFOCLASS(17i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadIdealProcessor: THREADINFOCLASS = THREADINFOCLASS(13i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadIdealProcessorEx: THREADINFOCLASS = THREADINFOCLASS(33i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadImpersonationToken: THREADINFOCLASS = THREADINFOCLASS(5i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadIoPriority: THREADINFOCLASS = THREADINFOCLASS(22i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadIsIoPending: THREADINFOCLASS = THREADINFOCLASS(16i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadIsTerminated: THREADINFOCLASS = THREADINFOCLASS(20i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadLastSystemCall: THREADINFOCLASS = THREADINFOCLASS(21i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadPagePriority: THREADINFOCLASS = THREADINFOCLASS(24i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadPerformanceCount: THREADINFOCLASS = THREADINFOCLASS(11i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadPriority: THREADINFOCLASS = THREADINFOCLASS(2i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadPriorityBoost: THREADINFOCLASS = THREADINFOCLASS(14i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadQuerySetWin32StartAddress: THREADINFOCLASS = THREADINFOCLASS(9i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadSetTlsArrayAddress: THREADINFOCLASS = THREADINFOCLASS(15i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadSubsystemInformation: THREADINFOCLASS = THREADINFOCLASS(45i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadSuspendCount: THREADINFOCLASS = THREADINFOCLASS(35i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadSwitchLegacyState: THREADINFOCLASS = THREADINFOCLASS(19i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadTebInformation: THREADINFOCLASS = THREADINFOCLASS(26i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadTimes: THREADINFOCLASS = THREADINFOCLASS(1i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadUmsInformation: THREADINFOCLASS = THREADINFOCLASS(31i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadWow64Context: THREADINFOCLASS = THREADINFOCLASS(29i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
pub const ThreadZeroTlsCell: THREADINFOCLASS = THREADINFOCLASS(10i32);
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESSINFOCLASS(pub i32);
impl ::core::marker::Copy for PROCESSINFOCLASS {}
impl ::core::clone::Clone for PROCESSINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PROCESSINFOCLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PROCESSINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSINFOCLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Wdk_System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct THREADINFOCLASS(pub i32);
impl ::core::marker::Copy for THREADINFOCLASS {}
impl ::core::clone::Clone for THREADINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREADINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for THREADINFOCLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for THREADINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREADINFOCLASS").field(&self.0).finish()
    }
}
