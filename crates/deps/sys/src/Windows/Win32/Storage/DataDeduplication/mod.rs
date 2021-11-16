#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: i32 = 1i32;
pub const DEDUP_RECONSTRUCT_OPTIMIZED: i32 = 2i32;
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
pub const DEDUP_PT_MinChunkSizeBytes: i32 = 1i32;
pub const DEDUP_PT_MaxChunkSizeBytes: i32 = 2i32;
pub const DEDUP_PT_AvgChunkSizeBytes: i32 = 3i32;
pub const DEDUP_PT_InvariantChunking: i32 = 4i32;
pub const DEDUP_PT_DisableStrongHashComputation: i32 = 5i32;
pub const DedupBackupSupport: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
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
pub const DedupChunkFlags_None: i32 = 0i32;
pub const DedupChunkFlags_Compressed: i32 = 1i32;
pub const DedupChunkingAlgorithm_Unknonwn: i32 = 0i32;
pub const DedupChunkingAlgorithm_V1: i32 = 1i32;
pub const DedupCompressionAlgorithm_Unknonwn: i32 = 0i32;
pub const DedupCompressionAlgorithm_Xpress: i32 = 1i32;
pub const DedupDataPort: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2400219655, data2: 6185, data3: 18610, data4: [166, 75, 230, 31, 142, 13, 154, 203] };
pub const DedupDataPortManagerOption_None: i32 = 0i32;
pub const DedupDataPortManagerOption_AutoStart: i32 = 1i32;
pub const DedupDataPortManagerOption_SkipReconciliation: i32 = 2i32;
pub const DedupDataPortRequestStatus_Unknown: i32 = 0i32;
pub const DedupDataPortRequestStatus_Queued: i32 = 1i32;
pub const DedupDataPortRequestStatus_Processing: i32 = 2i32;
pub const DedupDataPortRequestStatus_Partial: i32 = 3i32;
pub const DedupDataPortRequestStatus_Complete: i32 = 4i32;
pub const DedupDataPortRequestStatus_Failed: i32 = 5i32;
pub const DedupDataPortVolumeStatus_Unknown: i32 = 0i32;
pub const DedupDataPortVolumeStatus_NotEnabled: i32 = 1i32;
pub const DedupDataPortVolumeStatus_NotAvailable: i32 = 2i32;
pub const DedupDataPortVolumeStatus_Initializing: i32 = 3i32;
pub const DedupDataPortVolumeStatus_Ready: i32 = 4i32;
pub const DedupDataPortVolumeStatus_Maintenance: i32 = 5i32;
pub const DedupDataPortVolumeStatus_Shutdown: i32 = 6i32;
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
pub const DedupHashingAlgorithm_Unknonwn: i32 = 0i32;
pub const DedupHashingAlgorithm_V1: i32 = 1i32;
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
impl ::core::marker::Copy for IDedupBackupSupport {}
impl ::core::clone::Clone for IDedupBackupSupport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDedupChunkLibrary(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDedupChunkLibrary {}
impl ::core::clone::Clone for IDedupChunkLibrary {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDedupDataPort(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDedupDataPort {}
impl ::core::clone::Clone for IDedupDataPort {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDedupDataPortManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDedupDataPortManager {}
impl ::core::clone::Clone for IDedupDataPortManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDedupIterateChunksHash32(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDedupIterateChunksHash32 {}
impl ::core::clone::Clone for IDedupIterateChunksHash32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDedupReadFileCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDedupReadFileCallback {}
impl ::core::clone::Clone for IDedupReadFileCallback {
    fn clone(&self) -> Self {
        *self
    }
}
