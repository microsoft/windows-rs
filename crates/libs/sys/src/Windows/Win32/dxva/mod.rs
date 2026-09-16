pub const COPP_ACP_ForceDWORD: COPP_ACP_Protection_Level = 2147483647;
pub const COPP_ACP_Level0: COPP_ACP_Protection_Level = 0;
pub const COPP_ACP_Level1: COPP_ACP_Protection_Level = 1;
pub const COPP_ACP_Level2: COPP_ACP_Protection_Level = 2;
pub const COPP_ACP_Level3: COPP_ACP_Protection_Level = 3;
pub const COPP_ACP_LevelMax: COPP_ACP_Protection_Level = 3;
pub const COPP_ACP_LevelMin: COPP_ACP_Protection_Level = 0;
pub type COPP_ACP_Protection_Level = i32;
pub const COPP_AspectRatio_EN300294_Box14by9Center: COPP_ImageAspectRatio_EN300294 = 1;
pub const COPP_AspectRatio_EN300294_Box14by9Top: COPP_ImageAspectRatio_EN300294 = 2;
pub const COPP_AspectRatio_EN300294_Box16by9Center: COPP_ImageAspectRatio_EN300294 = 3;
pub const COPP_AspectRatio_EN300294_Box16by9Top: COPP_ImageAspectRatio_EN300294 = 4;
pub const COPP_AspectRatio_EN300294_BoxGT16by9Center: COPP_ImageAspectRatio_EN300294 = 5;
pub const COPP_AspectRatio_EN300294_FullFormat16by9Anamorphic: COPP_ImageAspectRatio_EN300294 = 7;
pub const COPP_AspectRatio_EN300294_FullFormat4by3: COPP_ImageAspectRatio_EN300294 = 0;
pub const COPP_AspectRatio_EN300294_FullFormat4by3ProtectedCenter: COPP_ImageAspectRatio_EN300294 = 6;
pub const COPP_AspectRatio_ForceDWORD: COPP_ImageAspectRatio_EN300294 = 2147483647;
pub type COPP_BusType = i32;
pub const COPP_BusType_AGP: COPP_BusType = 4;
pub const COPP_BusType_ForceDWORD: COPP_BusType = 2147483647;
pub const COPP_BusType_Integrated: COPP_BusType = -2147483648;
pub const COPP_BusType_PCI: COPP_BusType = 1;
pub const COPP_BusType_PCIExpress: COPP_BusType = 3;
pub const COPP_BusType_PCIX: COPP_BusType = 2;
pub const COPP_BusType_Unknown: COPP_BusType = 0;
pub const COPP_CGMSA_CopyFreely: COPP_CGMSA_Protection_Level = 1;
pub const COPP_CGMSA_CopyNever: COPP_CGMSA_Protection_Level = 4;
pub const COPP_CGMSA_CopyNoMore: COPP_CGMSA_Protection_Level = 2;
pub const COPP_CGMSA_CopyOneGeneration: COPP_CGMSA_Protection_Level = 3;
pub const COPP_CGMSA_Disabled: COPP_CGMSA_Protection_Level = 0;
pub const COPP_CGMSA_ForceDWORD: COPP_CGMSA_Protection_Level = 2147483647;
pub const COPP_CGMSA_LevelMax: COPP_CGMSA_Protection_Level = 12;
pub const COPP_CGMSA_LevelMin: COPP_CGMSA_Protection_Level = 0;
pub type COPP_CGMSA_Protection_Level = i32;
pub const COPP_CGMSA_RedistributionControlRequired: COPP_CGMSA_Protection_Level = 8;
pub type COPP_ConnectorType = i32;
pub const COPP_ConnectorType_ComponentVideo: COPP_ConnectorType = 3;
pub const COPP_ConnectorType_CompositeVideo: COPP_ConnectorType = 2;
pub const COPP_ConnectorType_DVI: COPP_ConnectorType = 4;
pub const COPP_ConnectorType_D_JPN: COPP_ConnectorType = 8;
pub const COPP_ConnectorType_DisplayPortEmbedded: COPP_ConnectorType = 11;
pub const COPP_ConnectorType_DisplayPortExternal: COPP_ConnectorType = 10;
pub const COPP_ConnectorType_ForceDWORD: COPP_ConnectorType = 2147483647;
pub const COPP_ConnectorType_HDMI: COPP_ConnectorType = 5;
pub const COPP_ConnectorType_Internal: COPP_ConnectorType = -2147483648;
pub const COPP_ConnectorType_LVDS: COPP_ConnectorType = 6;
pub const COPP_ConnectorType_SDI: COPP_ConnectorType = 9;
pub const COPP_ConnectorType_SVideo: COPP_ConnectorType = 1;
pub const COPP_ConnectorType_TMDS: COPP_ConnectorType = 7;
pub const COPP_ConnectorType_UDIEmbedded: COPP_ConnectorType = 13;
pub const COPP_ConnectorType_UDIExternal: COPP_ConnectorType = 12;
pub const COPP_ConnectorType_Unknown: COPP_ConnectorType = -1;
pub const COPP_ConnectorType_VGA: COPP_ConnectorType = 0;
pub const COPP_DPCP_ForceDWORD: COPP_DPCP_Protection_Level = 2147483647;
pub const COPP_DPCP_Level0: COPP_DPCP_Protection_Level = 0;
pub const COPP_DPCP_Level1: COPP_DPCP_Protection_Level = 1;
pub const COPP_DPCP_LevelMax: COPP_DPCP_Protection_Level = 1;
pub const COPP_DPCP_LevelMin: COPP_DPCP_Protection_Level = 0;
pub type COPP_DPCP_Protection_Level = i32;
pub const COPP_DefaultProtectionLevel: i32 = 0;
pub const COPP_HDCPFlagsReserved: COPP_StatusHDCPFlags = -2;
pub const COPP_HDCPRepeater: COPP_StatusHDCPFlags = 1;
pub const COPP_HDCP_ForceDWORD: COPP_HDCP_Protection_Level = 2147483647;
pub const COPP_HDCP_Level0: COPP_HDCP_Protection_Level = 0;
pub const COPP_HDCP_Level1: COPP_HDCP_Protection_Level = 1;
pub const COPP_HDCP_LevelMax: COPP_HDCP_Protection_Level = 1;
pub const COPP_HDCP_LevelMin: COPP_HDCP_Protection_Level = 0;
pub type COPP_HDCP_Protection_Level = i32;
pub type COPP_ImageAspectRatio_EN300294 = i32;
pub const COPP_ImageAspectRatio_EN300294_Mask: i32 = 7;
pub const COPP_LinkLost: COPP_StatusFlags = 1;
pub const COPP_NoProtectionLevelAvailable: i32 = -1;
pub const COPP_ProtectionStandard_ARIBTRB15_1125i: COPP_TVProtectionStandard = 16384;
pub const COPP_ProtectionStandard_ARIBTRB15_525i: COPP_TVProtectionStandard = 2048;
pub const COPP_ProtectionStandard_ARIBTRB15_525p: COPP_TVProtectionStandard = 4096;
pub const COPP_ProtectionStandard_ARIBTRB15_750p: COPP_TVProtectionStandard = 8192;
pub const COPP_ProtectionStandard_CEA805A_TypeA_1125i: COPP_TVProtectionStandard = 128;
pub const COPP_ProtectionStandard_CEA805A_TypeA_525p: COPP_TVProtectionStandard = 32;
pub const COPP_ProtectionStandard_CEA805A_TypeA_750p: COPP_TVProtectionStandard = 64;
pub const COPP_ProtectionStandard_CEA805A_TypeB_1125i: COPP_TVProtectionStandard = 1024;
pub const COPP_ProtectionStandard_CEA805A_TypeB_525p: COPP_TVProtectionStandard = 256;
pub const COPP_ProtectionStandard_CEA805A_TypeB_750p: COPP_TVProtectionStandard = 512;
pub const COPP_ProtectionStandard_EIA608B_525: COPP_TVProtectionStandard = 8;
pub const COPP_ProtectionStandard_EN300294_625i: COPP_TVProtectionStandard = 16;
pub const COPP_ProtectionStandard_IEC61880_2_525i: COPP_TVProtectionStandard = 2;
pub const COPP_ProtectionStandard_IEC61880_525i: COPP_TVProtectionStandard = 1;
pub const COPP_ProtectionStandard_IEC62375_625p: COPP_TVProtectionStandard = 4;
pub const COPP_ProtectionStandard_Mask: COPP_TVProtectionStandard = -2147450881;
pub const COPP_ProtectionStandard_None: COPP_TVProtectionStandard = 0;
pub const COPP_ProtectionStandard_Reserved: COPP_TVProtectionStandard = 2147450880;
pub const COPP_ProtectionStandard_Unknown: COPP_TVProtectionStandard = -2147483648;
pub const COPP_ProtectionType_ACP: i32 = 2;
pub const COPP_ProtectionType_CGMSA: i32 = 4;
pub const COPP_ProtectionType_DPCP: i32 = 16;
pub const COPP_ProtectionType_HDCP: i32 = 1;
pub const COPP_ProtectionType_Mask: i32 = -2147483625;
pub const COPP_ProtectionType_None: i32 = 0;
pub const COPP_ProtectionType_Reserved: i32 = 2147483640;
pub const COPP_ProtectionType_Unknown: i32 = -2147483648;
pub const COPP_RenegotiationRequired: COPP_StatusFlags = 2;
pub type COPP_StatusFlags = i32;
pub const COPP_StatusFlagsReserved: COPP_StatusFlags = -4;
pub type COPP_StatusHDCPFlags = i32;
pub const COPP_StatusNormal: COPP_StatusFlags = 0;
pub type COPP_TVProtectionStandard = i32;
pub type DXVA_AI44sample = u8;
pub const DXVA_ALPHA_BLEND_COMBINATION_BUFFER: i32 = 13;
pub const DXVA_ALPHA_BLEND_COMBINATION_FUNCTION: i32 = 3;
pub const DXVA_ALPHA_BLEND_DATA_LOAD_FUNCTION: i32 = 2;
pub const DXVA_APV_MAX_COMPONENTS: i32 = 4;
pub const DXVA_APV_QM_DIMENSION: i32 = 8;
pub const DXVA_AYUV_BUFFER: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_AYUVsample2 {
    pub bCrValue: u8,
    pub bCbValue: u8,
    pub bY_Value: u8,
    pub bSampleAlpha8: u8,
}
pub const DXVA_BIDIRECTIONAL_AVERAGING_H263_TRUNC: i32 = 1;
pub const DXVA_BIDIRECTIONAL_AVERAGING_MPEG2_ROUND: i32 = 0;
pub const DXVA_BITSTREAM_CONCEALMENT_METHOD_BACKWARD: i32 = 3;
pub const DXVA_BITSTREAM_CONCEALMENT_METHOD_FORWARD: i32 = 2;
pub const DXVA_BITSTREAM_CONCEALMENT_METHOD_INTRA: i32 = 1;
pub const DXVA_BITSTREAM_CONCEALMENT_METHOD_UNSPECIFIED: i32 = 0;
pub const DXVA_BITSTREAM_CONCEALMENT_NEED_LIKELY: i32 = 2;
pub const DXVA_BITSTREAM_CONCEALMENT_NEED_MILD: i32 = 1;
pub const DXVA_BITSTREAM_CONCEALMENT_NEED_SEVERE: i32 = 3;
pub const DXVA_BITSTREAM_CONCEALMENT_NEED_UNLIKELY: i32 = 0;
pub const DXVA_BITSTREAM_DATA_BUFFER: i32 = 7;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_BlendCombination {
    pub wPictureSourceIndex: u16,
    pub wBlendedDestinationIndex: u16,
    pub PictureSourceRect16thPel: super::RECT,
    pub PictureDestinationRect: super::RECT,
    pub GraphicSourceRect: super::RECT,
    pub GraphicDestinationRect: super::RECT,
    pub wBlendDelay: u16,
    pub bBlendOn: u8,
    pub bWholePlaneAlpha: u8,
    pub OutsideYUVcolor: DXVA_AYUVsample2,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_BufferDescription {
    pub dwTypeIndex: u32,
    pub dwBufferIndex: u32,
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
    pub dwFirstMBaddress: u32,
    pub dwNumMBsInBuffer: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwStride: u32,
    pub dwReservedBits: u32,
}
pub const DXVA_CHROMA_FORMAT_420: i32 = 1;
pub const DXVA_CHROMA_FORMAT_422: i32 = 2;
pub const DXVA_CHROMA_FORMAT_444: i32 = 3;
pub const DXVA_COMPBUFFER_TYPE_THAT_IS_NOT_USED: i32 = 0;
pub const DXVA_CONFIG_BLEND_TYPE_BACK_HARDWARE: i32 = 1;
pub const DXVA_CONFIG_BLEND_TYPE_FRONT_BUFFER: i32 = 0;
pub const DXVA_CONFIG_DATA_TYPE_AI44: i32 = 1;
pub const DXVA_CONFIG_DATA_TYPE_AYUV: i32 = 3;
pub const DXVA_CONFIG_DATA_TYPE_DPXD: i32 = 2;
pub const DXVA_CONFIG_DATA_TYPE_IA44: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_COPPCommand {
    pub macKDI: windows_sys::core::GUID,
    pub guidCommandID: windows_sys::core::GUID,
    pub dwSequence: u32,
    pub cbSizeData: u32,
    pub CommandData: [u8; 4056],
}
impl Default for DXVA_COPPCommand {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_COPPCommandFnCode: i32 = 4;
pub const DXVA_COPPDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd2457add_8999_45ed_8a8a_d1aa047ba4d5);
pub const DXVA_COPPGetCertificateLengthFnCode: i32 = 1;
pub const DXVA_COPPKeyExchangeFnCode: i32 = 2;
pub const DXVA_COPPQueryBusData: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc6f4d673_6174_4184_8e35_f6db5200bcba);
pub const DXVA_COPPQueryConnectorType: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81d0bfd5_6afe_48c2_99c0_95a08f97c5da);
pub const DXVA_COPPQueryDisplayData: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7bf1ba3_ad13_4f8e_af98_0dcb3ca204cc);
pub const DXVA_COPPQueryGlobalProtectionLevel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1957210a_7766_452a_b99a_d27aed54f03a);
pub const DXVA_COPPQueryHDCPKeyData: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0db59d74_a992_492e_a0bd_c23fda564e00);
pub const DXVA_COPPQueryLocalProtectionLevel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb2075857_3eda_4d5d_88db_748f8c1a0549);
pub const DXVA_COPPQueryProtectionType: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x38f2a801_9a6c_48bb_9107_b6696e6f1797);
pub const DXVA_COPPQuerySignaling: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6629a591_3b79_4cf3_924a_11e8e7811671);
pub const DXVA_COPPQueryStatusFnCode: i32 = 5;
pub const DXVA_COPPSequenceStartFnCode: i32 = 3;
pub const DXVA_COPPSetProtectionLevel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9bb9327c_4eb5_4727_9f00_b42b0919c0da);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_COPPSetProtectionLevelCmdData {
    pub ProtType: u32,
    pub ProtLevel: u32,
    pub ExtendedInfoChangeMask: u32,
    pub ExtendedInfoData: u32,
}
pub const DXVA_COPPSetSignaling: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09a631a5_d684_4c60_8e4d_d3bb0f0be3ee);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_COPPSetSignalingCmdData {
    pub ActiveTVProtectionStandard: u32,
    pub AspectRatioChangeMask1: u32,
    pub AspectRatioData1: u32,
    pub AspectRatioChangeMask2: u32,
    pub AspectRatioData2: u32,
    pub AspectRatioChangeMask3: u32,
    pub AspectRatioData3: u32,
    pub ExtendedInfoChangeMask: [u32; 4],
    pub ExtendedInfoData: [u32; 4],
    pub Reserved: u32,
}
impl Default for DXVA_COPPSetSignalingCmdData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_COPPSignature {
    pub Signature: [u8; 256],
}
impl Default for DXVA_COPPSignature {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_COPPStatusData {
    pub rApp: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub dwData: u32,
    pub ExtendedInfoValidMask: u32,
    pub ExtendedInfoData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_COPPStatusDisplayData {
    pub rApp: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub DisplayWidth: u32,
    pub DisplayHeight: u32,
    pub Format: u32,
    pub d3dFormat: u32,
    pub FreqNumerator: u32,
    pub FreqDenominator: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_COPPStatusHDCPKeyData {
    pub rApp: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub dwHDCPFlags: u32,
    pub BKey: windows_sys::core::GUID,
    pub Reserved1: windows_sys::core::GUID,
    pub Reserved2: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_COPPStatusInput {
    pub rApp: windows_sys::core::GUID,
    pub guidStatusRequestID: windows_sys::core::GUID,
    pub dwSequence: u32,
    pub cbSizeData: u32,
    pub StatusData: [u8; 4056],
}
impl Default for DXVA_COPPStatusInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_COPPStatusOutput {
    pub macKDI: windows_sys::core::GUID,
    pub cbSizeData: u32,
    pub COPPStatus: [u8; 4076],
}
impl Default for DXVA_COPPStatusOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_COPPStatusSignalingCmdData {
    pub rApp: windows_sys::core::GUID,
    pub dwFlags: u32,
    pub AvailableTVProtectionStandards: u32,
    pub ActiveTVProtectionStandard: u32,
    pub TVType: u32,
    pub AspectRatioValidMask1: u32,
    pub AspectRatioData1: u32,
    pub AspectRatioValidMask2: u32,
    pub AspectRatioData2: u32,
    pub AspectRatioValidMask3: u32,
    pub AspectRatioData3: u32,
    pub ExtendedInfoValidMask: [u32; 4],
    pub ExtendedInfoData: [u32; 4],
}
impl Default for DXVA_COPPStatusSignalingCmdData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_ConfigAlphaCombine {
    pub dwFunction: DXVA_ConfigQueryOrReplyFunc,
    pub dwReservedBits: [u32; 3],
    pub bConfigBlendType: u8,
    pub bConfigPictureResizing: u8,
    pub bConfigOnlyUsePicDestRectArea: u8,
    pub bConfigGraphicResizing: u8,
    pub bConfigWholePlaneAlpha: u8,
}
impl Default for DXVA_ConfigAlphaCombine {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_ConfigAlphaLoad {
    pub dwFunction: DXVA_ConfigQueryOrReplyFunc,
    pub dwReservedBits: [u32; 3],
    pub bConfigDataType: u8,
}
impl Default for DXVA_ConfigAlphaLoad {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_ConfigPictureDecode {
    pub dwFunction: DXVA_ConfigQueryOrReplyFunc,
    pub dwReservedBits: [u32; 3],
    pub guidConfigBitstreamEncryption: windows_sys::core::GUID,
    pub guidConfigMBcontrolEncryption: windows_sys::core::GUID,
    pub guidConfigResidDiffEncryption: windows_sys::core::GUID,
    pub bConfigBitstreamRaw: u8,
    pub bConfigMBcontrolRasterOrder: u8,
    pub bConfigResidDiffHost: u8,
    pub bConfigSpatialResid8: u8,
    pub bConfigResid8Subtraction: u8,
    pub bConfigSpatialHost8or9Clipping: u8,
    pub bConfigSpatialResidInterleaved: u8,
    pub bConfigIntraResidUnsigned: u8,
    pub bConfigResidDiffAccelerator: u8,
    pub bConfigHostInverseScan: u8,
    pub bConfigSpecificIDCT: u8,
    pub bConfig4GroupedCoefs: u8,
}
impl Default for DXVA_ConfigPictureDecode {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXVA_ConfigQueryOrReplyFunc = u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_ConnectMode {
    pub guidMode: windows_sys::core::GUID,
    pub wRestrictedMode: u16,
}
pub type DXVA_DCCMD = u16;
pub const DXVA_DCCMD_SURFACE_BUFFER: i32 = 12;
pub const DXVA_DEBLOCKING_CONTROL_BUFFER: i32 = 4;
pub const DXVA_DEBLOCKING_FILTER_FUNCTION: i32 = 5;
pub type DXVA_DPXD = u8;
pub const DXVA_DPXD_SURFACE_BUFFER: i32 = 10;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_DeblockIndexAB_H264 {
    pub bIndexAinternal: u8,
    pub bIndexBinternal: u8,
    pub bIndexAleft0: u8,
    pub bIndexBleft0: u8,
    pub bIndexAleft1: u8,
    pub bIndexBleft1: u8,
    pub bIndexAtop0: u8,
    pub bIndexBtop0: u8,
    pub bIndexAtop1: u8,
    pub bIndexBtop1: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_Deblock_H264 {
    pub CurrMbAddr: u16,
    pub Anonymous: DXVA_Deblock_H264_0,
    pub Reserved8Bits: u8,
    pub bbSinternalLeftVert: u8,
    pub bbSinternalMidVert: u8,
    pub bbSinternalRightVert: u8,
    pub bbSinternalTopHorz: u8,
    pub bbSinternalMidHorz: u8,
    pub bbSinternalBotHorz: u8,
    pub wbSLeft0: u16,
    pub wbSLeft1: u16,
    pub wbSTop0: u16,
    pub wbSTop1: u16,
    pub IndexAB: [DXVA_DeblockIndexAB_H264; 3],
}
impl Default for DXVA_Deblock_H264 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_Deblock_H264_0 {
    pub Anonymous: DXVA_Deblock_H264_0_0,
    pub FirstByte: u8,
}
impl Default for DXVA_Deblock_H264_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_Deblock_H264_0_0 {
    pub _bitfield: u8,
}
pub type DXVA_DeblockingEdgeControl = u8;
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct DXVA_DeinterlaceBlt {
    pub Size: u32,
    pub Reserved: u32,
    pub rtTarget: super::REFERENCE_TIME,
    pub DstRect: super::RECT,
    pub SrcRect: super::RECT,
    pub NumSourceSurfaces: u32,
    pub Alpha: f32,
    pub Source: [DXVA_VideoSample; 32],
}
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for DXVA_DeinterlaceBlt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct DXVA_DeinterlaceBltEx {
    pub Size: u32,
    pub BackgroundColor: DXVA_AYUVsample2,
    pub rcTarget: super::RECT,
    pub rtTarget: super::REFERENCE_TIME,
    pub NumSourceSurfaces: u32,
    pub Alpha: f32,
    pub Source: [DXVA_VideoSample2; 32],
    pub DestinationFormat: u32,
    pub DestinationFlags: u32,
}
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for DXVA_DeinterlaceBltEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct DXVA_DeinterlaceBltEx32 {
    pub Size: u32,
    pub BackgroundColor: DXVA_AYUVsample2,
    pub rcTarget: super::RECT,
    pub rtTarget: super::REFERENCE_TIME,
    pub NumSourceSurfaces: u32,
    pub Alpha: f32,
    pub Source: [DXVA_VideoSample32; 32],
    pub DestinationFormat: u32,
    pub DestinationFlags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for DXVA_DeinterlaceBltEx32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_DeinterlaceBltExFnCode: i32 = 2;
pub const DXVA_DeinterlaceBltFnCode: i32 = 1;
pub const DXVA_DeinterlaceBobDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x335aa36e_7884_43a4_9c91_7f87faf3e37e);
#[repr(C)]
#[cfg(feature = "d3d9")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_DeinterlaceCaps {
    pub Size: u32,
    pub NumPreviousOutputFrames: u32,
    pub InputPool: u32,
    pub NumForwardRefSamples: u32,
    pub NumBackwardRefSamples: u32,
    pub d3dOutputFormat: super::D3DFORMAT,
    pub VideoProcessingCaps: DXVA_VideoProcessCaps,
    pub DeinterlaceTechnology: DXVA_DeinterlaceTech,
}
pub const DXVA_DeinterlaceContainerDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e85cb93_3046_4ff0_aecc_d58cb5f035fd);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_DeinterlaceQueryAvailableModes {
    pub Size: u32,
    pub NumGuids: u32,
    pub Guids: [windows_sys::core::GUID; 32],
}
impl Default for DXVA_DeinterlaceQueryAvailableModes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_DeinterlaceQueryAvailableModesFnCode: i32 = 1;
#[repr(C)]
#[cfg(feature = "d3d9")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_DeinterlaceQueryModeCaps {
    pub Size: u32,
    pub Guid: windows_sys::core::GUID,
    pub VideoDesc: DXVA_VideoDesc,
}
pub const DXVA_DeinterlaceQueryModeCapsFnCode: i32 = 2;
pub type DXVA_DeinterlaceTech = i32;
pub const DXVA_DeinterlaceTech_BOBLineReplicate: DXVA_DeinterlaceTech = 1;
pub const DXVA_DeinterlaceTech_BOBVerticalStretch: DXVA_DeinterlaceTech = 2;
pub const DXVA_DeinterlaceTech_BOBVerticalStretch4Tap: DXVA_DeinterlaceTech = 256;
pub const DXVA_DeinterlaceTech_EdgeFiltering: DXVA_DeinterlaceTech = 16;
pub const DXVA_DeinterlaceTech_FieldAdaptive: DXVA_DeinterlaceTech = 32;
pub const DXVA_DeinterlaceTech_MedianFiltering: DXVA_DeinterlaceTech = 4;
pub const DXVA_DeinterlaceTech_MotionVectorSteered: DXVA_DeinterlaceTech = 128;
pub const DXVA_DeinterlaceTech_PixelAdaptive: DXVA_DeinterlaceTech = 64;
pub const DXVA_DeinterlaceTech_Unknown: DXVA_DeinterlaceTech = 0;
pub const DXVA_DestinationFlagMask: DXVA_DestinationFlags = 15;
pub const DXVA_DestinationFlag_Alpha_Changed: DXVA_DestinationFlags = 8;
pub const DXVA_DestinationFlag_Background_Changed: DXVA_DestinationFlags = 1;
pub const DXVA_DestinationFlag_ColorData_Changed: DXVA_DestinationFlags = 4;
pub const DXVA_DestinationFlag_TargetRect_Changed: DXVA_DestinationFlags = 2;
pub type DXVA_DestinationFlags = i32;
pub const DXVA_ENCRYPTPROTOCOLFUNCFLAG_ACCEL: i32 = 16776968;
pub const DXVA_ENCRYPTPROTOCOLFUNCFLAG_HOST: i32 = 16776960;
pub const DXVA_EXECUTE_RETURN_DATA_ERROR_MINOR: i32 = 1;
pub const DXVA_EXECUTE_RETURN_DATA_ERROR_SEVERE: i32 = 3;
pub const DXVA_EXECUTE_RETURN_DATA_ERROR_SIGNIF: i32 = 2;
pub const DXVA_EXECUTE_RETURN_OK: i32 = 0;
pub const DXVA_EXECUTE_RETURN_OTHER_ERROR_SEVERE: i32 = 4;
pub type DXVA_EncryptProtocolFunc = u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_EncryptProtocolHeader {
    pub dwFunction: DXVA_EncryptProtocolFunc,
    pub ReservedBits: [u32; 3],
    pub guidEncryptProtocol: windows_sys::core::GUID,
}
impl Default for DXVA_EncryptProtocolHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_ExtColorData_ShiftBase: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_ExtendedFormat {
    pub _bitfield: u32,
}
pub const DXVA_FILM_GRAIN_BUFFER: i32 = 17;
pub const DXVA_FILM_GRAIN_SYNTHESIS_FUNCTION: i32 = 6;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_FilmGrainChar_H264 {
    pub wFrameWidthInMbsMinus1: u16,
    pub wFrameHeightInMbsMinus1: u16,
    pub InPic: DXVA_PicEntry_H264,
    pub OutPic: DXVA_PicEntry_H264,
    pub PicOrderCnt_offset: u16,
    pub CurrPicOrderCnt: i32,
    pub StatusReportFeedbackNumber: u32,
    pub model_id: u8,
    pub separate_colour_description_present_flag: u8,
    pub film_grain_bit_depth_luma_minus8: u8,
    pub film_grain_bit_depth_chroma_minus8: u8,
    pub film_grain_full_range_flag: u8,
    pub film_grain_colour_primaries: u8,
    pub film_grain_transfer_characteristics: u8,
    pub film_grain_matrix_coefficients: u8,
    pub blending_mode_id: u8,
    pub log2_scale_factor: u8,
    pub comp_model_present_flag: [u8; 4],
    pub num_intensity_intervals_minus1: [u8; 4],
    pub num_model_values_minus1: [u8; 4],
    pub intensity_interval_lower_bound: [[u8; 16]; 3],
    pub intensity_interval_upper_bound: [[u8; 16]; 3],
    pub comp_model_value: [[[i16; 8]; 16]; 3],
}
impl Default for DXVA_FilmGrainChar_H264 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_Frequency {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub const DXVA_HIGHLIGHT_BUFFER: i32 = 11;
#[repr(C, packed(1))]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_Highlight {
    pub wHighlightActive: u16,
    pub wHighlightIndices: u16,
    pub wHighlightAlphas: u16,
    pub HighlightRect: super::RECT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_HuffmanTable_MJPEG {
    pub bits_ac: [[u8; 16]; 4],
    pub table_ac: [[u8; 256]; 4],
    pub bits_dc: [[u8; 16]; 4],
    pub table_dc: [[u8; 256]; 4],
}
impl Default for DXVA_HuffmanTable_MJPEG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_IA44_SURFACE_BUFFER: i32 = 9;
pub type DXVA_IA44sample = u8;
pub const DXVA_INVERSE_QUANTIZATION_MATRIX_BUFFER: i32 = 5;
pub const DXVA_MACROBLOCK_CONTROL_BUFFER: i32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_MBctrl_H264 {
    pub Anonymous: DXVA_MBctrl_H264_0,
    pub CurrMbAddr: u16,
    pub wPatternCode: [u16; 3],
    pub bQpPrime: [u8; 3],
    pub bMBresidDataQuantity: u8,
    pub dwMBdataLocation: u32,
    pub Anonymous2: DXVA_MBctrl_H264_1,
}
impl Default for DXVA_MBctrl_H264 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_MBctrl_H264_0 {
    pub Anonymous: DXVA_MBctrl_H264_0_0,
    pub dwMBtype: u32,
}
impl Default for DXVA_MBctrl_H264_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_MBctrl_H264_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_MBctrl_H264_1 {
    pub Anonymous: DXVA_MBctrl_H264_1_0,
    pub Anonymous2: DXVA_MBctrl_H264_1_1,
}
impl Default for DXVA_MBctrl_H264_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_MBctrl_H264_1_0 {
    pub LumaIntraPredModes: [u16; 4],
    pub Anonymous: DXVA_MBctrl_H264_1_0_0,
    pub ReservedIntra24Bits: [u8; 3],
}
impl Default for DXVA_MBctrl_H264_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_MBctrl_H264_1_0_0 {
    pub Anonymous: DXVA_MBctrl_H264_1_0_0_0,
    pub bMbIntraStruct: u8,
}
impl Default for DXVA_MBctrl_H264_1_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_MBctrl_H264_1_0_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_MBctrl_H264_1_1 {
    pub bSubMbShapes: u8,
    pub bSubMbPredModes: u8,
    pub wMvBuffOffset: u16,
    pub bRefPicSelect: [[u8; 4]; 2],
}
impl Default for DXVA_MBctrl_H264_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_MBctrl_I_HostResidDiff_1 {
    pub wMBaddress: u16,
    pub wMBtype: u16,
    pub dwMB_SNL: u32,
    pub wPatternCode: u16,
    pub wPC_Overflow: u16,
    pub dwReservedBits2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_MBctrl_I_OffHostIDCT_1 {
    pub wMBaddress: u16,
    pub wMBtype: u16,
    pub dwMB_SNL: u32,
    pub wPatternCode: u16,
    pub bNumCoef: [u8; 6],
}
impl Default for DXVA_MBctrl_I_OffHostIDCT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_MBctrl_P_HostResidDiff_1 {
    pub wMBaddress: u16,
    pub wMBtype: u16,
    pub dwMB_SNL: u32,
    pub wPatternCode: u16,
    pub wPC_Overflow: u16,
    pub dwReservedBits2: u32,
    pub MVector: [DXVA_MVvalue; 4],
}
impl Default for DXVA_MBctrl_P_HostResidDiff_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_MBctrl_P_OffHostIDCT_1 {
    pub wMBaddress: u16,
    pub wMBtype: u16,
    pub dwMB_SNL: u32,
    pub wPatternCode: u16,
    pub bNumCoef: [u8; 6],
    pub MVector: [DXVA_MVvalue; 4],
}
impl Default for DXVA_MBctrl_P_OffHostIDCT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_MOTION_VECTOR_BUFFER: i32 = 16;
pub const DXVA_MV_PRECISION_AND_CHROMA_RELATION_H261: i32 = 2;
pub const DXVA_MV_PRECISION_AND_CHROMA_RELATION_H263: i32 = 1;
pub const DXVA_MV_PRECISION_AND_CHROMA_RELATION_MPEG2: i32 = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_MVvalue {
    pub horz: i16,
    pub vert: i16,
}
pub const DXVA_ModeAPV_VLD_400_10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x37148862_6bd6_4618_8293_777b686b0824);
pub const DXVA_ModeAPV_VLD_422_10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x226a709d_ae12_44c5_ba21_164feeb7f9b6);
pub const DXVA_ModeAPV_VLD_422_12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf6f152ad_94e5_4bfa_9227_676cddeff42b);
pub const DXVA_ModeAPV_VLD_4444_10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc83799b9_9655_4b95_8008_56a322ce5d81);
pub const DXVA_ModeAPV_VLD_4444_12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a763ee3_4d05_47fe_a429_723474b69d7c);
pub const DXVA_ModeAPV_VLD_444_10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a4a8d7d_7610_469f_855f_39f13051c013);
pub const DXVA_ModeAPV_VLD_444_12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf1039a1c_e208_45c1_952c_040841b67667);
pub const DXVA_ModeAV1_VLD_12bit_Profile2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x17127009_a00f_4ce1_994e_bf4081f6f3f0);
pub const DXVA_ModeAV1_VLD_12bit_Profile2_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d80bed6_9cac_4835_9e91_327bbc4f9ee8);
pub const DXVA_ModeAV1_VLD_Profile0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb8be4ccb_cf53_46ba_8d59_d6b8a6da5d2a);
pub const DXVA_ModeAV1_VLD_Profile1: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6936ff0f_45b1_4163_9cc1_646ef6946108);
pub const DXVA_ModeAV1_VLD_Profile2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0c5f2aa1_e541_4089_bb7b_98110a19d7c8);
pub const DXVA_ModeH261_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be01_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH261_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be02_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be03_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be04_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_C: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be05_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_D: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be06_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_E: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be07_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_F: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be08_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be64_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be65_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_C: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be66_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_D: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be67_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_E: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_F: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be69_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_VLD_Multiview_NoFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const DXVA_ModeH264_VLD_Stereo_NoFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const DXVA_ModeH264_VLD_Stereo_Progressive_NoFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const DXVA_ModeH264_VLD_WithFMOASO_NoFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd5f04ff9_3418_45d8_9561_32a76aae2ddd);
pub const DXVA_ModeHEVC_VLD_Main: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const DXVA_ModeHEVC_VLD_Main10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const DXVA_ModeHEVC_VLD_Main10_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0bac4fe5_1532_4429_a854_f84de04953db);
pub const DXVA_ModeHEVC_VLD_Main10_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0dabeffa_4458_4602_bc03_0795659d617c);
pub const DXVA_ModeHEVC_VLD_Main10_Ext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9cc55490_e37c_4932_8684_4920f9f6409c);
pub const DXVA_ModeHEVC_VLD_Main12: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1a72925f_0c2c_4f15_96fb_b17d1473603f);
pub const DXVA_ModeHEVC_VLD_Main12_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55bcac81_f311_4093_a7d0_1cbc0b849bee);
pub const DXVA_ModeHEVC_VLD_Main12_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9798634d_fe9d_48e5_b4da_dbec45b3df01);
pub const DXVA_ModeHEVC_VLD_Main16: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4fbdbb0_a113_482b_a232_635cc0697f6d);
pub const DXVA_ModeHEVC_VLD_Main_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4008018f_f537_4b36_98cf_61af8a2c1a33);
pub const DXVA_ModeHEVC_VLD_Monochrome: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0685b993_3d8c_43a0_8b28_d74c2d6899a4);
pub const DXVA_ModeHEVC_VLD_Monochrome10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x142a1d0f_69dd_4ec9_8591_b12ffcb91a29);
pub const DXVA_ModeJPEG_VLD_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf782c83_bef5_4a2c_87cb_6019e7b175ac);
pub const DXVA_ModeJPEG_VLD_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf04df417_eee2_4067_a778_f35c15ab9721);
pub const DXVA_ModeJPEG_VLD_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4cd00e17_89ba_48ef_b9f9_edcb82713f65);
pub const DXVA_ModeMJPEG_VLD_420: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x725cb506_0c29_43c4_9440_8e9397903a04);
pub const DXVA_ModeMJPEG_VLD_422: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b77b9cd_1a35_4c30_9fd8_ef4b60c035dd);
pub const DXVA_ModeMJPEG_VLD_444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd95161f9_0d44_47e6_bcf5_1bfbfb268f97);
pub const DXVA_ModeMJPEG_VLD_4444: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc91748d5_fd18_4aca_9db3_3a6634ab547d);
pub const DXVA_ModeMPEG1_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be09_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG1_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f3ec719_3735_42cc_8063_65cc3cb36616);
pub const DXVA_ModeMPEG2_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be0a_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG2_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be0b_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG2_C: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be0c_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG2_D: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be0d_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG2and1_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const DXVA_ModeMPEG4pt2_VLD_AdvSimple_GMC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab998b5b_4258_44a9_9feb_94e597a6baae);
pub const DXVA_ModeMPEG4pt2_VLD_AdvSimple_NoGMC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const DXVA_ModeMPEG4pt2_VLD_Simple: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const DXVA_ModeNone: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be00_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea0_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea1_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_C: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea2_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_D: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_D2010: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVP8_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const DXVA_ModeVP9_VLD_10bit_Profile2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
pub const DXVA_ModeVP9_VLD_Profile0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const DXVA_ModeWMV8_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be80_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeWMV8_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be81_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeWMV9_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be90_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeWMV9_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be91_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeWMV9_C: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be94_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_NUM_TYPES_COMP_BUFFERS: i32 = 18;
pub const DXVA_NoEncrypt: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bed0_a0c7_11d3_b984_00c04f2e73c5);
pub type DXVA_NominalRange = i32;
pub const DXVA_NominalRangeMask: DXVA_NominalRange = 28672;
pub const DXVA_NominalRangeShift: DXVA_NominalRange = 12;
pub const DXVA_NominalRange_0_255: DXVA_NominalRange = 1;
pub const DXVA_NominalRange_16_235: DXVA_NominalRange = 2;
pub const DXVA_NominalRange_48_208: DXVA_NominalRange = 3;
pub const DXVA_NominalRange_Normal: DXVA_NominalRange = 1;
pub const DXVA_NominalRange_Unknown: DXVA_NominalRange = 0;
pub const DXVA_NominalRange_Wide: DXVA_NominalRange = 2;
pub const DXVA_NumBlocksPerMB_420: i32 = 6;
pub const DXVA_NumBlocksPerMB_422: i32 = 8;
pub const DXVA_NumBlocksPerMB_444: i32 = 12;
pub const DXVA_NumMV_OBMC_off_BinPBwith4MV_off: i32 = 4;
pub const DXVA_NumMV_OBMC_off_BinPBwith4MV_on: i32 = 5;
pub const DXVA_NumMV_OBMC_on__BinPB_off: i32 = 10;
pub const DXVA_NumMV_OBMC_on__BinPB_on: i32 = 11;
pub const DXVA_PICTURE_DECODE_BUFFER: i32 = 1;
pub const DXVA_PICTURE_DECODING_FUNCTION: i32 = 1;
pub const DXVA_PICTURE_RESAMPLE_BUFFER: i32 = 14;
pub const DXVA_PICTURE_RESAMPLE_FUNCTION: i32 = 4;
pub const DXVA_PICTURE_STRUCTURE_BOTTOM_FIELD: i32 = 2;
pub const DXVA_PICTURE_STRUCTURE_FRAME: i32 = 3;
pub const DXVA_PICTURE_STRUCTURE_TOP_FIELD: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_PicEntry_APV {
    pub Anonymous: DXVA_PicEntry_APV_0,
}
impl Default for DXVA_PicEntry_APV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicEntry_APV_0 {
    pub Anonymous: DXVA_PicEntry_APV_0_0,
    pub bPicEntry: u8,
}
impl Default for DXVA_PicEntry_APV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicEntry_APV_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicEntry_AV1 {
    pub width: u32,
    pub height: u32,
    pub wmmat: [i32; 6],
    pub Anonymous: DXVA_PicEntry_AV1_0,
    pub Index: u8,
    pub Reserved16Bits: u16,
}
impl Default for DXVA_PicEntry_AV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicEntry_AV1_0 {
    pub Anonymous: DXVA_PicEntry_AV1_0_0,
    pub GlobalMotionFlags: u8,
}
impl Default for DXVA_PicEntry_AV1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicEntry_AV1_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_PicEntry_H264 {
    pub Anonymous: DXVA_PicEntry_H264_0,
}
impl Default for DXVA_PicEntry_H264 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicEntry_H264_0 {
    pub Anonymous: DXVA_PicEntry_H264_0_0,
    pub bPicEntry: u8,
}
impl Default for DXVA_PicEntry_H264_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicEntry_H264_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_PicEntry_HEVC {
    pub Anonymous: DXVA_PicEntry_HEVC_0,
}
impl Default for DXVA_PicEntry_HEVC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicEntry_HEVC_0 {
    pub Anonymous: DXVA_PicEntry_HEVC_0_0,
    pub bPicEntry: u8,
}
impl Default for DXVA_PicEntry_HEVC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicEntry_HEVC_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_PicEntry_VPx {
    pub Anonymous: DXVA_PicEntry_VPx_0,
}
impl Default for DXVA_PicEntry_VPx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicEntry_VPx_0 {
    pub Anonymous: DXVA_PicEntry_VPx_0_0,
    pub bPicEntry: u8,
}
impl Default for DXVA_PicEntry_VPx_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicEntry_VPx_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_APV {
    pub pbu_reserved_zero_8bits: u8,
    pub reserved_zero_32bits: u32,
    pub frame_width: u32,
    pub frame_height: u32,
    pub chroma_format_idc: u8,
    pub bit_depth_minus8: u8,
    pub frame_info_reserved_zero_32bits: u32,
    pub frame_header_reserved_zero_8bits: u8,
    pub use_q_matrix: u8,
    pub q_reserved_zero_8bits: u8,
    pub tile_width_in_mbs: u32,
    pub tile_height_in_mbs: u32,
    pub tile_size_present_in_fh_flag: u8,
    pub tileinfo_reserved_zero_8bits: u8,
    pub tile_reserved_16bits: u16,
    pub statusReportFeedbackNumber: u32,
    pub CurrPic: DXVA_PicEntry_APV,
}
impl Default for DXVA_PicParams_APV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_AV1 {
    pub width: u32,
    pub height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub CurrPicTextureIndex: u8,
    pub superres_denom: u8,
    pub bitdepth: u8,
    pub seq_profile: u8,
    pub tiles: DXVA_PicParams_AV1_0,
    pub coding: DXVA_PicParams_AV1_1,
    pub format: DXVA_PicParams_AV1_2,
    pub primary_ref_frame: u8,
    pub order_hint: u8,
    pub order_hint_bits: u8,
    pub frame_refs: [DXVA_PicEntry_AV1; 7],
    pub RefFrameMapTextureIndex: [u8; 8],
    pub loop_filter: DXVA_PicParams_AV1_3,
    pub quantization: DXVA_PicParams_AV1_4,
    pub cdef: DXVA_PicParams_AV1_5,
    pub interp_filter: u8,
    pub segmentation: DXVA_PicParams_AV1_6,
    pub film_grain: DXVA_PicParams_AV1_7,
    pub Reserved32Bits: u32,
    pub StatusReportFeedbackNumber: u32,
}
impl Default for DXVA_PicParams_AV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_AV1_0 {
    pub cols: u8,
    pub rows: u8,
    pub context_update_id: u16,
    pub widths: [u16; 64],
    pub heights: [u16; 64],
}
impl Default for DXVA_PicParams_AV1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_1 {
    pub Anonymous: DXVA_PicParams_AV1_1_0,
    pub CodingParamToolFlags: u32,
}
impl Default for DXVA_PicParams_AV1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_2 {
    pub Anonymous: DXVA_PicParams_AV1_2_0,
    pub FormatAndPictureInfoFlags: u8,
}
impl Default for DXVA_PicParams_AV1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_2_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_AV1_3 {
    pub filter_level: [u8; 2],
    pub filter_level_u: u8,
    pub filter_level_v: u8,
    pub sharpness_level: u8,
    pub Anonymous: DXVA_PicParams_AV1_3_0,
    pub ref_deltas: [i8; 8],
    pub mode_deltas: [i8; 2],
    pub delta_lf_res: u8,
    pub frame_restoration_type: [u8; 3],
    pub log2_restoration_unit_size: [u16; 3],
    pub Reserved16Bits: u16,
}
impl Default for DXVA_PicParams_AV1_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_3_0 {
    pub Anonymous: DXVA_PicParams_AV1_3_0_0,
    pub ControlFlags: u8,
}
impl Default for DXVA_PicParams_AV1_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_3_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_AV1_4 {
    pub Anonymous: DXVA_PicParams_AV1_4_0,
    pub base_qindex: u8,
    pub y_dc_delta_q: i8,
    pub u_dc_delta_q: i8,
    pub v_dc_delta_q: i8,
    pub u_ac_delta_q: i8,
    pub v_ac_delta_q: i8,
    pub qm_y: u8,
    pub qm_u: u8,
    pub qm_v: u8,
    pub Reserved16Bits: u16,
}
impl Default for DXVA_PicParams_AV1_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_4_0 {
    pub Anonymous: DXVA_PicParams_AV1_4_0_0,
    pub ControlFlags: u8,
}
impl Default for DXVA_PicParams_AV1_4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_4_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_AV1_5 {
    pub Anonymous: DXVA_PicParams_AV1_5_0,
    pub y_strengths: [DXVA_PicParams_AV1_5_1; 8],
    pub uv_strengths: [DXVA_PicParams_AV1_5_2; 8],
}
impl Default for DXVA_PicParams_AV1_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_5_0 {
    pub Anonymous: DXVA_PicParams_AV1_5_0_0,
    pub ControlFlags: u8,
}
impl Default for DXVA_PicParams_AV1_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_5_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_AV1_6 {
    pub Anonymous: DXVA_PicParams_AV1_6_0,
    pub Reserved24Bits: [u8; 3],
    pub feature_mask: [DXVA_PicParams_AV1_6_1; 8],
    pub feature_data: [[i16; 8]; 8],
}
impl Default for DXVA_PicParams_AV1_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_6_0 {
    pub Anonymous: DXVA_PicParams_AV1_6_0_0,
    pub ControlFlags: u8,
}
impl Default for DXVA_PicParams_AV1_6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_6_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_AV1_7 {
    pub Anonymous: DXVA_PicParams_AV1_7_0,
    pub grain_seed: u16,
    pub scaling_points_y: [[u8; 2]; 14],
    pub num_y_points: u8,
    pub scaling_points_cb: [[u8; 2]; 10],
    pub num_cb_points: u8,
    pub scaling_points_cr: [[u8; 2]; 10],
    pub num_cr_points: u8,
    pub ar_coeffs_y: [u8; 24],
    pub ar_coeffs_cb: [u8; 25],
    pub ar_coeffs_cr: [u8; 25],
    pub cb_mult: u8,
    pub cb_luma_mult: u8,
    pub cr_mult: u8,
    pub cr_luma_mult: u8,
    pub Reserved8Bits: u8,
    pub cb_offset: i16,
    pub cr_offset: i16,
}
impl Default for DXVA_PicParams_AV1_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_7_0 {
    pub Anonymous: DXVA_PicParams_AV1_7_0_0,
    pub ControlFlags: u16,
}
impl Default for DXVA_PicParams_AV1_7_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_7_0_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_5_1 {
    pub Anonymous: DXVA_PicParams_AV1_5_1_0,
    pub combined: u8,
}
impl Default for DXVA_PicParams_AV1_5_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_5_1_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_5_2 {
    pub Anonymous: DXVA_PicParams_AV1_5_2_0,
    pub combined: u8,
}
impl Default for DXVA_PicParams_AV1_5_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_5_2_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_AV1_6_1 {
    pub Anonymous: DXVA_PicParams_AV1_6_1_0,
    pub mask: u8,
}
impl Default for DXVA_PicParams_AV1_6_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_AV1_6_1_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_H264 {
    pub wFrameWidthInMbsMinus1: u16,
    pub wFrameHeightInMbsMinus1: u16,
    pub CurrPic: DXVA_PicEntry_H264,
    pub num_ref_frames: u8,
    pub Anonymous: DXVA_PicParams_H264_0,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub Reserved16Bits: u16,
    pub StatusReportFeedbackNumber: u32,
    pub RefFrameList: [DXVA_PicEntry_H264; 16],
    pub CurrFieldOrderCnt: [i32; 2],
    pub FieldOrderCntList: [[i32; 2]; 16],
    pub pic_init_qs_minus26: i8,
    pub chroma_qp_index_offset: i8,
    pub second_chroma_qp_index_offset: i8,
    pub ContinuationFlag: u8,
    pub pic_init_qp_minus26: i8,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub Reserved8BitsA: u8,
    pub FrameNumList: [u16; 16],
    pub UsedForReferenceFlags: u32,
    pub NonExistingFrameFlags: u16,
    pub frame_num: u16,
    pub log2_max_frame_num_minus4: u8,
    pub pic_order_cnt_type: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub delta_pic_order_always_zero_flag: u8,
    pub direct_8x8_inference_flag: u8,
    pub entropy_coding_mode_flag: u8,
    pub pic_order_present_flag: u8,
    pub num_slice_groups_minus1: u8,
    pub slice_group_map_type: u8,
    pub deblocking_filter_control_present_flag: u8,
    pub redundant_pic_cnt_present_flag: u8,
    pub Reserved8BitsB: u8,
    pub slice_group_change_rate_minus1: u16,
    pub SliceGroupMap: [u8; 810],
}
impl Default for DXVA_PicParams_H264 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_H264_0 {
    pub Anonymous: DXVA_PicParams_H264_0_0,
    pub wBitFields: u16,
}
impl Default for DXVA_PicParams_H264_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_H264_0_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_H264_MVC {
    pub wFrameWidthInMbsMinus1: u16,
    pub wFrameHeightInMbsMinus1: u16,
    pub CurrPic: DXVA_PicEntry_H264,
    pub num_ref_frames: u8,
    pub Anonymous: DXVA_PicParams_H264_MVC_0,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub Reserved16Bits: u16,
    pub StatusReportFeedbackNumber: u32,
    pub RefFrameList: [DXVA_PicEntry_H264; 16],
    pub CurrFieldOrderCnt: [i32; 2],
    pub FieldOrderCntList: [[i32; 2]; 16],
    pub pic_init_qs_minus26: i8,
    pub chroma_qp_index_offset: i8,
    pub second_chroma_qp_index_offset: i8,
    pub ContinuationFlag: u8,
    pub pic_init_qp_minus26: i8,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub Reserved8BitsA: u8,
    pub FrameNumList: [u16; 16],
    pub UsedForReferenceFlags: u32,
    pub NonExistingFrameFlags: u16,
    pub frame_num: u16,
    pub log2_max_frame_num_minus4: u8,
    pub pic_order_cnt_type: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub delta_pic_order_always_zero_flag: u8,
    pub direct_8x8_inference_flag: u8,
    pub entropy_coding_mode_flag: u8,
    pub pic_order_present_flag: u8,
    pub num_slice_groups_minus1: u8,
    pub slice_group_map_type: u8,
    pub deblocking_filter_control_present_flag: u8,
    pub redundant_pic_cnt_present_flag: u8,
    pub Reserved8BitsB: u8,
    pub slice_group_change_rate_minus1: u16,
    pub num_views_minus1: u8,
    pub view_id: [u16; 16],
    pub num_anchor_refs_l0: [u8; 16],
    pub anchor_ref_l0: [[u16; 16]; 16],
    pub num_anchor_refs_l1: [u8; 16],
    pub anchor_ref_l1: [[u16; 16]; 16],
    pub num_non_anchor_refs_l0: [u8; 16],
    pub non_anchor_ref_l0: [[u16; 16]; 16],
    pub num_non_anchor_refs_l1: [u8; 16],
    pub non_anchor_ref_l1: [[u16; 16]; 16],
    pub curr_view_id: u16,
    pub anchor_pic_flag: u8,
    pub inter_view_flag: u8,
    pub ViewIDList: [u16; 16],
}
impl Default for DXVA_PicParams_H264_MVC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_H264_MVC_0 {
    pub Anonymous: DXVA_PicParams_H264_MVC_0_0,
    pub wBitFields: u16,
}
impl Default for DXVA_PicParams_H264_MVC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_H264_MVC_0_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_HEVC {
    pub PicWidthInMinCbsY: u16,
    pub PicHeightInMinCbsY: u16,
    pub Anonymous: DXVA_PicParams_HEVC_0,
    pub CurrPic: DXVA_PicEntry_HEVC,
    pub sps_max_dec_pic_buffering_minus1: u8,
    pub log2_min_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_luma_coding_block_size: u8,
    pub log2_min_transform_block_size_minus2: u8,
    pub log2_diff_max_min_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub num_long_term_ref_pics_sps: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub init_qp_minus26: i8,
    pub ucNumDeltaPocsOfRefRpsIdx: u8,
    pub wNumBitsForShortTermRPSInSlice: u16,
    pub ReservedBits2: u16,
    pub Anonymous2: DXVA_PicParams_HEVC_1,
    pub Anonymous3: DXVA_PicParams_HEVC_2,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub num_tile_columns_minus1: u8,
    pub num_tile_rows_minus1: u8,
    pub column_width_minus1: [u16; 19],
    pub row_height_minus1: [u16; 21],
    pub diff_cu_qp_delta_depth: u8,
    pub pps_beta_offset_div2: i8,
    pub pps_tc_offset_div2: i8,
    pub log2_parallel_merge_level_minus2: u8,
    pub CurrPicOrderCntVal: i32,
    pub RefPicList: [DXVA_PicEntry_HEVC; 15],
    pub ReservedBits5: u8,
    pub PicOrderCntValList: [i32; 15],
    pub RefPicSetStCurrBefore: [u8; 8],
    pub RefPicSetStCurrAfter: [u8; 8],
    pub RefPicSetLtCurr: [u8; 8],
    pub ReservedBits6: u16,
    pub ReservedBits7: u16,
    pub StatusReportFeedbackNumber: u32,
}
impl Default for DXVA_PicParams_HEVC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_HEVC_0 {
    pub Anonymous: DXVA_PicParams_HEVC_0_0,
    pub wFormatAndSequenceInfoFlags: u16,
}
impl Default for DXVA_PicParams_HEVC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_HEVC_0_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_HEVC_1 {
    pub Anonymous: DXVA_PicParams_HEVC_1_0,
    pub dwCodingParamToolFlags: u32,
}
impl Default for DXVA_PicParams_HEVC_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_HEVC_1_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_HEVC_2 {
    pub Anonymous: DXVA_PicParams_HEVC_2_0,
    pub dwCodingSettingPicturePropertyFlags: u32,
}
impl Default for DXVA_PicParams_HEVC_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_HEVC_2_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_HEVC_RangeExt {
    pub params: DXVA_PicParams_HEVC,
    pub Anonymous: DXVA_PicParams_HEVC_RangeExt_0,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub log2_sao_offset_scale_luma: u8,
    pub log2_sao_offset_scale_chroma: u8,
    pub log2_max_transform_skip_block_size_minus2: u8,
    pub cb_qp_offset_list: [i8; 6],
    pub cr_qp_offset_list: [i8; 6],
    pub chroma_qp_offset_list_len_minus1: u8,
    pub ReservedBits9: u16,
}
impl Default for DXVA_PicParams_HEVC_RangeExt {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_HEVC_RangeExt_0 {
    pub Anonymous: DXVA_PicParams_HEVC_RangeExt_0_0,
    pub dwRangeExtensionFlags: u16,
}
impl Default for DXVA_PicParams_HEVC_RangeExt_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_HEVC_RangeExt_0_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_MJPEG {
    pub width: u32,
    pub height: u32,
    pub numComponents: u8,
    pub bitDepth: u8,
    pub reserved16Bits: u16,
    pub quantizationTableSelector: [u8; 4],
    pub scanOffset: [u32; 4],
    pub scanSize: [u32; 4],
    pub componentIdentifier: [u8; 4],
    pub restartInterval: u16,
    pub reserved16Bits2: u16,
    pub reserved32Bits: u32,
    pub statusReportFeedbackNumber: u32,
}
impl Default for DXVA_PicParams_MJPEG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_MPEG4_PART2 {
    pub short_video_header: u8,
    pub vop_coding_type: u8,
    pub vop_quant: u8,
    pub wDecodedPictureIndex: u16,
    pub wDeblockedPictureIndex: u16,
    pub wForwardRefPictureIndex: u16,
    pub wBackwardRefPictureIndex: u16,
    pub vop_time_increment_resolution: u16,
    pub TRB: [u32; 2],
    pub TRD: [u32; 2],
    pub Anonymous: DXVA_PicParams_MPEG4_PART2_0,
    pub profile_and_level_indication: u8,
    pub video_object_layer_verid: u8,
    pub vop_width: u16,
    pub vop_height: u16,
    pub Anonymous2: DXVA_PicParams_MPEG4_PART2_1,
    pub warping_mv: [[i16; 2]; 4],
    pub Anonymous3: DXVA_PicParams_MPEG4_PART2_2,
    pub StatusReportFeedbackNumber: u16,
    pub Reserved16BitsA: u16,
    pub Reserved16BitsB: u16,
}
impl Default for DXVA_PicParams_MPEG4_PART2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_MPEG4_PART2_0 {
    pub Anonymous: DXVA_PicParams_MPEG4_PART2_0_0,
    pub wPicFlagBitFields: u16,
}
impl Default for DXVA_PicParams_MPEG4_PART2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_MPEG4_PART2_0_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_MPEG4_PART2_1 {
    pub Anonymous: DXVA_PicParams_MPEG4_PART2_1_0,
    pub wSpriteBitFields: u16,
}
impl Default for DXVA_PicParams_MPEG4_PART2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_MPEG4_PART2_1_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_MPEG4_PART2_2 {
    pub Anonymous: DXVA_PicParams_MPEG4_PART2_2_0,
    pub wFcodeBitFields: u8,
}
impl Default for DXVA_PicParams_MPEG4_PART2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_MPEG4_PART2_2_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_VP8 {
    pub first_part_size: u32,
    pub width: u32,
    pub height: u32,
    pub CurrPic: DXVA_PicEntry_VPx,
    pub Anonymous: DXVA_PicParams_VP8_0,
    pub stVP8Segments: DXVA_segmentation_VP8,
    pub filter_type: u8,
    pub filter_level: u8,
    pub sharpness_level: u8,
    pub mode_ref_lf_delta_enabled: u8,
    pub mode_ref_lf_delta_update: u8,
    pub ref_lf_deltas: [i8; 4],
    pub mode_lf_deltas: [i8; 4],
    pub log2_nbr_of_dct_partitions: u8,
    pub base_qindex: u8,
    pub y1dc_delta_q: i8,
    pub y2dc_delta_q: i8,
    pub y2ac_delta_q: i8,
    pub uvdc_delta_q: i8,
    pub uvac_delta_q: i8,
    pub alt_fb_idx: DXVA_PicEntry_VPx,
    pub gld_fb_idx: DXVA_PicEntry_VPx,
    pub lst_fb_idx: DXVA_PicEntry_VPx,
    pub ref_frame_sign_bias_golden: u8,
    pub ref_frame_sign_bias_altref: u8,
    pub refresh_entropy_probs: u8,
    pub vp8_coef_update_probs: [[[[u8; 11]; 3]; 8]; 4],
    pub mb_no_coeff_skip: u8,
    pub prob_skip_false: u8,
    pub prob_intra: u8,
    pub prob_last: u8,
    pub prob_golden: u8,
    pub intra_16x16_prob: [u8; 4],
    pub intra_chroma_prob: [u8; 3],
    pub vp8_mv_update_probs: [[u8; 19]; 2],
    pub ReservedBits1: u16,
    pub ReservedBits2: u16,
    pub ReservedBits3: u16,
    pub StatusReportFeedbackNumber: u32,
}
impl Default for DXVA_PicParams_VP8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_VP8_0 {
    pub Anonymous: DXVA_PicParams_VP8_0_0,
    pub wFrameTagFlags: u8,
}
impl Default for DXVA_PicParams_VP8_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_VP8_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_PicParams_VP9 {
    pub CurrPic: DXVA_PicEntry_VPx,
    pub profile: u8,
    pub Anonymous: DXVA_PicParams_VP9_0,
    pub width: u32,
    pub height: u32,
    pub BitDepthMinus8Luma: u8,
    pub BitDepthMinus8Chroma: u8,
    pub interp_filter: u8,
    pub Reserved8Bits: u8,
    pub ref_frame_map: [DXVA_PicEntry_VPx; 8],
    pub ref_frame_coded_width: [u32; 8],
    pub ref_frame_coded_height: [u32; 8],
    pub frame_refs: [DXVA_PicEntry_VPx; 3],
    pub ref_frame_sign_bias: [i8; 4],
    pub filter_level: i8,
    pub sharpness_level: i8,
    pub Anonymous2: DXVA_PicParams_VP9_1,
    pub ref_deltas: [i8; 4],
    pub mode_deltas: [i8; 2],
    pub base_qindex: i16,
    pub y_dc_delta_q: i8,
    pub uv_dc_delta_q: i8,
    pub uv_ac_delta_q: i8,
    pub stVP9Segments: DXVA_segmentation_VP9,
    pub log2_tile_cols: u8,
    pub log2_tile_rows: u8,
    pub uncompressed_header_size_byte_aligned: u16,
    pub first_partition_size: u16,
    pub Reserved16Bits: u16,
    pub Reserved32Bits: u32,
    pub StatusReportFeedbackNumber: u32,
}
impl Default for DXVA_PicParams_VP9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_VP9_0 {
    pub Anonymous: DXVA_PicParams_VP9_0_0,
    pub wFormatAndPictureInfoFlags: u16,
}
impl Default for DXVA_PicParams_VP9_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_VP9_0_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_PicParams_VP9_1 {
    pub Anonymous: DXVA_PicParams_VP9_1_0,
    pub wControlInfoFlags: u8,
}
impl Default for DXVA_PicParams_VP9_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicParams_VP9_1_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PicResample {
    pub wPicResampleSourcePicIndex: u16,
    pub wPicResampleDestPicIndex: u16,
    pub wPicResampleRcontrol: u16,
    pub bPicResampleExtrapWidth: u8,
    pub bPicResampleExtrapHeight: u8,
    pub dwPicResampleSourceWidth: u32,
    pub dwPicResampleSourceHeight: u32,
    pub dwPicResampleDestWidth: u32,
    pub dwPicResampleDestHeight: u32,
    pub dwPicResampleFullDestWidth: u32,
    pub dwPicResampleFullDestHeight: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_PictureParameters {
    pub wDecodedPictureIndex: u16,
    pub wDeblockedPictureIndex: u16,
    pub wForwardRefPictureIndex: u16,
    pub wBackwardRefPictureIndex: u16,
    pub wPicWidthInMBminus1: u16,
    pub wPicHeightInMBminus1: u16,
    pub bMacroblockWidthMinus1: u8,
    pub bMacroblockHeightMinus1: u8,
    pub bBlockWidthMinus1: u8,
    pub bBlockHeightMinus1: u8,
    pub bBPPminus1: u8,
    pub bPicStructure: u8,
    pub bSecondField: u8,
    pub bPicIntra: u8,
    pub bPicBackwardPrediction: u8,
    pub bBidirectionalAveragingMode: u8,
    pub bMVprecisionAndChromaRelation: u8,
    pub bChromaFormat: u8,
    pub bPicScanFixed: u8,
    pub bPicScanMethod: u8,
    pub bPicReadbackRequests: u8,
    pub bRcontrol: u8,
    pub bPicSpatialResid8: u8,
    pub bPicOverflowBlocks: u8,
    pub bPicExtrapolation: u8,
    pub bPicDeblocked: u8,
    pub bPicDeblockConfined: u8,
    pub bPic4MVallowed: u8,
    pub bPicOBMC: u8,
    pub bPicBinPB: u8,
    pub bMV_RPS: u8,
    pub bReservedBits: u8,
    pub wBitstreamFcodes: u16,
    pub wBitstreamPCEelements: u16,
    pub bBitstreamConcealmentNeed: u8,
    pub bBitstreamConcealmentMethod: u8,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_ProcAmpControlBlt {
    pub Size: u32,
    pub DstRect: super::RECT,
    pub SrcRect: super::RECT,
    pub Alpha: f32,
    pub Brightness: f32,
    pub Contrast: f32,
    pub Hue: f32,
    pub Saturation: f32,
}
pub const DXVA_ProcAmpControlBltFnCode: i32 = 1;
#[repr(C)]
#[cfg(feature = "d3d9")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_ProcAmpControlCaps {
    pub Size: u32,
    pub InputPool: u32,
    pub d3dOutputFormat: super::D3DFORMAT,
    pub ProcAmpControlProps: u32,
    pub VideoProcessingCaps: u32,
}
pub const DXVA_ProcAmpControlDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9f200913_2ffd_4056_9f1e_e1b508f22dcf);
pub type DXVA_ProcAmpControlProp = i32;
pub const DXVA_ProcAmpControlQueryCapsFnCode: i32 = 3;
#[repr(C)]
#[cfg(feature = "d3d9")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_ProcAmpControlQueryRange {
    pub Size: u32,
    pub ProcAmpControlProp: DXVA_ProcAmpControlProp,
    pub VideoDesc: DXVA_VideoDesc,
}
pub const DXVA_ProcAmpControlQueryRangeFnCode: i32 = 4;
pub const DXVA_ProcAmp_Brightness: DXVA_ProcAmpControlProp = 1;
pub const DXVA_ProcAmp_Contrast: DXVA_ProcAmpControlProp = 2;
pub const DXVA_ProcAmp_Hue: DXVA_ProcAmpControlProp = 4;
pub const DXVA_ProcAmp_None: DXVA_ProcAmpControlProp = 0;
pub const DXVA_ProcAmp_Saturation: DXVA_ProcAmpControlProp = 8;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_QMatrix_MJPEG {
    pub quantvals: [[u16; 64]; 4],
}
impl Default for DXVA_QMatrix_MJPEG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_LOCK_FALSE_PLUS: i32 = 16777215;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_LOCK_OK_COPY: i32 = 16777212;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_FALSE_PLUS: i32 = 16777211;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_OK_COPY: i32 = 16777208;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_OK_PLUS: i32 = 16777209;
pub const DXVA_QUERYORREPLYFUNCFLAG_DECODER_LOCK_QUERY: i32 = 16777205;
pub const DXVA_QUERYORREPLYFUNCFLAG_DECODER_PROBE_QUERY: i32 = 16777201;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_QmatrixData {
    pub bNewQmatrix: [u8; 4],
    pub Qmatrix: [[u16; 64]; 4],
}
impl Default for DXVA_QmatrixData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_Qmatrix_APV {
    pub NumComponents: u8,
    pub qmatrix_reserved_zero_32bits: u32,
    pub QMatrix: [[[u8; 8]; 8]; 4],
    pub qmatrix_reserved_zero_64bits: u64,
}
impl Default for DXVA_Qmatrix_APV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_Qmatrix_H264 {
    pub bScalingLists4x4: [[u8; 16]; 6],
    pub bScalingLists8x8: [[u8; 64]; 2],
}
impl Default for DXVA_Qmatrix_H264 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_Qmatrix_HEVC {
    pub ucScalingLists0: [[u8; 16]; 6],
    pub ucScalingLists1: [[u8; 64]; 6],
    pub ucScalingLists2: [[u8; 64]; 6],
    pub ucScalingLists3: [[u8; 64]; 2],
    pub ucScalingListDCCoefSizeID2: [u8; 6],
    pub ucScalingListDCCoefSizeID3: [u8; 2],
}
impl Default for DXVA_Qmatrix_HEVC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_READ_BACK_BUFFER: i32 = 15;
pub const DXVA_RESIDUAL_DIFFERENCE_BUFFER: i32 = 3;
pub const DXVA_RESTRICTED_MODE_H261_A: i32 = 1;
pub const DXVA_RESTRICTED_MODE_H261_B: i32 = 2;
pub const DXVA_RESTRICTED_MODE_H263_A: i32 = 3;
pub const DXVA_RESTRICTED_MODE_H263_B: i32 = 4;
pub const DXVA_RESTRICTED_MODE_H263_C: i32 = 5;
pub const DXVA_RESTRICTED_MODE_H263_D: i32 = 6;
pub const DXVA_RESTRICTED_MODE_H263_E: i32 = 7;
pub const DXVA_RESTRICTED_MODE_H263_F: i32 = 8;
pub const DXVA_RESTRICTED_MODE_H264_A: i32 = 100;
pub const DXVA_RESTRICTED_MODE_H264_B: i32 = 101;
pub const DXVA_RESTRICTED_MODE_H264_C: i32 = 102;
pub const DXVA_RESTRICTED_MODE_H264_D: i32 = 103;
pub const DXVA_RESTRICTED_MODE_H264_E: i32 = 104;
pub const DXVA_RESTRICTED_MODE_H264_F: i32 = 105;
pub const DXVA_RESTRICTED_MODE_H264_IDCT_FGT: i32 = 103;
pub const DXVA_RESTRICTED_MODE_H264_IDCT_NOFGT: i32 = 102;
pub const DXVA_RESTRICTED_MODE_H264_MOCOMP_FGT: i32 = 101;
pub const DXVA_RESTRICTED_MODE_H264_MOCOMP_NOFGT: i32 = 100;
pub const DXVA_RESTRICTED_MODE_H264_VLD_FGT: i32 = 105;
pub const DXVA_RESTRICTED_MODE_H264_VLD_MULTIVIEW_NOFGT: i32 = 115;
pub const DXVA_RESTRICTED_MODE_H264_VLD_NOFGT: i32 = 104;
pub const DXVA_RESTRICTED_MODE_H264_VLD_STEREO_NOFGT: i32 = 114;
pub const DXVA_RESTRICTED_MODE_H264_VLD_STEREO_PROGRESSIVE_NOFGT: i32 = 113;
pub const DXVA_RESTRICTED_MODE_H264_VLD_WITHFMOASO_NOFGT: i32 = 112;
pub const DXVA_RESTRICTED_MODE_MPEG1_A: i32 = 9;
pub const DXVA_RESTRICTED_MODE_MPEG1_VLD: i32 = 16;
pub const DXVA_RESTRICTED_MODE_MPEG2_A: i32 = 10;
pub const DXVA_RESTRICTED_MODE_MPEG2_B: i32 = 11;
pub const DXVA_RESTRICTED_MODE_MPEG2_C: i32 = 12;
pub const DXVA_RESTRICTED_MODE_MPEG2_D: i32 = 13;
pub const DXVA_RESTRICTED_MODE_MPEG2and1_VLD: i32 = 17;
pub const DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_ADV_SIMPLE_GMC: i32 = 178;
pub const DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_ADV_SIMPLE_NOGMC: i32 = 177;
pub const DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_SIMPLE: i32 = 176;
pub const DXVA_RESTRICTED_MODE_UNRESTRICTED: i32 = 65535;
pub const DXVA_RESTRICTED_MODE_VC1_A: i32 = 160;
pub const DXVA_RESTRICTED_MODE_VC1_B: i32 = 161;
pub const DXVA_RESTRICTED_MODE_VC1_C: i32 = 162;
pub const DXVA_RESTRICTED_MODE_VC1_D: i32 = 163;
pub const DXVA_RESTRICTED_MODE_VC1_D2010: i32 = 164;
pub const DXVA_RESTRICTED_MODE_VC1_IDCT: i32 = 162;
pub const DXVA_RESTRICTED_MODE_VC1_MOCOMP: i32 = 161;
pub const DXVA_RESTRICTED_MODE_VC1_POSTPROC: i32 = 160;
pub const DXVA_RESTRICTED_MODE_VC1_VLD: i32 = 163;
pub const DXVA_RESTRICTED_MODE_WMV8_A: i32 = 128;
pub const DXVA_RESTRICTED_MODE_WMV8_B: i32 = 129;
pub const DXVA_RESTRICTED_MODE_WMV8_MOCOMP: i32 = 129;
pub const DXVA_RESTRICTED_MODE_WMV8_POSTPROC: i32 = 128;
pub const DXVA_RESTRICTED_MODE_WMV9_A: i32 = 144;
pub const DXVA_RESTRICTED_MODE_WMV9_B: i32 = 145;
pub const DXVA_RESTRICTED_MODE_WMV9_C: i32 = 148;
pub const DXVA_RESTRICTED_MODE_WMV9_IDCT: i32 = 148;
pub const DXVA_RESTRICTED_MODE_WMV9_MOCOMP: i32 = 145;
pub const DXVA_RESTRICTED_MODE_WMV9_POSTPROC: i32 = 144;
pub const DXVA_SCAN_METHOD_ALTERNATE_HORIZONTAL: i32 = 2;
pub const DXVA_SCAN_METHOD_ALTERNATE_VERTICAL: i32 = 1;
pub const DXVA_SCAN_METHOD_ARBITRARY: i32 = 3;
pub const DXVA_SCAN_METHOD_ZIG_ZAG: i32 = 0;
pub const DXVA_SLICE_CONTROL_BUFFER: i32 = 6;
pub const DXVA_STATUS_REPORTING_FUNCTION: i32 = 7;
pub type DXVA_Sample16 = [i16; 64];
pub type DXVA_Sample8 = [i8; 64];
pub const DXVA_SampleFieldInterleavedEvenFirst: DXVA_SampleFormat = 3;
pub const DXVA_SampleFieldInterleavedOddFirst: DXVA_SampleFormat = 4;
pub const DXVA_SampleFieldSingleEven: DXVA_SampleFormat = 5;
pub const DXVA_SampleFieldSingleOdd: DXVA_SampleFormat = 6;
pub const DXVA_SampleFlag_ColorData_Changed: DXVA_SampleFlags = 8;
pub const DXVA_SampleFlag_DstRect_Changed: DXVA_SampleFlags = 4;
pub const DXVA_SampleFlag_Palette_Changed: DXVA_SampleFlags = 1;
pub const DXVA_SampleFlag_SrcRect_Changed: DXVA_SampleFlags = 2;
pub type DXVA_SampleFlags = i32;
pub const DXVA_SampleFlagsMask: DXVA_SampleFlags = 15;
pub type DXVA_SampleFormat = i32;
pub const DXVA_SampleFormatMask: DXVA_SampleFormat = 255;
pub const DXVA_SamplePreviousFrame: DXVA_SampleFormat = 1;
pub const DXVA_SampleProgressiveFrame: DXVA_SampleFormat = 2;
pub const DXVA_SampleSubStream: DXVA_SampleFormat = 7;
pub const DXVA_SampleUnknown: DXVA_SampleFormat = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_SliceInfo {
    pub wHorizontalPosition: u16,
    pub wVerticalPosition: u16,
    pub dwSliceBitsInBuffer: u32,
    pub dwSliceDataLocation: u32,
    pub bStartCodeBitOffset: u8,
    pub bReservedBits: u8,
    pub wMBbitOffset: u16,
    pub wNumberMBsInSlice: u16,
    pub wQuantizerScaleCode: u16,
    pub wBadSliceChopping: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_Slice_H264_Long {
    pub BSNALunitDataLocation: u32,
    pub SliceBytesInBuffer: u32,
    pub wBadSliceChopping: u16,
    pub first_mb_in_slice: u16,
    pub NumMbsForSlice: u16,
    pub BitOffsetToSliceData: u16,
    pub slice_type: u8,
    pub luma_log2_weight_denom: u8,
    pub chroma_log2_weight_denom: u8,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub slice_alpha_c0_offset_div2: i8,
    pub slice_beta_offset_div2: i8,
    pub Reserved8Bits: u8,
    pub RefPicList: [[DXVA_PicEntry_H264; 32]; 2],
    pub Weights: [[[[i16; 2]; 3]; 32]; 2],
    pub slice_qs_delta: i8,
    pub slice_qp_delta: i8,
    pub redundant_pic_cnt: u8,
    pub direct_spatial_mv_pred_flag: u8,
    pub cabac_init_idc: u8,
    pub disable_deblocking_filter_idc: u8,
    pub slice_id: u16,
}
impl Default for DXVA_Slice_H264_Long {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_Slice_H264_Short {
    pub BSNALunitDataLocation: u32,
    pub SliceBytesInBuffer: u32,
    pub wBadSliceChopping: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_Slice_HEVC_Short {
    pub BSNALunitDataLocation: u32,
    pub SliceBytesInBuffer: u32,
    pub wBadSliceChopping: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_Slice_VPx_Short {
    pub BSNALunitDataLocation: u32,
    pub SliceBytesInBuffer: u32,
    pub wBadSliceChopping: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_Status_AV1 {
    pub StatusReportFeedbackNumber: u32,
    pub CurrPic: DXVA_PicEntry_AV1,
    pub BufType: u8,
    pub Status: u8,
    pub Reserved8Bits: u8,
    pub NumMbsAffected: u16,
}
impl Default for DXVA_Status_AV1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_Status_H264 {
    pub StatusReportFeedbackNumber: u32,
    pub CurrPic: DXVA_PicEntry_H264,
    pub field_pic_flag: u8,
    pub bDXVA_Func: u8,
    pub bBufType: u8,
    pub bStatus: u8,
    pub bReserved8Bits: u8,
    pub wNumMbsAffected: u16,
}
impl Default for DXVA_Status_H264 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_Status_HEVC {
    pub StatusReportFeedbackNumber: u16,
    pub CurrPic: DXVA_PicEntry_HEVC,
    pub bBufType: u8,
    pub bStatus: u8,
    pub bReserved8Bits: u8,
    pub wNumMbsAffected: u16,
}
impl Default for DXVA_Status_HEVC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_Status_VC1 {
    pub StatusReportFeedbackNumber: u16,
    pub wDecodedPictureIndex: u16,
    pub wDeblockedPictureIndex: u16,
    pub bPicStructure: u8,
    pub bBufType: u8,
    pub bStatus: u8,
    pub bReserved8Bits: u8,
    pub wNumMbsAffected: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_Status_VPx {
    pub StatusReportFeedbackNumber: u32,
    pub CurrPic: DXVA_PicEntry_VPx,
    pub bBufType: u8,
    pub bStatus: u8,
    pub bReserved8Bits: u8,
    pub wNumMbsAffected: u16,
}
impl Default for DXVA_Status_VPx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_TCoef4Group {
    pub TCoefIDX: [u8; 4],
    pub TCoefValue: [i16; 4],
}
impl Default for DXVA_TCoef4Group {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_TCoefSingle {
    pub wIndexWithEOB: u16,
    pub TCoefValue: i16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_Tile_APV {
    pub tile_data_offset: u32,
    pub tile_total_size: u32,
    pub tile_data_reserved_zero_32bits: u32,
    pub tile_index: u16,
    pub tile_header_reserved_zero_32bits: u32,
    pub tile_header_size: u16,
    pub tile_component_data_size: [u32; 4],
    pub tile_component_qp: [u8; 4],
    pub tile_component_qp_zero_32bits: u32,
}
impl Default for DXVA_Tile_APV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DXVA_Tile_AV1 {
    pub DataOffset: u32,
    pub DataSize: u32,
    pub row: u16,
    pub column: u16,
    pub Reserved16Bits: u16,
    pub anchor_frame: u8,
    pub Reserved8Bits: u8,
}
pub const DXVA_USUAL_BLOCK_HEIGHT: i32 = 8;
pub const DXVA_USUAL_BLOCK_SIZE: i32 = 64;
pub const DXVA_USUAL_BLOCK_WIDTH: i32 = 8;
pub type DXVA_VideoChromaSubsampling = i32;
pub const DXVA_VideoChromaSubsamplingMask: DXVA_VideoChromaSubsampling = 3840;
pub const DXVA_VideoChromaSubsamplingShift: DXVA_VideoChromaSubsampling = 8;
pub const DXVA_VideoChromaSubsampling_Cosited: DXVA_VideoChromaSubsampling = 7;
pub const DXVA_VideoChromaSubsampling_DV_PAL: DXVA_VideoChromaSubsampling = 6;
pub const DXVA_VideoChromaSubsampling_Horizontally_Cosited: DXVA_VideoChromaSubsampling = 4;
pub const DXVA_VideoChromaSubsampling_MPEG1: DXVA_VideoChromaSubsampling = 1;
pub const DXVA_VideoChromaSubsampling_MPEG2: DXVA_VideoChromaSubsampling = 5;
pub const DXVA_VideoChromaSubsampling_ProgressiveChroma: DXVA_VideoChromaSubsampling = 8;
pub const DXVA_VideoChromaSubsampling_Unknown: DXVA_VideoChromaSubsampling = 0;
pub const DXVA_VideoChromaSubsampling_Vertically_AlignedChromaPlanes: DXVA_VideoChromaSubsampling = 1;
pub const DXVA_VideoChromaSubsampling_Vertically_Cosited: DXVA_VideoChromaSubsampling = 2;
#[repr(C)]
#[cfg(feature = "d3d9")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_VideoDesc {
    pub Size: u32,
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub SampleFormat: u32,
    pub d3dFormat: super::D3DFORMAT,
    pub InputSampleFreq: DXVA_Frequency,
    pub OutputFrameFreq: DXVA_Frequency,
}
pub type DXVA_VideoLighting = i32;
pub const DXVA_VideoLightingMask: DXVA_VideoLighting = 3932160;
pub const DXVA_VideoLightingShift: DXVA_VideoLighting = 18;
pub const DXVA_VideoLighting_Unknown: DXVA_VideoLighting = 0;
pub const DXVA_VideoLighting_bright: DXVA_VideoLighting = 1;
pub const DXVA_VideoLighting_dark: DXVA_VideoLighting = 4;
pub const DXVA_VideoLighting_dim: DXVA_VideoLighting = 3;
pub const DXVA_VideoLighting_office: DXVA_VideoLighting = 2;
pub type DXVA_VideoPrimaries = i32;
pub const DXVA_VideoPrimariesMask: DXVA_VideoPrimaries = 130023424;
pub const DXVA_VideoPrimariesShift: DXVA_VideoPrimaries = 22;
pub const DXVA_VideoPrimaries_BT470_2_SysBG: DXVA_VideoPrimaries = 4;
pub const DXVA_VideoPrimaries_BT470_2_SysM: DXVA_VideoPrimaries = 3;
pub const DXVA_VideoPrimaries_BT709: DXVA_VideoPrimaries = 2;
pub const DXVA_VideoPrimaries_EBU3213: DXVA_VideoPrimaries = 7;
pub const DXVA_VideoPrimaries_SMPTE170M: DXVA_VideoPrimaries = 5;
pub const DXVA_VideoPrimaries_SMPTE240M: DXVA_VideoPrimaries = 6;
pub const DXVA_VideoPrimaries_SMPTE_C: DXVA_VideoPrimaries = 8;
pub const DXVA_VideoPrimaries_Unknown: DXVA_VideoPrimaries = 0;
pub const DXVA_VideoPrimaries_reserved: DXVA_VideoPrimaries = 1;
pub type DXVA_VideoProcessCaps = i32;
pub const DXVA_VideoProcess_AlphaBlend: DXVA_VideoProcessCaps = 8;
pub const DXVA_VideoProcess_AlphaBlendExtended: DXVA_VideoProcessCaps = 256;
pub const DXVA_VideoProcess_None: DXVA_VideoProcessCaps = 0;
pub const DXVA_VideoProcess_StretchX: DXVA_VideoProcessCaps = 2;
pub const DXVA_VideoProcess_StretchY: DXVA_VideoProcessCaps = 4;
pub const DXVA_VideoProcess_SubRects: DXVA_VideoProcessCaps = 16;
pub const DXVA_VideoProcess_SubStreams: DXVA_VideoProcessCaps = 32;
pub const DXVA_VideoProcess_SubStreamsExtended: DXVA_VideoProcessCaps = 64;
pub const DXVA_VideoProcess_YUV2RGB: DXVA_VideoProcessCaps = 1;
pub const DXVA_VideoProcess_YUV2RGBExtended: DXVA_VideoProcessCaps = 128;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_VideoPropertyRange {
    pub MinValue: f32,
    pub MaxValue: f32,
    pub DefaultValue: f32,
    pub StepSize: f32,
}
#[repr(C)]
#[cfg(feature = "mediaobj")]
#[derive(Clone, Copy, Default)]
pub struct DXVA_VideoSample {
    pub rtStart: super::REFERENCE_TIME,
    pub rtEnd: super::REFERENCE_TIME,
    pub SampleFormat: DXVA_SampleFormat,
    pub lpDDSSrcSurface: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct DXVA_VideoSample2 {
    pub rtStart: super::REFERENCE_TIME,
    pub rtEnd: super::REFERENCE_TIME,
    pub SampleFormat: u32,
    pub SampleFlags: u32,
    pub lpDDSSrcSurface: *mut core::ffi::c_void,
    pub rcSrc: super::RECT,
    pub rcDst: super::RECT,
    pub Palette: [DXVA_AYUVsample2; 16],
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for DXVA_VideoSample2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct DXVA_VideoSample2 {
    pub Size: u32,
    pub Reserved: u32,
    pub rtStart: super::REFERENCE_TIME,
    pub rtEnd: super::REFERENCE_TIME,
    pub SampleFormat: u32,
    pub SampleFlags: u32,
    pub lpDDSSrcSurface: *mut core::ffi::c_void,
    pub rcSrc: super::RECT,
    pub rcDst: super::RECT,
    pub Palette: [DXVA_AYUVsample2; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for DXVA_VideoSample2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct DXVA_VideoSample32 {
    pub rtStart: super::REFERENCE_TIME,
    pub rtEnd: super::REFERENCE_TIME,
    pub SampleFormat: u32,
    pub SampleFlags: u32,
    pub lpDDSSrcSurface: u32,
    pub rcSrc: super::RECT,
    pub rcDst: super::RECT,
    pub Palette: [DXVA_AYUVsample2; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "mediaobj", feature = "windef"))]
impl Default for DXVA_VideoSample32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA_VideoTransFuncMask: DXVA_VideoTransferFunction = -134217728;
pub const DXVA_VideoTransFuncShift: DXVA_VideoTransferFunction = 27;
pub const DXVA_VideoTransFunc_10: DXVA_VideoTransferFunction = 1;
pub const DXVA_VideoTransFunc_18: DXVA_VideoTransferFunction = 2;
pub const DXVA_VideoTransFunc_20: DXVA_VideoTransferFunction = 3;
pub const DXVA_VideoTransFunc_22: DXVA_VideoTransferFunction = 4;
pub const DXVA_VideoTransFunc_22_240M: DXVA_VideoTransferFunction = 6;
pub const DXVA_VideoTransFunc_22_709: DXVA_VideoTransferFunction = 5;
pub const DXVA_VideoTransFunc_22_8bit_sRGB: DXVA_VideoTransferFunction = 7;
pub const DXVA_VideoTransFunc_28: DXVA_VideoTransferFunction = 8;
pub const DXVA_VideoTransFunc_Unknown: DXVA_VideoTransferFunction = 0;
pub type DXVA_VideoTransferFunction = i32;
pub type DXVA_VideoTransferMatrix = i32;
pub const DXVA_VideoTransferMatrixMask: DXVA_VideoTransferMatrix = 229376;
pub const DXVA_VideoTransferMatrixShift: DXVA_VideoTransferMatrix = 15;
pub const DXVA_VideoTransferMatrix_BT601: DXVA_VideoTransferMatrix = 2;
pub const DXVA_VideoTransferMatrix_BT709: DXVA_VideoTransferMatrix = 1;
pub const DXVA_VideoTransferMatrix_SMPTE240M: DXVA_VideoTransferMatrix = 3;
pub const DXVA_VideoTransferMatrix_Unknown: DXVA_VideoTransferMatrix = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA_segmentation_VP8 {
    pub Anonymous: DXVA_segmentation_VP8_0,
    pub segment_feature_data: [[i8; 4]; 2],
    pub mb_segment_tree_probs: [u8; 3],
}
impl Default for DXVA_segmentation_VP8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_segmentation_VP8_0 {
    pub Anonymous: DXVA_segmentation_VP8_0_0,
    pub wSegmentFlags: u8,
}
impl Default for DXVA_segmentation_VP8_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_segmentation_VP8_0_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DXVA_segmentation_VP9 {
    pub Anonymous: DXVA_segmentation_VP9_0,
    pub tree_probs: [u8; 7],
    pub pred_probs: [u8; 3],
    pub feature_data: [[i16; 4]; 8],
    pub feature_mask: [u8; 8],
}
impl Default for DXVA_segmentation_VP9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA_segmentation_VP9_0 {
    pub Anonymous: DXVA_segmentation_VP9_0_0,
    pub wSegmentInfoFlags: u8,
}
impl Default for DXVA_segmentation_VP9_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA_segmentation_VP9_0_0 {
    pub _bitfield: u8,
}
pub type LPDXVA_AI44sample = *mut u8;
pub type LPDXVA_AYUVsample2 = *mut DXVA_AYUVsample2;
#[cfg(feature = "windef")]
pub type LPDXVA_BlendCombination = *mut DXVA_BlendCombination;
pub type LPDXVA_BufferDescription = *mut DXVA_BufferDescription;
pub type LPDXVA_COPPCommand = *mut DXVA_COPPCommand;
pub type LPDXVA_COPPSignature = *mut DXVA_COPPSignature;
pub type LPDXVA_COPPStatusInput = *mut DXVA_COPPStatusInput;
pub type LPDXVA_COPPStatusOutput = *mut DXVA_COPPStatusOutput;
pub type LPDXVA_ConfigAlphaCombine = *mut DXVA_ConfigAlphaCombine;
pub type LPDXVA_ConfigAlphaLoad = *mut DXVA_ConfigAlphaLoad;
pub type LPDXVA_ConfigPictureDecode = *mut DXVA_ConfigPictureDecode;
pub type LPDXVA_ConfigQueryOrReplyFunc = *mut u32;
pub type LPDXVA_ConnectMode = *mut DXVA_ConnectMode;
pub type LPDXVA_DCCMD = *mut u16;
pub type LPDXVA_DPXD = *mut u8;
pub type LPDXVA_DeblockIndexAB_H264 = *mut DXVA_DeblockIndexAB_H264;
pub type LPDXVA_Deblock_H264 = *mut DXVA_Deblock_H264;
pub type LPDXVA_DeblockingEdgeControl = *mut DXVA_DeblockingEdgeControl;
#[cfg(feature = "d3d9")]
pub type LPDXVA_DeinterlaceCaps = *mut DXVA_DeinterlaceCaps;
pub type LPDXVA_EncryptProtocolFunc = *mut u32;
pub type LPDXVA_EncryptProtocolHeader = *mut DXVA_EncryptProtocolHeader;
pub type LPDXVA_FilmGrainChar_H264 = *mut DXVA_FilmGrainChar_H264;
#[cfg(feature = "windef")]
pub type LPDXVA_Highlight = *mut DXVA_Highlight;
pub type LPDXVA_HuffmanTable_MJPEG = *mut DXVA_HuffmanTable_MJPEG;
pub type LPDXVA_IA44sample = *mut u8;
pub type LPDXVA_MBctrl_H264 = *mut DXVA_MBctrl_H264;
pub type LPDXVA_MBctrl_I_HostResidDiff_1 = *mut DXVA_MBctrl_I_HostResidDiff_1;
pub type LPDXVA_MBctrl_I_OffHostIDCT_1 = *mut DXVA_MBctrl_I_OffHostIDCT_1;
pub type LPDXVA_MBctrl_P_HostResidDiff_1 = *mut DXVA_MBctrl_P_HostResidDiff_1;
pub type LPDXVA_MBctrl_P_OffHostIDCT_1 = *mut DXVA_MBctrl_P_OffHostIDCT_1;
pub type LPDXVA_MVvalue = *mut DXVA_MVvalue;
pub type LPDXVA_PicEntry_APV = *mut DXVA_PicEntry_APV;
pub type LPDXVA_PicEntry_AV1 = *mut DXVA_PicEntry_AV1;
pub type LPDXVA_PicEntry_H264 = *mut DXVA_PicEntry_H264;
pub type LPDXVA_PicEntry_HEVC = *mut DXVA_PicEntry_HEVC;
pub type LPDXVA_PicEntry_VPx = *mut DXVA_PicEntry_VPx;
pub type LPDXVA_PicParams_APV = *mut DXVA_PicParams_APV;
pub type LPDXVA_PicParams_AV1 = *mut DXVA_PicParams_AV1;
pub type LPDXVA_PicParams_H264 = *mut DXVA_PicParams_H264;
pub type LPDXVA_PicParams_H264_MVC = *mut DXVA_PicParams_H264_MVC;
pub type LPDXVA_PicParams_HEVC = *mut DXVA_PicParams_HEVC;
pub type LPDXVA_PicParams_HEVC_RangeExt = *mut DXVA_PicParams_HEVC_RangeExt;
pub type LPDXVA_PicParams_MJPEG = *mut DXVA_PicParams_MJPEG;
pub type LPDXVA_PicParams_MPEG4_PART2 = *mut DXVA_PicParams_MPEG4_PART2;
pub type LPDXVA_PicParams_VP8 = *mut DXVA_PicParams_VP8;
pub type LPDXVA_PicParams_VP9 = *mut DXVA_PicParams_VP9;
pub type LPDXVA_PicResample = *mut DXVA_PicResample;
pub type LPDXVA_PictureParameters = *mut DXVA_PictureParameters;
#[cfg(feature = "d3d9")]
pub type LPDXVA_ProcAmpControlCaps = *mut DXVA_ProcAmpControlCaps;
#[cfg(feature = "d3d9")]
pub type LPDXVA_ProcAmpControlQueryRange = *mut DXVA_ProcAmpControlQueryRange;
pub type LPDXVA_QMatrix_MJPEG = *mut DXVA_QMatrix_MJPEG;
pub type LPDXVA_QmatrixData = *mut DXVA_QmatrixData;
pub type LPDXVA_Qmatrix_APV = *mut DXVA_Qmatrix_APV;
pub type LPDXVA_Qmatrix_H264 = *mut DXVA_Qmatrix_H264;
pub type LPDXVA_Qmatrix_HEVC = *mut DXVA_Qmatrix_HEVC;
pub type LPDXVA_SliceInfo = *mut DXVA_SliceInfo;
pub type LPDXVA_Slice_H264_Long = *mut DXVA_Slice_H264_Long;
pub type LPDXVA_Slice_H264_Short = *mut DXVA_Slice_H264_Short;
pub type LPDXVA_Slice_HEVC_Short = *mut DXVA_Slice_HEVC_Short;
pub type LPDXVA_Slice_VPx_Short = *mut DXVA_Slice_VPx_Short;
pub type LPDXVA_Status_AV1 = *mut DXVA_Status_AV1;
pub type LPDXVA_Status_H264 = *mut DXVA_Status_H264;
pub type LPDXVA_Status_HEVC = *mut DXVA_Status_HEVC;
pub type LPDXVA_Status_VC1 = *mut DXVA_Status_VC1;
pub type LPDXVA_Status_VPx = *mut DXVA_Status_VPx;
pub type LPDXVA_TCoef4Group = *mut DXVA_TCoef4Group;
pub type LPDXVA_TCoefSingle = *mut DXVA_TCoefSingle;
pub type LPDXVA_Tile_APV = *mut DXVA_Tile_APV;
pub type LPDXVA_Tile_AV1 = *mut DXVA_Tile_AV1;
#[cfg(feature = "d3d9")]
pub type LPDXVA_VideoDesc = *mut DXVA_VideoDesc;
pub type LPDXVA_VideoPropertyRange = *mut DXVA_VideoPropertyRange;
#[cfg(feature = "mediaobj")]
pub type LPDXVA_VideoSample = *mut DXVA_VideoSample;
#[cfg(all(feature = "mediaobj", feature = "windef"))]
pub type LPDXVA_VideoSample2 = *mut DXVA_VideoSample2;
pub const MAX_DEINTERLACE_DEVICE_GUIDS: i32 = 32;
pub const MAX_DEINTERLACE_SURFACES: i32 = 32;
