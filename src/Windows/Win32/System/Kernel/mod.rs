#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type COMPARTMENT_ID = i32;
pub const UNSPECIFIED_COMPARTMENT_ID: COMPARTMENT_ID = 0i32;
pub const DEFAULT_COMPARTMENT_ID: COMPARTMENT_ID = 1i32;
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CSTRING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CSTRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSTRING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CSTRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type EVENT_TYPE = i32;
pub const NotificationEvent: EVENT_TYPE = 0i32;
pub const SynchronizationEvent: EVENT_TYPE = 1i32;
pub type EXCEPTION_DISPOSITION = i32;
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = 0i32;
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = 1i32;
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = 2i32;
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = 3i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub struct EXCEPTION_REGISTRATION_RECORD {
    pub Next: *mut EXCEPTION_REGISTRATION_RECORD,
    pub Handler: EXCEPTION_ROUTINE,
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
unsafe impl ::windows::core::Abi for EXCEPTION_REGISTRATION_RECORD {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::cmp::PartialEq for EXCEPTION_REGISTRATION_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXCEPTION_REGISTRATION_RECORD>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::cmp::Eq for EXCEPTION_REGISTRATION_RECORD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::default::Default for EXCEPTION_REGISTRATION_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type EXCEPTION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(exceptionrecord: *mut super::Diagnostics::Debug::EXCEPTION_RECORD, establisherframe: *const ::core::ffi::c_void, contextrecord: *mut super::Diagnostics::Debug::CONTEXT, dispatchercontext: *const ::core::ffi::c_void) -> EXCEPTION_DISPOSITION>;
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
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for FLOATING_SAVE_AREA {
    type Abi = Self;
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLOATING_SAVE_AREA>()) == 0 }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for FLOATING_SAVE_AREA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for FLOATING_SAVE_AREA {
    type Abi = Self;
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLOATING_SAVE_AREA>()) == 0 }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for FLOATING_SAVE_AREA {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for LIST_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LIST_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for LIST_ENTRY {}
impl ::core::default::Default for LIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for LIST_ENTRY32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LIST_ENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LIST_ENTRY32>()) == 0 }
    }
}
impl ::core::cmp::Eq for LIST_ENTRY32 {}
impl ::core::default::Default for LIST_ENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for LIST_ENTRY64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LIST_ENTRY64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LIST_ENTRY64>()) == 0 }
    }
}
impl ::core::cmp::Eq for LIST_ENTRY64 {}
impl ::core::default::Default for LIST_ENTRY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MAXUCHAR: u32 = 255u32;
pub const MAXULONG: u32 = 4294967295u32;
pub const MAXUSHORT: u32 = 65535u32;
pub type NT_PRODUCT_TYPE = i32;
pub const NtProductWinNt: NT_PRODUCT_TYPE = 1i32;
pub const NtProductLanManNt: NT_PRODUCT_TYPE = 2i32;
pub const NtProductServer: NT_PRODUCT_TYPE = 3i32;
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
unsafe impl ::windows::core::Abi for NT_TIB {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::cmp::PartialEq for NT_TIB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NT_TIB>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::cmp::Eq for NT_TIB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::default::Default for NT_TIB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
unsafe impl ::windows::core::Abi for NT_TIB_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::cmp::PartialEq for NT_TIB_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NT_TIB_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::cmp::Eq for NT_TIB_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::default::Default for NT_TIB_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const NULL64: u32 = 0u32;
#[repr(C)]
pub struct OBJECTID {
    pub Lineage: ::windows::core::GUID,
    pub Uniquifier: u32,
}
impl ::core::marker::Copy for OBJECTID {}
impl ::core::clone::Clone for OBJECTID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OBJECTID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OBJECTID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECTID>()) == 0 }
    }
}
impl ::core::cmp::Eq for OBJECTID {}
impl ::core::default::Default for OBJECTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for OBJECT_ATTRIBUTES32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECT_ATTRIBUTES32>()) == 0 }
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES32 {}
impl ::core::default::Default for OBJECT_ATTRIBUTES32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for OBJECT_ATTRIBUTES64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECT_ATTRIBUTES64>()) == 0 }
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES64 {}
impl ::core::default::Default for OBJECT_ATTRIBUTES64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for PROCESSOR_NUMBER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESSOR_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESSOR_NUMBER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESSOR_NUMBER {}
impl ::core::default::Default for PROCESSOR_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for QUAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUAD {}
impl ::core::default::Default for QUAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for QUAD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUAD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUAD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUAD_0 {}
impl ::core::default::Default for QUAD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for RTL_BALANCED_NODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_BALANCED_NODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE {}
impl ::core::default::Default for RTL_BALANCED_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for RTL_BALANCED_NODE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_BALANCED_NODE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE_0 {}
impl ::core::default::Default for RTL_BALANCED_NODE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for RTL_BALANCED_NODE_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_BALANCED_NODE_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE_0_0 {}
impl ::core::default::Default for RTL_BALANCED_NODE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for RTL_BALANCED_NODE_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_BALANCED_NODE_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE_1 {}
impl ::core::default::Default for RTL_BALANCED_NODE_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
#[inline]
pub unsafe fn RtlFirstEntrySList(listhead: *const SLIST_HEADER) -> *mut SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFirstEntrySList(listhead: *const SLIST_HEADER) -> *mut SLIST_ENTRY;
        }
        ::core::mem::transmute(RtlFirstEntrySList(::core::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlInitializeSListHead(listhead: *mut SLIST_HEADER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitializeSListHead(listhead: *mut SLIST_HEADER);
        }
        ::core::mem::transmute(RtlInitializeSListHead(::core::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlInterlockedFlushSList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInterlockedFlushSList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY;
        }
        ::core::mem::transmute(RtlInterlockedFlushSList(::core::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlInterlockedPopEntrySList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInterlockedPopEntrySList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY;
        }
        ::core::mem::transmute(RtlInterlockedPopEntrySList(::core::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SLIST_ENTRY) -> *mut SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SLIST_ENTRY) -> *mut SLIST_ENTRY;
        }
        ::core::mem::transmute(RtlInterlockedPushEntrySList(::core::mem::transmute(listhead), ::core::mem::transmute(listentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SLIST_ENTRY, listend: *mut SLIST_ENTRY, count: u32) -> *mut SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SLIST_ENTRY, listend: *mut SLIST_ENTRY, count: u32) -> *mut SLIST_ENTRY;
        }
        ::core::mem::transmute(RtlInterlockedPushListSListEx(::core::mem::transmute(listhead), ::core::mem::transmute(list), ::core::mem::transmute(listend), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlQueryDepthSList(listhead: *const SLIST_HEADER) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlQueryDepthSList(listhead: *const SLIST_HEADER) -> u16;
        }
        ::core::mem::transmute(RtlQueryDepthSList(::core::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
unsafe impl ::windows::core::Abi for SINGLE_LIST_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SINGLE_LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SINGLE_LIST_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SINGLE_LIST_ENTRY {}
impl ::core::default::Default for SINGLE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SINGLE_LIST_ENTRY32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SINGLE_LIST_ENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SINGLE_LIST_ENTRY32>()) == 0 }
    }
}
impl ::core::cmp::Eq for SINGLE_LIST_ENTRY32 {}
impl ::core::default::Default for SINGLE_LIST_ENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for SLIST_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SLIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SLIST_ENTRY {}
impl ::core::default::Default for SLIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for SLIST_HEADER {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for SLIST_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_HEADER>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::cmp::Eq for SLIST_HEADER {}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for SLIST_HEADER_0 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_HEADER_0>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::default::Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for SLIST_HEADER_1 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for SLIST_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_HEADER_1>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::cmp::Eq for SLIST_HEADER_1 {}
#[cfg(any(target_arch = "aarch64",))]
impl ::core::default::Default for SLIST_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "x86_64",))]
unsafe impl ::windows::core::Abi for SLIST_HEADER {
    type Abi = Self;
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::cmp::PartialEq for SLIST_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_HEADER>()) == 0 }
    }
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::cmp::Eq for SLIST_HEADER {}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "x86_64",))]
unsafe impl ::windows::core::Abi for SLIST_HEADER_0 {
    type Abi = Self;
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_HEADER_0>()) == 0 }
    }
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::default::Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "x86_64",))]
unsafe impl ::windows::core::Abi for SLIST_HEADER_1 {
    type Abi = Self;
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::cmp::PartialEq for SLIST_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_HEADER_1>()) == 0 }
    }
}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::cmp::Eq for SLIST_HEADER_1 {}
#[cfg(any(target_arch = "x86_64",))]
impl ::core::default::Default for SLIST_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for SLIST_HEADER {
    type Abi = Self;
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for SLIST_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_HEADER>()) == 0 }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for SLIST_HEADER {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for SLIST_HEADER_0 {
    type Abi = Self;
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SLIST_HEADER_0>()) == 0 }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STRING {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRING>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STRING32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRING32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRING32>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRING32 {}
impl ::core::default::Default for STRING32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for STRING64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STRING64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STRING64>()) == 0 }
    }
}
impl ::core::cmp::Eq for STRING64 {}
impl ::core::default::Default for STRING64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type SUITE_TYPE = i32;
pub const SmallBusiness: SUITE_TYPE = 0i32;
pub const Enterprise: SUITE_TYPE = 1i32;
pub const BackOffice: SUITE_TYPE = 2i32;
pub const CommunicationServer: SUITE_TYPE = 3i32;
pub const TerminalServer: SUITE_TYPE = 4i32;
pub const SmallBusinessRestricted: SUITE_TYPE = 5i32;
pub const EmbeddedNT: SUITE_TYPE = 6i32;
pub const DataCenter: SUITE_TYPE = 7i32;
pub const SingleUserTS: SUITE_TYPE = 8i32;
pub const Personal: SUITE_TYPE = 9i32;
pub const Blade: SUITE_TYPE = 10i32;
pub const EmbeddedRestricted: SUITE_TYPE = 11i32;
pub const SecurityAppliance: SUITE_TYPE = 12i32;
pub const StorageServer: SUITE_TYPE = 13i32;
pub const ComputeServer: SUITE_TYPE = 14i32;
pub const WHServer: SUITE_TYPE = 15i32;
pub const PhoneNT: SUITE_TYPE = 16i32;
pub const MultiUserTS: SUITE_TYPE = 17i32;
pub const MaxSuiteType: SUITE_TYPE = 18i32;
pub type TIMER_TYPE = i32;
pub const NotificationTimer: TIMER_TYPE = 0i32;
pub const SynchronizationTimer: TIMER_TYPE = 1i32;
pub type WAIT_TYPE = i32;
pub const WaitAll: WAIT_TYPE = 0i32;
pub const WaitAny: WAIT_TYPE = 1i32;
pub const WaitNotification: WAIT_TYPE = 2i32;
pub const WaitDequeue: WAIT_TYPE = 3i32;
pub const WaitDpc: WAIT_TYPE = 4i32;
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
unsafe impl ::windows::core::Abi for WNF_STATE_NAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WNF_STATE_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNF_STATE_NAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for WNF_STATE_NAME {}
impl ::core::default::Default for WNF_STATE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
