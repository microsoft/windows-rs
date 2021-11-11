#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlFirstEntrySList();
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInitializeSListHead();
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInterlockedFlushSList();
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInterlockedPopEntrySList();
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInterlockedPushEntrySList();
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInterlockedPushListSListEx();
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlQueryDepthSList();
}
