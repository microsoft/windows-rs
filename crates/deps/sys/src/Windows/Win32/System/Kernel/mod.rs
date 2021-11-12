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
pub struct COMPARTMENT_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CSTRING(i32);
pub struct EVENT_TYPE(i32);
pub struct EXCEPTION_DISPOSITION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub struct EXCEPTION_REGISTRATION_RECORD(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub struct EXCEPTION_ROUTINE(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct FLOATING_SAVE_AREA(i32);
#[cfg(any(target_arch = "x86",))]
pub struct FLOATING_SAVE_AREA(i32);
pub struct LIST_ENTRY(i32);
pub struct LIST_ENTRY32(i32);
pub struct LIST_ENTRY64(i32);
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const MAXUCHAR: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const MAXULONG: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const MAXUSHORT: u32 = 65535u32;
pub struct NT_PRODUCT_TYPE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub struct NT_TIB(i32);
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const NULL64: u32 = 0u32;
pub struct OBJECTID(i32);
pub struct OBJECT_ATTRIBUTES32(i32);
pub struct OBJECT_ATTRIBUTES64(i32);
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
pub struct PROCESSOR_NUMBER(i32);
pub struct QUAD(i32);
pub struct RTL_BALANCED_NODE(i32);
#[doc = "*Required features: `Win32_System_Kernel`*"]
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
pub struct SINGLE_LIST_ENTRY(i32);
pub struct SINGLE_LIST_ENTRY32(i32);
pub struct SLIST_ENTRY(i32);
#[cfg(any(target_arch = "aarch64",))]
pub struct SLIST_HEADER(i32);
#[cfg(any(target_arch = "x86_64",))]
pub struct SLIST_HEADER(i32);
#[cfg(any(target_arch = "x86",))]
pub struct SLIST_HEADER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct STRING(i32);
pub struct STRING32(i32);
pub struct STRING64(i32);
pub struct SUITE_TYPE(i32);
pub struct TIMER_TYPE(i32);
pub struct WAIT_TYPE(i32);
pub struct WNF_STATE_NAME(i32);
