#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AddVirtualDiskParent();
    fn ApplySnapshotVhdSet();
    fn AttachVirtualDisk();
    fn BreakMirrorVirtualDisk();
    fn CompactVirtualDisk();
    fn CompleteForkVirtualDisk();
    fn CreateVirtualDisk();
    fn DeleteSnapshotVhdSet();
    fn DeleteVirtualDiskMetadata();
    fn DetachVirtualDisk();
    fn EnumerateVirtualDiskMetadata();
    fn ExpandVirtualDisk();
    fn ForkVirtualDisk();
    fn GetAllAttachedVirtualDiskPhysicalPaths();
    fn GetStorageDependencyInformation();
    fn GetVirtualDiskInformation();
    fn GetVirtualDiskMetadata();
    fn GetVirtualDiskOperationProgress();
    fn GetVirtualDiskPhysicalPath();
    fn MergeVirtualDisk();
    fn MirrorVirtualDisk();
    fn ModifyVhdSet();
    fn OpenVirtualDisk();
    fn QueryChangesVirtualDisk();
    fn RawSCSIVirtualDisk();
    fn ResizeVirtualDisk();
    fn SetVirtualDiskInformation();
    fn SetVirtualDiskMetadata();
    fn TakeSnapshotVhdSet();
}
