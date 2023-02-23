#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[inline]
pub unsafe fn RtlFirstEntrySList(listhead: *const SLIST_HEADER) -> *mut SLIST_ENTRY {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlFirstEntrySList ( listhead : *const SLIST_HEADER ) -> *mut SLIST_ENTRY );
    RtlFirstEntrySList(listhead)
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[inline]
pub unsafe fn RtlInitializeSListHead() -> SLIST_HEADER {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlInitializeSListHead ( listhead : *mut SLIST_HEADER ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<SLIST_HEADER>();
    RtlInitializeSListHead(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[inline]
pub unsafe fn RtlInterlockedFlushSList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlInterlockedFlushSList ( listhead : *mut SLIST_HEADER ) -> *mut SLIST_ENTRY );
    RtlInterlockedFlushSList(listhead)
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[inline]
pub unsafe fn RtlInterlockedPopEntrySList(listhead: *mut SLIST_HEADER) -> *mut SLIST_ENTRY {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlInterlockedPopEntrySList ( listhead : *mut SLIST_HEADER ) -> *mut SLIST_ENTRY );
    RtlInterlockedPopEntrySList(listhead)
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[inline]
pub unsafe fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SLIST_ENTRY) -> *mut SLIST_ENTRY {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlInterlockedPushEntrySList ( listhead : *mut SLIST_HEADER , listentry : *mut SLIST_ENTRY ) -> *mut SLIST_ENTRY );
    RtlInterlockedPushEntrySList(listhead, listentry)
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[inline]
pub unsafe fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SLIST_ENTRY, listend: *mut SLIST_ENTRY, count: u32) -> *mut SLIST_ENTRY {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlInterlockedPushListSListEx ( listhead : *mut SLIST_HEADER , list : *mut SLIST_ENTRY , listend : *mut SLIST_ENTRY , count : u32 ) -> *mut SLIST_ENTRY );
    RtlInterlockedPushListSListEx(listhead, list, listend, count)
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[inline]
pub unsafe fn RtlQueryDepthSList(listhead: *const SLIST_HEADER) -> u16 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlQueryDepthSList ( listhead : *const SLIST_HEADER ) -> u16 );
    RtlQueryDepthSList(listhead)
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MAXUCHAR: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MAXULONG: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MAXUSHORT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NULL64: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_CASE_INSENSITIVE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_DONT_REPARSE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_EXCLUSIVE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_FORCE_ACCESS_CHECK: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_HANDLE_TAGBITS: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_INHERIT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_KERNEL_HANDLE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_OPENIF: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_OPENLINK: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_PERMANENT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const OBJ_VALID_ATTRIBUTES: i32 = 8178i32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const RTL_BALANCED_NODE_RESERVED_PARENT_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPARTMENT_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const UNSPECIFIED_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(0i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const DEFAULT_COMPARTMENT_ID: COMPARTMENT_ID = COMPARTMENT_ID(1i32);
impl ::core::marker::Copy for COMPARTMENT_ID {}
impl ::core::clone::Clone for COMPARTMENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPARTMENT_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COMPARTMENT_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COMPARTMENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPARTMENT_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NotificationEvent: EVENT_TYPE = EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SynchronizationEvent: EVENT_TYPE = EVENT_TYPE(1i32);
impl ::core::marker::Copy for EVENT_TYPE {}
impl ::core::clone::Clone for EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXCEPTION_DISPOSITION(pub i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ExceptionContinueExecution: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(0i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ExceptionContinueSearch: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(1i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ExceptionNestedException: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(2i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ExceptionCollidedUnwind: EXCEPTION_DISPOSITION = EXCEPTION_DISPOSITION(3i32);
impl ::core::marker::Copy for EXCEPTION_DISPOSITION {}
impl ::core::clone::Clone for EXCEPTION_DISPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXCEPTION_DISPOSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EXCEPTION_DISPOSITION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EXCEPTION_DISPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXCEPTION_DISPOSITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NT_PRODUCT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NtProductWinNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NtProductLanManNt: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NtProductServer: NT_PRODUCT_TYPE = NT_PRODUCT_TYPE(3i32);
impl ::core::marker::Copy for NT_PRODUCT_TYPE {}
impl ::core::clone::Clone for NT_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NT_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NT_PRODUCT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NT_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NT_PRODUCT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SUITE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SmallBusiness: SUITE_TYPE = SUITE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const Enterprise: SUITE_TYPE = SUITE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const BackOffice: SUITE_TYPE = SUITE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const CommunicationServer: SUITE_TYPE = SUITE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const TerminalServer: SUITE_TYPE = SUITE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SmallBusinessRestricted: SUITE_TYPE = SUITE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const EmbeddedNT: SUITE_TYPE = SUITE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const DataCenter: SUITE_TYPE = SUITE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SingleUserTS: SUITE_TYPE = SUITE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const Personal: SUITE_TYPE = SUITE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const Blade: SUITE_TYPE = SUITE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const EmbeddedRestricted: SUITE_TYPE = SUITE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SecurityAppliance: SUITE_TYPE = SUITE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const StorageServer: SUITE_TYPE = SUITE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const ComputeServer: SUITE_TYPE = SUITE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WHServer: SUITE_TYPE = SUITE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const PhoneNT: SUITE_TYPE = SUITE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MultiUserTS: SUITE_TYPE = SUITE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const MaxSuiteType: SUITE_TYPE = SUITE_TYPE(18i32);
impl ::core::marker::Copy for SUITE_TYPE {}
impl ::core::clone::Clone for SUITE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SUITE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SUITE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SUITE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SUITE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TIMER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const NotificationTimer: TIMER_TYPE = TIMER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const SynchronizationTimer: TIMER_TYPE = TIMER_TYPE(1i32);
impl ::core::marker::Copy for TIMER_TYPE {}
impl ::core::clone::Clone for TIMER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TIMER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TIMER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TIMER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TIMER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WAIT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitAll: WAIT_TYPE = WAIT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitAny: WAIT_TYPE = WAIT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitNotification: WAIT_TYPE = WAIT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitDequeue: WAIT_TYPE = WAIT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub const WaitDpc: WAIT_TYPE = WAIT_TYPE(4i32);
impl ::core::marker::Copy for WAIT_TYPE {}
impl ::core::clone::Clone for WAIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WAIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WAIT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WAIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WAIT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct CSTRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for CSTRING {}
impl ::core::clone::Clone for CSTRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSTRING").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for CSTRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CSTRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for CSTRING {}
impl ::core::default::Default for CSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
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
impl ::core::fmt::Debug for EXCEPTION_REGISTRATION_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPTION_REGISTRATION_RECORD").field("Next", &self.Next).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::windows::core::TypeKind for EXCEPTION_REGISTRATION_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::default::Default for EXCEPTION_REGISTRATION_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FLOATING_SAVE_AREA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for FLOATING_SAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATING_SAVE_AREA").field("ControlWord", &self.ControlWord).field("StatusWord", &self.StatusWord).field("TagWord", &self.TagWord).field("ErrorOffset", &self.ErrorOffset).field("ErrorSelector", &self.ErrorSelector).field("DataOffset", &self.DataOffset).field("DataSelector", &self.DataSelector).field("RegisterArea", &self.RegisterArea).field("Cr0NpxState", &self.Cr0NpxState).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for FLOATING_SAVE_AREA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.RegisterArea == other.RegisterArea && self.Cr0NpxState == other.Cr0NpxState
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FLOATING_SAVE_AREA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86")]
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
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FLOATING_SAVE_AREA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FLOATING_SAVE_AREA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for FLOATING_SAVE_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATING_SAVE_AREA").field("ControlWord", &self.ControlWord).field("StatusWord", &self.StatusWord).field("TagWord", &self.TagWord).field("ErrorOffset", &self.ErrorOffset).field("ErrorSelector", &self.ErrorSelector).field("DataOffset", &self.DataOffset).field("DataSelector", &self.DataSelector).field("RegisterArea", &self.RegisterArea).field("Spare0", &self.Spare0).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for FLOATING_SAVE_AREA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FLOATING_SAVE_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.ControlWord == other.ControlWord && self.StatusWord == other.StatusWord && self.TagWord == other.TagWord && self.ErrorOffset == other.ErrorOffset && self.ErrorSelector == other.ErrorSelector && self.DataOffset == other.DataOffset && self.DataSelector == other.DataSelector && self.RegisterArea == other.RegisterArea && self.Spare0 == other.Spare0
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FLOATING_SAVE_AREA {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for LIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY").field("Flink", &self.Flink).field("Blink", &self.Blink).finish()
    }
}
impl ::windows::core::TypeKind for LIST_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY {}
impl ::core::default::Default for LIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for LIST_ENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY32").field("Flink", &self.Flink).field("Blink", &self.Blink).finish()
    }
}
impl ::windows::core::TypeKind for LIST_ENTRY32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LIST_ENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY32 {}
impl ::core::default::Default for LIST_ENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for LIST_ENTRY64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIST_ENTRY64").field("Flink", &self.Flink).field("Blink", &self.Blink).finish()
    }
}
impl ::windows::core::TypeKind for LIST_ENTRY64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LIST_ENTRY64 {
    fn eq(&self, other: &Self) -> bool {
        self.Flink == other.Flink && self.Blink == other.Blink
    }
}
impl ::core::cmp::Eq for LIST_ENTRY64 {}
impl ::core::default::Default for LIST_ENTRY64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
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
impl ::windows::core::TypeKind for NT_TIB {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::default::Default for NT_TIB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
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
impl ::windows::core::TypeKind for NT_TIB_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
impl ::core::default::Default for NT_TIB_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for OBJECTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTID").field("Lineage", &self.Lineage).field("Uniquifier", &self.Uniquifier).finish()
    }
}
impl ::windows::core::TypeKind for OBJECTID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OBJECTID {
    fn eq(&self, other: &Self) -> bool {
        self.Lineage == other.Lineage && self.Uniquifier == other.Uniquifier
    }
}
impl ::core::cmp::Eq for OBJECTID {}
impl ::core::default::Default for OBJECTID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES32").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
impl ::windows::core::TypeKind for OBJECT_ATTRIBUTES32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES32 {}
impl ::core::default::Default for OBJECT_ATTRIBUTES32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for OBJECT_ATTRIBUTES64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECT_ATTRIBUTES64").field("Length", &self.Length).field("RootDirectory", &self.RootDirectory).field("ObjectName", &self.ObjectName).field("Attributes", &self.Attributes).field("SecurityDescriptor", &self.SecurityDescriptor).field("SecurityQualityOfService", &self.SecurityQualityOfService).finish()
    }
}
impl ::windows::core::TypeKind for OBJECT_ATTRIBUTES64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OBJECT_ATTRIBUTES64 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
impl ::core::cmp::Eq for OBJECT_ATTRIBUTES64 {}
impl ::core::default::Default for OBJECT_ATTRIBUTES64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for PROCESSOR_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_NUMBER").field("Group", &self.Group).field("Number", &self.Number).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for PROCESSOR_NUMBER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESSOR_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.Group == other.Group && self.Number == other.Number && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PROCESSOR_NUMBER {}
impl ::core::default::Default for PROCESSOR_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct QUAD {
    pub Anonymous: QUAD_0,
}
impl ::core::marker::Copy for QUAD {}
impl ::core::clone::Clone for QUAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for QUAD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for QUAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::windows::core::TypeKind for QUAD_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for QUAD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::windows::core::TypeKind for RTL_BALANCED_NODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RTL_BALANCED_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::windows::core::TypeKind for RTL_BALANCED_NODE_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RTL_BALANCED_NODE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for RTL_BALANCED_NODE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BALANCED_NODE_0_0").field("Left", &self.Left).field("Right", &self.Right).finish()
    }
}
impl ::windows::core::TypeKind for RTL_BALANCED_NODE_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RTL_BALANCED_NODE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Right == other.Right
    }
}
impl ::core::cmp::Eq for RTL_BALANCED_NODE_0_0 {}
impl ::core::default::Default for RTL_BALANCED_NODE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::windows::core::TypeKind for RTL_BALANCED_NODE_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RTL_BALANCED_NODE_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut SINGLE_LIST_ENTRY,
}
impl ::core::marker::Copy for SINGLE_LIST_ENTRY {}
impl ::core::clone::Clone for SINGLE_LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SINGLE_LIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SINGLE_LIST_ENTRY").field("Next", &self.Next).finish()
    }
}
impl ::windows::core::TypeKind for SINGLE_LIST_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SINGLE_LIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SINGLE_LIST_ENTRY {}
impl ::core::default::Default for SINGLE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct SINGLE_LIST_ENTRY32 {
    pub Next: u32,
}
impl ::core::marker::Copy for SINGLE_LIST_ENTRY32 {}
impl ::core::clone::Clone for SINGLE_LIST_ENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SINGLE_LIST_ENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SINGLE_LIST_ENTRY32").field("Next", &self.Next).finish()
    }
}
impl ::windows::core::TypeKind for SINGLE_LIST_ENTRY32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SINGLE_LIST_ENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SINGLE_LIST_ENTRY32 {}
impl ::core::default::Default for SINGLE_LIST_ENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct SLIST_ENTRY {
    pub Next: *mut SLIST_ENTRY,
}
impl ::core::marker::Copy for SLIST_ENTRY {}
impl ::core::clone::Clone for SLIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SLIST_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_ENTRY").field("Next", &self.Next).finish()
    }
}
impl ::windows::core::TypeKind for SLIST_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SLIST_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next
    }
}
impl ::core::cmp::Eq for SLIST_ENTRY {}
impl ::core::default::Default for SLIST_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderArm64: SLIST_HEADER_1,
}
#[cfg(target_arch = "aarch64")]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(target_arch = "aarch64")]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
impl ::windows::core::TypeKind for SLIST_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[cfg(target_arch = "aarch64")]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(target_arch = "aarch64")]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::fmt::Debug for SLIST_HEADER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_0").field("Alignment", &self.Alignment).field("Region", &self.Region).finish()
    }
}
#[cfg(target_arch = "aarch64")]
impl ::windows::core::TypeKind for SLIST_HEADER_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Alignment == other.Alignment && self.Region == other.Region
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
#[cfg(target_arch = "aarch64")]
impl ::core::default::Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "aarch64")]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[cfg(target_arch = "aarch64")]
impl ::core::marker::Copy for SLIST_HEADER_1 {}
#[cfg(target_arch = "aarch64")]
impl ::core::clone::Clone for SLIST_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::fmt::Debug for SLIST_HEADER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_1").field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).finish()
    }
}
#[cfg(target_arch = "aarch64")]
impl ::windows::core::TypeKind for SLIST_HEADER_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::PartialEq for SLIST_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2
    }
}
#[cfg(target_arch = "aarch64")]
impl ::core::cmp::Eq for SLIST_HEADER_1 {}
#[cfg(target_arch = "aarch64")]
impl ::core::default::Default for SLIST_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderX64: SLIST_HEADER_1,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for SLIST_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::fmt::Debug for SLIST_HEADER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_0").field("Alignment", &self.Alignment).field("Region", &self.Region).finish()
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for SLIST_HEADER_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Alignment == other.Alignment && self.Region == other.Region
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86_64")]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[cfg(target_arch = "x86_64")]
impl ::core::marker::Copy for SLIST_HEADER_1 {}
#[cfg(target_arch = "x86_64")]
impl ::core::clone::Clone for SLIST_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::fmt::Debug for SLIST_HEADER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_1").field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).finish()
    }
}
#[cfg(target_arch = "x86_64")]
impl ::windows::core::TypeKind for SLIST_HEADER_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::PartialEq for SLIST_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2
    }
}
#[cfg(target_arch = "x86_64")]
impl ::core::cmp::Eq for SLIST_HEADER_1 {}
#[cfg(target_arch = "x86_64")]
impl ::core::default::Default for SLIST_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86")]
pub union SLIST_HEADER {
    pub Alignment: u64,
    pub Anonymous: SLIST_HEADER_0,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SLIST_HEADER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SLIST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for SLIST_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
#[cfg(target_arch = "x86")]
pub struct SLIST_HEADER_0 {
    pub Next: SINGLE_LIST_ENTRY,
    pub Depth: u16,
    pub CpuId: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SLIST_HEADER_0 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SLIST_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for SLIST_HEADER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SLIST_HEADER_0").field("Next", &self.Next).field("Depth", &self.Depth).field("CpuId", &self.CpuId).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for SLIST_HEADER_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for SLIST_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Depth == other.Depth && self.CpuId == other.CpuId
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for SLIST_HEADER_0 {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for SLIST_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: ::windows::core::PSTR,
}
impl ::core::marker::Copy for STRING {}
impl ::core::clone::Clone for STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for STRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING {}
impl ::core::default::Default for STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for STRING32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING32").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for STRING32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STRING32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING32 {}
impl ::core::default::Default for STRING32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
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
impl ::core::fmt::Debug for STRING64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STRING64").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for STRING64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STRING64 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for STRING64 {}
impl ::core::default::Default for STRING64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Kernel\"`*"]
pub struct WNF_STATE_NAME {
    pub Data: [u32; 2],
}
impl ::core::marker::Copy for WNF_STATE_NAME {}
impl ::core::clone::Clone for WNF_STATE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WNF_STATE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNF_STATE_NAME").field("Data", &self.Data).finish()
    }
}
impl ::windows::core::TypeKind for WNF_STATE_NAME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WNF_STATE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WNF_STATE_NAME {}
impl ::core::default::Default for WNF_STATE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Kernel\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type EXCEPTION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(exceptionrecord: *mut super::Diagnostics::Debug::EXCEPTION_RECORD, establisherframe: *const ::core::ffi::c_void, contextrecord: *mut super::Diagnostics::Debug::CONTEXT, dispatchercontext: *const ::core::ffi::c_void) -> EXCEPTION_DISPOSITION>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
