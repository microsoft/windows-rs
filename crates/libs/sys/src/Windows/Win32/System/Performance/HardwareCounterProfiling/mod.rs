#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisableThreadProfiling(performancedatahandle: super::super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, enabled: *mut super::super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadThreadProfilingData(performancedatahandle: super::super::super::Foundation::HANDLE, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`*"]
pub struct HARDWARE_COUNTER_DATA {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
impl ::core::marker::Copy for HARDWARE_COUNTER_DATA {}
impl ::core::clone::Clone for HARDWARE_COUNTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`*"]
pub type HARDWARE_COUNTER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`*"]
pub const PMCCounter: HARDWARE_COUNTER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`*"]
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`*"]
pub struct PERFORMANCE_DATA {
    pub Size: u16,
    pub Version: u8,
    pub HwCountersCount: u8,
    pub ContextSwitchCount: u32,
    pub WaitReasonBitMap: u64,
    pub CycleTime: u64,
    pub RetryCount: u32,
    pub Reserved: u32,
    pub HwCounters: [HARDWARE_COUNTER_DATA; 16],
}
impl ::core::marker::Copy for PERFORMANCE_DATA {}
impl ::core::clone::Clone for PERFORMANCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
