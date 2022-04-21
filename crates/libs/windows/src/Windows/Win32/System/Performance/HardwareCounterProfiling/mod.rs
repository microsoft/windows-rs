#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisableThreadProfiling<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(performancedatahandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisableThreadProfiling(performancedatahandle: super::super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(DisableThreadProfiling(performancedatahandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableThreadProfiling<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(threadhandle: Param0, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, flags: u32, hardwarecounters: u64, performancedatahandle: *mut super::super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(EnableThreadProfiling(threadhandle.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(hardwarecounters), ::core::mem::transmute(performancedatahandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
impl ::core::fmt::Debug for HARDWARE_COUNTER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HARDWARE_COUNTER_DATA").field("Type", &self.Type).field("Reserved", &self.Reserved).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for HARDWARE_COUNTER_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HARDWARE_COUNTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HARDWARE_COUNTER_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for HARDWARE_COUNTER_DATA {}
impl ::core::default::Default for HARDWARE_COUNTER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HARDWARE_COUNTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`*"]
pub const PMCCounter: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`*"]
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = HARDWARE_COUNTER_TYPE(1i32);
impl ::core::marker::Copy for HARDWARE_COUNTER_TYPE {}
impl ::core::clone::Clone for HARDWARE_COUNTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HARDWARE_COUNTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HARDWARE_COUNTER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HARDWARE_COUNTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HARDWARE_COUNTER_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for PERFORMANCE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERFORMANCE_DATA").field("Size", &self.Size).field("Version", &self.Version).field("HwCountersCount", &self.HwCountersCount).field("ContextSwitchCount", &self.ContextSwitchCount).field("WaitReasonBitMap", &self.WaitReasonBitMap).field("CycleTime", &self.CycleTime).field("RetryCount", &self.RetryCount).field("Reserved", &self.Reserved).field("HwCounters", &self.HwCounters).finish()
    }
}
unsafe impl ::windows::core::Abi for PERFORMANCE_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERFORMANCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERFORMANCE_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERFORMANCE_DATA {}
impl ::core::default::Default for PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryThreadProfiling<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(threadhandle: Param0, enabled: *mut super::super::super::Foundation::BOOLEAN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryThreadProfiling(threadhandle: super::super::super::Foundation::HANDLE, enabled: *mut super::super::super::Foundation::BOOLEAN) -> u32;
        }
        ::core::mem::transmute(QueryThreadProfiling(threadhandle.into_param().abi(), ::core::mem::transmute(enabled)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Performance_HardwareCounterProfiling\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadThreadProfilingData<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(performancedatahandle: Param0, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadThreadProfilingData(performancedatahandle: super::super::super::Foundation::HANDLE, flags: u32, performancedata: *mut PERFORMANCE_DATA) -> u32;
        }
        ::core::mem::transmute(ReadThreadProfilingData(performancedatahandle.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(performancedata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
