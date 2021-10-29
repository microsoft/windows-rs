#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisableThreadProfiling<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(performancedatahandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisableThreadProfiling(performancedatahandle: super::super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(DisableThreadProfiling(performancedatahandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableThreadProfiling<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(threadhandle: Param0, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(EnableThreadProfiling(threadhandle.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(hardwarecounters), ::std::mem::transmute(performancedatahandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HARDWARE_COUNTER_DATA {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
impl HARDWARE_COUNTER_DATA {}
impl ::std::default::Default for HARDWARE_COUNTER_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HARDWARE_COUNTER_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HARDWARE_COUNTER_DATA").field("Type", &self.Type).field("Reserved", &self.Reserved).field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for HARDWARE_COUNTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved == other.Reserved && self.Value == other.Value
    }
}
impl ::std::cmp::Eq for HARDWARE_COUNTER_DATA {}
unsafe impl ::windows::runtime::Abi for HARDWARE_COUNTER_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HARDWARE_COUNTER_TYPE(pub i32);
pub const PMCCounter: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(0i32);
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(1i32);
impl ::std::convert::From<i32> for HARDWARE_COUNTER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HARDWARE_COUNTER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PERFORMANCE_DATA {}
impl ::std::default::Default for PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PERFORMANCE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERFORMANCE_DATA")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("HwCountersCount", &self.HwCountersCount)
            .field("ContextSwitchCount", &self.ContextSwitchCount)
            .field("WaitReasonBitMap", &self.WaitReasonBitMap)
            .field("CycleTime", &self.CycleTime)
            .field("RetryCount", &self.RetryCount)
            .field("Reserved", &self.Reserved)
            .field("HwCounters", &self.HwCounters)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PERFORMANCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.HwCountersCount == other.HwCountersCount && self.ContextSwitchCount == other.ContextSwitchCount && self.WaitReasonBitMap == other.WaitReasonBitMap && self.CycleTime == other.CycleTime && self.RetryCount == other.RetryCount && self.Reserved == other.Reserved && self.HwCounters == other.HwCounters
    }
}
impl ::std::cmp::Eq for PERFORMANCE_DATA {}
unsafe impl ::windows::runtime::Abi for PERFORMANCE_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryThreadProfiling<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(threadhandle: Param0, enabled: *mut super::super::super::Foundation::BOOLEAN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, enabled: *mut super::super::super::Foundation::BOOLEAN) -> u32;
        }
        ::std::mem::transmute(QueryThreadProfiling(threadhandle.into_param().abi(), ::std::mem::transmute(enabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadThreadProfilingData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(performancedatahandle: Param0, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadThreadProfilingData(performancedatahandle: super::super::super::Foundation::HANDLE, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32;
        }
        ::std::mem::transmute(ReadThreadProfilingData(performancedatahandle.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(performancedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
