windows_targets::link!("kernel32.dll" "system" fn DisableThreadProfiling(performancedatahandle : super::super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("kernel32.dll" "system" fn EnableThreadProfiling(threadhandle : super::super::super::Foundation:: HANDLE, flags : u32, hardwarecounters : u64, performancedatahandle : *mut super::super::super::Foundation:: HANDLE) -> u32);
windows_targets::link!("kernel32.dll" "system" fn QueryThreadProfiling(threadhandle : super::super::super::Foundation:: HANDLE, enabled : *mut super::super::super::Foundation:: BOOLEAN) -> u32);
windows_targets::link!("kernel32.dll" "system" fn ReadThreadProfilingData(performancedatahandle : super::super::super::Foundation:: HANDLE, flags : u32, performancedata : *mut PERFORMANCE_DATA) -> u32);
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = 1i32;
pub const PMCCounter: HARDWARE_COUNTER_TYPE = 0i32;
pub type HARDWARE_COUNTER_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HARDWARE_COUNTER_DATA {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
