#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn AvQuerySystemResponsiveness(avrthandle: super::HANDLE, systemresponsivenessvalue: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("avrt.dll" "system" fn AvQuerySystemResponsiveness(avrthandle : super::HANDLE, systemresponsivenessvalue : super::PULONG) -> windows_core::BOOL);
    unsafe { AvQuerySystemResponsiveness(avrthandle, systemresponsivenessvalue as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvRevertMmThreadCharacteristics(avrthandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("avrt.dll" "system" fn AvRevertMmThreadCharacteristics(avrthandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { AvRevertMmThreadCharacteristics(avrthandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroup(context: super::PHANDLE, period: super::PLARGE_INTEGER, threadorderingguid: *mut windows_core::GUID, timeout: Option<super::PLARGE_INTEGER>) -> windows_core::BOOL {
    windows_core::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroup(context : super::PHANDLE, period : super::PLARGE_INTEGER, threadorderingguid : *mut windows_core::GUID, timeout : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { AvRtCreateThreadOrderingGroup(context as _, period, threadorderingguid as _, timeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroupExA<P4>(context: super::PHANDLE, period: super::PLARGE_INTEGER, threadorderingguid: *mut windows_core::GUID, timeout: Option<super::PLARGE_INTEGER>, taskname: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroupExA(context : super::PHANDLE, period : super::PLARGE_INTEGER, threadorderingguid : *mut windows_core::GUID, timeout : super::PLARGE_INTEGER, taskname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AvRtCreateThreadOrderingGroupExA(context as _, period, threadorderingguid as _, timeout.unwrap_or(core::mem::zeroed()) as _, taskname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroupExW<P4>(context: super::PHANDLE, period: super::PLARGE_INTEGER, threadorderingguid: *mut windows_core::GUID, timeout: Option<super::PLARGE_INTEGER>, taskname: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroupExW(context : super::PHANDLE, period : super::PLARGE_INTEGER, threadorderingguid : *mut windows_core::GUID, timeout : super::PLARGE_INTEGER, taskname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AvRtCreateThreadOrderingGroupExW(context as _, period, threadorderingguid as _, timeout.unwrap_or(core::mem::zeroed()) as _, taskname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvRtDeleteThreadOrderingGroup(context: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("avrt.dll" "system" fn AvRtDeleteThreadOrderingGroup(context : super::HANDLE) -> windows_core::BOOL);
    unsafe { AvRtDeleteThreadOrderingGroup(context) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvRtJoinThreadOrderingGroup(context: super::PHANDLE, threadorderingguid: *const windows_core::GUID, before: bool) -> windows_core::BOOL {
    windows_core::link!("avrt.dll" "system" fn AvRtJoinThreadOrderingGroup(context : super::PHANDLE, threadorderingguid : *const windows_core::GUID, before : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { AvRtJoinThreadOrderingGroup(context as _, threadorderingguid, before.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvRtLeaveThreadOrderingGroup(context: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("avrt.dll" "system" fn AvRtLeaveThreadOrderingGroup(context : super::HANDLE) -> windows_core::BOOL);
    unsafe { AvRtLeaveThreadOrderingGroup(context) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvRtWaitOnThreadOrderingGroup(context: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("avrt.dll" "system" fn AvRtWaitOnThreadOrderingGroup(context : super::HANDLE) -> windows_core::BOOL);
    unsafe { AvRtWaitOnThreadOrderingGroup(context) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn AvSetMmMaxThreadCharacteristicsA<P0, P1>(firsttask: P0, secondtask: P1, taskindex: super::LPDWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("avrt.dll" "system" fn AvSetMmMaxThreadCharacteristicsA(firsttask : windows_core::PCSTR, secondtask : windows_core::PCSTR, taskindex : super::LPDWORD) -> super::HANDLE);
    unsafe { AvSetMmMaxThreadCharacteristicsA(firsttask.param().abi(), secondtask.param().abi(), taskindex as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn AvSetMmMaxThreadCharacteristicsW<P0, P1>(firsttask: P0, secondtask: P1, taskindex: super::LPDWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("avrt.dll" "system" fn AvSetMmMaxThreadCharacteristicsW(firsttask : windows_core::PCWSTR, secondtask : windows_core::PCWSTR, taskindex : super::LPDWORD) -> super::HANDLE);
    unsafe { AvSetMmMaxThreadCharacteristicsW(firsttask.param().abi(), secondtask.param().abi(), taskindex as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn AvSetMmThreadCharacteristicsA<P0>(taskname: P0, taskindex: super::LPDWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("avrt.dll" "system" fn AvSetMmThreadCharacteristicsA(taskname : windows_core::PCSTR, taskindex : super::LPDWORD) -> super::HANDLE);
    unsafe { AvSetMmThreadCharacteristicsA(taskname.param().abi(), taskindex as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn AvSetMmThreadCharacteristicsW<P0>(taskname: P0, taskindex: super::LPDWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("avrt.dll" "system" fn AvSetMmThreadCharacteristicsW(taskname : windows_core::PCWSTR, taskindex : super::LPDWORD) -> super::HANDLE);
    unsafe { AvSetMmThreadCharacteristicsW(taskname.param().abi(), taskindex as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AvSetMmThreadPriority(avrthandle: super::HANDLE, priority: AVRT_PRIORITY) -> windows_core::BOOL {
    windows_core::link!("avrt.dll" "system" fn AvSetMmThreadPriority(avrthandle : super::HANDLE, priority : AVRT_PRIORITY) -> windows_core::BOOL);
    unsafe { AvSetMmThreadPriority(avrthandle, priority) }
}
pub type AVRT_PRIORITY = i32;
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = 2;
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = 1;
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = -1;
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = 0;
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = -2;
pub type PAVRT_PRIORITY = *mut AVRT_PRIORITY;
pub const THREAD_ORDER_GROUP_INFINITE_TIMEOUT: i64 = -1;
