pub const ADVISE_ALL: u32 = 15;
pub const ADVISE_ALL2: u32 = 31;
pub const ADVISE_CLIPPING: i32 = 1;
pub const ADVISE_COLORKEY: i32 = 4;
pub const ADVISE_DISPLAY_CHANGE: i32 = 16;
pub const ADVISE_NONE: i32 = 0;
pub const ADVISE_PALETTE: i32 = 2;
pub const ADVISE_POSITION: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ALLOCATOR_PROPERTIES {
    pub cBuffers: i32,
    pub cbBuffer: i32,
    pub cbAlign: i32,
    pub cbPrefix: i32,
}
pub const AMAP_3D_TARGET: VMRSurfaceAllocationFlags = 2;
pub const AMAP_ALLOW_SYSMEM: VMRSurfaceAllocationFlags = 4;
pub const AMAP_DIRECTED_FLIP: VMRSurfaceAllocationFlags = 16;
pub const AMAP_DXVA_TARGET: VMRSurfaceAllocationFlags = 32;
pub const AMAP_FORCE_SYSMEM: VMRSurfaceAllocationFlags = 8;
pub const AMAP_PIXELFORMAT_VALID: VMRSurfaceAllocationFlags = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AMCOPPCommand {
    pub macKDI: windows_sys::core::GUID,
    pub guidCommandID: windows_sys::core::GUID,
    pub dwSequence: u32,
    pub cbSizeData: u32,
    pub CommandData: [u8; 4056],
}
impl Default for AMCOPPCommand {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AMCOPPSignature {
    pub Signature: [u8; 256],
}
impl Default for AMCOPPSignature {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AMCOPPStatusInput {
    pub rApp: windows_sys::core::GUID,
    pub guidStatusRequestID: windows_sys::core::GUID,
    pub dwSequence: u32,
    pub cbSizeData: u32,
    pub StatusData: [u8; 4056],
}
impl Default for AMCOPPStatusInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AMCOPPStatusOutput {
    pub macKDI: windows_sys::core::GUID,
    pub cbSizeData: u32,
    pub COPPStatus: [u8; 4076],
}
impl Default for AMCOPPStatusOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AMF_AUTOMATICGAIN: f64 = -1.0;
pub const AMOVERFX_DEINTERLACE: AMOVERLAYFX = 8;
pub const AMOVERFX_MIRRORLEFTRIGHT: AMOVERLAYFX = 2;
pub const AMOVERFX_MIRRORUPDOWN: AMOVERLAYFX = 4;
pub const AMOVERFX_NOFX: AMOVERLAYFX = 0;
pub type AMOVERLAYFX = i32;
pub type AMPROPERTY_PIN = i32;
pub const AMPROPERTY_PIN_CATEGORY: AMPROPERTY_PIN = 0;
pub const AMPROPERTY_PIN_MEDIUM: AMPROPERTY_PIN = 1;
pub const AMRESCTL_RESERVEFLAGS_RESERVE: _AMRESCTL_RESERVEFLAGS = 0;
pub const AMRESCTL_RESERVEFLAGS_UNRESERVE: _AMRESCTL_RESERVEFLAGS = 1;
pub const AMSTREAMSELECTENABLE_ENABLE: _AMSTREAMSELECTENABLEFLAGS = 1;
pub const AMSTREAMSELECTENABLE_ENABLEALL: _AMSTREAMSELECTENABLEFLAGS = 2;
pub const AMSTREAMSELECTINFO_ENABLED: _AMSTREAMSELECTINFOFLAGS = 1;
pub const AMSTREAMSELECTINFO_EXCLUSIVE: _AMSTREAMSELECTINFOFLAGS = 2;
pub const AMTUNER_EVENT_CHANGED: AMTunerEventType = 1;
pub const AMTUNER_HASNOSIGNALSTRENGTH: AMTunerSignalStrength = -1;
pub const AMTUNER_MODE_AM_RADIO: AMTunerModeType = 4;
pub const AMTUNER_MODE_DEFAULT: AMTunerModeType = 0;
pub const AMTUNER_MODE_DSS: AMTunerModeType = 8;
pub const AMTUNER_MODE_FM_RADIO: AMTunerModeType = 2;
pub const AMTUNER_MODE_TV: AMTunerModeType = 1;
pub const AMTUNER_NOSIGNAL: AMTunerSignalStrength = 0;
pub const AMTUNER_SIGNALPRESENT: AMTunerSignalStrength = 1;
pub const AMTUNER_SUBCHAN_DEFAULT: AMTunerSubChannel = -1;
pub const AMTUNER_SUBCHAN_NO_TUNE: AMTunerSubChannel = -2;
pub const AMTVAUDIO_EVENT_CHANGED: AMTVAudioEventType = 1;
pub const AMTVAUDIO_MODE_LANG_A: TVAudioMode = 16;
pub const AMTVAUDIO_MODE_LANG_B: TVAudioMode = 32;
pub const AMTVAUDIO_MODE_LANG_C: TVAudioMode = 64;
pub const AMTVAUDIO_MODE_MONO: TVAudioMode = 1;
pub const AMTVAUDIO_MODE_STEREO: TVAudioMode = 2;
pub const AMTVAUDIO_PRESET_LANG_A: TVAudioMode = 4096;
pub const AMTVAUDIO_PRESET_LANG_B: TVAudioMode = 8192;
pub const AMTVAUDIO_PRESET_LANG_C: TVAudioMode = 16384;
pub const AMTVAUDIO_PRESET_STEREO: TVAudioMode = 512;
pub type AMTVAudioEventType = i32;
pub type AMTunerEventType = i32;
pub type AMTunerModeType = i32;
pub type AMTunerSignalStrength = i32;
pub type AMTunerSubChannel = i32;
pub const AM_AUDREND_STAT_PARAM_BREAK_COUNT: _AM_AUDIO_RENDERER_STAT_PARAM = 1;
pub const AM_AUDREND_STAT_PARAM_BUFFERFULLNESS: _AM_AUDIO_RENDERER_STAT_PARAM = 11;
pub const AM_AUDREND_STAT_PARAM_DISCONTINUITIES: _AM_AUDIO_RENDERER_STAT_PARAM = 5;
pub const AM_AUDREND_STAT_PARAM_JITTER: _AM_AUDIO_RENDERER_STAT_PARAM = 12;
pub const AM_AUDREND_STAT_PARAM_LAST_BUFFER_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = 4;
pub const AM_AUDREND_STAT_PARAM_SILENCE_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = 3;
pub const AM_AUDREND_STAT_PARAM_SLAVE_ACCUMERROR: _AM_AUDIO_RENDERER_STAT_PARAM = 10;
pub const AM_AUDREND_STAT_PARAM_SLAVE_DROPWRITE_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = 7;
pub const AM_AUDREND_STAT_PARAM_SLAVE_HIGHLOWERROR: _AM_AUDIO_RENDERER_STAT_PARAM = 8;
pub const AM_AUDREND_STAT_PARAM_SLAVE_LASTHIGHLOWERROR: _AM_AUDIO_RENDERER_STAT_PARAM = 9;
pub const AM_AUDREND_STAT_PARAM_SLAVE_MODE: _AM_AUDIO_RENDERER_STAT_PARAM = 2;
pub const AM_AUDREND_STAT_PARAM_SLAVE_RATE: _AM_AUDIO_RENDERER_STAT_PARAM = 6;
pub const AM_DVD_ADAPT_GRAPH: AM_DVD_GRAPH_FLAGS = 16384;
pub const AM_DVD_DO_NOT_CLEAR: AM_DVD_GRAPH_FLAGS = 512;
pub const AM_DVD_EVR_ONLY: AM_DVD_GRAPH_FLAGS = 4096;
pub const AM_DVD_EVR_QOS: AM_DVD_GRAPH_FLAGS = 8192;
pub type AM_DVD_GRAPH_FLAGS = i32;
pub const AM_DVD_HWDEC_ONLY: AM_DVD_GRAPH_FLAGS = 2;
pub const AM_DVD_HWDEC_PREFER: AM_DVD_GRAPH_FLAGS = 1;
pub const AM_DVD_MASK: AM_DVD_GRAPH_FLAGS = 65535;
pub const AM_DVD_NOVPE: AM_DVD_GRAPH_FLAGS = 256;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AM_DVD_RENDERSTATUS {
    pub hrVPEStatus: windows_sys::core::HRESULT,
    pub bDvdVolInvalid: windows_sys::core::BOOL,
    pub bDvdVolUnknown: windows_sys::core::BOOL,
    pub bNoLine21In: windows_sys::core::BOOL,
    pub bNoLine21Out: windows_sys::core::BOOL,
    pub iNumStreams: i32,
    pub iNumStreamsFailed: i32,
    pub dwFailedStreamsFlag: u32,
}
pub const AM_DVD_STREAM_AUDIO: AM_DVD_STREAM_FLAGS = 2;
pub type AM_DVD_STREAM_FLAGS = i32;
pub const AM_DVD_STREAM_SUBPIC: AM_DVD_STREAM_FLAGS = 4;
pub const AM_DVD_STREAM_VIDEO: AM_DVD_STREAM_FLAGS = 1;
pub const AM_DVD_SWDEC_ONLY: AM_DVD_GRAPH_FLAGS = 8;
pub const AM_DVD_SWDEC_PREFER: AM_DVD_GRAPH_FLAGS = 4;
pub const AM_DVD_VMR9_ONLY: AM_DVD_GRAPH_FLAGS = 2048;
pub type AM_FILESINK_FLAGS = i32;
pub const AM_FILE_OVERWRITE: AM_FILESINK_FLAGS = 1;
pub type AM_FILTER_FLAGS = i32;
pub const AM_FILTER_FLAGS_REMOVABLE: AM_FILTER_FLAGS = 1;
pub const AM_FILTER_MISC_FLAGS_IS_RENDERER: _AM_FILTER_MISC_FLAGS = 1;
pub const AM_FILTER_MISC_FLAGS_IS_SOURCE: _AM_FILTER_MISC_FLAGS = 2;
pub const AM_GBF_NODDSURFACELOCK: u32 = 8;
pub const AM_GBF_NOTASYNCPOINT: u32 = 2;
pub const AM_GBF_NOWAIT: u32 = 4;
pub const AM_GBF_PREVFRAMESKIPPED: u32 = 1;
pub const AM_GETDECODERCAP_QUERY_EVR_SUPPORT: u32 = 7;
pub const AM_GETDECODERCAP_QUERY_VMR9_SUPPORT: u32 = 6;
pub const AM_GETDECODERCAP_QUERY_VMR_SUPPORT: u32 = 1;
pub const AM_GRAPH_CONFIG_RECONNECT_CACHE_REMOVED_FILTERS: AM_GRAPH_CONFIG_RECONNECT_FLAGS = 2;
pub const AM_GRAPH_CONFIG_RECONNECT_DIRECTCONNECT: AM_GRAPH_CONFIG_RECONNECT_FLAGS = 1;
pub type AM_GRAPH_CONFIG_RECONNECT_FLAGS = i32;
pub const AM_GRAPH_CONFIG_RECONNECT_USE_ONLY_CACHED_FILTERS: AM_GRAPH_CONFIG_RECONNECT_FLAGS = 4;
pub const AM_INTF_SEARCH_FILTER: _AM_INTF_SEARCH_FLAGS = 4;
pub const AM_INTF_SEARCH_INPUT_PIN: _AM_INTF_SEARCH_FLAGS = 1;
pub const AM_INTF_SEARCH_OUTPUT_PIN: _AM_INTF_SEARCH_FLAGS = 2;
pub const AM_MEDIAEVENT_NONOTIFY: tagAM_MEDIAEVENT_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AM_MEDIA_TYPE {
    pub majortype: windows_sys::core::GUID,
    pub subtype: windows_sys::core::GUID,
    pub bFixedSizeSamples: windows_sys::core::BOOL,
    pub bTemporalCompression: windows_sys::core::BOOL,
    pub lSampleSize: u32,
    pub formattype: windows_sys::core::GUID,
    pub pUnk: *mut core::ffi::c_void,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
impl Default for AM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AM_OVERLAY_NOTIFY_DEST_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = 4;
pub const AM_OVERLAY_NOTIFY_SOURCE_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = 2;
pub const AM_OVERLAY_NOTIFY_VISIBLE_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = 1;
pub const AM_PIN_FLOW_CONTROL_BLOCK: _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS = 1;
pub const AM_PUSHSOURCECAPS_INTERNAL_RM: _AM_PUSHSOURCE_FLAGS = 1;
pub const AM_PUSHSOURCECAPS_NOT_LIVE: _AM_PUSHSOURCE_FLAGS = 2;
pub const AM_PUSHSOURCECAPS_PRIVATE_CLOCK: _AM_PUSHSOURCE_FLAGS = 4;
pub const AM_PUSHSOURCEREQS_USE_CLOCK_CHAIN: _AM_PUSHSOURCE_FLAGS = 131072;
pub const AM_PUSHSOURCEREQS_USE_STREAM_CLOCK: _AM_PUSHSOURCE_FLAGS = 65536;
pub const AM_QUERY_DECODER_ATSC_HD_SUPPORT: u32 = 5;
pub const AM_QUERY_DECODER_ATSC_SD_SUPPORT: u32 = 4;
pub const AM_QUERY_DECODER_DVD_SUPPORT: u32 = 3;
pub const AM_QUERY_DECODER_DXVA_1_SUPPORT: u32 = 2;
pub const AM_QUERY_DECODER_VMR_SUPPORT: u32 = 1;
pub const AM_RENDEREX_RENDERTOEXISTINGRENDERERS: _AM_RENSDEREXFLAGS = 1;
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy)]
pub struct AM_SAMPLE2_PROPERTIES {
    pub cbData: u32,
    pub dwTypeSpecificFlags: u32,
    pub dwSampleFlags: u32,
    pub lActual: i32,
    pub tStart: super::REFERENCE_TIME,
    pub tStop: super::REFERENCE_TIME,
    pub dwStreamId: u32,
    pub pMediaType: *mut AM_MEDIA_TYPE,
    pub pbBuffer: *mut u8,
    pub cbBuffer: i32,
}
#[cfg(feature = "ksmedia")]
impl Default for AM_SAMPLE2_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AM_SAMPLE_DATADISCONTINUITY: tagAM_SAMPLE_PROPERTY_FLAGS = 4;
pub const AM_SAMPLE_ENDOFSTREAM: tagAM_SAMPLE_PROPERTY_FLAGS = 512;
pub const AM_SAMPLE_FLUSH_ON_PAUSE: tagAM_SAMPLE_PROPERTY_FLAGS = 128;
pub const AM_SAMPLE_PREROLL: tagAM_SAMPLE_PROPERTY_FLAGS = 2;
pub const AM_SAMPLE_SPLICEPOINT: tagAM_SAMPLE_PROPERTY_FLAGS = 1;
pub const AM_SAMPLE_STOPVALID: tagAM_SAMPLE_PROPERTY_FLAGS = 256;
pub const AM_SAMPLE_TIMEDISCONTINUITY: tagAM_SAMPLE_PROPERTY_FLAGS = 64;
pub const AM_SAMPLE_TIMEVALID: tagAM_SAMPLE_PROPERTY_FLAGS = 16;
pub const AM_SAMPLE_TYPECHANGED: tagAM_SAMPLE_PROPERTY_FLAGS = 8;
pub const AM_SEEKING_AbsolutePositioning: AM_SEEKING_SEEKING_FLAGS = 1;
pub const AM_SEEKING_CanDoSegments: AM_SEEKING_SEEKING_CAPABILITIES = 128;
pub const AM_SEEKING_CanGetCurrentPos: AM_SEEKING_SEEKING_CAPABILITIES = 8;
pub const AM_SEEKING_CanGetDuration: AM_SEEKING_SEEKING_CAPABILITIES = 32;
pub const AM_SEEKING_CanGetStopPos: AM_SEEKING_SEEKING_CAPABILITIES = 16;
pub const AM_SEEKING_CanPlayBackwards: AM_SEEKING_SEEKING_CAPABILITIES = 64;
pub const AM_SEEKING_CanSeekAbsolute: AM_SEEKING_SEEKING_CAPABILITIES = 1;
pub const AM_SEEKING_CanSeekBackwards: AM_SEEKING_SEEKING_CAPABILITIES = 4;
pub const AM_SEEKING_CanSeekForwards: AM_SEEKING_SEEKING_CAPABILITIES = 2;
pub const AM_SEEKING_IncrementalPositioning: AM_SEEKING_SEEKING_FLAGS = 3;
pub const AM_SEEKING_NoFlush: AM_SEEKING_SEEKING_FLAGS = 32;
pub const AM_SEEKING_NoPositioning: AM_SEEKING_SEEKING_FLAGS = 0;
pub const AM_SEEKING_PositioningBitsMask: AM_SEEKING_SEEKING_FLAGS = 3;
pub const AM_SEEKING_RelativePositioning: AM_SEEKING_SEEKING_FLAGS = 2;
pub const AM_SEEKING_ReturnTime: AM_SEEKING_SEEKING_FLAGS = 8;
pub type AM_SEEKING_SEEKING_CAPABILITIES = i32;
pub type AM_SEEKING_SEEKING_FLAGS = i32;
pub const AM_SEEKING_SeekToKeyFrame: AM_SEEKING_SEEKING_FLAGS = 4;
pub const AM_SEEKING_Segment: AM_SEEKING_SEEKING_FLAGS = 16;
pub const AM_SEEKING_Source: AM_SEEKING_SEEKING_CAPABILITIES = 256;
pub const AM_STREAM_CONTROL: tagAM_SAMPLE_PROPERTY_FLAGS = 1;
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy, Default)]
pub struct AM_STREAM_INFO {
    pub tStart: super::REFERENCE_TIME,
    pub tStop: super::REFERENCE_TIME,
    pub dwStartCookie: u32,
    pub dwStopCookie: u32,
    pub dwFlags: u32,
}
pub const AM_STREAM_INFO_DISCARDING: AM_STREAM_INFO_FLAGS = 4;
pub type AM_STREAM_INFO_FLAGS = i32;
pub const AM_STREAM_INFO_START_DEFINED: AM_STREAM_INFO_FLAGS = 1;
pub const AM_STREAM_INFO_STOP_DEFINED: AM_STREAM_INFO_FLAGS = 2;
pub const AM_STREAM_INFO_STOP_SEND_EXTRA: AM_STREAM_INFO_FLAGS = 16;
pub const AM_STREAM_MEDIA: tagAM_SAMPLE_PROPERTY_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_STREAM_CONFIG_CAPS {
    pub guid: windows_sys::core::GUID,
    pub MinimumChannels: u32,
    pub MaximumChannels: u32,
    pub ChannelsGranularity: u32,
    pub MinimumBitsPerSample: u32,
    pub MaximumBitsPerSample: u32,
    pub BitsPerSampleGranularity: u32,
    pub MinimumSampleFrequency: u32,
    pub MaximumSampleFrequency: u32,
    pub SampleFrequencyGranularity: u32,
}
pub const AnalogVideoMask_MCE_NTSC: AnalogVideoStandard = 1052167;
pub const AnalogVideoMask_MCE_PAL: AnalogVideoStandard = 496;
pub const AnalogVideoMask_MCE_SECAM: AnalogVideoStandard = 1044480;
pub type AnalogVideoStandard = i32;
pub const AnalogVideo_NTSC_433: AnalogVideoStandard = 4;
pub const AnalogVideo_NTSC_M: AnalogVideoStandard = 1;
pub const AnalogVideo_NTSC_M_J: AnalogVideoStandard = 2;
pub const AnalogVideo_NTSC_Mask: u32 = 7;
pub const AnalogVideo_None: AnalogVideoStandard = 0;
pub const AnalogVideo_PAL_60: AnalogVideoStandard = 2048;
pub const AnalogVideo_PAL_B: AnalogVideoStandard = 16;
pub const AnalogVideo_PAL_D: AnalogVideoStandard = 32;
pub const AnalogVideo_PAL_G: AnalogVideoStandard = 64;
pub const AnalogVideo_PAL_H: AnalogVideoStandard = 128;
pub const AnalogVideo_PAL_I: AnalogVideoStandard = 256;
pub const AnalogVideo_PAL_M: AnalogVideoStandard = 512;
pub const AnalogVideo_PAL_Mask: u32 = 1052656;
pub const AnalogVideo_PAL_N: AnalogVideoStandard = 1024;
pub const AnalogVideo_PAL_N_COMBO: AnalogVideoStandard = 1048576;
pub const AnalogVideo_SECAM_B: AnalogVideoStandard = 4096;
pub const AnalogVideo_SECAM_D: AnalogVideoStandard = 8192;
pub const AnalogVideo_SECAM_G: AnalogVideoStandard = 16384;
pub const AnalogVideo_SECAM_H: AnalogVideoStandard = 32768;
pub const AnalogVideo_SECAM_K: AnalogVideoStandard = 65536;
pub const AnalogVideo_SECAM_K1: AnalogVideoStandard = 131072;
pub const AnalogVideo_SECAM_L: AnalogVideoStandard = 262144;
pub const AnalogVideo_SECAM_L1: AnalogVideoStandard = 524288;
pub const AnalogVideo_SECAM_Mask: u32 = 1044480;
pub const CDEF_BYPASS_CLASS_MANAGER: u32 = 2;
pub const CDEF_CLASS_DEFAULT: u32 = 1;
pub const CDEF_DEVMON_CMGR_DEVICE: u32 = 16;
pub const CDEF_DEVMON_DMO: u32 = 32;
pub const CDEF_DEVMON_FILTER: u32 = 128;
pub const CDEF_DEVMON_PNP_DEVICE: u32 = 64;
pub const CDEF_DEVMON_SELECTIVE_MASK: u32 = 240;
pub const CDEF_MERIT_ABOVE_DO_NOT_USE: u32 = 8;
pub const CHARS_IN_GUID: u32 = 39;
pub const CK_INDEX: i32 = 1;
pub const CK_NOCOLORKEY: i32 = 0;
pub const CK_RGB: i32 = 2;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct COLORKEY {
    pub KeyType: u32,
    pub PaletteIndex: u32,
    pub LowColorValue: super::COLORREF,
    pub HighColorValue: super::COLORREF,
}
pub type CameraControlFlags = i32;
pub type CameraControlProperty = i32;
pub const CameraControl_Exposure: CameraControlProperty = 4;
pub const CameraControl_Flags_Auto: CameraControlFlags = 1;
pub const CameraControl_Flags_Manual: CameraControlFlags = 2;
pub const CameraControl_Focus: CameraControlProperty = 6;
pub const CameraControl_Iris: CameraControlProperty = 5;
pub const CameraControl_Pan: CameraControlProperty = 0;
pub const CameraControl_Roll: CameraControlProperty = 2;
pub const CameraControl_Tilt: CameraControlProperty = 1;
pub const CameraControl_Zoom: CameraControlProperty = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CodecAPIEventData {
    pub guid: windows_sys::core::GUID,
    pub dataLength: u32,
    pub reserved: [u32; 3],
}
impl Default for CodecAPIEventData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CompressionCaps = i32;
pub const CompressionCaps_CanBFrame: CompressionCaps = 8;
pub const CompressionCaps_CanCrunch: CompressionCaps = 2;
pub const CompressionCaps_CanKeyFrame: CompressionCaps = 4;
pub const CompressionCaps_CanQuality: CompressionCaps = 1;
pub const CompressionCaps_CanWindow: CompressionCaps = 16;
pub const DECIMATION_DEFAULT: DECIMATION_USAGE = 4;
pub const DECIMATION_LEGACY: DECIMATION_USAGE = 0;
pub type DECIMATION_USAGE = i32;
pub const DECIMATION_USE_DECODER_ONLY: DECIMATION_USAGE = 1;
pub const DECIMATION_USE_OVERLAY_ONLY: DECIMATION_USAGE = 3;
pub const DECIMATION_USE_VIDEOPORT_ONLY: DECIMATION_USAGE = 2;
pub const DECODER_CAP_NOTSUPPORTED: u32 = 0;
pub const DECODER_CAP_SUPPORTED: u32 = 1;
pub const DISPLAY_16x9: DVD_PREFERRED_DISPLAY_MODE = 1;
pub const DISPLAY_4x3_LETTERBOX_PREFERRED: DVD_PREFERRED_DISPLAY_MODE = 3;
pub const DISPLAY_4x3_PANSCAN_PREFERRED: DVD_PREFERRED_DISPLAY_MODE = 2;
pub const DISPLAY_CONTENT_DEFAULT: DVD_PREFERRED_DISPLAY_MODE = 0;
pub const DVDECODERRESOLUTION_180x120: _DVDECODERRESOLUTION = 1002;
pub const DVDECODERRESOLUTION_360x240: _DVDECODERRESOLUTION = 1001;
pub const DVDECODERRESOLUTION_720x480: _DVDECODERRESOLUTION = 1000;
pub const DVDECODERRESOLUTION_88x60: _DVDECODERRESOLUTION = 1003;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_ATR {
    pub ulCAT: u32,
    pub pbATRI: [u8; 768],
}
impl Default for DVD_ATR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DVD_AUDIO_APPMODE = i32;
pub const DVD_AUDIO_CAPS_AC3: u32 = 1;
pub const DVD_AUDIO_CAPS_DTS: u32 = 8;
pub const DVD_AUDIO_CAPS_LPCM: u32 = 4;
pub const DVD_AUDIO_CAPS_MPEG2: u32 = 2;
pub const DVD_AUDIO_CAPS_SDDS: u32 = 16;
pub type DVD_AUDIO_FORMAT = i32;
pub type DVD_AUDIO_LANG_EXT = i32;
pub const DVD_AUD_EXT_Captions: DVD_AUDIO_LANG_EXT = 1;
pub const DVD_AUD_EXT_DirectorComments1: DVD_AUDIO_LANG_EXT = 3;
pub const DVD_AUD_EXT_DirectorComments2: DVD_AUDIO_LANG_EXT = 4;
pub const DVD_AUD_EXT_NotSpecified: DVD_AUDIO_LANG_EXT = 0;
pub const DVD_AUD_EXT_VisuallyImpaired: DVD_AUDIO_LANG_EXT = 2;
pub const DVD_AppMode_Karaoke: DVD_TITLE_APPMODE = 1;
pub const DVD_AppMode_Not_Specified: DVD_TITLE_APPMODE = 0;
pub const DVD_AppMode_Other: DVD_TITLE_APPMODE = 3;
pub const DVD_Assignment_LR: DVD_KARAOKE_ASSIGNMENT = 2;
pub const DVD_Assignment_LR1: DVD_KARAOKE_ASSIGNMENT = 4;
pub const DVD_Assignment_LR12: DVD_KARAOKE_ASSIGNMENT = 6;
pub const DVD_Assignment_LRM: DVD_KARAOKE_ASSIGNMENT = 3;
pub const DVD_Assignment_LRM1: DVD_KARAOKE_ASSIGNMENT = 5;
pub const DVD_Assignment_LRM12: DVD_KARAOKE_ASSIGNMENT = 7;
pub const DVD_Assignment_reserved0: DVD_KARAOKE_ASSIGNMENT = 0;
pub const DVD_Assignment_reserved1: DVD_KARAOKE_ASSIGNMENT = 1;
pub type DVD_AudioATR = [u8; 8];
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DVD_AudioAttributes {
    pub AppMode: DVD_AUDIO_APPMODE,
    pub AppModeData: u8,
    pub AudioFormat: DVD_AUDIO_FORMAT,
    pub Language: super::LCID,
    pub LanguageExtension: DVD_AUDIO_LANG_EXT,
    pub fHasMultichannelInfo: windows_sys::core::BOOL,
    pub dwFrequency: u32,
    pub bQuantization: u8,
    pub bNumberOfChannels: u8,
    pub dwReserved: [u32; 2],
}
#[cfg(feature = "winnt")]
impl Default for DVD_AudioAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DVD_AudioDuringFFwdRew: DVD_OPTION_FLAG = 4;
pub const DVD_AudioFormat_AC3: DVD_AUDIO_FORMAT = 0;
pub const DVD_AudioFormat_DTS: DVD_AUDIO_FORMAT = 6;
pub const DVD_AudioFormat_LPCM: DVD_AUDIO_FORMAT = 5;
pub const DVD_AudioFormat_MPEG1: DVD_AUDIO_FORMAT = 1;
pub const DVD_AudioFormat_MPEG1_DRC: DVD_AUDIO_FORMAT = 2;
pub const DVD_AudioFormat_MPEG2: DVD_AUDIO_FORMAT = 3;
pub const DVD_AudioFormat_MPEG2_DRC: DVD_AUDIO_FORMAT = 4;
pub const DVD_AudioFormat_Other: DVD_AUDIO_FORMAT = 8;
pub const DVD_AudioFormat_SDDS: DVD_AUDIO_FORMAT = 7;
pub const DVD_AudioMode_Karaoke: DVD_AUDIO_APPMODE = 1;
pub const DVD_AudioMode_None: DVD_AUDIO_APPMODE = 0;
pub const DVD_AudioMode_Other: DVD_AUDIO_APPMODE = 3;
pub const DVD_AudioMode_Surround: DVD_AUDIO_APPMODE = 2;
pub type DVD_CMD_FLAGS = i32;
pub const DVD_CMD_FLAG_Block: DVD_CMD_FLAGS = 4;
pub const DVD_CMD_FLAG_EndAfterRendered: DVD_CMD_FLAGS = 16;
pub const DVD_CMD_FLAG_Flush: DVD_CMD_FLAGS = 1;
pub const DVD_CMD_FLAG_None: DVD_CMD_FLAGS = 0;
pub const DVD_CMD_FLAG_SendEvents: DVD_CMD_FLAGS = 2;
pub const DVD_CMD_FLAG_StartWhenRendered: DVD_CMD_FLAGS = 8;
pub const DVD_CacheSizeInMB: DVD_OPTION_FLAG = 6;
pub const DVD_Channel_Audio: DVD_TextStringType = 32;
pub const DVD_CharSet_ISO646: DVD_TextCharSet = 1;
pub const DVD_CharSet_ISO8859_1: DVD_TextCharSet = 3;
pub const DVD_CharSet_JIS_Roman_Kanji: DVD_TextCharSet = 2;
pub const DVD_CharSet_ShiftJIS_Kanji_Roman_Katakana: DVD_TextCharSet = 4;
pub const DVD_CharSet_Unicode: DVD_TextCharSet = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVD_DECODER_CAPS {
    pub dwSize: u32,
    pub dwAudioCaps: u32,
    pub dFwdMaxRateVideo: f64,
    pub dFwdMaxRateAudio: f64,
    pub dFwdMaxRateSP: f64,
    pub dBwdMaxRateVideo: f64,
    pub dBwdMaxRateAudio: f64,
    pub dBwdMaxRateSP: f64,
    pub dwRes1: u32,
    pub dwRes2: u32,
    pub dwRes3: u32,
    pub dwRes4: u32,
}
pub const DVD_DEFAULT_AUDIO_STREAM: u32 = 15;
pub type DVD_DISC_SIDE = i32;
pub type DVD_DOMAIN = i32;
pub const DVD_DOMAIN_FirstPlay: DVD_DOMAIN = 1;
pub const DVD_DOMAIN_Stop: DVD_DOMAIN = 5;
pub const DVD_DOMAIN_Title: DVD_DOMAIN = 4;
pub const DVD_DOMAIN_VideoManagerMenu: DVD_DOMAIN = 2;
pub const DVD_DOMAIN_VideoTitleSetMenu: DVD_DOMAIN = 3;
pub const DVD_DisableStillThrottle: DVD_OPTION_FLAG = 14;
pub const DVD_EnableCC: DVD_OPTION_FLAG = 19;
pub const DVD_EnableESOutput: DVD_OPTION_FLAG = 12;
pub const DVD_EnableExtendedCopyProtectErrors: DVD_OPTION_FLAG = 8;
pub const DVD_EnableLoggingEvents: DVD_OPTION_FLAG = 15;
pub const DVD_EnableNonblockingAPIs: DVD_OPTION_FLAG = 5;
pub const DVD_EnablePortableBookmarks: DVD_OPTION_FLAG = 7;
pub const DVD_EnableStreaming: DVD_OPTION_FLAG = 11;
pub const DVD_EnableTitleLength: DVD_OPTION_FLAG = 13;
pub const DVD_FPS_25: DVD_FRAMERATE = 1;
pub const DVD_FPS_30NonDrop: DVD_FRAMERATE = 3;
pub type DVD_FRAMERATE = i32;
pub const DVD_General_Comments: DVD_TextStringType = 49;
pub const DVD_General_Name: DVD_TextStringType = 48;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVD_HMSF_TIMECODE {
    pub bHours: u8,
    pub bMinutes: u8,
    pub bSeconds: u8,
    pub bFrames: u8,
}
pub const DVD_HMSF_TimeCodeEvents: DVD_OPTION_FLAG = 3;
pub const DVD_IncreaseOutputControl: DVD_OPTION_FLAG = 10;
pub type DVD_KARAOKE_ASSIGNMENT = i32;
pub type DVD_KARAOKE_CONTENTS = i32;
pub type DVD_KARAOKE_DOWNMIX = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_KaraokeAttributes {
    pub bVersion: u8,
    pub fMasterOfCeremoniesInGuideVocal1: windows_sys::core::BOOL,
    pub fDuet: windows_sys::core::BOOL,
    pub ChannelAssignment: DVD_KARAOKE_ASSIGNMENT,
    pub wChannelContents: [u16; 8],
}
impl Default for DVD_KaraokeAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DVD_Karaoke_GuideMelody1: DVD_KARAOKE_CONTENTS = 4;
pub const DVD_Karaoke_GuideMelody2: DVD_KARAOKE_CONTENTS = 8;
pub const DVD_Karaoke_GuideMelodyA: DVD_KARAOKE_CONTENTS = 16;
pub const DVD_Karaoke_GuideMelodyB: DVD_KARAOKE_CONTENTS = 32;
pub const DVD_Karaoke_GuideVocal1: DVD_KARAOKE_CONTENTS = 1;
pub const DVD_Karaoke_GuideVocal2: DVD_KARAOKE_CONTENTS = 2;
pub const DVD_Karaoke_SoundEffectA: DVD_KARAOKE_CONTENTS = 64;
pub const DVD_Karaoke_SoundEffectB: DVD_KARAOKE_CONTENTS = 128;
pub const DVD_MENU_Angle: DVD_MENU_ID = 6;
pub const DVD_MENU_Audio: DVD_MENU_ID = 5;
pub const DVD_MENU_Chapter: DVD_MENU_ID = 7;
pub type DVD_MENU_ID = i32;
pub const DVD_MENU_Root: DVD_MENU_ID = 3;
pub const DVD_MENU_Subpicture: DVD_MENU_ID = 4;
pub const DVD_MENU_Title: DVD_MENU_ID = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVD_MUA_Coeff {
    pub log2_alpha: f64,
    pub log2_beta: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVD_MUA_MixingInfo {
    pub fMixTo0: windows_sys::core::BOOL,
    pub fMixTo1: windows_sys::core::BOOL,
    pub fMix0InPhase: windows_sys::core::BOOL,
    pub fMix1InPhase: windows_sys::core::BOOL,
    pub dwSpeakerPosition: u32,
}
pub const DVD_MaxReadBurstInKB: DVD_OPTION_FLAG = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DVD_MenuAttributes {
    pub fCompatibleRegion: [windows_sys::core::BOOL; 8],
    pub VideoAttributes: DVD_VideoAttributes,
    pub fAudioPresent: windows_sys::core::BOOL,
    pub AudioAttributes: DVD_AudioAttributes,
    pub fSubpicturePresent: windows_sys::core::BOOL,
    pub SubpictureAttributes: DVD_SubpictureAttributes,
}
#[cfg(feature = "winnt")]
impl Default for DVD_MenuAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DVD_Mix_0to0: DVD_KARAOKE_DOWNMIX = 1;
pub const DVD_Mix_0to1: DVD_KARAOKE_DOWNMIX = 256;
pub const DVD_Mix_1to0: DVD_KARAOKE_DOWNMIX = 2;
pub const DVD_Mix_1to1: DVD_KARAOKE_DOWNMIX = 512;
pub const DVD_Mix_2to0: DVD_KARAOKE_DOWNMIX = 4;
pub const DVD_Mix_2to1: DVD_KARAOKE_DOWNMIX = 1024;
pub const DVD_Mix_3to0: DVD_KARAOKE_DOWNMIX = 8;
pub const DVD_Mix_3to1: DVD_KARAOKE_DOWNMIX = 2048;
pub const DVD_Mix_4to0: DVD_KARAOKE_DOWNMIX = 16;
pub const DVD_Mix_4to1: DVD_KARAOKE_DOWNMIX = 4096;
pub const DVD_Mix_Lto0: DVD_KARAOKE_DOWNMIX = 32;
pub const DVD_Mix_Lto1: DVD_KARAOKE_DOWNMIX = 8192;
pub const DVD_Mix_Rto0: DVD_KARAOKE_DOWNMIX = 64;
pub const DVD_Mix_Rto1: DVD_KARAOKE_DOWNMIX = 16384;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVD_MultichannelAudioAttributes {
    pub Info: [DVD_MUA_MixingInfo; 8],
    pub Coeff: [DVD_MUA_Coeff; 8],
}
impl Default for DVD_MultichannelAudioAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DVD_NavCmdType = i32;
pub const DVD_NavCmdType_Button: DVD_NavCmdType = 4;
pub const DVD_NavCmdType_Cell: DVD_NavCmdType = 3;
pub const DVD_NavCmdType_Post: DVD_NavCmdType = 2;
pub const DVD_NavCmdType_Pre: DVD_NavCmdType = 1;
pub const DVD_NotifyParentalLevelChange: DVD_OPTION_FLAG = 2;
pub const DVD_NotifyPositionChange: DVD_OPTION_FLAG = 9;
pub type DVD_OPTION_FLAG = i32;
pub const DVD_Other_Cut: DVD_TextStringType = 81;
pub const DVD_Other_Scene: DVD_TextStringType = 80;
pub const DVD_Other_Take: DVD_TextStringType = 82;
pub type DVD_PARENTAL_LEVEL = i32;
pub const DVD_PARENTAL_LEVEL_1: DVD_PARENTAL_LEVEL = 256;
pub const DVD_PARENTAL_LEVEL_2: DVD_PARENTAL_LEVEL = 512;
pub const DVD_PARENTAL_LEVEL_3: DVD_PARENTAL_LEVEL = 1024;
pub const DVD_PARENTAL_LEVEL_4: DVD_PARENTAL_LEVEL = 2048;
pub const DVD_PARENTAL_LEVEL_5: DVD_PARENTAL_LEVEL = 4096;
pub const DVD_PARENTAL_LEVEL_6: DVD_PARENTAL_LEVEL = 8192;
pub const DVD_PARENTAL_LEVEL_7: DVD_PARENTAL_LEVEL = 16384;
pub const DVD_PARENTAL_LEVEL_8: DVD_PARENTAL_LEVEL = 32768;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVD_PLAYBACK_LOCATION {
    pub TitleNum: u32,
    pub ChapterNum: u32,
    pub TimeCode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVD_PLAYBACK_LOCATION2 {
    pub TitleNum: u32,
    pub ChapterNum: u32,
    pub TimeCode: DVD_HMSF_TIMECODE,
    pub TimeCodeFlags: u32,
}
pub type DVD_PREFERRED_DISPLAY_MODE = i32;
pub type DVD_REGISTER = u16;
pub type DVD_RELATIVE_BUTTON = i32;
pub const DVD_ReadBurstPeriodInMS: DVD_OPTION_FLAG = 17;
pub const DVD_Relative_Left: DVD_RELATIVE_BUTTON = 3;
pub const DVD_Relative_Lower: DVD_RELATIVE_BUTTON = 2;
pub const DVD_Relative_Right: DVD_RELATIVE_BUTTON = 4;
pub const DVD_Relative_Upper: DVD_RELATIVE_BUTTON = 1;
pub const DVD_ResetOnStop: DVD_OPTION_FLAG = 1;
pub const DVD_RestartDisc: DVD_OPTION_FLAG = 18;
pub const DVD_SIDE_A: DVD_DISC_SIDE = 1;
pub const DVD_SIDE_B: DVD_DISC_SIDE = 2;
pub const DVD_SPCoding_Extended: DVD_SUBPICTURE_CODING = 1;
pub const DVD_SPCoding_Other: DVD_SUBPICTURE_CODING = 2;
pub const DVD_SPCoding_RunLength: DVD_SUBPICTURE_CODING = 0;
pub const DVD_SPType_Language: DVD_SUBPICTURE_TYPE = 1;
pub const DVD_SPType_NotSpecified: DVD_SUBPICTURE_TYPE = 0;
pub const DVD_SPType_Other: DVD_SUBPICTURE_TYPE = 2;
pub const DVD_SP_EXT_CC_Big: DVD_SUBPICTURE_LANG_EXT = 6;
pub const DVD_SP_EXT_CC_Children: DVD_SUBPICTURE_LANG_EXT = 7;
pub const DVD_SP_EXT_CC_Normal: DVD_SUBPICTURE_LANG_EXT = 5;
pub const DVD_SP_EXT_Caption_Big: DVD_SUBPICTURE_LANG_EXT = 2;
pub const DVD_SP_EXT_Caption_Children: DVD_SUBPICTURE_LANG_EXT = 3;
pub const DVD_SP_EXT_Caption_Normal: DVD_SUBPICTURE_LANG_EXT = 1;
pub const DVD_SP_EXT_DirectorComments_Big: DVD_SUBPICTURE_LANG_EXT = 14;
pub const DVD_SP_EXT_DirectorComments_Children: DVD_SUBPICTURE_LANG_EXT = 15;
pub const DVD_SP_EXT_DirectorComments_Normal: DVD_SUBPICTURE_LANG_EXT = 13;
pub const DVD_SP_EXT_Forced: DVD_SUBPICTURE_LANG_EXT = 9;
pub const DVD_SP_EXT_NotSpecified: DVD_SUBPICTURE_LANG_EXT = 0;
pub const DVD_STREAM_DATA_CURRENT: u32 = 2048;
pub const DVD_STREAM_DATA_VMGM: u32 = 1024;
pub const DVD_STREAM_DATA_VTSM: u32 = 1025;
pub type DVD_SUBPICTURE_CODING = i32;
pub type DVD_SUBPICTURE_LANG_EXT = i32;
pub type DVD_SUBPICTURE_TYPE = i32;
pub const DVD_Stream_Angle: DVD_TextStringType = 18;
pub const DVD_Stream_Audio: DVD_TextStringType = 16;
pub const DVD_Stream_Subpicture: DVD_TextStringType = 17;
pub const DVD_Struct_Cell: DVD_TextStringType = 5;
pub const DVD_Struct_ParentalID: DVD_TextStringType = 3;
pub const DVD_Struct_PartOfTitle: DVD_TextStringType = 4;
pub const DVD_Struct_Title: DVD_TextStringType = 2;
pub const DVD_Struct_Volume: DVD_TextStringType = 1;
pub type DVD_SubpictureATR = [u8; 6];
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DVD_SubpictureAttributes {
    pub Type: DVD_SUBPICTURE_TYPE,
    pub CodingMode: DVD_SUBPICTURE_CODING,
    pub Language: super::LCID,
    pub LanguageExtension: DVD_SUBPICTURE_LANG_EXT,
}
pub const DVD_TC_FLAG_25fps: DVD_TIMECODE_FLAGS = 1;
pub const DVD_TC_FLAG_30fps: DVD_TIMECODE_FLAGS = 2;
pub const DVD_TC_FLAG_DropFrame: DVD_TIMECODE_FLAGS = 4;
pub const DVD_TC_FLAG_Interpolated: DVD_TIMECODE_FLAGS = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVD_TIMECODE {
    pub _bitfield: u32,
}
pub type DVD_TIMECODE_FLAGS = i32;
pub type DVD_TITLE_APPMODE = i32;
pub const DVD_TITLE_MENU: u32 = 0;
pub type DVD_TextCharSet = i32;
pub type DVD_TextStringType = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DVD_TitleAttributes {
    pub Anonymous: DVD_TitleAttributes_0,
    pub VideoAttributes: DVD_VideoAttributes,
    pub ulNumberOfAudioStreams: u32,
    pub AudioAttributes: [DVD_AudioAttributes; 8],
    pub MultichannelAudioAttributes: [DVD_MultichannelAudioAttributes; 8],
    pub ulNumberOfSubpictureStreams: u32,
    pub SubpictureAttributes: [DVD_SubpictureAttributes; 32],
}
#[cfg(feature = "winnt")]
impl Default for DVD_TitleAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DVD_TitleAttributes_0 {
    pub AppMode: DVD_TITLE_APPMODE,
    pub TitleLength: DVD_HMSF_TIMECODE,
}
#[cfg(feature = "winnt")]
impl Default for DVD_TitleAttributes_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DVD_Title_Album: DVD_TextStringType = 59;
pub const DVD_Title_Movie: DVD_TextStringType = 57;
pub const DVD_Title_Orig_Album: DVD_TextStringType = 75;
pub const DVD_Title_Orig_Movie: DVD_TextStringType = 73;
pub const DVD_Title_Orig_Other: DVD_TextStringType = 79;
pub const DVD_Title_Orig_Series: DVD_TextStringType = 72;
pub const DVD_Title_Orig_Song: DVD_TextStringType = 76;
pub const DVD_Title_Orig_Video: DVD_TextStringType = 74;
pub const DVD_Title_Other: DVD_TextStringType = 63;
pub const DVD_Title_Series: DVD_TextStringType = 56;
pub const DVD_Title_Song: DVD_TextStringType = 60;
pub const DVD_Title_Sub_Album: DVD_TextStringType = 67;
pub const DVD_Title_Sub_Movie: DVD_TextStringType = 65;
pub const DVD_Title_Sub_Other: DVD_TextStringType = 71;
pub const DVD_Title_Sub_Series: DVD_TextStringType = 64;
pub const DVD_Title_Sub_Song: DVD_TextStringType = 68;
pub const DVD_Title_Sub_Video: DVD_TextStringType = 66;
pub const DVD_Title_Video: DVD_TextStringType = 58;
pub type DVD_VIDEO_COMPRESSION = i32;
pub type DVD_VideoATR = [u8; 2];
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVD_VideoAttributes {
    pub fPanscanPermitted: windows_sys::core::BOOL,
    pub fLetterboxPermitted: windows_sys::core::BOOL,
    pub ulAspectX: u32,
    pub ulAspectY: u32,
    pub ulFrameRate: u32,
    pub ulFrameHeight: u32,
    pub Compression: DVD_VIDEO_COMPRESSION,
    pub fLine21Field1InGOP: windows_sys::core::BOOL,
    pub fLine21Field2InGOP: windows_sys::core::BOOL,
    pub ulSourceResolutionX: u32,
    pub ulSourceResolutionY: u32,
    pub fIsSourceLetterboxed: windows_sys::core::BOOL,
    pub fIsFilmMode: windows_sys::core::BOOL,
}
pub const DVD_VideoCompression_MPEG1: DVD_VIDEO_COMPRESSION = 1;
pub const DVD_VideoCompression_MPEG2: DVD_VIDEO_COMPRESSION = 2;
pub const DVD_VideoCompression_Other: DVD_VIDEO_COMPRESSION = 0;
pub const DVENCODERFORMAT_DVHD: _DVENCODERFORMAT = 2008;
pub const DVENCODERFORMAT_DVSD: _DVENCODERFORMAT = 2007;
pub const DVENCODERFORMAT_DVSL: _DVENCODERFORMAT = 2009;
pub const DVENCODERRESOLUTION_180x120: _DVENCODERRESOLUTION = 2014;
pub const DVENCODERRESOLUTION_360x240: _DVENCODERRESOLUTION = 2013;
pub const DVENCODERRESOLUTION_720x480: _DVENCODERRESOLUTION = 2012;
pub const DVENCODERRESOLUTION_88x60: _DVENCODERRESOLUTION = 2015;
pub const DVENCODERVIDEOFORMAT_NTSC: _DVENCODERVIDEOFORMAT = 2000;
pub const DVENCODERVIDEOFORMAT_PAL: _DVENCODERVIDEOFORMAT = 2001;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DVINFO {
    pub dwDVAAuxSrc: u32,
    pub dwDVAAuxCtl: u32,
    pub dwDVAAuxSrc1: u32,
    pub dwDVAAuxCtl1: u32,
    pub dwDVVAuxSrc: u32,
    pub dwDVVAuxCtl: u32,
    pub dwDVReserved: [u32; 2],
}
impl Default for DVINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DVRESOLUTION_DC: _DVRESOLUTION = 1003;
pub const DVRESOLUTION_FULL: _DVRESOLUTION = 1000;
pub const DVRESOLUTION_HALF: _DVRESOLUTION = 1001;
pub const DVRESOLUTION_QUARTER: _DVRESOLUTION = 1002;
pub const DeinterlacePref_BOB: VMRDeinterlacePrefs = 2;
pub const DeinterlacePref_Mask: VMRDeinterlacePrefs = 7;
pub const DeinterlacePref_NextBest: VMRDeinterlacePrefs = 1;
pub const DeinterlacePref_Weave: VMRDeinterlacePrefs = 4;
pub const DeinterlaceTech_BOBLineReplicate: VMRDeinterlaceTech = 1;
pub const DeinterlaceTech_BOBVerticalStretch: VMRDeinterlaceTech = 2;
pub const DeinterlaceTech_EdgeFiltering: VMRDeinterlaceTech = 16;
pub const DeinterlaceTech_FieldAdaptive: VMRDeinterlaceTech = 32;
pub const DeinterlaceTech_MedianFiltering: VMRDeinterlaceTech = 4;
pub const DeinterlaceTech_MotionVectorSteered: VMRDeinterlaceTech = 128;
pub const DeinterlaceTech_PixelAdaptive: VMRDeinterlaceTech = 64;
pub const DeinterlaceTech_Unknown: VMRDeinterlaceTech = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILTER_INFO {
    pub achName: [u16; 128],
    pub pGraph: *mut core::ffi::c_void,
}
impl Default for FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FILTER_STATE = i32;
pub const Famine: QualityMessageType = 0;
pub const Flood: QualityMessageType = 1;
pub type GPRMARRAY = [DVD_REGISTER; 16];
pub type HEVENT = usize;
pub type HSEMAPHORE = usize;
pub const INTERLEAVE_CAPTURE: InterleavingMode = 1;
pub const INTERLEAVE_FULL: InterleavingMode = 2;
pub const INTERLEAVE_NONE: InterleavingMode = 0;
pub const INTERLEAVE_NONE_BUFFERED: InterleavingMode = 3;
pub type InterleavingMode = i32;
pub type LPAMCOPPCommand = *mut AMCOPPCommand;
pub type LPAMCOPPStatusInput = *mut AMCOPPStatusInput;
pub type LPAMCOPPStatusOutput = *mut AMCOPPStatusOutput;
pub const MAX_FILTER_NAME: u32 = 128;
pub const MAX_NUMBER_OF_STREAMS: i32 = 16;
pub const MAX_PIN_NAME: u32 = 128;
pub const MERIT_DO_NOT_USE: i32 = 2097152;
pub const MERIT_HW_COMPRESSOR: i32 = 1048656;
pub const MERIT_NORMAL: i32 = 6291456;
pub const MERIT_PREFERRED: i32 = 8388608;
pub const MERIT_SW_COMPRESSOR: i32 = 1048576;
pub const MERIT_UNLIKELY: i32 = 4194304;
pub const MPEG2_PROGRAM_DIRECTORY_PES_PACKET: u32 = 2;
pub const MPEG2_PROGRAM_ELEMENTARY_STREAM: u32 = 1;
pub const MPEG2_PROGRAM_PACK_HEADER: u32 = 3;
pub const MPEG2_PROGRAM_PES_STREAM: u32 = 4;
pub const MPEG2_PROGRAM_STREAM_MAP: u32 = 0;
pub const MPEG2_PROGRAM_SYSTEM_HEADER: u32 = 5;
pub const MixerPref_ARAdjustXorY: VMRMixerPrefs = 4;
pub const MixerPref_BiLinearFiltering: VMRMixerPrefs = 16;
pub const MixerPref_DecimateMask: VMRMixerPrefs = 15;
pub const MixerPref_DecimateOutput: VMRMixerPrefs = 2;
pub const MixerPref_DecimationReserved: VMRMixerPrefs = 8;
pub const MixerPref_DynamicDecimateBy2: VMRMixerPrefs = 131072;
pub const MixerPref_DynamicMask: VMRMixerPrefs = 983040;
pub const MixerPref_DynamicReserved: VMRMixerPrefs = 786432;
pub const MixerPref_DynamicSwitchToBOB: VMRMixerPrefs = 65536;
pub const MixerPref_FilteringMask: VMRMixerPrefs = 240;
pub const MixerPref_NoDecimation: VMRMixerPrefs = 1;
pub const MixerPref_PointFiltering: VMRMixerPrefs = 32;
pub const MixerPref_RenderTargetMask: VMRMixerPrefs = 65280;
pub const MixerPref_RenderTargetRGB: VMRMixerPrefs = 256;
pub const MixerPref_RenderTargetReserved: VMRMixerPrefs = 57344;
pub const MixerPref_RenderTargetYUV: VMRMixerPrefs = 4096;
pub const MixerPref_RenderTargetYUV420: VMRMixerPrefs = 512;
pub const MixerPref_RenderTargetYUV422: VMRMixerPrefs = 1024;
pub const MixerPref_RenderTargetYUV444: VMRMixerPrefs = 2048;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NORMALIZEDRECT {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
pub type PDVINFO = *mut DVINFO;
pub const PINDIR_INPUT: PIN_DIRECTION = 0;
pub const PINDIR_OUTPUT: PIN_DIRECTION = 1;
pub type PIN_DIRECTION = i32;
#[repr(C)]
#[cfg(feature = "objidl")]
#[derive(Clone, Copy)]
pub struct PIN_INFO {
    pub pFilter: *mut core::ffi::c_void,
    pub dir: PIN_DIRECTION,
    pub achName: [u16; 128],
}
#[cfg(feature = "objidl")]
impl Default for PIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PNORMALIZEDRECT = *mut NORMALIZEDRECT;
#[cfg(all(feature = "ddraw", feature = "windef"))]
pub type PVMRALPHABITMAP = *mut VMRALPHABITMAP;
pub const PhysConn_Audio_1394: PhysicalConnectorType = 4103;
pub const PhysConn_Audio_AESDigital: PhysicalConnectorType = 4099;
pub const PhysConn_Audio_AUX: PhysicalConnectorType = 4102;
pub const PhysConn_Audio_AudioDecoder: PhysicalConnectorType = 4105;
pub const PhysConn_Audio_Line: PhysicalConnectorType = 4097;
pub const PhysConn_Audio_Mic: PhysicalConnectorType = 4098;
pub const PhysConn_Audio_SCSI: PhysicalConnectorType = 4101;
pub const PhysConn_Audio_SPDIFDigital: PhysicalConnectorType = 4100;
pub const PhysConn_Audio_Tuner: PhysicalConnectorType = 4096;
pub const PhysConn_Audio_USB: PhysicalConnectorType = 4104;
pub const PhysConn_Video_1394: PhysicalConnectorType = 10;
pub const PhysConn_Video_AUX: PhysicalConnectorType = 9;
pub const PhysConn_Video_Black: PhysicalConnectorType = 15;
pub const PhysConn_Video_Composite: PhysicalConnectorType = 2;
pub const PhysConn_Video_ParallelDigital: PhysicalConnectorType = 7;
pub const PhysConn_Video_RGB: PhysicalConnectorType = 4;
pub const PhysConn_Video_SCART: PhysicalConnectorType = 14;
pub const PhysConn_Video_SCSI: PhysicalConnectorType = 8;
pub const PhysConn_Video_SVideo: PhysicalConnectorType = 3;
pub const PhysConn_Video_SerialDigital: PhysicalConnectorType = 6;
pub const PhysConn_Video_Tuner: PhysicalConnectorType = 1;
pub const PhysConn_Video_USB: PhysicalConnectorType = 11;
pub const PhysConn_Video_VideoDecoder: PhysicalConnectorType = 12;
pub const PhysConn_Video_VideoEncoder: PhysicalConnectorType = 13;
pub const PhysConn_Video_YRYBY: PhysicalConnectorType = 5;
pub type PhysicalConnectorType = i32;
#[repr(C)]
#[cfg(feature = "ksmedia")]
#[derive(Clone, Copy, Default)]
pub struct Quality {
    pub Type: QualityMessageType,
    pub Proportion: i32,
    pub Late: super::REFERENCE_TIME,
    pub TimeStamp: super::REFERENCE_TIME,
}
pub type QualityMessageType = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REGFILTER {
    pub Clsid: windows_sys::core::GUID,
    pub Name: windows_sys::core::PWSTR,
}
impl Default for REGFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REGFILTER2 {
    pub dwVersion: u32,
    pub dwMerit: u32,
    pub Anonymous: REGFILTER2_0,
}
impl Default for REGFILTER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union REGFILTER2_0 {
    pub Anonymous: REGFILTER2_0_0,
    pub Anonymous2: REGFILTER2_0_1,
}
impl Default for REGFILTER2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REGFILTER2_0_0 {
    pub cPins: u32,
    pub rgPins: *const REGFILTERPINS,
}
impl Default for REGFILTER2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REGFILTER2_0_1 {
    pub cPins2: u32,
    pub rgPins2: *const REGFILTERPINS2,
}
impl Default for REGFILTER2_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REGFILTERPINS {
    pub strName: windows_sys::core::PWSTR,
    pub bRendered: windows_sys::core::BOOL,
    pub bOutput: windows_sys::core::BOOL,
    pub bZero: windows_sys::core::BOOL,
    pub bMany: windows_sys::core::BOOL,
    pub clsConnectsToFilter: *const windows_sys::core::GUID,
    pub strConnectsToPin: *const u16,
    pub nMediaTypes: u32,
    pub lpMediaType: *const REGPINTYPES,
}
impl Default for REGFILTERPINS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REGFILTERPINS2 {
    pub dwFlags: u32,
    pub cInstances: u32,
    pub nMediaTypes: u32,
    pub lpMediaType: *const REGPINTYPES,
    pub nMediums: u32,
    pub lpMedium: *const REGPINMEDIUM,
    pub clsPinCategory: *const windows_sys::core::GUID,
}
impl Default for REGFILTERPINS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REGPINMEDIUM {
    pub clsMedium: windows_sys::core::GUID,
    pub dw1: u32,
    pub dw2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REGPINTYPES {
    pub clsMajorType: *const windows_sys::core::GUID,
    pub clsMinorType: *const windows_sys::core::GUID,
}
impl Default for REGPINTYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REG_PINFLAG_B_MANY: i32 = 4;
pub const REG_PINFLAG_B_OUTPUT: i32 = 8;
pub const REG_PINFLAG_B_RENDERER: i32 = 2;
pub const REG_PINFLAG_B_ZERO: i32 = 1;
pub const REMFILTERF_LEAVECONNECTED: _REM_FILTER_FLAGS = 1;
pub const RenderPrefs_AllowOffscreen: VMRRenderPrefs = 0;
pub const RenderPrefs_AllowOverlays: VMRRenderPrefs = 0;
pub const RenderPrefs_DoNotRenderColorKeyAndBorder: VMRRenderPrefs = 8;
pub const RenderPrefs_ForceOffscreen: VMRRenderPrefs = 1;
pub const RenderPrefs_ForceOverlays: VMRRenderPrefs = 2;
pub const RenderPrefs_Mask: VMRRenderPrefs = 63;
pub const RenderPrefs_PreferAGPMemWhenMixing: VMRRenderPrefs = 32;
pub const RenderPrefs_Reserved: VMRRenderPrefs = 16;
pub const RenderPrefs_RestrictToInitialMonitor: VMRRenderPrefs = 0;
pub type SPRMARRAY = [DVD_REGISTER; 24];
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STREAM_ID_MAP {
    pub stream_id: u32,
    pub dwMediaSampleContent: u32,
    pub ulSubstreamFilterValue: u32,
    pub iDataOffset: i32,
}
pub const SUBSTREAM_FILTER_VAL_NONE: u32 = 268435456;
pub const State_Paused: FILTER_STATE = 1;
pub const State_Running: FILTER_STATE = 2;
pub const State_Stopped: FILTER_STATE = 0;
pub type TVAudioMode = i32;
pub const TunerInputAntenna: TunerInputType = 1;
pub const TunerInputCable: TunerInputType = 0;
pub type TunerInputType = i32;
pub const UOP_FLAG_Pause_On: VALID_UOP_FLAG = 524288;
pub const UOP_FLAG_PlayNext_Chapter: VALID_UOP_FLAG = 128;
pub const UOP_FLAG_PlayPrev_Or_Replay_Chapter: VALID_UOP_FLAG = 64;
pub const UOP_FLAG_Play_Backwards: VALID_UOP_FLAG = 512;
pub const UOP_FLAG_Play_Chapter: VALID_UOP_FLAG = 2;
pub const UOP_FLAG_Play_Chapter_Or_AtTime: VALID_UOP_FLAG = 32;
pub const UOP_FLAG_Play_Forwards: VALID_UOP_FLAG = 256;
pub const UOP_FLAG_Play_Title: VALID_UOP_FLAG = 4;
pub const UOP_FLAG_Play_Title_Or_AtTime: VALID_UOP_FLAG = 1;
pub const UOP_FLAG_Resume: VALID_UOP_FLAG = 65536;
pub const UOP_FLAG_ReturnFromSubMenu: VALID_UOP_FLAG = 16;
pub const UOP_FLAG_Select_Angle: VALID_UOP_FLAG = 4194304;
pub const UOP_FLAG_Select_Audio_Stream: VALID_UOP_FLAG = 1048576;
pub const UOP_FLAG_Select_Karaoke_Audio_Presentation_Mode: VALID_UOP_FLAG = 8388608;
pub const UOP_FLAG_Select_Or_Activate_Button: VALID_UOP_FLAG = 131072;
pub const UOP_FLAG_Select_SubPic_Stream: VALID_UOP_FLAG = 2097152;
pub const UOP_FLAG_Select_Video_Mode_Preference: VALID_UOP_FLAG = 16777216;
pub const UOP_FLAG_ShowMenu_Angle: VALID_UOP_FLAG = 16384;
pub const UOP_FLAG_ShowMenu_Audio: VALID_UOP_FLAG = 8192;
pub const UOP_FLAG_ShowMenu_Chapter: VALID_UOP_FLAG = 32768;
pub const UOP_FLAG_ShowMenu_Root: VALID_UOP_FLAG = 2048;
pub const UOP_FLAG_ShowMenu_SubPic: VALID_UOP_FLAG = 4096;
pub const UOP_FLAG_ShowMenu_Title: VALID_UOP_FLAG = 1024;
pub const UOP_FLAG_Still_Off: VALID_UOP_FLAG = 262144;
pub const UOP_FLAG_Stop: VALID_UOP_FLAG = 8;
pub type VALID_UOP_FLAG = i32;
pub type VALID_UOP_SOMTHING_OR_OTHER = u32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct VIDEO_STREAM_CONFIG_CAPS {
    pub guid: windows_sys::core::GUID,
    pub VideoStandard: u32,
    pub InputSize: super::SIZE,
    pub MinCroppingSize: super::SIZE,
    pub MaxCroppingSize: super::SIZE,
    pub CropGranularityX: i32,
    pub CropGranularityY: i32,
    pub CropAlignX: i32,
    pub CropAlignY: i32,
    pub MinOutputSize: super::SIZE,
    pub MaxOutputSize: super::SIZE,
    pub OutputGranularityX: i32,
    pub OutputGranularityY: i32,
    pub StretchTapsX: i32,
    pub StretchTapsY: i32,
    pub ShrinkTapsX: i32,
    pub ShrinkTapsY: i32,
    pub MinFrameInterval: i64,
    pub MaxFrameInterval: i64,
    pub MinBitsPerSecond: i32,
    pub MaxBitsPerSecond: i32,
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct VMRALLOCATIONINFO {
    pub dwFlags: u32,
    pub lpHdr: super::LPBITMAPINFOHEADER,
    pub lpPixFmt: super::LPDDPIXELFORMAT,
    pub szAspectRatio: super::SIZE,
    pub dwMinBuffers: u32,
    pub dwMaxBuffers: u32,
    pub dwInterlaceFlags: u32,
    pub szNativeSize: super::SIZE,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
impl Default for VMRALLOCATIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ddraw", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct VMRALPHABITMAP {
    pub dwFlags: u32,
    pub hdc: super::HDC,
    pub pDDS: *mut core::ffi::c_void,
    pub rSrc: super::RECT,
    pub rDest: NORMALIZEDRECT,
    pub fAlpha: f32,
    pub clrSrcKey: super::COLORREF,
}
#[cfg(all(feature = "ddraw", feature = "windef"))]
impl Default for VMRALPHABITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VMRBITMAP_DISABLE: u32 = 1;
pub const VMRBITMAP_ENTIREDDS: u32 = 4;
pub const VMRBITMAP_HDC: u32 = 2;
pub const VMRBITMAP_SRCCOLORKEY: u32 = 8;
pub const VMRBITMAP_SRCRECT: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VMRDeinterlaceCaps {
    pub dwSize: u32,
    pub dwNumPreviousOutputFrames: u32,
    pub dwNumForwardRefSamples: u32,
    pub dwNumBackwardRefSamples: u32,
    pub DeinterlaceTechnology: VMRDeinterlaceTech,
}
pub type VMRDeinterlacePrefs = i32;
pub type VMRDeinterlaceTech = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VMRFrequency {
    pub dwNumerator: u32,
    pub dwDenominator: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VMRGUID {
    pub pGUID: *mut windows_sys::core::GUID,
    pub GUID: windows_sys::core::GUID,
}
impl Default for VMRGUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct VMRMONITORINFO {
    pub guid: VMRGUID,
    pub rcMonitor: super::RECT,
    pub hMon: super::HMONITOR,
    pub dwFlags: u32,
    pub szDevice: [u16; 32],
    pub szDescription: [u16; 256],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
}
#[cfg(feature = "windef")]
impl Default for VMRMONITORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VMRMixerPrefs = i32;
pub type VMRMode = i32;
pub const VMRMode_Mask: VMRMode = 7;
pub const VMRMode_Renderless: VMRMode = 4;
pub const VMRMode_Windowed: VMRMode = 1;
pub const VMRMode_Windowless: VMRMode = 2;
#[repr(C)]
#[cfg(all(feature = "ddraw", feature = "ksmedia", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct VMRPRESENTATIONINFO {
    pub dwFlags: u32,
    pub lpSurf: *mut core::ffi::c_void,
    pub rtStart: super::REFERENCE_TIME,
    pub rtEnd: super::REFERENCE_TIME,
    pub szAspectRatio: super::SIZE,
    pub rcSrc: super::RECT,
    pub rcDst: super::RECT,
    pub dwTypeSpecificFlags: u32,
    pub dwInterlaceFlags: u32,
}
#[cfg(all(feature = "ddraw", feature = "ksmedia", feature = "windef"))]
impl Default for VMRPRESENTATIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VMRPresentationFlags = i32;
pub type VMRRenderPrefs = i32;
pub const VMRSample_Discontinuity: VMRPresentationFlags = 4;
pub const VMRSample_Preroll: VMRPresentationFlags = 2;
pub const VMRSample_SrcDstRectsValid: VMRPresentationFlags = 16;
pub const VMRSample_SyncPoint: VMRPresentationFlags = 1;
pub const VMRSample_TimeValid: VMRPresentationFlags = 8;
pub type VMRSurfaceAllocationFlags = i32;
#[repr(C)]
#[cfg(feature = "ddraw")]
#[derive(Clone, Copy)]
pub struct VMRVIDEOSTREAMINFO {
    pub pddsVideoSurface: *mut core::ffi::c_void,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwStrmID: u32,
    pub fAlpha: f32,
    pub ddClrKey: super::DDCOLORKEY,
    pub rNormal: NORMALIZEDRECT,
}
#[cfg(feature = "ddraw")]
impl Default for VMRVIDEOSTREAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VMRVideoDesc {
    pub dwSize: u32,
    pub dwSampleWidth: u32,
    pub dwSampleHeight: u32,
    pub SingleFieldPerSample: windows_sys::core::BOOL,
    pub dwFourCC: u32,
    pub InputSampleFreq: VMRFrequency,
    pub OutputFrameFreq: VMRFrequency,
}
pub const VMR_ARMODE_LETTER_BOX: VMR_ASPECT_RATIO_MODE = 1;
pub const VMR_ARMODE_NONE: VMR_ASPECT_RATIO_MODE = 0;
pub type VMR_ASPECT_RATIO_MODE = i32;
pub const VMR_NOTSUPPORTED: u32 = 0;
pub const VMR_SUPPORTED: u32 = 1;
pub const VfwCaptureDialog_Display: VfwCaptureDialogs = 4;
pub const VfwCaptureDialog_Format: VfwCaptureDialogs = 2;
pub const VfwCaptureDialog_Source: VfwCaptureDialogs = 1;
pub type VfwCaptureDialogs = i32;
pub const VfwCompressDialog_About: VfwCompressDialogs = 2;
pub const VfwCompressDialog_Config: VfwCompressDialogs = 1;
pub const VfwCompressDialog_QueryAbout: VfwCompressDialogs = 8;
pub const VfwCompressDialog_QueryConfig: VfwCompressDialogs = 4;
pub type VfwCompressDialogs = i32;
pub const VideoControlFlag_ExternalTriggerEnable: VideoControlFlags = 4;
pub const VideoControlFlag_FlipHorizontal: VideoControlFlags = 1;
pub const VideoControlFlag_FlipVertical: VideoControlFlags = 2;
pub const VideoControlFlag_Trigger: VideoControlFlags = 8;
pub type VideoControlFlags = i32;
pub const VideoCopyProtectionMacrovisionBasic: VideoCopyProtectionType = 0;
pub const VideoCopyProtectionMacrovisionCBI: VideoCopyProtectionType = 1;
pub type VideoCopyProtectionType = i32;
pub type VideoProcAmpFlags = i32;
pub type VideoProcAmpProperty = i32;
pub const VideoProcAmp_BacklightCompensation: VideoProcAmpProperty = 8;
pub const VideoProcAmp_Brightness: VideoProcAmpProperty = 0;
pub const VideoProcAmp_ColorEnable: VideoProcAmpProperty = 6;
pub const VideoProcAmp_Contrast: VideoProcAmpProperty = 1;
pub const VideoProcAmp_Flags_Auto: VideoProcAmpFlags = 1;
pub const VideoProcAmp_Flags_Manual: VideoProcAmpFlags = 2;
pub const VideoProcAmp_Gain: VideoProcAmpProperty = 9;
pub const VideoProcAmp_Gamma: VideoProcAmpProperty = 5;
pub const VideoProcAmp_Hue: VideoProcAmpProperty = 2;
pub const VideoProcAmp_Saturation: VideoProcAmpProperty = 3;
pub const VideoProcAmp_Sharpness: VideoProcAmpProperty = 4;
pub const VideoProcAmp_WhiteBalance: VideoProcAmpProperty = 7;
pub type _AMRESCTL_RESERVEFLAGS = i32;
pub type _AMSTREAMSELECTENABLEFLAGS = i32;
pub type _AMSTREAMSELECTINFOFLAGS = i32;
pub type _AM_AUDIO_RENDERER_STAT_PARAM = i32;
pub type _AM_FILTER_MISC_FLAGS = i32;
pub type _AM_INTF_SEARCH_FLAGS = i32;
pub type _AM_OVERLAY_NOTIFY_FLAGS = i32;
pub type _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS = i32;
pub type _AM_PUSHSOURCE_FLAGS = i32;
pub type _AM_RENSDEREXFLAGS = i32;
pub type _DVDECODERRESOLUTION = i32;
pub type _DVENCODERFORMAT = i32;
pub type _DVENCODERRESOLUTION = i32;
pub type _DVENCODERVIDEOFORMAT = i32;
pub type _DVRESOLUTION = i32;
pub type _REM_FILTER_FLAGS = i32;
pub type tagAM_MEDIAEVENT_FLAGS = i32;
pub type tagAM_SAMPLE_PROPERTY_FLAGS = i32;
