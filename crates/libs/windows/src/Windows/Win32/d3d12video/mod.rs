pub type D3D12_BITSTREAM_ENCRYPTION_TYPE = i32;
pub const D3D12_BITSTREAM_ENCRYPTION_TYPE_NONE: D3D12_BITSTREAM_ENCRYPTION_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {
    pub IOCoherent: windows_core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {
    pub VideoDecoderHeapDesc: D3D12_VIDEO_DECODER_HEAP_DESC,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {
    pub VideoDecoderHeapDesc: D3D12_VIDEO_DECODER_HEAP_DESC,
    pub Protected: windows_core::BOOL,
    pub MemoryPoolL0Size: u64,
    pub MemoryPoolL1Size: u64,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub FormatCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {
    pub NodeIndex: u32,
    pub DecodeProfile: windows_core::GUID,
    pub Width: u32,
    pub Height: u32,
    pub DecodeFormat: super::dxgiformat::DXGI_FORMAT,
    pub Components: D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS,
    pub BinCount: u32,
    pub CounterBitDepth: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    pub NodeIndex: u32,
    pub ProfileCount: u32,
    pub pProfiles: *mut windows_core::GUID,
}
impl Default for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {
    pub NodeIndex: u32,
    pub ProfileCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub IsSupported: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub Profile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
    pub BidirectionalRefFrameEnabled: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub ResolutionRatiosCount: u32,
    pub IsSupported: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {
    pub NodeIndex: u32,
    pub Codec: D3D12_VIDEO_ENCODER_CODEC,
    pub RateControlMode: D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE,
    pub IsSupported: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_DIRTY_REGIONS {
    pub DirtyRegionsSupportFlags: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_SUPPORT_FLAGS,
    pub MapSourcePreferenceRanking: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_FRAME_ANALYSIS {
    pub SupportFlags: D3D12_VIDEO_ENCODER_RATE_CONTROL_FRAME_ANALYSIS_SUPPORT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {
    pub MaxSubregionsNumber: u32,
    pub MaxIntraRefreshFrameDuration: u32,
    pub SubregionBlockPixelsSize: u32,
    pub QPMapRegionPixelsSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_MOTION_SEARCH {
    pub MaxMotionHints: u32,
    pub MinDeviation: u32,
    pub MaxDeviation: u32,
    pub MapSourcePreferenceRanking: u32,
    pub MotionUnitPrecisionSupportFlags: D3D12_VIDEO_ENCODER_FRAME_INPUT_MOTION_UNIT_PRECISION_SUPPORT_FLAGS,
    pub SupportFlags: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
    pub IsSupported: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {
    pub NodeIndex: u32,
    pub CommandCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {
    pub CommandId: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {
    pub CommandId: windows_core::GUID,
    pub Stage: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE,
    pub ParameterCount: u32,
    pub ParameterPacking: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {
    pub NodeIndex: u32,
    pub CommandId: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {
    pub NodeIndex: u32,
    pub CommandId: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {
    pub NodeIndex: u32,
    pub VideoDecodeSupport: windows_core::BOOL,
    pub VideoProcessSupport: windows_core::BOOL,
    pub VideoEncodeSupport: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {
    pub NodeIndex: u32,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub BlockSizeFlags: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS,
    pub PrecisionFlags: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgiformat")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {
    pub NodeIndex: u32,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub Protected: windows_core::BOOL,
    pub MotionVectorHeapMemoryPoolL0Size: u64,
    pub MotionVectorHeapMemoryPoolL1Size: u64,
    pub MotionEstimatorMemoryPoolL0Size: u64,
    pub MotionEstimatorMemoryPoolL1Size: u64,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {
    pub NodeMask: u32,
    pub pOutputStreamDesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC,
    pub NumInputStreamDescs: u32,
    pub pInputStreamDescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC,
    pub Protected: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {
    pub NodeIndex: u32,
    pub MaxInputStreams: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {
    pub NodeIndex: u32,
    pub SupportFlags: D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS,
}
#[repr(C)]
#[cfg(feature = "Win32_dxgicommon")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {
    pub NodeIndex: u32,
    pub DeinterlaceMode: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS,
    pub Filters: D3D12_VIDEO_PROCESS_FILTER_FLAGS,
    pub FeatureSupport: D3D12_VIDEO_PROCESS_FEATURE_FLAGS,
    pub InputFrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub OutputFrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub EnableAutoProcessing: windows_core::BOOL,
    pub PastFrames: u32,
    pub FutureFrames: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {
    pub Status: u64,
    pub NumMacroblocksAffected: u64,
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub BitRate: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {
    pub pMotionVectorHeap: core::mem::ManuallyDrop<Option<ID3D12VideoMotionVectorHeap>>,
    pub PixelWidth: u32,
    pub PixelHeight: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {
    pub pMotionVectorTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub MotionVectorCoordinate: D3D12_RESOURCE_COORDINATE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_RESOURCE_COORDINATE {
    pub X: u64,
    pub Y: u32,
    pub Z: u32,
    pub SubresourceIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_DECODER_DESC {
    pub NodeMask: u32,
    pub Configuration: D3D12_VIDEO_DECODE_CONFIGURATION,
}
#[repr(C)]
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {
    pub pBuffer: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub Offset: u64,
    pub Size: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_DECODE_CONFIGURATION {
    pub DecodeProfile: windows_core::GUID,
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {
    pub Enable: windows_core::BOOL,
    pub pReferenceTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub ReferenceSubresource: u32,
    pub OutputColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub DecodeColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {
    pub Enable: windows_core::BOOL,
    pub pReferenceTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub ReferenceSubresource: u32,
    pub OutputColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub DecodeColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
}
pub type D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAG_NONE: D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    pub NumFrameArguments: u32,
    pub FrameArguments: [D3D12_VIDEO_DECODE_FRAME_ARGUMENT; 10],
    pub ReferenceFrames: D3D12_VIDEO_DECODE_REFERENCE_FRAMES,
    pub CompressedBitstream: D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM,
    pub pHeap: core::mem::ManuallyDrop<Option<ID3D12VideoDecoderHeap>>,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {
    pub Offset: u64,
    pub pBuffer: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {
    pub pOutputTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub OutputSubresource: u32,
    pub ConversionArguments: D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
#[derive(Clone, Debug, PartialEq)]
pub struct D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {
    pub pOutputTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
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
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_12BIT_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0x17127009_a00f_4ce1_994e_bf4081f6f3f0);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_12BIT_PROFILE2_420: windows_core::GUID = windows_core::GUID::from_u128(0x2d80bed6_9cac_4835_9e91_327bbc4f9ee8);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE0: windows_core::GUID = windows_core::GUID::from_u128(0xb8be4ccb_cf53_46ba_8d59_d6b8a6da5d2a);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE1: windows_core::GUID = windows_core::GUID::from_u128(0x6936ff0f_45b1_4163_9cc1_646ef6946108);
pub const D3D12_VIDEO_DECODE_PROFILE_AV1_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0x0c5f2aa1_e541_4089_bb7b_98110a19d7c8);
pub const D3D12_VIDEO_DECODE_PROFILE_H264: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_MULTIVIEW: windows_core::GUID = windows_core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_STEREO: windows_core::GUID = windows_core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const D3D12_VIDEO_DECODE_PROFILE_H264_STEREO_PROGRESSIVE: windows_core::GUID = windows_core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN: windows_core::GUID = windows_core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10: windows_core::GUID = windows_core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10_422: windows_core::GUID = windows_core::GUID::from_u128(0x0bac4fe5_1532_4429_a854_f84de04953db);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10_444: windows_core::GUID = windows_core::GUID::from_u128(0x0dabeffa_4458_4602_bc03_0795659d617c);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN10_EXT: windows_core::GUID = windows_core::GUID::from_u128(0x9cc55490_e37c_4932_8684_4920f9f6409c);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN12: windows_core::GUID = windows_core::GUID::from_u128(0x1a72925f_0c2c_4f15_96fb_b17d1473603f);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN12_422: windows_core::GUID = windows_core::GUID::from_u128(0x55bcac81_f311_4093_a7d0_1cbc0b849bee);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN12_444: windows_core::GUID = windows_core::GUID::from_u128(0x9798634d_fe9d_48e5_b4da_dbec45b3df01);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN16: windows_core::GUID = windows_core::GUID::from_u128(0xa4fbdbb0_a113_482b_a232_635cc0697f6d);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MAIN_444: windows_core::GUID = windows_core::GUID::from_u128(0x4008018f_f537_4b36_98cf_61af8a2c1a33);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MONOCHROME: windows_core::GUID = windows_core::GUID::from_u128(0x0685b993_3d8c_43a0_8b28_d74c2d6899a4);
pub const D3D12_VIDEO_DECODE_PROFILE_HEVC_MONOCHROME10: windows_core::GUID = windows_core::GUID::from_u128(0x142a1d0f_69dd_4ec9_8591_b12ffcb91a29);
pub const D3D12_VIDEO_DECODE_PROFILE_JPEG_VLD_420: windows_core::GUID = windows_core::GUID::from_u128(0xcf782c83_bef5_4a2c_87cb_6019e7b175ac);
pub const D3D12_VIDEO_DECODE_PROFILE_JPEG_VLD_422: windows_core::GUID = windows_core::GUID::from_u128(0xf04df417_eee2_4067_a778_f35c15ab9721);
pub const D3D12_VIDEO_DECODE_PROFILE_JPEG_VLD_444: windows_core::GUID = windows_core::GUID::from_u128(0x4cd00e17_89ba_48ef_b9f9_edcb82713f65);
pub const D3D12_VIDEO_DECODE_PROFILE_MJPEG_VLD_420: windows_core::GUID = windows_core::GUID::from_u128(0x725cb506_0c29_43c4_9440_8e9397903a04);
pub const D3D12_VIDEO_DECODE_PROFILE_MJPEG_VLD_422: windows_core::GUID = windows_core::GUID::from_u128(0x5b77b9cd_1a35_4c30_9fd8_ef4b60c035dd);
pub const D3D12_VIDEO_DECODE_PROFILE_MJPEG_VLD_444: windows_core::GUID = windows_core::GUID::from_u128(0xd95161f9_0d44_47e6_bcf5_1bfbfb268f97);
pub const D3D12_VIDEO_DECODE_PROFILE_MJPEG_VLD_4444: windows_core::GUID = windows_core::GUID::from_u128(0xc91748d5_fd18_4aca_9db3_3a6634ab547d);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG1_AND_MPEG2: windows_core::GUID = windows_core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG2: windows_core::GUID = windows_core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG4PT2_ADVSIMPLE_NOGMC: windows_core::GUID = windows_core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const D3D12_VIDEO_DECODE_PROFILE_MPEG4PT2_SIMPLE: windows_core::GUID = windows_core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const D3D12_VIDEO_DECODE_PROFILE_VC1: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_VC1_D2010: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const D3D12_VIDEO_DECODE_PROFILE_VP8: windows_core::GUID = windows_core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const D3D12_VIDEO_DECODE_PROFILE_VP9: windows_core::GUID = windows_core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const D3D12_VIDEO_DECODE_PROFILE_VP9_10BIT_PROFILE2: windows_core::GUID = windows_core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_VIDEO_DECODE_REFERENCE_FRAMES {
    pub NumTexture2Ds: u32,
    pub ppTexture2Ds: *mut Option<super::d3d12::ID3D12Resource>,
    pub pSubresources: *mut u32,
    pub ppHeaps: *mut Option<ID3D12VideoDecoderHeap>,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_AV1_CODEC_CONFIGURATION {
    pub FeatureFlags: D3D12_VIDEO_ENCODER_AV1_FEATURE_FLAGS,
    pub OrderHintBitsMinus1: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_AV1_FRAME_SUBREGION_LAYOUT_CONFIG_SUPPORT {
    pub Use128SuperBlocks: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_AV1_LEVEL_TIER_CONSTRAINTS {
    pub Level: D3D12_VIDEO_ENCODER_AV1_LEVELS,
    pub Tier: D3D12_VIDEO_ENCODER_AV1_TIER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_AV1_REFERENCE_PICTURE_WARPED_MOTION_INFO {
    pub TransformationType: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION,
    pub TransformationMatrix: [i32; 8],
    pub InvalidAffineSet: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_CODEC_AV1_LOOP_FILTER_DELTA_CONFIG {
    pub DeltaLFPresent: u64,
    pub DeltaLFMulti: u64,
    pub DeltaLFRes: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_CODEC_AV1_PICTURE_CONTROL_SUPPORT {
    pub PredictionMode: D3D12_VIDEO_ENCODER_AV1_COMP_PREDICTION_TYPE,
    pub MaxUniqueReferencesPerFrame: u32,
    pub SupportedFrameTypes: D3D12_VIDEO_ENCODER_AV1_FRAME_TYPE_FLAGS,
    pub SupportedReferenceWarpedMotionFlags: D3D12_VIDEO_ENCODER_AV1_REFERENCE_WARPED_MOTION_TRANSFORMATION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {
    pub MaxL0ReferencesForP: u32,
    pub MaxL0ReferencesForB: u32,
    pub MaxL1ReferencesForB: u32,
    pub MaxLongTermReferences: u32,
    pub MaxDPBCapacity: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {
    pub MaxL0ReferencesForP: u32,
    pub MaxL0ReferencesForB: u32,
    pub MaxL1ReferencesForB: u32,
    pub MaxLongTermReferences: u32,
    pub MaxDPBCapacity: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {
    pub pBuffer: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub FrameStartOffset: u64,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
pub struct D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1 {
    pub NotificationMode: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM_NOTIFICATION_MODE,
    pub Anonymous: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1_0,
}
#[cfg(feature = "Win32_d3d12")]
impl Clone for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
pub union D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1_0 {
    pub FrameOutputBuffer: core::mem::ManuallyDrop<D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM>,
    pub SubregionOutputBuffers: D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM,
}
#[cfg(feature = "Win32_d3d12")]
impl Clone for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_DIRTY_RECT_INFO {
    pub FullFrameIdentical: windows_core::BOOL,
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
pub struct D3D12_VIDEO_ENCODER_DIRTY_REGIONS {
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub Anonymous: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_0,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Clone for D3D12_VIDEO_ENCODER_DIRTY_REGIONS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_DIRTY_REGIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub union D3D12_VIDEO_ENCODER_DIRTY_REGIONS_0 {
    pub pOpaqueLayoutBuffer: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub pCPUBuffer: *mut D3D12_VIDEO_ENCODER_DIRTY_RECT_INFO,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Clone for D3D12_VIDEO_ENCODER_DIRTY_REGIONS_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_DIRTY_REGIONS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_DIRTY_REGIONS_CONFIGURATION {
    pub Enabled: windows_core::BOOL,
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
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {
    pub SequenceControlDesc: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC,
    pub PictureControlDesc: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC,
    pub pInputFrame: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub InputFrameSubresource: u32,
    pub CurrentFrameBitstreamMetadataSize: u32,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
impl Clone for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon"))]
impl Default for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1 {
    pub SequenceControlDesc: D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC,
    pub PictureControlDesc: D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC1,
    pub pInputFrame: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub InputFrameSubresource: u32,
    pub CurrentFrameBitstreamMetadataSize: u32,
    pub OptionalMetadata: D3D12_VIDEO_ENCODER_OPTIONAL_METADATA_ENABLE_FLAGS,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl Clone for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {
    pub Bitstream: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM,
    pub ReconstructedPicture: D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE,
    pub EncoderOutputMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
pub struct D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1 {
    pub Bitstream: D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM1,
    pub ReconstructedPicture: D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE,
    pub EncoderOutputMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
    pub FrameAnalysisReconstructedPicture: D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE,
}
#[cfg(feature = "Win32_d3d12")]
impl Clone for D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {
    pub pBuffer: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub Offset: u64,
}
pub type D3D12_VIDEO_ENCODER_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_FLAG_NONE: D3D12_VIDEO_ENCODER_FLAGS = 0;
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_FRAME_ANALYSIS {
    pub pDownscaledFrame: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub Subresource: u64,
    pub DownscaledReferences: D3D12_VIDEO_ENCODE_REFERENCE_FRAMES,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_FRAME_ANALYSIS_CONFIGURATION {
    pub Enabled: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_CONFIG {
    pub MotionSearchMode: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE,
    pub SearchDeviationLimit: u32,
}
pub const D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_FULL_SEARCH: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE = 0;
pub const D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_START_HINT: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE = 1;
pub const D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_START_HINT_LIMITED_DISTANCE: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE = 2;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub struct D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS {
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub Anonymous: D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS_0,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Clone for D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Default for D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub union D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS_0 {
    pub pOpaqueLayoutBuffer: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub pCPUBuffer: *mut D3D12_VIDEO_ENCODER_MOVEREGION_INFO,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl Clone for D3D12_VIDEO_ENCODER_FRAME_MOTION_VECTORS_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_DATA {
    pub MapType: D3D12_VIDEO_ENCODER_INPUT_MAP_TYPE,
    pub Anonymous: D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_0,
}
#[cfg(feature = "Win32_d3d12")]
impl Clone for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
pub union D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_0 {
    pub Quantization: core::mem::ManuallyDrop<D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_QUANTIZATION_MATRIX>,
    pub DirtyRegions: core::mem::ManuallyDrop<D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_DIRTY_REGIONS>,
    pub MotionVectors: D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_MOTION_VECTORS,
}
#[cfg(feature = "Win32_d3d12")]
impl Clone for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_DIRTY_REGIONS {
    pub FullFrameIdentical: windows_core::BOOL,
    pub MapValuesType: D3D12_VIDEO_ENCODER_DIRTY_REGIONS_MAP_VALUES_MODE,
    pub pDirtyRegionsMap: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub SourceDPBFrameReference: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy)]
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_MOTION_VECTORS {
    pub MotionSearchModeConfiguration: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE_CONFIG,
    pub NumHintsPerPixel: u32,
    pub ppMotionVectorMaps: *mut Option<super::d3d12::ID3D12Resource>,
    pub pMotionVectorMapsSubresources: *mut u32,
    pub ppMotionVectorMapsMetadata: *mut Option<super::d3d12::ID3D12Resource>,
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_INPUT_MAP_DATA_QUANTIZATION_MATRIX {
    pub pQuantizationMap: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_MOTION_SEARCH_CONFIGURATION {
    pub Enabled: windows_core::BOOL,
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
    pub MotionSearchMode: D3D12_VIDEO_ENCODER_FRAME_MOTION_SEARCH_MODE,
    pub BidirectionalRefFrameEnabled: windows_core::BOOL,
}
pub type D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = u32;
pub const D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAG_GPU_TEXTURE_MULTIPLE_REFERENCES: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = 4;
pub const D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAG_MULTIPLE_HINTS: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = 2;
pub const D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAG_NONE: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = 0;
pub const D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAG_SUPPORTED: D3D12_VIDEO_ENCODER_MOTION_SEARCH_SUPPORT_FLAGS = 1;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_OUTPUT_METADATA {
    pub EncodeErrorFlags: u64,
    pub EncodeStats: D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS,
    pub EncodedBitstreamWrittenBytesCount: u64,
    pub WrittenSubregionsCount: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {
    pub modification_of_pic_nums_idc: u8,
    pub abs_diff_pic_num_minus1: u32,
    pub long_term_pic_num: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {
    pub memory_management_control_operation: u8,
    pub difference_of_pic_nums_minus1: u32,
    pub long_term_pic_num: u32,
    pub long_term_frame_idx: u32,
    pub max_long_term_frame_idx_plus1: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Clone for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {
    pub Width: u32,
    pub Height: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_QPMAP_CONFIGURATION {
    pub Enabled: windows_core::BOOL,
    pub MapSource: D3D12_VIDEO_ENCODER_INPUT_MAP_SOURCE,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_QUANTIZATION_OPAQUE_MAP {
    pub pOpaqueQuantizationMap: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_ABSOLUTE_QP_MAP {
    pub QualityVsSpeed: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {
    pub ConstantQP_FullIntracodedFrame: u32,
    pub ConstantQP_InterPredictedFrame_PrevRefOnly: u32,
    pub ConstantQP_InterPredictedFrame_BiDirectionalRef: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {
    pub pReconstructedPicture: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub ReconstructedPictureSubresource: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {
    pub ReconstructedPictureResourceIndex: u32,
    pub IsLongTermReference: windows_core::BOOL,
    pub LongTermPictureIdx: u32,
    pub PictureOrderCountNumber: u32,
    pub FrameDecodingOrderNumber: u32,
    pub TemporalLayerIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {
    pub ReconstructedPictureResourceIndex: u32,
    pub IsRefUsedByCurrentPic: windows_core::BOOL,
    pub IsLongTermReference: windows_core::BOOL,
    pub PictureOrderCountNumber: u32,
    pub TemporalLayerIndex: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS {
    pub SessionInfo: D3D12_VIDEO_ENCODER_INPUT_MAP_SESSION_INFO,
    pub InputData: D3D12_VIDEO_ENCODER_INPUT_MAP_DATA,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl Clone for D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl Default for D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_OUTPUT_ARGUMENTS {
    pub pOpaqueLayoutBuffer: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {
    pub EncoderCodec: D3D12_VIDEO_ENCODER_CODEC,
    pub EncoderProfile: D3D12_VIDEO_ENCODER_PROFILE_DESC,
    pub EncoderInputFormat: super::dxgiformat::DXGI_FORMAT,
    pub EncodedPictureEffectiveResolution: D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC,
    pub HWLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl Clone for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl Default for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
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
impl Clone for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl Default for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {
    pub ResolvedLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS1 {
    pub ResolvedLayoutMetadata: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
    pub pOutputQPMap: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub pOutputSATDMap: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub pOutputBitAllocationMap: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub ResolvedFramePSNRData: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
    pub ResolvedSubregionsPSNRData: D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {
    pub GOPLength: u32,
    pub PPicturePeriod: u32,
    pub pic_order_cnt_type: u8,
    pub log2_max_frame_num_minus4: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {
    pub GOPLength: u32,
    pub PPicturePeriod: u32,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM {
    pub BufferMode: D3D12_VIDEO_ENCODER_SUBREGION_COMPRESSED_BITSTREAM_BUFFER_MODE,
    pub ExpectedSubregionCount: u32,
    pub pSubregionBitstreamsBaseOffsets: *mut u64,
    pub ppSubregionBitstreams: *mut Option<super::d3d12::ID3D12Resource>,
    pub ppSubregionSizes: *mut Option<super::d3d12::ID3D12Resource>,
    pub ppSubregionOffsets: *mut Option<super::d3d12::ID3D12Resource>,
    pub ppSubregionFences: *mut Option<super::d3d12::ID3D12Fence>,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    pub NumTexture2Ds: u32,
    pub ppTexture2Ds: *mut Option<super::d3d12::ID3D12Resource>,
    pub pSubresources: *mut u32,
}
#[cfg(feature = "Win32_d3d12")]
impl Default for D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_DESC {
    pub NodeMask: u32,
    pub CommandId: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_INFO {
    pub CommandId: windows_core::GUID,
    pub Name: windows_core::PCWSTR,
    pub CommandListSupportFlags: super::d3d12::D3D12_COMMAND_LIST_SUPPORT_FLAGS,
}
pub type D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = u32;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_NONE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 0;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_READ: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 1;
pub const D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAG_WRITE: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {
    pub Name: windows_core::PCWSTR,
    pub Type: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE,
    pub Flags: D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_DESC {
    pub NodeMask: u32,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {
    pub pInputTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub InputSubresourceIndex: u32,
    pub pReferenceTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub ReferenceSubresourceIndex: u32,
    pub pHintMotionVectorHeap: core::mem::ManuallyDrop<Option<ID3D12VideoMotionVectorHeap>>,
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {
    pub pMotionVectorHeap: core::mem::ManuallyDrop<Option<ID3D12VideoMotionVectorHeap>>,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {
    pub NodeMask: u32,
    pub InputFormat: super::dxgiformat::DXGI_FORMAT,
    pub BlockSize: D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE,
    pub Precision: D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION,
    pub SizeRange: D3D12_VIDEO_SIZE_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_PROCESS_ALPHA_BLENDING {
    pub Enable: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM {
    pub pTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub Subresource: u32,
    pub ReferenceSet: D3D12_VIDEO_PROCESS_REFERENCE_SET,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub SourceAspectRatio: super::dxgicommon::DXGI_RATIONAL,
    pub DestinationAspectRatio: super::dxgicommon::DXGI_RATIONAL,
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub SourceSizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub DestinationSizeRange: D3D12_VIDEO_SIZE_RANGE,
    pub EnableOrientation: windows_core::BOOL,
    pub FilterFlags: D3D12_VIDEO_PROCESS_FILTER_FLAGS,
    pub StereoFormat: D3D12_VIDEO_FRAME_STEREO_FORMAT,
    pub FieldType: D3D12_VIDEO_FIELD_TYPE,
    pub DeinterlaceMode: D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS,
    pub EnableAlphaBlending: windows_core::BOOL,
    pub LumaKey: D3D12_VIDEO_PROCESS_LUMA_KEY,
    pub NumPastFrames: u32,
    pub NumFutureFrames: u32,
    pub EnableAutoProcessing: windows_core::BOOL,
}
pub type D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = u32;
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_FRAME_DISCONTINUITY: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 1;
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_FRAME_REPEAT: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 2;
pub const D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAG_NONE: D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_PROCESS_LUMA_KEY {
    pub Enable: windows_core::BOOL,
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_PROCESS_OUTPUT_STREAM {
    pub pTexture2D: core::mem::ManuallyDrop<Option<super::d3d12::ID3D12Resource>>,
    pub Subresource: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    pub Format: super::dxgiformat::DXGI_FORMAT,
    pub ColorSpace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    pub AlphaFillMode: D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE,
    pub AlphaFillModeSourceStreamIndex: u32,
    pub BackgroundColor: [f32; 4],
    pub FrameRate: super::dxgicommon::DXGI_RATIONAL,
    pub EnableStereo: windows_core::BOOL,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl Default for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_d3d12")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D12_VIDEO_PROCESS_REFERENCE_SET {
    pub NumPastFrames: u32,
    pub ppPastFrames: *mut Option<super::d3d12::ID3D12Resource>,
    pub pPastSubresources: *mut u32,
    pub NumFutureFrames: u32,
    pub ppFutureFrames: *mut Option<super::d3d12::ID3D12Resource>,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_SAMPLE {
    pub Width: u32,
    pub Height: u32,
    pub Format: D3D12_VIDEO_FORMAT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D12_VIDEO_SIZE_RANGE {
    pub MaxWidth: u32,
    pub MaxHeight: u32,
    pub MinWidth: u32,
    pub MinHeight: u32,
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoDecodeCommandList, ID3D12VideoDecodeCommandList_Vtbl, 0x3b60536e_ad29_4e64_a269_f853837e5e53);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoDecodeCommandList {
    type Target = super::d3d12::ID3D12CommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoDecodeCommandList, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecodeCommandList {
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d12::ID3D12CommandAllocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), pallocator.param().abi()) }
    }
    pub unsafe fn ClearState(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::d3d12::D3D12_RESOURCE_BARRIER]) {
        unsafe {
            (windows_core::Interface::vtable(self).ResourceBarrier)(windows_core::Interface::as_raw(self), pbarriers.len().try_into().unwrap(), core::mem::transmute(pbarriers.as_ptr()));
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: Option<*const super::d3d12::D3D12_DISCARD_REGION>)
    where
        P0: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DiscardResource)(windows_core::Interface::as_raw(self), presource.param().abi(), pregion.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).BeginQuery)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, index);
        }
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).EndQuery)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, index);
        }
    }
    pub unsafe fn ResolveQueryData<P0, P4>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P4, aligneddestinationbufferoffset: u64)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
        P4: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveQueryData)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, startindex, numqueries, pdestinationbuffer.param().abi(), aligneddestinationbufferoffset);
        }
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP)
    where
        P0: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), pbuffer.param().abi(), alignedbufferoffset, operation);
        }
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMarker)(windows_core::Interface::as_raw(self), metadata, pdata.unwrap_or(core::mem::zeroed()) as _, size);
        }
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginEvent)(windows_core::Interface::as_raw(self), metadata, pdata.unwrap_or(core::mem::zeroed()) as _, size);
        }
    }
    pub unsafe fn EndEvent(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn DecodeFrame<P0>(&self, pdecoder: P0, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS)
    where
        P0: windows_core::Param<ID3D12VideoDecoder>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DecodeFrame)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), core::mem::transmute(poutputarguments), core::mem::transmute(pinputarguments));
        }
    }
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: Option<*const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBufferImmediate)(windows_core::Interface::as_raw(self), count, pparams, pmodes.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDecodeCommandList_Vtbl {
    pub base__: super::d3d12::ID3D12CommandList_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub ResourceBarrier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_RESOURCE_BARRIER),
    #[cfg(feature = "Win32_windef")]
    pub DiscardResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::d3d12::D3D12_DISCARD_REGION),
    #[cfg(not(feature = "Win32_windef"))]
    DiscardResource: usize,
    pub BeginQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32),
    pub EndQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32),
    pub ResolveQueryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32, u32, *mut core::ffi::c_void, u64),
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, super::d3d12::D3D12_PREDICATION_OP),
    pub SetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub BeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_dxgicommon")]
    pub DecodeFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS, *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    DecodeFrame: usize,
    pub WriteBufferImmediate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
pub trait ID3D12VideoDecodeCommandList_Impl: super::d3d12::ID3D12CommandList_Impl {
    fn Close(&self) -> windows_core::Result<()>;
    fn Reset(&self, pallocator: windows_core::Ref<super::d3d12::ID3D12CommandAllocator>) -> windows_core::Result<()>;
    fn ClearState(&self);
    fn ResourceBarrier(&self, numbarriers: u32, pbarriers: *const super::d3d12::D3D12_RESOURCE_BARRIER);
    fn DiscardResource(&self, presource: windows_core::Ref<super::d3d12::ID3D12Resource>, pregion: *const super::d3d12::D3D12_DISCARD_REGION);
    fn BeginQuery(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32);
    fn EndQuery(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32);
    fn ResolveQueryData(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: windows_core::Ref<super::d3d12::ID3D12Resource>, aligneddestinationbufferoffset: u64);
    fn SetPredication(&self, pbuffer: windows_core::Ref<super::d3d12::ID3D12Resource>, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP);
    fn SetMarker(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn BeginEvent(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn EndEvent(&self);
    fn DecodeFrame(&self, pdecoder: windows_core::Ref<ID3D12VideoDecoder>, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS);
    fn WriteBufferImmediate(&self, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl ID3D12VideoDecodeCommandList_Vtbl {
    pub const fn new<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::Reset(this, core::mem::transmute_copy(&pallocator)).into()
            }
        }
        unsafe extern "system" fn ClearState<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::ClearState(this);
            }
        }
        unsafe extern "system" fn ResourceBarrier<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbarriers: u32, pbarriers: *const super::d3d12::D3D12_RESOURCE_BARRIER) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::ResourceBarrier(this, core::mem::transmute_copy(&numbarriers), core::mem::transmute_copy(&pbarriers));
            }
        }
        unsafe extern "system" fn DiscardResource<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pregion: *const super::d3d12::D3D12_DISCARD_REGION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::DiscardResource(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pregion));
            }
        }
        unsafe extern "system" fn BeginQuery<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::BeginQuery(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&index));
            }
        }
        unsafe extern "system" fn EndQuery<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::EndQuery(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&index));
            }
        }
        unsafe extern "system" fn ResolveQueryData<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: *mut core::ffi::c_void, aligneddestinationbufferoffset: u64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::ResolveQueryData(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&numqueries), core::mem::transmute_copy(&pdestinationbuffer), core::mem::transmute_copy(&aligneddestinationbufferoffset));
            }
        }
        unsafe extern "system" fn SetPredication<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::SetPredication(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&alignedbufferoffset), core::mem::transmute_copy(&operation));
            }
        }
        unsafe extern "system" fn SetMarker<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::SetMarker(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size));
            }
        }
        unsafe extern "system" fn BeginEvent<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::BeginEvent(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size));
            }
        }
        unsafe extern "system" fn EndEvent<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::EndEvent(this);
            }
        }
        unsafe extern "system" fn DecodeFrame<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::DecodeFrame(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&poutputarguments), core::mem::transmute_copy(&pinputarguments));
            }
        }
        unsafe extern "system" fn WriteBufferImmediate<Identity: ID3D12VideoDecodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList_Impl::WriteBufferImmediate(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&pmodes));
            }
        }
        Self {
            base__: super::d3d12::ID3D12CommandList_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ClearState: ClearState::<Identity, OFFSET>,
            ResourceBarrier: ResourceBarrier::<Identity, OFFSET>,
            DiscardResource: DiscardResource::<Identity, OFFSET>,
            BeginQuery: BeginQuery::<Identity, OFFSET>,
            EndQuery: EndQuery::<Identity, OFFSET>,
            ResolveQueryData: ResolveQueryData::<Identity, OFFSET>,
            SetPredication: SetPredication::<Identity, OFFSET>,
            SetMarker: SetMarker::<Identity, OFFSET>,
            BeginEvent: BeginEvent::<Identity, OFFSET>,
            EndEvent: EndEvent::<Identity, OFFSET>,
            DecodeFrame: DecodeFrame::<Identity, OFFSET>,
            WriteBufferImmediate: WriteBufferImmediate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDecodeCommandList as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoDecodeCommandList {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoDecodeCommandList1, ID3D12VideoDecodeCommandList1_Vtbl, 0xd52f011b_b56e_453c_a05a_a7f311c8f472);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoDecodeCommandList1 {
    type Target = ID3D12VideoDecodeCommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoDecodeCommandList1, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoDecodeCommandList);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecodeCommandList1 {
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn DecodeFrame1<P0>(&self, pdecoder: P0, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS)
    where
        P0: windows_core::Param<ID3D12VideoDecoder>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DecodeFrame1)(windows_core::Interface::as_raw(self), pdecoder.param().abi(), core::mem::transmute(poutputarguments), core::mem::transmute(pinputarguments));
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDecodeCommandList1_Vtbl {
    pub base__: ID3D12VideoDecodeCommandList_Vtbl,
    #[cfg(feature = "Win32_dxgicommon")]
    pub DecodeFrame1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1, *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    DecodeFrame1: usize,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
pub trait ID3D12VideoDecodeCommandList1_Impl: ID3D12VideoDecodeCommandList_Impl {
    fn DecodeFrame1(&self, pdecoder: windows_core::Ref<ID3D12VideoDecoder>, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl ID3D12VideoDecodeCommandList1_Vtbl {
    pub const fn new<Identity: ID3D12VideoDecodeCommandList1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DecodeFrame1<Identity: ID3D12VideoDecodeCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList1_Impl::DecodeFrame1(this, core::mem::transmute_copy(&pdecoder), core::mem::transmute_copy(&poutputarguments), core::mem::transmute_copy(&pinputarguments));
            }
        }
        Self { base__: ID3D12VideoDecodeCommandList_Vtbl::new::<Identity, OFFSET>(), DecodeFrame1: DecodeFrame1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDecodeCommandList1 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoDecodeCommandList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoDecodeCommandList1 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoDecodeCommandList2, ID3D12VideoDecodeCommandList2_Vtbl, 0x6e120880_c114_4153_8036_d247051e1729);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoDecodeCommandList2 {
    type Target = ID3D12VideoDecodeCommandList1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoDecodeCommandList2, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoDecodeCommandList, ID3D12VideoDecodeCommandList1);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecodeCommandList2 {
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetProtectedResourceSession)(windows_core::Interface::as_raw(self), pprotectedresourcesession.param().abi());
        }
    }
    pub unsafe fn InitializeExtensionCommand<P0>(&self, pextensioncommand: P0, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize)
    where
        P0: windows_core::Param<ID3D12VideoExtensionCommand>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeExtensionCommand)(windows_core::Interface::as_raw(self), pextensioncommand.param().abi(), pinitializationparameters, initializationparameterssizeinbytes);
        }
    }
    pub unsafe fn ExecuteExtensionCommand<P0>(&self, pextensioncommand: P0, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize)
    where
        P0: windows_core::Param<ID3D12VideoExtensionCommand>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ExecuteExtensionCommand)(windows_core::Interface::as_raw(self), pextensioncommand.param().abi(), pexecutionparameters, executionparameterssizeinbytes);
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDecodeCommandList2_Vtbl {
    pub base__: ID3D12VideoDecodeCommandList1_Vtbl,
    pub SetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub InitializeExtensionCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize),
    pub ExecuteExtensionCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
pub trait ID3D12VideoDecodeCommandList2_Impl: ID3D12VideoDecodeCommandList1_Impl {
    fn SetProtectedResourceSession(&self, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>);
    fn InitializeExtensionCommand(&self, pextensioncommand: windows_core::Ref<ID3D12VideoExtensionCommand>, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize);
    fn ExecuteExtensionCommand(&self, pextensioncommand: windows_core::Ref<ID3D12VideoExtensionCommand>, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl ID3D12VideoDecodeCommandList2_Vtbl {
    pub const fn new<Identity: ID3D12VideoDecodeCommandList2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProtectedResourceSession<Identity: ID3D12VideoDecodeCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotectedresourcesession: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList2_Impl::SetProtectedResourceSession(this, core::mem::transmute_copy(&pprotectedresourcesession));
            }
        }
        unsafe extern "system" fn InitializeExtensionCommand<Identity: ID3D12VideoDecodeCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextensioncommand: *mut core::ffi::c_void, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList2_Impl::InitializeExtensionCommand(this, core::mem::transmute_copy(&pextensioncommand), core::mem::transmute_copy(&pinitializationparameters), core::mem::transmute_copy(&initializationparameterssizeinbytes));
            }
        }
        unsafe extern "system" fn ExecuteExtensionCommand<Identity: ID3D12VideoDecodeCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextensioncommand: *mut core::ffi::c_void, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList2_Impl::ExecuteExtensionCommand(this, core::mem::transmute_copy(&pextensioncommand), core::mem::transmute_copy(&pexecutionparameters), core::mem::transmute_copy(&executionparameterssizeinbytes));
            }
        }
        Self {
            base__: ID3D12VideoDecodeCommandList1_Vtbl::new::<Identity, OFFSET>(),
            SetProtectedResourceSession: SetProtectedResourceSession::<Identity, OFFSET>,
            InitializeExtensionCommand: InitializeExtensionCommand::<Identity, OFFSET>,
            ExecuteExtensionCommand: ExecuteExtensionCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDecodeCommandList2 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoDecodeCommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoDecodeCommandList1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoDecodeCommandList2 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoDecodeCommandList3, ID3D12VideoDecodeCommandList3_Vtbl, 0x2aee8c37_9562_42da_8abf_61efeb2e4513);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoDecodeCommandList3 {
    type Target = ID3D12VideoDecodeCommandList2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoDecodeCommandList3, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoDecodeCommandList, ID3D12VideoDecodeCommandList1, ID3D12VideoDecodeCommandList2);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecodeCommandList3 {
    pub unsafe fn Barrier(&self, pbarriergroups: &[super::d3d12::D3D12_BARRIER_GROUP]) {
        unsafe {
            (windows_core::Interface::vtable(self).Barrier)(windows_core::Interface::as_raw(self), pbarriergroups.len().try_into().unwrap(), core::mem::transmute(pbarriergroups.as_ptr()));
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDecodeCommandList3_Vtbl {
    pub base__: ID3D12VideoDecodeCommandList2_Vtbl,
    pub Barrier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_BARRIER_GROUP),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
pub trait ID3D12VideoDecodeCommandList3_Impl: ID3D12VideoDecodeCommandList2_Impl {
    fn Barrier(&self, numbarriergroups: u32, pbarriergroups: *const super::d3d12::D3D12_BARRIER_GROUP);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl ID3D12VideoDecodeCommandList3_Vtbl {
    pub const fn new<Identity: ID3D12VideoDecodeCommandList3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Barrier<Identity: ID3D12VideoDecodeCommandList3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbarriergroups: u32, pbarriergroups: *const super::d3d12::D3D12_BARRIER_GROUP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecodeCommandList3_Impl::Barrier(this, core::mem::transmute_copy(&numbarriergroups), core::mem::transmute_copy(&pbarriergroups));
            }
        }
        Self { base__: ID3D12VideoDecodeCommandList2_Vtbl::new::<Identity, OFFSET>(), Barrier: Barrier::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDecodeCommandList3 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoDecodeCommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoDecodeCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12VideoDecodeCommandList2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoDecodeCommandList3 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoDecoder, ID3D12VideoDecoder_Vtbl, 0xc59b6bdc_7720_4074_a136_17a156037470);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoDecoder {
    type Target = super::d3d12::ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoDecoder, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecoder {
    pub unsafe fn GetDesc(&self) -> D3D12_VIDEO_DECODER_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDecoder_Vtbl {
    pub base__: super::d3d12::ID3D12Pageable_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_VIDEO_DECODER_DESC),
}
#[cfg(feature = "Win32_d3d12")]
pub trait ID3D12VideoDecoder_Impl: super::d3d12::ID3D12Pageable_Impl {
    fn GetDesc(&self) -> D3D12_VIDEO_DECODER_DESC;
}
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecoder_Vtbl {
    pub const fn new<Identity: ID3D12VideoDecoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12VideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_VIDEO_DECODER_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID3D12VideoDecoder_Impl::GetDesc(this);
            }
        }
        Self { base__: super::d3d12::ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDecoder as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d12")]
impl windows_core::RuntimeName for ID3D12VideoDecoder {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoDecoder1, ID3D12VideoDecoder1_Vtbl, 0x79a2e5fb_ccd2_469a_9fde_195d10951f7e);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoDecoder1 {
    type Target = ID3D12VideoDecoder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoDecoder1, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable, ID3D12VideoDecoder);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecoder1 {
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProtectedResourceSession)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok() }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDecoder1_Vtbl {
    pub base__: ID3D12VideoDecoder_Vtbl,
    pub GetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3d12")]
pub trait ID3D12VideoDecoder1_Impl: ID3D12VideoDecoder_Impl {
    fn GetProtectedResourceSession(&self, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecoder1_Vtbl {
    pub const fn new<Identity: ID3D12VideoDecoder1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ID3D12VideoDecoder1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecoder1_Impl::GetProtectedResourceSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppprotectedsession)).into()
            }
        }
        Self { base__: ID3D12VideoDecoder_Vtbl::new::<Identity, OFFSET>(), GetProtectedResourceSession: GetProtectedResourceSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDecoder1 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12VideoDecoder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d12")]
impl windows_core::RuntimeName for ID3D12VideoDecoder1 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoDecoderHeap, ID3D12VideoDecoderHeap_Vtbl, 0x0946b7c9_ebf6_4047_bb73_8683e27dbb1f);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoDecoderHeap {
    type Target = super::d3d12::ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoDecoderHeap, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecoderHeap {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetDesc(&self) -> D3D12_VIDEO_DECODER_HEAP_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDecoderHeap_Vtbl {
    pub base__: super::d3d12::ID3D12Pageable_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_VIDEO_DECODER_HEAP_DESC),
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    GetDesc: usize,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoDecoderHeap_Impl: super::d3d12::ID3D12Pageable_Impl {
    fn GetDesc(&self) -> D3D12_VIDEO_DECODER_HEAP_DESC;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoDecoderHeap_Vtbl {
    pub const fn new<Identity: ID3D12VideoDecoderHeap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12VideoDecoderHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_VIDEO_DECODER_HEAP_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID3D12VideoDecoderHeap_Impl::GetDesc(this);
            }
        }
        Self { base__: super::d3d12::ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDecoderHeap as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoDecoderHeap {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoDecoderHeap1, ID3D12VideoDecoderHeap1_Vtbl, 0xda1d98c5_539f_41b2_bf6b_1198a03b6d26);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoDecoderHeap1 {
    type Target = ID3D12VideoDecoderHeap;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoDecoderHeap1, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable, ID3D12VideoDecoderHeap);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoDecoderHeap1 {
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProtectedResourceSession)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok() }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDecoderHeap1_Vtbl {
    pub base__: ID3D12VideoDecoderHeap_Vtbl,
    pub GetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoDecoderHeap1_Impl: ID3D12VideoDecoderHeap_Impl {
    fn GetProtectedResourceSession(&self, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoDecoderHeap1_Vtbl {
    pub const fn new<Identity: ID3D12VideoDecoderHeap1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ID3D12VideoDecoderHeap1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDecoderHeap1_Impl::GetProtectedResourceSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppprotectedsession)).into()
            }
        }
        Self { base__: ID3D12VideoDecoderHeap_Vtbl::new::<Identity, OFFSET>(), GetProtectedResourceSession: GetProtectedResourceSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDecoderHeap1 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12VideoDecoderHeap as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoDecoderHeap1 {}
windows_core::imp::define_interface!(ID3D12VideoDevice, ID3D12VideoDevice_Vtbl, 0x1f052807_0b46_4acc_8a89_364f793718a4);
windows_core::imp::interface_hierarchy!(ID3D12VideoDevice, windows_core::IUnknown);
impl ID3D12VideoDevice {
    pub unsafe fn CheckFeatureSupport(&self, featurevideo: D3D12_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), featurevideo, pfeaturesupportdata as _, featuresupportdatasize) }
    }
    pub unsafe fn CreateVideoDecoder<T>(&self, pdesc: *const D3D12_VIDEO_DECODER_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoDecoder)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateVideoDecoderHeap<T>(&self, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoDecoderHeap)(windows_core::Interface::as_raw(self), pvideodecoderheapdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateVideoProcessor<T>(&self, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, pinputstreamdescs: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC]) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoProcessor)(windows_core::Interface::as_raw(self), nodemask, poutputstreamdesc, pinputstreamdescs.len().try_into().unwrap(), core::mem::transmute(pinputstreamdescs.as_ptr()), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_FEATURE_VIDEO, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateVideoDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_DECODER_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateVideoDecoderHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_DECODER_HEAP_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateVideoDecoderHeap: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateVideoProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, u32, *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateVideoProcessor: usize,
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoDevice_Impl: windows_core::IUnknownImpl {
    fn CheckFeatureSupport(&self, featurevideo: D3D12_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
    fn CreateVideoDecoder(&self, pdesc: *const D3D12_VIDEO_DECODER_DESC, riid: *const windows_core::GUID, ppvideodecoder: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateVideoDecoderHeap(&self, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC, riid: *const windows_core::GUID, ppvideodecoderheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateVideoProcessor(&self, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, numinputstreamdescs: u32, pinputstreamdescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC, riid: *const windows_core::GUID, ppvideoprocessor: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoDevice_Vtbl {
    pub const fn new<Identity: ID3D12VideoDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ID3D12VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, featurevideo: D3D12_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&featurevideo), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
            }
        }
        unsafe extern "system" fn CreateVideoDecoder<Identity: ID3D12VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VIDEO_DECODER_DESC, riid: *const windows_core::GUID, ppvideodecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice_Impl::CreateVideoDecoder(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideodecoder)).into()
            }
        }
        unsafe extern "system" fn CreateVideoDecoderHeap<Identity: ID3D12VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC, riid: *const windows_core::GUID, ppvideodecoderheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice_Impl::CreateVideoDecoderHeap(this, core::mem::transmute_copy(&pvideodecoderheapdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideodecoderheap)).into()
            }
        }
        unsafe extern "system" fn CreateVideoProcessor<Identity: ID3D12VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, numinputstreamdescs: u32, pinputstreamdescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC, riid: *const windows_core::GUID, ppvideoprocessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice_Impl::CreateVideoProcessor(this, core::mem::transmute_copy(&nodemask), core::mem::transmute_copy(&poutputstreamdesc), core::mem::transmute_copy(&numinputstreamdescs), core::mem::transmute_copy(&pinputstreamdescs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideoprocessor)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET>,
            CreateVideoDecoder: CreateVideoDecoder::<Identity, OFFSET>,
            CreateVideoDecoderHeap: CreateVideoDecoderHeap::<Identity, OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoDevice {}
windows_core::imp::define_interface!(ID3D12VideoDevice1, ID3D12VideoDevice1_Vtbl, 0x981611ad_a144_4c83_9890_f30e26d658ab);
impl core::ops::Deref for ID3D12VideoDevice1 {
    type Target = ID3D12VideoDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12VideoDevice1, windows_core::IUnknown, ID3D12VideoDevice);
impl ID3D12VideoDevice1 {
    #[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateVideoMotionEstimator<P1, T>(&self, pdesc: *const D3D12_VIDEO_MOTION_ESTIMATOR_DESC, pprotectedresourcesession: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoMotionEstimator)(windows_core::Interface::as_raw(self), pdesc, pprotectedresourcesession.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateVideoMotionVectorHeap<P1, T>(&self, pdesc: *const D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC, pprotectedresourcesession: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoMotionVectorHeap)(windows_core::Interface::as_raw(self), pdesc, pprotectedresourcesession.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDevice1_Vtbl {
    pub base__: ID3D12VideoDevice_Vtbl,
    #[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
    pub CreateVideoMotionEstimator: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_MOTION_ESTIMATOR_DESC, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat")))]
    CreateVideoMotionEstimator: usize,
    #[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
    pub CreateVideoMotionVectorHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat")))]
    CreateVideoMotionVectorHeap: usize,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoDevice1_Impl: ID3D12VideoDevice_Impl {
    fn CreateVideoMotionEstimator(&self, pdesc: *const D3D12_VIDEO_MOTION_ESTIMATOR_DESC, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>, riid: *const windows_core::GUID, ppvideomotionestimator: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateVideoMotionVectorHeap(&self, pdesc: *const D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>, riid: *const windows_core::GUID, ppvideomotionvectorheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoDevice1_Vtbl {
    pub const fn new<Identity: ID3D12VideoDevice1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVideoMotionEstimator<Identity: ID3D12VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VIDEO_MOTION_ESTIMATOR_DESC, pprotectedresourcesession: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvideomotionestimator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice1_Impl::CreateVideoMotionEstimator(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pprotectedresourcesession), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideomotionestimator)).into()
            }
        }
        unsafe extern "system" fn CreateVideoMotionVectorHeap<Identity: ID3D12VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC, pprotectedresourcesession: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvideomotionvectorheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice1_Impl::CreateVideoMotionVectorHeap(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pprotectedresourcesession), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideomotionvectorheap)).into()
            }
        }
        Self {
            base__: ID3D12VideoDevice_Vtbl::new::<Identity, OFFSET>(),
            CreateVideoMotionEstimator: CreateVideoMotionEstimator::<Identity, OFFSET>,
            CreateVideoMotionVectorHeap: CreateVideoMotionVectorHeap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDevice1 as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoDevice1 {}
windows_core::imp::define_interface!(ID3D12VideoDevice2, ID3D12VideoDevice2_Vtbl, 0xf019ac49_f838_4a95_9b17_579437c8f513);
impl core::ops::Deref for ID3D12VideoDevice2 {
    type Target = ID3D12VideoDevice1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12VideoDevice2, windows_core::IUnknown, ID3D12VideoDevice, ID3D12VideoDevice1);
impl ID3D12VideoDevice2 {
    #[cfg(feature = "Win32_d3d12")]
    pub unsafe fn CreateVideoDecoder1<P1, T>(&self, pdesc: *const D3D12_VIDEO_DECODER_DESC, pprotectedresourcesession: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoDecoder1)(windows_core::Interface::as_raw(self), pdesc, pprotectedresourcesession.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateVideoDecoderHeap1<P1, T>(&self, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC, pprotectedresourcesession: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoDecoderHeap1)(windows_core::Interface::as_raw(self), pvideodecoderheapdesc, pprotectedresourcesession.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CreateVideoProcessor1<P4, T>(&self, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, pinputstreamdescs: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC], pprotectedresourcesession: P4) -> windows_core::Result<T>
    where
        P4: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoProcessor1)(windows_core::Interface::as_raw(self), nodemask, poutputstreamdesc, pinputstreamdescs.len().try_into().unwrap(), core::mem::transmute(pinputstreamdescs.as_ptr()), pprotectedresourcesession.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_d3d12")]
    pub unsafe fn CreateVideoExtensionCommand<P3, T>(&self, pdesc: *const D3D12_VIDEO_EXTENSION_COMMAND_DESC, pcreationparameters: *const core::ffi::c_void, creationparametersdatasizeinbytes: usize, pprotectedresourcesession: P3) -> windows_core::Result<T>
    where
        P3: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoExtensionCommand)(windows_core::Interface::as_raw(self), pdesc, pcreationparameters, creationparametersdatasizeinbytes, pprotectedresourcesession.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_d3d12")]
    pub unsafe fn ExecuteExtensionCommand<P0>(&self, pextensioncommand: P0, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize, poutputdata: *mut core::ffi::c_void, outputdatasizeinbytes: usize) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID3D12VideoExtensionCommand>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExecuteExtensionCommand)(windows_core::Interface::as_raw(self), pextensioncommand.param().abi(), pexecutionparameters, executionparameterssizeinbytes, poutputdata as _, outputdatasizeinbytes) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDevice2_Vtbl {
    pub base__: ID3D12VideoDevice1_Vtbl,
    #[cfg(feature = "Win32_d3d12")]
    pub CreateVideoDecoder1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_DECODER_DESC, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d12"))]
    CreateVideoDecoder1: usize,
    #[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateVideoDecoderHeap1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_DECODER_HEAP_DESC, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateVideoDecoderHeap1: usize,
    #[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CreateVideoProcessor1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, u32, *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CreateVideoProcessor1: usize,
    #[cfg(feature = "Win32_d3d12")]
    pub CreateVideoExtensionCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_EXTENSION_COMMAND_DESC, *const core::ffi::c_void, usize, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d12"))]
    CreateVideoExtensionCommand: usize,
    #[cfg(feature = "Win32_d3d12")]
    pub ExecuteExtensionCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d12"))]
    ExecuteExtensionCommand: usize,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoDevice2_Impl: ID3D12VideoDevice1_Impl {
    fn CreateVideoDecoder1(&self, pdesc: *const D3D12_VIDEO_DECODER_DESC, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>, riid: *const windows_core::GUID, ppvideodecoder: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateVideoDecoderHeap1(&self, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>, riid: *const windows_core::GUID, ppvideodecoderheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateVideoProcessor1(&self, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, numinputstreamdescs: u32, pinputstreamdescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>, riid: *const windows_core::GUID, ppvideoprocessor: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateVideoExtensionCommand(&self, pdesc: *const D3D12_VIDEO_EXTENSION_COMMAND_DESC, pcreationparameters: *const core::ffi::c_void, creationparametersdatasizeinbytes: usize, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>, riid: *const windows_core::GUID, ppvideoextensioncommand: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ExecuteExtensionCommand(&self, pextensioncommand: windows_core::Ref<ID3D12VideoExtensionCommand>, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize, poutputdata: *mut core::ffi::c_void, outputdatasizeinbytes: usize) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoDevice2_Vtbl {
    pub const fn new<Identity: ID3D12VideoDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVideoDecoder1<Identity: ID3D12VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VIDEO_DECODER_DESC, pprotectedresourcesession: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvideodecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice2_Impl::CreateVideoDecoder1(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pprotectedresourcesession), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideodecoder)).into()
            }
        }
        unsafe extern "system" fn CreateVideoDecoderHeap1<Identity: ID3D12VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC, pprotectedresourcesession: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvideodecoderheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice2_Impl::CreateVideoDecoderHeap1(this, core::mem::transmute_copy(&pvideodecoderheapdesc), core::mem::transmute_copy(&pprotectedresourcesession), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideodecoderheap)).into()
            }
        }
        unsafe extern "system" fn CreateVideoProcessor1<Identity: ID3D12VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, numinputstreamdescs: u32, pinputstreamdescs: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC, pprotectedresourcesession: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvideoprocessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice2_Impl::CreateVideoProcessor1(this, core::mem::transmute_copy(&nodemask), core::mem::transmute_copy(&poutputstreamdesc), core::mem::transmute_copy(&numinputstreamdescs), core::mem::transmute_copy(&pinputstreamdescs), core::mem::transmute_copy(&pprotectedresourcesession), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideoprocessor)).into()
            }
        }
        unsafe extern "system" fn CreateVideoExtensionCommand<Identity: ID3D12VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VIDEO_EXTENSION_COMMAND_DESC, pcreationparameters: *const core::ffi::c_void, creationparametersdatasizeinbytes: usize, pprotectedresourcesession: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvideoextensioncommand: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice2_Impl::CreateVideoExtensionCommand(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pcreationparameters), core::mem::transmute_copy(&creationparametersdatasizeinbytes), core::mem::transmute_copy(&pprotectedresourcesession), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideoextensioncommand)).into()
            }
        }
        unsafe extern "system" fn ExecuteExtensionCommand<Identity: ID3D12VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextensioncommand: *mut core::ffi::c_void, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize, poutputdata: *mut core::ffi::c_void, outputdatasizeinbytes: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice2_Impl::ExecuteExtensionCommand(this, core::mem::transmute_copy(&pextensioncommand), core::mem::transmute_copy(&pexecutionparameters), core::mem::transmute_copy(&executionparameterssizeinbytes), core::mem::transmute_copy(&poutputdata), core::mem::transmute_copy(&outputdatasizeinbytes)).into()
            }
        }
        Self {
            base__: ID3D12VideoDevice1_Vtbl::new::<Identity, OFFSET>(),
            CreateVideoDecoder1: CreateVideoDecoder1::<Identity, OFFSET>,
            CreateVideoDecoderHeap1: CreateVideoDecoderHeap1::<Identity, OFFSET>,
            CreateVideoProcessor1: CreateVideoProcessor1::<Identity, OFFSET>,
            CreateVideoExtensionCommand: CreateVideoExtensionCommand::<Identity, OFFSET>,
            ExecuteExtensionCommand: ExecuteExtensionCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDevice2 as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoDevice2 {}
windows_core::imp::define_interface!(ID3D12VideoDevice3, ID3D12VideoDevice3_Vtbl, 0x4243adb4_3a32_4666_973c_0ccc5625dc44);
impl core::ops::Deref for ID3D12VideoDevice3 {
    type Target = ID3D12VideoDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12VideoDevice3, windows_core::IUnknown, ID3D12VideoDevice, ID3D12VideoDevice1, ID3D12VideoDevice2);
impl ID3D12VideoDevice3 {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CreateVideoEncoder<T>(&self, pdesc: *const D3D12_VIDEO_ENCODER_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoEncoder)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn CreateVideoEncoderHeap<T>(&self, pdesc: *const D3D12_VIDEO_ENCODER_HEAP_DESC) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoEncoderHeap)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDevice3_Vtbl {
    pub base__: ID3D12VideoDevice2_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub CreateVideoEncoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_ENCODER_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CreateVideoEncoder: usize,
    pub CreateVideoEncoderHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_ENCODER_HEAP_DESC, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoDevice3_Impl: ID3D12VideoDevice2_Impl {
    fn CreateVideoEncoder(&self, pdesc: *const D3D12_VIDEO_ENCODER_DESC, riid: *const windows_core::GUID, ppvideoencoder: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateVideoEncoderHeap(&self, pdesc: *const D3D12_VIDEO_ENCODER_HEAP_DESC, riid: *const windows_core::GUID, ppvideoencoderheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoDevice3_Vtbl {
    pub const fn new<Identity: ID3D12VideoDevice3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVideoEncoder<Identity: ID3D12VideoDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VIDEO_ENCODER_DESC, riid: *const windows_core::GUID, ppvideoencoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice3_Impl::CreateVideoEncoder(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideoencoder)).into()
            }
        }
        unsafe extern "system" fn CreateVideoEncoderHeap<Identity: ID3D12VideoDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VIDEO_ENCODER_HEAP_DESC, riid: *const windows_core::GUID, ppvideoencoderheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice3_Impl::CreateVideoEncoderHeap(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideoencoderheap)).into()
            }
        }
        Self {
            base__: ID3D12VideoDevice2_Vtbl::new::<Identity, OFFSET>(),
            CreateVideoEncoder: CreateVideoEncoder::<Identity, OFFSET>,
            CreateVideoEncoderHeap: CreateVideoEncoderHeap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDevice3 as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice1 as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoDevice3 {}
windows_core::imp::define_interface!(ID3D12VideoDevice4, ID3D12VideoDevice4_Vtbl, 0xe59ad09e_f1ae_42bb_8983_9f6e5586c4eb);
impl core::ops::Deref for ID3D12VideoDevice4 {
    type Target = ID3D12VideoDevice3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D12VideoDevice4, windows_core::IUnknown, ID3D12VideoDevice, ID3D12VideoDevice1, ID3D12VideoDevice2, ID3D12VideoDevice3);
impl ID3D12VideoDevice4 {
    pub unsafe fn CreateVideoEncoderHeap1<T>(&self, pdesc: *const D3D12_VIDEO_ENCODER_HEAP_DESC1) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateVideoEncoderHeap1)(windows_core::Interface::as_raw(self), pdesc, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoDevice4_Vtbl {
    pub base__: ID3D12VideoDevice3_Vtbl,
    pub CreateVideoEncoderHeap1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_ENCODER_HEAP_DESC1, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoDevice4_Impl: ID3D12VideoDevice3_Impl {
    fn CreateVideoEncoderHeap1(&self, pdesc: *const D3D12_VIDEO_ENCODER_HEAP_DESC1, riid: *const windows_core::GUID, ppvideoencoderheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoDevice4_Vtbl {
    pub const fn new<Identity: ID3D12VideoDevice4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVideoEncoderHeap1<Identity: ID3D12VideoDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VIDEO_ENCODER_HEAP_DESC1, riid: *const windows_core::GUID, ppvideoencoderheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoDevice4_Impl::CreateVideoEncoderHeap1(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvideoencoderheap)).into()
            }
        }
        Self { base__: ID3D12VideoDevice3_Vtbl::new::<Identity, OFFSET>(), CreateVideoEncoderHeap1: CreateVideoEncoderHeap1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoDevice4 as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice1 as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice2 as windows_core::Interface>::IID || iid == &<ID3D12VideoDevice3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoDevice4 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoEncodeCommandList, ID3D12VideoEncodeCommandList_Vtbl, 0x8455293a_0cbd_4831_9b39_fbdbab724723);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoEncodeCommandList {
    type Target = super::d3d12::ID3D12CommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoEncodeCommandList, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncodeCommandList {
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d12::ID3D12CommandAllocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), pallocator.param().abi()) }
    }
    pub unsafe fn ClearState(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::d3d12::D3D12_RESOURCE_BARRIER]) {
        unsafe {
            (windows_core::Interface::vtable(self).ResourceBarrier)(windows_core::Interface::as_raw(self), pbarriers.len().try_into().unwrap(), core::mem::transmute(pbarriers.as_ptr()));
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: Option<*const super::d3d12::D3D12_DISCARD_REGION>)
    where
        P0: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DiscardResource)(windows_core::Interface::as_raw(self), presource.param().abi(), pregion.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).BeginQuery)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, index);
        }
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).EndQuery)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, index);
        }
    }
    pub unsafe fn ResolveQueryData<P0, P4>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P4, aligneddestinationbufferoffset: u64)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
        P4: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveQueryData)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, startindex, numqueries, pdestinationbuffer.param().abi(), aligneddestinationbufferoffset);
        }
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP)
    where
        P0: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), pbuffer.param().abi(), alignedbufferoffset, operation);
        }
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMarker)(windows_core::Interface::as_raw(self), metadata, pdata.unwrap_or(core::mem::zeroed()) as _, size);
        }
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginEvent)(windows_core::Interface::as_raw(self), metadata, pdata.unwrap_or(core::mem::zeroed()) as _, size);
        }
    }
    pub unsafe fn EndEvent(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn EstimateMotion<P0>(&self, pmotionestimator: P0, poutputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT, pinputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_INPUT)
    where
        P0: windows_core::Param<ID3D12VideoMotionEstimator>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).EstimateMotion)(windows_core::Interface::as_raw(self), pmotionestimator.param().abi(), core::mem::transmute(poutputarguments), core::mem::transmute(pinputarguments));
        }
    }
    pub unsafe fn ResolveMotionVectorHeap(&self, poutputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT, pinputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT) {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveMotionVectorHeap)(windows_core::Interface::as_raw(self), core::mem::transmute(poutputarguments), core::mem::transmute(pinputarguments));
        }
    }
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: Option<*const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBufferImmediate)(windows_core::Interface::as_raw(self), count, pparams, pmodes.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetProtectedResourceSession)(windows_core::Interface::as_raw(self), pprotectedresourcesession.param().abi());
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoEncodeCommandList_Vtbl {
    pub base__: super::d3d12::ID3D12CommandList_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub ResourceBarrier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_RESOURCE_BARRIER),
    #[cfg(feature = "Win32_windef")]
    pub DiscardResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::d3d12::D3D12_DISCARD_REGION),
    #[cfg(not(feature = "Win32_windef"))]
    DiscardResource: usize,
    pub BeginQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32),
    pub EndQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32),
    pub ResolveQueryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32, u32, *mut core::ffi::c_void, u64),
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, super::d3d12::D3D12_PREDICATION_OP),
    pub SetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub BeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EstimateMotion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT, *const D3D12_VIDEO_MOTION_ESTIMATOR_INPUT),
    pub ResolveMotionVectorHeap: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT, *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT),
    pub WriteBufferImmediate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE),
    pub SetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub trait ID3D12VideoEncodeCommandList_Impl: super::d3d12::ID3D12CommandList_Impl {
    fn Close(&self) -> windows_core::Result<()>;
    fn Reset(&self, pallocator: windows_core::Ref<super::d3d12::ID3D12CommandAllocator>) -> windows_core::Result<()>;
    fn ClearState(&self);
    fn ResourceBarrier(&self, numbarriers: u32, pbarriers: *const super::d3d12::D3D12_RESOURCE_BARRIER);
    fn DiscardResource(&self, presource: windows_core::Ref<super::d3d12::ID3D12Resource>, pregion: *const super::d3d12::D3D12_DISCARD_REGION);
    fn BeginQuery(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32);
    fn EndQuery(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32);
    fn ResolveQueryData(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: windows_core::Ref<super::d3d12::ID3D12Resource>, aligneddestinationbufferoffset: u64);
    fn SetPredication(&self, pbuffer: windows_core::Ref<super::d3d12::ID3D12Resource>, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP);
    fn SetMarker(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn BeginEvent(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn EndEvent(&self);
    fn EstimateMotion(&self, pmotionestimator: windows_core::Ref<ID3D12VideoMotionEstimator>, poutputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT, pinputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_INPUT);
    fn ResolveMotionVectorHeap(&self, poutputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT, pinputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT);
    fn WriteBufferImmediate(&self, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE);
    fn SetProtectedResourceSession(&self, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl ID3D12VideoEncodeCommandList_Vtbl {
    pub const fn new<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::Reset(this, core::mem::transmute_copy(&pallocator)).into()
            }
        }
        unsafe extern "system" fn ClearState<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::ClearState(this);
            }
        }
        unsafe extern "system" fn ResourceBarrier<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbarriers: u32, pbarriers: *const super::d3d12::D3D12_RESOURCE_BARRIER) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::ResourceBarrier(this, core::mem::transmute_copy(&numbarriers), core::mem::transmute_copy(&pbarriers));
            }
        }
        unsafe extern "system" fn DiscardResource<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pregion: *const super::d3d12::D3D12_DISCARD_REGION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::DiscardResource(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pregion));
            }
        }
        unsafe extern "system" fn BeginQuery<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::BeginQuery(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&index));
            }
        }
        unsafe extern "system" fn EndQuery<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::EndQuery(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&index));
            }
        }
        unsafe extern "system" fn ResolveQueryData<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: *mut core::ffi::c_void, aligneddestinationbufferoffset: u64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::ResolveQueryData(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&numqueries), core::mem::transmute_copy(&pdestinationbuffer), core::mem::transmute_copy(&aligneddestinationbufferoffset));
            }
        }
        unsafe extern "system" fn SetPredication<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::SetPredication(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&alignedbufferoffset), core::mem::transmute_copy(&operation));
            }
        }
        unsafe extern "system" fn SetMarker<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::SetMarker(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size));
            }
        }
        unsafe extern "system" fn BeginEvent<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::BeginEvent(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size));
            }
        }
        unsafe extern "system" fn EndEvent<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::EndEvent(this);
            }
        }
        unsafe extern "system" fn EstimateMotion<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmotionestimator: *mut core::ffi::c_void, poutputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT, pinputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_INPUT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::EstimateMotion(this, core::mem::transmute_copy(&pmotionestimator), core::mem::transmute_copy(&poutputarguments), core::mem::transmute_copy(&pinputarguments));
            }
        }
        unsafe extern "system" fn ResolveMotionVectorHeap<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT, pinputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::ResolveMotionVectorHeap(this, core::mem::transmute_copy(&poutputarguments), core::mem::transmute_copy(&pinputarguments));
            }
        }
        unsafe extern "system" fn WriteBufferImmediate<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::WriteBufferImmediate(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&pmodes));
            }
        }
        unsafe extern "system" fn SetProtectedResourceSession<Identity: ID3D12VideoEncodeCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotectedresourcesession: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList_Impl::SetProtectedResourceSession(this, core::mem::transmute_copy(&pprotectedresourcesession));
            }
        }
        Self {
            base__: super::d3d12::ID3D12CommandList_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ClearState: ClearState::<Identity, OFFSET>,
            ResourceBarrier: ResourceBarrier::<Identity, OFFSET>,
            DiscardResource: DiscardResource::<Identity, OFFSET>,
            BeginQuery: BeginQuery::<Identity, OFFSET>,
            EndQuery: EndQuery::<Identity, OFFSET>,
            ResolveQueryData: ResolveQueryData::<Identity, OFFSET>,
            SetPredication: SetPredication::<Identity, OFFSET>,
            SetMarker: SetMarker::<Identity, OFFSET>,
            BeginEvent: BeginEvent::<Identity, OFFSET>,
            EndEvent: EndEvent::<Identity, OFFSET>,
            EstimateMotion: EstimateMotion::<Identity, OFFSET>,
            ResolveMotionVectorHeap: ResolveMotionVectorHeap::<Identity, OFFSET>,
            WriteBufferImmediate: WriteBufferImmediate::<Identity, OFFSET>,
            SetProtectedResourceSession: SetProtectedResourceSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoEncodeCommandList as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoEncodeCommandList {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoEncodeCommandList1, ID3D12VideoEncodeCommandList1_Vtbl, 0x94971eca_2bdb_4769_88cf_3675ea757ebc);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoEncodeCommandList1 {
    type Target = ID3D12VideoEncodeCommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoEncodeCommandList1, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoEncodeCommandList);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncodeCommandList1 {
    pub unsafe fn InitializeExtensionCommand<P0>(&self, pextensioncommand: P0, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize)
    where
        P0: windows_core::Param<ID3D12VideoExtensionCommand>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeExtensionCommand)(windows_core::Interface::as_raw(self), pextensioncommand.param().abi(), pinitializationparameters, initializationparameterssizeinbytes);
        }
    }
    pub unsafe fn ExecuteExtensionCommand<P0>(&self, pextensioncommand: P0, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize)
    where
        P0: windows_core::Param<ID3D12VideoExtensionCommand>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ExecuteExtensionCommand)(windows_core::Interface::as_raw(self), pextensioncommand.param().abi(), pexecutionparameters, executionparameterssizeinbytes);
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoEncodeCommandList1_Vtbl {
    pub base__: ID3D12VideoEncodeCommandList_Vtbl,
    pub InitializeExtensionCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize),
    pub ExecuteExtensionCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub trait ID3D12VideoEncodeCommandList1_Impl: ID3D12VideoEncodeCommandList_Impl {
    fn InitializeExtensionCommand(&self, pextensioncommand: windows_core::Ref<ID3D12VideoExtensionCommand>, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize);
    fn ExecuteExtensionCommand(&self, pextensioncommand: windows_core::Ref<ID3D12VideoExtensionCommand>, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl ID3D12VideoEncodeCommandList1_Vtbl {
    pub const fn new<Identity: ID3D12VideoEncodeCommandList1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeExtensionCommand<Identity: ID3D12VideoEncodeCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextensioncommand: *mut core::ffi::c_void, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList1_Impl::InitializeExtensionCommand(this, core::mem::transmute_copy(&pextensioncommand), core::mem::transmute_copy(&pinitializationparameters), core::mem::transmute_copy(&initializationparameterssizeinbytes));
            }
        }
        unsafe extern "system" fn ExecuteExtensionCommand<Identity: ID3D12VideoEncodeCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextensioncommand: *mut core::ffi::c_void, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList1_Impl::ExecuteExtensionCommand(this, core::mem::transmute_copy(&pextensioncommand), core::mem::transmute_copy(&pexecutionparameters), core::mem::transmute_copy(&executionparameterssizeinbytes));
            }
        }
        Self {
            base__: ID3D12VideoEncodeCommandList_Vtbl::new::<Identity, OFFSET>(),
            InitializeExtensionCommand: InitializeExtensionCommand::<Identity, OFFSET>,
            ExecuteExtensionCommand: ExecuteExtensionCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoEncodeCommandList1 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoEncodeCommandList1 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoEncodeCommandList2, ID3D12VideoEncodeCommandList2_Vtbl, 0x895491e2_e701_46a9_9a1f_8d3480ed867a);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoEncodeCommandList2 {
    type Target = ID3D12VideoEncodeCommandList1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoEncodeCommandList2, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoEncodeCommandList, ID3D12VideoEncodeCommandList1);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncodeCommandList2 {
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn EncodeFrame<P0, P1>(&self, pencoder: P0, pheap: P1, pinputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS)
    where
        P0: windows_core::Param<ID3D12VideoEncoder>,
        P1: windows_core::Param<ID3D12VideoEncoderHeap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).EncodeFrame)(windows_core::Interface::as_raw(self), pencoder.param().abi(), pheap.param().abi(), core::mem::transmute(pinputarguments), core::mem::transmute(poutputarguments));
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn ResolveEncoderOutputMetadata(&self, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS) {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveEncoderOutputMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputarguments), core::mem::transmute(poutputarguments));
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoEncodeCommandList2_Vtbl {
    pub base__: ID3D12VideoEncodeCommandList1_Vtbl,
    #[cfg(feature = "Win32_dxgicommon")]
    pub EncodeFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS, *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS),
    #[cfg(not(feature = "Win32_dxgicommon"))]
    EncodeFrame: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub ResolveEncoderOutputMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS, *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    ResolveEncoderOutputMetadata: usize,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
