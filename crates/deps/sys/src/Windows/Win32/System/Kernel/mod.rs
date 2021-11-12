#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn RtlFirstEntrySList(listhead: *const SLIST_HEADER) -> *mut SLIST_ENTRY;
    pub fn RtlInitializeSListHead(listhead: *mut SLIST_HEADER);
    pub fn RtlInterlockedFlushSList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY;
    pub fn RtlInterlockedPopEntrySList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY;
    pub fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SLIST_ENTRY) -> *mut SLIST_ENTRY;
    pub fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SLIST_ENTRY, listend: *mut SLIST_ENTRY, count: u32) -> *mut SLIST_ENTRY;
    pub fn RtlQueryDepthSList(listhead: *const SLIST_HEADER) -> u16;
}
#[repr(transparent)]
pub struct COMPARTMENT_ID(pub i32);
pub const UNSPECIFIED_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(0i32);
pub const DEFAULT_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CSTRING(i32);
#[repr(transparent)]
pub struct EVENT_TYPE(pub i32);
pub const NotificationEvent: EVENT_TYPE = EVENT_TYPE(0i32);
pub const SynchronizationEvent: EVENT_TYPE = EVENT_TYPE(1i32);
#[repr(transparent)]
pub struct EXCEPTION_DISPOSITION(pub i32);
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(0i32);
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(1i32);
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(2i32);
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(3i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[repr(C)]
pub struct EXCEPTION_REGISTRATION_RECORD(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type EXCEPTION_ROUTINE = unsafe extern "system" fn(exceptionrecord: *mut super::Diagnostics::Debug::EXCEPTION_RECORD, establisherframe: *const ::core::ffi::c_void, contextrecord: *mut super::Diagnostics::Debug::CONTEXT, dispatchercontext: *const ::core::ffi::c_void) -> EXCEPTION_DISPOSITION;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct FLOATING_SAVE_AREA(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct FLOATING_SAVE_AREA(i32);
#[repr(C)]
pub struct LIST_ENTRY(i32);
#[repr(C)]
pub struct LIST_ENTRY32(i32);
#[repr(C)]
pub struct LIST_ENTRY64(i32);
pub const MAXUCHAR: u32 = 255u32;
pub const MAXULONG: u32 = 4294967295u32;
pub const MAXUSHORT: u32 = 65535u32;
#[repr(transparent)]
pub struct NT_PRODUCT_TYPE(pub i32);
pub const NtProductWinNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(1i32);
pub const NtProductLanManNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(2i32);
pub const NtProductServer: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(3i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
#[repr(C)]
pub struct NT_TIB(i32);
pub const NULL64: u32 = 0u32;
#[repr(C)]
pub struct OBJECTID(i32);
#[repr(C)]
pub struct OBJECT_ATTRIBUTES32(i32);
#[repr(C)]
pub struct OBJECT_ATTRIBUTES64(i32);
pub const OBJ_CASE_INSENSITIVE: i32 = 64i32;
pub const OBJ_DONT_REPARSE: i32 = 4096i32;
pub const OBJ_EXCLUSIVE: i32 = 32i32;
pub const OBJ_FORCE_ACCESS_CHECK: i32 = 1024i32;
pub const OBJ_HANDLE_TAGBITS: i32 = 3i32;
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: i32 = 2048i32;
pub const OBJ_INHERIT: i32 = 2i32;
pub const OBJ_KERNEL_HANDLE: i32 = 512i32;
pub const OBJ_OPENIF: i32 = 128i32;
pub const OBJ_OPENLINK: i32 = 256i32;
pub const OBJ_PERMANENT: i32 = 16i32;
pub const OBJ_VALID_ATTRIBUTES: i32 = 8178i32;
#[repr(C)]
pub struct PROCESSOR_NUMBER(i32);
#[repr(C)]
pub struct QUAD(i32);
#[repr(C)]
pub struct RTL_BALANCED_NODE(i32);
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
#[repr(C)]
pub struct SINGLE_LIST_ENTRY(i32);
#[repr(C)]
pub struct SINGLE_LIST_ENTRY32(i32);
#[repr(C)]
pub struct SLIST_ENTRY(i32);
#[cfg(any(target_arch = "aarch64",))]
#[repr(C)]
pub struct SLIST_HEADER(i32);
#[cfg(any(target_arch = "x86_64",))]
#[repr(C)]
pub struct SLIST_HEADER(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct SLIST_HEADER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STRING(i32);
#[repr(C)]
pub struct STRING32(i32);
#[repr(C)]
pub struct STRING64(i32);
#[repr(transparent)]
pub struct SUITE_TYPE(pub i32);
pub const SmallBusiness: SUITE_TYPE = SUITE_TYPE(0i32);
pub const Enterprise: SUITE_TYPE = SUITE_TYPE(1i32);
pub const BackOffice: SUITE_TYPE = SUITE_TYPE(2i32);
pub const CommunicationServer: SUITE_TYPE = SUITE_TYPE(3i32);
pub const TerminalServer: SUITE_TYPE = SUITE_TYPE(4i32);
pub const SmallBusinessRestricted: SUITE_TYPE = SUITE_TYPE(5i32);
pub const EmbeddedNT: SUITE_TYPE = SUITE_TYPE(6i32);
pub const DataCenter: SUITE_TYPE = SUITE_TYPE(7i32);
pub const SingleUserTS: SUITE_TYPE = SUITE_TYPE(8i32);
pub const Personal: SUITE_TYPE = SUITE_TYPE(9i32);
pub const Blade: SUITE_TYPE = SUITE_TYPE(10i32);
pub const EmbeddedRestricted: SUITE_TYPE = SUITE_TYPE(11i32);
pub const SecurityAppliance: SUITE_TYPE = SUITE_TYPE(12i32);
pub const StorageServer: SUITE_TYPE = SUITE_TYPE(13i32);
pub const ComputeServer: SUITE_TYPE = SUITE_TYPE(14i32);
pub const WHServer: SUITE_TYPE = SUITE_TYPE(15i32);
pub const PhoneNT: SUITE_TYPE = SUITE_TYPE(16i32);
pub const MultiUserTS: SUITE_TYPE = SUITE_TYPE(17i32);
pub const MaxSuiteType: SUITE_TYPE = SUITE_TYPE(18i32);
#[repr(transparent)]
pub struct TIMER_TYPE(pub i32);
pub const NotificationTimer: TIMER_TYPE = TIMER_TYPE(0i32);
pub const SynchronizationTimer: TIMER_TYPE = TIMER_TYPE(1i32);
#[repr(transparent)]
pub struct WAIT_TYPE(pub i32);
pub const WaitAll: WAIT_TYPE = WAIT_TYPE(0i32);
pub const WaitAny: WAIT_TYPE = WAIT_TYPE(1i32);
pub const WaitNotification: WAIT_TYPE = WAIT_TYPE(2i32);
pub const WaitDequeue: WAIT_TYPE = WAIT_TYPE(3i32);
pub const WaitDpc: WAIT_TYPE = WAIT_TYPE(4i32);
#[repr(C)]
pub struct WNF_STATE_NAME(i32);
