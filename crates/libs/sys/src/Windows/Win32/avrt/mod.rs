#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvQuerySystemResponsiveness(avrthandle : super::winnt::HANDLE, systemresponsivenessvalue : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvRevertMmThreadCharacteristics(avrthandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroup(context : *mut super::winnt::HANDLE, period : *const i64, threadorderingguid : *mut windows_sys::core::GUID, timeout : *const i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroupExA(context : *mut super::winnt::HANDLE, period : *const i64, threadorderingguid : *mut windows_sys::core::GUID, timeout : *const i64, taskname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroupExW(context : *mut super::winnt::HANDLE, period : *const i64, threadorderingguid : *mut windows_sys::core::GUID, timeout : *const i64, taskname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvRtDeleteThreadOrderingGroup(context : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvRtJoinThreadOrderingGroup(context : *mut super::winnt::HANDLE, threadorderingguid : *const windows_sys::core::GUID, before : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvRtLeaveThreadOrderingGroup(context : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvRtWaitOnThreadOrderingGroup(context : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvSetMmMaxThreadCharacteristicsA(firsttask : windows_sys::core::PCSTR, secondtask : windows_sys::core::PCSTR, taskindex : *mut u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvSetMmMaxThreadCharacteristicsW(firsttask : windows_sys::core::PCWSTR, secondtask : windows_sys::core::PCWSTR, taskindex : *mut u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvSetMmThreadCharacteristicsA(taskname : windows_sys::core::PCSTR, taskindex : *mut u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvSetMmThreadCharacteristicsW(taskname : windows_sys::core::PCWSTR, taskindex : *mut u32) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("avrt.dll" "system" fn AvSetMmThreadPriority(avrthandle : super::winnt::HANDLE, priority : AVRT_PRIORITY) -> windows_sys::core::BOOL);
pub type AVRT_PRIORITY = i32;
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = 2;
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = 1;
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = -1;
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = 0;
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = -2;
pub type PAVRT_PRIORITY = *mut AVRT_PRIORITY;
pub const THREAD_ORDER_GROUP_INFINITE_TIMEOUT: i32 = -1;
