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
impl ::core::marker::Copy for COMPARTMENT_ID {}
impl ::core::clone::Clone for COMPARTMENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CSTRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CSTRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EVENT_TYPE(pub i32);
pub const NotificationEvent: EVENT_TYPE = EVENT_TYPE(0i32);
pub const SynchronizationEvent: EVENT_TYPE = EVENT_TYPE(1i32);
impl ::core::marker::Copy for EVENT_TYPE {}
impl ::core::clone::Clone for EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EXCEPTION_DISPOSITION(pub i32);
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(0i32);
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(1i32);
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(2i32);
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(3i32);
impl ::core::marker::Copy for EXCEPTION_DISPOSITION {}
impl ::core::clone::Clone for EXCEPTION_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub struct EXCEPTION_REGISTRATION_RECORD {
    pub Next: *mut EXCEPTION_REGISTRATION_RECORD,
    pub Handler: ::core::option::Option<EXCEPTION_ROUTINE>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::marker::Copy for EXCEPTION_REGISTRATION_RECORD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::clone::Clone for EXCEPTION_REGISTRATION_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type EXCEPTION_ROUTINE = unsafe extern "system" fn(exceptionrecord: *mut super::Diagnostics::Debug::EXCEPTION_RECORD, establisherframe: *const ::core::ffi::c_void, contextrecord: *mut super::Diagnostics::Debug::CONTEXT, dispatchercontext: *const ::core::ffi::c_void) -> EXCEPTION_DISPOSITION;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Cr0NpxState: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for FLOATING_SAVE_AREA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Spare0: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for FLOATING_SAVE_AREA {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}
impl ::core::marker::Copy for LIST_ENTRY {}
impl ::core::clone::Clone for LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LIST_ENTRY32 {
    pub Flink: u32,
    pub Blink: u32,
}
impl ::core::marker::Copy for LIST_ENTRY32 {}
impl ::core::clone::Clone for LIST_ENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LIST_ENTRY64 {
    pub Flink: u64,
    pub Blink: u64,
}
impl ::core::marker::Copy for LIST_ENTRY64 {}
impl ::core::clone::Clone for LIST_ENTRY64 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAXUCHAR: u32 = 255u32;
pub const MAXULONG: u32 = 4294967295u32;
pub const MAXUSHORT: u32 = 65535u32;
#[repr(transparent)]
pub struct NT_PRODUCT_TYPE(pub i32);
pub const NtProductWinNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(1i32);
pub const NtProductLanManNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(2i32);
pub const NtProductServer: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(3i32);
impl ::core::marker::Copy for NT_PRODUCT_TYPE {}
impl ::core::clone::Clone for NT_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub struct NT_TIB {
    pub ExceptionList: *mut EXCEPTION_REGISTRATION_RECORD,
    pub StackBase: *mut ::core::ffi::c_void,
    pub StackLimit: *mut ::core::ffi::c_void,
    pub SubSystemTib: *mut ::core::ffi::c_void,
    pub Anonymous: NT_TIB_0,
    pub ArbitraryUserPointer: *mut ::core::ffi::c_void,
    pub Self_: *mut NT_TIB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::marker::Copy for NT_TIB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::clone::Clone for NT_TIB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub union NT_TIB_0 {
    pub FiberData: *mut ::core::ffi::c_void,
    pub Version: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::marker::Copy for NT_TIB_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::clone::Clone for NT_TIB_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NULL64: u32 = 0u32;
#[repr(C)]
pub struct OBJECTID {
    pub Lineage: ::windows_sys::core::GUID,
    pub Uniquifier: u32,
}
impl ::core::marker::Copy for OBJECTID {}
impl ::core::clone::Clone for OBJECTID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OBJECT_ATTRIBUTES32 {
    pub Length: u32,
    pub RootDirectory: u32,
    pub ObjectName: u32,
    pub Attributes: u32,
    pub SecurityDescriptor: u32,
    pub SecurityQualityOfService: u32,
}
impl ::core::marker::Copy for OBJECT_ATTRIBUTES32 {}
impl ::core::clone::Clone for OBJECT_ATTRIBUTES32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OBJECT_ATTRIBUTES64 {
    pub Length: u32,
    pub RootDirectory: u64,
    pub ObjectName: u64,
    pub Attributes: u32,
    pub SecurityDescriptor: u64,
    pub SecurityQualityOfService: u64,
}
impl ::core::marker::Copy for OBJECT_ATTRIBUTES64 {}
impl ::core::clone::Clone for OBJECT_ATTRIBUTES64 {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub struct PROCESSOR_NUMBER {
    pub Group: u16,
    pub Number: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for PROCESSOR_NUMBER {}
impl ::core::clone::Clone for PROCESSOR_NUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct QUAD {
    pub Anonymous: QUAD_0,
}
impl ::core::marker::Copy for QUAD {}
impl ::core::clone::Clone for QUAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union QUAD_0 {
    pub UseThisFieldToCopy: i64,
    pub DoNotUseThisField: f64,
}
impl ::core::marker::Copy for QUAD_0 {}
impl ::core::clone::Clone for QUAD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_BALANCED_NODE {
    pub Anonymous1: RTL_BALANCED_NODE_0,
    pub Anonymous2: RTL_BALANCED_NODE_1,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE {}
impl ::core::clone::Clone for RTL_BALANCED_NODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RTL_BALANCED_NODE_0 {
    pub Children: [*mut RTL_BALANCED_NODE; 2],
    pub Anonymous: RTL_BALANCED_NODE_0_0,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_0 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RTL_BALANCED_NODE_0_0 {
    pub Left: *mut RTL_BALANCED_NODE,
    pub Right: *mut RTL_BALANCED_NODE,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_0_0 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union RTL_BALANCED_NODE_1 {
    pub _bitfield: u8,
    pub ParentValue: usize,
}
impl ::core::marker::Copy for RTL_BALANCED_NODE_1 {}
impl ::core::clone::Clone for RTL_BALANCED_NODE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
#[repr(C)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
impl ::core::marker::Copy for SINGLE_LIST_ENTRY {}
impl ::core::clone::Clone for SINGLE_LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SINGLE_LIST_ENTRY32 {
    pub Next: u32,
}
impl ::core::marker::Copy for SINGLE_LIST_ENTRY32 {}
impl ::core::clone::Clone for SINGLE_LIST_ENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SLIST_ENTRY {
    pub Next: *mut SLIST_ENTRY,
}
impl ::core::marker::Copy for SLIST_ENTRY {}
impl ::core::clone::Clone for SLIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64",))]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderArm64: SLIST_HEADER_1,
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64",))]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64",))]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::marker::Copy for SLIST_HEADER_1 {}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::clone::Clone for SLIST_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64",))]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderX64: SLIST_HEADER_1,
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64",))]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64",))]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::marker::Copy for SLIST_HEADER_1 {}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::clone::Clone for SLIST_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
pub union SLIST_HEADER {
    pub Alignment: u64,
    pub Anonymous: SLIST_HEADER_0,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
pub struct SLIST_HEADER_0 {
    pub Next: SINGLE_LIST_ENTRY,
    pub Depth: u16,
    pub CpuId: u16,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STRING32 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u32,
}
impl ::core::marker::Copy for STRING32 {}
impl ::core::clone::Clone for STRING32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STRING64 {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: u64,
}
impl ::core::marker::Copy for STRING64 {}
impl ::core::clone::Clone for STRING64 {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SUITE_TYPE {}
impl ::core::clone::Clone for SUITE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TIMER_TYPE(pub i32);
pub const NotificationTimer: TIMER_TYPE = TIMER_TYPE(0i32);
pub const SynchronizationTimer: TIMER_TYPE = TIMER_TYPE(1i32);
impl ::core::marker::Copy for TIMER_TYPE {}
impl ::core::clone::Clone for TIMER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WAIT_TYPE(pub i32);
pub const WaitAll: WAIT_TYPE = WAIT_TYPE(0i32);
pub const WaitAny: WAIT_TYPE = WAIT_TYPE(1i32);
pub const WaitNotification: WAIT_TYPE = WAIT_TYPE(2i32);
pub const WaitDequeue: WAIT_TYPE = WAIT_TYPE(3i32);
pub const WaitDpc: WAIT_TYPE = WAIT_TYPE(4i32);
impl ::core::marker::Copy for WAIT_TYPE {}
impl ::core::clone::Clone for WAIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WNF_STATE_NAME {
    pub Data: [u32; 2],
}
impl ::core::marker::Copy for WNF_STATE_NAME {}
impl ::core::clone::Clone for WNF_STATE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
