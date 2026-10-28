#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddVirtualDiskParent<P1>(virtualdiskhandle: super::HANDLE, parentpath: P1) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("virtdisk.dll" "system" fn AddVirtualDiskParent(virtualdiskhandle : super::HANDLE, parentpath : windows_core::PCWSTR) -> super::DWORD);
    unsafe { AddVirtualDiskParent(virtualdiskhandle, parentpath.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ApplySnapshotVhdSet(virtualdiskhandle: super::HANDLE, parameters: PAPPLY_SNAPSHOT_VHDSET_PARAMETERS, flags: APPLY_SNAPSHOT_VHDSET_FLAG) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn ApplySnapshotVhdSet(virtualdiskhandle : super::HANDLE, parameters : PAPPLY_SNAPSHOT_VHDSET_PARAMETERS, flags : APPLY_SNAPSHOT_VHDSET_FLAG) -> super::DWORD);
    unsafe { ApplySnapshotVhdSet(virtualdiskhandle, parameters, flags) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AttachVirtualDisk(virtualdiskhandle: super::HANDLE, securitydescriptor: Option<super::PSECURITY_DESCRIPTOR>, flags: ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: super::ULONG, parameters: Option<PATTACH_VIRTUAL_DISK_PARAMETERS>, overlapped: Option<super::LPOVERLAPPED>) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn AttachVirtualDisk(virtualdiskhandle : super::HANDLE, securitydescriptor : super::PSECURITY_DESCRIPTOR, flags : ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags : super::ULONG, parameters : PATTACH_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { AttachVirtualDisk(virtualdiskhandle, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, flags, providerspecificflags, parameters.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BreakMirrorVirtualDisk(virtualdiskhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn BreakMirrorVirtualDisk(virtualdiskhandle : super::HANDLE) -> super::DWORD);
    unsafe { BreakMirrorVirtualDisk(virtualdiskhandle) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CompactVirtualDisk(virtualdiskhandle: super::HANDLE, flags: COMPACT_VIRTUAL_DISK_FLAG, parameters: Option<PCOMPACT_VIRTUAL_DISK_PARAMETERS>, overlapped: Option<super::LPOVERLAPPED>) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn CompactVirtualDisk(virtualdiskhandle : super::HANDLE, flags : COMPACT_VIRTUAL_DISK_FLAG, parameters : PCOMPACT_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { CompactVirtualDisk(virtualdiskhandle, flags, parameters.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CompleteForkVirtualDisk(virtualdiskhandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn CompleteForkVirtualDisk(virtualdiskhandle : super::HANDLE) -> super::DWORD);
    unsafe { CompleteForkVirtualDisk(virtualdiskhandle) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winioctl", feature = "winnt"))]
#[inline]
pub unsafe fn CreateVirtualDisk<P1>(virtualstoragetype: super::PVIRTUAL_STORAGE_TYPE, path: P1, virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK, securitydescriptor: Option<super::PSECURITY_DESCRIPTOR>, flags: CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: super::ULONG, parameters: PCREATE_VIRTUAL_DISK_PARAMETERS, overlapped: Option<super::LPOVERLAPPED>, handle: super::PHANDLE) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("virtdisk.dll" "system" fn CreateVirtualDisk(virtualstoragetype : super::PVIRTUAL_STORAGE_TYPE, path : windows_core::PCWSTR, virtualdiskaccessmask : VIRTUAL_DISK_ACCESS_MASK, securitydescriptor : super::PSECURITY_DESCRIPTOR, flags : CREATE_VIRTUAL_DISK_FLAG, providerspecificflags : super::ULONG, parameters : PCREATE_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED, handle : super::PHANDLE) -> super::DWORD);
    unsafe { CreateVirtualDisk(virtualstoragetype, path.param().abi(), virtualdiskaccessmask, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, flags, providerspecificflags, parameters, overlapped.unwrap_or(core::mem::zeroed()) as _, handle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DeleteSnapshotVhdSet(virtualdiskhandle: super::HANDLE, parameters: PDELETE_SNAPSHOT_VHDSET_PARAMETERS, flags: DELETE_SNAPSHOT_VHDSET_FLAG) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn DeleteSnapshotVhdSet(virtualdiskhandle : super::HANDLE, parameters : PDELETE_SNAPSHOT_VHDSET_PARAMETERS, flags : DELETE_SNAPSHOT_VHDSET_FLAG) -> super::DWORD);
    unsafe { DeleteSnapshotVhdSet(virtualdiskhandle, parameters, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DeleteVirtualDiskMetadata(virtualdiskhandle: super::HANDLE, item: *const windows_core::GUID) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn DeleteVirtualDiskMetadata(virtualdiskhandle : super::HANDLE, item : *const windows_core::GUID) -> super::DWORD);
    unsafe { DeleteVirtualDiskMetadata(virtualdiskhandle, item) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DetachVirtualDisk(virtualdiskhandle: super::HANDLE, flags: DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: super::ULONG) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn DetachVirtualDisk(virtualdiskhandle : super::HANDLE, flags : DETACH_VIRTUAL_DISK_FLAG, providerspecificflags : super::ULONG) -> super::DWORD);
    unsafe { DetachVirtualDisk(virtualdiskhandle, flags, providerspecificflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumerateVirtualDiskMetadata(virtualdiskhandle: super::HANDLE, numberofitems: super::PULONG, items: *mut windows_core::GUID) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn EnumerateVirtualDiskMetadata(virtualdiskhandle : super::HANDLE, numberofitems : super::PULONG, items : *mut windows_core::GUID) -> super::DWORD);
    unsafe { EnumerateVirtualDiskMetadata(virtualdiskhandle, numberofitems as _, items as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ExpandVirtualDisk(virtualdiskhandle: super::HANDLE, flags: EXPAND_VIRTUAL_DISK_FLAG, parameters: PEXPAND_VIRTUAL_DISK_PARAMETERS, overlapped: Option<super::LPOVERLAPPED>) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn ExpandVirtualDisk(virtualdiskhandle : super::HANDLE, flags : EXPAND_VIRTUAL_DISK_FLAG, parameters : PEXPAND_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { ExpandVirtualDisk(virtualdiskhandle, flags, parameters, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ForkVirtualDisk(virtualdiskhandle: super::HANDLE, flags: FORK_VIRTUAL_DISK_FLAG, parameters: *const FORK_VIRTUAL_DISK_PARAMETERS, overlapped: super::LPOVERLAPPED) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn ForkVirtualDisk(virtualdiskhandle : super::HANDLE, flags : FORK_VIRTUAL_DISK_FLAG, parameters : *const FORK_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { ForkVirtualDisk(virtualdiskhandle, flags, parameters, overlapped as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetAllAttachedVirtualDiskPhysicalPaths(pathsbuffersizeinbytes: super::PULONG, pathsbuffer: windows_core::PWSTR) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn GetAllAttachedVirtualDiskPhysicalPaths(pathsbuffersizeinbytes : super::PULONG, pathsbuffer : windows_core::PWSTR) -> super::DWORD);
    unsafe { GetAllAttachedVirtualDiskPhysicalPaths(pathsbuffersizeinbytes as _, pathsbuffer) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winioctl", feature = "winnt"))]
#[inline]
pub unsafe fn GetVirtualDiskInformation(virtualdiskhandle: super::HANDLE, virtualdiskinfosize: super::PULONG, virtualdiskinfo: PGET_VIRTUAL_DISK_INFO, sizeused: Option<super::PULONG>) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn GetVirtualDiskInformation(virtualdiskhandle : super::HANDLE, virtualdiskinfosize : super::PULONG, virtualdiskinfo : PGET_VIRTUAL_DISK_INFO, sizeused : super::PULONG) -> super::DWORD);
    unsafe { GetVirtualDiskInformation(virtualdiskhandle, virtualdiskinfosize as _, virtualdiskinfo as _, sizeused.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetVirtualDiskMetadata(virtualdiskhandle: super::HANDLE, item: *const windows_core::GUID, metadatasize: super::PULONG, metadata: super::PVOID) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn GetVirtualDiskMetadata(virtualdiskhandle : super::HANDLE, item : *const windows_core::GUID, metadatasize : super::PULONG, metadata : super::PVOID) -> super::DWORD);
    unsafe { GetVirtualDiskMetadata(virtualdiskhandle, item, metadatasize as _, metadata as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetVirtualDiskOperationProgress(virtualdiskhandle: super::HANDLE, overlapped: super::LPOVERLAPPED, progress: PVIRTUAL_DISK_PROGRESS) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn GetVirtualDiskOperationProgress(virtualdiskhandle : super::HANDLE, overlapped : super::LPOVERLAPPED, progress : PVIRTUAL_DISK_PROGRESS) -> super::DWORD);
    unsafe { GetVirtualDiskOperationProgress(virtualdiskhandle, overlapped, progress as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetVirtualDiskPhysicalPath(virtualdiskhandle: super::HANDLE, diskpathsizeinbytes: super::PULONG, diskpath: windows_core::PWSTR) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn GetVirtualDiskPhysicalPath(virtualdiskhandle : super::HANDLE, diskpathsizeinbytes : super::PULONG, diskpath : windows_core::PWSTR) -> super::DWORD);
    unsafe { GetVirtualDiskPhysicalPath(virtualdiskhandle, diskpathsizeinbytes as _, diskpath) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MergeVirtualDisk(virtualdiskhandle: super::HANDLE, flags: MERGE_VIRTUAL_DISK_FLAG, parameters: PMERGE_VIRTUAL_DISK_PARAMETERS, overlapped: Option<super::LPOVERLAPPED>) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn MergeVirtualDisk(virtualdiskhandle : super::HANDLE, flags : MERGE_VIRTUAL_DISK_FLAG, parameters : PMERGE_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { MergeVirtualDisk(virtualdiskhandle, flags, parameters, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MirrorVirtualDisk(virtualdiskhandle: super::HANDLE, flags: MIRROR_VIRTUAL_DISK_FLAG, parameters: PMIRROR_VIRTUAL_DISK_PARAMETERS, overlapped: super::LPOVERLAPPED) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn MirrorVirtualDisk(virtualdiskhandle : super::HANDLE, flags : MIRROR_VIRTUAL_DISK_FLAG, parameters : PMIRROR_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { MirrorVirtualDisk(virtualdiskhandle, flags, parameters, overlapped) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ModifyVhdSet(virtualdiskhandle: super::HANDLE, parameters: PMODIFY_VHDSET_PARAMETERS, flags: MODIFY_VHDSET_FLAG) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn ModifyVhdSet(virtualdiskhandle : super::HANDLE, parameters : PMODIFY_VHDSET_PARAMETERS, flags : MODIFY_VHDSET_FLAG) -> super::DWORD);
    unsafe { ModifyVhdSet(virtualdiskhandle, parameters, flags) }
}
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
#[inline]
pub unsafe fn OpenVirtualDisk<P1>(virtualstoragetype: super::PVIRTUAL_STORAGE_TYPE, path: P1, virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK, flags: OPEN_VIRTUAL_DISK_FLAG, parameters: Option<POPEN_VIRTUAL_DISK_PARAMETERS>, handle: super::PHANDLE) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("virtdisk.dll" "system" fn OpenVirtualDisk(virtualstoragetype : super::PVIRTUAL_STORAGE_TYPE, path : windows_core::PCWSTR, virtualdiskaccessmask : VIRTUAL_DISK_ACCESS_MASK, flags : OPEN_VIRTUAL_DISK_FLAG, parameters : POPEN_VIRTUAL_DISK_PARAMETERS, handle : super::PHANDLE) -> super::DWORD);
    unsafe { OpenVirtualDisk(virtualstoragetype, path.param().abi(), virtualdiskaccessmask, flags, parameters.unwrap_or(core::mem::zeroed()) as _, handle as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryChangesVirtualDisk<P1>(virtualdiskhandle: super::HANDLE, changetrackingid: P1, byteoffset: super::ULONG64, bytelength: super::ULONG64, flags: QUERY_CHANGES_VIRTUAL_DISK_FLAG, ranges: PQUERY_CHANGES_VIRTUAL_DISK_RANGE, rangecount: super::PULONG, processedlength: super::PULONG64) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("virtdisk.dll" "system" fn QueryChangesVirtualDisk(virtualdiskhandle : super::HANDLE, changetrackingid : windows_core::PCWSTR, byteoffset : super::ULONG64, bytelength : super::ULONG64, flags : QUERY_CHANGES_VIRTUAL_DISK_FLAG, ranges : PQUERY_CHANGES_VIRTUAL_DISK_RANGE, rangecount : super::PULONG, processedlength : super::PULONG64) -> super::DWORD);
    unsafe { QueryChangesVirtualDisk(virtualdiskhandle, changetrackingid.param().abi(), byteoffset, bytelength, flags, ranges as _, rangecount as _, processedlength as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RawSCSIVirtualDisk(virtualdiskhandle: super::HANDLE, parameters: PRAW_SCSI_VIRTUAL_DISK_PARAMETERS, flags: RAW_SCSI_VIRTUAL_DISK_FLAG, response: PRAW_SCSI_VIRTUAL_DISK_RESPONSE) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn RawSCSIVirtualDisk(virtualdiskhandle : super::HANDLE, parameters : PRAW_SCSI_VIRTUAL_DISK_PARAMETERS, flags : RAW_SCSI_VIRTUAL_DISK_FLAG, response : PRAW_SCSI_VIRTUAL_DISK_RESPONSE) -> super::DWORD);
    unsafe { RawSCSIVirtualDisk(virtualdiskhandle, parameters, flags, response as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ResizeVirtualDisk(virtualdiskhandle: super::HANDLE, flags: RESIZE_VIRTUAL_DISK_FLAG, parameters: PRESIZE_VIRTUAL_DISK_PARAMETERS, overlapped: Option<super::LPOVERLAPPED>) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn ResizeVirtualDisk(virtualdiskhandle : super::HANDLE, flags : RESIZE_VIRTUAL_DISK_FLAG, parameters : PRESIZE_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { ResizeVirtualDisk(virtualdiskhandle, flags, parameters, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetVirtualDiskInformation(virtualdiskhandle: super::HANDLE, virtualdiskinfo: PSET_VIRTUAL_DISK_INFO) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn SetVirtualDiskInformation(virtualdiskhandle : super::HANDLE, virtualdiskinfo : PSET_VIRTUAL_DISK_INFO) -> super::DWORD);
    unsafe { SetVirtualDiskInformation(virtualdiskhandle, virtualdiskinfo) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetVirtualDiskMetadata(virtualdiskhandle: super::HANDLE, item: *const windows_core::GUID, metadatasize: super::ULONG, metadata: *const core::ffi::c_void) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn SetVirtualDiskMetadata(virtualdiskhandle : super::HANDLE, item : *const windows_core::GUID, metadatasize : super::ULONG, metadata : *const core::ffi::c_void) -> super::DWORD);
    unsafe { SetVirtualDiskMetadata(virtualdiskhandle, item, metadatasize, metadata) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SurfaceVirtualDisk(virtualdiskhandle: super::HANDLE, securitydescriptor: Option<super::PSECURITY_DESCRIPTOR>, flags: ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: super::ULONG, parameters: Option<PATTACH_VIRTUAL_DISK_PARAMETERS>, overlapped: Option<super::LPOVERLAPPED>) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" "AttachVirtualDisk" fn SurfaceVirtualDisk(virtualdiskhandle : super::HANDLE, securitydescriptor : super::PSECURITY_DESCRIPTOR, flags : ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags : super::ULONG, parameters : PATTACH_VIRTUAL_DISK_PARAMETERS, overlapped : super::LPOVERLAPPED) -> super::DWORD);
    unsafe { SurfaceVirtualDisk(virtualdiskhandle, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, flags, providerspecificflags, parameters.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TakeSnapshotVhdSet(virtualdiskhandle: super::HANDLE, parameters: PTAKE_SNAPSHOT_VHDSET_PARAMETERS, flags: TAKE_SNAPSHOT_VHDSET_FLAG) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" fn TakeSnapshotVhdSet(virtualdiskhandle : super::HANDLE, parameters : PTAKE_SNAPSHOT_VHDSET_PARAMETERS, flags : TAKE_SNAPSHOT_VHDSET_FLAG) -> super::DWORD);
    unsafe { TakeSnapshotVhdSet(virtualdiskhandle, parameters, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UnsurfaceVirtualDisk(virtualdiskhandle: super::HANDLE, flags: DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: super::ULONG) -> super::DWORD {
    windows_core::link!("virtdisk.dll" "system" "DetachVirtualDisk" fn UnsurfaceVirtualDisk(virtualdiskhandle : super::HANDLE, flags : DETACH_VIRTUAL_DISK_FLAG, providerspecificflags : super::ULONG) -> super::DWORD);
    unsafe { UnsurfaceVirtualDisk(virtualdiskhandle, flags, providerspecificflags) }
}
pub type APPLY_SNAPSHOT_VHDSET_FLAG = i32;
pub const APPLY_SNAPSHOT_VHDSET_FLAG_NONE: APPLY_SNAPSHOT_VHDSET_FLAG = 0;
pub const APPLY_SNAPSHOT_VHDSET_FLAG_WRITEABLE: APPLY_SNAPSHOT_VHDSET_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: APPLY_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: APPLY_SNAPSHOT_VHDSET_PARAMETERS_0,
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
impl Default for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: windows_core::GUID,
    pub LeafSnapshotId: windows_core::GUID,
}
pub type APPLY_SNAPSHOT_VHDSET_VERSION = i32;
pub const APPLY_SNAPSHOT_VHDSET_VERSION_1: APPLY_SNAPSHOT_VHDSET_VERSION = 1;
pub const APPLY_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: APPLY_SNAPSHOT_VHDSET_VERSION = 0;
pub type ATTACH_VIRTUAL_DISK_FLAG = i32;
pub const ATTACH_VIRTUAL_DISK_FLAG_AT_BOOT: ATTACH_VIRTUAL_DISK_FLAG = 1024;
pub const ATTACH_VIRTUAL_DISK_FLAG_BYPASS_DEFAULT_ENCRYPTION_POLICY: ATTACH_VIRTUAL_DISK_FLAG = 32;
pub const ATTACH_VIRTUAL_DISK_FLAG_NONE: ATTACH_VIRTUAL_DISK_FLAG = 0;
pub const ATTACH_VIRTUAL_DISK_FLAG_NON_PNP: ATTACH_VIRTUAL_DISK_FLAG = 64;
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_DRIVE_LETTER: ATTACH_VIRTUAL_DISK_FLAG = 2;
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_LOCAL_HOST: ATTACH_VIRTUAL_DISK_FLAG = 8;
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_SECURITY_DESCRIPTOR: ATTACH_VIRTUAL_DISK_FLAG = 16;
pub const ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME: ATTACH_VIRTUAL_DISK_FLAG = 4;
pub const ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY: ATTACH_VIRTUAL_DISK_FLAG = 1;
pub const ATTACH_VIRTUAL_DISK_FLAG_REGISTER_VOLUME: ATTACH_VIRTUAL_DISK_FLAG = 512;
pub const ATTACH_VIRTUAL_DISK_FLAG_RESTRICTED_RANGE: ATTACH_VIRTUAL_DISK_FLAG = 128;
pub const ATTACH_VIRTUAL_DISK_FLAG_SINGLE_PARTITION: ATTACH_VIRTUAL_DISK_FLAG = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS {
    pub Version: ATTACH_VIRTUAL_DISK_VERSION,
    pub Anonymous: ATTACH_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "shared")]
impl Default for ATTACH_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: ATTACH_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: ATTACH_VIRTUAL_DISK_PARAMETERS_0_1,
}
#[cfg(feature = "shared")]
impl Default for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub RestrictedOffset: super::ULONGLONG,
    pub RestrictedLength: super::ULONGLONG,
}
pub type ATTACH_VIRTUAL_DISK_VERSION = i32;
pub const ATTACH_VIRTUAL_DISK_VERSION_1: ATTACH_VIRTUAL_DISK_VERSION = 1;
pub const ATTACH_VIRTUAL_DISK_VERSION_2: ATTACH_VIRTUAL_DISK_VERSION = 2;
pub const ATTACH_VIRTUAL_DISK_VERSION_UNSPECIFIED: ATTACH_VIRTUAL_DISK_VERSION = 0;
pub type COMPACT_VIRTUAL_DISK_FLAG = i32;
pub const COMPACT_VIRTUAL_DISK_FLAG_NONE: COMPACT_VIRTUAL_DISK_FLAG = 0;
pub const COMPACT_VIRTUAL_DISK_FLAG_NO_BLOCK_MOVES: COMPACT_VIRTUAL_DISK_FLAG = 2;
pub const COMPACT_VIRTUAL_DISK_FLAG_NO_ZERO_SCAN: COMPACT_VIRTUAL_DISK_FLAG = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS {
    pub Version: COMPACT_VIRTUAL_DISK_VERSION,
    pub Anonymous: COMPACT_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "shared")]
impl Default for COMPACT_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: COMPACT_VIRTUAL_DISK_PARAMETERS_0_0,
}
#[cfg(feature = "shared")]
impl Default for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub Reserved: super::ULONG,
}
pub type COMPACT_VIRTUAL_DISK_VERSION = i32;
pub const COMPACT_VIRTUAL_DISK_VERSION_1: COMPACT_VIRTUAL_DISK_VERSION = 1;
pub const COMPACT_VIRTUAL_DISK_VERSION_UNSPECIFIED: COMPACT_VIRTUAL_DISK_VERSION = 0;
pub type CREATE_VIRTUAL_DISK_FLAG = i32;
pub const CREATE_VIRTUAL_DISK_FLAG_CREATE_BACKING_STORAGE: CREATE_VIRTUAL_DISK_FLAG = 8;
pub const CREATE_VIRTUAL_DISK_FLAG_DO_NOT_COPY_METADATA_FROM_PARENT: CREATE_VIRTUAL_DISK_FLAG = 4;
pub const CREATE_VIRTUAL_DISK_FLAG_FULL_PHYSICAL_ALLOCATION: CREATE_VIRTUAL_DISK_FLAG = 1;
pub const CREATE_VIRTUAL_DISK_FLAG_NONE: CREATE_VIRTUAL_DISK_FLAG = 0;
pub const CREATE_VIRTUAL_DISK_FLAG_PMEM_COMPATIBLE: CREATE_VIRTUAL_DISK_FLAG = 256;
pub const CREATE_VIRTUAL_DISK_FLAG_PRESERVE_PARENT_CHANGE_TRACKING_STATE: CREATE_VIRTUAL_DISK_FLAG = 32;
pub const CREATE_VIRTUAL_DISK_FLAG_PREVENT_WRITES_TO_SOURCE_DISK: CREATE_VIRTUAL_DISK_FLAG = 2;
pub const CREATE_VIRTUAL_DISK_FLAG_SPARSE_FILE: CREATE_VIRTUAL_DISK_FLAG = 128;
pub const CREATE_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: CREATE_VIRTUAL_DISK_FLAG = 512;
pub const CREATE_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS: CREATE_VIRTUAL_DISK_FLAG = 1024;
pub const CREATE_VIRTUAL_DISK_FLAG_USE_CHANGE_TRACKING_SOURCE_LIMIT: CREATE_VIRTUAL_DISK_FLAG = 16;
pub const CREATE_VIRTUAL_DISK_FLAG_USE_RCT_SOURCE_LIMIT: CREATE_VIRTUAL_DISK_FLAG = 16;
pub const CREATE_VIRTUAL_DISK_FLAG_VHD_SET_USE_ORIGINAL_BACKING_STORAGE: CREATE_VIRTUAL_DISK_FLAG = 64;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl"))]
#[derive(Clone, Copy)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS {
    pub Version: CREATE_VIRTUAL_DISK_VERSION,
    pub Anonymous: CREATE_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(all(feature = "shared", feature = "winioctl"))]
impl Default for CREATE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl"))]
#[derive(Clone, Copy)]
pub union CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: CREATE_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: CREATE_VIRTUAL_DISK_PARAMETERS_0_1,
    pub Version3: CREATE_VIRTUAL_DISK_PARAMETERS_0_2,
    pub Version4: CREATE_VIRTUAL_DISK_PARAMETERS_0_3,
}
#[cfg(all(feature = "shared", feature = "winioctl"))]
impl Default for CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: super::ULONGLONG,
    pub BlockSizeInBytes: super::ULONG,
    pub SectorSizeInBytes: super::ULONG,
    pub ParentPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: super::ULONGLONG,
    pub BlockSizeInBytes: super::ULONG,
    pub SectorSizeInBytes: super::ULONG,
    pub PhysicalSectorSizeInBytes: super::ULONG,
    pub ParentPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: super::ULONGLONG,
    pub BlockSizeInBytes: super::ULONG,
    pub SectorSizeInBytes: super::ULONG,
    pub PhysicalSectorSizeInBytes: super::ULONG,
    pub ParentPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: windows_core::GUID,
    pub SourceLimitPath: windows_core::PCWSTR,
    pub BackingStorageType: super::VIRTUAL_STORAGE_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    pub UniqueId: windows_core::GUID,
    pub MaximumSize: super::ULONGLONG,
    pub BlockSizeInBytes: super::ULONG,
    pub SectorSizeInBytes: super::ULONG,
    pub PhysicalSectorSizeInBytes: super::ULONG,
    pub ParentPath: windows_core::PCWSTR,
    pub SourcePath: windows_core::PCWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: windows_core::GUID,
    pub SourceLimitPath: windows_core::PCWSTR,
    pub BackingStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub PmemAddressAbstractionType: windows_core::GUID,
    pub DataAlignment: super::ULONGLONG,
}
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_BLOCK_SIZE: i32 = 0;
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_SECTOR_SIZE: i32 = 0;
pub type CREATE_VIRTUAL_DISK_VERSION = i32;
pub const CREATE_VIRTUAL_DISK_VERSION_1: CREATE_VIRTUAL_DISK_VERSION = 1;
pub const CREATE_VIRTUAL_DISK_VERSION_2: CREATE_VIRTUAL_DISK_VERSION = 2;
pub const CREATE_VIRTUAL_DISK_VERSION_3: CREATE_VIRTUAL_DISK_VERSION = 3;
pub const CREATE_VIRTUAL_DISK_VERSION_4: CREATE_VIRTUAL_DISK_VERSION = 4;
pub const CREATE_VIRTUAL_DISK_VERSION_UNSPECIFIED: CREATE_VIRTUAL_DISK_VERSION = 0;
pub type DELETE_SNAPSHOT_VHDSET_FLAG = i32;
pub const DELETE_SNAPSHOT_VHDSET_FLAG_NONE: DELETE_SNAPSHOT_VHDSET_FLAG = 0;
pub const DELETE_SNAPSHOT_VHDSET_FLAG_PERSIST_RCT: DELETE_SNAPSHOT_VHDSET_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: DELETE_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: DELETE_SNAPSHOT_VHDSET_PARAMETERS_0,
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
impl Default for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: windows_core::GUID,
}
pub type DELETE_SNAPSHOT_VHDSET_VERSION = i32;
pub const DELETE_SNAPSHOT_VHDSET_VERSION_1: DELETE_SNAPSHOT_VHDSET_VERSION = 1;
pub const DELETE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: DELETE_SNAPSHOT_VHDSET_VERSION = 0;
pub type DEPENDENT_DISK_FLAG = i32;
pub const DEPENDENT_DISK_FLAG_ALWAYS_ALLOW_SPARSE: DEPENDENT_DISK_FLAG = 4096;
pub const DEPENDENT_DISK_FLAG_FULLY_ALLOCATED: DEPENDENT_DISK_FLAG = 2;
pub const DEPENDENT_DISK_FLAG_MULT_BACKING_FILES: DEPENDENT_DISK_FLAG = 1;
pub const DEPENDENT_DISK_FLAG_NONE: DEPENDENT_DISK_FLAG = 0;
pub const DEPENDENT_DISK_FLAG_NO_DRIVE_LETTER: DEPENDENT_DISK_FLAG = 128;
pub const DEPENDENT_DISK_FLAG_NO_HOST_DISK: DEPENDENT_DISK_FLAG = 512;
pub const DEPENDENT_DISK_FLAG_PARENT: DEPENDENT_DISK_FLAG = 256;
pub const DEPENDENT_DISK_FLAG_PERMANENT_LIFETIME: DEPENDENT_DISK_FLAG = 1024;
pub const DEPENDENT_DISK_FLAG_READ_ONLY: DEPENDENT_DISK_FLAG = 4;
pub const DEPENDENT_DISK_FLAG_REMOTE: DEPENDENT_DISK_FLAG = 8;
pub const DEPENDENT_DISK_FLAG_REMOVABLE: DEPENDENT_DISK_FLAG = 64;
pub const DEPENDENT_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: DEPENDENT_DISK_FLAG = 2048;
pub const DEPENDENT_DISK_FLAG_SUPPORT_ENCRYPTED_FILES: DEPENDENT_DISK_FLAG = 8192;
pub const DEPENDENT_DISK_FLAG_SYSTEM_VOLUME: DEPENDENT_DISK_FLAG = 16;
pub const DEPENDENT_DISK_FLAG_SYSTEM_VOLUME_PARENT: DEPENDENT_DISK_FLAG = 32;
pub type DETACH_VIRTUAL_DISK_FLAG = i32;
pub const DETACH_VIRTUAL_DISK_FLAG_NONE: DETACH_VIRTUAL_DISK_FLAG = 0;
pub type EXPAND_VIRTUAL_DISK_FLAG = i32;
pub const EXPAND_VIRTUAL_DISK_FLAG_NONE: EXPAND_VIRTUAL_DISK_FLAG = 0;
pub const EXPAND_VIRTUAL_DISK_FLAG_NOTIFY_CHANGE: EXPAND_VIRTUAL_DISK_FLAG = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS {
    pub Version: EXPAND_VIRTUAL_DISK_VERSION,
    pub Anonymous: EXPAND_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "shared")]
impl Default for EXPAND_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: EXPAND_VIRTUAL_DISK_PARAMETERS_0_0,
}
#[cfg(feature = "shared")]
impl Default for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub NewSize: super::ULONGLONG,
}
pub type EXPAND_VIRTUAL_DISK_VERSION = i32;
pub const EXPAND_VIRTUAL_DISK_VERSION_1: EXPAND_VIRTUAL_DISK_VERSION = 1;
pub const EXPAND_VIRTUAL_DISK_VERSION_UNSPECIFIED: EXPAND_VIRTUAL_DISK_VERSION = 0;
pub type FORK_VIRTUAL_DISK_FLAG = i32;
pub const FORK_VIRTUAL_DISK_FLAG_EXISTING_FILE: FORK_VIRTUAL_DISK_FLAG = 1;
pub const FORK_VIRTUAL_DISK_FLAG_NONE: FORK_VIRTUAL_DISK_FLAG = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FORK_VIRTUAL_DISK_PARAMETERS {
    pub Version: FORK_VIRTUAL_DISK_VERSION,
    pub Anonymous: FORK_VIRTUAL_DISK_PARAMETERS_0,
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
impl Default for FORK_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub ForkedVirtualDiskPath: windows_core::PCWSTR,
}
pub type FORK_VIRTUAL_DISK_VERSION = i32;
pub const FORK_VIRTUAL_DISK_VERSION_1: FORK_VIRTUAL_DISK_VERSION = 1;
pub const FORK_VIRTUAL_DISK_VERSION_UNSPECIFIED: FORK_VIRTUAL_DISK_VERSION = 0;
pub type GET_STORAGE_DEPENDENCY_FLAG = i32;
pub const GET_STORAGE_DEPENDENCY_FLAG_DISK_HANDLE: GET_STORAGE_DEPENDENCY_FLAG = 2;
pub const GET_STORAGE_DEPENDENCY_FLAG_HOST_VOLUMES: GET_STORAGE_DEPENDENCY_FLAG = 1;
pub const GET_STORAGE_DEPENDENCY_FLAG_NONE: GET_STORAGE_DEPENDENCY_FLAG = 0;
pub const GET_STORAGE_DEPENDENCY_FLAG_PARENTS: GET_STORAGE_DEPENDENCY_FLAG = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct GET_VIRTUAL_DISK_INFO {
    pub Version: GET_VIRTUAL_DISK_INFO_VERSION,
    pub Anonymous: GET_VIRTUAL_DISK_INFO_0,
}
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
impl Default for GET_VIRTUAL_DISK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union GET_VIRTUAL_DISK_INFO_0 {
    pub Size: GET_VIRTUAL_DISK_INFO_0_0,
    pub Identifier: windows_core::GUID,
    pub ParentLocation: GET_VIRTUAL_DISK_INFO_0_1,
    pub ParentIdentifier: windows_core::GUID,
    pub ParentTimestamp: super::ULONG,
    pub VirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub ProviderSubtype: super::ULONG,
    pub Is4kAligned: windows_core::BOOL,
    pub IsLoaded: windows_core::BOOL,
    pub PhysicalDisk: GET_VIRTUAL_DISK_INFO_0_2,
    pub VhdPhysicalSectorSize: super::ULONG,
    pub SmallestSafeVirtualSize: super::ULONGLONG,
    pub FragmentationPercentage: super::ULONG,
    pub VirtualDiskId: windows_core::GUID,
    pub ChangeTrackingState: GET_VIRTUAL_DISK_INFO_0_3,
}
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
impl Default for GET_VIRTUAL_DISK_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_VIRTUAL_DISK_INFO_0_0 {
    pub VirtualSize: super::ULONGLONG,
    pub PhysicalSize: super::ULONGLONG,
    pub BlockSize: super::ULONG,
    pub SectorSize: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_VIRTUAL_DISK_INFO_0_1 {
    pub ParentResolved: windows_core::BOOL,
    pub ParentLocationBuffer: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
impl Default for GET_VIRTUAL_DISK_INFO_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_VIRTUAL_DISK_INFO_0_2 {
    pub LogicalSectorSize: super::ULONG,
    pub PhysicalSectorSize: super::ULONG,
    pub IsRemote: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_VIRTUAL_DISK_INFO_0_3 {
    pub Enabled: windows_core::BOOL,
    pub NewerChanges: windows_core::BOOL,
    pub MostRecentId: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
impl Default for GET_VIRTUAL_DISK_INFO_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE: GET_VIRTUAL_DISK_INFO_VERSION = 15;
pub const GET_VIRTUAL_DISK_INFO_FRAGMENTATION: GET_VIRTUAL_DISK_INFO_VERSION = 12;
pub const GET_VIRTUAL_DISK_INFO_IDENTIFIER: GET_VIRTUAL_DISK_INFO_VERSION = 2;
pub const GET_VIRTUAL_DISK_INFO_IS_4K_ALIGNED: GET_VIRTUAL_DISK_INFO_VERSION = 8;
pub const GET_VIRTUAL_DISK_INFO_IS_LOADED: GET_VIRTUAL_DISK_INFO_VERSION = 13;
pub const GET_VIRTUAL_DISK_INFO_PARENT_IDENTIFIER: GET_VIRTUAL_DISK_INFO_VERSION = 4;
pub const GET_VIRTUAL_DISK_INFO_PARENT_LOCATION: GET_VIRTUAL_DISK_INFO_VERSION = 3;
pub const GET_VIRTUAL_DISK_INFO_PARENT_TIMESTAMP: GET_VIRTUAL_DISK_INFO_VERSION = 5;
pub const GET_VIRTUAL_DISK_INFO_PHYSICAL_DISK: GET_VIRTUAL_DISK_INFO_VERSION = 9;
pub const GET_VIRTUAL_DISK_INFO_PROVIDER_SUBTYPE: GET_VIRTUAL_DISK_INFO_VERSION = 7;
pub const GET_VIRTUAL_DISK_INFO_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = 1;
pub const GET_VIRTUAL_DISK_INFO_SMALLEST_SAFE_VIRTUAL_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = 11;
pub const GET_VIRTUAL_DISK_INFO_UNSPECIFIED: GET_VIRTUAL_DISK_INFO_VERSION = 0;
pub type GET_VIRTUAL_DISK_INFO_VERSION = i32;
pub const GET_VIRTUAL_DISK_INFO_VHD_PHYSICAL_SECTOR_SIZE: GET_VIRTUAL_DISK_INFO_VERSION = 10;
pub const GET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID: GET_VIRTUAL_DISK_INFO_VERSION = 14;
pub const GET_VIRTUAL_DISK_INFO_VIRTUAL_STORAGE_TYPE: GET_VIRTUAL_DISK_INFO_VERSION = 6;
pub const MERGE_VIRTUAL_DISK_DEFAULT_MERGE_DEPTH: i32 = 1;
pub type MERGE_VIRTUAL_DISK_FLAG = i32;
pub const MERGE_VIRTUAL_DISK_FLAG_NONE: MERGE_VIRTUAL_DISK_FLAG = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS {
    pub Version: MERGE_VIRTUAL_DISK_VERSION,
    pub Anonymous: MERGE_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "shared")]
impl Default for MERGE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: MERGE_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: MERGE_VIRTUAL_DISK_PARAMETERS_0_1,
}
#[cfg(feature = "shared")]
impl Default for MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub MergeDepth: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub MergeSourceDepth: super::ULONG,
    pub MergeTargetDepth: super::ULONG,
}
pub type MERGE_VIRTUAL_DISK_VERSION = i32;
pub const MERGE_VIRTUAL_DISK_VERSION_1: MERGE_VIRTUAL_DISK_VERSION = 1;
pub const MERGE_VIRTUAL_DISK_VERSION_2: MERGE_VIRTUAL_DISK_VERSION = 2;
pub const MERGE_VIRTUAL_DISK_VERSION_UNSPECIFIED: MERGE_VIRTUAL_DISK_VERSION = 0;
pub type MIRROR_VIRTUAL_DISK_FLAG = i32;
pub const MIRROR_VIRTUAL_DISK_FLAG_ENABLE_SMB_COMPRESSION: MIRROR_VIRTUAL_DISK_FLAG = 4;
pub const MIRROR_VIRTUAL_DISK_FLAG_EXISTING_FILE: MIRROR_VIRTUAL_DISK_FLAG = 1;
pub const MIRROR_VIRTUAL_DISK_FLAG_IS_LIVE_MIGRATION: MIRROR_VIRTUAL_DISK_FLAG = 8;
pub const MIRROR_VIRTUAL_DISK_FLAG_NONE: MIRROR_VIRTUAL_DISK_FLAG = 0;
pub const MIRROR_VIRTUAL_DISK_FLAG_SKIP_MIRROR_ACTIVATION: MIRROR_VIRTUAL_DISK_FLAG = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS {
    pub Version: MIRROR_VIRTUAL_DISK_VERSION,
    pub Anonymous: MIRROR_VIRTUAL_DISK_PARAMETERS_0,
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
impl Default for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub MirrorVirtualDiskPath: windows_core::PCWSTR,
}
pub type MIRROR_VIRTUAL_DISK_VERSION = i32;
pub const MIRROR_VIRTUAL_DISK_VERSION_1: MIRROR_VIRTUAL_DISK_VERSION = 1;
pub const MIRROR_VIRTUAL_DISK_VERSION_UNSPECIFIED: MIRROR_VIRTUAL_DISK_VERSION = 0;
pub const MODIFY_VHDSET_DEFAULT_SNAPSHOT_PATH: MODIFY_VHDSET_VERSION = 3;
pub type MODIFY_VHDSET_FLAG = i32;
pub const MODIFY_VHDSET_FLAG_NONE: MODIFY_VHDSET_FLAG = 0;
pub const MODIFY_VHDSET_FLAG_WRITEABLE_SNAPSHOT: MODIFY_VHDSET_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MODIFY_VHDSET_PARAMETERS {
    pub Version: MODIFY_VHDSET_VERSION,
    pub Anonymous: MODIFY_VHDSET_PARAMETERS_0,
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
impl Default for MODIFY_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODIFY_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: windows_core::GUID,
    pub SnapshotFilePath: windows_core::PCWSTR,
}
pub const MODIFY_VHDSET_REMOVE_SNAPSHOT: MODIFY_VHDSET_VERSION = 2;
pub const MODIFY_VHDSET_SNAPSHOT_PATH: MODIFY_VHDSET_VERSION = 1;
pub const MODIFY_VHDSET_UNSPECIFIED: MODIFY_VHDSET_VERSION = 0;
pub type MODIFY_VHDSET_VERSION = i32;
pub type OPEN_VIRTUAL_DISK_FLAG = i32;
pub const OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE: OPEN_VIRTUAL_DISK_FLAG = 2;
pub const OPEN_VIRTUAL_DISK_FLAG_BOOT_DRIVE: OPEN_VIRTUAL_DISK_FLAG = 4;
pub const OPEN_VIRTUAL_DISK_FLAG_CACHED_IO: OPEN_VIRTUAL_DISK_FLAG = 8;
pub const OPEN_VIRTUAL_DISK_FLAG_CUSTOM_DIFF_CHAIN: OPEN_VIRTUAL_DISK_FLAG = 16;
pub const OPEN_VIRTUAL_DISK_FLAG_IGNORE_RELATIVE_PARENT_LOCATOR: OPEN_VIRTUAL_DISK_FLAG = 128;
pub const OPEN_VIRTUAL_DISK_FLAG_NONE: OPEN_VIRTUAL_DISK_FLAG = 0;
pub const OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS: OPEN_VIRTUAL_DISK_FLAG = 1;
pub const OPEN_VIRTUAL_DISK_FLAG_NO_WRITE_HARDENING: OPEN_VIRTUAL_DISK_FLAG = 256;
pub const OPEN_VIRTUAL_DISK_FLAG_PARENT_CACHED_IO: OPEN_VIRTUAL_DISK_FLAG = 32;
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: OPEN_VIRTUAL_DISK_FLAG = 512;
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_ENCRYPTED_FILES: OPEN_VIRTUAL_DISK_FLAG = 2048;
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS: OPEN_VIRTUAL_DISK_FLAG = 1024;
pub const OPEN_VIRTUAL_DISK_FLAG_VHDSET_FILE_ONLY: OPEN_VIRTUAL_DISK_FLAG = 64;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS {
    pub Version: OPEN_VIRTUAL_DISK_VERSION,
    pub Anonymous: OPEN_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "shared")]
impl Default for OPEN_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: OPEN_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: OPEN_VIRTUAL_DISK_PARAMETERS_0_1,
    pub Version3: OPEN_VIRTUAL_DISK_PARAMETERS_0_2,
}
#[cfg(feature = "shared")]
impl Default for OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub RWDepth: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub GetInfoOnly: windows_core::BOOL,
    pub ReadOnly: windows_core::BOOL,
    pub ResiliencyGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    pub GetInfoOnly: windows_core::BOOL,
    pub ReadOnly: windows_core::BOOL,
    pub ResiliencyGuid: windows_core::GUID,
    pub SnapshotId: windows_core::GUID,
}
pub const OPEN_VIRTUAL_DISK_RW_DEPTH_DEFAULT: i32 = 1;
pub type OPEN_VIRTUAL_DISK_VERSION = i32;
pub const OPEN_VIRTUAL_DISK_VERSION_1: OPEN_VIRTUAL_DISK_VERSION = 1;
pub const OPEN_VIRTUAL_DISK_VERSION_2: OPEN_VIRTUAL_DISK_VERSION = 2;
pub const OPEN_VIRTUAL_DISK_VERSION_3: OPEN_VIRTUAL_DISK_VERSION = 3;
pub const OPEN_VIRTUAL_DISK_VERSION_UNSPECIFIED: OPEN_VIRTUAL_DISK_VERSION = 0;
pub type PAPPLY_SNAPSHOT_VHDSET_FLAG = *mut APPLY_SNAPSHOT_VHDSET_FLAG;
pub type PAPPLY_SNAPSHOT_VHDSET_PARAMETERS = *mut APPLY_SNAPSHOT_VHDSET_PARAMETERS;
#[cfg(feature = "shared")]
pub type PATTACH_VIRTUAL_DISK_PARAMETERS = *mut ATTACH_VIRTUAL_DISK_PARAMETERS;
#[cfg(feature = "shared")]
pub type PCOMPACT_VIRTUAL_DISK_PARAMETERS = *mut COMPACT_VIRTUAL_DISK_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winioctl"))]
pub type PCREATE_VIRTUAL_DISK_PARAMETERS = *mut CREATE_VIRTUAL_DISK_PARAMETERS;
pub type PDELETE_SNAPSHOT_VHDSET_FLAG = *mut DELETE_SNAPSHOT_VHDSET_FLAG;
pub type PDELETE_SNAPSHOT_VHDSET_PARAMETERS = *mut DELETE_SNAPSHOT_VHDSET_PARAMETERS;
#[cfg(feature = "shared")]
pub type PEXPAND_VIRTUAL_DISK_PARAMETERS = *mut EXPAND_VIRTUAL_DISK_PARAMETERS;
pub type PFORK_VIRTUAL_DISK_PARAMETERS = *mut FORK_VIRTUAL_DISK_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winioctl", feature = "winnt"))]
pub type PGET_VIRTUAL_DISK_INFO = *mut GET_VIRTUAL_DISK_INFO;
#[cfg(feature = "shared")]
pub type PMERGE_VIRTUAL_DISK_PARAMETERS = *mut MERGE_VIRTUAL_DISK_PARAMETERS;
pub type PMIRROR_VIRTUAL_DISK_PARAMETERS = *mut MIRROR_VIRTUAL_DISK_PARAMETERS;
pub type PMODIFY_VHDSET_FLAG = *mut MODIFY_VHDSET_FLAG;
pub type PMODIFY_VHDSET_PARAMETERS = *mut MODIFY_VHDSET_PARAMETERS;
pub type PMODIFY_VHDSET_VERSION = *mut MODIFY_VHDSET_VERSION;
#[cfg(feature = "shared")]
pub type POPEN_VIRTUAL_DISK_PARAMETERS = *mut OPEN_VIRTUAL_DISK_PARAMETERS;
#[cfg(feature = "shared")]
pub type PQUERY_CHANGES_VIRTUAL_DISK_RANGE = *mut QUERY_CHANGES_VIRTUAL_DISK_RANGE;
pub type PRAW_SCSI_VIRTUAL_DISK_FLAG = *mut RAW_SCSI_VIRTUAL_DISK_FLAG;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRAW_SCSI_VIRTUAL_DISK_PARAMETERS = *mut RAW_SCSI_VIRTUAL_DISK_PARAMETERS;
#[cfg(feature = "shared")]
pub type PRAW_SCSI_VIRTUAL_DISK_RESPONSE = *mut RAW_SCSI_VIRTUAL_DISK_RESPONSE;
#[cfg(feature = "shared")]
pub type PRESIZE_VIRTUAL_DISK_PARAMETERS = *mut RESIZE_VIRTUAL_DISK_PARAMETERS;
#[cfg(feature = "shared")]
pub type PSET_VIRTUAL_DISK_INFO = *mut SET_VIRTUAL_DISK_INFO;
#[cfg(all(feature = "shared", feature = "winioctl"))]
pub type PSTORAGE_DEPENDENCY_INFO_TYPE_1 = *mut STORAGE_DEPENDENCY_INFO_TYPE_1;
#[cfg(all(feature = "shared", feature = "winioctl"))]
pub type PSTORAGE_DEPENDENCY_INFO_TYPE_2 = *mut STORAGE_DEPENDENCY_INFO_TYPE_2;
pub type PTAKE_SNAPSHOT_VHDSET_FLAG = *mut TAKE_SNAPSHOT_VHDSET_FLAG;
pub type PTAKE_SNAPSHOT_VHDSET_PARAMETERS = *mut TAKE_SNAPSHOT_VHDSET_PARAMETERS;
#[cfg(feature = "shared")]
pub type PVIRTUAL_DISK_PROGRESS = *mut VIRTUAL_DISK_PROGRESS;
pub type QUERY_CHANGES_VIRTUAL_DISK_FLAG = i32;
pub const QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE: QUERY_CHANGES_VIRTUAL_DISK_FLAG = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    pub ByteOffset: super::ULONG64,
    pub ByteLength: super::ULONG64,
    pub Reserved: super::ULONG64,
}
pub type RAW_SCSI_VIRTUAL_DISK_FLAG = i32;
pub const RAW_SCSI_VIRTUAL_DISK_FLAG_NONE: RAW_SCSI_VIRTUAL_DISK_FLAG = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    pub Version: RAW_SCSI_VIRTUAL_DISK_VERSION,
    pub Anonymous: RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub RSVDHandle: windows_core::BOOL,
    pub DataIn: super::UCHAR,
    pub CdbLength: super::UCHAR,
    pub SenseInfoLength: super::UCHAR,
    pub SrbFlags: super::ULONG,
    pub DataTransferLength: super::ULONG,
    pub DataBuffer: super::PVOID,
    pub SenseInfo: *mut super::UCHAR,
    pub Cdb: *mut super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    pub Version: RAW_SCSI_VIRTUAL_DISK_VERSION,
    pub Anonymous: RAW_SCSI_VIRTUAL_DISK_RESPONSE_0,
}
#[cfg(feature = "shared")]
impl Default for RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    pub Version1: RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0,
}
#[cfg(feature = "shared")]
impl Default for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    pub ScsiStatus: super::UCHAR,
    pub SenseInfoLength: super::UCHAR,
    pub DataTransferLength: super::ULONG,
}
pub type RAW_SCSI_VIRTUAL_DISK_VERSION = i32;
pub const RAW_SCSI_VIRTUAL_DISK_VERSION_1: RAW_SCSI_VIRTUAL_DISK_VERSION = 1;
pub const RAW_SCSI_VIRTUAL_DISK_VERSION_UNSPECIFIED: RAW_SCSI_VIRTUAL_DISK_VERSION = 0;
pub type RESIZE_VIRTUAL_DISK_FLAG = i32;
pub const RESIZE_VIRTUAL_DISK_FLAG_ALLOW_UNSAFE_VIRTUAL_SIZE: RESIZE_VIRTUAL_DISK_FLAG = 1;
pub const RESIZE_VIRTUAL_DISK_FLAG_NONE: RESIZE_VIRTUAL_DISK_FLAG = 0;
pub const RESIZE_VIRTUAL_DISK_FLAG_RESIZE_TO_SMALLEST_SAFE_VIRTUAL_SIZE: RESIZE_VIRTUAL_DISK_FLAG = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS {
    pub Version: RESIZE_VIRTUAL_DISK_VERSION,
    pub Anonymous: RESIZE_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "shared")]
impl Default for RESIZE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: RESIZE_VIRTUAL_DISK_PARAMETERS_0_0,
}
#[cfg(feature = "shared")]
impl Default for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub NewSize: super::ULONGLONG,
}
pub type RESIZE_VIRTUAL_DISK_VERSION = i32;
pub const RESIZE_VIRTUAL_DISK_VERSION_1: RESIZE_VIRTUAL_DISK_VERSION = 1;
pub const RESIZE_VIRTUAL_DISK_VERSION_UNSPECIFIED: RESIZE_VIRTUAL_DISK_VERSION = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SET_VIRTUAL_DISK_INFO {
    pub Version: SET_VIRTUAL_DISK_INFO_VERSION,
    pub Anonymous: SET_VIRTUAL_DISK_INFO_0,
}
#[cfg(feature = "shared")]
impl Default for SET_VIRTUAL_DISK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union SET_VIRTUAL_DISK_INFO_0 {
    pub ParentFilePath: windows_core::PCWSTR,
    pub UniqueIdentifier: windows_core::GUID,
    pub ParentPathWithDepthInfo: SET_VIRTUAL_DISK_INFO_0_0,
    pub VhdPhysicalSectorSize: super::ULONG,
    pub VirtualDiskId: windows_core::GUID,
    pub ChangeTrackingEnabled: windows_core::BOOL,
    pub ParentLocator: SET_VIRTUAL_DISK_INFO_0_1,
}
#[cfg(feature = "shared")]
impl Default for SET_VIRTUAL_DISK_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_VIRTUAL_DISK_INFO_0_0 {
    pub ChildDepth: super::ULONG,
    pub ParentFilePath: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_VIRTUAL_DISK_INFO_0_1 {
    pub LinkageId: windows_core::GUID,
    pub ParentFilePath: windows_core::PCWSTR,
}
pub const SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE: SET_VIRTUAL_DISK_INFO_VERSION = 6;
pub const SET_VIRTUAL_DISK_INFO_IDENTIFIER: SET_VIRTUAL_DISK_INFO_VERSION = 2;
pub const SET_VIRTUAL_DISK_INFO_PARENT_LOCATOR: SET_VIRTUAL_DISK_INFO_VERSION = 7;
pub const SET_VIRTUAL_DISK_INFO_PARENT_PATH: SET_VIRTUAL_DISK_INFO_VERSION = 1;
pub const SET_VIRTUAL_DISK_INFO_PARENT_PATH_WITH_DEPTH: SET_VIRTUAL_DISK_INFO_VERSION = 3;
pub const SET_VIRTUAL_DISK_INFO_PHYSICAL_SECTOR_SIZE: SET_VIRTUAL_DISK_INFO_VERSION = 4;
pub const SET_VIRTUAL_DISK_INFO_UNSPECIFIED: SET_VIRTUAL_DISK_INFO_VERSION = 0;
pub type SET_VIRTUAL_DISK_INFO_VERSION = i32;
pub const SET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID: SET_VIRTUAL_DISK_INFO_VERSION = 5;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEPENDENCY_INFO_TYPE_1 {
    pub DependencyTypeFlags: DEPENDENT_DISK_FLAG,
    pub ProviderSpecificFlags: super::ULONG,
    pub VirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winioctl"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STORAGE_DEPENDENCY_INFO_TYPE_2 {
    pub DependencyTypeFlags: DEPENDENT_DISK_FLAG,
    pub ProviderSpecificFlags: super::ULONG,
    pub VirtualStorageType: super::VIRTUAL_STORAGE_TYPE,
    pub AncestorLevel: super::ULONG,
    pub DependencyDeviceName: windows_core::PWSTR,
    pub HostVolumeName: windows_core::PWSTR,
    pub DependentVolumeName: windows_core::PWSTR,
    pub DependentVolumeRelativePath: windows_core::PWSTR,
}
pub type STORAGE_DEPENDENCY_INFO_VERSION = i32;
pub const STORAGE_DEPENDENCY_INFO_VERSION_1: STORAGE_DEPENDENCY_INFO_VERSION = 1;
pub const STORAGE_DEPENDENCY_INFO_VERSION_2: STORAGE_DEPENDENCY_INFO_VERSION = 2;
pub const STORAGE_DEPENDENCY_INFO_VERSION_UNSPECIFIED: STORAGE_DEPENDENCY_INFO_VERSION = 0;
pub const SURFACE_VIRTUAL_DISK_FLAG_BYPASS_DEFAULT_ENCRYPTION_POLICY: ATTACH_VIRTUAL_DISK_FLAG = 32;
pub const SURFACE_VIRTUAL_DISK_FLAG_NONE: ATTACH_VIRTUAL_DISK_FLAG = 0;
pub const SURFACE_VIRTUAL_DISK_FLAG_NO_DRIVE_LETTER: ATTACH_VIRTUAL_DISK_FLAG = 2;
pub const SURFACE_VIRTUAL_DISK_FLAG_NO_LOCAL_HOST: ATTACH_VIRTUAL_DISK_FLAG = 8;
pub const SURFACE_VIRTUAL_DISK_FLAG_NO_SECURITY_DESCRIPTOR: ATTACH_VIRTUAL_DISK_FLAG = 16;
pub const SURFACE_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME: ATTACH_VIRTUAL_DISK_FLAG = 4;
pub const SURFACE_VIRTUAL_DISK_FLAG_READ_ONLY: ATTACH_VIRTUAL_DISK_FLAG = 1;
pub const SURFACE_VIRTUAL_DISK_VERSION_1: ATTACH_VIRTUAL_DISK_VERSION = 1;
pub const SURFACE_VIRTUAL_DISK_VERSION_UNSPECIFIED: ATTACH_VIRTUAL_DISK_VERSION = 0;
pub type TAKE_SNAPSHOT_VHDSET_FLAG = i32;
pub const TAKE_SNAPSHOT_VHDSET_FLAG_NONE: TAKE_SNAPSHOT_VHDSET_FLAG = 0;
pub const TAKE_SNAPSHOT_VHDSET_FLAG_WRITEABLE: TAKE_SNAPSHOT_VHDSET_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: TAKE_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: TAKE_SNAPSHOT_VHDSET_PARAMETERS_0,
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
impl Default for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: windows_core::GUID,
}
pub type TAKE_SNAPSHOT_VHDSET_VERSION = i32;
pub const TAKE_SNAPSHOT_VHDSET_VERSION_1: TAKE_SNAPSHOT_VHDSET_VERSION = 1;
pub const TAKE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: TAKE_SNAPSHOT_VHDSET_VERSION = 0;
pub const UNSURFACE_VIRTUAL_DISK_FLAG_NONE: DETACH_VIRTUAL_DISK_FLAG = 0;
pub const VIRTUAL_DISK_ACCESS_ALL: VIRTUAL_DISK_ACCESS_MASK = 4128768;
pub const VIRTUAL_DISK_ACCESS_ATTACH_RO: VIRTUAL_DISK_ACCESS_MASK = 65536;
pub const VIRTUAL_DISK_ACCESS_ATTACH_RW: VIRTUAL_DISK_ACCESS_MASK = 131072;
pub const VIRTUAL_DISK_ACCESS_CREATE: VIRTUAL_DISK_ACCESS_MASK = 1048576;
pub const VIRTUAL_DISK_ACCESS_DETACH: VIRTUAL_DISK_ACCESS_MASK = 262144;
pub const VIRTUAL_DISK_ACCESS_GET_INFO: VIRTUAL_DISK_ACCESS_MASK = 524288;
pub type VIRTUAL_DISK_ACCESS_MASK = i32;
pub const VIRTUAL_DISK_ACCESS_METAOPS: VIRTUAL_DISK_ACCESS_MASK = 2097152;
pub const VIRTUAL_DISK_ACCESS_NONE: VIRTUAL_DISK_ACCESS_MASK = 0;
pub const VIRTUAL_DISK_ACCESS_READ: VIRTUAL_DISK_ACCESS_MASK = 851968;
pub const VIRTUAL_DISK_ACCESS_SURFACE_RO: VIRTUAL_DISK_ACCESS_MASK = 65536;
pub const VIRTUAL_DISK_ACCESS_SURFACE_RW: VIRTUAL_DISK_ACCESS_MASK = 131072;
pub const VIRTUAL_DISK_ACCESS_UNSURFACE: VIRTUAL_DISK_ACCESS_MASK = 262144;
pub const VIRTUAL_DISK_ACCESS_WRITABLE: VIRTUAL_DISK_ACCESS_MASK = 3276800;
pub const VIRTUAL_DISK_MAXIMUM_CHANGE_TRACKING_ID_LENGTH: i32 = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_DISK_PROGRESS {
    pub OperationStatus: super::DWORD,
    pub CurrentValue: super::ULONGLONG,
    pub CompletionValue: super::ULONGLONG,
}
pub const VIRTUAL_STORAGE_TYPE_DEVICE_ISO: i32 = 1;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_UNKNOWN: i32 = 0;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHD: i32 = 2;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDSET: i32 = 4;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDX: i32 = 3;
pub const VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT: windows_core::GUID = windows_core::GUID::from_u128(0xec984aec_a0f9_47e9_901f_71415a66345b);
pub const VIRTUAL_STORAGE_TYPE_VENDOR_UNKNOWN: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
