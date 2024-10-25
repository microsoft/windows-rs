#[cfg(feature = "Win32_System_Memory_NonVolatile")]
pub mod NonVolatile;
pub const FILE_CACHE_MAX_HARD_DISABLE: u32 = 2u32;
pub const FILE_CACHE_MAX_HARD_ENABLE: u32 = 1u32;
pub const FILE_CACHE_MIN_HARD_DISABLE: u32 = 8u32;
pub const FILE_CACHE_MIN_HARD_ENABLE: u32 = 4u32;
pub const FILE_MAP_ALL_ACCESS: FILE_MAP = 983071u32;
pub const FILE_MAP_COPY: FILE_MAP = 1u32;
pub const FILE_MAP_EXECUTE: FILE_MAP = 32u32;
pub const FILE_MAP_LARGE_PAGES: FILE_MAP = 536870912u32;
pub const FILE_MAP_READ: FILE_MAP = 4u32;
pub const FILE_MAP_RESERVE: FILE_MAP = 2147483648u32;
pub const FILE_MAP_TARGETS_INVALID: FILE_MAP = 1073741824u32;
pub const FILE_MAP_WRITE: FILE_MAP = 2u32;
pub const GHND: GLOBAL_ALLOC_FLAGS = 66u32;
pub const GMEM_FIXED: GLOBAL_ALLOC_FLAGS = 0u32;
pub const GMEM_MOVEABLE: GLOBAL_ALLOC_FLAGS = 2u32;
pub const GMEM_ZEROINIT: GLOBAL_ALLOC_FLAGS = 64u32;
pub const GPTR: GLOBAL_ALLOC_FLAGS = 64u32;
pub const HEAP_CREATE_ALIGN_16: HEAP_FLAGS = 65536u32;
pub const HEAP_CREATE_ENABLE_EXECUTE: HEAP_FLAGS = 262144u32;
pub const HEAP_CREATE_ENABLE_TRACING: HEAP_FLAGS = 131072u32;
pub const HEAP_CREATE_HARDENED: HEAP_FLAGS = 512u32;
pub const HEAP_CREATE_SEGMENT_HEAP: HEAP_FLAGS = 256u32;
pub const HEAP_DISABLE_COALESCE_ON_FREE: HEAP_FLAGS = 128u32;
pub const HEAP_FREE_CHECKING_ENABLED: HEAP_FLAGS = 64u32;
pub const HEAP_GENERATE_EXCEPTIONS: HEAP_FLAGS = 4u32;
pub const HEAP_GROWABLE: HEAP_FLAGS = 2u32;
pub const HEAP_MAXIMUM_TAG: HEAP_FLAGS = 4095u32;
pub const HEAP_NONE: HEAP_FLAGS = 0u32;
pub const HEAP_NO_SERIALIZE: HEAP_FLAGS = 1u32;
pub const HEAP_PSEUDO_TAG_FLAG: HEAP_FLAGS = 32768u32;
pub const HEAP_REALLOC_IN_PLACE_ONLY: HEAP_FLAGS = 16u32;
pub const HEAP_TAG_SHIFT: HEAP_FLAGS = 18u32;
pub const HEAP_TAIL_CHECKING_ENABLED: HEAP_FLAGS = 32u32;
pub const HEAP_ZERO_MEMORY: HEAP_FLAGS = 8u32;
pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = 0i32;
pub const HeapEnableTerminationOnCorruption: HEAP_INFORMATION_CLASS = 1i32;
pub const HeapOptimizeResources: HEAP_INFORMATION_CLASS = 3i32;
pub const HeapTag: HEAP_INFORMATION_CLASS = 7i32;
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = 1i32;
pub const LHND: LOCAL_ALLOC_FLAGS = 66u32;
pub const LMEM_FIXED: LOCAL_ALLOC_FLAGS = 0u32;
pub const LMEM_MOVEABLE: LOCAL_ALLOC_FLAGS = 2u32;
pub const LMEM_ZEROINIT: LOCAL_ALLOC_FLAGS = 64u32;
pub const LPTR: LOCAL_ALLOC_FLAGS = 64u32;
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = 0i32;
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1u32;
pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = 4096u32;
pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = 16384u32;
pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = 65536u32;
pub const MEM_IMAGE: PAGE_TYPE = 16777216u32;
pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE = 536870912u32;
pub const MEM_MAPPED: PAGE_TYPE = 262144u32;
pub const MEM_PRESERVE_PLACEHOLDER: UNMAP_VIEW_OF_FILE_FLAGS = 2u32;
pub const MEM_PRIVATE: PAGE_TYPE = 131072u32;
pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = 32768u32;
pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = 16384u32;
pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = 8192u32;
pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = 262144u32;
pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = 524288u32;
pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE = 16777216u32;
pub const MEM_UNMAP_NONE: UNMAP_VIEW_OF_FILE_FLAGS = 0u32;
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: UNMAP_VIEW_OF_FILE_FLAGS = 1u32;
pub const MemDedicatedAttributeMax: MEM_DEDICATED_ATTRIBUTE_TYPE = 4i32;
pub const MemDedicatedAttributeReadBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = 0i32;
pub const MemDedicatedAttributeReadLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = 1i32;
pub const MemDedicatedAttributeWriteBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = 2i32;
pub const MemDedicatedAttributeWriteLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = 3i32;
pub const MemExtendedParameterAddressRequirements: MEM_EXTENDED_PARAMETER_TYPE = 1i32;
pub const MemExtendedParameterAttributeFlags: MEM_EXTENDED_PARAMETER_TYPE = 5i32;
pub const MemExtendedParameterImageMachine: MEM_EXTENDED_PARAMETER_TYPE = 6i32;
pub const MemExtendedParameterInvalidType: MEM_EXTENDED_PARAMETER_TYPE = 0i32;
pub const MemExtendedParameterMax: MEM_EXTENDED_PARAMETER_TYPE = 7i32;
pub const MemExtendedParameterNumaNode: MEM_EXTENDED_PARAMETER_TYPE = 2i32;
pub const MemExtendedParameterPartitionHandle: MEM_EXTENDED_PARAMETER_TYPE = 3i32;
pub const MemExtendedParameterUserPhysicalHandle: MEM_EXTENDED_PARAMETER_TYPE = 4i32;
pub const MemSectionExtendedParameterInvalidType: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 0i32;
pub const MemSectionExtendedParameterMax: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 4i32;
pub const MemSectionExtendedParameterNumaNode: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 2i32;
pub const MemSectionExtendedParameterSigningLevel: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 3i32;
pub const MemSectionExtendedParameterUserPhysicalFlags: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 1i32;
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = 1i32;
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = 0i32;
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = 0i32;
pub const NONZEROLHND: LOCAL_ALLOC_FLAGS = 2u32;
pub const NONZEROLPTR: LOCAL_ALLOC_FLAGS = 0u32;
pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS = 268435456u32;
pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS = 268435456u32;
pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS = 268435457u32;
pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS = 268435458u32;
pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS = 2147483648u32;
pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS = 536870912u32;
pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = 16u32;
pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = 32u32;
pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = 64u32;
pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS = 128u32;
pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS = 131072u32;
pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS = 16384u32;
pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS = 32768u32;
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = 65536u32;
pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS = 2048u32;
pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS = 262144u32;
pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS = 4096u32;
pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS = 8192u32;
pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = 256u32;
pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = 1u32;
pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = 512u32;
pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = 2u32;
pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = 4u32;
pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS = 2147483648u32;
pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS = 1073741824u32;
pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS = 1073741824u32;
pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = 1024u32;
pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = 8u32;
pub const QUOTA_LIMITS_HARDWS_MAX_DISABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = 8u32;
pub const QUOTA_LIMITS_HARDWS_MAX_ENABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = 4u32;
pub const QUOTA_LIMITS_HARDWS_MIN_DISABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = 2u32;
pub const QUOTA_LIMITS_HARDWS_MIN_ENABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = 1u32;
pub const SECTION_ALL_ACCESS: SECTION_FLAGS = 983071u32;
pub const SECTION_EXTEND_SIZE: SECTION_FLAGS = 16u32;
pub const SECTION_MAP_EXECUTE: SECTION_FLAGS = 8u32;
pub const SECTION_MAP_EXECUTE_EXPLICIT: SECTION_FLAGS = 32u32;
pub const SECTION_MAP_READ: SECTION_FLAGS = 4u32;
pub const SECTION_MAP_WRITE: SECTION_FLAGS = 2u32;
pub const SECTION_QUERY: SECTION_FLAGS = 1u32;
pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = 524288u32;
pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = 134217728u32;
pub const SEC_FILE: PAGE_PROTECTION_FLAGS = 8388608u32;
pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = 16777216u32;
pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS = 285212672u32;
pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS = 2147483648u32;
pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = 268435456u32;
pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS = 262144u32;
pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS = 33554432u32;
pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = 67108864u32;
pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS = 1073741824u32;
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = 3i32;
pub const VmOfferPriorityLow: OFFER_PRIORITY = 2i32;
pub const VmOfferPriorityNormal: OFFER_PRIORITY = 4i32;
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FILE_MAP(pub u32);
impl windows_core::TypeKind for FILE_MAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GLOBAL_ALLOC_FLAGS(pub u32);
impl windows_core::TypeKind for GLOBAL_ALLOC_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HEAP_FLAGS(pub u32);
impl windows_core::TypeKind for HEAP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HEAP_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for HEAP_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LOCAL_ALLOC_FLAGS(pub u32);
impl windows_core::TypeKind for LOCAL_ALLOC_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MEMORY_RESOURCE_NOTIFICATION_TYPE(pub i32);
impl windows_core::TypeKind for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MEM_DEDICATED_ATTRIBUTE_TYPE(pub i32);
impl windows_core::TypeKind for MEM_DEDICATED_ATTRIBUTE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MEM_EXTENDED_PARAMETER_TYPE(pub i32);
impl windows_core::TypeKind for MEM_EXTENDED_PARAMETER_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MEM_SECTION_EXTENDED_PARAMETER_TYPE(pub i32);
impl windows_core::TypeKind for MEM_SECTION_EXTENDED_PARAMETER_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OFFER_PRIORITY(pub i32);
impl windows_core::TypeKind for OFFER_PRIORITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PAGE_PROTECTION_FLAGS(pub u32);
impl windows_core::TypeKind for PAGE_PROTECTION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PAGE_TYPE(pub u32);
impl windows_core::TypeKind for PAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SECTION_FLAGS(pub u32);
impl windows_core::TypeKind for SECTION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SETPROCESSWORKINGSETSIZEEX_FLAGS(pub u32);
impl windows_core::TypeKind for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UNMAP_VIEW_OF_FILE_FLAGS(pub u32);
impl windows_core::TypeKind for UNMAP_VIEW_OF_FILE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VIRTUAL_ALLOCATION_TYPE(pub u32);
impl windows_core::TypeKind for VIRTUAL_ALLOCATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VIRTUAL_FREE_TYPE(pub u32);
impl windows_core::TypeKind for VIRTUAL_FREE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WIN32_MEMORY_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for WIN32_MEMORY_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFG_CALL_TARGET_INFO {
    pub Offset: usize,
    pub Flags: usize,
}
impl Default for CFG_CALL_TARGET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CFG_CALL_TARGET_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbReserved: usize,
    pub cbMaxReserve: usize,
}
impl Default for HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HEAP_SUMMARY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for MEMORY_BASIC_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(target_arch = "x86")]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for MEMORY_BASIC_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION32 {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: u32,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
impl Default for MEMORY_BASIC_INFORMATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEMORY_BASIC_INFORMATION32 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION64 {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
    pub __alignment2: u32,
}
impl Default for MEMORY_BASIC_INFORMATION64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEMORY_BASIC_INFORMATION64 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_MAPPED_VIEW_ADDRESS {
    pub Value: *mut core::ffi::c_void,
}
impl Default for MEMORY_MAPPED_VIEW_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEMORY_MAPPED_VIEW_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    pub Type: MEM_DEDICATED_ATTRIBUTE_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
impl Default for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub SizeOfInformation: u32,
    pub Flags: u32,
    pub AttributesOffset: u32,
    pub AttributeCount: u32,
    pub Reserved: u32,
    pub TypeId: u64,
}
impl Default for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEM_ADDRESS_REQUIREMENTS {
    pub LowestStartingAddress: *mut core::ffi::c_void,
    pub HighestEndingAddress: *mut core::ffi::c_void,
    pub Alignment: usize,
}
impl Default for MEM_ADDRESS_REQUIREMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEM_ADDRESS_REQUIREMENTS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEM_EXTENDED_PARAMETER {
    pub Anonymous1: MEM_EXTENDED_PARAMETER_0,
    pub Anonymous2: MEM_EXTENDED_PARAMETER_1,
}
impl Default for MEM_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEM_EXTENDED_PARAMETER {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEM_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
impl Default for MEM_EXTENDED_PARAMETER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEM_EXTENDED_PARAMETER_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union MEM_EXTENDED_PARAMETER_1 {
    pub ULong64: u64,
    pub Pointer: *mut core::ffi::c_void,
    pub Size: usize,
    pub Handle: super::super::Foundation::HANDLE,
    pub ULong: u32,
}
impl Default for MEM_EXTENDED_PARAMETER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEM_EXTENDED_PARAMETER_1 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: *mut core::ffi::c_void,
    pub cbData: u32,
    pub cbOverhead: u8,
    pub iRegionIndex: u8,
    pub wFlags: u16,
    pub Anonymous: PROCESS_HEAP_ENTRY_0,
}
impl Default for PROCESS_HEAP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_HEAP_ENTRY {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union PROCESS_HEAP_ENTRY_0 {
    pub Block: PROCESS_HEAP_ENTRY_0_0,
    pub Region: PROCESS_HEAP_ENTRY_0_1,
}
impl Default for PROCESS_HEAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_HEAP_ENTRY_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_HEAP_ENTRY_0_0 {
    pub hMem: super::super::Foundation::HANDLE,
    pub dwReserved: [u32; 3],
}
impl Default for PROCESS_HEAP_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_HEAP_ENTRY_0_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_HEAP_ENTRY_0_1 {
    pub dwCommittedSize: u32,
    pub dwUnCommittedSize: u32,
    pub lpFirstBlock: *mut core::ffi::c_void,
    pub lpLastBlock: *mut core::ffi::c_void,
}
impl Default for PROCESS_HEAP_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_HEAP_ENTRY_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION {
    pub Flags: u32,
    pub NumaNode: u32,
    pub Channel: u32,
    pub NumberOfNumaNodes: u32,
    pub ResidentAvailablePages: u64,
    pub CommittedPages: u64,
    pub CommitLimit: u64,
    pub PeakCommitment: u64,
    pub TotalNumberOfPages: u64,
    pub AvailablePages: u64,
    pub ZeroPages: u64,
    pub FreePages: u64,
    pub StandbyPages: u64,
    pub Reserved: [u64; 16],
    pub MaximumCommitLimit: u64,
    pub Reserved2: u64,
    pub PartitionId: u32,
}
impl Default for WIN32_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WIN32_MEMORY_PARTITION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl Default for WIN32_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WIN32_MEMORY_RANGE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_REGION_INFORMATION {
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0,
    pub RegionSize: usize,
    pub CommitSize: usize,
}
impl Default for WIN32_MEMORY_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WIN32_MEMORY_REGION_INFORMATION {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union WIN32_MEMORY_REGION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0_0,
}
impl Default for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WIN32_MEMORY_REGION_INFORMATION_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl Default for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
pub type PBAD_MEMORY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PSECURE_MEMORY_CACHE_CALLBACK = Option<unsafe extern "system" fn(addr: *const core::ffi::c_void, range: usize) -> super::super::Foundation::BOOLEAN>;
