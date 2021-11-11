#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Performance_HardwareCounterProfiling`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisableThreadProfiling();
    #[doc = "*Required features: `Win32_System_Performance_HardwareCounterProfiling`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableThreadProfiling();
    #[doc = "*Required features: `Win32_System_Performance_HardwareCounterProfiling`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadProfiling();
    #[doc = "*Required features: `Win32_System_Performance_HardwareCounterProfiling`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadThreadProfilingData();
}
