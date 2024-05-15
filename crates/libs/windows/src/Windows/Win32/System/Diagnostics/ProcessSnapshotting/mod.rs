#[inline]
pub unsafe fn PssCaptureSnapshot<P0>(processhandle: P0, captureflags: PSS_CAPTURE_FLAGS, threadcontextflags: u32, snapshothandle: *mut HPSS) -> u32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssCaptureSnapshot(processhandle : super::super::super::Foundation:: HANDLE, captureflags : PSS_CAPTURE_FLAGS, threadcontextflags : u32, snapshothandle : *mut HPSS) -> u32);
    PssCaptureSnapshot(processhandle.param().abi(), captureflags, threadcontextflags, snapshothandle)
}
#[inline]
pub unsafe fn PssDuplicateSnapshot<P0, P1, P2>(sourceprocesshandle: P0, snapshothandle: P1, targetprocesshandle: P2, targetsnapshothandle: *mut HPSS, flags: PSS_DUPLICATE_FLAGS) -> u32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<HPSS>,
    P2: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssDuplicateSnapshot(sourceprocesshandle : super::super::super::Foundation:: HANDLE, snapshothandle : HPSS, targetprocesshandle : super::super::super::Foundation:: HANDLE, targetsnapshothandle : *mut HPSS, flags : PSS_DUPLICATE_FLAGS) -> u32);
    PssDuplicateSnapshot(sourceprocesshandle.param().abi(), snapshothandle.param().abi(), targetprocesshandle.param().abi(), targetsnapshothandle, flags)
}
#[inline]
pub unsafe fn PssFreeSnapshot<P0, P1>(processhandle: P0, snapshothandle: P1) -> u32
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    P1: windows_core::Param<HPSS>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssFreeSnapshot(processhandle : super::super::super::Foundation:: HANDLE, snapshothandle : HPSS) -> u32);
    PssFreeSnapshot(processhandle.param().abi(), snapshothandle.param().abi())
}
#[inline]
pub unsafe fn PssQuerySnapshot<P0>(snapshothandle: P0, informationclass: PSS_QUERY_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, bufferlength: u32) -> u32
where
    P0: windows_core::Param<HPSS>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssQuerySnapshot(snapshothandle : HPSS, informationclass : PSS_QUERY_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, bufferlength : u32) -> u32);
    PssQuerySnapshot(snapshothandle.param().abi(), informationclass, buffer, bufferlength)
}
#[inline]
pub unsafe fn PssWalkMarkerCreate(allocator: Option<*const PSS_ALLOCATOR>, walkmarkerhandle: *mut HPSSWALK) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn PssWalkMarkerCreate(allocator : *const PSS_ALLOCATOR, walkmarkerhandle : *mut HPSSWALK) -> u32);
    PssWalkMarkerCreate(core::mem::transmute(allocator.unwrap_or(std::ptr::null())), walkmarkerhandle)
}
#[inline]
pub unsafe fn PssWalkMarkerFree<P0>(walkmarkerhandle: P0) -> u32
where
    P0: windows_core::Param<HPSSWALK>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssWalkMarkerFree(walkmarkerhandle : HPSSWALK) -> u32);
    PssWalkMarkerFree(walkmarkerhandle.param().abi())
}
#[inline]
pub unsafe fn PssWalkMarkerGetPosition<P0>(walkmarkerhandle: P0, position: *mut usize) -> u32
where
    P0: windows_core::Param<HPSSWALK>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssWalkMarkerGetPosition(walkmarkerhandle : HPSSWALK, position : *mut usize) -> u32);
    PssWalkMarkerGetPosition(walkmarkerhandle.param().abi(), position)
}
#[inline]
pub unsafe fn PssWalkMarkerSeekToBeginning<P0>(walkmarkerhandle: P0) -> u32
where
    P0: windows_core::Param<HPSSWALK>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssWalkMarkerSeekToBeginning(walkmarkerhandle : HPSSWALK) -> u32);
    PssWalkMarkerSeekToBeginning(walkmarkerhandle.param().abi())
}
#[inline]
pub unsafe fn PssWalkMarkerSetPosition<P0>(walkmarkerhandle: P0, position: usize) -> u32
where
    P0: windows_core::Param<HPSSWALK>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssWalkMarkerSetPosition(walkmarkerhandle : HPSSWALK, position : usize) -> u32);
    PssWalkMarkerSetPosition(walkmarkerhandle.param().abi(), position)
}
#[inline]
pub unsafe fn PssWalkSnapshot<P0, P1>(snapshothandle: P0, informationclass: PSS_WALK_INFORMATION_CLASS, walkmarkerhandle: P1, buffer: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<HPSS>,
    P1: windows_core::Param<HPSSWALK>,
{
    windows_targets::link!("kernel32.dll" "system" fn PssWalkSnapshot(snapshothandle : HPSS, informationclass : PSS_WALK_INFORMATION_CLASS, walkmarkerhandle : HPSSWALK, buffer : *mut core::ffi::c_void, bufferlength : u32) -> u32);
    PssWalkSnapshot(snapshothandle.param().abi(), informationclass, walkmarkerhandle.param().abi(), core::mem::transmute(buffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
pub const PSS_CAPTURE_HANDLES: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(4u32);
pub const PSS_CAPTURE_HANDLE_BASIC_INFORMATION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(16u32);
pub const PSS_CAPTURE_HANDLE_NAME_INFORMATION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(8u32);
pub const PSS_CAPTURE_HANDLE_TRACE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(64u32);
pub const PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(32u32);
pub const PSS_CAPTURE_IPT_TRACE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(8192u32);
pub const PSS_CAPTURE_NONE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(0u32);
pub const PSS_CAPTURE_RESERVED_00000002: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(2u32);
pub const PSS_CAPTURE_RESERVED_00000400: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(1024u32);
pub const PSS_CAPTURE_RESERVED_00004000: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(16384u32);
pub const PSS_CAPTURE_THREADS: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(128u32);
pub const PSS_CAPTURE_THREAD_CONTEXT: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(256u32);
pub const PSS_CAPTURE_THREAD_CONTEXT_EXTENDED: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(512u32);
pub const PSS_CAPTURE_VA_CLONE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(1u32);
pub const PSS_CAPTURE_VA_SPACE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(2048u32);
pub const PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(4096u32);
pub const PSS_CREATE_BREAKAWAY: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(134217728u32);
pub const PSS_CREATE_BREAKAWAY_OPTIONAL: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(67108864u32);
pub const PSS_CREATE_FORCE_BREAKAWAY: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(268435456u32);
pub const PSS_CREATE_MEASURE_PERFORMANCE: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(1073741824u32);
pub const PSS_CREATE_RELEASE_SECTION: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(2147483648u32);
pub const PSS_CREATE_USE_VM_ALLOCATIONS: PSS_CAPTURE_FLAGS = PSS_CAPTURE_FLAGS(536870912u32);
pub const PSS_DUPLICATE_CLOSE_SOURCE: PSS_DUPLICATE_FLAGS = PSS_DUPLICATE_FLAGS(1i32);
pub const PSS_DUPLICATE_NONE: PSS_DUPLICATE_FLAGS = PSS_DUPLICATE_FLAGS(0i32);
pub const PSS_HANDLE_HAVE_BASIC_INFORMATION: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(4i32);
pub const PSS_HANDLE_HAVE_NAME: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(2i32);
pub const PSS_HANDLE_HAVE_TYPE: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(1i32);
pub const PSS_HANDLE_HAVE_TYPE_SPECIFIC_INFORMATION: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(8i32);
pub const PSS_HANDLE_NONE: PSS_HANDLE_FLAGS = PSS_HANDLE_FLAGS(0i32);
pub const PSS_OBJECT_TYPE_EVENT: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(4i32);
pub const PSS_OBJECT_TYPE_MUTANT: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(3i32);
pub const PSS_OBJECT_TYPE_PROCESS: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(1i32);
pub const PSS_OBJECT_TYPE_SECTION: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(5i32);
pub const PSS_OBJECT_TYPE_SEMAPHORE: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(6i32);
pub const PSS_OBJECT_TYPE_THREAD: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(2i32);
pub const PSS_OBJECT_TYPE_UNKNOWN: PSS_OBJECT_TYPE = PSS_OBJECT_TYPE(0i32);
pub const PSS_PERF_RESOLUTION: u32 = 1000000u32;
pub const PSS_PROCESS_FLAGS_FROZEN: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(16i32);
pub const PSS_PROCESS_FLAGS_NONE: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(0i32);
pub const PSS_PROCESS_FLAGS_PROTECTED: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(1i32);
pub const PSS_PROCESS_FLAGS_RESERVED_03: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(4i32);
pub const PSS_PROCESS_FLAGS_RESERVED_04: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(8i32);
pub const PSS_PROCESS_FLAGS_WOW64: PSS_PROCESS_FLAGS = PSS_PROCESS_FLAGS(2i32);
pub const PSS_QUERY_AUXILIARY_PAGES_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(2i32);
pub const PSS_QUERY_HANDLE_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(4i32);
pub const PSS_QUERY_HANDLE_TRACE_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(6i32);
pub const PSS_QUERY_PERFORMANCE_COUNTERS: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(7i32);
pub const PSS_QUERY_PROCESS_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(0i32);
pub const PSS_QUERY_THREAD_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(5i32);
pub const PSS_QUERY_VA_CLONE_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(1i32);
pub const PSS_QUERY_VA_SPACE_INFORMATION: PSS_QUERY_INFORMATION_CLASS = PSS_QUERY_INFORMATION_CLASS(3i32);
pub const PSS_THREAD_FLAGS_NONE: PSS_THREAD_FLAGS = PSS_THREAD_FLAGS(0i32);
pub const PSS_THREAD_FLAGS_TERMINATED: PSS_THREAD_FLAGS = PSS_THREAD_FLAGS(1i32);
pub const PSS_WALK_AUXILIARY_PAGES: PSS_WALK_INFORMATION_CLASS = PSS_WALK_INFORMATION_CLASS(0i32);
pub const PSS_WALK_HANDLES: PSS_WALK_INFORMATION_CLASS = PSS_WALK_INFORMATION_CLASS(2i32);
pub const PSS_WALK_THREADS: PSS_WALK_INFORMATION_CLASS = PSS_WALK_INFORMATION_CLASS(3i32);
pub const PSS_WALK_VA_SPACE: PSS_WALK_INFORMATION_CLASS = PSS_WALK_INFORMATION_CLASS(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSS_CAPTURE_FLAGS(pub u32);
impl windows_core::TypeKind for PSS_CAPTURE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSS_CAPTURE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSS_CAPTURE_FLAGS").field(&self.0).finish()
    }
}
impl PSS_CAPTURE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PSS_CAPTURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PSS_CAPTURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSS_DUPLICATE_FLAGS(pub i32);
impl windows_core::TypeKind for PSS_DUPLICATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSS_DUPLICATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSS_DUPLICATE_FLAGS").field(&self.0).finish()
    }
}
impl PSS_DUPLICATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PSS_DUPLICATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PSS_DUPLICATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSS_HANDLE_FLAGS(pub i32);
impl windows_core::TypeKind for PSS_HANDLE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSS_HANDLE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSS_HANDLE_FLAGS").field(&self.0).finish()
    }
}
impl PSS_HANDLE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PSS_HANDLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PSS_HANDLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSS_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for PSS_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSS_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSS_PROCESS_FLAGS(pub i32);
impl windows_core::TypeKind for PSS_PROCESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSS_PROCESS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSS_PROCESS_FLAGS").field(&self.0).finish()
    }
}
impl PSS_PROCESS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PSS_PROCESS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PSS_PROCESS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSS_QUERY_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for PSS_QUERY_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSS_QUERY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSS_QUERY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSS_THREAD_FLAGS(pub i32);
impl windows_core::TypeKind for PSS_THREAD_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSS_THREAD_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSS_THREAD_FLAGS").field(&self.0).finish()
    }
}
impl PSS_THREAD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PSS_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PSS_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PSS_THREAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PSS_THREAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PSS_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSS_WALK_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for PSS_WALK_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSS_WALK_INFORMATION_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSS_WALK_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPSS(pub isize);
impl HPSS {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HPSS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HPSS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPSSWALK(pub isize);
impl HPSSWALK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for HPSSWALK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HPSSWALK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_ALLOCATOR {
    pub Context: *mut core::ffi::c_void,
    pub AllocRoutine: isize,
    pub FreeRoutine: isize,
}
impl windows_core::TypeKind for PSS_ALLOCATOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_ALLOCATOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_AUXILIARY_PAGES_INFORMATION {
    pub AuxPagesCaptured: u32,
}
impl windows_core::TypeKind for PSS_AUXILIARY_PAGES_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_AUXILIARY_PAGES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Memory")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_AUXILIARY_PAGE_ENTRY {
    pub Address: *mut core::ffi::c_void,
    pub BasicInformation: super::super::Memory::MEMORY_BASIC_INFORMATION,
    pub CaptureTime: super::super::super::Foundation::FILETIME,
    pub PageContents: *mut core::ffi::c_void,
    pub PageSize: u32,
}
#[cfg(feature = "Win32_System_Memory")]
impl windows_core::TypeKind for PSS_AUXILIARY_PAGE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Memory")]
impl Default for PSS_AUXILIARY_PAGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub TypeName: windows_core::PCWSTR,
    pub ObjectNameLength: u16,
    pub ObjectName: windows_core::PCWSTR,
    pub TypeSpecificInformation: PSS_HANDLE_ENTRY_0,
}
impl windows_core::TypeKind for PSS_HANDLE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSS_HANDLE_ENTRY_0 {
    pub Process: PSS_HANDLE_ENTRY_0_2,
    pub Thread: PSS_HANDLE_ENTRY_0_5,
    pub Mutant: PSS_HANDLE_ENTRY_0_1,
    pub Event: PSS_HANDLE_ENTRY_0_0,
    pub Section: PSS_HANDLE_ENTRY_0_3,
    pub Semaphore: PSS_HANDLE_ENTRY_0_4,
}
impl windows_core::TypeKind for PSS_HANDLE_ENTRY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_HANDLE_ENTRY_0_0 {
    pub ManualReset: super::super::super::Foundation::BOOL,
    pub Signaled: super::super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for PSS_HANDLE_ENTRY_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_HANDLE_ENTRY_0_1 {
    pub CurrentCount: i32,
    pub Abandoned: super::super::super::Foundation::BOOL,
    pub OwnerProcessId: u32,
    pub OwnerThreadId: u32,
}
impl windows_core::TypeKind for PSS_HANDLE_ENTRY_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_HANDLE_ENTRY_0_2 {
    pub ExitStatus: u32,
    pub PebBaseAddress: *mut core::ffi::c_void,
    pub AffinityMask: usize,
    pub BasePriority: i32,
    pub ProcessId: u32,
    pub ParentProcessId: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for PSS_HANDLE_ENTRY_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_ENTRY_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_HANDLE_ENTRY_0_3 {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationAttributes: u32,
    pub MaximumSize: i64,
}
impl windows_core::TypeKind for PSS_HANDLE_ENTRY_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_ENTRY_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_HANDLE_ENTRY_0_4 {
    pub CurrentCount: i32,
    pub MaximumCount: i32,
}
impl windows_core::TypeKind for PSS_HANDLE_ENTRY_0_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_ENTRY_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_HANDLE_ENTRY_0_5 {
    pub ExitStatus: u32,
    pub TebBaseAddress: *mut core::ffi::c_void,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub AffinityMask: usize,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Win32StartAddress: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for PSS_HANDLE_ENTRY_0_5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_ENTRY_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_HANDLE_INFORMATION {
    pub HandlesCaptured: u32,
}
impl windows_core::TypeKind for PSS_HANDLE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_HANDLE_TRACE_INFORMATION {
    pub SectionHandle: super::super::super::Foundation::HANDLE,
    pub Size: u32,
}
impl windows_core::TypeKind for PSS_HANDLE_TRACE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_HANDLE_TRACE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for PSS_PERFORMANCE_COUNTERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_PERFORMANCE_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_PROCESS_INFORMATION {
    pub ExitStatus: u32,
    pub PebBaseAddress: *mut core::ffi::c_void,
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
impl windows_core::TypeKind for PSS_PROCESS_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_THREAD_ENTRY {
    pub ExitStatus: u32,
    pub TebBaseAddress: *mut core::ffi::c_void,
    pub ProcessId: u32,
    pub ThreadId: u32,
    pub AffinityMask: usize,
    pub Priority: i32,
    pub BasePriority: i32,
    pub LastSyscallFirstArgument: *mut core::ffi::c_void,
    pub LastSyscallNumber: u16,
    pub CreateTime: super::super::super::Foundation::FILETIME,
    pub ExitTime: super::super::super::Foundation::FILETIME,
    pub KernelTime: super::super::super::Foundation::FILETIME,
    pub UserTime: super::super::super::Foundation::FILETIME,
    pub Win32StartAddress: *mut core::ffi::c_void,
    pub CaptureTime: super::super::super::Foundation::FILETIME,
    pub Flags: PSS_THREAD_FLAGS,
    pub SuspendCount: u16,
    pub SizeOfContextRecord: u16,
    pub ContextRecord: *mut super::Debug::CONTEXT,
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl windows_core::TypeKind for PSS_THREAD_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl Default for PSS_THREAD_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_THREAD_INFORMATION {
    pub ThreadsCaptured: u32,
    pub ContextLength: u32,
}
impl windows_core::TypeKind for PSS_THREAD_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_VA_CLONE_INFORMATION {
    pub VaCloneHandle: super::super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for PSS_VA_CLONE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_VA_CLONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_VA_SPACE_ENTRY {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: u32,
    pub RegionSize: usize,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
    pub TimeDateStamp: u32,
    pub SizeOfImage: u32,
    pub ImageBase: *mut core::ffi::c_void,
    pub CheckSum: u32,
    pub MappedFileNameLength: u16,
    pub MappedFileName: windows_core::PCWSTR,
}
impl windows_core::TypeKind for PSS_VA_SPACE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_VA_SPACE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PSS_VA_SPACE_INFORMATION {
    pub RegionCount: u32,
}
impl windows_core::TypeKind for PSS_VA_SPACE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PSS_VA_SPACE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