pub trait ID3D12VideoEncodeCommandList2_Impl: ID3D12VideoEncodeCommandList1_Impl {
    fn EncodeFrame(&self, pencoder: windows_core::Ref<ID3D12VideoEncoder>, pheap: windows_core::Ref<ID3D12VideoEncoderHeap>, pinputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS);
    fn ResolveEncoderOutputMetadata(&self, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl ID3D12VideoEncodeCommandList2_Vtbl {
    pub const fn new<Identity: ID3D12VideoEncodeCommandList2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EncodeFrame<Identity: ID3D12VideoEncodeCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pencoder: *mut core::ffi::c_void, pheap: *mut core::ffi::c_void, pinputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList2_Impl::EncodeFrame(this, core::mem::transmute_copy(&pencoder), core::mem::transmute_copy(&pheap), core::mem::transmute_copy(&pinputarguments), core::mem::transmute_copy(&poutputarguments));
            }
        }
        unsafe extern "system" fn ResolveEncoderOutputMetadata<Identity: ID3D12VideoEncodeCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList2_Impl::ResolveEncoderOutputMetadata(this, core::mem::transmute_copy(&pinputarguments), core::mem::transmute_copy(&poutputarguments));
            }
        }
        Self {
            base__: ID3D12VideoEncodeCommandList1_Vtbl::new::<Identity, OFFSET>(),
            EncodeFrame: EncodeFrame::<Identity, OFFSET>,
            ResolveEncoderOutputMetadata: ResolveEncoderOutputMetadata::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoEncodeCommandList2 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoEncodeCommandList2 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoEncodeCommandList3, ID3D12VideoEncodeCommandList3_Vtbl, 0x7f027b22_1515_4e85_aa0d_026486580576);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoEncodeCommandList3 {
    type Target = ID3D12VideoEncodeCommandList2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoEncodeCommandList3, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoEncodeCommandList, ID3D12VideoEncodeCommandList1, ID3D12VideoEncodeCommandList2);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncodeCommandList3 {
    pub unsafe fn Barrier(&self, pbarriergroups: &[super::d3d12::D3D12_BARRIER_GROUP]) {
        unsafe {
            (windows_core::Interface::vtable(self).Barrier)(windows_core::Interface::as_raw(self), pbarriergroups.len().try_into().unwrap(), core::mem::transmute(pbarriergroups.as_ptr()));
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoEncodeCommandList3_Vtbl {
    pub base__: ID3D12VideoEncodeCommandList2_Vtbl,
    pub Barrier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_BARRIER_GROUP),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
pub trait ID3D12VideoEncodeCommandList3_Impl: ID3D12VideoEncodeCommandList2_Impl {
    fn Barrier(&self, numbarriergroups: u32, pbarriergroups: *const super::d3d12::D3D12_BARRIER_GROUP);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl ID3D12VideoEncodeCommandList3_Vtbl {
    pub const fn new<Identity: ID3D12VideoEncodeCommandList3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Barrier<Identity: ID3D12VideoEncodeCommandList3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbarriergroups: u32, pbarriergroups: *const super::d3d12::D3D12_BARRIER_GROUP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList3_Impl::Barrier(this, core::mem::transmute_copy(&numbarriergroups), core::mem::transmute_copy(&pbarriergroups));
            }
        }
        Self { base__: ID3D12VideoEncodeCommandList2_Vtbl::new::<Identity, OFFSET>(), Barrier: Barrier::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoEncodeCommandList3 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoEncodeCommandList3 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoEncodeCommandList4, ID3D12VideoEncodeCommandList4_Vtbl, 0x69aeb5b7_55f2_4012_8b73_3a88d65a204c);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoEncodeCommandList4 {
    type Target = ID3D12VideoEncodeCommandList3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoEncodeCommandList4, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoEncodeCommandList, ID3D12VideoEncodeCommandList1, ID3D12VideoEncodeCommandList2, ID3D12VideoEncodeCommandList3);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncodeCommandList4 {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_windef"))]
    pub unsafe fn EncodeFrame1<P0, P1>(&self, pencoder: P0, pheap: P1, pinputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1, poutputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1)
    where
        P0: windows_core::Param<ID3D12VideoEncoder>,
        P1: windows_core::Param<ID3D12VideoEncoderHeap1>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).EncodeFrame1)(windows_core::Interface::as_raw(self), pencoder.param().abi(), pheap.param().abi(), core::mem::transmute(pinputarguments), core::mem::transmute(poutputarguments));
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn ResolveEncoderOutputMetadata1(&self, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS1) {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveEncoderOutputMetadata1)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputarguments), core::mem::transmute(poutputarguments));
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn ResolveInputParamLayout(&self, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_OUTPUT_ARGUMENTS) {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveInputParamLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(pinputarguments), core::mem::transmute(poutputarguments));
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoEncodeCommandList4_Vtbl {
    pub base__: ID3D12VideoEncodeCommandList3_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_windef"))]
    pub EncodeFrame1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1, *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1),
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_windef")))]
    EncodeFrame1: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub ResolveEncoderOutputMetadata1: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1, *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS1),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    ResolveEncoderOutputMetadata1: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub ResolveInputParamLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS, *const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_OUTPUT_ARGUMENTS),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    ResolveInputParamLayout: usize,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
