#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
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
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DEDUP_BACKUP_SUPPORT_PARAM_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DEDUP_RECONSTRUCT_UNOPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DEDUP_RECONSTRUCT_OPTIMIZED: DEDUP_BACKUP_SUPPORT_PARAM_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DEDUP_CHUNKLIB_MAX_CHUNKS_ENUM: u32 = 1024u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
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
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
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
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DEDUP_SET_PARAM_TYPE = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DEDUP_PT_MinChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DEDUP_PT_MaxChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DEDUP_PT_AvgChunkSizeBytes: DEDUP_SET_PARAM_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DEDUP_PT_InvariantChunking: DEDUP_SET_PARAM_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DEDUP_PT_DisableStrongHashComputation: DEDUP_SET_PARAM_TYPE = 5i32;
pub const DedupBackupSupport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73d6b2ad_2984_4715_b2e3_924c149744dd);
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
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
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DedupChunkFlags = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupChunkFlags_None: DedupChunkFlags = 0i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupChunkFlags_Compressed: DedupChunkFlags = 1i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DedupChunkingAlgorithm = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupChunkingAlgorithm_Unknonwn: DedupChunkingAlgorithm = 0i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupChunkingAlgorithm_V1: DedupChunkingAlgorithm = 1i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DedupCompressionAlgorithm = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupCompressionAlgorithm_Unknonwn: DedupCompressionAlgorithm = 0i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupCompressionAlgorithm_Xpress: DedupCompressionAlgorithm = 1i32;
pub const DedupDataPort: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f107207_1829_48b2_a64b_e61f8e0d9acb);
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DedupDataPortManagerOption = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortManagerOption_None: DedupDataPortManagerOption = 0i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortManagerOption_AutoStart: DedupDataPortManagerOption = 1i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortManagerOption_SkipReconciliation: DedupDataPortManagerOption = 2i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DedupDataPortRequestStatus = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortRequestStatus_Unknown: DedupDataPortRequestStatus = 0i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortRequestStatus_Queued: DedupDataPortRequestStatus = 1i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortRequestStatus_Processing: DedupDataPortRequestStatus = 2i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortRequestStatus_Partial: DedupDataPortRequestStatus = 3i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortRequestStatus_Complete: DedupDataPortRequestStatus = 4i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortRequestStatus_Failed: DedupDataPortRequestStatus = 5i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DedupDataPortVolumeStatus = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortVolumeStatus_Unknown: DedupDataPortVolumeStatus = 0i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortVolumeStatus_NotEnabled: DedupDataPortVolumeStatus = 1i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortVolumeStatus_NotAvailable: DedupDataPortVolumeStatus = 2i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortVolumeStatus_Initializing: DedupDataPortVolumeStatus = 3i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortVolumeStatus_Ready: DedupDataPortVolumeStatus = 4i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortVolumeStatus_Maintenance: DedupDataPortVolumeStatus = 5i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupDataPortVolumeStatus_Shutdown: DedupDataPortVolumeStatus = 6i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
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
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub type DedupHashingAlgorithm = i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupHashingAlgorithm_Unknonwn: DedupHashingAlgorithm = 0i32;
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
pub const DedupHashingAlgorithm_V1: DedupHashingAlgorithm = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
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
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
#[repr(transparent)]
pub struct IDedupBackupSupport(::windows::core::IUnknown);
impl IDedupBackupSupport {
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreFiles<'a, Param2: ::windows::core::IntoParam<'a, IDedupReadFileCallback>>(&self, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: Param2, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(numberoffiles), ::core::mem::transmute(filefullpaths), store.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(fileresults)).ok()
    }
}
impl ::core::convert::From<IDedupBackupSupport> for ::windows::core::IUnknown {
    fn from(value: IDedupBackupSupport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupBackupSupport> for ::windows::core::IUnknown {
    fn from(value: &IDedupBackupSupport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDedupBackupSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDedupBackupSupport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
    type Vtable = IDedupBackupSupportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc719d963_2b2d_415e_acf7_7eb7ca596ff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupBackupSupportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberoffiles: u32, filefullpaths: *const super::super::Foundation::BSTR, store: ::windows::core::RawPtr, flags: u32, fileresults: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
#[repr(transparent)]
pub struct IDedupChunkLibrary(::windows::core::IUnknown);
impl IDedupChunkLibrary {
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn InitializeForPushBuffers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn Uninitialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetParameter<'a, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, dwparamtype: u32, vparamvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwparamtype), vparamvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn StartChunking<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, iiditeratorinterfaceid: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), iiditeratorinterfaceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IDedupChunkLibrary> for ::windows::core::IUnknown {
    fn from(value: IDedupChunkLibrary) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupChunkLibrary> for ::windows::core::IUnknown {
    fn from(value: &IDedupChunkLibrary) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDedupChunkLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDedupChunkLibrary {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
    type Vtable = IDedupChunkLibraryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb5144d7_2720_4dcc_8777_78597416ec23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupChunkLibraryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwparamtype: u32, vparamvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iiditeratorinterfaceid: ::windows::core::GUID, ppchunksenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
#[repr(transparent)]
pub struct IDedupDataPort(::windows::core::IUnknown);
impl IDedupDataPort {
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn GetStatus(&self, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatus), ::core::mem::transmute(pdataheadroommb)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn LookupChunks(&self, count: u32, phashes: *const DedupHash) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(phashes), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn InsertChunks(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(chunkcount), ::core::mem::transmute(pchunkmetadata), ::core::mem::transmute(databytecount), ::core::mem::transmute(pchunkdata), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertChunksWithStream<'a, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: Param3) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(chunkcount), ::core::mem::transmute(pchunkmetadata), ::core::mem::transmute(databytecount), pchunkdatastream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CommitStreams(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamcount), ::core::mem::transmute(pstreams), ::core::mem::transmute(entrycount), ::core::mem::transmute(pentries), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CommitStreamsWithStream<'a, Param3: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: Param3) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamcount), ::core::mem::transmute(pstreams), ::core::mem::transmute(entrycount), pentriesstream.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreams(&self, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamcount), ::core::mem::transmute(pstreampaths), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreamsResults<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, requestid: Param0, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), requestid.into_param().abi(), ::core::mem::transmute(maxwaitms), ::core::mem::transmute(streamentryindex), ::core::mem::transmute(pstreamcount), ::core::mem::transmute(ppstreams), ::core::mem::transmute(pentrycount), ::core::mem::transmute(ppentries), ::core::mem::transmute(pstatus), ::core::mem::transmute(ppitemresults)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn GetChunks(&self, count: u32, phashes: *const DedupHash) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(count), ::core::mem::transmute(phashes), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn GetChunksResults<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, requestid: Param0, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), requestid.into_param().abi(), ::core::mem::transmute(maxwaitms), ::core::mem::transmute(chunkindex), ::core::mem::transmute(pchunkcount), ::core::mem::transmute(ppchunkmetadata), ::core::mem::transmute(pdatabytecount), ::core::mem::transmute(ppchunkdata), ::core::mem::transmute(pstatus), ::core::mem::transmute(ppitemresults)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn GetRequestStatus<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, requestid: Param0) -> ::windows::core::Result<DedupDataPortRequestStatus> {
        let mut result__: DedupDataPortRequestStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), requestid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DedupDataPortRequestStatus>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn GetRequestResults<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, requestid: Param0, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), requestid.into_param().abi(), ::core::mem::transmute(maxwaitms), ::core::mem::transmute(pbatchresult), ::core::mem::transmute(pbatchcount), ::core::mem::transmute(pstatus), ::core::mem::transmute(ppitemresults)).ok()
    }
}
impl ::core::convert::From<IDedupDataPort> for ::windows::core::IUnknown {
    fn from(value: IDedupDataPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupDataPort> for ::windows::core::IUnknown {
    fn from(value: &IDedupDataPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDedupDataPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDedupDataPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
    type Vtable = IDedupDataPortVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7963d734_40a9_4ea3_bbf6_5a89d26f7ae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPortVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: ::windows::core::RawPtr, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: ::windows::core::RawPtr, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamcount: u32, pstreampaths: *const super::super::Foundation::BSTR, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows::core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
#[repr(transparent)]
pub struct IDedupDataPortManager(::windows::core::IUnknown);
impl IDedupDataPortManager {
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn GetConfiguration(&self, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pminchunksize), ::core::mem::transmute(pmaxchunksize), ::core::mem::transmute(pchunkingalgorithm), ::core::mem::transmute(phashingalgorithm), ::core::mem::transmute(pcompressionalgorithm)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeStatus<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, options: u32, path: Param1) -> ::windows::core::Result<DedupDataPortVolumeStatus> {
        let mut result__: DedupDataPortVolumeStatus = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<DedupDataPortVolumeStatus>(result__)
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeDataPort<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, options: u32, path: Param1) -> ::windows::core::Result<IDedupDataPort> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), path.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDedupDataPort>(result__)
    }
}
impl ::core::convert::From<IDedupDataPortManager> for ::windows::core::IUnknown {
    fn from(value: IDedupDataPortManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupDataPortManager> for ::windows::core::IUnknown {
    fn from(value: &IDedupDataPortManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDedupDataPortManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDedupDataPortManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
    type Vtable = IDedupDataPortManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44677452_b90a_445e_8192_cdcfe81511fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupDataPortManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: u32, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdataport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
#[repr(transparent)]
pub struct IDedupIterateChunksHash32(::windows::core::IUnknown);
impl IDedupIterateChunksHash32 {
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn PushBuffer(&self, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(ulbufferlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn Next(&self, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulmaxchunks), ::core::mem::transmute(parrchunks), ::core::mem::transmute(pulfetched)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn Drain(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDedupIterateChunksHash32> for ::windows::core::IUnknown {
    fn from(value: IDedupIterateChunksHash32) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupIterateChunksHash32> for ::windows::core::IUnknown {
    fn from(value: &IDedupIterateChunksHash32) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDedupIterateChunksHash32 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDedupIterateChunksHash32 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
    type Vtable = IDedupIterateChunksHash32Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90b584d3_72aa_400f_9767_cad866a5a2d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupIterateChunksHash32Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *const u8, ulbufferlength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_Storage_DataDeduplication'*"]
#[repr(transparent)]
pub struct IDedupReadFileCallback(::windows::core::IUnknown);
impl IDedupReadFileCallback {
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReadBackupFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filefullpath: Param0, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), filefullpath.into_param().abi(), ::core::mem::transmute(fileoffset), ::core::mem::transmute(sizetoread), ::core::mem::transmute(filebuffer), ::core::mem::transmute(returnedsize), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OrderContainersRestore(&self, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(numberofcontainers), ::core::mem::transmute(containerpaths), ::core::mem::transmute(readplanentries), ::core::mem::transmute(readplan)).ok()
    }
    #[doc = "*Required features: 'Win32_Storage_DataDeduplication', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreviewContainerRead<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filefullpath: Param0, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), filefullpath.into_param().abi(), ::core::mem::transmute(numberofreads), ::core::mem::transmute(readoffsets)).ok()
    }
}
impl ::core::convert::From<IDedupReadFileCallback> for ::windows::core::IUnknown {
    fn from(value: IDedupReadFileCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDedupReadFileCallback> for ::windows::core::IUnknown {
    fn from(value: &IDedupReadFileCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDedupReadFileCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDedupReadFileCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
    type Vtable = IDedupReadFileCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bacc67a_2f1d_42d0_897e_6ff62dd533bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDedupReadFileCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberofcontainers: u32, containerpaths: *const super::super::Foundation::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filefullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
