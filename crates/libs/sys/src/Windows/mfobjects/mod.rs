#[cfg(feature = "objidlbase")]
windows_link::link!("mfplat.dll" "system" fn MFDeserializeAttributesFromStream(pattr : *mut core::ffi::c_void, dwoptions : u32, pstm : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("mfplat.dll" "system" fn MFSerializeAttributesToStream(pattr : *mut core::ffi::c_void, dwoptions : u32, pstm : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const MEAudioSessionDeviceRemoved: i32 = 315;
pub const MEAudioSessionDisconnected: i32 = 320;
pub const MEAudioSessionExclusiveModeOverride: i32 = 321;
pub const MEAudioSessionFormatChanged: i32 = 319;
pub const MEAudioSessionGroupingParamChanged: i32 = 317;
pub const MEAudioSessionIconChanged: i32 = 318;
pub const MEAudioSessionNameChanged: i32 = 313;
pub const MEAudioSessionServerShutdown: i32 = 316;
pub const MEAudioSessionVolumeChanged: i32 = 314;
pub const MEBufferingStarted: i32 = 122;
pub const MEBufferingStopped: i32 = 123;
pub const MEByteStreamCharacteristicsChanged: i32 = 700;
pub const MECaptureAudioSessionDeviceRemoved: i32 = 323;
pub const MECaptureAudioSessionDisconnected: i32 = 325;
pub const MECaptureAudioSessionExclusiveModeOverride: i32 = 326;
pub const MECaptureAudioSessionFormatChanged: i32 = 324;
pub const MECaptureAudioSessionServerShutdown: i32 = 327;
pub const MECaptureAudioSessionVolumeChanged: i32 = 322;
pub const MEConnectEnd: i32 = 125;
pub const MEConnectStart: i32 = 124;
pub const MEContentProtectionMessage: i32 = 402;
pub const MEContentProtectionMetadata: i32 = 900;
pub const MEDeviceThermalStateChanged: i32 = 950;
pub const MEEnablerCompleted: i32 = 119;
pub const MEEnablerProgress: i32 = 118;
pub const MEEncodingParameters: i32 = 803;
pub const MEEndOfPresentation: i32 = 211;
pub const MEEndOfPresentationSegment: i32 = 218;
pub const MEEndOfStream: i32 = 212;
pub const MEError: i32 = 1;
pub const MEExtendedType: i32 = 2;
pub const MEGenericV1Anchor: i32 = 3;
pub const MEIndividualizationCompleted: i32 = 117;
pub const MEIndividualizationStart: i32 = 116;
pub const MELicenseAcquisitionCompleted: i32 = 115;
pub const MELicenseAcquisitionStart: i32 = 114;
pub const MEMediaSample: i32 = 213;
pub const MENewPresentation: i32 = 113;
pub const MENewStream: i32 = 205;
pub const MENonFatalError: i32 = 3;
pub const MEPolicyChanged: i32 = 401;
pub const MEPolicyError: i32 = 120;
pub const MEPolicyReport: i32 = 121;
pub const MEPolicySet: i32 = 403;
pub const MEQualityNotify: i32 = 311;
pub const MEReconnectEnd: i32 = 127;
pub const MEReconnectStart: i32 = 126;
pub const MERendererEvent: i32 = 128;
pub const MEReservedMax: i32 = 10000;
pub const MESequencerSourceTopologyUpdated: i32 = 222;
pub const MESessionCapabilitiesChanged: i32 = 110;
pub const MESessionClosed: i32 = 106;
pub const MESessionEnded: i32 = 107;
pub const MESessionNotifyPresentationTime: i32 = 112;
pub const MESessionPaused: i32 = 104;
pub const MESessionRateChanged: i32 = 108;
pub const MESessionScrubSampleComplete: i32 = 109;
pub const MESessionStarted: i32 = 103;
pub const MESessionStopped: i32 = 105;
pub const MESessionStreamSinkFormatChanged: i32 = 129;
pub const MESessionTopologiesCleared: i32 = 102;
pub const MESessionTopologySet: i32 = 101;
pub const MESessionTopologyStatus: i32 = 111;
pub const MESessionUnknown: i32 = 100;
pub const MESessionV1Anchor: i32 = 129;
pub const MESinkInvalidated: i32 = 312;
pub const MESinkUnknown: i32 = 300;
pub const MESinkV1Anchor: i32 = 321;
pub const MESinkV2Anchor: i32 = 327;
pub const MESourceCharacteristicsChanged: i32 = 219;
pub const MESourceMetadataChanged: i32 = 221;
pub const MESourcePaused: i32 = 209;
pub const MESourceRateChangeRequested: i32 = 220;
pub const MESourceRateChanged: i32 = 217;
pub const MESourceSeeked: i32 = 203;
pub const MESourceStarted: i32 = 201;
pub const MESourceStopped: i32 = 207;
pub const MESourceUnknown: i32 = 200;
pub const MESourceV1Anchor: i32 = 222;
pub const MEStreamFormatChanged: i32 = 216;
pub const MEStreamPaused: i32 = 210;
pub const MEStreamSeeked: i32 = 204;
pub const MEStreamSinkDeviceChanged: i32 = 310;
pub const MEStreamSinkFormatChanged: i32 = 309;
pub const MEStreamSinkFormatInvalidated: i32 = 802;
pub const MEStreamSinkMarker: i32 = 306;
pub const MEStreamSinkPaused: i32 = 303;
pub const MEStreamSinkPrerolled: i32 = 307;
pub const MEStreamSinkRateChanged: i32 = 304;
pub const MEStreamSinkRequestSample: i32 = 305;
pub const MEStreamSinkScrubSampleComplete: i32 = 308;
pub const MEStreamSinkStarted: i32 = 301;
pub const MEStreamSinkStopped: i32 = 302;
pub const MEStreamStarted: i32 = 202;
pub const MEStreamStopped: i32 = 208;
pub const MEStreamThinMode: i32 = 215;
pub const MEStreamTick: i32 = 214;
pub const METransformDrainComplete: i32 = 603;
pub const METransformHaveOutput: i32 = 602;
pub const METransformInputStreamStateChanged: i32 = 605;
pub const METransformMarker: i32 = 604;
pub const METransformNeedInput: i32 = 601;
pub const METransformUnknown: i32 = 600;
pub const METrustUnknown: i32 = 400;
pub const METrustV1Anchor: i32 = 403;
pub const MEUnknown: i32 = 0;
pub const MEUpdatedStream: i32 = 206;
pub const MEVideoCaptureDevicePreempted: i32 = 801;
pub const MEVideoCaptureDeviceRemoved: i32 = 800;
pub const MEWMDRMIndividualizationCompleted: i32 = 508;
pub const MEWMDRMIndividualizationProgress: i32 = 513;
pub const MEWMDRMLicenseAcquisitionCompleted: i32 = 506;
pub const MEWMDRMLicenseBackupCompleted: i32 = 500;
pub const MEWMDRMLicenseBackupProgress: i32 = 501;
pub const MEWMDRMLicenseRestoreCompleted: i32 = 502;
pub const MEWMDRMLicenseRestoreProgress: i32 = 503;
pub const MEWMDRMLicenseStoreCleaned: i32 = 515;
pub const MEWMDRMProximityCompleted: i32 = 514;
pub const MEWMDRMRevocationDownloadCompleted: i32 = 516;
pub const MEWMDRMV1Anchor: i32 = 516;
pub type MF2DBuffer_LockFlags = i32;
pub const MF2DBuffer_LockFlags_ForceDWORD: MF2DBuffer_LockFlags = 2147483647;
pub const MF2DBuffer_LockFlags_LockTypeMask: MF2DBuffer_LockFlags = 3;
pub const MF2DBuffer_LockFlags_Read: MF2DBuffer_LockFlags = 1;
pub const MF2DBuffer_LockFlags_ReadWrite: MF2DBuffer_LockFlags = 3;
pub const MF2DBuffer_LockFlags_Write: MF2DBuffer_LockFlags = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFARGB {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbAlpha: u8,
}
pub const MFASYNC_BLOCKING_CALLBACK: u32 = 4;
pub const MFASYNC_CALLBACK_QUEUE_ALL: u32 = 4294967295;
pub const MFASYNC_CALLBACK_QUEUE_IO: u32 = 3;
pub const MFASYNC_CALLBACK_QUEUE_LONG_FUNCTION: u32 = 7;
pub const MFASYNC_CALLBACK_QUEUE_MULTITHREADED: u32 = 5;
pub const MFASYNC_CALLBACK_QUEUE_PRIVATE_MASK: u32 = 4294901760;
pub const MFASYNC_CALLBACK_QUEUE_RT: u32 = 2;
pub const MFASYNC_CALLBACK_QUEUE_STANDARD: u32 = 1;
pub const MFASYNC_CALLBACK_QUEUE_TIMER: u32 = 4;
pub const MFASYNC_CALLBACK_QUEUE_UNDEFINED: u32 = 0;
pub const MFASYNC_FAST_IO_PROCESSING_CALLBACK: u32 = 1;
pub const MFASYNC_LOCALIZE_REMOTE_CALLBACK: u32 = 16;
pub const MFASYNC_REPLY_CALLBACK: u32 = 8;
pub const MFASYNC_SIGNAL_CALLBACK: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFAYUVSample {
    pub bCrValue: u8,
    pub bCbValue: u8,
    pub bYValue: u8,
    pub bSampleAlpha8: u8,
}
pub const MFBYTESTREAM_DOES_NOT_USE_NETWORK: u32 = 2048;
pub const MFBYTESTREAM_HAS_SLOW_SEEK: u32 = 256;
pub const MFBYTESTREAM_IS_DIRECTORY: u32 = 128;
pub const MFBYTESTREAM_IS_PARTIALLY_DOWNLOADED: u32 = 512;
pub const MFBYTESTREAM_IS_READABLE: u32 = 1;
pub const MFBYTESTREAM_IS_REMOTE: u32 = 8;
pub const MFBYTESTREAM_IS_SEEKABLE: u32 = 4;
pub const MFBYTESTREAM_IS_WRITABLE: u32 = 2;
pub const MFBYTESTREAM_SEEK_FLAG_CANCEL_PENDING_IO: u32 = 1;
pub type MFBYTESTREAM_SEEK_ORIGIN = i32;
pub const MFBYTESTREAM_SHARE_WRITE: u32 = 1024;
pub type MFNominalRange = i32;
pub const MFNominalRange_0_255: MFNominalRange = 1;
pub const MFNominalRange_16_235: MFNominalRange = 2;
pub const MFNominalRange_48_208: MFNominalRange = 3;
pub const MFNominalRange_64_127: MFNominalRange = 4;
pub const MFNominalRange_ForceDWORD: MFNominalRange = 2147483647;
pub const MFNominalRange_Last: MFNominalRange = 5;
pub const MFNominalRange_Normal: MFNominalRange = 1;
pub const MFNominalRange_Unknown: MFNominalRange = 0;
pub const MFNominalRange_Wide: MFNominalRange = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFOffset {
    pub fract: u16,
    pub value: i16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MFPaletteEntry {
    pub ARGB: MFARGB,
    pub AYCbCr: MFAYUVSample,
}
impl Default for MFPaletteEntry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFRatio {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub type MFStandardVideoFormat = i32;
pub const MFStdVideoFormat_ATSC_HD1080i: MFStandardVideoFormat = 8;
pub const MFStdVideoFormat_ATSC_HD720p: MFStandardVideoFormat = 9;
pub const MFStdVideoFormat_ATSC_SD480i: MFStandardVideoFormat = 7;
pub const MFStdVideoFormat_DVD_NTSC: MFStandardVideoFormat = 3;
pub const MFStdVideoFormat_DVD_PAL: MFStandardVideoFormat = 4;
pub const MFStdVideoFormat_DV_NTSC: MFStandardVideoFormat = 6;
pub const MFStdVideoFormat_DV_PAL: MFStandardVideoFormat = 5;
pub const MFStdVideoFormat_NTSC: MFStandardVideoFormat = 1;
pub const MFStdVideoFormat_PAL: MFStandardVideoFormat = 2;
pub const MFStdVideoFormat_reserved: MFStandardVideoFormat = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFT_REGISTER_TYPE_INFO {
    pub guidMajorType: windows_sys::core::GUID,
    pub guidSubtype: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct MFVIDEOFORMAT {
    pub dwSize: u32,
    pub videoInfo: MFVideoInfo,
    pub guidFormat: windows_sys::core::GUID,
    pub compressedInfo: MFVideoCompressedInfo,
    pub surfaceInfo: MFVideoSurfaceInfo,
}
#[cfg(feature = "windef")]
impl Default for MFVIDEOFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MFVideoArea {
    pub OffsetX: MFOffset,
    pub OffsetY: MFOffset,
    pub Area: super::windef::SIZE,
}
pub type MFVideoChromaSubsampling = i32;
pub const MFVideoChromaSubsampling_Cosited: MFVideoChromaSubsampling = 7;
pub const MFVideoChromaSubsampling_DV_PAL: MFVideoChromaSubsampling = 6;
pub const MFVideoChromaSubsampling_ForceDWORD: MFVideoChromaSubsampling = 2147483647;
pub const MFVideoChromaSubsampling_Horizontally_Cosited: MFVideoChromaSubsampling = 4;
pub const MFVideoChromaSubsampling_Last: MFVideoChromaSubsampling = 8;
pub const MFVideoChromaSubsampling_MPEG1: MFVideoChromaSubsampling = 1;
pub const MFVideoChromaSubsampling_MPEG2: MFVideoChromaSubsampling = 5;
pub const MFVideoChromaSubsampling_ProgressiveChroma: MFVideoChromaSubsampling = 8;
pub const MFVideoChromaSubsampling_Unknown: MFVideoChromaSubsampling = 0;
pub const MFVideoChromaSubsampling_Vertically_AlignedChromaPlanes: MFVideoChromaSubsampling = 1;
pub const MFVideoChromaSubsampling_Vertically_Cosited: MFVideoChromaSubsampling = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFVideoCompressedInfo {
    pub AvgBitrate: i64,
    pub AvgBitErrorRate: i64,
    pub MaxKeyFrameSpacing: u32,
}
pub const MFVideoFlag_AnalogProtected: MFVideoFlags = 32;
pub const MFVideoFlag_BottomUpLinearRep: MFVideoFlags = 524288;
pub const MFVideoFlag_DigitallyProtected: MFVideoFlags = 64;
pub const MFVideoFlag_FieldRepeatCountMask: MFVideoFlags = 1792;
pub const MFVideoFlag_FieldRepeatCountShift: MFVideoFlags = 8;
pub const MFVideoFlag_LowerFieldFirst: MFVideoFlags = 262144;
pub const MFVideoFlag_PAD_TO_16x9: MFVideoFlags = 2;
pub const MFVideoFlag_PAD_TO_4x3: MFVideoFlags = 1;
pub const MFVideoFlag_PAD_TO_Mask: MFVideoFlags = 3;
pub const MFVideoFlag_PAD_TO_None: MFVideoFlags = 0;
pub const MFVideoFlag_PanScanEnabled: MFVideoFlags = 131072;
pub const MFVideoFlag_ProgressiveContent: MFVideoFlags = 128;
pub const MFVideoFlag_ProgressiveSeqReset: MFVideoFlags = 2048;
pub const MFVideoFlag_SrcContentHint16x9: MFVideoFlags = 4;
pub const MFVideoFlag_SrcContentHint235_1: MFVideoFlags = 8;
pub const MFVideoFlag_SrcContentHintMask: MFVideoFlags = 28;
pub const MFVideoFlag_SrcContentHintNone: MFVideoFlags = 0;
pub type MFVideoFlags = i32;
pub const MFVideoFlags_DXVASurface: MFVideoFlags = 1048576;
pub const MFVideoFlags_ForceQWORD: MFVideoFlags = 2147483647;
pub const MFVideoFlags_RenderTargetSurface: MFVideoFlags = 4194304;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct MFVideoInfo {
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub PixelAspectRatio: MFRatio,
    pub SourceChromaSubsampling: MFVideoChromaSubsampling,
    pub InterlaceMode: MFVideoInterlaceMode,
    pub TransferFunction: MFVideoTransferFunction,
    pub ColorPrimaries: MFVideoPrimaries,
    pub TransferMatrix: MFVideoTransferMatrix,
    pub SourceLighting: MFVideoLighting,
    pub FramesPerSecond: MFRatio,
    pub NominalRange: MFNominalRange,
    pub GeometricAperture: MFVideoArea,
    pub MinimumDisplayAperture: MFVideoArea,
    pub PanScanAperture: MFVideoArea,
    pub VideoFlags: u64,
}
pub type MFVideoInterlaceMode = i32;
pub const MFVideoInterlace_FieldInterleavedLowerFirst: MFVideoInterlaceMode = 4;
pub const MFVideoInterlace_FieldInterleavedUpperFirst: MFVideoInterlaceMode = 3;
pub const MFVideoInterlace_FieldSingleLower: MFVideoInterlaceMode = 6;
pub const MFVideoInterlace_FieldSingleLowerFirst: u32 = 6;
pub const MFVideoInterlace_FieldSingleUpper: MFVideoInterlaceMode = 5;
pub const MFVideoInterlace_FieldSingleUpperFirst: u32 = 5;
pub const MFVideoInterlace_ForceDWORD: MFVideoInterlaceMode = 2147483647;
pub const MFVideoInterlace_Last: MFVideoInterlaceMode = 8;
pub const MFVideoInterlace_MixedInterlaceOrProgressive: MFVideoInterlaceMode = 7;
pub const MFVideoInterlace_Progressive: MFVideoInterlaceMode = 2;
pub const MFVideoInterlace_Unknown: MFVideoInterlaceMode = 0;
pub type MFVideoLighting = i32;
pub const MFVideoLighting_ForceDWORD: MFVideoLighting = 2147483647;
pub const MFVideoLighting_Last: MFVideoLighting = 5;
pub const MFVideoLighting_Unknown: MFVideoLighting = 0;
pub const MFVideoLighting_bright: MFVideoLighting = 1;
pub const MFVideoLighting_dark: MFVideoLighting = 4;
pub const MFVideoLighting_dim: MFVideoLighting = 3;
pub const MFVideoLighting_office: MFVideoLighting = 2;
pub type MFVideoPrimaries = i32;
pub const MFVideoPrimaries_ACES: MFVideoPrimaries = 12;
pub const MFVideoPrimaries_BT2020: MFVideoPrimaries = 9;
pub const MFVideoPrimaries_BT470_2_SysBG: MFVideoPrimaries = 4;
pub const MFVideoPrimaries_BT470_2_SysM: MFVideoPrimaries = 3;
pub const MFVideoPrimaries_BT709: MFVideoPrimaries = 2;
pub const MFVideoPrimaries_DCI_P3: MFVideoPrimaries = 11;
pub const MFVideoPrimaries_Display_P3: MFVideoPrimaries = 13;
pub const MFVideoPrimaries_EBU3213: MFVideoPrimaries = 7;
pub const MFVideoPrimaries_ForceDWORD: MFVideoPrimaries = 2147483647;
pub const MFVideoPrimaries_Last: MFVideoPrimaries = 14;
pub const MFVideoPrimaries_SMPTE170M: MFVideoPrimaries = 5;
pub const MFVideoPrimaries_SMPTE240M: MFVideoPrimaries = 6;
pub const MFVideoPrimaries_SMPTE_C: MFVideoPrimaries = 8;
pub const MFVideoPrimaries_Unknown: MFVideoPrimaries = 0;
pub const MFVideoPrimaries_XYZ: MFVideoPrimaries = 10;
pub const MFVideoPrimaries_reserved: MFVideoPrimaries = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFVideoSurfaceInfo {
    pub Format: u32,
    pub PaletteEntries: u32,
    pub Palette: [MFPaletteEntry; 1],
}
impl Default for MFVideoSurfaceInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFVideoTransFunc_10: MFVideoTransferFunction = 1;
pub const MFVideoTransFunc_10_rel: MFVideoTransferFunction = 17;
pub const MFVideoTransFunc_18: MFVideoTransferFunction = 2;
pub const MFVideoTransFunc_20: MFVideoTransferFunction = 3;
pub const MFVideoTransFunc_2020: MFVideoTransferFunction = 13;
pub const MFVideoTransFunc_2020_const: MFVideoTransferFunction = 12;
pub const MFVideoTransFunc_2084: MFVideoTransferFunction = 15;
pub const MFVideoTransFunc_22: MFVideoTransferFunction = 4;
pub const MFVideoTransFunc_240M: MFVideoTransferFunction = 6;
pub const MFVideoTransFunc_26: MFVideoTransferFunction = 14;
pub const MFVideoTransFunc_28: MFVideoTransferFunction = 8;
pub const MFVideoTransFunc_709: MFVideoTransferFunction = 5;
pub const MFVideoTransFunc_709_sym: MFVideoTransferFunction = 11;
pub const MFVideoTransFunc_BT1361_ECG: MFVideoTransferFunction = 18;
pub const MFVideoTransFunc_ForceDWORD: MFVideoTransferFunction = 2147483647;
pub const MFVideoTransFunc_HLG: MFVideoTransferFunction = 16;
pub const MFVideoTransFunc_Last: MFVideoTransferFunction = 20;
pub const MFVideoTransFunc_Log_100: MFVideoTransferFunction = 9;
pub const MFVideoTransFunc_Log_316: MFVideoTransferFunction = 10;
pub const MFVideoTransFunc_SMPTE428: MFVideoTransferFunction = 19;
pub const MFVideoTransFunc_Unknown: MFVideoTransferFunction = 0;
pub const MFVideoTransFunc_sRGB: MFVideoTransferFunction = 7;
pub type MFVideoTransferFunction = i32;
pub type MFVideoTransferMatrix = i32;
pub const MFVideoTransferMatrix_BT2020_10: MFVideoTransferMatrix = 4;
pub const MFVideoTransferMatrix_BT2020_12: MFVideoTransferMatrix = 5;
pub const MFVideoTransferMatrix_BT601: MFVideoTransferMatrix = 2;
pub const MFVideoTransferMatrix_BT709: MFVideoTransferMatrix = 1;
pub const MFVideoTransferMatrix_Chroma: MFVideoTransferMatrix = 10;
pub const MFVideoTransferMatrix_Chroma_const: MFVideoTransferMatrix = 11;
pub const MFVideoTransferMatrix_FCC47: MFVideoTransferMatrix = 7;
pub const MFVideoTransferMatrix_ForceDWORD: MFVideoTransferMatrix = 2147483647;
pub const MFVideoTransferMatrix_ICtCp: MFVideoTransferMatrix = 12;
pub const MFVideoTransferMatrix_Identity: MFVideoTransferMatrix = 6;
pub const MFVideoTransferMatrix_Last: MFVideoTransferMatrix = 13;
pub const MFVideoTransferMatrix_SMPTE2085: MFVideoTransferMatrix = 9;
pub const MFVideoTransferMatrix_SMPTE240M: MFVideoTransferMatrix = 3;
pub const MFVideoTransferMatrix_Unknown: MFVideoTransferMatrix = 0;
pub const MFVideoTransferMatrix_YCgCo: MFVideoTransferMatrix = 8;
pub const MF_ACCESSMODE_READ: MF_FILE_ACCESSMODE = 1;
pub const MF_ACCESSMODE_READWRITE: MF_FILE_ACCESSMODE = 3;
pub const MF_ACCESSMODE_WRITE: MF_FILE_ACCESSMODE = 2;
pub const MF_ATTRIBUTES_MATCH_ALL_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 2;
pub const MF_ATTRIBUTES_MATCH_INTERSECTION: MF_ATTRIBUTES_MATCH_TYPE = 3;
pub const MF_ATTRIBUTES_MATCH_OUR_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 0;
pub const MF_ATTRIBUTES_MATCH_SMALLER: MF_ATTRIBUTES_MATCH_TYPE = 4;
pub const MF_ATTRIBUTES_MATCH_THEIR_ITEMS: MF_ATTRIBUTES_MATCH_TYPE = 1;
pub type MF_ATTRIBUTES_MATCH_TYPE = i32;
pub const MF_ATTRIBUTE_BLOB: MF_ATTRIBUTE_TYPE = 4113;
pub const MF_ATTRIBUTE_DOUBLE: MF_ATTRIBUTE_TYPE = 5;
pub const MF_ATTRIBUTE_GUID: MF_ATTRIBUTE_TYPE = 72;
pub const MF_ATTRIBUTE_IUNKNOWN: MF_ATTRIBUTE_TYPE = 13;
pub type MF_ATTRIBUTE_SERIALIZE_OPTIONS = i32;
pub const MF_ATTRIBUTE_SERIALIZE_UNKNOWN_BYREF: MF_ATTRIBUTE_SERIALIZE_OPTIONS = 1;
pub const MF_ATTRIBUTE_STRING: MF_ATTRIBUTE_TYPE = 31;
pub type MF_ATTRIBUTE_TYPE = i32;
pub const MF_ATTRIBUTE_UINT32: MF_ATTRIBUTE_TYPE = 19;
pub const MF_ATTRIBUTE_UINT64: MF_ATTRIBUTE_TYPE = 21;
pub type MF_DXGI_DEVICE_MANAGER_MODE = i32;
pub const MF_DXGI_DEVICE_MANAGER_MODE_D3D11: MF_DXGI_DEVICE_MANAGER_MODE = 1;
pub const MF_DXGI_DEVICE_MANAGER_MODE_D3D12: MF_DXGI_DEVICE_MANAGER_MODE = 2;
pub const MF_DXGI_DEVICE_MANAGER_MODE_INVALID: MF_DXGI_DEVICE_MANAGER_MODE = 0;
pub const MF_EVENT_FLAG_NO_WAIT: u32 = 1;
pub const MF_FILEFLAGS_ALLOW_WRITE_SHARING: MF_FILE_FLAGS = 2;
pub const MF_FILEFLAGS_NOBUFFERING: MF_FILE_FLAGS = 1;
pub const MF_FILEFLAGS_NONE: MF_FILE_FLAGS = 0;
pub type MF_FILE_ACCESSMODE = i32;
pub type MF_FILE_FLAGS = i32;
pub type MF_FILE_OPENMODE = i32;
pub const MF_MEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4;
pub const MF_MEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2;
pub const MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8;
pub const MF_MEDIATYPE_EQUAL_MAJOR_TYPES: u32 = 1;
pub const MF_OPENMODE_APPEND_IF_EXIST: MF_FILE_OPENMODE = 3;
pub const MF_OPENMODE_DELETE_IF_EXIST: MF_FILE_OPENMODE = 4;
pub const MF_OPENMODE_FAIL_IF_EXIST: MF_FILE_OPENMODE = 1;
pub const MF_OPENMODE_FAIL_IF_NOT_EXIST: MF_FILE_OPENMODE = 0;
pub const MF_OPENMODE_RESET_IF_EXIST: MF_FILE_OPENMODE = 2;
pub type MF_PLUGIN_CONTROL_POLICY = i32;
pub const MF_PLUGIN_CONTROL_POLICY_USE_ALL_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 0;
pub const MF_PLUGIN_CONTROL_POLICY_USE_APPROVED_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 1;
pub const MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS: MF_PLUGIN_CONTROL_POLICY = 2;
pub const MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS_EDGEMODE: MF_PLUGIN_CONTROL_POLICY = 3;
pub type MF_Plugin_Type = i32;
pub const MF_Plugin_Type_MFT: MF_Plugin_Type = 0;
pub const MF_Plugin_Type_MFT_MatchOutputType: MF_Plugin_Type = 2;
pub const MF_Plugin_Type_MediaSource: MF_Plugin_Type = 1;
pub const MF_Plugin_Type_Other: MF_Plugin_Type = -1;
pub type MF_STREAM_STATE = i32;
pub const MF_STREAM_STATE_PAUSED: MF_STREAM_STATE = 1;
pub const MF_STREAM_STATE_RUNNING: MF_STREAM_STATE = 2;
pub const MF_STREAM_STATE_STOPPED: MF_STREAM_STATE = 0;
pub type MediaEventType = u32;
pub const msoBegin: MFBYTESTREAM_SEEK_ORIGIN = 0;
pub const msoCurrent: MFBYTESTREAM_SEEK_ORIGIN = 1;
