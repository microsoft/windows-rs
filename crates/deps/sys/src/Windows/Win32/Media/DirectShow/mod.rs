#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Win32_Media_DirectShow_Xml")]
pub mod Xml;
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AMGetErrorTextA(hr: ::windows_sys::core::HRESULT, pbuffer: super::super::Foundation::PSTR, maxlen: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AMGetErrorTextW(hr: ::windows_sys::core::HRESULT, pbuffer: super::super::Foundation::PWSTR, maxlen: u32) -> u32;
}
#[repr(transparent)]
pub struct ADVISE_TYPE(pub u32);
pub const ADVISE_NONE: ADVISE_TYPE = ADVISE_TYPE(0u32);
pub const ADVISE_CLIPPING: ADVISE_TYPE = ADVISE_TYPE(1u32);
pub const ADVISE_PALETTE: ADVISE_TYPE = ADVISE_TYPE(2u32);
pub const ADVISE_COLORKEY: ADVISE_TYPE = ADVISE_TYPE(4u32);
pub const ADVISE_POSITION: ADVISE_TYPE = ADVISE_TYPE(8u32);
pub const ADVISE_DISPLAY_CHANGE: ADVISE_TYPE = ADVISE_TYPE(16u32);
#[repr(C)]
pub struct ALLOCATOR_PROPERTIES(i32);
pub const AMCONTROL_COLORINFO_PRESENT: u32 = 128u32;
pub const AMCONTROL_PAD_TO_16x9: u32 = 4u32;
pub const AMCONTROL_PAD_TO_4x3: u32 = 2u32;
pub const AMCONTROL_USED: u32 = 1u32;
#[repr(C)]
pub struct AMCOPPCommand(i32);
#[repr(C)]
pub struct AMCOPPSignature(i32);
#[repr(C)]
pub struct AMCOPPStatusInput(i32);
#[repr(C)]
pub struct AMCOPPStatusOutput(i32);
pub const AMCOPYPROTECT_RestrictDuplication: u32 = 1u32;
pub const AMDDS_ALL: u32 = 255u32;
pub const AMDDS_DCIPS: u32 = 1u32;
pub const AMDDS_DEFAULT: u32 = 255u32;
pub const AMDDS_NONE: u32 = 0u32;
pub const AMDDS_PS: u32 = 2u32;
pub const AMDDS_RGBFLP: u32 = 64u32;
pub const AMDDS_RGBOFF: u32 = 16u32;
pub const AMDDS_RGBOVR: u32 = 4u32;
pub const AMDDS_YUVFLP: u32 = 128u32;
pub const AMDDS_YUVOFF: u32 = 32u32;
pub const AMDDS_YUVOVR: u32 = 8u32;
#[repr(transparent)]
pub struct AMExtendedSeekingCapabilities(pub i32);
pub const AM_EXSEEK_CANSEEK: AMExtendedSeekingCapabilities = AMExtendedSeekingCapabilities(1i32);
pub const AM_EXSEEK_CANSCAN: AMExtendedSeekingCapabilities = AMExtendedSeekingCapabilities(2i32);
pub const AM_EXSEEK_MARKERSEEK: AMExtendedSeekingCapabilities = AMExtendedSeekingCapabilities(4i32);
pub const AM_EXSEEK_SCANWITHOUTCLOCK: AMExtendedSeekingCapabilities = AMExtendedSeekingCapabilities(8i32);
pub const AM_EXSEEK_NOSTANDARDREPAINT: AMExtendedSeekingCapabilities = AMExtendedSeekingCapabilities(16i32);
pub const AM_EXSEEK_BUFFERING: AMExtendedSeekingCapabilities = AMExtendedSeekingCapabilities(32i32);
pub const AM_EXSEEK_SENDS_VIDEOFRAMEREADY: AMExtendedSeekingCapabilities = AMExtendedSeekingCapabilities(64i32);
pub const AMF_AUTOMATICGAIN: f64 = -1f64;
#[repr(C)]
pub struct AMGETERRORTEXTPROCA(i32);
#[repr(C)]
pub struct AMGETERRORTEXTPROCW(i32);
pub const AMINTERLACE_1FieldPerSample: u32 = 2u32;
pub const AMINTERLACE_DisplayModeBobOnly: u32 = 0u32;
pub const AMINTERLACE_DisplayModeBobOrWeave: u32 = 128u32;
pub const AMINTERLACE_DisplayModeMask: u32 = 192u32;
pub const AMINTERLACE_DisplayModeWeaveOnly: u32 = 64u32;
pub const AMINTERLACE_Field1First: u32 = 4u32;
pub const AMINTERLACE_FieldPatBothIrregular: u32 = 48u32;
pub const AMINTERLACE_FieldPatBothRegular: u32 = 32u32;
pub const AMINTERLACE_FieldPatField1Only: u32 = 0u32;
pub const AMINTERLACE_FieldPatField2Only: u32 = 16u32;
pub const AMINTERLACE_FieldPatternMask: u32 = 48u32;
pub const AMINTERLACE_IsInterlaced: u32 = 1u32;
pub const AMINTERLACE_UNUSED: u32 = 8u32;
#[repr(transparent)]
pub struct AMMSF_MMS_INIT_FLAGS(pub u32);
pub const AMMSF_NOGRAPHTHREAD: AMMSF_MMS_INIT_FLAGS = AMMSF_MMS_INIT_FLAGS(1u32);
#[repr(transparent)]
pub struct AMMSF_MS_FLAGS(pub u32);
pub const AMMSF_ADDDEFAULTRENDERER: AMMSF_MS_FLAGS = AMMSF_MS_FLAGS(1u32);
pub const AMMSF_CREATEPEER: AMMSF_MS_FLAGS = AMMSF_MS_FLAGS(2u32);
pub const AMMSF_STOPIFNOSAMPLES: AMMSF_MS_FLAGS = AMMSF_MS_FLAGS(4u32);
pub const AMMSF_NOSTALL: AMMSF_MS_FLAGS = AMMSF_MS_FLAGS(8u32);
#[repr(transparent)]
pub struct AMMSF_RENDER_FLAGS(pub u32);
pub const AMMSF_RENDERTYPEMASK: AMMSF_RENDER_FLAGS = AMMSF_RENDER_FLAGS(3u32);
pub const AMMSF_RENDERTOEXISTING: AMMSF_RENDER_FLAGS = AMMSF_RENDER_FLAGS(0u32);
pub const AMMSF_RENDERALLSTREAMS: AMMSF_RENDER_FLAGS = AMMSF_RENDER_FLAGS(1u32);
pub const AMMSF_NORENDER: AMMSF_RENDER_FLAGS = AMMSF_RENDER_FLAGS(2u32);
pub const AMMSF_NOCLOCK: AMMSF_RENDER_FLAGS = AMMSF_RENDER_FLAGS(4u32);
pub const AMMSF_RUN: AMMSF_RENDER_FLAGS = AMMSF_RENDER_FLAGS(8u32);
#[repr(transparent)]
pub struct AMOVERLAYFX(pub i32);
pub const AMOVERFX_NOFX: AMOVERLAYFX = AMOVERLAYFX(0i32);
pub const AMOVERFX_MIRRORLEFTRIGHT: AMOVERLAYFX = AMOVERLAYFX(2i32);
pub const AMOVERFX_MIRRORUPDOWN: AMOVERLAYFX = AMOVERLAYFX(4i32);
pub const AMOVERFX_DEINTERLACE: AMOVERLAYFX = AMOVERLAYFX(8i32);
#[repr(transparent)]
pub struct AMPROPERTY_PIN(pub i32);
pub const AMPROPERTY_PIN_CATEGORY: AMPROPERTY_PIN = AMPROPERTY_PIN(0i32);
pub const AMPROPERTY_PIN_MEDIUM: AMPROPERTY_PIN = AMPROPERTY_PIN(1i32);
#[repr(transparent)]
pub struct AMPlayListEventFlags(pub i32);
pub const AMPLAYLISTEVENT_RESUME: AMPlayListEventFlags = AMPlayListEventFlags(0i32);
pub const AMPLAYLISTEVENT_BREAK: AMPlayListEventFlags = AMPlayListEventFlags(1i32);
pub const AMPLAYLISTEVENT_NEXT: AMPlayListEventFlags = AMPlayListEventFlags(2i32);
pub const AMPLAYLISTEVENT_MASK: AMPlayListEventFlags = AMPlayListEventFlags(15i32);
pub const AMPLAYLISTEVENT_REFRESH: AMPlayListEventFlags = AMPlayListEventFlags(16i32);
#[repr(transparent)]
pub struct AMPlayListFlags(pub i32);
pub const AMPLAYLIST_STARTINSCANMODE: AMPlayListFlags = AMPlayListFlags(1i32);
pub const AMPLAYLIST_FORCEBANNER: AMPlayListFlags = AMPlayListFlags(2i32);
#[repr(transparent)]
pub struct AMPlayListItemFlags(pub i32);
pub const AMPLAYLISTITEM_CANSKIP: AMPlayListItemFlags = AMPlayListItemFlags(1i32);
pub const AMPLAYLISTITEM_CANBIND: AMPlayListItemFlags = AMPlayListItemFlags(2i32);
#[repr(transparent)]
pub struct AMTVAudioEventType(pub i32);
pub const AMTVAUDIO_EVENT_CHANGED: AMTVAudioEventType = AMTVAudioEventType(1i32);
#[repr(transparent)]
pub struct AMTunerEventType(pub i32);
pub const AMTUNER_EVENT_CHANGED: AMTunerEventType = AMTunerEventType(1i32);
#[repr(transparent)]
pub struct AMTunerModeType(pub i32);
pub const AMTUNER_MODE_DEFAULT: AMTunerModeType = AMTunerModeType(0i32);
pub const AMTUNER_MODE_TV: AMTunerModeType = AMTunerModeType(1i32);
pub const AMTUNER_MODE_FM_RADIO: AMTunerModeType = AMTunerModeType(2i32);
pub const AMTUNER_MODE_AM_RADIO: AMTunerModeType = AMTunerModeType(4i32);
pub const AMTUNER_MODE_DSS: AMTunerModeType = AMTunerModeType(8i32);
#[repr(transparent)]
pub struct AMTunerSignalStrength(pub i32);
pub const AMTUNER_HASNOSIGNALSTRENGTH: AMTunerSignalStrength = AMTunerSignalStrength(-1i32);
pub const AMTUNER_NOSIGNAL: AMTunerSignalStrength = AMTunerSignalStrength(0i32);
pub const AMTUNER_SIGNALPRESENT: AMTunerSignalStrength = AMTunerSignalStrength(1i32);
#[repr(transparent)]
pub struct AMTunerSubChannel(pub i32);
pub const AMTUNER_SUBCHAN_NO_TUNE: AMTunerSubChannel = AMTunerSubChannel(-2i32);
pub const AMTUNER_SUBCHAN_DEFAULT: AMTunerSubChannel = AMTunerSubChannel(-1i32);
#[repr(C)]
pub struct AMVABUFFERINFO(i32);
#[repr(C)]
pub struct AMVABeginFrameInfo(i32);
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[repr(C)]
pub struct AMVACompBufferInfo(i32);
#[repr(C)]
pub struct AMVAEndFrameInfo(i32);
#[repr(C)]
pub struct AMVAInternalMemInfo(i32);
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[repr(C)]
pub struct AMVAUncompBufferInfo(i32);
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[repr(C)]
pub struct AMVAUncompDataInfo(i32);
pub const AMVA_QUERYRENDERSTATUSF_READ: u32 = 1u32;
pub const AMVA_TYPEINDEX_OUTPUTFRAME: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AMVPDATAINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AMVPDIMINFO(i32);
#[repr(C)]
pub struct AMVPSIZE(i32);
#[repr(transparent)]
pub struct AMVP_MODE(pub i32);
pub const AMVP_MODE_WEAVE: AMVP_MODE = AMVP_MODE(0i32);
pub const AMVP_MODE_BOBINTERLEAVED: AMVP_MODE = AMVP_MODE(1i32);
pub const AMVP_MODE_BOBNONINTERLEAVED: AMVP_MODE = AMVP_MODE(2i32);
pub const AMVP_MODE_SKIPEVEN: AMVP_MODE = AMVP_MODE(3i32);
pub const AMVP_MODE_SKIPODD: AMVP_MODE = AMVP_MODE(4i32);
#[repr(transparent)]
pub struct AMVP_SELECT_FORMAT_BY(pub i32);
pub const AMVP_DO_NOT_CARE: AMVP_SELECT_FORMAT_BY = AMVP_SELECT_FORMAT_BY(0i32);
pub const AMVP_BEST_BANDWIDTH: AMVP_SELECT_FORMAT_BY = AMVP_SELECT_FORMAT_BY(1i32);
pub const AMVP_INPUT_SAME_AS_OUTPUT: AMVP_SELECT_FORMAT_BY = AMVP_SELECT_FORMAT_BY(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_AC3_ALTERNATE_AUDIO(i32);
pub const AM_AC3_ALTERNATE_AUDIO_1: u32 = 1u32;
pub const AM_AC3_ALTERNATE_AUDIO_2: u32 = 2u32;
pub const AM_AC3_ALTERNATE_AUDIO_BOTH: u32 = 3u32;
#[repr(C)]
pub struct AM_AC3_BIT_STREAM_MODE(i32);
#[repr(C)]
pub struct AM_AC3_DIALOGUE_LEVEL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_AC3_DOWNMIX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_AC3_ERROR_CONCEALMENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_AC3_ROOM_TYPE(i32);
pub const AM_AC3_SERVICE_COMMENTARY: u32 = 5u32;
pub const AM_AC3_SERVICE_DIALOG_ONLY: u32 = 4u32;
pub const AM_AC3_SERVICE_EMERGENCY_FLASH: u32 = 6u32;
pub const AM_AC3_SERVICE_HEARING_IMPAIRED: u32 = 3u32;
pub const AM_AC3_SERVICE_MAIN_AUDIO: u32 = 0u32;
pub const AM_AC3_SERVICE_NO_DIALOG: u32 = 1u32;
pub const AM_AC3_SERVICE_VISUALLY_IMPAIRED: u32 = 2u32;
pub const AM_AC3_SERVICE_VOICE_OVER: u32 = 7u32;
#[repr(transparent)]
pub struct AM_ASPECT_RATIO_MODE(pub i32);
pub const AM_ARMODE_STRETCHED: AM_ASPECT_RATIO_MODE = AM_ASPECT_RATIO_MODE(0i32);
pub const AM_ARMODE_LETTER_BOX: AM_ASPECT_RATIO_MODE = AM_ASPECT_RATIO_MODE(1i32);
pub const AM_ARMODE_CROP: AM_ASPECT_RATIO_MODE = AM_ASPECT_RATIO_MODE(2i32);
pub const AM_ARMODE_STRETCHED_AS_PRIMARY: AM_ASPECT_RATIO_MODE = AM_ASPECT_RATIO_MODE(3i32);
#[repr(C)]
pub struct AM_COLCON(i32);
pub const AM_CONTENTPROPERTY_AUTHOR: u32 = 2u32;
pub const AM_CONTENTPROPERTY_COPYRIGHT: u32 = 4u32;
pub const AM_CONTENTPROPERTY_DESCRIPTION: u32 = 8u32;
pub const AM_CONTENTPROPERTY_TITLE: u32 = 1u32;
#[repr(C)]
pub struct AM_COPY_MACROVISION(i32);
#[repr(transparent)]
pub struct AM_COPY_MACROVISION_LEVEL(pub i32);
pub const AM_MACROVISION_DISABLED: AM_COPY_MACROVISION_LEVEL = AM_COPY_MACROVISION_LEVEL(0i32);
pub const AM_MACROVISION_LEVEL1: AM_COPY_MACROVISION_LEVEL = AM_COPY_MACROVISION_LEVEL(1i32);
pub const AM_MACROVISION_LEVEL2: AM_COPY_MACROVISION_LEVEL = AM_COPY_MACROVISION_LEVEL(2i32);
pub const AM_MACROVISION_LEVEL3: AM_COPY_MACROVISION_LEVEL = AM_COPY_MACROVISION_LEVEL(3i32);
#[repr(transparent)]
pub struct AM_DIGITAL_CP(pub i32);
pub const AM_DIGITAL_CP_OFF: AM_DIGITAL_CP = AM_DIGITAL_CP(0i32);
pub const AM_DIGITAL_CP_ON: AM_DIGITAL_CP = AM_DIGITAL_CP(1i32);
pub const AM_DIGITAL_CP_DVD_COMPLIANT: AM_DIGITAL_CP = AM_DIGITAL_CP(2i32);
#[repr(transparent)]
pub struct AM_DVDCOPYSTATE(pub i32);
pub const AM_DVDCOPYSTATE_INITIALIZE: AM_DVDCOPYSTATE = AM_DVDCOPYSTATE(0i32);
pub const AM_DVDCOPYSTATE_INITIALIZE_TITLE: AM_DVDCOPYSTATE = AM_DVDCOPYSTATE(1i32);
pub const AM_DVDCOPYSTATE_AUTHENTICATION_NOT_REQUIRED: AM_DVDCOPYSTATE = AM_DVDCOPYSTATE(2i32);
pub const AM_DVDCOPYSTATE_AUTHENTICATION_REQUIRED: AM_DVDCOPYSTATE = AM_DVDCOPYSTATE(3i32);
pub const AM_DVDCOPYSTATE_DONE: AM_DVDCOPYSTATE = AM_DVDCOPYSTATE(4i32);
#[repr(C)]
pub struct AM_DVDCOPY_BUSKEY(i32);
#[repr(C)]
pub struct AM_DVDCOPY_CHLGKEY(i32);
#[repr(C)]
pub struct AM_DVDCOPY_DISCKEY(i32);
#[repr(C)]
pub struct AM_DVDCOPY_SET_COPY_STATE(i32);
#[repr(C)]
pub struct AM_DVDCOPY_TITLEKEY(i32);
pub const AM_DVD_CGMS_COPY_ONCE: u32 = 16u32;
pub const AM_DVD_CGMS_COPY_PERMITTED: u32 = 0u32;
pub const AM_DVD_CGMS_COPY_PROTECT_MASK: u32 = 24u32;
pub const AM_DVD_CGMS_NO_COPY: u32 = 24u32;
pub const AM_DVD_CGMS_RESERVED_MASK: u32 = 120u32;
pub const AM_DVD_COPYRIGHTED: u32 = 64u32;
pub const AM_DVD_COPYRIGHT_MASK: u32 = 64u32;
#[repr(C)]
pub struct AM_DVD_ChangeRate(i32);
#[repr(transparent)]
pub struct AM_DVD_GRAPH_FLAGS(pub i32);
pub const AM_DVD_HWDEC_PREFER: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(1i32);
pub const AM_DVD_HWDEC_ONLY: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(2i32);
pub const AM_DVD_SWDEC_PREFER: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(4i32);
pub const AM_DVD_SWDEC_ONLY: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(8i32);
pub const AM_DVD_NOVPE: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(256i32);
pub const AM_DVD_DO_NOT_CLEAR: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(512i32);
pub const AM_DVD_VMR9_ONLY: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(2048i32);
pub const AM_DVD_EVR_ONLY: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(4096i32);
pub const AM_DVD_EVR_QOS: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(8192i32);
pub const AM_DVD_ADAPT_GRAPH: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(16384i32);
pub const AM_DVD_MASK: AM_DVD_GRAPH_FLAGS = AM_DVD_GRAPH_FLAGS(65535i32);
pub const AM_DVD_NOT_COPYRIGHTED: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_DVD_RENDERSTATUS(i32);
pub const AM_DVD_SECTOR_NOT_PROTECTED: u32 = 0u32;
pub const AM_DVD_SECTOR_PROTECTED: u32 = 32u32;
pub const AM_DVD_SECTOR_PROTECT_MASK: u32 = 32u32;
#[repr(transparent)]
pub struct AM_DVD_STREAM_FLAGS(pub i32);
pub const AM_DVD_STREAM_VIDEO: AM_DVD_STREAM_FLAGS = AM_DVD_STREAM_FLAGS(1i32);
pub const AM_DVD_STREAM_AUDIO: AM_DVD_STREAM_FLAGS = AM_DVD_STREAM_FLAGS(2i32);
pub const AM_DVD_STREAM_SUBPIC: AM_DVD_STREAM_FLAGS = AM_DVD_STREAM_FLAGS(4i32);
#[repr(C)]
pub struct AM_DVD_YUV(i32);
#[repr(C)]
pub struct AM_DvdKaraokeData(i32);
#[repr(C)]
pub struct AM_ExactRateChange(i32);
#[repr(transparent)]
pub struct AM_FILESINK_FLAGS(pub i32);
pub const AM_FILE_OVERWRITE: AM_FILESINK_FLAGS = AM_FILESINK_FLAGS(1i32);
#[repr(transparent)]
pub struct AM_FILTER_FLAGS(pub i32);
pub const AM_FILTER_FLAGS_REMOVABLE: AM_FILTER_FLAGS = AM_FILTER_FLAGS(1i32);
#[repr(C)]
pub struct AM_FRAMESTEP_STEP(i32);
pub const AM_GBF_NODDSURFACELOCK: u32 = 8u32;
pub const AM_GBF_NOTASYNCPOINT: u32 = 2u32;
pub const AM_GBF_NOWAIT: u32 = 4u32;
pub const AM_GBF_PREVFRAMESKIPPED: u32 = 1u32;
pub const AM_GETDECODERCAP_QUERY_EVR_SUPPORT: u32 = 7u32;
pub const AM_GETDECODERCAP_QUERY_VMR9_SUPPORT: u32 = 6u32;
pub const AM_GETDECODERCAP_QUERY_VMR_SUPPORT: u32 = 1u32;
#[repr(transparent)]
pub struct AM_GRAPH_CONFIG_RECONNECT_FLAGS(pub i32);
pub const AM_GRAPH_CONFIG_RECONNECT_DIRECTCONNECT: AM_GRAPH_CONFIG_RECONNECT_FLAGS = AM_GRAPH_CONFIG_RECONNECT_FLAGS(1i32);
pub const AM_GRAPH_CONFIG_RECONNECT_CACHE_REMOVED_FILTERS: AM_GRAPH_CONFIG_RECONNECT_FLAGS = AM_GRAPH_CONFIG_RECONNECT_FLAGS(2i32);
pub const AM_GRAPH_CONFIG_RECONNECT_USE_ONLY_CACHED_FILTERS: AM_GRAPH_CONFIG_RECONNECT_FLAGS = AM_GRAPH_CONFIG_RECONNECT_FLAGS(4i32);
#[repr(transparent)]
pub struct AM_LINE21_CCLEVEL(pub i32);
pub const AM_L21_CCLEVEL_TC2: AM_LINE21_CCLEVEL = AM_LINE21_CCLEVEL(0i32);
#[repr(transparent)]
pub struct AM_LINE21_CCSERVICE(pub i32);
pub const AM_L21_CCSERVICE_None: AM_LINE21_CCSERVICE = AM_LINE21_CCSERVICE(0i32);
pub const AM_L21_CCSERVICE_Caption1: AM_LINE21_CCSERVICE = AM_LINE21_CCSERVICE(1i32);
pub const AM_L21_CCSERVICE_Caption2: AM_LINE21_CCSERVICE = AM_LINE21_CCSERVICE(2i32);
pub const AM_L21_CCSERVICE_Text1: AM_LINE21_CCSERVICE = AM_LINE21_CCSERVICE(3i32);
pub const AM_L21_CCSERVICE_Text2: AM_LINE21_CCSERVICE = AM_LINE21_CCSERVICE(4i32);
pub const AM_L21_CCSERVICE_XDS: AM_LINE21_CCSERVICE = AM_LINE21_CCSERVICE(5i32);
pub const AM_L21_CCSERVICE_DefChannel: AM_LINE21_CCSERVICE = AM_LINE21_CCSERVICE(10i32);
pub const AM_L21_CCSERVICE_Invalid: AM_LINE21_CCSERVICE = AM_LINE21_CCSERVICE(11i32);
#[repr(transparent)]
pub struct AM_LINE21_CCSTATE(pub i32);
pub const AM_L21_CCSTATE_Off: AM_LINE21_CCSTATE = AM_LINE21_CCSTATE(0i32);
pub const AM_L21_CCSTATE_On: AM_LINE21_CCSTATE = AM_LINE21_CCSTATE(1i32);
#[repr(transparent)]
pub struct AM_LINE21_CCSTYLE(pub i32);
pub const AM_L21_CCSTYLE_None: AM_LINE21_CCSTYLE = AM_LINE21_CCSTYLE(0i32);
pub const AM_L21_CCSTYLE_PopOn: AM_LINE21_CCSTYLE = AM_LINE21_CCSTYLE(1i32);
pub const AM_L21_CCSTYLE_PaintOn: AM_LINE21_CCSTYLE = AM_LINE21_CCSTYLE(2i32);
pub const AM_L21_CCSTYLE_RollUp: AM_LINE21_CCSTYLE = AM_LINE21_CCSTYLE(3i32);
#[repr(transparent)]
pub struct AM_LINE21_DRAWBGMODE(pub i32);
pub const AM_L21_DRAWBGMODE_Opaque: AM_LINE21_DRAWBGMODE = AM_LINE21_DRAWBGMODE(0i32);
pub const AM_L21_DRAWBGMODE_Transparent: AM_LINE21_DRAWBGMODE = AM_LINE21_DRAWBGMODE(1i32);
pub const AM_LOADSTATUS_CLOSED: u32 = 0u32;
pub const AM_LOADSTATUS_CONNECTING: u32 = 4u32;
pub const AM_LOADSTATUS_LOADINGDESCR: u32 = 1u32;
pub const AM_LOADSTATUS_LOADINGMCAST: u32 = 2u32;
pub const AM_LOADSTATUS_LOCATING: u32 = 3u32;
pub const AM_LOADSTATUS_OPEN: u32 = 6u32;
pub const AM_LOADSTATUS_OPENING: u32 = 5u32;
#[repr(transparent)]
pub struct AM_MEDIAEVENT_FLAGS(pub i32);
pub const AM_MEDIAEVENT_NONOTIFY: AM_MEDIAEVENT_FLAGS = AM_MEDIAEVENT_FLAGS(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_MEDIA_TYPE(i32);
#[repr(transparent)]
pub struct AM_MPEG2Level(pub i32);
pub const AM_MPEG2Level_Low: AM_MPEG2Level = AM_MPEG2Level(1i32);
pub const AM_MPEG2Level_Main: AM_MPEG2Level = AM_MPEG2Level(2i32);
pub const AM_MPEG2Level_High1440: AM_MPEG2Level = AM_MPEG2Level(3i32);
pub const AM_MPEG2Level_High: AM_MPEG2Level = AM_MPEG2Level(4i32);
#[repr(transparent)]
pub struct AM_MPEG2Profile(pub i32);
pub const AM_MPEG2Profile_Simple: AM_MPEG2Profile = AM_MPEG2Profile(1i32);
pub const AM_MPEG2Profile_Main: AM_MPEG2Profile = AM_MPEG2Profile(2i32);
pub const AM_MPEG2Profile_SNRScalable: AM_MPEG2Profile = AM_MPEG2Profile(3i32);
pub const AM_MPEG2Profile_SpatiallyScalable: AM_MPEG2Profile = AM_MPEG2Profile(4i32);
pub const AM_MPEG2Profile_High: AM_MPEG2Profile = AM_MPEG2Profile(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_MPEGSTREAMTYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_MPEGSYSTEMTYPE(i32);
pub const AM_MPEG_AUDIO_DUAL_LEFT: u32 = 1u32;
pub const AM_MPEG_AUDIO_DUAL_MERGE: u32 = 0u32;
pub const AM_MPEG_AUDIO_DUAL_RIGHT: u32 = 2u32;
#[repr(transparent)]
pub struct AM_PROPERTY_AC3(pub i32);
pub const AM_PROPERTY_AC3_ERROR_CONCEALMENT: AM_PROPERTY_AC3 = AM_PROPERTY_AC3(1i32);
pub const AM_PROPERTY_AC3_ALTERNATE_AUDIO: AM_PROPERTY_AC3 = AM_PROPERTY_AC3(2i32);
pub const AM_PROPERTY_AC3_DOWNMIX: AM_PROPERTY_AC3 = AM_PROPERTY_AC3(3i32);
pub const AM_PROPERTY_AC3_BIT_STREAM_MODE: AM_PROPERTY_AC3 = AM_PROPERTY_AC3(4i32);
pub const AM_PROPERTY_AC3_DIALOGUE_LEVEL: AM_PROPERTY_AC3 = AM_PROPERTY_AC3(5i32);
pub const AM_PROPERTY_AC3_LANGUAGE_CODE: AM_PROPERTY_AC3 = AM_PROPERTY_AC3(6i32);
pub const AM_PROPERTY_AC3_ROOM_TYPE: AM_PROPERTY_AC3 = AM_PROPERTY_AC3(7i32);
#[repr(transparent)]
pub struct AM_PROPERTY_DVDCOPYPROT(pub i32);
pub const AM_PROPERTY_DVDCOPY_CHLG_KEY: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(1i32);
pub const AM_PROPERTY_DVDCOPY_DVD_KEY1: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(2i32);
pub const AM_PROPERTY_DVDCOPY_DEC_KEY2: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(3i32);
pub const AM_PROPERTY_DVDCOPY_TITLE_KEY: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(4i32);
pub const AM_PROPERTY_COPY_MACROVISION: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(5i32);
pub const AM_PROPERTY_DVDCOPY_REGION: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(6i32);
pub const AM_PROPERTY_DVDCOPY_SET_COPY_STATE: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(7i32);
pub const AM_PROPERTY_COPY_ANALOG_COMPONENT: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(8i32);
pub const AM_PROPERTY_COPY_DIGITAL_CP: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(9i32);
pub const AM_PROPERTY_COPY_DVD_SRM: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(10i32);
pub const AM_PROPERTY_DVDCOPY_SUPPORTS_NEW_KEYCOUNT: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(11i32);
pub const AM_PROPERTY_DVDCOPY_DISC_KEY: AM_PROPERTY_DVDCOPYPROT = AM_PROPERTY_DVDCOPYPROT(128i32);
#[repr(transparent)]
pub struct AM_PROPERTY_DVDKARAOKE(pub i32);
pub const AM_PROPERTY_DVDKARAOKE_ENABLE: AM_PROPERTY_DVDKARAOKE = AM_PROPERTY_DVDKARAOKE(0i32);
pub const AM_PROPERTY_DVDKARAOKE_DATA: AM_PROPERTY_DVDKARAOKE = AM_PROPERTY_DVDKARAOKE(1i32);
#[repr(transparent)]
pub struct AM_PROPERTY_DVDSUBPIC(pub i32);
pub const AM_PROPERTY_DVDSUBPIC_PALETTE: AM_PROPERTY_DVDSUBPIC = AM_PROPERTY_DVDSUBPIC(0i32);
pub const AM_PROPERTY_DVDSUBPIC_HLI: AM_PROPERTY_DVDSUBPIC = AM_PROPERTY_DVDSUBPIC(1i32);
pub const AM_PROPERTY_DVDSUBPIC_COMPOSIT_ON: AM_PROPERTY_DVDSUBPIC = AM_PROPERTY_DVDSUBPIC(2i32);
#[repr(transparent)]
pub struct AM_PROPERTY_DVD_RATE_CHANGE(pub i32);
pub const AM_RATE_ChangeRate: AM_PROPERTY_DVD_RATE_CHANGE = AM_PROPERTY_DVD_RATE_CHANGE(1i32);
pub const AM_RATE_FullDataRateMax: AM_PROPERTY_DVD_RATE_CHANGE = AM_PROPERTY_DVD_RATE_CHANGE(2i32);
pub const AM_RATE_ReverseDecode: AM_PROPERTY_DVD_RATE_CHANGE = AM_PROPERTY_DVD_RATE_CHANGE(3i32);
pub const AM_RATE_DecoderPosition: AM_PROPERTY_DVD_RATE_CHANGE = AM_PROPERTY_DVD_RATE_CHANGE(4i32);
pub const AM_RATE_DecoderVersion: AM_PROPERTY_DVD_RATE_CHANGE = AM_PROPERTY_DVD_RATE_CHANGE(5i32);
#[repr(transparent)]
pub struct AM_PROPERTY_FRAMESTEP(pub i32);
pub const AM_PROPERTY_FRAMESTEP_STEP: AM_PROPERTY_FRAMESTEP = AM_PROPERTY_FRAMESTEP(1i32);
pub const AM_PROPERTY_FRAMESTEP_CANCEL: AM_PROPERTY_FRAMESTEP = AM_PROPERTY_FRAMESTEP(2i32);
pub const AM_PROPERTY_FRAMESTEP_CANSTEP: AM_PROPERTY_FRAMESTEP = AM_PROPERTY_FRAMESTEP(3i32);
pub const AM_PROPERTY_FRAMESTEP_CANSTEPMULTIPLE: AM_PROPERTY_FRAMESTEP = AM_PROPERTY_FRAMESTEP(4i32);
#[repr(C)]
pub struct AM_PROPERTY_SPHLI(i32);
#[repr(C)]
pub struct AM_PROPERTY_SPPAL(i32);
#[repr(transparent)]
pub struct AM_PROPERTY_TS_RATE_CHANGE(pub i32);
pub const AM_RATE_SimpleRateChange: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(1i32);
pub const AM_RATE_ExactRateChange: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(2i32);
pub const AM_RATE_MaxFullDataRate: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(3i32);
pub const AM_RATE_Step: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(4i32);
pub const AM_RATE_UseRateVersion: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(5i32);
pub const AM_RATE_QueryFullFrameRate: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(6i32);
pub const AM_RATE_QueryLastRateSegPTS: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(7i32);
pub const AM_RATE_CorrectTS: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(8i32);
pub const AM_RATE_ReverseMaxFullDataRate: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(9i32);
pub const AM_RATE_ResetOnTimeDisc: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(10i32);
pub const AM_RATE_QueryMapping: AM_PROPERTY_TS_RATE_CHANGE = AM_PROPERTY_TS_RATE_CHANGE(11i32);
pub const AM_QUERY_DECODER_ATSC_HD_SUPPORT: u32 = 5u32;
pub const AM_QUERY_DECODER_ATSC_SD_SUPPORT: u32 = 4u32;
pub const AM_QUERY_DECODER_DVD_SUPPORT: u32 = 3u32;
pub const AM_QUERY_DECODER_DXVA_1_SUPPORT: u32 = 2u32;
pub const AM_QUERY_DECODER_VMR_SUPPORT: u32 = 1u32;
#[repr(C)]
pub struct AM_QueryRate(i32);
pub const AM_ReverseBlockEnd: u32 = 4u32;
pub const AM_ReverseBlockStart: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_SAMPLE2_PROPERTIES(i32);
#[repr(transparent)]
pub struct AM_SAMPLE_PROPERTY_FLAGS(pub i32);
pub const AM_SAMPLE_SPLICEPOINT: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(1i32);
pub const AM_SAMPLE_PREROLL: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(2i32);
pub const AM_SAMPLE_DATADISCONTINUITY: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(4i32);
pub const AM_SAMPLE_TYPECHANGED: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(8i32);
pub const AM_SAMPLE_TIMEVALID: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(16i32);
pub const AM_SAMPLE_TIMEDISCONTINUITY: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(64i32);
pub const AM_SAMPLE_FLUSH_ON_PAUSE: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(128i32);
pub const AM_SAMPLE_STOPVALID: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(256i32);
pub const AM_SAMPLE_ENDOFSTREAM: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(512i32);
pub const AM_STREAM_MEDIA: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(0i32);
pub const AM_STREAM_CONTROL: AM_SAMPLE_PROPERTY_FLAGS = AM_SAMPLE_PROPERTY_FLAGS(1i32);
#[repr(transparent)]
pub struct AM_SEEKING_SEEKING_CAPABILITIES(pub i32);
pub const AM_SEEKING_CanSeekAbsolute: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(1i32);
pub const AM_SEEKING_CanSeekForwards: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(2i32);
pub const AM_SEEKING_CanSeekBackwards: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(4i32);
pub const AM_SEEKING_CanGetCurrentPos: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(8i32);
pub const AM_SEEKING_CanGetStopPos: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(16i32);
pub const AM_SEEKING_CanGetDuration: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(32i32);
pub const AM_SEEKING_CanPlayBackwards: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(64i32);
pub const AM_SEEKING_CanDoSegments: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(128i32);
pub const AM_SEEKING_Source: AM_SEEKING_SEEKING_CAPABILITIES = AM_SEEKING_SEEKING_CAPABILITIES(256i32);
#[repr(transparent)]
pub struct AM_SEEKING_SeekingFlags(pub i32);
pub const AM_SEEKING_NoPositioning: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(0i32);
pub const AM_SEEKING_AbsolutePositioning: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(1i32);
pub const AM_SEEKING_RelativePositioning: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(2i32);
pub const AM_SEEKING_IncrementalPositioning: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(3i32);
pub const AM_SEEKING_PositioningBitsMask: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(3i32);
pub const AM_SEEKING_SeekToKeyFrame: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(4i32);
pub const AM_SEEKING_ReturnTime: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(8i32);
pub const AM_SEEKING_Segment: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(16i32);
pub const AM_SEEKING_NoFlush: AM_SEEKING_SeekingFlags = AM_SEEKING_SeekingFlags(32i32);
#[repr(C)]
pub struct AM_STREAM_INFO(i32);
#[repr(transparent)]
pub struct AM_STREAM_INFO_FLAGS(pub i32);
pub const AM_STREAM_INFO_START_DEFINED: AM_STREAM_INFO_FLAGS = AM_STREAM_INFO_FLAGS(1i32);
pub const AM_STREAM_INFO_STOP_DEFINED: AM_STREAM_INFO_FLAGS = AM_STREAM_INFO_FLAGS(2i32);
pub const AM_STREAM_INFO_DISCARDING: AM_STREAM_INFO_FLAGS = AM_STREAM_INFO_FLAGS(4i32);
pub const AM_STREAM_INFO_STOP_SEND_EXTRA: AM_STREAM_INFO_FLAGS = AM_STREAM_INFO_FLAGS(16i32);
#[repr(C)]
pub struct AM_SimpleRateChange(i32);
pub const AM_UseNewCSSKey: u32 = 1u32;
pub const AM_VIDEO_FLAG_B_SAMPLE: i32 = 32i32;
pub const AM_VIDEO_FLAG_FIELD1: i32 = 1i32;
pub const AM_VIDEO_FLAG_FIELD1FIRST: i32 = 4i32;
pub const AM_VIDEO_FLAG_FIELD2: i32 = 2i32;
pub const AM_VIDEO_FLAG_FIELD_MASK: i32 = 3i32;
pub const AM_VIDEO_FLAG_INTERLEAVED_FRAME: i32 = 0i32;
pub const AM_VIDEO_FLAG_IPB_MASK: i32 = 48i32;
pub const AM_VIDEO_FLAG_I_SAMPLE: i32 = 0i32;
pub const AM_VIDEO_FLAG_P_SAMPLE: i32 = 16i32;
pub const AM_VIDEO_FLAG_REPEAT_FIELD: i32 = 64i32;
pub const AM_VIDEO_FLAG_WEAVE: i32 = 8i32;
#[repr(transparent)]
pub struct AM_WST_DRAWBGMODE(pub i32);
pub const AM_WST_DRAWBGMODE_Opaque: AM_WST_DRAWBGMODE = AM_WST_DRAWBGMODE(0i32);
pub const AM_WST_DRAWBGMODE_Transparent: AM_WST_DRAWBGMODE = AM_WST_DRAWBGMODE(1i32);
#[repr(transparent)]
pub struct AM_WST_LEVEL(pub i32);
pub const AM_WST_LEVEL_1_5: AM_WST_LEVEL = AM_WST_LEVEL(0i32);
#[repr(C)]
pub struct AM_WST_PAGE(i32);
#[repr(transparent)]
pub struct AM_WST_SERVICE(pub i32);
pub const AM_WST_SERVICE_None: AM_WST_SERVICE = AM_WST_SERVICE(0i32);
pub const AM_WST_SERVICE_Text: AM_WST_SERVICE = AM_WST_SERVICE(1i32);
pub const AM_WST_SERVICE_IDS: AM_WST_SERVICE = AM_WST_SERVICE(2i32);
pub const AM_WST_SERVICE_Invalid: AM_WST_SERVICE = AM_WST_SERVICE(3i32);
#[repr(transparent)]
pub struct AM_WST_STATE(pub i32);
pub const AM_WST_STATE_Off: AM_WST_STATE = AM_WST_STATE(0i32);
pub const AM_WST_STATE_On: AM_WST_STATE = AM_WST_STATE(1i32);
#[repr(transparent)]
pub struct AM_WST_STYLE(pub i32);
pub const AM_WST_STYLE_None: AM_WST_STYLE = AM_WST_STYLE(0i32);
pub const AM_WST_STYLE_Invers: AM_WST_STYLE = AM_WST_STYLE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ANALOGVIDEOINFO(i32);
#[repr(C)]
pub struct ANALOG_AUXIN_NETWORK_TYPE(i32);
#[repr(C)]
pub struct ANALOG_FM_NETWORK_TYPE(i32);
#[repr(C)]
pub struct ANALOG_TV_NETWORK_TYPE(i32);
#[repr(C)]
pub struct ATSCChannelTuneRequest(i32);
#[repr(C)]
pub struct ATSCComponentType(i32);
#[repr(transparent)]
pub struct ATSCComponentTypeFlags(pub i32);
pub const ATSCCT_AC3: ATSCComponentTypeFlags = ATSCComponentTypeFlags(1i32);
#[repr(C)]
pub struct ATSCLocator(i32);
#[repr(C)]
pub struct ATSCTuningSpace(i32);
pub const ATSC_EIT_TID: u32 = 203u32;
pub const ATSC_ETM_LOCATION_IN_PTC_FOR_EVENT: u32 = 2u32;
pub const ATSC_ETM_LOCATION_IN_PTC_FOR_PSIP: u32 = 1u32;
pub const ATSC_ETM_LOCATION_NOT_PRESENT: u32 = 0u32;
pub const ATSC_ETM_LOCATION_RESERVED: u32 = 3u32;
pub const ATSC_ETT_TID: u32 = 204u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ATSC_FILTER_OPTIONS(i32);
pub const ATSC_MGT_PID: u32 = 8187u32;
pub const ATSC_MGT_TID: u32 = 199u32;
pub const ATSC_PIT_TID: u32 = 208u32;
pub const ATSC_RRT_PID: u32 = 8187u32;
pub const ATSC_RRT_TID: u32 = 202u32;
pub const ATSC_STT_PID: u32 = 8187u32;
pub const ATSC_STT_TID: u32 = 205u32;
#[repr(C)]
pub struct ATSC_TERRESTRIAL_TV_NETWORK_TYPE(i32);
pub const ATSC_VCT_CABL_TID: u32 = 201u32;
pub const ATSC_VCT_PID: u32 = 8187u32;
pub const ATSC_VCT_TERR_TID: u32 = 200u32;
#[repr(C)]
pub struct AUDIO_STREAM_CONFIG_CAPS(i32);
#[repr(C)]
pub struct AVIEXTHEADER(i32);
#[repr(C)]
pub struct AVIFIELDINDEX(i32);
pub const AVIF_COPYRIGHTED: u32 = 131072u32;
pub const AVIF_HASINDEX: u32 = 16u32;
pub const AVIF_ISINTERLEAVED: u32 = 256u32;
pub const AVIF_MUSTUSEINDEX: u32 = 32u32;
pub const AVIF_TRUSTCKTYPE: u32 = 2048u32;
pub const AVIF_WASCAPTUREFILE: u32 = 65536u32;
pub const AVIIF_COMPRESSOR: u32 = 268369920u32;
pub const AVIIF_COMPUSE: i32 = 268369920i32;
pub const AVIIF_FIRSTPART: i32 = 32i32;
pub const AVIIF_KEYFRAME: i32 = 16i32;
pub const AVIIF_LASTPART: i32 = 64i32;
pub const AVIIF_LIST: i32 = 1i32;
pub const AVIIF_NOTIME: i32 = 256i32;
pub const AVIIF_NO_TIME: u32 = 256u32;
#[repr(C)]
pub struct AVIINDEXENTRY(i32);
#[repr(C)]
pub struct AVIMAINHEADER(i32);
#[repr(C)]
pub struct AVIMETAINDEX(i32);
#[repr(C)]
pub struct AVIOLDINDEX(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct AVIPALCHANGE(i32);
pub const AVISF_DISABLED: u32 = 1u32;
pub const AVISF_VIDEO_PALCHANGES: u32 = 65536u32;
#[repr(C)]
pub struct AVISTDINDEX(i32);
pub const AVISTDINDEX_DELTAFRAME: u32 = 2147483648u32;
#[repr(C)]
pub struct AVISTDINDEX_ENTRY(i32);
#[repr(C)]
pub struct AVISTREAMHEADER(i32);
#[repr(C)]
pub struct AVISUPERINDEX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AVIStreamHeader(i32);
#[repr(C)]
pub struct AVITCDLINDEX_ENTRY(i32);
#[repr(C)]
pub struct AVITIMECODEINDEX(i32);
#[repr(C)]
pub struct AVITIMEDINDEX_ENTRY(i32);
pub const AVI_HEADERSIZE: u32 = 2048u32;
pub const AVI_INDEX_IS_DATA: u32 = 128u32;
pub const AVI_INDEX_OF_CHUNKS: u32 = 1u32;
pub const AVI_INDEX_OF_INDEXES: u32 = 0u32;
pub const AVI_INDEX_OF_SUB_2FIELD: u32 = 3u32;
pub const AVI_INDEX_OF_TIMED_CHUNKS: u32 = 2u32;
pub const AVI_INDEX_SUB_2FIELD: u32 = 1u32;
pub const AVI_INDEX_SUB_DEFAULT: u32 = 0u32;
#[repr(C)]
pub struct AnalogAudioComponentType(i32);
#[repr(C)]
pub struct AnalogLocator(i32);
#[repr(C)]
pub struct AnalogRadioTuningSpace(i32);
#[repr(C)]
pub struct AnalogTVTuningSpace(i32);
#[repr(transparent)]
pub struct AnalogVideoStandard(pub i32);
pub const AnalogVideo_None: AnalogVideoStandard = AnalogVideoStandard(0i32);
pub const AnalogVideo_NTSC_M: AnalogVideoStandard = AnalogVideoStandard(1i32);
pub const AnalogVideo_NTSC_M_J: AnalogVideoStandard = AnalogVideoStandard(2i32);
pub const AnalogVideo_NTSC_433: AnalogVideoStandard = AnalogVideoStandard(4i32);
pub const AnalogVideo_PAL_B: AnalogVideoStandard = AnalogVideoStandard(16i32);
pub const AnalogVideo_PAL_D: AnalogVideoStandard = AnalogVideoStandard(32i32);
pub const AnalogVideo_PAL_G: AnalogVideoStandard = AnalogVideoStandard(64i32);
pub const AnalogVideo_PAL_H: AnalogVideoStandard = AnalogVideoStandard(128i32);
pub const AnalogVideo_PAL_I: AnalogVideoStandard = AnalogVideoStandard(256i32);
pub const AnalogVideo_PAL_M: AnalogVideoStandard = AnalogVideoStandard(512i32);
pub const AnalogVideo_PAL_N: AnalogVideoStandard = AnalogVideoStandard(1024i32);
pub const AnalogVideo_PAL_60: AnalogVideoStandard = AnalogVideoStandard(2048i32);
pub const AnalogVideo_SECAM_B: AnalogVideoStandard = AnalogVideoStandard(4096i32);
pub const AnalogVideo_SECAM_D: AnalogVideoStandard = AnalogVideoStandard(8192i32);
pub const AnalogVideo_SECAM_G: AnalogVideoStandard = AnalogVideoStandard(16384i32);
pub const AnalogVideo_SECAM_H: AnalogVideoStandard = AnalogVideoStandard(32768i32);
pub const AnalogVideo_SECAM_K: AnalogVideoStandard = AnalogVideoStandard(65536i32);
pub const AnalogVideo_SECAM_K1: AnalogVideoStandard = AnalogVideoStandard(131072i32);
pub const AnalogVideo_SECAM_L: AnalogVideoStandard = AnalogVideoStandard(262144i32);
pub const AnalogVideo_SECAM_L1: AnalogVideoStandard = AnalogVideoStandard(524288i32);
pub const AnalogVideo_PAL_N_COMBO: AnalogVideoStandard = AnalogVideoStandard(1048576i32);
pub const AnalogVideoMask_MCE_NTSC: AnalogVideoStandard = AnalogVideoStandard(1052167i32);
pub const AnalogVideoMask_MCE_PAL: AnalogVideoStandard = AnalogVideoStandard(496i32);
pub const AnalogVideoMask_MCE_SECAM: AnalogVideoStandard = AnalogVideoStandard(1044480i32);
pub const AnalogVideo_NTSC_Mask: u32 = 7u32;
pub const AnalogVideo_PAL_Mask: u32 = 1052656u32;
pub const AnalogVideo_SECAM_Mask: u32 = 1044480u32;
#[repr(transparent)]
pub struct ApplicationTypeType(pub i32);
pub const SCTE28_ConditionalAccess: ApplicationTypeType = ApplicationTypeType(0i32);
pub const SCTE28_POD_Host_Binding_Information: ApplicationTypeType = ApplicationTypeType(1i32);
pub const SCTE28_IPService: ApplicationTypeType = ApplicationTypeType(2i32);
pub const SCTE28_NetworkInterface_SCTE55_2: ApplicationTypeType = ApplicationTypeType(3i32);
pub const SCTE28_NetworkInterface_SCTE55_1: ApplicationTypeType = ApplicationTypeType(4i32);
pub const SCTE28_CopyProtection: ApplicationTypeType = ApplicationTypeType(5i32);
pub const SCTE28_Diagnostic: ApplicationTypeType = ApplicationTypeType(6i32);
pub const SCTE28_Undesignated: ApplicationTypeType = ApplicationTypeType(7i32);
pub const SCTE28_Reserved: ApplicationTypeType = ApplicationTypeType(8i32);
pub const AudioType_Commentary: u32 = 5u32;
pub const AudioType_Dialogue: u32 = 4u32;
pub const AudioType_Emergency: u32 = 6u32;
pub const AudioType_Hearing_Impaired: u32 = 3u32;
pub const AudioType_Music_And_Effects: u32 = 1u32;
pub const AudioType_Reserved: i32 = -1i32;
pub const AudioType_Standard: u32 = 0u32;
pub const AudioType_Visually_Impaired: u32 = 2u32;
pub const AudioType_Voiceover: u32 = 7u32;
#[repr(C)]
pub struct AuxInTuningSpace(i32);
#[repr(C)]
pub struct BDANETWORKTYPE_ATSC(i32);
#[repr(C)]
pub struct BDANODE_DESCRIPTOR(i32);
#[repr(C)]
pub struct BDA_BUFFER(i32);
#[repr(C)]
pub struct BDA_CAS_CHECK_ENTITLEMENTTOKEN(i32);
#[repr(C)]
pub struct BDA_CAS_CLOSEMMIDATA(i32);
#[repr(C)]
pub struct BDA_CAS_CLOSE_MMIDIALOG(i32);
#[repr(C)]
pub struct BDA_CAS_OPENMMIDATA(i32);
#[repr(C)]
pub struct BDA_CAS_REQUESTTUNERDATA(i32);
#[repr(C)]
pub struct BDA_CA_MODULE_UI(i32);
#[repr(transparent)]
pub struct BDA_CHANGE_STATE(pub i32);
pub const BDA_CHANGES_COMPLETE: BDA_CHANGE_STATE = BDA_CHANGE_STATE(0i32);
pub const BDA_CHANGES_PENDING: BDA_CHANGE_STATE = BDA_CHANGE_STATE(1i32);
#[repr(transparent)]
pub struct BDA_CONDITIONALACCESS_MMICLOSEREASON(pub i32);
pub const CONDITIONALACCESS_UNSPECIFIED: BDA_CONDITIONALACCESS_MMICLOSEREASON = BDA_CONDITIONALACCESS_MMICLOSEREASON(0i32);
pub const CONDITIONALACCESS_CLOSED_ITSELF: BDA_CONDITIONALACCESS_MMICLOSEREASON = BDA_CONDITIONALACCESS_MMICLOSEREASON(1i32);
pub const CONDITIONALACCESS_TUNER_REQUESTED_CLOSE: BDA_CONDITIONALACCESS_MMICLOSEREASON = BDA_CONDITIONALACCESS_MMICLOSEREASON(2i32);
pub const CONDITIONALACCESS_DIALOG_TIMEOUT: BDA_CONDITIONALACCESS_MMICLOSEREASON = BDA_CONDITIONALACCESS_MMICLOSEREASON(3i32);
pub const CONDITIONALACCESS_DIALOG_FOCUS_CHANGE: BDA_CONDITIONALACCESS_MMICLOSEREASON = BDA_CONDITIONALACCESS_MMICLOSEREASON(4i32);
pub const CONDITIONALACCESS_DIALOG_USER_DISMISSED: BDA_CONDITIONALACCESS_MMICLOSEREASON = BDA_CONDITIONALACCESS_MMICLOSEREASON(5i32);
pub const CONDITIONALACCESS_DIALOG_USER_NOT_AVAILABLE: BDA_CONDITIONALACCESS_MMICLOSEREASON = BDA_CONDITIONALACCESS_MMICLOSEREASON(6i32);
#[repr(transparent)]
pub struct BDA_CONDITIONALACCESS_REQUESTTYPE(pub i32);
pub const CONDITIONALACCESS_ACCESS_UNSPECIFIED: BDA_CONDITIONALACCESS_REQUESTTYPE = BDA_CONDITIONALACCESS_REQUESTTYPE(0i32);
pub const CONDITIONALACCESS_ACCESS_NOT_POSSIBLE: BDA_CONDITIONALACCESS_REQUESTTYPE = BDA_CONDITIONALACCESS_REQUESTTYPE(1i32);
pub const CONDITIONALACCESS_ACCESS_POSSIBLE: BDA_CONDITIONALACCESS_REQUESTTYPE = BDA_CONDITIONALACCESS_REQUESTTYPE(2i32);
pub const CONDITIONALACCESS_ACCESS_POSSIBLE_NO_STREAMING_DISRUPTION: BDA_CONDITIONALACCESS_REQUESTTYPE = BDA_CONDITIONALACCESS_REQUESTTYPE(3i32);
#[repr(transparent)]
pub struct BDA_CONDITIONALACCESS_SESSION_RESULT(pub i32);
pub const CONDITIONALACCESS_SUCCESSFULL: BDA_CONDITIONALACCESS_SESSION_RESULT = BDA_CONDITIONALACCESS_SESSION_RESULT(0i32);
pub const CONDITIONALACCESS_ENDED_NOCHANGE: BDA_CONDITIONALACCESS_SESSION_RESULT = BDA_CONDITIONALACCESS_SESSION_RESULT(1i32);
pub const CONDITIONALACCESS_ABORTED: BDA_CONDITIONALACCESS_SESSION_RESULT = BDA_CONDITIONALACCESS_SESSION_RESULT(2i32);
#[repr(transparent)]
pub struct BDA_Channel(pub i32);
pub const BDA_UNDEFINED_CHANNEL: BDA_Channel = BDA_Channel(-1i32);
#[repr(transparent)]
pub struct BDA_Channel_Bandwidth(pub i32);
pub const BDA_CHAN_BANDWITH_NOT_SET: BDA_Channel_Bandwidth = BDA_Channel_Bandwidth(-1i32);
pub const BDA_CHAN_BANDWITH_NOT_DEFINED: BDA_Channel_Bandwidth = BDA_Channel_Bandwidth(0i32);
#[repr(transparent)]
pub struct BDA_Comp_Flags(pub i32);
pub const BDACOMP_NOT_DEFINED: BDA_Comp_Flags = BDA_Comp_Flags(0i32);
pub const BDACOMP_EXCLUDE_TS_FROM_TR: BDA_Comp_Flags = BDA_Comp_Flags(1i32);
pub const BDACOMP_INCLUDE_LOCATOR_IN_TR: BDA_Comp_Flags = BDA_Comp_Flags(2i32);
pub const BDACOMP_INCLUDE_COMPONENTS_IN_TR: BDA_Comp_Flags = BDA_Comp_Flags(4i32);
#[repr(C)]
pub struct BDA_DEBUG_DATA(i32);
#[repr(C)]
pub struct BDA_DEBUG_DATA_AVAILABLE(i32);
#[repr(C)]
pub struct BDA_DEBUG_DATA_TYPE_STRING(i32);
#[repr(transparent)]
pub struct BDA_DISCOVERY_STATE(pub i32);
pub const BDA_DISCOVERY_UNSPECIFIED: BDA_DISCOVERY_STATE = BDA_DISCOVERY_STATE(0i32);
pub const BDA_DISCOVERY_REQUIRED: BDA_DISCOVERY_STATE = BDA_DISCOVERY_STATE(1i32);
pub const BDA_DISCOVERY_COMPLETE: BDA_DISCOVERY_STATE = BDA_DISCOVERY_STATE(2i32);
#[repr(C)]
pub struct BDA_DISEQC_RESPONSE(i32);
#[repr(C)]
pub struct BDA_DISEQC_SEND(i32);
#[repr(C)]
pub struct BDA_DRM_DRMSTATUS(i32);
#[repr(C)]
pub struct BDA_DVBT2_L1_SIGNALLING_DATA(i32);
#[repr(transparent)]
pub struct BDA_DigitalSignalStandard(pub i32);
pub const Bda_DigitalStandard_None: BDA_DigitalSignalStandard = BDA_DigitalSignalStandard(0i32);
pub const Bda_DigitalStandard_DVB_T: BDA_DigitalSignalStandard = BDA_DigitalSignalStandard(1i32);
pub const Bda_DigitalStandard_DVB_S: BDA_DigitalSignalStandard = BDA_DigitalSignalStandard(2i32);
pub const Bda_DigitalStandard_DVB_C: BDA_DigitalSignalStandard = BDA_DigitalSignalStandard(4i32);
pub const Bda_DigitalStandard_ATSC: BDA_DigitalSignalStandard = BDA_DigitalSignalStandard(8i32);
pub const Bda_DigitalStandard_ISDB_T: BDA_DigitalSignalStandard = BDA_DigitalSignalStandard(16i32);
pub const Bda_DigitalStandard_ISDB_S: BDA_DigitalSignalStandard = BDA_DigitalSignalStandard(32i32);
pub const Bda_DigitalStandard_ISDB_C: BDA_DigitalSignalStandard = BDA_DigitalSignalStandard(64i32);
#[repr(transparent)]
pub struct BDA_DrmPairingError(pub i32);
pub const BDA_DrmPairing_Succeeded: BDA_DrmPairingError = BDA_DrmPairingError(0i32);
pub const BDA_DrmPairing_HardwareFailure: BDA_DrmPairingError = BDA_DrmPairingError(1i32);
pub const BDA_DrmPairing_NeedRevocationData: BDA_DrmPairingError = BDA_DrmPairingError(2i32);
pub const BDA_DrmPairing_NeedIndiv: BDA_DrmPairingError = BDA_DrmPairingError(3i32);
pub const BDA_DrmPairing_Other: BDA_DrmPairingError = BDA_DrmPairingError(4i32);
pub const BDA_DrmPairing_DrmInitFailed: BDA_DrmPairingError = BDA_DrmPairingError(5i32);
pub const BDA_DrmPairing_DrmNotPaired: BDA_DrmPairingError = BDA_DrmPairingError(6i32);
pub const BDA_DrmPairing_DrmRePairSoon: BDA_DrmPairingError = BDA_DrmPairingError(7i32);
pub const BDA_DrmPairing_Aborted: BDA_DrmPairingError = BDA_DrmPairingError(8i32);
pub const BDA_DrmPairing_NeedSDKUpdate: BDA_DrmPairingError = BDA_DrmPairingError(9i32);
#[repr(C)]
pub struct BDA_ETHERNET_ADDRESS(i32);
#[repr(C)]
pub struct BDA_ETHERNET_ADDRESS_LIST(i32);
#[repr(C)]
pub struct BDA_EVENT_DATA(i32);
#[repr(transparent)]
pub struct BDA_EVENT_ID(pub i32);
pub const BDA_EVENT_SIGNAL_LOSS: BDA_EVENT_ID = BDA_EVENT_ID(0i32);
pub const BDA_EVENT_SIGNAL_LOCK: BDA_EVENT_ID = BDA_EVENT_ID(1i32);
pub const BDA_EVENT_DATA_START: BDA_EVENT_ID = BDA_EVENT_ID(2i32);
pub const BDA_EVENT_DATA_STOP: BDA_EVENT_ID = BDA_EVENT_ID(3i32);
pub const BDA_EVENT_CHANNEL_ACQUIRED: BDA_EVENT_ID = BDA_EVENT_ID(4i32);
pub const BDA_EVENT_CHANNEL_LOST: BDA_EVENT_ID = BDA_EVENT_ID(5i32);
pub const BDA_EVENT_CHANNEL_SOURCE_CHANGED: BDA_EVENT_ID = BDA_EVENT_ID(6i32);
pub const BDA_EVENT_CHANNEL_ACTIVATED: BDA_EVENT_ID = BDA_EVENT_ID(7i32);
pub const BDA_EVENT_CHANNEL_DEACTIVATED: BDA_EVENT_ID = BDA_EVENT_ID(8i32);
pub const BDA_EVENT_SUBCHANNEL_ACQUIRED: BDA_EVENT_ID = BDA_EVENT_ID(9i32);
pub const BDA_EVENT_SUBCHANNEL_LOST: BDA_EVENT_ID = BDA_EVENT_ID(10i32);
pub const BDA_EVENT_SUBCHANNEL_SOURCE_CHANGED: BDA_EVENT_ID = BDA_EVENT_ID(11i32);
pub const BDA_EVENT_SUBCHANNEL_ACTIVATED: BDA_EVENT_ID = BDA_EVENT_ID(12i32);
pub const BDA_EVENT_SUBCHANNEL_DEACTIVATED: BDA_EVENT_ID = BDA_EVENT_ID(13i32);
pub const BDA_EVENT_ACCESS_GRANTED: BDA_EVENT_ID = BDA_EVENT_ID(14i32);
pub const BDA_EVENT_ACCESS_DENIED: BDA_EVENT_ID = BDA_EVENT_ID(15i32);
pub const BDA_EVENT_OFFER_EXTENDED: BDA_EVENT_ID = BDA_EVENT_ID(16i32);
pub const BDA_EVENT_PURCHASE_COMPLETED: BDA_EVENT_ID = BDA_EVENT_ID(17i32);
pub const BDA_EVENT_SMART_CARD_INSERTED: BDA_EVENT_ID = BDA_EVENT_ID(18i32);
pub const BDA_EVENT_SMART_CARD_REMOVED: BDA_EVENT_ID = BDA_EVENT_ID(19i32);
pub const BDA_E_ACCESS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479671i32 as _);
pub const BDA_E_BUFFER_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479669i32 as _);
pub const BDA_E_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479666i32 as _);
pub const BDA_E_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479679i32 as _);
pub const BDA_E_INVALID_CAPTURE_TOKEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073463294i32 as _);
pub const BDA_E_INVALID_ENTITLEMENT_TOKEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073463295i32 as _);
pub const BDA_E_INVALID_HANDLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479674i32 as _);
pub const BDA_E_INVALID_LANGUAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479664i32 as _);
pub const BDA_E_INVALID_PURCHASE_TOKEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073463292i32 as _);
pub const BDA_E_INVALID_SCHEMA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479675i32 as _);
pub const BDA_E_INVALID_TUNE_REQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073467388i32 as _);
pub const BDA_E_INVALID_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479673i32 as _);
pub const BDA_E_IPNETWORK_ADDRESS_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073459198i32 as _);
pub const BDA_E_IPNETWORK_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073459199i32 as _);
pub const BDA_E_IPNETWORK_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073459197i32 as _);
pub const BDA_E_IPNETWORK_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073459196i32 as _);
pub const BDA_E_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479670i32 as _);
pub const BDA_E_NOT_IMPLEMENTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479678i32 as _);
pub const BDA_E_NO_HANDLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479665i32 as _);
pub const BDA_E_NO_MORE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073475582i32 as _);
pub const BDA_E_NO_MORE_EVENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073475583i32 as _);
pub const BDA_E_NO_SUCH_COMMAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479677i32 as _);
pub const BDA_E_OUT_OF_BOUNDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479676i32 as _);
pub const BDA_E_OUT_OF_MEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479667i32 as _);
pub const BDA_E_OUT_OF_RESOURCES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479668i32 as _);
pub const BDA_E_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479672i32 as _);
pub const BDA_E_TIMEOUT_ELAPSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073479663i32 as _);
pub const BDA_E_TUNER_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073467389i32 as _);
pub const BDA_E_TUNER_INITIALIZING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073467391i32 as _);
pub const BDA_E_TUNER_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073467390i32 as _);
pub const BDA_E_TUNE_FAILED_SDV01: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073455103i32 as _);
pub const BDA_E_TUNE_FAILED_SDV02: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073455102i32 as _);
pub const BDA_E_TUNE_FAILED_SDV03: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073455101i32 as _);
pub const BDA_E_TUNE_FAILED_SDV04: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073455100i32 as _);
pub const BDA_E_TUNE_FAILED_SDV05: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073455099i32 as _);
pub const BDA_E_TUNE_FAILED_SDV06: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073455098i32 as _);
pub const BDA_E_TUNE_FAILED_SDV07: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073455097i32 as _);
pub const BDA_E_TUNE_FAILED_SDV08: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073455096i32 as _);
pub const BDA_E_TUNE_FAILED_SDVFF: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073454849i32 as _);
pub const BDA_E_WMDRM_INVALID_CERTIFICATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418238i32 as _);
pub const BDA_E_WMDRM_INVALID_DATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418235i32 as _);
pub const BDA_E_WMDRM_INVALID_PROXIMITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418234i32 as _);
pub const BDA_E_WMDRM_INVALID_SIGNATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418239i32 as _);
pub const BDA_E_WMDRM_INVALID_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418236i32 as _);
pub const BDA_E_WMDRM_KEY_ID_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073418232i32 as _);
pub const BDA_E_WOULD_DISRUPT_STREAMING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1073463293i32 as _);
#[repr(transparent)]
pub struct BDA_Frequency(pub i32);
pub const BDA_FREQUENCY_NOT_SET: BDA_Frequency = BDA_Frequency(-1i32);
pub const BDA_FREQUENCY_NOT_DEFINED: BDA_Frequency = BDA_Frequency(0i32);
#[repr(transparent)]
pub struct BDA_Frequency_Multiplier(pub i32);
pub const BDA_FREQUENCY_MULTIPLIER_NOT_SET: BDA_Frequency_Multiplier = BDA_Frequency_Multiplier(-1i32);
pub const BDA_FREQUENCY_MULTIPLIER_NOT_DEFINED: BDA_Frequency_Multiplier = BDA_Frequency_Multiplier(0i32);
#[repr(C)]
pub struct BDA_GDDS_DATA(i32);
#[repr(C)]
pub struct BDA_GDDS_DATATYPE(i32);
#[repr(C)]
pub struct BDA_IPv4_ADDRESS(i32);
#[repr(C)]
pub struct BDA_IPv4_ADDRESS_LIST(i32);
#[repr(C)]
pub struct BDA_IPv6_ADDRESS(i32);
#[repr(C)]
pub struct BDA_IPv6_ADDRESS_LIST(i32);
#[repr(C)]
pub struct BDA_ISDBCAS_EMG_REQ(i32);
#[repr(C)]
pub struct BDA_ISDBCAS_REQUESTHEADER(i32);
#[repr(C)]
pub struct BDA_ISDBCAS_RESPONSEDATA(i32);
#[repr(transparent)]
pub struct BDA_LockType(pub i32);
pub const Bda_LockType_None: BDA_LockType = BDA_LockType(0i32);
pub const Bda_LockType_PLL: BDA_LockType = BDA_LockType(1i32);
pub const Bda_LockType_DecoderDemod: BDA_LockType = BDA_LockType(2i32);
pub const Bda_LockType_Complete: BDA_LockType = BDA_LockType(128i32);
#[repr(transparent)]
pub struct BDA_MULTICAST_MODE(pub i32);
pub const BDA_PROMISCUOUS_MULTICAST: BDA_MULTICAST_MODE = BDA_MULTICAST_MODE(0i32);
pub const BDA_FILTERED_MULTICAST: BDA_MULTICAST_MODE = BDA_MULTICAST_MODE(1i32);
pub const BDA_NO_MULTICAST: BDA_MULTICAST_MODE = BDA_MULTICAST_MODE(2i32);
#[repr(C)]
pub struct BDA_MUX_PIDLISTITEM(i32);
#[repr(C)]
pub struct BDA_PID_MAP(i32);
#[repr(C)]
pub struct BDA_PID_UNMAP(i32);
pub const BDA_PLP_ID_NOT_SET: i32 = -1i32;
#[repr(C)]
pub struct BDA_PROGRAM_PID_LIST(i32);
#[repr(C)]
pub struct BDA_RATING_PINRESET(i32);
#[repr(transparent)]
pub struct BDA_Range(pub i32);
pub const BDA_RANGE_NOT_SET: BDA_Range = BDA_Range(-1i32);
pub const BDA_RANGE_NOT_DEFINED: BDA_Range = BDA_Range(0i32);
#[repr(C)]
pub struct BDA_SCAN_CAPABILTIES(i32);
#[repr(C)]
pub struct BDA_SCAN_START(i32);
#[repr(C)]
pub struct BDA_SCAN_STATE(i32);
#[repr(transparent)]
pub struct BDA_SIGNAL_STATE(pub i32);
pub const BDA_SIGNAL_UNAVAILABLE: BDA_SIGNAL_STATE = BDA_SIGNAL_STATE(0i32);
pub const BDA_SIGNAL_INACTIVE: BDA_SIGNAL_STATE = BDA_SIGNAL_STATE(1i32);
pub const BDA_SIGNAL_ACTIVE: BDA_SIGNAL_STATE = BDA_SIGNAL_STATE(2i32);
#[repr(C)]
pub struct BDA_SIGNAL_TIMEOUTS(i32);
#[repr(C)]
pub struct BDA_STRING(i32);
#[repr(transparent)]
pub struct BDA_SignalType(pub i32);
pub const Bda_SignalType_Unknown: BDA_SignalType = BDA_SignalType(0i32);
pub const Bda_SignalType_Analog: BDA_SignalType = BDA_SignalType(1i32);
pub const Bda_SignalType_Digital: BDA_SignalType = BDA_SignalType(2i32);
#[repr(C)]
pub struct BDA_TABLE_SECTION(i32);
#[repr(C)]
pub struct BDA_TEMPLATE_CONNECTION(i32);
#[repr(C)]
pub struct BDA_TEMPLATE_PIN_JOINT(i32);
#[repr(C)]
pub struct BDA_TRANSPORT_INFO(i32);
#[repr(C)]
pub struct BDA_TS_SELECTORINFO(i32);
#[repr(C)]
pub struct BDA_TS_SELECTORINFO_ISDBS_EXT(i32);
#[repr(C)]
pub struct BDA_TUNER_DIAGNOSTICS(i32);
#[repr(C)]
pub struct BDA_TUNER_TUNERSTATE(i32);
#[repr(C)]
pub struct BDA_USERACTIVITY_INTERVAL(i32);
#[repr(C)]
pub struct BDA_WMDRMTUNER_PIDPROTECTION(i32);
#[repr(C)]
pub struct BDA_WMDRMTUNER_PURCHASEENTITLEMENT(i32);
#[repr(C)]
pub struct BDA_WMDRM_KEYINFOLIST(i32);
#[repr(C)]
pub struct BDA_WMDRM_RENEWLICENSE(i32);
#[repr(C)]
pub struct BDA_WMDRM_STATUS(i32);
#[repr(C)]
pub struct BSKYB_TERRESTRIAL_TV_NETWORK_TYPE(i32);
#[repr(C)]
pub struct BadSampleInfo(i32);
#[repr(transparent)]
pub struct BfEnTvRat_Attributes_CAE_TV(pub i32);
pub const CAE_IsBlocked: BfEnTvRat_Attributes_CAE_TV = BfEnTvRat_Attributes_CAE_TV(1i32);
pub const CAE_ValidAttrSubmask: BfEnTvRat_Attributes_CAE_TV = BfEnTvRat_Attributes_CAE_TV(1i32);
#[repr(transparent)]
pub struct BfEnTvRat_Attributes_CAF_TV(pub i32);
pub const CAF_IsBlocked: BfEnTvRat_Attributes_CAF_TV = BfEnTvRat_Attributes_CAF_TV(1i32);
pub const CAF_ValidAttrSubmask: BfEnTvRat_Attributes_CAF_TV = BfEnTvRat_Attributes_CAF_TV(1i32);
#[repr(transparent)]
pub struct BfEnTvRat_Attributes_MPAA(pub i32);
pub const MPAA_IsBlocked: BfEnTvRat_Attributes_MPAA = BfEnTvRat_Attributes_MPAA(1i32);
pub const MPAA_ValidAttrSubmask: BfEnTvRat_Attributes_MPAA = BfEnTvRat_Attributes_MPAA(1i32);
#[repr(transparent)]
pub struct BfEnTvRat_Attributes_US_TV(pub i32);
pub const US_TV_IsBlocked: BfEnTvRat_Attributes_US_TV = BfEnTvRat_Attributes_US_TV(1i32);
pub const US_TV_IsViolent: BfEnTvRat_Attributes_US_TV = BfEnTvRat_Attributes_US_TV(2i32);
pub const US_TV_IsSexualSituation: BfEnTvRat_Attributes_US_TV = BfEnTvRat_Attributes_US_TV(4i32);
pub const US_TV_IsAdultLanguage: BfEnTvRat_Attributes_US_TV = BfEnTvRat_Attributes_US_TV(8i32);
pub const US_TV_IsSexuallySuggestiveDialog: BfEnTvRat_Attributes_US_TV = BfEnTvRat_Attributes_US_TV(16i32);
pub const US_TV_ValidAttrSubmask: BfEnTvRat_Attributes_US_TV = BfEnTvRat_Attributes_US_TV(31i32);
#[repr(transparent)]
pub struct BfEnTvRat_GenericAttributes(pub i32);
pub const BfAttrNone: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(0i32);
pub const BfIsBlocked: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(1i32);
pub const BfIsAttr_1: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(2i32);
pub const BfIsAttr_2: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(4i32);
pub const BfIsAttr_3: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(8i32);
pub const BfIsAttr_4: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(16i32);
pub const BfIsAttr_5: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(32i32);
pub const BfIsAttr_6: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(64i32);
pub const BfIsAttr_7: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(128i32);
pub const BfValidAttrSubmask: BfEnTvRat_GenericAttributes = BfEnTvRat_GenericAttributes(255i32);
#[repr(transparent)]
pub struct BinaryConvolutionCodeRate(pub i32);
pub const BDA_BCC_RATE_NOT_SET: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(-1i32);
pub const BDA_BCC_RATE_NOT_DEFINED: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(0i32);
pub const BDA_BCC_RATE_1_2: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(1i32);
pub const BDA_BCC_RATE_2_3: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(2i32);
pub const BDA_BCC_RATE_3_4: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(3i32);
pub const BDA_BCC_RATE_3_5: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(4i32);
pub const BDA_BCC_RATE_4_5: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(5i32);
pub const BDA_BCC_RATE_5_6: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(6i32);
pub const BDA_BCC_RATE_5_11: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(7i32);
pub const BDA_BCC_RATE_7_8: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(8i32);
pub const BDA_BCC_RATE_1_4: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(9i32);
pub const BDA_BCC_RATE_1_3: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(10i32);
pub const BDA_BCC_RATE_2_5: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(11i32);
pub const BDA_BCC_RATE_6_7: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(12i32);
pub const BDA_BCC_RATE_8_9: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(13i32);
pub const BDA_BCC_RATE_9_10: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(14i32);
pub const BDA_BCC_RATE_MAX: BinaryConvolutionCodeRate = BinaryConvolutionCodeRate(15i32);
#[repr(C)]
pub struct BroadcastEventService(i32);
#[repr(C)]
pub struct CAPTURE_STREAMTIME(i32);
pub const CDEF_BYPASS_CLASS_MANAGER: u32 = 2u32;
pub const CDEF_CLASS_DEFAULT: u32 = 1u32;
pub const CDEF_DEVMON_CMGR_DEVICE: u32 = 16u32;
pub const CDEF_DEVMON_DMO: u32 = 32u32;
pub const CDEF_DEVMON_FILTER: u32 = 128u32;
pub const CDEF_DEVMON_PNP_DEVICE: u32 = 64u32;
pub const CDEF_DEVMON_SELECTIVE_MASK: u32 = 240u32;
pub const CDEF_MERIT_ABOVE_DO_NOT_USE: u32 = 8u32;
pub const CHARS_IN_GUID: u32 = 39u32;
pub const CLSID_AMAudioData: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4064707968, data2: 44938, data3: 4560, data4: [130, 18, 0, 192, 79, 195, 44, 69] };
pub const CLSID_AMAudioStream: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2224480320, data2: 44876, data3: 4560, data4: [130, 18, 0, 192, 79, 195, 44, 69] };
pub const CLSID_AMDirectDrawStream: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1237613796, data2: 39844, data3: 4560, data4: [130, 18, 0, 192, 79, 195, 44, 69] };
pub const CLSID_AMMediaTypeStream: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3473878908, data2: 63423, data3: 4560, data4: [144, 13, 0, 192, 79, 217, 24, 157] };
pub const CLSID_AMMultiMediaStream: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1237613797, data2: 39844, data3: 4560, data4: [130, 18, 0, 192, 79, 195, 44, 69] };
pub const CLSID_CPCAFiltersCategory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229820, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const CLSID_DMOFilterCategory: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3168106860,
    data2: 48466,
    data3: 19760,
    data4: [171, 118, 112, 249, 117, 184, 145, 153],
};
pub const CLSID_DMOWrapperFilter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2485743683,
    data2: 48514,
    data3: 19965,
    data4: [176, 222, 129, 119, 115, 156, 109, 32],
};
pub const CLSID_DTFilterEncProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229698, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const CLSID_DTFilterTagProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229714, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const CLSID_ETFilterEncProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229697, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const CLSID_ETFilterTagProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229713, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const CLSID_Mpeg2TableFilter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1965573617,
    data2: 30095,
    data3: 19587,
    data4: [160, 67, 66, 112, 197, 147, 48, 142],
};
pub const CLSID_PBDA_AUX_DATA_TYPE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4249183091,
    data2: 13091,
    data3: 16528,
    data4: [173, 202, 142, 212, 95, 85, 207, 16],
};
pub const CLSID_PBDA_Encoder_DATA_TYPE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1922029244,
    data2: 21830,
    data3: 18198,
    data4: [177, 3, 248, 153, 245, 161, 250, 104],
};
pub const CLSID_PBDA_FDC_DATA_TYPE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3889953184, data2: 8875, data3: 16455, data4: [142, 103, 239, 154, 213, 4, 231, 41] };
pub const CLSID_PBDA_GDDS_DATA_TYPE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3356233203, data2: 24658, data3: 19478, data4: [159, 86, 196, 76, 33, 247, 60, 69] };
pub const CLSID_PTFilter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2631079447,
    data2: 45827,
    data3: 20374,
    data4: [131, 48, 46, 177, 115, 234, 77, 198],
};
pub const CLSID_XDSCodecProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229699, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const CLSID_XDSCodecTagProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229715, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
#[repr(C)]
pub struct COLORKEY(i32);
#[repr(transparent)]
pub struct COLORKEY_TYPE(pub i32);
pub const CK_NOCOLORKEY: COLORKEY_TYPE = COLORKEY_TYPE(0i32);
pub const CK_INDEX: COLORKEY_TYPE = COLORKEY_TYPE(1i32);
pub const CK_RGB: COLORKEY_TYPE = COLORKEY_TYPE(2i32);
#[repr(transparent)]
pub struct COMPLETION_STATUS_FLAGS(pub i32);
pub const COMPSTAT_NOUPDATEOK: COMPLETION_STATUS_FLAGS = COMPLETION_STATUS_FLAGS(1i32);
pub const COMPSTAT_WAIT: COMPLETION_STATUS_FLAGS = COMPLETION_STATUS_FLAGS(2i32);
pub const COMPSTAT_ABORT: COMPLETION_STATUS_FLAGS = COMPLETION_STATUS_FLAGS(4i32);
pub const COMPONENT_TAG_CAPTION_MAX: u32 = 55u32;
pub const COMPONENT_TAG_CAPTION_MIN: u32 = 48u32;
pub const COMPONENT_TAG_SUPERIMPOSE_MAX: u32 = 63u32;
pub const COMPONENT_TAG_SUPERIMPOSE_MIN: u32 = 56u32;
#[repr(transparent)]
pub struct COPPEventBlockReason(pub i32);
pub const COPP_Unknown: COPPEventBlockReason = COPPEventBlockReason(-1i32);
pub const COPP_BadDriver: COPPEventBlockReason = COPPEventBlockReason(0i32);
pub const COPP_NoCardHDCPSupport: COPPEventBlockReason = COPPEventBlockReason(1i32);
pub const COPP_NoMonitorHDCPSupport: COPPEventBlockReason = COPPEventBlockReason(2i32);
pub const COPP_BadCertificate: COPPEventBlockReason = COPPEventBlockReason(3i32);
pub const COPP_InvalidBusProtection: COPPEventBlockReason = COPPEventBlockReason(4i32);
pub const COPP_AeroGlassOff: COPPEventBlockReason = COPPEventBlockReason(5i32);
pub const COPP_RogueApp: COPPEventBlockReason = COPPEventBlockReason(6i32);
pub const COPP_ForbiddenVideo: COPPEventBlockReason = COPPEventBlockReason(7i32);
pub const COPP_Activate: COPPEventBlockReason = COPPEventBlockReason(8i32);
pub const COPP_DigitalAudioUnprotected: COPPEventBlockReason = COPPEventBlockReason(9i32);
#[repr(transparent)]
pub struct COPP_ACP_Protection_Level(pub i32);
pub const COPP_ACP_Level0: COPP_ACP_Protection_Level = COPP_ACP_Protection_Level(0i32);
pub const COPP_ACP_LevelMin: COPP_ACP_Protection_Level = COPP_ACP_Protection_Level(0i32);
pub const COPP_ACP_Level1: COPP_ACP_Protection_Level = COPP_ACP_Protection_Level(1i32);
pub const COPP_ACP_Level2: COPP_ACP_Protection_Level = COPP_ACP_Protection_Level(2i32);
pub const COPP_ACP_Level3: COPP_ACP_Protection_Level = COPP_ACP_Protection_Level(3i32);
pub const COPP_ACP_LevelMax: COPP_ACP_Protection_Level = COPP_ACP_Protection_Level(3i32);
pub const COPP_ACP_ForceDWORD: COPP_ACP_Protection_Level = COPP_ACP_Protection_Level(2147483647i32);
#[repr(transparent)]
pub struct COPP_BusType(pub i32);
pub const COPP_BusType_Unknown: COPP_BusType = COPP_BusType(0i32);
pub const COPP_BusType_PCI: COPP_BusType = COPP_BusType(1i32);
pub const COPP_BusType_PCIX: COPP_BusType = COPP_BusType(2i32);
pub const COPP_BusType_PCIExpress: COPP_BusType = COPP_BusType(3i32);
pub const COPP_BusType_AGP: COPP_BusType = COPP_BusType(4i32);
pub const COPP_BusType_Integrated: COPP_BusType = COPP_BusType(-2147483648i32);
pub const COPP_BusType_ForceDWORD: COPP_BusType = COPP_BusType(2147483647i32);
#[repr(transparent)]
pub struct COPP_CGMSA_Protection_Level(pub i32);
pub const COPP_CGMSA_Disabled: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(0i32);
pub const COPP_CGMSA_LevelMin: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(0i32);
pub const COPP_CGMSA_CopyFreely: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(1i32);
pub const COPP_CGMSA_CopyNoMore: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(2i32);
pub const COPP_CGMSA_CopyOneGeneration: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(3i32);
pub const COPP_CGMSA_CopyNever: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(4i32);
pub const COPP_CGMSA_RedistributionControlRequired: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(8i32);
pub const COPP_CGMSA_LevelMax: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(12i32);
pub const COPP_CGMSA_ForceDWORD: COPP_CGMSA_Protection_Level = COPP_CGMSA_Protection_Level(2147483647i32);
#[repr(transparent)]
pub struct COPP_ConnectorType(pub i32);
pub const COPP_ConnectorType_Unknown: COPP_ConnectorType = COPP_ConnectorType(-1i32);
pub const COPP_ConnectorType_VGA: COPP_ConnectorType = COPP_ConnectorType(0i32);
pub const COPP_ConnectorType_SVideo: COPP_ConnectorType = COPP_ConnectorType(1i32);
pub const COPP_ConnectorType_CompositeVideo: COPP_ConnectorType = COPP_ConnectorType(2i32);
pub const COPP_ConnectorType_ComponentVideo: COPP_ConnectorType = COPP_ConnectorType(3i32);
pub const COPP_ConnectorType_DVI: COPP_ConnectorType = COPP_ConnectorType(4i32);
pub const COPP_ConnectorType_HDMI: COPP_ConnectorType = COPP_ConnectorType(5i32);
pub const COPP_ConnectorType_LVDS: COPP_ConnectorType = COPP_ConnectorType(6i32);
pub const COPP_ConnectorType_TMDS: COPP_ConnectorType = COPP_ConnectorType(7i32);
pub const COPP_ConnectorType_D_JPN: COPP_ConnectorType = COPP_ConnectorType(8i32);
pub const COPP_ConnectorType_Internal: COPP_ConnectorType = COPP_ConnectorType(-2147483648i32);
pub const COPP_ConnectorType_ForceDWORD: COPP_ConnectorType = COPP_ConnectorType(2147483647i32);
pub const COPP_DefaultProtectionLevel: u32 = 0u32;
#[repr(transparent)]
pub struct COPP_HDCP_Protection_Level(pub i32);
pub const COPP_HDCP_Level0: COPP_HDCP_Protection_Level = COPP_HDCP_Protection_Level(0i32);
pub const COPP_HDCP_LevelMin: COPP_HDCP_Protection_Level = COPP_HDCP_Protection_Level(0i32);
pub const COPP_HDCP_Level1: COPP_HDCP_Protection_Level = COPP_HDCP_Protection_Level(1i32);
pub const COPP_HDCP_LevelMax: COPP_HDCP_Protection_Level = COPP_HDCP_Protection_Level(1i32);
pub const COPP_HDCP_ForceDWORD: COPP_HDCP_Protection_Level = COPP_HDCP_Protection_Level(2147483647i32);
#[repr(transparent)]
pub struct COPP_ImageAspectRatio_EN300294(pub i32);
pub const COPP_AspectRatio_EN300294_FullFormat4by3: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(0i32);
pub const COPP_AspectRatio_EN300294_Box14by9Center: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(1i32);
pub const COPP_AspectRatio_EN300294_Box14by9Top: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(2i32);
pub const COPP_AspectRatio_EN300294_Box16by9Center: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(3i32);
pub const COPP_AspectRatio_EN300294_Box16by9Top: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(4i32);
pub const COPP_AspectRatio_EN300294_BoxGT16by9Center: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(5i32);
pub const COPP_AspectRatio_EN300294_FullFormat4by3ProtectedCenter: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(6i32);
pub const COPP_AspectRatio_EN300294_FullFormat16by9Anamorphic: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(7i32);
pub const COPP_AspectRatio_ForceDWORD: COPP_ImageAspectRatio_EN300294 = COPP_ImageAspectRatio_EN300294(2147483647i32);
pub const COPP_ImageAspectRatio_EN300294_Mask: u32 = 7u32;
pub const COPP_NoProtectionLevelAvailable: i32 = -1i32;
#[repr(transparent)]
pub struct COPP_StatusFlags(pub i32);
pub const COPP_StatusNormal: COPP_StatusFlags = COPP_StatusFlags(0i32);
pub const COPP_LinkLost: COPP_StatusFlags = COPP_StatusFlags(1i32);
pub const COPP_RenegotiationRequired: COPP_StatusFlags = COPP_StatusFlags(2i32);
pub const COPP_StatusFlagsReserved: COPP_StatusFlags = COPP_StatusFlags(-4i32);
#[repr(transparent)]
pub struct COPP_StatusHDCPFlags(pub i32);
pub const COPP_HDCPRepeater: COPP_StatusHDCPFlags = COPP_StatusHDCPFlags(1i32);
pub const COPP_HDCPFlagsReserved: COPP_StatusHDCPFlags = COPP_StatusHDCPFlags(-2i32);
#[repr(transparent)]
pub struct COPP_TVProtectionStandard(pub i32);
pub const COPP_ProtectionStandard_Unknown: COPP_TVProtectionStandard = COPP_TVProtectionStandard(-2147483648i32);
pub const COPP_ProtectionStandard_None: COPP_TVProtectionStandard = COPP_TVProtectionStandard(0i32);
pub const COPP_ProtectionStandard_IEC61880_525i: COPP_TVProtectionStandard = COPP_TVProtectionStandard(1i32);
pub const COPP_ProtectionStandard_IEC61880_2_525i: COPP_TVProtectionStandard = COPP_TVProtectionStandard(2i32);
pub const COPP_ProtectionStandard_IEC62375_625p: COPP_TVProtectionStandard = COPP_TVProtectionStandard(4i32);
pub const COPP_ProtectionStandard_EIA608B_525: COPP_TVProtectionStandard = COPP_TVProtectionStandard(8i32);
pub const COPP_ProtectionStandard_EN300294_625i: COPP_TVProtectionStandard = COPP_TVProtectionStandard(16i32);
pub const COPP_ProtectionStandard_CEA805A_TypeA_525p: COPP_TVProtectionStandard = COPP_TVProtectionStandard(32i32);
pub const COPP_ProtectionStandard_CEA805A_TypeA_750p: COPP_TVProtectionStandard = COPP_TVProtectionStandard(64i32);
pub const COPP_ProtectionStandard_CEA805A_TypeA_1125i: COPP_TVProtectionStandard = COPP_TVProtectionStandard(128i32);
pub const COPP_ProtectionStandard_CEA805A_TypeB_525p: COPP_TVProtectionStandard = COPP_TVProtectionStandard(256i32);
pub const COPP_ProtectionStandard_CEA805A_TypeB_750p: COPP_TVProtectionStandard = COPP_TVProtectionStandard(512i32);
pub const COPP_ProtectionStandard_CEA805A_TypeB_1125i: COPP_TVProtectionStandard = COPP_TVProtectionStandard(1024i32);
pub const COPP_ProtectionStandard_ARIBTRB15_525i: COPP_TVProtectionStandard = COPP_TVProtectionStandard(2048i32);
pub const COPP_ProtectionStandard_ARIBTRB15_525p: COPP_TVProtectionStandard = COPP_TVProtectionStandard(4096i32);
pub const COPP_ProtectionStandard_ARIBTRB15_750p: COPP_TVProtectionStandard = COPP_TVProtectionStandard(8192i32);
pub const COPP_ProtectionStandard_ARIBTRB15_1125i: COPP_TVProtectionStandard = COPP_TVProtectionStandard(16384i32);
pub const COPP_ProtectionStandard_Mask: COPP_TVProtectionStandard = COPP_TVProtectionStandard(-2147450881i32);
pub const COPP_ProtectionStandard_Reserved: COPP_TVProtectionStandard = COPP_TVProtectionStandard(2147450880i32);
#[repr(transparent)]
pub struct CPEventBitShift(pub i32);
pub const CPEVENT_BITSHIFT_RATINGS: CPEventBitShift = CPEventBitShift(0i32);
pub const CPEVENT_BITSHIFT_COPP: CPEventBitShift = CPEventBitShift(1i32);
pub const CPEVENT_BITSHIFT_LICENSE: CPEventBitShift = CPEventBitShift(2i32);
pub const CPEVENT_BITSHIFT_ROLLBACK: CPEventBitShift = CPEventBitShift(3i32);
pub const CPEVENT_BITSHIFT_SAC: CPEventBitShift = CPEventBitShift(4i32);
pub const CPEVENT_BITSHIFT_DOWNRES: CPEventBitShift = CPEventBitShift(5i32);
pub const CPEVENT_BITSHIFT_STUBLIB: CPEventBitShift = CPEventBitShift(6i32);
pub const CPEVENT_BITSHIFT_UNTRUSTEDGRAPH: CPEventBitShift = CPEventBitShift(7i32);
pub const CPEVENT_BITSHIFT_PENDING_CERTIFICATE: CPEventBitShift = CPEventBitShift(8i32);
pub const CPEVENT_BITSHIFT_NO_PLAYREADY: CPEventBitShift = CPEventBitShift(9i32);
#[repr(transparent)]
pub struct CPEvents(pub i32);
pub const CPEVENT_NONE: CPEvents = CPEvents(0i32);
pub const CPEVENT_RATINGS: CPEvents = CPEvents(1i32);
pub const CPEVENT_COPP: CPEvents = CPEvents(2i32);
pub const CPEVENT_LICENSE: CPEvents = CPEvents(3i32);
pub const CPEVENT_ROLLBACK: CPEvents = CPEvents(4i32);
pub const CPEVENT_SAC: CPEvents = CPEvents(5i32);
pub const CPEVENT_DOWNRES: CPEvents = CPEvents(6i32);
pub const CPEVENT_STUBLIB: CPEvents = CPEvents(7i32);
pub const CPEVENT_UNTRUSTEDGRAPH: CPEvents = CPEvents(8i32);
pub const CPEVENT_PROTECTWINDOWED: CPEvents = CPEvents(9i32);
#[repr(transparent)]
pub struct CPRecordingStatus(pub i32);
pub const RECORDING_STOPPED: CPRecordingStatus = CPRecordingStatus(0i32);
pub const RECORDING_STARTED: CPRecordingStatus = CPRecordingStatus(1i32);
#[repr(transparent)]
pub struct CRID_LOCATION(pub i32);
pub const CRID_LOCATION_IN_DESCRIPTOR: CRID_LOCATION = CRID_LOCATION(0i32);
pub const CRID_LOCATION_IN_CIT: CRID_LOCATION = CRID_LOCATION(1i32);
pub const CRID_LOCATION_DVB_RESERVED1: CRID_LOCATION = CRID_LOCATION(2i32);
pub const CRID_LOCATION_DVB_RESERVED2: CRID_LOCATION = CRID_LOCATION(3i32);
#[repr(transparent)]
pub struct CROSSBAR_DEFAULT_FLAGS(pub i32);
pub const DEF_MODE_PROFILE: CROSSBAR_DEFAULT_FLAGS = CROSSBAR_DEFAULT_FLAGS(1i32);
pub const DEF_MODE_STREAMS: CROSSBAR_DEFAULT_FLAGS = CROSSBAR_DEFAULT_FLAGS(2i32);
#[repr(C)]
pub struct CXDSData(i32);
#[repr(transparent)]
pub struct CameraControlFlags(pub i32);
pub const CameraControl_Flags_Auto: CameraControlFlags = CameraControlFlags(1i32);
pub const CameraControl_Flags_Manual: CameraControlFlags = CameraControlFlags(2i32);
#[repr(transparent)]
pub struct CameraControlProperty(pub i32);
pub const CameraControl_Pan: CameraControlProperty = CameraControlProperty(0i32);
pub const CameraControl_Tilt: CameraControlProperty = CameraControlProperty(1i32);
pub const CameraControl_Roll: CameraControlProperty = CameraControlProperty(2i32);
pub const CameraControl_Zoom: CameraControlProperty = CameraControlProperty(3i32);
pub const CameraControl_Exposure: CameraControlProperty = CameraControlProperty(4i32);
pub const CameraControl_Iris: CameraControlProperty = CameraControlProperty(5i32);
pub const CameraControl_Focus: CameraControlProperty = CameraControlProperty(6i32);
#[repr(C)]
pub struct ChannelChangeInfo(i32);
#[repr(transparent)]
pub struct ChannelChangeSpanningEvent_State(pub i32);
pub const ChannelChangeSpanningEvent_Start: ChannelChangeSpanningEvent_State = ChannelChangeSpanningEvent_State(0i32);
pub const ChannelChangeSpanningEvent_End: ChannelChangeSpanningEvent_State = ChannelChangeSpanningEvent_State(2i32);
#[repr(C)]
pub struct ChannelIDTuneRequest(i32);
#[repr(C)]
pub struct ChannelIDTuningSpace(i32);
#[repr(C)]
pub struct ChannelInfo(i32);
#[repr(C)]
pub struct ChannelTuneRequest(i32);
#[repr(transparent)]
pub struct ChannelType(pub i32);
pub const ChannelTypeNone: ChannelType = ChannelType(0i32);
pub const ChannelTypeOther: ChannelType = ChannelType(1i32);
pub const ChannelTypeVideo: ChannelType = ChannelType(2i32);
pub const ChannelTypeAudio: ChannelType = ChannelType(4i32);
pub const ChannelTypeText: ChannelType = ChannelType(8i32);
pub const ChannelTypeSubtitles: ChannelType = ChannelType(16i32);
pub const ChannelTypeCaptions: ChannelType = ChannelType(32i32);
pub const ChannelTypeSuperimpose: ChannelType = ChannelType(64i32);
pub const ChannelTypeData: ChannelType = ChannelType(128i32);
#[repr(C)]
pub struct ChannelTypeInfo(i32);
#[repr(C)]
pub struct Component(i32);
#[repr(transparent)]
pub struct ComponentCategory(pub i32);
pub const CategoryNotSet: ComponentCategory = ComponentCategory(-1i32);
pub const CategoryOther: ComponentCategory = ComponentCategory(0i32);
pub const CategoryVideo: ComponentCategory = ComponentCategory(1i32);
pub const CategoryAudio: ComponentCategory = ComponentCategory(2i32);
pub const CategoryText: ComponentCategory = ComponentCategory(3i32);
pub const CategorySubtitles: ComponentCategory = ComponentCategory(4i32);
pub const CategoryCaptions: ComponentCategory = ComponentCategory(5i32);
pub const CategorySuperimpose: ComponentCategory = ComponentCategory(6i32);
pub const CategoryData: ComponentCategory = ComponentCategory(7i32);
pub const CATEGORY_COUNT: ComponentCategory = ComponentCategory(8i32);
#[repr(transparent)]
pub struct ComponentStatus(pub i32);
pub const StatusActive: ComponentStatus = ComponentStatus(0i32);
pub const StatusInactive: ComponentStatus = ComponentStatus(1i32);
pub const StatusUnavailable: ComponentStatus = ComponentStatus(2i32);
#[repr(C)]
pub struct ComponentType(i32);
#[repr(C)]
pub struct ComponentTypes(i32);
#[repr(C)]
pub struct Components(i32);
#[repr(transparent)]
pub struct CompressionCaps(pub i32);
pub const CompressionCaps_CanQuality: CompressionCaps = CompressionCaps(1i32);
pub const CompressionCaps_CanCrunch: CompressionCaps = CompressionCaps(2i32);
pub const CompressionCaps_CanKeyFrame: CompressionCaps = CompressionCaps(4i32);
pub const CompressionCaps_CanBFrame: CompressionCaps = CompressionCaps(8i32);
pub const CompressionCaps_CanWindow: CompressionCaps = CompressionCaps(16i32);
#[repr(C)]
pub struct CreatePropBagOnRegKey(i32);
#[repr(transparent)]
pub struct DDSFF_FLAGS(pub u32);
pub const DDSFF_PROGRESSIVERENDER: DDSFF_FLAGS = DDSFF_FLAGS(1u32);
#[repr(transparent)]
pub struct DECIMATION_USAGE(pub i32);
pub const DECIMATION_LEGACY: DECIMATION_USAGE = DECIMATION_USAGE(0i32);
pub const DECIMATION_USE_DECODER_ONLY: DECIMATION_USAGE = DECIMATION_USAGE(1i32);
pub const DECIMATION_USE_VIDEOPORT_ONLY: DECIMATION_USAGE = DECIMATION_USAGE(2i32);
pub const DECIMATION_USE_OVERLAY_ONLY: DECIMATION_USAGE = DECIMATION_USAGE(3i32);
pub const DECIMATION_DEFAULT: DECIMATION_USAGE = DECIMATION_USAGE(4i32);
pub const DECODER_CAP_NOTSUPPORTED: u32 = 0u32;
pub const DECODER_CAP_SUPPORTED: u32 = 1u32;
#[repr(transparent)]
pub struct DESC_LINKAGE_TYPE(pub i32);
pub const DESC_LINKAGE_RESERVED0: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(0i32);
pub const DESC_LINKAGE_INFORMATION: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(1i32);
pub const DESC_LINKAGE_EPG: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(2i32);
pub const DESC_LINKAGE_CA_REPLACEMENT: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(3i32);
pub const DESC_LINKAGE_COMPLETE_NET_BOUQUET_SI: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(4i32);
pub const DESC_LINKAGE_REPLACEMENT: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(5i32);
pub const DESC_LINKAGE_DATA: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(6i32);
pub const DESC_LINKAGE_RESERVED1: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(7i32);
pub const DESC_LINKAGE_USER: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(8i32);
pub const DESC_LINKAGE_RESERVED2: DESC_LINKAGE_TYPE = DESC_LINKAGE_TYPE(255i32);
#[repr(C)]
pub struct DIGITAL_CABLE_NETWORK_TYPE(i32);
#[repr(C)]
pub struct DIRECT_TV_SATELLITE_TV_NETWORK_TYPE(i32);
#[repr(transparent)]
pub struct DISPID_TUNER(pub i32);
pub const DISPID_TUNER_TS_UNIQUENAME: DISPID_TUNER = DISPID_TUNER(1i32);
pub const DISPID_TUNER_TS_FRIENDLYNAME: DISPID_TUNER = DISPID_TUNER(2i32);
pub const DISPID_TUNER_TS_CLSID: DISPID_TUNER = DISPID_TUNER(3i32);
pub const DISPID_TUNER_TS_NETWORKTYPE: DISPID_TUNER = DISPID_TUNER(4i32);
pub const DISPID_TUNER_TS__NETWORKTYPE: DISPID_TUNER = DISPID_TUNER(5i32);
pub const DISPID_TUNER_TS_CREATETUNEREQUEST: DISPID_TUNER = DISPID_TUNER(6i32);
pub const DISPID_TUNER_TS_ENUMCATEGORYGUIDS: DISPID_TUNER = DISPID_TUNER(7i32);
pub const DISPID_TUNER_TS_ENUMDEVICEMONIKERS: DISPID_TUNER = DISPID_TUNER(8i32);
pub const DISPID_TUNER_TS_DEFAULTPREFERREDCOMPONENTTYPES: DISPID_TUNER = DISPID_TUNER(9i32);
pub const DISPID_TUNER_TS_FREQMAP: DISPID_TUNER = DISPID_TUNER(10i32);
pub const DISPID_TUNER_TS_DEFLOCATOR: DISPID_TUNER = DISPID_TUNER(11i32);
pub const DISPID_TUNER_TS_CLONE: DISPID_TUNER = DISPID_TUNER(12i32);
pub const DISPID_TUNER_TR_TUNINGSPACE: DISPID_TUNER = DISPID_TUNER(1i32);
pub const DISPID_TUNER_TR_COMPONENTS: DISPID_TUNER = DISPID_TUNER(2i32);
pub const DISPID_TUNER_TR_CLONE: DISPID_TUNER = DISPID_TUNER(3i32);
pub const DISPID_TUNER_TR_LOCATOR: DISPID_TUNER = DISPID_TUNER(4i32);
pub const DISPID_TUNER_CT_CATEGORY: DISPID_TUNER = DISPID_TUNER(1i32);
pub const DISPID_TUNER_CT_MEDIAMAJORTYPE: DISPID_TUNER = DISPID_TUNER(2i32);
pub const DISPID_TUNER_CT__MEDIAMAJORTYPE: DISPID_TUNER = DISPID_TUNER(3i32);
pub const DISPID_TUNER_CT_MEDIASUBTYPE: DISPID_TUNER = DISPID_TUNER(4i32);
pub const DISPID_TUNER_CT__MEDIASUBTYPE: DISPID_TUNER = DISPID_TUNER(5i32);
pub const DISPID_TUNER_CT_MEDIAFORMATTYPE: DISPID_TUNER = DISPID_TUNER(6i32);
pub const DISPID_TUNER_CT__MEDIAFORMATTYPE: DISPID_TUNER = DISPID_TUNER(7i32);
pub const DISPID_TUNER_CT_MEDIATYPE: DISPID_TUNER = DISPID_TUNER(8i32);
pub const DISPID_TUNER_CT_CLONE: DISPID_TUNER = DISPID_TUNER(9i32);
pub const DISPID_TUNER_LCT_LANGID: DISPID_TUNER = DISPID_TUNER(100i32);
pub const DISPID_TUNER_MP2CT_TYPE: DISPID_TUNER = DISPID_TUNER(200i32);
pub const DISPID_TUNER_ATSCCT_FLAGS: DISPID_TUNER = DISPID_TUNER(300i32);
pub const DISPID_TUNER_L_CARRFREQ: DISPID_TUNER = DISPID_TUNER(1i32);
pub const DISPID_TUNER_L_INNERFECMETHOD: DISPID_TUNER = DISPID_TUNER(2i32);
pub const DISPID_TUNER_L_INNERFECRATE: DISPID_TUNER = DISPID_TUNER(3i32);
pub const DISPID_TUNER_L_OUTERFECMETHOD: DISPID_TUNER = DISPID_TUNER(4i32);
pub const DISPID_TUNER_L_OUTERFECRATE: DISPID_TUNER = DISPID_TUNER(5i32);
pub const DISPID_TUNER_L_MOD: DISPID_TUNER = DISPID_TUNER(6i32);
pub const DISPID_TUNER_L_SYMRATE: DISPID_TUNER = DISPID_TUNER(7i32);
pub const DISPID_TUNER_L_CLONE: DISPID_TUNER = DISPID_TUNER(8i32);
pub const DISPID_TUNER_L_ATSC_PHYS_CHANNEL: DISPID_TUNER = DISPID_TUNER(201i32);
pub const DISPID_TUNER_L_ATSC_TSID: DISPID_TUNER = DISPID_TUNER(202i32);
pub const DISPID_TUNER_L_ATSC_MP2_PROGNO: DISPID_TUNER = DISPID_TUNER(203i32);
pub const DISPID_TUNER_L_DVBT_BANDWIDTH: DISPID_TUNER = DISPID_TUNER(301i32);
pub const DISPID_TUNER_L_DVBT_LPINNERFECMETHOD: DISPID_TUNER = DISPID_TUNER(302i32);
pub const DISPID_TUNER_L_DVBT_LPINNERFECRATE: DISPID_TUNER = DISPID_TUNER(303i32);
pub const DISPID_TUNER_L_DVBT_GUARDINTERVAL: DISPID_TUNER = DISPID_TUNER(304i32);
pub const DISPID_TUNER_L_DVBT_HALPHA: DISPID_TUNER = DISPID_TUNER(305i32);
pub const DISPID_TUNER_L_DVBT_TRANSMISSIONMODE: DISPID_TUNER = DISPID_TUNER(306i32);
pub const DISPID_TUNER_L_DVBT_INUSE: DISPID_TUNER = DISPID_TUNER(307i32);
pub const DISPID_TUNER_L_DVBT2_PHYSICALLAYERPIPEID: DISPID_TUNER = DISPID_TUNER(351i32);
pub const DISPID_TUNER_L_DVBS_POLARISATION: DISPID_TUNER = DISPID_TUNER(401i32);
pub const DISPID_TUNER_L_DVBS_WEST: DISPID_TUNER = DISPID_TUNER(402i32);
pub const DISPID_TUNER_L_DVBS_ORBITAL: DISPID_TUNER = DISPID_TUNER(403i32);
pub const DISPID_TUNER_L_DVBS_AZIMUTH: DISPID_TUNER = DISPID_TUNER(404i32);
pub const DISPID_TUNER_L_DVBS_ELEVATION: DISPID_TUNER = DISPID_TUNER(405i32);
pub const DISPID_TUNER_L_DVBS2_DISEQ_LNB_SOURCE: DISPID_TUNER = DISPID_TUNER(406i32);
pub const DISPID_TUNER_TS_DVBS2_LOW_OSC_FREQ_OVERRIDE: DISPID_TUNER = DISPID_TUNER(407i32);
pub const DISPID_TUNER_TS_DVBS2_HI_OSC_FREQ_OVERRIDE: DISPID_TUNER = DISPID_TUNER(408i32);
pub const DISPID_TUNER_TS_DVBS2_LNB_SWITCH_FREQ_OVERRIDE: DISPID_TUNER = DISPID_TUNER(409i32);
pub const DISPID_TUNER_TS_DVBS2_SPECTRAL_INVERSION_OVERRIDE: DISPID_TUNER = DISPID_TUNER(410i32);
pub const DISPID_TUNER_L_DVBS2_ROLLOFF: DISPID_TUNER = DISPID_TUNER(411i32);
pub const DISPID_TUNER_L_DVBS2_PILOT: DISPID_TUNER = DISPID_TUNER(412i32);
pub const DISPID_TUNER_L_ANALOG_STANDARD: DISPID_TUNER = DISPID_TUNER(601i32);
pub const DISPID_TUNER_L_DTV_O_MAJOR_CHANNEL: DISPID_TUNER = DISPID_TUNER(701i32);
pub const DISPID_TUNER_C_TYPE: DISPID_TUNER = DISPID_TUNER(1i32);
pub const DISPID_TUNER_C_STATUS: DISPID_TUNER = DISPID_TUNER(2i32);
pub const DISPID_TUNER_C_LANGID: DISPID_TUNER = DISPID_TUNER(3i32);
pub const DISPID_TUNER_C_DESCRIPTION: DISPID_TUNER = DISPID_TUNER(4i32);
pub const DISPID_TUNER_C_CLONE: DISPID_TUNER = DISPID_TUNER(5i32);
pub const DISPID_TUNER_C_MP2_PID: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_TUNER_C_MP2_PCRPID: DISPID_TUNER = DISPID_TUNER(102i32);
pub const DISPID_TUNER_C_MP2_PROGNO: DISPID_TUNER = DISPID_TUNER(103i32);
pub const DISPID_TUNER_C_ANALOG_AUDIO: DISPID_TUNER = DISPID_TUNER(201i32);
pub const DISPID_TUNER_TS_DVB_SYSTEMTYPE: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_TUNER_TS_DVB2_NETWORK_ID: DISPID_TUNER = DISPID_TUNER(102i32);
pub const DISPID_TUNER_TS_DVBS_LOW_OSC_FREQ: DISPID_TUNER = DISPID_TUNER(1001i32);
pub const DISPID_TUNER_TS_DVBS_HI_OSC_FREQ: DISPID_TUNER = DISPID_TUNER(1002i32);
pub const DISPID_TUNER_TS_DVBS_LNB_SWITCH_FREQ: DISPID_TUNER = DISPID_TUNER(1003i32);
pub const DISPID_TUNER_TS_DVBS_INPUT_RANGE: DISPID_TUNER = DISPID_TUNER(1004i32);
pub const DISPID_TUNER_TS_DVBS_SPECTRAL_INVERSION: DISPID_TUNER = DISPID_TUNER(1005i32);
pub const DISPID_TUNER_TS_AR_MINFREQUENCY: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_TUNER_TS_AR_MAXFREQUENCY: DISPID_TUNER = DISPID_TUNER(102i32);
pub const DISPID_TUNER_TS_AR_STEP: DISPID_TUNER = DISPID_TUNER(103i32);
pub const DISPID_TUNER_TS_AR_COUNTRYCODE: DISPID_TUNER = DISPID_TUNER(104i32);
pub const DISPID_TUNER_TS_AUX_COUNTRYCODE: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_TUNER_TS_ATV_MINCHANNEL: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_TUNER_TS_ATV_MAXCHANNEL: DISPID_TUNER = DISPID_TUNER(102i32);
pub const DISPID_TUNER_TS_ATV_INPUTTYPE: DISPID_TUNER = DISPID_TUNER(103i32);
pub const DISPID_TUNER_TS_ATV_COUNTRYCODE: DISPID_TUNER = DISPID_TUNER(104i32);
pub const DISPID_TUNER_TS_ATSC_MINMINORCHANNEL: DISPID_TUNER = DISPID_TUNER(201i32);
pub const DISPID_TUNER_TS_ATSC_MAXMINORCHANNEL: DISPID_TUNER = DISPID_TUNER(202i32);
pub const DISPID_TUNER_TS_ATSC_MINPHYSCHANNEL: DISPID_TUNER = DISPID_TUNER(203i32);
pub const DISPID_TUNER_TS_ATSC_MAXPHYSCHANNEL: DISPID_TUNER = DISPID_TUNER(204i32);
pub const DISPID_TUNER_TS_DC_MINMAJORCHANNEL: DISPID_TUNER = DISPID_TUNER(301i32);
pub const DISPID_TUNER_TS_DC_MAXMAJORCHANNEL: DISPID_TUNER = DISPID_TUNER(302i32);
pub const DISPID_TUNER_TS_DC_MINSOURCEID: DISPID_TUNER = DISPID_TUNER(303i32);
pub const DISPID_TUNER_TS_DC_MAXSOURCEID: DISPID_TUNER = DISPID_TUNER(304i32);
pub const DISPID_CHTUNER_ATVAC_CHANNEL: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_CHTUNER_ATVDC_SYSTEM: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_CHTUNER_ATVDC_CONTENT: DISPID_TUNER = DISPID_TUNER(102i32);
pub const DISPID_CHTUNER_CIDTR_CHANNELID: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_CHTUNER_CTR_CHANNEL: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_CHTUNER_ACTR_MINOR_CHANNEL: DISPID_TUNER = DISPID_TUNER(201i32);
pub const DISPID_CHTUNER_DCTR_MAJOR_CHANNEL: DISPID_TUNER = DISPID_TUNER(301i32);
pub const DISPID_CHTUNER_DCTR_SRCID: DISPID_TUNER = DISPID_TUNER(302i32);
pub const DISPID_DVBTUNER_DVBC_ATTRIBUTESVALID: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_DVBTUNER_DVBC_PID: DISPID_TUNER = DISPID_TUNER(102i32);
pub const DISPID_DVBTUNER_DVBC_TAG: DISPID_TUNER = DISPID_TUNER(103i32);
pub const DISPID_DVBTUNER_DVBC_COMPONENTTYPE: DISPID_TUNER = DISPID_TUNER(104i32);
pub const DISPID_DVBTUNER_ONID: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_DVBTUNER_TSID: DISPID_TUNER = DISPID_TUNER(102i32);
pub const DISPID_DVBTUNER_SID: DISPID_TUNER = DISPID_TUNER(103i32);
pub const DISPID_MP2TUNER_TSID: DISPID_TUNER = DISPID_TUNER(101i32);
pub const DISPID_MP2TUNER_PROGNO: DISPID_TUNER = DISPID_TUNER(102i32);
pub const DISPID_MP2TUNERFACTORY_CREATETUNEREQUEST: DISPID_TUNER = DISPID_TUNER(1i32);
pub const DSATTRIB_BadSampleInfo: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3833884122,
    data2: 22584,
    data3: 17076,
    data4: [184, 151, 111, 126, 95, 170, 47, 47],
};
pub const DSATTRIB_WMDRMProtectionInfo: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1081382275,
    data2: 27549,
    data3: 20204,
    data4: [180, 60, 103, 161, 128, 30, 26, 155],
};
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSHOW_STREAM_DESC(i32);
#[repr(C)]
pub struct DSMCC_ELEMENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DSMCC_FILTER_OPTIONS(i32);
#[repr(C)]
pub struct DSMCC_SECTION(i32);
#[repr(C)]
pub struct DTFilter(i32);
pub const DTV_CardStatus_Error: u32 = 2u32;
pub const DTV_CardStatus_FirmwareDownload: u32 = 3u32;
pub const DTV_CardStatus_Inserted: u32 = 0u32;
pub const DTV_CardStatus_Removed: u32 = 1u32;
pub const DTV_Entitlement_CanDecrypt: u32 = 0u32;
pub const DTV_Entitlement_NotEntitled: u32 = 1u32;
pub const DTV_Entitlement_TechnicalFailure: u32 = 2u32;
pub const DTV_MMIMessage_Close: u32 = 1u32;
pub const DTV_MMIMessage_Open: u32 = 0u32;
#[repr(C)]
pub struct DVBCLocator(i32);
#[repr(C)]
pub struct DVBSLocator(i32);
#[repr(C)]
pub struct DVBSTuningSpace(i32);
pub const DVBS_SCAN_TABLE_MAX_SIZE: u32 = 400u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVBScramblingControlSpanningEvent(i32);
#[repr(transparent)]
pub struct DVBSystemType(pub i32);
pub const DVB_Cable: DVBSystemType = DVBSystemType(0i32);
pub const DVB_Terrestrial: DVBSystemType = DVBSystemType(1i32);
pub const DVB_Satellite: DVBSystemType = DVBSystemType(2i32);
pub const ISDB_Terrestrial: DVBSystemType = DVBSystemType(3i32);
pub const ISDB_Satellite: DVBSystemType = DVBSystemType(4i32);
#[repr(C)]
pub struct DVBTLocator(i32);
#[repr(C)]
pub struct DVBTLocator2(i32);
#[repr(C)]
pub struct DVBTuneRequest(i32);
#[repr(C)]
pub struct DVBTuningSpace(i32);
pub const DVB_BAT_PID: u32 = 17u32;
pub const DVB_BAT_TID: u32 = 74u32;
#[repr(C)]
pub struct DVB_CABLE_TV_NETWORK_TYPE(i32);
pub const DVB_DIT_PID: u32 = 30u32;
pub const DVB_DIT_TID: u32 = 126u32;
pub const DVB_EIT_ACTUAL_TID: u32 = 78u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVB_EIT_FILTER_OPTIONS(i32);
pub const DVB_EIT_OTHER_TID: u32 = 79u32;
pub const DVB_EIT_PID: u32 = 18u32;
pub const DVB_NIT_ACTUAL_TID: u32 = 64u32;
pub const DVB_NIT_OTHER_TID: u32 = 65u32;
pub const DVB_NIT_PID: u32 = 16u32;
pub const DVB_RST_PID: u32 = 19u32;
pub const DVB_RST_TID: u32 = 113u32;
#[repr(C)]
pub struct DVB_SATELLITE_TV_NETWORK_TYPE(i32);
pub const DVB_SDT_ACTUAL_TID: u32 = 66u32;
pub const DVB_SDT_OTHER_TID: u32 = 70u32;
pub const DVB_SDT_PID: u32 = 17u32;
pub const DVB_SIT_PID: u32 = 31u32;
pub const DVB_SIT_TID: u32 = 127u32;
#[repr(transparent)]
pub struct DVB_STRCONV_MODE(pub i32);
pub const STRCONV_MODE_DVB: DVB_STRCONV_MODE = DVB_STRCONV_MODE(0i32);
pub const STRCONV_MODE_DVB_EMPHASIS: DVB_STRCONV_MODE = DVB_STRCONV_MODE(1i32);
pub const STRCONV_MODE_DVB_WITHOUT_EMPHASIS: DVB_STRCONV_MODE = DVB_STRCONV_MODE(2i32);
pub const STRCONV_MODE_ISDB: DVB_STRCONV_MODE = DVB_STRCONV_MODE(3i32);
pub const DVB_ST_PID_16: u32 = 16u32;
pub const DVB_ST_PID_17: u32 = 17u32;
pub const DVB_ST_PID_18: u32 = 18u32;
pub const DVB_ST_PID_19: u32 = 19u32;
pub const DVB_ST_PID_20: u32 = 20u32;
pub const DVB_ST_TID: u32 = 114u32;
pub const DVB_TDT_PID: u32 = 20u32;
pub const DVB_TDT_TID: u32 = 112u32;
#[repr(C)]
pub struct DVB_TERRESTRIAL_TV_NETWORK_TYPE(i32);
pub const DVB_TOT_PID: u32 = 20u32;
pub const DVB_TOT_TID: u32 = 115u32;
#[repr(transparent)]
pub struct DVDFilterState(pub i32);
pub const dvdState_Undefined: DVDFilterState = DVDFilterState(-2i32);
pub const dvdState_Unitialized: DVDFilterState = DVDFilterState(-1i32);
pub const dvdState_Stopped: DVDFilterState = DVDFilterState(0i32);
pub const dvdState_Paused: DVDFilterState = DVDFilterState(1i32);
pub const dvdState_Running: DVDFilterState = DVDFilterState(2i32);
#[repr(transparent)]
pub struct DVDMenuIDConstants(pub i32);
pub const dvdMenu_Title: DVDMenuIDConstants = DVDMenuIDConstants(2i32);
pub const dvdMenu_Root: DVDMenuIDConstants = DVDMenuIDConstants(3i32);
pub const dvdMenu_Subpicture: DVDMenuIDConstants = DVDMenuIDConstants(4i32);
pub const dvdMenu_Audio: DVDMenuIDConstants = DVDMenuIDConstants(5i32);
pub const dvdMenu_Angle: DVDMenuIDConstants = DVDMenuIDConstants(6i32);
pub const dvdMenu_Chapter: DVDMenuIDConstants = DVDMenuIDConstants(7i32);
#[repr(transparent)]
pub struct DVDSPExt(pub i32);
pub const dvdSPExt_NotSpecified: DVDSPExt = DVDSPExt(0i32);
pub const dvdSPExt_Caption_Normal: DVDSPExt = DVDSPExt(1i32);
pub const dvdSPExt_Caption_Big: DVDSPExt = DVDSPExt(2i32);
pub const dvdSPExt_Caption_Children: DVDSPExt = DVDSPExt(3i32);
pub const dvdSPExt_CC_Normal: DVDSPExt = DVDSPExt(5i32);
pub const dvdSPExt_CC_Big: DVDSPExt = DVDSPExt(6i32);
pub const dvdSPExt_CC_Children: DVDSPExt = DVDSPExt(7i32);
pub const dvdSPExt_Forced: DVDSPExt = DVDSPExt(9i32);
pub const dvdSPExt_DirectorComments_Normal: DVDSPExt = DVDSPExt(13i32);
pub const dvdSPExt_DirectorComments_Big: DVDSPExt = DVDSPExt(14i32);
pub const dvdSPExt_DirectorComments_Children: DVDSPExt = DVDSPExt(15i32);
#[repr(transparent)]
pub struct DVDTextStringType(pub i32);
pub const dvdStruct_Volume: DVDTextStringType = DVDTextStringType(1i32);
pub const dvdStruct_Title: DVDTextStringType = DVDTextStringType(2i32);
pub const dvdStruct_ParentalID: DVDTextStringType = DVDTextStringType(3i32);
pub const dvdStruct_PartOfTitle: DVDTextStringType = DVDTextStringType(4i32);
pub const dvdStruct_Cell: DVDTextStringType = DVDTextStringType(5i32);
pub const dvdStream_Audio: DVDTextStringType = DVDTextStringType(16i32);
pub const dvdStream_Subpicture: DVDTextStringType = DVDTextStringType(17i32);
pub const dvdStream_Angle: DVDTextStringType = DVDTextStringType(18i32);
pub const dvdChannel_Audio: DVDTextStringType = DVDTextStringType(32i32);
pub const dvdGeneral_Name: DVDTextStringType = DVDTextStringType(48i32);
pub const dvdGeneral_Comments: DVDTextStringType = DVDTextStringType(49i32);
pub const dvdTitle_Series: DVDTextStringType = DVDTextStringType(56i32);
pub const dvdTitle_Movie: DVDTextStringType = DVDTextStringType(57i32);
pub const dvdTitle_Video: DVDTextStringType = DVDTextStringType(58i32);
pub const dvdTitle_Album: DVDTextStringType = DVDTextStringType(59i32);
pub const dvdTitle_Song: DVDTextStringType = DVDTextStringType(60i32);
pub const dvdTitle_Other: DVDTextStringType = DVDTextStringType(63i32);
pub const dvdTitle_Sub_Series: DVDTextStringType = DVDTextStringType(64i32);
pub const dvdTitle_Sub_Movie: DVDTextStringType = DVDTextStringType(65i32);
pub const dvdTitle_Sub_Video: DVDTextStringType = DVDTextStringType(66i32);
pub const dvdTitle_Sub_Album: DVDTextStringType = DVDTextStringType(67i32);
pub const dvdTitle_Sub_Song: DVDTextStringType = DVDTextStringType(68i32);
pub const dvdTitle_Sub_Other: DVDTextStringType = DVDTextStringType(71i32);
pub const dvdTitle_Orig_Series: DVDTextStringType = DVDTextStringType(72i32);
pub const dvdTitle_Orig_Movie: DVDTextStringType = DVDTextStringType(73i32);
pub const dvdTitle_Orig_Video: DVDTextStringType = DVDTextStringType(74i32);
pub const dvdTitle_Orig_Album: DVDTextStringType = DVDTextStringType(75i32);
pub const dvdTitle_Orig_Song: DVDTextStringType = DVDTextStringType(76i32);
pub const dvdTitle_Orig_Other: DVDTextStringType = DVDTextStringType(79i32);
pub const dvdOther_Scene: DVDTextStringType = DVDTextStringType(80i32);
pub const dvdOther_Cut: DVDTextStringType = DVDTextStringType(81i32);
pub const dvdOther_Take: DVDTextStringType = DVDTextStringType(82i32);
#[repr(C)]
pub struct DVD_ATR(i32);
#[repr(transparent)]
pub struct DVD_AUDIO_APPMODE(pub i32);
pub const DVD_AudioMode_None: DVD_AUDIO_APPMODE = DVD_AUDIO_APPMODE(0i32);
pub const DVD_AudioMode_Karaoke: DVD_AUDIO_APPMODE = DVD_AUDIO_APPMODE(1i32);
pub const DVD_AudioMode_Surround: DVD_AUDIO_APPMODE = DVD_AUDIO_APPMODE(2i32);
pub const DVD_AudioMode_Other: DVD_AUDIO_APPMODE = DVD_AUDIO_APPMODE(3i32);
pub const DVD_AUDIO_CAPS_AC3: u32 = 1u32;
pub const DVD_AUDIO_CAPS_DTS: u32 = 8u32;
pub const DVD_AUDIO_CAPS_LPCM: u32 = 4u32;
pub const DVD_AUDIO_CAPS_MPEG2: u32 = 2u32;
pub const DVD_AUDIO_CAPS_SDDS: u32 = 16u32;
#[repr(transparent)]
pub struct DVD_AUDIO_FORMAT(pub i32);
pub const DVD_AudioFormat_AC3: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(0i32);
pub const DVD_AudioFormat_MPEG1: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(1i32);
pub const DVD_AudioFormat_MPEG1_DRC: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(2i32);
pub const DVD_AudioFormat_MPEG2: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(3i32);
pub const DVD_AudioFormat_MPEG2_DRC: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(4i32);
pub const DVD_AudioFormat_LPCM: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(5i32);
pub const DVD_AudioFormat_DTS: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(6i32);
pub const DVD_AudioFormat_SDDS: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(7i32);
pub const DVD_AudioFormat_Other: DVD_AUDIO_FORMAT = DVD_AUDIO_FORMAT(8i32);
#[repr(transparent)]
pub struct DVD_AUDIO_LANG_EXT(pub i32);
pub const DVD_AUD_EXT_NotSpecified: DVD_AUDIO_LANG_EXT = DVD_AUDIO_LANG_EXT(0i32);
pub const DVD_AUD_EXT_Captions: DVD_AUDIO_LANG_EXT = DVD_AUDIO_LANG_EXT(1i32);
pub const DVD_AUD_EXT_VisuallyImpaired: DVD_AUDIO_LANG_EXT = DVD_AUDIO_LANG_EXT(2i32);
pub const DVD_AUD_EXT_DirectorComments1: DVD_AUDIO_LANG_EXT = DVD_AUDIO_LANG_EXT(3i32);
pub const DVD_AUD_EXT_DirectorComments2: DVD_AUDIO_LANG_EXT = DVD_AUDIO_LANG_EXT(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_AudioAttributes(i32);
#[repr(transparent)]
pub struct DVD_CMD_FLAGS(pub i32);
pub const DVD_CMD_FLAG_None: DVD_CMD_FLAGS = DVD_CMD_FLAGS(0i32);
pub const DVD_CMD_FLAG_Flush: DVD_CMD_FLAGS = DVD_CMD_FLAGS(1i32);
pub const DVD_CMD_FLAG_SendEvents: DVD_CMD_FLAGS = DVD_CMD_FLAGS(2i32);
pub const DVD_CMD_FLAG_Block: DVD_CMD_FLAGS = DVD_CMD_FLAGS(4i32);
pub const DVD_CMD_FLAG_StartWhenRendered: DVD_CMD_FLAGS = DVD_CMD_FLAGS(8i32);
pub const DVD_CMD_FLAG_EndAfterRendered: DVD_CMD_FLAGS = DVD_CMD_FLAGS(16i32);
#[repr(C)]
pub struct DVD_DECODER_CAPS(i32);
pub const DVD_DEFAULT_AUDIO_STREAM: u32 = 15u32;
#[repr(transparent)]
pub struct DVD_DISC_SIDE(pub i32);
pub const DVD_SIDE_A: DVD_DISC_SIDE = DVD_DISC_SIDE(1i32);
pub const DVD_SIDE_B: DVD_DISC_SIDE = DVD_DISC_SIDE(2i32);
#[repr(transparent)]
pub struct DVD_DOMAIN(pub i32);
pub const DVD_DOMAIN_FirstPlay: DVD_DOMAIN = DVD_DOMAIN(1i32);
pub const DVD_DOMAIN_VideoManagerMenu: DVD_DOMAIN = DVD_DOMAIN(2i32);
pub const DVD_DOMAIN_VideoTitleSetMenu: DVD_DOMAIN = DVD_DOMAIN(3i32);
pub const DVD_DOMAIN_Title: DVD_DOMAIN = DVD_DOMAIN(4i32);
pub const DVD_DOMAIN_Stop: DVD_DOMAIN = DVD_DOMAIN(5i32);
#[repr(transparent)]
pub struct DVD_ERROR(pub i32);
pub const DVD_ERROR_Unexpected: DVD_ERROR = DVD_ERROR(1i32);
pub const DVD_ERROR_CopyProtectFail: DVD_ERROR = DVD_ERROR(2i32);
pub const DVD_ERROR_InvalidDVD1_0Disc: DVD_ERROR = DVD_ERROR(3i32);
pub const DVD_ERROR_InvalidDiscRegion: DVD_ERROR = DVD_ERROR(4i32);
pub const DVD_ERROR_LowParentalLevel: DVD_ERROR = DVD_ERROR(5i32);
pub const DVD_ERROR_MacrovisionFail: DVD_ERROR = DVD_ERROR(6i32);
pub const DVD_ERROR_IncompatibleSystemAndDecoderRegions: DVD_ERROR = DVD_ERROR(7i32);
pub const DVD_ERROR_IncompatibleDiscAndDecoderRegions: DVD_ERROR = DVD_ERROR(8i32);
pub const DVD_ERROR_CopyProtectOutputFail: DVD_ERROR = DVD_ERROR(9i32);
pub const DVD_ERROR_CopyProtectOutputNotSupported: DVD_ERROR = DVD_ERROR(10i32);
#[repr(transparent)]
pub struct DVD_FRAMERATE(pub i32);
pub const DVD_FPS_25: DVD_FRAMERATE = DVD_FRAMERATE(1i32);
pub const DVD_FPS_30NonDrop: DVD_FRAMERATE = DVD_FRAMERATE(3i32);
#[repr(C)]
pub struct DVD_HMSF_TIMECODE(i32);
#[repr(transparent)]
pub struct DVD_KARAOKE_ASSIGNMENT(pub i32);
pub const DVD_Assignment_reserved0: DVD_KARAOKE_ASSIGNMENT = DVD_KARAOKE_ASSIGNMENT(0i32);
pub const DVD_Assignment_reserved1: DVD_KARAOKE_ASSIGNMENT = DVD_KARAOKE_ASSIGNMENT(1i32);
pub const DVD_Assignment_LR: DVD_KARAOKE_ASSIGNMENT = DVD_KARAOKE_ASSIGNMENT(2i32);
pub const DVD_Assignment_LRM: DVD_KARAOKE_ASSIGNMENT = DVD_KARAOKE_ASSIGNMENT(3i32);
pub const DVD_Assignment_LR1: DVD_KARAOKE_ASSIGNMENT = DVD_KARAOKE_ASSIGNMENT(4i32);
pub const DVD_Assignment_LRM1: DVD_KARAOKE_ASSIGNMENT = DVD_KARAOKE_ASSIGNMENT(5i32);
pub const DVD_Assignment_LR12: DVD_KARAOKE_ASSIGNMENT = DVD_KARAOKE_ASSIGNMENT(6i32);
pub const DVD_Assignment_LRM12: DVD_KARAOKE_ASSIGNMENT = DVD_KARAOKE_ASSIGNMENT(7i32);
#[repr(transparent)]
pub struct DVD_KARAOKE_CONTENTS(pub i32);
pub const DVD_Karaoke_GuideVocal1: DVD_KARAOKE_CONTENTS = DVD_KARAOKE_CONTENTS(1i32);
pub const DVD_Karaoke_GuideVocal2: DVD_KARAOKE_CONTENTS = DVD_KARAOKE_CONTENTS(2i32);
pub const DVD_Karaoke_GuideMelody1: DVD_KARAOKE_CONTENTS = DVD_KARAOKE_CONTENTS(4i32);
pub const DVD_Karaoke_GuideMelody2: DVD_KARAOKE_CONTENTS = DVD_KARAOKE_CONTENTS(8i32);
pub const DVD_Karaoke_GuideMelodyA: DVD_KARAOKE_CONTENTS = DVD_KARAOKE_CONTENTS(16i32);
pub const DVD_Karaoke_GuideMelodyB: DVD_KARAOKE_CONTENTS = DVD_KARAOKE_CONTENTS(32i32);
pub const DVD_Karaoke_SoundEffectA: DVD_KARAOKE_CONTENTS = DVD_KARAOKE_CONTENTS(64i32);
pub const DVD_Karaoke_SoundEffectB: DVD_KARAOKE_CONTENTS = DVD_KARAOKE_CONTENTS(128i32);
#[repr(transparent)]
pub struct DVD_KARAOKE_DOWNMIX(pub i32);
pub const DVD_Mix_0to0: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(1i32);
pub const DVD_Mix_1to0: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(2i32);
pub const DVD_Mix_2to0: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(4i32);
pub const DVD_Mix_3to0: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(8i32);
pub const DVD_Mix_4to0: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(16i32);
pub const DVD_Mix_Lto0: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(32i32);
pub const DVD_Mix_Rto0: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(64i32);
pub const DVD_Mix_0to1: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(256i32);
pub const DVD_Mix_1to1: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(512i32);
pub const DVD_Mix_2to1: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(1024i32);
pub const DVD_Mix_3to1: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(2048i32);
pub const DVD_Mix_4to1: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(4096i32);
pub const DVD_Mix_Lto1: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(8192i32);
pub const DVD_Mix_Rto1: DVD_KARAOKE_DOWNMIX = DVD_KARAOKE_DOWNMIX(16384i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_KaraokeAttributes(i32);
#[repr(transparent)]
pub struct DVD_MENU_ID(pub i32);
pub const DVD_MENU_Title: DVD_MENU_ID = DVD_MENU_ID(2i32);
pub const DVD_MENU_Root: DVD_MENU_ID = DVD_MENU_ID(3i32);
pub const DVD_MENU_Subpicture: DVD_MENU_ID = DVD_MENU_ID(4i32);
pub const DVD_MENU_Audio: DVD_MENU_ID = DVD_MENU_ID(5i32);
pub const DVD_MENU_Angle: DVD_MENU_ID = DVD_MENU_ID(6i32);
pub const DVD_MENU_Chapter: DVD_MENU_ID = DVD_MENU_ID(7i32);
#[repr(C)]
pub struct DVD_MUA_Coeff(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_MUA_MixingInfo(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_MenuAttributes(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_MultichannelAudioAttributes(i32);
#[repr(transparent)]
pub struct DVD_NavCmdType(pub i32);
pub const DVD_NavCmdType_Pre: DVD_NavCmdType = DVD_NavCmdType(1i32);
pub const DVD_NavCmdType_Post: DVD_NavCmdType = DVD_NavCmdType(2i32);
pub const DVD_NavCmdType_Cell: DVD_NavCmdType = DVD_NavCmdType(3i32);
pub const DVD_NavCmdType_Button: DVD_NavCmdType = DVD_NavCmdType(4i32);
#[repr(transparent)]
pub struct DVD_OPTION_FLAG(pub i32);
pub const DVD_ResetOnStop: DVD_OPTION_FLAG = DVD_OPTION_FLAG(1i32);
pub const DVD_NotifyParentalLevelChange: DVD_OPTION_FLAG = DVD_OPTION_FLAG(2i32);
pub const DVD_HMSF_TimeCodeEvents: DVD_OPTION_FLAG = DVD_OPTION_FLAG(3i32);
pub const DVD_AudioDuringFFwdRew: DVD_OPTION_FLAG = DVD_OPTION_FLAG(4i32);
pub const DVD_EnableNonblockingAPIs: DVD_OPTION_FLAG = DVD_OPTION_FLAG(5i32);
pub const DVD_CacheSizeInMB: DVD_OPTION_FLAG = DVD_OPTION_FLAG(6i32);
pub const DVD_EnablePortableBookmarks: DVD_OPTION_FLAG = DVD_OPTION_FLAG(7i32);
pub const DVD_EnableExtendedCopyProtectErrors: DVD_OPTION_FLAG = DVD_OPTION_FLAG(8i32);
pub const DVD_NotifyPositionChange: DVD_OPTION_FLAG = DVD_OPTION_FLAG(9i32);
pub const DVD_IncreaseOutputControl: DVD_OPTION_FLAG = DVD_OPTION_FLAG(10i32);
pub const DVD_EnableStreaming: DVD_OPTION_FLAG = DVD_OPTION_FLAG(11i32);
pub const DVD_EnableESOutput: DVD_OPTION_FLAG = DVD_OPTION_FLAG(12i32);
pub const DVD_EnableTitleLength: DVD_OPTION_FLAG = DVD_OPTION_FLAG(13i32);
pub const DVD_DisableStillThrottle: DVD_OPTION_FLAG = DVD_OPTION_FLAG(14i32);
pub const DVD_EnableLoggingEvents: DVD_OPTION_FLAG = DVD_OPTION_FLAG(15i32);
pub const DVD_MaxReadBurstInKB: DVD_OPTION_FLAG = DVD_OPTION_FLAG(16i32);
pub const DVD_ReadBurstPeriodInMS: DVD_OPTION_FLAG = DVD_OPTION_FLAG(17i32);
pub const DVD_RestartDisc: DVD_OPTION_FLAG = DVD_OPTION_FLAG(18i32);
pub const DVD_EnableCC: DVD_OPTION_FLAG = DVD_OPTION_FLAG(19i32);
#[repr(transparent)]
pub struct DVD_PARENTAL_LEVEL(pub i32);
pub const DVD_PARENTAL_LEVEL_8: DVD_PARENTAL_LEVEL = DVD_PARENTAL_LEVEL(32768i32);
pub const DVD_PARENTAL_LEVEL_7: DVD_PARENTAL_LEVEL = DVD_PARENTAL_LEVEL(16384i32);
pub const DVD_PARENTAL_LEVEL_6: DVD_PARENTAL_LEVEL = DVD_PARENTAL_LEVEL(8192i32);
pub const DVD_PARENTAL_LEVEL_5: DVD_PARENTAL_LEVEL = DVD_PARENTAL_LEVEL(4096i32);
pub const DVD_PARENTAL_LEVEL_4: DVD_PARENTAL_LEVEL = DVD_PARENTAL_LEVEL(2048i32);
pub const DVD_PARENTAL_LEVEL_3: DVD_PARENTAL_LEVEL = DVD_PARENTAL_LEVEL(1024i32);
pub const DVD_PARENTAL_LEVEL_2: DVD_PARENTAL_LEVEL = DVD_PARENTAL_LEVEL(512i32);
pub const DVD_PARENTAL_LEVEL_1: DVD_PARENTAL_LEVEL = DVD_PARENTAL_LEVEL(256i32);
#[repr(transparent)]
pub struct DVD_PB_STOPPED(pub i32);
pub const DVD_PB_STOPPED_Other: DVD_PB_STOPPED = DVD_PB_STOPPED(0i32);
pub const DVD_PB_STOPPED_NoBranch: DVD_PB_STOPPED = DVD_PB_STOPPED(1i32);
pub const DVD_PB_STOPPED_NoFirstPlayDomain: DVD_PB_STOPPED = DVD_PB_STOPPED(2i32);
pub const DVD_PB_STOPPED_StopCommand: DVD_PB_STOPPED = DVD_PB_STOPPED(3i32);
pub const DVD_PB_STOPPED_Reset: DVD_PB_STOPPED = DVD_PB_STOPPED(4i32);
pub const DVD_PB_STOPPED_DiscEjected: DVD_PB_STOPPED = DVD_PB_STOPPED(5i32);
pub const DVD_PB_STOPPED_IllegalNavCommand: DVD_PB_STOPPED = DVD_PB_STOPPED(6i32);
pub const DVD_PB_STOPPED_PlayPeriodAutoStop: DVD_PB_STOPPED = DVD_PB_STOPPED(7i32);
pub const DVD_PB_STOPPED_PlayChapterAutoStop: DVD_PB_STOPPED = DVD_PB_STOPPED(8i32);
pub const DVD_PB_STOPPED_ParentalFailure: DVD_PB_STOPPED = DVD_PB_STOPPED(9i32);
pub const DVD_PB_STOPPED_RegionFailure: DVD_PB_STOPPED = DVD_PB_STOPPED(10i32);
pub const DVD_PB_STOPPED_MacrovisionFailure: DVD_PB_STOPPED = DVD_PB_STOPPED(11i32);
pub const DVD_PB_STOPPED_DiscReadError: DVD_PB_STOPPED = DVD_PB_STOPPED(12i32);
pub const DVD_PB_STOPPED_CopyProtectFailure: DVD_PB_STOPPED = DVD_PB_STOPPED(13i32);
pub const DVD_PB_STOPPED_CopyProtectOutputFailure: DVD_PB_STOPPED = DVD_PB_STOPPED(14i32);
pub const DVD_PB_STOPPED_CopyProtectOutputNotSupported: DVD_PB_STOPPED = DVD_PB_STOPPED(15i32);
#[repr(C)]
pub struct DVD_PLAYBACK_LOCATION(i32);
#[repr(C)]
pub struct DVD_PLAYBACK_LOCATION2(i32);
#[repr(transparent)]
pub struct DVD_PLAY_DIRECTION(pub i32);
pub const DVD_DIR_FORWARD: DVD_PLAY_DIRECTION = DVD_PLAY_DIRECTION(0i32);
pub const DVD_DIR_BACKWARD: DVD_PLAY_DIRECTION = DVD_PLAY_DIRECTION(1i32);
#[repr(transparent)]
pub struct DVD_PREFERRED_DISPLAY_MODE(pub i32);
pub const DISPLAY_CONTENT_DEFAULT: DVD_PREFERRED_DISPLAY_MODE = DVD_PREFERRED_DISPLAY_MODE(0i32);
pub const DISPLAY_16x9: DVD_PREFERRED_DISPLAY_MODE = DVD_PREFERRED_DISPLAY_MODE(1i32);
pub const DISPLAY_4x3_PANSCAN_PREFERRED: DVD_PREFERRED_DISPLAY_MODE = DVD_PREFERRED_DISPLAY_MODE(2i32);
pub const DISPLAY_4x3_LETTERBOX_PREFERRED: DVD_PREFERRED_DISPLAY_MODE = DVD_PREFERRED_DISPLAY_MODE(3i32);
#[repr(C)]
pub struct DVD_REGION(i32);
#[repr(transparent)]
pub struct DVD_RELATIVE_BUTTON(pub i32);
pub const DVD_Relative_Upper: DVD_RELATIVE_BUTTON = DVD_RELATIVE_BUTTON(1i32);
pub const DVD_Relative_Lower: DVD_RELATIVE_BUTTON = DVD_RELATIVE_BUTTON(2i32);
pub const DVD_Relative_Left: DVD_RELATIVE_BUTTON = DVD_RELATIVE_BUTTON(3i32);
pub const DVD_Relative_Right: DVD_RELATIVE_BUTTON = DVD_RELATIVE_BUTTON(4i32);
pub const DVD_STREAM_DATA_CURRENT: u32 = 2048u32;
pub const DVD_STREAM_DATA_VMGM: u32 = 1024u32;
pub const DVD_STREAM_DATA_VTSM: u32 = 1025u32;
#[repr(transparent)]
pub struct DVD_SUBPICTURE_CODING(pub i32);
pub const DVD_SPCoding_RunLength: DVD_SUBPICTURE_CODING = DVD_SUBPICTURE_CODING(0i32);
pub const DVD_SPCoding_Extended: DVD_SUBPICTURE_CODING = DVD_SUBPICTURE_CODING(1i32);
pub const DVD_SPCoding_Other: DVD_SUBPICTURE_CODING = DVD_SUBPICTURE_CODING(2i32);
#[repr(transparent)]
pub struct DVD_SUBPICTURE_LANG_EXT(pub i32);
pub const DVD_SP_EXT_NotSpecified: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(0i32);
pub const DVD_SP_EXT_Caption_Normal: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(1i32);
pub const DVD_SP_EXT_Caption_Big: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(2i32);
pub const DVD_SP_EXT_Caption_Children: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(3i32);
pub const DVD_SP_EXT_CC_Normal: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(5i32);
pub const DVD_SP_EXT_CC_Big: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(6i32);
pub const DVD_SP_EXT_CC_Children: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(7i32);
pub const DVD_SP_EXT_Forced: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(9i32);
pub const DVD_SP_EXT_DirectorComments_Normal: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(13i32);
pub const DVD_SP_EXT_DirectorComments_Big: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(14i32);
pub const DVD_SP_EXT_DirectorComments_Children: DVD_SUBPICTURE_LANG_EXT = DVD_SUBPICTURE_LANG_EXT(15i32);
#[repr(transparent)]
pub struct DVD_SUBPICTURE_TYPE(pub i32);
pub const DVD_SPType_NotSpecified: DVD_SUBPICTURE_TYPE = DVD_SUBPICTURE_TYPE(0i32);
pub const DVD_SPType_Language: DVD_SUBPICTURE_TYPE = DVD_SUBPICTURE_TYPE(1i32);
pub const DVD_SPType_Other: DVD_SUBPICTURE_TYPE = DVD_SUBPICTURE_TYPE(2i32);
#[repr(C)]
pub struct DVD_SubpictureAttributes(i32);
#[repr(C)]
pub struct DVD_TIMECODE(i32);
#[repr(transparent)]
pub struct DVD_TIMECODE_FLAGS(pub i32);
pub const DVD_TC_FLAG_25fps: DVD_TIMECODE_FLAGS = DVD_TIMECODE_FLAGS(1i32);
pub const DVD_TC_FLAG_30fps: DVD_TIMECODE_FLAGS = DVD_TIMECODE_FLAGS(2i32);
pub const DVD_TC_FLAG_DropFrame: DVD_TIMECODE_FLAGS = DVD_TIMECODE_FLAGS(4i32);
pub const DVD_TC_FLAG_Interpolated: DVD_TIMECODE_FLAGS = DVD_TIMECODE_FLAGS(8i32);
#[repr(transparent)]
pub struct DVD_TITLE_APPMODE(pub i32);
pub const DVD_AppMode_Not_Specified: DVD_TITLE_APPMODE = DVD_TITLE_APPMODE(0i32);
pub const DVD_AppMode_Karaoke: DVD_TITLE_APPMODE = DVD_TITLE_APPMODE(1i32);
pub const DVD_AppMode_Other: DVD_TITLE_APPMODE = DVD_TITLE_APPMODE(3i32);
pub const DVD_TITLE_MENU: u32 = 0u32;
#[repr(transparent)]
pub struct DVD_TextCharSet(pub i32);
pub const DVD_CharSet_Unicode: DVD_TextCharSet = DVD_TextCharSet(0i32);
pub const DVD_CharSet_ISO646: DVD_TextCharSet = DVD_TextCharSet(1i32);
pub const DVD_CharSet_JIS_Roman_Kanji: DVD_TextCharSet = DVD_TextCharSet(2i32);
pub const DVD_CharSet_ISO8859_1: DVD_TextCharSet = DVD_TextCharSet(3i32);
pub const DVD_CharSet_ShiftJIS_Kanji_Roman_Katakana: DVD_TextCharSet = DVD_TextCharSet(4i32);
#[repr(transparent)]
pub struct DVD_TextStringType(pub i32);
pub const DVD_Struct_Volume: DVD_TextStringType = DVD_TextStringType(1i32);
pub const DVD_Struct_Title: DVD_TextStringType = DVD_TextStringType(2i32);
pub const DVD_Struct_ParentalID: DVD_TextStringType = DVD_TextStringType(3i32);
pub const DVD_Struct_PartOfTitle: DVD_TextStringType = DVD_TextStringType(4i32);
pub const DVD_Struct_Cell: DVD_TextStringType = DVD_TextStringType(5i32);
pub const DVD_Stream_Audio: DVD_TextStringType = DVD_TextStringType(16i32);
pub const DVD_Stream_Subpicture: DVD_TextStringType = DVD_TextStringType(17i32);
pub const DVD_Stream_Angle: DVD_TextStringType = DVD_TextStringType(18i32);
pub const DVD_Channel_Audio: DVD_TextStringType = DVD_TextStringType(32i32);
pub const DVD_General_Name: DVD_TextStringType = DVD_TextStringType(48i32);
pub const DVD_General_Comments: DVD_TextStringType = DVD_TextStringType(49i32);
pub const DVD_Title_Series: DVD_TextStringType = DVD_TextStringType(56i32);
pub const DVD_Title_Movie: DVD_TextStringType = DVD_TextStringType(57i32);
pub const DVD_Title_Video: DVD_TextStringType = DVD_TextStringType(58i32);
pub const DVD_Title_Album: DVD_TextStringType = DVD_TextStringType(59i32);
pub const DVD_Title_Song: DVD_TextStringType = DVD_TextStringType(60i32);
pub const DVD_Title_Other: DVD_TextStringType = DVD_TextStringType(63i32);
pub const DVD_Title_Sub_Series: DVD_TextStringType = DVD_TextStringType(64i32);
pub const DVD_Title_Sub_Movie: DVD_TextStringType = DVD_TextStringType(65i32);
pub const DVD_Title_Sub_Video: DVD_TextStringType = DVD_TextStringType(66i32);
pub const DVD_Title_Sub_Album: DVD_TextStringType = DVD_TextStringType(67i32);
pub const DVD_Title_Sub_Song: DVD_TextStringType = DVD_TextStringType(68i32);
pub const DVD_Title_Sub_Other: DVD_TextStringType = DVD_TextStringType(71i32);
pub const DVD_Title_Orig_Series: DVD_TextStringType = DVD_TextStringType(72i32);
pub const DVD_Title_Orig_Movie: DVD_TextStringType = DVD_TextStringType(73i32);
pub const DVD_Title_Orig_Video: DVD_TextStringType = DVD_TextStringType(74i32);
pub const DVD_Title_Orig_Album: DVD_TextStringType = DVD_TextStringType(75i32);
pub const DVD_Title_Orig_Song: DVD_TextStringType = DVD_TextStringType(76i32);
pub const DVD_Title_Orig_Other: DVD_TextStringType = DVD_TextStringType(79i32);
pub const DVD_Other_Scene: DVD_TextStringType = DVD_TextStringType(80i32);
pub const DVD_Other_Cut: DVD_TextStringType = DVD_TextStringType(81i32);
pub const DVD_Other_Take: DVD_TextStringType = DVD_TextStringType(82i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_TitleAttributes(i32);
#[repr(transparent)]
pub struct DVD_VIDEO_COMPRESSION(pub i32);
pub const DVD_VideoCompression_Other: DVD_VIDEO_COMPRESSION = DVD_VIDEO_COMPRESSION(0i32);
pub const DVD_VideoCompression_MPEG1: DVD_VIDEO_COMPRESSION = DVD_VIDEO_COMPRESSION(1i32);
pub const DVD_VideoCompression_MPEG2: DVD_VIDEO_COMPRESSION = DVD_VIDEO_COMPRESSION(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_VideoAttributes(i32);
#[repr(transparent)]
pub struct DVD_WARNING(pub i32);
pub const DVD_WARNING_InvalidDVD1_0Disc: DVD_WARNING = DVD_WARNING(1i32);
pub const DVD_WARNING_FormatNotSupported: DVD_WARNING = DVD_WARNING(2i32);
pub const DVD_WARNING_IllegalNavCommand: DVD_WARNING = DVD_WARNING(3i32);
pub const DVD_WARNING_Open: DVD_WARNING = DVD_WARNING(4i32);
pub const DVD_WARNING_Seek: DVD_WARNING = DVD_WARNING(5i32);
pub const DVD_WARNING_Read: DVD_WARNING = DVD_WARNING(6i32);
#[repr(C)]
pub struct DVINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVR_STREAM_DESC(i32);
pub const DWORD_ALLPARAMS: i32 = -1i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
#[repr(C)]
pub struct DXVA2SW_CALLBACKS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Etw"))]
#[repr(C)]
pub struct DXVA2TraceVideoProcessBltData(i32);
pub const DXVA2Trace_Control: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2688052853, data2: 63244, data3: 17996, data4: [169, 206, 51, 196, 78, 9, 22, 35] };
pub const DXVA2Trace_DecodeDevBeginFrame: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2681318646, data2: 17611, data3: 17975, data4: [188, 98, 44, 17, 169, 96, 143, 144] };
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Etw"))]
#[repr(C)]
pub struct DXVA2Trace_DecodeDevBeginFrameData(i32);
pub const DXVA2Trace_DecodeDevCreated: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3034453921,
    data2: 50610,
    data3: 17662,
    data4: [134, 213, 217, 122, 100, 129, 20, 255],
};
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Etw"))]
#[repr(C)]
pub struct DXVA2Trace_DecodeDevCreatedData(i32);
pub const DXVA2Trace_DecodeDevDestroyed: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2235481586,
    data2: 16736,
    data3: 16925,
    data4: [136, 147, 99, 220, 234, 79, 24, 187],
};
pub const DXVA2Trace_DecodeDevEndFrame: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2679360307,
    data2: 18396,
    data3: 18585,
    data4: [152, 200, 192, 198, 205, 124, 211, 203],
};
pub const DXVA2Trace_DecodeDevExecute: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2232085324, data2: 53658, data3: 17929, data4: [179, 180, 188, 191, 14, 34, 18, 30] };
pub const DXVA2Trace_DecodeDevGetBuffer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1471228155, data2: 29387, data3: 16695, data4: [165, 117, 217, 31, 163, 22, 8, 151] };
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Etw"))]
#[repr(C)]
pub struct DXVA2Trace_DecodeDevGetBufferData(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Etw"))]
#[repr(C)]
pub struct DXVA2Trace_DecodeDeviceData(i32);
pub const DXVA2Trace_VideoProcessBlt: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1762172096, data2: 29099, data3: 17104, data4: [149, 58, 40, 135, 191, 5, 168, 175] };
pub const DXVA2Trace_VideoProcessDevCreated: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2304051398,
    data2: 21517,
    data3: 19591,
    data4: [152, 248, 141, 203, 242, 218, 187, 42],
};
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Etw"))]
#[repr(C)]
pub struct DXVA2Trace_VideoProcessDevCreatedData(i32);
pub const DXVA2Trace_VideoProcessDevDestroyed: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4185862321,
    data2: 64329,
    data3: 17095,
    data4: [142, 232, 136, 189, 250, 146, 212, 226],
};
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Etw"))]
#[repr(C)]
pub struct DXVA2Trace_VideoProcessDeviceData(i32);
#[repr(transparent)]
pub struct DXVA2_DestinationFlags(pub i32);
pub const DXVA2_DestinationFlag_Background_Changed: DXVA2_DestinationFlags = DXVA2_DestinationFlags(1i32);
pub const DXVA2_DestinationFlag_TargetRect_Changed: DXVA2_DestinationFlags = DXVA2_DestinationFlags(2i32);
pub const DXVA2_DestinationFlag_ColorData_Changed: DXVA2_DestinationFlags = DXVA2_DestinationFlags(4i32);
pub const DXVA2_DestinationFlag_Alpha_Changed: DXVA2_DestinationFlags = DXVA2_DestinationFlags(8i32);
pub const DXVA2_DestinationFlag_RFF: DXVA2_DestinationFlags = DXVA2_DestinationFlags(65536i32);
pub const DXVA2_DestinationFlag_TFF: DXVA2_DestinationFlags = DXVA2_DestinationFlags(131072i32);
pub const DXVA2_DestinationFlag_RFF_TFF_Present: DXVA2_DestinationFlags = DXVA2_DestinationFlags(262144i32);
pub const DXVA2_DestinationFlagMask: DXVA2_DestinationFlags = DXVA2_DestinationFlags(-65521i32);
#[repr(transparent)]
pub struct DXVA2_SampleFlags(pub i32);
pub const DXVA2_SampleFlag_Palette_Changed: DXVA2_SampleFlags = DXVA2_SampleFlags(1i32);
pub const DXVA2_SampleFlag_SrcRect_Changed: DXVA2_SampleFlags = DXVA2_SampleFlags(2i32);
pub const DXVA2_SampleFlag_DstRect_Changed: DXVA2_SampleFlags = DXVA2_SampleFlags(4i32);
pub const DXVA2_SampleFlag_ColorData_Changed: DXVA2_SampleFlags = DXVA2_SampleFlags(8i32);
pub const DXVA2_SampleFlag_PlanarAlpha_Changed: DXVA2_SampleFlags = DXVA2_SampleFlags(16i32);
pub const DXVA2_SampleFlag_RFF: DXVA2_SampleFlags = DXVA2_SampleFlags(65536i32);
pub const DXVA2_SampleFlag_TFF: DXVA2_SampleFlags = DXVA2_SampleFlags(131072i32);
pub const DXVA2_SampleFlag_RFF_TFF_Present: DXVA2_SampleFlags = DXVA2_SampleFlags(262144i32);
pub const DXVA2_SampleFlagsMask: DXVA2_SampleFlags = DXVA2_SampleFlags(-65505i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
#[repr(C)]
pub struct DXVA2_VIDEOPROCESSBLT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
#[repr(C)]
pub struct DXVA2_VIDEOSAMPLE(i32);
pub const DXVA_ALPHA_BLEND_COMBINATION_BUFFER: u32 = 13u32;
pub const DXVA_ALPHA_BLEND_COMBINATION_FUNCTION: u32 = 3u32;
pub const DXVA_ALPHA_BLEND_DATA_LOAD_FUNCTION: u32 = 2u32;
pub const DXVA_AYUV_BUFFER: u32 = 8u32;
pub const DXVA_BIDIRECTIONAL_AVERAGING_H263_TRUNC: u32 = 1u32;
pub const DXVA_BIDIRECTIONAL_AVERAGING_MPEG2_ROUND: u32 = 0u32;
pub const DXVA_BITSTREAM_CONCEALMENT_METHOD_BACKWARD: u32 = 3u32;
pub const DXVA_BITSTREAM_CONCEALMENT_METHOD_FORWARD: u32 = 2u32;
pub const DXVA_BITSTREAM_CONCEALMENT_METHOD_INTRA: u32 = 1u32;
pub const DXVA_BITSTREAM_CONCEALMENT_METHOD_UNSPECIFIED: u32 = 0u32;
pub const DXVA_BITSTREAM_CONCEALMENT_NEED_LIKELY: u32 = 2u32;
pub const DXVA_BITSTREAM_CONCEALMENT_NEED_MILD: u32 = 1u32;
pub const DXVA_BITSTREAM_CONCEALMENT_NEED_SEVERE: u32 = 3u32;
pub const DXVA_BITSTREAM_CONCEALMENT_NEED_UNLIKELY: u32 = 0u32;
pub const DXVA_BITSTREAM_DATA_BUFFER: u32 = 7u32;
pub const DXVA_CHROMA_FORMAT_420: u32 = 1u32;
pub const DXVA_CHROMA_FORMAT_422: u32 = 2u32;
pub const DXVA_CHROMA_FORMAT_444: u32 = 3u32;
pub const DXVA_COMPBUFFER_TYPE_THAT_IS_NOT_USED: u32 = 0u32;
pub const DXVA_CONFIG_BLEND_TYPE_BACK_HARDWARE: u32 = 1u32;
pub const DXVA_CONFIG_BLEND_TYPE_FRONT_BUFFER: u32 = 0u32;
pub const DXVA_CONFIG_DATA_TYPE_AI44: u32 = 1u32;
pub const DXVA_CONFIG_DATA_TYPE_AYUV: u32 = 3u32;
pub const DXVA_CONFIG_DATA_TYPE_DPXD: u32 = 2u32;
pub const DXVA_CONFIG_DATA_TYPE_IA44: u32 = 0u32;
pub const DXVA_COPPCommandFnCode: u32 = 4u32;
pub const DXVA_COPPDevice: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3527768797,
    data2: 35225,
    data3: 17901,
    data4: [138, 138, 209, 170, 4, 123, 164, 213],
};
pub const DXVA_COPPGetCertificateLengthFnCode: u32 = 1u32;
pub const DXVA_COPPKeyExchangeFnCode: u32 = 2u32;
pub const DXVA_COPPQueryBusData: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3337934451, data2: 24948, data3: 16772, data4: [142, 53, 246, 219, 82, 0, 188, 186] };
pub const DXVA_COPPQueryConnectorType: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2177941461,
    data2: 27390,
    data3: 18626,
    data4: [153, 192, 149, 160, 143, 151, 197, 218],
};
pub const DXVA_COPPQueryDisplayData: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3619625891, data2: 44307, data3: 20366, data4: [175, 152, 13, 203, 60, 162, 4, 204] };
pub const DXVA_COPPQueryGlobalProtectionLevel: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 425140490,
    data2: 30566,
    data3: 17706,
    data4: [185, 154, 210, 122, 237, 84, 240, 58],
};
pub const DXVA_COPPQueryHDCPKeyData: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 230006132, data2: 43410, data3: 18734, data4: [160, 189, 194, 63, 218, 86, 78, 0] };
pub const DXVA_COPPQueryLocalProtectionLevel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2986825815, data2: 16090, data3: 19805, data4: [136, 219, 116, 143, 140, 26, 5, 73] };
pub const DXVA_COPPQueryProtectionType: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 955426817, data2: 39532, data3: 18619, data4: [145, 7, 182, 105, 110, 111, 23, 151] };
pub const DXVA_COPPQuerySignaling: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1714005393,
    data2: 15225,
    data3: 19699,
    data4: [146, 74, 17, 232, 231, 129, 22, 113],
};
pub const DXVA_COPPQueryStatusFnCode: u32 = 5u32;
pub const DXVA_COPPSequenceStartFnCode: u32 = 3u32;
pub const DXVA_COPPSetProtectionLevel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2612605564, data2: 20149, data3: 18215, data4: [159, 0, 180, 43, 9, 25, 192, 218] };
#[repr(C)]
pub struct DXVA_COPPSetProtectionLevelCmdData(i32);
pub const DXVA_COPPSetSignaling: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 161886629, data2: 54916, data3: 19552, data4: [142, 77, 211, 187, 15, 11, 227, 238] };
#[repr(C)]
pub struct DXVA_COPPSetSignalingCmdData(i32);
#[repr(C)]
pub struct DXVA_COPPStatusData(i32);
#[repr(C)]
pub struct DXVA_COPPStatusDisplayData(i32);
#[repr(C)]
pub struct DXVA_COPPStatusHDCPKeyData(i32);
#[repr(C)]
pub struct DXVA_COPPStatusSignalingCmdData(i32);
pub const DXVA_DCCMD_SURFACE_BUFFER: u32 = 12u32;
pub const DXVA_DEBLOCKING_CONTROL_BUFFER: u32 = 4u32;
pub const DXVA_DEBLOCKING_FILTER_FUNCTION: u32 = 5u32;
pub const DXVA_DPXD_SURFACE_BUFFER: u32 = 10u32;
pub const DXVA_DeinterlaceBltExFnCode: u32 = 2u32;
pub const DXVA_DeinterlaceBltFnCode: u32 = 1u32;
pub const DXVA_DeinterlaceBobDevice: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 861578094,
    data2: 30852,
    data3: 17316,
    data4: [156, 145, 127, 135, 250, 243, 227, 126],
};
pub const DXVA_DeinterlaceContainerDevice: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 243649427,
    data2: 12358,
    data3: 20464,
    data4: [174, 204, 213, 140, 181, 240, 53, 253],
};
pub const DXVA_DeinterlaceQueryAvailableModesFnCode: u32 = 1u32;
pub const DXVA_DeinterlaceQueryModeCapsFnCode: u32 = 2u32;
pub const DXVA_ENCRYPTPROTOCOLFUNCFLAG_ACCEL: u32 = 16776968u32;
pub const DXVA_ENCRYPTPROTOCOLFUNCFLAG_HOST: u32 = 16776960u32;
pub const DXVA_EXECUTE_RETURN_DATA_ERROR_MINOR: u32 = 1u32;
pub const DXVA_EXECUTE_RETURN_DATA_ERROR_SEVERE: u32 = 3u32;
pub const DXVA_EXECUTE_RETURN_DATA_ERROR_SIGNIF: u32 = 2u32;
pub const DXVA_EXECUTE_RETURN_OK: u32 = 0u32;
pub const DXVA_EXECUTE_RETURN_OTHER_ERROR_SEVERE: u32 = 4u32;
pub const DXVA_ExtColorData_ShiftBase: u32 = 8u32;
pub const DXVA_FILM_GRAIN_BUFFER: u32 = 17u32;
pub const DXVA_FILM_GRAIN_SYNTHESIS_FUNCTION: u32 = 6u32;
pub const DXVA_HIGHLIGHT_BUFFER: u32 = 11u32;
pub const DXVA_IA44_SURFACE_BUFFER: u32 = 9u32;
pub const DXVA_INVERSE_QUANTIZATION_MATRIX_BUFFER: u32 = 5u32;
pub const DXVA_MACROBLOCK_CONTROL_BUFFER: u32 = 2u32;
pub const DXVA_MOTION_VECTOR_BUFFER: u32 = 16u32;
pub const DXVA_MV_PRECISION_AND_CHROMA_RELATION_H261: u32 = 2u32;
pub const DXVA_MV_PRECISION_AND_CHROMA_RELATION_H263: u32 = 1u32;
pub const DXVA_MV_PRECISION_AND_CHROMA_RELATION_MPEG2: u32 = 0u32;
pub const DXVA_ModeAV1_VLD_12bit_Profile2: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 387084297,
    data2: 40975,
    data3: 19681,
    data4: [153, 78, 191, 64, 129, 246, 243, 240],
};
pub const DXVA_ModeAV1_VLD_12bit_Profile2_420: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 763412182,
    data2: 40108,
    data3: 18485,
    data4: [158, 145, 50, 123, 188, 79, 158, 232],
};
pub const DXVA_ModeAV1_VLD_Profile0: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3099479243,
    data2: 53075,
    data3: 18106,
    data4: [141, 89, 214, 184, 166, 218, 93, 42],
};
pub const DXVA_ModeAV1_VLD_Profile1: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1765211919,
    data2: 17841,
    data3: 16739,
    data4: [156, 193, 100, 110, 246, 148, 97, 8],
};
pub const DXVA_ModeAV1_VLD_Profile2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 207563425, data2: 58689, data3: 16521, data4: [187, 123, 152, 17, 10, 25, 215, 200] };
pub const DXVA_ModeH261_A: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487617, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH261_B: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487618, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH263_A: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487619, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH263_B: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487620, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH263_C: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487621, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH263_D: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487622, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH263_E: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487623, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH263_F: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487624, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH264_A: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487716, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH264_B: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487717, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH264_C: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487718, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH264_D: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487719, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH264_E: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487720, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH264_F: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487721, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeH264_VLD_Multiview_NoFGT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1885052290,
    data2: 30415,
    data3: 18902,
    data4: [183, 230, 172, 136, 114, 219, 1, 60],
};
pub const DXVA_ModeH264_VLD_Stereo_NoFGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4188720315, data2: 49846, data3: 19708, data4: [135, 121, 87, 7, 177, 118, 5, 82] };
pub const DXVA_ModeH264_VLD_Stereo_Progressive_NoFGT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3617319130, data2: 3313, data3: 19585, data4: [184, 42, 105, 164, 226, 54, 244, 61] };
pub const DXVA_ModeH264_VLD_WithFMOASO_NoFGT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3589296121,
    data2: 13336,
    data3: 17880,
    data4: [149, 97, 50, 167, 106, 174, 45, 221],
};
pub const DXVA_ModeHEVC_VLD_Main: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1527895323, data2: 12108, data3: 17490, data4: [188, 195, 9, 242, 161, 22, 12, 192] };
pub const DXVA_ModeHEVC_VLD_Main10: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 276492512, data2: 61210, data3: 19737, data4: [171, 168, 103, 161, 99, 7, 61, 19] };
pub const DXVA_ModeMPEG1_A: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487625, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeMPEG1_VLD: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1866385177,
    data2: 14133,
    data3: 17100,
    data4: [128, 99, 101, 204, 60, 179, 102, 22],
};
pub const DXVA_ModeMPEG2_A: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487626, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeMPEG2_B: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487627, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeMPEG2_C: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487628, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeMPEG2_D: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487629, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeMPEG2and1_VLD: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2255052562,
    data2: 13326,
    data3: 20228,
    data4: [159, 211, 146, 83, 221, 50, 116, 96],
};
pub const DXVA_ModeMPEG4pt2_VLD_AdvSimple_GMC: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2878966619,
    data2: 16984,
    data3: 17577,
    data4: [159, 235, 148, 229, 151, 166, 186, 174],
};
pub const DXVA_ModeMPEG4pt2_VLD_AdvSimple_NoGMC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3980495519, data2: 269, data3: 20186, data4: [154, 227, 154, 101, 53, 141, 141, 46] };
pub const DXVA_ModeMPEG4pt2_VLD_Simple: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4023799156,
    data2: 51688,
    data3: 16855,
    data4: [165, 233, 233, 176, 227, 159, 163, 25],
};
pub const DXVA_ModeNone: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487616, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeVC1_A: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487776, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeVC1_B: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487777, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeVC1_C: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487778, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeVC1_D: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487779, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeVC1_D2010: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487780, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeVP8_VLD: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2428017130,
    data2: 14946,
    data3: 18181,
    data4: [136, 179, 141, 240, 75, 39, 68, 231],
};
pub const DXVA_ModeVP9_VLD_10bit_Profile2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2764524015, data2: 28367, data3: 18602, data4: [132, 72, 80, 167, 161, 22, 95, 247] };
pub const DXVA_ModeVP9_VLD_Profile0: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1178011640,
    data2: 41424,
    data3: 17797,
    data4: [135, 109, 131, 170, 109, 96, 184, 158],
};
pub const DXVA_ModeWMV8_A: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487744, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeWMV8_B: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487745, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeWMV9_A: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487760, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeWMV9_B: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487761, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_ModeWMV9_C: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487764, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_NUM_TYPES_COMP_BUFFERS: u32 = 18u32;
pub const DXVA_NoEncrypt: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 461487824, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const DXVA_NumMV_OBMC_off_BinPBwith4MV_off: u32 = 4u32;
pub const DXVA_NumMV_OBMC_off_BinPBwith4MV_on: u32 = 5u32;
pub const DXVA_NumMV_OBMC_on__BinPB_off: u32 = 10u32;
pub const DXVA_NumMV_OBMC_on__BinPB_on: u32 = 11u32;
pub const DXVA_PICTURE_DECODE_BUFFER: u32 = 1u32;
pub const DXVA_PICTURE_DECODING_FUNCTION: u32 = 1u32;
pub const DXVA_PICTURE_RESAMPLE_BUFFER: u32 = 14u32;
pub const DXVA_PICTURE_RESAMPLE_FUNCTION: u32 = 4u32;
pub const DXVA_PICTURE_STRUCTURE_BOTTOM_FIELD: u32 = 2u32;
pub const DXVA_PICTURE_STRUCTURE_FRAME: u32 = 3u32;
pub const DXVA_PICTURE_STRUCTURE_TOP_FIELD: u32 = 1u32;
pub const DXVA_ProcAmpControlBltFnCode: u32 = 1u32;
pub const DXVA_ProcAmpControlDevice: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2669676819, data2: 12285, data3: 16470, data4: [159, 30, 225, 181, 8, 242, 45, 207] };
pub const DXVA_ProcAmpControlQueryCapsFnCode: u32 = 3u32;
pub const DXVA_ProcAmpControlQueryRangeFnCode: u32 = 4u32;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_LOCK_FALSE_PLUS: u32 = 16777215u32;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_LOCK_OK_COPY: u32 = 16777212u32;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_FALSE_PLUS: u32 = 16777211u32;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_OK_COPY: u32 = 16777208u32;
pub const DXVA_QUERYORREPLYFUNCFLAG_ACCEL_PROBE_OK_PLUS: u32 = 16777209u32;
pub const DXVA_QUERYORREPLYFUNCFLAG_DECODER_LOCK_QUERY: u32 = 16777205u32;
pub const DXVA_QUERYORREPLYFUNCFLAG_DECODER_PROBE_QUERY: u32 = 16777201u32;
pub const DXVA_READ_BACK_BUFFER: u32 = 15u32;
pub const DXVA_RESIDUAL_DIFFERENCE_BUFFER: u32 = 3u32;
pub const DXVA_RESTRICTED_MODE_H261_A: u32 = 1u32;
pub const DXVA_RESTRICTED_MODE_H261_B: u32 = 2u32;
pub const DXVA_RESTRICTED_MODE_H263_A: u32 = 3u32;
pub const DXVA_RESTRICTED_MODE_H263_B: u32 = 4u32;
pub const DXVA_RESTRICTED_MODE_H263_C: u32 = 5u32;
pub const DXVA_RESTRICTED_MODE_H263_D: u32 = 6u32;
pub const DXVA_RESTRICTED_MODE_H263_E: u32 = 7u32;
pub const DXVA_RESTRICTED_MODE_H263_F: u32 = 8u32;
pub const DXVA_RESTRICTED_MODE_H264_A: u32 = 100u32;
pub const DXVA_RESTRICTED_MODE_H264_B: u32 = 101u32;
pub const DXVA_RESTRICTED_MODE_H264_C: u32 = 102u32;
pub const DXVA_RESTRICTED_MODE_H264_D: u32 = 103u32;
pub const DXVA_RESTRICTED_MODE_H264_E: u32 = 104u32;
pub const DXVA_RESTRICTED_MODE_H264_F: u32 = 105u32;
pub const DXVA_RESTRICTED_MODE_H264_IDCT_FGT: u32 = 103u32;
pub const DXVA_RESTRICTED_MODE_H264_IDCT_NOFGT: u32 = 102u32;
pub const DXVA_RESTRICTED_MODE_H264_MOCOMP_FGT: u32 = 101u32;
pub const DXVA_RESTRICTED_MODE_H264_MOCOMP_NOFGT: u32 = 100u32;
pub const DXVA_RESTRICTED_MODE_H264_VLD_FGT: u32 = 105u32;
pub const DXVA_RESTRICTED_MODE_H264_VLD_MULTIVIEW_NOFGT: u32 = 115u32;
pub const DXVA_RESTRICTED_MODE_H264_VLD_NOFGT: u32 = 104u32;
pub const DXVA_RESTRICTED_MODE_H264_VLD_STEREO_NOFGT: u32 = 114u32;
pub const DXVA_RESTRICTED_MODE_H264_VLD_STEREO_PROGRESSIVE_NOFGT: u32 = 113u32;
pub const DXVA_RESTRICTED_MODE_H264_VLD_WITHFMOASO_NOFGT: u32 = 112u32;
pub const DXVA_RESTRICTED_MODE_MPEG1_A: u32 = 9u32;
pub const DXVA_RESTRICTED_MODE_MPEG1_VLD: u32 = 16u32;
pub const DXVA_RESTRICTED_MODE_MPEG2_A: u32 = 10u32;
pub const DXVA_RESTRICTED_MODE_MPEG2_B: u32 = 11u32;
pub const DXVA_RESTRICTED_MODE_MPEG2_C: u32 = 12u32;
pub const DXVA_RESTRICTED_MODE_MPEG2_D: u32 = 13u32;
pub const DXVA_RESTRICTED_MODE_MPEG2and1_VLD: u32 = 17u32;
pub const DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_ADV_SIMPLE_GMC: u32 = 178u32;
pub const DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_ADV_SIMPLE_NOGMC: u32 = 177u32;
pub const DXVA_RESTRICTED_MODE_MPEG4PT2_VLD_SIMPLE: u32 = 176u32;
pub const DXVA_RESTRICTED_MODE_UNRESTRICTED: u32 = 65535u32;
pub const DXVA_RESTRICTED_MODE_VC1_A: u32 = 160u32;
pub const DXVA_RESTRICTED_MODE_VC1_B: u32 = 161u32;
pub const DXVA_RESTRICTED_MODE_VC1_C: u32 = 162u32;
pub const DXVA_RESTRICTED_MODE_VC1_D: u32 = 163u32;
pub const DXVA_RESTRICTED_MODE_VC1_D2010: u32 = 164u32;
pub const DXVA_RESTRICTED_MODE_VC1_IDCT: u32 = 162u32;
pub const DXVA_RESTRICTED_MODE_VC1_MOCOMP: u32 = 161u32;
pub const DXVA_RESTRICTED_MODE_VC1_POSTPROC: u32 = 160u32;
pub const DXVA_RESTRICTED_MODE_VC1_VLD: u32 = 163u32;
pub const DXVA_RESTRICTED_MODE_WMV8_A: u32 = 128u32;
pub const DXVA_RESTRICTED_MODE_WMV8_B: u32 = 129u32;
pub const DXVA_RESTRICTED_MODE_WMV8_MOCOMP: u32 = 129u32;
pub const DXVA_RESTRICTED_MODE_WMV8_POSTPROC: u32 = 128u32;
pub const DXVA_RESTRICTED_MODE_WMV9_A: u32 = 144u32;
pub const DXVA_RESTRICTED_MODE_WMV9_B: u32 = 145u32;
pub const DXVA_RESTRICTED_MODE_WMV9_C: u32 = 148u32;
pub const DXVA_RESTRICTED_MODE_WMV9_IDCT: u32 = 148u32;
pub const DXVA_RESTRICTED_MODE_WMV9_MOCOMP: u32 = 145u32;
pub const DXVA_RESTRICTED_MODE_WMV9_POSTPROC: u32 = 144u32;
pub const DXVA_SCAN_METHOD_ALTERNATE_HORIZONTAL: u32 = 2u32;
pub const DXVA_SCAN_METHOD_ALTERNATE_VERTICAL: u32 = 1u32;
pub const DXVA_SCAN_METHOD_ARBITRARY: u32 = 3u32;
pub const DXVA_SCAN_METHOD_ZIG_ZAG: u32 = 0u32;
pub const DXVA_SLICE_CONTROL_BUFFER: u32 = 6u32;
pub const DXVA_STATUS_REPORTING_FUNCTION: u32 = 7u32;
pub const DXVA_USUAL_BLOCK_HEIGHT: u32 = 8u32;
pub const DXVA_USUAL_BLOCK_WIDTH: u32 = 8u32;
#[repr(C)]
pub struct DigitalCableLocator(i32);
#[repr(C)]
pub struct DigitalCableTuneRequest(i32);
#[repr(C)]
pub struct DigitalCableTuningSpace(i32);
#[repr(C)]
pub struct DigitalLocator(i32);
#[repr(transparent)]
pub struct DisplaySizeList(pub i32);
pub const dslDefaultSize: DisplaySizeList = DisplaySizeList(0i32);
pub const dslSourceSize: DisplaySizeList = DisplaySizeList(0i32);
pub const dslHalfSourceSize: DisplaySizeList = DisplaySizeList(1i32);
pub const dslDoubleSourceSize: DisplaySizeList = DisplaySizeList(2i32);
pub const dslFullScreen: DisplaySizeList = DisplaySizeList(3i32);
pub const dslHalfScreen: DisplaySizeList = DisplaySizeList(4i32);
pub const dslQuarterScreen: DisplaySizeList = DisplaySizeList(5i32);
pub const dslSixteenthScreen: DisplaySizeList = DisplaySizeList(6i32);
#[repr(transparent)]
pub struct DownResEventParam(pub i32);
pub const DOWNRES_Always: DownResEventParam = DownResEventParam(0i32);
pub const DOWNRES_InWindowOnly: DownResEventParam = DownResEventParam(1i32);
pub const DOWNRES_Undefined: DownResEventParam = DownResEventParam(2i32);
#[repr(C)]
pub struct DualMonoInfo(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DvbParentalRatingDescriptor(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DvbParentalRatingParam(i32);
#[repr(C)]
pub struct EALocationCodeType(i32);
#[repr(C)]
pub struct ECHOSTAR_SATELLITE_TV_NETWORK_TYPE(i32);
pub const EC_ACTIVATE: u32 = 19u32;
pub const EC_BANDWIDTHCHANGE: u32 = 72u32;
pub const EC_BUFFERING_DATA: u32 = 17u32;
pub const EC_BUILT: u32 = 768u32;
pub const EC_CLOCK_CHANGED: u32 = 13u32;
pub const EC_CLOCK_UNSET: u32 = 81u32;
pub const EC_CODECAPI_EVENT: u32 = 87u32;
pub const EC_COMPLETE: u32 = 1u32;
pub const EC_CONTENTPROPERTY_CHANGED: u32 = 71u32;
pub const EC_DEVICE_LOST: u32 = 31u32;
pub const EC_DISPLAY_CHANGED: u32 = 22u32;
pub const EC_DVDBASE: u32 = 256u32;
pub const EC_DVD_ANGLES_AVAILABLE: u32 = 275u32;
pub const EC_DVD_ANGLE_CHANGE: u32 = 262u32;
pub const EC_DVD_AUDIO_STREAM_CHANGE: u32 = 260u32;
pub const EC_DVD_BUTTON_AUTO_ACTIVATED: u32 = 277u32;
pub const EC_DVD_BUTTON_CHANGE: u32 = 263u32;
pub const EC_DVD_BeginNavigationCommands: u32 = 291u32;
pub const EC_DVD_CHAPTER_AUTOSTOP: u32 = 270u32;
pub const EC_DVD_CHAPTER_START: u32 = 259u32;
pub const EC_DVD_CMD_END: u32 = 279u32;
pub const EC_DVD_CMD_START: u32 = 278u32;
pub const EC_DVD_CURRENT_HMSF_TIME: u32 = 282u32;
pub const EC_DVD_CURRENT_TIME: u32 = 267u32;
pub const EC_DVD_DISC_EJECTED: u32 = 280u32;
pub const EC_DVD_DISC_INSERTED: u32 = 281u32;
pub const EC_DVD_DOMAIN_CHANGE: u32 = 257u32;
pub const EC_DVD_ERROR: u32 = 268u32;
pub const EC_DVD_GPRM_Change: u32 = 289u32;
pub const EC_DVD_KARAOKE_MODE: u32 = 283u32;
pub const EC_DVD_NO_FP_PGC: u32 = 271u32;
pub const EC_DVD_NavigationCommand: u32 = 292u32;
pub const EC_DVD_PARENTAL_LEVEL_CHANGE: u32 = 273u32;
pub const EC_DVD_PLAYBACK_RATE_CHANGE: u32 = 272u32;
pub const EC_DVD_PLAYBACK_STOPPED: u32 = 274u32;
pub const EC_DVD_PLAYPERIOD_AUTOSTOP: u32 = 276u32;
pub const EC_DVD_PROGRAM_CELL_CHANGE: u32 = 284u32;
pub const EC_DVD_PROGRAM_CHAIN_CHANGE: u32 = 286u32;
pub const EC_DVD_SPRM_Change: u32 = 290u32;
pub const EC_DVD_STILL_OFF: u32 = 266u32;
pub const EC_DVD_STILL_ON: u32 = 265u32;
pub const EC_DVD_SUBPICTURE_STREAM_CHANGE: u32 = 261u32;
pub const EC_DVD_TITLE_CHANGE: u32 = 258u32;
pub const EC_DVD_TITLE_SET_CHANGE: u32 = 285u32;
pub const EC_DVD_VALID_UOPS_CHANGE: u32 = 264u32;
pub const EC_DVD_VOBU_Offset: u32 = 287u32;
pub const EC_DVD_VOBU_Timestamp: u32 = 288u32;
pub const EC_DVD_WARNING: u32 = 269u32;
pub const EC_END_OF_SEGMENT: u32 = 28u32;
pub const EC_EOS_SOON: u32 = 70u32;
pub const EC_ERRORABORT: u32 = 3u32;
pub const EC_ERRORABORTEX: u32 = 69u32;
pub const EC_ERROR_STILLPLAYING: u32 = 8u32;
pub const EC_EXTDEVICE_MODE_CHANGE: u32 = 49u32;
pub const EC_FILE_CLOSED: u32 = 68u32;
pub const EC_FULLSCREEN_LOST: u32 = 18u32;
pub const EC_GRAPH_CHANGED: u32 = 80u32;
pub const EC_LENGTH_CHANGED: u32 = 30u32;
pub const EC_LOADSTATUS: u32 = 67u32;
pub const EC_MARKER_HIT: u32 = 66u32;
pub const EC_NEED_RESTART: u32 = 20u32;
pub const EC_NEW_PIN: u32 = 32u32;
pub const EC_NOTIFY_WINDOW: u32 = 25u32;
pub const EC_OLE_EVENT: u32 = 24u32;
pub const EC_OPENING_FILE: u32 = 16u32;
pub const EC_PALETTE_CHANGED: u32 = 9u32;
pub const EC_PAUSED: u32 = 14u32;
pub const EC_PLEASE_REOPEN: u32 = 64u32;
pub const EC_PREPROCESS_COMPLETE: u32 = 86u32;
pub const EC_PROCESSING_LATENCY: u32 = 33u32;
pub const EC_QUALITY_CHANGE: u32 = 11u32;
pub const EC_RENDER_FINISHED: u32 = 33u32;
pub const EC_REPAINT: u32 = 5u32;
pub const EC_SAMPLE_LATENCY: u32 = 34u32;
pub const EC_SAMPLE_NEEDED: u32 = 32u32;
pub const EC_SCRUB_TIME: u32 = 35u32;
pub const EC_SEGMENT_STARTED: u32 = 29u32;
pub const EC_SHUTTING_DOWN: u32 = 12u32;
pub const EC_SKIP_FRAMES: u32 = 37u32;
pub const EC_SNDDEV_IN_ERROR: u32 = 512u32;
pub const EC_SNDDEV_OUT_ERROR: u32 = 513u32;
pub const EC_SND_DEVICE_ERROR_BASE: u32 = 512u32;
pub const EC_STARVATION: u32 = 23u32;
pub const EC_STATE_CHANGE: u32 = 50u32;
pub const EC_STATUS: u32 = 65u32;
pub const EC_STEP_COMPLETE: u32 = 36u32;
pub const EC_STREAM_CONTROL_STARTED: u32 = 27u32;
pub const EC_STREAM_CONTROL_STOPPED: u32 = 26u32;
pub const EC_STREAM_ERROR_STILLPLAYING: u32 = 7u32;
pub const EC_STREAM_ERROR_STOPPED: u32 = 6u32;
pub const EC_SYSTEMBASE: u32 = 0u32;
pub const EC_TIME: u32 = 4u32;
pub const EC_TIMECODE_AVAILABLE: u32 = 48u32;
pub const EC_UNBUILT: u32 = 769u32;
pub const EC_USER: u32 = 32768u32;
pub const EC_USERABORT: u32 = 2u32;
pub const EC_VIDEOFRAMEREADY: u32 = 73u32;
pub const EC_VIDEO_SIZE_CHANGED: u32 = 10u32;
pub const EC_VMR_RECONNECTION_FAILED: u32 = 85u32;
pub const EC_VMR_RENDERDEVICE_SET: u32 = 83u32;
pub const EC_VMR_SURFACE_FLIPPED: u32 = 84u32;
pub const EC_WINDOW_DESTROYED: u32 = 21u32;
pub const EC_WMT_EVENT: u32 = 594u32;
pub const EC_WMT_EVENT_BASE: u32 = 593u32;
pub const EC_WMT_INDEX_EVENT: u32 = 593u32;
#[repr(C)]
pub struct ESEventFactory(i32);
#[repr(C)]
pub struct ESEventService(i32);
#[repr(C)]
pub struct ETFilter(i32);
#[repr(C)]
pub struct EVENTID_ARIBcontentSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_AudioDescriptorSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_AudioTypeSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_BDAConditionalAccessTAG(i32);
#[repr(C)]
pub struct EVENTID_BDAEventingServicePendingEvent(i32);
#[repr(C)]
pub struct EVENTID_BDA_CASBroadcastMMI(i32);
#[repr(C)]
pub struct EVENTID_BDA_CASCloseMMI(i32);
#[repr(C)]
pub struct EVENTID_BDA_CASOpenMMI(i32);
#[repr(C)]
pub struct EVENTID_BDA_CASReleaseTuner(i32);
#[repr(C)]
pub struct EVENTID_BDA_CASRequestTuner(i32);
#[repr(C)]
pub struct EVENTID_BDA_DiseqCResponseAvailable(i32);
#[repr(C)]
pub struct EVENTID_BDA_EncoderSignalLock(i32);
#[repr(C)]
pub struct EVENTID_BDA_FdcStatus(i32);
#[repr(C)]
pub struct EVENTID_BDA_FdcTableSection(i32);
#[repr(C)]
pub struct EVENTID_BDA_GPNVValueUpdate(i32);
#[repr(C)]
pub struct EVENTID_BDA_GuideDataAvailable(i32);
#[repr(C)]
pub struct EVENTID_BDA_GuideDataError(i32);
#[repr(C)]
pub struct EVENTID_BDA_GuideServiceInformationUpdated(i32);
#[repr(C)]
pub struct EVENTID_BDA_IsdbCASResponse(i32);
#[repr(C)]
pub struct EVENTID_BDA_LbigsCloseConnectionHandle(i32);
#[repr(C)]
pub struct EVENTID_BDA_LbigsOpenConnection(i32);
#[repr(C)]
pub struct EVENTID_BDA_LbigsSendData(i32);
#[repr(C)]
pub struct EVENTID_BDA_RatingPinReset(i32);
#[repr(C)]
pub struct EVENTID_BDA_TransprtStreamSelectorInfo(i32);
#[repr(C)]
pub struct EVENTID_BDA_TunerNoSignal(i32);
#[repr(C)]
pub struct EVENTID_BDA_TunerSignalLock(i32);
#[repr(C)]
pub struct EVENTID_BDA_UpdateDrmStatus(i32);
#[repr(C)]
pub struct EVENTID_BDA_UpdateScanState(i32);
#[repr(C)]
pub struct EVENTID_CADenialCountChanged(i32);
#[repr(C)]
pub struct EVENTID_CASFailureSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_CSDescriptorSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_CandidatePostTuneData(i32);
#[repr(C)]
pub struct EVENTID_CardStatusChanged(i32);
#[repr(C)]
pub struct EVENTID_ChannelChangeSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_ChannelInfoSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_ChannelTypeSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_CtxADescriptorSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_DFNWithNoActualAVData(i32);
#[repr(C)]
pub struct EVENTID_DRMParingStatusChanged(i32);
#[repr(C)]
pub struct EVENTID_DRMParingStepComplete(i32);
pub const EVENTID_DTFilterCOPPBlock: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229802, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_DTFilterCOPPUnblock: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229800, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_DTFilterDataFormatFailure: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229805, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_DTFilterDataFormatOK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229804, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_DTFilterRatingChange: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229794, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_DTFilterRatingsBlock: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229795, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_DTFilterRatingsUnblock: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229796, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_DTFilterXDSPacket: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229797, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
#[repr(C)]
pub struct EVENTID_DVBScramblingControlSpanningEvent(i32);
pub const EVENTID_DemultiplexerFilterDiscontinuity: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 370497392,
    data2: 44757,
    data3: 18268,
    data4: [187, 152, 149, 163, 48, 112, 223, 12],
};
#[repr(C)]
pub struct EVENTID_DualMonoSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_DvbParentalRatingDescriptor(i32);
#[repr(C)]
pub struct EVENTID_EASMessageReceived(i32);
pub const EVENTID_ETDTFilterLicenseFailure: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229807, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_ETDTFilterLicenseOK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229806, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_ETFilterCopyNever: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229808, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_ETFilterCopyOnce: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229803, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_ETFilterEncryptionOff: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229799, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_ETFilterEncryptionOn: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229798, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
#[repr(C)]
pub struct EVENTID_EmmMessageSpanningEvent(i32);
pub const EVENTID_EncDecFilterError: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229801, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_EncDecFilterEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1243301467, data2: 4025, data3: 16729, data4: [175, 189, 227, 48, 6, 160, 249, 244] };
#[repr(C)]
pub struct EVENTID_EntitlementChanged(i32);
pub const EVENTID_FormatNotSupportedEvent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 615655434, data2: 45738, data3: 18295, data4: [191, 101, 99, 243, 94, 123, 2, 74] };
#[repr(C)]
pub struct EVENTID_LanguageSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_MMIMessage(i32);
#[repr(C)]
pub struct EVENTID_NewSignalAcquired(i32);
#[repr(C)]
pub struct EVENTID_PBDAParentalControlEvent(i32);
#[repr(C)]
pub struct EVENTID_PIDListSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_PSITable(i32);
#[repr(C)]
pub struct EVENTID_RRTSpanningEvent(i32);
pub const EVENTID_SBE2RecControlStarted: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2305206430,
    data2: 63550,
    data3: 19470,
    data4: [188, 59, 191, 167, 100, 158, 4, 203],
};
pub const EVENTID_SBE2RecControlStopped: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1162550984,
    data2: 3227,
    data3: 19626,
    data4: [177, 161, 30, 122, 38, 102, 246, 195],
};
#[repr(C)]
pub struct EVENTID_STBChannelNumber(i32);
#[repr(C)]
pub struct EVENTID_ServiceTerminated(i32);
#[repr(C)]
pub struct EVENTID_SignalAndServiceStatusSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_SignalStatusChanged(i32);
#[repr(C)]
pub struct EVENTID_StreamIDSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_StreamTypeSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_SubtitleSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_TeletextSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_TuneFailureEvent(i32);
#[repr(C)]
pub struct EVENTID_TuneFailureSpanningEvent(i32);
#[repr(C)]
pub struct EVENTID_TuningChanged(i32);
#[repr(C)]
pub struct EVENTID_TuningChanging(i32);
pub const EVENTID_XDSCodecDuplicateXDSRating: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229791, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_XDSCodecNewXDSPacket: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229793, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const EVENTID_XDSCodecNewXDSRating: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229792, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
#[repr(C)]
pub struct EVENTTYPE_CASDescrambleFailureEvent(i32);
pub const E_PROP_ID_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147023728i32 as _);
pub const E_PROP_SET_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147023726i32 as _);
#[repr(transparent)]
pub struct EnTag_Mode(pub i32);
pub const EnTag_Remove: EnTag_Mode = EnTag_Mode(0i32);
pub const EnTag_Once: EnTag_Mode = EnTag_Mode(1i32);
pub const EnTag_Repeat: EnTag_Mode = EnTag_Mode(2i32);
#[repr(transparent)]
pub struct EnTvRat_CAE_TV(pub i32);
pub const CAE_TV_Exempt: EnTvRat_CAE_TV = EnTvRat_CAE_TV(0i32);
pub const CAE_TV_C: EnTvRat_CAE_TV = EnTvRat_CAE_TV(1i32);
pub const CAE_TV_C8: EnTvRat_CAE_TV = EnTvRat_CAE_TV(2i32);
pub const CAE_TV_G: EnTvRat_CAE_TV = EnTvRat_CAE_TV(3i32);
pub const CAE_TV_PG: EnTvRat_CAE_TV = EnTvRat_CAE_TV(4i32);
pub const CAE_TV_14: EnTvRat_CAE_TV = EnTvRat_CAE_TV(5i32);
pub const CAE_TV_18: EnTvRat_CAE_TV = EnTvRat_CAE_TV(6i32);
pub const CAE_TV_Reserved: EnTvRat_CAE_TV = EnTvRat_CAE_TV(7i32);
#[repr(transparent)]
pub struct EnTvRat_CAF_TV(pub i32);
pub const CAF_TV_Exempt: EnTvRat_CAF_TV = EnTvRat_CAF_TV(0i32);
pub const CAF_TV_G: EnTvRat_CAF_TV = EnTvRat_CAF_TV(1i32);
pub const CAF_TV_8: EnTvRat_CAF_TV = EnTvRat_CAF_TV(2i32);
pub const CAF_TV_13: EnTvRat_CAF_TV = EnTvRat_CAF_TV(3i32);
pub const CAF_TV_16: EnTvRat_CAF_TV = EnTvRat_CAF_TV(4i32);
pub const CAF_TV_18: EnTvRat_CAF_TV = EnTvRat_CAF_TV(5i32);
pub const CAF_TV_Reserved6: EnTvRat_CAF_TV = EnTvRat_CAF_TV(6i32);
pub const CAF_TV_Reserved: EnTvRat_CAF_TV = EnTvRat_CAF_TV(7i32);
#[repr(transparent)]
pub struct EnTvRat_GenericLevel(pub i32);
pub const TvRat_0: EnTvRat_GenericLevel = EnTvRat_GenericLevel(0i32);
pub const TvRat_1: EnTvRat_GenericLevel = EnTvRat_GenericLevel(1i32);
pub const TvRat_2: EnTvRat_GenericLevel = EnTvRat_GenericLevel(2i32);
pub const TvRat_3: EnTvRat_GenericLevel = EnTvRat_GenericLevel(3i32);
pub const TvRat_4: EnTvRat_GenericLevel = EnTvRat_GenericLevel(4i32);
pub const TvRat_5: EnTvRat_GenericLevel = EnTvRat_GenericLevel(5i32);
pub const TvRat_6: EnTvRat_GenericLevel = EnTvRat_GenericLevel(6i32);
pub const TvRat_7: EnTvRat_GenericLevel = EnTvRat_GenericLevel(7i32);
pub const TvRat_8: EnTvRat_GenericLevel = EnTvRat_GenericLevel(8i32);
pub const TvRat_9: EnTvRat_GenericLevel = EnTvRat_GenericLevel(9i32);
pub const TvRat_10: EnTvRat_GenericLevel = EnTvRat_GenericLevel(10i32);
pub const TvRat_11: EnTvRat_GenericLevel = EnTvRat_GenericLevel(11i32);
pub const TvRat_12: EnTvRat_GenericLevel = EnTvRat_GenericLevel(12i32);
pub const TvRat_13: EnTvRat_GenericLevel = EnTvRat_GenericLevel(13i32);
pub const TvRat_14: EnTvRat_GenericLevel = EnTvRat_GenericLevel(14i32);
pub const TvRat_15: EnTvRat_GenericLevel = EnTvRat_GenericLevel(15i32);
pub const TvRat_16: EnTvRat_GenericLevel = EnTvRat_GenericLevel(16i32);
pub const TvRat_17: EnTvRat_GenericLevel = EnTvRat_GenericLevel(17i32);
pub const TvRat_18: EnTvRat_GenericLevel = EnTvRat_GenericLevel(18i32);
pub const TvRat_19: EnTvRat_GenericLevel = EnTvRat_GenericLevel(19i32);
pub const TvRat_20: EnTvRat_GenericLevel = EnTvRat_GenericLevel(20i32);
pub const TvRat_21: EnTvRat_GenericLevel = EnTvRat_GenericLevel(21i32);
pub const TvRat_kLevels: EnTvRat_GenericLevel = EnTvRat_GenericLevel(22i32);
pub const TvRat_Unblock: EnTvRat_GenericLevel = EnTvRat_GenericLevel(-1i32);
pub const TvRat_LevelDontKnow: EnTvRat_GenericLevel = EnTvRat_GenericLevel(255i32);
#[repr(transparent)]
pub struct EnTvRat_MPAA(pub i32);
pub const MPAA_NotApplicable: EnTvRat_MPAA = EnTvRat_MPAA(0i32);
pub const MPAA_G: EnTvRat_MPAA = EnTvRat_MPAA(1i32);
pub const MPAA_PG: EnTvRat_MPAA = EnTvRat_MPAA(2i32);
pub const MPAA_PG13: EnTvRat_MPAA = EnTvRat_MPAA(3i32);
pub const MPAA_R: EnTvRat_MPAA = EnTvRat_MPAA(4i32);
pub const MPAA_NC17: EnTvRat_MPAA = EnTvRat_MPAA(5i32);
pub const MPAA_X: EnTvRat_MPAA = EnTvRat_MPAA(6i32);
pub const MPAA_NotRated: EnTvRat_MPAA = EnTvRat_MPAA(7i32);
#[repr(transparent)]
pub struct EnTvRat_System(pub i32);
pub const MPAA: EnTvRat_System = EnTvRat_System(0i32);
pub const US_TV: EnTvRat_System = EnTvRat_System(1i32);
pub const Canadian_English: EnTvRat_System = EnTvRat_System(2i32);
pub const Canadian_French: EnTvRat_System = EnTvRat_System(3i32);
pub const Reserved4: EnTvRat_System = EnTvRat_System(4i32);
pub const System5: EnTvRat_System = EnTvRat_System(5i32);
pub const System6: EnTvRat_System = EnTvRat_System(6i32);
pub const Reserved7: EnTvRat_System = EnTvRat_System(7i32);
pub const PBDA: EnTvRat_System = EnTvRat_System(8i32);
pub const AgeBased: EnTvRat_System = EnTvRat_System(9i32);
pub const TvRat_kSystems: EnTvRat_System = EnTvRat_System(10i32);
pub const TvRat_SystemDontKnow: EnTvRat_System = EnTvRat_System(255i32);
#[repr(transparent)]
pub struct EnTvRat_US_TV(pub i32);
pub const US_TV_None: EnTvRat_US_TV = EnTvRat_US_TV(0i32);
pub const US_TV_Y: EnTvRat_US_TV = EnTvRat_US_TV(1i32);
pub const US_TV_Y7: EnTvRat_US_TV = EnTvRat_US_TV(2i32);
pub const US_TV_G: EnTvRat_US_TV = EnTvRat_US_TV(3i32);
pub const US_TV_PG: EnTvRat_US_TV = EnTvRat_US_TV(4i32);
pub const US_TV_14: EnTvRat_US_TV = EnTvRat_US_TV(5i32);
pub const US_TV_MA: EnTvRat_US_TV = EnTvRat_US_TV(6i32);
pub const US_TV_None7: EnTvRat_US_TV = EnTvRat_US_TV(7i32);
#[repr(transparent)]
pub struct EncDecEvents(pub i32);
pub const ENCDEC_CPEVENT: EncDecEvents = EncDecEvents(0i32);
pub const ENCDEC_RECORDING_STATUS: EncDecEvents = EncDecEvents(1i32);
#[repr(transparent)]
pub struct EntitlementType(pub i32);
pub const Entitled: EntitlementType = EntitlementType(0i32);
pub const NotEntitled: EntitlementType = EntitlementType(1i32);
pub const TechnicalFailure: EntitlementType = EntitlementType(2i32);
#[repr(C)]
pub struct EvalRat(i32);
#[repr(transparent)]
pub struct FECMethod(pub i32);
pub const BDA_FEC_METHOD_NOT_SET: FECMethod = FECMethod(-1i32);
pub const BDA_FEC_METHOD_NOT_DEFINED: FECMethod = FECMethod(0i32);
pub const BDA_FEC_VITERBI: FECMethod = FECMethod(1i32);
pub const BDA_FEC_RS_204_188: FECMethod = FECMethod(2i32);
pub const BDA_FEC_LDPC: FECMethod = FECMethod(3i32);
pub const BDA_FEC_BCH: FECMethod = FECMethod(4i32);
pub const BDA_FEC_RS_147_130: FECMethod = FECMethod(5i32);
pub const BDA_FEC_MAX: FECMethod = FECMethod(6i32);
#[repr(C)]
pub struct FILTER_INFO(i32);
#[repr(transparent)]
pub struct FILTER_STATE(pub i32);
pub const State_Stopped: FILTER_STATE = FILTER_STATE(0i32);
pub const State_Paused: FILTER_STATE = FILTER_STATE(1i32);
pub const State_Running: FILTER_STATE = FILTER_STATE(2i32);
pub const FORMATTYPE_CPFilters_Processed: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1731834735,
    data2: 7519,
    data3: 19138,
    data4: [129, 146, 40, 187, 14, 115, 209, 106],
};
pub const FORMATTYPE_ETDTFilter_Tagged: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229777, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
#[repr(C)]
pub struct FilgraphManager(i32);
#[repr(transparent)]
pub struct FormatNotSupportedEvents(pub i32);
pub const FORMATNOTSUPPORTED_CLEAR: FormatNotSupportedEvents = FormatNotSupportedEvents(0i32);
pub const FORMATNOTSUPPORTED_NOTSUPPORTED: FormatNotSupportedEvents = FormatNotSupportedEvents(1i32);
pub const GUID_TIME_MUSIC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 91538589, data2: 23300, data3: 19221, data4: [165, 66, 174, 40, 32, 48, 17, 123] };
pub const GUID_TIME_REFERENCE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2477617451,
    data2: 55968,
    data3: 20478,
    data4: [188, 129, 176, 206, 80, 15, 205, 217],
};
pub const GUID_TIME_SAMPLES: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2824420613, data2: 3139, data3: 18820, data4: [154, 99, 151, 175, 158, 2, 196, 192] };
#[repr(transparent)]
pub struct GuardInterval(pub i32);
pub const BDA_GUARD_NOT_SET: GuardInterval = GuardInterval(-1i32);
pub const BDA_GUARD_NOT_DEFINED: GuardInterval = GuardInterval(0i32);
pub const BDA_GUARD_1_32: GuardInterval = GuardInterval(1i32);
pub const BDA_GUARD_1_16: GuardInterval = GuardInterval(2i32);
pub const BDA_GUARD_1_8: GuardInterval = GuardInterval(3i32);
pub const BDA_GUARD_1_4: GuardInterval = GuardInterval(4i32);
pub const BDA_GUARD_1_128: GuardInterval = GuardInterval(5i32);
pub const BDA_GUARD_19_128: GuardInterval = GuardInterval(6i32);
pub const BDA_GUARD_19_256: GuardInterval = GuardInterval(7i32);
pub const BDA_GUARD_MAX: GuardInterval = GuardInterval(8i32);
#[cfg(feature = "Win32_Media_Audio")]
#[repr(C)]
pub struct HEAACWAVEFORMAT(i32);
#[cfg(feature = "Win32_Media_Audio")]
#[repr(C)]
pub struct HEAACWAVEINFO(i32);
#[repr(transparent)]
pub struct HierarchyAlpha(pub i32);
pub const BDA_HALPHA_NOT_SET: HierarchyAlpha = HierarchyAlpha(-1i32);
pub const BDA_HALPHA_NOT_DEFINED: HierarchyAlpha = HierarchyAlpha(0i32);
pub const BDA_HALPHA_1: HierarchyAlpha = HierarchyAlpha(1i32);
pub const BDA_HALPHA_2: HierarchyAlpha = HierarchyAlpha(2i32);
pub const BDA_HALPHA_4: HierarchyAlpha = HierarchyAlpha(3i32);
pub const BDA_HALPHA_MAX: HierarchyAlpha = HierarchyAlpha(4i32);
#[repr(transparent)]
pub struct IAMAnalogVideoDecoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMAnalogVideoEncoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMAsyncReaderTimestampScaling(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMAudioInputMixer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMAudioRendererStats(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMBufferNegotiation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMCameraControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMCertifiedOutputProtection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMChannelInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMClockAdjust(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMClockSlave(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMCopyCaptureFileProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMCrossbar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMDecoderCaps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMDevMemoryAllocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMDevMemoryControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMDeviceRemoval(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMDirectSound(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMDroppedFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMExtDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMExtTransport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMExtendedErrorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMExtendedSeeking(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMFilterGraphCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMFilterMiscFlags(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMGraphBuilderCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMGraphStreams(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMLatency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMLine21Decoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMMediaContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMMediaContent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMMediaStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMMediaTypeSample(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMMediaTypeStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMMultiMediaStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMNetShowConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMNetShowExProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMNetShowPreroll(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMNetworkStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMOpenProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMOverlayFX(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMParse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMPhysicalPinInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMPlayList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMPlayListItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMPluginControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMPushSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMRebuild(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMResourceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMStats(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMStreamConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMStreamControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMStreamSelect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMTVAudio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMTVAudioNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMTVTuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMTimecodeDisplay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMTimecodeGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMTimecodeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMTuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMTunerNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMVfwCaptureDialogs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMVfwCompressDialogs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMVideoAccelerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMVideoAcceleratorNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMVideoCompression(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMVideoControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMVideoDecimationProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMVideoProcAmp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMWstDecoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAMovieSetup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSCChannelTuneRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSCComponentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSCLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSCLocator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSCTuningSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSC_EIT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSC_ETT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSC_MGT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSC_STT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IATSC_VCT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalogAudioComponentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalogLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalogRadioTuningSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalogRadioTuningSpace2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalogTVTuningSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsyncReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAtscContentAdvisoryDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAtscPsipParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAttributeGet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAttributeSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioMediaStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioStreamSample(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAuxInTuningSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAuxInTuningSpace2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDAComparable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDACreateTuneRequestEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_AUX(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_AutoDemodulate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_AutoDemodulateEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_ConditionalAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_ConditionalAccessEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DRIDRMService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DRIWMDRMSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DRM(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DRMService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DeviceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DiagnosticProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DigitalDemodulator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DigitalDemodulator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DigitalDemodulator3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_DiseqCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_EasMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_Encoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_EthernetFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_EventingService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_FDC(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_FrequencyFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_GuideDataDeliveryService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_IPSinkControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_IPSinkInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_IPV4Filter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_IPV6Filter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_ISDBConditionalAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_LNBInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_MUX(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_NameValueService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_NetworkProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_NullTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_PinControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_SignalProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_SignalStatistics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_TIF_REGISTRATION(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_Topology(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_TransportStreamInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_TransportStreamSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_UserActivityService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_VoidTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_WMDRMSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBDA_WMDRMTuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBPCSatelliteTuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBaseFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBaseVideoMixer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBasicAudio(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBasicVideo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBasicVideo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBroadcastEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBroadcastEventEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBufferingTime(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICAT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICCSubStreamFiltering(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICameraControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICaptionServiceDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICaptureGraphBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICaptureGraphBuilder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChannelIDTuneRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChannelTuneRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponentTypes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponentsOld(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConfigAsfWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConfigAsfWriter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConfigAviMux(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConfigInterleaving(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateDevEnum(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreatePropBagOnRegKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDDrawExclModeVideo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDDrawExclModeVideoCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMOWrapperFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDShowPlugin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDTFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDTFilter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDTFilter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDTFilterConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDTFilterEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDTFilterLicenseRenewal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBCLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBSLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBSLocator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBSTuningSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBTLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBTLocator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBTuneRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBTuningSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVBTuningSpace2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_BAT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_DIT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_EIT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_EIT2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_NIT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_RST(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_SDT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_SIT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_ST(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_TDT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVB_TOT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVEnc(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVRGB219(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDVSplitter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDecimateVideoImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeferredCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDigitalCableLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDigitalCableTuneRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDigitalCableTuningSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDigitalLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectDrawMediaSample(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectDrawMediaSampleAllocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectDrawMediaStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectDrawStreamSample(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirectDrawVideo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDistributorNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDrawVideoImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbCableDeliverySystemDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbComponentDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbContentDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbContentIdentifierDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbDataBroadcastDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbDataBroadcastIDDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbDefaultAuthorityDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbExtendedEventDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbFrequencyListDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbHDSimulcastLogicalChannelDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbLinkageDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbLogicalChannel2Descriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbLogicalChannelDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbLogicalChannelDescriptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbMultilingualServiceNameDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbNetworkNameDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbParentalRatingDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbPrivateDataSpecifierDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbSatelliteDeliverySystemDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbServiceAttributeDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbServiceDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbServiceDescriptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbServiceListDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbShortEventDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbSiParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbSiParser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbSubtitlingDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbTeletextDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbTerrestrial2DeliverySystemDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvbTerrestrialDeliverySystemDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvdCmd(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvdControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvdControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvdGraphBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvdInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvdInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDvdState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESCloseMmiEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESEventFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESEventService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESEventServiceConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESFileExpiryDateEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESIsdbCasResponseEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESLicenseRenewalResultEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESOpenMmiEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESRequestTunerEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IESValueUpdatedEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IETFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IETFilterConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IETFilterEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEncoderAPI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumComponentTypes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumComponents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumFilters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumGuideDataProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumMSVidGraphSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumMediaTypes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumPIDMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumPins(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumRegFilters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumStreamBufferRecordingAttrib(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumStreamIdMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumTuneRequests(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumTuningSpaces(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEvalRat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFILTERMAPPER_MERIT(pub i32);
pub const MERIT_PREFERRED: IFILTERMAPPER_MERIT = IFILTERMAPPER_MERIT(8388608i32);
pub const MERIT_NORMAL: IFILTERMAPPER_MERIT = IFILTERMAPPER_MERIT(6291456i32);
pub const MERIT_UNLIKELY: IFILTERMAPPER_MERIT = IFILTERMAPPER_MERIT(4194304i32);
pub const MERIT_DO_NOT_USE: IFILTERMAPPER_MERIT = IFILTERMAPPER_MERIT(2097152i32);
pub const MERIT_SW_COMPRESSOR: IFILTERMAPPER_MERIT = IFILTERMAPPER_MERIT(1048576i32);
pub const MERIT_HW_COMPRESSOR: IFILTERMAPPER_MERIT = IFILTERMAPPER_MERIT(1048656i32);
#[repr(transparent)]
pub struct IFileSinkFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSinkFilter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSourceFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterChain(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterGraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterGraph2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterGraph3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterMapper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterMapper2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFilterMapper3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrequencyMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFullScreenVideo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFullScreenVideoEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGenericDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGenericDescriptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetCapabilitiesKey(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpnvsCommonBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphConfigCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphVersion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuideData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuideDataEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuideDataLoader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuideDataProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIPDVDec(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IISDBSLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IISDB_BIT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IISDB_CDT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IISDB_EMM(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IISDB_LDT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IISDB_NBIT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IISDB_SDT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IISDB_SDTT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbAudioComponentDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbCAContractInformationDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbCADescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbCAServiceDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbComponentGroupDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbDataContentDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbDigitalCopyControlDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbDownloadContentDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbEmergencyInformationDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbEventGroupDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbHierarchicalTransmissionDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbLogoTransmissionDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbSIParameterDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbSeriesDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbSiParser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbTSInformationDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsdbTerrestrialDeliverySystemDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKsNodeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKsTopologyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanguageComponentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMPEG2Component(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMPEG2ComponentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMPEG2PIDMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMPEG2StreamIdMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMPEG2TuneRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMPEG2TuneRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMPEG2TuneRequestSupport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMPEG2_TIF_CONTROL(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSEventBinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidAnalogTuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidAnalogTuner2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidAnalogTunerEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidAudioRenderer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidAudioRendererDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidAudioRendererEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidAudioRendererEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidClosedCaptioning(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidClosedCaptioning2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidClosedCaptioning3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidCompositionSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidCtl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidDataServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidDataServicesEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidDeviceEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidEVR(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidEVREvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidEncoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidFeature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidFeatureEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidFeatures(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidFilePlayback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidFilePlayback2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidFilePlaybackEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidGenericSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidGenericSink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidGraphSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidGraphSegmentContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidGraphSegmentUserInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidInputDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidInputDeviceEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidInputDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidOutputDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidOutputDeviceEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidOutputDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidPlayback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidPlaybackEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidRect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferRecordingControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSink3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSinkEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSinkEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSinkEvent3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSinkEvent4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSourceEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSourceEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferSourceEvent3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidStreamBufferV2SourceEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidTuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidTunerEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidVMR9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidVRGraphSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidVideoInputDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidVideoRenderer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidVideoRenderer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidVideoRendererDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidVideoRendererEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidVideoRendererEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidWebDVD(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidWebDVD2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidWebDVDAdm(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidWebDVDEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidXDS(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSVidXDSEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMceBurnerControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEventEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaEventSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaParamInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaParams(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPosition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPropertyBag(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSample(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSample2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSample2Config(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaSeeking(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaStreamFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTypeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemAllocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemAllocatorCallbackTemp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemAllocatorNotifyCallbackTemp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemInputPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMemoryData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMixerOCX(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMixerOCXNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMixerPinConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMixerPinConfig2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMpeg2Data(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMpeg2Demultiplexer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMpeg2Stream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMpeg2TableFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMpegAudioDecoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiMediaStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOverlay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOverlayNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOverlayNotify2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPAT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPBDAAttributesDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPBDAEntitlementDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPBDASiParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPBDA_EIT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPBDA_Services(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPSITables(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPTFilterLicenseRenewal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistMediaPropertyBag(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistTuneXml(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistTuneXmlUtility(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistTuneXmlUtility2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPinConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPinFlowControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPinInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQualProp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQualityControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueueCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegFilterInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegisterServiceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegisterTuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceConsumer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISBE2Crossbar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISBE2EnumStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISBE2FileScan(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISBE2GlobalEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISBE2GlobalEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISBE2MediaTypeProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISBE2SpanningEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISBE2StreamMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISCTE_EAS(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISDBCAS_REQUEST_ID(pub i32);
pub const ISDBCAS_REQUEST_ID_EMG: ISDBCAS_REQUEST_ID = ISDBCAS_REQUEST_ID(56i32);
pub const ISDBCAS_REQUEST_ID_EMD: ISDBCAS_REQUEST_ID = ISDBCAS_REQUEST_ID(58i32);
#[repr(C)]
pub struct ISDBSLocator(i32);
pub const ISDB_BIT_PID: u32 = 36u32;
pub const ISDB_BIT_TID: u32 = 196u32;
#[repr(C)]
pub struct ISDB_CABLE_TV_NETWORK_TYPE(i32);
pub const ISDB_CDT_PID: u32 = 41u32;
pub const ISDB_CDT_TID: u32 = 200u32;
pub const ISDB_EMM_TID: u32 = 133u32;
pub const ISDB_LDT_PID: u32 = 37u32;
pub const ISDB_LDT_TID: u32 = 199u32;
pub const ISDB_NBIT_MSG_TID: u32 = 197u32;
pub const ISDB_NBIT_PID: u32 = 37u32;
pub const ISDB_NBIT_REF_TID: u32 = 198u32;
#[repr(C)]
pub struct ISDB_SATELLITE_TV_NETWORK_TYPE(i32);
pub const ISDB_SDTT_ALT_PID: u32 = 40u32;
pub const ISDB_SDTT_PID: u32 = 35u32;
pub const ISDB_SDTT_TID: u32 = 195u32;
pub const ISDB_ST_TID: u32 = 114u32;
#[repr(C)]
pub struct ISDB_S_NETWORK_TYPE(i32);
#[repr(C)]
pub struct ISDB_TERRESTRIAL_TV_NETWORK_TYPE(i32);
#[repr(C)]
pub struct ISDB_T_NETWORK_TYPE(i32);
#[repr(transparent)]
pub struct ISIInbandEPG(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISIInbandEPGEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScanningTuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScanningTunerEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISectionList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISeekingPassThru(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceLocationDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpecifyParticularPages(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferConfigure(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferConfigure2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferConfigure3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferDataCounters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferInitialize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferMediaSeeking(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferMediaSeeking2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferRecComp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferRecordControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferRecordingAttribute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferSink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferSink3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBufferSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamSample(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITSDT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITuneRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITuneRequestInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITuneRequestInfoEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITuner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITunerCap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITunerCapEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITuningSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITuningSpaceContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITuningSpaces(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRAspectRatioControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRAspectRatioControl9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRDeinterlaceControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRDeinterlaceControl9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRFilterConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRFilterConfig9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRImageCompositor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRImageCompositor9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRImagePresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRImagePresenter9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRImagePresenterConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRImagePresenterConfig9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRImagePresenterExclModeConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRMixerBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRMixerBitmap9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRMixerControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRMixerControl9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRMonitorConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRMonitorConfig9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRSurface9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRSurfaceAllocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRSurfaceAllocator9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRSurfaceAllocatorEx9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRSurfaceAllocatorNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRSurfaceAllocatorNotify9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRVideoStreamControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRVideoStreamControl9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRWindowlessControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVMRWindowlessControl9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVPBaseConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVPBaseNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVPConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVPManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVPNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVPNotify2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVPVBIConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVPVBINotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoEncoder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoFrameStep(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoProcAmp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXDSCodec(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXDSCodecConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXDSCodecEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXDSToRat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InterleavingMode(pub i32);
pub const INTERLEAVE_NONE: InterleavingMode = InterleavingMode(0i32);
pub const INTERLEAVE_CAPTURE: InterleavingMode = InterleavingMode(1i32);
pub const INTERLEAVE_FULL: InterleavingMode = InterleavingMode(2i32);
pub const INTERLEAVE_NONE_BUFFERED: InterleavingMode = InterleavingMode(3i32);
#[repr(C)]
pub struct KSCATEGORY_BDA_IP_SINK(i32);
#[repr(C)]
pub struct KSCATEGORY_BDA_NETWORK_EPG(i32);
#[repr(C)]
pub struct KSCATEGORY_BDA_NETWORK_PROVIDER(i32);
#[repr(C)]
pub struct KSCATEGORY_BDA_NETWORK_TUNER(i32);
#[repr(C)]
pub struct KSCATEGORY_BDA_RECEIVER_COMPONENT(i32);
#[repr(C)]
pub struct KSCATEGORY_BDA_TRANSPORT_INFORMATION(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SPECIFIER_BDA_IP(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SPECIFIER_BDA_TRANSPORT(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_ATSC_SI(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_BDA_IP(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_BDA_IP_CONTROL(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_BDA_MPEG2_TRANSPORT(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_OOB_PSIP(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_PSIP(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_DVB_SI(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_ISDB_SI(i32);
#[repr(C)]
pub struct KSDATAFORMAT_SUBTYPE_PBDA_TRANSPORT_RAW(i32);
#[repr(C)]
pub struct KSDATAFORMAT_TYPE_BDA_ANTENNA(i32);
#[repr(C)]
pub struct KSDATAFORMAT_TYPE_BDA_IF_SIGNAL(i32);
#[repr(C)]
pub struct KSDATAFORMAT_TYPE_BDA_IP(i32);
#[repr(C)]
pub struct KSDATAFORMAT_TYPE_BDA_IP_CONTROL(i32);
#[repr(C)]
pub struct KSDATAFORMAT_TYPE_MPE(i32);
#[repr(C)]
pub struct KSDATAFORMAT_TYPE_MPEG2_SECTIONS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_KernelStreaming"))]
#[repr(C)]
pub struct KSEVENTDATA_BDA_RF_TUNER_SCAN_S(i32);
#[repr(C)]
pub struct KSEVENTSETID_BdaCAEvent(i32);
#[repr(C)]
pub struct KSEVENTSETID_BdaDiseqCEvent(i32);
#[repr(C)]
pub struct KSEVENTSETID_BdaEvent(i32);
#[repr(C)]
pub struct KSEVENTSETID_BdaPinEvent(i32);
#[repr(C)]
pub struct KSEVENTSETID_BdaTunerEvent(i32);
#[repr(transparent)]
pub struct KSEVENT_BDA_EVENT_TYPE(pub i32);
pub const KSEVENT_BDA_EVENT_PENDINGEVENT: KSEVENT_BDA_EVENT_TYPE = KSEVENT_BDA_EVENT_TYPE(0i32);
#[repr(transparent)]
pub struct KSEVENT_BDA_TUNER(pub i32);
pub const KSEVENT_BDA_TUNER_SCAN: KSEVENT_BDA_TUNER = KSEVENT_BDA_TUNER(0i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaChangeSync(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaConditionalAccessService(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaDebug(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaDeviceConfiguration(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaDrmService(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaEventing(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaGuideDataDeliveryService(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaIsdbConditionalAccess(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaMux(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaNameValue(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaNameValueA(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaScanning(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaTSSelector(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaTuner(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaUserActivity(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaWmdrmSession(i32);
#[repr(C)]
pub struct KSMETHODSETID_BdaWmdrmTuner(i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_CAS_SERVICE(pub i32);
pub const KSMETHOD_BDA_CAS_CHECKENTITLEMENTTOKEN: KSMETHOD_BDA_CAS_SERVICE = KSMETHOD_BDA_CAS_SERVICE(0i32);
pub const KSMETHOD_BDA_CAS_SETCAPTURETOKEN: KSMETHOD_BDA_CAS_SERVICE = KSMETHOD_BDA_CAS_SERVICE(1i32);
pub const KSMETHOD_BDA_CAS_OPENBROADCASTMMI: KSMETHOD_BDA_CAS_SERVICE = KSMETHOD_BDA_CAS_SERVICE(2i32);
pub const KSMETHOD_BDA_CAS_CLOSEMMIDIALOG: KSMETHOD_BDA_CAS_SERVICE = KSMETHOD_BDA_CAS_SERVICE(3i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_CHANGE_SYNC(pub i32);
pub const KSMETHOD_BDA_START_CHANGES: KSMETHOD_BDA_CHANGE_SYNC = KSMETHOD_BDA_CHANGE_SYNC(0i32);
pub const KSMETHOD_BDA_CHECK_CHANGES: KSMETHOD_BDA_CHANGE_SYNC = KSMETHOD_BDA_CHANGE_SYNC(1i32);
pub const KSMETHOD_BDA_COMMIT_CHANGES: KSMETHOD_BDA_CHANGE_SYNC = KSMETHOD_BDA_CHANGE_SYNC(2i32);
pub const KSMETHOD_BDA_GET_CHANGE_STATE: KSMETHOD_BDA_CHANGE_SYNC = KSMETHOD_BDA_CHANGE_SYNC(3i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_DEBUG_SERVICE(pub i32);
pub const KSMETHOD_BDA_DEBUG_LEVEL: KSMETHOD_BDA_DEBUG_SERVICE = KSMETHOD_BDA_DEBUG_SERVICE(0i32);
pub const KSMETHOD_BDA_DEBUG_DATA: KSMETHOD_BDA_DEBUG_SERVICE = KSMETHOD_BDA_DEBUG_SERVICE(1i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_DEVICE_CONFIGURATION(pub i32);
pub const KSMETHOD_BDA_CREATE_PIN_FACTORY: KSMETHOD_BDA_DEVICE_CONFIGURATION = KSMETHOD_BDA_DEVICE_CONFIGURATION(0i32);
pub const KSMETHOD_BDA_DELETE_PIN_FACTORY: KSMETHOD_BDA_DEVICE_CONFIGURATION = KSMETHOD_BDA_DEVICE_CONFIGURATION(1i32);
pub const KSMETHOD_BDA_CREATE_TOPOLOGY: KSMETHOD_BDA_DEVICE_CONFIGURATION = KSMETHOD_BDA_DEVICE_CONFIGURATION(2i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_DRM(pub i32);
pub const KSMETHOD_BDA_DRM_CURRENT: KSMETHOD_BDA_DRM = KSMETHOD_BDA_DRM(0i32);
pub const KSMETHOD_BDA_DRM_DRMSTATUS: KSMETHOD_BDA_DRM = KSMETHOD_BDA_DRM(1i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_EVENTING_SERVICE(pub i32);
pub const KSMETHOD_BDA_EVENT_DATA: KSMETHOD_BDA_EVENTING_SERVICE = KSMETHOD_BDA_EVENTING_SERVICE(0i32);
pub const KSMETHOD_BDA_EVENT_COMPLETE: KSMETHOD_BDA_EVENTING_SERVICE = KSMETHOD_BDA_EVENTING_SERVICE(1i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_GDDS_SERVICE(pub i32);
pub const KSMETHOD_BDA_GDDS_DATATYPE: KSMETHOD_BDA_GDDS_SERVICE = KSMETHOD_BDA_GDDS_SERVICE(0i32);
pub const KSMETHOD_BDA_GDDS_DATA: KSMETHOD_BDA_GDDS_SERVICE = KSMETHOD_BDA_GDDS_SERVICE(1i32);
pub const KSMETHOD_BDA_GDDS_TUNEXMLFROMIDX: KSMETHOD_BDA_GDDS_SERVICE = KSMETHOD_BDA_GDDS_SERVICE(2i32);
pub const KSMETHOD_BDA_GDDS_GETSERVICES: KSMETHOD_BDA_GDDS_SERVICE = KSMETHOD_BDA_GDDS_SERVICE(3i32);
pub const KSMETHOD_BDA_GDDS_SERVICEFROMTUNEXML: KSMETHOD_BDA_GDDS_SERVICE = KSMETHOD_BDA_GDDS_SERVICE(4i32);
pub const KSMETHOD_BDA_GDDS_DATAUPDATE: KSMETHOD_BDA_GDDS_SERVICE = KSMETHOD_BDA_GDDS_SERVICE(5i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_GPNV_SERVICE(pub i32);
pub const KSMETHOD_BDA_GPNV_GETVALUE: KSMETHOD_BDA_GPNV_SERVICE = KSMETHOD_BDA_GPNV_SERVICE(0i32);
pub const KSMETHOD_BDA_GPNV_SETVALUE: KSMETHOD_BDA_GPNV_SERVICE = KSMETHOD_BDA_GPNV_SERVICE(1i32);
pub const KSMETHOD_BDA_GPNV_NAMEFROMINDEX: KSMETHOD_BDA_GPNV_SERVICE = KSMETHOD_BDA_GPNV_SERVICE(2i32);
pub const KSMETHOD_BDA_GPNV_GETVALUEUPDATENAME: KSMETHOD_BDA_GPNV_SERVICE = KSMETHOD_BDA_GPNV_SERVICE(3i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_ISDB_CAS(pub i32);
pub const KSMETHOD_BDA_ISDBCAS_SETREQUEST: KSMETHOD_BDA_ISDB_CAS = KSMETHOD_BDA_ISDB_CAS(0i32);
pub const KSMETHOD_BDA_ISDBCAS_RESPONSEDATA: KSMETHOD_BDA_ISDB_CAS = KSMETHOD_BDA_ISDB_CAS(1i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_MUX_SERVICE(pub i32);
pub const KSMETHOD_BDA_MUX_GETPIDLIST: KSMETHOD_BDA_MUX_SERVICE = KSMETHOD_BDA_MUX_SERVICE(0i32);
pub const KSMETHOD_BDA_MUX_SETPIDLIST: KSMETHOD_BDA_MUX_SERVICE = KSMETHOD_BDA_MUX_SERVICE(1i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_SCAN_SERVICE(pub i32);
pub const KSMETHOD_BDA_SCAN_CAPABILTIES: KSMETHOD_BDA_SCAN_SERVICE = KSMETHOD_BDA_SCAN_SERVICE(0i32);
pub const KSMETHOD_BDA_SCANNING_STATE: KSMETHOD_BDA_SCAN_SERVICE = KSMETHOD_BDA_SCAN_SERVICE(1i32);
pub const KSMETHOD_BDA_SCAN_FILTER: KSMETHOD_BDA_SCAN_SERVICE = KSMETHOD_BDA_SCAN_SERVICE(2i32);
pub const KSMETHOD_BDA_SCAN_START: KSMETHOD_BDA_SCAN_SERVICE = KSMETHOD_BDA_SCAN_SERVICE(3i32);
pub const KSMETHOD_BDA_SCAN_RESUME: KSMETHOD_BDA_SCAN_SERVICE = KSMETHOD_BDA_SCAN_SERVICE(4i32);
pub const KSMETHOD_BDA_SCAN_STOP: KSMETHOD_BDA_SCAN_SERVICE = KSMETHOD_BDA_SCAN_SERVICE(5i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_TS_SELECTOR(pub i32);
pub const KSMETHOD_BDA_TS_SELECTOR_SETTSID: KSMETHOD_BDA_TS_SELECTOR = KSMETHOD_BDA_TS_SELECTOR(0i32);
pub const KSMETHOD_BDA_TS_SELECTOR_GETTSINFORMATION: KSMETHOD_BDA_TS_SELECTOR = KSMETHOD_BDA_TS_SELECTOR(1i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_TUNER_SERVICE(pub i32);
pub const KSMETHOD_BDA_TUNER_SETTUNER: KSMETHOD_BDA_TUNER_SERVICE = KSMETHOD_BDA_TUNER_SERVICE(0i32);
pub const KSMETHOD_BDA_TUNER_GETTUNERSTATE: KSMETHOD_BDA_TUNER_SERVICE = KSMETHOD_BDA_TUNER_SERVICE(1i32);
pub const KSMETHOD_BDA_TUNER_SIGNALNOISERATIO: KSMETHOD_BDA_TUNER_SERVICE = KSMETHOD_BDA_TUNER_SERVICE(2i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_USERACTIVITY_SERVICE(pub i32);
pub const KSMETHOD_BDA_USERACTIVITY_USEREASON: KSMETHOD_BDA_USERACTIVITY_SERVICE = KSMETHOD_BDA_USERACTIVITY_SERVICE(0i32);
pub const KSMETHOD_BDA_USERACTIVITY_INTERVAL: KSMETHOD_BDA_USERACTIVITY_SERVICE = KSMETHOD_BDA_USERACTIVITY_SERVICE(1i32);
pub const KSMETHOD_BDA_USERACTIVITY_DETECTED: KSMETHOD_BDA_USERACTIVITY_SERVICE = KSMETHOD_BDA_USERACTIVITY_SERVICE(2i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_WMDRM(pub i32);
pub const KSMETHOD_BDA_WMDRM_STATUS: KSMETHOD_BDA_WMDRM = KSMETHOD_BDA_WMDRM(0i32);
pub const KSMETHOD_BDA_WMDRM_REVINFO: KSMETHOD_BDA_WMDRM = KSMETHOD_BDA_WMDRM(1i32);
pub const KSMETHOD_BDA_WMDRM_CRL: KSMETHOD_BDA_WMDRM = KSMETHOD_BDA_WMDRM(2i32);
pub const KSMETHOD_BDA_WMDRM_MESSAGE: KSMETHOD_BDA_WMDRM = KSMETHOD_BDA_WMDRM(3i32);
pub const KSMETHOD_BDA_WMDRM_REISSUELICENSE: KSMETHOD_BDA_WMDRM = KSMETHOD_BDA_WMDRM(4i32);
pub const KSMETHOD_BDA_WMDRM_RENEWLICENSE: KSMETHOD_BDA_WMDRM = KSMETHOD_BDA_WMDRM(5i32);
pub const KSMETHOD_BDA_WMDRM_LICENSE: KSMETHOD_BDA_WMDRM = KSMETHOD_BDA_WMDRM(6i32);
pub const KSMETHOD_BDA_WMDRM_KEYINFO: KSMETHOD_BDA_WMDRM = KSMETHOD_BDA_WMDRM(7i32);
#[repr(transparent)]
pub struct KSMETHOD_BDA_WMDRM_TUNER(pub i32);
pub const KSMETHOD_BDA_WMDRMTUNER_CANCELCAPTURETOKEN: KSMETHOD_BDA_WMDRM_TUNER = KSMETHOD_BDA_WMDRM_TUNER(0i32);
pub const KSMETHOD_BDA_WMDRMTUNER_SETPIDPROTECTION: KSMETHOD_BDA_WMDRM_TUNER = KSMETHOD_BDA_WMDRM_TUNER(1i32);
pub const KSMETHOD_BDA_WMDRMTUNER_GETPIDPROTECTION: KSMETHOD_BDA_WMDRM_TUNER = KSMETHOD_BDA_WMDRM_TUNER(2i32);
pub const KSMETHOD_BDA_WMDRMTUNER_SETSYNCVALUE: KSMETHOD_BDA_WMDRM_TUNER = KSMETHOD_BDA_WMDRM_TUNER(3i32);
pub const KSMETHOD_BDA_WMDRMTUNER_STARTCODEPROFILE: KSMETHOD_BDA_WMDRM_TUNER = KSMETHOD_BDA_WMDRM_TUNER(4i32);
pub const KSMETHOD_BDA_WMDRMTUNER_PURCHASE_ENTITLEMENT: KSMETHOD_BDA_WMDRM_TUNER = KSMETHOD_BDA_WMDRM_TUNER(5i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_BUFFER(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_CAS_CAPTURETOKEN(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_KernelStreaming"))]
#[repr(C)]
pub struct KSM_BDA_CAS_CLOSEMMIDIALOG(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_KernelStreaming"))]
#[repr(C)]
pub struct KSM_BDA_CAS_ENTITLEMENTTOKEN(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_KernelStreaming"))]
#[repr(C)]
pub struct KSM_BDA_CAS_OPENBROADCASTMMI(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_DEBUG_LEVEL(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_DRM_SETDRM(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_EVENT_COMPLETE(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_GDDS_SERVICEFROMTUNEXML(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_GDDS_TUNEXMLFROMIDX(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_KernelStreaming"))]
#[repr(C)]
pub struct KSM_BDA_GPNV_GETVALUE(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_GPNV_NAMEINDEX(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_KernelStreaming"))]
#[repr(C)]
pub struct KSM_BDA_GPNV_SETVALUE(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_ISDBCAS_REQUEST(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_PIN(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_PIN_PAIR(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_SCAN_CAPABILTIES(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_SCAN_FILTER(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_SCAN_START(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_TS_SELECTOR_SETTSID(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_TUNER_TUNEREQUEST(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_USERACTIVITY_USEREASON(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_WMDRMTUNER_GETPIDPROTECTION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_KernelStreaming"))]
#[repr(C)]
pub struct KSM_BDA_WMDRMTUNER_PURCHASEENTITLEMENT(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_WMDRMTUNER_SETPIDPROTECTION(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_WMDRMTUNER_SYNCVALUE(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_WMDRM_LICENSE(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSM_BDA_WMDRM_RENEWLICENSE(i32);
#[repr(C)]
pub struct KSNODE_BDA_8PSK_DEMODULATOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_8VSB_DEMODULATOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_ANALOG_DEMODULATOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_COFDM_DEMODULATOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_COMMON_CA_POD(i32);
#[repr(C)]
pub struct KSNODE_BDA_DRI_DRM(i32);
#[repr(C)]
pub struct KSNODE_BDA_IP_SINK(i32);
#[repr(C)]
pub struct KSNODE_BDA_ISDB_S_DEMODULATOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_ISDB_T_DEMODULATOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_OPENCABLE_POD(i32);
#[repr(C)]
pub struct KSNODE_BDA_PBDA_CAS(i32);
#[repr(C)]
pub struct KSNODE_BDA_PBDA_DRM(i32);
#[repr(C)]
pub struct KSNODE_BDA_PBDA_ISDBCAS(i32);
#[repr(C)]
pub struct KSNODE_BDA_PBDA_MUX(i32);
#[repr(C)]
pub struct KSNODE_BDA_PBDA_TUNER(i32);
#[repr(C)]
pub struct KSNODE_BDA_PID_FILTER(i32);
#[repr(C)]
pub struct KSNODE_BDA_QAM_DEMODULATOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_QPSK_DEMODULATOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_RF_TUNER(i32);
#[repr(C)]
pub struct KSNODE_BDA_TS_SELECTOR(i32);
#[repr(C)]
pub struct KSNODE_BDA_VIDEO_ENCODER(i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_AUTODEMODULATE(pub i32);
pub const KSPROPERTY_BDA_AUTODEMODULATE_START: KSPROPERTY_BDA_AUTODEMODULATE = KSPROPERTY_BDA_AUTODEMODULATE(0i32);
pub const KSPROPERTY_BDA_AUTODEMODULATE_STOP: KSPROPERTY_BDA_AUTODEMODULATE = KSPROPERTY_BDA_AUTODEMODULATE(1i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_CA(pub i32);
pub const KSPROPERTY_BDA_ECM_MAP_STATUS: KSPROPERTY_BDA_CA = KSPROPERTY_BDA_CA(0i32);
pub const KSPROPERTY_BDA_CA_MODULE_STATUS: KSPROPERTY_BDA_CA = KSPROPERTY_BDA_CA(1i32);
pub const KSPROPERTY_BDA_CA_SMART_CARD_STATUS: KSPROPERTY_BDA_CA = KSPROPERTY_BDA_CA(2i32);
pub const KSPROPERTY_BDA_CA_MODULE_UI: KSPROPERTY_BDA_CA = KSPROPERTY_BDA_CA(3i32);
pub const KSPROPERTY_BDA_CA_SET_PROGRAM_PIDS: KSPROPERTY_BDA_CA = KSPROPERTY_BDA_CA(4i32);
pub const KSPROPERTY_BDA_CA_REMOVE_PROGRAM: KSPROPERTY_BDA_CA = KSPROPERTY_BDA_CA(5i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_CA_EVENT(pub i32);
pub const KSEVENT_BDA_PROGRAM_FLOW_STATUS_CHANGED: KSPROPERTY_BDA_CA_EVENT = KSPROPERTY_BDA_CA_EVENT(0i32);
pub const KSEVENT_BDA_CA_MODULE_STATUS_CHANGED: KSPROPERTY_BDA_CA_EVENT = KSPROPERTY_BDA_CA_EVENT(1i32);
pub const KSEVENT_BDA_CA_SMART_CARD_STATUS_CHANGED: KSPROPERTY_BDA_CA_EVENT = KSPROPERTY_BDA_CA_EVENT(2i32);
pub const KSEVENT_BDA_CA_MODULE_UI_REQUESTED: KSPROPERTY_BDA_CA_EVENT = KSPROPERTY_BDA_CA_EVENT(3i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_DIGITAL_DEMODULATOR(pub i32);
pub const KSPROPERTY_BDA_MODULATION_TYPE: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(0i32);
pub const KSPROPERTY_BDA_INNER_FEC_TYPE: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(1i32);
pub const KSPROPERTY_BDA_INNER_FEC_RATE: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(2i32);
pub const KSPROPERTY_BDA_OUTER_FEC_TYPE: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(3i32);
pub const KSPROPERTY_BDA_OUTER_FEC_RATE: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(4i32);
pub const KSPROPERTY_BDA_SYMBOL_RATE: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(5i32);
pub const KSPROPERTY_BDA_SPECTRAL_INVERSION: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(6i32);
pub const KSPROPERTY_BDA_GUARD_INTERVAL: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(7i32);
pub const KSPROPERTY_BDA_TRANSMISSION_MODE: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(8i32);
pub const KSPROPERTY_BDA_ROLL_OFF: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(9i32);
pub const KSPROPERTY_BDA_PILOT: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(10i32);
pub const KSPROPERTY_BDA_SIGNALTIMEOUTS: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(11i32);
pub const KSPROPERTY_BDA_PLP_NUMBER: KSPROPERTY_BDA_DIGITAL_DEMODULATOR = KSPROPERTY_BDA_DIGITAL_DEMODULATOR(12i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_DISEQC_COMMAND(pub i32);
pub const KSPROPERTY_BDA_DISEQC_ENABLE: KSPROPERTY_BDA_DISEQC_COMMAND = KSPROPERTY_BDA_DISEQC_COMMAND(0i32);
pub const KSPROPERTY_BDA_DISEQC_LNB_SOURCE: KSPROPERTY_BDA_DISEQC_COMMAND = KSPROPERTY_BDA_DISEQC_COMMAND(1i32);
pub const KSPROPERTY_BDA_DISEQC_USETONEBURST: KSPROPERTY_BDA_DISEQC_COMMAND = KSPROPERTY_BDA_DISEQC_COMMAND(2i32);
pub const KSPROPERTY_BDA_DISEQC_REPEATS: KSPROPERTY_BDA_DISEQC_COMMAND = KSPROPERTY_BDA_DISEQC_COMMAND(3i32);
pub const KSPROPERTY_BDA_DISEQC_SEND: KSPROPERTY_BDA_DISEQC_COMMAND = KSPROPERTY_BDA_DISEQC_COMMAND(4i32);
pub const KSPROPERTY_BDA_DISEQC_RESPONSE: KSPROPERTY_BDA_DISEQC_COMMAND = KSPROPERTY_BDA_DISEQC_COMMAND(5i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_DISEQC_EVENT(pub i32);
pub const KSEVENT_BDA_DISEQC_DATA_RECEIVED: KSPROPERTY_BDA_DISEQC_EVENT = KSPROPERTY_BDA_DISEQC_EVENT(0i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_ETHERNET_FILTER(pub i32);
pub const KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_LIST_SIZE: KSPROPERTY_BDA_ETHERNET_FILTER = KSPROPERTY_BDA_ETHERNET_FILTER(0i32);
pub const KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_LIST: KSPROPERTY_BDA_ETHERNET_FILTER = KSPROPERTY_BDA_ETHERNET_FILTER(1i32);
pub const KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_MODE: KSPROPERTY_BDA_ETHERNET_FILTER = KSPROPERTY_BDA_ETHERNET_FILTER(2i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_FREQUENCY_FILTER(pub i32);
pub const KSPROPERTY_BDA_RF_TUNER_FREQUENCY: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(0i32);
pub const KSPROPERTY_BDA_RF_TUNER_POLARITY: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(1i32);
pub const KSPROPERTY_BDA_RF_TUNER_RANGE: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(2i32);
pub const KSPROPERTY_BDA_RF_TUNER_TRANSPONDER: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(3i32);
pub const KSPROPERTY_BDA_RF_TUNER_BANDWIDTH: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(4i32);
pub const KSPROPERTY_BDA_RF_TUNER_FREQUENCY_MULTIPLIER: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(5i32);
pub const KSPROPERTY_BDA_RF_TUNER_CAPS: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(6i32);
pub const KSPROPERTY_BDA_RF_TUNER_SCAN_STATUS: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(7i32);
pub const KSPROPERTY_BDA_RF_TUNER_STANDARD: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(8i32);
pub const KSPROPERTY_BDA_RF_TUNER_STANDARD_MODE: KSPROPERTY_BDA_FREQUENCY_FILTER = KSPROPERTY_BDA_FREQUENCY_FILTER(9i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_IPv4_FILTER(pub i32);
pub const KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_LIST_SIZE: KSPROPERTY_BDA_IPv4_FILTER = KSPROPERTY_BDA_IPv4_FILTER(0i32);
pub const KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_LIST: KSPROPERTY_BDA_IPv4_FILTER = KSPROPERTY_BDA_IPv4_FILTER(1i32);
pub const KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_MODE: KSPROPERTY_BDA_IPv4_FILTER = KSPROPERTY_BDA_IPv4_FILTER(2i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_IPv6_FILTER(pub i32);
pub const KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_LIST_SIZE: KSPROPERTY_BDA_IPv6_FILTER = KSPROPERTY_BDA_IPv6_FILTER(0i32);
pub const KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_LIST: KSPROPERTY_BDA_IPv6_FILTER = KSPROPERTY_BDA_IPv6_FILTER(1i32);
pub const KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_MODE: KSPROPERTY_BDA_IPv6_FILTER = KSPROPERTY_BDA_IPv6_FILTER(2i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_LNB_INFO(pub i32);
pub const KSPROPERTY_BDA_LNB_LOF_LOW_BAND: KSPROPERTY_BDA_LNB_INFO = KSPROPERTY_BDA_LNB_INFO(0i32);
pub const KSPROPERTY_BDA_LNB_LOF_HIGH_BAND: KSPROPERTY_BDA_LNB_INFO = KSPROPERTY_BDA_LNB_INFO(1i32);
pub const KSPROPERTY_BDA_LNB_SWITCH_FREQUENCY: KSPROPERTY_BDA_LNB_INFO = KSPROPERTY_BDA_LNB_INFO(2i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_NULL_TRANSFORM(pub i32);
pub const KSPROPERTY_BDA_NULL_TRANSFORM_START: KSPROPERTY_BDA_NULL_TRANSFORM = KSPROPERTY_BDA_NULL_TRANSFORM(0i32);
pub const KSPROPERTY_BDA_NULL_TRANSFORM_STOP: KSPROPERTY_BDA_NULL_TRANSFORM = KSPROPERTY_BDA_NULL_TRANSFORM(1i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_PIDFILTER(pub i32);
pub const KSPROPERTY_BDA_PIDFILTER_MAP_PIDS: KSPROPERTY_BDA_PIDFILTER = KSPROPERTY_BDA_PIDFILTER(0i32);
pub const KSPROPERTY_BDA_PIDFILTER_UNMAP_PIDS: KSPROPERTY_BDA_PIDFILTER = KSPROPERTY_BDA_PIDFILTER(1i32);
pub const KSPROPERTY_BDA_PIDFILTER_LIST_PIDS: KSPROPERTY_BDA_PIDFILTER = KSPROPERTY_BDA_PIDFILTER(2i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_PIN_CONTROL(pub i32);
pub const KSPROPERTY_BDA_PIN_ID: KSPROPERTY_BDA_PIN_CONTROL = KSPROPERTY_BDA_PIN_CONTROL(0i32);
pub const KSPROPERTY_BDA_PIN_TYPE: KSPROPERTY_BDA_PIN_CONTROL = KSPROPERTY_BDA_PIN_CONTROL(1i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_PIN_EVENT(pub i32);
pub const KSEVENT_BDA_PIN_CONNECTED: KSPROPERTY_BDA_PIN_EVENT = KSPROPERTY_BDA_PIN_EVENT(0i32);
pub const KSEVENT_BDA_PIN_DISCONNECTED: KSPROPERTY_BDA_PIN_EVENT = KSPROPERTY_BDA_PIN_EVENT(1i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSPROPERTY_BDA_RF_TUNER_CAPS_S(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSPROPERTY_BDA_RF_TUNER_SCAN_STATUS_S(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_KernelStreaming"))]
#[repr(C)]
pub struct KSPROPERTY_BDA_RF_TUNER_STANDARD_MODE_S(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSPROPERTY_BDA_RF_TUNER_STANDARD_S(i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_SIGNAL_STATS(pub i32);
pub const KSPROPERTY_BDA_SIGNAL_STRENGTH: KSPROPERTY_BDA_SIGNAL_STATS = KSPROPERTY_BDA_SIGNAL_STATS(0i32);
pub const KSPROPERTY_BDA_SIGNAL_QUALITY: KSPROPERTY_BDA_SIGNAL_STATS = KSPROPERTY_BDA_SIGNAL_STATS(1i32);
pub const KSPROPERTY_BDA_SIGNAL_PRESENT: KSPROPERTY_BDA_SIGNAL_STATS = KSPROPERTY_BDA_SIGNAL_STATS(2i32);
pub const KSPROPERTY_BDA_SIGNAL_LOCKED: KSPROPERTY_BDA_SIGNAL_STATS = KSPROPERTY_BDA_SIGNAL_STATS(3i32);
pub const KSPROPERTY_BDA_SAMPLE_TIME: KSPROPERTY_BDA_SIGNAL_STATS = KSPROPERTY_BDA_SIGNAL_STATS(4i32);
pub const KSPROPERTY_BDA_SIGNAL_LOCK_CAPS: KSPROPERTY_BDA_SIGNAL_STATS = KSPROPERTY_BDA_SIGNAL_STATS(5i32);
pub const KSPROPERTY_BDA_SIGNAL_LOCK_TYPE: KSPROPERTY_BDA_SIGNAL_STATS = KSPROPERTY_BDA_SIGNAL_STATS(6i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_TOPOLOGY(pub i32);
pub const KSPROPERTY_BDA_NODE_TYPES: KSPROPERTY_BDA_TOPOLOGY = KSPROPERTY_BDA_TOPOLOGY(0i32);
pub const KSPROPERTY_BDA_PIN_TYPES: KSPROPERTY_BDA_TOPOLOGY = KSPROPERTY_BDA_TOPOLOGY(1i32);
pub const KSPROPERTY_BDA_TEMPLATE_CONNECTIONS: KSPROPERTY_BDA_TOPOLOGY = KSPROPERTY_BDA_TOPOLOGY(2i32);
pub const KSPROPERTY_BDA_NODE_METHODS: KSPROPERTY_BDA_TOPOLOGY = KSPROPERTY_BDA_TOPOLOGY(3i32);
pub const KSPROPERTY_BDA_NODE_PROPERTIES: KSPROPERTY_BDA_TOPOLOGY = KSPROPERTY_BDA_TOPOLOGY(4i32);
pub const KSPROPERTY_BDA_NODE_EVENTS: KSPROPERTY_BDA_TOPOLOGY = KSPROPERTY_BDA_TOPOLOGY(5i32);
pub const KSPROPERTY_BDA_CONTROLLING_PIN_ID: KSPROPERTY_BDA_TOPOLOGY = KSPROPERTY_BDA_TOPOLOGY(6i32);
pub const KSPROPERTY_BDA_NODE_DESCRIPTORS: KSPROPERTY_BDA_TOPOLOGY = KSPROPERTY_BDA_TOPOLOGY(7i32);
#[repr(transparent)]
pub struct KSPROPERTY_BDA_VOID_TRANSFORM(pub i32);
pub const KSPROPERTY_BDA_VOID_TRANSFORM_START: KSPROPERTY_BDA_VOID_TRANSFORM = KSPROPERTY_BDA_VOID_TRANSFORM(0i32);
pub const KSPROPERTY_BDA_VOID_TRANSFORM_STOP: KSPROPERTY_BDA_VOID_TRANSFORM = KSPROPERTY_BDA_VOID_TRANSFORM(1i32);
#[repr(transparent)]
pub struct KSPROPERTY_IDS_BDA_TABLE(pub i32);
pub const KSPROPERTY_BDA_TABLE_SECTION: KSPROPERTY_IDS_BDA_TABLE = KSPROPERTY_IDS_BDA_TABLE(0i32);
#[repr(transparent)]
pub struct KSPROPERTY_IPSINK(pub u32);
pub const KSPROPERTY_IPSINK_MULTICASTLIST: KSPROPERTY_IPSINK = KSPROPERTY_IPSINK(0u32);
pub const KSPROPERTY_IPSINK_ADAPTER_DESCRIPTION: KSPROPERTY_IPSINK = KSPROPERTY_IPSINK(1u32);
pub const KSPROPERTY_IPSINK_ADAPTER_ADDRESS: KSPROPERTY_IPSINK = KSPROPERTY_IPSINK(2u32);
#[repr(C)]
pub struct KSPROPSETID_BdaAutodemodulate(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaCA(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaDigitalDemodulator(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaDiseqCommand(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaEthernetFilter(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaFrequencyFilter(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaIPv4Filter(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaIPv6Filter(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaLNBInfo(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaNullTransform(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaPIDFilter(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaPinControl(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaSignalStats(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaTableSection(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaTopology(i32);
#[repr(C)]
pub struct KSPROPSETID_BdaVoidTransform(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSP_BDA_NODE_PIN(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KSP_NODE_ESPID(i32);
#[repr(C)]
pub struct KS_BDA_FRAME_INFO(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KS_DATARANGE_BDA_ANTENNA(i32);
#[cfg(feature = "Win32_Media_KernelStreaming")]
#[repr(C)]
pub struct KS_DATARANGE_BDA_TRANSPORT(i32);
pub const LIBID_QuartzNetTypeLib: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1453877425, data2: 2772, data3: 4558, data4: [176, 58, 0, 32, 175, 11, 167, 112] };
pub const LIBID_QuartzTypeLib: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1453877424, data2: 2772, data3: 4558, data4: [176, 58, 0, 32, 175, 11, 167, 112] };
#[repr(transparent)]
pub struct LNB_Source(pub i32);
pub const BDA_LNB_SOURCE_NOT_SET: LNB_Source = LNB_Source(-1i32);
pub const BDA_LNB_SOURCE_NOT_DEFINED: LNB_Source = LNB_Source(0i32);
pub const BDA_LNB_SOURCE_A: LNB_Source = LNB_Source(1i32);
pub const BDA_LNB_SOURCE_B: LNB_Source = LNB_Source(2i32);
pub const BDA_LNB_SOURCE_C: LNB_Source = LNB_Source(3i32);
pub const BDA_LNB_SOURCE_D: LNB_Source = LNB_Source(4i32);
pub const BDA_LNB_SOURCE_MAX: LNB_Source = LNB_Source(5i32);
#[repr(C)]
pub struct LONG_SECTION(i32);
#[repr(C)]
pub struct LanguageComponentType(i32);
#[repr(C)]
pub struct LanguageInfo(i32);
#[repr(transparent)]
pub struct LicenseEventBlockReason(pub i32);
pub const LIC_BadLicense: LicenseEventBlockReason = LicenseEventBlockReason(0i32);
pub const LIC_NeedIndiv: LicenseEventBlockReason = LicenseEventBlockReason(1i32);
pub const LIC_Expired: LicenseEventBlockReason = LicenseEventBlockReason(2i32);
pub const LIC_NeedActivation: LicenseEventBlockReason = LicenseEventBlockReason(3i32);
pub const LIC_ExtenderBlocked: LicenseEventBlockReason = LicenseEventBlockReason(4i32);
#[repr(transparent)]
pub struct LocationCodeSchemeType(pub i32);
pub const SCTE_18: LocationCodeSchemeType = LocationCodeSchemeType(0i32);
#[repr(C)]
pub struct Locator(i32);
pub const MAX_COUNTRY_CODE_STRING: u32 = 3u32;
pub const MAX_DEINTERLACE_DEVICE_GUIDS: u32 = 32u32;
pub const MAX_DEINTERLACE_SURFACES: u32 = 32u32;
pub const MAX_ERROR_TEXT_LEN: u32 = 160u32;
pub const MAX_FILTER_NAME: u32 = 128u32;
pub const MAX_PIN_NAME: u32 = 128u32;
pub const MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140u32;
pub const MEDIASUBTYPE_CPFilters_Processed: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1185791272, data2: 28624, data3: 18326, data4: [147, 178, 21, 92, 81, 220, 4, 141] };
pub const MEDIASUBTYPE_ETDTFilter_Tagged: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229776, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const MEDIATYPE_MPEG2_PACK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 911358739, data2: 36581, data3: 4561, data4: [140, 163, 0, 96, 176, 87, 102, 74] };
#[repr(transparent)]
pub struct MEDIA_SAMPLE_CONTENT(pub i32);
pub const MEDIA_TRANSPORT_PACKET: MEDIA_SAMPLE_CONTENT = MEDIA_SAMPLE_CONTENT(0i32);
pub const MEDIA_ELEMENTARY_STREAM: MEDIA_SAMPLE_CONTENT = MEDIA_SAMPLE_CONTENT(1i32);
pub const MEDIA_MPEG2_PSI: MEDIA_SAMPLE_CONTENT = MEDIA_SAMPLE_CONTENT(2i32);
pub const MEDIA_TRANSPORT_PAYLOAD: MEDIA_SAMPLE_CONTENT = MEDIA_SAMPLE_CONTENT(3i32);
pub const MIN_DIMENSION: u32 = 1u32;
#[repr(transparent)]
pub struct MMSSF_GET_INFORMATION_FLAGS(pub u32);
pub const MMSSF_HASCLOCK: MMSSF_GET_INFORMATION_FLAGS = MMSSF_GET_INFORMATION_FLAGS(1u32);
pub const MMSSF_SUPPORTSEEK: MMSSF_GET_INFORMATION_FLAGS = MMSSF_GET_INFORMATION_FLAGS(2u32);
pub const MMSSF_ASYNCHRONOUS: MMSSF_GET_INFORMATION_FLAGS = MMSSF_GET_INFORMATION_FLAGS(4u32);
pub const MPBOOL_FALSE: u32 = 0u32;
pub const MPBOOL_TRUE: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct MPEG1VIDEOINFO(i32);
#[cfg(feature = "Win32_Media_Audio")]
#[repr(C)]
pub struct MPEG1WAVEFORMAT(i32);
#[repr(C)]
pub struct MPEG2Component(i32);
#[repr(C)]
pub struct MPEG2ComponentType(i32);
#[repr(transparent)]
pub struct MPEG2StreamType(pub i32);
pub const BDA_UNITIALIZED_MPEG2STREAMTYPE: MPEG2StreamType = MPEG2StreamType(-1i32);
pub const Reserved1: MPEG2StreamType = MPEG2StreamType(0i32);
pub const ISO_IEC_11172_2_VIDEO: MPEG2StreamType = MPEG2StreamType(1i32);
pub const ISO_IEC_13818_2_VIDEO: MPEG2StreamType = MPEG2StreamType(2i32);
pub const ISO_IEC_11172_3_AUDIO: MPEG2StreamType = MPEG2StreamType(3i32);
pub const ISO_IEC_13818_3_AUDIO: MPEG2StreamType = MPEG2StreamType(4i32);
pub const ISO_IEC_13818_1_PRIVATE_SECTION: MPEG2StreamType = MPEG2StreamType(5i32);
pub const ISO_IEC_13818_1_PES: MPEG2StreamType = MPEG2StreamType(6i32);
pub const ISO_IEC_13522_MHEG: MPEG2StreamType = MPEG2StreamType(7i32);
pub const ANNEX_A_DSM_CC: MPEG2StreamType = MPEG2StreamType(8i32);
pub const ITU_T_REC_H_222_1: MPEG2StreamType = MPEG2StreamType(9i32);
pub const ISO_IEC_13818_6_TYPE_A: MPEG2StreamType = MPEG2StreamType(10i32);
pub const ISO_IEC_13818_6_TYPE_B: MPEG2StreamType = MPEG2StreamType(11i32);
pub const ISO_IEC_13818_6_TYPE_C: MPEG2StreamType = MPEG2StreamType(12i32);
pub const ISO_IEC_13818_6_TYPE_D: MPEG2StreamType = MPEG2StreamType(13i32);
pub const ISO_IEC_13818_1_AUXILIARY: MPEG2StreamType = MPEG2StreamType(14i32);
pub const ISO_IEC_13818_7_AUDIO: MPEG2StreamType = MPEG2StreamType(15i32);
pub const ISO_IEC_14496_2_VISUAL: MPEG2StreamType = MPEG2StreamType(16i32);
pub const ISO_IEC_14496_3_AUDIO: MPEG2StreamType = MPEG2StreamType(17i32);
pub const ISO_IEC_14496_1_IN_PES: MPEG2StreamType = MPEG2StreamType(18i32);
pub const ISO_IEC_14496_1_IN_SECTION: MPEG2StreamType = MPEG2StreamType(19i32);
pub const ISO_IEC_13818_6_DOWNLOAD: MPEG2StreamType = MPEG2StreamType(20i32);
pub const METADATA_IN_PES: MPEG2StreamType = MPEG2StreamType(21i32);
pub const METADATA_IN_SECTION: MPEG2StreamType = MPEG2StreamType(22i32);
pub const METADATA_IN_DATA_CAROUSEL: MPEG2StreamType = MPEG2StreamType(23i32);
pub const METADATA_IN_OBJECT_CAROUSEL: MPEG2StreamType = MPEG2StreamType(24i32);
pub const METADATA_IN_DOWNLOAD_PROTOCOL: MPEG2StreamType = MPEG2StreamType(25i32);
pub const IRPM_STREAMM: MPEG2StreamType = MPEG2StreamType(26i32);
pub const ITU_T_H264: MPEG2StreamType = MPEG2StreamType(27i32);
pub const ISO_IEC_13818_1_RESERVED: MPEG2StreamType = MPEG2StreamType(28i32);
pub const USER_PRIVATE: MPEG2StreamType = MPEG2StreamType(16i32);
pub const HEVC_VIDEO_OR_TEMPORAL_VIDEO: MPEG2StreamType = MPEG2StreamType(36i32);
pub const HEVC_TEMPORAL_VIDEO_SUBSET: MPEG2StreamType = MPEG2StreamType(37i32);
pub const ISO_IEC_USER_PRIVATE: MPEG2StreamType = MPEG2StreamType(128i32);
pub const DOLBY_AC3_AUDIO: MPEG2StreamType = MPEG2StreamType(129i32);
pub const DOLBY_DIGITAL_PLUS_AUDIO_ATSC: MPEG2StreamType = MPEG2StreamType(135i32);
#[repr(C)]
pub struct MPEG2TuneRequest(i32);
#[repr(C)]
pub struct MPEG2TuneRequestFactory(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct MPEG2VIDEOINFO(i32);
#[repr(transparent)]
pub struct MPEG2VIDEOINFO_FLAGS(pub u32);
pub const AMMPEG2_DoPanScan: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(1u32);
pub const AMMPEG2_DVDLine21Field1: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(2u32);
pub const AMMPEG2_DVDLine21Field2: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(4u32);
pub const AMMPEG2_SourceIsLetterboxed: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(8u32);
pub const AMMPEG2_FilmCameraMode: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(16u32);
pub const AMMPEG2_LetterboxAnalogOut: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(32u32);
pub const AMMPEG2_DSS_UserData: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(64u32);
pub const AMMPEG2_DVB_UserData: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(128u32);
pub const AMMPEG2_27MhzTimebase: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(256u32);
pub const AMMPEG2_WidescreenAnalogOut: MPEG2VIDEOINFO_FLAGS = MPEG2VIDEOINFO_FLAGS(512u32);
pub const MPEG2_BASE: u32 = 512u32;
pub const MPEG2_E_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220991i32 as _);
pub const MPEG2_E_BUFFER_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220967i32 as _);
pub const MPEG2_E_DATA_SOURCE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220970i32 as _);
pub const MPEG2_E_DII_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220969i32 as _);
pub const MPEG2_E_DSHOW_PIN_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220968i32 as _);
pub const MPEG2_E_DSI_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220982i32 as _);
pub const MPEG2_E_FILE_OFFSET_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220974i32 as _);
pub const MPEG2_E_INCORRECT_DESCRIPTOR_TAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220963i32 as _);
pub const MPEG2_E_INVALID_CAROUSEL_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220980i32 as _);
pub const MPEG2_E_INVALID_SG_OBJECT_KIND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220978i32 as _);
pub const MPEG2_E_INVALID_UDP_PORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220971i32 as _);
pub const MPEG2_E_MALFORMED_DSMCC_MESSAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220979i32 as _);
pub const MPEG2_E_MALFORMED_TABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220989i32 as _);
pub const MPEG2_E_MISSING_SECTIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220966i32 as _);
pub const MPEG2_E_NEXT_TABLE_OPS_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220964i32 as _);
pub const MPEG2_E_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220987i32 as _);
pub const MPEG2_E_OBJECT_KIND_NOT_A_DIRECTORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220976i32 as _);
pub const MPEG2_E_OBJECT_KIND_NOT_A_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220975i32 as _);
pub const MPEG2_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220977i32 as _);
pub const MPEG2_E_OUT_OF_BOUNDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220990i32 as _);
pub const MPEG2_E_REGISTRY_ACCESS_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220972i32 as _);
pub const MPEG2_E_SECTION_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220986i32 as _);
pub const MPEG2_E_SERVER_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220981i32 as _);
pub const MPEG2_E_SERVICE_ID_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220984i32 as _);
pub const MPEG2_E_SERVICE_PMT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220983i32 as _);
pub const MPEG2_E_STREAM_STOPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220973i32 as _);
pub const MPEG2_E_TOO_MANY_SECTIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220965i32 as _);
pub const MPEG2_E_TX_STREAM_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220985i32 as _);
pub const MPEG2_E_UNDEFINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220988i32 as _);
pub const MPEG2_E_UNINITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MPEG2_FILTER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MPEG2_FILTER2(i32);
pub const MPEG2_FILTER_VERSION_1_SIZE: u32 = 124u32;
pub const MPEG2_FILTER_VERSION_2_SIZE: u32 = 133u32;
pub const MPEG2_PROGRAM_DIRECTORY_PES_PACKET: u32 = 2u32;
pub const MPEG2_PROGRAM_ELEMENTARY_STREAM: u32 = 1u32;
pub const MPEG2_PROGRAM_PACK_HEADER: u32 = 3u32;
pub const MPEG2_PROGRAM_PES_STREAM: u32 = 4u32;
pub const MPEG2_PROGRAM_STREAM_MAP: u32 = 0u32;
pub const MPEG2_PROGRAM_SYSTEM_HEADER: u32 = 5u32;
pub const MPEG2_S_MORE_DATA_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262656i32 as _);
pub const MPEG2_S_MPE_INFO_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262660i32 as _);
pub const MPEG2_S_MPE_INFO_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262661i32 as _);
pub const MPEG2_S_NEW_MODULE_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262662i32 as _);
pub const MPEG2_S_NO_MORE_DATA_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262657i32 as _);
pub const MPEG2_S_SG_INFO_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262658i32 as _);
pub const MPEG2_S_SG_INFO_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262659i32 as _);
#[repr(C)]
pub struct MPEG2_TRANSPORT_STRIDE(i32);
#[cfg(feature = "Win32_Media_Audio")]
#[repr(C)]
pub struct MPEGLAYER3WAVEFORMAT(i32);
#[repr(transparent)]
pub struct MPEGLAYER3WAVEFORMAT_FLAGS(pub u32);
pub const MPEGLAYER3_FLAG_PADDING_ISO: MPEGLAYER3WAVEFORMAT_FLAGS = MPEGLAYER3WAVEFORMAT_FLAGS(0u32);
pub const MPEGLAYER3_FLAG_PADDING_ON: MPEGLAYER3WAVEFORMAT_FLAGS = MPEGLAYER3WAVEFORMAT_FLAGS(1u32);
pub const MPEGLAYER3_FLAG_PADDING_OFF: MPEGLAYER3WAVEFORMAT_FLAGS = MPEGLAYER3WAVEFORMAT_FLAGS(2u32);
#[repr(C)]
pub struct MPEG_BCS_DEMUX(i32);
pub const MPEG_CAT_PID: u32 = 1u32;
pub const MPEG_CAT_TID: u32 = 1u32;
#[repr(C)]
pub struct MPEG_CONTEXT(i32);
#[repr(transparent)]
pub struct MPEG_CONTEXT_TYPE(pub i32);
pub const MPEG_CONTEXT_BCS_DEMUX: MPEG_CONTEXT_TYPE = MPEG_CONTEXT_TYPE(0i32);
pub const MPEG_CONTEXT_WINSOCK: MPEG_CONTEXT_TYPE = MPEG_CONTEXT_TYPE(1i32);
#[repr(transparent)]
pub struct MPEG_CURRENT_NEXT_BIT(pub i32);
pub const MPEG_SECTION_IS_NEXT: MPEG_CURRENT_NEXT_BIT = MPEG_CURRENT_NEXT_BIT(0i32);
pub const MPEG_SECTION_IS_CURRENT: MPEG_CURRENT_NEXT_BIT = MPEG_CURRENT_NEXT_BIT(1i32);
#[repr(C)]
pub struct MPEG_DATE(i32);
#[repr(C)]
pub struct MPEG_DATE_AND_TIME(i32);
#[repr(C)]
pub struct MPEG_HEADER_BITS(i32);
#[repr(C)]
pub struct MPEG_HEADER_BITS_MIDL(i32);
#[repr(C)]
pub struct MPEG_HEADER_VERSION_BITS(i32);
#[repr(C)]
pub struct MPEG_HEADER_VERSION_BITS_MIDL(i32);
#[repr(C)]
pub struct MPEG_PACKET_LIST(i32);
pub const MPEG_PAT_PID: u32 = 0u32;
pub const MPEG_PAT_TID: u32 = 0u32;
pub const MPEG_PMT_TID: u32 = 2u32;
#[repr(transparent)]
pub struct MPEG_REQUEST_TYPE(pub i32);
pub const MPEG_RQST_UNKNOWN: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(0i32);
pub const MPEG_RQST_GET_SECTION: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(1i32);
pub const MPEG_RQST_GET_SECTION_ASYNC: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(2i32);
pub const MPEG_RQST_GET_TABLE: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(3i32);
pub const MPEG_RQST_GET_TABLE_ASYNC: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(4i32);
pub const MPEG_RQST_GET_SECTIONS_STREAM: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(5i32);
pub const MPEG_RQST_GET_PES_STREAM: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(6i32);
pub const MPEG_RQST_GET_TS_STREAM: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(7i32);
pub const MPEG_RQST_START_MPE_STREAM: MPEG_REQUEST_TYPE = MPEG_REQUEST_TYPE(8i32);
#[repr(C)]
pub struct MPEG_RQST_PACKET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MPEG_SERVICE_REQUEST(i32);
#[repr(C)]
pub struct MPEG_SERVICE_RESPONSE(i32);
#[repr(C)]
pub struct MPEG_STREAM_BUFFER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MPEG_STREAM_FILTER(i32);
#[repr(C)]
pub struct MPEG_TIME(i32);
pub const MPEG_TSDT_PID: u32 = 2u32;
pub const MPEG_TSDT_TID: u32 = 3u32;
#[repr(C)]
pub struct MPEG_WINSOCK(i32);
#[repr(C)]
pub struct MPE_ELEMENT(i32);
pub const MPF_ENVLP_BEGIN_CURRENTVAL: u32 = 1u32;
pub const MPF_ENVLP_BEGIN_NEUTRALVAL: u32 = 2u32;
pub const MPF_ENVLP_STANDARD: u32 = 0u32;
pub const MPF_PUNCHIN_NOW: u32 = 1u32;
pub const MPF_PUNCHIN_REFTIME: u32 = 0u32;
pub const MPF_PUNCHIN_STOPPED: u32 = 2u32;
#[repr(transparent)]
pub struct MP_CURVE_TYPE(pub i32);
pub const MP_CURVE_JUMP: MP_CURVE_TYPE = MP_CURVE_TYPE(1i32);
pub const MP_CURVE_LINEAR: MP_CURVE_TYPE = MP_CURVE_TYPE(2i32);
pub const MP_CURVE_SQUARE: MP_CURVE_TYPE = MP_CURVE_TYPE(4i32);
pub const MP_CURVE_INVSQUARE: MP_CURVE_TYPE = MP_CURVE_TYPE(8i32);
pub const MP_CURVE_SINE: MP_CURVE_TYPE = MP_CURVE_TYPE(16i32);
#[repr(C)]
pub struct MP_ENVELOPE_SEGMENT(i32);
#[repr(C)]
pub struct MP_PARAMINFO(i32);
#[repr(transparent)]
pub struct MP_TYPE(pub i32);
pub const MPT_INT: MP_TYPE = MP_TYPE(0i32);
pub const MPT_FLOAT: MP_TYPE = MP_TYPE(1i32);
pub const MPT_BOOL: MP_TYPE = MP_TYPE(2i32);
pub const MPT_ENUM: MP_TYPE = MP_TYPE(3i32);
pub const MPT_MAX: MP_TYPE = MP_TYPE(4i32);
pub const MSDRI_S_MMI_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2i32 as _);
pub const MSDRI_S_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1i32 as _);
#[repr(C)]
pub struct MSEventBinder(i32);
pub const MSPID_PrimaryAudio: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2740974955, data2: 40922, data3: 4560, data4: [143, 223, 0, 192, 79, 217, 24, 157] };
pub const MSPID_PrimaryVideo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2740974954, data2: 40922, data3: 4560, data4: [143, 223, 0, 192, 79, 217, 24, 157] };
#[repr(C)]
pub struct MSVidAnalogCaptureToCCA(i32);
#[repr(C)]
pub struct MSVidAnalogCaptureToDataServices(i32);
#[repr(C)]
pub struct MSVidAnalogCaptureToOverlayMixer(i32);
#[repr(C)]
pub struct MSVidAnalogCaptureToStreamBufferSink(i32);
#[repr(C)]
pub struct MSVidAnalogCaptureToXDS(i32);
#[repr(C)]
pub struct MSVidAnalogTVToEncoder(i32);
#[repr(C)]
pub struct MSVidAnalogTunerDevice(i32);
#[repr(C)]
pub struct MSVidAudioRenderer(i32);
#[repr(C)]
pub struct MSVidAudioRendererDevices(i32);
#[repr(C)]
pub struct MSVidBDATunerDevice(i32);
#[repr(C)]
pub struct MSVidCCA(i32);
#[repr(C)]
pub struct MSVidCCAToStreamBufferSink(i32);
#[repr(transparent)]
pub struct MSVidCCService(pub i32);
impl MSVidCCService {
    pub const None: Self = Self(0i32);
    pub const Caption1: Self = Self(1i32);
    pub const Caption2: Self = Self(2i32);
    pub const Text1: Self = Self(3i32);
    pub const Text2: Self = Self(4i32);
    pub const XDS: Self = Self(5i32);
}
#[repr(C)]
pub struct MSVidCCToAR(i32);
#[repr(C)]
pub struct MSVidCCToVMR(i32);
#[repr(C)]
pub struct MSVidClosedCaptioning(i32);
#[repr(C)]
pub struct MSVidClosedCaptioningSI(i32);
#[repr(C)]
pub struct MSVidCtl(i32);
#[repr(transparent)]
pub struct MSVidCtlButtonstate(pub i32);
pub const MSVIDCTL_LEFT_BUTTON: MSVidCtlButtonstate = MSVidCtlButtonstate(1i32);
pub const MSVIDCTL_RIGHT_BUTTON: MSVidCtlButtonstate = MSVidCtlButtonstate(2i32);
pub const MSVIDCTL_MIDDLE_BUTTON: MSVidCtlButtonstate = MSVidCtlButtonstate(4i32);
pub const MSVIDCTL_X_BUTTON1: MSVidCtlButtonstate = MSVidCtlButtonstate(8i32);
pub const MSVIDCTL_X_BUTTON2: MSVidCtlButtonstate = MSVidCtlButtonstate(16i32);
pub const MSVIDCTL_SHIFT: MSVidCtlButtonstate = MSVidCtlButtonstate(1i32);
pub const MSVIDCTL_CTRL: MSVidCtlButtonstate = MSVidCtlButtonstate(2i32);
pub const MSVIDCTL_ALT: MSVidCtlButtonstate = MSVidCtlButtonstate(4i32);
#[repr(transparent)]
pub struct MSVidCtlStateList(pub i32);
pub const STATE_UNBUILT: MSVidCtlStateList = MSVidCtlStateList(-1i32);
pub const STATE_STOP: MSVidCtlStateList = MSVidCtlStateList(0i32);
pub const STATE_PAUSE: MSVidCtlStateList = MSVidCtlStateList(1i32);
pub const STATE_PLAY: MSVidCtlStateList = MSVidCtlStateList(2i32);
#[repr(C)]
pub struct MSVidDataServices(i32);
#[repr(C)]
pub struct MSVidDataServicesToStreamBufferSink(i32);
#[repr(C)]
pub struct MSVidDataServicesToXDS(i32);
#[repr(C)]
pub struct MSVidDevice(i32);
#[repr(C)]
pub struct MSVidDevice2(i32);
#[repr(C)]
pub struct MSVidDigitalCaptureToCCA(i32);
#[repr(C)]
pub struct MSVidDigitalCaptureToITV(i32);
#[repr(C)]
pub struct MSVidDigitalCaptureToStreamBufferSink(i32);
#[repr(C)]
pub struct MSVidEVR(i32);
#[repr(C)]
pub struct MSVidEncoder(i32);
#[repr(C)]
pub struct MSVidEncoderToStreamBufferSink(i32);
#[repr(C)]
pub struct MSVidFeature(i32);
#[repr(C)]
pub struct MSVidFeatures(i32);
#[repr(C)]
pub struct MSVidFilePlaybackDevice(i32);
#[repr(C)]
pub struct MSVidFilePlaybackToAudioRenderer(i32);
#[repr(C)]
pub struct MSVidFilePlaybackToVideoRenderer(i32);
#[repr(C)]
pub struct MSVidGenericComposite(i32);
#[repr(C)]
pub struct MSVidGenericSink(i32);
#[repr(C)]
pub struct MSVidITVCapture(i32);
#[repr(C)]
pub struct MSVidITVPlayback(i32);
#[repr(C)]
pub struct MSVidITVToStreamBufferSink(i32);
#[repr(C)]
pub struct MSVidInputDevice(i32);
#[repr(C)]
pub struct MSVidInputDevices(i32);
#[repr(C)]
pub struct MSVidMPEG2DecoderToClosedCaptioning(i32);
#[repr(C)]
pub struct MSVidOutput(i32);
#[repr(C)]
pub struct MSVidOutputDevices(i32);
#[repr(C)]
pub struct MSVidRect(i32);
#[repr(C)]
pub struct MSVidSBESourceToCC(i32);
#[repr(C)]
pub struct MSVidSBESourceToGenericSink(i32);
#[repr(C)]
pub struct MSVidSBESourceToITV(i32);
#[repr(transparent)]
pub struct MSVidSegmentType(pub i32);
pub const MSVidSEG_SOURCE: MSVidSegmentType = MSVidSegmentType(0i32);
pub const MSVidSEG_XFORM: MSVidSegmentType = MSVidSegmentType(1i32);
pub const MSVidSEG_DEST: MSVidSegmentType = MSVidSegmentType(2i32);
#[repr(transparent)]
pub struct MSVidSinkStreams(pub i32);
pub const MSVidSink_Video: MSVidSinkStreams = MSVidSinkStreams(1i32);
pub const MSVidSink_Audio: MSVidSinkStreams = MSVidSinkStreams(2i32);
pub const MSVidSink_Other: MSVidSinkStreams = MSVidSinkStreams(4i32);
#[repr(C)]
pub struct MSVidStreamBufferRecordingControl(i32);
#[repr(C)]
pub struct MSVidStreamBufferSink(i32);
#[repr(C)]
pub struct MSVidStreamBufferSource(i32);
#[repr(C)]
pub struct MSVidStreamBufferSourceToVideoRenderer(i32);
#[repr(C)]
pub struct MSVidStreamBufferV2Source(i32);
#[repr(C)]
pub struct MSVidVMR9(i32);
#[repr(C)]
pub struct MSVidVideoInputDevice(i32);
#[repr(C)]
pub struct MSVidVideoPlaybackDevice(i32);
#[repr(C)]
pub struct MSVidVideoRenderer(i32);
#[repr(C)]
pub struct MSVidVideoRendererDevices(i32);
#[repr(C)]
pub struct MSVidWebDVD(i32);
#[repr(C)]
pub struct MSVidWebDVDAdm(i32);
#[repr(C)]
pub struct MSVidWebDVDToAudioRenderer(i32);
#[repr(C)]
pub struct MSVidWebDVDToVideoRenderer(i32);
#[repr(C)]
pub struct MSVidXDS(i32);
#[repr(transparent)]
pub struct MSViddispidList(pub i32);
pub const dispidInputs: MSViddispidList = MSViddispidList(0i32);
pub const dispidOutputs: MSViddispidList = MSViddispidList(1i32);
pub const dispid_Inputs: MSViddispidList = MSViddispidList(2i32);
pub const dispid_Outputs: MSViddispidList = MSViddispidList(3i32);
pub const dispidVideoRenderers: MSViddispidList = MSViddispidList(4i32);
pub const dispidAudioRenderers: MSViddispidList = MSViddispidList(5i32);
pub const dispidFeatures: MSViddispidList = MSViddispidList(6i32);
pub const dispidInput: MSViddispidList = MSViddispidList(7i32);
pub const dispidOutput: MSViddispidList = MSViddispidList(8i32);
pub const dispidVideoRenderer: MSViddispidList = MSViddispidList(9i32);
pub const dispidAudioRenderer: MSViddispidList = MSViddispidList(10i32);
pub const dispidSelectedFeatures: MSViddispidList = MSViddispidList(11i32);
pub const dispidView: MSViddispidList = MSViddispidList(12i32);
pub const dispidBuild: MSViddispidList = MSViddispidList(13i32);
pub const dispidPause: MSViddispidList = MSViddispidList(14i32);
pub const dispidRun: MSViddispidList = MSViddispidList(15i32);
pub const dispidStop: MSViddispidList = MSViddispidList(16i32);
pub const dispidDecompose: MSViddispidList = MSViddispidList(17i32);
pub const dispidDisplaySize: MSViddispidList = MSViddispidList(18i32);
pub const dispidMaintainAspectRatio: MSViddispidList = MSViddispidList(19i32);
pub const dispidColorKey: MSViddispidList = MSViddispidList(20i32);
pub const dispidStateChange: MSViddispidList = MSViddispidList(21i32);
pub const dispidgetState: MSViddispidList = MSViddispidList(22i32);
pub const dispidunbind: MSViddispidList = MSViddispidList(23i32);
pub const dispidbind: MSViddispidList = MSViddispidList(24i32);
pub const dispidDisableVideo: MSViddispidList = MSViddispidList(25i32);
pub const dispidDisableAudio: MSViddispidList = MSViddispidList(26i32);
pub const dispidViewNext: MSViddispidList = MSViddispidList(27i32);
pub const dispidServiceP: MSViddispidList = MSViddispidList(28i32);
#[repr(transparent)]
pub struct MUX_PID_TYPE(pub i32);
pub const PID_OTHER: MUX_PID_TYPE = MUX_PID_TYPE(-1i32);
pub const PID_ELEMENTARY_STREAM: MUX_PID_TYPE = MUX_PID_TYPE(0i32);
pub const PID_MPEG2_SECTION_PSI_SI: MUX_PID_TYPE = MUX_PID_TYPE(1i32);
#[repr(C)]
pub struct MainAVIHeader(i32);
#[repr(transparent)]
pub struct ModulationType(pub i32);
pub const BDA_MOD_NOT_SET: ModulationType = ModulationType(-1i32);
pub const BDA_MOD_NOT_DEFINED: ModulationType = ModulationType(0i32);
pub const BDA_MOD_16QAM: ModulationType = ModulationType(1i32);
pub const BDA_MOD_32QAM: ModulationType = ModulationType(2i32);
pub const BDA_MOD_64QAM: ModulationType = ModulationType(3i32);
pub const BDA_MOD_80QAM: ModulationType = ModulationType(4i32);
pub const BDA_MOD_96QAM: ModulationType = ModulationType(5i32);
pub const BDA_MOD_112QAM: ModulationType = ModulationType(6i32);
pub const BDA_MOD_128QAM: ModulationType = ModulationType(7i32);
pub const BDA_MOD_160QAM: ModulationType = ModulationType(8i32);
pub const BDA_MOD_192QAM: ModulationType = ModulationType(9i32);
pub const BDA_MOD_224QAM: ModulationType = ModulationType(10i32);
pub const BDA_MOD_256QAM: ModulationType = ModulationType(11i32);
pub const BDA_MOD_320QAM: ModulationType = ModulationType(12i32);
pub const BDA_MOD_384QAM: ModulationType = ModulationType(13i32);
pub const BDA_MOD_448QAM: ModulationType = ModulationType(14i32);
pub const BDA_MOD_512QAM: ModulationType = ModulationType(15i32);
pub const BDA_MOD_640QAM: ModulationType = ModulationType(16i32);
pub const BDA_MOD_768QAM: ModulationType = ModulationType(17i32);
pub const BDA_MOD_896QAM: ModulationType = ModulationType(18i32);
pub const BDA_MOD_1024QAM: ModulationType = ModulationType(19i32);
pub const BDA_MOD_QPSK: ModulationType = ModulationType(20i32);
pub const BDA_MOD_BPSK: ModulationType = ModulationType(21i32);
pub const BDA_MOD_OQPSK: ModulationType = ModulationType(22i32);
pub const BDA_MOD_8VSB: ModulationType = ModulationType(23i32);
pub const BDA_MOD_16VSB: ModulationType = ModulationType(24i32);
pub const BDA_MOD_ANALOG_AMPLITUDE: ModulationType = ModulationType(25i32);
pub const BDA_MOD_ANALOG_FREQUENCY: ModulationType = ModulationType(26i32);
pub const BDA_MOD_8PSK: ModulationType = ModulationType(27i32);
pub const BDA_MOD_RF: ModulationType = ModulationType(28i32);
pub const BDA_MOD_16APSK: ModulationType = ModulationType(29i32);
pub const BDA_MOD_32APSK: ModulationType = ModulationType(30i32);
pub const BDA_MOD_NBC_QPSK: ModulationType = ModulationType(31i32);
pub const BDA_MOD_NBC_8PSK: ModulationType = ModulationType(32i32);
pub const BDA_MOD_DIRECTV: ModulationType = ModulationType(33i32);
pub const BDA_MOD_ISDB_T_TMCC: ModulationType = ModulationType(34i32);
pub const BDA_MOD_ISDB_S_TMCC: ModulationType = ModulationType(35i32);
pub const BDA_MOD_MAX: ModulationType = ModulationType(36i32);
#[repr(C)]
pub struct Mpeg2Data(i32);
#[repr(C)]
pub struct Mpeg2DataLib(i32);
#[repr(C)]
pub struct Mpeg2Stream(i32);
#[repr(C)]
pub struct Mpeg2TableSampleHdr(i32);
#[repr(C)]
pub struct NORMALIZEDRECT(i32);
#[repr(transparent)]
pub struct OA_BOOL(pub i32);
pub const OATRUE: OA_BOOL = OA_BOOL(-1i32);
pub const OAFALSE: OA_BOOL = OA_BOOL(0i32);
pub const OCUR_PAIRING_PROTOCOL_VERSION: u32 = 2u32;
#[repr(transparent)]
pub struct OUTPUT_STATE(pub u32);
pub const Disabled: OUTPUT_STATE = OUTPUT_STATE(0u32);
pub const ReadData: OUTPUT_STATE = OUTPUT_STATE(1u32);
pub const RenderData: OUTPUT_STATE = OUTPUT_STATE(2u32);
pub const PARENTAL_CONTROL_ATTRIB_DIALOGUE: u32 = 515u32;
pub const PARENTAL_CONTROL_ATTRIB_FANTASY: u32 = 516u32;
pub const PARENTAL_CONTROL_ATTRIB_LANGUAGE: u32 = 513u32;
pub const PARENTAL_CONTROL_ATTRIB_SEXUAL: u32 = 514u32;
pub const PARENTAL_CONTROL_ATTRIB_VIOLENCE: u32 = 512u32;
pub const PARENTAL_CONTROL_CONTENT_RATING: u32 = 256u32;
pub const PARENTAL_CONTROL_TIME_RANGE: u32 = 1u32;
pub const PARENTAL_CONTROL_VALUE_UNDEFINED: u32 = 0u32;
#[repr(C)]
pub struct PBDAParentalControl(i32);
#[repr(C)]
pub struct PBDA_ALWAYS_TUNE_IN_MUX(i32);
pub const PBDA_AUX_CONNECTOR_TYPE_Composite: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4129917772, data2: 50981, data3: 19778, data4: [132, 155, 65, 11, 187, 20, 234, 98] };
pub const PBDA_AUX_CONNECTOR_TYPE_SVideo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2699625972, data2: 9417, data3: 19028, data4: [183, 97, 33, 51, 85, 239, 193, 58] };
pub const PBDA_Encoder_Audio_AlgorithmType_AC3: u32 = 1u32;
pub const PBDA_Encoder_Audio_AlgorithmType_MPEG1LayerII: u32 = 0u32;
pub const PBDA_Encoder_BitrateMode_Average: u32 = 3u32;
pub const PBDA_Encoder_BitrateMode_Constant: u32 = 1u32;
pub const PBDA_Encoder_BitrateMode_Variable: u32 = 2u32;
pub const PBDA_Encoder_Video_AVC: u32 = 1u32;
pub const PBDA_Encoder_Video_H264: u32 = 1u32;
pub const PBDA_Encoder_Video_MPEG2PartII: u32 = 0u32;
pub const PBDA_Encoder_Video_MPEG4Part10: u32 = 1u32;
pub const PBDA_PAIRING_PROTOCOL_VERSION: u32 = 3u32;
#[repr(C)]
pub struct PBDA_TAG_ATTRIBUTE(i32);
#[repr(C)]
pub struct PDXVA2SW_CREATEVIDEOPROCESSDEVICE(i32);
#[repr(C)]
pub struct PDXVA2SW_DESTROYVIDEOPROCESSDEVICE(i32);
#[repr(C)]
pub struct PDXVA2SW_GETFILTERPROPERTYRANGE(i32);
#[repr(C)]
pub struct PDXVA2SW_GETPROCAMPRANGE(i32);
#[repr(C)]
pub struct PDXVA2SW_GETVIDEOPROCESSORCAPS(i32);
#[repr(C)]
pub struct PDXVA2SW_GETVIDEOPROCESSORRENDERTARGETCOUNT(i32);
#[repr(C)]
pub struct PDXVA2SW_GETVIDEOPROCESSORRENDERTARGETS(i32);
#[repr(C)]
pub struct PDXVA2SW_GETVIDEOPROCESSORSUBSTREAMFORMATCOUNT(i32);
#[repr(C)]
pub struct PDXVA2SW_GETVIDEOPROCESSORSUBSTREAMFORMATS(i32);
#[repr(C)]
pub struct PDXVA2SW_VIDEOPROCESSBEGINFRAME(i32);
#[repr(C)]
pub struct PDXVA2SW_VIDEOPROCESSBLT(i32);
#[repr(C)]
pub struct PDXVA2SW_VIDEOPROCESSENDFRAME(i32);
#[repr(C)]
pub struct PDXVA2SW_VIDEOPROCESSSETRENDERTARGET(i32);
#[repr(C)]
pub struct PIC_SEQ_SAMPLE(i32);
#[repr(C)]
pub struct PIDListSpanningEvent(i32);
#[repr(C)]
pub struct PID_BITS(i32);
#[repr(C)]
pub struct PID_BITS_MIDL(i32);
#[repr(C)]
pub struct PID_MAP(i32);
#[repr(C)]
pub struct PINNAME_BDA_ANALOG_AUDIO(i32);
#[repr(C)]
pub struct PINNAME_BDA_ANALOG_VIDEO(i32);
#[repr(C)]
pub struct PINNAME_BDA_FM_RADIO(i32);
#[repr(C)]
pub struct PINNAME_BDA_IF_PIN(i32);
#[repr(C)]
pub struct PINNAME_BDA_OPENCABLE_PSIP_PIN(i32);
#[repr(C)]
pub struct PINNAME_BDA_TRANSPORT(i32);
#[repr(C)]
pub struct PINNAME_IPSINK_INPUT(i32);
#[repr(C)]
pub struct PINNAME_MPE(i32);
#[repr(transparent)]
pub struct PIN_DIRECTION(pub i32);
pub const PINDIR_INPUT: PIN_DIRECTION = PIN_DIRECTION(0i32);
pub const PINDIR_OUTPUT: PIN_DIRECTION = PIN_DIRECTION(1i32);
#[repr(C)]
pub struct PIN_INFO(i32);
#[repr(C)]
pub struct PersistTuneXmlUtility(i32);
#[repr(transparent)]
pub struct PhysicalConnectorType(pub i32);
pub const PhysConn_Video_Tuner: PhysicalConnectorType = PhysicalConnectorType(1i32);
pub const PhysConn_Video_Composite: PhysicalConnectorType = PhysicalConnectorType(2i32);
pub const PhysConn_Video_SVideo: PhysicalConnectorType = PhysicalConnectorType(3i32);
pub const PhysConn_Video_RGB: PhysicalConnectorType = PhysicalConnectorType(4i32);
pub const PhysConn_Video_YRYBY: PhysicalConnectorType = PhysicalConnectorType(5i32);
pub const PhysConn_Video_SerialDigital: PhysicalConnectorType = PhysicalConnectorType(6i32);
pub const PhysConn_Video_ParallelDigital: PhysicalConnectorType = PhysicalConnectorType(7i32);
pub const PhysConn_Video_SCSI: PhysicalConnectorType = PhysicalConnectorType(8i32);
pub const PhysConn_Video_AUX: PhysicalConnectorType = PhysicalConnectorType(9i32);
pub const PhysConn_Video_1394: PhysicalConnectorType = PhysicalConnectorType(10i32);
pub const PhysConn_Video_USB: PhysicalConnectorType = PhysicalConnectorType(11i32);
pub const PhysConn_Video_VideoDecoder: PhysicalConnectorType = PhysicalConnectorType(12i32);
pub const PhysConn_Video_VideoEncoder: PhysicalConnectorType = PhysicalConnectorType(13i32);
pub const PhysConn_Video_SCART: PhysicalConnectorType = PhysicalConnectorType(14i32);
pub const PhysConn_Video_Black: PhysicalConnectorType = PhysicalConnectorType(15i32);
pub const PhysConn_Audio_Tuner: PhysicalConnectorType = PhysicalConnectorType(4096i32);
pub const PhysConn_Audio_Line: PhysicalConnectorType = PhysicalConnectorType(4097i32);
pub const PhysConn_Audio_Mic: PhysicalConnectorType = PhysicalConnectorType(4098i32);
pub const PhysConn_Audio_AESDigital: PhysicalConnectorType = PhysicalConnectorType(4099i32);
pub const PhysConn_Audio_SPDIFDigital: PhysicalConnectorType = PhysicalConnectorType(4100i32);
pub const PhysConn_Audio_SCSI: PhysicalConnectorType = PhysicalConnectorType(4101i32);
pub const PhysConn_Audio_AUX: PhysicalConnectorType = PhysicalConnectorType(4102i32);
pub const PhysConn_Audio_1394: PhysicalConnectorType = PhysicalConnectorType(4103i32);
pub const PhysConn_Audio_USB: PhysicalConnectorType = PhysicalConnectorType(4104i32);
pub const PhysConn_Audio_AudioDecoder: PhysicalConnectorType = PhysicalConnectorType(4105i32);
#[repr(transparent)]
pub struct Pilot(pub i32);
pub const BDA_PILOT_NOT_SET: Pilot = Pilot(-1i32);
pub const BDA_PILOT_NOT_DEFINED: Pilot = Pilot(0i32);
pub const BDA_PILOT_OFF: Pilot = Pilot(1i32);
pub const BDA_PILOT_ON: Pilot = Pilot(2i32);
pub const BDA_PILOT_MAX: Pilot = Pilot(3i32);
#[repr(transparent)]
pub struct Polarisation(pub i32);
pub const BDA_POLARISATION_NOT_SET: Polarisation = Polarisation(-1i32);
pub const BDA_POLARISATION_NOT_DEFINED: Polarisation = Polarisation(0i32);
pub const BDA_POLARISATION_LINEAR_H: Polarisation = Polarisation(1i32);
pub const BDA_POLARISATION_LINEAR_V: Polarisation = Polarisation(2i32);
pub const BDA_POLARISATION_CIRCULAR_L: Polarisation = Polarisation(3i32);
pub const BDA_POLARISATION_CIRCULAR_R: Polarisation = Polarisation(4i32);
pub const BDA_POLARISATION_MAX: Polarisation = Polarisation(5i32);
#[repr(transparent)]
pub struct PositionModeList(pub i32);
pub const FrameMode: PositionModeList = PositionModeList(0i32);
pub const TenthsSecondsMode: PositionModeList = PositionModeList(1i32);
#[repr(C)]
pub struct ProgramElement(i32);
#[repr(transparent)]
pub struct ProtType(pub i32);
pub const PROT_COPY_FREE: ProtType = ProtType(1i32);
pub const PROT_COPY_ONCE: ProtType = ProtType(2i32);
pub const PROT_COPY_NEVER: ProtType = ProtType(3i32);
pub const PROT_COPY_NEVER_REALLY: ProtType = ProtType(4i32);
pub const PROT_COPY_NO_MORE: ProtType = ProtType(5i32);
pub const PROT_COPY_FREE_CIT: ProtType = ProtType(6i32);
pub const PROT_COPY_BF: ProtType = ProtType(7i32);
pub const PROT_COPY_CN_RECORDING_STOP: ProtType = ProtType(8i32);
pub const PROT_COPY_FREE_SECURE: ProtType = ProtType(9i32);
pub const PROT_COPY_INVALID: ProtType = ProtType(50i32);
#[repr(C)]
pub struct Quality(i32);
#[repr(transparent)]
pub struct QualityMessageType(pub i32);
pub const Famine: QualityMessageType = QualityMessageType(0i32);
pub const Flood: QualityMessageType = QualityMessageType(1i32);
#[repr(C)]
pub struct RATING_ATTRIBUTE(i32);
#[repr(C)]
pub struct RATING_INFO(i32);
#[repr(C)]
pub struct RATING_SYSTEM(i32);
#[repr(transparent)]
pub struct RECORDING_TYPE(pub i32);
pub const RECORDING_TYPE_CONTENT: RECORDING_TYPE = RECORDING_TYPE(0i32);
pub const RECORDING_TYPE_REFERENCE: RECORDING_TYPE = RECORDING_TYPE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REGFILTER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REGFILTER2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REGFILTERPINS(i32);
#[repr(C)]
pub struct REGFILTERPINS2(i32);
#[repr(C)]
pub struct REGPINMEDIUM(i32);
#[repr(C)]
pub struct REGPINTYPES(i32);
#[repr(transparent)]
pub struct REG_PINFLAG(pub u32);
pub const REG_PINFLAG_B_ZERO: REG_PINFLAG = REG_PINFLAG(1u32);
pub const REG_PINFLAG_B_RENDERER: REG_PINFLAG = REG_PINFLAG(2u32);
pub const REG_PINFLAG_B_MANY: REG_PINFLAG = REG_PINFLAG(4u32);
pub const REG_PINFLAG_B_OUTPUT: REG_PINFLAG = REG_PINFLAG(8u32);
pub const REQUIRED_PARENTAL_CONTROL_TIME_RANGE: u32 = 2u32;
#[repr(C)]
pub struct RIFFCHUNK(i32);
#[repr(C)]
pub struct RIFFLIST(i32);
#[repr(transparent)]
pub struct RecordingType(pub i32);
pub const CONTENT: RecordingType = RecordingType(0i32);
pub const REFERENCE: RecordingType = RecordingType(1i32);
#[repr(transparent)]
pub struct RevokedComponent(pub i32);
pub const REVOKED_COPP: RevokedComponent = RevokedComponent(0i32);
pub const REVOKED_SAC: RevokedComponent = RevokedComponent(1i32);
pub const REVOKED_APP_STUB: RevokedComponent = RevokedComponent(2i32);
pub const REVOKED_SECURE_PIPELINE: RevokedComponent = RevokedComponent(3i32);
pub const REVOKED_MAX_TYPES: RevokedComponent = RevokedComponent(4i32);
#[repr(transparent)]
pub struct RollOff(pub i32);
pub const BDA_ROLL_OFF_NOT_SET: RollOff = RollOff(-1i32);
pub const BDA_ROLL_OFF_NOT_DEFINED: RollOff = RollOff(0i32);
pub const BDA_ROLL_OFF_20: RollOff = RollOff(1i32);
pub const BDA_ROLL_OFF_25: RollOff = RollOff(2i32);
pub const BDA_ROLL_OFF_35: RollOff = RollOff(3i32);
pub const BDA_ROLL_OFF_MAX: RollOff = RollOff(4i32);
#[repr(C)]
pub struct SAMPLE_LIVE_STREAM_TIME(i32);
pub const SAMPLE_SEQ_CONTENT_B_FRAME: u32 = 3u32;
pub const SAMPLE_SEQ_CONTENT_I_FRAME: u32 = 1u32;
pub const SAMPLE_SEQ_CONTENT_NONREF_FRAME: u32 = 3u32;
pub const SAMPLE_SEQ_CONTENT_P_FRAME: u32 = 2u32;
pub const SAMPLE_SEQ_CONTENT_REF_FRAME: u32 = 2u32;
pub const SAMPLE_SEQ_CONTENT_STANDALONE_FRAME: u32 = 1u32;
pub const SAMPLE_SEQ_CONTENT_UNKNOWN: u32 = 0u32;
pub const SAMPLE_SEQ_FRAME_START: u32 = 3u32;
pub const SAMPLE_SEQ_GOP_HEADER: u32 = 2u32;
#[repr(C)]
pub struct SAMPLE_SEQ_OFFSET(i32);
pub const SAMPLE_SEQ_PICTURE_HEADER: u32 = 3u32;
pub const SAMPLE_SEQ_SEEK_POINT: u32 = 2u32;
pub const SAMPLE_SEQ_SEQUENCE_HEADER: u32 = 1u32;
pub const SAMPLE_SEQ_SEQUENCE_START: u32 = 1u32;
#[repr(C)]
pub struct SBE2_STREAM_DESC(i32);
pub const SBE2_STREAM_DESC_EVENT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 588489965,
    data2: 48941,
    data3: 17743,
    data4: [173, 138, 217, 91, 167, 249, 31, 238],
};
pub const SBE2_STREAM_DESC_VERSION: u32 = 1u32;
pub const SBE2_V1_STREAMS_CREATION_EVENT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1036041, data2: 38901, data3: 18092, data4: [151, 105, 122, 131, 179, 83, 132, 251] };
pub const SBE2_V2_STREAMS_CREATION_EVENT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2804232355,
    data2: 836,
    data3: 19627,
    data4: [162, 208, 254, 147, 125, 189, 202, 179],
};
#[repr(C)]
pub struct SBE_PIN_DATA(i32);
pub const SCTE_EAS_IB_PID: u32 = 8187u32;
pub const SCTE_EAS_OOB_PID: u32 = 8188u32;
pub const SCTE_EAS_TID: u32 = 216u32;
#[repr(C)]
pub struct SECTION(i32);
pub const SID_DRMSecureServiceChannel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229764, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
pub const SID_MSVidCtl_CurrentAudioEndpoint: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3483011316,
    data2: 43983,
    data3: 20184,
    data4: [155, 116, 125, 179, 52, 69, 69, 158],
};
#[repr(transparent)]
pub struct SNDDEV_ERR(pub i32);
pub const SNDDEV_ERROR_Open: SNDDEV_ERR = SNDDEV_ERR(1i32);
pub const SNDDEV_ERROR_Close: SNDDEV_ERR = SNDDEV_ERR(2i32);
pub const SNDDEV_ERROR_GetCaps: SNDDEV_ERR = SNDDEV_ERR(3i32);
pub const SNDDEV_ERROR_PrepareHeader: SNDDEV_ERR = SNDDEV_ERR(4i32);
pub const SNDDEV_ERROR_UnprepareHeader: SNDDEV_ERR = SNDDEV_ERR(5i32);
pub const SNDDEV_ERROR_Reset: SNDDEV_ERR = SNDDEV_ERR(6i32);
pub const SNDDEV_ERROR_Restart: SNDDEV_ERR = SNDDEV_ERR(7i32);
pub const SNDDEV_ERROR_GetPosition: SNDDEV_ERR = SNDDEV_ERR(8i32);
pub const SNDDEV_ERROR_Write: SNDDEV_ERR = SNDDEV_ERR(9i32);
pub const SNDDEV_ERROR_Pause: SNDDEV_ERR = SNDDEV_ERR(10i32);
pub const SNDDEV_ERROR_Stop: SNDDEV_ERR = SNDDEV_ERR(11i32);
pub const SNDDEV_ERROR_Start: SNDDEV_ERR = SNDDEV_ERR(12i32);
pub const SNDDEV_ERROR_AddBuffer: SNDDEV_ERR = SNDDEV_ERR(13i32);
pub const SNDDEV_ERROR_Query: SNDDEV_ERR = SNDDEV_ERR(14i32);
pub const SPECIFYPAGES_STATISTICS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1279490962, data2: 28318, data3: 4561, data4: [167, 4, 0, 96, 151, 196, 228, 118] };
#[repr(transparent)]
pub struct SSUPDATE_TYPE(pub i32);
pub const SSUPDATE_ASYNC: SSUPDATE_TYPE = SSUPDATE_TYPE(1i32);
pub const SSUPDATE_CONTINUOUS: SSUPDATE_TYPE = SSUPDATE_TYPE(2i32);
pub const STDINDEXSIZE: u32 = 16384u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STREAMBUFFER_ATTRIBUTE(i32);
#[repr(transparent)]
pub struct STREAMBUFFER_ATTR_DATATYPE(pub i32);
pub const STREAMBUFFER_TYPE_DWORD: STREAMBUFFER_ATTR_DATATYPE = STREAMBUFFER_ATTR_DATATYPE(0i32);
pub const STREAMBUFFER_TYPE_STRING: STREAMBUFFER_ATTR_DATATYPE = STREAMBUFFER_ATTR_DATATYPE(1i32);
pub const STREAMBUFFER_TYPE_BINARY: STREAMBUFFER_ATTR_DATATYPE = STREAMBUFFER_ATTR_DATATYPE(2i32);
pub const STREAMBUFFER_TYPE_BOOL: STREAMBUFFER_ATTR_DATATYPE = STREAMBUFFER_ATTR_DATATYPE(3i32);
pub const STREAMBUFFER_TYPE_QWORD: STREAMBUFFER_ATTR_DATATYPE = STREAMBUFFER_ATTR_DATATYPE(4i32);
pub const STREAMBUFFER_TYPE_WORD: STREAMBUFFER_ATTR_DATATYPE = STREAMBUFFER_ATTR_DATATYPE(5i32);
pub const STREAMBUFFER_TYPE_GUID: STREAMBUFFER_ATTR_DATATYPE = STREAMBUFFER_ATTR_DATATYPE(6i32);
pub const STREAMBUFFER_EC_BASE: u32 = 806u32;
pub const STREAMBUFFER_EC_CONTENT_BECOMING_STALE: i32 = 809i32;
pub const STREAMBUFFER_EC_PRIMARY_AUDIO: i32 = 814i32;
pub const STREAMBUFFER_EC_RATE_CHANGED: i32 = 813i32;
pub const STREAMBUFFER_EC_RATE_CHANGING_FOR_SETPOSITIONS: i32 = 815i32;
pub const STREAMBUFFER_EC_READ_FAILURE: i32 = 812i32;
pub const STREAMBUFFER_EC_SETPOSITIONS_EVENTS_DONE: i32 = 816i32;
pub const STREAMBUFFER_EC_STALE_DATA_READ: i32 = 807i32;
pub const STREAMBUFFER_EC_STALE_FILE_DELETED: i32 = 808i32;
pub const STREAMBUFFER_EC_TIMEHOLE: i32 = 806i32;
pub const STREAMBUFFER_EC_WRITE_FAILURE: i32 = 810i32;
pub const STREAMBUFFER_EC_WRITE_FAILURE_CLEAR: i32 = 811i32;
#[repr(transparent)]
pub struct STREAMIF_CONSTANTS(pub i32);
pub const MAX_NUMBER_OF_STREAMS: STREAMIF_CONSTANTS = STREAMIF_CONSTANTS(16i32);
#[repr(C)]
pub struct STREAM_ID_MAP(i32);
#[repr(transparent)]
pub struct STREAM_STATE(pub i32);
pub const STREAMSTATE_STOP: STREAM_STATE = STREAM_STATE(0i32);
pub const STREAMSTATE_RUN: STREAM_STATE = STREAM_STATE(1i32);
#[repr(transparent)]
pub struct STREAM_TYPE(pub i32);
pub const STREAMTYPE_READ: STREAM_TYPE = STREAM_TYPE(0i32);
pub const STREAMTYPE_WRITE: STREAM_TYPE = STREAM_TYPE(1i32);
pub const STREAMTYPE_TRANSFORM: STREAM_TYPE = STREAM_TYPE(2i32);
pub const SUBSTREAM_FILTER_VAL_NONE: u32 = 268435456u32;
#[repr(transparent)]
pub struct ScanModulationTypes(pub i32);
pub const BDA_SCAN_MOD_16QAM: ScanModulationTypes = ScanModulationTypes(1i32);
pub const BDA_SCAN_MOD_32QAM: ScanModulationTypes = ScanModulationTypes(2i32);
pub const BDA_SCAN_MOD_64QAM: ScanModulationTypes = ScanModulationTypes(4i32);
pub const BDA_SCAN_MOD_80QAM: ScanModulationTypes = ScanModulationTypes(8i32);
pub const BDA_SCAN_MOD_96QAM: ScanModulationTypes = ScanModulationTypes(16i32);
pub const BDA_SCAN_MOD_112QAM: ScanModulationTypes = ScanModulationTypes(32i32);
pub const BDA_SCAN_MOD_128QAM: ScanModulationTypes = ScanModulationTypes(64i32);
pub const BDA_SCAN_MOD_160QAM: ScanModulationTypes = ScanModulationTypes(128i32);
pub const BDA_SCAN_MOD_192QAM: ScanModulationTypes = ScanModulationTypes(256i32);
pub const BDA_SCAN_MOD_224QAM: ScanModulationTypes = ScanModulationTypes(512i32);
pub const BDA_SCAN_MOD_256QAM: ScanModulationTypes = ScanModulationTypes(1024i32);
pub const BDA_SCAN_MOD_320QAM: ScanModulationTypes = ScanModulationTypes(2048i32);
pub const BDA_SCAN_MOD_384QAM: ScanModulationTypes = ScanModulationTypes(4096i32);
pub const BDA_SCAN_MOD_448QAM: ScanModulationTypes = ScanModulationTypes(8192i32);
pub const BDA_SCAN_MOD_512QAM: ScanModulationTypes = ScanModulationTypes(16384i32);
pub const BDA_SCAN_MOD_640QAM: ScanModulationTypes = ScanModulationTypes(32768i32);
pub const BDA_SCAN_MOD_768QAM: ScanModulationTypes = ScanModulationTypes(65536i32);
pub const BDA_SCAN_MOD_896QAM: ScanModulationTypes = ScanModulationTypes(131072i32);
pub const BDA_SCAN_MOD_1024QAM: ScanModulationTypes = ScanModulationTypes(262144i32);
pub const BDA_SCAN_MOD_QPSK: ScanModulationTypes = ScanModulationTypes(524288i32);
pub const BDA_SCAN_MOD_BPSK: ScanModulationTypes = ScanModulationTypes(1048576i32);
pub const BDA_SCAN_MOD_OQPSK: ScanModulationTypes = ScanModulationTypes(2097152i32);
pub const BDA_SCAN_MOD_8VSB: ScanModulationTypes = ScanModulationTypes(4194304i32);
pub const BDA_SCAN_MOD_16VSB: ScanModulationTypes = ScanModulationTypes(8388608i32);
pub const BDA_SCAN_MOD_AM_RADIO: ScanModulationTypes = ScanModulationTypes(16777216i32);
pub const BDA_SCAN_MOD_FM_RADIO: ScanModulationTypes = ScanModulationTypes(33554432i32);
pub const BDA_SCAN_MOD_8PSK: ScanModulationTypes = ScanModulationTypes(67108864i32);
pub const BDA_SCAN_MOD_RF: ScanModulationTypes = ScanModulationTypes(134217728i32);
pub const ScanModulationTypesMask_MCE_DigitalCable: ScanModulationTypes = ScanModulationTypes(11i32);
pub const ScanModulationTypesMask_MCE_TerrestrialATSC: ScanModulationTypes = ScanModulationTypes(23i32);
pub const ScanModulationTypesMask_MCE_AnalogTv: ScanModulationTypes = ScanModulationTypes(28i32);
pub const ScanModulationTypesMask_MCE_All_TV: ScanModulationTypes = ScanModulationTypes(-1i32);
pub const ScanModulationTypesMask_DVBC: ScanModulationTypes = ScanModulationTypes(75i32);
pub const BDA_SCAN_MOD_16APSK: ScanModulationTypes = ScanModulationTypes(268435456i32);
pub const BDA_SCAN_MOD_32APSK: ScanModulationTypes = ScanModulationTypes(536870912i32);
#[repr(C)]
pub struct SectionList(i32);
#[repr(transparent)]
pub struct SegDispidList(pub i32);
pub const dispidName: SegDispidList = SegDispidList(0i32);
pub const dispidStatus: SegDispidList = SegDispidList(1i32);
pub const dispidDevImageSourceWidth: SegDispidList = SegDispidList(2i32);
pub const dispidDevImageSourceHeight: SegDispidList = SegDispidList(3i32);
pub const dispidDevCountryCode: SegDispidList = SegDispidList(4i32);
pub const dispidDevOverScan: SegDispidList = SegDispidList(5i32);
pub const dispidSegment: SegDispidList = SegDispidList(6i32);
pub const dispidDevVolume: SegDispidList = SegDispidList(7i32);
pub const dispidDevBalance: SegDispidList = SegDispidList(8i32);
pub const dispidDevPower: SegDispidList = SegDispidList(9i32);
pub const dispidTuneChan: SegDispidList = SegDispidList(10i32);
pub const dispidDevVideoSubchannel: SegDispidList = SegDispidList(11i32);
pub const dispidDevAudioSubchannel: SegDispidList = SegDispidList(12i32);
pub const dispidChannelAvailable: SegDispidList = SegDispidList(13i32);
pub const dispidDevVideoFrequency: SegDispidList = SegDispidList(14i32);
pub const dispidDevAudioFrequency: SegDispidList = SegDispidList(15i32);
pub const dispidCount: SegDispidList = SegDispidList(16i32);
pub const dispidDevFileName: SegDispidList = SegDispidList(17i32);
pub const dispidVisible: SegDispidList = SegDispidList(18i32);
pub const dispidOwner: SegDispidList = SegDispidList(19i32);
pub const dispidMessageDrain: SegDispidList = SegDispidList(20i32);
pub const dispidViewable: SegDispidList = SegDispidList(21i32);
pub const dispidDevView: SegDispidList = SegDispidList(22i32);
pub const dispidKSCat: SegDispidList = SegDispidList(23i32);
pub const dispidCLSID: SegDispidList = SegDispidList(24i32);
pub const dispid_KSCat: SegDispidList = SegDispidList(25i32);
pub const dispid_CLSID: SegDispidList = SegDispidList(26i32);
pub const dispidTune: SegDispidList = SegDispidList(27i32);
pub const dispidTS: SegDispidList = SegDispidList(28i32);
pub const dispidDevSAP: SegDispidList = SegDispidList(29i32);
pub const dispidClip: SegDispidList = SegDispidList(30i32);
pub const dispidRequestedClipRect: SegDispidList = SegDispidList(31i32);
pub const dispidClippedSourceRect: SegDispidList = SegDispidList(32i32);
pub const dispidAvailableSourceRect: SegDispidList = SegDispidList(33i32);
pub const dispidMediaPosition: SegDispidList = SegDispidList(34i32);
pub const dispidDevRun: SegDispidList = SegDispidList(35i32);
pub const dispidDevPause: SegDispidList = SegDispidList(36i32);
pub const dispidDevStop: SegDispidList = SegDispidList(37i32);
pub const dispidCCEnable: SegDispidList = SegDispidList(38i32);
pub const dispidDevStep: SegDispidList = SegDispidList(39i32);
pub const dispidDevCanStep: SegDispidList = SegDispidList(40i32);
pub const dispidSourceSize: SegDispidList = SegDispidList(41i32);
pub const dispid_playtitle: SegDispidList = SegDispidList(42i32);
pub const dispid_playchapterintitle: SegDispidList = SegDispidList(43i32);
pub const dispid_playchapter: SegDispidList = SegDispidList(44i32);
pub const dispid_playchaptersautostop: SegDispidList = SegDispidList(45i32);
pub const dispid_playattime: SegDispidList = SegDispidList(46i32);
pub const dispid_playattimeintitle: SegDispidList = SegDispidList(47i32);
pub const dispid_playperiodintitleautostop: SegDispidList = SegDispidList(48i32);
pub const dispid_replaychapter: SegDispidList = SegDispidList(49i32);
pub const dispid_playprevchapter: SegDispidList = SegDispidList(50i32);
pub const dispid_playnextchapter: SegDispidList = SegDispidList(51i32);
pub const dispid_playforwards: SegDispidList = SegDispidList(52i32);
pub const dispid_playbackwards: SegDispidList = SegDispidList(53i32);
pub const dispid_stilloff: SegDispidList = SegDispidList(54i32);
pub const dispid_audiolanguage: SegDispidList = SegDispidList(55i32);
pub const dispid_showmenu: SegDispidList = SegDispidList(56i32);
pub const dispid_resume: SegDispidList = SegDispidList(57i32);
pub const dispid_returnfromsubmenu: SegDispidList = SegDispidList(58i32);
pub const dispid_buttonsavailable: SegDispidList = SegDispidList(59i32);
pub const dispid_currentbutton: SegDispidList = SegDispidList(60i32);
pub const dispid_SelectAndActivateButton: SegDispidList = SegDispidList(61i32);
pub const dispid_ActivateButton: SegDispidList = SegDispidList(62i32);
pub const dispid_SelectRightButton: SegDispidList = SegDispidList(63i32);
pub const dispid_SelectLeftButton: SegDispidList = SegDispidList(64i32);
pub const dispid_SelectLowerButton: SegDispidList = SegDispidList(65i32);
pub const dispid_SelectUpperButton: SegDispidList = SegDispidList(66i32);
pub const dispid_ActivateAtPosition: SegDispidList = SegDispidList(67i32);
pub const dispid_SelectAtPosition: SegDispidList = SegDispidList(68i32);
pub const dispid_ButtonAtPosition: SegDispidList = SegDispidList(69i32);
pub const dispid_NumberOfChapters: SegDispidList = SegDispidList(70i32);
pub const dispid_TotalTitleTime: SegDispidList = SegDispidList(71i32);
pub const dispid_TitlesAvailable: SegDispidList = SegDispidList(72i32);
pub const dispid_VolumesAvailable: SegDispidList = SegDispidList(73i32);
pub const dispid_CurrentVolume: SegDispidList = SegDispidList(74i32);
pub const dispid_CurrentDiscSide: SegDispidList = SegDispidList(75i32);
pub const dispid_CurrentDomain: SegDispidList = SegDispidList(76i32);
pub const dispid_CurrentChapter: SegDispidList = SegDispidList(77i32);
pub const dispid_CurrentTitle: SegDispidList = SegDispidList(78i32);
pub const dispid_CurrentTime: SegDispidList = SegDispidList(79i32);
pub const dispid_FramesPerSecond: SegDispidList = SegDispidList(80i32);
pub const dispid_DVDTimeCode2bstr: SegDispidList = SegDispidList(81i32);
pub const dispid_DVDDirectory: SegDispidList = SegDispidList(82i32);
pub const dispid_IsSubpictureStreamEnabled: SegDispidList = SegDispidList(83i32);
pub const dispid_IsAudioStreamEnabled: SegDispidList = SegDispidList(84i32);
pub const dispid_CurrentSubpictureStream: SegDispidList = SegDispidList(85i32);
pub const dispid_SubpictureLanguage: SegDispidList = SegDispidList(86i32);
pub const dispid_CurrentAudioStream: SegDispidList = SegDispidList(87i32);
pub const dispid_AudioStreamsAvailable: SegDispidList = SegDispidList(88i32);
pub const dispid_AnglesAvailable: SegDispidList = SegDispidList(89i32);
pub const dispid_CurrentAngle: SegDispidList = SegDispidList(90i32);
pub const dispid_CCActive: SegDispidList = SegDispidList(91i32);
pub const dispid_CurrentCCService: SegDispidList = SegDispidList(92i32);
pub const dispid_SubpictureStreamsAvailable: SegDispidList = SegDispidList(93i32);
pub const dispid_SubpictureOn: SegDispidList = SegDispidList(94i32);
pub const dispid_DVDUniqueID: SegDispidList = SegDispidList(95i32);
pub const dispid_EnableResetOnStop: SegDispidList = SegDispidList(96i32);
pub const dispid_AcceptParentalLevelChange: SegDispidList = SegDispidList(97i32);
pub const dispid_NotifyParentalLevelChange: SegDispidList = SegDispidList(98i32);
pub const dispid_SelectParentalCountry: SegDispidList = SegDispidList(99i32);
pub const dispid_SelectParentalLevel: SegDispidList = SegDispidList(100i32);
pub const dispid_TitleParentalLevels: SegDispidList = SegDispidList(101i32);
pub const dispid_PlayerParentalCountry: SegDispidList = SegDispidList(102i32);
pub const dispid_PlayerParentalLevel: SegDispidList = SegDispidList(103i32);
pub const dispid_Eject: SegDispidList = SegDispidList(104i32);
pub const dispid_UOPValid: SegDispidList = SegDispidList(105i32);
pub const dispid_SPRM: SegDispidList = SegDispidList(106i32);
pub const dispid_GPRM: SegDispidList = SegDispidList(107i32);
pub const dispid_DVDTextStringType: SegDispidList = SegDispidList(108i32);
pub const dispid_DVDTextString: SegDispidList = SegDispidList(109i32);
pub const dispid_DVDTextNumberOfStrings: SegDispidList = SegDispidList(110i32);
pub const dispid_DVDTextNumberOfLanguages: SegDispidList = SegDispidList(111i32);
pub const dispid_DVDTextLanguageLCID: SegDispidList = SegDispidList(112i32);
pub const dispid_RegionChange: SegDispidList = SegDispidList(113i32);
pub const dispid_DVDAdm: SegDispidList = SegDispidList(114i32);
pub const dispid_DeleteBookmark: SegDispidList = SegDispidList(115i32);
pub const dispid_RestoreBookmark: SegDispidList = SegDispidList(116i32);
pub const dispid_SaveBookmark: SegDispidList = SegDispidList(117i32);
pub const dispid_SelectDefaultAudioLanguage: SegDispidList = SegDispidList(118i32);
pub const dispid_SelectDefaultSubpictureLanguage: SegDispidList = SegDispidList(119i32);
pub const dispid_PreferredSubpictureStream: SegDispidList = SegDispidList(120i32);
pub const dispid_DefaultMenuLanguage: SegDispidList = SegDispidList(121i32);
pub const dispid_DefaultSubpictureLanguage: SegDispidList = SegDispidList(122i32);
pub const dispid_DefaultAudioLanguage: SegDispidList = SegDispidList(123i32);
pub const dispid_DefaultSubpictureLanguageExt: SegDispidList = SegDispidList(124i32);
pub const dispid_DefaultAudioLanguageExt: SegDispidList = SegDispidList(125i32);
pub const dispid_LanguageFromLCID: SegDispidList = SegDispidList(126i32);
pub const dispid_KaraokeAudioPresentationMode: SegDispidList = SegDispidList(127i32);
pub const dispid_KaraokeChannelContent: SegDispidList = SegDispidList(128i32);
pub const dispid_KaraokeChannelAssignment: SegDispidList = SegDispidList(129i32);
pub const dispid_RestorePreferredSettings: SegDispidList = SegDispidList(130i32);
pub const dispid_ButtonRect: SegDispidList = SegDispidList(131i32);
pub const dispid_DVDScreenInMouseCoordinates: SegDispidList = SegDispidList(132i32);
pub const dispid_CustomCompositorClass: SegDispidList = SegDispidList(133i32);
pub const dispidCustomCompositorClass: SegDispidList = SegDispidList(134i32);
pub const dispid_CustomCompositor: SegDispidList = SegDispidList(135i32);
pub const dispidMixerBitmap: SegDispidList = SegDispidList(136i32);
pub const dispid_MixerBitmap: SegDispidList = SegDispidList(137i32);
pub const dispidMixerBitmapOpacity: SegDispidList = SegDispidList(138i32);
pub const dispidMixerBitmapRect: SegDispidList = SegDispidList(139i32);
pub const dispidSetupMixerBitmap: SegDispidList = SegDispidList(140i32);
pub const dispidUsingOverlay: SegDispidList = SegDispidList(141i32);
pub const dispidDisplayChange: SegDispidList = SegDispidList(142i32);
pub const dispidRePaint: SegDispidList = SegDispidList(143i32);
pub const dispid_IsEqualDevice: SegDispidList = SegDispidList(144i32);
pub const dispidrate: SegDispidList = SegDispidList(145i32);
pub const dispidposition: SegDispidList = SegDispidList(146i32);
pub const dispidpositionmode: SegDispidList = SegDispidList(147i32);
pub const dispidlength: SegDispidList = SegDispidList(148i32);
pub const dispidChangePassword: SegDispidList = SegDispidList(149i32);
pub const dispidSaveParentalLevel: SegDispidList = SegDispidList(150i32);
pub const dispidSaveParentalCountry: SegDispidList = SegDispidList(151i32);
pub const dispidConfirmPassword: SegDispidList = SegDispidList(152i32);
pub const dispidGetParentalLevel: SegDispidList = SegDispidList(153i32);
pub const dispidGetParentalCountry: SegDispidList = SegDispidList(154i32);
pub const dispidDefaultAudioLCID: SegDispidList = SegDispidList(155i32);
pub const dispidDefaultSubpictureLCID: SegDispidList = SegDispidList(156i32);
pub const dispidDefaultMenuLCID: SegDispidList = SegDispidList(157i32);
pub const dispidBookmarkOnStop: SegDispidList = SegDispidList(158i32);
pub const dispidMaxVidRect: SegDispidList = SegDispidList(159i32);
pub const dispidMinVidRect: SegDispidList = SegDispidList(160i32);
pub const dispidCapture: SegDispidList = SegDispidList(161i32);
pub const dispid_DecimateInput: SegDispidList = SegDispidList(162i32);
pub const dispidAlloctor: SegDispidList = SegDispidList(163i32);
pub const dispid_Allocator: SegDispidList = SegDispidList(164i32);
pub const dispidAllocPresentID: SegDispidList = SegDispidList(165i32);
pub const dispidSetAllocator: SegDispidList = SegDispidList(166i32);
pub const dispid_SetAllocator: SegDispidList = SegDispidList(167i32);
pub const dispidStreamBufferSinkName: SegDispidList = SegDispidList(168i32);
pub const dispidStreamBufferSourceName: SegDispidList = SegDispidList(169i32);
pub const dispidStreamBufferContentRecording: SegDispidList = SegDispidList(170i32);
pub const dispidStreamBufferReferenceRecording: SegDispidList = SegDispidList(171i32);
pub const dispidstarttime: SegDispidList = SegDispidList(172i32);
pub const dispidstoptime: SegDispidList = SegDispidList(173i32);
pub const dispidrecordingstopped: SegDispidList = SegDispidList(174i32);
pub const dispidrecordingstarted: SegDispidList = SegDispidList(175i32);
pub const dispidNameSetLock: SegDispidList = SegDispidList(176i32);
pub const dispidrecordingtype: SegDispidList = SegDispidList(177i32);
pub const dispidstart: SegDispidList = SegDispidList(178i32);
pub const dispidRecordingAttribute: SegDispidList = SegDispidList(179i32);
pub const dispid_RecordingAttribute: SegDispidList = SegDispidList(180i32);
pub const dispidSBEConfigure: SegDispidList = SegDispidList(181i32);
pub const dispid_CurrentRatings: SegDispidList = SegDispidList(182i32);
pub const dispid_MaxRatingsLevel: SegDispidList = SegDispidList(183i32);
pub const dispid_audioencoderint: SegDispidList = SegDispidList(184i32);
pub const dispid_videoencoderint: SegDispidList = SegDispidList(185i32);
pub const dispidService: SegDispidList = SegDispidList(186i32);
pub const dispid_BlockUnrated: SegDispidList = SegDispidList(187i32);
pub const dispid_UnratedDelay: SegDispidList = SegDispidList(188i32);
pub const dispid_SuppressEffects: SegDispidList = SegDispidList(189i32);
pub const dispidsbesource: SegDispidList = SegDispidList(190i32);
pub const dispidSetSinkFilter: SegDispidList = SegDispidList(191i32);
pub const dispid_SinkStreams: SegDispidList = SegDispidList(192i32);
pub const dispidTVFormats: SegDispidList = SegDispidList(193i32);
pub const dispidModes: SegDispidList = SegDispidList(194i32);
pub const dispidAuxInputs: SegDispidList = SegDispidList(195i32);
pub const dispidTeleTextFilter: SegDispidList = SegDispidList(196i32);
pub const dispid_channelchangeint: SegDispidList = SegDispidList(197i32);
pub const dispidUnlockProfile: SegDispidList = SegDispidList(198i32);
pub const dispid_AddFilter: SegDispidList = SegDispidList(199i32);
pub const dispidSetMinSeek: SegDispidList = SegDispidList(200i32);
pub const dispidRateEx: SegDispidList = SegDispidList(201i32);
pub const dispidaudiocounter: SegDispidList = SegDispidList(202i32);
pub const dispidvideocounter: SegDispidList = SegDispidList(203i32);
pub const dispidcccounter: SegDispidList = SegDispidList(204i32);
pub const dispidwstcounter: SegDispidList = SegDispidList(205i32);
pub const dispid_audiocounter: SegDispidList = SegDispidList(206i32);
pub const dispid_videocounter: SegDispidList = SegDispidList(207i32);
pub const dispid_cccounter: SegDispidList = SegDispidList(208i32);
pub const dispid_wstcounter: SegDispidList = SegDispidList(209i32);
pub const dispidaudioanalysis: SegDispidList = SegDispidList(210i32);
pub const dispidvideoanalysis: SegDispidList = SegDispidList(211i32);
pub const dispiddataanalysis: SegDispidList = SegDispidList(212i32);
pub const dispidaudio_analysis: SegDispidList = SegDispidList(213i32);
pub const dispidvideo_analysis: SegDispidList = SegDispidList(214i32);
pub const dispiddata_analysis: SegDispidList = SegDispidList(215i32);
pub const dispid_resetFilterList: SegDispidList = SegDispidList(216i32);
pub const dispidDevicePath: SegDispidList = SegDispidList(217i32);
pub const dispid_SourceFilter: SegDispidList = SegDispidList(218i32);
pub const dispid__SourceFilter: SegDispidList = SegDispidList(219i32);
pub const dispidUserEvent: SegDispidList = SegDispidList(220i32);
pub const dispid_Bookmark: SegDispidList = SegDispidList(221i32);
pub const LastReservedDeviceDispid: SegDispidList = SegDispidList(16383i32);
#[repr(transparent)]
pub struct SegEventidList(pub i32);
pub const eventidStateChange: SegEventidList = SegEventidList(0i32);
pub const eventidOnTuneChanged: SegEventidList = SegEventidList(1i32);
pub const eventidEndOfMedia: SegEventidList = SegEventidList(2i32);
pub const eventidDVDNotify: SegEventidList = SegEventidList(3i32);
pub const eventidPlayForwards: SegEventidList = SegEventidList(4i32);
pub const eventidPlayBackwards: SegEventidList = SegEventidList(5i32);
pub const eventidShowMenu: SegEventidList = SegEventidList(6i32);
pub const eventidResume: SegEventidList = SegEventidList(7i32);
pub const eventidSelectOrActivateButton: SegEventidList = SegEventidList(8i32);
pub const eventidStillOff: SegEventidList = SegEventidList(9i32);
pub const eventidPauseOn: SegEventidList = SegEventidList(10i32);
pub const eventidChangeCurrentAudioStream: SegEventidList = SegEventidList(11i32);
pub const eventidChangeCurrentSubpictureStream: SegEventidList = SegEventidList(12i32);
pub const eventidChangeCurrentAngle: SegEventidList = SegEventidList(13i32);
pub const eventidPlayAtTimeInTitle: SegEventidList = SegEventidList(14i32);
pub const eventidPlayAtTime: SegEventidList = SegEventidList(15i32);
pub const eventidPlayChapterInTitle: SegEventidList = SegEventidList(16i32);
pub const eventidPlayChapter: SegEventidList = SegEventidList(17i32);
pub const eventidReplayChapter: SegEventidList = SegEventidList(18i32);
pub const eventidPlayNextChapter: SegEventidList = SegEventidList(19i32);
pub const eventidStop: SegEventidList = SegEventidList(20i32);
pub const eventidReturnFromSubmenu: SegEventidList = SegEventidList(21i32);
pub const eventidPlayTitle: SegEventidList = SegEventidList(22i32);
pub const eventidPlayPrevChapter: SegEventidList = SegEventidList(23i32);
pub const eventidChangeKaraokePresMode: SegEventidList = SegEventidList(24i32);
pub const eventidChangeVideoPresMode: SegEventidList = SegEventidList(25i32);
pub const eventidOverlayUnavailable: SegEventidList = SegEventidList(26i32);
pub const eventidSinkCertificateFailure: SegEventidList = SegEventidList(27i32);
pub const eventidSinkCertificateSuccess: SegEventidList = SegEventidList(28i32);
pub const eventidSourceCertificateFailure: SegEventidList = SegEventidList(29i32);
pub const eventidSourceCertificateSuccess: SegEventidList = SegEventidList(30i32);
pub const eventidRatingsBlocked: SegEventidList = SegEventidList(31i32);
pub const eventidRatingsUnlocked: SegEventidList = SegEventidList(32i32);
pub const eventidRatingsChanged: SegEventidList = SegEventidList(33i32);
pub const eventidWriteFailure: SegEventidList = SegEventidList(34i32);
pub const eventidTimeHole: SegEventidList = SegEventidList(35i32);
pub const eventidStaleDataRead: SegEventidList = SegEventidList(36i32);
pub const eventidContentBecomingStale: SegEventidList = SegEventidList(37i32);
pub const eventidStaleFileDeleted: SegEventidList = SegEventidList(38i32);
pub const eventidEncryptionOn: SegEventidList = SegEventidList(39i32);
pub const eventidEncryptionOff: SegEventidList = SegEventidList(40i32);
pub const eventidRateChange: SegEventidList = SegEventidList(41i32);
pub const eventidLicenseChange: SegEventidList = SegEventidList(42i32);
pub const eventidCOPPBlocked: SegEventidList = SegEventidList(43i32);
pub const eventidCOPPUnblocked: SegEventidList = SegEventidList(44i32);
pub const dispidlicenseerrorcode: SegEventidList = SegEventidList(45i32);
pub const eventidBroadcastEvent: SegEventidList = SegEventidList(46i32);
pub const eventidBroadcastEventEx: SegEventidList = SegEventidList(47i32);
pub const eventidContentPrimarilyAudio: SegEventidList = SegEventidList(48i32);
pub const dispidAVDecAudioDualMonoEvent: SegEventidList = SegEventidList(49i32);
pub const dispidAVAudioSampleRateEvent: SegEventidList = SegEventidList(50i32);
pub const dispidAVAudioChannelConfigEvent: SegEventidList = SegEventidList(51i32);
pub const dispidAVAudioChannelCountEvent: SegEventidList = SegEventidList(52i32);
pub const dispidAVDecCommonMeanBitRateEvent: SegEventidList = SegEventidList(53i32);
pub const dispidAVDDSurroundModeEvent: SegEventidList = SegEventidList(54i32);
pub const dispidAVDecCommonInputFormatEvent: SegEventidList = SegEventidList(55i32);
pub const dispidAVDecCommonOutputFormatEvent: SegEventidList = SegEventidList(56i32);
pub const eventidWriteFailureClear: SegEventidList = SegEventidList(57i32);
pub const LastReservedDeviceEvent: SegEventidList = SegEventidList(16383i32);
#[repr(transparent)]
pub struct SignalAndServiceStatusSpanningEvent_State(pub i32);
pub const SignalAndServiceStatusSpanningEvent_None: SignalAndServiceStatusSpanningEvent_State = SignalAndServiceStatusSpanningEvent_State(-1i32);
pub const SignalAndServiceStatusSpanningEvent_Clear: SignalAndServiceStatusSpanningEvent_State = SignalAndServiceStatusSpanningEvent_State(0i32);
pub const SignalAndServiceStatusSpanningEvent_NoTVSignal: SignalAndServiceStatusSpanningEvent_State = SignalAndServiceStatusSpanningEvent_State(1i32);
pub const SignalAndServiceStatusSpanningEvent_ServiceOffAir: SignalAndServiceStatusSpanningEvent_State = SignalAndServiceStatusSpanningEvent_State(2i32);
pub const SignalAndServiceStatusSpanningEvent_WeakTVSignal: SignalAndServiceStatusSpanningEvent_State = SignalAndServiceStatusSpanningEvent_State(3i32);
pub const SignalAndServiceStatusSpanningEvent_NoSubscription: SignalAndServiceStatusSpanningEvent_State = SignalAndServiceStatusSpanningEvent_State(4i32);
pub const SignalAndServiceStatusSpanningEvent_AllAVScrambled: SignalAndServiceStatusSpanningEvent_State = SignalAndServiceStatusSpanningEvent_State(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SmartCardApplication(i32);
#[repr(transparent)]
pub struct SmartCardAssociationType(pub i32);
pub const NotAssociated: SmartCardAssociationType = SmartCardAssociationType(0i32);
pub const Associated: SmartCardAssociationType = SmartCardAssociationType(1i32);
pub const AssociationUnknown: SmartCardAssociationType = SmartCardAssociationType(2i32);
#[repr(transparent)]
pub struct SmartCardStatusType(pub i32);
pub const CardInserted: SmartCardStatusType = SmartCardStatusType(0i32);
pub const CardRemoved: SmartCardStatusType = SmartCardStatusType(1i32);
pub const CardError: SmartCardStatusType = SmartCardStatusType(2i32);
pub const CardDataChanged: SmartCardStatusType = SmartCardStatusType(3i32);
pub const CardFirmwareUpgrade: SmartCardStatusType = SmartCardStatusType(4i32);
#[repr(transparent)]
pub struct SourceSizeList(pub i32);
pub const sslFullSize: SourceSizeList = SourceSizeList(0i32);
pub const sslClipByOverScan: SourceSizeList = SourceSizeList(1i32);
pub const sslClipByClipRect: SourceSizeList = SourceSizeList(2i32);
#[repr(C)]
pub struct SpanningEventDescriptor(i32);
#[repr(C)]
pub struct SpanningEventEmmMessage(i32);
#[repr(transparent)]
pub struct SpectralInversion(pub i32);
pub const BDA_SPECTRAL_INVERSION_NOT_SET: SpectralInversion = SpectralInversion(-1i32);
pub const BDA_SPECTRAL_INVERSION_NOT_DEFINED: SpectralInversion = SpectralInversion(0i32);
pub const BDA_SPECTRAL_INVERSION_AUTOMATIC: SpectralInversion = SpectralInversion(1i32);
pub const BDA_SPECTRAL_INVERSION_NORMAL: SpectralInversion = SpectralInversion(2i32);
pub const BDA_SPECTRAL_INVERSION_INVERTED: SpectralInversion = SpectralInversion(3i32);
pub const BDA_SPECTRAL_INVERSION_MAX: SpectralInversion = SpectralInversion(4i32);
#[repr(C)]
pub struct SystemTuningSpaces(i32);
#[repr(C)]
pub struct TID_EXTENSION(i32);
#[repr(C)]
pub struct TIFLoad(i32);
#[repr(C)]
pub struct TIMECODEDATA(i32);
pub const TIMECODE_RATE_30DROP: u32 = 0u32;
pub const TIMECODE_SMPTE_BINARY_GROUP: u32 = 7u32;
pub const TIMECODE_SMPTE_COLOR_FRAME: u32 = 8u32;
#[repr(C)]
pub struct TRANSPORT_PROPERTIES(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct TRUECOLORINFO(i32);
#[repr(transparent)]
pub struct TVAudioMode(pub i32);
pub const AMTVAUDIO_MODE_MONO: TVAudioMode = TVAudioMode(1i32);
pub const AMTVAUDIO_MODE_STEREO: TVAudioMode = TVAudioMode(2i32);
pub const AMTVAUDIO_MODE_LANG_A: TVAudioMode = TVAudioMode(16i32);
pub const AMTVAUDIO_MODE_LANG_B: TVAudioMode = TVAudioMode(32i32);
pub const AMTVAUDIO_MODE_LANG_C: TVAudioMode = TVAudioMode(64i32);
pub const AMTVAUDIO_PRESET_STEREO: TVAudioMode = TVAudioMode(512i32);
pub const AMTVAUDIO_PRESET_LANG_A: TVAudioMode = TVAudioMode(4096i32);
pub const AMTVAUDIO_PRESET_LANG_B: TVAudioMode = TVAudioMode(8192i32);
pub const AMTVAUDIO_PRESET_LANG_C: TVAudioMode = TVAudioMode(16384i32);
#[repr(transparent)]
pub struct TransmissionMode(pub i32);
pub const BDA_XMIT_MODE_NOT_SET: TransmissionMode = TransmissionMode(-1i32);
pub const BDA_XMIT_MODE_NOT_DEFINED: TransmissionMode = TransmissionMode(0i32);
pub const BDA_XMIT_MODE_2K: TransmissionMode = TransmissionMode(1i32);
pub const BDA_XMIT_MODE_8K: TransmissionMode = TransmissionMode(2i32);
pub const BDA_XMIT_MODE_4K: TransmissionMode = TransmissionMode(3i32);
pub const BDA_XMIT_MODE_2K_INTERLEAVED: TransmissionMode = TransmissionMode(4i32);
pub const BDA_XMIT_MODE_4K_INTERLEAVED: TransmissionMode = TransmissionMode(5i32);
pub const BDA_XMIT_MODE_1K: TransmissionMode = TransmissionMode(6i32);
pub const BDA_XMIT_MODE_16K: TransmissionMode = TransmissionMode(7i32);
pub const BDA_XMIT_MODE_32K: TransmissionMode = TransmissionMode(8i32);
pub const BDA_XMIT_MODE_MAX: TransmissionMode = TransmissionMode(9i32);
#[repr(C)]
pub struct TuneRequest(i32);
#[repr(transparent)]
pub struct TunerInputType(pub i32);
pub const TunerInputCable: TunerInputType = TunerInputType(0i32);
pub const TunerInputAntenna: TunerInputType = TunerInputType(1i32);
#[repr(C)]
pub struct TunerMarshaler(i32);
#[repr(C)]
pub struct TuningSpace(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UDCR_TAG(i32);
#[repr(transparent)]
pub struct UICloseReasonType(pub i32);
pub const NotReady: UICloseReasonType = UICloseReasonType(0i32);
pub const UserClosed: UICloseReasonType = UICloseReasonType(1i32);
pub const SystemClosed: UICloseReasonType = UICloseReasonType(2i32);
pub const DeviceClosed: UICloseReasonType = UICloseReasonType(3i32);
pub const ErrorClosed: UICloseReasonType = UICloseReasonType(4i32);
#[repr(transparent)]
pub struct VALID_UOP_FLAG(pub i32);
pub const UOP_FLAG_Play_Title_Or_AtTime: VALID_UOP_FLAG = VALID_UOP_FLAG(1i32);
pub const UOP_FLAG_Play_Chapter: VALID_UOP_FLAG = VALID_UOP_FLAG(2i32);
pub const UOP_FLAG_Play_Title: VALID_UOP_FLAG = VALID_UOP_FLAG(4i32);
pub const UOP_FLAG_Stop: VALID_UOP_FLAG = VALID_UOP_FLAG(8i32);
pub const UOP_FLAG_ReturnFromSubMenu: VALID_UOP_FLAG = VALID_UOP_FLAG(16i32);
pub const UOP_FLAG_Play_Chapter_Or_AtTime: VALID_UOP_FLAG = VALID_UOP_FLAG(32i32);
pub const UOP_FLAG_PlayPrev_Or_Replay_Chapter: VALID_UOP_FLAG = VALID_UOP_FLAG(64i32);
pub const UOP_FLAG_PlayNext_Chapter: VALID_UOP_FLAG = VALID_UOP_FLAG(128i32);
pub const UOP_FLAG_Play_Forwards: VALID_UOP_FLAG = VALID_UOP_FLAG(256i32);
pub const UOP_FLAG_Play_Backwards: VALID_UOP_FLAG = VALID_UOP_FLAG(512i32);
pub const UOP_FLAG_ShowMenu_Title: VALID_UOP_FLAG = VALID_UOP_FLAG(1024i32);
pub const UOP_FLAG_ShowMenu_Root: VALID_UOP_FLAG = VALID_UOP_FLAG(2048i32);
pub const UOP_FLAG_ShowMenu_SubPic: VALID_UOP_FLAG = VALID_UOP_FLAG(4096i32);
pub const UOP_FLAG_ShowMenu_Audio: VALID_UOP_FLAG = VALID_UOP_FLAG(8192i32);
pub const UOP_FLAG_ShowMenu_Angle: VALID_UOP_FLAG = VALID_UOP_FLAG(16384i32);
pub const UOP_FLAG_ShowMenu_Chapter: VALID_UOP_FLAG = VALID_UOP_FLAG(32768i32);
pub const UOP_FLAG_Resume: VALID_UOP_FLAG = VALID_UOP_FLAG(65536i32);
pub const UOP_FLAG_Select_Or_Activate_Button: VALID_UOP_FLAG = VALID_UOP_FLAG(131072i32);
pub const UOP_FLAG_Still_Off: VALID_UOP_FLAG = VALID_UOP_FLAG(262144i32);
pub const UOP_FLAG_Pause_On: VALID_UOP_FLAG = VALID_UOP_FLAG(524288i32);
pub const UOP_FLAG_Select_Audio_Stream: VALID_UOP_FLAG = VALID_UOP_FLAG(1048576i32);
pub const UOP_FLAG_Select_SubPic_Stream: VALID_UOP_FLAG = VALID_UOP_FLAG(2097152i32);
pub const UOP_FLAG_Select_Angle: VALID_UOP_FLAG = VALID_UOP_FLAG(4194304i32);
pub const UOP_FLAG_Select_Karaoke_Audio_Presentation_Mode: VALID_UOP_FLAG = VALID_UOP_FLAG(8388608i32);
pub const UOP_FLAG_Select_Video_Mode_Preference: VALID_UOP_FLAG = VALID_UOP_FLAG(16777216i32);
#[repr(transparent)]
pub struct VA_COLOR_PRIMARIES(pub i32);
pub const VA_PRIMARIES_ITU_R_BT_709: VA_COLOR_PRIMARIES = VA_COLOR_PRIMARIES(1i32);
pub const VA_PRIMARIES_UNSPECIFIED: VA_COLOR_PRIMARIES = VA_COLOR_PRIMARIES(2i32);
pub const VA_PRIMARIES_ITU_R_BT_470_SYSTEM_M: VA_COLOR_PRIMARIES = VA_COLOR_PRIMARIES(4i32);
pub const VA_PRIMARIES_ITU_R_BT_470_SYSTEM_B_G: VA_COLOR_PRIMARIES = VA_COLOR_PRIMARIES(5i32);
pub const VA_PRIMARIES_SMPTE_170M: VA_COLOR_PRIMARIES = VA_COLOR_PRIMARIES(6i32);
pub const VA_PRIMARIES_SMPTE_240M: VA_COLOR_PRIMARIES = VA_COLOR_PRIMARIES(7i32);
pub const VA_PRIMARIES_H264_GENERIC_FILM: VA_COLOR_PRIMARIES = VA_COLOR_PRIMARIES(8i32);
#[repr(transparent)]
pub struct VA_MATRIX_COEFFICIENTS(pub i32);
pub const VA_MATRIX_COEFF_H264_RGB: VA_MATRIX_COEFFICIENTS = VA_MATRIX_COEFFICIENTS(0i32);
pub const VA_MATRIX_COEFF_ITU_R_BT_709: VA_MATRIX_COEFFICIENTS = VA_MATRIX_COEFFICIENTS(1i32);
pub const VA_MATRIX_COEFF_UNSPECIFIED: VA_MATRIX_COEFFICIENTS = VA_MATRIX_COEFFICIENTS(2i32);
pub const VA_MATRIX_COEFF_FCC: VA_MATRIX_COEFFICIENTS = VA_MATRIX_COEFFICIENTS(4i32);
pub const VA_MATRIX_COEFF_ITU_R_BT_470_SYSTEM_B_G: VA_MATRIX_COEFFICIENTS = VA_MATRIX_COEFFICIENTS(5i32);
pub const VA_MATRIX_COEFF_SMPTE_170M: VA_MATRIX_COEFFICIENTS = VA_MATRIX_COEFFICIENTS(6i32);
pub const VA_MATRIX_COEFF_SMPTE_240M: VA_MATRIX_COEFFICIENTS = VA_MATRIX_COEFFICIENTS(7i32);
pub const VA_MATRIX_COEFF_H264_YCgCo: VA_MATRIX_COEFFICIENTS = VA_MATRIX_COEFFICIENTS(8i32);
#[repr(C)]
pub struct VA_OPTIONAL_VIDEO_PROPERTIES(i32);
#[repr(transparent)]
pub struct VA_TRANSFER_CHARACTERISTICS(pub i32);
pub const VA_TRANSFER_CHARACTERISTICS_ITU_R_BT_709: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(1i32);
pub const VA_TRANSFER_CHARACTERISTICS_UNSPECIFIED: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(2i32);
pub const VA_TRANSFER_CHARACTERISTICS_ITU_R_BT_470_SYSTEM_M: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(4i32);
pub const VA_TRANSFER_CHARACTERISTICS_ITU_R_BT_470_SYSTEM_B_G: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(5i32);
pub const VA_TRANSFER_CHARACTERISTICS_SMPTE_170M: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(6i32);
pub const VA_TRANSFER_CHARACTERISTICS_SMPTE_240M: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(7i32);
pub const VA_TRANSFER_CHARACTERISTICS_LINEAR: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(8i32);
pub const VA_TRANSFER_CHARACTERISTICS_H264_LOG_100_TO_1: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(9i32);
pub const VA_TRANSFER_CHARACTERISTICS_H264_LOG_316_TO_1: VA_TRANSFER_CHARACTERISTICS = VA_TRANSFER_CHARACTERISTICS(10i32);
#[repr(transparent)]
pub struct VA_VIDEO_FORMAT(pub i32);
pub const VA_VIDEO_COMPONENT: VA_VIDEO_FORMAT = VA_VIDEO_FORMAT(0i32);
pub const VA_VIDEO_PAL: VA_VIDEO_FORMAT = VA_VIDEO_FORMAT(1i32);
pub const VA_VIDEO_NTSC: VA_VIDEO_FORMAT = VA_VIDEO_FORMAT(2i32);
pub const VA_VIDEO_SECAM: VA_VIDEO_FORMAT = VA_VIDEO_FORMAT(3i32);
pub const VA_VIDEO_MAC: VA_VIDEO_FORMAT = VA_VIDEO_FORMAT(4i32);
pub const VA_VIDEO_UNSPECIFIED: VA_VIDEO_FORMAT = VA_VIDEO_FORMAT(5i32);
pub const VFW_E_ADVISE_ALREADY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220938i32 as _);
pub const VFW_E_ALREADY_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220940i32 as _);
pub const VFW_E_ALREADY_COMMITTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220977i32 as _);
pub const VFW_E_ALREADY_CONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220988i32 as _);
pub const VFW_E_BADALIGN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220978i32 as _);
pub const VFW_E_BAD_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220494i32 as _);
pub const VFW_E_BAD_VIDEOCD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220887i32 as _);
pub const VFW_E_BUFFERS_OUTSTANDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220976i32 as _);
pub const VFW_E_BUFFER_NOTSET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220980i32 as _);
pub const VFW_E_BUFFER_OVERFLOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220979i32 as _);
pub const VFW_E_BUFFER_UNDERFLOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220892i32 as _);
pub const VFW_E_CANNOT_CONNECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220969i32 as _);
pub const VFW_E_CANNOT_LOAD_SOURCE_FILTER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220927i32 as _);
pub const VFW_E_CANNOT_RENDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220968i32 as _);
pub const VFW_E_CERTIFICATION_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220843i32 as _);
pub const VFW_E_CHANGING_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220967i32 as _);
pub const VFW_E_CIRCULAR_GRAPH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220943i32 as _);
pub const VFW_E_CODECAPI_ENUMERATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220719i32 as _);
pub const VFW_E_CODECAPI_LINEAR_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220720i32 as _);
pub const VFW_E_CODECAPI_NO_CURRENT_VALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220716i32 as _);
pub const VFW_E_CODECAPI_NO_DEFAULT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220717i32 as _);
pub const VFW_E_COLOR_KEY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220962i32 as _);
pub const VFW_E_COPYPROT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220867i32 as _);
pub const VFW_E_CORRUPT_GRAPH_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220939i32 as _);
pub const VFW_E_DDRAW_CAPS_NOT_SUITABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220877i32 as _);
pub const VFW_E_DDRAW_VERSION_NOT_SUITABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220868i32 as _);
pub const VFW_E_DUPLICATE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220947i32 as _);
pub const VFW_E_DVD_CHAPTER_DOES_NOT_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220715i32 as _);
pub const VFW_E_DVD_CMD_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220861i32 as _);
pub const VFW_E_DVD_DECNOTENOUGH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220869i32 as _);
pub const VFW_E_DVD_GRAPHNOTREADY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220871i32 as _);
pub const VFW_E_DVD_INCOMPATIBLE_REGION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220857i32 as _);
pub const VFW_E_DVD_INVALIDDOMAIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220873i32 as _);
pub const VFW_E_DVD_INVALID_DISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220847i32 as _);
pub const VFW_E_DVD_LOW_PARENTAL_LEVEL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220854i32 as _);
pub const VFW_E_DVD_MENU_DOES_NOT_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220862i32 as _);
pub const VFW_E_DVD_NONBLOCKING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220836i32 as _);
pub const VFW_E_DVD_NON_EVR_RENDERER_IN_FILTER_GRAPH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220834i32 as _);
pub const VFW_E_DVD_NOT_IN_KARAOKE_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220853i32 as _);
pub const VFW_E_DVD_NO_ATTRIBUTES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220856i32 as _);
pub const VFW_E_DVD_NO_BUTTON: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220872i32 as _);
pub const VFW_E_DVD_NO_GOUP_PGC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220855i32 as _);
pub const VFW_E_DVD_NO_RESUME_INFORMATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220846i32 as _);
pub const VFW_E_DVD_OPERATION_INHIBITED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220874i32 as _);
pub const VFW_E_DVD_RENDERFAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220870i32 as _);
pub const VFW_E_DVD_RESOLUTION_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220833i32 as _);
pub const VFW_E_DVD_STATE_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220859i32 as _);
pub const VFW_E_DVD_STATE_WRONG_DISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220858i32 as _);
pub const VFW_E_DVD_STATE_WRONG_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220860i32 as _);
pub const VFW_E_DVD_STREAM_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220849i32 as _);
pub const VFW_E_DVD_TITLE_UNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220848i32 as _);
pub const VFW_E_DVD_TOO_MANY_RENDERERS_IN_FILTER_GRAPH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220835i32 as _);
pub const VFW_E_DVD_VMR9_INCOMPATIBLEDEC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220838i32 as _);
pub const VFW_E_DVD_WRONG_SPEED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220863i32 as _);
pub const VFW_E_ENUM_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220944i32 as _);
pub const VFW_E_ENUM_OUT_OF_SYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220989i32 as _);
pub const VFW_E_FILE_TOO_SHORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220925i32 as _);
pub const VFW_E_FILTER_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220987i32 as _);
pub const VFW_E_FRAME_STEP_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220850i32 as _);
pub const VFW_E_INVALIDMEDIATYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
pub const VFW_E_INVALIDSUBTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220991i32 as _);
pub const VFW_E_INVALID_CLSID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220921i32 as _);
pub const VFW_E_INVALID_DIRECTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220984i32 as _);
pub const VFW_E_INVALID_FILE_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220945i32 as _);
pub const VFW_E_INVALID_FILE_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220924i32 as _);
pub const VFW_E_INVALID_MEDIA_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220920i32 as _);
pub const VFW_E_INVALID_RECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220951i32 as _);
pub const VFW_E_IN_FULLSCREEN_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220933i32 as _);
pub const VFW_E_MEDIA_TIME_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220911i32 as _);
pub const VFW_E_MONO_AUDIO_HW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220909i32 as _);
pub const VFW_E_MPEG_NOT_CONSTRAINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220898i32 as _);
pub const VFW_E_NEED_OWNER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220990i32 as _);
pub const VFW_E_NOT_ALLOWED_TO_SAVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220942i32 as _);
pub const VFW_E_NOT_COMMITTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220975i32 as _);
pub const VFW_E_NOT_CONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220983i32 as _);
pub const VFW_E_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220970i32 as _);
pub const VFW_E_NOT_IN_GRAPH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220897i32 as _);
pub const VFW_E_NOT_OVERLAY_CONNECTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220965i32 as _);
pub const VFW_E_NOT_PAUSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220955i32 as _);
pub const VFW_E_NOT_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220954i32 as _);
pub const VFW_E_NOT_SAMPLE_CONNECTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220964i32 as _);
pub const VFW_E_NOT_STOPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220956i32 as _);
pub const VFW_E_NO_ACCEPTABLE_TYPES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220985i32 as _);
pub const VFW_E_NO_ADVISE_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220935i32 as _);
pub const VFW_E_NO_ALLOCATOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220982i32 as _);
pub const VFW_E_NO_AUDIO_HARDWARE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220906i32 as _);
pub const VFW_E_NO_CAPTURE_HARDWARE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220875i32 as _);
pub const VFW_E_NO_CLOCK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220973i32 as _);
pub const VFW_E_NO_COLOR_KEY_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220961i32 as _);
pub const VFW_E_NO_COLOR_KEY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220966i32 as _);
pub const VFW_E_NO_COPP_HW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220837i32 as _);
pub const VFW_E_NO_DECOMPRESSOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220907i32 as _);
pub const VFW_E_NO_DISPLAY_PALETTE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220959i32 as _);
pub const VFW_E_NO_FULLSCREEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220934i32 as _);
pub const VFW_E_NO_INTERFACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220971i32 as _);
pub const VFW_E_NO_MODEX_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220936i32 as _);
pub const VFW_E_NO_PALETTE_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220960i32 as _);
pub const VFW_E_NO_SINK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220972i32 as _);
pub const VFW_E_NO_TIME_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220895i32 as _);
pub const VFW_E_NO_TIME_FORMAT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220910i32 as _);
pub const VFW_E_NO_TRANSPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220890i32 as _);
pub const VFW_E_NO_TYPES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220986i32 as _);
pub const VFW_E_NO_VP_HARDWARE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220876i32 as _);
pub const VFW_E_OUT_OF_VIDEO_MEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220879i32 as _);
pub const VFW_E_PALETTE_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220963i32 as _);
pub const VFW_E_PIN_ALREADY_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220844i32 as _);
pub const VFW_E_PIN_ALREADY_BLOCKED_ON_THIS_THREAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220845i32 as _);
pub const VFW_E_PROCESSOR_NOT_SUITABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220901i32 as _);
pub const VFW_E_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220894i32 as _);
pub const VFW_E_RPZA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220903i32 as _);
pub const VFW_E_RUNTIME_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220981i32 as _);
pub const VFW_E_SAMPLE_REJECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220949i32 as _);
pub const VFW_E_SAMPLE_REJECTED_EOS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220948i32 as _);
pub const VFW_E_SAMPLE_TIME_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220919i32 as _);
pub const VFW_E_SIZENOTSET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220974i32 as _);
pub const VFW_E_START_TIME_AFTER_END: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220952i32 as _);
pub const VFW_E_STATE_CHANGED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220957i32 as _);
pub const VFW_E_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220946i32 as _);
pub const VFW_E_TIME_ALREADY_PASSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220941i32 as _);
pub const VFW_E_TIME_EXPIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220865i32 as _);
pub const VFW_E_TOO_MANY_COLORS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220958i32 as _);
pub const VFW_E_TYPE_NOT_ACCEPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220950i32 as _);
pub const VFW_E_UNKNOWN_FILE_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220928i32 as _);
pub const VFW_E_UNSUPPORTED_AUDIO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220900i32 as _);
pub const VFW_E_UNSUPPORTED_STREAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220891i32 as _);
pub const VFW_E_UNSUPPORTED_VIDEO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220899i32 as _);
pub const VFW_E_VMR_NOT_IN_MIXER_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220842i32 as _);
pub const VFW_E_VMR_NO_AP_SUPPLIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220841i32 as _);
pub const VFW_E_VMR_NO_DEINTERLACE_HW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220840i32 as _);
pub const VFW_E_VMR_NO_PROCAMP_HW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220839i32 as _);
pub const VFW_E_VP_NEGOTIATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220878i32 as _);
pub const VFW_E_WRONG_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220953i32 as _);
#[repr(C)]
pub struct VFW_FILTERLIST(i32);
pub const VFW_FIRST_CODE: u32 = 512u32;
pub const VFW_S_AUDIO_NOT_RENDERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262744i32 as _);
pub const VFW_S_CANT_CUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262760i32 as _);
pub const VFW_S_CONNECTIONS_DEFERRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262726i32 as _);
pub const VFW_S_DUPLICATE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262701i32 as _);
pub const VFW_S_DVD_CHANNEL_CONTENTS_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262796i32 as _);
pub const VFW_S_DVD_NON_ONE_SEQUENTIAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262784i32 as _);
pub const VFW_S_DVD_NOT_ACCURATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262797i32 as _);
pub const VFW_S_DVD_RENDER_STATUS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262944i32 as _);
pub const VFW_S_ESTIMATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262752i32 as _);
pub const VFW_S_MEDIA_TYPE_IGNORED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262740i32 as _);
pub const VFW_S_NOPREVIEWPIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262782i32 as _);
pub const VFW_S_NO_MORE_ITEMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262403i32 as _);
pub const VFW_S_NO_STOP_TIME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262768i32 as _);
pub const VFW_S_PARTIAL_RENDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262722i32 as _);
pub const VFW_S_RESERVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262755i32 as _);
pub const VFW_S_RESOURCE_NOT_NEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262736i32 as _);
pub const VFW_S_RPZA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262746i32 as _);
pub const VFW_S_SOME_DATA_IGNORED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262725i32 as _);
pub const VFW_S_STATE_INTERMEDIATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262711i32 as _);
pub const VFW_S_STREAM_OFF: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262759i32 as _);
pub const VFW_S_VIDEO_NOT_RENDERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262743i32 as _);
#[repr(transparent)]
pub struct VIDEOENCODER_BITRATE_MODE(pub i32);
pub const ConstantBitRate: VIDEOENCODER_BITRATE_MODE = VIDEOENCODER_BITRATE_MODE(0i32);
pub const VariableBitRateAverage: VIDEOENCODER_BITRATE_MODE = VIDEOENCODER_BITRATE_MODE(1i32);
pub const VariableBitRatePeak: VIDEOENCODER_BITRATE_MODE = VIDEOENCODER_BITRATE_MODE(2i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VIDEOINFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VIDEOINFOHEADER(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VIDEOINFOHEADER2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VIDEO_STREAM_CONFIG_CAPS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[repr(C)]
pub struct VMR9AllocationInfo(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VMR9AlphaBitmap(i32);
#[repr(transparent)]
pub struct VMR9AlphaBitmapFlags(pub i32);
pub const VMR9AlphaBitmap_Disable: VMR9AlphaBitmapFlags = VMR9AlphaBitmapFlags(1i32);
pub const VMR9AlphaBitmap_hDC: VMR9AlphaBitmapFlags = VMR9AlphaBitmapFlags(2i32);
pub const VMR9AlphaBitmap_EntireDDS: VMR9AlphaBitmapFlags = VMR9AlphaBitmapFlags(4i32);
pub const VMR9AlphaBitmap_SrcColorKey: VMR9AlphaBitmapFlags = VMR9AlphaBitmapFlags(8i32);
pub const VMR9AlphaBitmap_SrcRect: VMR9AlphaBitmapFlags = VMR9AlphaBitmapFlags(16i32);
pub const VMR9AlphaBitmap_FilterMode: VMR9AlphaBitmapFlags = VMR9AlphaBitmapFlags(32i32);
#[repr(transparent)]
pub struct VMR9AspectRatioMode(pub i32);
pub const VMR9ARMode_None: VMR9AspectRatioMode = VMR9AspectRatioMode(0i32);
pub const VMR9ARMode_LetterBox: VMR9AspectRatioMode = VMR9AspectRatioMode(1i32);
#[repr(C)]
pub struct VMR9DeinterlaceCaps(i32);
#[repr(transparent)]
pub struct VMR9DeinterlacePrefs(pub i32);
pub const DeinterlacePref9_NextBest: VMR9DeinterlacePrefs = VMR9DeinterlacePrefs(1i32);
pub const DeinterlacePref9_BOB: VMR9DeinterlacePrefs = VMR9DeinterlacePrefs(2i32);
pub const DeinterlacePref9_Weave: VMR9DeinterlacePrefs = VMR9DeinterlacePrefs(4i32);
pub const DeinterlacePref9_Mask: VMR9DeinterlacePrefs = VMR9DeinterlacePrefs(7i32);
#[repr(transparent)]
pub struct VMR9DeinterlaceTech(pub i32);
pub const DeinterlaceTech9_Unknown: VMR9DeinterlaceTech = VMR9DeinterlaceTech(0i32);
pub const DeinterlaceTech9_BOBLineReplicate: VMR9DeinterlaceTech = VMR9DeinterlaceTech(1i32);
pub const DeinterlaceTech9_BOBVerticalStretch: VMR9DeinterlaceTech = VMR9DeinterlaceTech(2i32);
pub const DeinterlaceTech9_MedianFiltering: VMR9DeinterlaceTech = VMR9DeinterlaceTech(4i32);
pub const DeinterlaceTech9_EdgeFiltering: VMR9DeinterlaceTech = VMR9DeinterlaceTech(16i32);
pub const DeinterlaceTech9_FieldAdaptive: VMR9DeinterlaceTech = VMR9DeinterlaceTech(32i32);
pub const DeinterlaceTech9_PixelAdaptive: VMR9DeinterlaceTech = VMR9DeinterlaceTech(64i32);
pub const DeinterlaceTech9_MotionVectorSteered: VMR9DeinterlaceTech = VMR9DeinterlaceTech(128i32);
#[repr(C)]
pub struct VMR9Frequency(i32);
#[repr(transparent)]
pub struct VMR9MixerPrefs(pub i32);
pub const MixerPref9_NoDecimation: VMR9MixerPrefs = VMR9MixerPrefs(1i32);
pub const MixerPref9_DecimateOutput: VMR9MixerPrefs = VMR9MixerPrefs(2i32);
pub const MixerPref9_ARAdjustXorY: VMR9MixerPrefs = VMR9MixerPrefs(4i32);
pub const MixerPref9_NonSquareMixing: VMR9MixerPrefs = VMR9MixerPrefs(8i32);
pub const MixerPref9_DecimateMask: VMR9MixerPrefs = VMR9MixerPrefs(15i32);
pub const MixerPref9_BiLinearFiltering: VMR9MixerPrefs = VMR9MixerPrefs(16i32);
pub const MixerPref9_PointFiltering: VMR9MixerPrefs = VMR9MixerPrefs(32i32);
pub const MixerPref9_AnisotropicFiltering: VMR9MixerPrefs = VMR9MixerPrefs(64i32);
pub const MixerPref9_PyramidalQuadFiltering: VMR9MixerPrefs = VMR9MixerPrefs(128i32);
pub const MixerPref9_GaussianQuadFiltering: VMR9MixerPrefs = VMR9MixerPrefs(256i32);
pub const MixerPref9_FilteringReserved: VMR9MixerPrefs = VMR9MixerPrefs(3584i32);
pub const MixerPref9_FilteringMask: VMR9MixerPrefs = VMR9MixerPrefs(4080i32);
pub const MixerPref9_RenderTargetRGB: VMR9MixerPrefs = VMR9MixerPrefs(4096i32);
pub const MixerPref9_RenderTargetYUV: VMR9MixerPrefs = VMR9MixerPrefs(8192i32);
pub const MixerPref9_RenderTargetReserved: VMR9MixerPrefs = VMR9MixerPrefs(1032192i32);
pub const MixerPref9_RenderTargetMask: VMR9MixerPrefs = VMR9MixerPrefs(1044480i32);
pub const MixerPref9_DynamicSwitchToBOB: VMR9MixerPrefs = VMR9MixerPrefs(1048576i32);
pub const MixerPref9_DynamicDecimateBy2: VMR9MixerPrefs = VMR9MixerPrefs(2097152i32);
pub const MixerPref9_DynamicReserved: VMR9MixerPrefs = VMR9MixerPrefs(12582912i32);
pub const MixerPref9_DynamicMask: VMR9MixerPrefs = VMR9MixerPrefs(15728640i32);
#[repr(transparent)]
pub struct VMR9Mode(pub i32);
pub const VMR9Mode_Windowed: VMR9Mode = VMR9Mode(1i32);
pub const VMR9Mode_Windowless: VMR9Mode = VMR9Mode(2i32);
pub const VMR9Mode_Renderless: VMR9Mode = VMR9Mode(4i32);
pub const VMR9Mode_Mask: VMR9Mode = VMR9Mode(7i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VMR9MonitorInfo(i32);
#[repr(C)]
pub struct VMR9NormalizedRect(i32);
#[repr(transparent)]
pub struct VMR9PresentationFlags(pub i32);
pub const VMR9Sample_SyncPoint: VMR9PresentationFlags = VMR9PresentationFlags(1i32);
pub const VMR9Sample_Preroll: VMR9PresentationFlags = VMR9PresentationFlags(2i32);
pub const VMR9Sample_Discontinuity: VMR9PresentationFlags = VMR9PresentationFlags(4i32);
pub const VMR9Sample_TimeValid: VMR9PresentationFlags = VMR9PresentationFlags(8i32);
pub const VMR9Sample_SrcDstRectsValid: VMR9PresentationFlags = VMR9PresentationFlags(16i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[repr(C)]
pub struct VMR9PresentationInfo(i32);
#[repr(C)]
pub struct VMR9ProcAmpControl(i32);
#[repr(transparent)]
pub struct VMR9ProcAmpControlFlags(pub i32);
pub const ProcAmpControl9_Brightness: VMR9ProcAmpControlFlags = VMR9ProcAmpControlFlags(1i32);
pub const ProcAmpControl9_Contrast: VMR9ProcAmpControlFlags = VMR9ProcAmpControlFlags(2i32);
pub const ProcAmpControl9_Hue: VMR9ProcAmpControlFlags = VMR9ProcAmpControlFlags(4i32);
pub const ProcAmpControl9_Saturation: VMR9ProcAmpControlFlags = VMR9ProcAmpControlFlags(8i32);
pub const ProcAmpControl9_Mask: VMR9ProcAmpControlFlags = VMR9ProcAmpControlFlags(15i32);
#[repr(C)]
pub struct VMR9ProcAmpControlRange(i32);
#[repr(transparent)]
pub struct VMR9RenderPrefs(pub i32);
pub const RenderPrefs9_DoNotRenderBorder: VMR9RenderPrefs = VMR9RenderPrefs(1i32);
pub const RenderPrefs9_Mask: VMR9RenderPrefs = VMR9RenderPrefs(1i32);
#[repr(transparent)]
pub struct VMR9SurfaceAllocationFlags(pub i32);
pub const VMR9AllocFlag_3DRenderTarget: VMR9SurfaceAllocationFlags = VMR9SurfaceAllocationFlags(1i32);
pub const VMR9AllocFlag_DXVATarget: VMR9SurfaceAllocationFlags = VMR9SurfaceAllocationFlags(2i32);
pub const VMR9AllocFlag_TextureSurface: VMR9SurfaceAllocationFlags = VMR9SurfaceAllocationFlags(4i32);
pub const VMR9AllocFlag_OffscreenSurface: VMR9SurfaceAllocationFlags = VMR9SurfaceAllocationFlags(8i32);
pub const VMR9AllocFlag_RGBDynamicSwitch: VMR9SurfaceAllocationFlags = VMR9SurfaceAllocationFlags(16i32);
pub const VMR9AllocFlag_UsageReserved: VMR9SurfaceAllocationFlags = VMR9SurfaceAllocationFlags(224i32);
pub const VMR9AllocFlag_UsageMask: VMR9SurfaceAllocationFlags = VMR9SurfaceAllocationFlags(255i32);
#[repr(C)]
pub struct VMR9VideoDesc(i32);
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[repr(C)]
pub struct VMR9VideoStreamInfo(i32);
#[repr(transparent)]
pub struct VMR9_SampleFormat(pub i32);
pub const VMR9_SampleReserved: VMR9_SampleFormat = VMR9_SampleFormat(1i32);
pub const VMR9_SampleProgressiveFrame: VMR9_SampleFormat = VMR9_SampleFormat(2i32);
pub const VMR9_SampleFieldInterleavedEvenFirst: VMR9_SampleFormat = VMR9_SampleFormat(3i32);
pub const VMR9_SampleFieldInterleavedOddFirst: VMR9_SampleFormat = VMR9_SampleFormat(4i32);
pub const VMR9_SampleFieldSingleEven: VMR9_SampleFormat = VMR9_SampleFormat(5i32);
pub const VMR9_SampleFieldSingleOdd: VMR9_SampleFormat = VMR9_SampleFormat(6i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VMRALLOCATIONINFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VMRALPHABITMAP(i32);
pub const VMRBITMAP_DISABLE: u32 = 1u32;
pub const VMRBITMAP_ENTIREDDS: u32 = 4u32;
pub const VMRBITMAP_HDC: u32 = 2u32;
pub const VMRBITMAP_SRCCOLORKEY: u32 = 8u32;
pub const VMRBITMAP_SRCRECT: u32 = 16u32;
#[repr(C)]
pub struct VMRDeinterlaceCaps(i32);
#[repr(transparent)]
pub struct VMRDeinterlacePrefs(pub i32);
pub const DeinterlacePref_NextBest: VMRDeinterlacePrefs = VMRDeinterlacePrefs(1i32);
pub const DeinterlacePref_BOB: VMRDeinterlacePrefs = VMRDeinterlacePrefs(2i32);
pub const DeinterlacePref_Weave: VMRDeinterlacePrefs = VMRDeinterlacePrefs(4i32);
pub const DeinterlacePref_Mask: VMRDeinterlacePrefs = VMRDeinterlacePrefs(7i32);
#[repr(transparent)]
pub struct VMRDeinterlaceTech(pub i32);
pub const DeinterlaceTech_Unknown: VMRDeinterlaceTech = VMRDeinterlaceTech(0i32);
pub const DeinterlaceTech_BOBLineReplicate: VMRDeinterlaceTech = VMRDeinterlaceTech(1i32);
pub const DeinterlaceTech_BOBVerticalStretch: VMRDeinterlaceTech = VMRDeinterlaceTech(2i32);
pub const DeinterlaceTech_MedianFiltering: VMRDeinterlaceTech = VMRDeinterlaceTech(4i32);
pub const DeinterlaceTech_EdgeFiltering: VMRDeinterlaceTech = VMRDeinterlaceTech(16i32);
pub const DeinterlaceTech_FieldAdaptive: VMRDeinterlaceTech = VMRDeinterlaceTech(32i32);
pub const DeinterlaceTech_PixelAdaptive: VMRDeinterlaceTech = VMRDeinterlaceTech(64i32);
pub const DeinterlaceTech_MotionVectorSteered: VMRDeinterlaceTech = VMRDeinterlaceTech(128i32);
#[repr(C)]
pub struct VMRFrequency(i32);
#[repr(C)]
pub struct VMRGUID(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VMRMONITORINFO(i32);
#[repr(transparent)]
pub struct VMRMixerPrefs(pub i32);
pub const MixerPref_NoDecimation: VMRMixerPrefs = VMRMixerPrefs(1i32);
pub const MixerPref_DecimateOutput: VMRMixerPrefs = VMRMixerPrefs(2i32);
pub const MixerPref_ARAdjustXorY: VMRMixerPrefs = VMRMixerPrefs(4i32);
pub const MixerPref_DecimationReserved: VMRMixerPrefs = VMRMixerPrefs(8i32);
pub const MixerPref_DecimateMask: VMRMixerPrefs = VMRMixerPrefs(15i32);
pub const MixerPref_BiLinearFiltering: VMRMixerPrefs = VMRMixerPrefs(16i32);
pub const MixerPref_PointFiltering: VMRMixerPrefs = VMRMixerPrefs(32i32);
pub const MixerPref_FilteringMask: VMRMixerPrefs = VMRMixerPrefs(240i32);
pub const MixerPref_RenderTargetRGB: VMRMixerPrefs = VMRMixerPrefs(256i32);
pub const MixerPref_RenderTargetYUV: VMRMixerPrefs = VMRMixerPrefs(4096i32);
pub const MixerPref_RenderTargetYUV420: VMRMixerPrefs = VMRMixerPrefs(512i32);
pub const MixerPref_RenderTargetYUV422: VMRMixerPrefs = VMRMixerPrefs(1024i32);
pub const MixerPref_RenderTargetYUV444: VMRMixerPrefs = VMRMixerPrefs(2048i32);
pub const MixerPref_RenderTargetReserved: VMRMixerPrefs = VMRMixerPrefs(57344i32);
pub const MixerPref_RenderTargetMask: VMRMixerPrefs = VMRMixerPrefs(65280i32);
pub const MixerPref_DynamicSwitchToBOB: VMRMixerPrefs = VMRMixerPrefs(65536i32);
pub const MixerPref_DynamicDecimateBy2: VMRMixerPrefs = VMRMixerPrefs(131072i32);
pub const MixerPref_DynamicReserved: VMRMixerPrefs = VMRMixerPrefs(786432i32);
pub const MixerPref_DynamicMask: VMRMixerPrefs = VMRMixerPrefs(983040i32);
#[repr(transparent)]
pub struct VMRMode(pub i32);
pub const VMRMode_Windowed: VMRMode = VMRMode(1i32);
pub const VMRMode_Windowless: VMRMode = VMRMode(2i32);
pub const VMRMode_Renderless: VMRMode = VMRMode(4i32);
pub const VMRMode_Mask: VMRMode = VMRMode(7i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[repr(C)]
pub struct VMRPRESENTATIONINFO(i32);
#[repr(transparent)]
pub struct VMRPresentationFlags(pub i32);
pub const VMRSample_SyncPoint: VMRPresentationFlags = VMRPresentationFlags(1i32);
pub const VMRSample_Preroll: VMRPresentationFlags = VMRPresentationFlags(2i32);
pub const VMRSample_Discontinuity: VMRPresentationFlags = VMRPresentationFlags(4i32);
pub const VMRSample_TimeValid: VMRPresentationFlags = VMRPresentationFlags(8i32);
pub const VMRSample_SrcDstRectsValid: VMRPresentationFlags = VMRPresentationFlags(16i32);
#[repr(transparent)]
pub struct VMRRenderPrefs(pub i32);
pub const RenderPrefs_RestrictToInitialMonitor: VMRRenderPrefs = VMRRenderPrefs(0i32);
pub const RenderPrefs_ForceOffscreen: VMRRenderPrefs = VMRRenderPrefs(1i32);
pub const RenderPrefs_ForceOverlays: VMRRenderPrefs = VMRRenderPrefs(2i32);
pub const RenderPrefs_AllowOverlays: VMRRenderPrefs = VMRRenderPrefs(0i32);
pub const RenderPrefs_AllowOffscreen: VMRRenderPrefs = VMRRenderPrefs(0i32);
pub const RenderPrefs_DoNotRenderColorKeyAndBorder: VMRRenderPrefs = VMRRenderPrefs(8i32);
pub const RenderPrefs_Reserved: VMRRenderPrefs = VMRRenderPrefs(16i32);
pub const RenderPrefs_PreferAGPMemWhenMixing: VMRRenderPrefs = VMRRenderPrefs(32i32);
pub const RenderPrefs_Mask: VMRRenderPrefs = VMRRenderPrefs(63i32);
#[repr(transparent)]
pub struct VMRSurfaceAllocationFlags(pub i32);
pub const AMAP_PIXELFORMAT_VALID: VMRSurfaceAllocationFlags = VMRSurfaceAllocationFlags(1i32);
pub const AMAP_3D_TARGET: VMRSurfaceAllocationFlags = VMRSurfaceAllocationFlags(2i32);
pub const AMAP_ALLOW_SYSMEM: VMRSurfaceAllocationFlags = VMRSurfaceAllocationFlags(4i32);
pub const AMAP_FORCE_SYSMEM: VMRSurfaceAllocationFlags = VMRSurfaceAllocationFlags(8i32);
pub const AMAP_DIRECTED_FLIP: VMRSurfaceAllocationFlags = VMRSurfaceAllocationFlags(16i32);
pub const AMAP_DXVA_TARGET: VMRSurfaceAllocationFlags = VMRSurfaceAllocationFlags(32i32);
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[repr(C)]
pub struct VMRVIDEOSTREAMINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VMRVideoDesc(i32);
#[repr(transparent)]
pub struct VMR_ASPECT_RATIO_MODE(pub i32);
pub const VMR_ARMODE_NONE: VMR_ASPECT_RATIO_MODE = VMR_ASPECT_RATIO_MODE(0i32);
pub const VMR_ARMODE_LETTER_BOX: VMR_ASPECT_RATIO_MODE = VMR_ASPECT_RATIO_MODE(1i32);
pub const VMR_NOTSUPPORTED: u32 = 0u32;
pub const VMR_RENDER_DEVICE_OVERLAY: u32 = 1u32;
pub const VMR_RENDER_DEVICE_SYSMEM: u32 = 4u32;
pub const VMR_RENDER_DEVICE_VIDMEM: u32 = 2u32;
pub const VMR_SUPPORTED: u32 = 1u32;
#[repr(transparent)]
pub struct VfwCaptureDialogs(pub i32);
pub const VfwCaptureDialog_Source: VfwCaptureDialogs = VfwCaptureDialogs(1i32);
pub const VfwCaptureDialog_Format: VfwCaptureDialogs = VfwCaptureDialogs(2i32);
pub const VfwCaptureDialog_Display: VfwCaptureDialogs = VfwCaptureDialogs(4i32);
#[repr(transparent)]
pub struct VfwCompressDialogs(pub i32);
pub const VfwCompressDialog_Config: VfwCompressDialogs = VfwCompressDialogs(1i32);
pub const VfwCompressDialog_About: VfwCompressDialogs = VfwCompressDialogs(2i32);
pub const VfwCompressDialog_QueryConfig: VfwCompressDialogs = VfwCompressDialogs(4i32);
pub const VfwCompressDialog_QueryAbout: VfwCompressDialogs = VfwCompressDialogs(8i32);
#[repr(transparent)]
pub struct VideoControlFlags(pub i32);
pub const VideoControlFlag_FlipHorizontal: VideoControlFlags = VideoControlFlags(1i32);
pub const VideoControlFlag_FlipVertical: VideoControlFlags = VideoControlFlags(2i32);
pub const VideoControlFlag_ExternalTriggerEnable: VideoControlFlags = VideoControlFlags(4i32);
pub const VideoControlFlag_Trigger: VideoControlFlags = VideoControlFlags(8i32);
#[repr(transparent)]
pub struct VideoCopyProtectionType(pub i32);
pub const VideoCopyProtectionMacrovisionBasic: VideoCopyProtectionType = VideoCopyProtectionType(0i32);
pub const VideoCopyProtectionMacrovisionCBI: VideoCopyProtectionType = VideoCopyProtectionType(1i32);
#[repr(transparent)]
pub struct VideoProcAmpFlags(pub i32);
pub const VideoProcAmp_Flags_Auto: VideoProcAmpFlags = VideoProcAmpFlags(1i32);
pub const VideoProcAmp_Flags_Manual: VideoProcAmpFlags = VideoProcAmpFlags(2i32);
#[repr(transparent)]
pub struct VideoProcAmpProperty(pub i32);
pub const VideoProcAmp_Brightness: VideoProcAmpProperty = VideoProcAmpProperty(0i32);
pub const VideoProcAmp_Contrast: VideoProcAmpProperty = VideoProcAmpProperty(1i32);
pub const VideoProcAmp_Hue: VideoProcAmpProperty = VideoProcAmpProperty(2i32);
pub const VideoProcAmp_Saturation: VideoProcAmpProperty = VideoProcAmpProperty(3i32);
pub const VideoProcAmp_Sharpness: VideoProcAmpProperty = VideoProcAmpProperty(4i32);
pub const VideoProcAmp_Gamma: VideoProcAmpProperty = VideoProcAmpProperty(5i32);
pub const VideoProcAmp_ColorEnable: VideoProcAmpProperty = VideoProcAmpProperty(6i32);
pub const VideoProcAmp_WhiteBalance: VideoProcAmpProperty = VideoProcAmpProperty(7i32);
pub const VideoProcAmp_BacklightCompensation: VideoProcAmpProperty = VideoProcAmpProperty(8i32);
pub const VideoProcAmp_Gain: VideoProcAmpProperty = VideoProcAmpProperty(9i32);
#[repr(C)]
pub struct WMDRMProtectionInfo(i32);
#[repr(C)]
pub struct XDSCodec(i32);
#[repr(C)]
pub struct XDSToRat(i32);
#[repr(transparent)]
pub struct _AMRESCTL_RESERVEFLAGS(pub i32);
pub const AMRESCTL_RESERVEFLAGS_RESERVE: _AMRESCTL_RESERVEFLAGS = _AMRESCTL_RESERVEFLAGS(0i32);
pub const AMRESCTL_RESERVEFLAGS_UNRESERVE: _AMRESCTL_RESERVEFLAGS = _AMRESCTL_RESERVEFLAGS(1i32);
#[repr(transparent)]
pub struct _AMSTREAMSELECTENABLEFLAGS(pub i32);
pub const AMSTREAMSELECTENABLE_ENABLE: _AMSTREAMSELECTENABLEFLAGS = _AMSTREAMSELECTENABLEFLAGS(1i32);
pub const AMSTREAMSELECTENABLE_ENABLEALL: _AMSTREAMSELECTENABLEFLAGS = _AMSTREAMSELECTENABLEFLAGS(2i32);
#[repr(transparent)]
pub struct _AMSTREAMSELECTINFOFLAGS(pub i32);
pub const AMSTREAMSELECTINFO_ENABLED: _AMSTREAMSELECTINFOFLAGS = _AMSTREAMSELECTINFOFLAGS(1i32);
pub const AMSTREAMSELECTINFO_EXCLUSIVE: _AMSTREAMSELECTINFOFLAGS = _AMSTREAMSELECTINFOFLAGS(2i32);
#[repr(transparent)]
pub struct _AM_AUDIO_RENDERER_STAT_PARAM(pub i32);
pub const AM_AUDREND_STAT_PARAM_BREAK_COUNT: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(1i32);
pub const AM_AUDREND_STAT_PARAM_SLAVE_MODE: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(2i32);
pub const AM_AUDREND_STAT_PARAM_SILENCE_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(3i32);
pub const AM_AUDREND_STAT_PARAM_LAST_BUFFER_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(4i32);
pub const AM_AUDREND_STAT_PARAM_DISCONTINUITIES: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(5i32);
pub const AM_AUDREND_STAT_PARAM_SLAVE_RATE: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(6i32);
pub const AM_AUDREND_STAT_PARAM_SLAVE_DROPWRITE_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(7i32);
pub const AM_AUDREND_STAT_PARAM_SLAVE_HIGHLOWERROR: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(8i32);
pub const AM_AUDREND_STAT_PARAM_SLAVE_LASTHIGHLOWERROR: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(9i32);
pub const AM_AUDREND_STAT_PARAM_SLAVE_ACCUMERROR: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(10i32);
pub const AM_AUDREND_STAT_PARAM_BUFFERFULLNESS: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(11i32);
pub const AM_AUDREND_STAT_PARAM_JITTER: _AM_AUDIO_RENDERER_STAT_PARAM = _AM_AUDIO_RENDERER_STAT_PARAM(12i32);
#[repr(transparent)]
pub struct _AM_FILTER_MISC_FLAGS(pub i32);
pub const AM_FILTER_MISC_FLAGS_IS_RENDERER: _AM_FILTER_MISC_FLAGS = _AM_FILTER_MISC_FLAGS(1i32);
pub const AM_FILTER_MISC_FLAGS_IS_SOURCE: _AM_FILTER_MISC_FLAGS = _AM_FILTER_MISC_FLAGS(2i32);
#[repr(transparent)]
pub struct _AM_INTF_SEARCH_FLAGS(pub i32);
pub const AM_INTF_SEARCH_INPUT_PIN: _AM_INTF_SEARCH_FLAGS = _AM_INTF_SEARCH_FLAGS(1i32);
pub const AM_INTF_SEARCH_OUTPUT_PIN: _AM_INTF_SEARCH_FLAGS = _AM_INTF_SEARCH_FLAGS(2i32);
pub const AM_INTF_SEARCH_FILTER: _AM_INTF_SEARCH_FLAGS = _AM_INTF_SEARCH_FLAGS(4i32);
#[repr(transparent)]
pub struct _AM_OVERLAY_NOTIFY_FLAGS(pub i32);
pub const AM_OVERLAY_NOTIFY_VISIBLE_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = _AM_OVERLAY_NOTIFY_FLAGS(1i32);
pub const AM_OVERLAY_NOTIFY_SOURCE_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = _AM_OVERLAY_NOTIFY_FLAGS(2i32);
pub const AM_OVERLAY_NOTIFY_DEST_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = _AM_OVERLAY_NOTIFY_FLAGS(4i32);
#[repr(transparent)]
pub struct _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS(pub i32);
pub const AM_PIN_FLOW_CONTROL_BLOCK: _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS = _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS(1i32);
#[repr(transparent)]
pub struct _AM_PUSHSOURCE_FLAGS(pub i32);
pub const AM_PUSHSOURCECAPS_INTERNAL_RM: _AM_PUSHSOURCE_FLAGS = _AM_PUSHSOURCE_FLAGS(1i32);
pub const AM_PUSHSOURCECAPS_NOT_LIVE: _AM_PUSHSOURCE_FLAGS = _AM_PUSHSOURCE_FLAGS(2i32);
pub const AM_PUSHSOURCECAPS_PRIVATE_CLOCK: _AM_PUSHSOURCE_FLAGS = _AM_PUSHSOURCE_FLAGS(4i32);
pub const AM_PUSHSOURCEREQS_USE_STREAM_CLOCK: _AM_PUSHSOURCE_FLAGS = _AM_PUSHSOURCE_FLAGS(65536i32);
pub const AM_PUSHSOURCEREQS_USE_CLOCK_CHAIN: _AM_PUSHSOURCE_FLAGS = _AM_PUSHSOURCE_FLAGS(131072i32);
#[repr(transparent)]
pub struct _AM_RENSDEREXFLAGS(pub i32);
pub const AM_RENDEREX_RENDERTOEXISTINGRENDERERS: _AM_RENSDEREXFLAGS = _AM_RENSDEREXFLAGS(1i32);
#[repr(transparent)]
pub struct _DVDECODERRESOLUTION(pub i32);
pub const DVDECODERRESOLUTION_720x480: _DVDECODERRESOLUTION = _DVDECODERRESOLUTION(1000i32);
pub const DVDECODERRESOLUTION_360x240: _DVDECODERRESOLUTION = _DVDECODERRESOLUTION(1001i32);
pub const DVDECODERRESOLUTION_180x120: _DVDECODERRESOLUTION = _DVDECODERRESOLUTION(1002i32);
pub const DVDECODERRESOLUTION_88x60: _DVDECODERRESOLUTION = _DVDECODERRESOLUTION(1003i32);
#[repr(transparent)]
pub struct _DVENCODERFORMAT(pub i32);
pub const DVENCODERFORMAT_DVSD: _DVENCODERFORMAT = _DVENCODERFORMAT(2007i32);
pub const DVENCODERFORMAT_DVHD: _DVENCODERFORMAT = _DVENCODERFORMAT(2008i32);
pub const DVENCODERFORMAT_DVSL: _DVENCODERFORMAT = _DVENCODERFORMAT(2009i32);
#[repr(transparent)]
pub struct _DVENCODERRESOLUTION(pub i32);
pub const DVENCODERRESOLUTION_720x480: _DVENCODERRESOLUTION = _DVENCODERRESOLUTION(2012i32);
pub const DVENCODERRESOLUTION_360x240: _DVENCODERRESOLUTION = _DVENCODERRESOLUTION(2013i32);
pub const DVENCODERRESOLUTION_180x120: _DVENCODERRESOLUTION = _DVENCODERRESOLUTION(2014i32);
pub const DVENCODERRESOLUTION_88x60: _DVENCODERRESOLUTION = _DVENCODERRESOLUTION(2015i32);
#[repr(transparent)]
pub struct _DVENCODERVIDEOFORMAT(pub i32);
pub const DVENCODERVIDEOFORMAT_NTSC: _DVENCODERVIDEOFORMAT = _DVENCODERVIDEOFORMAT(2000i32);
pub const DVENCODERVIDEOFORMAT_PAL: _DVENCODERVIDEOFORMAT = _DVENCODERVIDEOFORMAT(2001i32);
#[repr(transparent)]
pub struct _DVRESOLUTION(pub i32);
pub const DVRESOLUTION_FULL: _DVRESOLUTION = _DVRESOLUTION(1000i32);
pub const DVRESOLUTION_HALF: _DVRESOLUTION = _DVRESOLUTION(1001i32);
pub const DVRESOLUTION_QUARTER: _DVRESOLUTION = _DVRESOLUTION(1002i32);
pub const DVRESOLUTION_DC: _DVRESOLUTION = _DVRESOLUTION(1003i32);
#[repr(transparent)]
pub struct _IMSVidCtlEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _REM_FILTER_FLAGS(pub i32);
pub const REMFILTERF_LEAVECONNECTED: _REM_FILTER_FLAGS = _REM_FILTER_FLAGS(1i32);
#[repr(C)]
pub struct _avitcdlindex(i32);
#[repr(C)]
pub struct _avitimedindex(i32);
pub const g_wszExcludeScriptStreamDeliverySynchronization: &'static str = "ExcludeScriptStreamDeliverySynchronization";
pub const g_wszStreamBufferRecordingAlbumArtist: &'static str = "WM/AlbumArtist";
pub const g_wszStreamBufferRecordingAlbumCoverURL: &'static str = "WM/AlbumCoverURL";
pub const g_wszStreamBufferRecordingAlbumTitle: &'static str = "WM/AlbumTitle";
pub const g_wszStreamBufferRecordingAspectRatioX: &'static str = "AspectRatioX";
pub const g_wszStreamBufferRecordingAspectRatioY: &'static str = "AspectRatioY";
pub const g_wszStreamBufferRecordingAuthor: &'static str = "Author";
pub const g_wszStreamBufferRecordingBannerImageData: &'static str = "BannerImageData";
pub const g_wszStreamBufferRecordingBannerImageType: &'static str = "BannerImageType";
pub const g_wszStreamBufferRecordingBannerImageURL: &'static str = "BannerImageURL";
pub const g_wszStreamBufferRecordingBitrate: &'static str = "Bitrate";
pub const g_wszStreamBufferRecordingBroadcast: &'static str = "Broadcast";
pub const g_wszStreamBufferRecordingComposer: &'static str = "WM/Composer";
pub const g_wszStreamBufferRecordingCopyright: &'static str = "Copyright";
pub const g_wszStreamBufferRecordingCopyrightURL: &'static str = "CopyrightURL";
pub const g_wszStreamBufferRecordingCurrentBitrate: &'static str = "CurrentBitrate";
pub const g_wszStreamBufferRecordingDRM_Flags: &'static str = "DRM_Flags";
pub const g_wszStreamBufferRecordingDRM_Level: &'static str = "DRM_Level";
pub const g_wszStreamBufferRecordingDescription: &'static str = "Description";
pub const g_wszStreamBufferRecordingDuration: &'static str = "Duration";
pub const g_wszStreamBufferRecordingFileSize: &'static str = "FileSize";
pub const g_wszStreamBufferRecordingGenre: &'static str = "WM/Genre";
pub const g_wszStreamBufferRecordingGenreID: &'static str = "WM/GenreID";
pub const g_wszStreamBufferRecordingHasArbitraryDataStream: &'static str = "HasArbitraryDataStream";
pub const g_wszStreamBufferRecordingHasAttachedImages: &'static str = "HasAttachedImages";
pub const g_wszStreamBufferRecordingHasAudio: &'static str = "HasAudio";
pub const g_wszStreamBufferRecordingHasFileTransferStream: &'static str = "HasFileTransferStream";
pub const g_wszStreamBufferRecordingHasImage: &'static str = "HasImage";
pub const g_wszStreamBufferRecordingHasScript: &'static str = "HasScript";
pub const g_wszStreamBufferRecordingHasVideo: &'static str = "HasVideo";
pub const g_wszStreamBufferRecordingIsVBR: &'static str = "IsVBR";
pub const g_wszStreamBufferRecordingLyrics: &'static str = "WM/Lyrics";
pub const g_wszStreamBufferRecordingMCDI: &'static str = "WM/MCDI";
pub const g_wszStreamBufferRecordingNSCAddress: &'static str = "NSC_Address";
pub const g_wszStreamBufferRecordingNSCDescription: &'static str = "NSC_Description";
pub const g_wszStreamBufferRecordingNSCEmail: &'static str = "NSC_Email";
pub const g_wszStreamBufferRecordingNSCName: &'static str = "NSC_Name";
pub const g_wszStreamBufferRecordingNSCPhone: &'static str = "NSC_Phone";
pub const g_wszStreamBufferRecordingNumberOfFrames: &'static str = "NumberOfFrames";
pub const g_wszStreamBufferRecordingOptimalBitrate: &'static str = "OptimalBitrate";
pub const g_wszStreamBufferRecordingPromotionURL: &'static str = "WM/PromotionURL";
pub const g_wszStreamBufferRecordingProtected: &'static str = "Is_Protected";
pub const g_wszStreamBufferRecordingRating: &'static str = "Rating";
pub const g_wszStreamBufferRecordingSeekable: &'static str = "Seekable";
pub const g_wszStreamBufferRecordingSignature_Name: &'static str = "Signature_Name";
pub const g_wszStreamBufferRecordingSkipBackward: &'static str = "Can_Skip_Backward";
pub const g_wszStreamBufferRecordingSkipForward: &'static str = "Can_Skip_Forward";
pub const g_wszStreamBufferRecordingStridable: &'static str = "Stridable";
pub const g_wszStreamBufferRecordingTitle: &'static str = "Title";
pub const g_wszStreamBufferRecordingToolName: &'static str = "WM/ToolName";
pub const g_wszStreamBufferRecordingToolVersion: &'static str = "WM/ToolVersion";
pub const g_wszStreamBufferRecordingTrack: &'static str = "WM/Track";
pub const g_wszStreamBufferRecordingTrackNumber: &'static str = "WM/TrackNumber";
pub const g_wszStreamBufferRecordingTrusted: &'static str = "Is_Trusted";
pub const g_wszStreamBufferRecordingUse_DRM: &'static str = "Use_DRM";
pub const g_wszStreamBufferRecordingYear: &'static str = "WM/Year";
