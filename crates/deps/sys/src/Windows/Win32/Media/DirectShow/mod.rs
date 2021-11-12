#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Media_DirectShow_Xml")]
pub mod Xml;
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AMGetErrorTextA(hr: ::windows_sys::core::HRESULT, pbuffer: super::super::Foundation::PSTR, maxlen: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AMGetErrorTextW(hr: ::windows_sys::core::HRESULT, pbuffer: super::super::Foundation::PWSTR, maxlen: u32) -> u32;
}
#[repr(C)]
pub struct ADVISE_TYPE(i32);
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
#[repr(C)]
pub struct AMExtendedSeekingCapabilities(i32);
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
#[repr(C)]
pub struct AMMSF_MMS_INIT_FLAGS(i32);
#[repr(C)]
pub struct AMMSF_MS_FLAGS(i32);
#[repr(C)]
pub struct AMMSF_RENDER_FLAGS(i32);
#[repr(C)]
pub struct AMOVERLAYFX(i32);
#[repr(C)]
pub struct AMPROPERTY_PIN(i32);
#[repr(C)]
pub struct AMPlayListEventFlags(i32);
#[repr(C)]
pub struct AMPlayListFlags(i32);
#[repr(C)]
pub struct AMPlayListItemFlags(i32);
#[repr(C)]
pub struct AMTVAudioEventType(i32);
#[repr(C)]
pub struct AMTunerEventType(i32);
#[repr(C)]
pub struct AMTunerModeType(i32);
#[repr(C)]
pub struct AMTunerSignalStrength(i32);
#[repr(C)]
pub struct AMTunerSubChannel(i32);
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
#[repr(C)]
pub struct AMVP_MODE(i32);
#[repr(C)]
pub struct AMVP_SELECT_FORMAT_BY(i32);
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
#[repr(C)]
pub struct AM_ASPECT_RATIO_MODE(i32);
#[repr(C)]
pub struct AM_COLCON(i32);
pub const AM_CONTENTPROPERTY_AUTHOR: u32 = 2u32;
pub const AM_CONTENTPROPERTY_COPYRIGHT: u32 = 4u32;
pub const AM_CONTENTPROPERTY_DESCRIPTION: u32 = 8u32;
pub const AM_CONTENTPROPERTY_TITLE: u32 = 1u32;
#[repr(C)]
pub struct AM_COPY_MACROVISION(i32);
#[repr(C)]
pub struct AM_COPY_MACROVISION_LEVEL(i32);
#[repr(C)]
pub struct AM_DIGITAL_CP(i32);
#[repr(C)]
pub struct AM_DVDCOPYSTATE(i32);
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
#[repr(C)]
pub struct AM_DVD_GRAPH_FLAGS(i32);
pub const AM_DVD_NOT_COPYRIGHTED: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_DVD_RENDERSTATUS(i32);
pub const AM_DVD_SECTOR_NOT_PROTECTED: u32 = 0u32;
pub const AM_DVD_SECTOR_PROTECTED: u32 = 32u32;
pub const AM_DVD_SECTOR_PROTECT_MASK: u32 = 32u32;
#[repr(C)]
pub struct AM_DVD_STREAM_FLAGS(i32);
#[repr(C)]
pub struct AM_DVD_YUV(i32);
#[repr(C)]
pub struct AM_DvdKaraokeData(i32);
#[repr(C)]
pub struct AM_ExactRateChange(i32);
#[repr(C)]
pub struct AM_FILESINK_FLAGS(i32);
#[repr(C)]
pub struct AM_FILTER_FLAGS(i32);
#[repr(C)]
pub struct AM_FRAMESTEP_STEP(i32);
pub const AM_GBF_NODDSURFACELOCK: u32 = 8u32;
pub const AM_GBF_NOTASYNCPOINT: u32 = 2u32;
pub const AM_GBF_NOWAIT: u32 = 4u32;
pub const AM_GBF_PREVFRAMESKIPPED: u32 = 1u32;
pub const AM_GETDECODERCAP_QUERY_EVR_SUPPORT: u32 = 7u32;
pub const AM_GETDECODERCAP_QUERY_VMR9_SUPPORT: u32 = 6u32;
pub const AM_GETDECODERCAP_QUERY_VMR_SUPPORT: u32 = 1u32;
#[repr(C)]
pub struct AM_GRAPH_CONFIG_RECONNECT_FLAGS(i32);
#[repr(C)]
pub struct AM_LINE21_CCLEVEL(i32);
#[repr(C)]
pub struct AM_LINE21_CCSERVICE(i32);
#[repr(C)]
pub struct AM_LINE21_CCSTATE(i32);
#[repr(C)]
pub struct AM_LINE21_CCSTYLE(i32);
#[repr(C)]
pub struct AM_LINE21_DRAWBGMODE(i32);
pub const AM_LOADSTATUS_CLOSED: u32 = 0u32;
pub const AM_LOADSTATUS_CONNECTING: u32 = 4u32;
pub const AM_LOADSTATUS_LOADINGDESCR: u32 = 1u32;
pub const AM_LOADSTATUS_LOADINGMCAST: u32 = 2u32;
pub const AM_LOADSTATUS_LOCATING: u32 = 3u32;
pub const AM_LOADSTATUS_OPEN: u32 = 6u32;
pub const AM_LOADSTATUS_OPENING: u32 = 5u32;
#[repr(C)]
pub struct AM_MEDIAEVENT_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_MEDIA_TYPE(i32);
#[repr(C)]
pub struct AM_MPEG2Level(i32);
#[repr(C)]
pub struct AM_MPEG2Profile(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_MPEGSTREAMTYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AM_MPEGSYSTEMTYPE(i32);
pub const AM_MPEG_AUDIO_DUAL_LEFT: u32 = 1u32;
pub const AM_MPEG_AUDIO_DUAL_MERGE: u32 = 0u32;
pub const AM_MPEG_AUDIO_DUAL_RIGHT: u32 = 2u32;
#[repr(C)]
pub struct AM_PROPERTY_AC3(i32);
#[repr(C)]
pub struct AM_PROPERTY_DVDCOPYPROT(i32);
#[repr(C)]
pub struct AM_PROPERTY_DVDKARAOKE(i32);
#[repr(C)]
pub struct AM_PROPERTY_DVDSUBPIC(i32);
#[repr(C)]
pub struct AM_PROPERTY_DVD_RATE_CHANGE(i32);
#[repr(C)]
pub struct AM_PROPERTY_FRAMESTEP(i32);
#[repr(C)]
pub struct AM_PROPERTY_SPHLI(i32);
#[repr(C)]
pub struct AM_PROPERTY_SPPAL(i32);
#[repr(C)]
pub struct AM_PROPERTY_TS_RATE_CHANGE(i32);
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
#[repr(C)]
pub struct AM_SAMPLE_PROPERTY_FLAGS(i32);
#[repr(C)]
pub struct AM_SEEKING_SEEKING_CAPABILITIES(i32);
#[repr(C)]
pub struct AM_SEEKING_SeekingFlags(i32);
#[repr(C)]
pub struct AM_STREAM_INFO(i32);
#[repr(C)]
pub struct AM_STREAM_INFO_FLAGS(i32);
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
#[repr(C)]
pub struct AM_WST_DRAWBGMODE(i32);
#[repr(C)]
pub struct AM_WST_LEVEL(i32);
#[repr(C)]
pub struct AM_WST_PAGE(i32);
#[repr(C)]
pub struct AM_WST_SERVICE(i32);
#[repr(C)]
pub struct AM_WST_STATE(i32);
#[repr(C)]
pub struct AM_WST_STYLE(i32);
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
#[repr(C)]
pub struct ATSCComponentTypeFlags(i32);
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
#[repr(C)]
pub struct AnalogVideoStandard(i32);
pub const AnalogVideo_NTSC_Mask: u32 = 7u32;
pub const AnalogVideo_PAL_Mask: u32 = 1052656u32;
pub const AnalogVideo_SECAM_Mask: u32 = 1044480u32;
#[repr(C)]
pub struct ApplicationTypeType(i32);
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
#[repr(C)]
pub struct BDA_CHANGE_STATE(i32);
#[repr(C)]
pub struct BDA_CONDITIONALACCESS_MMICLOSEREASON(i32);
#[repr(C)]
pub struct BDA_CONDITIONALACCESS_REQUESTTYPE(i32);
#[repr(C)]
pub struct BDA_CONDITIONALACCESS_SESSION_RESULT(i32);
#[repr(C)]
pub struct BDA_Channel(i32);
#[repr(C)]
pub struct BDA_Channel_Bandwidth(i32);
#[repr(C)]
pub struct BDA_Comp_Flags(i32);
#[repr(C)]
pub struct BDA_DEBUG_DATA(i32);
#[repr(C)]
pub struct BDA_DEBUG_DATA_AVAILABLE(i32);
#[repr(C)]
pub struct BDA_DEBUG_DATA_TYPE_STRING(i32);
#[repr(C)]
pub struct BDA_DISCOVERY_STATE(i32);
#[repr(C)]
pub struct BDA_DISEQC_RESPONSE(i32);
#[repr(C)]
pub struct BDA_DISEQC_SEND(i32);
#[repr(C)]
pub struct BDA_DRM_DRMSTATUS(i32);
#[repr(C)]
pub struct BDA_DVBT2_L1_SIGNALLING_DATA(i32);
#[repr(C)]
pub struct BDA_DigitalSignalStandard(i32);
#[repr(C)]
pub struct BDA_DrmPairingError(i32);
#[repr(C)]
pub struct BDA_ETHERNET_ADDRESS(i32);
#[repr(C)]
pub struct BDA_ETHERNET_ADDRESS_LIST(i32);
#[repr(C)]
pub struct BDA_EVENT_DATA(i32);
#[repr(C)]
pub struct BDA_EVENT_ID(i32);
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
#[repr(C)]
pub struct BDA_Frequency(i32);
#[repr(C)]
pub struct BDA_Frequency_Multiplier(i32);
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
#[repr(C)]
pub struct BDA_LockType(i32);
#[repr(C)]
pub struct BDA_MULTICAST_MODE(i32);
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
#[repr(C)]
pub struct BDA_Range(i32);
#[repr(C)]
pub struct BDA_SCAN_CAPABILTIES(i32);
#[repr(C)]
pub struct BDA_SCAN_START(i32);
#[repr(C)]
pub struct BDA_SCAN_STATE(i32);
#[repr(C)]
pub struct BDA_SIGNAL_STATE(i32);
#[repr(C)]
pub struct BDA_SIGNAL_TIMEOUTS(i32);
#[repr(C)]
pub struct BDA_STRING(i32);
#[repr(C)]
pub struct BDA_SignalType(i32);
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
#[repr(C)]
pub struct BfEnTvRat_Attributes_CAE_TV(i32);
#[repr(C)]
pub struct BfEnTvRat_Attributes_CAF_TV(i32);
#[repr(C)]
pub struct BfEnTvRat_Attributes_MPAA(i32);
#[repr(C)]
pub struct BfEnTvRat_Attributes_US_TV(i32);
#[repr(C)]
pub struct BfEnTvRat_GenericAttributes(i32);
#[repr(C)]
pub struct BinaryConvolutionCodeRate(i32);
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
#[repr(C)]
pub struct COLORKEY_TYPE(i32);
#[repr(C)]
pub struct COMPLETION_STATUS_FLAGS(i32);
pub const COMPONENT_TAG_CAPTION_MAX: u32 = 55u32;
pub const COMPONENT_TAG_CAPTION_MIN: u32 = 48u32;
pub const COMPONENT_TAG_SUPERIMPOSE_MAX: u32 = 63u32;
pub const COMPONENT_TAG_SUPERIMPOSE_MIN: u32 = 56u32;
#[repr(C)]
pub struct COPPEventBlockReason(i32);
#[repr(C)]
pub struct COPP_ACP_Protection_Level(i32);
#[repr(C)]
pub struct COPP_BusType(i32);
#[repr(C)]
pub struct COPP_CGMSA_Protection_Level(i32);
#[repr(C)]
pub struct COPP_ConnectorType(i32);
pub const COPP_DefaultProtectionLevel: u32 = 0u32;
#[repr(C)]
pub struct COPP_HDCP_Protection_Level(i32);
#[repr(C)]
pub struct COPP_ImageAspectRatio_EN300294(i32);
pub const COPP_ImageAspectRatio_EN300294_Mask: u32 = 7u32;
pub const COPP_NoProtectionLevelAvailable: i32 = -1i32;
#[repr(C)]
pub struct COPP_StatusFlags(i32);
#[repr(C)]
pub struct COPP_StatusHDCPFlags(i32);
#[repr(C)]
pub struct COPP_TVProtectionStandard(i32);
#[repr(C)]
pub struct CPEventBitShift(i32);
#[repr(C)]
pub struct CPEvents(i32);
#[repr(C)]
pub struct CPRecordingStatus(i32);
#[repr(C)]
pub struct CRID_LOCATION(i32);
#[repr(C)]
pub struct CROSSBAR_DEFAULT_FLAGS(i32);
#[repr(C)]
pub struct CXDSData(i32);
#[repr(C)]
pub struct CameraControlFlags(i32);
#[repr(C)]
pub struct CameraControlProperty(i32);
#[repr(C)]
pub struct ChannelChangeInfo(i32);
#[repr(C)]
pub struct ChannelChangeSpanningEvent_State(i32);
#[repr(C)]
pub struct ChannelIDTuneRequest(i32);
#[repr(C)]
pub struct ChannelIDTuningSpace(i32);
#[repr(C)]
pub struct ChannelInfo(i32);
#[repr(C)]
pub struct ChannelTuneRequest(i32);
#[repr(C)]
pub struct ChannelType(i32);
#[repr(C)]
pub struct ChannelTypeInfo(i32);
#[repr(C)]
pub struct Component(i32);
#[repr(C)]
pub struct ComponentCategory(i32);
#[repr(C)]
pub struct ComponentStatus(i32);
#[repr(C)]
pub struct ComponentType(i32);
#[repr(C)]
pub struct ComponentTypes(i32);
#[repr(C)]
pub struct Components(i32);
#[repr(C)]
pub struct CompressionCaps(i32);
#[repr(C)]
pub struct CreatePropBagOnRegKey(i32);
#[repr(C)]
pub struct DDSFF_FLAGS(i32);
#[repr(C)]
pub struct DECIMATION_USAGE(i32);
pub const DECODER_CAP_NOTSUPPORTED: u32 = 0u32;
pub const DECODER_CAP_SUPPORTED: u32 = 1u32;
#[repr(C)]
pub struct DESC_LINKAGE_TYPE(i32);
#[repr(C)]
pub struct DIGITAL_CABLE_NETWORK_TYPE(i32);
#[repr(C)]
pub struct DIRECT_TV_SATELLITE_TV_NETWORK_TYPE(i32);
#[repr(C)]
pub struct DISPID_TUNER(i32);
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
#[repr(C)]
pub struct DVBSystemType(i32);
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
#[repr(C)]
pub struct DVB_STRCONV_MODE(i32);
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
#[repr(C)]
pub struct DVDFilterState(i32);
#[repr(C)]
pub struct DVDMenuIDConstants(i32);
#[repr(C)]
pub struct DVDSPExt(i32);
#[repr(C)]
pub struct DVDTextStringType(i32);
#[repr(C)]
pub struct DVD_ATR(i32);
#[repr(C)]
pub struct DVD_AUDIO_APPMODE(i32);
pub const DVD_AUDIO_CAPS_AC3: u32 = 1u32;
pub const DVD_AUDIO_CAPS_DTS: u32 = 8u32;
pub const DVD_AUDIO_CAPS_LPCM: u32 = 4u32;
pub const DVD_AUDIO_CAPS_MPEG2: u32 = 2u32;
pub const DVD_AUDIO_CAPS_SDDS: u32 = 16u32;
#[repr(C)]
pub struct DVD_AUDIO_FORMAT(i32);
#[repr(C)]
pub struct DVD_AUDIO_LANG_EXT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_AudioAttributes(i32);
#[repr(C)]
pub struct DVD_CMD_FLAGS(i32);
#[repr(C)]
pub struct DVD_DECODER_CAPS(i32);
pub const DVD_DEFAULT_AUDIO_STREAM: u32 = 15u32;
#[repr(C)]
pub struct DVD_DISC_SIDE(i32);
#[repr(C)]
pub struct DVD_DOMAIN(i32);
#[repr(C)]
pub struct DVD_ERROR(i32);
#[repr(C)]
pub struct DVD_FRAMERATE(i32);
#[repr(C)]
pub struct DVD_HMSF_TIMECODE(i32);
#[repr(C)]
pub struct DVD_KARAOKE_ASSIGNMENT(i32);
#[repr(C)]
pub struct DVD_KARAOKE_CONTENTS(i32);
#[repr(C)]
pub struct DVD_KARAOKE_DOWNMIX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_KaraokeAttributes(i32);
#[repr(C)]
pub struct DVD_MENU_ID(i32);
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
#[repr(C)]
pub struct DVD_NavCmdType(i32);
#[repr(C)]
pub struct DVD_OPTION_FLAG(i32);
#[repr(C)]
pub struct DVD_PARENTAL_LEVEL(i32);
#[repr(C)]
pub struct DVD_PB_STOPPED(i32);
#[repr(C)]
pub struct DVD_PLAYBACK_LOCATION(i32);
#[repr(C)]
pub struct DVD_PLAYBACK_LOCATION2(i32);
#[repr(C)]
pub struct DVD_PLAY_DIRECTION(i32);
#[repr(C)]
pub struct DVD_PREFERRED_DISPLAY_MODE(i32);
#[repr(C)]
pub struct DVD_REGION(i32);
#[repr(C)]
pub struct DVD_RELATIVE_BUTTON(i32);
pub const DVD_STREAM_DATA_CURRENT: u32 = 2048u32;
pub const DVD_STREAM_DATA_VMGM: u32 = 1024u32;
pub const DVD_STREAM_DATA_VTSM: u32 = 1025u32;
#[repr(C)]
pub struct DVD_SUBPICTURE_CODING(i32);
#[repr(C)]
pub struct DVD_SUBPICTURE_LANG_EXT(i32);
#[repr(C)]
pub struct DVD_SUBPICTURE_TYPE(i32);
#[repr(C)]
pub struct DVD_SubpictureAttributes(i32);
#[repr(C)]
pub struct DVD_TIMECODE(i32);
#[repr(C)]
pub struct DVD_TIMECODE_FLAGS(i32);
#[repr(C)]
pub struct DVD_TITLE_APPMODE(i32);
pub const DVD_TITLE_MENU: u32 = 0u32;
#[repr(C)]
pub struct DVD_TextCharSet(i32);
#[repr(C)]
pub struct DVD_TextStringType(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_TitleAttributes(i32);
#[repr(C)]
pub struct DVD_VIDEO_COMPRESSION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DVD_VideoAttributes(i32);
#[repr(C)]
pub struct DVD_WARNING(i32);
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
#[repr(C)]
pub struct DXVA2_DestinationFlags(i32);
#[repr(C)]
pub struct DXVA2_SampleFlags(i32);
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
#[repr(C)]
pub struct DisplaySizeList(i32);
#[repr(C)]
pub struct DownResEventParam(i32);
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
#[repr(C)]
pub struct EnTag_Mode(i32);
#[repr(C)]
pub struct EnTvRat_CAE_TV(i32);
#[repr(C)]
pub struct EnTvRat_CAF_TV(i32);
#[repr(C)]
pub struct EnTvRat_GenericLevel(i32);
#[repr(C)]
pub struct EnTvRat_MPAA(i32);
#[repr(C)]
pub struct EnTvRat_System(i32);
#[repr(C)]
pub struct EnTvRat_US_TV(i32);
#[repr(C)]
pub struct EncDecEvents(i32);
#[repr(C)]
pub struct EntitlementType(i32);
#[repr(C)]
pub struct EvalRat(i32);
#[repr(C)]
pub struct FECMethod(i32);
#[repr(C)]
pub struct FILTER_INFO(i32);
#[repr(C)]
pub struct FILTER_STATE(i32);
pub const FORMATTYPE_CPFilters_Processed: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1731834735,
    data2: 7519,
    data3: 19138,
    data4: [129, 146, 40, 187, 14, 115, 209, 106],
};
pub const FORMATTYPE_ETDTFilter_Tagged: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3301229777, data2: 73, data3: 20011, data4: [152, 251, 149, 55, 246, 206, 81, 109] };
#[repr(C)]
pub struct FilgraphManager(i32);
#[repr(C)]
pub struct FormatNotSupportedEvents(i32);
pub const GUID_TIME_MUSIC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 91538589, data2: 23300, data3: 19221, data4: [165, 66, 174, 40, 32, 48, 17, 123] };
pub const GUID_TIME_REFERENCE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2477617451,
    data2: 55968,
    data3: 20478,
    data4: [188, 129, 176, 206, 80, 15, 205, 217],
};
pub const GUID_TIME_SAMPLES: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2824420613, data2: 3139, data3: 18820, data4: [154, 99, 151, 175, 158, 2, 196, 192] };
#[repr(C)]
pub struct GuardInterval(i32);
#[cfg(feature = "Win32_Media_Audio")]
#[repr(C)]
pub struct HEAACWAVEFORMAT(i32);
#[cfg(feature = "Win32_Media_Audio")]
#[repr(C)]
pub struct HEAACWAVEINFO(i32);
#[repr(C)]
pub struct HierarchyAlpha(i32);
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
#[repr(C)]
pub struct IFILTERMAPPER_MERIT(i32);
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
#[repr(C)]
pub struct ISDBCAS_REQUEST_ID(i32);
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
#[repr(C)]
pub struct InterleavingMode(i32);
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
#[repr(C)]
pub struct KSEVENT_BDA_EVENT_TYPE(i32);
#[repr(C)]
pub struct KSEVENT_BDA_TUNER(i32);
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
#[repr(C)]
pub struct KSMETHOD_BDA_CAS_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_CHANGE_SYNC(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_DEBUG_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_DEVICE_CONFIGURATION(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_DRM(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_EVENTING_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_GDDS_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_GPNV_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_ISDB_CAS(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_MUX_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_SCAN_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_TS_SELECTOR(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_TUNER_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_USERACTIVITY_SERVICE(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_WMDRM(i32);
#[repr(C)]
pub struct KSMETHOD_BDA_WMDRM_TUNER(i32);
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
#[repr(C)]
pub struct KSPROPERTY_BDA_AUTODEMODULATE(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_CA(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_CA_EVENT(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_DIGITAL_DEMODULATOR(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_DISEQC_COMMAND(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_DISEQC_EVENT(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_ETHERNET_FILTER(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_FREQUENCY_FILTER(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_IPv4_FILTER(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_IPv6_FILTER(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_LNB_INFO(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_NULL_TRANSFORM(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_PIDFILTER(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_PIN_CONTROL(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_PIN_EVENT(i32);
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
#[repr(C)]
pub struct KSPROPERTY_BDA_SIGNAL_STATS(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_TOPOLOGY(i32);
#[repr(C)]
pub struct KSPROPERTY_BDA_VOID_TRANSFORM(i32);
#[repr(C)]
pub struct KSPROPERTY_IDS_BDA_TABLE(i32);
#[repr(C)]
pub struct KSPROPERTY_IPSINK(i32);
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
#[repr(C)]
pub struct LNB_Source(i32);
#[repr(C)]
pub struct LONG_SECTION(i32);
#[repr(C)]
pub struct LanguageComponentType(i32);
#[repr(C)]
pub struct LanguageInfo(i32);
#[repr(C)]
pub struct LicenseEventBlockReason(i32);
#[repr(C)]
pub struct LocationCodeSchemeType(i32);
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
#[repr(C)]
pub struct MEDIA_SAMPLE_CONTENT(i32);
pub const MIN_DIMENSION: u32 = 1u32;
#[repr(C)]
pub struct MMSSF_GET_INFORMATION_FLAGS(i32);
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
#[repr(C)]
pub struct MPEG2StreamType(i32);
#[repr(C)]
pub struct MPEG2TuneRequest(i32);
#[repr(C)]
pub struct MPEG2TuneRequestFactory(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct MPEG2VIDEOINFO(i32);
#[repr(C)]
pub struct MPEG2VIDEOINFO_FLAGS(i32);
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
#[repr(C)]
pub struct MPEGLAYER3WAVEFORMAT_FLAGS(i32);
#[repr(C)]
pub struct MPEG_BCS_DEMUX(i32);
pub const MPEG_CAT_PID: u32 = 1u32;
pub const MPEG_CAT_TID: u32 = 1u32;
#[repr(C)]
pub struct MPEG_CONTEXT(i32);
#[repr(C)]
pub struct MPEG_CONTEXT_TYPE(i32);
#[repr(C)]
pub struct MPEG_CURRENT_NEXT_BIT(i32);
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
#[repr(C)]
pub struct MPEG_REQUEST_TYPE(i32);
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
#[repr(C)]
pub struct MP_CURVE_TYPE(i32);
#[repr(C)]
pub struct MP_ENVELOPE_SEGMENT(i32);
#[repr(C)]
pub struct MP_PARAMINFO(i32);
#[repr(C)]
pub struct MP_TYPE(i32);
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
#[repr(C)]
pub struct MSVidCCService(i32);
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
#[repr(C)]
pub struct MSVidCtlButtonstate(i32);
#[repr(C)]
pub struct MSVidCtlStateList(i32);
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
#[repr(C)]
pub struct MSVidSegmentType(i32);
#[repr(C)]
pub struct MSVidSinkStreams(i32);
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
#[repr(C)]
pub struct MSViddispidList(i32);
#[repr(C)]
pub struct MUX_PID_TYPE(i32);
#[repr(C)]
pub struct MainAVIHeader(i32);
#[repr(C)]
pub struct ModulationType(i32);
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
#[repr(C)]
pub struct OA_BOOL(i32);
pub const OCUR_PAIRING_PROTOCOL_VERSION: u32 = 2u32;
#[repr(C)]
pub struct OUTPUT_STATE(i32);
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
#[repr(C)]
pub struct PIN_DIRECTION(i32);
#[repr(C)]
pub struct PIN_INFO(i32);
#[repr(C)]
pub struct PersistTuneXmlUtility(i32);
#[repr(C)]
pub struct PhysicalConnectorType(i32);
#[repr(C)]
pub struct Pilot(i32);
#[repr(C)]
pub struct Polarisation(i32);
#[repr(C)]
pub struct PositionModeList(i32);
#[repr(C)]
pub struct ProgramElement(i32);
#[repr(C)]
pub struct ProtType(i32);
#[repr(C)]
pub struct Quality(i32);
#[repr(C)]
pub struct QualityMessageType(i32);
#[repr(C)]
pub struct RATING_ATTRIBUTE(i32);
#[repr(C)]
pub struct RATING_INFO(i32);
#[repr(C)]
pub struct RATING_SYSTEM(i32);
#[repr(C)]
pub struct RECORDING_TYPE(i32);
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
#[repr(C)]
pub struct REG_PINFLAG(i32);
pub const REQUIRED_PARENTAL_CONTROL_TIME_RANGE: u32 = 2u32;
#[repr(C)]
pub struct RIFFCHUNK(i32);
#[repr(C)]
pub struct RIFFLIST(i32);
#[repr(C)]
pub struct RecordingType(i32);
#[repr(C)]
pub struct RevokedComponent(i32);
#[repr(C)]
pub struct RollOff(i32);
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
#[repr(C)]
pub struct SNDDEV_ERR(i32);
pub const SPECIFYPAGES_STATISTICS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1279490962, data2: 28318, data3: 4561, data4: [167, 4, 0, 96, 151, 196, 228, 118] };
#[repr(C)]
pub struct SSUPDATE_TYPE(i32);
pub const STDINDEXSIZE: u32 = 16384u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STREAMBUFFER_ATTRIBUTE(i32);
#[repr(C)]
pub struct STREAMBUFFER_ATTR_DATATYPE(i32);
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
#[repr(C)]
pub struct STREAMIF_CONSTANTS(i32);
#[repr(C)]
pub struct STREAM_ID_MAP(i32);
#[repr(C)]
pub struct STREAM_STATE(i32);
#[repr(C)]
pub struct STREAM_TYPE(i32);
pub const SUBSTREAM_FILTER_VAL_NONE: u32 = 268435456u32;
#[repr(C)]
pub struct ScanModulationTypes(i32);
#[repr(C)]
pub struct SectionList(i32);
#[repr(C)]
pub struct SegDispidList(i32);
#[repr(C)]
pub struct SegEventidList(i32);
#[repr(C)]
pub struct SignalAndServiceStatusSpanningEvent_State(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SmartCardApplication(i32);
#[repr(C)]
pub struct SmartCardAssociationType(i32);
#[repr(C)]
pub struct SmartCardStatusType(i32);
#[repr(C)]
pub struct SourceSizeList(i32);
#[repr(C)]
pub struct SpanningEventDescriptor(i32);
#[repr(C)]
pub struct SpanningEventEmmMessage(i32);
#[repr(C)]
pub struct SpectralInversion(i32);
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
#[repr(C)]
pub struct TVAudioMode(i32);
#[repr(C)]
pub struct TransmissionMode(i32);
#[repr(C)]
pub struct TuneRequest(i32);
#[repr(C)]
pub struct TunerInputType(i32);
#[repr(C)]
pub struct TunerMarshaler(i32);
#[repr(C)]
pub struct TuningSpace(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UDCR_TAG(i32);
#[repr(C)]
pub struct UICloseReasonType(i32);
#[repr(C)]
pub struct VALID_UOP_FLAG(i32);
#[repr(C)]
pub struct VA_COLOR_PRIMARIES(i32);
#[repr(C)]
pub struct VA_MATRIX_COEFFICIENTS(i32);
#[repr(C)]
pub struct VA_OPTIONAL_VIDEO_PROPERTIES(i32);
#[repr(C)]
pub struct VA_TRANSFER_CHARACTERISTICS(i32);
#[repr(C)]
pub struct VA_VIDEO_FORMAT(i32);
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
#[repr(C)]
pub struct VIDEOENCODER_BITRATE_MODE(i32);
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
#[repr(C)]
pub struct VMR9AlphaBitmapFlags(i32);
#[repr(C)]
pub struct VMR9AspectRatioMode(i32);
#[repr(C)]
pub struct VMR9DeinterlaceCaps(i32);
#[repr(C)]
pub struct VMR9DeinterlacePrefs(i32);
#[repr(C)]
pub struct VMR9DeinterlaceTech(i32);
#[repr(C)]
pub struct VMR9Frequency(i32);
#[repr(C)]
pub struct VMR9MixerPrefs(i32);
#[repr(C)]
pub struct VMR9Mode(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VMR9MonitorInfo(i32);
#[repr(C)]
pub struct VMR9NormalizedRect(i32);
#[repr(C)]
pub struct VMR9PresentationFlags(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[repr(C)]
pub struct VMR9PresentationInfo(i32);
#[repr(C)]
pub struct VMR9ProcAmpControl(i32);
#[repr(C)]
pub struct VMR9ProcAmpControlFlags(i32);
#[repr(C)]
pub struct VMR9ProcAmpControlRange(i32);
#[repr(C)]
pub struct VMR9RenderPrefs(i32);
#[repr(C)]
pub struct VMR9SurfaceAllocationFlags(i32);
#[repr(C)]
pub struct VMR9VideoDesc(i32);
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[repr(C)]
pub struct VMR9VideoStreamInfo(i32);
#[repr(C)]
pub struct VMR9_SampleFormat(i32);
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
#[repr(C)]
pub struct VMRDeinterlacePrefs(i32);
#[repr(C)]
pub struct VMRDeinterlaceTech(i32);
#[repr(C)]
pub struct VMRFrequency(i32);
#[repr(C)]
pub struct VMRGUID(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct VMRMONITORINFO(i32);
#[repr(C)]
pub struct VMRMixerPrefs(i32);
#[repr(C)]
pub struct VMRMode(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[repr(C)]
pub struct VMRPRESENTATIONINFO(i32);
#[repr(C)]
pub struct VMRPresentationFlags(i32);
#[repr(C)]
pub struct VMRRenderPrefs(i32);
#[repr(C)]
pub struct VMRSurfaceAllocationFlags(i32);
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[repr(C)]
pub struct VMRVIDEOSTREAMINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VMRVideoDesc(i32);
#[repr(C)]
pub struct VMR_ASPECT_RATIO_MODE(i32);
pub const VMR_NOTSUPPORTED: u32 = 0u32;
pub const VMR_RENDER_DEVICE_OVERLAY: u32 = 1u32;
pub const VMR_RENDER_DEVICE_SYSMEM: u32 = 4u32;
pub const VMR_RENDER_DEVICE_VIDMEM: u32 = 2u32;
pub const VMR_SUPPORTED: u32 = 1u32;
#[repr(C)]
pub struct VfwCaptureDialogs(i32);
#[repr(C)]
pub struct VfwCompressDialogs(i32);
#[repr(C)]
pub struct VideoControlFlags(i32);
#[repr(C)]
pub struct VideoCopyProtectionType(i32);
#[repr(C)]
pub struct VideoProcAmpFlags(i32);
#[repr(C)]
pub struct VideoProcAmpProperty(i32);
#[repr(C)]
pub struct WMDRMProtectionInfo(i32);
#[repr(C)]
pub struct XDSCodec(i32);
#[repr(C)]
pub struct XDSToRat(i32);
#[repr(C)]
pub struct _AMRESCTL_RESERVEFLAGS(i32);
#[repr(C)]
pub struct _AMSTREAMSELECTENABLEFLAGS(i32);
#[repr(C)]
pub struct _AMSTREAMSELECTINFOFLAGS(i32);
#[repr(C)]
pub struct _AM_AUDIO_RENDERER_STAT_PARAM(i32);
#[repr(C)]
pub struct _AM_FILTER_MISC_FLAGS(i32);
#[repr(C)]
pub struct _AM_INTF_SEARCH_FLAGS(i32);
#[repr(C)]
pub struct _AM_OVERLAY_NOTIFY_FLAGS(i32);
#[repr(C)]
pub struct _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS(i32);
#[repr(C)]
pub struct _AM_PUSHSOURCE_FLAGS(i32);
#[repr(C)]
pub struct _AM_RENSDEREXFLAGS(i32);
#[repr(C)]
pub struct _DVDECODERRESOLUTION(i32);
#[repr(C)]
pub struct _DVENCODERFORMAT(i32);
#[repr(C)]
pub struct _DVENCODERRESOLUTION(i32);
#[repr(C)]
pub struct _DVENCODERVIDEOFORMAT(i32);
#[repr(C)]
pub struct _DVRESOLUTION(i32);
#[repr(transparent)]
pub struct _IMSVidCtlEvents(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct _REM_FILTER_FLAGS(i32);
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
