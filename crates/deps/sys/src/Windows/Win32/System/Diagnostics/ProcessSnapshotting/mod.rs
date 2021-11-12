#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssCaptureSnapshot(processhandle: super::super::super::Foundation::HANDLE, captureflags: PSS_CAPTURE_FLAGS, threadcontextflags: u32, snapshothandle: *mut HPSS) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssDuplicateSnapshot(sourceprocesshandle: super::super::super::Foundation::HANDLE, snapshothandle: HPSS, targetprocesshandle: super::super::super::Foundation::HANDLE, targetsnapshothandle: *mut HPSS, flags: PSS_DUPLICATE_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssFreeSnapshot(processhandle: super::super::super::Foundation::HANDLE, snapshothandle: HPSS) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssQuerySnapshot(snapshothandle: HPSS, informationclass: PSS_QUERY_INFORMATION_CLASS, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerCreate(allocator: *const PSS_ALLOCATOR, walkmarkerhandle: *mut HPSSWALK) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerFree(walkmarkerhandle: HPSSWALK) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerGetPosition(walkmarkerhandle: HPSSWALK, position: *mut usize) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerSeekToBeginning(walkmarkerhandle: HPSSWALK) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerSetPosition(walkmarkerhandle: HPSSWALK, position: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkSnapshot(snapshothandle: HPSS, informationclass: PSS_WALK_INFORMATION_CLASS, walkmarkerhandle: HPSSWALK, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> u32;
}
pub struct HPSS(i32);
pub struct HPSSWALK(i32);
pub struct PSS_ALLOCATOR(i32);
pub struct PSS_AUXILIARY_PAGES_INFORMATION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
pub struct PSS_AUXILIARY_PAGE_ENTRY(i32);
pub struct PSS_CAPTURE_FLAGS(i32);
pub struct PSS_DUPLICATE_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_ENTRY(i32);
pub struct PSS_HANDLE_FLAGS(i32);
pub struct PSS_HANDLE_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_HANDLE_TRACE_INFORMATION(i32);
pub struct PSS_OBJECT_TYPE(i32);
pub struct PSS_PERFORMANCE_COUNTERS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
pub const PSS_PERF_RESOLUTION: u32 = 1000000u32;
pub struct PSS_PROCESS_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_PROCESS_INFORMATION(i32);
pub struct PSS_QUERY_INFORMATION_CLASS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub struct PSS_THREAD_ENTRY(i32);
pub struct PSS_THREAD_FLAGS(i32);
pub struct PSS_THREAD_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_VA_CLONE_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PSS_VA_SPACE_ENTRY(i32);
pub struct PSS_VA_SPACE_INFORMATION(i32);
pub struct PSS_WALK_INFORMATION_CLASS(i32);
