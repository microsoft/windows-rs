#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DDP_FILE_EXTENT(i32);
#[repr(transparent)]
pub struct DEDUP_BACKUP_SUPPORT_PARAM_TYPE(pub i32);
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(1i32);
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(2i32);
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[repr(C)]
pub struct DEDUP_CHUNK_INFO_HASH32(i32);
#[repr(C)]
pub struct DEDUP_CONTAINER_EXTENT(i32);
#[repr(transparent)]
pub struct DEDUP_SET_PARAM_TYPE(pub i32);
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(1i32);
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(2i32);
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(3i32);
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(4i32);
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(5i32);
#[repr(C)]
pub struct DedupBackupSupport(i32);
#[repr(C)]
pub struct DedupChunk(i32);
#[repr(transparent)]
pub struct DedupChunkFlags(pub i32);
pub const DedupChunkFlags_None: DedupChunkFlags = DedupChunkFlags(0i32);
pub const DedupChunkFlags_Compressed: DedupChunkFlags = DedupChunkFlags(1i32);
#[repr(transparent)]
pub struct DedupChunkingAlgorithm(pub i32);
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = DedupChunkingAlgorithm(0i32);
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = DedupChunkingAlgorithm(1i32);
#[repr(transparent)]
pub struct DedupCompressionAlgorithm(pub i32);
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = DedupCompressionAlgorithm(0i32);
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = DedupCompressionAlgorithm(1i32);
#[repr(C)]
pub struct DedupDataPort(i32);
#[repr(transparent)]
pub struct DedupDataPortManagerOption(pub i32);
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = DedupDataPortManagerOption(0i32);
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = DedupDataPortManagerOption(1i32);
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = DedupDataPortManagerOption(2i32);
#[repr(transparent)]
pub struct DedupDataPortRequestStatus(pub i32);
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = DedupDataPortRequestStatus(0i32);
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = DedupDataPortRequestStatus(1i32);
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = DedupDataPortRequestStatus(2i32);
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = DedupDataPortRequestStatus(3i32);
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = DedupDataPortRequestStatus(4i32);
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = DedupDataPortRequestStatus(5i32);
#[repr(transparent)]
pub struct DedupDataPortVolumeStatus(pub i32);
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(0i32);
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(1i32);
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(2i32);
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(3i32);
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(4i32);
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(5i32);
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(6i32);
#[repr(C)]
pub struct DedupHash(i32);
#[repr(transparent)]
pub struct DedupHashingAlgorithm(pub i32);
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = DedupHashingAlgorithm(0i32);
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = DedupHashingAlgorithm(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DedupStream(i32);
#[repr(C)]
pub struct DedupStreamEntry(i32);
#[repr(transparent)]
pub struct IDedupBackupSupport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDedupChunkLibrary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDedupDataPort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDedupDataPortManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDedupIterateChunksHash32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDedupReadFileCallback(pub *mut ::core::ffi::c_void);
