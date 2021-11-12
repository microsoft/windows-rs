#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisableThreadProfiling(performancedatahandle: super::super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, enabled: *mut super::super::super::Foundation::BOOLEAN) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadThreadProfilingData(performancedatahandle: super::super::super::Foundation::HANDLE, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32;
}
#[repr(C)]
pub struct HARDWARE_COUNTER_DATA(i32);
#[repr(transparent)]
pub struct HARDWARE_COUNTER_TYPE(pub i32);
pub const PMCCounter: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(0i32);
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(1i32);
#[repr(C)]
pub struct PERFORMANCE_DATA(i32);
