#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateAllocator();
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateAllocator2();
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateClock();
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateClock2();
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreatePin();
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreatePin2();
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateTopologyNode();
    #[doc = "*Required features: `Win32_Media_KernelStreaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KsCreateTopologyNode2();
}
