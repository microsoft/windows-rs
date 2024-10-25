pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 3i32;
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = 5i32;
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = 4i32;
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 2i32;
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 1i32;
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = 2i32;
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = 1i32;
pub const DedupChunkFlags_Compressed: DedupChunkFlags = 1i32;
pub const DedupChunkFlags_None: DedupChunkFlags = 0i32;
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = 0i32;
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = 1i32;
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = 0i32;
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = 1i32;
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = 1i32;
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = 0i32;
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = 2i32;
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = 4i32;
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = 5i32;
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = 3i32;
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = 2i32;
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = 1i32;
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = 0i32;
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = 3i32;
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = 5i32;
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = 2i32;
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = 1i32;
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = 4i32;
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = 6i32;
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = 0i32;
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = 0i32;
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEDUP_BACKUP_SUPPORT_PARAM_TYPE(pub i32);
impl windows_core::TypeKind for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEDUP_SET_PARAM_TYPE(pub i32);
impl windows_core::TypeKind for DEDUP_SET_PARAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DedupChunkFlags(pub i32);
impl windows_core::TypeKind for DedupChunkFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DedupChunkingAlgorithm(pub i32);
impl windows_core::TypeKind for DedupChunkingAlgorithm {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DedupCompressionAlgorithm(pub i32);
impl windows_core::TypeKind for DedupCompressionAlgorithm {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DedupDataPortManagerOption(pub i32);
impl windows_core::TypeKind for DedupDataPortManagerOption {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DedupDataPortRequestStatus(pub i32);
impl windows_core::TypeKind for DedupDataPortRequestStatus {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DedupDataPortVolumeStatus(pub i32);
impl windows_core::TypeKind for DedupDataPortVolumeStatus {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DedupHashingAlgorithm(pub i32);
impl windows_core::TypeKind for DedupHashingAlgorithm {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DDP_FILE_EXTENT {
    pub Length: i64,
    pub Offset: i64,
}
impl Default for DDP_FILE_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DDP_FILE_EXTENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEDUP_CHUNK_INFO_HASH32 {
    pub ChunkFlags: u32,
    pub ChunkOffsetInStream: u64,
    pub ChunkSize: u64,
    pub HashVal: [u8; 32],
}
impl Default for DEDUP_CHUNK_INFO_HASH32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DEDUP_CHUNK_INFO_HASH32 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEDUP_CONTAINER_EXTENT {
    pub ContainerIndex: u32,
    pub StartOffset: i64,
    pub Length: i64,
}
impl Default for DEDUP_CONTAINER_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DEDUP_CONTAINER_EXTENT {
    type TypeKind = windows_core::CopyType;
}
pub const DedupBackupSupport: windows_core::GUID = windows_core::GUID::from_u128(0x73d6b2ad_2984_4715_b2e3_924c149744dd);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DedupChunk {
    pub Hash: DedupHash,
    pub Flags: DedupChunkFlags,
    pub LogicalSize: u32,
    pub DataSize: u32,
}
impl Default for DedupChunk {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DedupChunk {
    type TypeKind = windows_core::CloneType;
}
pub const DedupDataPort: windows_core::GUID = windows_core::GUID::from_u128(0x8f107207_1829_48b2_a64b_e61f8e0d9acb);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DedupHash {
    pub Hash: [u8; 32],
}
impl Default for DedupHash {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DedupHash {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DedupStream {
    pub Path: windows_core::BSTR,
    pub Offset: u64,
    pub Length: u64,
    pub ChunkCount: u32,
}
impl Default for DedupStream {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DedupStream {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DedupStreamEntry {
    pub Hash: DedupHash,
    pub LogicalSize: u32,
    pub Offset: u64,
}
impl Default for DedupStreamEntry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DedupStreamEntry {
    type TypeKind = windows_core::CloneType;
}
