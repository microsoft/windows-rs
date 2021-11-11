#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssCaptureSnapshot();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssDuplicateSnapshot();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PssFreeSnapshot();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssQuerySnapshot();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerCreate();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerFree();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerGetPosition();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerSeekToBeginning();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkMarkerSetPosition();
    #[doc = "*Required features: `Win32_System_Diagnostics_ProcessSnapshotting`*"]
    pub fn PssWalkSnapshot();
}
