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
