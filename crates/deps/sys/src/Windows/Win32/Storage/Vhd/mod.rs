#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddVirtualDiskParent();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplySnapshotVhdSet();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_Security`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_IO"))]
    pub fn AttachVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BreakMirrorVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CompactVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompleteForkVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_Security`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_IO"))]
    pub fn CreateVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSnapshotVhdSet();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteVirtualDiskMetadata();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetachVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateVirtualDiskMetadata();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ExpandVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ForkVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAllAttachedVirtualDiskPhysicalPaths();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStorageDependencyInformation();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVirtualDiskInformation();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVirtualDiskMetadata();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetVirtualDiskOperationProgress();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVirtualDiskPhysicalPath();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn MergeVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn MirrorVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyVhdSet();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryChangesVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RawSCSIVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ResizeVirtualDisk();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVirtualDiskInformation();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVirtualDiskMetadata();
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TakeSnapshotVhdSet();
}
