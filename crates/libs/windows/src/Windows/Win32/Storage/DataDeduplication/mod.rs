#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
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
impl ::core::fmt::Debug for DDP_FILE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDP_FILE_EXTENT").field("Length", &self.Length).field("Offset", &self.Offset).finish()
    }
}
unsafe impl ::windows::core::Abi for DDP_FILE_EXTENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DDP_FILE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DDP_FILE_EXTENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DDP_FILE_EXTENT {}
impl ::core::default::Default for DDP_FILE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEDUP_BACKUP_SUPPORT_PARAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(2i32);
impl ::core::marker::Copy for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {}
impl ::core::clone::Clone for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEDUP_BACKUP_SUPPORT_PARAM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
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
impl ::core::fmt::Debug for DEDUP_CHUNK_INFO_HASH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEDUP_CHUNK_INFO_HASH32").field("ChunkFlags", &self.ChunkFlags).field("ChunkOffsetInStream", &self.ChunkOffsetInStream).field("ChunkSize", &self.ChunkSize).field("HashVal", &self.HashVal).finish()
    }
}
unsafe impl ::windows::core::Abi for DEDUP_CHUNK_INFO_HASH32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEDUP_CHUNK_INFO_HASH32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEDUP_CHUNK_INFO_HASH32>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEDUP_CHUNK_INFO_HASH32 {}
impl ::core::default::Default for DEDUP_CHUNK_INFO_HASH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
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
impl ::core::fmt::Debug for DEDUP_CONTAINER_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEDUP_CONTAINER_EXTENT").field("ContainerIndex", &self.ContainerIndex).field("StartOffset", &self.StartOffset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for DEDUP_CONTAINER_EXTENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEDUP_CONTAINER_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEDUP_CONTAINER_EXTENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEDUP_CONTAINER_EXTENT {}
impl ::core::default::Default for DEDUP_CONTAINER_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEDUP_SET_PARAM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(5i32);
impl ::core::marker::Copy for DEDUP_SET_PARAM_TYPE {}
impl ::core::clone::Clone for DEDUP_SET_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEDUP_SET_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEDUP_SET_PARAM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEDUP_SET_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEDUP_SET_PARAM_TYPE").field(&self.0).finish()
    }
}
pub const DedupBackupSupport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73d6b2ad_2984_4715_b2e3_924c149744dd);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
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
impl ::core::fmt::Debug for DedupChunk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DedupChunk").field("Hash", &self.Hash).field("Flags", &self.Flags).field("LogicalSize", &self.LogicalSize).field("DataSize", &self.DataSize).finish()
    }
}
unsafe impl ::windows::core::Abi for DedupChunk {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DedupChunk {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DedupChunk>()) == 0 }
    }
}
impl ::core::cmp::Eq for DedupChunk {}
impl ::core::default::Default for DedupChunk {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DedupChunkFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkFlags_None: DedupChunkFlags = DedupChunkFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkFlags_Compressed: DedupChunkFlags = DedupChunkFlags(1i32);
impl ::core::marker::Copy for DedupChunkFlags {}
impl ::core::clone::Clone for DedupChunkFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DedupChunkFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DedupChunkFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for DedupChunkFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupChunkFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DedupChunkingAlgorithm(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = DedupChunkingAlgorithm(0i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = DedupChunkingAlgorithm(1i32);
impl ::core::marker::Copy for DedupChunkingAlgorithm {}
impl ::core::clone::Clone for DedupChunkingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DedupChunkingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DedupChunkingAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for DedupChunkingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupChunkingAlgorithm").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DedupCompressionAlgorithm(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = DedupCompressionAlgorithm(0i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = DedupCompressionAlgorithm(1i32);
impl ::core::marker::Copy for DedupCompressionAlgorithm {}
impl ::core::clone::Clone for DedupCompressionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DedupCompressionAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DedupCompressionAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for DedupCompressionAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupCompressionAlgorithm").field(&self.0).finish()
    }
}
pub const DedupDataPort: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f107207_1829_48b2_a64b_e61f8e0d9acb);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DedupDataPortManagerOption(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = DedupDataPortManagerOption(0i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = DedupDataPortManagerOption(1i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = DedupDataPortManagerOption(2i32);
impl ::core::marker::Copy for DedupDataPortManagerOption {}
impl ::core::clone::Clone for DedupDataPortManagerOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DedupDataPortManagerOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DedupDataPortManagerOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for DedupDataPortManagerOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupDataPortManagerOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DedupDataPortRequestStatus(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = DedupDataPortRequestStatus(0i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = DedupDataPortRequestStatus(1i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = DedupDataPortRequestStatus(2i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = DedupDataPortRequestStatus(3i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = DedupDataPortRequestStatus(4i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = DedupDataPortRequestStatus(5i32);
impl ::core::marker::Copy for DedupDataPortRequestStatus {}
impl ::core::clone::Clone for DedupDataPortRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DedupDataPortRequestStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DedupDataPortRequestStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DedupDataPortRequestStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupDataPortRequestStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DedupDataPortVolumeStatus(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(0i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(1i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(2i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(3i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(4i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(5i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(6i32);
impl ::core::marker::Copy for DedupDataPortVolumeStatus {}
impl ::core::clone::Clone for DedupDataPortVolumeStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DedupDataPortVolumeStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DedupDataPortVolumeStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DedupDataPortVolumeStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupDataPortVolumeStatus").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub struct DedupHash {
    pub Hash: [u8; 32],
}
impl ::core::marker::Copy for DedupHash {}
impl ::core::clone::Clone for DedupHash {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DedupHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DedupHash").field("Hash", &self.Hash).finish()
    }
}
unsafe impl ::windows::core::Abi for DedupHash {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DedupHash {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DedupHash>()) == 0 }
    }
}
impl ::core::cmp::Eq for DedupHash {}
impl ::core::default::Default for DedupHash {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DedupHashingAlgorithm(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = DedupHashingAlgorithm(0i32);
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = DedupHashingAlgorithm(1i32);
impl ::core::marker::Copy for DedupHashingAlgorithm {}
impl ::core::clone::Clone for DedupHashingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DedupHashingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DedupHashingAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for DedupHashingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DedupHashingAlgorithm").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DedupStream {
    pub Path: super::super::Foundation::BSTR,
    pub Offset: u64,
    pub Length: u64,
    pub ChunkCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DedupStream {
    fn clone(&self) -> Self {
        Self { Path: self.Path.clone(), Offset: self.Offset, Length: self.Length, ChunkCount: self.ChunkCount }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DedupStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DedupStream").field("Path", &self.Path).field("Offset", &self.Offset).field("Length", &self.Length).field("ChunkCount", &self.ChunkCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DedupStream {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DedupStream {
    fn eq(&self, other: &Self) -> bool {
        self.Path == other.Path && self.Offset == other.Offset && self.Length == other.Length && self.ChunkCount == other.ChunkCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DedupStream {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DedupStream {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
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
impl ::core::fmt::Debug for DedupStreamEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DedupStreamEntry").field("Hash", &self.Hash).field("LogicalSize", &self.LogicalSize).field("Offset", &self.Offset).finish()
    }
}
unsafe impl ::windows::core::Abi for DedupStreamEntry {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DedupStreamEntry {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DedupStreamEntry>()) == 0 }
    }
}
impl ::core::cmp::Eq for DedupStreamEntry {}
impl ::core::default::Default for DedupStreamEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
pub struct IDedupBackupSupport(::windows::core::IUnknown);
impl IDedupBackupSupport {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreFiles<'a, P0>(&self, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: P0, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDedupReadFileCallback>>,
    {
        (::windows::core::Interface::vtable(self).RestoreFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(numberoffiles), ::core::mem::transmute(filefullpaths), store.into().abi(), flags, ::core::mem::transmute(fileresults)).ok()
    }
}
impl ::core::convert::From<IDedupBackupSupport> for ::windows::core::IUnknown {
    fn from(value: IDedupBackupSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDedupBackupSupport> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDedupBackupSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupBackupSupport> for ::windows::core::IUnknown {
    fn from(value: &IDedupBackupSupport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDedupBackupSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IDedupBackupSupport {
    type Vtable = IDedupBackupSupport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc719d963_2b2d_415e_acf7_7eb7ca596ff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupBackupSupport_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RestoreFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: *mut ::core::ffi::c_void, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestoreFiles: usize,
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
pub struct IDedupChunkLibrary(::windows::core::IUnknown);
impl IDedupChunkLibrary {
    pub unsafe fn InitializeForPushBuffers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeForPushBuffers)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Uninitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Uninitialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetParameter<'a, P0>(&self, dwparamtype: u32, vparamvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetParameter)(::windows::core::Interface::as_raw(self), dwparamtype, vparamvalue.into().abi()).ok()
    }
    pub unsafe fn StartChunking(&self, iiditeratorinterfaceid: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartChunking)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(iiditeratorinterfaceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IDedupChunkLibrary> for ::windows::core::IUnknown {
    fn from(value: IDedupChunkLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDedupChunkLibrary> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDedupChunkLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupChunkLibrary> for ::windows::core::IUnknown {
    fn from(value: &IDedupChunkLibrary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDedupChunkLibrary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IDedupChunkLibrary {
    type Vtable = IDedupChunkLibrary_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb5144d7_2720_4dcc_8777_78597416ec23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupChunkLibrary_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub InitializeForPushBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwparamtype: u32, vparamvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetParameter: usize,
    pub StartChunking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iiditeratorinterfaceid: ::windows::core::GUID, ppchunksenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
pub struct IDedupDataPort(::windows::core::IUnknown);
impl IDedupDataPort {
    pub unsafe fn GetStatus(&self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatus), ::core::mem::transmute(pdataheadroommb)).ok()
    }
    pub unsafe fn LookupChunks(&self, phashes: &[DedupHash]) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LookupChunks)(::windows::core::Interface::as_raw(self), phashes.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(phashes)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn InsertChunks(&self, pchunkmetadata: &[DedupChunk], pchunkdata: &[u8]) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).InsertChunks)(::windows::core::Interface::as_raw(self), pchunkmetadata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pchunkmetadata)), pchunkdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pchunkdata)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertChunksWithStream<'a, P0>(&self, pchunkmetadata: &[DedupChunk], databytecount: u32, pchunkdatastream: P0) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).InsertChunksWithStream)(::windows::core::Interface::as_raw(self), pchunkmetadata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pchunkmetadata)), databytecount, pchunkdatastream.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CommitStreams(&self, pstreams: &[DedupStream], pentries: &[DedupStreamEntry]) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CommitStreams)(::windows::core::Interface::as_raw(self), pstreams.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pstreams)), pentries.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pentries)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CommitStreamsWithStream<'a, P0>(&self, pstreams: &[DedupStream], entrycount: u32, pentriesstream: P0) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CommitStreamsWithStream)(::windows::core::Interface::as_raw(self), pstreams.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pstreams)), entrycount, pentriesstream.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreams(&self, pstreampaths: &[super::super::Foundation::BSTR]) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetStreams)(::windows::core::Interface::as_raw(self), pstreampaths.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pstreampaths)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreamsResults(&self, requestid: ::windows::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamsResults)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(requestid), maxwaitms, streamentryindex, ::core::mem::transmute(pstreamcount), ::core::mem::transmute(ppstreams), ::core::mem::transmute(pentrycount), ::core::mem::transmute(ppentries), ::core::mem::transmute(pstatus), ::core::mem::transmute(ppitemresults)).ok()
    }
    pub unsafe fn GetChunks(&self, phashes: &[DedupHash]) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChunks)(::windows::core::Interface::as_raw(self), phashes.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(phashes)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetChunksResults(&self, requestid: ::windows::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChunksResults)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(requestid), maxwaitms, chunkindex, ::core::mem::transmute(pchunkcount), ::core::mem::transmute(ppchunkmetadata), ::core::mem::transmute(pdatabytecount), ::core::mem::transmute(ppchunkdata), ::core::mem::transmute(pstatus), ::core::mem::transmute(ppitemresults)).ok()
    }
    pub unsafe fn GetRequestStatus(&self, requestid: ::windows::core::GUID) -> ::windows::core::Result<DedupDataPortRequestStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRequestStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(requestid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DedupDataPortRequestStatus>(result__)
    }
    pub unsafe fn GetRequestResults(&self, requestid: ::windows::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRequestResults)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(requestid), maxwaitms, ::core::mem::transmute(pbatchresult), ::core::mem::transmute(pbatchcount), ::core::mem::transmute(pstatus), ::core::mem::transmute(ppitemresults)).ok()
    }
}
impl ::core::convert::From<IDedupDataPort> for ::windows::core::IUnknown {
    fn from(value: IDedupDataPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDedupDataPort> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDedupDataPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupDataPort> for ::windows::core::IUnknown {
    fn from(value: &IDedupDataPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDedupDataPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IDedupDataPort {
    type Vtable = IDedupDataPort_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7963d734_40a9_4ea3_bbf6_5a89d26f7ae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPort_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::HRESULT,
    pub LookupChunks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub InsertChunks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertChunksWithStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: *mut ::core::ffi::c_void, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertChunksWithStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CommitStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CommitStreams: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CommitStreamsWithStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: *mut ::core::ffi::c_void, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CommitStreamsWithStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStreams: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStreamsResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStreamsResults: usize,
    pub GetChunks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetChunksResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetRequestStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows::core::HRESULT,
    pub GetRequestResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
pub struct IDedupDataPortManager(::windows::core::IUnknown);
impl IDedupDataPortManager {
    pub unsafe fn GetConfiguration(&self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConfiguration)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pminchunksize), ::core::mem::transmute(pmaxchunksize), ::core::mem::transmute(pchunkingalgorithm), ::core::mem::transmute(phashingalgorithm), ::core::mem::transmute(pcompressionalgorithm)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeStatus<'a, P0>(&self, options: u32, path: P0) -> ::windows::core::Result<DedupDataPortVolumeStatus>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetVolumeStatus)(::windows::core::Interface::as_raw(self), options, path.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DedupDataPortVolumeStatus>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeDataPort<'a, P0>(&self, options: u32, path: P0) -> ::windows::core::Result<IDedupDataPort>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetVolumeDataPort)(::windows::core::Interface::as_raw(self), options, path.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDedupDataPort>(result__)
    }
}
impl ::core::convert::From<IDedupDataPortManager> for ::windows::core::IUnknown {
    fn from(value: IDedupDataPortManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDedupDataPortManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDedupDataPortManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupDataPortManager> for ::windows::core::IUnknown {
    fn from(value: &IDedupDataPortManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDedupDataPortManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IDedupDataPortManager {
    type Vtable = IDedupDataPortManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44677452_b90a_445e_8192_cdcfe81511fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPortManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVolumeStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVolumeStatus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVolumeDataPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdataport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVolumeDataPort: usize,
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
pub struct IDedupIterateChunksHash32(::windows::core::IUnknown);
impl IDedupIterateChunksHash32 {
    pub unsafe fn PushBuffer(&self, pbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PushBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbuffer)), pbuffer.len() as _).ok()
    }
    pub unsafe fn Next(&self, parrchunks: &mut [DEDUP_CHUNK_INFO_HASH32], pulfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), parrchunks.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(parrchunks)), ::core::mem::transmute(pulfetched)).ok()
    }
    pub unsafe fn Drain(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Drain)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDedupIterateChunksHash32> for ::windows::core::IUnknown {
    fn from(value: IDedupIterateChunksHash32) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDedupIterateChunksHash32> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDedupIterateChunksHash32) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupIterateChunksHash32> for ::windows::core::IUnknown {
    fn from(value: &IDedupIterateChunksHash32) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDedupIterateChunksHash32 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IDedupIterateChunksHash32 {
    type Vtable = IDedupIterateChunksHash32_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90b584d3_72aa_400f_9767_cad866a5a2d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupIterateChunksHash32_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub PushBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Drain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_DataDeduplication\"`*"]
#[repr(transparent)]
pub struct IDedupReadFileCallback(::windows::core::IUnknown);
impl IDedupReadFileCallback {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadBackupFile<'a, P0>(&self, filefullpath: P0, fileoffset: i64, filebuffer: &mut [u8], returnedsize: *mut u32, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ReadBackupFile)(::windows::core::Interface::as_raw(self), filefullpath.into().abi(), fileoffset, filebuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(filebuffer)), ::core::mem::transmute(returnedsize), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OrderContainersRestore(&self, containerpaths: &[super::super::Foundation::BSTR], readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OrderContainersRestore)(::windows::core::Interface::as_raw(self), containerpaths.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(containerpaths)), ::core::mem::transmute(readplanentries), ::core::mem::transmute(readplan)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreviewContainerRead<'a, P0>(&self, filefullpath: P0, readoffsets: &[DDP_FILE_EXTENT]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).PreviewContainerRead)(::windows::core::Interface::as_raw(self), filefullpath.into().abi(), readoffsets.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(readoffsets))).ok()
    }
}
impl ::core::convert::From<IDedupReadFileCallback> for ::windows::core::IUnknown {
    fn from(value: IDedupReadFileCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDedupReadFileCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDedupReadFileCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupReadFileCallback> for ::windows::core::IUnknown {
    fn from(value: &IDedupReadFileCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDedupReadFileCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IDedupReadFileCallback {
    type Vtable = IDedupReadFileCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bacc67a_2f1d_42d0_897e_6ff62dd533bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupReadFileCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReadBackupFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReadBackupFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OrderContainersRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OrderContainersRestore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreviewContainerRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreviewContainerRead: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
