#[inline]
pub unsafe fn AddVirtualDiskParent<P0, P1>(virtualdiskhandle: P0, parentpath: P1) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("virtdisk.dll" "system" fn AddVirtualDiskParent(virtualdiskhandle : super::super::Foundation:: HANDLE, parentpath : windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    AddVirtualDiskParent(virtualdiskhandle.param().abi(), parentpath.param().abi())
}
#[inline]
pub unsafe fn ApplySnapshotVhdSet<P0>(virtualdiskhandle: P0, parameters: *const APPLY_SNAPSHOT_VHDSET_PARAMETERS, flags: APPLY_SNAPSHOT_VHDSET_FLAG) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn ApplySnapshotVhdSet(virtualdiskhandle : super::super::Foundation:: HANDLE, parameters : *const APPLY_SNAPSHOT_VHDSET_PARAMETERS, flags : APPLY_SNAPSHOT_VHDSET_FLAG) -> super::super::Foundation:: WIN32_ERROR);
    ApplySnapshotVhdSet(virtualdiskhandle.param().abi(), parameters, flags)
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn AttachVirtualDisk<P0, P1>(virtualdiskhandle: P0, securitydescriptor: P1, flags: ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, parameters: Option<*const ATTACH_VIRTUAL_DISK_PARAMETERS>, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("virtdisk.dll" "system" fn AttachVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, securitydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, flags : ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags : u32, parameters : *const ATTACH_VIRTUAL_DISK_PARAMETERS, overlapped : *const super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: WIN32_ERROR);
    AttachVirtualDisk(virtualdiskhandle.param().abi(), securitydescriptor.param().abi(), flags, providerspecificflags, core::mem::transmute(parameters.unwrap_or(std::ptr::null())), core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn BreakMirrorVirtualDisk<P0>(virtualdiskhandle: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn BreakMirrorVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    BreakMirrorVirtualDisk(virtualdiskhandle.param().abi())
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn CompactVirtualDisk<P0>(virtualdiskhandle: P0, flags: COMPACT_VIRTUAL_DISK_FLAG, parameters: Option<*const COMPACT_VIRTUAL_DISK_PARAMETERS>, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn CompactVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, flags : COMPACT_VIRTUAL_DISK_FLAG, parameters : *const COMPACT_VIRTUAL_DISK_PARAMETERS, overlapped : *const super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: WIN32_ERROR);
    CompactVirtualDisk(virtualdiskhandle.param().abi(), flags, core::mem::transmute(parameters.unwrap_or(std::ptr::null())), core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CompleteForkVirtualDisk<P0>(virtualdiskhandle: P0) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn CompleteForkVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    CompleteForkVirtualDisk(virtualdiskhandle.param().abi())
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn CreateVirtualDisk<P0, P1>(virtualstoragetype: *const VIRTUAL_STORAGE_TYPE, path: P0, virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK, securitydescriptor: P1, flags: CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, parameters: *const CREATE_VIRTUAL_DISK_PARAMETERS, overlapped: Option<*const super::super::System::IO::OVERLAPPED>, handle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("virtdisk.dll" "system" fn CreateVirtualDisk(virtualstoragetype : *const VIRTUAL_STORAGE_TYPE, path : windows_core::PCWSTR, virtualdiskaccessmask : VIRTUAL_DISK_ACCESS_MASK, securitydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, flags : CREATE_VIRTUAL_DISK_FLAG, providerspecificflags : u32, parameters : *const CREATE_VIRTUAL_DISK_PARAMETERS, overlapped : *const super::super::System::IO:: OVERLAPPED, handle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    CreateVirtualDisk(virtualstoragetype, path.param().abi(), virtualdiskaccessmask, securitydescriptor.param().abi(), flags, providerspecificflags, parameters, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())), handle)
}
#[inline]
pub unsafe fn DeleteSnapshotVhdSet<P0>(virtualdiskhandle: P0, parameters: *const DELETE_SNAPSHOT_VHDSET_PARAMETERS, flags: DELETE_SNAPSHOT_VHDSET_FLAG) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn DeleteSnapshotVhdSet(virtualdiskhandle : super::super::Foundation:: HANDLE, parameters : *const DELETE_SNAPSHOT_VHDSET_PARAMETERS, flags : DELETE_SNAPSHOT_VHDSET_FLAG) -> super::super::Foundation:: WIN32_ERROR);
    DeleteSnapshotVhdSet(virtualdiskhandle.param().abi(), parameters, flags)
}
#[inline]
pub unsafe fn DeleteVirtualDiskMetadata<P0>(virtualdiskhandle: P0, item: *const windows_core::GUID) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn DeleteVirtualDiskMetadata(virtualdiskhandle : super::super::Foundation:: HANDLE, item : *const windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    DeleteVirtualDiskMetadata(virtualdiskhandle.param().abi(), item)
}
#[inline]
pub unsafe fn DetachVirtualDisk<P0>(virtualdiskhandle: P0, flags: DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn DetachVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, flags : DETACH_VIRTUAL_DISK_FLAG, providerspecificflags : u32) -> super::super::Foundation:: WIN32_ERROR);
    DetachVirtualDisk(virtualdiskhandle.param().abi(), flags, providerspecificflags)
}
#[inline]
pub unsafe fn EnumerateVirtualDiskMetadata<P0>(virtualdiskhandle: P0, numberofitems: *mut u32, items: *mut windows_core::GUID) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn EnumerateVirtualDiskMetadata(virtualdiskhandle : super::super::Foundation:: HANDLE, numberofitems : *mut u32, items : *mut windows_core::GUID) -> super::super::Foundation:: WIN32_ERROR);
    EnumerateVirtualDiskMetadata(virtualdiskhandle.param().abi(), numberofitems, items)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ExpandVirtualDisk<P0>(virtualdiskhandle: P0, flags: EXPAND_VIRTUAL_DISK_FLAG, parameters: *const EXPAND_VIRTUAL_DISK_PARAMETERS, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn ExpandVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, flags : EXPAND_VIRTUAL_DISK_FLAG, parameters : *const EXPAND_VIRTUAL_DISK_PARAMETERS, overlapped : *const super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: WIN32_ERROR);
    ExpandVirtualDisk(virtualdiskhandle.param().abi(), flags, parameters, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ForkVirtualDisk<P0>(virtualdiskhandle: P0, flags: FORK_VIRTUAL_DISK_FLAG, parameters: *const FORK_VIRTUAL_DISK_PARAMETERS, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn ForkVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, flags : FORK_VIRTUAL_DISK_FLAG, parameters : *const FORK_VIRTUAL_DISK_PARAMETERS, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: WIN32_ERROR);
    ForkVirtualDisk(virtualdiskhandle.param().abi(), flags, parameters, overlapped)
}
#[inline]
pub unsafe fn GetAllAttachedVirtualDiskPhysicalPaths(pathsbuffersizeinbytes: *mut u32, pathsbuffer: windows_core::PWSTR) -> super::super::Foundation::WIN32_ERROR {
    windows_targets::link!("virtdisk.dll" "system" fn GetAllAttachedVirtualDiskPhysicalPaths(pathsbuffersizeinbytes : *mut u32, pathsbuffer : windows_core::PWSTR) -> super::super::Foundation:: WIN32_ERROR);
    GetAllAttachedVirtualDiskPhysicalPaths(pathsbuffersizeinbytes, core::mem::transmute(pathsbuffer))
}
#[inline]
pub unsafe fn GetStorageDependencyInformation<P0>(objecthandle: P0, flags: GET_STORAGE_DEPENDENCY_FLAG, storagedependencyinfosize: u32, storagedependencyinfo: *mut STORAGE_DEPENDENCY_INFO, sizeused: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn GetStorageDependencyInformation(objecthandle : super::super::Foundation:: HANDLE, flags : GET_STORAGE_DEPENDENCY_FLAG, storagedependencyinfosize : u32, storagedependencyinfo : *mut STORAGE_DEPENDENCY_INFO, sizeused : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    GetStorageDependencyInformation(objecthandle.param().abi(), flags, storagedependencyinfosize, storagedependencyinfo, core::mem::transmute(sizeused.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetVirtualDiskInformation<P0>(virtualdiskhandle: P0, virtualdiskinfosize: *mut u32, virtualdiskinfo: *mut GET_VIRTUAL_DISK_INFO, sizeused: Option<*mut u32>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn GetVirtualDiskInformation(virtualdiskhandle : super::super::Foundation:: HANDLE, virtualdiskinfosize : *mut u32, virtualdiskinfo : *mut GET_VIRTUAL_DISK_INFO, sizeused : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    GetVirtualDiskInformation(virtualdiskhandle.param().abi(), virtualdiskinfosize, virtualdiskinfo, core::mem::transmute(sizeused.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GetVirtualDiskMetadata<P0>(virtualdiskhandle: P0, item: *const windows_core::GUID, metadatasize: *mut u32, metadata: *mut core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn GetVirtualDiskMetadata(virtualdiskhandle : super::super::Foundation:: HANDLE, item : *const windows_core::GUID, metadatasize : *mut u32, metadata : *mut core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    GetVirtualDiskMetadata(virtualdiskhandle.param().abi(), item, metadatasize, metadata)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn GetVirtualDiskOperationProgress<P0>(virtualdiskhandle: P0, overlapped: *const super::super::System::IO::OVERLAPPED, progress: *mut VIRTUAL_DISK_PROGRESS) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn GetVirtualDiskOperationProgress(virtualdiskhandle : super::super::Foundation:: HANDLE, overlapped : *const super::super::System::IO:: OVERLAPPED, progress : *mut VIRTUAL_DISK_PROGRESS) -> super::super::Foundation:: WIN32_ERROR);
    GetVirtualDiskOperationProgress(virtualdiskhandle.param().abi(), overlapped, progress)
}
#[inline]
pub unsafe fn GetVirtualDiskPhysicalPath<P0>(virtualdiskhandle: P0, diskpathsizeinbytes: *mut u32, diskpath: windows_core::PWSTR) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn GetVirtualDiskPhysicalPath(virtualdiskhandle : super::super::Foundation:: HANDLE, diskpathsizeinbytes : *mut u32, diskpath : windows_core::PWSTR) -> super::super::Foundation:: WIN32_ERROR);
    GetVirtualDiskPhysicalPath(virtualdiskhandle.param().abi(), diskpathsizeinbytes, core::mem::transmute(diskpath))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn MergeVirtualDisk<P0>(virtualdiskhandle: P0, flags: MERGE_VIRTUAL_DISK_FLAG, parameters: *const MERGE_VIRTUAL_DISK_PARAMETERS, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn MergeVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, flags : MERGE_VIRTUAL_DISK_FLAG, parameters : *const MERGE_VIRTUAL_DISK_PARAMETERS, overlapped : *const super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: WIN32_ERROR);
    MergeVirtualDisk(virtualdiskhandle.param().abi(), flags, parameters, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn MirrorVirtualDisk<P0>(virtualdiskhandle: P0, flags: MIRROR_VIRTUAL_DISK_FLAG, parameters: *const MIRROR_VIRTUAL_DISK_PARAMETERS, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn MirrorVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, flags : MIRROR_VIRTUAL_DISK_FLAG, parameters : *const MIRROR_VIRTUAL_DISK_PARAMETERS, overlapped : *const super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: WIN32_ERROR);
    MirrorVirtualDisk(virtualdiskhandle.param().abi(), flags, parameters, overlapped)
}
#[inline]
pub unsafe fn ModifyVhdSet<P0>(virtualdiskhandle: P0, parameters: *const MODIFY_VHDSET_PARAMETERS, flags: MODIFY_VHDSET_FLAG) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn ModifyVhdSet(virtualdiskhandle : super::super::Foundation:: HANDLE, parameters : *const MODIFY_VHDSET_PARAMETERS, flags : MODIFY_VHDSET_FLAG) -> super::super::Foundation:: WIN32_ERROR);
    ModifyVhdSet(virtualdiskhandle.param().abi(), parameters, flags)
}
#[inline]
pub unsafe fn OpenVirtualDisk<P0>(virtualstoragetype: *const VIRTUAL_STORAGE_TYPE, path: P0, virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK, flags: OPEN_VIRTUAL_DISK_FLAG, parameters: Option<*const OPEN_VIRTUAL_DISK_PARAMETERS>, handle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("virtdisk.dll" "system" fn OpenVirtualDisk(virtualstoragetype : *const VIRTUAL_STORAGE_TYPE, path : windows_core::PCWSTR, virtualdiskaccessmask : VIRTUAL_DISK_ACCESS_MASK, flags : OPEN_VIRTUAL_DISK_FLAG, parameters : *const OPEN_VIRTUAL_DISK_PARAMETERS, handle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: WIN32_ERROR);
    OpenVirtualDisk(virtualstoragetype, path.param().abi(), virtualdiskaccessmask, flags, core::mem::transmute(parameters.unwrap_or(std::ptr::null())), handle)
}
#[inline]
pub unsafe fn QueryChangesVirtualDisk<P0, P1>(virtualdiskhandle: P0, changetrackingid: P1, byteoffset: u64, bytelength: u64, flags: QUERY_CHANGES_VIRTUAL_DISK_FLAG, ranges: *mut QUERY_CHANGES_VIRTUAL_DISK_RANGE, rangecount: *mut u32, processedlength: *mut u64) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("virtdisk.dll" "system" fn QueryChangesVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, changetrackingid : windows_core::PCWSTR, byteoffset : u64, bytelength : u64, flags : QUERY_CHANGES_VIRTUAL_DISK_FLAG, ranges : *mut QUERY_CHANGES_VIRTUAL_DISK_RANGE, rangecount : *mut u32, processedlength : *mut u64) -> super::super::Foundation:: WIN32_ERROR);
    QueryChangesVirtualDisk(virtualdiskhandle.param().abi(), changetrackingid.param().abi(), byteoffset, bytelength, flags, ranges, rangecount, processedlength)
}
#[inline]
pub unsafe fn RawSCSIVirtualDisk<P0>(virtualdiskhandle: P0, parameters: *const RAW_SCSI_VIRTUAL_DISK_PARAMETERS, flags: RAW_SCSI_VIRTUAL_DISK_FLAG, response: *mut RAW_SCSI_VIRTUAL_DISK_RESPONSE) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn RawSCSIVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, parameters : *const RAW_SCSI_VIRTUAL_DISK_PARAMETERS, flags : RAW_SCSI_VIRTUAL_DISK_FLAG, response : *mut RAW_SCSI_VIRTUAL_DISK_RESPONSE) -> super::super::Foundation:: WIN32_ERROR);
    RawSCSIVirtualDisk(virtualdiskhandle.param().abi(), parameters, flags, response)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ResizeVirtualDisk<P0>(virtualdiskhandle: P0, flags: RESIZE_VIRTUAL_DISK_FLAG, parameters: *const RESIZE_VIRTUAL_DISK_PARAMETERS, overlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn ResizeVirtualDisk(virtualdiskhandle : super::super::Foundation:: HANDLE, flags : RESIZE_VIRTUAL_DISK_FLAG, parameters : *const RESIZE_VIRTUAL_DISK_PARAMETERS, overlapped : *const super::super::System::IO:: OVERLAPPED) -> super::super::Foundation:: WIN32_ERROR);
    ResizeVirtualDisk(virtualdiskhandle.param().abi(), flags, parameters, core::mem::transmute(overlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn SetVirtualDiskInformation<P0>(virtualdiskhandle: P0, virtualdiskinfo: *const SET_VIRTUAL_DISK_INFO) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn SetVirtualDiskInformation(virtualdiskhandle : super::super::Foundation:: HANDLE, virtualdiskinfo : *const SET_VIRTUAL_DISK_INFO) -> super::super::Foundation:: WIN32_ERROR);
    SetVirtualDiskInformation(virtualdiskhandle.param().abi(), virtualdiskinfo)
}
#[inline]
pub unsafe fn SetVirtualDiskMetadata<P0>(virtualdiskhandle: P0, item: *const windows_core::GUID, metadatasize: u32, metadata: *const core::ffi::c_void) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn SetVirtualDiskMetadata(virtualdiskhandle : super::super::Foundation:: HANDLE, item : *const windows_core::GUID, metadatasize : u32, metadata : *const core::ffi::c_void) -> super::super::Foundation:: WIN32_ERROR);
    SetVirtualDiskMetadata(virtualdiskhandle.param().abi(), item, metadatasize, metadata)
}
#[inline]
pub unsafe fn TakeSnapshotVhdSet<P0>(virtualdiskhandle: P0, parameters: *const TAKE_SNAPSHOT_VHDSET_PARAMETERS, flags: TAKE_SNAPSHOT_VHDSET_FLAG) -> super::super::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("virtdisk.dll" "system" fn TakeSnapshotVhdSet(virtualdiskhandle : super::super::Foundation:: HANDLE, parameters : *const TAKE_SNAPSHOT_VHDSET_PARAMETERS, flags : TAKE_SNAPSHOT_VHDSET_FLAG) -> super::super::Foundation:: WIN32_ERROR);
    TakeSnapshotVhdSet(virtualdiskhandle.param().abi(), parameters, flags)
}
pub const APPLY_SNAPSHOT_VHDSET_FLAG_NONE: APPLY_SNAPSHOT_VHDSET_FLAG = APPLY_SNAPSHOT_VHDSET_FLAG(0i32);
pub const APPLY_SNAPSHOT_VHDSET_FLAG_WRITEABLE: APPLY_SNAPSHOT_VHDSET_FLAG = APPLY_SNAPSHOT_VHDSET_FLAG(1i32);
pub const APPLY_SNAPSHOT_VHDSET_VERSION_1: APPLY_SNAPSHOT_VHDSET_VERSION = APPLY_SNAPSHOT_VHDSET_VERSION(1i32);
pub const APPLY_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: APPLY_SNAPSHOT_VHDSET_VERSION = APPLY_SNAPSHOT_VHDSET_VERSION(0i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_AT_BOOT: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(1024i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_BYPASS_DEFAULT_ENCRYPTION_POLICY: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(32i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NONE: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(0i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NON_PNP: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(64i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_DRIVE_LETTER: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(2i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_LOCAL_HOST: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(8i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_SECURITY_DESCRIPTOR: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(16i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(4i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(1i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_REGISTER_VOLUME: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(512i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_RESTRICTED_RANGE: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(128i32);
pub const ATTACH_VIRTUAL_DISK_FLAG_SINGLE_PARTITION: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(256i32);
pub const ATTACH_VIRTUAL_DISK_VERSION_1: ATTACH_VIRTUAL_DISK_VERSION = ATTACH_VIRTUAL_DISK_VERSION(1i32);
pub const ATTACH_VIRTUAL_DISK_VERSION_2: ATTACH_VIRTUAL_DISK_VERSION = ATTACH_VIRTUAL_DISK_VERSION(2i32);
pub const ATTACH_VIRTUAL_DISK_VERSION_UNSPECIFIED: ATTACH_VIRTUAL_DISK_VERSION = ATTACH_VIRTUAL_DISK_VERSION(0i32);
pub const COMPACT_VIRTUAL_DISK_FLAG_NONE: COMPACT_VIRTUAL_DISK_FLAG = COMPACT_VIRTUAL_DISK_FLAG(0i32);
pub const COMPACT_VIRTUAL_DISK_FLAG_NO_BLOCK_MOVES: COMPACT_VIRTUAL_DISK_FLAG = COMPACT_VIRTUAL_DISK_FLAG(2i32);
pub const COMPACT_VIRTUAL_DISK_FLAG_NO_ZERO_SCAN: COMPACT_VIRTUAL_DISK_FLAG = COMPACT_VIRTUAL_DISK_FLAG(1i32);
pub const COMPACT_VIRTUAL_DISK_VERSION_1: COMPACT_VIRTUAL_DISK_VERSION = COMPACT_VIRTUAL_DISK_VERSION(1i32);
pub const COMPACT_VIRTUAL_DISK_VERSION_UNSPECIFIED: COMPACT_VIRTUAL_DISK_VERSION = COMPACT_VIRTUAL_DISK_VERSION(0i32);
pub const CREATE_VIRTUAL_DISK_FLAG_CREATE_BACKING_STORAGE: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(8i32);
pub const CREATE_VIRTUAL_DISK_FLAG_DO_NOT_COPY_METADATA_FROM_PARENT: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(4i32);
pub const CREATE_VIRTUAL_DISK_FLAG_FULL_PHYSICAL_ALLOCATION: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(1i32);
pub const CREATE_VIRTUAL_DISK_FLAG_NONE: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(0i32);
pub const CREATE_VIRTUAL_DISK_FLAG_PMEM_COMPATIBLE: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(256i32);
pub const CREATE_VIRTUAL_DISK_FLAG_PRESERVE_PARENT_CHANGE_TRACKING_STATE: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(32i32);
pub const CREATE_VIRTUAL_DISK_FLAG_PREVENT_WRITES_TO_SOURCE_DISK: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(2i32);
pub const CREATE_VIRTUAL_DISK_FLAG_SPARSE_FILE: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(128i32);
pub const CREATE_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(512i32);
pub const CREATE_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(1024i32);
pub const CREATE_VIRTUAL_DISK_FLAG_USE_CHANGE_TRACKING_SOURCE_LIMIT: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(16i32);
pub const CREATE_VIRTUAL_DISK_FLAG_VHD_SET_USE_ORIGINAL_BACKING_STORAGE: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(64i32);
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_BLOCK_SIZE: u32 = 0u32;
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_SECTOR_SIZE: u32 = 0u32;
pub const CREATE_VIRTUAL_DISK_VERSION_1: CREATE_VIRTUAL_DISK_VERSION = CREATE_VIRTUAL_DISK_VERSION(1i32);
pub const CREATE_VIRTUAL_DISK_VERSION_2: CREATE_VIRTUAL_DISK_VERSION = CREATE_VIRTUAL_DISK_VERSION(2i32);
pub const CREATE_VIRTUAL_DISK_VERSION_3: CREATE_VIRTUAL_DISK_VERSION = CREATE_VIRTUAL_DISK_VERSION(3i32);
pub const CREATE_VIRTUAL_DISK_VERSION_4: CREATE_VIRTUAL_DISK_VERSION = CREATE_VIRTUAL_DISK_VERSION(4i32);
pub const CREATE_VIRTUAL_DISK_VERSION_UNSPECIFIED: CREATE_VIRTUAL_DISK_VERSION = CREATE_VIRTUAL_DISK_VERSION(0i32);
pub const DELETE_SNAPSHOT_VHDSET_FLAG_NONE: DELETE_SNAPSHOT_VHDSET_FLAG = DELETE_SNAPSHOT_VHDSET_FLAG(0i32);
pub const DELETE_SNAPSHOT_VHDSET_FLAG_PERSIST_RCT: DELETE_SNAPSHOT_VHDSET_FLAG = DELETE_SNAPSHOT_VHDSET_FLAG(1i32);
pub const DELETE_SNAPSHOT_VHDSET_VERSION_1: DELETE_SNAPSHOT_VHDSET_VERSION = DELETE_SNAPSHOT_VHDSET_VERSION(1i32);
pub const DELETE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: DELETE_SNAPSHOT_VHDSET_VERSION = DELETE_SNAPSHOT_VHDSET_VERSION(0i32);
pub const DEPENDENT_DISK_FLAG_ALWAYS_ALLOW_SPARSE: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(4096i32);
pub const DEPENDENT_DISK_FLAG_FULLY_ALLOCATED: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(2i32);
pub const DEPENDENT_DISK_FLAG_MULT_BACKING_FILES: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(1i32);
pub const DEPENDENT_DISK_FLAG_NONE: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(0i32);
pub const DEPENDENT_DISK_FLAG_NO_DRIVE_LETTER: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(128i32);
pub const DEPENDENT_DISK_FLAG_NO_HOST_DISK: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(512i32);
pub const DEPENDENT_DISK_FLAG_PARENT: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(256i32);
pub const DEPENDENT_DISK_FLAG_PERMANENT_LIFETIME: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(1024i32);
pub const DEPENDENT_DISK_FLAG_READ_ONLY: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(4i32);
pub const DEPENDENT_DISK_FLAG_REMOTE: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(8i32);
pub const DEPENDENT_DISK_FLAG_REMOVABLE: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(64i32);
pub const DEPENDENT_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(2048i32);
pub const DEPENDENT_DISK_FLAG_SUPPORT_ENCRYPTED_FILES: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(8192i32);
pub const DEPENDENT_DISK_FLAG_SYSTEM_VOLUME: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(16i32);
pub const DEPENDENT_DISK_FLAG_SYSTEM_VOLUME_PARENT: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(32i32);
pub const DETACH_VIRTUAL_DISK_FLAG_NONE: DETACH_VIRTUAL_DISK_FLAG = DETACH_VIRTUAL_DISK_FLAG(0i32);
pub const EXPAND_VIRTUAL_DISK_FLAG_NONE: EXPAND_VIRTUAL_DISK_FLAG = EXPAND_VIRTUAL_DISK_FLAG(0i32);
pub const EXPAND_VIRTUAL_DISK_FLAG_NOTIFY_CHANGE: EXPAND_VIRTUAL_DISK_FLAG = EXPAND_VIRTUAL_DISK_FLAG(1i32);
pub const EXPAND_VIRTUAL_DISK_VERSION_1: EXPAND_VIRTUAL_DISK_VERSION = EXPAND_VIRTUAL_DISK_VERSION(1i32);
pub const EXPAND_VIRTUAL_DISK_VERSION_UNSPECIFIED: EXPAND_VIRTUAL_DISK_VERSION = EXPAND_VIRTUAL_DISK_VERSION(0i32);
pub const FORK_VIRTUAL_DISK_FLAG_EXISTING_FILE: FORK_VIRTUAL_DISK_FLAG = FORK_VIRTUAL_DISK_FLAG(1i32);
pub const FORK_VIRTUAL_DISK_FLAG_NONE: FORK_VIRTUAL_DISK_FLAG = FORK_VIRTUAL_DISK_FLAG(0i32);
pub const FORK_VIRTUAL_DISK_VERSION_1: FORK_VIRTUAL_DISK_VERSION = FORK_VIRTUAL_DISK_VERSION(1i32);
pub const FORK_VIRTUAL_DISK_VERSION_UNSPECIFIED: FORK_VIRTUAL_DISK_VERSION = FORK_VIRTUAL_DISK_VERSION(0i32);
pub const GET_STORAGE_DEPENDENCY_FLAG_DISK_HANDLE: GET_STORAGE_DEPENDENCY_FLAG = GET_STORAGE_DEPENDENCY_FLAG(2i32);
pub const GET_STORAGE_DEPENDENCY_FLAG_HOST_VOLUMES: GET_STORAGE_DEPENDENCY_FLAG = GET_STORAGE_DEPENDENCY_FLAG(1i32);
pub const GET_STORAGE_DEPENDENCY_FLAG_NONE: GET_STORAGE_DEPENDENCY_FLAG = GET_STORAGE_DEPENDENCY_FLAG(0i32);
pub const GET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(15i32);
pub const GET_VIRTUAL_DISK_INFO_FRAGMENTATION: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(12i32);
pub const GET_VIRTUAL_DISK_INFO_IDENTIFIER: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(2i32);
pub const GET_VIRTUAL_DISK_INFO_IS_4K_ALIGNED: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(8i32);
pub const GET_VIRTUAL_DISK_INFO_IS_LOADED: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(13i32);
pub const GET_VIRTUAL_DISK_INFO_PARENT_IDENTIFIER: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(4i32);
pub const GET_VIRTUAL_DISK_INFO_PARENT_LOCATION: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(3i32);
pub const GET_VIRTUAL_DISK_INFO_PARENT_TIMESTAMP: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(5i32);
pub const GET_VIRTUAL_DISK_INFO_PHYSICAL_DISK: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(9i32);
pub const GET_VIRTUAL_DISK_INFO_PROVIDER_SUBTYPE: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(7i32);
pub const GET_VIRTUAL_DISK_INFO_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(1i32);
pub const GET_VIRTUAL_DISK_INFO_SMALLEST_SAFE_VIRTUAL_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(11i32);
pub const GET_VIRTUAL_DISK_INFO_UNSPECIFIED: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(0i32);
pub const GET_VIRTUAL_DISK_INFO_VHD_PHYSICAL_SECTOR_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(10i32);
pub const GET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(14i32);
pub const GET_VIRTUAL_DISK_INFO_VIRTUAL_STORAGE_TYPE: GET_VIRTUAL_DISK_INFO_VERSION = GET_VIRTUAL_DISK_INFO_VERSION(6i32);
pub const MERGE_VIRTUAL_DISK_DEFAULT_MERGE_DEPTH: u32 = 1u32;
pub const MERGE_VIRTUAL_DISK_FLAG_NONE: MERGE_VIRTUAL_DISK_FLAG = MERGE_VIRTUAL_DISK_FLAG(0i32);
pub const MERGE_VIRTUAL_DISK_VERSION_1: MERGE_VIRTUAL_DISK_VERSION = MERGE_VIRTUAL_DISK_VERSION(1i32);
pub const MERGE_VIRTUAL_DISK_VERSION_2: MERGE_VIRTUAL_DISK_VERSION = MERGE_VIRTUAL_DISK_VERSION(2i32);
pub const MERGE_VIRTUAL_DISK_VERSION_UNSPECIFIED: MERGE_VIRTUAL_DISK_VERSION = MERGE_VIRTUAL_DISK_VERSION(0i32);
pub const MIRROR_VIRTUAL_DISK_FLAG_ENABLE_SMB_COMPRESSION: MIRROR_VIRTUAL_DISK_FLAG = MIRROR_VIRTUAL_DISK_FLAG(4i32);
pub const MIRROR_VIRTUAL_DISK_FLAG_EXISTING_FILE: MIRROR_VIRTUAL_DISK_FLAG = MIRROR_VIRTUAL_DISK_FLAG(1i32);
pub const MIRROR_VIRTUAL_DISK_FLAG_IS_LIVE_MIGRATION: MIRROR_VIRTUAL_DISK_FLAG = MIRROR_VIRTUAL_DISK_FLAG(8i32);
pub const MIRROR_VIRTUAL_DISK_FLAG_NONE: MIRROR_VIRTUAL_DISK_FLAG = MIRROR_VIRTUAL_DISK_FLAG(0i32);
pub const MIRROR_VIRTUAL_DISK_FLAG_SKIP_MIRROR_ACTIVATION: MIRROR_VIRTUAL_DISK_FLAG = MIRROR_VIRTUAL_DISK_FLAG(2i32);
pub const MIRROR_VIRTUAL_DISK_VERSION_1: MIRROR_VIRTUAL_DISK_VERSION = MIRROR_VIRTUAL_DISK_VERSION(1i32);
pub const MIRROR_VIRTUAL_DISK_VERSION_UNSPECIFIED: MIRROR_VIRTUAL_DISK_VERSION = MIRROR_VIRTUAL_DISK_VERSION(0i32);
pub const MODIFY_VHDSET_DEFAULT_SNAPSHOT_PATH: MODIFY_VHDSET_VERSION = MODIFY_VHDSET_VERSION(3i32);
pub const MODIFY_VHDSET_FLAG_NONE: MODIFY_VHDSET_FLAG = MODIFY_VHDSET_FLAG(0i32);
pub const MODIFY_VHDSET_FLAG_WRITEABLE_SNAPSHOT: MODIFY_VHDSET_FLAG = MODIFY_VHDSET_FLAG(1i32);
pub const MODIFY_VHDSET_REMOVE_SNAPSHOT: MODIFY_VHDSET_VERSION = MODIFY_VHDSET_VERSION(2i32);
pub const MODIFY_VHDSET_SNAPSHOT_PATH: MODIFY_VHDSET_VERSION = MODIFY_VHDSET_VERSION(1i32);
pub const MODIFY_VHDSET_UNSPECIFIED: MODIFY_VHDSET_VERSION = MODIFY_VHDSET_VERSION(0i32);
pub const OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(2i32);
pub const OPEN_VIRTUAL_DISK_FLAG_BOOT_DRIVE: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(4i32);
pub const OPEN_VIRTUAL_DISK_FLAG_CACHED_IO: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(8i32);
pub const OPEN_VIRTUAL_DISK_FLAG_CUSTOM_DIFF_CHAIN: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(16i32);
pub const OPEN_VIRTUAL_DISK_FLAG_IGNORE_RELATIVE_PARENT_LOCATOR: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(128i32);
pub const OPEN_VIRTUAL_DISK_FLAG_NONE: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(0i32);
pub const OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(1i32);
pub const OPEN_VIRTUAL_DISK_FLAG_NO_WRITE_HARDENING: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(256i32);
pub const OPEN_VIRTUAL_DISK_FLAG_PARENT_CACHED_IO: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(32i32);
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(512i32);
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_ENCRYPTED_FILES: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(2048i32);
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(1024i32);
pub const OPEN_VIRTUAL_DISK_FLAG_VHDSET_FILE_ONLY: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(64i32);
pub const OPEN_VIRTUAL_DISK_RW_DEPTH_DEFAULT: u32 = 1u32;
pub const OPEN_VIRTUAL_DISK_VERSION_1: OPEN_VIRTUAL_DISK_VERSION = OPEN_VIRTUAL_DISK_VERSION(1i32);
pub const OPEN_VIRTUAL_DISK_VERSION_2: OPEN_VIRTUAL_DISK_VERSION = OPEN_VIRTUAL_DISK_VERSION(2i32);
pub const OPEN_VIRTUAL_DISK_VERSION_3: OPEN_VIRTUAL_DISK_VERSION = OPEN_VIRTUAL_DISK_VERSION(3i32);
pub const OPEN_VIRTUAL_DISK_VERSION_UNSPECIFIED: OPEN_VIRTUAL_DISK_VERSION = OPEN_VIRTUAL_DISK_VERSION(0i32);
pub const QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE: QUERY_CHANGES_VIRTUAL_DISK_FLAG = QUERY_CHANGES_VIRTUAL_DISK_FLAG(0i32);
pub const RAW_SCSI_VIRTUAL_DISK_FLAG_NONE: RAW_SCSI_VIRTUAL_DISK_FLAG = RAW_SCSI_VIRTUAL_DISK_FLAG(0i32);
pub const RAW_SCSI_VIRTUAL_DISK_VERSION_1: RAW_SCSI_VIRTUAL_DISK_VERSION = RAW_SCSI_VIRTUAL_DISK_VERSION(1i32);
pub const RAW_SCSI_VIRTUAL_DISK_VERSION_UNSPECIFIED: RAW_SCSI_VIRTUAL_DISK_VERSION = RAW_SCSI_VIRTUAL_DISK_VERSION(0i32);
pub const RESIZE_VIRTUAL_DISK_FLAG_ALLOW_UNSAFE_VIRTUAL_SIZE: RESIZE_VIRTUAL_DISK_FLAG = RESIZE_VIRTUAL_DISK_FLAG(1i32);
pub const RESIZE_VIRTUAL_DISK_FLAG_NONE: RESIZE_VIRTUAL_DISK_FLAG = RESIZE_VIRTUAL_DISK_FLAG(0i32);
pub const RESIZE_VIRTUAL_DISK_FLAG_RESIZE_TO_SMALLEST_SAFE_VIRTUAL_SIZE: RESIZE_VIRTUAL_DISK_FLAG = RESIZE_VIRTUAL_DISK_FLAG(2i32);
pub const RESIZE_VIRTUAL_DISK_VERSION_1: RESIZE_VIRTUAL_DISK_VERSION = RESIZE_VIRTUAL_DISK_VERSION(1i32);
pub const RESIZE_VIRTUAL_DISK_VERSION_UNSPECIFIED: RESIZE_VIRTUAL_DISK_VERSION = RESIZE_VIRTUAL_DISK_VERSION(0i32);
pub const SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE: SET_VIRTUAL_DISK_INFO_VERSION = SET_VIRTUAL_DISK_INFO_VERSION(6i32);
pub const SET_VIRTUAL_DISK_INFO_IDENTIFIER: SET_VIRTUAL_DISK_INFO_VERSION = SET_VIRTUAL_DISK_INFO_VERSION(2i32);
pub const SET_VIRTUAL_DISK_INFO_PARENT_LOCATOR: SET_VIRTUAL_DISK_INFO_VERSION = SET_VIRTUAL_DISK_INFO_VERSION(7i32);
pub const SET_VIRTUAL_DISK_INFO_PARENT_PATH: SET_VIRTUAL_DISK_INFO_VERSION = SET_VIRTUAL_DISK_INFO_VERSION(1i32);
pub const SET_VIRTUAL_DISK_INFO_PARENT_PATH_WITH_DEPTH: SET_VIRTUAL_DISK_INFO_VERSION = SET_VIRTUAL_DISK_INFO_VERSION(3i32);
pub const SET_VIRTUAL_DISK_INFO_PHYSICAL_SECTOR_SIZE: SET_VIRTUAL_DISK_INFO_VERSION = SET_VIRTUAL_DISK_INFO_VERSION(4i32);
pub const SET_VIRTUAL_DISK_INFO_UNSPECIFIED: SET_VIRTUAL_DISK_INFO_VERSION = SET_VIRTUAL_DISK_INFO_VERSION(0i32);
pub const SET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID: SET_VIRTUAL_DISK_INFO_VERSION = SET_VIRTUAL_DISK_INFO_VERSION(5i32);
pub const STORAGE_DEPENDENCY_INFO_VERSION_1: STORAGE_DEPENDENCY_INFO_VERSION = STORAGE_DEPENDENCY_INFO_VERSION(1i32);
pub const STORAGE_DEPENDENCY_INFO_VERSION_2: STORAGE_DEPENDENCY_INFO_VERSION = STORAGE_DEPENDENCY_INFO_VERSION(2i32);
pub const STORAGE_DEPENDENCY_INFO_VERSION_UNSPECIFIED: STORAGE_DEPENDENCY_INFO_VERSION = STORAGE_DEPENDENCY_INFO_VERSION(0i32);
pub const TAKE_SNAPSHOT_VHDSET_FLAG_NONE: TAKE_SNAPSHOT_VHDSET_FLAG = TAKE_SNAPSHOT_VHDSET_FLAG(0i32);
pub const TAKE_SNAPSHOT_VHDSET_FLAG_WRITEABLE: TAKE_SNAPSHOT_VHDSET_FLAG = TAKE_SNAPSHOT_VHDSET_FLAG(1i32);
pub const TAKE_SNAPSHOT_VHDSET_VERSION_1: TAKE_SNAPSHOT_VHDSET_VERSION = TAKE_SNAPSHOT_VHDSET_VERSION(1i32);
pub const TAKE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: TAKE_SNAPSHOT_VHDSET_VERSION = TAKE_SNAPSHOT_VHDSET_VERSION(0i32);
pub const VIRTUAL_DISK_ACCESS_ALL: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(4128768i32);
pub const VIRTUAL_DISK_ACCESS_ATTACH_RO: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(65536i32);
pub const VIRTUAL_DISK_ACCESS_ATTACH_RW: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(131072i32);
pub const VIRTUAL_DISK_ACCESS_CREATE: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(1048576i32);
pub const VIRTUAL_DISK_ACCESS_DETACH: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(262144i32);
pub const VIRTUAL_DISK_ACCESS_GET_INFO: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(524288i32);
pub const VIRTUAL_DISK_ACCESS_METAOPS: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(2097152i32);
pub const VIRTUAL_DISK_ACCESS_NONE: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(0i32);
pub const VIRTUAL_DISK_ACCESS_READ: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(851968i32);
pub const VIRTUAL_DISK_ACCESS_WRITABLE: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(3276800i32);
pub const VIRTUAL_DISK_MAXIMUM_CHANGE_TRACKING_ID_LENGTH: u32 = 256u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_ISO: u32 = 1u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_UNKNOWN: u32 = 0u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHD: u32 = 2u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDSET: u32 = 4u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDX: u32 = 3u32;
pub const VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT: windows_core::GUID = windows_core::GUID::from_u128(0xec984aec_a0f9_47e9_901f_71415a66345b);
pub const VIRTUAL_STORAGE_TYPE_VENDOR_UNKNOWN: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPLY_SNAPSHOT_VHDSET_FLAG(pub i32);
impl windows_core::TypeKind for APPLY_SNAPSHOT_VHDSET_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPLY_SNAPSHOT_VHDSET_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPLY_SNAPSHOT_VHDSET_FLAG").field(&self.0).finish()
    }
}
impl APPLY_SNAPSHOT_VHDSET_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for APPLY_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for APPLY_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for APPLY_SNAPSHOT_VHDSET_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for APPLY_SNAPSHOT_VHDSET_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for APPLY_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APPLY_SNAPSHOT_VHDSET_VERSION(pub i32);
impl windows_core::TypeKind for APPLY_SNAPSHOT_VHDSET_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APPLY_SNAPSHOT_VHDSET_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APPLY_SNAPSHOT_VHDSET_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ATTACH_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for ATTACH_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ATTACH_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ATTACH_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl ATTACH_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ATTACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ATTACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ATTACH_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ATTACH_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ATTACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ATTACH_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for ATTACH_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ATTACH_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ATTACH_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPACT_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for COMPACT_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPACT_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPACT_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl COMPACT_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for COMPACT_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for COMPACT_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for COMPACT_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for COMPACT_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for COMPACT_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPACT_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for COMPACT_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPACT_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPACT_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CREATE_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for CREATE_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CREATE_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CREATE_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl CREATE_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CREATE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CREATE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CREATE_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CREATE_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CREATE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CREATE_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for CREATE_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CREATE_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CREATE_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DELETE_SNAPSHOT_VHDSET_FLAG(pub i32);
impl windows_core::TypeKind for DELETE_SNAPSHOT_VHDSET_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DELETE_SNAPSHOT_VHDSET_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DELETE_SNAPSHOT_VHDSET_FLAG").field(&self.0).finish()
    }
}
impl DELETE_SNAPSHOT_VHDSET_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DELETE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DELETE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DELETE_SNAPSHOT_VHDSET_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DELETE_SNAPSHOT_VHDSET_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DELETE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DELETE_SNAPSHOT_VHDSET_VERSION(pub i32);
impl windows_core::TypeKind for DELETE_SNAPSHOT_VHDSET_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DELETE_SNAPSHOT_VHDSET_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DELETE_SNAPSHOT_VHDSET_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEPENDENT_DISK_FLAG(pub i32);
impl windows_core::TypeKind for DEPENDENT_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEPENDENT_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEPENDENT_DISK_FLAG").field(&self.0).finish()
    }
}
impl DEPENDENT_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DEPENDENT_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DEPENDENT_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DEPENDENT_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DEPENDENT_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DEPENDENT_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DETACH_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for DETACH_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DETACH_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DETACH_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl DETACH_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DETACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DETACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DETACH_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DETACH_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DETACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EXPAND_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for EXPAND_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EXPAND_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EXPAND_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl EXPAND_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for EXPAND_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for EXPAND_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for EXPAND_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for EXPAND_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for EXPAND_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EXPAND_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for EXPAND_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EXPAND_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EXPAND_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FORK_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for FORK_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FORK_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FORK_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl FORK_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FORK_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FORK_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FORK_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FORK_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FORK_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FORK_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for FORK_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FORK_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FORK_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_STORAGE_DEPENDENCY_FLAG(pub i32);
impl windows_core::TypeKind for GET_STORAGE_DEPENDENCY_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_STORAGE_DEPENDENCY_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_STORAGE_DEPENDENCY_FLAG").field(&self.0).finish()
    }
}
impl GET_STORAGE_DEPENDENCY_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GET_STORAGE_DEPENDENCY_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GET_STORAGE_DEPENDENCY_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GET_STORAGE_DEPENDENCY_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GET_STORAGE_DEPENDENCY_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GET_STORAGE_DEPENDENCY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_VIRTUAL_DISK_INFO_VERSION(pub i32);
impl windows_core::TypeKind for GET_VIRTUAL_DISK_INFO_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_VIRTUAL_DISK_INFO_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_VIRTUAL_DISK_INFO_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MERGE_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for MERGE_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MERGE_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MERGE_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl MERGE_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MERGE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MERGE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MERGE_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MERGE_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MERGE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MERGE_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for MERGE_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MERGE_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MERGE_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MIRROR_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for MIRROR_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MIRROR_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MIRROR_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl MIRROR_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MIRROR_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MIRROR_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MIRROR_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MIRROR_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MIRROR_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MIRROR_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for MIRROR_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MIRROR_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MIRROR_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MODIFY_VHDSET_FLAG(pub i32);
impl windows_core::TypeKind for MODIFY_VHDSET_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MODIFY_VHDSET_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MODIFY_VHDSET_FLAG").field(&self.0).finish()
    }
}
impl MODIFY_VHDSET_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MODIFY_VHDSET_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MODIFY_VHDSET_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MODIFY_VHDSET_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MODIFY_VHDSET_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MODIFY_VHDSET_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MODIFY_VHDSET_VERSION(pub i32);
impl windows_core::TypeKind for MODIFY_VHDSET_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MODIFY_VHDSET_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MODIFY_VHDSET_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPEN_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for OPEN_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPEN_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPEN_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl OPEN_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OPEN_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OPEN_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OPEN_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OPEN_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OPEN_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OPEN_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for OPEN_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OPEN_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OPEN_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QUERY_CHANGES_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QUERY_CHANGES_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAW_SCSI_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for RAW_SCSI_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAW_SCSI_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAW_SCSI_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl RAW_SCSI_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RAW_SCSI_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RAW_SCSI_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RAW_SCSI_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RAW_SCSI_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RAW_SCSI_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAW_SCSI_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for RAW_SCSI_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAW_SCSI_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAW_SCSI_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RESIZE_VIRTUAL_DISK_FLAG(pub i32);
impl windows_core::TypeKind for RESIZE_VIRTUAL_DISK_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RESIZE_VIRTUAL_DISK_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RESIZE_VIRTUAL_DISK_FLAG").field(&self.0).finish()
    }
}
impl RESIZE_VIRTUAL_DISK_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RESIZE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RESIZE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RESIZE_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RESIZE_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RESIZE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RESIZE_VIRTUAL_DISK_VERSION(pub i32);
impl windows_core::TypeKind for RESIZE_VIRTUAL_DISK_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RESIZE_VIRTUAL_DISK_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RESIZE_VIRTUAL_DISK_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SET_VIRTUAL_DISK_INFO_VERSION(pub i32);
impl windows_core::TypeKind for SET_VIRTUAL_DISK_INFO_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SET_VIRTUAL_DISK_INFO_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SET_VIRTUAL_DISK_INFO_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STORAGE_DEPENDENCY_INFO_VERSION(pub i32);
impl windows_core::TypeKind for STORAGE_DEPENDENCY_INFO_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STORAGE_DEPENDENCY_INFO_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STORAGE_DEPENDENCY_INFO_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAKE_SNAPSHOT_VHDSET_FLAG(pub i32);
impl windows_core::TypeKind for TAKE_SNAPSHOT_VHDSET_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAKE_SNAPSHOT_VHDSET_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAKE_SNAPSHOT_VHDSET_FLAG").field(&self.0).finish()
    }
}
impl TAKE_SNAPSHOT_VHDSET_FLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TAKE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TAKE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TAKE_SNAPSHOT_VHDSET_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TAKE_SNAPSHOT_VHDSET_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TAKE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TAKE_SNAPSHOT_VHDSET_VERSION(pub i32);
impl windows_core::TypeKind for TAKE_SNAPSHOT_VHDSET_VERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TAKE_SNAPSHOT_VHDSET_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TAKE_SNAPSHOT_VHDSET_VERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VIRTUAL_DISK_ACCESS_MASK(pub i32);
impl windows_core::TypeKind for VIRTUAL_DISK_ACCESS_MASK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VIRTUAL_DISK_ACCESS_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VIRTUAL_DISK_ACCESS_MASK").field(&self.0).finish()
    }
}
impl VIRTUAL_DISK_ACCESS_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VIRTUAL_DISK_ACCESS_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VIRTUAL_DISK_ACCESS_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VIRTUAL_DISK_ACCESS_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VIRTUAL_DISK_ACCESS_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VIRTUAL_DISK_ACCESS_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: APPLY_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: APPLY_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl windows_core::TypeKind for APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl windows_core::TypeKind for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: windows_core::GUID,
    pub LeafSnapshotId: windows_core::GUID,
}
impl windows_core::TypeKind for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS {
    pub Version: ATTACH_VIRTUAL_DISK_VERSION,
    pub Anonymous: ATTACH_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for ATTACH_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ATTACH_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: ATTACH_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: ATTACH_VIRTUAL_DISK_PARAMETERS_0_1,
}
impl windows_core::TypeKind for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub Reserved: u32,
}
impl windows_core::TypeKind for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub RestrictedOffset: u64,
    pub RestrictedLength: u64,
}
impl windows_core::TypeKind for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS {
    pub Version: COMPACT_VIRTUAL_DISK_VERSION,
    pub Anonymous: COMPACT_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for COMPACT_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPACT_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: COMPACT_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl windows_core::TypeKind for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub Reserved: u32,
}
impl windows_core::TypeKind for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS {
    pub Version: CREATE_VIRTUAL_DISK_VERSION,
    pub Anonymous: CREATE_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for CREATE_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: CREATE_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: CREATE_VIRTUAL_DISK_PARAMETERS_0_1,
    pub Version3: CREATE_VIRTUAL_DISK_PARAMETERS_0_2,
    pub Version4: CREATE_VIRTUAL_DISK_PARAMETERS_0_3,
}
impl windows_core::TypeKind for CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub ParentPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
}
impl windows_core::TypeKind for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: windows_core::GUID,
}
impl windows_core::TypeKind for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: windows_core::GUID,
    pub SourceLimitPath: windows_core::PCWSTR,
    pub BackingStorageType: VIRTUAL_STORAGE_TYPE,
}
impl windows_core::TypeKind for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: windows_core::GUID,
    pub SourceLimitPath: windows_core::PCWSTR,
    pub BackingStorageType: VIRTUAL_STORAGE_TYPE,
    pub PmemAddressAbstractionType: windows_core::GUID,
    pub DataAlignment: u64,
}
impl windows_core::TypeKind for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: DELETE_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: DELETE_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl windows_core::TypeKind for DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl windows_core::TypeKind for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: windows_core::GUID,
}
impl windows_core::TypeKind for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS {
    pub Version: EXPAND_VIRTUAL_DISK_VERSION,
    pub Anonymous: EXPAND_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for EXPAND_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXPAND_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: EXPAND_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl windows_core::TypeKind for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub NewSize: u64,
}
impl windows_core::TypeKind for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FORK_VIRTUAL_DISK_PARAMETERS {
    pub Version: FORK_VIRTUAL_DISK_VERSION,
    pub Anonymous: FORK_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for FORK_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for FORK_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FORK_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: FORK_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl windows_core::TypeKind for FORK_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FORK_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub ForkedVirtualDiskPath: windows_core::PCWSTR,
}
impl windows_core::TypeKind for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GET_VIRTUAL_DISK_INFO {
    pub Version: GET_VIRTUAL_DISK_INFO_VERSION,
    pub Anonymous: GET_VIRTUAL_DISK_INFO_0,
}
impl windows_core::TypeKind for GET_VIRTUAL_DISK_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for GET_VIRTUAL_DISK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union GET_VIRTUAL_DISK_INFO_0 {
    pub Size: GET_VIRTUAL_DISK_INFO_0_3,
    pub Identifier: windows_core::GUID,
    pub ParentLocation: GET_VIRTUAL_DISK_INFO_0_1,
    pub ParentIdentifier: windows_core::GUID,
    pub ParentTimestamp: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ProviderSubtype: u32,
    pub Is4kAligned: super::super::Foundation::BOOL,
    pub IsLoaded: super::super::Foundation::BOOL,
    pub PhysicalDisk: GET_VIRTUAL_DISK_INFO_0_2,
    pub VhdPhysicalSectorSize: u32,
    pub SmallestSafeVirtualSize: u64,
    pub FragmentationPercentage: u32,
    pub VirtualDiskId: windows_core::GUID,
    pub ChangeTrackingState: GET_VIRTUAL_DISK_INFO_0_0,
}
impl windows_core::TypeKind for GET_VIRTUAL_DISK_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GET_VIRTUAL_DISK_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_VIRTUAL_DISK_INFO_0_0 {
    pub Enabled: super::super::Foundation::BOOL,
    pub NewerChanges: super::super::Foundation::BOOL,
    pub MostRecentId: [u16; 1],
}
impl windows_core::TypeKind for GET_VIRTUAL_DISK_INFO_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GET_VIRTUAL_DISK_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_VIRTUAL_DISK_INFO_0_1 {
    pub ParentResolved: super::super::Foundation::BOOL,
    pub ParentLocationBuffer: [u16; 1],
}
impl windows_core::TypeKind for GET_VIRTUAL_DISK_INFO_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GET_VIRTUAL_DISK_INFO_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_VIRTUAL_DISK_INFO_0_2 {
    pub LogicalSectorSize: u32,
    pub PhysicalSectorSize: u32,
    pub IsRemote: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for GET_VIRTUAL_DISK_INFO_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GET_VIRTUAL_DISK_INFO_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_VIRTUAL_DISK_INFO_0_3 {
    pub VirtualSize: u64,
    pub PhysicalSize: u64,
    pub BlockSize: u32,
    pub SectorSize: u32,
}
impl windows_core::TypeKind for GET_VIRTUAL_DISK_INFO_0_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GET_VIRTUAL_DISK_INFO_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS {
    pub Version: MERGE_VIRTUAL_DISK_VERSION,
    pub Anonymous: MERGE_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for MERGE_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MERGE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: MERGE_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: MERGE_VIRTUAL_DISK_PARAMETERS_0_1,
}
impl windows_core::TypeKind for MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub MergeDepth: u32,
}
impl windows_core::TypeKind for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub MergeSourceDepth: u32,
    pub MergeTargetDepth: u32,
}
impl windows_core::TypeKind for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS {
    pub Version: MIRROR_VIRTUAL_DISK_VERSION,
    pub Anonymous: MIRROR_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for MIRROR_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIRROR_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: MIRROR_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl windows_core::TypeKind for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub MirrorVirtualDiskPath: windows_core::PCWSTR,
}
impl windows_core::TypeKind for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MODIFY_VHDSET_PARAMETERS {
    pub Version: MODIFY_VHDSET_VERSION,
    pub Anonymous: MODIFY_VHDSET_PARAMETERS_0,
}
impl windows_core::TypeKind for MODIFY_VHDSET_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for MODIFY_VHDSET_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MODIFY_VHDSET_PARAMETERS_0 {
    pub SnapshotPath: MODIFY_VHDSET_PARAMETERS_0_0,
    pub SnapshotId: windows_core::GUID,
    pub DefaultFilePath: windows_core::PCWSTR,
}
impl windows_core::TypeKind for MODIFY_VHDSET_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MODIFY_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MODIFY_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: windows_core::GUID,
    pub SnapshotFilePath: windows_core::PCWSTR,
}
impl windows_core::TypeKind for MODIFY_VHDSET_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MODIFY_VHDSET_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS {
    pub Version: OPEN_VIRTUAL_DISK_VERSION,
    pub Anonymous: OPEN_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for OPEN_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPEN_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: OPEN_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: OPEN_VIRTUAL_DISK_PARAMETERS_0_1,
    pub Version3: OPEN_VIRTUAL_DISK_PARAMETERS_0_2,
}
impl windows_core::TypeKind for OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub RWDepth: u32,
}
impl windows_core::TypeKind for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub GetInfoOnly: super::super::Foundation::BOOL,
    pub ReadOnly: super::super::Foundation::BOOL,
    pub ResiliencyGuid: windows_core::GUID,
}
impl windows_core::TypeKind for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    pub GetInfoOnly: super::super::Foundation::BOOL,
    pub ReadOnly: super::super::Foundation::BOOL,
    pub ResiliencyGuid: windows_core::GUID,
    pub SnapshotId: windows_core::GUID,
}
impl windows_core::TypeKind for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    pub ByteOffset: u64,
    pub ByteLength: u64,
    pub Reserved: u64,
}
impl windows_core::TypeKind for QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    pub Version: RAW_SCSI_VIRTUAL_DISK_VERSION,
    pub Anonymous: RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl windows_core::TypeKind for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub RSVDHandle: super::super::Foundation::BOOL,
    pub DataIn: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub SrbFlags: u32,
    pub DataTransferLength: u32,
    pub DataBuffer: *mut core::ffi::c_void,
    pub SenseInfo: *mut u8,
    pub Cdb: *mut u8,
}
impl windows_core::TypeKind for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    pub Version: RAW_SCSI_VIRTUAL_DISK_VERSION,
    pub Anonymous: RAW_SCSI_VIRTUAL_DISK_RESPONSE_0,
}
impl windows_core::TypeKind for RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    pub Version1: RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0,
}
impl windows_core::TypeKind for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataTransferLength: u32,
}
impl windows_core::TypeKind for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS {
    pub Version: RESIZE_VIRTUAL_DISK_VERSION,
    pub Anonymous: RESIZE_VIRTUAL_DISK_PARAMETERS_0,
}
impl windows_core::TypeKind for RESIZE_VIRTUAL_DISK_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESIZE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: RESIZE_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl windows_core::TypeKind for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub NewSize: u64,
}
impl windows_core::TypeKind for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SET_VIRTUAL_DISK_INFO {
    pub Version: SET_VIRTUAL_DISK_INFO_VERSION,
    pub Anonymous: SET_VIRTUAL_DISK_INFO_0,
}
impl windows_core::TypeKind for SET_VIRTUAL_DISK_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SET_VIRTUAL_DISK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SET_VIRTUAL_DISK_INFO_0 {
    pub ParentFilePath: windows_core::PCWSTR,
    pub UniqueIdentifier: windows_core::GUID,
    pub ParentPathWithDepthInfo: SET_VIRTUAL_DISK_INFO_0_1,
    pub VhdPhysicalSectorSize: u32,
    pub VirtualDiskId: windows_core::GUID,
    pub ChangeTrackingEnabled: super::super::Foundation::BOOL,
    pub ParentLocator: SET_VIRTUAL_DISK_INFO_0_0,
}
impl windows_core::TypeKind for SET_VIRTUAL_DISK_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SET_VIRTUAL_DISK_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SET_VIRTUAL_DISK_INFO_0_0 {
    pub LinkageId: windows_core::GUID,
    pub ParentFilePath: windows_core::PCWSTR,
}
impl windows_core::TypeKind for SET_VIRTUAL_DISK_INFO_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SET_VIRTUAL_DISK_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SET_VIRTUAL_DISK_INFO_0_1 {
    pub ChildDepth: u32,
    pub ParentFilePath: windows_core::PCWSTR,
}
impl windows_core::TypeKind for SET_VIRTUAL_DISK_INFO_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SET_VIRTUAL_DISK_INFO_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STORAGE_DEPENDENCY_INFO {
    pub Version: STORAGE_DEPENDENCY_INFO_VERSION,
    pub NumberEntries: u32,
    pub Anonymous: STORAGE_DEPENDENCY_INFO_0,
}
impl windows_core::TypeKind for STORAGE_DEPENDENCY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_DEPENDENCY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STORAGE_DEPENDENCY_INFO_0 {
    pub Version1Entries: [STORAGE_DEPENDENCY_INFO_TYPE_1; 1],
    pub Version2Entries: [STORAGE_DEPENDENCY_INFO_TYPE_2; 1],
}
impl windows_core::TypeKind for STORAGE_DEPENDENCY_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_DEPENDENCY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DEPENDENCY_INFO_TYPE_1 {
    pub DependencyTypeFlags: DEPENDENT_DISK_FLAG,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
}
impl windows_core::TypeKind for STORAGE_DEPENDENCY_INFO_TYPE_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_DEPENDENCY_INFO_TYPE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STORAGE_DEPENDENCY_INFO_TYPE_2 {
    pub DependencyTypeFlags: DEPENDENT_DISK_FLAG,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub AncestorLevel: u32,
    pub DependencyDeviceName: windows_core::PWSTR,
    pub HostVolumeName: windows_core::PWSTR,
    pub DependentVolumeName: windows_core::PWSTR,
    pub DependentVolumeRelativePath: windows_core::PWSTR,
}
impl windows_core::TypeKind for STORAGE_DEPENDENCY_INFO_TYPE_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for STORAGE_DEPENDENCY_INFO_TYPE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: TAKE_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: TAKE_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl windows_core::TypeKind for TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl windows_core::TypeKind for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: windows_core::GUID,
}
impl windows_core::TypeKind for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VIRTUAL_DISK_PROGRESS {
    pub OperationStatus: u32,
    pub CurrentValue: u64,
    pub CompletionValue: u64,
}
impl windows_core::TypeKind for VIRTUAL_DISK_PROGRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for VIRTUAL_DISK_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VIRTUAL_STORAGE_TYPE {
    pub DeviceId: u32,
    pub VendorId: windows_core::GUID,
}
impl windows_core::TypeKind for VIRTUAL_STORAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl Default for VIRTUAL_STORAGE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
