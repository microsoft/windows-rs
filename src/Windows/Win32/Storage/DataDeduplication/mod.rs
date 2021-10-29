#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DDP_FILE_EXTENT {
    pub Length: i64,
    pub Offset: i64,
}
impl DDP_FILE_EXTENT {}
impl ::std::default::Default for DDP_FILE_EXTENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDP_FILE_EXTENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDP_FILE_EXTENT").field("Length", &self.Length).field("Offset", &self.Offset).finish()
    }
}
impl ::std::cmp::PartialEq for DDP_FILE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Offset == other.Offset
    }
}
impl ::std::cmp::Eq for DDP_FILE_EXTENT {}
unsafe impl ::windows::runtime::Abi for DDP_FILE_EXTENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEDUP_BACKUP_SUPPORT_PARAM_TYPE(pub i32);
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(1i32);
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = DEDUP_BACKUP_SUPPORT_PARAM_TYPE(2i32);
impl ::std::convert::From<i32> for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEDUP_BACKUP_SUPPORT_PARAM_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DEDUP_CHUNK_INFO_HASH32 {
    pub ChunkFlags: u32,
    pub ChunkOffsetInStream: u64,
    pub ChunkSize: u64,
    pub HashVal: [u8; 32],
}
impl DEDUP_CHUNK_INFO_HASH32 {}
impl ::std::default::Default for DEDUP_CHUNK_INFO_HASH32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DEDUP_CHUNK_INFO_HASH32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEDUP_CHUNK_INFO_HASH32").field("ChunkFlags", &self.ChunkFlags).field("ChunkOffsetInStream", &self.ChunkOffsetInStream).field("ChunkSize", &self.ChunkSize).field("HashVal", &self.HashVal).finish()
    }
}
impl ::std::cmp::PartialEq for DEDUP_CHUNK_INFO_HASH32 {
    fn eq(&self, other: &Self) -> bool {
        self.ChunkFlags == other.ChunkFlags && self.ChunkOffsetInStream == other.ChunkOffsetInStream && self.ChunkSize == other.ChunkSize && self.HashVal == other.HashVal
    }
}
impl ::std::cmp::Eq for DEDUP_CHUNK_INFO_HASH32 {}
unsafe impl ::windows::runtime::Abi for DEDUP_CHUNK_INFO_HASH32 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DEDUP_CONTAINER_EXTENT {
    pub ContainerIndex: u32,
    pub StartOffset: i64,
    pub Length: i64,
}
impl DEDUP_CONTAINER_EXTENT {}
impl ::std::default::Default for DEDUP_CONTAINER_EXTENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DEDUP_CONTAINER_EXTENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEDUP_CONTAINER_EXTENT").field("ContainerIndex", &self.ContainerIndex).field("StartOffset", &self.StartOffset).field("Length", &self.Length).finish()
    }
}
impl ::std::cmp::PartialEq for DEDUP_CONTAINER_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.ContainerIndex == other.ContainerIndex && self.StartOffset == other.StartOffset && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for DEDUP_CONTAINER_EXTENT {}
unsafe impl ::windows::runtime::Abi for DEDUP_CONTAINER_EXTENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEDUP_SET_PARAM_TYPE(pub i32);
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(1i32);
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(2i32);
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(3i32);
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(4i32);
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = DEDUP_SET_PARAM_TYPE(5i32);
impl ::std::convert::From<i32> for DEDUP_SET_PARAM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEDUP_SET_PARAM_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DedupBackupSupport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1943450285, 10628, 18197, [178, 227, 146, 76, 20, 151, 68, 221]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DedupChunk {
    pub Hash: DedupHash,
    pub Flags: DedupChunkFlags,
    pub LogicalSize: u32,
    pub DataSize: u32,
}
impl DedupChunk {}
impl ::std::default::Default for DedupChunk {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DedupChunk {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DedupChunk").field("Hash", &self.Hash).field("Flags", &self.Flags).field("LogicalSize", &self.LogicalSize).field("DataSize", &self.DataSize).finish()
    }
}
impl ::std::cmp::PartialEq for DedupChunk {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash && self.Flags == other.Flags && self.LogicalSize == other.LogicalSize && self.DataSize == other.DataSize
    }
}
impl ::std::cmp::Eq for DedupChunk {}
unsafe impl ::windows::runtime::Abi for DedupChunk {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupChunkFlags(pub i32);
pub const DedupChunkFlags_None: DedupChunkFlags = DedupChunkFlags(0i32);
pub const DedupChunkFlags_Compressed: DedupChunkFlags = DedupChunkFlags(1i32);
impl ::std::convert::From<i32> for DedupChunkFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupChunkFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupChunkingAlgorithm(pub i32);
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = DedupChunkingAlgorithm(0i32);
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = DedupChunkingAlgorithm(1i32);
impl ::std::convert::From<i32> for DedupChunkingAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupChunkingAlgorithm {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupCompressionAlgorithm(pub i32);
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = DedupCompressionAlgorithm(0i32);
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = DedupCompressionAlgorithm(1i32);
impl ::std::convert::From<i32> for DedupCompressionAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupCompressionAlgorithm {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DedupDataPort: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2400219655, 6185, 18610, [166, 75, 230, 31, 142, 13, 154, 203]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupDataPortManagerOption(pub i32);
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = DedupDataPortManagerOption(0i32);
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = DedupDataPortManagerOption(1i32);
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = DedupDataPortManagerOption(2i32);
impl ::std::convert::From<i32> for DedupDataPortManagerOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupDataPortManagerOption {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupDataPortRequestStatus(pub i32);
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = DedupDataPortRequestStatus(0i32);
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = DedupDataPortRequestStatus(1i32);
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = DedupDataPortRequestStatus(2i32);
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = DedupDataPortRequestStatus(3i32);
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = DedupDataPortRequestStatus(4i32);
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = DedupDataPortRequestStatus(5i32);
impl ::std::convert::From<i32> for DedupDataPortRequestStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupDataPortRequestStatus {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupDataPortVolumeStatus(pub i32);
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(0i32);
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(1i32);
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(2i32);
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(3i32);
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(4i32);
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(5i32);
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = DedupDataPortVolumeStatus(6i32);
impl ::std::convert::From<i32> for DedupDataPortVolumeStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupDataPortVolumeStatus {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DedupHash {
    pub Hash: [u8; 32],
}
impl DedupHash {}
impl ::std::default::Default for DedupHash {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DedupHash {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DedupHash").field("Hash", &self.Hash).finish()
    }
}
impl ::std::cmp::PartialEq for DedupHash {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash
    }
}
impl ::std::cmp::Eq for DedupHash {}
unsafe impl ::windows::runtime::Abi for DedupHash {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DedupHashingAlgorithm(pub i32);
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = DedupHashingAlgorithm(0i32);
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = DedupHashingAlgorithm(1i32);
impl ::std::convert::From<i32> for DedupHashingAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DedupHashingAlgorithm {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DedupStream {
    pub Path: super::super::Foundation::BSTR,
    pub Offset: u64,
    pub Length: u64,
    pub ChunkCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DedupStream {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DedupStream {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DedupStream {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DedupStream").field("Path", &self.Path).field("Offset", &self.Offset).field("Length", &self.Length).field("ChunkCount", &self.ChunkCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DedupStream {
    fn eq(&self, other: &Self) -> bool {
        self.Path == other.Path && self.Offset == other.Offset && self.Length == other.Length && self.ChunkCount == other.ChunkCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DedupStream {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DedupStream {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DedupStreamEntry {
    pub Hash: DedupHash,
    pub LogicalSize: u32,
    pub Offset: u64,
}
impl DedupStreamEntry {}
impl ::std::default::Default for DedupStreamEntry {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DedupStreamEntry {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DedupStreamEntry").field("Hash", &self.Hash).field("LogicalSize", &self.LogicalSize).field("Offset", &self.Offset).finish()
    }
}
impl ::std::cmp::PartialEq for DedupStreamEntry {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash && self.LogicalSize == other.LogicalSize && self.Offset == other.Offset
    }
}
impl ::std::cmp::Eq for DedupStreamEntry {}
unsafe impl ::windows::runtime::Abi for DedupStreamEntry {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDedupBackupSupport(::windows::runtime::IUnknown);
impl IDedupBackupSupport {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreFiles<'a, Param2: ::windows::runtime::IntoParam<'a, IDedupReadFileCallback>>(&self, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: Param2, flags: u32, fileresults: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(numberoffiles), ::std::mem::transmute(filefullpaths), store.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(fileresults)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDedupBackupSupport {
    type Vtable = IDedupBackupSupport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3340360035, 11053, 16734, [172, 247, 126, 183, 202, 89, 111, 244]);
}
impl ::std::convert::From<IDedupBackupSupport> for ::windows::runtime::IUnknown {
    fn from(value: IDedupBackupSupport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDedupBackupSupport> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupBackupSupport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupBackupSupport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDedupBackupSupport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupBackupSupport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, numberoffiles: u32, filefullpaths: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, store: ::windows::runtime::RawPtr, flags: u32, fileresults: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDedupChunkLibrary(::windows::runtime::IUnknown);
impl IDedupChunkLibrary {
    pub unsafe fn InitializeForPushBuffers(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Uninitialize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetParameter<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, dwparamtype: u32, vparamvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwparamtype), vparamvalue.into_param().abi()).ok()
    }
    pub unsafe fn StartChunking<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, iiditeratorinterfaceid: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), iiditeratorinterfaceid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDedupChunkLibrary {
    type Vtable = IDedupChunkLibrary_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3142665431, 10016, 19916, [135, 119, 120, 89, 116, 22, 236, 35]);
}
impl ::std::convert::From<IDedupChunkLibrary> for ::windows::runtime::IUnknown {
    fn from(value: IDedupChunkLibrary) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDedupChunkLibrary> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupChunkLibrary) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupChunkLibrary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDedupChunkLibrary {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwparamtype: u32, vparamvalue: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iiditeratorinterfaceid: ::windows::runtime::GUID, ppchunksenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDedupDataPort(::windows::runtime::IUnknown);
impl IDedupDataPort {
    pub unsafe fn GetStatus(&self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatus), ::std::mem::transmute(pdataheadroommb)).ok()
    }
    pub unsafe fn LookupChunks(&self, count: u32, phashes: *const DedupHash) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(count), ::std::mem::transmute(phashes), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn InsertChunks(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(chunkcount), ::std::mem::transmute(pchunkmetadata), ::std::mem::transmute(databytecount), ::std::mem::transmute(pchunkdata), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertChunksWithStream<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: Param3) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(chunkcount), ::std::mem::transmute(pchunkmetadata), ::std::mem::transmute(databytecount), pchunkdatastream.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CommitStreams(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(streamcount), ::std::mem::transmute(pstreams), ::std::mem::transmute(entrycount), ::std::mem::transmute(pentries), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CommitStreamsWithStream<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: Param3) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(streamcount), ::std::mem::transmute(pstreams), ::std::mem::transmute(entrycount), pentriesstream.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreams(&self, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(streamcount), ::std::mem::transmute(pstreampaths), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreamsResults<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, requestid: Param0, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            requestid.into_param().abi(),
            ::std::mem::transmute(maxwaitms),
            ::std::mem::transmute(streamentryindex),
            ::std::mem::transmute(pstreamcount),
            ::std::mem::transmute(ppstreams),
            ::std::mem::transmute(pentrycount),
            ::std::mem::transmute(ppentries),
            ::std::mem::transmute(pstatus),
            ::std::mem::transmute(ppitemresults),
        )
        .ok()
    }
    pub unsafe fn GetChunks(&self, count: u32, phashes: *const DedupHash) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(count), ::std::mem::transmute(phashes), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn GetChunksResults<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, requestid: Param0, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            requestid.into_param().abi(),
            ::std::mem::transmute(maxwaitms),
            ::std::mem::transmute(chunkindex),
            ::std::mem::transmute(pchunkcount),
            ::std::mem::transmute(ppchunkmetadata),
            ::std::mem::transmute(pdatabytecount),
            ::std::mem::transmute(ppchunkdata),
            ::std::mem::transmute(pstatus),
            ::std::mem::transmute(ppitemresults),
        )
        .ok()
    }
    pub unsafe fn GetRequestStatus<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, requestid: Param0) -> ::windows::runtime::Result<DedupDataPortRequestStatus> {
        let mut result__: <DedupDataPortRequestStatus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), requestid.into_param().abi(), &mut result__).from_abi::<DedupDataPortRequestStatus>(result__)
    }
    pub unsafe fn GetRequestResults<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, requestid: Param0, maxwaitms: u32, pbatchresult: *mut ::windows::runtime::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), requestid.into_param().abi(), ::std::mem::transmute(maxwaitms), ::std::mem::transmute(pbatchresult), ::std::mem::transmute(pbatchcount), ::std::mem::transmute(pstatus), ::std::mem::transmute(ppitemresults)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDedupDataPort {
    type Vtable = IDedupDataPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2036586292, 16553, 20131, [187, 246, 90, 137, 210, 111, 122, 232]);
}
impl ::std::convert::From<IDedupDataPort> for ::windows::runtime::IUnknown {
    fn from(value: IDedupDataPort) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDedupDataPort> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupDataPort) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupDataPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDedupDataPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamcount: u32, pstreams: *const ::std::mem::ManuallyDrop<DedupStream>, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamcount: u32, pstreams: *const ::std::mem::ManuallyDrop<DedupStream>, entrycount: u32, pentriesstream: ::windows::runtime::RawPtr, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamcount: u32, pstreampaths: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: ::windows::runtime::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: ::windows::runtime::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: ::windows::runtime::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestid: ::windows::runtime::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::runtime::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDedupDataPortManager(::windows::runtime::IUnknown);
impl IDedupDataPortManager {
    pub unsafe fn GetConfiguration(&self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pminchunksize), ::std::mem::transmute(pmaxchunksize), ::std::mem::transmute(pchunkingalgorithm), ::std::mem::transmute(phashingalgorithm), ::std::mem::transmute(pcompressionalgorithm)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeStatus<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, options: u32, path: Param1) -> ::windows::runtime::Result<DedupDataPortVolumeStatus> {
        let mut result__: <DedupDataPortVolumeStatus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), path.into_param().abi(), &mut result__).from_abi::<DedupDataPortVolumeStatus>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeDataPort<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, options: u32, path: Param1) -> ::windows::runtime::Result<IDedupDataPort> {
        let mut result__: <IDedupDataPort as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), path.into_param().abi(), &mut result__).from_abi::<IDedupDataPort>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDedupDataPortManager {
    type Vtable = IDedupDataPortManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1147630674, 47370, 17502, [129, 146, 205, 207, 232, 21, 17, 251]);
}
impl ::std::convert::From<IDedupDataPortManager> for ::windows::runtime::IUnknown {
    fn from(value: IDedupDataPortManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDedupDataPortManager> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupDataPortManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupDataPortManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDedupDataPortManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPortManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: u32, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: u32, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdataport: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDedupIterateChunksHash32(::windows::runtime::IUnknown);
impl IDedupIterateChunksHash32 {
    pub unsafe fn PushBuffer(&self, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer), ::std::mem::transmute(ulbufferlength)).ok()
    }
    pub unsafe fn Next(&self, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulmaxchunks), ::std::mem::transmute(parrchunks), ::std::mem::transmute(pulfetched)).ok()
    }
    pub unsafe fn Drain(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDedupIterateChunksHash32 {
    type Vtable = IDedupIterateChunksHash32_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2427815123, 29354, 16399, [151, 103, 202, 216, 102, 165, 162, 216]);
}
impl ::std::convert::From<IDedupIterateChunksHash32> for ::windows::runtime::IUnknown {
    fn from(value: IDedupIterateChunksHash32) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDedupIterateChunksHash32> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupIterateChunksHash32) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupIterateChunksHash32 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDedupIterateChunksHash32 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDedupReadFileCallback(::windows::runtime::IUnknown);
impl IDedupReadFileCallback {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadBackupFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filefullpath: Param0, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filefullpath.into_param().abi(), ::std::mem::transmute(fileoffset), ::std::mem::transmute(sizetoread), ::std::mem::transmute(filebuffer), ::std::mem::transmute(returnedsize), ::std::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OrderContainersRestore(&self, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(numberofcontainers), ::std::mem::transmute(containerpaths), ::std::mem::transmute(readplanentries), ::std::mem::transmute(readplan)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreviewContainerRead<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filefullpath: Param0, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), filefullpath.into_param().abi(), ::std::mem::transmute(numberofreads), ::std::mem::transmute(readoffsets)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDedupReadFileCallback {
    type Vtable = IDedupReadFileCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2074920570, 12061, 17104, [137, 126, 111, 246, 45, 213, 51, 187]);
}
impl ::std::convert::From<IDedupReadFileCallback> for ::windows::runtime::IUnknown {
    fn from(value: IDedupReadFileCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDedupReadFileCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IDedupReadFileCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDedupReadFileCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDedupReadFileCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupReadFileCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filefullpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, numberofcontainers: u32, containerpaths: *const ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filefullpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
