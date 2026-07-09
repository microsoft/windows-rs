pub type D3D12_BITSTREAM_ENCRYPTION_TYPE = i32;
pub const D3D12_BITSTREAM_ENCRYPTION_TYPE_NONE: D3D12_BITSTREAM_ENCRYPTION_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {
    pub IOCoherent: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {
    pub VideoDecoderHeapDesc: D3D12_VIDEO_DECODER_HEAP_DESC,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {
    pub VideoDecoderHeapDesc: D3D12_VIDEO_DECODER_HEAP_DESC,
    pub Protected: windows_sys::core::BOOL,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub DecodeSample: D3D12_VIDEO_SAMPLE,
    pub OutputFormat: D3D12_VIDEO_FORMAT,
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub BitRate: u32,
    pub SupportFlags: D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS,
    pub ScaleSupport: D3D12_VIDEO_SCALE_SUPPORT,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub FormatCount: u32,
    pub pOutputFormats: *mut super::dxgiformat::DXGI_FORMAT,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub FormatCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {
    pub NodeIndex: u32,
    pub DecodeProfile: windows_sys::core::GUID,
    pub Width: u32,
    pub Height: u32,
    pub DecodeFormat: super::dxgiformat::DXGI_FORMAT,
    pub Components: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS,
    pub BinCount: u32,
    pub CounterBitDepth: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    pub NodeIndex: u32,
    pub ProfileCount: u32,
    pub pProfiles: *mut windows_sys::core::GUID,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {
    pub NodeIndex: u32,
    pub ProfileCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub Width: u32,
    pub Height: u32,
    pub DecodeFormat: super::dxgiformat::DXGI_FORMAT,
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub BitRate: u32,
    pub SupportFlags: D3D12_VIDEO_DECODE_SUPPORT_FLAGS,
    pub ConfigurationFlags: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS,
    pub DecodeTier: D3D12_VIDEO_DECODE_TIER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub IsSupported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub IsSupported: windows_sys::core::BOOL,
    pub CodecSupportLimits: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub IsSupported: windows_sys::core::BOOL,
    pub PictureSupport: D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_DIRTY_REGIONS {
    pub NodeIndex: u32,
    pub SessionInfo: D3D12_VIDEO_ENCODER_INPUT_MAP_SESSION_INFO,
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub MapValuesType: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE,
    pub SupportFlags: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAGS,
    pub MapSourcePreferenceRanking: u32,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_DIRTY_REGIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Level: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub SubregionMode: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub FrameResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub CodecSupport: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT,
    pub IsSupported: windows_sys::core::BOOL,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Level: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub SubregionMode: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub IsSupported: windows_sys::core::BOOL,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_HEAP_SIZE {
    pub HeapDesc: D3D12_VIDEO_ENCODER_HEAP_DESC,
    pub IsSupported: windows_sys::core::BOOL,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_HEAP_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_HEAP_SIZE1 {
    pub HeapDesc: D3D12_VIDEO_ENCODER_HEAP_DESC1,
    pub IsSupported: windows_sys::core::BOOL,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_HEAP_SIZE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_INPUT_FORMAT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub IsSupported: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_INPUT_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_INTRA_REFRESH_MODE {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Level: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub IntraRefreshMode: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE,
    pub IsSupported: windows_sys::core::BOOL,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_INTRA_REFRESH_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_MOTION_SEARCH {
    pub NodeIndex: u32,
    pub SessionInfo: D3D12_VIDEO_ENCODER_INPUT_MAP_SESSION_INFO,
    pub MotionSearchMode: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE,
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub BidirectionalRefFrameEnabled: windows_sys::core::BOOL,
    pub SupportFlags: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS,
    pub MaxMotionHints: u32,
    pub MinDeviation: u32,
    pub MaxDeviation: u32,
    pub MapSourcePreferenceRanking: u32,
    pub MotionUnitPrecisionSupport: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAGS,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_MOTION_SEARCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub ResolutionRatiosCount: u32,
    pub IsSupported: windows_sys::core::BOOL,
    pub MinResolutionSupported: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub MaxResolutionSupported: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub ResolutionWidthMultipleRequirement: u32,
    pub ResolutionHeightMultipleRequirement: u32,
    pub pResolutionRatios: *mut D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub ResolutionRatiosCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_PROFILE_LEVEL {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub IsSupported: windows_sys::core::BOOL,
    pub MinSupportedLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub MaxSupportedLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_PROFILE_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_QPMAP_INPUT {
    pub NodeIndex: u32,
    pub SessionInfo: D3D12_VIDEO_ENCODER_INPUT_MAP_SESSION_INFO,
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub IsSupported: windows_sys::core::BOOL,
    pub MapSourcePreferenceRanking: u32,
    pub BlockSize: u32,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_QPMAP_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Level: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub InputResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub SubregionFrameEncoding: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub SubregionFrameEncodingData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA,
    pub QPMap: D3D12_VIDEO_ENCODER_QPMAP_CONFIGURATION,
    pub DirtyRegions: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_CONFIGURATION,
    pub MotionSearch: D3D12_VIDEO_ENCODER_MOTION_SEARCH_CONFIGURATION,
    pub Pow2DownscaleFactor: u32,
    pub SupportFlags: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub RateControlMode: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE,
    pub IsSupported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_DIRTY_REGIONS {
    pub DirtyRegionsSupportFlags: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAGS,
    pub MapSourcePreferenceRanking: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_FRAME_ANALYSIS {
    pub SupportFlags: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {
    pub MaxSubregionsNumber: u32,
    pub MaxIntraRefreshFrameDuration: u32,
    pub SubregionBlockPixelsSize: u32,
    pub QPMapRegionPixelsSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS1 {
    pub MaxSubregionsNumber: u32,
    pub MaxIntraRefreshFrameDuration: u32,
    pub SubregionBlockPixelsSize: u32,
    pub QPMapRegionPixelsSize: u32,
    pub QPMap: D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_QPMAP,
    pub DirtyRegions: D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_DIRTY_REGIONS,
    pub MotionSearch: D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_MOTION_SEARCH,
    pub FrameAnalysis: D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_FRAME_ANALYSIS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_MOTION_SEARCH {
    pub MaxMotionHints: u32,
    pub MinDeviation: u32,
    pub MaxDeviation: u32,
    pub MapSourcePreferenceRanking: u32,
    pub MotionUnitPrecisionSupportFlags: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAGS,
    pub SupportFlags: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_QPMAP {
    pub MapSourcePreferenceRanking: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT {
    pub NodeIndex: u32,
    pub SessionInfo: D3D12_VIDEO_ENCODER_INPUT_MAP_SESSION_INFO,
    pub MapType: D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE,
    pub IsSupported: windows_sys::core::BOOL,
    pub MaxResolvedBufferAllocationSize: u64,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub PictureTargetResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub IsSupported: windows_sys::core::BOOL,
    pub CompressedBitstreamBufferAccessAlignment: u32,
    pub EncoderMetadataBufferAccessAlignment: u32,
    pub MaxEncoderOutputMetadataBufferSize: u32,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS1 {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub PictureTargetResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub IsSupported: windows_sys::core::BOOL,
    pub CompressedBitstreamBufferAccessAlignment: u32,
    pub EncoderMetadataBufferAccessAlignment: u32,
    pub MaxEncoderOutputMetadataBufferSize: u32,
    pub OptionalMetadata: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub EncoderOutputMetadataQPMapTextureDimensions: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub EncoderOutputMetadataSATDMapTextureDimensions: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub EncoderOutputMetadataBitAllocationMapTextureDimensions: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub EncoderOutputMetadataFramePSNRComponentsNumber: u32,
    pub EncoderOutputMetadataSubregionsPSNRComponentsNumber: u32,
    pub EncoderOutputMetadataSubregionsPSNRResolvedMetadataBufferSize: u32,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub CodecGopSequence: D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE,
    pub RateControl: D3D12_VIDEO_ENCODER_RATE_CONTROL,
    pub IntraRefresh: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE,
    pub SubregionFrameEncoding: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub ResolutionsListCount: u32,
    pub pResolutionList: *const D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub MaxReferenceFramesInDPB: u32,
    pub ValidationFlags: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS,
    pub SupportFlags: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS,
    pub SuggestedProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub SuggestedLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub pResolutionDependentSupport: *mut D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT1 {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub CodecGopSequence: D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE,
    pub RateControl: D3D12_VIDEO_ENCODER_RATE_CONTROL,
    pub IntraRefresh: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE,
    pub SubregionFrameEncoding: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub ResolutionsListCount: u32,
    pub pResolutionList: *const D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub MaxReferenceFramesInDPB: u32,
    pub ValidationFlags: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS,
    pub SupportFlags: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS,
    pub SuggestedProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub SuggestedLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub pResolutionDependentSupport: *mut D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS,
    pub SubregionFrameEncodingData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA,
    pub MaxQualityVsSpeed: u32,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT2 {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub CodecGopSequence: D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE,
    pub RateControl: D3D12_VIDEO_ENCODER_RATE_CONTROL,
    pub IntraRefresh: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE,
    pub SubregionFrameEncoding: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub ResolutionsListCount: u32,
    pub pResolutionList: *const D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub MaxReferenceFramesInDPB: u32,
    pub ValidationFlags: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS,
    pub SupportFlags: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS,
    pub SuggestedProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub SuggestedLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub pResolutionDependentSupport: *mut D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS1,
    pub SubregionFrameEncodingData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA,
    pub MaxQualityVsSpeed: u32,
    pub QPMap: D3D12_VIDEO_ENCODER_QPMAP_CONFIGURATION,
    pub DirtyRegions: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_CONFIGURATION,
    pub MotionSearch: D3D12_VIDEO_ENCODER_MOTION_SEARCH_CONFIGURATION,
    pub FrameAnalysis: D3D12_VIDEO_ENCODER_FRAME_ANALYSIS_CONFIGURATION,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {
    pub NodeIndex: u32,
    pub CommandCount: u32,
    pub pCommandInfos: *mut D3D12_VIDEO_EXTENSION_COMMAND_INFO,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {
    pub NodeIndex: u32,
    pub CommandCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {
    pub CommandId: windows_sys::core::GUID,
    pub Stage: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE,
    pub ParameterCount: u32,
    pub pParameterInfos: *mut D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {
    pub CommandId: windows_sys::core::GUID,
    pub Stage: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE,
    pub ParameterCount: u32,
    pub ParameterPacking: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {
    pub NodeIndex: u32,
    pub CommandId: windows_sys::core::GUID,
    pub pCreationParameters: *const core::ffi::c_void,
    pub CreationParametersSizeInBytes: usize,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {
    pub NodeIndex: u32,
    pub CommandId: windows_sys::core::GUID,
    pub pInputData: *const core::ffi::c_void,
    pub InputDataSizeInBytes: usize,
    pub pOutputData: *mut core::ffi::c_void,
    pub OutputDataSizeInBytes: usize,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {
    pub NodeIndex: u32,
    pub VideoDecodeSupport: windows_sys::core::BOOL,
    pub VideoProcessSupport: windows_sys::core::BOOL,
    pub VideoEncodeSupport: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {
    pub NodeIndex: u32,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub BlockSizeFlags: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS,
    pub PrecisionFlags: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {
    pub NodeIndex: u32,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub Protected: windows_sys::core::BOOL,
    pub MotionVectorHeapMemoryPoolL0Size: u64,
    pub MotionVectorHeapMemoryPoolL1Size: u64,
    pub MotionEstimatorMemoryPoolL0Size: u64,
    pub MotionEstimatorMemoryPoolL1Size: u64,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {
    pub NodeMask: u32,
    pub pOutputStreamDesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC,
    pub NumInputStreamDescs: u32,
    pub pInputStreamDescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl Default for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {
    pub NodeMask: u32,
    pub pOutputStreamDesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC,
    pub NumInputStreamDescs: u32,
    pub pInputStreamDescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC,
    pub Protected: windows_sys::core::BOOL,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl Default for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {
    pub NodeIndex: u32,
    pub MaxInputStreams: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgicommon")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {
    pub NodeIndex: u32,
    pub DeinterlaceMode: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS,
    pub Filters: D3D12_VIDEO_PROCESS_FILTER_FLAGS,
    pub FeatureSupport: D3D12_VIDEO_PROCESS_FEATURE_FLAGS,
    pub InputFrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub OutputFrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub EnableAutoProcessing: windows_sys::core::BOOL,
    pub PastFrames: u32,
    pub FutureFrames: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {
    pub NodeIndex: u32,
    pub InputSample: D3D12_VIDEO_SAMPLE,
    pub InputFieldType: D3D12_VIDEO_FIELD_TYPE,
    pub InputStereoFormat: D3D12_VIDEO_FRAME_STEREO_FORMAT,
    pub InputFrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub OutputFormat: D3D12_VIDEO_FORMAT,
    pub OutputStereoFormat: D3D12_VIDEO_FRAME_STEREO_FORMAT,
    pub OutputFrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub SupportFlags: D3D12_VIDEO_PROCESS_SUPPORT_FLAGS,
    pub ScaleSupport: D3D12_VIDEO_SCALE_SUPPORT,
    pub FeatureSupport: D3D12_VIDEO_PROCESS_FEATURE_FLAGS,
    pub DeinterlaceSupport: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS,
    pub AutoProcessingSupport: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS,
    pub FilterSupport: D3D12_VIDEO_PROCESS_FILTER_FLAGS,
    pub FilterRangeSupport: [D3D12_VIDEO_PROCESS_FILTER_RANGE; 32],
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl Default for D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_FEATURE_VIDEO = i32;
pub const D3D12_FEATURE_VIDEO_ARCHITECTURE: D3D12_FEATURE_VIDEO = 17;
pub const D3D12_FEATURE_VIDEO_DECODER_HEAP_SIZE: D3D12_FEATURE_VIDEO = 8;
pub const D3D12_FEATURE_VIDEO_DECODER_HEAP_SIZE1: D3D12_FEATURE_VIDEO = 31;
pub const D3D12_FEATURE_VIDEO_DECODE_CONVERSION_SUPPORT: D3D12_FEATURE_VIDEO = 3;
pub const D3D12_FEATURE_VIDEO_DECODE_FORMATS: D3D12_FEATURE_VIDEO = 2;
pub const D3D12_FEATURE_VIDEO_DECODE_FORMAT_COUNT: D3D12_FEATURE_VIDEO = 11;
pub const D3D12_FEATURE_VIDEO_DECODE_HISTOGRAM: D3D12_FEATURE_VIDEO = 18;
pub const D3D12_FEATURE_VIDEO_DECODE_PROFILES: D3D12_FEATURE_VIDEO = 1;
pub const D3D12_FEATURE_VIDEO_DECODE_PROFILE_COUNT: D3D12_FEATURE_VIDEO = 10;
pub const D3D12_FEATURE_VIDEO_DECODE_PROTECTED_RESOURCES: D3D12_FEATURE_VIDEO = 28;
pub const D3D12_FEATURE_VIDEO_DECODE_SUPPORT: D3D12_FEATURE_VIDEO = 0;
pub const D3D12_FEATURE_VIDEO_ENCODER_CODEC: D3D12_FEATURE_VIDEO = 33;
pub const D3D12_FEATURE_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT: D3D12_FEATURE_VIDEO = 42;
pub const D3D12_FEATURE_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT: D3D12_FEATURE_VIDEO = 44;
pub const D3D12_FEATURE_VIDEO_ENCODER_DIRTY_REGIONS: D3D12_FEATURE_VIDEO = 51;
pub const D3D12_FEATURE_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG: D3D12_FEATURE_VIDEO = 46;
pub const D3D12_FEATURE_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE: D3D12_FEATURE_VIDEO = 40;
pub const D3D12_FEATURE_VIDEO_ENCODER_HEAP_SIZE: D3D12_FEATURE_VIDEO = 41;
pub const D3D12_FEATURE_VIDEO_ENCODER_HEAP_SIZE1: D3D12_FEATURE_VIDEO = 56;
pub const D3D12_FEATURE_VIDEO_ENCODER_INPUT_FORMAT: D3D12_FEATURE_VIDEO = 37;
pub const D3D12_FEATURE_VIDEO_ENCODER_INTRA_REFRESH_MODE: D3D12_FEATURE_VIDEO = 39;
pub const D3D12_FEATURE_VIDEO_ENCODER_MOTION_SEARCH: D3D12_FEATURE_VIDEO = 52;
pub const D3D12_FEATURE_VIDEO_ENCODER_OUTPUT_RESOLUTION: D3D12_FEATURE_VIDEO = 36;
pub const D3D12_FEATURE_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT: D3D12_FEATURE_VIDEO = 35;
pub const D3D12_FEATURE_VIDEO_ENCODER_PROFILE_LEVEL: D3D12_FEATURE_VIDEO = 34;
pub const D3D12_FEATURE_VIDEO_ENCODER_QPMAP_INPUT: D3D12_FEATURE_VIDEO = 50;
pub const D3D12_FEATURE_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS: D3D12_FEATURE_VIDEO = 57;
pub const D3D12_FEATURE_VIDEO_ENCODER_RATE_CONTROL_MODE: D3D12_FEATURE_VIDEO = 38;
pub const D3D12_FEATURE_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT: D3D12_FEATURE_VIDEO = 49;
pub const D3D12_FEATURE_VIDEO_ENCODER_RESOURCE_REQUIREMENTS: D3D12_FEATURE_VIDEO = 45;
pub const D3D12_FEATURE_VIDEO_ENCODER_RESOURCE_REQUIREMENTS1: D3D12_FEATURE_VIDEO = 48;
pub const D3D12_FEATURE_VIDEO_ENCODER_SUPPORT: D3D12_FEATURE_VIDEO = 43;
pub const D3D12_FEATURE_VIDEO_ENCODER_SUPPORT1: D3D12_FEATURE_VIDEO = 47;
pub const D3D12_FEATURE_VIDEO_ENCODER_SUPPORT2: D3D12_FEATURE_VIDEO = 55;
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMANDS: D3D12_FEATURE_VIDEO = 23;
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_COUNT: D3D12_FEATURE_VIDEO = 22;
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_PARAMETERS: D3D12_FEATURE_VIDEO = 25;
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT: D3D12_FEATURE_VIDEO = 24;
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_SIZE: D3D12_FEATURE_VIDEO = 27;
pub const D3D12_FEATURE_VIDEO_EXTENSION_COMMAND_SUPPORT: D3D12_FEATURE_VIDEO = 26;
pub const D3D12_FEATURE_VIDEO_FEATURE_AREA_SUPPORT: D3D12_FEATURE_VIDEO = 19;
pub const D3D12_FEATURE_VIDEO_MOTION_ESTIMATOR: D3D12_FEATURE_VIDEO = 20;
pub const D3D12_FEATURE_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES: D3D12_FEATURE_VIDEO = 30;
pub const D3D12_FEATURE_VIDEO_MOTION_ESTIMATOR_SIZE: D3D12_FEATURE_VIDEO = 21;
pub const D3D12_FEATURE_VIDEO_PROCESSOR_SIZE: D3D12_FEATURE_VIDEO = 9;
pub const D3D12_FEATURE_VIDEO_PROCESSOR_SIZE1: D3D12_FEATURE_VIDEO = 32;
pub const D3D12_FEATURE_VIDEO_PROCESS_MAX_INPUT_STREAMS: D3D12_FEATURE_VIDEO = 6;
pub const D3D12_FEATURE_VIDEO_PROCESS_PROTECTED_RESOURCES: D3D12_FEATURE_VIDEO = 29;
pub const D3D12_FEATURE_VIDEO_PROCESS_REFERENCE_INFO: D3D12_FEATURE_VIDEO = 7;
pub const D3D12_FEATURE_VIDEO_PROCESS_SUPPORT: D3D12_FEATURE_VIDEO = 5;
#[repr(C)]
#[cfg(feature = "Win32_dxgicommon")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {
    pub Status: u64,
    pub NumMacroblocksAffected: u64,
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub BitRate: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {
    pub pMotionVectorHeap: *mut core::ffi::c_void,
    pub PixelWidth: u32,
    pub PixelHeight: u32,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {
    pub pMotionVectorTexture2D: *mut core::ffi::c_void,
    pub MotionVectorCoordinate: D3D12_RESOURCE_COORDINATE,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_RESOURCE_COORDINATE {
    pub X: u64,
    pub Y: u32,
    pub Z: u32,
    pub SubresourceIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_DECODER_DESC {
    pub NodeMask: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_DECODER_HEAP_DESC {
    pub NodeMask: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub DecodeWidth: u32,
    pub DecodeHeight: u32,
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub BitRate: u32,
    pub MaxDecodePictureBufferCount: u32,
}
pub type D3D12_VIDEO_DECODE_ARGUMENT_TYPE = i32;
pub const D3D12_VIDEO_DECODE_ARGUMENT_TYPE_HUFFMAN_TABLE: D3D12_VIDEO_DECODE_ARGUMENT_TYPE = 3;
pub const D3D12_VIDEO_DECODE_ARGUMENT_TYPE_INVERSE_QUANTIZATION_MATRIX: D3D12_VIDEO_DECODE_ARGUMENT_TYPE = 1;
pub const D3D12_VIDEO_DECODE_ARGUMENT_TYPE_PICTURE_PARAMETERS: D3D12_VIDEO_DECODE_ARGUMENT_TYPE = 0;
pub const D3D12_VIDEO_DECODE_ARGUMENT_TYPE_SLICE_CONTROL: D3D12_VIDEO_DECODE_ARGUMENT_TYPE = 2;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {
    pub pBuffer: *mut core::ffi::c_void,
    pub Offset: u64,
    pub Size: u64,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_DECODE_CONFIGURATION {
    pub DecodeProfile: windows_sys::core::GUID,
    pub BitstreamEncryption: D3D12_BITSTREAM_ENCRYPTION_TYPE,
    pub InterlaceType: D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE,
}
pub type D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = u32;
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_ALLOW_RESOLUTION_CHANGE_ON_NON_KEY_FRAME: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 8;
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_HEIGHT_ALIGNMENT_MULTIPLE_32_REQUIRED: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 1;
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_NONE: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 0;
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_POST_PROCESSING_SUPPORTED: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 2;
pub const D3D12_VIDEO_DECODE_CONFIGURATION_FLAG_REFERENCE_ONLY_ALLOCATIONS_REQUIRED: D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS = 4;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {
    pub Enable: windows_sys::core::BOOL,
    pub pReferenceTexture2D: *mut core::ffi::c_void,
    pub ReferenceSubresource: u32,
    pub OutputColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub DecodeColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
impl Default for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {
    pub Enable: windows_sys::core::BOOL,
    pub pReferenceTexture2D: *mut core::ffi::c_void,
    pub ReferenceSubresource: u32,
    pub OutputColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub DecodeColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
impl Default for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAG_NONE: D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_FRAME_ARGUMENT {
    pub Type: D3D12_VIDEO_DECODE_ARGUMENT_TYPE,
    pub Size: u32,
    pub pData: *mut core::ffi::c_void,
}
impl Default for D3D12_VIDEO_DECODE_FRAME_ARGUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = i32;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_A: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 3;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_B: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 2;
pub type D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = u32;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_A: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 8;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_B: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 4;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_G: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 2;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_NONE: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 0;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_R: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 1;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_U: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 2;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_V: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 4;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAG_Y: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS = 1;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_G: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 1;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_R: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 0;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_U: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 1;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_V: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 2;
pub const D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_Y: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT = 0;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    pub NumFrameArguments: u32,
    pub FrameArguments: [D3D12_VIDEO_DECODE_FRAME_ARGUMENT; 10],
    pub ReferenceFrames: D3D12_VIDEO_DECODE_REFERENCE_FRAMES,
    pub CompressedBitstream: D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM,
    pub pHeap: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {
    pub Offset: u64,
    pub pBuffer: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {
    pub pOutputTexture2D: *mut core::ffi::c_void,
    pub OutputSubresource: u32,
    pub ConversionArguments: D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
impl Default for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {
    pub pOutputTexture2D: *mut core::ffi::c_void,
    pub OutputSubresource: u32,
    pub ConversionArguments: D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1,
    pub Histograms: [D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM; 4],
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
impl Default for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_12BIT_PROFILE2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x17127009_a00f_4ce1_994e_bf4081f6f3f0);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_12BIT_PROFILE2_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d80bed6_9cac_4835_9e91_327bbc4f9ee8);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb8be4ccb_cf53_46ba_8d59_d6b8a6da5d2a);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6936ff0f_45b1_4163_9cc1_646ef6946108);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0c5f2aa1_e541_4089_bb7b_98110a19d7c8);
pub const D3D12_VIDEO_DECODE_PROFILE_H264: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_MULTIVIEW: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_STEREO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_STEREO_PROGRESSIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0bac4fe5_1532_4429_a854_f84de04953db);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0dabeffa_4458_4602_bc03_0795659d617c);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10_EXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9cc55490_e37c_4932_8684_4920f9f6409c);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1a72925f_0c2c_4f15_96fb_b17d1473603f);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN12_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55bcac81_f311_4093_a7d0_1cbc0b849bee);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN12_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9798634d_fe9d_48e5_b4da_dbec45b3df01);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN16: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4fbdbb0_a113_482b_a232_635cc0697f6d);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4008018f_f537_4b36_98cf_61af8a2c1a33);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MONOCHROME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0685b993_3d8c_43a0_8b28_d74c2d6899a4);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MONOCHROME10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x142a1d0f_69dd_4ec9_8591_b12ffcb91a29);
pub const D3D12_VIDEO_DECODE_PROFILE_JPEG_VLD_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf782c83_bef5_4a2c_87cb_6019e7b175ac);
pub const D3D12_VIDEO_DECODE_PROFILE_JPEG_VLD_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf04df417_eee2_4067_a778_f35c15ab9721);
pub const D3D12_VIDEO_DECODE_PROFILE_JPEG_VLD_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4cd00e17_89ba_48ef_b9f9_edcb82713f65);
pub const D3D12_VIDEO_DECODE_PROFILE_MJPEG_VLD_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x725cb506_0c29_43c4_9440_8e9397903a04);
pub const D3D12_VIDEO_DECODE_PROFILE_MJPEG_VLD_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b77b9cd_1a35_4c30_9fd8_ef4b60c035dd);
pub const D3D12_VIDEO_DECODE_PROFILE_MJPEG_VLD_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd95161f9_0d44_47e6_bcf5_1bfbfb268f97);
pub const D3D12_VIDEO_DECODE_PROFILE_MJPEG_VLD_4444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc91748d5_fd18_4aca_9db3_3a6634ab547d);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG1_AND_MPEG2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG4PT2_ADVSIMPLE_NOGMC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG4PT2_SIMPLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const D3D12_VIDEO_DECODE_PROFILE_VC1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_VC1_D2010: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_VP8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const D3D12_VIDEO_DECODE_PROFILE_VP9: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const D3D12_VIDEO_DECODE_PROFILE_VP9_10BIT_PROFILE2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_DECODE_REFERENCE_FRAMES {
    pub NumTexture2Ds: u32,
    pub ppTexture2Ds: *mut *mut core::ffi::c_void,
    pub pSubresources: *mut u32,
    pub ppHeaps: *mut *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_DECODE_REFERENCE_FRAMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_DECODE_STATUS = i32;
pub const D3D12_VIDEO_DECODE_STATUS_CONTINUE: D3D12_VIDEO_DECODE_STATUS = 1;
pub const D3D12_VIDEO_DECODE_STATUS_CONTINUE_SKIP_DISPLAY: D3D12_VIDEO_DECODE_STATUS = 2;
pub const D3D12_VIDEO_DECODE_STATUS_OK: D3D12_VIDEO_DECODE_STATUS = 0;
pub const D3D12_VIDEO_DECODE_STATUS_RATE_EXCEEDED: D3D12_VIDEO_DECODE_STATUS = 4;
pub const D3D12_VIDEO_DECODE_STATUS_RESTART: D3D12_VIDEO_DECODE_STATUS = 3;
pub type D3D12_VIDEO_DECODE_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_DECODE_SUPPORT_FLAG_NONE: D3D12_VIDEO_DECODE_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_DECODE_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_DECODE_SUPPORT_FLAGS = 1;
pub type D3D12_VIDEO_DECODE_TIER = i32;
pub const D3D12_VIDEO_DECODE_TIER_1: D3D12_VIDEO_DECODE_TIER = 1;
pub const D3D12_VIDEO_DECODE_TIER_2: D3D12_VIDEO_DECODE_TIER = 2;
pub const D3D12_VIDEO_DECODE_TIER_3: D3D12_VIDEO_DECODE_TIER = 3;
pub const D3D12_VIDEO_DECODE_TIER_NOT_SUPPORTED: D3D12_VIDEO_DECODE_TIER = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_CDEF_CONFIG {
    pub CdefBits: u64,
    pub CdefDampingMinus3: u64,
    pub CdefYPriStrength: [u64; 8],
    pub CdefUVPriStrength: [u64; 8],
    pub CdefYSecStrength: [u64; 8],
    pub CdefUVSecStrength: [u64; 8],
}
impl Default for D3D12_VIDEO_ENCODER_AV1_CDEF_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_AV1_CODEC_CONFIGURATION {
    pub FeatureFlags: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS,
    pub OrderHintBitsMinus1: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_CODEC_CONFIGURATION_SUPPORT {
    pub SupportedFeatureFlags: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS,
    pub RequiredFeatureFlags: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS,
    pub SupportedInterpolationFilters: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAGS,
    pub SupportedRestorationParams: [[D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAGS; 3]; 3],
    pub SupportedSegmentationModes: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS,
    pub SupportedTxModes: [D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAGS; 4],
    pub SegmentationBlockSize: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE,
    pub PostEncodeValuesFlags: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS,
    pub MaxTemporalLayers: u32,
    pub MaxSpatialLayers: u32,
}
impl Default for D3D12_VIDEO_ENCODER_AV1_CODEC_CONFIGURATION_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_AV1_COMP_PREDICTION_TYPE = i32;
pub const D3D12_VIDEO_ENCODER_AV1_COMP_PREDICTION_TYPE_COMPOUND_REFERENCE: D3D12_VIDEO_ENCODER_AV1_COMP_PREDICTION_TYPE = 1;
pub const D3D12_VIDEO_ENCODER_AV1_COMP_PREDICTION_TYPE_SINGLE_REFERENCE: D3D12_VIDEO_ENCODER_AV1_COMP_PREDICTION_TYPE = 0;
pub type D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_128x128_SUPERBLOCK: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_ALLOW_HIGH_PRECISION_MV: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 8388608;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_AUTO_SEGMENTATION: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 65536;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_CDEF_FILTERING: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 4096;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_CUSTOM_SEGMENTATION: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 131072;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_DELTA_LF_PARAMS: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 33554432;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_DUAL_FILTER: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_FILTER_INTRA: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_FORCED_INTEGER_MOTION_VECTORS: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 256;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_FRAME_REFERENCE_MOTION_VECTORS: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 16384;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_INTERINTRA_COMPOUND: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_INTRA_BLOCK_COPY: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 8192;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_INTRA_EDGE_FILTER: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_JNT_COMP: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_LOOP_FILTER_DELTAS: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 262144;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_LOOP_RESTORATION_FILTER: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 1024;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_MASKED_COMPOUND: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_MOTION_MODE_SWITCHABLE: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 4194304;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_ORDER_HINT_TOOLS: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 32768;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_PALETTE_ENCODING: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 2048;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_QUANTIZATION_DELTAS: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 524288;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_QUANTIZATION_MATRIX: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 1048576;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_REDUCED_TX_SET: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 2097152;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_SKIP_MODE_PRESENT: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 16777216;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_SUPER_RESOLUTION: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 512;
pub const D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAG_WARPED_MOTION: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT {
    pub Use128SuperBlocks: windows_sys::core::BOOL,
    pub TilesConfiguration: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_TILES,
    pub ValidationFlags: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS,
    pub MinTileRows: u32,
    pub MaxTileRows: u32,
    pub MinTileCols: u32,
    pub MaxTileCols: u32,
    pub MinTileWidth: u32,
    pub MaxTileWidth: u32,
    pub MinTileArea: u32,
    pub MaxTileArea: u32,
    pub TileSizeBytesMinus1: u32,
}
pub type D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_AREA: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_CODEC_CONSTRAINT: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_COLS_COUNT: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_HARDWARE_CONSTRAINT: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_NOT_SPECIFIED: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_ROWS_COUNT: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_TOTAL_TILES: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAG_WIDTH: D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_VALIDATION_FLAGS = 32;
pub type D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE = i32;
pub type D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAG_INTER_FRAME: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAG_INTRA_ONLY_FRAME: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAG_KEY_FRAME: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAG_SWITCH_FRAME: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_INTER_FRAME: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE = 1;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_INTRA_ONLY_FRAME: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE = 2;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_KEY_FRAME: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE = 0;
pub const D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_SWITCH_FRAME: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE = 3;
pub type D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS = i32;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_BILINEAR: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS = 3;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_EIGHTTAP: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_EIGHTTAP_SHARP: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_EIGHTTAP_SMOOTH: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS = 1;
pub type D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAG_BILINEAR: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAG_EIGHTTAP: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAG_EIGHTTAP_SHARP: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAG_EIGHTTAP_SMOOTH: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAG_SWITCHABLE: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS_SWITCHABLE: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS = 4;
pub type D3D12_VIDEO_ENCODER_AV1_LEVELS = i32;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_2_0: D3D12_VIDEO_ENCODER_AV1_LEVELS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_2_1: D3D12_VIDEO_ENCODER_AV1_LEVELS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_2_2: D3D12_VIDEO_ENCODER_AV1_LEVELS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_2_3: D3D12_VIDEO_ENCODER_AV1_LEVELS = 3;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_3_0: D3D12_VIDEO_ENCODER_AV1_LEVELS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_3_1: D3D12_VIDEO_ENCODER_AV1_LEVELS = 5;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_3_2: D3D12_VIDEO_ENCODER_AV1_LEVELS = 6;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_3_3: D3D12_VIDEO_ENCODER_AV1_LEVELS = 7;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_4_0: D3D12_VIDEO_ENCODER_AV1_LEVELS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_4_1: D3D12_VIDEO_ENCODER_AV1_LEVELS = 9;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_4_2: D3D12_VIDEO_ENCODER_AV1_LEVELS = 10;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_4_3: D3D12_VIDEO_ENCODER_AV1_LEVELS = 11;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_5_0: D3D12_VIDEO_ENCODER_AV1_LEVELS = 12;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_5_1: D3D12_VIDEO_ENCODER_AV1_LEVELS = 13;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_5_2: D3D12_VIDEO_ENCODER_AV1_LEVELS = 14;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_5_3: D3D12_VIDEO_ENCODER_AV1_LEVELS = 15;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_6_0: D3D12_VIDEO_ENCODER_AV1_LEVELS = 16;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_6_1: D3D12_VIDEO_ENCODER_AV1_LEVELS = 17;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_6_2: D3D12_VIDEO_ENCODER_AV1_LEVELS = 18;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_6_3: D3D12_VIDEO_ENCODER_AV1_LEVELS = 19;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_7_0: D3D12_VIDEO_ENCODER_AV1_LEVELS = 20;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_7_1: D3D12_VIDEO_ENCODER_AV1_LEVELS = 21;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_7_2: D3D12_VIDEO_ENCODER_AV1_LEVELS = 22;
pub const D3D12_VIDEO_ENCODER_AV1_LEVELS_7_3: D3D12_VIDEO_ENCODER_AV1_LEVELS = 23;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_AV1_LEVEL_TIER_CONSTRAINTS {
    pub Level: D3D12_VIDEO_ENCODER_AV1_LEVELS,
    pub Tier: D3D12_VIDEO_ENCODER_AV1_TIER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_CODEC_DATA {
    pub Flags: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS,
    pub FrameType: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE,
    pub CompoundPredictionType: D3D12_VIDEO_ENCODER_AV1_COMP_PREDICTION_TYPE,
    pub InterpolationFilter: D3D12_VIDEO_ENCODER_AV1_INTERPOLATION_FILTERS,
    pub FrameRestorationConfig: D3D12_VIDEO_ENCODER_AV1_RESTORATION_CONFIG,
    pub TxMode: D3D12_VIDEO_ENCODER_AV1_TX_MODE,
    pub SuperResDenominator: u32,
    pub OrderHint: u32,
    pub PictureIndex: u32,
    pub TemporalLayerIndexPlus1: u32,
    pub SpatialLayerIndexPlus1: u32,
    pub ReferenceFramesReconPictureDescriptors: [D3D12_VIDEO_ENCODER_AV1_REFERENCE_PICTURE_DESCRIPTOR; 8],
    pub ReferenceIndices: [u32; 7],
    pub PrimaryRefFrame: u32,
    pub RefreshFrameFlags: u32,
    pub LoopFilter: D3D12_VIDEO_ENCODER_CODEC_AV1_LOOP_FILTER_CONFIG,
    pub LoopFilterDelta: D3D12_VIDEO_ENCODER_CODEC_AV1_LOOP_FILTER_DELTA_CONFIG,
    pub Quantization: D3D12_VIDEO_ENCODER_CODEC_AV1_QUANTIZATION_CONFIG,
    pub QuantizationDelta: D3D12_VIDEO_ENCODER_CODEC_AV1_QUANTIZATION_DELTA_CONFIG,
    pub CDEF: D3D12_VIDEO_ENCODER_AV1_CDEF_CONFIG,
    pub QPMapValuesCount: u32,
    pub pRateControlQPMap: *mut i16,
    pub CustomSegmentation: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_CONFIG,
    pub CustomSegmentsMap: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MAP,
}
impl Default for D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_CODEC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_ALLOW_HIGH_PRECISION_MV: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 16384;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_ALLOW_INTRA_BLOCK_COPY: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_DISABLE_CDF_UPDATE: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_DISABLE_FRAME_END_UPDATE_CDF: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 256;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_ENABLE_ERROR_RESILIENT_MODE: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_ENABLE_FRAME_SEGMENTATION_AUTO: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 512;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_ENABLE_FRAME_SEGMENTATION_CUSTOM: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 1024;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_ENABLE_PALETTE_ENCODING: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_ENABLE_SKIP_MODE: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_ENABLE_WARPED_MOTION: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 2048;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_FORCE_INTEGER_MOTION_VECTORS: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_FRAME_REFERENCE_MOTION_VECTORS: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_MOTION_MODE_SWITCHABLE: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 8192;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_REDUCED_TX_SET: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 4096;
pub const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAG_USE_SUPER_RESOLUTION: D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_FLAGS = 128;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_TILES {
    pub RowCount: u64,
    pub ColCount: u64,
    pub RowHeights: [u64; 64],
    pub ColWidths: [u64; 64],
    pub ContextUpdateTileId: u64,
}
impl Default for D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_TILES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES {
    pub CompoundPredictionType: u64,
    pub LoopFilter: D3D12_VIDEO_ENCODER_CODEC_AV1_LOOP_FILTER_CONFIG,
    pub LoopFilterDelta: D3D12_VIDEO_ENCODER_CODEC_AV1_LOOP_FILTER_DELTA_CONFIG,
    pub Quantization: D3D12_VIDEO_ENCODER_CODEC_AV1_QUANTIZATION_CONFIG,
    pub QuantizationDelta: D3D12_VIDEO_ENCODER_CODEC_AV1_QUANTIZATION_DELTA_CONFIG,
    pub CDEF: D3D12_VIDEO_ENCODER_AV1_CDEF_CONFIG,
    pub SegmentationConfig: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_CONFIG,
    pub PrimaryRefFrame: u64,
    pub ReferenceIndices: [u64; 7],
}
impl Default for D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_CDEF_DATA: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_COMPOUND_PREDICTION_MODE: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_CONTEXT_UPDATE_TILE_ID: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_LOOP_FILTER: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_LOOP_FILTER_DELTA: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_PRIMARY_REF_FRAME: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_QUANTIZATION: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_QUANTIZATION_DELTA: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAG_REFERENCE_INDICES: D3D12_VIDEO_ENCODER_AV1_POST_ENCODE_VALUES_FLAGS = 256;
pub type D3D12_VIDEO_ENCODER_AV1_PROFILE = i32;
pub const D3D12_VIDEO_ENCODER_AV1_PROFILE_HIGH: D3D12_VIDEO_ENCODER_AV1_PROFILE = 1;
pub const D3D12_VIDEO_ENCODER_AV1_PROFILE_MAIN: D3D12_VIDEO_ENCODER_AV1_PROFILE = 0;
pub const D3D12_VIDEO_ENCODER_AV1_PROFILE_PROFESSIONAL: D3D12_VIDEO_ENCODER_AV1_PROFILE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_AV1_REFERENCE_PICTURE_DESCRIPTOR {
    pub ReconstructedPictureResourceIndex: u32,
    pub TemporalLayerIndexPlus1: u32,
    pub SpatialLayerIndexPlus1: u32,
    pub FrameType: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE,
    pub WarpedMotionInfo: D3D12_VIDEO_ENCODER_AV1_REFERENCE_PICTURE_WARPED_MOTION_INFO,
    pub OrderHint: u32,
    pub PictureIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_REFERENCE_PICTURE_WARPED_MOTION_INFO {
    pub TransformationType: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION,
    pub TransformationMatrix: [i32; 8],
    pub InvalidAffineSet: windows_sys::core::BOOL,
}
impl Default for D3D12_VIDEO_ENCODER_AV1_REFERENCE_PICTURE_WARPED_MOTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION = i32;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_AFFINE: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION = 3;
pub type D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAG_AFFINE: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAG_IDENTITY: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAG_ROTZOOM: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAG_TRANSLATION: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_IDENTITY: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION = 0;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_ROTZOOM: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION = 2;
pub const D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_TRANSLATION: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_RESTORATION_CONFIG {
    pub FrameRestorationType: [D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE; 3],
    pub LoopRestorationPixelSize: [D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE; 3],
}
impl Default for D3D12_VIDEO_ENCODER_AV1_RESTORATION_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAG_128x128: D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAG_256x256: D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAG_32x32: D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAG_64x64: D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAG_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_AV1_RESTORATION_SUPPORT_FLAGS = 0;
pub type D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE = i32;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE_128x128: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE = 3;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE_256x256: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE = 4;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE_32x32: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE = 1;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE_64x64: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE = 2;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE_DISABLED: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TILESIZE = 0;
pub type D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE = i32;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE_DISABLED: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE = 0;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE_SGRPROJ: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE = 3;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE_SWITCHABLE: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE = 1;
pub const D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE_WIENER: D3D12_VIDEO_ENCODER_AV1_RESTORATION_TYPE = 2;
pub type D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE = i32;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE_16x16: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE = 2;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE_32x32: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE = 3;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE_4x4: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE = 0;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE_64x64: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE = 4;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE_8x8: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_BLOCK_SIZE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_CONFIG {
    pub UpdateMap: u64,
    pub TemporalUpdate: u64,
    pub UpdateData: u64,
    pub NumSegments: u64,
    pub SegmentsData: [D3D12_VIDEO_ENCODER_AV1_SEGMENT_DATA; 8],
}
impl Default for D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MAP {
    pub SegmentsMapByteSize: u32,
    pub pSegmentsMap: *mut u8,
}
impl Default for D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = i32;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_ALT_GLOBALMV: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 8;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_ALT_LF_U: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 4;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_ALT_LF_V: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 5;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_ALT_LF_Y_H: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 3;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_ALT_LF_Y_V: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 2;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_ALT_Q: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 1;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_ALT_REF_FRAME: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 6;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_ALT_SKIP: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 7;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_DISABLED: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE = 0;
pub type D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_ALT_GLOBALMV: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 256;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_ALT_LF_U: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_ALT_LF_V: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_ALT_LF_Y_H: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_ALT_LF_Y_V: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_ALT_Q: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_ALT_SKIP: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_DISABLED: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAG_REF_FRAME: D3D12_VIDEO_ENCODER_AV1_SEGMENTATION_MODE_FLAGS = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_AV1_SEGMENT_DATA {
    pub EnabledFeatures: u64,
    pub FeatureValue: [i64; 8],
}
impl Default for D3D12_VIDEO_ENCODER_AV1_SEGMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_AV1_SEQUENCE_STRUCTURE {
    pub IntraDistance: u32,
    pub InterFramePeriod: u32,
}
pub type D3D12_VIDEO_ENCODER_AV1_TIER = i32;
pub const D3D12_VIDEO_ENCODER_AV1_TIER_HIGH: D3D12_VIDEO_ENCODER_AV1_TIER = 1;
pub const D3D12_VIDEO_ENCODER_AV1_TIER_MAIN: D3D12_VIDEO_ENCODER_AV1_TIER = 0;
pub type D3D12_VIDEO_ENCODER_AV1_TX_MODE = i32;
pub type D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAG_LARGEST: D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAG_NONE: D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAG_ONLY4x4: D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAG_SELECT: D3D12_VIDEO_ENCODER_AV1_TX_MODE_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_AV1_TX_MODE_LARGEST: D3D12_VIDEO_ENCODER_AV1_TX_MODE = 1;
pub const D3D12_VIDEO_ENCODER_AV1_TX_MODE_ONLY4x4: D3D12_VIDEO_ENCODER_AV1_TX_MODE = 0;
pub const D3D12_VIDEO_ENCODER_AV1_TX_MODE_SELECT: D3D12_VIDEO_ENCODER_AV1_TX_MODE = 2;
pub type D3D12_VIDEO_ENCODER_CODEC = i32;
pub const D3D12_VIDEO_ENCODER_CODEC_AV1: D3D12_VIDEO_ENCODER_CODEC = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_CODEC_AV1_LOOP_FILTER_CONFIG {
    pub LoopFilterLevel: [u64; 2],
    pub LoopFilterLevelU: u64,
    pub LoopFilterLevelV: u64,
    pub LoopFilterSharpnessLevel: u64,
    pub LoopFilterDeltaEnabled: u64,
    pub UpdateRefDelta: u64,
    pub RefDeltas: [i64; 8],
    pub UpdateModeDelta: u64,
    pub ModeDeltas: [i64; 2],
}
impl Default for D3D12_VIDEO_ENCODER_CODEC_AV1_LOOP_FILTER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_AV1_LOOP_FILTER_DELTA_CONFIG {
    pub DeltaLFPresent: u64,
    pub DeltaLFMulti: u64,
    pub DeltaLFRes: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_AV1_PICTURE_CONTROL_SUPPORT {
    pub PredictionMode: D3D12_VIDEO_ENCODER_AV1_COMP_PREDICTION_TYPE,
    pub MaxUniqueReferencesPerFrame: u32,
    pub SupportedFrameTypes: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAGS,
    pub SupportedReferenceWarpedMotionFlags: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_AV1_QUANTIZATION_CONFIG {
    pub BaseQIndex: u64,
    pub YDCDeltaQ: i64,
    pub UDCDeltaQ: i64,
    pub UACDeltaQ: i64,
    pub VDCDeltaQ: i64,
    pub VACDeltaQ: i64,
    pub UsingQMatrix: u64,
    pub QMY: u64,
    pub QMU: u64,
    pub QMV: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_AV1_QUANTIZATION_DELTA_CONFIG {
    pub DeltaQPresent: u64,
    pub DeltaQRes: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_0,
}
impl Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_0 {
    pub pH264Config: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264,
    pub pHEVCConfig: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC,
    pub pAV1Config: *mut D3D12_VIDEO_ENCODER_AV1_CODEC_CONFIGURATION,
}
impl Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264 {
    pub ConfigurationFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS,
    pub DirectModeConfig: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES,
    pub DisableDeblockingFilterConfig: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES,
}
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES = i32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES_DISABLED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES_SPATIAL: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES_TEMPORAL: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES = 1;
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_ALLOW_REQUEST_INTRA_CONSTRAINED_SLICES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_ENABLE_CABAC_ENCODING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_USE_ADAPTIVE_8x8_TRANSFORM: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAG_USE_CONSTRAINED_INTRAPREDICTION: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS = 1;
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = i32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_0_ALL_LUMA_CHROMA_SLICE_BLOCK_EDGES_ALWAYS_FILTERED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_1_DISABLE_ALL_SLICE_BLOCK_EDGES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 1;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_2_DISABLE_SLICE_BOUNDARIES_BLOCKS: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_3_USE_TWO_STAGE_DEBLOCKING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 3;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_4_DISABLE_CHROMA_BLOCK_EDGES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 4;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_5_DISABLE_CHROMA_BLOCK_EDGES_AND_LUMA_BOUNDARIES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 5;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_6_DISABLE_CHROMA_BLOCK_EDGES_AND_USE_LUMA_TWO_STAGE_DEBLOCKING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES = 6;
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_0_ALL_LUMA_CHROMA_SLICE_BLOCK_EDGES_ALWAYS_FILTERED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_1_DISABLE_ALL_SLICE_BLOCK_EDGES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_2_DISABLE_SLICE_BOUNDARIES_BLOCKS: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_3_USE_TWO_STAGE_DEBLOCKING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_4_DISABLE_CHROMA_BLOCK_EDGES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_5_DISABLE_CHROMA_BLOCK_EDGES_AND_LUMA_BOUNDARIES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_6_DISABLE_CHROMA_BLOCK_EDGES_AND_USE_LUMA_TWO_STAGE_DEBLOCKING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC {
    pub ConfigurationFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS,
    pub MinLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MaxLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MinLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub MaxLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
}
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = i32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE_16x16: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = 1;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE_32x32: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE_64x64: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = 3;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE_8x8: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE = 0;
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_ALLOW_REQUEST_INTRA_CONSTRAINED_SLICES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_CABAC_BYPASS_ALIGNMENT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 32768;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_DISABLE_LOOP_FILTER_ACROSS_SLICES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_ENABLE_LONG_TERM_REFERENCES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_ENABLE_SAO_FILTER: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_ENABLE_TRANSFORM_SKIPPING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_EXPLICIT_RDPCM: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 1024;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_EXTENDED_PRECISION_PROCESSING: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 2048;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_HIGH_PRECISION_OFFSETS: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 8192;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_IMPLICIT_RDPCM: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 512;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_INTRA_SMOOTHING_DISABLED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 4096;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_PERSISTENT_RICE_ADAPTATION: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 16384;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_SEPARATE_COLOUR_PLANE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 65536;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_TRANSFORM_SKIP_CONTEXT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 256;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_TRANSFORM_SKIP_ROTATION: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_USE_ASYMETRIC_MOTION_PARTITION: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAG_USE_CONSTRAINED_INTRAPREDICTION: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS = 64;
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = i32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE_16x16: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE_32x32: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = 3;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE_4x4: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE_8x8: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_0,
}
impl Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_0 {
    pub pH264Support: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264,
    pub pHEVCSupport: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC,
    pub pHEVCSupport1: *mut D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC1,
    pub pAV1Support: *mut D3D12_VIDEO_ENCODER_AV1_CODEC_CONFIGURATION_SUPPORT,
}
impl Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264 {
    pub SupportFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS,
    pub DisableDeblockingFilterSupportedModes: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS,
}
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_ADAPTIVE_8x8_TRANSFORM_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_BFRAME_LTR_COMBINED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_CABAC_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_CONSTRAINED_INTRAPREDICTION_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_DIRECT_SPATIAL_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_DIRECT_TEMPORAL_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_INTRA_SLICE_CONSTRAINED_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAG_NUM_REF_IDX_ACTIVE_OVERRIDE_FLAG_SLICE_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS = 128;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC {
    pub SupportFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS,
    pub MinLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MaxLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MinLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub MaxLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC1 {
    pub SupportFlags: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS,
    pub MinLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MaxLumaCodingUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE,
    pub MinLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub MaxLumaTransformUnitSize: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub allowed_diff_cu_chroma_qp_offset_depth_values: u32,
    pub allowed_log2_sao_offset_scale_luma_values: u32,
    pub allowed_log2_sao_offset_scale_chroma_values: u32,
    pub allowed_log2_max_transform_skip_block_size_minus2_values: u32,
    pub allowed_chroma_qp_offset_list_len_minus1_values: u32,
    pub allowed_cb_qp_offset_list_values: [u32; 6],
    pub allowed_cr_qp_offset_list_values: [u32; 6],
    pub SupportFlags1: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS1,
}
impl Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG1_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS1 = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG1_SEPARATE_COLOUR_PLANE_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS1 = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG1_SEPARATE_COLOUR_PLANE_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS1 = 1;
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = u32;
pub type D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS1 = u32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_ASYMETRIC_MOTION_PARTITION_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_ASYMETRIC_MOTION_PARTITION_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_BFRAME_LTR_COMBINED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_CABAC_BYPASS_ALIGNMENT_ENABLED_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 134217728;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_CABAC_BYPASS_ALIGNMENT_ENABLED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 67108864;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_CHROMA_QP_OFFSET_LIST_ENABLED_FLAG_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 2147483648;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_CHROMA_QP_OFFSET_LIST_ENABLED_FLAG_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 1073741824;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_CONSTRAINED_INTRAPREDICTION_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_CROSS_COMPONENT_PREDICTION_ENABLED_FLAG_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 536870912;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_CROSS_COMPONENT_PREDICTION_ENABLED_FLAG_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 268435456;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_DISABLING_LOOP_FILTER_ACROSS_SLICES_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_EXPLICIT_RDPCM_ENABLED_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 131072;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_EXPLICIT_RDPCM_ENABLED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 65536;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_EXTENDED_PRECISION_PROCESSING_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 524288;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_EXTENDED_PRECISION_PROCESSING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 262144;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_HIGH_PRECISION_OFFSETS_ENABLED_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 8388608;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_HIGH_PRECISION_OFFSETS_ENABLED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 4194304;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_IMPLICIT_RDPCM_ENABLED_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 32768;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_IMPLICIT_RDPCM_ENABLED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 16384;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_INTRA_SLICE_CONSTRAINED_ENCODING_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_INTRA_SMOOTHING_DISABLED_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 2097152;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_INTRA_SMOOTHING_DISABLED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 1048576;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_NONE: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_NUM_REF_IDX_ACTIVE_OVERRIDE_FLAG_SLICE_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 512;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_PERSISTENT_RICE_ADAPTATION_ENABLED_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 33554432;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_PERSISTENT_RICE_ADAPTATION_ENABLED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 16777216;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_P_FRAMES_IMPLEMENTED_AS_LOW_DELAY_B_FRAMES: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 256;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_SAO_FILTER_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_TRANSFORM_SKIP_CONTEXT_ENABLED_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 8192;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_TRANSFORM_SKIP_CONTEXT_ENABLED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 4096;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_TRANSFORM_SKIP_ROTATION_ENABLED_REQUIRED: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 2048;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_TRANSFORM_SKIP_ROTATION_ENABLED_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 1024;
pub const D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAG_TRANSFORM_SKIP_SUPPORT: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_CODEC_H264: D3D12_VIDEO_ENCODER_CODEC = 0;
pub const D3D12_VIDEO_ENCODER_CODEC_HEVC: D3D12_VIDEO_ENCODER_CODEC = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_0,
}
impl Default for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_0 {
    pub pH264Support: *mut D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264,
    pub pHEVCSupport: *mut D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC,
    pub pAV1Support: *mut D3D12_VIDEO_ENCODER_CODEC_AV1_PICTURE_CONTROL_SUPPORT,
}
impl Default for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {
    pub MaxL0ReferencesForP: u32,
    pub MaxL0ReferencesForB: u32,
    pub MaxL1ReferencesForB: u32,
    pub MaxLongTermReferences: u32,
    pub MaxDPBCapacity: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {
    pub MaxL0ReferencesForP: u32,
    pub MaxL0ReferencesForB: u32,
    pub MaxL1ReferencesForB: u32,
    pub MaxLongTermReferences: u32,
    pub MaxDPBCapacity: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {
    pub pBuffer: *mut core::ffi::c_void,
    pub FrameStartOffset: u64,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1 {
    pub NotificationMode: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM_NOTIFICATION_MODE,
    pub Anonymous: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1_0,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1_0 {
    pub FrameOutputBuffer: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM,
    pub SubregionOutputBuffers: D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM_NOTIFICATION_MODE = i32;
pub const D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM_NOTIFICATION_MODE_FULL_FRAME: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM_NOTIFICATION_MODE = 0;
pub const D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM_NOTIFICATION_MODE_SUBREGIONS: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM_NOTIFICATION_MODE = 1;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_DESC {
    pub NodeMask: u32,
    pub Flags: D3D12_VIDEO_ENCODER_FLAGS,
    pub EncodeCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncodeProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub MaxMotionEstimationPrecision: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_VIDEO_ENCODER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_DIRTY_RECT_INFO {
    pub FullFrameIdentical: windows_sys::core::BOOL,
    pub MapValuesType: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE,
    pub NumDirtyRects: u32,
    pub pDirtyRects: *mut super::windef::RECT,
    pub SourceDPBFrameReference: u32,
}
#[cfg(feature = "Win32_windef")]
impl Default for D3D12_VIDEO_ENCODER_DIRTY_RECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_DIRTY_REGIONS {
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub Anonymous: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_0,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_DIRTY_REGIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_DIRTY_REGIONS_0 {
    pub pOpaqueLayoutBuffer: *mut core::ffi::c_void,
    pub pCPUBuffer: *mut D3D12_VIDEO_ENCODER_DIRTY_RECT_INFO,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_DIRTY_REGIONS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_DIRTY_REGIONS_CONFIGURATION {
    pub Enabled: windows_sys::core::BOOL,
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub MapValuesType: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE,
}
pub type D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE = i32;
pub const D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE_DIRTY: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE = 0;
pub const D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE_SKIP: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE = 1;
pub type D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAG_DIRTY_REGIONS: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAG_DIRTY_REGIONS_REQUIRE_FULL_ROW: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAG_NONE: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAG_REPEAT_FRAME: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAGS = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {
    pub SequenceControlDesc: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC,
    pub PictureControlDesc: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC,
    pub pInputFrame: *mut core::ffi::c_void,
    pub InputFrameSubresource: u32,
    pub CurrentFrameBitstreamMetadataSize: u32,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
impl Default for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1 {
    pub SequenceControlDesc: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC,
    pub PictureControlDesc: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC1,
    pub pInputFrame: *mut core::ffi::c_void,
    pub InputFrameSubresource: u32,
    pub CurrentFrameBitstreamMetadataSize: u32,
    pub OptionalMetadata: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {
    pub Bitstream: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM,
    pub ReconstructedPicture: D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE,
    pub EncoderOutputMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1 {
    pub Bitstream: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1,
    pub ReconstructedPicture: D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE,
    pub EncoderOutputMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
    pub FrameAnalysisReconstructedPicture: D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_CODEC_PICTURE_CONTROL_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_INVALID_METADATA_BUFFER_SOURCE: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_INVALID_REFERENCE_PICTURES: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_NO_ERROR: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_RECONFIGURATION_REQUEST_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAG_SUBREGION_LAYOUT_CONFIGURATION_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS = 2;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {
    pub pBuffer: *mut core::ffi::c_void,
    pub Offset: u64,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_FLAG_NONE: D3D12_VIDEO_ENCODER_FLAGS = 0;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_FRAME_ANALYSIS {
    pub pDownscaledFrame: *mut core::ffi::c_void,
    pub Subresource: u64,
    pub DownscaledReferences: D3D12_VIDEO_ENCODE_REFERENCE_FRAMES,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_FRAME_ANALYSIS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_FRAME_ANALYSIS_CONFIGURATION {
    pub Enabled: windows_sys::core::BOOL,
    pub Pow2DownscaleFactor: u32,
}
pub type D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION = i32;
pub const D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_FULL_PIXEL: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION = 0;
pub const D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_HALF_PIXEL: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION = 1;
pub const D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_QUARTER_PIXEL: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION = 2;
pub type D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAG_FULL_PIXEL: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAG_HALF_PIXEL: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAG_NONE: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAG_QUARTER_PIXEL: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAGS = 4;
pub type D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_CONFIG {
    pub MotionSearchMode: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE,
    pub SearchDeviationLimit: u32,
}
pub const D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_FULL_SEARCH: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE = 0;
pub const D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_START_HINT: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE = 1;
pub const D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_START_HINT_LIMITED_DISTANCE: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS {
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub Anonymous: D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS_0,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS_0 {
    pub pOpaqueLayoutBuffer: *mut core::ffi::c_void,
    pub pCPUBuffer: *mut D3D12_VIDEO_ENCODER_MOVEREGION_INFO,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT_0,
}
impl Default for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT_0 {
    pub pAV1Support: *mut D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT,
}
impl Default for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = i32;
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_AUTO: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 7;
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_BYTES_PER_SUBREGION: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 1;
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_CONFIGURABLE_GRID_PARTITION: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 6;
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_FULL_FRAME: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 0;
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_SQUARE_UNITS_PER_SUBREGION_ROW_UNALIGNED: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 2;
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_UNIFORM_GRID_PARTITION: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 5;
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_UNIFORM_PARTITIONING_ROWS_PER_SUBREGION: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 3;
pub const D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE_UNIFORM_PARTITIONING_SUBREGIONS_PER_FRAME: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA {
    pub bSize: u64,
    pub bStartOffset: u64,
    pub bHeaderSize: u64,
}
pub type D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = i32;
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_H264_B_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = 2;
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_H264_IDR_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = 3;
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_H264_I_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = 0;
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_H264_P_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 = 1;
pub type D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = i32;
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC_B_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = 2;
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC_IDR_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = 3;
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC_I_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = 0;
pub const D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC_P_FRAME: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_HEAP_DESC {
    pub NodeMask: u32,
    pub Flags: D3D12_VIDEO_ENCODER_HEAP_FLAGS,
    pub EncodeCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncodeProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub EncodeLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub ResolutionsListCount: u32,
    pub pResolutionList: *const D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
}
impl Default for D3D12_VIDEO_ENCODER_HEAP_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_HEAP_DESC1 {
    pub NodeMask: u32,
    pub Flags: D3D12_VIDEO_ENCODER_HEAP_FLAGS,
    pub EncodeCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncodeProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub EncodeLevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub ResolutionsListCount: u32,
    pub pResolutionList: *const D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub Pow2DownscaleFactor: u32,
}
impl Default for D3D12_VIDEO_ENCODER_HEAP_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_HEAP_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_HEAP_FLAG_ALLOW_DIRTY_REGIONS: D3D12_VIDEO_ENCODER_HEAP_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_HEAP_FLAG_ALLOW_RATE_CONTROL_FRAME_ANALYSIS: D3D12_VIDEO_ENCODER_HEAP_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_HEAP_FLAG_ALLOW_SUBREGION_NOTIFICATION_ARRAY_OF_BUFFERS: D3D12_VIDEO_ENCODER_HEAP_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_HEAP_FLAG_ALLOW_SUBREGION_NOTIFICATION_SINGLE_BUFFER: D3D12_VIDEO_ENCODER_HEAP_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_HEAP_FLAG_NONE: D3D12_VIDEO_ENCODER_HEAP_FLAGS = 0;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_DATA {
    pub MapType: D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE,
    pub Anonymous: D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_0,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_0 {
    pub Quantization: D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_QUANTIZATION_MATRIX,
    pub DirtyRegions: D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_DIRTY_REGIONS,
    pub MotionVectors: D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_MOTION_VECTORS,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_DIRTY_REGIONS {
    pub FullFrameIdentical: windows_sys::core::BOOL,
    pub MapValuesType: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE,
    pub pDirtyRegionsMap: *mut core::ffi::c_void,
    pub SourceDPBFrameReference: u32,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_DIRTY_REGIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_MOTION_VECTORS {
    pub MotionSearchModeConfiguration: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_CONFIG,
    pub NumHintsPerPixel: u32,
    pub ppMotionVectorMaps: *mut *mut core::ffi::c_void,
    pub pMotionVectorMapsSubresources: *mut u32,
    pub ppMotionVectorMapsMetadata: *mut *mut core::ffi::c_void,
    pub pMotionVectorMapsMetadataSubresources: *mut u32,
    pub MotionUnitPrecision: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION,
    pub PictureControlConfiguration: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA1,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_MOTION_VECTORS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_QUANTIZATION_MATRIX {
    pub pQuantizationMap: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_QUANTIZATION_MATRIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_SESSION_INFO {
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub Level: D3D12_VIDEO_ENCODER_LEVEL_SETTING,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub InputResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
    pub SubregionFrameEncoding: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub SubregionFrameEncodingData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA,
}
#[cfg(feature = "Win32_dxgiformat")]
impl Default for D3D12_VIDEO_ENCODER_INPUT_MAP_SESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE = i32;
pub const D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE_CPU_BUFFER: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE = 0;
pub const D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE_GPU_TEXTURE: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE = 1;
pub type D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE = i32;
pub const D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE_DIRTY_REGIONS: D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE = 1;
pub const D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE_MOTION_VECTORS: D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE = 2;
pub const D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE_QUANTIZATION_MATRIX: D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_INTRA_REFRESH {
    pub Mode: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE,
    pub IntraRefreshDuration: u32,
}
pub type D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE = i32;
pub const D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE_NONE: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE = 0;
pub const D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE_ROW_BASED: D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE = 1;
pub type D3D12_VIDEO_ENCODER_LEVELS_H264 = i32;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_1: D3D12_VIDEO_ENCODER_LEVELS_H264 = 0;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_11: D3D12_VIDEO_ENCODER_LEVELS_H264 = 2;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_12: D3D12_VIDEO_ENCODER_LEVELS_H264 = 3;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_13: D3D12_VIDEO_ENCODER_LEVELS_H264 = 4;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_1b: D3D12_VIDEO_ENCODER_LEVELS_H264 = 1;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_2: D3D12_VIDEO_ENCODER_LEVELS_H264 = 5;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_21: D3D12_VIDEO_ENCODER_LEVELS_H264 = 6;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_22: D3D12_VIDEO_ENCODER_LEVELS_H264 = 7;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_3: D3D12_VIDEO_ENCODER_LEVELS_H264 = 8;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_31: D3D12_VIDEO_ENCODER_LEVELS_H264 = 9;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_32: D3D12_VIDEO_ENCODER_LEVELS_H264 = 10;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_4: D3D12_VIDEO_ENCODER_LEVELS_H264 = 11;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_41: D3D12_VIDEO_ENCODER_LEVELS_H264 = 12;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_42: D3D12_VIDEO_ENCODER_LEVELS_H264 = 13;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_5: D3D12_VIDEO_ENCODER_LEVELS_H264 = 14;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_51: D3D12_VIDEO_ENCODER_LEVELS_H264 = 15;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_52: D3D12_VIDEO_ENCODER_LEVELS_H264 = 16;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_6: D3D12_VIDEO_ENCODER_LEVELS_H264 = 17;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_61: D3D12_VIDEO_ENCODER_LEVELS_H264 = 18;
pub const D3D12_VIDEO_ENCODER_LEVELS_H264_62: D3D12_VIDEO_ENCODER_LEVELS_H264 = 19;
pub type D3D12_VIDEO_ENCODER_LEVELS_HEVC = i32;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_1: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 0;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_2: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 1;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_21: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 2;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_3: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 3;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_31: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 4;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_4: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 5;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_41: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 6;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_5: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 7;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_51: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 8;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_52: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 9;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_6: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 10;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_61: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 11;
pub const D3D12_VIDEO_ENCODER_LEVELS_HEVC_62: D3D12_VIDEO_ENCODER_LEVELS_HEVC = 12;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_LEVEL_SETTING {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_LEVEL_SETTING_0,
}
impl Default for D3D12_VIDEO_ENCODER_LEVEL_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_LEVEL_SETTING_0 {
    pub pH264LevelSetting: *mut D3D12_VIDEO_ENCODER_LEVELS_H264,
    pub pHEVCLevelSetting: *mut D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC,
    pub pAV1LevelSetting: *mut D3D12_VIDEO_ENCODER_AV1_LEVEL_TIER_CONSTRAINTS,
}
impl Default for D3D12_VIDEO_ENCODER_LEVEL_SETTING_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC {
    pub Level: D3D12_VIDEO_ENCODER_LEVELS_HEVC,
    pub Tier: D3D12_VIDEO_ENCODER_TIER_HEVC,
}
pub type D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = i32;
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_EIGHTH_PIXEL: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 4;
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_FULL_PIXEL: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 1;
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_HALF_PIXEL: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 2;
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_MAXIMUM: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 0;
pub const D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE_QUARTER_PIXEL: D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_MOTION_SEARCH_CONFIGURATION {
    pub Enabled: windows_sys::core::BOOL,
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub MotionSearchMode: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE,
    pub BidirectionalRefFrameEnabled: windows_sys::core::BOOL,
}
pub type D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAG_GPU_TEXTURE_MULTIPLE_REFERENCES: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAG_MULTIPLE_HINTS: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAG_NONE: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = 1;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_MOVEREGION_INFO {
    pub NumMoveRegions: u32,
    pub pMoveRegions: *mut D3D12_VIDEO_ENCODER_MOVE_RECT,
    pub MotionSearchModeConfiguration: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_CONFIG,
    pub SourceDPBFrameReference: u32,
    pub MotionUnitPrecision: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION,
    pub Flags: D3D12_VIDEO_ENCODER_MOVEREGION_INFO_FLAGS,
}
#[cfg(feature = "Win32_windef")]
impl Default for D3D12_VIDEO_ENCODER_MOVEREGION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_MOVEREGION_INFO_FLAGS = i32;
pub const D3D12_VIDEO_ENCODER_MOVEREGION_INFO_FLAG_MULTIPLE_HINTS: D3D12_VIDEO_ENCODER_MOVEREGION_INFO_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_MOVEREGION_INFO_FLAG_NONE: D3D12_VIDEO_ENCODER_MOVEREGION_INFO_FLAGS = 0;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_MOVE_RECT {
    pub SourcePoint: super::windef::POINT,
    pub DestRect: super::windef::RECT,
}
pub type D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAG_FRAME_PSNR: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAG_NONE: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAG_QP_MAP: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAG_RC_BIT_ALLOCATION_MAP: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAG_SATD_MAP: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAG_SUBREGIONS_PSNR: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_OUTPUT_METADATA {
    pub EncodeErrorFlags: u64,
    pub EncodeStats: D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS,
    pub EncodedBitstreamWrittenBytesCount: u64,
    pub WrittenSubregionsCount: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS {
    pub AverageQP: u64,
    pub IntraCodingUnitsCount: u64,
    pub InterCodingUnitsCount: u64,
    pub SkipCodingUnitsCount: u64,
    pub AverageMotionEstimationXDirection: u64,
    pub AverageMotionEstimationYDirection: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_0,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_0 {
    pub pH264PicData: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264,
    pub pHEVCPicData: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC,
    pub pHEVCPicData1: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC1,
    pub pAV1PicData: *mut D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_CODEC_DATA,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA1 {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA1_0,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA1_0 {
    pub pH264PicData: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264,
    pub pHEVCPicData: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC2,
    pub pAV1PicData: *mut D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_CODEC_DATA,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS,
    pub FrameType: D3D12_VIDEO_ENCODER_FRAME_TYPE_H264,
    pub pic_parameter_set_id: u32,
    pub idr_pic_id: u32,
    pub PictureOrderCountNumber: u32,
    pub FrameDecodingOrderNumber: u32,
    pub TemporalLayerIndex: u32,
    pub List0ReferenceFramesCount: u32,
    pub pList0ReferenceFrames: *mut u32,
    pub List1ReferenceFramesCount: u32,
    pub pList1ReferenceFrames: *mut u32,
    pub ReferenceFramesReconPictureDescriptorsCount: u32,
    pub pReferenceFramesReconPictureDescriptors: *mut D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264,
    pub adaptive_ref_pic_marking_mode_flag: u8,
    pub RefPicMarkingOperationsCommandsCount: u32,
    pub pRefPicMarkingOperationsCommands: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION,
    pub List0RefPicModificationsCount: u32,
    pub pList0RefPicModifications: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION,
    pub List1RefPicModificationsCount: u32,
    pub pList1RefPicModifications: *mut D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION,
    pub QPMapValuesCount: u32,
    pub pRateControlQPMap: *mut i8,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAG_NONE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAG_REQUEST_INTRA_CONSTRAINED_SLICES: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAG_REQUEST_NUM_REF_IDX_ACTIVE_OVERRIDE_FLAG_SLICE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {
    pub modification_of_pic_nums_idc: u8,
    pub abs_diff_pic_num_minus1: u32,
    pub long_term_pic_num: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {
    pub memory_management_control_operation: u8,
    pub difference_of_pic_nums_minus1: u32,
    pub long_term_pic_num: u32,
    pub long_term_frame_idx: u32,
    pub max_long_term_frame_idx_plus1: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS,
    pub FrameType: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC,
    pub slice_pic_parameter_set_id: u32,
    pub PictureOrderCountNumber: u32,
    pub TemporalLayerIndex: u32,
    pub List0ReferenceFramesCount: u32,
    pub pList0ReferenceFrames: *mut u32,
    pub List1ReferenceFramesCount: u32,
    pub pList1ReferenceFrames: *mut u32,
    pub ReferenceFramesReconPictureDescriptorsCount: u32,
    pub pReferenceFramesReconPictureDescriptors: *mut D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC,
    pub List0RefPicModificationsCount: u32,
    pub pList0RefPicModifications: *mut u32,
    pub List1RefPicModificationsCount: u32,
    pub pList1RefPicModifications: *mut u32,
    pub QPMapValuesCount: u32,
    pub pRateControlQPMap: *mut i8,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC1 {
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS,
    pub FrameType: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC,
    pub slice_pic_parameter_set_id: u32,
    pub PictureOrderCountNumber: u32,
    pub TemporalLayerIndex: u32,
    pub List0ReferenceFramesCount: u32,
    pub pList0ReferenceFrames: *mut u32,
    pub List1ReferenceFramesCount: u32,
    pub pList1ReferenceFrames: *mut u32,
    pub ReferenceFramesReconPictureDescriptorsCount: u32,
    pub pReferenceFramesReconPictureDescriptors: *mut D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC,
    pub List0RefPicModificationsCount: u32,
    pub pList0RefPicModifications: *mut u32,
    pub List1RefPicModificationsCount: u32,
    pub pList1RefPicModifications: *mut u32,
    pub QPMapValuesCount: u32,
    pub pRateControlQPMap: *mut i8,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub log2_sao_offset_scale_luma: u8,
    pub log2_sao_offset_scale_chroma: u8,
    pub log2_max_transform_skip_block_size_minus2: u8,
    pub chroma_qp_offset_list_len_minus1: u8,
    pub cb_qp_offset_list: [i8; 6],
    pub cr_qp_offset_list: [i8; 6],
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC2 {
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS,
    pub FrameType: D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC,
    pub slice_pic_parameter_set_id: u32,
    pub PictureOrderCountNumber: u32,
    pub TemporalLayerIndex: u32,
    pub List0ReferenceFramesCount: u32,
    pub pList0ReferenceFrames: *mut u32,
    pub List1ReferenceFramesCount: u32,
    pub pList1ReferenceFrames: *mut u32,
    pub ReferenceFramesReconPictureDescriptorsCount: u32,
    pub pReferenceFramesReconPictureDescriptors: *mut D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC,
    pub List0RefPicModificationsCount: u32,
    pub pList0RefPicModifications: *mut u32,
    pub List1RefPicModificationsCount: u32,
    pub pList1RefPicModifications: *mut u32,
    pub QPMapValuesCount: u32,
    pub pRateControlQPMap: *mut i8,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub log2_sao_offset_scale_luma: u8,
    pub log2_sao_offset_scale_chroma: u8,
    pub log2_max_transform_skip_block_size_minus2: u8,
    pub chroma_qp_offset_list_len_minus1: u8,
    pub cb_qp_offset_list: [i8; 6],
    pub cr_qp_offset_list: [i8; 6],
    pub num_ref_idx_l0_active_minus1: u32,
    pub num_ref_idx_l1_active_minus1: u32,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAG_CHROMA_QP_OFFSET_LIST: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAG_CROSS_COMPONENT_PREDICTION: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAG_NONE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAG_REQUEST_INTRA_CONSTRAINED_SLICES: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAG_REQUEST_NUM_REF_IDX_ACTIVE_OVERRIDE_FLAG_SLICE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS = 2;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC {
    pub IntraRefreshFrameIndex: u32,
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS,
    pub PictureControlCodecData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA,
    pub ReferenceFrames: D3D12_VIDEO_ENCODE_REFERENCE_FRAMES,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC1 {
    pub IntraRefreshFrameIndex: u32,
    pub Flags: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS,
    pub PictureControlCodecData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA1,
    pub ReferenceFrames: D3D12_VIDEO_ENCODE_REFERENCE_FRAMES,
    pub MotionVectors: D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS,
    pub DirtyRects: D3D12_VIDEO_ENCODER_DIRTY_REGIONS,
    pub QuantizationTextureMap: D3D12_VIDEO_ENCODER_QUANTIZATION_OPAQUE_MAP,
    pub FrameAnalysis: D3D12_VIDEO_ENCODER_FRAME_ANALYSIS,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAG_ENABLE_DIRTY_REGIONS_INPUT: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAG_ENABLE_MOTION_VECTORS_INPUT: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAG_ENABLE_QUANTIZATION_MATRIX_INPUT: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAG_NONE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAG_USED_AS_REFERENCE_PICTURE: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_0,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_0 {
    pub pSlicesPartition_H264: *const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES,
    pub pSlicesPartition_HEVC: *const D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES,
    pub pTilesPartition_AV1: *const D3D12_VIDEO_ENCODER_AV1_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_TILES,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES {
    pub Anonymous: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES_0,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES_0 {
    pub MaxBytesPerSlice: u32,
    pub NumberOfCodingUnitsPerSlice: u32,
    pub NumberOfRowsPerSlice: u32,
    pub NumberOfSlicesPerFrame: u32,
}
impl Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {
    pub Width: u32,
    pub Height: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC {
    pub WidthRatio: u32,
    pub HeightRatio: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_PROFILE_DESC {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_PROFILE_DESC_0,
}
impl Default for D3D12_VIDEO_ENCODER_PROFILE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_PROFILE_DESC_0 {
    pub pH264Profile: *mut D3D12_VIDEO_ENCODER_PROFILE_H264,
    pub pHEVCProfile: *mut D3D12_VIDEO_ENCODER_PROFILE_HEVC,
    pub pAV1Profile: *mut D3D12_VIDEO_ENCODER_AV1_PROFILE,
}
impl Default for D3D12_VIDEO_ENCODER_PROFILE_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_PROFILE_H264 = i32;
pub const D3D12_VIDEO_ENCODER_PROFILE_H264_HIGH: D3D12_VIDEO_ENCODER_PROFILE_H264 = 1;
pub const D3D12_VIDEO_ENCODER_PROFILE_H264_HIGH_10: D3D12_VIDEO_ENCODER_PROFILE_H264 = 2;
pub const D3D12_VIDEO_ENCODER_PROFILE_H264_MAIN: D3D12_VIDEO_ENCODER_PROFILE_H264 = 0;
pub type D3D12_VIDEO_ENCODER_PROFILE_HEVC = i32;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 0;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN10: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 1;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN10_422: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 3;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN10_444: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 6;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN12: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 2;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN12_422: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 4;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN12_444: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 7;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN16_444: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 8;
pub const D3D12_VIDEO_ENCODER_PROFILE_HEVC_MAIN_444: D3D12_VIDEO_ENCODER_PROFILE_HEVC = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_QPMAP_CONFIGURATION {
    pub Enabled: windows_sys::core::BOOL,
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_QUANTIZATION_OPAQUE_MAP {
    pub pOpaqueQuantizationMap: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_QUANTIZATION_OPAQUE_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_dxgicommon")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL {
    pub Mode: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE,
    pub Flags: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS,
    pub ConfigParams: D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS,
    pub TargetFrameRate: super::dxgicommon::DXGI_RATIONAL,
}
#[cfg(feature = "Win32_dxgicommon")]
impl Default for D3D12_VIDEO_ENCODER_RATE_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_ABSOLUTE_QP_MAP {
    pub QualityVsSpeed: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetBitRate: u64,
    pub VBVCapacity: u64,
    pub InitialVBVFullness: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR1 {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetBitRate: u64,
    pub VBVCapacity: u64,
    pub InitialVBVFullness: u64,
    pub QualityVsSpeed: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS_0,
}
impl Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS_0 {
    pub pConfiguration_CQP: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP,
    pub pConfiguration_CBR: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR,
    pub pConfiguration_VBR: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR,
    pub pConfiguration_QVBR: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR,
    pub pConfiguration_CQP1: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP1,
    pub pConfiguration_CBR1: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR1,
    pub pConfiguration_VBR1: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR1,
    pub pConfiguration_QVBR1: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR1,
    pub pConfiguration_AbsoluteQPMap: *const D3D12_VIDEO_ENCODER_RATE_CONTROL_ABSOLUTE_QP_MAP,
}
impl Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {
    pub ConstantQP_FullIntracodedFrame: u32,
    pub ConstantQP_InterPredictedFrame_PrevRefOnly: u32,
    pub ConstantQP_InterPredictedFrame_BiDirectionalRef: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP1 {
    pub ConstantQP_FullIntracodedFrame: u32,
    pub ConstantQP_InterPredictedFrame_PrevRefOnly: u32,
    pub ConstantQP_InterPredictedFrame_BiDirectionalRef: u32,
    pub QualityVsSpeed: u32,
}
pub type D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_DELTA_QP: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_EXTENSION1_SUPPORT: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_FRAME_ANALYSIS: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_INITIAL_QP: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_MAX_FRAME_SIZE: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_QP_RANGE: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_QUALITY_VS_SPEED: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_SPATIAL_ADAPTIVE_QP: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 256;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_ENABLE_VBV_SIZES: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAG_NONE: D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS = 0;
pub type D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAG_BIDIR_INTER_FRAME_SUPPORTED: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAG_DYNAMIC_1ST_PASS_SKIP: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAG_DYNAMIC_DOWNSCALE_FACTOR_CHANGE_KEY_FRAME: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAG_EXTERNAL_DPB_DOWNSCALING: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAG_INTRACODED_FRAME_SUPPORTED: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAG_NONE: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = 7;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAG_UNIDIR_INTER_FRAME_SUPPORTED: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS = 2;
pub type D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = i32;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_ABSOLUTE_QP_MAP: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 0;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_CBR: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 2;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_CQP: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 1;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_QVBR: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 4;
pub const D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE_VBR: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetAvgBitRate: u64,
    pub PeakBitRate: u64,
    pub ConstantQualityTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR1 {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetAvgBitRate: u64,
    pub PeakBitRate: u64,
    pub ConstantQualityTarget: u32,
    pub VBVCapacity: u64,
    pub InitialVBVFullness: u64,
    pub QualityVsSpeed: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetAvgBitRate: u64,
    pub PeakBitRate: u64,
    pub VBVCapacity: u64,
    pub InitialVBVFullness: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR1 {
    pub InitialQP: u32,
    pub MinQP: u32,
    pub MaxQP: u32,
    pub MaxFrameBitSize: u64,
    pub TargetAvgBitRate: u64,
    pub PeakBitRate: u64,
    pub VBVCapacity: u64,
    pub InitialVBVFullness: u64,
    pub QualityVsSpeed: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {
    pub pReconstructedPicture: *mut core::ffi::c_void,
    pub ReconstructedPictureSubresource: u32,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {
    pub ReconstructedPictureResourceIndex: u32,
    pub IsLongTermReference: windows_sys::core::BOOL,
    pub LongTermPictureIdx: u32,
    pub PictureOrderCountNumber: u32,
    pub FrameDecodingOrderNumber: u32,
    pub TemporalLayerIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {
    pub ReconstructedPictureResourceIndex: u32,
    pub IsRefUsedByCurrentPic: windows_sys::core::BOOL,
    pub IsLongTermReference: windows_sys::core::BOOL,
    pub PictureOrderCountNumber: u32,
    pub TemporalLayerIndex: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS {
    pub SessionInfo: D3D12_VIDEO_ENCODER_INPUT_MAP_SESSION_INFO,
    pub InputData: D3D12_VIDEO_ENCODER_INPUT_MAP_DATA,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl Default for D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_OUTPUT_ARGUMENTS {
    pub pOpaqueLayoutBuffer: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_OUTPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {
    pub EncoderCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncoderProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub EncoderInputFormat: super::dxgiformat::DXGI_FORMAT,
    pub EncodedPictureEffectiveResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub HWLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl Default for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1 {
    pub EncoderCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncoderProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub EncoderInputFormat: super::dxgiformat::DXGI_FORMAT,
    pub EncodedPictureEffectiveResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub HWLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
    pub OptionalMetadata: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS,
    pub CodecConfiguration: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl Default for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {
    pub ResolvedLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS1 {
    pub ResolvedLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
    pub pOutputQPMap: *mut core::ffi::c_void,
    pub pOutputSATDMap: *mut core::ffi::c_void,
    pub pOutputBitAllocationMap: *mut core::ffi::c_void,
    pub ResolvedFramePSNRData: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
    pub ResolvedSubregionsPSNRData: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_PSNR_RESOLVED_LAYOUT {
    pub PSNRY: f32,
    pub PSNRU: f32,
    pub PSNRV: f32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgicommon")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC {
    pub Flags: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS,
    pub IntraRefreshConfig: D3D12_VIDEO_ENCODER_INTRA_REFRESH,
    pub RateControl: D3D12_VIDEO_ENCODER_RATE_CONTROL,
    pub PictureTargetResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub SelectedLayoutMode: D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE,
    pub FrameSubregionsLayoutData: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA,
    pub CodecGopSequence: D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE,
}
#[cfg(feature = "Win32_dxgicommon")]
impl Default for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_GOP_SEQUENCE_CHANGE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_NONE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_RATE_CONTROL_CHANGE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_REQUEST_INTRA_REFRESH: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_RESOLUTION_CHANGE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAG_SUBREGION_LAYOUT_CHANGE: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE {
    pub DataSize: u32,
    pub Anonymous: D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_0,
}
impl Default for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_0 {
    pub pH264GroupOfPictures: *mut D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264,
    pub pHEVCGroupOfPictures: *mut D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC,
    pub pAV1SequenceStructure: *mut D3D12_VIDEO_ENCODER_AV1_SEQUENCE_STRUCTURE,
}
impl Default for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {
    pub GOPLength: u32,
    pub PPicturePeriod: u32,
    pub pic_order_cnt_type: u8,
    pub log2_max_frame_num_minus4: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {
    pub GOPLength: u32,
    pub PPicturePeriod: u32,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM {
    pub BufferMode: D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM_BUFFER_MODE,
    pub ExpectedSubregionCount: u32,
    pub pSubregionBitstreamsBaseOffsets: *mut u64,
    pub ppSubregionBitstreams: *mut *mut core::ffi::c_void,
    pub ppSubregionSizes: *mut *mut core::ffi::c_void,
    pub ppSubregionOffsets: *mut *mut core::ffi::c_void,
    pub ppSubregionFences: *mut *mut core::ffi::c_void,
    pub pSubregionFenceValues: *mut u64,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM_BUFFER_MODE = i32;
pub const D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM_BUFFER_MODE_ARRAY_OF_BUFFERS: D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM_BUFFER_MODE = 0;
pub const D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM_BUFFER_MODE_SINGLE_BUFFER: D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM_BUFFER_MODE = 1;
pub type D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_FRAME_PSNR_METADATA_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 2097152;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_GENERAL_SUPPORT_OK: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_MOTION_ESTIMATION_PRECISION_MODE_LIMIT_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 4096;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_NONE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_PER_BLOCK_QP_MAP_METADATA_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 65536;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_PER_BLOCK_RC_BIT_ALLOCATION_MAP_METADATA_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 262144;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_PER_BLOCK_SATD_MAP_METADATA_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 131072;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_ADJUSTABLE_QP_RANGE_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 256;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_DELTA_QP_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_EXTENSION1_SUPPORT: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 8192;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_FRAME_ANALYSIS_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_INITIAL_QP_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 512;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_MAX_FRAME_SIZE_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 1024;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_QUALITY_VS_SPEED_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 16384;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_RECONFIGURATION_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_SPATIAL_ADAPTIVE_QP_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 8388608;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RATE_CONTROL_VBV_SIZE_CONFIG_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_READABLE_RECONSTRUCTED_PICTURE_LAYOUT_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 32768;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RECONSTRUCTED_FRAMES_REQUIRE_TEXTURE_ARRAYS: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_RESOLUTION_RECONFIGURATION_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_SEQUENCE_GOP_RECONFIGURATION_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 2048;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_SUBREGIONS_PSNR_METADATA_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 4194304;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_SUBREGION_LAYOUT_RECONFIGURATION_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_SUBREGION_NOTIFICATION_ARRAY_OF_BUFFERS_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 524288;
pub const D3D12_VIDEO_ENCODER_SUPPORT_FLAG_SUBREGION_NOTIFICATION_SINGLE_BUFFER_AVAILABLE: D3D12_VIDEO_ENCODER_SUPPORT_FLAGS = 1048576;
pub type D3D12_VIDEO_ENCODER_TIER_HEVC = i32;
pub const D3D12_VIDEO_ENCODER_TIER_HEVC_HIGH: D3D12_VIDEO_ENCODER_TIER_HEVC = 1;
pub const D3D12_VIDEO_ENCODER_TIER_HEVC_MAIN: D3D12_VIDEO_ENCODER_TIER_HEVC = 0;
pub type D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_CODEC_CONFIGURATION_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 16;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_CODEC_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 1;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_DIRTY_REGIONS_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 16384;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_FRAME_ANALYSIS_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 65536;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_GOP_STRUCTURE_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 2048;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_INPUT_FORMAT_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 8;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_INTRA_REFRESH_MODE_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 128;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_MOTION_SEARCH_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 32768;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_NONE: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_QPMAP_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 8192;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_RATE_CONTROL_CONFIGURATION_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 64;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_RATE_CONTROL_MODE_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 32;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_RESOLUTION_NOT_SUPPORTED_IN_LIST: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 512;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_SUBREGION_LAYOUT_DATA_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 4096;
pub const D3D12_VIDEO_ENCODER_VALIDATION_FLAG_SUBREGION_LAYOUT_MODE_NOT_SUPPORTED: D3D12_VIDEO_ENCODER_VALIDATION_FLAGS = 256;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    pub NumTexture2Ds: u32,
    pub ppTexture2Ds: *mut *mut core::ffi::c_void,
    pub pSubresources: *mut u32,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_DESC {
    pub NodeMask: u32,
    pub CommandId: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_INFO {
    pub CommandId: windows_sys::core::GUID,
    pub Name: windows_sys::core::PCWSTR,
    pub CommandListSupportFlags: super::d3d12::D3D12_COMMAND_LIST_SUPPORT_FLAGS,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_EXTENSION_COMMAND_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = u32;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_NONE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 0;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_READ: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 1;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_WRITE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {
    pub Name: windows_sys::core::PCWSTR,
    pub Type: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE,
    pub Flags: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS,
}
impl Default for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = i32;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_CAPS_INPUT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 3;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_CAPS_OUTPUT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 4;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_CREATION: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 0;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_DEVICE_EXECUTE_INPUT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 5;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_DEVICE_EXECUTE_OUTPUT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 6;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_EXECUTION: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 2;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE_INITIALIZATION: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE = 1;
pub type D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = i32;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_DOUBLE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 9;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_FLOAT: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 8;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_RESOURCE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 10;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_SINT16: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 5;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_SINT32: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 6;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_SINT64: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 7;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_SINT8: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 4;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_UINT16: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 1;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_UINT32: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 2;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_UINT64: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 3;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE_UINT8: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE = 0;
pub type D3D12_VIDEO_FIELD_TYPE = i32;
pub const D3D12_VIDEO_FIELD_TYPE_INTERLACED_BOTTOM_FIELD_FIRST: D3D12_VIDEO_FIELD_TYPE = 2;
pub const D3D12_VIDEO_FIELD_TYPE_INTERLACED_TOP_FIELD_FIRST: D3D12_VIDEO_FIELD_TYPE = 1;
pub const D3D12_VIDEO_FIELD_TYPE_NONE: D3D12_VIDEO_FIELD_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_FORMAT {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
}
pub type D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE = i32;
pub const D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE_FIELD_BASED: D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE = 1;
pub const D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE_NONE: D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE = 0;
pub type D3D12_VIDEO_FRAME_STEREO_FORMAT = i32;
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_HORIZONTAL: D3D12_VIDEO_FRAME_STEREO_FORMAT = 2;
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_MONO: D3D12_VIDEO_FRAME_STEREO_FORMAT = 1;
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_NONE: D3D12_VIDEO_FRAME_STEREO_FORMAT = 0;
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_SEPARATE: D3D12_VIDEO_FRAME_STEREO_FORMAT = 4;
pub const D3D12_VIDEO_FRAME_STEREO_FORMAT_VERTICAL: D3D12_VIDEO_FRAME_STEREO_FORMAT = 3;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_DESC {
    pub NodeMask: u32,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {
    pub pInputTexture2D: *mut core::ffi::c_void,
    pub InputSubresourceIndex: u32,
    pub pReferenceTexture2D: *mut core::ffi::c_void,
    pub ReferenceSubresourceIndex: u32,
    pub pHintMotionVectorHeap: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {
    pub pMotionVectorHeap: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE = i32;
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_16X16: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE = 1;
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_8X8: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE = 0;
pub type D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS = u32;
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAG_16X16: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS = 2;
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAG_8X8: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS = 1;
pub const D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAG_NONE: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS = 0;
pub type D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION = i32;
pub type D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS = u32;
pub const D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAG_NONE: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS = 0;
pub const D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAG_QUARTER_PEL: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS = 1;
pub const D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_QUARTER_PEL: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION = 0;
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {
    pub NodeMask: u32,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_PROCESS_ALPHA_BLENDING {
    pub Enable: windows_sys::core::BOOL,
    pub Alpha: f32,
}
pub type D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = i32;
pub const D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE_BACKGROUND: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = 1;
pub const D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE_DESTINATION: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = 2;
pub const D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE_OPAQUE: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = 0;
pub const D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE_SOURCE_STREAM: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE = 3;
pub type D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = u32;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_ANAMORPHIC_SCALING: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 128;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_COLOR_CORRECTION: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 8;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_CUSTOM: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 2147483648;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_DENOISE: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 1;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_DERINGING: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 2;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_EDGE_ENHANCEMENT: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 4;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_FLESH_TONE_MAPPING: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 16;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_IMAGE_STABILIZATION: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 32;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_NONE: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 0;
pub const D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAG_SUPER_RESOLUTION: D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS = 64;
pub type D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS = u32;
pub const D3D12_VIDEO_PROCESS_DEINTERLACE_FLAG_BOB: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS = 1;
pub const D3D12_VIDEO_PROCESS_DEINTERLACE_FLAG_CUSTOM: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS = 2147483648;
pub const D3D12_VIDEO_PROCESS_DEINTERLACE_FLAG_NONE: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS = 0;
pub type D3D12_VIDEO_PROCESS_FEATURE_FLAGS = u32;
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_ALPHA_BLENDING: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 32;
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_ALPHA_FILL: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 1;
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_FLIP: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 16;
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_LUMA_KEY: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 2;
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_NONE: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 0;
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_PIXEL_ASPECT_RATIO: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 64;
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_ROTATION: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 8;
pub const D3D12_VIDEO_PROCESS_FEATURE_FLAG_STEREO: D3D12_VIDEO_PROCESS_FEATURE_FLAGS = 4;
pub type D3D12_VIDEO_PROCESS_FILTER = i32;
pub const D3D12_VIDEO_PROCESS_FILTER_ANAMORPHIC_SCALING: D3D12_VIDEO_PROCESS_FILTER = 6;
pub const D3D12_VIDEO_PROCESS_FILTER_BRIGHTNESS: D3D12_VIDEO_PROCESS_FILTER = 0;
pub const D3D12_VIDEO_PROCESS_FILTER_CONTRAST: D3D12_VIDEO_PROCESS_FILTER = 1;
pub const D3D12_VIDEO_PROCESS_FILTER_EDGE_ENHANCEMENT: D3D12_VIDEO_PROCESS_FILTER = 5;
pub type D3D12_VIDEO_PROCESS_FILTER_FLAGS = u32;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_ANAMORPHIC_SCALING: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 64;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_BRIGHTNESS: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 1;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_CONTRAST: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 2;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_EDGE_ENHANCEMENT: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 32;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_HUE: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 4;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_NOISE_REDUCTION: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 16;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_NONE: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 0;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_SATURATION: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 8;
pub const D3D12_VIDEO_PROCESS_FILTER_FLAG_STEREO_ADJUSTMENT: D3D12_VIDEO_PROCESS_FILTER_FLAGS = 128;
pub const D3D12_VIDEO_PROCESS_FILTER_HUE: D3D12_VIDEO_PROCESS_FILTER = 2;
pub const D3D12_VIDEO_PROCESS_FILTER_NOISE_REDUCTION: D3D12_VIDEO_PROCESS_FILTER = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_PROCESS_FILTER_RANGE {
    pub Minimum: i32,
    pub Maximum: i32,
    pub Default: i32,
    pub Multiplier: f32,
}
pub const D3D12_VIDEO_PROCESS_FILTER_SATURATION: D3D12_VIDEO_PROCESS_FILTER = 3;
pub const D3D12_VIDEO_PROCESS_FILTER_STEREO_ADJUSTMENT: D3D12_VIDEO_PROCESS_FILTER = 7;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM {
    pub pTexture2D: *mut core::ffi::c_void,
    pub Subresource: u32,
    pub ReferenceSet: D3D12_VIDEO_PROCESS_REFERENCE_SET,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_PROCESS_INPUT_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {
    pub InputStream: [D3D12_VIDEO_PROCESS_INPUT_STREAM; 2],
    pub Transform: D3D12_VIDEO_PROCESS_TRANSFORM,
    pub Flags: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS,
    pub RateInfo: D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE,
    pub FilterLevels: [i32; 32],
    pub AlphaBlending: D3D12_VIDEO_PROCESS_ALPHA_BLENDING,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {
    pub InputStream: [D3D12_VIDEO_PROCESS_INPUT_STREAM; 2],
    pub Transform: D3D12_VIDEO_PROCESS_TRANSFORM,
    pub Flags: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS,
    pub RateInfo: D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE,
    pub FilterLevels: [i32; 32],
    pub AlphaBlending: D3D12_VIDEO_PROCESS_ALPHA_BLENDING,
    pub FieldType: D3D12_VIDEO_FIELD_TYPE,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub SourceAspectRatio: super::dxgicommon::DXGI_RATIONAL,
    pub DestinationAspectRatio: super::dxgicommon::DXGI_RATIONAL,
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub SourceSizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub DestinationSizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub EnableOrientation: windows_sys::core::BOOL,
    pub FilterFlags: D3D12_VIDEO_PROCESS_FILTER_FLAGS,
    pub StereoFormat: D3D12_VIDEO_FRAME_STEREO_FORMAT,
    pub FieldType: D3D12_VIDEO_FIELD_TYPE,
    pub DeinterlaceMode: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS,
    pub EnableAlphaBlending: windows_sys::core::BOOL,
    pub LumaKey: D3D12_VIDEO_PROCESS_LUMA_KEY,
    pub NumPastFrames: u32,
    pub NumFutureFrames: u32,
    pub EnableAutoProcessing: windows_sys::core::BOOL,
}
pub type D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = u32;
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_FRAME_DISCONTINUITY: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 1;
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_FRAME_REPEAT: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 2;
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_NONE: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_PROCESS_LUMA_KEY {
    pub Enable: windows_sys::core::BOOL,
    pub Lower: f32,
    pub Upper: f32,
}
pub type D3D12_VIDEO_PROCESS_ORIENTATION = i32;
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_180: D3D12_VIDEO_PROCESS_ORIENTATION = 4;
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_270: D3D12_VIDEO_PROCESS_ORIENTATION = 6;
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_270_FLIP_HORIZONTAL: D3D12_VIDEO_PROCESS_ORIENTATION = 7;
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_90: D3D12_VIDEO_PROCESS_ORIENTATION = 2;
pub const D3D12_VIDEO_PROCESS_ORIENTATION_CLOCKWISE_90_FLIP_HORIZONTAL: D3D12_VIDEO_PROCESS_ORIENTATION = 3;
pub const D3D12_VIDEO_PROCESS_ORIENTATION_DEFAULT: D3D12_VIDEO_PROCESS_ORIENTATION = 0;
pub const D3D12_VIDEO_PROCESS_ORIENTATION_FLIP_HORIZONTAL: D3D12_VIDEO_PROCESS_ORIENTATION = 1;
pub const D3D12_VIDEO_PROCESS_ORIENTATION_FLIP_VERTICAL: D3D12_VIDEO_PROCESS_ORIENTATION = 5;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_PROCESS_OUTPUT_STREAM {
    pub pTexture2D: *mut core::ffi::c_void,
    pub Subresource: u32,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_PROCESS_OUTPUT_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {
    pub OutputStream: [D3D12_VIDEO_PROCESS_OUTPUT_STREAM; 2],
    pub TargetRectangle: super::d3d12::D3D12_RECT,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub AlphaFillMode: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE,
    pub AlphaFillModeSourceStreamIndex: u32,
    pub BackgroundColor: [f32; 4],
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub EnableStereo: windows_sys::core::BOOL,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl Default for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_PROCESS_REFERENCE_SET {
    pub NumPastFrames: u32,
    pub ppPastFrames: *mut *mut core::ffi::c_void,
    pub pPastSubresources: *mut u32,
    pub NumFutureFrames: u32,
    pub ppFutureFrames: *mut *mut core::ffi::c_void,
    pub pFutureSubresources: *mut u32,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_PROCESS_REFERENCE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D12_VIDEO_PROCESS_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_PROCESS_SUPPORT_FLAG_NONE: D3D12_VIDEO_PROCESS_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_PROCESS_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_PROCESS_SUPPORT_FLAGS = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_PROCESS_TRANSFORM {
    pub SourceRectangle: super::d3d12::D3D12_RECT,
    pub DestinationRectangle: super::d3d12::D3D12_RECT,
    pub Orientation: D3D12_VIDEO_PROCESS_ORIENTATION,
}
pub type D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAG_NONE: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_SAMPLE {
    pub Width: u32,
    pub Height: u32,
    pub Format: D3D12_VIDEO_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_SCALE_SUPPORT {
    pub OutputSizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub Flags: D3D12_VIDEO_SCALE_SUPPORT_FLAGS,
}
pub type D3D12_VIDEO_SCALE_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_SCALE_SUPPORT_FLAG_DPB_ENCODER_RESOURCES: D3D12_VIDEO_SCALE_SUPPORT_FLAGS = 4;
pub const D3D12_VIDEO_SCALE_SUPPORT_FLAG_EVEN_DIMENSIONS_ONLY: D3D12_VIDEO_SCALE_SUPPORT_FLAGS = 2;
pub const D3D12_VIDEO_SCALE_SUPPORT_FLAG_NONE: D3D12_VIDEO_SCALE_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_SCALE_SUPPORT_FLAG_POW2_ONLY: D3D12_VIDEO_SCALE_SUPPORT_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D12_VIDEO_SIZE_RANGE {
    pub MaxWidth: u32,
    pub MaxHeight: u32,
    pub MinWidth: u32,
    pub MinHeight: u32,
}
