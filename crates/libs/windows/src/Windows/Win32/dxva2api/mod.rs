#[inline]
pub unsafe fn DXVA2CreateDirect3DDeviceManager9(presettoken: *mut u32, ppdevicemanager: *mut Option<IDirect3DDeviceManager9>) -> windows_core::HRESULT {
    windows_core::link!("dxva2.dll" "system" fn DXVA2CreateDirect3DDeviceManager9(presettoken : *mut u32, ppdevicemanager : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DXVA2CreateDirect3DDeviceManager9(presettoken as _, core::mem::transmute(ppdevicemanager)) }
}
#[cfg(feature = "Win32_d3d9")]
#[inline]
pub unsafe fn DXVA2CreateVideoService<P0>(pdd: P0, riid: *const windows_core::GUID, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::d3d9::IDirect3DDevice9>,
{
    windows_core::link!("dxva2.dll" "system" fn DXVA2CreateVideoService(pdd : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppservice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DXVA2CreateVideoService(pdd.param().abi(), riid, ppservice as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXVA2_AES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXVA2_AYUVSample16 {
    pub Cr: u16,
    pub Cb: u16,
    pub Y: u16,
    pub Alpha: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXVA2_AYUVSample8 {
    pub Cr: u8,
    pub Cb: u8,
    pub Y: u8,
    pub Alpha: u8,
}
pub const DXVA2_BitStreamDateBufferType: i32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXVA2_ConfigPictureDecode {
    pub guidConfigBitstreamEncryption: windows_core::GUID,
    pub guidConfigMBcontrolEncryption: windows_core::GUID,
    pub guidConfigResidDiffEncryption: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const DXVA2_E_NEW_VIDEO_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x80041001_u32 as _);
pub const DXVA2_E_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80041003_u32 as _);
pub const DXVA2_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80041000_u32 as _);
pub const DXVA2_E_VIDEO_DEVICE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x80041002_u32 as _);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXVA2_Fixed32_0_0 {
    pub Fraction: u16,
    pub Value: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXVA2_Frequency {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub const DXVA2_InverseQuantizationMatrixBufferType: i32 = 4;
pub const DXVA2_MacroBlockControlBufferType: i32 = 1;
pub const DXVA2_ModeH264_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be64_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be65_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_C: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be66_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_D: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be67_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_E: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_F: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be69_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeH264_VLD_Multiview_NoFGT: windows_core::GUID = windows_core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const DXVA2_ModeH264_VLD_Stereo_NoFGT: windows_core::GUID = windows_core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const DXVA2_ModeH264_VLD_Stereo_Progressive_NoFGT: windows_core::GUID = windows_core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const DXVA2_ModeH264_VLD_WithFMOASO_NoFGT: windows_core::GUID = windows_core::GUID::from_u128(0xd5f04ff9_3418_45d8_9561_32a76aae2ddd);
pub const DXVA2_ModeHEVC_VLD_Main: windows_core::GUID = windows_core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const DXVA2_ModeHEVC_VLD_Main10: windows_core::GUID = windows_core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const DXVA2_ModeMPEG1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x6f3ec719_3735_42cc_8063_65cc3cb36616);
pub const DXVA2_ModeMPEG2_IDCT: windows_core::GUID = windows_core::GUID::from_u128(0xbf22ad00_03ea_4690_8077_473346209b7e);
pub const DXVA2_ModeMPEG2_MoComp: windows_core::GUID = windows_core::GUID::from_u128(0xe6a9f44b_61b0_4563_9ea4_63d2a3c6fe66);
pub const DXVA2_ModeMPEG2_VLD: windows_core::GUID = windows_core::GUID::from_u128(0xee27417f_5e28_4e65_beea_1d26b508adc9);
pub const DXVA2_ModeMPEG2and1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const DXVA2_ModeMPEG4pt2_VLD_AdvSimple_GMC: windows_core::GUID = windows_core::GUID::from_u128(0xab998b5b_4258_44a9_9feb_94e597a6baae);
pub const DXVA2_ModeMPEG4pt2_VLD_AdvSimple_NoGMC: windows_core::GUID = windows_core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const DXVA2_ModeMPEG4pt2_VLD_Simple: windows_core::GUID = windows_core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const DXVA2_ModeVC1_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea0_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea1_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_C: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea2_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_D: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVC1_D2010: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeVP8_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const DXVA2_ModeVP9_VLD_10bit_Profile2: windows_core::GUID = windows_core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
pub const DXVA2_ModeVP9_VLD_Profile0: windows_core::GUID = windows_core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const DXVA2_ModeWMV8_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be80_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV8_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be81_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be90_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be91_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_ModeWMV9_C: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be94_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA2_MotionVectorBuffer: i32 = 7;
pub const DXVA2_NoEncrypt: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bed0_a0c7_11d3_b984_00c04f2e73c5);
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
pub const DXVA2_VideoProcBobDevice: windows_core::GUID = windows_core::GUID::from_u128(0x335aa36e_7884_43a4_9c91_7f87faf3e37e);
pub const DXVA2_VideoProcProgressiveDevice: windows_core::GUID = windows_core::GUID::from_u128(0x5a54a0c9_c7ec_4bd9_8ede_f3c75dc4393b);
pub const DXVA2_VideoProcSoftwareDevice: windows_core::GUID = windows_core::GUID::from_u128(0x4553d47f_ee7e_4e3f_9475_dbf1376c4810);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub struct DXVA2_VideoSample {
    pub Start: super::mediaobj::REFERENCE_TIME,
    pub End: super::mediaobj::REFERENCE_TIME,
    pub SampleFormat: DXVA2_ExtendedFormat,
    pub SrcSurface: core::mem::ManuallyDrop<Option<super::d3d9::IDirect3DSurface9>>,
    pub SrcRect: super::windef::RECT,
    pub DstRect: super::windef::RECT,
    pub Pal: [DXVA2_AYUVSample8; 16],
    pub PlanarAlpha: DXVA2_Fixed32,
    pub SampleData: u32,
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef"))]
impl Clone for DXVA2_VideoSample {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
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
windows_core::imp::define_interface!(IDirect3DDeviceManager9, IDirect3DDeviceManager9_Vtbl, 0xa0cade0f_06d5_4cf4_a1c7_f3cdd725aa75);
windows_core::imp::interface_hierarchy!(IDirect3DDeviceManager9, windows_core::IUnknown);
impl IDirect3DDeviceManager9 {
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn ResetDevice<P0>(&self, pdevice: P0, resettoken: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d9::IDirect3DDevice9>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResetDevice)(windows_core::Interface::as_raw(self), pdevice.param().abi(), resettoken) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn OpenDeviceHandle(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenDeviceHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CloseDeviceHandle(&self, hdevice: super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloseDeviceHandle)(windows_core::Interface::as_raw(self), hdevice) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn TestDevice(&self, hdevice: super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestDevice)(windows_core::Interface::as_raw(self), hdevice) }
    }
    #[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
    pub unsafe fn LockDevice(&self, hdevice: super::winnt::HANDLE, ppdevice: *mut Option<super::d3d9::IDirect3DDevice9>, fblock: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockDevice)(windows_core::Interface::as_raw(self), hdevice, core::mem::transmute(ppdevice), fblock.into()) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn UnlockDevice(&self, hdevice: super::winnt::HANDLE, fsavestate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockDevice)(windows_core::Interface::as_raw(self), hdevice, fsavestate.into()) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetVideoService(&self, hdevice: super::winnt::HANDLE, riid: *const windows_core::GUID, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoService)(windows_core::Interface::as_raw(self), hdevice, riid, ppservice as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDeviceManager9_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_d3d9")]
    pub ResetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    ResetDevice: usize,
    #[cfg(feature = "Win32_winnt")]
    pub OpenDeviceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    OpenDeviceHandle: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CloseDeviceHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CloseDeviceHandle: usize,
    #[cfg(feature = "Win32_winnt")]
    pub TestDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    TestDevice: usize,
    #[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
    pub LockDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9", feature = "Win32_winnt")))]
    LockDevice: usize,
    #[cfg(feature = "Win32_winnt")]
    pub UnlockDevice: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    UnlockDevice: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetVideoService: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetVideoService: usize,
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
pub trait IDirect3DDeviceManager9_Impl: windows_core::IUnknownImpl {
    fn ResetDevice(&self, pdevice: windows_core::Ref<super::d3d9::IDirect3DDevice9>, resettoken: u32) -> windows_core::Result<()>;
    fn OpenDeviceHandle(&self) -> windows_core::Result<super::winnt::HANDLE>;
    fn CloseDeviceHandle(&self, hdevice: super::winnt::HANDLE) -> windows_core::Result<()>;
    fn TestDevice(&self, hdevice: super::winnt::HANDLE) -> windows_core::Result<()>;
    fn LockDevice(&self, hdevice: super::winnt::HANDLE, ppdevice: windows_core::OutRef<super::d3d9::IDirect3DDevice9>, fblock: windows_core::BOOL) -> windows_core::Result<()>;
    fn UnlockDevice(&self, hdevice: super::winnt::HANDLE, fsavestate: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetVideoService(&self, hdevice: super::winnt::HANDLE, riid: *const windows_core::GUID, ppservice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl IDirect3DDeviceManager9_Vtbl {
    pub const fn new<Identity: IDirect3DDeviceManager9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResetDevice<Identity: IDirect3DDeviceManager9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, resettoken: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDeviceManager9_Impl::ResetDevice(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&resettoken)).into()
            }
        }
        unsafe extern "system" fn OpenDeviceHandle<Identity: IDirect3DDeviceManager9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phdevice: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirect3DDeviceManager9_Impl::OpenDeviceHandle(this) {
                    Ok(ok__) => {
                        phdevice.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloseDeviceHandle<Identity: IDirect3DDeviceManager9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDeviceManager9_Impl::CloseDeviceHandle(this, core::mem::transmute_copy(&hdevice)).into()
            }
        }
        unsafe extern "system" fn TestDevice<Identity: IDirect3DDeviceManager9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDeviceManager9_Impl::TestDevice(this, core::mem::transmute_copy(&hdevice)).into()
            }
        }
        unsafe extern "system" fn LockDevice<Identity: IDirect3DDeviceManager9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::winnt::HANDLE, ppdevice: *mut *mut core::ffi::c_void, fblock: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDeviceManager9_Impl::LockDevice(this, core::mem::transmute_copy(&hdevice), core::mem::transmute_copy(&ppdevice), core::mem::transmute_copy(&fblock)).into()
            }
        }
        unsafe extern "system" fn UnlockDevice<Identity: IDirect3DDeviceManager9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::winnt::HANDLE, fsavestate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDeviceManager9_Impl::UnlockDevice(this, core::mem::transmute_copy(&hdevice), core::mem::transmute_copy(&fsavestate)).into()
            }
        }
        unsafe extern "system" fn GetVideoService<Identity: IDirect3DDeviceManager9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdevice: super::winnt::HANDLE, riid: *const windows_core::GUID, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirect3DDeviceManager9_Impl::GetVideoService(this, core::mem::transmute_copy(&hdevice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppservice)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ResetDevice: ResetDevice::<Identity, OFFSET>,
            OpenDeviceHandle: OpenDeviceHandle::<Identity, OFFSET>,
            CloseDeviceHandle: CloseDeviceHandle::<Identity, OFFSET>,
            TestDevice: TestDevice::<Identity, OFFSET>,
            LockDevice: LockDevice::<Identity, OFFSET>,
            UnlockDevice: UnlockDevice::<Identity, OFFSET>,
            GetVideoService: GetVideoService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDeviceManager9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirect3DDeviceManager9 {}
windows_core::imp::define_interface!(IDirectXVideoAccelerationService, IDirectXVideoAccelerationService_Vtbl, 0xfc51a550_d5e7_11d9_af55_00054e43ff02);
windows_core::imp::interface_hierarchy!(IDirectXVideoAccelerationService, windows_core::IUnknown);
impl IDirectXVideoAccelerationService {
    #[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, backbuffers: u32, format: super::d3d9::D3DFORMAT, pool: super::d3d9::D3DPOOL, usage: u32, dxvatype: u32, ppsurface: *mut Option<super::d3d9::IDirect3DSurface9>, psharedhandle: Option<*mut super::winnt::HANDLE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateSurface)(windows_core::Interface::as_raw(self), width, height, backbuffers, format, pool, usage, dxvatype, core::mem::transmute(ppsurface), psharedhandle.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectXVideoAccelerationService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
    pub CreateSurface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, super::d3d9::D3DFORMAT, super::d3d9::D3DPOOL, u32, u32, *mut *mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9", feature = "Win32_winnt")))]
    CreateSurface: usize,
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
pub trait IDirectXVideoAccelerationService_Impl: windows_core::IUnknownImpl {
    fn CreateSurface(&self, width: u32, height: u32, backbuffers: u32, format: super::d3d9::D3DFORMAT, pool: super::d3d9::D3DPOOL, usage: u32, dxvatype: u32, ppsurface: windows_core::OutRef<super::d3d9::IDirect3DSurface9>, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl IDirectXVideoAccelerationService_Vtbl {
    pub const fn new<Identity: IDirectXVideoAccelerationService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSurface<Identity: IDirectXVideoAccelerationService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32, backbuffers: u32, format: super::d3d9::D3DFORMAT, pool: super::d3d9::D3DPOOL, usage: u32, dxvatype: u32, ppsurface: *mut *mut core::ffi::c_void, psharedhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoAccelerationService_Impl::CreateSurface(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&backbuffers), core::mem::transmute_copy(&format), core::mem::transmute_copy(&pool), core::mem::transmute_copy(&usage), core::mem::transmute_copy(&dxvatype), core::mem::transmute_copy(&ppsurface), core::mem::transmute_copy(&psharedhandle)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateSurface: CreateSurface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectXVideoAccelerationService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirectXVideoAccelerationService {}
windows_core::imp::define_interface!(IDirectXVideoDecoder, IDirectXVideoDecoder_Vtbl, 0xf2b0810a_fd00_43c9_918c_df94e2d8ef7d);
windows_core::imp::interface_hierarchy!(IDirectXVideoDecoder, windows_core::IUnknown);
impl IDirectXVideoDecoder {
    pub unsafe fn GetVideoDecoderService(&self) -> windows_core::Result<IDirectXVideoDecoderService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoDecoderService)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetCreationParameters(&self, pdeviceguid: Option<*mut windows_core::GUID>, pvideodesc: Option<*mut DXVA2_VideoDesc>, pconfig: Option<*mut DXVA2_ConfigPictureDecode>, pdecoderrendertargets: *mut *mut Option<super::d3d9::IDirect3DSurface9>, pnumsurfaces: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCreationParameters)(windows_core::Interface::as_raw(self), pdeviceguid.unwrap_or(core::mem::zeroed()) as _, pvideodesc.unwrap_or(core::mem::zeroed()) as _, pconfig.unwrap_or(core::mem::zeroed()) as _, pdecoderrendertargets as _, pnumsurfaces.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetBuffer(&self, buffertype: u32, ppbuffer: *mut *mut core::ffi::c_void, pbuffersize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), buffertype, ppbuffer as _, pbuffersize as _) }
    }
    pub unsafe fn ReleaseBuffer(&self, buffertype: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseBuffer)(windows_core::Interface::as_raw(self), buffertype) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn BeginFrame<P0>(&self, prendertarget: P0, pvpvpdata: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d9::IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginFrame)(windows_core::Interface::as_raw(self), prendertarget.param().abi(), pvpvpdata.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn EndFrame(&self, phandlecomplete: Option<*mut super::winnt::HANDLE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndFrame)(windows_core::Interface::as_raw(self), phandlecomplete.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Execute(&self, pexecuteparams: *const DXVA2_DecodeExecuteParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), pexecuteparams) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectXVideoDecoder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVideoDecoderService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9")]
    pub GetCreationParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut DXVA2_VideoDesc, *mut DXVA2_ConfigPictureDecode, *mut *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetCreationParameters: usize,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9")]
    pub BeginFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    BeginFrame: usize,
    #[cfg(feature = "Win32_winnt")]
    pub EndFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    EndFrame: usize,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXVA2_DecodeExecuteParams) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
pub trait IDirectXVideoDecoder_Impl: windows_core::IUnknownImpl {
    fn GetVideoDecoderService(&self) -> windows_core::Result<IDirectXVideoDecoderService>;
    fn GetCreationParameters(&self, pdeviceguid: *mut windows_core::GUID, pvideodesc: *mut DXVA2_VideoDesc, pconfig: *mut DXVA2_ConfigPictureDecode, pdecoderrendertargets: *mut *mut Option<super::d3d9::IDirect3DSurface9>, pnumsurfaces: *mut u32) -> windows_core::Result<()>;
    fn GetBuffer(&self, buffertype: u32, ppbuffer: *mut *mut core::ffi::c_void, pbuffersize: *mut u32) -> windows_core::Result<()>;
    fn ReleaseBuffer(&self, buffertype: u32) -> windows_core::Result<()>;
    fn BeginFrame(&self, prendertarget: windows_core::Ref<super::d3d9::IDirect3DSurface9>, pvpvpdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn EndFrame(&self, phandlecomplete: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
    fn Execute(&self, pexecuteparams: *const DXVA2_DecodeExecuteParams) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl IDirectXVideoDecoder_Vtbl {
    pub const fn new<Identity: IDirectXVideoDecoder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVideoDecoderService<Identity: IDirectXVideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoDecoder_Impl::GetVideoDecoderService(this) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCreationParameters<Identity: IDirectXVideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceguid: *mut windows_core::GUID, pvideodesc: *mut DXVA2_VideoDesc, pconfig: *mut DXVA2_ConfigPictureDecode, pdecoderrendertargets: *mut *mut *mut core::ffi::c_void, pnumsurfaces: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoder_Impl::GetCreationParameters(this, core::mem::transmute_copy(&pdeviceguid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&pconfig), core::mem::transmute_copy(&pdecoderrendertargets), core::mem::transmute_copy(&pnumsurfaces)).into()
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: IDirectXVideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffertype: u32, ppbuffer: *mut *mut core::ffi::c_void, pbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoder_Impl::GetBuffer(this, core::mem::transmute_copy(&buffertype), core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&pbuffersize)).into()
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: IDirectXVideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffertype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoder_Impl::ReleaseBuffer(this, core::mem::transmute_copy(&buffertype)).into()
            }
        }
        unsafe extern "system" fn BeginFrame<Identity: IDirectXVideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, pvpvpdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoder_Impl::BeginFrame(this, core::mem::transmute_copy(&prendertarget), core::mem::transmute_copy(&pvpvpdata)).into()
            }
        }
        unsafe extern "system" fn EndFrame<Identity: IDirectXVideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandlecomplete: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoder_Impl::EndFrame(this, core::mem::transmute_copy(&phandlecomplete)).into()
            }
        }
        unsafe extern "system" fn Execute<Identity: IDirectXVideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexecuteparams: *const DXVA2_DecodeExecuteParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoder_Impl::Execute(this, core::mem::transmute_copy(&pexecuteparams)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVideoDecoderService: GetVideoDecoderService::<Identity, OFFSET>,
            GetCreationParameters: GetCreationParameters::<Identity, OFFSET>,
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, OFFSET>,
            BeginFrame: BeginFrame::<Identity, OFFSET>,
            EndFrame: EndFrame::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectXVideoDecoder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirectXVideoDecoder {}
windows_core::imp::define_interface!(IDirectXVideoDecoderService, IDirectXVideoDecoderService_Vtbl, 0xfc51a551_d5e7_11d9_af55_00054e43ff02);
impl core::ops::Deref for IDirectXVideoDecoderService {
    type Target = IDirectXVideoAccelerationService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectXVideoDecoderService, windows_core::IUnknown, IDirectXVideoAccelerationService);
impl IDirectXVideoDecoderService {
    pub unsafe fn GetDecoderDeviceGuids(&self, pcount: *mut u32, pguids: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDecoderDeviceGuids)(windows_core::Interface::as_raw(self), pcount as _, pguids as _) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetDecoderRenderTargets(&self, guid: *const windows_core::GUID, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDecoderRenderTargets)(windows_core::Interface::as_raw(self), guid, pcount as _, pformats as _) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetDecoderConfigurations(&self, guid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, preserved: Option<*const core::ffi::c_void>, pcount: *mut u32, ppconfigs: *mut *mut DXVA2_ConfigPictureDecode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDecoderConfigurations)(windows_core::Interface::as_raw(self), guid, pvideodesc, preserved.unwrap_or(core::mem::zeroed()) as _, pcount as _, ppconfigs as _) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn CreateVideoDecoder(&self, guid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, pconfig: *const DXVA2_ConfigPictureDecode, ppdecoderrendertargets: &[Option<super::d3d9::IDirect3DSurface9>]) -> windows_core::Result<IDirectXVideoDecoder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVideoDecoder)(windows_core::Interface::as_raw(self), guid, pvideodesc, pconfig, core::mem::transmute(ppdecoderrendertargets.as_ptr()), ppdecoderrendertargets.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectXVideoDecoderService_Vtbl {
    pub base__: IDirectXVideoAccelerationService_Vtbl,
    pub GetDecoderDeviceGuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9")]
    pub GetDecoderRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetDecoderRenderTargets: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub GetDecoderConfigurations: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DXVA2_VideoDesc, *const core::ffi::c_void, *mut u32, *mut *mut DXVA2_ConfigPictureDecode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetDecoderConfigurations: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub CreateVideoDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DXVA2_VideoDesc, *const DXVA2_ConfigPictureDecode, *const *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    CreateVideoDecoder: usize,
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
pub trait IDirectXVideoDecoderService_Impl: IDirectXVideoAccelerationService_Impl {
    fn GetDecoderDeviceGuids(&self, pcount: *mut u32, pguids: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetDecoderRenderTargets(&self, guid: *const windows_core::GUID, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::Result<()>;
    fn GetDecoderConfigurations(&self, guid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, preserved: *const core::ffi::c_void, pcount: *mut u32, ppconfigs: *mut *mut DXVA2_ConfigPictureDecode) -> windows_core::Result<()>;
    fn CreateVideoDecoder(&self, guid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, pconfig: *const DXVA2_ConfigPictureDecode, ppdecoderrendertargets: *const Option<super::d3d9::IDirect3DSurface9>, numrendertargets: u32) -> windows_core::Result<IDirectXVideoDecoder>;
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl IDirectXVideoDecoderService_Vtbl {
    pub const fn new<Identity: IDirectXVideoDecoderService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDecoderDeviceGuids<Identity: IDirectXVideoDecoderService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32, pguids: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoderService_Impl::GetDecoderDeviceGuids(this, core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pguids)).into()
            }
        }
        unsafe extern "system" fn GetDecoderRenderTargets<Identity: IDirectXVideoDecoderService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoderService_Impl::GetDecoderRenderTargets(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pformats)).into()
            }
        }
        unsafe extern "system" fn GetDecoderConfigurations<Identity: IDirectXVideoDecoderService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, preserved: *const core::ffi::c_void, pcount: *mut u32, ppconfigs: *mut *mut DXVA2_ConfigPictureDecode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoDecoderService_Impl::GetDecoderConfigurations(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&preserved), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&ppconfigs)).into()
            }
        }
        unsafe extern "system" fn CreateVideoDecoder<Identity: IDirectXVideoDecoderService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, pconfig: *const DXVA2_ConfigPictureDecode, ppdecoderrendertargets: *const *mut core::ffi::c_void, numrendertargets: u32, ppdecode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoDecoderService_Impl::CreateVideoDecoder(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&pconfig), core::mem::transmute_copy(&ppdecoderrendertargets), core::mem::transmute_copy(&numrendertargets)) {
                    Ok(ok__) => {
                        ppdecode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDirectXVideoAccelerationService_Vtbl::new::<Identity, OFFSET>(),
            GetDecoderDeviceGuids: GetDecoderDeviceGuids::<Identity, OFFSET>,
            GetDecoderRenderTargets: GetDecoderRenderTargets::<Identity, OFFSET>,
            GetDecoderConfigurations: GetDecoderConfigurations::<Identity, OFFSET>,
            CreateVideoDecoder: CreateVideoDecoder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectXVideoDecoderService as windows_core::Interface>::IID || iid == &<IDirectXVideoAccelerationService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirectXVideoDecoderService {}
windows_core::imp::define_interface!(IDirectXVideoMemoryConfiguration, IDirectXVideoMemoryConfiguration_Vtbl, 0xb7f916dd_db3b_49c1_84d7_e45ef99ec726);
windows_core::imp::interface_hierarchy!(IDirectXVideoMemoryConfiguration, windows_core::IUnknown);
impl IDirectXVideoMemoryConfiguration {
    pub unsafe fn GetAvailableSurfaceTypeByIndex(&self, dwtypeindex: u32) -> windows_core::Result<DXVA2_SurfaceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableSurfaceTypeByIndex)(windows_core::Interface::as_raw(self), dwtypeindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSurfaceType(&self, dwtype: DXVA2_SurfaceType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSurfaceType)(windows_core::Interface::as_raw(self), dwtype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectXVideoMemoryConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableSurfaceTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXVA2_SurfaceType) -> windows_core::HRESULT,
    pub SetSurfaceType: unsafe extern "system" fn(*mut core::ffi::c_void, DXVA2_SurfaceType) -> windows_core::HRESULT,
}
pub trait IDirectXVideoMemoryConfiguration_Impl: windows_core::IUnknownImpl {
    fn GetAvailableSurfaceTypeByIndex(&self, dwtypeindex: u32) -> windows_core::Result<DXVA2_SurfaceType>;
    fn SetSurfaceType(&self, dwtype: DXVA2_SurfaceType) -> windows_core::Result<()>;
}
impl IDirectXVideoMemoryConfiguration_Vtbl {
    pub const fn new<Identity: IDirectXVideoMemoryConfiguration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailableSurfaceTypeByIndex<Identity: IDirectXVideoMemoryConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtypeindex: u32, pdwtype: *mut DXVA2_SurfaceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoMemoryConfiguration_Impl::GetAvailableSurfaceTypeByIndex(this, core::mem::transmute_copy(&dwtypeindex)) {
                    Ok(ok__) => {
                        pdwtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSurfaceType<Identity: IDirectXVideoMemoryConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: DXVA2_SurfaceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoMemoryConfiguration_Impl::SetSurfaceType(this, core::mem::transmute_copy(&dwtype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableSurfaceTypeByIndex: GetAvailableSurfaceTypeByIndex::<Identity, OFFSET>,
            SetSurfaceType: SetSurfaceType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectXVideoMemoryConfiguration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectXVideoMemoryConfiguration {}
windows_core::imp::define_interface!(IDirectXVideoProcessor, IDirectXVideoProcessor_Vtbl, 0x8c3a39f0_916e_4690_804f_4c8001355d25);
windows_core::imp::interface_hierarchy!(IDirectXVideoProcessor, windows_core::IUnknown);
impl IDirectXVideoProcessor {
    pub unsafe fn GetVideoProcessorService(&self) -> windows_core::Result<IDirectXVideoProcessorService> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoProcessorService)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetCreationParameters(&self, pdeviceguid: Option<*mut windows_core::GUID>, pvideodesc: Option<*mut DXVA2_VideoDesc>, prendertargetformat: Option<*mut super::d3d9::D3DFORMAT>, pmaxnumsubstreams: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCreationParameters)(windows_core::Interface::as_raw(self), pdeviceguid.unwrap_or(core::mem::zeroed()) as _, pvideodesc.unwrap_or(core::mem::zeroed()) as _, prendertargetformat.unwrap_or(core::mem::zeroed()) as _, pmaxnumsubstreams.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetVideoProcessorCaps(&self, pcaps: *mut DXVA2_VideoProcessorCaps) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorCaps)(windows_core::Interface::as_raw(self), pcaps as _) }
    }
    pub unsafe fn GetProcAmpRange(&self, procampcap: u32) -> windows_core::Result<DXVA2_ValueRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcAmpRange)(windows_core::Interface::as_raw(self), procampcap, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFilterPropertyRange(&self, filtersetting: u32) -> windows_core::Result<DXVA2_ValueRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilterPropertyRange)(windows_core::Interface::as_raw(self), filtersetting, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_winnt"))]
    pub unsafe fn VideoProcessBlt<P0>(&self, prendertarget: P0, pbltparams: *const DXVA2_VideoProcessBltParams, psamples: &[DXVA2_VideoSample], phandlecomplete: Option<*mut super::winnt::HANDLE>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d9::IDirect3DSurface9>,
    {
        unsafe { (windows_core::Interface::vtable(self).VideoProcessBlt)(windows_core::Interface::as_raw(self), prendertarget.param().abi(), pbltparams, core::mem::transmute(psamples.as_ptr()), psamples.len().try_into().unwrap(), phandlecomplete.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectXVideoProcessor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVideoProcessorService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9")]
    pub GetCreationParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut DXVA2_VideoDesc, *mut super::d3d9::D3DFORMAT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetCreationParameters: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub GetVideoProcessorCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXVA2_VideoProcessorCaps) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetVideoProcessorCaps: usize,
    pub GetProcAmpRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXVA2_ValueRange) -> windows_core::HRESULT,
    pub GetFilterPropertyRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DXVA2_ValueRange) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_winnt"))]
    pub VideoProcessBlt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DXVA2_VideoProcessBltParams, *const DXVA2_VideoSample, u32, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_winnt")))]
    VideoProcessBlt: usize,
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDirectXVideoProcessor_Impl: windows_core::IUnknownImpl {
    fn GetVideoProcessorService(&self) -> windows_core::Result<IDirectXVideoProcessorService>;
    fn GetCreationParameters(&self, pdeviceguid: *mut windows_core::GUID, pvideodesc: *mut DXVA2_VideoDesc, prendertargetformat: *mut super::d3d9::D3DFORMAT, pmaxnumsubstreams: *mut u32) -> windows_core::Result<()>;
    fn GetVideoProcessorCaps(&self, pcaps: *mut DXVA2_VideoProcessorCaps) -> windows_core::Result<()>;
    fn GetProcAmpRange(&self, procampcap: u32) -> windows_core::Result<DXVA2_ValueRange>;
    fn GetFilterPropertyRange(&self, filtersetting: u32) -> windows_core::Result<DXVA2_ValueRange>;
    fn VideoProcessBlt(&self, prendertarget: windows_core::Ref<super::d3d9::IDirect3DSurface9>, pbltparams: *const DXVA2_VideoProcessBltParams, psamples: *const DXVA2_VideoSample, numsamples: u32, phandlecomplete: *mut super::winnt::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDirectXVideoProcessor_Vtbl {
    pub const fn new<Identity: IDirectXVideoProcessor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVideoProcessorService<Identity: IDirectXVideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoProcessor_Impl::GetVideoProcessorService(this) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCreationParameters<Identity: IDirectXVideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceguid: *mut windows_core::GUID, pvideodesc: *mut DXVA2_VideoDesc, prendertargetformat: *mut super::d3d9::D3DFORMAT, pmaxnumsubstreams: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoProcessor_Impl::GetCreationParameters(this, core::mem::transmute_copy(&pdeviceguid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&prendertargetformat), core::mem::transmute_copy(&pmaxnumsubstreams)).into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Identity: IDirectXVideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut DXVA2_VideoProcessorCaps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoProcessor_Impl::GetVideoProcessorCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetProcAmpRange<Identity: IDirectXVideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, procampcap: u32, prange: *mut DXVA2_ValueRange) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoProcessor_Impl::GetProcAmpRange(this, core::mem::transmute_copy(&procampcap)) {
                    Ok(ok__) => {
                        prange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilterPropertyRange<Identity: IDirectXVideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filtersetting: u32, prange: *mut DXVA2_ValueRange) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoProcessor_Impl::GetFilterPropertyRange(this, core::mem::transmute_copy(&filtersetting)) {
                    Ok(ok__) => {
                        prange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VideoProcessBlt<Identity: IDirectXVideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertarget: *mut core::ffi::c_void, pbltparams: *const DXVA2_VideoProcessBltParams, psamples: *const DXVA2_VideoSample, numsamples: u32, phandlecomplete: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoProcessor_Impl::VideoProcessBlt(this, core::mem::transmute_copy(&prendertarget), core::mem::transmute_copy(&pbltparams), core::mem::transmute_copy(&psamples), core::mem::transmute_copy(&numsamples), core::mem::transmute_copy(&phandlecomplete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVideoProcessorService: GetVideoProcessorService::<Identity, OFFSET>,
            GetCreationParameters: GetCreationParameters::<Identity, OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Identity, OFFSET>,
            GetProcAmpRange: GetProcAmpRange::<Identity, OFFSET>,
            GetFilterPropertyRange: GetFilterPropertyRange::<Identity, OFFSET>,
            VideoProcessBlt: VideoProcessBlt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectXVideoProcessor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirectXVideoProcessor {}
windows_core::imp::define_interface!(IDirectXVideoProcessorService, IDirectXVideoProcessorService_Vtbl, 0xfc51a552_d5e7_11d9_af55_00054e43ff02);
impl core::ops::Deref for IDirectXVideoProcessorService {
    type Target = IDirectXVideoAccelerationService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectXVideoProcessorService, windows_core::IUnknown, IDirectXVideoAccelerationService);
impl IDirectXVideoProcessorService {
    pub unsafe fn RegisterVideoProcessorSoftwareDevice(&self, pcallbacks: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterVideoProcessorSoftwareDevice)(windows_core::Interface::as_raw(self), pcallbacks) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetVideoProcessorDeviceGuids(&self, pvideodesc: *const DXVA2_VideoDesc, pcount: *mut u32, pguids: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorDeviceGuids)(windows_core::Interface::as_raw(self), pvideodesc, pcount as _, pguids as _) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetVideoProcessorRenderTargets(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorRenderTargets)(windows_core::Interface::as_raw(self), videoprocdeviceguid, pvideodesc, pcount as _, pformats as _) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetVideoProcessorSubStreamFormats(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorSubStreamFormats)(windows_core::Interface::as_raw(self), videoprocdeviceguid, pvideodesc, rendertargetformat, pcount as _, pformats as _) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetVideoProcessorCaps(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, pcaps: *mut DXVA2_VideoProcessorCaps) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoProcessorCaps)(windows_core::Interface::as_raw(self), videoprocdeviceguid, pvideodesc, rendertargetformat, pcaps as _) }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetProcAmpRange(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, procampcap: u32) -> windows_core::Result<DXVA2_ValueRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcAmpRange)(windows_core::Interface::as_raw(self), videoprocdeviceguid, pvideodesc, rendertargetformat, procampcap, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn GetFilterPropertyRange(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, filtersetting: u32) -> windows_core::Result<DXVA2_ValueRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilterPropertyRange)(windows_core::Interface::as_raw(self), videoprocdeviceguid, pvideodesc, rendertargetformat, filtersetting, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_d3d9")]
    pub unsafe fn CreateVideoProcessor(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, maxnumsubstreams: u32) -> windows_core::Result<IDirectXVideoProcessor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVideoProcessor)(windows_core::Interface::as_raw(self), videoprocdeviceguid, pvideodesc, rendertargetformat, maxnumsubstreams, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectXVideoProcessorService_Vtbl {
    pub base__: IDirectXVideoAccelerationService_Vtbl,
    pub RegisterVideoProcessorSoftwareDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_d3d9")]
    pub GetVideoProcessorDeviceGuids: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXVA2_VideoDesc, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetVideoProcessorDeviceGuids: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub GetVideoProcessorRenderTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DXVA2_VideoDesc, *mut u32, *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetVideoProcessorRenderTargets: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub GetVideoProcessorSubStreamFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DXVA2_VideoDesc, super::d3d9::D3DFORMAT, *mut u32, *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetVideoProcessorSubStreamFormats: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub GetVideoProcessorCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DXVA2_VideoDesc, super::d3d9::D3DFORMAT, *mut DXVA2_VideoProcessorCaps) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetVideoProcessorCaps: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub GetProcAmpRange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DXVA2_VideoDesc, super::d3d9::D3DFORMAT, u32, *mut DXVA2_ValueRange) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetProcAmpRange: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub GetFilterPropertyRange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DXVA2_VideoDesc, super::d3d9::D3DFORMAT, u32, *mut DXVA2_ValueRange) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    GetFilterPropertyRange: usize,
    #[cfg(feature = "Win32_d3d9")]
    pub CreateVideoProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const DXVA2_VideoDesc, super::d3d9::D3DFORMAT, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d3d9"))]
    CreateVideoProcessor: usize,
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
pub trait IDirectXVideoProcessorService_Impl: IDirectXVideoAccelerationService_Impl {
    fn RegisterVideoProcessorSoftwareDevice(&self, pcallbacks: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetVideoProcessorDeviceGuids(&self, pvideodesc: *const DXVA2_VideoDesc, pcount: *mut u32, pguids: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetVideoProcessorRenderTargets(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::Result<()>;
    fn GetVideoProcessorSubStreamFormats(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::Result<()>;
    fn GetVideoProcessorCaps(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, pcaps: *mut DXVA2_VideoProcessorCaps) -> windows_core::Result<()>;
    fn GetProcAmpRange(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, procampcap: u32) -> windows_core::Result<DXVA2_ValueRange>;
    fn GetFilterPropertyRange(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, filtersetting: u32) -> windows_core::Result<DXVA2_ValueRange>;
    fn CreateVideoProcessor(&self, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, maxnumsubstreams: u32) -> windows_core::Result<IDirectXVideoProcessor>;
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl IDirectXVideoProcessorService_Vtbl {
    pub const fn new<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterVideoProcessorSoftwareDevice<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallbacks: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoProcessorService_Impl::RegisterVideoProcessorSoftwareDevice(this, core::mem::transmute_copy(&pcallbacks)).into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorDeviceGuids<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideodesc: *const DXVA2_VideoDesc, pcount: *mut u32, pguids: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoProcessorService_Impl::GetVideoProcessorDeviceGuids(this, core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pguids)).into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorRenderTargets<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoProcessorService_Impl::GetVideoProcessorRenderTargets(this, core::mem::transmute_copy(&videoprocdeviceguid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pformats)).into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorSubStreamFormats<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, pcount: *mut u32, pformats: *mut *mut super::d3d9::D3DFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoProcessorService_Impl::GetVideoProcessorSubStreamFormats(this, core::mem::transmute_copy(&videoprocdeviceguid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pformats)).into()
            }
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, pcaps: *mut DXVA2_VideoProcessorCaps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectXVideoProcessorService_Impl::GetVideoProcessorCaps(this, core::mem::transmute_copy(&videoprocdeviceguid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetProcAmpRange<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, procampcap: u32, prange: *mut DXVA2_ValueRange) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoProcessorService_Impl::GetProcAmpRange(this, core::mem::transmute_copy(&videoprocdeviceguid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&procampcap)) {
                    Ok(ok__) => {
                        prange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilterPropertyRange<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, filtersetting: u32, prange: *mut DXVA2_ValueRange) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoProcessorService_Impl::GetFilterPropertyRange(this, core::mem::transmute_copy(&videoprocdeviceguid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&filtersetting)) {
                    Ok(ok__) => {
                        prange.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateVideoProcessor<Identity: IDirectXVideoProcessorService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, videoprocdeviceguid: *const windows_core::GUID, pvideodesc: *const DXVA2_VideoDesc, rendertargetformat: super::d3d9::D3DFORMAT, maxnumsubstreams: u32, ppvidprocess: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectXVideoProcessorService_Impl::CreateVideoProcessor(this, core::mem::transmute_copy(&videoprocdeviceguid), core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&rendertargetformat), core::mem::transmute_copy(&maxnumsubstreams)) {
                    Ok(ok__) => {
                        ppvidprocess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDirectXVideoAccelerationService_Vtbl::new::<Identity, OFFSET>(),
            RegisterVideoProcessorSoftwareDevice: RegisterVideoProcessorSoftwareDevice::<Identity, OFFSET>,
            GetVideoProcessorDeviceGuids: GetVideoProcessorDeviceGuids::<Identity, OFFSET>,
            GetVideoProcessorRenderTargets: GetVideoProcessorRenderTargets::<Identity, OFFSET>,
            GetVideoProcessorSubStreamFormats: GetVideoProcessorSubStreamFormats::<Identity, OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Identity, OFFSET>,
            GetProcAmpRange: GetProcAmpRange::<Identity, OFFSET>,
            GetFilterPropertyRange: GetFilterPropertyRange::<Identity, OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectXVideoProcessorService as windows_core::Interface>::IID || iid == &<IDirectXVideoAccelerationService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d9", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDirectXVideoProcessorService {}
pub const MAX_DEINTERLACE_SURFACES: u32 = 32;
pub const MAX_SUBSTREAMS: u32 = 15;
