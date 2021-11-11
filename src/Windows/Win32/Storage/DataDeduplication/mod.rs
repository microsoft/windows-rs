#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
pub struct DDP_FILE_EXTENT {
    pub Length: i64,
    pub Offset: i64,
}
impl DDP_FILE_EXTENT {}
impl ::core::default::Default for DDP_FILE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DDP_FILE_EXTENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DDP_FILE_EXTENT").field("Length", &self.Length).field("Offset", &self.Offset).finish()
    }
}
impl ::core::cmp::PartialEq for DDP_FILE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for DDP_FILE_EXTENT {}
unsafe impl ::windows::runtime::Abi for DDP_FILE_EXTENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEDUP_BACKUP_SUPPORT_PARAM_TYPE(pub i32);
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(1i32);
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(2i32);
impl ::core::convert::From<i32> for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
pub struct DEDUP_CHUNK_INFO_HASH32 {
    pub ChunkFlags: u32,
    pub ChunkOffsetInStream: u64,
    pub ChunkSize: u64,
    pub HashVal: [u8; 32],
}
impl DEDUP_CHUNK_INFO_HASH32 {}
impl ::core::default::Default for DEDUP_CHUNK_INFO_HASH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DEDUP_CHUNK_INFO_HASH32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DEDUP_CHUNK_INFO_HASH32").field("ChunkFlags", &self.ChunkFlags).field("ChunkOffsetInStream", &self.ChunkOffsetInStream).field("ChunkSize", &self.ChunkSize).field("HashVal", &self.HashVal).finish()
    }
}
impl ::core::cmp::PartialEq for DEDUP_CHUNK_INFO_HASH32 {
    fn eq(&self, other: &Self) -> bool {
        self.ChunkFlags == other.ChunkFlags && self.ChunkOffsetInStream == other.ChunkOffsetInStream && self.ChunkSize == other.ChunkSize && self.HashVal == other.HashVal
    }
}
impl ::core::cmp::Eq for DEDUP_CHUNK_INFO_HASH32 {}
unsafe impl ::windows::runtime::Abi for DEDUP_CHUNK_INFO_HASH32 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
pub struct DEDUP_CONTAINER_EXTENT {
    pub ContainerIndex: u32,
    pub StartOffset: i64,
    pub Length: i64,
}
impl DEDUP_CONTAINER_EXTENT {}
impl ::core::default::Default for DEDUP_CONTAINER_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DEDUP_CONTAINER_EXTENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DEDUP_CONTAINER_EXTENT").field("ContainerIndex", &self.ContainerIndex).field("StartOffset", &self.StartOffset).field("Length", &self.Length).finish()
    }
}
impl ::core::cmp::PartialEq for DEDUP_CONTAINER_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.ContainerIndex == other.ContainerIndex && self.StartOffset == other.StartOffset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for DEDUP_CONTAINER_EXTENT {}
unsafe impl ::windows::runtime::Abi for DEDUP_CONTAINER_EXTENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEDUP_SET_PARAM_TYPE(pub i32);
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(1i32);
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(2i32);
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(3i32);
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(4i32);
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(5i32);
impl ::core::convert::From<i32> for DEDUP_SET_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEDUP_SET_PARAM_TYPE {
    type Abi = Self;
}
pub const DedupBackupSupport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x73d6b2ad_2984_4715_b2e3_924c149744dd);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
pub struct DedupChunk {
    pub Hash: DedupHash,
    pub Flags: DedupChunkFlags,
    pub LogicalSize: u32,
    pub DataSize: u32,
}
impl DedupChunk {}
impl ::core::default::Default for DedupChunk {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DedupChunk {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DedupChunk").field("Hash", &self.Hash).field("Flags", &self.Flags).field("LogicalSize", &self.LogicalSize).field("DataSize", &self.DataSize).finish()
    }
}
impl ::core::cmp::PartialEq for DedupChunk {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash && self.Flags == other.Flags && self.LogicalSize == other.LogicalSize && self.DataSize == other.DataSize
    }
}
impl ::core::cmp::Eq for DedupChunk {}
unsafe impl ::windows::runtime::Abi for DedupChunk {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupChunkFlags(pub i32);
pub const DedupChunkFlags_None: DedupChunkFlags = DedupChunkFlags(0i32);
pub const DedupChunkFlags_Compressed: DedupChunkFlags = DedupChunkFlags(1i32);
impl ::core::convert::From<i32> for DedupChunkFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupChunkFlags {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupChunkingAlgorithm(pub i32);
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = DedupChunkingAlgorithm(0i32);
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = DedupChunkingAlgorithm(1i32);
impl ::core::convert::From<i32> for DedupChunkingAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupChunkingAlgorithm {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupCompressionAlgorithm(pub i32);
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = DedupCompressionAlgorithm(0i32);
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = DedupCompressionAlgorithm(1i32);
impl ::core::convert::From<i32> for DedupCompressionAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupCompressionAlgorithm {
    type Abi = Self;
}
pub const DedupDataPort: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8f107207_1829_48b2_a64b_e61f8e0d9acb);
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupDataPortManagerOption(pub i32);
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = DedupDataPortManagerOption(0i32);
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = DedupDataPortManagerOption(1i32);
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = DedupDataPortManagerOption(2i32);
impl ::core::convert::From<i32> for DedupDataPortManagerOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupDataPortManagerOption {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupDataPortRequestStatus(pub i32);
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = DedupDataPortRequestStatus(0i32);
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = DedupDataPortRequestStatus(1i32);
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = DedupDataPortRequestStatus(2i32);
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = DedupDataPortRequestStatus(3i32);
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = DedupDataPortRequestStatus(4i32);
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = DedupDataPortRequestStatus(5i32);
impl ::core::convert::From<i32> for DedupDataPortRequestStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupDataPortRequestStatus {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupDataPortVolumeStatus(pub i32);
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(0i32);
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(1i32);
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(2i32);
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(3i32);
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(4i32);
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(5i32);
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(6i32);
impl ::core::convert::From<i32> for DedupDataPortVolumeStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupDataPortVolumeStatus {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
pub struct DedupHash {
    pub Hash: [u8; 32],
}
impl DedupHash {}
impl ::core::default::Default for DedupHash {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DedupHash {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DedupHash").field("Hash", &self.Hash).finish()
    }
}
impl ::core::cmp::PartialEq for DedupHash {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash
    }
}
impl ::core::cmp::Eq for DedupHash {}
unsafe impl ::windows::runtime::Abi for DedupHash {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupHashingAlgorithm(pub i32);
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = DedupHashingAlgorithm(0i32);
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = DedupHashingAlgorithm(1i32);
impl ::core::convert::From<i32> for DedupHashingAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupHashingAlgorithm {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
pub struct DedupStream {
    pub Path: super::super::Foundation::BSTR,
    pub Offset: u64,
    pub Length: u64,
    pub ChunkCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DedupStream {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DedupStream {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DedupStream {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DedupStream").field("Path", &self.Path).field("Offset", &self.Offset).field("Length", &self.Length).field("ChunkCount", &self.ChunkCount).finish()
    }
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
unsafe impl ::windows::runtime::Abi for DedupStream {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
pub struct DedupStreamEntry {
    pub Hash: DedupHash,
    pub LogicalSize: u32,
    pub Offset: u64,
}
impl DedupStreamEntry {}
impl ::core::default::Default for DedupStreamEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DedupStreamEntry {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DedupStreamEntry").field("Hash", &self.Hash).field("LogicalSize", &self.LogicalSize).field("Offset", &self.Offset).finish()
    }
}
impl ::core::cmp::PartialEq for DedupStreamEntry {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash && self.LogicalSize == other.LogicalSize && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for DedupStreamEntry {}
unsafe impl ::windows::runtime::Abi for DedupStreamEntry {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDedupBackupSupport(pub ::windows::runtime::IUnknown);
impl IDedupBackupSupport {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn RestoreFiles<'a, Param2: ::windows::runtime::IntoParam<'a, IDedupReadFileCallback>>(&self, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: Param2, flags: u32, fileresults: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(numberoffiles), ::core::mem::transmute(filefullpaths), store.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(fileresults)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDedupBackupSupport {
    type Vtable = IDedupBackupSupport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc719d963_2b2d_415e_acf7_7eb7ca596ff4);
}
impl ::core::convert::From<IDedupBackupSupport> for ::windows::runtime::IUnknown {
    fn from(value: IDedupBackupSupport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDedupBackupSupport> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupBackupSupport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupBackupSupport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDedupBackupSupport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupBackupSupport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, numberoffiles: u32, filefullpaths: *const ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, store: ::windows::runtime::RawPtr, flags: u32, fileresults: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDedupChunkLibrary(pub ::windows::runtime::IUnknown);
impl IDedupChunkLibrary {
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn InitializeForPushBuffers(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn Uninitialize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetParameter<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, dwparamtype: u32, vparamvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwparamtype), vparamvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn StartChunking<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, iiditeratorinterfaceid: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), iiditeratorinterfaceid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDedupChunkLibrary {
    type Vtable = IDedupChunkLibrary_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbb5144d7_2720_4dcc_8777_78597416ec23);
}
impl ::core::convert::From<IDedupChunkLibrary> for ::windows::runtime::IUnknown {
    fn from(value: IDedupChunkLibrary) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDedupChunkLibrary> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupChunkLibrary) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupChunkLibrary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDedupChunkLibrary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupChunkLibrary_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwparamtype: u32, vparamvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iiditeratorinterfaceid: ::windows::runtime::GUID, ppchunksenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDedupDataPort(pub ::windows::runtime::IUnknown);
impl IDedupDataPort {
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn GetStatus(&self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatus), ::core::mem::transmute(pdataheadroommb)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn LookupChunks(&self, count: u32, phashes: *const DedupHash) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(phashes), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn InsertChunks(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(chunkcount), ::core::mem::transmute(pchunkmetadata), ::core::mem::transmute(databytecount), ::core::mem::transmute(pchunkdata), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_System_Com`*"]
    pub unsafe fn InsertChunksWithStream<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: Param3) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(chunkcount), ::core::mem::transmute(pchunkmetadata), ::core::mem::transmute(databytecount), pchunkdatastream.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn CommitStreams(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamcount), ::core::mem::transmute(pstreams), ::core::mem::transmute(entrycount), ::core::mem::transmute(pentries), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CommitStreamsWithStream<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: Param3) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamcount), ::core::mem::transmute(pstreams), ::core::mem::transmute(entrycount), pentriesstream.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn GetStreams(&self, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamcount), ::core::mem::transmute(pstreampaths), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn GetStreamsResults<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, requestid: Param0, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::core::mem::transmute_copy(self),
            requestid.into_param().abi(),
            ::core::mem::transmute(maxwaitms),
            ::core::mem::transmute(streamentryindex),
            ::core::mem::transmute(pstreamcount),
            ::core::mem::transmute(ppstreams),
            ::core::mem::transmute(pentrycount),
            ::core::mem::transmute(ppentries),
            ::core::mem::transmute(pstatus),
            ::core::mem::transmute(ppitemresults),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn GetChunks(&self, count: u32, phashes: *const DedupHash) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(phashes), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn GetChunksResults<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, requestid: Param0, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::core::mem::transmute_copy(self),
            requestid.into_param().abi(),
            ::core::mem::transmute(maxwaitms),
            ::core::mem::transmute(chunkindex),
            ::core::mem::transmute(pchunkcount),
            ::core::mem::transmute(ppchunkmetadata),
            ::core::mem::transmute(pdatabytecount),
            ::core::mem::transmute(ppchunkdata),
            ::core::mem::transmute(pstatus),
            ::core::mem::transmute(ppitemresults),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn GetRequestStatus<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, requestid: Param0) -> ::windows::runtime::Result<DedupDataPortRequestStatus> {
        let mut result__: <DedupDataPortRequestStatus as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), requestid.into_param().abi(), &mut result__).from_abi::<DedupDataPortRequestStatus>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn GetRequestResults<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, requestid: Param0, maxwaitms: u32, pbatchresult: *mut ::windows::runtime::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), requestid.into_param().abi(), ::core::mem::transmute(maxwaitms), ::core::mem::transmute(pbatchresult), ::core::mem::transmute(pbatchcount), ::core::mem::transmute(pstatus), ::core::mem::transmute(ppitemresults)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDedupDataPort {
    type Vtable = IDedupDataPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7963d734_40a9_4ea3_bbf6_5a89d26f7ae8);
}
impl ::core::convert::From<IDedupDataPort> for ::windows::runtime::IUnknown {
    fn from(value: IDedupDataPort) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDedupDataPort> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupDataPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupDataPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDedupDataPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: ::windows::runtime::RawPtr, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamcount: u32, pstreams: *const ::core::mem::ManuallyDrop<DedupStream>, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamcount: u32, pstreams: *const ::core::mem::ManuallyDrop<DedupStream>, entrycount: u32, pentriesstream: ::windows::runtime::RawPtr, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamcount: u32, pstreampaths: *const ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: ::windows::runtime::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: ::windows::runtime::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: ::windows::runtime::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: ::windows::runtime::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::runtime::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDedupDataPortManager(pub ::windows::runtime::IUnknown);
impl IDedupDataPortManager {
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn GetConfiguration(&self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pminchunksize), ::core::mem::transmute(pmaxchunksize), ::core::mem::transmute(pchunkingalgorithm), ::core::mem::transmute(phashingalgorithm), ::core::mem::transmute(pcompressionalgorithm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn GetVolumeStatus<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, options: u32, path: Param1) -> ::windows::runtime::Result<DedupDataPortVolumeStatus> {
        let mut result__: <DedupDataPortVolumeStatus as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), path.into_param().abi(), &mut result__).from_abi::<DedupDataPortVolumeStatus>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn GetVolumeDataPort<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, options: u32, path: Param1) -> ::windows::runtime::Result<IDedupDataPort> {
        let mut result__: <IDedupDataPort as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), path.into_param().abi(), &mut result__).from_abi::<IDedupDataPort>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDedupDataPortManager {
    type Vtable = IDedupDataPortManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x44677452_b90a_445e_8192_cdcfe81511fb);
}
impl ::core::convert::From<IDedupDataPortManager> for ::windows::runtime::IUnknown {
    fn from(value: IDedupDataPortManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDedupDataPortManager> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupDataPortManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupDataPortManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDedupDataPortManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPortManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdataport: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDedupIterateChunksHash32(pub ::windows::runtime::IUnknown);
impl IDedupIterateChunksHash32 {
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn PushBuffer(&self, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(ulbufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn Next(&self, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulmaxchunks), ::core::mem::transmute(parrchunks), ::core::mem::transmute(pulfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn Drain(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDedupIterateChunksHash32 {
    type Vtable = IDedupIterateChunksHash32_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x90b584d3_72aa_400f_9767_cad866a5a2d8);
}
impl ::core::convert::From<IDedupIterateChunksHash32> for ::windows::runtime::IUnknown {
    fn from(value: IDedupIterateChunksHash32) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDedupIterateChunksHash32> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupIterateChunksHash32) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupIterateChunksHash32 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDedupIterateChunksHash32 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupIterateChunksHash32_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_DataDeduplication`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDedupReadFileCallback(pub ::windows::runtime::IUnknown);
impl IDedupReadFileCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn ReadBackupFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filefullpath: Param0, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filefullpath.into_param().abi(), ::core::mem::transmute(fileoffset), ::core::mem::transmute(sizetoread), ::core::mem::transmute(filebuffer), ::core::mem::transmute(returnedsize), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn OrderContainersRestore(&self, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(numberofcontainers), ::core::mem::transmute(containerpaths), ::core::mem::transmute(readplanentries), ::core::mem::transmute(readplan)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_DataDeduplication`, `Win32_Foundation`*"]
    pub unsafe fn PreviewContainerRead<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filefullpath: Param0, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), filefullpath.into_param().abi(), ::core::mem::transmute(numberofreads), ::core::mem::transmute(readoffsets)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDedupReadFileCallback {
    type Vtable = IDedupReadFileCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7bacc67a_2f1d_42d0_897e_6ff62dd533bb);
}
impl ::core::convert::From<IDedupReadFileCallback> for ::windows::runtime::IUnknown {
    fn from(value: IDedupReadFileCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDedupReadFileCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupReadFileCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupReadFileCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDedupReadFileCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupReadFileCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, numberofcontainers: u32, containerpaths: *const ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
