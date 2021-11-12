#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DDP_FILE_EXTENT {
    pub Length: i64,
    pub Offset: i64,
}
impl ::core::marker::Copy for DDP_FILE_EXTENT {}
impl ::core::clone::Clone for DDP_FILE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DEDUP_BACKUP_SUPPORT_PARAM_TYPE(pub i32);
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(1i32);
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(2i32);
impl ::core::marker::Copy for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {}
impl ::core::clone::Clone for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[repr(C)]
pub struct DEDUP_CHUNK_INFO_HASH32 {
    pub ChunkFlags: u32,
    pub ChunkOffsetInStream: u64,
    pub ChunkSize: u64,
    pub HashVal: [u8; 32],
}
impl ::core::marker::Copy for DEDUP_CHUNK_INFO_HASH32 {}
impl ::core::clone::Clone for DEDUP_CHUNK_INFO_HASH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DEDUP_CONTAINER_EXTENT {
    pub ContainerIndex: u32,
    pub StartOffset: i64,
    pub Length: i64,
}
impl ::core::marker::Copy for DEDUP_CONTAINER_EXTENT {}
impl ::core::clone::Clone for DEDUP_CONTAINER_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DEDUP_SET_PARAM_TYPE(pub i32);
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(1i32);
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(2i32);
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(3i32);
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(4i32);
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(5i32);
impl ::core::marker::Copy for DEDUP_SET_PARAM_TYPE {}
impl ::core::clone::Clone for DEDUP_SET_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DedupBackupSupport: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1943450285,
    data2: 10628,
    data3: 18197,
    data4: [178, 227, 146, 76, 20, 151, 68, 221],
};
#[repr(C)]
pub struct DedupChunk {
    pub Hash: DedupHash,
    pub Flags: DedupChunkFlags,
    pub LogicalSize: u32,
    pub DataSize: u32,
}
impl ::core::marker::Copy for DedupChunk {}
impl ::core::clone::Clone for DedupChunk {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DedupChunkFlags(pub i32);
pub const DedupChunkFlags_None: DedupChunkFlags = DedupChunkFlags(0i32);
pub const DedupChunkFlags_Compressed: DedupChunkFlags = DedupChunkFlags(1i32);
impl ::core::marker::Copy for DedupChunkFlags {}
impl ::core::clone::Clone for DedupChunkFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DedupChunkingAlgorithm(pub i32);
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = DedupChunkingAlgorithm(0i32);
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = DedupChunkingAlgorithm(1i32);
impl ::core::marker::Copy for DedupChunkingAlgorithm {}
impl ::core::clone::Clone for DedupChunkingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DedupCompressionAlgorithm(pub i32);
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = DedupCompressionAlgorithm(0i32);
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = DedupCompressionAlgorithm(1i32);
impl ::core::marker::Copy for DedupCompressionAlgorithm {}
impl ::core::clone::Clone for DedupCompressionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DedupDataPort: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2400219655, data2: 6185, data3: 18610, data4: [166, 75, 230, 31, 142, 13, 154, 203] };
#[repr(transparent)]
pub struct DedupDataPortManagerOption(pub i32);
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = DedupDataPortManagerOption(0i32);
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = DedupDataPortManagerOption(1i32);
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = DedupDataPortManagerOption(2i32);
impl ::core::marker::Copy for DedupDataPortManagerOption {}
impl ::core::clone::Clone for DedupDataPortManagerOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DedupDataPortRequestStatus(pub i32);
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = DedupDataPortRequestStatus(0i32);
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = DedupDataPortRequestStatus(1i32);
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = DedupDataPortRequestStatus(2i32);
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = DedupDataPortRequestStatus(3i32);
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = DedupDataPortRequestStatus(4i32);
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = DedupDataPortRequestStatus(5i32);
impl ::core::marker::Copy for DedupDataPortRequestStatus {}
impl ::core::clone::Clone for DedupDataPortRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DedupDataPortVolumeStatus(pub i32);
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(0i32);
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(1i32);
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(2i32);
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(3i32);
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(4i32);
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(5i32);
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(6i32);
impl ::core::marker::Copy for DedupDataPortVolumeStatus {}
impl ::core::clone::Clone for DedupDataPortVolumeStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DedupHash {
    pub Hash: [u8; 32],
}
impl ::core::marker::Copy for DedupHash {}
impl ::core::clone::Clone for DedupHash {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DedupHashingAlgorithm(pub i32);
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = DedupHashingAlgorithm(0i32);
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = DedupHashingAlgorithm(1i32);
impl ::core::marker::Copy for DedupHashingAlgorithm {}
impl ::core::clone::Clone for DedupHashingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DedupStream {
    pub Path: super::super::Foundation::BSTR,
    pub Offset: u64,
    pub Length: u64,
    pub ChunkCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DedupStream {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DedupStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DedupStreamEntry {
    pub Hash: DedupHash,
    pub LogicalSize: u32,
    pub Offset: u64,
}
impl ::core::marker::Copy for DedupStreamEntry {}
impl ::core::clone::Clone for DedupStreamEntry {
    fn clone(&self) -> Self {
        *self
    }
}
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