pub trait ID3D12VideoEncodeCommandList4_Impl: ID3D12VideoEncodeCommandList3_Impl {
    fn EncodeFrame1(&self, pencoder: windows_core::Ref<ID3D12VideoEncoder>, pheap: windows_core::Ref<ID3D12VideoEncoderHeap1>, pinputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1, poutputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1);
    fn ResolveEncoderOutputMetadata1(&self, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS1);
    fn ResolveInputParamLayout(&self, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_OUTPUT_ARGUMENTS);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl ID3D12VideoEncodeCommandList4_Vtbl {
    pub const fn new<Identity: ID3D12VideoEncodeCommandList4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EncodeFrame1<Identity: ID3D12VideoEncodeCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pencoder: *mut core::ffi::c_void, pheap: *mut core::ffi::c_void, pinputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1, poutputarguments: *const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList4_Impl::EncodeFrame1(this, core::mem::transmute_copy(&pencoder), core::mem::transmute_copy(&pheap), core::mem::transmute_copy(&pinputarguments), core::mem::transmute_copy(&poutputarguments));
            }
        }
        unsafe extern "system" fn ResolveEncoderOutputMetadata1<Identity: ID3D12VideoEncodeCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList4_Impl::ResolveEncoderOutputMetadata1(this, core::mem::transmute_copy(&pinputarguments), core::mem::transmute_copy(&poutputarguments));
            }
        }
        unsafe extern "system" fn ResolveInputParamLayout<Identity: ID3D12VideoEncodeCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS, poutputarguments: *const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_OUTPUT_ARGUMENTS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncodeCommandList4_Impl::ResolveInputParamLayout(this, core::mem::transmute_copy(&pinputarguments), core::mem::transmute_copy(&poutputarguments));
            }
        }
        Self {
            base__: ID3D12VideoEncodeCommandList3_Vtbl::new::<Identity, OFFSET>(),
            EncodeFrame1: EncodeFrame1::<Identity, OFFSET>,
            ResolveEncoderOutputMetadata1: ResolveEncoderOutputMetadata1::<Identity, OFFSET>,
            ResolveInputParamLayout: ResolveInputParamLayout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoEncodeCommandList4 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList2 as windows_core::Interface>::IID || iid == &<ID3D12VideoEncodeCommandList3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoEncodeCommandList4 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoEncoder, ID3D12VideoEncoder_Vtbl, 0x2e0d212d_8df9_44a6_a770_bb289b182737);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoEncoder {
    type Target = super::d3d12::ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoEncoder, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncoder {
    pub unsafe fn GetNodeMask(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNodeMask)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetEncoderFlags(&self) -> D3D12_VIDEO_ENCODER_FLAGS {
        unsafe { (windows_core::Interface::vtable(self).GetEncoderFlags)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCodec(&self) -> D3D12_VIDEO_ENCODER_CODEC {
        unsafe { (windows_core::Interface::vtable(self).GetCodec)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCodecProfile(&self, dstprofile: D3D12_VIDEO_ENCODER_PROFILE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodecProfile)(windows_core::Interface::as_raw(self), core::mem::transmute(dstprofile)) }
    }
    pub unsafe fn GetCodecConfiguration(&self, dstcodecconfig: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodecConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute(dstcodecconfig)) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetInputFormat(&self) -> super::dxgiformat::DXGI_FORMAT {
        unsafe { (windows_core::Interface::vtable(self).GetInputFormat)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMaxMotionEstimationPrecision(&self) -> D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetMaxMotionEstimationPrecision)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoEncoder_Vtbl {
    pub base__: super::d3d12::ID3D12Pageable_Vtbl,
    pub GetNodeMask: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetEncoderFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_FLAGS,
    pub GetCodec: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_CODEC,
    pub GetCodecProfile: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_VIDEO_ENCODER_PROFILE_DESC) -> windows_core::HRESULT,
    pub GetCodecConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetInputFormat: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dxgiformat::DXGI_FORMAT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetInputFormat: usize,
    pub GetMaxMotionEstimationPrecision: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoEncoder_Impl: super::d3d12::ID3D12Pageable_Impl {
    fn GetNodeMask(&self) -> u32;
    fn GetEncoderFlags(&self) -> D3D12_VIDEO_ENCODER_FLAGS;
    fn GetCodec(&self) -> D3D12_VIDEO_ENCODER_CODEC;
    fn GetCodecProfile(&self, dstprofile: D3D12_VIDEO_ENCODER_PROFILE_DESC) -> windows_core::Result<()>;
    fn GetCodecConfiguration(&self, dstcodecconfig: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION) -> windows_core::Result<()>;
    fn GetInputFormat(&self) -> super::dxgiformat::DXGI_FORMAT;
    fn GetMaxMotionEstimationPrecision(&self) -> D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl ID3D12VideoEncoder_Vtbl {
    pub const fn new<Identity: ID3D12VideoEncoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNodeMask<Identity: ID3D12VideoEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoder_Impl::GetNodeMask(this)
            }
        }
        unsafe extern "system" fn GetEncoderFlags<Identity: ID3D12VideoEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_FLAGS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoder_Impl::GetEncoderFlags(this)
            }
        }
        unsafe extern "system" fn GetCodec<Identity: ID3D12VideoEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_CODEC {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoder_Impl::GetCodec(this)
            }
        }
        unsafe extern "system" fn GetCodecProfile<Identity: ID3D12VideoEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dstprofile: D3D12_VIDEO_ENCODER_PROFILE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoder_Impl::GetCodecProfile(this, core::mem::transmute_copy(&dstprofile)).into()
            }
        }
        unsafe extern "system" fn GetCodecConfiguration<Identity: ID3D12VideoEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dstcodecconfig: D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoder_Impl::GetCodecConfiguration(this, core::mem::transmute_copy(&dstcodecconfig)).into()
            }
        }
        unsafe extern "system" fn GetInputFormat<Identity: ID3D12VideoEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dxgiformat::DXGI_FORMAT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoder_Impl::GetInputFormat(this)
            }
        }
        unsafe extern "system" fn GetMaxMotionEstimationPrecision<Identity: ID3D12VideoEncoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoder_Impl::GetMaxMotionEstimationPrecision(this)
            }
        }
        Self {
            base__: super::d3d12::ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(),
            GetNodeMask: GetNodeMask::<Identity, OFFSET>,
            GetEncoderFlags: GetEncoderFlags::<Identity, OFFSET>,
            GetCodec: GetCodec::<Identity, OFFSET>,
            GetCodecProfile: GetCodecProfile::<Identity, OFFSET>,
            GetCodecConfiguration: GetCodecConfiguration::<Identity, OFFSET>,
            GetInputFormat: GetInputFormat::<Identity, OFFSET>,
            GetMaxMotionEstimationPrecision: GetMaxMotionEstimationPrecision::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoEncoder as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoEncoder {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoEncoderHeap, ID3D12VideoEncoderHeap_Vtbl, 0x22b35d96_876a_44c0_b25e_fb8c9c7f1c4a);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoEncoderHeap {
    type Target = super::d3d12::ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoEncoderHeap, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncoderHeap {
    pub unsafe fn GetNodeMask(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNodeMask)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetEncoderHeapFlags(&self) -> D3D12_VIDEO_ENCODER_HEAP_FLAGS {
        unsafe { (windows_core::Interface::vtable(self).GetEncoderHeapFlags)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCodec(&self) -> D3D12_VIDEO_ENCODER_CODEC {
        unsafe { (windows_core::Interface::vtable(self).GetCodec)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCodecProfile(&self, dstprofile: D3D12_VIDEO_ENCODER_PROFILE_DESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodecProfile)(windows_core::Interface::as_raw(self), core::mem::transmute(dstprofile)) }
    }
    pub unsafe fn GetCodecLevel(&self, dstlevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCodecLevel)(windows_core::Interface::as_raw(self), core::mem::transmute(dstlevel)) }
    }
    pub unsafe fn GetResolutionListCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetResolutionListCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetResolutionList(&self, presolutionlist: &mut [D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetResolutionList)(windows_core::Interface::as_raw(self), presolutionlist.len().try_into().unwrap(), core::mem::transmute(presolutionlist.as_ptr())) }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoEncoderHeap_Vtbl {
    pub base__: super::d3d12::ID3D12Pageable_Vtbl,
    pub GetNodeMask: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetEncoderHeapFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_HEAP_FLAGS,
    pub GetCodec: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_CODEC,
    pub GetCodecProfile: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_VIDEO_ENCODER_PROFILE_DESC) -> windows_core::HRESULT,
    pub GetCodecLevel: unsafe extern "system" fn(*mut core::ffi::c_void, D3D12_VIDEO_ENCODER_LEVEL_SETTING) -> windows_core::HRESULT,
    pub GetResolutionListCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetResolutionList: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3d12")]
pub trait ID3D12VideoEncoderHeap_Impl: super::d3d12::ID3D12Pageable_Impl {
    fn GetNodeMask(&self) -> u32;
    fn GetEncoderHeapFlags(&self) -> D3D12_VIDEO_ENCODER_HEAP_FLAGS;
    fn GetCodec(&self) -> D3D12_VIDEO_ENCODER_CODEC;
    fn GetCodecProfile(&self, dstprofile: D3D12_VIDEO_ENCODER_PROFILE_DESC) -> windows_core::Result<()>;
    fn GetCodecLevel(&self, dstlevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING) -> windows_core::Result<()>;
    fn GetResolutionListCount(&self) -> u32;
    fn GetResolutionList(&self, resolutionslistcount: u32, presolutionlist: *mut D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncoderHeap_Vtbl {
    pub const fn new<Identity: ID3D12VideoEncoderHeap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNodeMask<Identity: ID3D12VideoEncoderHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoderHeap_Impl::GetNodeMask(this)
            }
        }
        unsafe extern "system" fn GetEncoderHeapFlags<Identity: ID3D12VideoEncoderHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_HEAP_FLAGS {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoderHeap_Impl::GetEncoderHeapFlags(this)
            }
        }
        unsafe extern "system" fn GetCodec<Identity: ID3D12VideoEncoderHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_VIDEO_ENCODER_CODEC {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoderHeap_Impl::GetCodec(this)
            }
        }
        unsafe extern "system" fn GetCodecProfile<Identity: ID3D12VideoEncoderHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dstprofile: D3D12_VIDEO_ENCODER_PROFILE_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoderHeap_Impl::GetCodecProfile(this, core::mem::transmute_copy(&dstprofile)).into()
            }
        }
        unsafe extern "system" fn GetCodecLevel<Identity: ID3D12VideoEncoderHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dstlevel: D3D12_VIDEO_ENCODER_LEVEL_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoderHeap_Impl::GetCodecLevel(this, core::mem::transmute_copy(&dstlevel)).into()
            }
        }
        unsafe extern "system" fn GetResolutionListCount<Identity: ID3D12VideoEncoderHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoderHeap_Impl::GetResolutionListCount(this)
            }
        }
        unsafe extern "system" fn GetResolutionList<Identity: ID3D12VideoEncoderHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolutionslistcount: u32, presolutionlist: *mut D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoderHeap_Impl::GetResolutionList(this, core::mem::transmute_copy(&resolutionslistcount), core::mem::transmute_copy(&presolutionlist)).into()
            }
        }
        Self {
            base__: super::d3d12::ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(),
            GetNodeMask: GetNodeMask::<Identity, OFFSET>,
            GetEncoderHeapFlags: GetEncoderHeapFlags::<Identity, OFFSET>,
            GetCodec: GetCodec::<Identity, OFFSET>,
            GetCodecProfile: GetCodecProfile::<Identity, OFFSET>,
            GetCodecLevel: GetCodecLevel::<Identity, OFFSET>,
            GetResolutionListCount: GetResolutionListCount::<Identity, OFFSET>,
            GetResolutionList: GetResolutionList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoEncoderHeap as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d12")]
impl windows_core::RuntimeName for ID3D12VideoEncoderHeap {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoEncoderHeap1, ID3D12VideoEncoderHeap1_Vtbl, 0xea8f1968_4aa0_43a4_9d30_ba86ec84d4f9);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoEncoderHeap1 {
    type Target = ID3D12VideoEncoderHeap;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoEncoderHeap1, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable, ID3D12VideoEncoderHeap);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncoderHeap1 {
    pub unsafe fn GetPow2DownscaleFactor(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPow2DownscaleFactor)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoEncoderHeap1_Vtbl {
    pub base__: ID3D12VideoEncoderHeap_Vtbl,
    pub GetPow2DownscaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
#[cfg(feature = "Win32_d3d12")]
pub trait ID3D12VideoEncoderHeap1_Impl: ID3D12VideoEncoderHeap_Impl {
    fn GetPow2DownscaleFactor(&self) -> u32;
}
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoEncoderHeap1_Vtbl {
    pub const fn new<Identity: ID3D12VideoEncoderHeap1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPow2DownscaleFactor<Identity: ID3D12VideoEncoderHeap1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoEncoderHeap1_Impl::GetPow2DownscaleFactor(this)
            }
        }
        Self { base__: ID3D12VideoEncoderHeap_Vtbl::new::<Identity, OFFSET>(), GetPow2DownscaleFactor: GetPow2DownscaleFactor::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoEncoderHeap1 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12VideoEncoderHeap as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d12")]
impl windows_core::RuntimeName for ID3D12VideoEncoderHeap1 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoExtensionCommand, ID3D12VideoExtensionCommand_Vtbl, 0x554e41e8_ae8e_4a8c_b7d2_5b4f274a30e4);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoExtensionCommand {
    type Target = super::d3d12::ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoExtensionCommand, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoExtensionCommand {
    pub unsafe fn GetDesc(&self) -> D3D12_VIDEO_EXTENSION_COMMAND_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProtectedResourceSession)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok() }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoExtensionCommand_Vtbl {
    pub base__: super::d3d12::ID3D12Pageable_Vtbl,
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_VIDEO_EXTENSION_COMMAND_DESC),
    pub GetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d3d12")]
pub trait ID3D12VideoExtensionCommand_Impl: super::d3d12::ID3D12Pageable_Impl {
    fn GetDesc(&self) -> D3D12_VIDEO_EXTENSION_COMMAND_DESC;
    fn GetProtectedResourceSession(&self, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoExtensionCommand_Vtbl {
    pub const fn new<Identity: ID3D12VideoExtensionCommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12VideoExtensionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_VIDEO_EXTENSION_COMMAND_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID3D12VideoExtensionCommand_Impl::GetDesc(this);
            }
        }
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ID3D12VideoExtensionCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoExtensionCommand_Impl::GetProtectedResourceSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppprotectedsession)).into()
            }
        }
        Self {
            base__: super::d3d12::ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetProtectedResourceSession: GetProtectedResourceSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoExtensionCommand as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d3d12")]
impl windows_core::RuntimeName for ID3D12VideoExtensionCommand {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoMotionEstimator, ID3D12VideoMotionEstimator_Vtbl, 0x33fdae0e_098b_428f_87bb_34b695de08f8);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoMotionEstimator {
    type Target = super::d3d12::ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoMotionEstimator, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoMotionEstimator {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self) -> D3D12_VIDEO_MOTION_ESTIMATOR_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProtectedResourceSession)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok() }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoMotionEstimator_Vtbl {
    pub base__: super::d3d12::ID3D12Pageable_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_VIDEO_MOTION_ESTIMATOR_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
    pub GetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoMotionEstimator_Impl: super::d3d12::ID3D12Pageable_Impl {
    fn GetDesc(&self) -> D3D12_VIDEO_MOTION_ESTIMATOR_DESC;
    fn GetProtectedResourceSession(&self, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl ID3D12VideoMotionEstimator_Vtbl {
    pub const fn new<Identity: ID3D12VideoMotionEstimator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12VideoMotionEstimator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_VIDEO_MOTION_ESTIMATOR_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID3D12VideoMotionEstimator_Impl::GetDesc(this);
            }
        }
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ID3D12VideoMotionEstimator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoMotionEstimator_Impl::GetProtectedResourceSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppprotectedsession)).into()
            }
        }
        Self {
            base__: super::d3d12::ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetProtectedResourceSession: GetProtectedResourceSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoMotionEstimator as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoMotionEstimator {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoMotionVectorHeap, ID3D12VideoMotionVectorHeap_Vtbl, 0x5be17987_743a_4061_834b_23d22daea505);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoMotionVectorHeap {
    type Target = super::d3d12::ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoMotionVectorHeap, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoMotionVectorHeap {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn GetDesc(&self) -> D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProtectedResourceSession)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok() }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoMotionVectorHeap_Vtbl {
    pub base__: super::d3d12::ID3D12Pageable_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC),
    #[cfg(not(feature = "Win32_dxgiformat"))]
    GetDesc: usize,
    pub GetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoMotionVectorHeap_Impl: super::d3d12::ID3D12Pageable_Impl {
    fn GetDesc(&self) -> D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC;
    fn GetProtectedResourceSession(&self, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl ID3D12VideoMotionVectorHeap_Vtbl {
    pub const fn new<Identity: ID3D12VideoMotionVectorHeap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesc<Identity: ID3D12VideoMotionVectorHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID3D12VideoMotionVectorHeap_Impl::GetDesc(this);
            }
        }
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ID3D12VideoMotionVectorHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoMotionVectorHeap_Impl::GetProtectedResourceSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppprotectedsession)).into()
            }
        }
        Self {
            base__: super::d3d12::ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetProtectedResourceSession: GetProtectedResourceSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoMotionVectorHeap as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoMotionVectorHeap {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoProcessCommandList, ID3D12VideoProcessCommandList_Vtbl, 0xaeb2543a_167f_4682_acc8_d159ed4a6209);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoProcessCommandList {
    type Target = super::d3d12::ID3D12CommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoProcessCommandList, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoProcessCommandList {
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d12::ID3D12CommandAllocator>,
    {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self), pallocator.param().abi()) }
    }
    pub unsafe fn ClearState(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::d3d12::D3D12_RESOURCE_BARRIER]) {
        unsafe {
            (windows_core::Interface::vtable(self).ResourceBarrier)(windows_core::Interface::as_raw(self), pbarriers.len().try_into().unwrap(), core::mem::transmute(pbarriers.as_ptr()));
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: Option<*const super::d3d12::D3D12_DISCARD_REGION>)
    where
        P0: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DiscardResource)(windows_core::Interface::as_raw(self), presource.param().abi(), pregion.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).BeginQuery)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, index);
        }
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).EndQuery)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, index);
        }
    }
    pub unsafe fn ResolveQueryData<P0, P4>(&self, pqueryheap: P0, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P4, aligneddestinationbufferoffset: u64)
    where
        P0: windows_core::Param<super::d3d12::ID3D12QueryHeap>,
        P4: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveQueryData)(windows_core::Interface::as_raw(self), pqueryheap.param().abi(), r#type, startindex, numqueries, pdestinationbuffer.param().abi(), aligneddestinationbufferoffset);
        }
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP)
    where
        P0: windows_core::Param<super::d3d12::ID3D12Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPredication)(windows_core::Interface::as_raw(self), pbuffer.param().abi(), alignedbufferoffset, operation);
        }
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMarker)(windows_core::Interface::as_raw(self), metadata, pdata.unwrap_or(core::mem::zeroed()) as _, size);
        }
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: Option<*const core::ffi::c_void>, size: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginEvent)(windows_core::Interface::as_raw(self), metadata, pdata.unwrap_or(core::mem::zeroed()) as _, size);
        }
    }
    pub unsafe fn EndEvent(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn ProcessFrames<P0>(&self, pvideoprocessor: P0, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, pinputarguments: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS])
    where
        P0: windows_core::Param<ID3D12VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessFrames)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), core::mem::transmute(poutputarguments), pinputarguments.len().try_into().unwrap(), core::mem::transmute(pinputarguments.as_ptr()));
        }
    }
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: Option<*const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        unsafe {
            (windows_core::Interface::vtable(self).WriteBufferImmediate)(windows_core::Interface::as_raw(self), count, pparams, pmodes.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoProcessCommandList_Vtbl {
    pub base__: super::d3d12::ID3D12CommandList_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub ResourceBarrier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_RESOURCE_BARRIER),
    #[cfg(feature = "Win32_windef")]
    pub DiscardResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::d3d12::D3D12_DISCARD_REGION),
    #[cfg(not(feature = "Win32_windef"))]
    DiscardResource: usize,
    pub BeginQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32),
    pub EndQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32),
    pub ResolveQueryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d3d12::D3D12_QUERY_TYPE, u32, u32, *mut core::ffi::c_void, u64),
    pub SetPredication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, super::d3d12::D3D12_PREDICATION_OP),
    pub SetMarker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub BeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32),
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_windef")]
    pub ProcessFrames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, u32, *const D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS),
    #[cfg(not(feature = "Win32_windef"))]
    ProcessFrames: usize,
    pub WriteBufferImmediate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub trait ID3D12VideoProcessCommandList_Impl: super::d3d12::ID3D12CommandList_Impl {
    fn Close(&self) -> windows_core::Result<()>;
    fn Reset(&self, pallocator: windows_core::Ref<super::d3d12::ID3D12CommandAllocator>) -> windows_core::Result<()>;
    fn ClearState(&self);
    fn ResourceBarrier(&self, numbarriers: u32, pbarriers: *const super::d3d12::D3D12_RESOURCE_BARRIER);
    fn DiscardResource(&self, presource: windows_core::Ref<super::d3d12::ID3D12Resource>, pregion: *const super::d3d12::D3D12_DISCARD_REGION);
    fn BeginQuery(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32);
    fn EndQuery(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32);
    fn ResolveQueryData(&self, pqueryheap: windows_core::Ref<super::d3d12::ID3D12QueryHeap>, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: windows_core::Ref<super::d3d12::ID3D12Resource>, aligneddestinationbufferoffset: u64);
    fn SetPredication(&self, pbuffer: windows_core::Ref<super::d3d12::ID3D12Resource>, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP);
    fn SetMarker(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn BeginEvent(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn EndEvent(&self);
    fn ProcessFrames(&self, pvideoprocessor: windows_core::Ref<ID3D12VideoProcessor>, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, numinputstreams: u32, pinputarguments: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS);
    fn WriteBufferImmediate(&self, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl ID3D12VideoProcessCommandList_Vtbl {
    pub const fn new<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::Reset(this, core::mem::transmute_copy(&pallocator)).into()
            }
        }
        unsafe extern "system" fn ClearState<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::ClearState(this);
            }
        }
        unsafe extern "system" fn ResourceBarrier<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbarriers: u32, pbarriers: *const super::d3d12::D3D12_RESOURCE_BARRIER) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::ResourceBarrier(this, core::mem::transmute_copy(&numbarriers), core::mem::transmute_copy(&pbarriers));
            }
        }
        unsafe extern "system" fn DiscardResource<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pregion: *const super::d3d12::D3D12_DISCARD_REGION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::DiscardResource(this, core::mem::transmute_copy(&presource), core::mem::transmute_copy(&pregion));
            }
        }
        unsafe extern "system" fn BeginQuery<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::BeginQuery(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&index));
            }
        }
        unsafe extern "system" fn EndQuery<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, index: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::EndQuery(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&index));
            }
        }
        unsafe extern "system" fn ResolveQueryData<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: super::d3d12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: *mut core::ffi::c_void, aligneddestinationbufferoffset: u64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::ResolveQueryData(this, core::mem::transmute_copy(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&numqueries), core::mem::transmute_copy(&pdestinationbuffer), core::mem::transmute_copy(&aligneddestinationbufferoffset));
            }
        }
        unsafe extern "system" fn SetPredication<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void, alignedbufferoffset: u64, operation: super::d3d12::D3D12_PREDICATION_OP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::SetPredication(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&alignedbufferoffset), core::mem::transmute_copy(&operation));
            }
        }
        unsafe extern "system" fn SetMarker<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::SetMarker(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size));
            }
        }
        unsafe extern "system" fn BeginEvent<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::BeginEvent(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size));
            }
        }
        unsafe extern "system" fn EndEvent<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::EndEvent(this);
            }
        }
        unsafe extern "system" fn ProcessFrames<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, numinputstreams: u32, pinputarguments: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::ProcessFrames(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&poutputarguments), core::mem::transmute_copy(&numinputstreams), core::mem::transmute_copy(&pinputarguments));
            }
        }
        unsafe extern "system" fn WriteBufferImmediate<Identity: ID3D12VideoProcessCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, pparams: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const super::d3d12::D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList_Impl::WriteBufferImmediate(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&pmodes));
            }
        }
        Self {
            base__: super::d3d12::ID3D12CommandList_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ClearState: ClearState::<Identity, OFFSET>,
            ResourceBarrier: ResourceBarrier::<Identity, OFFSET>,
            DiscardResource: DiscardResource::<Identity, OFFSET>,
            BeginQuery: BeginQuery::<Identity, OFFSET>,
            EndQuery: EndQuery::<Identity, OFFSET>,
            ResolveQueryData: ResolveQueryData::<Identity, OFFSET>,
            SetPredication: SetPredication::<Identity, OFFSET>,
            SetMarker: SetMarker::<Identity, OFFSET>,
            BeginEvent: BeginEvent::<Identity, OFFSET>,
            EndEvent: EndEvent::<Identity, OFFSET>,
            ProcessFrames: ProcessFrames::<Identity, OFFSET>,
            WriteBufferImmediate: WriteBufferImmediate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoProcessCommandList as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoProcessCommandList {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoProcessCommandList1, ID3D12VideoProcessCommandList1_Vtbl, 0x542c5c4d_7596_434f_8c93_4efa6766f267);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoProcessCommandList1 {
    type Target = ID3D12VideoProcessCommandList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoProcessCommandList1, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoProcessCommandList);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoProcessCommandList1 {
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn ProcessFrames1<P0>(&self, pvideoprocessor: P0, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, pinputarguments: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1])
    where
        P0: windows_core::Param<ID3D12VideoProcessor>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ProcessFrames1)(windows_core::Interface::as_raw(self), pvideoprocessor.param().abi(), core::mem::transmute(poutputarguments), pinputarguments.len().try_into().unwrap(), core::mem::transmute(pinputarguments.as_ptr()));
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoProcessCommandList1_Vtbl {
    pub base__: ID3D12VideoProcessCommandList_Vtbl,
    #[cfg(feature = "Win32_windef")]
    pub ProcessFrames1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, u32, *const D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1),
    #[cfg(not(feature = "Win32_windef"))]
    ProcessFrames1: usize,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub trait ID3D12VideoProcessCommandList1_Impl: ID3D12VideoProcessCommandList_Impl {
    fn ProcessFrames1(&self, pvideoprocessor: windows_core::Ref<ID3D12VideoProcessor>, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, numinputstreams: u32, pinputarguments: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl ID3D12VideoProcessCommandList1_Vtbl {
    pub const fn new<Identity: ID3D12VideoProcessCommandList1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessFrames1<Identity: ID3D12VideoProcessCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, numinputstreams: u32, pinputarguments: *const D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList1_Impl::ProcessFrames1(this, core::mem::transmute_copy(&pvideoprocessor), core::mem::transmute_copy(&poutputarguments), core::mem::transmute_copy(&numinputstreams), core::mem::transmute_copy(&pinputarguments));
            }
        }
        Self { base__: ID3D12VideoProcessCommandList_Vtbl::new::<Identity, OFFSET>(), ProcessFrames1: ProcessFrames1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoProcessCommandList1 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoProcessCommandList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoProcessCommandList1 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoProcessCommandList2, ID3D12VideoProcessCommandList2_Vtbl, 0xdb525ae4_6ad6_473c_baa7_59b2e37082e4);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoProcessCommandList2 {
    type Target = ID3D12VideoProcessCommandList1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoProcessCommandList2, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoProcessCommandList, ID3D12VideoProcessCommandList1);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoProcessCommandList2 {
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: windows_core::Param<super::d3d12::ID3D12ProtectedResourceSession>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetProtectedResourceSession)(windows_core::Interface::as_raw(self), pprotectedresourcesession.param().abi());
        }
    }
    pub unsafe fn InitializeExtensionCommand<P0>(&self, pextensioncommand: P0, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize)
    where
        P0: windows_core::Param<ID3D12VideoExtensionCommand>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InitializeExtensionCommand)(windows_core::Interface::as_raw(self), pextensioncommand.param().abi(), pinitializationparameters, initializationparameterssizeinbytes);
        }
    }
    pub unsafe fn ExecuteExtensionCommand<P0>(&self, pextensioncommand: P0, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize)
    where
        P0: windows_core::Param<ID3D12VideoExtensionCommand>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ExecuteExtensionCommand)(windows_core::Interface::as_raw(self), pextensioncommand.param().abi(), pexecutionparameters, executionparameterssizeinbytes);
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoProcessCommandList2_Vtbl {
    pub base__: ID3D12VideoProcessCommandList1_Vtbl,
    pub SetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub InitializeExtensionCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize),
    pub ExecuteExtensionCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, usize),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub trait ID3D12VideoProcessCommandList2_Impl: ID3D12VideoProcessCommandList1_Impl {
    fn SetProtectedResourceSession(&self, pprotectedresourcesession: windows_core::Ref<super::d3d12::ID3D12ProtectedResourceSession>);
    fn InitializeExtensionCommand(&self, pextensioncommand: windows_core::Ref<ID3D12VideoExtensionCommand>, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize);
    fn ExecuteExtensionCommand(&self, pextensioncommand: windows_core::Ref<ID3D12VideoExtensionCommand>, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl ID3D12VideoProcessCommandList2_Vtbl {
    pub const fn new<Identity: ID3D12VideoProcessCommandList2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProtectedResourceSession<Identity: ID3D12VideoProcessCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotectedresourcesession: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList2_Impl::SetProtectedResourceSession(this, core::mem::transmute_copy(&pprotectedresourcesession));
            }
        }
        unsafe extern "system" fn InitializeExtensionCommand<Identity: ID3D12VideoProcessCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextensioncommand: *mut core::ffi::c_void, pinitializationparameters: *const core::ffi::c_void, initializationparameterssizeinbytes: usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList2_Impl::InitializeExtensionCommand(this, core::mem::transmute_copy(&pextensioncommand), core::mem::transmute_copy(&pinitializationparameters), core::mem::transmute_copy(&initializationparameterssizeinbytes));
            }
        }
        unsafe extern "system" fn ExecuteExtensionCommand<Identity: ID3D12VideoProcessCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextensioncommand: *mut core::ffi::c_void, pexecutionparameters: *const core::ffi::c_void, executionparameterssizeinbytes: usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList2_Impl::ExecuteExtensionCommand(this, core::mem::transmute_copy(&pextensioncommand), core::mem::transmute_copy(&pexecutionparameters), core::mem::transmute_copy(&executionparameterssizeinbytes));
            }
        }
        Self {
            base__: ID3D12VideoProcessCommandList1_Vtbl::new::<Identity, OFFSET>(),
            SetProtectedResourceSession: SetProtectedResourceSession::<Identity, OFFSET>,
            InitializeExtensionCommand: InitializeExtensionCommand::<Identity, OFFSET>,
            ExecuteExtensionCommand: ExecuteExtensionCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoProcessCommandList2 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoProcessCommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoProcessCommandList1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoProcessCommandList2 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoProcessCommandList3, ID3D12VideoProcessCommandList3_Vtbl, 0x1a0a4ca4_9f08_40ce_9558_b411fd2666ff);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoProcessCommandList3 {
    type Target = ID3D12VideoProcessCommandList2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoProcessCommandList3, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12CommandList, ID3D12VideoProcessCommandList, ID3D12VideoProcessCommandList1, ID3D12VideoProcessCommandList2);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoProcessCommandList3 {
    pub unsafe fn Barrier(&self, pbarriergroups: &[super::d3d12::D3D12_BARRIER_GROUP]) {
        unsafe {
            (windows_core::Interface::vtable(self).Barrier)(windows_core::Interface::as_raw(self), pbarriergroups.len().try_into().unwrap(), core::mem::transmute(pbarriergroups.as_ptr()));
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoProcessCommandList3_Vtbl {
    pub base__: ID3D12VideoProcessCommandList2_Vtbl,
    pub Barrier: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::d3d12::D3D12_BARRIER_GROUP),
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
pub trait ID3D12VideoProcessCommandList3_Impl: ID3D12VideoProcessCommandList2_Impl {
    fn Barrier(&self, numbarriergroups: u32, pbarriergroups: *const super::d3d12::D3D12_BARRIER_GROUP);
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl ID3D12VideoProcessCommandList3_Vtbl {
    pub const fn new<Identity: ID3D12VideoProcessCommandList3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Barrier<Identity: ID3D12VideoProcessCommandList3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbarriergroups: u32, pbarriergroups: *const super::d3d12::D3D12_BARRIER_GROUP) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessCommandList3_Impl::Barrier(this, core::mem::transmute_copy(&numbarriergroups), core::mem::transmute_copy(&pbarriergroups));
            }
        }
        Self { base__: ID3D12VideoProcessCommandList2_Vtbl::new::<Identity, OFFSET>(), Barrier: Barrier::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoProcessCommandList3 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoProcessCommandList as windows_core::Interface>::IID || iid == &<ID3D12VideoProcessCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12VideoProcessCommandList2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D12VideoProcessCommandList3 {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoProcessor, ID3D12VideoProcessor_Vtbl, 0x304fdb32_bede_410a_8545_943ac6a46138);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoProcessor {
    type Target = super::d3d12::ID3D12Pageable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoProcessor, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoProcessor {
    pub unsafe fn GetNodeMask(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNodeMask)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNumInputStreamDescs(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetNumInputStreamDescs)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetInputStreamDescs(&self, pinputstreamdescs: &mut [D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInputStreamDescs)(windows_core::Interface::as_raw(self), pinputstreamdescs.len().try_into().unwrap(), core::mem::transmute(pinputstreamdescs.as_ptr())) }
    }
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn GetOutputStreamDesc(&self) -> D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamDesc)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoProcessor_Vtbl {
    pub base__: super::d3d12::ID3D12Pageable_Vtbl,
    pub GetNodeMask: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetNumInputStreamDescs: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub GetInputStreamDescs: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    GetInputStreamDescs: usize,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub GetOutputStreamDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC),
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    GetOutputStreamDesc: usize,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoProcessor_Impl: super::d3d12::ID3D12Pageable_Impl {
    fn GetNodeMask(&self) -> u32;
    fn GetNumInputStreamDescs(&self) -> u32;
    fn GetInputStreamDescs(&self, numinputstreamdescs: u32, pinputstreamdescs: *mut D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC) -> windows_core::Result<()>;
    fn GetOutputStreamDesc(&self) -> D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoProcessor_Vtbl {
    pub const fn new<Identity: ID3D12VideoProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNodeMask<Identity: ID3D12VideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessor_Impl::GetNodeMask(this)
            }
        }
        unsafe extern "system" fn GetNumInputStreamDescs<Identity: ID3D12VideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessor_Impl::GetNumInputStreamDescs(this)
            }
        }
        unsafe extern "system" fn GetInputStreamDescs<Identity: ID3D12VideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numinputstreamdescs: u32, pinputstreamdescs: *mut D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessor_Impl::GetInputStreamDescs(this, core::mem::transmute_copy(&numinputstreamdescs), core::mem::transmute_copy(&pinputstreamdescs)).into()
            }
        }
        unsafe extern "system" fn GetOutputStreamDesc<Identity: ID3D12VideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID3D12VideoProcessor_Impl::GetOutputStreamDesc(this);
            }
        }
        Self {
            base__: super::d3d12::ID3D12Pageable_Vtbl::new::<Identity, OFFSET>(),
            GetNodeMask: GetNodeMask::<Identity, OFFSET>,
            GetNumInputStreamDescs: GetNumInputStreamDescs::<Identity, OFFSET>,
            GetInputStreamDescs: GetInputStreamDescs::<Identity, OFFSET>,
            GetOutputStreamDesc: GetOutputStreamDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoProcessor as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoProcessor {}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::define_interface!(ID3D12VideoProcessor1, ID3D12VideoProcessor1_Vtbl, 0xf3cfe615_553f_425c_86d8_ee8c1b1fb01c);
#[cfg(feature = "Win32_d3d12")]
impl core::ops::Deref for ID3D12VideoProcessor1 {
    type Target = ID3D12VideoProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d3d12")]
windows_core::imp::interface_hierarchy!(ID3D12VideoProcessor1, windows_core::IUnknown, super::d3d12::ID3D12Object, super::d3d12::ID3D12DeviceChild, super::d3d12::ID3D12Pageable, ID3D12VideoProcessor);
#[cfg(feature = "Win32_d3d12")]
impl ID3D12VideoProcessor1 {
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProtectedResourceSession)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _).ok() }
    }
}
#[cfg(feature = "Win32_d3d12")]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D12VideoProcessor1_Vtbl {
    pub base__: ID3D12VideoProcessor_Vtbl,
    pub GetProtectedResourceSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
pub trait ID3D12VideoProcessor1_Impl: ID3D12VideoProcessor_Impl {
    fn GetProtectedResourceSession(&self, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl ID3D12VideoProcessor1_Vtbl {
    pub const fn new<Identity: ID3D12VideoProcessor1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: ID3D12VideoProcessor1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D12VideoProcessor1_Impl::GetProtectedResourceSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppprotectedsession)).into()
            }
        }
        Self { base__: ID3D12VideoProcessor_Vtbl::new::<Identity, OFFSET>(), GetProtectedResourceSession: GetProtectedResourceSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VideoProcessor1 as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Object as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d12::ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12VideoProcessor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d12", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
impl windows_core::RuntimeName for ID3D12VideoProcessor1 {}
