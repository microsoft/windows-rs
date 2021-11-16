#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssCaptureSnapshot(processhandle: super::super::super::Foundation::HANDLE, captureflags: PSS_CAPTURE_FLAGS, threadcontextflags: u32, snapshothandle: *mut HPSS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssDuplicateSnapshot(sourceprocesshandle: super::super::super::Foundation::HANDLE, snapshothandle: HPSS, targetprocesshandle: super::super::super::Foundation::HANDLE, targetsnapshothandle: *mut HPSS, flags: PSS_DUPLICATE_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssFreeSnapshot(processhandle: super::super::super::Foundation::HANDLE, snapshothandle: HPSS) -> u32;
    pub fn PssQuerySnapshot(snapshothandle: HPSS, informationclass: PSS_QUERY_INFORMATION_CLASS, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> u32;
    pub fn PssWalkMarkerCreate(allocator: *const PSS_ALLOCATOR, walkmarkerhandle: *mut HPSSWALK) -> u32;
    pub fn PssWalkMarkerFree(walkmarkerhandle: HPSSWALK) -> u32;
    pub fn PssWalkMarkerGetPosition(walkmarkerhandle: HPSSWALK, position: *mut usize) -> u32;
    pub fn PssWalkMarkerSeekToBeginning(walkmarkerhandle: HPSSWALK) -> u32;
    pub fn PssWalkMarkerSetPosition(walkmarkerhandle: HPSSWALK, position: usize) -> u32;
    pub fn PssWalkSnapshot(snapshothandle: HPSS, informationclass: PSS_WALK_INFORMATION_CLASS, walkmarkerhandle: HPSSWALK, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> u32;
}
pub type HPSS = isize;
pub type HPSSWALK = isize;
#[repr(C)]
pub struct PSS_ALLOCATOR {
    pub Context: *mut ::core::ffi::c_void,
    pub AllocRoutine: isize,
    pub FreeRoutine: isize,
}
impl ::core::marker::Copy for PSS_ALLOCATOR {}
impl ::core::clone::Clone for PSS_ALLOCATOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PSS_AUXILIARY_PAGES_INFORMATION {
    pub AuxPagesCaptured: u32,
}
impl ::core::marker::Copy for PSS_AUXILIARY_PAGES_INFORMATION {}
impl ::core::clone::Clone for PSS_AUXILIARY_PAGES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct PSS_AUXILIARY_PAGE_ENTRY {
    pub Address: *mut ::core::ffi::c_void,
    pub BasicInformation: super::super::Memory::MEMORY_BASIC_INFORMATION,
    pub CaptureTime: super::super::super::Foundation::FILETIME,
    pub PageContents: *mut ::core::ffi::c_void,
    pub PageSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::marker::Copy for PSS_AUXILIARY_PAGE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::clone::Clone for PSS_AUXILIARY_PAGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSS_CAPTURE_NONE: u32 = 0u32;
pub const PSS_CAPTURE_VA_CLONE: u32 = 1u32;
pub const PSS_CAPTURE_RESERVED_00000002: u32 = 2u32;
pub const PSS_CAPTURE_HANDLES: u32 = 4u32;
pub const PSS_CAPTURE_HANDLE_NAME_INFORMATION: u32 = 8u32;
pub const PSS_CAPTURE_HANDLE_BASIC_INFORMATION: u32 = 16u32;
pub const PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION: u32 = 32u32;
pub const PSS_CAPTURE_HANDLE_TRACE: u32 = 64u32;
pub const PSS_CAPTURE_THREADS: u32 = 128u32;
pub const PSS_CAPTURE_THREAD_CONTEXT: u32 = 256u32;
pub const PSS_CAPTURE_THREAD_CONTEXT_EXTENDED: u32 = 512u32;
pub const PSS_CAPTURE_RESERVED_00000400: u32 = 1024u32;
pub const PSS_CAPTURE_VA_SPACE: u32 = 2048u32;
pub const PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION: u32 = 4096u32;
pub const PSS_CAPTURE_IPT_TRACE: u32 = 8192u32;
pub const PSS_CAPTURE_RESERVED_00004000: u32 = 16384u32;
pub const PSS_CREATE_BREAKAWAY_OPTIONAL: u32 = 67108864u32;
pub const PSS_CREATE_BREAKAWAY: u32 = 134217728u32;
pub const PSS_CREATE_FORCE_BREAKAWAY: u32 = 268435456u32;
pub const PSS_CREATE_USE_VM_ALLOCATIONS: u32 = 536870912u32;
pub const PSS_CREATE_MEASURE_PERFORMANCE: u32 = 1073741824u32;
pub const PSS_CREATE_RELEASE_SECTION: u32 = 2147483648u32;
pub const PSS_DUPLICATE_NONE: u32 = 0u32;
pub const PSS_DUPLICATE_CLOSE_SOURCE: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY {
    pub Handle: super::super::super::Foundation::HANDLE,
    pub Flags: PSS_HANDLE_FLAGS,
    pub ObjectType: PSS_OBJECT_TYPE,
    pub CaptureTime: super::super::super::Foundation::FILETIME,
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub PagedPoolCharge: u32,
    pub NonPagedPoolCharge: u32,
    pub CreationTime: super::super::super::Foundation::FILETIME,
    pub TypeNameLength: u16,
    pub TypeName: super::super::super::Foundation::PWSTR,
    pub ObjectNameLength: u16,
    pub ObjectName: super::super::super::Foundation::PWSTR,
    pub TypeSpecificInformation: PSS_HANDLE_ENTRY_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union PSS_HANDLE_ENTRY_0 {
    pub Process: PSS_HANDLE_ENTRY_0_2,
    pub Thread: PSS_HANDLE_ENTRY_0_5,
    pub Mutant: PSS_HANDLE_ENTRY_0_1,
    pub Event: PSS_HANDLE_ENTRY_0_0,
    pub Section: PSS_HANDLE_ENTRY_0_3,
    pub Semaphore: PSS_HANDLE_ENTRY_0_4,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_ENTRY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY_0_0 {
    pub ManualReset: super::super::super::Foundation::BOOL,
    pub Signaled: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_ENTRY_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY_0_1 {
    pub CurrentCount: i32,
    pub Abandoned: super::super::super::Foundation::BOOL,
    pub OwnerProcessId: u32,
    pub OwnerThreadId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_ENTRY_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_ENTRY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY_0_2 {
    pub ExitStatus: u32,
    pub PebBaseAddress: *mut ::core::ffi::c_void,
    pub AffinityMask: usize,
    pub BasePriority: i32,
    pub ProcessId: u32,
    pub ParentProcessId: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_ENTRY_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_ENTRY_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY_0_3 {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub AllocationAttributes: u32,
    pub MaximumSize: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_ENTRY_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_ENTRY_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY_0_4 {
    pub CurrentCount: i32,
    pub MaximumCount: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_ENTRY_0_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_ENTRY_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY_0_5 {
    pub ExitStatus: u32,
    pub TebBaseAddress: *mut ::core::ffi::c_void,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub AffinityMask: usize,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Win32StartAddress: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_ENTRY_0_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_ENTRY_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSS_HANDLE_NONE: u32 = 0u32;
pub const PSS_HANDLE_HAVE_TYPE: u32 = 1u32;
pub const PSS_HANDLE_HAVE_NAME: u32 = 2u32;
pub const PSS_HANDLE_HAVE_BASIC_INFORMATION: u32 = 4u32;
pub const PSS_HANDLE_HAVE_TYPE_SPECIFIC_INFORMATION: u32 = 8u32;
#[repr(C)]
pub struct PSS_HANDLE_INFORMATION {
    pub HandlesCaptured: u32,
}
impl ::core::marker::Copy for PSS_HANDLE_INFORMATION {}
impl ::core::clone::Clone for PSS_HANDLE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_TRACE_INFORMATION {
    pub SectionHandle: super::super::super::Foundation::HANDLE,
    pub Size: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_HANDLE_TRACE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_HANDLE_TRACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSS_OBJECT_TYPE_UNKNOWN: i32 = 0i32;
pub const PSS_OBJECT_TYPE_PROCESS: i32 = 1i32;
pub const PSS_OBJECT_TYPE_THREAD: i32 = 2i32;
pub const PSS_OBJECT_TYPE_MUTANT: i32 = 3i32;
pub const PSS_OBJECT_TYPE_EVENT: i32 = 4i32;
pub const PSS_OBJECT_TYPE_SECTION: i32 = 5i32;
pub const PSS_OBJECT_TYPE_SEMAPHORE: i32 = 6i32;
#[repr(C)]
pub struct PSS_PERFORMANCE_COUNTERS {
    pub TotalCycleCount: u64,
    pub TotalWallClockPeriod: u64,
    pub VaCloneCycleCount: u64,
    pub VaCloneWallClockPeriod: u64,
    pub VaSpaceCycleCount: u64,
    pub VaSpaceWallClockPeriod: u64,
    pub AuxPagesCycleCount: u64,
    pub AuxPagesWallClockPeriod: u64,
    pub HandlesCycleCount: u64,
    pub HandlesWallClockPeriod: u64,
    pub ThreadsCycleCount: u64,
    pub ThreadsWallClockPeriod: u64,
}
impl ::core::marker::Copy for PSS_PERFORMANCE_COUNTERS {}
impl ::core::clone::Clone for PSS_PERFORMANCE_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSS_PERF_RESOLUTION: u32 = 1000000u32;
pub const PSS_PROCESS_FLAGS_NONE: u32 = 0u32;
pub const PSS_PROCESS_FLAGS_PROTECTED: u32 = 1u32;
pub const PSS_PROCESS_FLAGS_WOW64: u32 = 2u32;
pub const PSS_PROCESS_FLAGS_RESERVED_03: u32 = 4u32;
pub const PSS_PROCESS_FLAGS_RESERVED_04: u32 = 8u32;
pub const PSS_PROCESS_FLAGS_FROZEN: u32 = 16u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_PROCESS_INFORMATION {
    pub ExitStatus: u32,
    pub PebBaseAddress: *mut ::core::ffi::c_void,
    pub AffinityMask: usize,
    pub BasePriority: i32,
    pub ProcessId: u32,
    pub ParentProcessId: u32,
    pub Flags: PSS_PROCESS_FLAGS,
    pub CreateTime: super::super::super::Foundation::FILETIME,
    pub ExitTime: super::super::super::Foundation::FILETIME,
    pub KernelTime: super::super::super::Foundation::FILETIME,
    pub UserTime: super::super::super::Foundation::FILETIME,
    pub PriorityClass: u32,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
    pub ExecuteFlags: u32,
    pub ImageFileName: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSS_QUERY_PROCESS_INFORMATION: i32 = 0i32;
pub const PSS_QUERY_VA_CLONE_INFORMATION: i32 = 1i32;
pub const PSS_QUERY_AUXILIARY_PAGES_INFORMATION: i32 = 2i32;
pub const PSS_QUERY_VA_SPACE_INFORMATION: i32 = 3i32;
pub const PSS_QUERY_HANDLE_INFORMATION: i32 = 4i32;
pub const PSS_QUERY_THREAD_INFORMATION: i32 = 5i32;
pub const PSS_QUERY_HANDLE_TRACE_INFORMATION: i32 = 6i32;
pub const PSS_QUERY_PERFORMANCE_COUNTERS: i32 = 7i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub struct PSS_THREAD_ENTRY {
    pub ExitStatus: u32,
    pub TebBaseAddress: *mut ::core::ffi::c_void,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub AffinityMask: usize,
    pub Priority: i32,
    pub BasePriority: i32,
    pub LastSyscallFirstArgument: *mut ::core::ffi::c_void,
    pub LastSyscallNumber: u16,
    pub CreateTime: super::super::super::Foundation::FILETIME,
    pub ExitTime: super::super::super::Foundation::FILETIME,
    pub KernelTime: super::super::super::Foundation::FILETIME,
    pub UserTime: super::super::super::Foundation::FILETIME,
    pub Win32StartAddress: *mut ::core::ffi::c_void,
    pub CaptureTime: super::super::super::Foundation::FILETIME,
    pub Flags: PSS_THREAD_FLAGS,
    pub SuspendCount: u16,
    pub SizeOfContextRecord: u16,
    pub ContextRecord: *mut super::Debug::CONTEXT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PSS_THREAD_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PSS_THREAD_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSS_THREAD_FLAGS_NONE: u32 = 0u32;
pub const PSS_THREAD_FLAGS_TERMINATED: u32 = 1u32;
#[repr(C)]
pub struct PSS_THREAD_INFORMATION {
    pub ThreadsCaptured: u32,
    pub ContextLength: u32,
}
impl ::core::marker::Copy for PSS_THREAD_INFORMATION {}
impl ::core::clone::Clone for PSS_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_VA_CLONE_INFORMATION {
    pub VaCloneHandle: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_VA_CLONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_VA_CLONE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_VA_SPACE_ENTRY {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub AllocationBase: *mut ::core::ffi::c_void,
    pub AllocationProtect: u32,
    pub RegionSize: usize,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
    pub TimeDateStamp: u32,
    pub SizeOfImage: u32,
    pub ImageBase: *mut ::core::ffi::c_void,
    pub CheckSum: u32,
    pub MappedFileNameLength: u16,
    pub MappedFileName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSS_VA_SPACE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSS_VA_SPACE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PSS_VA_SPACE_INFORMATION {
    pub RegionCount: u32,
}
impl ::core::marker::Copy for PSS_VA_SPACE_INFORMATION {}
impl ::core::clone::Clone for PSS_VA_SPACE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSS_WALK_AUXILIARY_PAGES: i32 = 0i32;
pub const PSS_WALK_VA_SPACE: i32 = 1i32;
pub const PSS_WALK_HANDLES: i32 = 2i32;
pub const PSS_WALK_THREADS: i32 = 3i32;
