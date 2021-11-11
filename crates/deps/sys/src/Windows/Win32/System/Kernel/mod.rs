#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlFirstEntrySList(listhead: *const SLIST_HEADER) -> *mut SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInitializeSListHead(listhead: *mut SLIST_HEADER);
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInterlockedFlushSList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInterlockedPopEntrySList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SLIST_ENTRY) -> *mut SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SLIST_ENTRY, listend: *mut SLIST_ENTRY, count: u32) -> *mut SLIST_ENTRY;
    #[doc = "*Required features: `Win32_System_Kernel`*"]
    pub fn RtlQueryDepthSList(listhead: *const SLIST_HEADER) -> u16;
}
