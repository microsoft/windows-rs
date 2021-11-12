#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DDP_FILE_EXTENT(i32);
#[repr(C)]
pub struct DEDUP_BACKUP_SUPPORT_PARAM_TYPE(i32);
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[repr(C)]
pub struct DEDUP_CHUNK_INFO_HASH32(i32);
#[repr(C)]
pub struct DEDUP_CONTAINER_EXTENT(i32);
#[repr(C)]
pub struct DEDUP_SET_PARAM_TYPE(i32);
#[repr(C)]
pub struct DedupBackupSupport(i32);
#[repr(C)]
pub struct DedupChunk(i32);
#[repr(C)]
pub struct DedupChunkFlags(i32);
#[repr(C)]
pub struct DedupChunkingAlgorithm(i32);
#[repr(C)]
pub struct DedupCompressionAlgorithm(i32);
#[repr(C)]
pub struct DedupDataPort(i32);
#[repr(C)]
pub struct DedupDataPortManagerOption(i32);
#[repr(C)]
pub struct DedupDataPortRequestStatus(i32);
#[repr(C)]
pub struct DedupDataPortVolumeStatus(i32);
#[repr(C)]
pub struct DedupHash(i32);
#[repr(C)]
pub struct DedupHashingAlgorithm(i32);
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
