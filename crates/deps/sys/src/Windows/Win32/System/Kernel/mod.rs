#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const MAXUCHAR: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const MAXULONG: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const MAXUSHORT: u32 = 65535u32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const NULL64: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_CASE_INSENSITIVE: i32 = 64i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_DONT_REPARSE: i32 = 4096i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_EXCLUSIVE: i32 = 32i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_FORCE_ACCESS_CHECK: i32 = 1024i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_HANDLE_TAGBITS: i32 = 3i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: i32 = 2048i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_INHERIT: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_KERNEL_HANDLE: i32 = 512i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_OPENIF: i32 = 128i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_OPENLINK: i32 = 256i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_PERMANENT: i32 = 16i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const OBJ_VALID_ATTRIBUTES: i32 = 8178i32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
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
