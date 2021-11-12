#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddVirtualDiskParent(virtualdiskhandle: super::super::Foundation::HANDLE, parentpath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplySnapshotVhdSet(virtualdiskhandle: super::super::Foundation::HANDLE, parameters: *const APPLY_SNAPSHOT_VHDSET_PARAMETERS, flags: APPLY_SNAPSHOT_VHDSET_FLAG) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_Security`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_IO"))]
    pub fn AttachVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, flags: ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, parameters: *const ATTACH_VIRTUAL_DISK_PARAMETERS, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BreakMirrorVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn CompactVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, flags: COMPACT_VIRTUAL_DISK_FLAG, parameters: *const COMPACT_VIRTUAL_DISK_PARAMETERS, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CompleteForkVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_Security`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_IO"))]
    pub fn CreateVirtualDisk(virtualstoragetype: *const VIRTUAL_STORAGE_TYPE, path: super::super::Foundation::PWSTR, virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, flags: CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, parameters: *const CREATE_VIRTUAL_DISK_PARAMETERS, overlapped: *const super::super::System::IO::OVERLAPPED, handle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteSnapshotVhdSet(virtualdiskhandle: super::super::Foundation::HANDLE, parameters: *const DELETE_SNAPSHOT_VHDSET_PARAMETERS, flags: DELETE_SNAPSHOT_VHDSET_FLAG) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteVirtualDiskMetadata(virtualdiskhandle: super::super::Foundation::HANDLE, item: *const ::windows_sys::core::GUID) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetachVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, flags: DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateVirtualDiskMetadata(virtualdiskhandle: super::super::Foundation::HANDLE, numberofitems: *mut u32, items: *mut ::windows_sys::core::GUID) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ExpandVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, flags: EXPAND_VIRTUAL_DISK_FLAG, parameters: *const EXPAND_VIRTUAL_DISK_PARAMETERS, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ForkVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, flags: FORK_VIRTUAL_DISK_FLAG, parameters: *const FORK_VIRTUAL_DISK_PARAMETERS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAllAttachedVirtualDiskPhysicalPaths(pathsbuffersizeinbytes: *mut u32, pathsbuffer: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStorageDependencyInformation(objecthandle: super::super::Foundation::HANDLE, flags: GET_STORAGE_DEPENDENCY_FLAG, storagedependencyinfosize: u32, storagedependencyinfo: *mut STORAGE_DEPENDENCY_INFO, sizeused: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVirtualDiskInformation(virtualdiskhandle: super::super::Foundation::HANDLE, virtualdiskinfosize: *mut u32, virtualdiskinfo: *mut GET_VIRTUAL_DISK_INFO, sizeused: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVirtualDiskMetadata(virtualdiskhandle: super::super::Foundation::HANDLE, item: *const ::windows_sys::core::GUID, metadatasize: *mut u32, metadata: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetVirtualDiskOperationProgress(virtualdiskhandle: super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED, progress: *mut VIRTUAL_DISK_PROGRESS) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVirtualDiskPhysicalPath(virtualdiskhandle: super::super::Foundation::HANDLE, diskpathsizeinbytes: *mut u32, diskpath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn MergeVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, flags: MERGE_VIRTUAL_DISK_FLAG, parameters: *const MERGE_VIRTUAL_DISK_PARAMETERS, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn MirrorVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, flags: MIRROR_VIRTUAL_DISK_FLAG, parameters: *const MIRROR_VIRTUAL_DISK_PARAMETERS, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyVhdSet(virtualdiskhandle: super::super::Foundation::HANDLE, parameters: *const MODIFY_VHDSET_PARAMETERS, flags: MODIFY_VHDSET_FLAG) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenVirtualDisk(virtualstoragetype: *const VIRTUAL_STORAGE_TYPE, path: super::super::Foundation::PWSTR, virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK, flags: OPEN_VIRTUAL_DISK_FLAG, parameters: *const OPEN_VIRTUAL_DISK_PARAMETERS, handle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryChangesVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, changetrackingid: super::super::Foundation::PWSTR, byteoffset: u64, bytelength: u64, flags: QUERY_CHANGES_VIRTUAL_DISK_FLAG, ranges: *mut QUERY_CHANGES_VIRTUAL_DISK_RANGE, rangecount: *mut u32, processedlength: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RawSCSIVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, parameters: *const RAW_SCSI_VIRTUAL_DISK_PARAMETERS, flags: RAW_SCSI_VIRTUAL_DISK_FLAG, response: *mut RAW_SCSI_VIRTUAL_DISK_RESPONSE) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ResizeVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE, flags: RESIZE_VIRTUAL_DISK_FLAG, parameters: *const RESIZE_VIRTUAL_DISK_PARAMETERS, overlapped: *const super::super::System::IO::OVERLAPPED) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVirtualDiskInformation(virtualdiskhandle: super::super::Foundation::HANDLE, virtualdiskinfo: *const SET_VIRTUAL_DISK_INFO) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetVirtualDiskMetadata(virtualdiskhandle: super::super::Foundation::HANDLE, item: *const ::windows_sys::core::GUID, metadatasize: u32, metadata: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Storage_Vhd`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TakeSnapshotVhdSet(virtualdiskhandle: super::super::Foundation::HANDLE, parameters: *const TAKE_SNAPSHOT_VHDSET_PARAMETERS, flags: TAKE_SNAPSHOT_VHDSET_FLAG) -> u32;
}
pub struct APPLY_SNAPSHOT_VHDSET_FLAG(i32);
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS(i32);
pub struct APPLY_SNAPSHOT_VHDSET_VERSION(i32);
pub struct ATTACH_VIRTUAL_DISK_FLAG(i32);
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS(i32);
pub struct ATTACH_VIRTUAL_DISK_VERSION(i32);
pub struct COMPACT_VIRTUAL_DISK_FLAG(i32);
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS(i32);
pub struct COMPACT_VIRTUAL_DISK_VERSION(i32);
pub struct CREATE_VIRTUAL_DISK_FLAG(i32);
pub struct CREATE_VIRTUAL_DISK_PARAMETERS(i32);
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_BLOCK_SIZE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_SECTOR_SIZE: u32 = 0u32;
pub struct CREATE_VIRTUAL_DISK_VERSION(i32);
pub struct DELETE_SNAPSHOT_VHDSET_FLAG(i32);
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS(i32);
pub struct DELETE_SNAPSHOT_VHDSET_VERSION(i32);
pub struct DEPENDENT_DISK_FLAG(i32);
pub struct DETACH_VIRTUAL_DISK_FLAG(i32);
pub struct EXPAND_VIRTUAL_DISK_FLAG(i32);
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS(i32);
pub struct EXPAND_VIRTUAL_DISK_VERSION(i32);
pub struct FORK_VIRTUAL_DISK_FLAG(i32);
pub struct FORK_VIRTUAL_DISK_PARAMETERS(i32);
pub struct FORK_VIRTUAL_DISK_VERSION(i32);
pub struct GET_STORAGE_DEPENDENCY_FLAG(i32);
pub struct GET_VIRTUAL_DISK_INFO(i32);
pub struct GET_VIRTUAL_DISK_INFO_VERSION(i32);
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const MERGE_VIRTUAL_DISK_DEFAULT_MERGE_DEPTH: u32 = 1u32;
pub struct MERGE_VIRTUAL_DISK_FLAG(i32);
pub struct MERGE_VIRTUAL_DISK_PARAMETERS(i32);
pub struct MERGE_VIRTUAL_DISK_VERSION(i32);
pub struct MIRROR_VIRTUAL_DISK_FLAG(i32);
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS(i32);
pub struct MIRROR_VIRTUAL_DISK_VERSION(i32);
pub struct MODIFY_VHDSET_FLAG(i32);
pub struct MODIFY_VHDSET_PARAMETERS(i32);
pub struct MODIFY_VHDSET_VERSION(i32);
pub struct OPEN_VIRTUAL_DISK_FLAG(i32);
pub struct OPEN_VIRTUAL_DISK_PARAMETERS(i32);
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const OPEN_VIRTUAL_DISK_RW_DEPTH_DEFAULT: u32 = 1u32;
pub struct OPEN_VIRTUAL_DISK_VERSION(i32);
pub struct QUERY_CHANGES_VIRTUAL_DISK_FLAG(i32);
pub struct QUERY_CHANGES_VIRTUAL_DISK_RANGE(i32);
pub struct RAW_SCSI_VIRTUAL_DISK_FLAG(i32);
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS(i32);
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE(i32);
pub struct RAW_SCSI_VIRTUAL_DISK_VERSION(i32);
pub struct RESIZE_VIRTUAL_DISK_FLAG(i32);
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS(i32);
pub struct RESIZE_VIRTUAL_DISK_VERSION(i32);
pub struct SET_VIRTUAL_DISK_INFO(i32);
pub struct SET_VIRTUAL_DISK_INFO_VERSION(i32);
pub struct STORAGE_DEPENDENCY_INFO(i32);
pub struct STORAGE_DEPENDENCY_INFO_TYPE_1(i32);
pub struct STORAGE_DEPENDENCY_INFO_TYPE_2(i32);
pub struct STORAGE_DEPENDENCY_INFO_VERSION(i32);
pub struct TAKE_SNAPSHOT_VHDSET_FLAG(i32);
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS(i32);
pub struct TAKE_SNAPSHOT_VHDSET_VERSION(i32);
pub struct VIRTUAL_DISK_ACCESS_MASK(i32);
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const VIRTUAL_DISK_MAXIMUM_CHANGE_TRACKING_ID_LENGTH: u32 = 256u32;
pub struct VIRTUAL_DISK_PROGRESS(i32);
pub struct VIRTUAL_STORAGE_TYPE(i32);
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const VIRTUAL_STORAGE_TYPE_DEVICE_ISO: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const VIRTUAL_STORAGE_TYPE_DEVICE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHD: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDSET: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Vhd`*"]
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDX: u32 = 3u32;
pub const VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3969403628, data2: 41209, data3: 18409, data4: [144, 31, 113, 65, 90, 102, 52, 91] };
pub const VIRTUAL_STORAGE_TYPE_VENDOR_UNKNOWN: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
