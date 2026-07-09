windows_link::link!("dxva2.dll" "system" fn DXVA2CreateDirect3DDeviceManager9(presettoken : *mut u32, ppdevicemanager : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_d3d9")]
windows_link::link!("dxva2.dll" "system" fn DXVA2CreateVideoService(pdd : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppservice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA2_AES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA2_AYUVSample16 {
    pub Cr: u16,
    pub Cb: u16,
    pub Y: u16,
    pub Alpha: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA2_AYUVSample8 {
    pub Cr: u8,
    pub Cb: u8,
    pub Y: u8,
    pub Alpha: u8,
}
pub const DXVA2_BitStreamDateBufferType: i32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA2_ConfigPictureDecode {
    pub guidConfigBitstreamEncryption: windows_sys::core::GUID,
    pub guidConfigMBcontrolEncryption: windows_sys::core::GUID,
    pub guidConfigResidDiffEncryption: windows_sys::core::GUID,
    pub ConfigBitstreamRaw: u32,
    pub ConfigMBcontrolRasterOrder: u32,
    pub ConfigResidDiffHost: u32,
    pub ConfigSpatialResid8: u32,
    pub ConfigResid8Subtraction: u32,
    pub ConfigSpatialHost8or9Clipping: u32,
    pub ConfigSpatialResidInterleaved: u32,
    pub ConfigIntraResidUnsigned: u32,
    pub ConfigResidDiffAccelerator: u32,
    pub ConfigHostInverseScan: u32,
    pub ConfigSpecificIDCT: u32,
    pub Config4GroupedCoefs: u32,
    pub ConfigMinRenderTargetBuffCount: u16,
    pub ConfigDecoderSpecific: u16,
}
pub const DXVA2_DECODE_GET_DRIVER_HANDLE: u32 = 1829;
pub const DXVA2_DECODE_SPECIFY_ENCRYPTED_BLOCKS: u32 = 1828;
pub const DXVA2_DeblockingControlBufferType: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA2_DecodeBufferDesc {
    pub CompressedBufferType: u32,
    pub BufferIndex: u32,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub FirstMBaddress: u32,
    pub NumMBsInBuffer: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub ReservedBits: u32,
    pub pvPVPState: *mut core::ffi::c_void,
}
impl Default for DXVA2_DecodeBufferDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA2_DecodeExecuteParams {
    pub NumCompBuffers: u32,
    pub pCompressedBuffers: *mut DXVA2_DecodeBufferDesc,
    pub pExtensionData: *mut DXVA2_DecodeExtensionData,
}
impl Default for DXVA2_DecodeExecuteParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA2_DecodeExtensionData {
    pub Function: u32,
    pub pPrivateInputData: *mut core::ffi::c_void,
    pub PrivateInputDataSize: u32,
    pub pPrivateOutputData: *mut core::ffi::c_void,
    pub PrivateOutputDataSize: u32,
}
impl Default for DXVA2_DecodeExtensionData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA2_DeinterlaceTech_BOBLineReplicate: i32 = 1;
pub const DXVA2_DeinterlaceTech_BOBVerticalStretch: i32 = 2;
pub const DXVA2_DeinterlaceTech_BOBVerticalStretch4Tap: i32 = 4;
pub const DXVA2_DeinterlaceTech_EdgeFiltering: i32 = 16;
pub const DXVA2_DeinterlaceTech_FieldAdaptive: i32 = 32;
pub const DXVA2_DeinterlaceTech_InverseTelecine: i32 = 256;
pub const DXVA2_DeinterlaceTech_Mask: i32 = 511;
pub const DXVA2_DeinterlaceTech_MedianFiltering: i32 = 8;
pub const DXVA2_DeinterlaceTech_MotionVectorSteered: i32 = 128;
pub const DXVA2_DeinterlaceTech_PixelAdaptive: i32 = 64;
pub const DXVA2_DeinterlaceTech_Unknown: i32 = 0;
pub const DXVA2_DestData_Mask: i32 = 65535;
pub const DXVA2_DestData_RFF: i32 = 1;
pub const DXVA2_DestData_RFF_TFF_Present: i32 = 4;
pub const DXVA2_DestData_TFF: i32 = 2;
pub const DXVA2_DetailFilterChromaLevel: i32 = 10;
pub const DXVA2_DetailFilterChromaRadius: i32 = 12;
pub const DXVA2_DetailFilterChromaThreshold: i32 = 11;
pub const DXVA2_DetailFilterLumaLevel: i32 = 7;
pub const DXVA2_DetailFilterLumaRadius: i32 = 9;
pub const DXVA2_DetailFilterLumaThreshold: i32 = 8;
pub const DXVA2_DetailFilterTech_Edge: i32 = 2;
pub const DXVA2_DetailFilterTech_Mask: i32 = 7;
pub const DXVA2_DetailFilterTech_Sharpening: i32 = 4;
pub const DXVA2_DetailFilterTech_Unknown: i32 = 1;
pub const DXVA2_DetailFilterTech_Unsupported: i32 = 0;
pub const DXVA2_E_NEW_VIDEO_DEVICE: windows_sys::core::HRESULT = 0x80041001_u32 as _;
pub const DXVA2_E_NOT_AVAILABLE: windows_sys::core::HRESULT = 0x80041003_u32 as _;
pub const DXVA2_E_NOT_INITIALIZED: windows_sys::core::HRESULT = 0x80041000_u32 as _;
pub const DXVA2_E_VIDEO_DEVICE_LOCKED: windows_sys::core::HRESULT = 0x80041002_u32 as _;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA2_ExtendedFormat {
    pub Anonymous: DXVA2_ExtendedFormat_0,
}
impl Default for DXVA2_ExtendedFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA2_ExtendedFormat_0 {
    pub Anonymous: DXVA2_ExtendedFormat_0_0,
    pub value: u32,
}
impl Default for DXVA2_ExtendedFormat_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA2_ExtendedFormat_0_0 {
    pub _bitfield: u32,
}
pub const DXVA2_FilmGrainBuffer: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA2_FilterValues {
    pub Level: DXVA2_Fixed32,
    pub Threshold: DXVA2_Fixed32,
    pub Radius: DXVA2_Fixed32,
}
impl Default for DXVA2_FilterValues {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA2_Fixed32 {
    pub Anonymous: DXVA2_Fixed32_0,
}
impl Default for DXVA2_Fixed32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXVA2_Fixed32_0 {
    pub Anonymous: DXVA2_Fixed32_0_0,
    pub ll: i32,
}
impl Default for DXVA2_Fixed32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA2_Fixed32_0_0 {
    pub Fraction: u16,
    pub Value: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXVA2_Frequency {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub const DXVA2_InverseQuantizationMatrixBufferType: i32 = 4;
pub const DXVA2_MacroBlockControlBufferType: i32 = 1;
pub const DXVA2_ModeH264_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be64_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be65_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_C: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be66_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_D: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be67_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_E: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_F: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be69_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_VLD_Multiview_NoFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const DXVA2_ModeH264_VLD_Stereo_NoFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const DXVA2_ModeH264_VLD_Stereo_Progressive_NoFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const DXVA2_ModeH264_VLD_WithFMOASO_NoFGT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd5f04ff9_3418_45d8_9561_32a76aae2ddd);
pub const DXVA2_ModeHEVC_VLD_Main: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const DXVA2_ModeHEVC_VLD_Main10: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const DXVA2_ModeMPEG1_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f3ec719_3735_42cc_8063_65cc3cb36616);
pub const DXVA2_ModeMPEG2_IDCT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbf22ad00_03ea_4690_8077_473346209b7e);
pub const DXVA2_ModeMPEG2_MoComp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe6a9f44b_61b0_4563_9ea4_63d2a3c6fe66);
pub const DXVA2_ModeMPEG2_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const DXVA2_ModeMPEG2and1_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const DXVA2_ModeMPEG4pt2_VLD_AdvSimple_GMC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab998b5b_4258_44a9_9feb_94e597a6baae);
pub const DXVA2_ModeMPEG4pt2_VLD_AdvSimple_NoGMC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const DXVA2_ModeMPEG4pt2_VLD_Simple: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const DXVA2_ModeVC1_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea0_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea1_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_C: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea2_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_D: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_D2010: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVP8_VLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const DXVA2_ModeVP9_VLD_10bit_Profile2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
pub const DXVA2_ModeVP9_VLD_Profile0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const DXVA2_ModeWMV8_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be80_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV8_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be81_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be90_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be91_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_C: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81be94_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_MotionVectorBuffer: i32 = 7;
pub const DXVA2_NoEncrypt: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b81bed0_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_NoiseFilterChromaLevel: i32 = 4;
pub const DXVA2_NoiseFilterChromaRadius: i32 = 6;
pub const DXVA2_NoiseFilterChromaThreshold: i32 = 5;
pub const DXVA2_NoiseFilterLumaLevel: i32 = 1;
pub const DXVA2_NoiseFilterLumaRadius: i32 = 3;
pub const DXVA2_NoiseFilterLumaThreshold: i32 = 2;
pub const DXVA2_NoiseFilterTech_BlockNoise: i32 = 8;
pub const DXVA2_NoiseFilterTech_Mask: i32 = 31;
pub const DXVA2_NoiseFilterTech_Median: i32 = 2;
pub const DXVA2_NoiseFilterTech_MosquitoNoise: i32 = 16;
pub const DXVA2_NoiseFilterTech_Temporal: i32 = 4;
pub const DXVA2_NoiseFilterTech_Unknown: i32 = 1;
pub const DXVA2_NoiseFilterTech_Unsupported: i32 = 0;
pub type DXVA2_NominalRange = i32;
pub const DXVA2_NominalRangeMask: DXVA2_NominalRange = 7;
pub const DXVA2_NominalRange_0_255: DXVA2_NominalRange = 1;
pub const DXVA2_NominalRange_16_235: DXVA2_NominalRange = 2;
pub const DXVA2_NominalRange_48_208: DXVA2_NominalRange = 3;
pub const DXVA2_NominalRange_Normal: DXVA2_NominalRange = 1;
pub const DXVA2_NominalRange_Unknown: DXVA2_NominalRange = 0;
pub const DXVA2_NominalRange_Wide: DXVA2_NominalRange = 2;
pub const DXVA2_PictureParametersBufferType: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA2_ProcAmpValues {
    pub Brightness: DXVA2_Fixed32,
    pub Contrast: DXVA2_Fixed32,
    pub Hue: DXVA2_Fixed32,
    pub Saturation: DXVA2_Fixed32,
}
impl Default for DXVA2_ProcAmpValues {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA2_ProcAmp_Brightness: i32 = 1;
pub const DXVA2_ProcAmp_Contrast: i32 = 2;
pub const DXVA2_ProcAmp_Hue: i32 = 4;
pub const DXVA2_ProcAmp_Mask: i32 = 15;
pub const DXVA2_ProcAmp_None: i32 = 0;
pub const DXVA2_ProcAmp_Saturation: i32 = 8;
pub const DXVA2_ResidualDifferenceBufferType: i32 = 2;
pub const DXVA2_SampleData_Mask: i32 = 65535;
pub const DXVA2_SampleData_RFF: i32 = 1;
pub const DXVA2_SampleData_RFF_TFF_Present: i32 = 4;
pub const DXVA2_SampleData_TFF: i32 = 2;
pub const DXVA2_SampleFieldInterleavedEvenFirst: DXVA2_SampleFormat = 3;
pub const DXVA2_SampleFieldInterleavedOddFirst: DXVA2_SampleFormat = 4;
pub const DXVA2_SampleFieldSingleEven: DXVA2_SampleFormat = 5;
pub const DXVA2_SampleFieldSingleOdd: DXVA2_SampleFormat = 6;
pub type DXVA2_SampleFormat = i32;
pub const DXVA2_SampleFormatMask: DXVA2_SampleFormat = 255;
pub const DXVA2_SampleProgressiveFrame: DXVA2_SampleFormat = 2;
pub const DXVA2_SampleSubStream: DXVA2_SampleFormat = 7;
pub const DXVA2_SampleUnknown: DXVA2_SampleFormat = 0;
pub const DXVA2_SliceControlBufferType: i32 = 5;
pub type DXVA2_SurfaceType = i32;
pub const DXVA2_SurfaceType_D3DRenderTargetTexture: DXVA2_SurfaceType = 2;
pub const DXVA2_SurfaceType_DecoderRenderTarget: DXVA2_SurfaceType = 0;
pub const DXVA2_SurfaceType_ProcessorRenderTarget: DXVA2_SurfaceType = 1;
pub const DXVA2_VPDev_EmulatedDXVA1: i32 = 2;
pub const DXVA2_VPDev_HardwareDevice: i32 = 1;
pub const DXVA2_VPDev_Mask: i32 = 7;
pub const DXVA2_VPDev_SoftwareDevice: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXVA2_ValueRange {
    pub MinValue: DXVA2_Fixed32,
    pub MaxValue: DXVA2_Fixed32,
    pub DefaultValue: DXVA2_Fixed32,
    pub StepSize: DXVA2_Fixed32,
}
impl Default for DXVA2_ValueRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXVA2_VideoChromaSubSampling = i32;
pub const DXVA2_VideoChromaSubsamplingMask: DXVA2_VideoChromaSubSampling = 15;
pub const DXVA2_VideoChromaSubsampling_Cosited: DXVA2_VideoChromaSubSampling = 7;
pub const DXVA2_VideoChromaSubsampling_DV_PAL: DXVA2_VideoChromaSubSampling = 6;
pub const DXVA2_VideoChromaSubsampling_Horizontally_Cosited: DXVA2_VideoChromaSubSampling = 4;
pub const DXVA2_VideoChromaSubsampling_MPEG1: DXVA2_VideoChromaSubSampling = 1;
pub const DXVA2_VideoChromaSubsampling_MPEG2: DXVA2_VideoChromaSubSampling = 5;
pub const DXVA2_VideoChromaSubsampling_ProgressiveChroma: DXVA2_VideoChromaSubSampling = 8;
pub const DXVA2_VideoChromaSubsampling_Unknown: DXVA2_VideoChromaSubSampling = 0;
pub const DXVA2_VideoChromaSubsampling_Vertically_AlignedChromaPlanes: DXVA2_VideoChromaSubSampling = 1;
pub const DXVA2_VideoChromaSubsampling_Vertically_Cosited: DXVA2_VideoChromaSubSampling = 2;
pub const DXVA2_VideoDecoderRenderTarget: i32 = 0;
#[repr(C)]
#[cfg(feature = "Win32_d3d9")]
#[derive(Clone, Copy)]
pub struct DXVA2_VideoDesc {
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub SampleFormat: DXVA2_ExtendedFormat,
    pub Format: super::d3d9::D3DFORMAT,
    pub InputSampleFreq: DXVA2_Frequency,
    pub OutputFrameFreq: DXVA2_Frequency,
    pub UABProtectionLevel: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_d3d9")]
impl Default for DXVA2_VideoDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXVA2_VideoLighting = i32;
pub const DXVA2_VideoLightingMask: DXVA2_VideoLighting = 15;
pub const DXVA2_VideoLighting_Unknown: DXVA2_VideoLighting = 0;
pub const DXVA2_VideoLighting_bright: DXVA2_VideoLighting = 1;
pub const DXVA2_VideoLighting_dark: DXVA2_VideoLighting = 4;
pub const DXVA2_VideoLighting_dim: DXVA2_VideoLighting = 3;
pub const DXVA2_VideoLighting_office: DXVA2_VideoLighting = 2;
pub type DXVA2_VideoPrimaries = i32;
pub const DXVA2_VideoPrimariesMask: DXVA2_VideoPrimaries = 31;
pub const DXVA2_VideoPrimaries_BT470_2_SysBG: DXVA2_VideoPrimaries = 4;
pub const DXVA2_VideoPrimaries_BT470_2_SysM: DXVA2_VideoPrimaries = 3;
pub const DXVA2_VideoPrimaries_BT709: DXVA2_VideoPrimaries = 2;
pub const DXVA2_VideoPrimaries_EBU3213: DXVA2_VideoPrimaries = 7;
pub const DXVA2_VideoPrimaries_SMPTE170M: DXVA2_VideoPrimaries = 5;
pub const DXVA2_VideoPrimaries_SMPTE240M: DXVA2_VideoPrimaries = 6;
pub const DXVA2_VideoPrimaries_SMPTE_C: DXVA2_VideoPrimaries = 8;
pub const DXVA2_VideoPrimaries_Unknown: DXVA2_VideoPrimaries = 0;
pub const DXVA2_VideoPrimaries_reserved: DXVA2_VideoPrimaries = 1;
pub const DXVA2_VideoProcBobDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x335aa36e_7884_43a4_9c91_7f87faf3e37e);
pub const DXVA2_VideoProcProgressiveDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5a54a0c9_c7ec_4bd9_8ede_f3c75dc4393b);
pub const DXVA2_VideoProcSoftwareDevice: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4553d47f_ee7e_4e3f_9475_dbf1376c4810);
#[repr(C)]
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct DXVA2_VideoProcessBltParams {
    pub TargetFrame: super::mediaobj::REFERENCE_TIME,
    pub TargetRect: super::windef::RECT,
    pub ConstrictionSize: super::windef::SIZE,
    pub StreamingFlags: u32,
    pub BackgroundColor: DXVA2_AYUVSample16,
    pub DestFormat: DXVA2_ExtendedFormat,
    pub ProcAmpValues: DXVA2_ProcAmpValues,
    pub Alpha: DXVA2_Fixed32,
    pub NoiseFilterLuma: DXVA2_FilterValues,
    pub NoiseFilterChroma: DXVA2_FilterValues,
    pub DetailFilterLuma: DXVA2_FilterValues,
    pub DetailFilterChroma: DXVA2_FilterValues,
    pub DestData: u32,
}
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef"))]
impl Default for DXVA2_VideoProcessBltParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA2_VideoProcess_AlphaBlend: i32 = 8;
pub const DXVA2_VideoProcess_AlphaBlendExtended: i32 = 256;
pub const DXVA2_VideoProcess_Constriction: i32 = 512;
pub const DXVA2_VideoProcess_DetailFilter: i32 = 2048;
pub const DXVA2_VideoProcess_GammaCompensated: i32 = 16384;
pub const DXVA2_VideoProcess_LinearScaling: i32 = 8192;
pub const DXVA2_VideoProcess_MaintainsOriginalFieldData: i32 = 32768;
pub const DXVA2_VideoProcess_Mask: i32 = 65535;
pub const DXVA2_VideoProcess_NoiseFilter: i32 = 1024;
pub const DXVA2_VideoProcess_None: i32 = 0;
pub const DXVA2_VideoProcess_PlanarAlpha: i32 = 4096;
pub const DXVA2_VideoProcess_StretchX: i32 = 2;
pub const DXVA2_VideoProcess_StretchY: i32 = 4;
pub const DXVA2_VideoProcess_SubRects: i32 = 16;
pub const DXVA2_VideoProcess_SubStreams: i32 = 32;
pub const DXVA2_VideoProcess_SubStreamsExtended: i32 = 64;
pub const DXVA2_VideoProcess_YUV2RGB: i32 = 1;
pub const DXVA2_VideoProcess_YUV2RGBExtended: i32 = 128;
#[repr(C)]
#[cfg(feature = "Win32_d3d9")]
#[derive(Clone, Copy, Default)]
pub struct DXVA2_VideoProcessorCaps {
    pub DeviceCaps: u32,
    pub InputPool: super::d3d9::D3DPOOL,
    pub NumForwardRefSamples: u32,
    pub NumBackwardRefSamples: u32,
    pub Reserved: u32,
    pub DeinterlaceTechnology: u32,
    pub ProcAmpControlCaps: u32,
    pub VideoProcessorOperations: u32,
    pub NoiseFilterTechnology: u32,
    pub DetailFilterTechnology: u32,
}
pub const DXVA2_VideoProcessorRenderTarget: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct DXVA2_VideoSample {
    pub Start: super::mediaobj::REFERENCE_TIME,
    pub End: super::mediaobj::REFERENCE_TIME,
    pub SampleFormat: DXVA2_ExtendedFormat,
    pub SrcSurface: *mut core::ffi::c_void,
    pub SrcRect: super::windef::RECT,
    pub DstRect: super::windef::RECT,
    pub Pal: [DXVA2_AYUVSample8; 16],
    pub PlanarAlpha: DXVA2_Fixed32,
    pub SampleData: u32,
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef"))]
impl Default for DXVA2_VideoSample {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXVA2_VideoSoftwareRenderTarget: i32 = 2;
pub const DXVA2_VideoTransFuncMask: DXVA2_VideoTransferFunction = 31;
pub const DXVA2_VideoTransFunc_10: DXVA2_VideoTransferFunction = 1;
pub const DXVA2_VideoTransFunc_18: DXVA2_VideoTransferFunction = 2;
pub const DXVA2_VideoTransFunc_20: DXVA2_VideoTransferFunction = 3;
pub const DXVA2_VideoTransFunc_22: DXVA2_VideoTransferFunction = 4;
pub const DXVA2_VideoTransFunc_22_240M: u32 = 6;
pub const DXVA2_VideoTransFunc_22_709: u32 = 5;
pub const DXVA2_VideoTransFunc_22_8bit_sRGB: u32 = 7;
pub const DXVA2_VideoTransFunc_240M: DXVA2_VideoTransferFunction = 6;
pub const DXVA2_VideoTransFunc_28: DXVA2_VideoTransferFunction = 8;
pub const DXVA2_VideoTransFunc_709: DXVA2_VideoTransferFunction = 5;
pub const DXVA2_VideoTransFunc_Unknown: DXVA2_VideoTransferFunction = 0;
pub const DXVA2_VideoTransFunc_sRGB: DXVA2_VideoTransferFunction = 7;
pub type DXVA2_VideoTransferFunction = i32;
pub type DXVA2_VideoTransferMatrix = i32;
pub const DXVA2_VideoTransferMatrixMask: DXVA2_VideoTransferMatrix = 7;
pub const DXVA2_VideoTransferMatrix_BT601: DXVA2_VideoTransferMatrix = 2;
pub const DXVA2_VideoTransferMatrix_BT709: DXVA2_VideoTransferMatrix = 1;
pub const DXVA2_VideoTransferMatrix_SMPTE240M: DXVA2_VideoTransferMatrix = 3;
pub const DXVA2_VideoTransferMatrix_Unknown: DXVA2_VideoTransferMatrix = 0;
pub const MAX_DEINTERLACE_SURFACES: u32 = 32;
pub const MAX_SUBSTREAMS: u32 = 15;
