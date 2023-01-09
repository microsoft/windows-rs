impl ::core::default::Default for DDP_FILE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDP_FILE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for DDP_FILE_EXTENT {}
impl ::core::fmt::Debug for DDP_FILE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDP_FILE_EXTENT").field("Length", &self.Length).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEDUP_BACKUP_SUPPORT_PARAM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEDUP_CHUNK_INFO_HASH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEDUP_CHUNK_INFO_HASH32 {
    fn eq(&self, other: &Self) -> bool {
        self.ChunkFlags == other.ChunkFlags && self.ChunkOffsetInStream == other.ChunkOffsetInStream && self.ChunkSize == other.ChunkSize && self.HashVal == other.HashVal
    }
}
impl ::core::cmp::Eq for DEDUP_CHUNK_INFO_HASH32 {}
impl ::core::fmt::Debug for DEDUP_CHUNK_INFO_HASH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEDUP_CHUNK_INFO_HASH32").field("ChunkFlags", &self.ChunkFlags).field("ChunkOffsetInStream", &self.ChunkOffsetInStream).field("ChunkSize", &self.ChunkSize).field("HashVal", &self.HashVal).finish()
    }
}
impl ::core::default::Default for DEDUP_CONTAINER_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEDUP_CONTAINER_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.ContainerIndex == other.ContainerIndex && self.StartOffset == other.StartOffset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for DEDUP_CONTAINER_EXTENT {}
impl ::core::fmt::Debug for DEDUP_CONTAINER_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEDUP_CONTAINER_EXTENT").field("ContainerIndex", &self.ContainerIndex).field("StartOffset", &self.StartOffset).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for DEDUP_SET_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEDUP_SET_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEDUP_SET_PARAM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DedupChunk {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DedupChunk {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash && self.Flags == other.Flags && self.LogicalSize == other.LogicalSize && self.DataSize == other.DataSize
    }
}
impl ::core::cmp::Eq for DedupChunk {}
impl ::core::fmt::Debug for DedupChunk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DedupChunk").field("Hash", &self.Hash).field("Flags", &self.Flags).field("LogicalSize", &self.LogicalSize).field("DataSize", &self.DataSize).finish()
    }
}
impl ::core::default::Default for DedupChunkFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DedupChunkFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupChunkFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for DedupChunkingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DedupChunkingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupChunkingAlgorithm").field(&self.0).finish()
    }
}
impl ::core::default::Default for DedupCompressionAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DedupCompressionAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupCompressionAlgorithm").field(&self.0).finish()
    }
}
impl ::core::default::Default for DedupDataPortManagerOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DedupDataPortManagerOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupDataPortManagerOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for DedupDataPortRequestStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DedupDataPortRequestStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupDataPortRequestStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for DedupDataPortVolumeStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DedupDataPortVolumeStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupDataPortVolumeStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for DedupHash {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DedupHash {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash
    }
}
impl ::core::cmp::Eq for DedupHash {}
impl ::core::fmt::Debug for DedupHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DedupHash").field("Hash", &self.Hash).finish()
    }
}
impl ::core::default::Default for DedupHashingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DedupHashingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupHashingAlgorithm").field(&self.0).finish()
    }
}
impl ::core::default::Default for DedupStream {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DedupStream {
    fn eq(&self, other: &Self) -> bool {
        self.Path == other.Path && self.Offset == other.Offset && self.Length == other.Length && self.ChunkCount == other.ChunkCount
    }
}
impl ::core::cmp::Eq for DedupStream {}
impl ::core::fmt::Debug for DedupStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DedupStream").field("Path", &self.Path).field("Offset", &self.Offset).field("Length", &self.Length).field("ChunkCount", &self.ChunkCount).finish()
    }
}
impl ::core::default::Default for DedupStreamEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DedupStreamEntry {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash && self.LogicalSize == other.LogicalSize && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for DedupStreamEntry {}
impl ::core::fmt::Debug for DedupStreamEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DedupStreamEntry").field("Hash", &self.Hash).field("LogicalSize", &self.LogicalSize).field("Offset", &self.Offset).finish()
    }
}
impl ::core::cmp::PartialEq for IDedupBackupSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDedupBackupSupport {}
impl ::core::fmt::Debug for IDedupBackupSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDedupBackupSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDedupChunkLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDedupChunkLibrary {}
impl ::core::fmt::Debug for IDedupChunkLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDedupChunkLibrary").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDedupDataPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDedupDataPort {}
impl ::core::fmt::Debug for IDedupDataPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDedupDataPort").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDedupDataPortManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDedupDataPortManager {}
impl ::core::fmt::Debug for IDedupDataPortManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDedupDataPortManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDedupIterateChunksHash32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDedupIterateChunksHash32 {}
impl ::core::fmt::Debug for IDedupIterateChunksHash32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDedupIterateChunksHash32").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDedupReadFileCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDedupReadFileCallback {}
impl ::core::fmt::Debug for IDedupReadFileCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDedupReadFileCallback").field(&self.0).finish()
    }
}
