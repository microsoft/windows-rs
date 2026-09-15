pub const SPATIAL_AUDIO_POSITION: i32 = 200;
#[cfg(target_arch = "x86")]
pub const SPATIAL_AUDIO_POSITION_BYTE_COUNT: u32 = 12;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SPATIAL_AUDIO_POSITION_BYTE_COUNT: u64 = 12;
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: i32 = 200;
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: windows_sys::core::HRESULT = 0x88890214_u32 as _;
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: windows_sys::core::HRESULT = 0x88890207_u32 as _;
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: windows_sys::core::HRESULT = 0x88890208_u32 as _;
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: windows_sys::core::HRESULT = 0x88890224_u32 as _;
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: windows_sys::core::HRESULT = 0x88890222_u32 as _;
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: windows_sys::core::HRESULT = 0x88890200_u32 as _;
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: windows_sys::core::HRESULT = 0x88890215_u32 as _;
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: windows_sys::core::HRESULT = 0x88890223_u32 as _;
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: windows_sys::core::HRESULT = 0x88890209_u32 as _;
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: windows_sys::core::HRESULT = 0x88890218_u32 as _;
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: windows_sys::core::HRESULT = 0x88890202_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: windows_sys::core::HRESULT = 0x88890213_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: windows_sys::core::HRESULT = 0x88890225_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: windows_sys::core::HRESULT = 0x88890211_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: windows_sys::core::HRESULT = 0x88890219_u32 as _;
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: windows_sys::core::HRESULT = 0x88890205_u32 as _;
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: windows_sys::core::HRESULT = 0x88890203_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: windows_sys::core::HRESULT = 0x88890216_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: windows_sys::core::HRESULT = 0x88890220_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: windows_sys::core::HRESULT = 0x88890210_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: windows_sys::core::HRESULT = 0x88890212_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: windows_sys::core::HRESULT = 0x88890221_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: windows_sys::core::HRESULT = 0x88890206_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: windows_sys::core::HRESULT = 0x88890217_u32 as _;
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: windows_sys::core::HRESULT = 0x88890201_u32 as _;
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: windows_sys::core::HRESULT = 0x88890204_u32 as _;
pub type SpatialAudioMetadataCopyMode = i32;
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = 1;
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = 3;
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = 2;
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
pub type SpatialAudioMetadataWriterOverflowMode = i32;
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = 0;
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = 2;
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const super::WAVEFORMATEX,
    pub StaticObjectTypeMask: super::AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::HANDLE,
    pub MetadataFormatId: windows_sys::core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const super::PROPVARIANT,
    pub NotifyObject: *mut core::ffi::c_void,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const super::WAVEFORMATEX,
    pub StaticObjectTypeMask: super::AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::HANDLE,
    pub MetadataFormatId: windows_sys::core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const super::PROPVARIANT,
    pub NotifyObject: *mut core::ffi::c_void,
    pub Options: super::SPATIAL_AUDIO_STREAM_OPTIONS,
}
