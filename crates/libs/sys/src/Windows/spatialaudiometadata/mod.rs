pub const SPATIAL_AUDIO_POSITION: u32 = 200;
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200;
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: i32 = -2004286956;
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: i32 = -2004286969;
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: i32 = -2004286968;
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: i32 = -2004286940;
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: i32 = -2004286942;
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: i32 = -2004286976;
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: i32 = -2004286955;
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: i32 = -2004286941;
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: i32 = -2004286967;
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: i32 = -2004286952;
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: i32 = -2004286974;
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: i32 = -2004286957;
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: i32 = -2004286939;
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: i32 = -2004286959;
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: i32 = -2004286951;
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: i32 = -2004286971;
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: i32 = -2004286973;
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: i32 = -2004286954;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: i32 = -2004286944;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: i32 = -2004286960;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: i32 = -2004286958;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: i32 = -2004286943;
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: i32 = -2004286970;
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: i32 = -2004286953;
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: i32 = -2004286975;
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: i32 = -2004286972;
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
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const super::mmeapi::WAVEFORMATEX,
    pub StaticObjectTypeMask: super::spatialaudioclient::AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::winnt::HANDLE,
    pub MetadataFormatId: windows_sys::core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const super::propidlbase::PROPVARIANT,
    pub NotifyObject: *mut core::ffi::c_void,
}
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const super::mmeapi::WAVEFORMATEX,
    pub StaticObjectTypeMask: super::spatialaudioclient::AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::winnt::HANDLE,
    pub MetadataFormatId: windows_sys::core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const super::propidlbase::PROPVARIANT,
    pub NotifyObject: *mut core::ffi::c_void,
    pub Options: super::spatialaudioclient::SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "audiosessiontypes", feature = "minwindef", feature = "mmeapi", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "spatialaudioclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
