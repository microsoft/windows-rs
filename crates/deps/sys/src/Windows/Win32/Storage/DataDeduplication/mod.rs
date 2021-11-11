#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DDP_FILE_EXTENT();
    fn DEDUP_BACKUP_SUPPORT_PARAM_TYPE();
    fn DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM();
    fn DEDUP_CHUNK_INFO_HASH32();
    fn DEDUP_CONTAINER_EXTENT();
    fn DEDUP_SET_PARAM_TYPE();
    fn DedupBackupSupport();
    fn DedupChunk();
    fn DedupChunkFlags();
    fn DedupChunkingAlgorithm();
    fn DedupCompressionAlgorithm();
    fn DedupDataPort();
    fn DedupDataPortManagerOption();
    fn DedupDataPortRequestStatus();
    fn DedupDataPortVolumeStatus();
    fn DedupHash();
    fn DedupHashingAlgorithm();
    fn DedupStream();
    fn DedupStreamEntry();
    fn IDedupBackupSupport();
    fn IDedupChunkLibrary();
    fn IDedupDataPort();
    fn IDedupDataPortManager();
    fn IDedupIterateChunksHash32();
    fn IDedupReadFileCallback();
}
