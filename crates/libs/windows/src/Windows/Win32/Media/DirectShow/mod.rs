#[cfg(feature = "Win32_Media_DirectShow_Tv")]
pub mod Tv;
#[cfg(feature = "Win32_Media_DirectShow_Xml")]
pub mod Xml;
pub const ADVISE_CLIPPING: ADVISE_TYPE = 1i32;
pub const ADVISE_COLORKEY: ADVISE_TYPE = 4i32;
pub const ADVISE_DISPLAY_CHANGE: ADVISE_TYPE = 16i32;
pub const ADVISE_NONE: ADVISE_TYPE = 0i32;
pub const ADVISE_PALETTE: ADVISE_TYPE = 2i32;
pub const ADVISE_POSITION: ADVISE_TYPE = 8i32;
pub const AMAP_3D_TARGET: VMRSurfaceAllocationFlags = 2i32;
pub const AMAP_ALLOW_SYSMEM: VMRSurfaceAllocationFlags = 4i32;
pub const AMAP_DIRECTED_FLIP: VMRSurfaceAllocationFlags = 16i32;
pub const AMAP_DXVA_TARGET: VMRSurfaceAllocationFlags = 32i32;
pub const AMAP_FORCE_SYSMEM: VMRSurfaceAllocationFlags = 8i32;
pub const AMAP_PIXELFORMAT_VALID: VMRSurfaceAllocationFlags = 1i32;
pub const AMCONTROL_COLORINFO_PRESENT: u32 = 128u32;
pub const AMCONTROL_PAD_TO_16x9: u32 = 4u32;
pub const AMCONTROL_PAD_TO_4x3: u32 = 2u32;
pub const AMCONTROL_USED: u32 = 1u32;
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
pub const AMF_AUTOMATICGAIN: f64 = -1f64;
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
pub const AMMSF_ADDDEFAULTRENDERER: AMMSF_MS_FLAGS = 1i32;
pub const AMMSF_CREATEPEER: AMMSF_MS_FLAGS = 2i32;
pub const AMMSF_NOCLOCK: AMMSF_RENDER_FLAGS = 4i32;
pub const AMMSF_NOGRAPHTHREAD: AMMSF_MMS_INIT_FLAGS = 1i32;
pub const AMMSF_NORENDER: AMMSF_RENDER_FLAGS = 2i32;
pub const AMMSF_NOSTALL: AMMSF_MS_FLAGS = 8i32;
pub const AMMSF_RENDERALLSTREAMS: AMMSF_RENDER_FLAGS = 1i32;
pub const AMMSF_RENDERTOEXISTING: AMMSF_RENDER_FLAGS = 0i32;
pub const AMMSF_RENDERTYPEMASK: AMMSF_RENDER_FLAGS = 3i32;
pub const AMMSF_RUN: AMMSF_RENDER_FLAGS = 8i32;
pub const AMMSF_STOPIFNOSAMPLES: AMMSF_MS_FLAGS = 4i32;
pub const AMOVERFX_DEINTERLACE: AMOVERLAYFX = 8i32;
pub const AMOVERFX_MIRRORLEFTRIGHT: AMOVERLAYFX = 2i32;
pub const AMOVERFX_MIRRORUPDOWN: AMOVERLAYFX = 4i32;
pub const AMOVERFX_NOFX: AMOVERLAYFX = 0i32;
pub const AMPLAYLISTEVENT_BREAK: AMPlayListEventFlags = 1i32;
pub const AMPLAYLISTEVENT_MASK: AMPlayListEventFlags = 15i32;
pub const AMPLAYLISTEVENT_NEXT: AMPlayListEventFlags = 2i32;
pub const AMPLAYLISTEVENT_REFRESH: AMPlayListEventFlags = 16i32;
pub const AMPLAYLISTEVENT_RESUME: AMPlayListEventFlags = 0i32;
pub const AMPLAYLISTITEM_CANBIND: AMPlayListItemFlags = 2i32;
pub const AMPLAYLISTITEM_CANSKIP: AMPlayListItemFlags = 1i32;
pub const AMPLAYLIST_FORCEBANNER: AMPlayListFlags = 2i32;
pub const AMPLAYLIST_STARTINSCANMODE: AMPlayListFlags = 1i32;
pub const AMPROPERTY_PIN_CATEGORY: AMPROPERTY_PIN = 0i32;
pub const AMPROPERTY_PIN_MEDIUM: AMPROPERTY_PIN = 1i32;
pub const AMRESCTL_RESERVEFLAGS_RESERVE: _AMRESCTL_RESERVEFLAGS = 0i32;
pub const AMRESCTL_RESERVEFLAGS_UNRESERVE: _AMRESCTL_RESERVEFLAGS = 1i32;
pub const AMSTREAMSELECTENABLE_ENABLE: _AMSTREAMSELECTENABLEFLAGS = 1i32;
pub const AMSTREAMSELECTENABLE_ENABLEALL: _AMSTREAMSELECTENABLEFLAGS = 2i32;
pub const AMSTREAMSELECTINFO_ENABLED: _AMSTREAMSELECTINFOFLAGS = 1i32;
pub const AMSTREAMSELECTINFO_EXCLUSIVE: _AMSTREAMSELECTINFOFLAGS = 2i32;
pub const AMTUNER_EVENT_CHANGED: AMTunerEventType = 1i32;
pub const AMTUNER_HASNOSIGNALSTRENGTH: AMTunerSignalStrength = -1i32;
pub const AMTUNER_MODE_AM_RADIO: AMTunerModeType = 4i32;
pub const AMTUNER_MODE_DEFAULT: AMTunerModeType = 0i32;
pub const AMTUNER_MODE_DSS: AMTunerModeType = 8i32;
pub const AMTUNER_MODE_FM_RADIO: AMTunerModeType = 2i32;
pub const AMTUNER_MODE_TV: AMTunerModeType = 1i32;
pub const AMTUNER_NOSIGNAL: AMTunerSignalStrength = 0i32;
pub const AMTUNER_SIGNALPRESENT: AMTunerSignalStrength = 1i32;
pub const AMTUNER_SUBCHAN_DEFAULT: AMTunerSubChannel = -1i32;
pub const AMTUNER_SUBCHAN_NO_TUNE: AMTunerSubChannel = -2i32;
pub const AMTVAUDIO_EVENT_CHANGED: AMTVAudioEventType = 1i32;
pub const AMTVAUDIO_MODE_LANG_A: TVAudioMode = 16i32;
pub const AMTVAUDIO_MODE_LANG_B: TVAudioMode = 32i32;
pub const AMTVAUDIO_MODE_LANG_C: TVAudioMode = 64i32;
pub const AMTVAUDIO_MODE_MONO: TVAudioMode = 1i32;
pub const AMTVAUDIO_MODE_STEREO: TVAudioMode = 2i32;
pub const AMTVAUDIO_PRESET_LANG_A: TVAudioMode = 4096i32;
pub const AMTVAUDIO_PRESET_LANG_B: TVAudioMode = 8192i32;
pub const AMTVAUDIO_PRESET_LANG_C: TVAudioMode = 16384i32;
pub const AMTVAUDIO_PRESET_STEREO: TVAudioMode = 512i32;
pub const AMVA_QUERYRENDERSTATUSF_READ: u32 = 1u32;
pub const AMVA_TYPEINDEX_OUTPUTFRAME: u32 = 4294967295u32;
pub const AMVP_BEST_BANDWIDTH: AMVP_SELECT_FORMAT_BY = 1i32;
pub const AMVP_DO_NOT_CARE: AMVP_SELECT_FORMAT_BY = 0i32;
pub const AMVP_INPUT_SAME_AS_OUTPUT: AMVP_SELECT_FORMAT_BY = 2i32;
pub const AMVP_MODE_BOBINTERLEAVED: AMVP_MODE = 1i32;
pub const AMVP_MODE_BOBNONINTERLEAVED: AMVP_MODE = 2i32;
pub const AMVP_MODE_SKIPEVEN: AMVP_MODE = 3i32;
pub const AMVP_MODE_SKIPODD: AMVP_MODE = 4i32;
pub const AMVP_MODE_WEAVE: AMVP_MODE = 0i32;
pub const AM_AC3_ALTERNATE_AUDIO_1: u32 = 1u32;
pub const AM_AC3_ALTERNATE_AUDIO_2: u32 = 2u32;
pub const AM_AC3_ALTERNATE_AUDIO_BOTH: u32 = 3u32;
pub const AM_AC3_SERVICE_COMMENTARY: u32 = 5u32;
pub const AM_AC3_SERVICE_DIALOG_ONLY: u32 = 4u32;
pub const AM_AC3_SERVICE_EMERGENCY_FLASH: u32 = 6u32;
pub const AM_AC3_SERVICE_HEARING_IMPAIRED: u32 = 3u32;
pub const AM_AC3_SERVICE_MAIN_AUDIO: u32 = 0u32;
pub const AM_AC3_SERVICE_NO_DIALOG: u32 = 1u32;
pub const AM_AC3_SERVICE_VISUALLY_IMPAIRED: u32 = 2u32;
pub const AM_AC3_SERVICE_VOICE_OVER: u32 = 7u32;
pub const AM_ARMODE_CROP: AM_ASPECT_RATIO_MODE = 2i32;
pub const AM_ARMODE_LETTER_BOX: AM_ASPECT_RATIO_MODE = 1i32;
pub const AM_ARMODE_STRETCHED: AM_ASPECT_RATIO_MODE = 0i32;
pub const AM_ARMODE_STRETCHED_AS_PRIMARY: AM_ASPECT_RATIO_MODE = 3i32;
pub const AM_AUDREND_STAT_PARAM_BREAK_COUNT: _AM_AUDIO_RENDERER_STAT_PARAM = 1i32;
pub const AM_AUDREND_STAT_PARAM_BUFFERFULLNESS: _AM_AUDIO_RENDERER_STAT_PARAM = 11i32;
pub const AM_AUDREND_STAT_PARAM_DISCONTINUITIES: _AM_AUDIO_RENDERER_STAT_PARAM = 5i32;
pub const AM_AUDREND_STAT_PARAM_JITTER: _AM_AUDIO_RENDERER_STAT_PARAM = 12i32;
pub const AM_AUDREND_STAT_PARAM_LAST_BUFFER_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = 4i32;
pub const AM_AUDREND_STAT_PARAM_SILENCE_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = 3i32;
pub const AM_AUDREND_STAT_PARAM_SLAVE_ACCUMERROR: _AM_AUDIO_RENDERER_STAT_PARAM = 10i32;
pub const AM_AUDREND_STAT_PARAM_SLAVE_DROPWRITE_DUR: _AM_AUDIO_RENDERER_STAT_PARAM = 7i32;
pub const AM_AUDREND_STAT_PARAM_SLAVE_HIGHLOWERROR: _AM_AUDIO_RENDERER_STAT_PARAM = 8i32;
pub const AM_AUDREND_STAT_PARAM_SLAVE_LASTHIGHLOWERROR: _AM_AUDIO_RENDERER_STAT_PARAM = 9i32;
pub const AM_AUDREND_STAT_PARAM_SLAVE_MODE: _AM_AUDIO_RENDERER_STAT_PARAM = 2i32;
pub const AM_AUDREND_STAT_PARAM_SLAVE_RATE: _AM_AUDIO_RENDERER_STAT_PARAM = 6i32;
pub const AM_CONTENTPROPERTY_AUTHOR: u32 = 2u32;
pub const AM_CONTENTPROPERTY_COPYRIGHT: u32 = 4u32;
pub const AM_CONTENTPROPERTY_DESCRIPTION: u32 = 8u32;
pub const AM_CONTENTPROPERTY_TITLE: u32 = 1u32;
pub const AM_DIGITAL_CP_DVD_COMPLIANT: AM_DIGITAL_CP = 2i32;
pub const AM_DIGITAL_CP_OFF: AM_DIGITAL_CP = 0i32;
pub const AM_DIGITAL_CP_ON: AM_DIGITAL_CP = 1i32;
pub const AM_DVDCOPYSTATE_AUTHENTICATION_NOT_REQUIRED: AM_DVDCOPYSTATE = 2i32;
pub const AM_DVDCOPYSTATE_AUTHENTICATION_REQUIRED: AM_DVDCOPYSTATE = 3i32;
pub const AM_DVDCOPYSTATE_DONE: AM_DVDCOPYSTATE = 4i32;
pub const AM_DVDCOPYSTATE_INITIALIZE: AM_DVDCOPYSTATE = 0i32;
pub const AM_DVDCOPYSTATE_INITIALIZE_TITLE: AM_DVDCOPYSTATE = 1i32;
pub const AM_DVD_ADAPT_GRAPH: AM_DVD_GRAPH_FLAGS = 16384i32;
pub const AM_DVD_CGMS_COPY_ONCE: u32 = 16u32;
pub const AM_DVD_CGMS_COPY_PERMITTED: u32 = 0u32;
pub const AM_DVD_CGMS_COPY_PROTECT_MASK: u32 = 24u32;
pub const AM_DVD_CGMS_NO_COPY: u32 = 24u32;
pub const AM_DVD_CGMS_RESERVED_MASK: u32 = 120u32;
pub const AM_DVD_COPYRIGHTED: u32 = 64u32;
pub const AM_DVD_COPYRIGHT_MASK: u32 = 64u32;
pub const AM_DVD_DO_NOT_CLEAR: AM_DVD_GRAPH_FLAGS = 512i32;
pub const AM_DVD_EVR_ONLY: AM_DVD_GRAPH_FLAGS = 4096i32;
pub const AM_DVD_EVR_QOS: AM_DVD_GRAPH_FLAGS = 8192i32;
pub const AM_DVD_HWDEC_ONLY: AM_DVD_GRAPH_FLAGS = 2i32;
pub const AM_DVD_HWDEC_PREFER: AM_DVD_GRAPH_FLAGS = 1i32;
pub const AM_DVD_MASK: AM_DVD_GRAPH_FLAGS = 65535i32;
pub const AM_DVD_NOT_COPYRIGHTED: u32 = 0u32;
pub const AM_DVD_NOVPE: AM_DVD_GRAPH_FLAGS = 256i32;
pub const AM_DVD_SECTOR_NOT_PROTECTED: u32 = 0u32;
pub const AM_DVD_SECTOR_PROTECTED: u32 = 32u32;
pub const AM_DVD_SECTOR_PROTECT_MASK: u32 = 32u32;
pub const AM_DVD_STREAM_AUDIO: AM_DVD_STREAM_FLAGS = 2i32;
pub const AM_DVD_STREAM_SUBPIC: AM_DVD_STREAM_FLAGS = 4i32;
pub const AM_DVD_STREAM_VIDEO: AM_DVD_STREAM_FLAGS = 1i32;
pub const AM_DVD_SWDEC_ONLY: AM_DVD_GRAPH_FLAGS = 8i32;
pub const AM_DVD_SWDEC_PREFER: AM_DVD_GRAPH_FLAGS = 4i32;
pub const AM_DVD_VMR9_ONLY: AM_DVD_GRAPH_FLAGS = 2048i32;
pub const AM_EXSEEK_BUFFERING: AMExtendedSeekingCapabilities = 32i32;
pub const AM_EXSEEK_CANSCAN: AMExtendedSeekingCapabilities = 2i32;
pub const AM_EXSEEK_CANSEEK: AMExtendedSeekingCapabilities = 1i32;
pub const AM_EXSEEK_MARKERSEEK: AMExtendedSeekingCapabilities = 4i32;
pub const AM_EXSEEK_NOSTANDARDREPAINT: AMExtendedSeekingCapabilities = 16i32;
pub const AM_EXSEEK_SCANWITHOUTCLOCK: AMExtendedSeekingCapabilities = 8i32;
pub const AM_EXSEEK_SENDS_VIDEOFRAMEREADY: AMExtendedSeekingCapabilities = 64i32;
pub const AM_FILE_OVERWRITE: AM_FILESINK_FLAGS = 1i32;
pub const AM_FILTER_FLAGS_REMOVABLE: AM_FILTER_FLAGS = 1i32;
pub const AM_FILTER_MISC_FLAGS_IS_RENDERER: _AM_FILTER_MISC_FLAGS = 1i32;
pub const AM_FILTER_MISC_FLAGS_IS_SOURCE: _AM_FILTER_MISC_FLAGS = 2i32;
pub const AM_GBF_NODDSURFACELOCK: u32 = 8u32;
pub const AM_GBF_NOTASYNCPOINT: u32 = 2u32;
pub const AM_GBF_NOWAIT: u32 = 4u32;
pub const AM_GBF_PREVFRAMESKIPPED: u32 = 1u32;
pub const AM_GETDECODERCAP_QUERY_EVR_SUPPORT: u32 = 7u32;
pub const AM_GETDECODERCAP_QUERY_VMR9_SUPPORT: u32 = 6u32;
pub const AM_GETDECODERCAP_QUERY_VMR_SUPPORT: u32 = 1u32;
pub const AM_GRAPH_CONFIG_RECONNECT_CACHE_REMOVED_FILTERS: AM_GRAPH_CONFIG_RECONNECT_FLAGS = 2i32;
pub const AM_GRAPH_CONFIG_RECONNECT_DIRECTCONNECT: AM_GRAPH_CONFIG_RECONNECT_FLAGS = 1i32;
pub const AM_GRAPH_CONFIG_RECONNECT_USE_ONLY_CACHED_FILTERS: AM_GRAPH_CONFIG_RECONNECT_FLAGS = 4i32;
pub const AM_INTERFACESETID_Standard: windows_core::GUID = windows_core::GUID::from_u128(0x1a8766a0_62ce_11cf_a5d6_28db04c10000);
pub const AM_INTF_SEARCH_FILTER: _AM_INTF_SEARCH_FLAGS = 4i32;
pub const AM_INTF_SEARCH_INPUT_PIN: _AM_INTF_SEARCH_FLAGS = 1i32;
pub const AM_INTF_SEARCH_OUTPUT_PIN: _AM_INTF_SEARCH_FLAGS = 2i32;
pub const AM_KSCATEGORY_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0x6994ad04_93ef_11d0_a3cc_00a0c9223196);
pub const AM_KSCATEGORY_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x65e8773d_8f56_11d0_a3b9_00a0c9223196);
pub const AM_KSCATEGORY_CROSSBAR: windows_core::GUID = windows_core::GUID::from_u128(0xa799a801_a46d_11d0_a18c_00a02401dcd4);
pub const AM_KSCATEGORY_DATACOMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0x1e84c900_7e70_11d0_a5d6_28db04c10000);
pub const AM_KSCATEGORY_RENDER: windows_core::GUID = windows_core::GUID::from_u128(0x65e8773e_8f56_11d0_a3b9_00a0c9223196);
pub const AM_KSCATEGORY_SPLITTER: windows_core::GUID = windows_core::GUID::from_u128(0x0a4252a0_7e70_11d0_a5d6_28db04c10000);
pub const AM_KSCATEGORY_TVAUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xa799a802_a46d_11d0_a18c_00a02401dcd4);
pub const AM_KSCATEGORY_TVTUNER: windows_core::GUID = windows_core::GUID::from_u128(0xa799a800_a46d_11d0_a18c_00a02401dcd4);
pub const AM_KSCATEGORY_VBICODEC: windows_core::GUID = windows_core::GUID::from_u128(0x07dad660_22f1_11d1_a9f4_00c04fbbde8f);
pub const AM_KSCATEGORY_VBICODEC_MI: windows_core::GUID = windows_core::GUID::from_u128(0x9c24a977_0951_451a_8006_0e49bd28cd5f);
pub const AM_KSCATEGORY_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0x6994ad05_93ef_11d0_a3cc_00a0c9223196);
pub const AM_KSPROPSETID_AC3: windows_core::GUID = windows_core::GUID::from_u128(0xbfabe720_6e1f_11d0_bcf2_444553540000);
pub const AM_KSPROPSETID_CopyProt: windows_core::GUID = windows_core::GUID::from_u128(0x0e8a0a40_6aef_11d0_9ed0_00a024ca19b3);
pub const AM_KSPROPSETID_DVD_RateChange: windows_core::GUID = windows_core::GUID::from_u128(0x3577eb09_9582_477f_b29c_b0c452a4ff9a);
pub const AM_KSPROPSETID_DvdKaraoke: windows_core::GUID = windows_core::GUID::from_u128(0xae4720ae_aa71_42d8_b82a_fffdf58b76fd);
pub const AM_KSPROPSETID_DvdSubPic: windows_core::GUID = windows_core::GUID::from_u128(0xac390460_43af_11d0_bd6a_003505c103a9);
pub const AM_KSPROPSETID_FrameStep: windows_core::GUID = windows_core::GUID::from_u128(0xc830acbd_ab07_492f_8852_45b6987c2979);
pub const AM_KSPROPSETID_MPEG4_MediaType_Attributes: windows_core::GUID = windows_core::GUID::from_u128(0xff6c4bfa_07a9_4c7b_a237_672f9d68065f);
pub const AM_KSPROPSETID_TSRateChange: windows_core::GUID = windows_core::GUID::from_u128(0xa503c5c0_1d1d_11d1_ad80_444553540000);
pub const AM_L21_CCLEVEL_TC2: AM_LINE21_CCLEVEL = 0i32;
pub const AM_L21_CCSERVICE_Caption1: AM_LINE21_CCSERVICE = 1i32;
pub const AM_L21_CCSERVICE_Caption2: AM_LINE21_CCSERVICE = 2i32;
pub const AM_L21_CCSERVICE_DefChannel: AM_LINE21_CCSERVICE = 10i32;
pub const AM_L21_CCSERVICE_Invalid: AM_LINE21_CCSERVICE = 11i32;
pub const AM_L21_CCSERVICE_None: AM_LINE21_CCSERVICE = 0i32;
pub const AM_L21_CCSERVICE_Text1: AM_LINE21_CCSERVICE = 3i32;
pub const AM_L21_CCSERVICE_Text2: AM_LINE21_CCSERVICE = 4i32;
pub const AM_L21_CCSERVICE_XDS: AM_LINE21_CCSERVICE = 5i32;
pub const AM_L21_CCSTATE_Off: AM_LINE21_CCSTATE = 0i32;
pub const AM_L21_CCSTATE_On: AM_LINE21_CCSTATE = 1i32;
pub const AM_L21_CCSTYLE_None: AM_LINE21_CCSTYLE = 0i32;
pub const AM_L21_CCSTYLE_PaintOn: AM_LINE21_CCSTYLE = 2i32;
pub const AM_L21_CCSTYLE_PopOn: AM_LINE21_CCSTYLE = 1i32;
pub const AM_L21_CCSTYLE_RollUp: AM_LINE21_CCSTYLE = 3i32;
pub const AM_L21_DRAWBGMODE_Opaque: AM_LINE21_DRAWBGMODE = 0i32;
pub const AM_L21_DRAWBGMODE_Transparent: AM_LINE21_DRAWBGMODE = 1i32;
pub const AM_LOADSTATUS_CLOSED: u32 = 0u32;
pub const AM_LOADSTATUS_CONNECTING: u32 = 4u32;
pub const AM_LOADSTATUS_LOADINGDESCR: u32 = 1u32;
pub const AM_LOADSTATUS_LOADINGMCAST: u32 = 2u32;
pub const AM_LOADSTATUS_LOCATING: u32 = 3u32;
pub const AM_LOADSTATUS_OPEN: u32 = 6u32;
pub const AM_LOADSTATUS_OPENING: u32 = 5u32;
pub const AM_MACROVISION_DISABLED: AM_COPY_MACROVISION_LEVEL = 0i32;
pub const AM_MACROVISION_LEVEL1: AM_COPY_MACROVISION_LEVEL = 1i32;
pub const AM_MACROVISION_LEVEL2: AM_COPY_MACROVISION_LEVEL = 2i32;
pub const AM_MACROVISION_LEVEL3: AM_COPY_MACROVISION_LEVEL = 3i32;
pub const AM_MEDIAEVENT_NONOTIFY: AM_MEDIAEVENT_FLAGS = 1i32;
pub const AM_MPEG2Level_High: AM_MPEG2Level = 4i32;
pub const AM_MPEG2Level_High1440: AM_MPEG2Level = 3i32;
pub const AM_MPEG2Level_Low: AM_MPEG2Level = 1i32;
pub const AM_MPEG2Level_Main: AM_MPEG2Level = 2i32;
pub const AM_MPEG2Profile_High: AM_MPEG2Profile = 5i32;
pub const AM_MPEG2Profile_Main: AM_MPEG2Profile = 2i32;
pub const AM_MPEG2Profile_SNRScalable: AM_MPEG2Profile = 3i32;
pub const AM_MPEG2Profile_Simple: AM_MPEG2Profile = 1i32;
pub const AM_MPEG2Profile_SpatiallyScalable: AM_MPEG2Profile = 4i32;
pub const AM_MPEG_AUDIO_DUAL_LEFT: u32 = 1u32;
pub const AM_MPEG_AUDIO_DUAL_MERGE: u32 = 0u32;
pub const AM_MPEG_AUDIO_DUAL_RIGHT: u32 = 2u32;
pub const AM_OVERLAY_NOTIFY_DEST_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = 4i32;
pub const AM_OVERLAY_NOTIFY_SOURCE_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = 2i32;
pub const AM_OVERLAY_NOTIFY_VISIBLE_CHANGE: _AM_OVERLAY_NOTIFY_FLAGS = 1i32;
pub const AM_PIN_FLOW_CONTROL_BLOCK: _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS = 1i32;
pub const AM_PROPERTY_AC3_ALTERNATE_AUDIO: AM_PROPERTY_AC3 = 2i32;
pub const AM_PROPERTY_AC3_BIT_STREAM_MODE: AM_PROPERTY_AC3 = 4i32;
pub const AM_PROPERTY_AC3_DIALOGUE_LEVEL: AM_PROPERTY_AC3 = 5i32;
pub const AM_PROPERTY_AC3_DOWNMIX: AM_PROPERTY_AC3 = 3i32;
pub const AM_PROPERTY_AC3_ERROR_CONCEALMENT: AM_PROPERTY_AC3 = 1i32;
pub const AM_PROPERTY_AC3_LANGUAGE_CODE: AM_PROPERTY_AC3 = 6i32;
pub const AM_PROPERTY_AC3_ROOM_TYPE: AM_PROPERTY_AC3 = 7i32;
pub const AM_PROPERTY_COPY_ANALOG_COMPONENT: AM_PROPERTY_DVDCOPYPROT = 8i32;
pub const AM_PROPERTY_COPY_DIGITAL_CP: AM_PROPERTY_DVDCOPYPROT = 9i32;
pub const AM_PROPERTY_COPY_DVD_SRM: AM_PROPERTY_DVDCOPYPROT = 10i32;
pub const AM_PROPERTY_COPY_MACROVISION: AM_PROPERTY_DVDCOPYPROT = 5i32;
pub const AM_PROPERTY_DVDCOPY_CHLG_KEY: AM_PROPERTY_DVDCOPYPROT = 1i32;
pub const AM_PROPERTY_DVDCOPY_DEC_KEY2: AM_PROPERTY_DVDCOPYPROT = 3i32;
pub const AM_PROPERTY_DVDCOPY_DISC_KEY: AM_PROPERTY_DVDCOPYPROT = 128i32;
pub const AM_PROPERTY_DVDCOPY_DVD_KEY1: AM_PROPERTY_DVDCOPYPROT = 2i32;
pub const AM_PROPERTY_DVDCOPY_REGION: AM_PROPERTY_DVDCOPYPROT = 6i32;
pub const AM_PROPERTY_DVDCOPY_SET_COPY_STATE: AM_PROPERTY_DVDCOPYPROT = 7i32;
pub const AM_PROPERTY_DVDCOPY_SUPPORTS_NEW_KEYCOUNT: AM_PROPERTY_DVDCOPYPROT = 11i32;
pub const AM_PROPERTY_DVDCOPY_TITLE_KEY: AM_PROPERTY_DVDCOPYPROT = 4i32;
pub const AM_PROPERTY_DVDKARAOKE_DATA: AM_PROPERTY_DVDKARAOKE = 1i32;
pub const AM_PROPERTY_DVDKARAOKE_ENABLE: AM_PROPERTY_DVDKARAOKE = 0i32;
pub const AM_PROPERTY_DVDSUBPIC_COMPOSIT_ON: AM_PROPERTY_DVDSUBPIC = 2i32;
pub const AM_PROPERTY_DVDSUBPIC_HLI: AM_PROPERTY_DVDSUBPIC = 1i32;
pub const AM_PROPERTY_DVDSUBPIC_PALETTE: AM_PROPERTY_DVDSUBPIC = 0i32;
pub const AM_PROPERTY_FRAMESTEP_CANCEL: AM_PROPERTY_FRAMESTEP = 2i32;
pub const AM_PROPERTY_FRAMESTEP_CANSTEP: AM_PROPERTY_FRAMESTEP = 3i32;
pub const AM_PROPERTY_FRAMESTEP_CANSTEPMULTIPLE: AM_PROPERTY_FRAMESTEP = 4i32;
pub const AM_PROPERTY_FRAMESTEP_STEP: AM_PROPERTY_FRAMESTEP = 1i32;
pub const AM_PUSHSOURCECAPS_INTERNAL_RM: _AM_PUSHSOURCE_FLAGS = 1i32;
pub const AM_PUSHSOURCECAPS_NOT_LIVE: _AM_PUSHSOURCE_FLAGS = 2i32;
pub const AM_PUSHSOURCECAPS_PRIVATE_CLOCK: _AM_PUSHSOURCE_FLAGS = 4i32;
pub const AM_PUSHSOURCEREQS_USE_CLOCK_CHAIN: _AM_PUSHSOURCE_FLAGS = 131072i32;
pub const AM_PUSHSOURCEREQS_USE_STREAM_CLOCK: _AM_PUSHSOURCE_FLAGS = 65536i32;
pub const AM_QUERY_DECODER_ATSC_HD_SUPPORT: u32 = 5u32;
pub const AM_QUERY_DECODER_ATSC_SD_SUPPORT: u32 = 4u32;
pub const AM_QUERY_DECODER_DVD_SUPPORT: u32 = 3u32;
pub const AM_QUERY_DECODER_DXVA_1_SUPPORT: u32 = 2u32;
pub const AM_QUERY_DECODER_VMR_SUPPORT: u32 = 1u32;
pub const AM_RATE_ChangeRate: AM_PROPERTY_DVD_RATE_CHANGE = 1i32;
pub const AM_RATE_CorrectTS: AM_PROPERTY_TS_RATE_CHANGE = 8i32;
pub const AM_RATE_DecoderPosition: AM_PROPERTY_DVD_RATE_CHANGE = 4i32;
pub const AM_RATE_DecoderVersion: AM_PROPERTY_DVD_RATE_CHANGE = 5i32;
pub const AM_RATE_ExactRateChange: AM_PROPERTY_TS_RATE_CHANGE = 2i32;
pub const AM_RATE_FullDataRateMax: AM_PROPERTY_DVD_RATE_CHANGE = 2i32;
pub const AM_RATE_MaxFullDataRate: AM_PROPERTY_TS_RATE_CHANGE = 3i32;
pub const AM_RATE_QueryFullFrameRate: AM_PROPERTY_TS_RATE_CHANGE = 6i32;
pub const AM_RATE_QueryLastRateSegPTS: AM_PROPERTY_TS_RATE_CHANGE = 7i32;
pub const AM_RATE_QueryMapping: AM_PROPERTY_TS_RATE_CHANGE = 11i32;
pub const AM_RATE_ResetOnTimeDisc: AM_PROPERTY_TS_RATE_CHANGE = 10i32;
pub const AM_RATE_ReverseDecode: AM_PROPERTY_DVD_RATE_CHANGE = 3i32;
pub const AM_RATE_ReverseMaxFullDataRate: AM_PROPERTY_TS_RATE_CHANGE = 9i32;
pub const AM_RATE_SimpleRateChange: AM_PROPERTY_TS_RATE_CHANGE = 1i32;
pub const AM_RATE_Step: AM_PROPERTY_TS_RATE_CHANGE = 4i32;
pub const AM_RATE_UseRateVersion: AM_PROPERTY_TS_RATE_CHANGE = 5i32;
pub const AM_RENDEREX_RENDERTOEXISTINGRENDERERS: _AM_RENSDEREXFLAGS = 1i32;
pub const AM_ReverseBlockEnd: u32 = 4u32;
pub const AM_ReverseBlockStart: u32 = 2u32;
pub const AM_SAMPLE_DATADISCONTINUITY: AM_SAMPLE_PROPERTY_FLAGS = 4i32;
pub const AM_SAMPLE_ENDOFSTREAM: AM_SAMPLE_PROPERTY_FLAGS = 512i32;
pub const AM_SAMPLE_FLUSH_ON_PAUSE: AM_SAMPLE_PROPERTY_FLAGS = 128i32;
pub const AM_SAMPLE_PREROLL: AM_SAMPLE_PROPERTY_FLAGS = 2i32;
pub const AM_SAMPLE_SPLICEPOINT: AM_SAMPLE_PROPERTY_FLAGS = 1i32;
pub const AM_SAMPLE_STOPVALID: AM_SAMPLE_PROPERTY_FLAGS = 256i32;
pub const AM_SAMPLE_TIMEDISCONTINUITY: AM_SAMPLE_PROPERTY_FLAGS = 64i32;
pub const AM_SAMPLE_TIMEVALID: AM_SAMPLE_PROPERTY_FLAGS = 16i32;
pub const AM_SAMPLE_TYPECHANGED: AM_SAMPLE_PROPERTY_FLAGS = 8i32;
pub const AM_SEEKING_AbsolutePositioning: AM_SEEKING_SEEKING_FLAGS = 1i32;
pub const AM_SEEKING_CanDoSegments: AM_SEEKING_SEEKING_CAPABILITIES = 128i32;
pub const AM_SEEKING_CanGetCurrentPos: AM_SEEKING_SEEKING_CAPABILITIES = 8i32;
pub const AM_SEEKING_CanGetDuration: AM_SEEKING_SEEKING_CAPABILITIES = 32i32;
pub const AM_SEEKING_CanGetStopPos: AM_SEEKING_SEEKING_CAPABILITIES = 16i32;
pub const AM_SEEKING_CanPlayBackwards: AM_SEEKING_SEEKING_CAPABILITIES = 64i32;
pub const AM_SEEKING_CanSeekAbsolute: AM_SEEKING_SEEKING_CAPABILITIES = 1i32;
pub const AM_SEEKING_CanSeekBackwards: AM_SEEKING_SEEKING_CAPABILITIES = 4i32;
pub const AM_SEEKING_CanSeekForwards: AM_SEEKING_SEEKING_CAPABILITIES = 2i32;
pub const AM_SEEKING_IncrementalPositioning: AM_SEEKING_SEEKING_FLAGS = 3i32;
pub const AM_SEEKING_NoFlush: AM_SEEKING_SEEKING_FLAGS = 32i32;
pub const AM_SEEKING_NoPositioning: AM_SEEKING_SEEKING_FLAGS = 0i32;
pub const AM_SEEKING_PositioningBitsMask: AM_SEEKING_SEEKING_FLAGS = 3i32;
pub const AM_SEEKING_RelativePositioning: AM_SEEKING_SEEKING_FLAGS = 2i32;
pub const AM_SEEKING_ReturnTime: AM_SEEKING_SEEKING_FLAGS = 8i32;
pub const AM_SEEKING_SeekToKeyFrame: AM_SEEKING_SEEKING_FLAGS = 4i32;
pub const AM_SEEKING_Segment: AM_SEEKING_SEEKING_FLAGS = 16i32;
pub const AM_SEEKING_Source: AM_SEEKING_SEEKING_CAPABILITIES = 256i32;
pub const AM_STREAM_CONTROL: AM_SAMPLE_PROPERTY_FLAGS = 1i32;
pub const AM_STREAM_INFO_DISCARDING: AM_STREAM_INFO_FLAGS = 4i32;
pub const AM_STREAM_INFO_START_DEFINED: AM_STREAM_INFO_FLAGS = 1i32;
pub const AM_STREAM_INFO_STOP_DEFINED: AM_STREAM_INFO_FLAGS = 2i32;
pub const AM_STREAM_INFO_STOP_SEND_EXTRA: AM_STREAM_INFO_FLAGS = 16i32;
pub const AM_STREAM_MEDIA: AM_SAMPLE_PROPERTY_FLAGS = 0i32;
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
pub const AM_WST_DRAWBGMODE_Opaque: AM_WST_DRAWBGMODE = 0i32;
pub const AM_WST_DRAWBGMODE_Transparent: AM_WST_DRAWBGMODE = 1i32;
pub const AM_WST_LEVEL_1_5: AM_WST_LEVEL = 0i32;
pub const AM_WST_SERVICE_IDS: AM_WST_SERVICE = 2i32;
pub const AM_WST_SERVICE_Invalid: AM_WST_SERVICE = 3i32;
pub const AM_WST_SERVICE_None: AM_WST_SERVICE = 0i32;
pub const AM_WST_SERVICE_Text: AM_WST_SERVICE = 1i32;
pub const AM_WST_STATE_Off: AM_WST_STATE = 0i32;
pub const AM_WST_STATE_On: AM_WST_STATE = 1i32;
pub const AM_WST_STYLE_Invers: AM_WST_STYLE = 1i32;
pub const AM_WST_STYLE_None: AM_WST_STYLE = 0i32;
pub const ANNEX_A_DSM_CC: MPEG2StreamType = 8i32;
pub const ATSCCT_AC3: ATSCComponentTypeFlags = 1i32;
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
pub const AVISF_DISABLED: u32 = 1u32;
pub const AVISF_VIDEO_PALCHANGES: u32 = 65536u32;
pub const AVISTDINDEX_DELTAFRAME: u32 = 2147483648u32;
pub const AVI_HEADERSIZE: u32 = 2048u32;
pub const AVI_INDEX_IS_DATA: u32 = 128u32;
pub const AVI_INDEX_OF_CHUNKS: u32 = 1u32;
pub const AVI_INDEX_OF_INDEXES: u32 = 0u32;
pub const AVI_INDEX_OF_SUB_2FIELD: u32 = 3u32;
pub const AVI_INDEX_OF_TIMED_CHUNKS: u32 = 2u32;
pub const AVI_INDEX_SUB_2FIELD: u32 = 1u32;
pub const AVI_INDEX_SUB_DEFAULT: u32 = 0u32;
pub const AnalogVideoMask_MCE_NTSC: AnalogVideoStandard = 1052167i32;
pub const AnalogVideoMask_MCE_PAL: AnalogVideoStandard = 496i32;
pub const AnalogVideoMask_MCE_SECAM: AnalogVideoStandard = 1044480i32;
pub const AnalogVideo_NTSC_433: AnalogVideoStandard = 4i32;
pub const AnalogVideo_NTSC_M: AnalogVideoStandard = 1i32;
pub const AnalogVideo_NTSC_M_J: AnalogVideoStandard = 2i32;
pub const AnalogVideo_NTSC_Mask: u32 = 7u32;
pub const AnalogVideo_None: AnalogVideoStandard = 0i32;
pub const AnalogVideo_PAL_60: AnalogVideoStandard = 2048i32;
pub const AnalogVideo_PAL_B: AnalogVideoStandard = 16i32;
pub const AnalogVideo_PAL_D: AnalogVideoStandard = 32i32;
pub const AnalogVideo_PAL_G: AnalogVideoStandard = 64i32;
pub const AnalogVideo_PAL_H: AnalogVideoStandard = 128i32;
pub const AnalogVideo_PAL_I: AnalogVideoStandard = 256i32;
pub const AnalogVideo_PAL_M: AnalogVideoStandard = 512i32;
pub const AnalogVideo_PAL_Mask: u32 = 1052656u32;
pub const AnalogVideo_PAL_N: AnalogVideoStandard = 1024i32;
pub const AnalogVideo_PAL_N_COMBO: AnalogVideoStandard = 1048576i32;
pub const AnalogVideo_SECAM_B: AnalogVideoStandard = 4096i32;
pub const AnalogVideo_SECAM_D: AnalogVideoStandard = 8192i32;
pub const AnalogVideo_SECAM_G: AnalogVideoStandard = 16384i32;
pub const AnalogVideo_SECAM_H: AnalogVideoStandard = 32768i32;
pub const AnalogVideo_SECAM_K: AnalogVideoStandard = 65536i32;
pub const AnalogVideo_SECAM_K1: AnalogVideoStandard = 131072i32;
pub const AnalogVideo_SECAM_L: AnalogVideoStandard = 262144i32;
pub const AnalogVideo_SECAM_L1: AnalogVideoStandard = 524288i32;
pub const AnalogVideo_SECAM_Mask: u32 = 1044480u32;
pub const Associated: SmartCardAssociationType = 1i32;
pub const AssociationUnknown: SmartCardAssociationType = 2i32;
pub const BDACOMP_EXCLUDE_TS_FROM_TR: BDA_Comp_Flags = 1i32;
pub const BDACOMP_INCLUDE_COMPONENTS_IN_TR: BDA_Comp_Flags = 4i32;
pub const BDACOMP_INCLUDE_LOCATOR_IN_TR: BDA_Comp_Flags = 2i32;
pub const BDACOMP_NOT_DEFINED: BDA_Comp_Flags = 0i32;
pub const BDA_BCC_RATE_1_2: BinaryConvolutionCodeRate = 1i32;
pub const BDA_BCC_RATE_1_3: BinaryConvolutionCodeRate = 10i32;
pub const BDA_BCC_RATE_1_4: BinaryConvolutionCodeRate = 9i32;
pub const BDA_BCC_RATE_2_3: BinaryConvolutionCodeRate = 2i32;
pub const BDA_BCC_RATE_2_5: BinaryConvolutionCodeRate = 11i32;
pub const BDA_BCC_RATE_3_4: BinaryConvolutionCodeRate = 3i32;
pub const BDA_BCC_RATE_3_5: BinaryConvolutionCodeRate = 4i32;
pub const BDA_BCC_RATE_4_5: BinaryConvolutionCodeRate = 5i32;
pub const BDA_BCC_RATE_5_11: BinaryConvolutionCodeRate = 7i32;
pub const BDA_BCC_RATE_5_6: BinaryConvolutionCodeRate = 6i32;
pub const BDA_BCC_RATE_6_7: BinaryConvolutionCodeRate = 12i32;
pub const BDA_BCC_RATE_7_8: BinaryConvolutionCodeRate = 8i32;
pub const BDA_BCC_RATE_8_9: BinaryConvolutionCodeRate = 13i32;
pub const BDA_BCC_RATE_9_10: BinaryConvolutionCodeRate = 14i32;
pub const BDA_BCC_RATE_MAX: BinaryConvolutionCodeRate = 15i32;
pub const BDA_BCC_RATE_NOT_DEFINED: BinaryConvolutionCodeRate = 0i32;
pub const BDA_BCC_RATE_NOT_SET: BinaryConvolutionCodeRate = -1i32;
pub const BDA_CHANGES_COMPLETE: BDA_CHANGE_STATE = 0i32;
pub const BDA_CHANGES_PENDING: BDA_CHANGE_STATE = 1i32;
pub const BDA_CHAN_BANDWITH_NOT_DEFINED: BDA_Channel_Bandwidth = 0i32;
pub const BDA_CHAN_BANDWITH_NOT_SET: BDA_Channel_Bandwidth = -1i32;
pub const BDA_DISCOVERY_COMPLETE: BDA_DISCOVERY_STATE = 2i32;
pub const BDA_DISCOVERY_REQUIRED: BDA_DISCOVERY_STATE = 1i32;
pub const BDA_DISCOVERY_UNSPECIFIED: BDA_DISCOVERY_STATE = 0i32;
pub const BDA_DrmPairing_Aborted: BDA_DrmPairingError = 8i32;
pub const BDA_DrmPairing_DrmInitFailed: BDA_DrmPairingError = 5i32;
pub const BDA_DrmPairing_DrmNotPaired: BDA_DrmPairingError = 6i32;
pub const BDA_DrmPairing_DrmRePairSoon: BDA_DrmPairingError = 7i32;
pub const BDA_DrmPairing_HardwareFailure: BDA_DrmPairingError = 1i32;
pub const BDA_DrmPairing_NeedIndiv: BDA_DrmPairingError = 3i32;
pub const BDA_DrmPairing_NeedRevocationData: BDA_DrmPairingError = 2i32;
pub const BDA_DrmPairing_NeedSDKUpdate: BDA_DrmPairingError = 9i32;
pub const BDA_DrmPairing_Other: BDA_DrmPairingError = 4i32;
pub const BDA_DrmPairing_Succeeded: BDA_DrmPairingError = 0i32;
pub const BDA_EVENT_ACCESS_DENIED: BDA_EVENT_ID = 15i32;
pub const BDA_EVENT_ACCESS_GRANTED: BDA_EVENT_ID = 14i32;
pub const BDA_EVENT_CHANNEL_ACQUIRED: BDA_EVENT_ID = 4i32;
pub const BDA_EVENT_CHANNEL_ACTIVATED: BDA_EVENT_ID = 7i32;
pub const BDA_EVENT_CHANNEL_DEACTIVATED: BDA_EVENT_ID = 8i32;
pub const BDA_EVENT_CHANNEL_LOST: BDA_EVENT_ID = 5i32;
pub const BDA_EVENT_CHANNEL_SOURCE_CHANGED: BDA_EVENT_ID = 6i32;
pub const BDA_EVENT_DATA_START: BDA_EVENT_ID = 2i32;
pub const BDA_EVENT_DATA_STOP: BDA_EVENT_ID = 3i32;
pub const BDA_EVENT_OFFER_EXTENDED: BDA_EVENT_ID = 16i32;
pub const BDA_EVENT_PURCHASE_COMPLETED: BDA_EVENT_ID = 17i32;
pub const BDA_EVENT_SIGNAL_LOCK: BDA_EVENT_ID = 1i32;
pub const BDA_EVENT_SIGNAL_LOSS: BDA_EVENT_ID = 0i32;
pub const BDA_EVENT_SMART_CARD_INSERTED: BDA_EVENT_ID = 18i32;
pub const BDA_EVENT_SMART_CARD_REMOVED: BDA_EVENT_ID = 19i32;
pub const BDA_EVENT_SUBCHANNEL_ACQUIRED: BDA_EVENT_ID = 9i32;
pub const BDA_EVENT_SUBCHANNEL_ACTIVATED: BDA_EVENT_ID = 12i32;
pub const BDA_EVENT_SUBCHANNEL_DEACTIVATED: BDA_EVENT_ID = 13i32;
pub const BDA_EVENT_SUBCHANNEL_LOST: BDA_EVENT_ID = 10i32;
pub const BDA_EVENT_SUBCHANNEL_SOURCE_CHANGED: BDA_EVENT_ID = 11i32;
pub const BDA_E_ACCESS_DENIED: windows_core::HRESULT = 0xC0040009_u32 as _;
pub const BDA_E_BUFFER_TOO_SMALL: windows_core::HRESULT = 0xC004000B_u32 as _;
pub const BDA_E_DISABLED: windows_core::HRESULT = 0xC004000E_u32 as _;
pub const BDA_E_FAILURE: windows_core::HRESULT = 0xC0040001_u32 as _;
pub const BDA_E_INVALID_CAPTURE_TOKEN: windows_core::HRESULT = 0xC0044002_u32 as _;
pub const BDA_E_INVALID_ENTITLEMENT_TOKEN: windows_core::HRESULT = 0xC0044001_u32 as _;
pub const BDA_E_INVALID_HANDLE: windows_core::HRESULT = 0xC0040006_u32 as _;
pub const BDA_E_INVALID_LANGUAGE: windows_core::HRESULT = 0xC0040010_u32 as _;
pub const BDA_E_INVALID_PURCHASE_TOKEN: windows_core::HRESULT = 0xC0044004_u32 as _;
pub const BDA_E_INVALID_SCHEMA: windows_core::HRESULT = 0xC0040005_u32 as _;
pub const BDA_E_INVALID_TUNE_REQUEST: windows_core::HRESULT = 0xC0043004_u32 as _;
pub const BDA_E_INVALID_TYPE: windows_core::HRESULT = 0xC0040007_u32 as _;
pub const BDA_E_IPNETWORK_ADDRESS_NOT_FOUND: windows_core::HRESULT = 0xC0045002_u32 as _;
pub const BDA_E_IPNETWORK_ERROR: windows_core::HRESULT = 0xC0045001_u32 as _;
pub const BDA_E_IPNETWORK_TIMEOUT: windows_core::HRESULT = 0xC0045003_u32 as _;
pub const BDA_E_IPNETWORK_UNAVAILABLE: windows_core::HRESULT = 0xC0045004_u32 as _;
pub const BDA_E_NOT_FOUND: windows_core::HRESULT = 0xC004000A_u32 as _;
pub const BDA_E_NOT_IMPLEMENTED: windows_core::HRESULT = 0xC0040002_u32 as _;
pub const BDA_E_NO_HANDLER: windows_core::HRESULT = 0xC004000F_u32 as _;
pub const BDA_E_NO_MORE_DATA: windows_core::HRESULT = 0xC0041002_u32 as _;
pub const BDA_E_NO_MORE_EVENTS: windows_core::HRESULT = 0xC0041001_u32 as _;
pub const BDA_E_NO_SUCH_COMMAND: windows_core::HRESULT = 0xC0040003_u32 as _;
pub const BDA_E_OUT_OF_BOUNDS: windows_core::HRESULT = 0xC0040004_u32 as _;
pub const BDA_E_OUT_OF_MEMORY: windows_core::HRESULT = 0xC004000D_u32 as _;
pub const BDA_E_OUT_OF_RESOURCES: windows_core::HRESULT = 0xC004000C_u32 as _;
pub const BDA_E_READ_ONLY: windows_core::HRESULT = 0xC0040008_u32 as _;
pub const BDA_E_TIMEOUT_ELAPSED: windows_core::HRESULT = 0xC0040011_u32 as _;
pub const BDA_E_TUNER_CONFLICT: windows_core::HRESULT = 0xC0043003_u32 as _;
pub const BDA_E_TUNER_INITIALIZING: windows_core::HRESULT = 0xC0043001_u32 as _;
pub const BDA_E_TUNER_REQUIRED: windows_core::HRESULT = 0xC0043002_u32 as _;
pub const BDA_E_TUNE_FAILED_SDV01: windows_core::HRESULT = 0xC0046001_u32 as _;
pub const BDA_E_TUNE_FAILED_SDV02: windows_core::HRESULT = 0xC0046002_u32 as _;
pub const BDA_E_TUNE_FAILED_SDV03: windows_core::HRESULT = 0xC0046003_u32 as _;
pub const BDA_E_TUNE_FAILED_SDV04: windows_core::HRESULT = 0xC0046004_u32 as _;
pub const BDA_E_TUNE_FAILED_SDV05: windows_core::HRESULT = 0xC0046005_u32 as _;
pub const BDA_E_TUNE_FAILED_SDV06: windows_core::HRESULT = 0xC0046006_u32 as _;
pub const BDA_E_TUNE_FAILED_SDV07: windows_core::HRESULT = 0xC0046007_u32 as _;
pub const BDA_E_TUNE_FAILED_SDV08: windows_core::HRESULT = 0xC0046008_u32 as _;
pub const BDA_E_TUNE_FAILED_SDVFF: windows_core::HRESULT = 0xC00460FF_u32 as _;
pub const BDA_E_WMDRM_INVALID_CERTIFICATE: windows_core::HRESULT = 0xC004F002_u32 as _;
pub const BDA_E_WMDRM_INVALID_DATE: windows_core::HRESULT = 0xC004F005_u32 as _;
pub const BDA_E_WMDRM_INVALID_PROXIMITY: windows_core::HRESULT = 0xC004F006_u32 as _;
pub const BDA_E_WMDRM_INVALID_SIGNATURE: windows_core::HRESULT = 0xC004F001_u32 as _;
pub const BDA_E_WMDRM_INVALID_VERSION: windows_core::HRESULT = 0xC004F004_u32 as _;
pub const BDA_E_WMDRM_KEY_ID_NOT_FOUND: windows_core::HRESULT = 0xC004F008_u32 as _;
pub const BDA_E_WOULD_DISRUPT_STREAMING: windows_core::HRESULT = 0xC0044003_u32 as _;
pub const BDA_FEC_BCH: FECMethod = 4i32;
pub const BDA_FEC_LDPC: FECMethod = 3i32;
pub const BDA_FEC_MAX: FECMethod = 6i32;
pub const BDA_FEC_METHOD_NOT_DEFINED: FECMethod = 0i32;
pub const BDA_FEC_METHOD_NOT_SET: FECMethod = -1i32;
pub const BDA_FEC_RS_147_130: FECMethod = 5i32;
pub const BDA_FEC_RS_204_188: FECMethod = 2i32;
pub const BDA_FEC_VITERBI: FECMethod = 1i32;
pub const BDA_FILTERED_MULTICAST: BDA_MULTICAST_MODE = 1i32;
pub const BDA_FREQUENCY_MULTIPLIER_NOT_DEFINED: BDA_Frequency_Multiplier = 0i32;
pub const BDA_FREQUENCY_MULTIPLIER_NOT_SET: BDA_Frequency_Multiplier = -1i32;
pub const BDA_FREQUENCY_NOT_DEFINED: BDA_Frequency = 0i32;
pub const BDA_FREQUENCY_NOT_SET: BDA_Frequency = -1i32;
pub const BDA_GUARD_19_128: GuardInterval = 6i32;
pub const BDA_GUARD_19_256: GuardInterval = 7i32;
pub const BDA_GUARD_1_128: GuardInterval = 5i32;
pub const BDA_GUARD_1_16: GuardInterval = 2i32;
pub const BDA_GUARD_1_32: GuardInterval = 1i32;
pub const BDA_GUARD_1_4: GuardInterval = 4i32;
pub const BDA_GUARD_1_8: GuardInterval = 3i32;
pub const BDA_GUARD_MAX: GuardInterval = 8i32;
pub const BDA_GUARD_NOT_DEFINED: GuardInterval = 0i32;
pub const BDA_GUARD_NOT_SET: GuardInterval = -1i32;
pub const BDA_HALPHA_1: HierarchyAlpha = 1i32;
pub const BDA_HALPHA_2: HierarchyAlpha = 2i32;
pub const BDA_HALPHA_4: HierarchyAlpha = 3i32;
pub const BDA_HALPHA_MAX: HierarchyAlpha = 4i32;
pub const BDA_HALPHA_NOT_DEFINED: HierarchyAlpha = 0i32;
pub const BDA_HALPHA_NOT_SET: HierarchyAlpha = -1i32;
pub const BDA_LNB_SOURCE_A: LNB_Source = 1i32;
pub const BDA_LNB_SOURCE_B: LNB_Source = 2i32;
pub const BDA_LNB_SOURCE_C: LNB_Source = 3i32;
pub const BDA_LNB_SOURCE_D: LNB_Source = 4i32;
pub const BDA_LNB_SOURCE_MAX: LNB_Source = 5i32;
pub const BDA_LNB_SOURCE_NOT_DEFINED: LNB_Source = 0i32;
pub const BDA_LNB_SOURCE_NOT_SET: LNB_Source = -1i32;
pub const BDA_MOD_1024QAM: ModulationType = 19i32;
pub const BDA_MOD_112QAM: ModulationType = 6i32;
pub const BDA_MOD_128QAM: ModulationType = 7i32;
pub const BDA_MOD_160QAM: ModulationType = 8i32;
pub const BDA_MOD_16APSK: ModulationType = 29i32;
pub const BDA_MOD_16QAM: ModulationType = 1i32;
pub const BDA_MOD_16VSB: ModulationType = 24i32;
pub const BDA_MOD_192QAM: ModulationType = 9i32;
pub const BDA_MOD_224QAM: ModulationType = 10i32;
pub const BDA_MOD_256QAM: ModulationType = 11i32;
pub const BDA_MOD_320QAM: ModulationType = 12i32;
pub const BDA_MOD_32APSK: ModulationType = 30i32;
pub const BDA_MOD_32QAM: ModulationType = 2i32;
pub const BDA_MOD_384QAM: ModulationType = 13i32;
pub const BDA_MOD_448QAM: ModulationType = 14i32;
pub const BDA_MOD_512QAM: ModulationType = 15i32;
pub const BDA_MOD_640QAM: ModulationType = 16i32;
pub const BDA_MOD_64QAM: ModulationType = 3i32;
pub const BDA_MOD_768QAM: ModulationType = 17i32;
pub const BDA_MOD_80QAM: ModulationType = 4i32;
pub const BDA_MOD_896QAM: ModulationType = 18i32;
pub const BDA_MOD_8PSK: ModulationType = 27i32;
pub const BDA_MOD_8VSB: ModulationType = 23i32;
pub const BDA_MOD_96QAM: ModulationType = 5i32;
pub const BDA_MOD_ANALOG_AMPLITUDE: ModulationType = 25i32;
pub const BDA_MOD_ANALOG_FREQUENCY: ModulationType = 26i32;
pub const BDA_MOD_BPSK: ModulationType = 21i32;
pub const BDA_MOD_DIRECTV: ModulationType = 33i32;
pub const BDA_MOD_ISDB_S_TMCC: ModulationType = 35i32;
pub const BDA_MOD_ISDB_T_TMCC: ModulationType = 34i32;
pub const BDA_MOD_MAX: ModulationType = 36i32;
pub const BDA_MOD_NBC_8PSK: ModulationType = 32i32;
pub const BDA_MOD_NBC_QPSK: ModulationType = 31i32;
pub const BDA_MOD_NOT_DEFINED: ModulationType = 0i32;
pub const BDA_MOD_NOT_SET: ModulationType = -1i32;
pub const BDA_MOD_OQPSK: ModulationType = 22i32;
pub const BDA_MOD_QPSK: ModulationType = 20i32;
pub const BDA_MOD_RF: ModulationType = 28i32;
pub const BDA_NO_MULTICAST: BDA_MULTICAST_MODE = 2i32;
pub const BDA_PILOT_MAX: Pilot = 3i32;
pub const BDA_PILOT_NOT_DEFINED: Pilot = 0i32;
pub const BDA_PILOT_NOT_SET: Pilot = -1i32;
pub const BDA_PILOT_OFF: Pilot = 1i32;
pub const BDA_PILOT_ON: Pilot = 2i32;
pub const BDA_PLP_ID_NOT_SET: i32 = -1i32;
pub const BDA_POLARISATION_CIRCULAR_L: Polarisation = 3i32;
pub const BDA_POLARISATION_CIRCULAR_R: Polarisation = 4i32;
pub const BDA_POLARISATION_LINEAR_H: Polarisation = 1i32;
pub const BDA_POLARISATION_LINEAR_V: Polarisation = 2i32;
pub const BDA_POLARISATION_MAX: Polarisation = 5i32;
pub const BDA_POLARISATION_NOT_DEFINED: Polarisation = 0i32;
pub const BDA_POLARISATION_NOT_SET: Polarisation = -1i32;
pub const BDA_PROMISCUOUS_MULTICAST: BDA_MULTICAST_MODE = 0i32;
pub const BDA_RANGE_NOT_DEFINED: BDA_Range = 0i32;
pub const BDA_RANGE_NOT_SET: BDA_Range = -1i32;
pub const BDA_ROLL_OFF_20: RollOff = 1i32;
pub const BDA_ROLL_OFF_25: RollOff = 2i32;
pub const BDA_ROLL_OFF_35: RollOff = 3i32;
pub const BDA_ROLL_OFF_MAX: RollOff = 4i32;
pub const BDA_ROLL_OFF_NOT_DEFINED: RollOff = 0i32;
pub const BDA_ROLL_OFF_NOT_SET: RollOff = -1i32;
pub const BDA_SCAN_MOD_1024QAM: ScanModulationTypes = 262144i32;
pub const BDA_SCAN_MOD_112QAM: ScanModulationTypes = 32i32;
pub const BDA_SCAN_MOD_128QAM: ScanModulationTypes = 64i32;
pub const BDA_SCAN_MOD_160QAM: ScanModulationTypes = 128i32;
pub const BDA_SCAN_MOD_16APSK: ScanModulationTypes = 268435456i32;
pub const BDA_SCAN_MOD_16QAM: ScanModulationTypes = 1i32;
pub const BDA_SCAN_MOD_16VSB: ScanModulationTypes = 8388608i32;
pub const BDA_SCAN_MOD_192QAM: ScanModulationTypes = 256i32;
pub const BDA_SCAN_MOD_224QAM: ScanModulationTypes = 512i32;
pub const BDA_SCAN_MOD_256QAM: ScanModulationTypes = 1024i32;
pub const BDA_SCAN_MOD_320QAM: ScanModulationTypes = 2048i32;
pub const BDA_SCAN_MOD_32APSK: ScanModulationTypes = 536870912i32;
pub const BDA_SCAN_MOD_32QAM: ScanModulationTypes = 2i32;
pub const BDA_SCAN_MOD_384QAM: ScanModulationTypes = 4096i32;
pub const BDA_SCAN_MOD_448QAM: ScanModulationTypes = 8192i32;
pub const BDA_SCAN_MOD_512QAM: ScanModulationTypes = 16384i32;
pub const BDA_SCAN_MOD_640QAM: ScanModulationTypes = 32768i32;
pub const BDA_SCAN_MOD_64QAM: ScanModulationTypes = 4i32;
pub const BDA_SCAN_MOD_768QAM: ScanModulationTypes = 65536i32;
pub const BDA_SCAN_MOD_80QAM: ScanModulationTypes = 8i32;
pub const BDA_SCAN_MOD_896QAM: ScanModulationTypes = 131072i32;
pub const BDA_SCAN_MOD_8PSK: ScanModulationTypes = 67108864i32;
pub const BDA_SCAN_MOD_8VSB: ScanModulationTypes = 4194304i32;
pub const BDA_SCAN_MOD_96QAM: ScanModulationTypes = 16i32;
pub const BDA_SCAN_MOD_AM_RADIO: ScanModulationTypes = 16777216i32;
pub const BDA_SCAN_MOD_BPSK: ScanModulationTypes = 1048576i32;
pub const BDA_SCAN_MOD_FM_RADIO: ScanModulationTypes = 33554432i32;
pub const BDA_SCAN_MOD_OQPSK: ScanModulationTypes = 2097152i32;
pub const BDA_SCAN_MOD_QPSK: ScanModulationTypes = 524288i32;
pub const BDA_SCAN_MOD_RF: ScanModulationTypes = 134217728i32;
pub const BDA_SIGNAL_ACTIVE: BDA_SIGNAL_STATE = 2i32;
pub const BDA_SIGNAL_INACTIVE: BDA_SIGNAL_STATE = 1i32;
pub const BDA_SIGNAL_UNAVAILABLE: BDA_SIGNAL_STATE = 0i32;
pub const BDA_SPECTRAL_INVERSION_AUTOMATIC: SpectralInversion = 1i32;
pub const BDA_SPECTRAL_INVERSION_INVERTED: SpectralInversion = 3i32;
pub const BDA_SPECTRAL_INVERSION_MAX: SpectralInversion = 4i32;
pub const BDA_SPECTRAL_INVERSION_NORMAL: SpectralInversion = 2i32;
pub const BDA_SPECTRAL_INVERSION_NOT_DEFINED: SpectralInversion = 0i32;
pub const BDA_SPECTRAL_INVERSION_NOT_SET: SpectralInversion = -1i32;
pub const BDA_UNDEFINED_CHANNEL: BDA_Channel = -1i32;
pub const BDA_UNITIALIZED_MPEG2STREAMTYPE: MPEG2StreamType = -1i32;
pub const BDA_XMIT_MODE_16K: TransmissionMode = 7i32;
pub const BDA_XMIT_MODE_1K: TransmissionMode = 6i32;
pub const BDA_XMIT_MODE_2K: TransmissionMode = 1i32;
pub const BDA_XMIT_MODE_2K_INTERLEAVED: TransmissionMode = 4i32;
pub const BDA_XMIT_MODE_32K: TransmissionMode = 8i32;
pub const BDA_XMIT_MODE_4K: TransmissionMode = 3i32;
pub const BDA_XMIT_MODE_4K_INTERLEAVED: TransmissionMode = 5i32;
pub const BDA_XMIT_MODE_8K: TransmissionMode = 2i32;
pub const BDA_XMIT_MODE_MAX: TransmissionMode = 9i32;
pub const BDA_XMIT_MODE_NOT_DEFINED: TransmissionMode = 0i32;
pub const BDA_XMIT_MODE_NOT_SET: TransmissionMode = -1i32;
pub const CATEGORY_COUNT: ComponentCategory = 8i32;
pub const CDEF_BYPASS_CLASS_MANAGER: u32 = 2u32;
pub const CDEF_CLASS_DEFAULT: u32 = 1u32;
pub const CDEF_DEVMON_CMGR_DEVICE: u32 = 16u32;
pub const CDEF_DEVMON_DMO: u32 = 32u32;
pub const CDEF_DEVMON_FILTER: u32 = 128u32;
pub const CDEF_DEVMON_PNP_DEVICE: u32 = 64u32;
pub const CDEF_DEVMON_SELECTIVE_MASK: u32 = 240u32;
pub const CDEF_MERIT_ABOVE_DO_NOT_USE: u32 = 8u32;
pub const CFSTR_VFW_FILTERLIST: windows_core::PCSTR = windows_core::s!("Video for Windows 4 Filters");
pub const CHARS_IN_GUID: u32 = 39u32;
pub const CK_INDEX: COLORKEY_TYPE = 1i32;
pub const CK_NOCOLORKEY: COLORKEY_TYPE = 0i32;
pub const CK_RGB: COLORKEY_TYPE = 2i32;
pub const CLSID_AMAudioData: windows_core::GUID = windows_core::GUID::from_u128(0xf2468580_af8a_11d0_8212_00c04fc32c45);
pub const CLSID_AMAudioStream: windows_core::GUID = windows_core::GUID::from_u128(0x8496e040_af4c_11d0_8212_00c04fc32c45);
pub const CLSID_AMDirectDrawStream: windows_core::GUID = windows_core::GUID::from_u128(0x49c47ce4_9ba4_11d0_8212_00c04fc32c45);
pub const CLSID_AMMediaTypeStream: windows_core::GUID = windows_core::GUID::from_u128(0xcf0f2f7c_f7bf_11d0_900d_00c04fd9189d);
pub const CLSID_AMMultiMediaStream: windows_core::GUID = windows_core::GUID::from_u128(0x49c47ce5_9ba4_11d0_8212_00c04fc32c45);
pub const CLSID_DMOFilterCategory: windows_core::GUID = windows_core::GUID::from_u128(0xbcd5796c_bd52_4d30_ab76_70f975b89199);
pub const CLSID_DMOWrapperFilter: windows_core::GUID = windows_core::GUID::from_u128(0x94297043_bd82_4dfd_b0de_8177739c6d20);
pub const CLSID_PBDA_AUX_DATA_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xfd456373_3323_4090_adca_8ed45f55cf10);
pub const CLSID_PBDA_Encoder_DATA_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x728fd6bc_5546_4716_b103_f899f5a1fa68);
pub const CLSID_PBDA_FDC_DATA_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xe7dbf9a0_22ab_4047_8e67_ef9ad504e729);
pub const CLSID_PBDA_GDDS_DATA_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0xc80c0df3_6052_4c16_9f56_c44c21f73c45);
pub const COMPSTAT_ABORT: COMPLETION_STATUS_FLAGS = 4i32;
pub const COMPSTAT_NOUPDATEOK: COMPLETION_STATUS_FLAGS = 1i32;
pub const COMPSTAT_WAIT: COMPLETION_STATUS_FLAGS = 2i32;
pub const CONDITIONALACCESS_ABORTED: BDA_CONDITIONALACCESS_SESSION_RESULT = 2i32;
pub const CONDITIONALACCESS_ACCESS_NOT_POSSIBLE: BDA_CONDITIONALACCESS_REQUESTTYPE = 1i32;
pub const CONDITIONALACCESS_ACCESS_POSSIBLE: BDA_CONDITIONALACCESS_REQUESTTYPE = 2i32;
pub const CONDITIONALACCESS_ACCESS_POSSIBLE_NO_STREAMING_DISRUPTION: BDA_CONDITIONALACCESS_REQUESTTYPE = 3i32;
pub const CONDITIONALACCESS_ACCESS_UNSPECIFIED: BDA_CONDITIONALACCESS_REQUESTTYPE = 0i32;
pub const CONDITIONALACCESS_CLOSED_ITSELF: BDA_CONDITIONALACCESS_MMICLOSEREASON = 1i32;
pub const CONDITIONALACCESS_DIALOG_FOCUS_CHANGE: BDA_CONDITIONALACCESS_MMICLOSEREASON = 4i32;
pub const CONDITIONALACCESS_DIALOG_TIMEOUT: BDA_CONDITIONALACCESS_MMICLOSEREASON = 3i32;
pub const CONDITIONALACCESS_DIALOG_USER_DISMISSED: BDA_CONDITIONALACCESS_MMICLOSEREASON = 5i32;
pub const CONDITIONALACCESS_DIALOG_USER_NOT_AVAILABLE: BDA_CONDITIONALACCESS_MMICLOSEREASON = 6i32;
pub const CONDITIONALACCESS_ENDED_NOCHANGE: BDA_CONDITIONALACCESS_SESSION_RESULT = 1i32;
pub const CONDITIONALACCESS_SUCCESSFULL: BDA_CONDITIONALACCESS_SESSION_RESULT = 0i32;
pub const CONDITIONALACCESS_TUNER_REQUESTED_CLOSE: BDA_CONDITIONALACCESS_MMICLOSEREASON = 2i32;
pub const CONDITIONALACCESS_UNSPECIFIED: BDA_CONDITIONALACCESS_MMICLOSEREASON = 0i32;
pub const COPP_ACP_ForceDWORD: COPP_ACP_Protection_Level = 2147483647i32;
pub const COPP_ACP_Level0: COPP_ACP_Protection_Level = 0i32;
pub const COPP_ACP_Level1: COPP_ACP_Protection_Level = 1i32;
pub const COPP_ACP_Level2: COPP_ACP_Protection_Level = 2i32;
pub const COPP_ACP_Level3: COPP_ACP_Protection_Level = 3i32;
pub const COPP_ACP_LevelMax: COPP_ACP_Protection_Level = 3i32;
pub const COPP_ACP_LevelMin: COPP_ACP_Protection_Level = 0i32;
pub const COPP_AspectRatio_EN300294_Box14by9Center: COPP_ImageAspectRatio_EN300294 = 1i32;
pub const COPP_AspectRatio_EN300294_Box14by9Top: COPP_ImageAspectRatio_EN300294 = 2i32;
pub const COPP_AspectRatio_EN300294_Box16by9Center: COPP_ImageAspectRatio_EN300294 = 3i32;
pub const COPP_AspectRatio_EN300294_Box16by9Top: COPP_ImageAspectRatio_EN300294 = 4i32;
pub const COPP_AspectRatio_EN300294_BoxGT16by9Center: COPP_ImageAspectRatio_EN300294 = 5i32;
pub const COPP_AspectRatio_EN300294_FullFormat16by9Anamorphic: COPP_ImageAspectRatio_EN300294 = 7i32;
pub const COPP_AspectRatio_EN300294_FullFormat4by3: COPP_ImageAspectRatio_EN300294 = 0i32;
pub const COPP_AspectRatio_EN300294_FullFormat4by3ProtectedCenter: COPP_ImageAspectRatio_EN300294 = 6i32;
pub const COPP_AspectRatio_ForceDWORD: COPP_ImageAspectRatio_EN300294 = 2147483647i32;
pub const COPP_BusType_AGP: COPP_BusType = 4i32;
pub const COPP_BusType_ForceDWORD: COPP_BusType = 2147483647i32;
pub const COPP_BusType_Integrated: COPP_BusType = -2147483648i32;
pub const COPP_BusType_PCI: COPP_BusType = 1i32;
pub const COPP_BusType_PCIExpress: COPP_BusType = 3i32;
pub const COPP_BusType_PCIX: COPP_BusType = 2i32;
pub const COPP_BusType_Unknown: COPP_BusType = 0i32;
pub const COPP_CGMSA_CopyFreely: COPP_CGMSA_Protection_Level = 1i32;
pub const COPP_CGMSA_CopyNever: COPP_CGMSA_Protection_Level = 4i32;
pub const COPP_CGMSA_CopyNoMore: COPP_CGMSA_Protection_Level = 2i32;
pub const COPP_CGMSA_CopyOneGeneration: COPP_CGMSA_Protection_Level = 3i32;
pub const COPP_CGMSA_Disabled: COPP_CGMSA_Protection_Level = 0i32;
pub const COPP_CGMSA_ForceDWORD: COPP_CGMSA_Protection_Level = 2147483647i32;
pub const COPP_CGMSA_LevelMax: COPP_CGMSA_Protection_Level = 12i32;
pub const COPP_CGMSA_LevelMin: COPP_CGMSA_Protection_Level = 0i32;
pub const COPP_CGMSA_RedistributionControlRequired: COPP_CGMSA_Protection_Level = 8i32;
pub const COPP_ConnectorType_ComponentVideo: COPP_ConnectorType = 3i32;
pub const COPP_ConnectorType_CompositeVideo: COPP_ConnectorType = 2i32;
pub const COPP_ConnectorType_DVI: COPP_ConnectorType = 4i32;
pub const COPP_ConnectorType_D_JPN: COPP_ConnectorType = 8i32;
pub const COPP_ConnectorType_ForceDWORD: COPP_ConnectorType = 2147483647i32;
pub const COPP_ConnectorType_HDMI: COPP_ConnectorType = 5i32;
pub const COPP_ConnectorType_Internal: COPP_ConnectorType = -2147483648i32;
pub const COPP_ConnectorType_LVDS: COPP_ConnectorType = 6i32;
pub const COPP_ConnectorType_SVideo: COPP_ConnectorType = 1i32;
pub const COPP_ConnectorType_TMDS: COPP_ConnectorType = 7i32;
pub const COPP_ConnectorType_Unknown: COPP_ConnectorType = -1i32;
pub const COPP_ConnectorType_VGA: COPP_ConnectorType = 0i32;
pub const COPP_DefaultProtectionLevel: u32 = 0u32;
pub const COPP_HDCPFlagsReserved: COPP_StatusHDCPFlags = -2i32;
pub const COPP_HDCPRepeater: COPP_StatusHDCPFlags = 1i32;
pub const COPP_HDCP_ForceDWORD: COPP_HDCP_Protection_Level = 2147483647i32;
pub const COPP_HDCP_Level0: COPP_HDCP_Protection_Level = 0i32;
pub const COPP_HDCP_Level1: COPP_HDCP_Protection_Level = 1i32;
pub const COPP_HDCP_LevelMax: COPP_HDCP_Protection_Level = 1i32;
pub const COPP_HDCP_LevelMin: COPP_HDCP_Protection_Level = 0i32;
pub const COPP_ImageAspectRatio_EN300294_Mask: u32 = 7u32;
pub const COPP_LinkLost: COPP_StatusFlags = 1i32;
pub const COPP_NoProtectionLevelAvailable: i32 = -1i32;
pub const COPP_ProtectionStandard_ARIBTRB15_1125i: COPP_TVProtectionStandard = 16384i32;
pub const COPP_ProtectionStandard_ARIBTRB15_525i: COPP_TVProtectionStandard = 2048i32;
pub const COPP_ProtectionStandard_ARIBTRB15_525p: COPP_TVProtectionStandard = 4096i32;
pub const COPP_ProtectionStandard_ARIBTRB15_750p: COPP_TVProtectionStandard = 8192i32;
pub const COPP_ProtectionStandard_CEA805A_TypeA_1125i: COPP_TVProtectionStandard = 128i32;
pub const COPP_ProtectionStandard_CEA805A_TypeA_525p: COPP_TVProtectionStandard = 32i32;
pub const COPP_ProtectionStandard_CEA805A_TypeA_750p: COPP_TVProtectionStandard = 64i32;
pub const COPP_ProtectionStandard_CEA805A_TypeB_1125i: COPP_TVProtectionStandard = 1024i32;
pub const COPP_ProtectionStandard_CEA805A_TypeB_525p: COPP_TVProtectionStandard = 256i32;
pub const COPP_ProtectionStandard_CEA805A_TypeB_750p: COPP_TVProtectionStandard = 512i32;
pub const COPP_ProtectionStandard_EIA608B_525: COPP_TVProtectionStandard = 8i32;
pub const COPP_ProtectionStandard_EN300294_625i: COPP_TVProtectionStandard = 16i32;
pub const COPP_ProtectionStandard_IEC61880_2_525i: COPP_TVProtectionStandard = 2i32;
pub const COPP_ProtectionStandard_IEC61880_525i: COPP_TVProtectionStandard = 1i32;
pub const COPP_ProtectionStandard_IEC62375_625p: COPP_TVProtectionStandard = 4i32;
pub const COPP_ProtectionStandard_Mask: COPP_TVProtectionStandard = -2147450881i32;
pub const COPP_ProtectionStandard_None: COPP_TVProtectionStandard = 0i32;
pub const COPP_ProtectionStandard_Reserved: COPP_TVProtectionStandard = 2147450880i32;
pub const COPP_ProtectionStandard_Unknown: COPP_TVProtectionStandard = -2147483648i32;
pub const COPP_RenegotiationRequired: COPP_StatusFlags = 2i32;
pub const COPP_StatusFlagsReserved: COPP_StatusFlags = -4i32;
pub const COPP_StatusNormal: COPP_StatusFlags = 0i32;
pub const CameraControl_Exposure: CameraControlProperty = 4i32;
pub const CameraControl_Flags_Auto: CameraControlFlags = 1i32;
pub const CameraControl_Flags_Manual: CameraControlFlags = 2i32;
pub const CameraControl_Focus: CameraControlProperty = 6i32;
pub const CameraControl_Iris: CameraControlProperty = 5i32;
pub const CameraControl_Pan: CameraControlProperty = 0i32;
pub const CameraControl_Roll: CameraControlProperty = 2i32;
pub const CameraControl_Tilt: CameraControlProperty = 1i32;
pub const CameraControl_Zoom: CameraControlProperty = 3i32;
pub const CardDataChanged: SmartCardStatusType = 3i32;
pub const CardError: SmartCardStatusType = 2i32;
pub const CardFirmwareUpgrade: SmartCardStatusType = 4i32;
pub const CardInserted: SmartCardStatusType = 0i32;
pub const CardRemoved: SmartCardStatusType = 1i32;
pub const CategoryAudio: ComponentCategory = 2i32;
pub const CategoryCaptions: ComponentCategory = 5i32;
pub const CategoryData: ComponentCategory = 7i32;
pub const CategoryNotSet: ComponentCategory = -1i32;
pub const CategoryOther: ComponentCategory = 0i32;
pub const CategorySubtitles: ComponentCategory = 4i32;
pub const CategorySuperimpose: ComponentCategory = 6i32;
pub const CategoryText: ComponentCategory = 3i32;
pub const CategoryVideo: ComponentCategory = 1i32;
pub const CompressionCaps_CanBFrame: CompressionCaps = 8i32;
pub const CompressionCaps_CanCrunch: CompressionCaps = 2i32;
pub const CompressionCaps_CanKeyFrame: CompressionCaps = 4i32;
pub const CompressionCaps_CanQuality: CompressionCaps = 1i32;
pub const CompressionCaps_CanWindow: CompressionCaps = 16i32;
pub const ConstantBitRate: VIDEOENCODER_BITRATE_MODE = 0i32;
pub const DDSFF_PROGRESSIVERENDER: DDSFF_FLAGS = 1i32;
pub const DECIMATION_DEFAULT: DECIMATION_USAGE = 4i32;
pub const DECIMATION_LEGACY: DECIMATION_USAGE = 0i32;
pub const DECIMATION_USE_DECODER_ONLY: DECIMATION_USAGE = 1i32;
pub const DECIMATION_USE_OVERLAY_ONLY: DECIMATION_USAGE = 3i32;
pub const DECIMATION_USE_VIDEOPORT_ONLY: DECIMATION_USAGE = 2i32;
pub const DECODER_CAP_NOTSUPPORTED: u32 = 0u32;
pub const DECODER_CAP_SUPPORTED: u32 = 1u32;
pub const DISPLAY_16x9: DVD_PREFERRED_DISPLAY_MODE = 1i32;
pub const DISPLAY_4x3_LETTERBOX_PREFERRED: DVD_PREFERRED_DISPLAY_MODE = 3i32;
pub const DISPLAY_4x3_PANSCAN_PREFERRED: DVD_PREFERRED_DISPLAY_MODE = 2i32;
pub const DISPLAY_CONTENT_DEFAULT: DVD_PREFERRED_DISPLAY_MODE = 0i32;
pub const DOLBY_AC3_AUDIO: MPEG2StreamType = 129i32;
pub const DOLBY_DIGITAL_PLUS_AUDIO_ATSC: MPEG2StreamType = 135i32;
pub const DVB_Cable: DVBSystemType = 0i32;
pub const DVB_Satellite: DVBSystemType = 2i32;
pub const DVB_Terrestrial: DVBSystemType = 1i32;
pub const DVDECODERRESOLUTION_180x120: _DVDECODERRESOLUTION = 1002i32;
pub const DVDECODERRESOLUTION_360x240: _DVDECODERRESOLUTION = 1001i32;
pub const DVDECODERRESOLUTION_720x480: _DVDECODERRESOLUTION = 1000i32;
pub const DVDECODERRESOLUTION_88x60: _DVDECODERRESOLUTION = 1003i32;
pub const DVD_AUDIO_CAPS_AC3: u32 = 1u32;
pub const DVD_AUDIO_CAPS_DTS: u32 = 8u32;
pub const DVD_AUDIO_CAPS_LPCM: u32 = 4u32;
pub const DVD_AUDIO_CAPS_MPEG2: u32 = 2u32;
pub const DVD_AUDIO_CAPS_SDDS: u32 = 16u32;
pub const DVD_AUD_EXT_Captions: DVD_AUDIO_LANG_EXT = 1i32;
pub const DVD_AUD_EXT_DirectorComments1: DVD_AUDIO_LANG_EXT = 3i32;
pub const DVD_AUD_EXT_DirectorComments2: DVD_AUDIO_LANG_EXT = 4i32;
pub const DVD_AUD_EXT_NotSpecified: DVD_AUDIO_LANG_EXT = 0i32;
pub const DVD_AUD_EXT_VisuallyImpaired: DVD_AUDIO_LANG_EXT = 2i32;
pub const DVD_AppMode_Karaoke: DVD_TITLE_APPMODE = 1i32;
pub const DVD_AppMode_Not_Specified: DVD_TITLE_APPMODE = 0i32;
pub const DVD_AppMode_Other: DVD_TITLE_APPMODE = 3i32;
pub const DVD_Assignment_LR: DVD_KARAOKE_ASSIGNMENT = 2i32;
pub const DVD_Assignment_LR1: DVD_KARAOKE_ASSIGNMENT = 4i32;
pub const DVD_Assignment_LR12: DVD_KARAOKE_ASSIGNMENT = 6i32;
pub const DVD_Assignment_LRM: DVD_KARAOKE_ASSIGNMENT = 3i32;
pub const DVD_Assignment_LRM1: DVD_KARAOKE_ASSIGNMENT = 5i32;
pub const DVD_Assignment_LRM12: DVD_KARAOKE_ASSIGNMENT = 7i32;
pub const DVD_Assignment_reserved0: DVD_KARAOKE_ASSIGNMENT = 0i32;
pub const DVD_Assignment_reserved1: DVD_KARAOKE_ASSIGNMENT = 1i32;
pub const DVD_AudioDuringFFwdRew: DVD_OPTION_FLAG = 4i32;
pub const DVD_AudioFormat_AC3: DVD_AUDIO_FORMAT = 0i32;
pub const DVD_AudioFormat_DTS: DVD_AUDIO_FORMAT = 6i32;
pub const DVD_AudioFormat_LPCM: DVD_AUDIO_FORMAT = 5i32;
pub const DVD_AudioFormat_MPEG1: DVD_AUDIO_FORMAT = 1i32;
pub const DVD_AudioFormat_MPEG1_DRC: DVD_AUDIO_FORMAT = 2i32;
pub const DVD_AudioFormat_MPEG2: DVD_AUDIO_FORMAT = 3i32;
pub const DVD_AudioFormat_MPEG2_DRC: DVD_AUDIO_FORMAT = 4i32;
pub const DVD_AudioFormat_Other: DVD_AUDIO_FORMAT = 8i32;
pub const DVD_AudioFormat_SDDS: DVD_AUDIO_FORMAT = 7i32;
pub const DVD_AudioMode_Karaoke: DVD_AUDIO_APPMODE = 1i32;
pub const DVD_AudioMode_None: DVD_AUDIO_APPMODE = 0i32;
pub const DVD_AudioMode_Other: DVD_AUDIO_APPMODE = 3i32;
pub const DVD_AudioMode_Surround: DVD_AUDIO_APPMODE = 2i32;
pub const DVD_CMD_FLAG_Block: DVD_CMD_FLAGS = 4i32;
pub const DVD_CMD_FLAG_EndAfterRendered: DVD_CMD_FLAGS = 16i32;
pub const DVD_CMD_FLAG_Flush: DVD_CMD_FLAGS = 1i32;
pub const DVD_CMD_FLAG_None: DVD_CMD_FLAGS = 0i32;
pub const DVD_CMD_FLAG_SendEvents: DVD_CMD_FLAGS = 2i32;
pub const DVD_CMD_FLAG_StartWhenRendered: DVD_CMD_FLAGS = 8i32;
pub const DVD_CacheSizeInMB: DVD_OPTION_FLAG = 6i32;
pub const DVD_Channel_Audio: DVD_TextStringType = 32i32;
pub const DVD_CharSet_ISO646: DVD_TextCharSet = 1i32;
pub const DVD_CharSet_ISO8859_1: DVD_TextCharSet = 3i32;
pub const DVD_CharSet_JIS_Roman_Kanji: DVD_TextCharSet = 2i32;
pub const DVD_CharSet_ShiftJIS_Kanji_Roman_Katakana: DVD_TextCharSet = 4i32;
pub const DVD_CharSet_Unicode: DVD_TextCharSet = 0i32;
pub const DVD_DEFAULT_AUDIO_STREAM: u32 = 15u32;
pub const DVD_DIR_BACKWARD: DVD_PLAY_DIRECTION = 1i32;
pub const DVD_DIR_FORWARD: DVD_PLAY_DIRECTION = 0i32;
pub const DVD_DOMAIN_FirstPlay: DVD_DOMAIN = 1i32;
pub const DVD_DOMAIN_Stop: DVD_DOMAIN = 5i32;
pub const DVD_DOMAIN_Title: DVD_DOMAIN = 4i32;
pub const DVD_DOMAIN_VideoManagerMenu: DVD_DOMAIN = 2i32;
pub const DVD_DOMAIN_VideoTitleSetMenu: DVD_DOMAIN = 3i32;
pub const DVD_DisableStillThrottle: DVD_OPTION_FLAG = 14i32;
pub const DVD_ERROR_CopyProtectFail: DVD_ERROR = 2i32;
pub const DVD_ERROR_CopyProtectOutputFail: DVD_ERROR = 9i32;
pub const DVD_ERROR_CopyProtectOutputNotSupported: DVD_ERROR = 10i32;
pub const DVD_ERROR_IncompatibleDiscAndDecoderRegions: DVD_ERROR = 8i32;
pub const DVD_ERROR_IncompatibleSystemAndDecoderRegions: DVD_ERROR = 7i32;
pub const DVD_ERROR_InvalidDVD1_0Disc: DVD_ERROR = 3i32;
pub const DVD_ERROR_InvalidDiscRegion: DVD_ERROR = 4i32;
pub const DVD_ERROR_LowParentalLevel: DVD_ERROR = 5i32;
pub const DVD_ERROR_MacrovisionFail: DVD_ERROR = 6i32;
pub const DVD_ERROR_Unexpected: DVD_ERROR = 1i32;
pub const DVD_EnableCC: DVD_OPTION_FLAG = 19i32;
pub const DVD_EnableESOutput: DVD_OPTION_FLAG = 12i32;
pub const DVD_EnableExtendedCopyProtectErrors: DVD_OPTION_FLAG = 8i32;
pub const DVD_EnableLoggingEvents: DVD_OPTION_FLAG = 15i32;
pub const DVD_EnableNonblockingAPIs: DVD_OPTION_FLAG = 5i32;
pub const DVD_EnablePortableBookmarks: DVD_OPTION_FLAG = 7i32;
pub const DVD_EnableStreaming: DVD_OPTION_FLAG = 11i32;
pub const DVD_EnableTitleLength: DVD_OPTION_FLAG = 13i32;
pub const DVD_FPS_25: DVD_FRAMERATE = 1i32;
pub const DVD_FPS_30NonDrop: DVD_FRAMERATE = 3i32;
pub const DVD_General_Comments: DVD_TextStringType = 49i32;
pub const DVD_General_Name: DVD_TextStringType = 48i32;
pub const DVD_HMSF_TimeCodeEvents: DVD_OPTION_FLAG = 3i32;
pub const DVD_IncreaseOutputControl: DVD_OPTION_FLAG = 10i32;
pub const DVD_Karaoke_GuideMelody1: DVD_KARAOKE_CONTENTS = 4i32;
pub const DVD_Karaoke_GuideMelody2: DVD_KARAOKE_CONTENTS = 8i32;
pub const DVD_Karaoke_GuideMelodyA: DVD_KARAOKE_CONTENTS = 16i32;
pub const DVD_Karaoke_GuideMelodyB: DVD_KARAOKE_CONTENTS = 32i32;
pub const DVD_Karaoke_GuideVocal1: DVD_KARAOKE_CONTENTS = 1i32;
pub const DVD_Karaoke_GuideVocal2: DVD_KARAOKE_CONTENTS = 2i32;
pub const DVD_Karaoke_SoundEffectA: DVD_KARAOKE_CONTENTS = 64i32;
pub const DVD_Karaoke_SoundEffectB: DVD_KARAOKE_CONTENTS = 128i32;
pub const DVD_MENU_Angle: DVD_MENU_ID = 6i32;
pub const DVD_MENU_Audio: DVD_MENU_ID = 5i32;
pub const DVD_MENU_Chapter: DVD_MENU_ID = 7i32;
pub const DVD_MENU_Root: DVD_MENU_ID = 3i32;
pub const DVD_MENU_Subpicture: DVD_MENU_ID = 4i32;
pub const DVD_MENU_Title: DVD_MENU_ID = 2i32;
pub const DVD_MaxReadBurstInKB: DVD_OPTION_FLAG = 16i32;
pub const DVD_Mix_0to0: DVD_KARAOKE_DOWNMIX = 1i32;
pub const DVD_Mix_0to1: DVD_KARAOKE_DOWNMIX = 256i32;
pub const DVD_Mix_1to0: DVD_KARAOKE_DOWNMIX = 2i32;
pub const DVD_Mix_1to1: DVD_KARAOKE_DOWNMIX = 512i32;
pub const DVD_Mix_2to0: DVD_KARAOKE_DOWNMIX = 4i32;
pub const DVD_Mix_2to1: DVD_KARAOKE_DOWNMIX = 1024i32;
pub const DVD_Mix_3to0: DVD_KARAOKE_DOWNMIX = 8i32;
pub const DVD_Mix_3to1: DVD_KARAOKE_DOWNMIX = 2048i32;
pub const DVD_Mix_4to0: DVD_KARAOKE_DOWNMIX = 16i32;
pub const DVD_Mix_4to1: DVD_KARAOKE_DOWNMIX = 4096i32;
pub const DVD_Mix_Lto0: DVD_KARAOKE_DOWNMIX = 32i32;
pub const DVD_Mix_Lto1: DVD_KARAOKE_DOWNMIX = 8192i32;
pub const DVD_Mix_Rto0: DVD_KARAOKE_DOWNMIX = 64i32;
pub const DVD_Mix_Rto1: DVD_KARAOKE_DOWNMIX = 16384i32;
pub const DVD_NavCmdType_Button: DVD_NavCmdType = 4i32;
pub const DVD_NavCmdType_Cell: DVD_NavCmdType = 3i32;
pub const DVD_NavCmdType_Post: DVD_NavCmdType = 2i32;
pub const DVD_NavCmdType_Pre: DVD_NavCmdType = 1i32;
pub const DVD_NotifyParentalLevelChange: DVD_OPTION_FLAG = 2i32;
pub const DVD_NotifyPositionChange: DVD_OPTION_FLAG = 9i32;
pub const DVD_Other_Cut: DVD_TextStringType = 81i32;
pub const DVD_Other_Scene: DVD_TextStringType = 80i32;
pub const DVD_Other_Take: DVD_TextStringType = 82i32;
pub const DVD_PARENTAL_LEVEL_1: DVD_PARENTAL_LEVEL = 256i32;
pub const DVD_PARENTAL_LEVEL_2: DVD_PARENTAL_LEVEL = 512i32;
pub const DVD_PARENTAL_LEVEL_3: DVD_PARENTAL_LEVEL = 1024i32;
pub const DVD_PARENTAL_LEVEL_4: DVD_PARENTAL_LEVEL = 2048i32;
pub const DVD_PARENTAL_LEVEL_5: DVD_PARENTAL_LEVEL = 4096i32;
pub const DVD_PARENTAL_LEVEL_6: DVD_PARENTAL_LEVEL = 8192i32;
pub const DVD_PARENTAL_LEVEL_7: DVD_PARENTAL_LEVEL = 16384i32;
pub const DVD_PARENTAL_LEVEL_8: DVD_PARENTAL_LEVEL = 32768i32;
pub const DVD_PB_STOPPED_CopyProtectFailure: DVD_PB_STOPPED = 13i32;
pub const DVD_PB_STOPPED_CopyProtectOutputFailure: DVD_PB_STOPPED = 14i32;
pub const DVD_PB_STOPPED_CopyProtectOutputNotSupported: DVD_PB_STOPPED = 15i32;
pub const DVD_PB_STOPPED_DiscEjected: DVD_PB_STOPPED = 5i32;
pub const DVD_PB_STOPPED_DiscReadError: DVD_PB_STOPPED = 12i32;
pub const DVD_PB_STOPPED_IllegalNavCommand: DVD_PB_STOPPED = 6i32;
pub const DVD_PB_STOPPED_MacrovisionFailure: DVD_PB_STOPPED = 11i32;
pub const DVD_PB_STOPPED_NoBranch: DVD_PB_STOPPED = 1i32;
pub const DVD_PB_STOPPED_NoFirstPlayDomain: DVD_PB_STOPPED = 2i32;
pub const DVD_PB_STOPPED_Other: DVD_PB_STOPPED = 0i32;
pub const DVD_PB_STOPPED_ParentalFailure: DVD_PB_STOPPED = 9i32;
pub const DVD_PB_STOPPED_PlayChapterAutoStop: DVD_PB_STOPPED = 8i32;
pub const DVD_PB_STOPPED_PlayPeriodAutoStop: DVD_PB_STOPPED = 7i32;
pub const DVD_PB_STOPPED_RegionFailure: DVD_PB_STOPPED = 10i32;
pub const DVD_PB_STOPPED_Reset: DVD_PB_STOPPED = 4i32;
pub const DVD_PB_STOPPED_StopCommand: DVD_PB_STOPPED = 3i32;
pub const DVD_ReadBurstPeriodInMS: DVD_OPTION_FLAG = 17i32;
pub const DVD_Relative_Left: DVD_RELATIVE_BUTTON = 3i32;
pub const DVD_Relative_Lower: DVD_RELATIVE_BUTTON = 2i32;
pub const DVD_Relative_Right: DVD_RELATIVE_BUTTON = 4i32;
pub const DVD_Relative_Upper: DVD_RELATIVE_BUTTON = 1i32;
pub const DVD_ResetOnStop: DVD_OPTION_FLAG = 1i32;
pub const DVD_RestartDisc: DVD_OPTION_FLAG = 18i32;
pub const DVD_SIDE_A: DVD_DISC_SIDE = 1i32;
pub const DVD_SIDE_B: DVD_DISC_SIDE = 2i32;
pub const DVD_SPCoding_Extended: DVD_SUBPICTURE_CODING = 1i32;
pub const DVD_SPCoding_Other: DVD_SUBPICTURE_CODING = 2i32;
pub const DVD_SPCoding_RunLength: DVD_SUBPICTURE_CODING = 0i32;
pub const DVD_SPType_Language: DVD_SUBPICTURE_TYPE = 1i32;
pub const DVD_SPType_NotSpecified: DVD_SUBPICTURE_TYPE = 0i32;
pub const DVD_SPType_Other: DVD_SUBPICTURE_TYPE = 2i32;
pub const DVD_SP_EXT_CC_Big: DVD_SUBPICTURE_LANG_EXT = 6i32;
pub const DVD_SP_EXT_CC_Children: DVD_SUBPICTURE_LANG_EXT = 7i32;
pub const DVD_SP_EXT_CC_Normal: DVD_SUBPICTURE_LANG_EXT = 5i32;
pub const DVD_SP_EXT_Caption_Big: DVD_SUBPICTURE_LANG_EXT = 2i32;
pub const DVD_SP_EXT_Caption_Children: DVD_SUBPICTURE_LANG_EXT = 3i32;
pub const DVD_SP_EXT_Caption_Normal: DVD_SUBPICTURE_LANG_EXT = 1i32;
pub const DVD_SP_EXT_DirectorComments_Big: DVD_SUBPICTURE_LANG_EXT = 14i32;
pub const DVD_SP_EXT_DirectorComments_Children: DVD_SUBPICTURE_LANG_EXT = 15i32;
pub const DVD_SP_EXT_DirectorComments_Normal: DVD_SUBPICTURE_LANG_EXT = 13i32;
pub const DVD_SP_EXT_Forced: DVD_SUBPICTURE_LANG_EXT = 9i32;
pub const DVD_SP_EXT_NotSpecified: DVD_SUBPICTURE_LANG_EXT = 0i32;
pub const DVD_STREAM_DATA_CURRENT: u32 = 2048u32;
pub const DVD_STREAM_DATA_VMGM: u32 = 1024u32;
pub const DVD_STREAM_DATA_VTSM: u32 = 1025u32;
pub const DVD_Stream_Angle: DVD_TextStringType = 18i32;
pub const DVD_Stream_Audio: DVD_TextStringType = 16i32;
pub const DVD_Stream_Subpicture: DVD_TextStringType = 17i32;
pub const DVD_Struct_Cell: DVD_TextStringType = 5i32;
pub const DVD_Struct_ParentalID: DVD_TextStringType = 3i32;
pub const DVD_Struct_PartOfTitle: DVD_TextStringType = 4i32;
pub const DVD_Struct_Title: DVD_TextStringType = 2i32;
pub const DVD_Struct_Volume: DVD_TextStringType = 1i32;
pub const DVD_TC_FLAG_25fps: DVD_TIMECODE_FLAGS = 1i32;
pub const DVD_TC_FLAG_30fps: DVD_TIMECODE_FLAGS = 2i32;
pub const DVD_TC_FLAG_DropFrame: DVD_TIMECODE_FLAGS = 4i32;
pub const DVD_TC_FLAG_Interpolated: DVD_TIMECODE_FLAGS = 8i32;
pub const DVD_TITLE_MENU: u32 = 0u32;
pub const DVD_Title_Album: DVD_TextStringType = 59i32;
pub const DVD_Title_Movie: DVD_TextStringType = 57i32;
pub const DVD_Title_Orig_Album: DVD_TextStringType = 75i32;
pub const DVD_Title_Orig_Movie: DVD_TextStringType = 73i32;
pub const DVD_Title_Orig_Other: DVD_TextStringType = 79i32;
pub const DVD_Title_Orig_Series: DVD_TextStringType = 72i32;
pub const DVD_Title_Orig_Song: DVD_TextStringType = 76i32;
pub const DVD_Title_Orig_Video: DVD_TextStringType = 74i32;
pub const DVD_Title_Other: DVD_TextStringType = 63i32;
pub const DVD_Title_Series: DVD_TextStringType = 56i32;
pub const DVD_Title_Song: DVD_TextStringType = 60i32;
pub const DVD_Title_Sub_Album: DVD_TextStringType = 67i32;
pub const DVD_Title_Sub_Movie: DVD_TextStringType = 65i32;
pub const DVD_Title_Sub_Other: DVD_TextStringType = 71i32;
pub const DVD_Title_Sub_Series: DVD_TextStringType = 64i32;
pub const DVD_Title_Sub_Song: DVD_TextStringType = 68i32;
pub const DVD_Title_Sub_Video: DVD_TextStringType = 66i32;
pub const DVD_Title_Video: DVD_TextStringType = 58i32;
pub const DVD_VideoCompression_MPEG1: DVD_VIDEO_COMPRESSION = 1i32;
pub const DVD_VideoCompression_MPEG2: DVD_VIDEO_COMPRESSION = 2i32;
pub const DVD_VideoCompression_Other: DVD_VIDEO_COMPRESSION = 0i32;
pub const DVD_WARNING_FormatNotSupported: DVD_WARNING = 2i32;
pub const DVD_WARNING_IllegalNavCommand: DVD_WARNING = 3i32;
pub const DVD_WARNING_InvalidDVD1_0Disc: DVD_WARNING = 1i32;
pub const DVD_WARNING_Open: DVD_WARNING = 4i32;
pub const DVD_WARNING_Read: DVD_WARNING = 6i32;
pub const DVD_WARNING_Seek: DVD_WARNING = 5i32;
pub const DVENCODERFORMAT_DVHD: _DVENCODERFORMAT = 2008i32;
pub const DVENCODERFORMAT_DVSD: _DVENCODERFORMAT = 2007i32;
pub const DVENCODERFORMAT_DVSL: _DVENCODERFORMAT = 2009i32;
pub const DVENCODERRESOLUTION_180x120: _DVENCODERRESOLUTION = 2014i32;
pub const DVENCODERRESOLUTION_360x240: _DVENCODERRESOLUTION = 2013i32;
pub const DVENCODERRESOLUTION_720x480: _DVENCODERRESOLUTION = 2012i32;
pub const DVENCODERRESOLUTION_88x60: _DVENCODERRESOLUTION = 2015i32;
pub const DVENCODERVIDEOFORMAT_NTSC: _DVENCODERVIDEOFORMAT = 2000i32;
pub const DVENCODERVIDEOFORMAT_PAL: _DVENCODERVIDEOFORMAT = 2001i32;
pub const DVRESOLUTION_DC: _DVRESOLUTION = 1003i32;
pub const DVRESOLUTION_FULL: _DVRESOLUTION = 1000i32;
pub const DVRESOLUTION_HALF: _DVRESOLUTION = 1001i32;
pub const DVRESOLUTION_QUARTER: _DVRESOLUTION = 1002i32;
pub const DWORD_ALLPARAMS: i32 = -1i32;
pub const DXVA2Trace_Control: windows_core::GUID = windows_core::GUID::from_u128(0xa0386e75_f70c_464c_a9ce_33c44e091623);
pub const DXVA2Trace_DecodeDevBeginFrame: windows_core::GUID = windows_core::GUID::from_u128(0x9fd1acf6_44cb_4637_bc62_2c11a9608f90);
pub const DXVA2Trace_DecodeDevCreated: windows_core::GUID = windows_core::GUID::from_u128(0xb4de17a1_c5b2_44fe_86d5_d97a648114ff);
pub const DXVA2Trace_DecodeDevDestroyed: windows_core::GUID = windows_core::GUID::from_u128(0x853ebdf2_4160_421d_8893_63dcea4f18bb);
pub const DXVA2Trace_DecodeDevEndFrame: windows_core::GUID = windows_core::GUID::from_u128(0x9fb3cb33_47dc_4899_98c8_c0c6cd7cd3cb);
pub const DXVA2Trace_DecodeDevExecute: windows_core::GUID = windows_core::GUID::from_u128(0x850aeb4c_d19a_4609_b3b4_bcbf0e22121e);
pub const DXVA2Trace_DecodeDevGetBuffer: windows_core::GUID = windows_core::GUID::from_u128(0x57b128fb_72cb_4137_a575_d91fa3160897);
pub const DXVA2Trace_VideoProcessBlt: windows_core::GUID = windows_core::GUID::from_u128(0x69089cc0_71ab_42d0_953a_2887bf05a8af);
pub const DXVA2Trace_VideoProcessDevCreated: windows_core::GUID = windows_core::GUID::from_u128(0x895508c6_540d_4c87_98f8_8dcbf2dabb2a);
pub const DXVA2Trace_VideoProcessDevDestroyed: windows_core::GUID = windows_core::GUID::from_u128(0xf97f30b1_fb49_42c7_8ee8_88bdfa92d4e2);
pub const DXVA2_DestinationFlagMask: DXVA2_DestinationFlags = -65521i32;
pub const DXVA2_DestinationFlag_Alpha_Changed: DXVA2_DestinationFlags = 8i32;
pub const DXVA2_DestinationFlag_Background_Changed: DXVA2_DestinationFlags = 1i32;
pub const DXVA2_DestinationFlag_ColorData_Changed: DXVA2_DestinationFlags = 4i32;
pub const DXVA2_DestinationFlag_RFF: DXVA2_DestinationFlags = 65536i32;
pub const DXVA2_DestinationFlag_RFF_TFF_Present: DXVA2_DestinationFlags = 262144i32;
pub const DXVA2_DestinationFlag_TFF: DXVA2_DestinationFlags = 131072i32;
pub const DXVA2_DestinationFlag_TargetRect_Changed: DXVA2_DestinationFlags = 2i32;
pub const DXVA2_SampleFlag_ColorData_Changed: DXVA2_SampleFlags = 8i32;
pub const DXVA2_SampleFlag_DstRect_Changed: DXVA2_SampleFlags = 4i32;
pub const DXVA2_SampleFlag_Palette_Changed: DXVA2_SampleFlags = 1i32;
pub const DXVA2_SampleFlag_PlanarAlpha_Changed: DXVA2_SampleFlags = 16i32;
pub const DXVA2_SampleFlag_RFF: DXVA2_SampleFlags = 65536i32;
pub const DXVA2_SampleFlag_RFF_TFF_Present: DXVA2_SampleFlags = 262144i32;
pub const DXVA2_SampleFlag_SrcRect_Changed: DXVA2_SampleFlags = 2i32;
pub const DXVA2_SampleFlag_TFF: DXVA2_SampleFlags = 131072i32;
pub const DXVA2_SampleFlagsMask: DXVA2_SampleFlags = -65505i32;
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
pub const DXVA_COPPDevice: windows_core::GUID = windows_core::GUID::from_u128(0xd2457add_8999_45ed_8a8a_d1aa047ba4d5);
pub const DXVA_COPPGetCertificateLengthFnCode: u32 = 1u32;
pub const DXVA_COPPKeyExchangeFnCode: u32 = 2u32;
pub const DXVA_COPPQueryBusData: windows_core::GUID = windows_core::GUID::from_u128(0xc6f4d673_6174_4184_8e35_f6db5200bcba);
pub const DXVA_COPPQueryConnectorType: windows_core::GUID = windows_core::GUID::from_u128(0x81d0bfd5_6afe_48c2_99c0_95a08f97c5da);
pub const DXVA_COPPQueryDisplayData: windows_core::GUID = windows_core::GUID::from_u128(0xd7bf1ba3_ad13_4f8e_af98_0dcb3ca204cc);
pub const DXVA_COPPQueryGlobalProtectionLevel: windows_core::GUID = windows_core::GUID::from_u128(0x1957210a_7766_452a_b99a_d27aed54f03a);
pub const DXVA_COPPQueryHDCPKeyData: windows_core::GUID = windows_core::GUID::from_u128(0x0db59d74_a992_492e_a0bd_c23fda564e00);
pub const DXVA_COPPQueryLocalProtectionLevel: windows_core::GUID = windows_core::GUID::from_u128(0xb2075857_3eda_4d5d_88db_748f8c1a0549);
pub const DXVA_COPPQueryProtectionType: windows_core::GUID = windows_core::GUID::from_u128(0x38f2a801_9a6c_48bb_9107_b6696e6f1797);
pub const DXVA_COPPQuerySignaling: windows_core::GUID = windows_core::GUID::from_u128(0x6629a591_3b79_4cf3_924a_11e8e7811671);
pub const DXVA_COPPQueryStatusFnCode: u32 = 5u32;
pub const DXVA_COPPSequenceStartFnCode: u32 = 3u32;
pub const DXVA_COPPSetProtectionLevel: windows_core::GUID = windows_core::GUID::from_u128(0x9bb9327c_4eb5_4727_9f00_b42b0919c0da);
pub const DXVA_COPPSetSignaling: windows_core::GUID = windows_core::GUID::from_u128(0x09a631a5_d684_4c60_8e4d_d3bb0f0be3ee);
pub const DXVA_DCCMD_SURFACE_BUFFER: u32 = 12u32;
pub const DXVA_DEBLOCKING_CONTROL_BUFFER: u32 = 4u32;
pub const DXVA_DEBLOCKING_FILTER_FUNCTION: u32 = 5u32;
pub const DXVA_DPXD_SURFACE_BUFFER: u32 = 10u32;
pub const DXVA_DeinterlaceBltExFnCode: u32 = 2u32;
pub const DXVA_DeinterlaceBltFnCode: u32 = 1u32;
pub const DXVA_DeinterlaceBobDevice: windows_core::GUID = windows_core::GUID::from_u128(0x335aa36e_7884_43a4_9c91_7f87faf3e37e);
pub const DXVA_DeinterlaceContainerDevice: windows_core::GUID = windows_core::GUID::from_u128(0x0e85cb93_3046_4ff0_aecc_d58cb5f035fd);
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
pub const DXVA_ModeAV1_VLD_12bit_Profile2: windows_core::GUID = windows_core::GUID::from_u128(0x17127009_a00f_4ce1_994e_bf4081f6f3f0);
pub const DXVA_ModeAV1_VLD_12bit_Profile2_420: windows_core::GUID = windows_core::GUID::from_u128(0x2d80bed6_9cac_4835_9e91_327bbc4f9ee8);
pub const DXVA_ModeAV1_VLD_Profile0: windows_core::GUID = windows_core::GUID::from_u128(0xb8be4ccb_cf53_46ba_8d59_d6b8a6da5d2a);
pub const DXVA_ModeAV1_VLD_Profile1: windows_core::GUID = windows_core::GUID::from_u128(0x6936ff0f_45b1_4163_9cc1_646ef6946108);
pub const DXVA_ModeAV1_VLD_Profile2: windows_core::GUID = windows_core::GUID::from_u128(0x0c5f2aa1_e541_4089_bb7b_98110a19d7c8);
pub const DXVA_ModeH261_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be01_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH261_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be02_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be03_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be04_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_C: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be05_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_D: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be06_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_E: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be07_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH263_F: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be08_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be64_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be65_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_C: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be66_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_D: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be67_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_E: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be68_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_F: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be69_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeH264_VLD_Multiview_NoFGT: windows_core::GUID = windows_core::GUID::from_u128(0x705b9d82_76cf_49d6_b7e6_ac8872db013c);
pub const DXVA_ModeH264_VLD_Stereo_NoFGT: windows_core::GUID = windows_core::GUID::from_u128(0xf9aaccbb_c2b6_4cfc_8779_5707b1760552);
pub const DXVA_ModeH264_VLD_Stereo_Progressive_NoFGT: windows_core::GUID = windows_core::GUID::from_u128(0xd79be8da_0cf1_4c81_b82a_69a4e236f43d);
pub const DXVA_ModeH264_VLD_WithFMOASO_NoFGT: windows_core::GUID = windows_core::GUID::from_u128(0xd5f04ff9_3418_45d8_9561_32a76aae2ddd);
pub const DXVA_ModeHEVC_VLD_Main: windows_core::GUID = windows_core::GUID::from_u128(0x5b11d51b_2f4c_4452_bcc3_09f2a1160cc0);
pub const DXVA_ModeHEVC_VLD_Main10: windows_core::GUID = windows_core::GUID::from_u128(0x107af0e0_ef1a_4d19_aba8_67a163073d13);
pub const DXVA_ModeMPEG1_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be09_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x6f3ec719_3735_42cc_8063_65cc3cb36616);
pub const DXVA_ModeMPEG2_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be0a_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG2_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be0b_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG2_C: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be0c_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG2_D: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be0d_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeMPEG2and1_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x86695f12_340e_4f04_9fd3_9253dd327460);
pub const DXVA_ModeMPEG4pt2_VLD_AdvSimple_GMC: windows_core::GUID = windows_core::GUID::from_u128(0xab998b5b_4258_44a9_9feb_94e597a6baae);
pub const DXVA_ModeMPEG4pt2_VLD_AdvSimple_NoGMC: windows_core::GUID = windows_core::GUID::from_u128(0xed418a9f_010d_4eda_9ae3_9a65358d8d2e);
pub const DXVA_ModeMPEG4pt2_VLD_Simple: windows_core::GUID = windows_core::GUID::from_u128(0xefd64d74_c9e8_41d7_a5e9_e9b0e39fa319);
pub const DXVA_ModeNone: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be00_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea0_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea1_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_C: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea2_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_D: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea3_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVC1_D2010: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bea4_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeVP8_VLD: windows_core::GUID = windows_core::GUID::from_u128(0x90b899ea_3a62_4705_88b3_8df04b2744e7);
pub const DXVA_ModeVP9_VLD_10bit_Profile2: windows_core::GUID = windows_core::GUID::from_u128(0xa4c749ef_6ecf_48aa_8448_50a7a1165ff7);
pub const DXVA_ModeVP9_VLD_Profile0: windows_core::GUID = windows_core::GUID::from_u128(0x463707f8_a1d0_4585_876d_83aa6d60b89e);
pub const DXVA_ModeWMV8_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be80_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeWMV8_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be81_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeWMV9_A: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be90_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeWMV9_B: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be91_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_ModeWMV9_C: windows_core::GUID = windows_core::GUID::from_u128(0x1b81be94_a0c7_11d3_b984_00c04f2e73c5);
pub const DXVA_NUM_TYPES_COMP_BUFFERS: u32 = 18u32;
pub const DXVA_NoEncrypt: windows_core::GUID = windows_core::GUID::from_u128(0x1b81bed0_a0c7_11d3_b984_00c04f2e73c5);
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
pub const DXVA_ProcAmpControlDevice: windows_core::GUID = windows_core::GUID::from_u128(0x9f200913_2ffd_4056_9f1e_e1b508f22dcf);
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
pub const DeinterlacePref9_BOB: VMR9DeinterlacePrefs = 2i32;
pub const DeinterlacePref9_Mask: VMR9DeinterlacePrefs = 7i32;
pub const DeinterlacePref9_NextBest: VMR9DeinterlacePrefs = 1i32;
pub const DeinterlacePref9_Weave: VMR9DeinterlacePrefs = 4i32;
pub const DeinterlacePref_BOB: VMRDeinterlacePrefs = 2i32;
pub const DeinterlacePref_Mask: VMRDeinterlacePrefs = 7i32;
pub const DeinterlacePref_NextBest: VMRDeinterlacePrefs = 1i32;
pub const DeinterlacePref_Weave: VMRDeinterlacePrefs = 4i32;
pub const DeinterlaceTech9_BOBLineReplicate: VMR9DeinterlaceTech = 1i32;
pub const DeinterlaceTech9_BOBVerticalStretch: VMR9DeinterlaceTech = 2i32;
pub const DeinterlaceTech9_EdgeFiltering: VMR9DeinterlaceTech = 16i32;
pub const DeinterlaceTech9_FieldAdaptive: VMR9DeinterlaceTech = 32i32;
pub const DeinterlaceTech9_MedianFiltering: VMR9DeinterlaceTech = 4i32;
pub const DeinterlaceTech9_MotionVectorSteered: VMR9DeinterlaceTech = 128i32;
pub const DeinterlaceTech9_PixelAdaptive: VMR9DeinterlaceTech = 64i32;
pub const DeinterlaceTech9_Unknown: VMR9DeinterlaceTech = 0i32;
pub const DeinterlaceTech_BOBLineReplicate: VMRDeinterlaceTech = 1i32;
pub const DeinterlaceTech_BOBVerticalStretch: VMRDeinterlaceTech = 2i32;
pub const DeinterlaceTech_EdgeFiltering: VMRDeinterlaceTech = 16i32;
pub const DeinterlaceTech_FieldAdaptive: VMRDeinterlaceTech = 32i32;
pub const DeinterlaceTech_MedianFiltering: VMRDeinterlaceTech = 4i32;
pub const DeinterlaceTech_MotionVectorSteered: VMRDeinterlaceTech = 128i32;
pub const DeinterlaceTech_PixelAdaptive: VMRDeinterlaceTech = 64i32;
pub const DeinterlaceTech_Unknown: VMRDeinterlaceTech = 0i32;
pub const DeviceClosed: UICloseReasonType = 3i32;
pub const Disabled: OUTPUT_STATE = 0i32;
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
pub const E_PROP_ID_UNSUPPORTED: windows_core::HRESULT = 0x80070490_u32 as _;
pub const E_PROP_SET_UNSUPPORTED: windows_core::HRESULT = 0x80070492_u32 as _;
pub const Entitled: EntitlementType = 0i32;
pub const ErrorClosed: UICloseReasonType = 4i32;
pub const FORMAT_DVD_LPCMAudio: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e6_db46_11cf_b4d1_00805f6cbbea);
pub const FORMAT_DolbyAC3: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e4_db46_11cf_b4d1_00805f6cbbea);
pub const FORMAT_Image: windows_core::GUID = windows_core::GUID::from_u128(0x692fa379_d3e8_4651_b5b4_0b94b013eeaf);
pub const FORMAT_JPEGImage: windows_core::GUID = windows_core::GUID::from_u128(0x692fa379_d3e8_4651_b5b4_0b94b013eeaf);
pub const FORMAT_MPEG2Audio: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e5_db46_11cf_b4d1_00805f6cbbea);
pub const FORMAT_MPEG2Video: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const FORMAT_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const FORMAT_UVCH264Video: windows_core::GUID = windows_core::GUID::from_u128(0x2017be05_6629_4248_aaed_7e1a47bc9b9c);
pub const Famine: QualityMessageType = 0i32;
pub const Flood: QualityMessageType = 1i32;
pub const GUID_TIME_MUSIC: windows_core::GUID = windows_core::GUID::from_u128(0x0574c49d_5b04_4b15_a542_ae282030117b);
pub const GUID_TIME_REFERENCE: windows_core::GUID = windows_core::GUID::from_u128(0x93ad712b_daa0_4ffe_bc81_b0ce500fcdd9);
pub const GUID_TIME_SAMPLES: windows_core::GUID = windows_core::GUID::from_u128(0xa8593d05_0c43_4984_9a63_97af9e02c4c0);
pub const HEVC_TEMPORAL_VIDEO_SUBSET: MPEG2StreamType = 37i32;
pub const HEVC_VIDEO_OR_TEMPORAL_VIDEO: MPEG2StreamType = 36i32;
pub const INTERLEAVE_CAPTURE: InterleavingMode = 1i32;
pub const INTERLEAVE_FULL: InterleavingMode = 2i32;
pub const INTERLEAVE_NONE: InterleavingMode = 0i32;
pub const INTERLEAVE_NONE_BUFFERED: InterleavingMode = 3i32;
pub const IRPM_STREAMM: MPEG2StreamType = 26i32;
pub const ISDBCAS_REQUEST_ID_EMD: ISDBCAS_REQUEST_ID = 58i32;
pub const ISDBCAS_REQUEST_ID_EMG: ISDBCAS_REQUEST_ID = 56i32;
pub const ISDB_Satellite: DVBSystemType = 4i32;
pub const ISDB_Terrestrial: DVBSystemType = 3i32;
pub const ISO_IEC_11172_2_VIDEO: MPEG2StreamType = 1i32;
pub const ISO_IEC_11172_3_AUDIO: MPEG2StreamType = 3i32;
pub const ISO_IEC_13522_MHEG: MPEG2StreamType = 7i32;
pub const ISO_IEC_13818_1_AUXILIARY: MPEG2StreamType = 14i32;
pub const ISO_IEC_13818_1_PES: MPEG2StreamType = 6i32;
pub const ISO_IEC_13818_1_PRIVATE_SECTION: MPEG2StreamType = 5i32;
pub const ISO_IEC_13818_1_RESERVED: MPEG2StreamType = 28i32;
pub const ISO_IEC_13818_2_VIDEO: MPEG2StreamType = 2i32;
pub const ISO_IEC_13818_3_AUDIO: MPEG2StreamType = 4i32;
pub const ISO_IEC_13818_6_DOWNLOAD: MPEG2StreamType = 20i32;
pub const ISO_IEC_13818_6_TYPE_A: MPEG2StreamType = 10i32;
pub const ISO_IEC_13818_6_TYPE_B: MPEG2StreamType = 11i32;
pub const ISO_IEC_13818_6_TYPE_C: MPEG2StreamType = 12i32;
pub const ISO_IEC_13818_6_TYPE_D: MPEG2StreamType = 13i32;
pub const ISO_IEC_13818_7_AUDIO: MPEG2StreamType = 15i32;
pub const ISO_IEC_14496_1_IN_PES: MPEG2StreamType = 18i32;
pub const ISO_IEC_14496_1_IN_SECTION: MPEG2StreamType = 19i32;
pub const ISO_IEC_14496_2_VISUAL: MPEG2StreamType = 16i32;
pub const ISO_IEC_14496_3_AUDIO: MPEG2StreamType = 17i32;
pub const ISO_IEC_USER_PRIVATE: MPEG2StreamType = 128i32;
pub const ITU_T_H264: MPEG2StreamType = 27i32;
pub const ITU_T_REC_H_222_1: MPEG2StreamType = 9i32;
pub const KSPROPERTY_IPSINK_ADAPTER_ADDRESS: KSPROPERTY_IPSINK = 2i32;
pub const KSPROPERTY_IPSINK_ADAPTER_DESCRIPTION: KSPROPERTY_IPSINK = 1i32;
pub const KSPROPERTY_IPSINK_MULTICASTLIST: KSPROPERTY_IPSINK = 0i32;
pub const LIBID_QuartzNetTypeLib: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b1_0ad4_11ce_b03a_0020af0ba770);
pub const LIBID_QuartzTypeLib: windows_core::GUID = windows_core::GUID::from_u128(0x56a868b0_0ad4_11ce_b03a_0020af0ba770);
pub const MAX_DEINTERLACE_DEVICE_GUIDS: u32 = 32u32;
pub const MAX_DEINTERLACE_SURFACES: u32 = 32u32;
pub const MAX_ERROR_TEXT_LEN: u32 = 160u32;
pub const MAX_FILTER_NAME: u32 = 128u32;
pub const MAX_NUMBER_OF_STREAMS: STREAMIF_CONSTANTS = 16i32;
pub const MAX_PIN_NAME: u32 = 128u32;
pub const MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140u32;
pub const MEDIASUBTYPE_ATSC_SI: windows_core::GUID = windows_core::GUID::from_u128(0xb3c7397c_d303_414d_b33c_4ed2c9d29733);
pub const MEDIASUBTYPE_DOLBY_AC3: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802c_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_DTS: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8033_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_DVB_SI: windows_core::GUID = windows_core::GUID::from_u128(0xe9dd31a3_221d_4adb_8532_9af309c1a408);
pub const MEDIASUBTYPE_DVD_LPCM_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8032_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_DVD_NAVIGATION_DSI: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8030_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_DVD_NAVIGATION_PCI: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802f_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_DVD_NAVIGATION_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8031_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_DVD_SUBPICTURE: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802d_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_ISDB_SI: windows_core::GUID = windows_core::GUID::from_u128(0xe89ad298_3601_4b06_aaec_9ddeedcc5bd0);
pub const MEDIASUBTYPE_MPEG2DATA: windows_core::GUID = windows_core::GUID::from_u128(0xc892e55b_252d_42b5_a316_d997e7a5d995);
pub const MEDIASUBTYPE_MPEG2_AUDIO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802b_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_MPEG2_PBDA_TRANSPORT_PROCESSED: windows_core::GUID = windows_core::GUID::from_u128(0xaf748dd4_0d80_11db_9705_005056c00008);
pub const MEDIASUBTYPE_MPEG2_PBDA_TRANSPORT_RAW: windows_core::GUID = windows_core::GUID::from_u128(0x0d7aed42_cb9a_11db_9705_005056c00008);
pub const MEDIASUBTYPE_MPEG2_PROGRAM: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8022_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_MPEG2_TRANSPORT: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8023_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_MPEG2_TRANSPORT_STRIDE: windows_core::GUID = windows_core::GUID::from_u128(0x138aa9a4_1ee2_4c5b_988e_19abfdbc8a11);
pub const MEDIASUBTYPE_MPEG2_UDCR_TRANSPORT: windows_core::GUID = windows_core::GUID::from_u128(0x18bec4ea_4676_450e_b478_0cd84c54b327);
pub const MEDIASUBTYPE_MPEG2_VERSIONED_TABLES: windows_core::GUID = windows_core::GUID::from_u128(0x1ed988b0_3ffc_4523_8725_347beec1a8a0);
pub const MEDIASUBTYPE_MPEG2_VIDEO: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_MPEG2_WMDRM_TRANSPORT: windows_core::GUID = windows_core::GUID::from_u128(0x18bec4ea_4676_450e_b478_0cd84c54b327);
pub const MEDIASUBTYPE_SDDS: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8034_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIASUBTYPE_TIF_SI: windows_core::GUID = windows_core::GUID::from_u128(0xec232eb2_cb96_4191_b226_0ea129f38250);
pub const MEDIATYPE_CONTROL: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8021_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIATYPE_DVD_ENCRYPTED_PACK: windows_core::GUID = windows_core::GUID::from_u128(0xed0b916a_044d_11d1_aa78_00c04fc31d60);
pub const MEDIATYPE_DVD_NAVIGATION: windows_core::GUID = windows_core::GUID::from_u128(0xe06d802e_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIATYPE_MPEG2_PACK: windows_core::GUID = windows_core::GUID::from_u128(0x36523b13_8ee5_11d1_8ca3_0060b057664a);
pub const MEDIATYPE_MPEG2_PES: windows_core::GUID = windows_core::GUID::from_u128(0xe06d8020_db46_11cf_b4d1_00805f6cbbea);
pub const MEDIATYPE_MPEG2_SECTIONS: windows_core::GUID = windows_core::GUID::from_u128(0x455f176c_4b06_47ce_9aef_8caef73df7b5);
pub const MEDIA_ELEMENTARY_STREAM: MEDIA_SAMPLE_CONTENT = 1i32;
pub const MEDIA_MPEG2_PSI: MEDIA_SAMPLE_CONTENT = 2i32;
pub const MEDIA_TRANSPORT_PACKET: MEDIA_SAMPLE_CONTENT = 0i32;
pub const MEDIA_TRANSPORT_PAYLOAD: MEDIA_SAMPLE_CONTENT = 3i32;
pub const MERIT_DO_NOT_USE: IFILTERMAPPER_MERIT = 2097152i32;
pub const MERIT_HW_COMPRESSOR: IFILTERMAPPER_MERIT = 1048656i32;
pub const MERIT_NORMAL: IFILTERMAPPER_MERIT = 6291456i32;
pub const MERIT_PREFERRED: IFILTERMAPPER_MERIT = 8388608i32;
pub const MERIT_SW_COMPRESSOR: IFILTERMAPPER_MERIT = 1048576i32;
pub const MERIT_UNLIKELY: IFILTERMAPPER_MERIT = 4194304i32;
pub const METADATA_IN_DATA_CAROUSEL: MPEG2StreamType = 23i32;
pub const METADATA_IN_DOWNLOAD_PROTOCOL: MPEG2StreamType = 25i32;
pub const METADATA_IN_OBJECT_CAROUSEL: MPEG2StreamType = 24i32;
pub const METADATA_IN_PES: MPEG2StreamType = 21i32;
pub const METADATA_IN_SECTION: MPEG2StreamType = 22i32;
pub const MIN_DIMENSION: u32 = 1u32;
pub const MMSSF_ASYNCHRONOUS: MMSSF_GET_INFORMATION_FLAGS = 4i32;
pub const MMSSF_HASCLOCK: MMSSF_GET_INFORMATION_FLAGS = 1i32;
pub const MMSSF_SUPPORTSEEK: MMSSF_GET_INFORMATION_FLAGS = 2i32;
pub const MPBOOL_FALSE: u32 = 0u32;
pub const MPBOOL_TRUE: u32 = 1u32;
pub const MPEG2_BASE: u32 = 512u32;
pub const MPEG2_E_ALREADY_INITIALIZED: windows_core::HRESULT = 0x80040201_u32 as _;
pub const MPEG2_E_BUFFER_TOO_SMALL: windows_core::HRESULT = 0x80040219_u32 as _;
pub const MPEG2_E_DATA_SOURCE_FAILED: windows_core::HRESULT = 0x80040216_u32 as _;
pub const MPEG2_E_DII_NOT_FOUND: windows_core::HRESULT = 0x80040217_u32 as _;
pub const MPEG2_E_DSHOW_PIN_NOT_FOUND: windows_core::HRESULT = 0x80040218_u32 as _;
pub const MPEG2_E_DSI_NOT_FOUND: windows_core::HRESULT = 0x8004020A_u32 as _;
pub const MPEG2_E_FILE_OFFSET_TOO_BIG: windows_core::HRESULT = 0x80040212_u32 as _;
pub const MPEG2_E_INCORRECT_DESCRIPTOR_TAG: windows_core::HRESULT = 0x8004021D_u32 as _;
pub const MPEG2_E_INVALID_CAROUSEL_ID: windows_core::HRESULT = 0x8004020C_u32 as _;
pub const MPEG2_E_INVALID_SG_OBJECT_KIND: windows_core::HRESULT = 0x8004020E_u32 as _;
pub const MPEG2_E_INVALID_UDP_PORT: windows_core::HRESULT = 0x80040215_u32 as _;
pub const MPEG2_E_MALFORMED_DSMCC_MESSAGE: windows_core::HRESULT = 0x8004020D_u32 as _;
pub const MPEG2_E_MALFORMED_TABLE: windows_core::HRESULT = 0x80040203_u32 as _;
pub const MPEG2_E_MISSING_SECTIONS: windows_core::HRESULT = 0x8004021A_u32 as _;
pub const MPEG2_E_NEXT_TABLE_OPS_NOT_AVAILABLE: windows_core::HRESULT = 0x8004021C_u32 as _;
pub const MPEG2_E_NOT_PRESENT: windows_core::HRESULT = 0x80040205_u32 as _;
pub const MPEG2_E_OBJECT_KIND_NOT_A_DIRECTORY: windows_core::HRESULT = 0x80040210_u32 as _;
pub const MPEG2_E_OBJECT_KIND_NOT_A_FILE: windows_core::HRESULT = 0x80040211_u32 as _;
pub const MPEG2_E_OBJECT_NOT_FOUND: windows_core::HRESULT = 0x8004020F_u32 as _;
pub const MPEG2_E_OUT_OF_BOUNDS: windows_core::HRESULT = 0x80040202_u32 as _;
pub const MPEG2_E_REGISTRY_ACCESS_FAILED: windows_core::HRESULT = 0x80040214_u32 as _;
pub const MPEG2_E_SECTION_NOT_FOUND: windows_core::HRESULT = 0x80040206_u32 as _;
pub const MPEG2_E_SERVER_UNAVAILABLE: windows_core::HRESULT = 0x8004020B_u32 as _;
pub const MPEG2_E_SERVICE_ID_NOT_FOUND: windows_core::HRESULT = 0x80040208_u32 as _;
pub const MPEG2_E_SERVICE_PMT_NOT_FOUND: windows_core::HRESULT = 0x80040209_u32 as _;
pub const MPEG2_E_STREAM_STOPPED: windows_core::HRESULT = 0x80040213_u32 as _;
pub const MPEG2_E_TOO_MANY_SECTIONS: windows_core::HRESULT = 0x8004021B_u32 as _;
pub const MPEG2_E_TX_STREAM_UNAVAILABLE: windows_core::HRESULT = 0x80040207_u32 as _;
pub const MPEG2_E_UNDEFINED: windows_core::HRESULT = 0x80040204_u32 as _;
pub const MPEG2_E_UNINITIALIZED: windows_core::HRESULT = 0x80040200_u32 as _;
pub const MPEG2_PROGRAM_DIRECTORY_PES_PACKET: u32 = 2u32;
pub const MPEG2_PROGRAM_ELEMENTARY_STREAM: u32 = 1u32;
pub const MPEG2_PROGRAM_PACK_HEADER: u32 = 3u32;
pub const MPEG2_PROGRAM_PES_STREAM: u32 = 4u32;
pub const MPEG2_PROGRAM_STREAM_MAP: u32 = 0u32;
pub const MPEG2_PROGRAM_SYSTEM_HEADER: u32 = 5u32;
pub const MPEG2_S_MORE_DATA_AVAILABLE: windows_core::HRESULT = 0x40200_u32 as _;
pub const MPEG2_S_MPE_INFO_FOUND: windows_core::HRESULT = 0x40204_u32 as _;
pub const MPEG2_S_MPE_INFO_NOT_FOUND: windows_core::HRESULT = 0x40205_u32 as _;
pub const MPEG2_S_NEW_MODULE_VERSION: windows_core::HRESULT = 0x40206_u32 as _;
pub const MPEG2_S_NO_MORE_DATA_AVAILABLE: windows_core::HRESULT = 0x40201_u32 as _;
pub const MPEG2_S_SG_INFO_FOUND: windows_core::HRESULT = 0x40202_u32 as _;
pub const MPEG2_S_SG_INFO_NOT_FOUND: windows_core::HRESULT = 0x40203_u32 as _;
pub const MPEGLAYER3_FLAG_PADDING_ISO: MPEGLAYER3WAVEFORMAT_FLAGS = 0u32;
pub const MPEGLAYER3_FLAG_PADDING_OFF: MPEGLAYER3WAVEFORMAT_FLAGS = 2u32;
pub const MPEGLAYER3_FLAG_PADDING_ON: MPEGLAYER3WAVEFORMAT_FLAGS = 1u32;
pub const MPF_ENVLP_BEGIN_CURRENTVAL: u32 = 1u32;
pub const MPF_ENVLP_BEGIN_NEUTRALVAL: u32 = 2u32;
pub const MPF_ENVLP_STANDARD: u32 = 0u32;
pub const MPF_PUNCHIN_NOW: u32 = 1u32;
pub const MPF_PUNCHIN_REFTIME: u32 = 0u32;
pub const MPF_PUNCHIN_STOPPED: u32 = 2u32;
pub const MPT_BOOL: MP_TYPE = 2i32;
pub const MPT_ENUM: MP_TYPE = 3i32;
pub const MPT_FLOAT: MP_TYPE = 1i32;
pub const MPT_INT: MP_TYPE = 0i32;
pub const MPT_MAX: MP_TYPE = 4i32;
pub const MP_CURVE_INVSQUARE: MP_CURVE_TYPE = 8i32;
pub const MP_CURVE_JUMP: MP_CURVE_TYPE = 1i32;
pub const MP_CURVE_LINEAR: MP_CURVE_TYPE = 2i32;
pub const MP_CURVE_SINE: MP_CURVE_TYPE = 16i32;
pub const MP_CURVE_SQUARE: MP_CURVE_TYPE = 4i32;
pub const MSDRI_S_MMI_PENDING: windows_core::HRESULT = 0x2_u32 as _;
pub const MSDRI_S_PENDING: windows_core::HRESULT = 0x1_u32 as _;
pub const MSPID_PrimaryAudio: windows_core::GUID = windows_core::GUID::from_u128(0xa35ff56b_9fda_11d0_8fdf_00c04fd9189d);
pub const MSPID_PrimaryVideo: windows_core::GUID = windows_core::GUID::from_u128(0xa35ff56a_9fda_11d0_8fdf_00c04fd9189d);
pub const MSTapeDeviceGUID: windows_core::GUID = windows_core::GUID::from_u128(0x8c0f6af2_0edb_44c1_8aeb_59040bd830ed);
pub const MixerPref9_ARAdjustXorY: VMR9MixerPrefs = 4i32;
pub const MixerPref9_AnisotropicFiltering: VMR9MixerPrefs = 64i32;
pub const MixerPref9_BiLinearFiltering: VMR9MixerPrefs = 16i32;
pub const MixerPref9_DecimateMask: VMR9MixerPrefs = 15i32;
pub const MixerPref9_DecimateOutput: VMR9MixerPrefs = 2i32;
pub const MixerPref9_DynamicDecimateBy2: VMR9MixerPrefs = 2097152i32;
pub const MixerPref9_DynamicMask: VMR9MixerPrefs = 15728640i32;
pub const MixerPref9_DynamicReserved: VMR9MixerPrefs = 12582912i32;
pub const MixerPref9_DynamicSwitchToBOB: VMR9MixerPrefs = 1048576i32;
pub const MixerPref9_FilteringMask: VMR9MixerPrefs = 4080i32;
pub const MixerPref9_FilteringReserved: VMR9MixerPrefs = 3584i32;
pub const MixerPref9_GaussianQuadFiltering: VMR9MixerPrefs = 256i32;
pub const MixerPref9_NoDecimation: VMR9MixerPrefs = 1i32;
pub const MixerPref9_NonSquareMixing: VMR9MixerPrefs = 8i32;
pub const MixerPref9_PointFiltering: VMR9MixerPrefs = 32i32;
pub const MixerPref9_PyramidalQuadFiltering: VMR9MixerPrefs = 128i32;
pub const MixerPref9_RenderTargetMask: VMR9MixerPrefs = 1044480i32;
pub const MixerPref9_RenderTargetRGB: VMR9MixerPrefs = 4096i32;
pub const MixerPref9_RenderTargetReserved: VMR9MixerPrefs = 1032192i32;
pub const MixerPref9_RenderTargetYUV: VMR9MixerPrefs = 8192i32;
pub const MixerPref_ARAdjustXorY: VMRMixerPrefs = 4i32;
pub const MixerPref_BiLinearFiltering: VMRMixerPrefs = 16i32;
pub const MixerPref_DecimateMask: VMRMixerPrefs = 15i32;
pub const MixerPref_DecimateOutput: VMRMixerPrefs = 2i32;
pub const MixerPref_DecimationReserved: VMRMixerPrefs = 8i32;
pub const MixerPref_DynamicDecimateBy2: VMRMixerPrefs = 131072i32;
pub const MixerPref_DynamicMask: VMRMixerPrefs = 983040i32;
pub const MixerPref_DynamicReserved: VMRMixerPrefs = 786432i32;
pub const MixerPref_DynamicSwitchToBOB: VMRMixerPrefs = 65536i32;
pub const MixerPref_FilteringMask: VMRMixerPrefs = 240i32;
pub const MixerPref_NoDecimation: VMRMixerPrefs = 1i32;
pub const MixerPref_PointFiltering: VMRMixerPrefs = 32i32;
pub const MixerPref_RenderTargetMask: VMRMixerPrefs = 65280i32;
pub const MixerPref_RenderTargetRGB: VMRMixerPrefs = 256i32;
pub const MixerPref_RenderTargetReserved: VMRMixerPrefs = 57344i32;
pub const MixerPref_RenderTargetYUV: VMRMixerPrefs = 4096i32;
pub const MixerPref_RenderTargetYUV420: VMRMixerPrefs = 512i32;
pub const MixerPref_RenderTargetYUV422: VMRMixerPrefs = 1024i32;
pub const MixerPref_RenderTargetYUV444: VMRMixerPrefs = 2048i32;
pub const NotAssociated: SmartCardAssociationType = 0i32;
pub const NotEntitled: EntitlementType = 1i32;
pub const NotReady: UICloseReasonType = 0i32;
pub const OAFALSE: OA_BOOL = 0i32;
pub const OATRUE: OA_BOOL = -1i32;
pub const PBDA_AUX_CONNECTOR_TYPE_Composite: windows_core::GUID = windows_core::GUID::from_u128(0xf6298b4c_c725_4d42_849b_410bbb14ea62);
pub const PBDA_AUX_CONNECTOR_TYPE_SVideo: windows_core::GUID = windows_core::GUID::from_u128(0xa0e905f4_24c9_4a54_b761_213355efc13a);
pub const PBDA_Encoder_Audio_AlgorithmType_AC3: u32 = 1u32;
pub const PBDA_Encoder_Audio_AlgorithmType_MPEG1LayerII: u32 = 0u32;
pub const PBDA_Encoder_BitrateMode_Average: u32 = 3u32;
pub const PBDA_Encoder_BitrateMode_Constant: u32 = 1u32;
pub const PBDA_Encoder_BitrateMode_Variable: u32 = 2u32;
pub const PBDA_Encoder_Video_AVC: u32 = 1u32;
pub const PBDA_Encoder_Video_H264: u32 = 1u32;
pub const PBDA_Encoder_Video_MPEG2PartII: u32 = 0u32;
pub const PBDA_Encoder_Video_MPEG4Part10: u32 = 1u32;
pub const PID_ELEMENTARY_STREAM: MUX_PID_TYPE = 0i32;
pub const PID_MPEG2_SECTION_PSI_SI: MUX_PID_TYPE = 1i32;
pub const PID_OTHER: MUX_PID_TYPE = -1i32;
pub const PINDIR_INPUT: PIN_DIRECTION = 0i32;
pub const PINDIR_OUTPUT: PIN_DIRECTION = 1i32;
pub const PhysConn_Audio_1394: PhysicalConnectorType = 4103i32;
pub const PhysConn_Audio_AESDigital: PhysicalConnectorType = 4099i32;
pub const PhysConn_Audio_AUX: PhysicalConnectorType = 4102i32;
pub const PhysConn_Audio_AudioDecoder: PhysicalConnectorType = 4105i32;
pub const PhysConn_Audio_Line: PhysicalConnectorType = 4097i32;
pub const PhysConn_Audio_Mic: PhysicalConnectorType = 4098i32;
pub const PhysConn_Audio_SCSI: PhysicalConnectorType = 4101i32;
pub const PhysConn_Audio_SPDIFDigital: PhysicalConnectorType = 4100i32;
pub const PhysConn_Audio_Tuner: PhysicalConnectorType = 4096i32;
pub const PhysConn_Audio_USB: PhysicalConnectorType = 4104i32;
pub const PhysConn_Video_1394: PhysicalConnectorType = 10i32;
pub const PhysConn_Video_AUX: PhysicalConnectorType = 9i32;
pub const PhysConn_Video_Black: PhysicalConnectorType = 15i32;
pub const PhysConn_Video_Composite: PhysicalConnectorType = 2i32;
pub const PhysConn_Video_ParallelDigital: PhysicalConnectorType = 7i32;
pub const PhysConn_Video_RGB: PhysicalConnectorType = 4i32;
pub const PhysConn_Video_SCART: PhysicalConnectorType = 14i32;
pub const PhysConn_Video_SCSI: PhysicalConnectorType = 8i32;
pub const PhysConn_Video_SVideo: PhysicalConnectorType = 3i32;
pub const PhysConn_Video_SerialDigital: PhysicalConnectorType = 6i32;
pub const PhysConn_Video_Tuner: PhysicalConnectorType = 1i32;
pub const PhysConn_Video_USB: PhysicalConnectorType = 11i32;
pub const PhysConn_Video_VideoDecoder: PhysicalConnectorType = 12i32;
pub const PhysConn_Video_VideoEncoder: PhysicalConnectorType = 13i32;
pub const PhysConn_Video_YRYBY: PhysicalConnectorType = 5i32;
pub const ProcAmpControl9_Brightness: VMR9ProcAmpControlFlags = 1i32;
pub const ProcAmpControl9_Contrast: VMR9ProcAmpControlFlags = 2i32;
pub const ProcAmpControl9_Hue: VMR9ProcAmpControlFlags = 4i32;
pub const ProcAmpControl9_Mask: VMR9ProcAmpControlFlags = 15i32;
pub const ProcAmpControl9_Saturation: VMR9ProcAmpControlFlags = 8i32;
pub const REG_PINFLAG_B_MANY: REG_PINFLAG = 4i32;
pub const REG_PINFLAG_B_OUTPUT: REG_PINFLAG = 8i32;
pub const REG_PINFLAG_B_RENDERER: REG_PINFLAG = 2i32;
pub const REG_PINFLAG_B_ZERO: REG_PINFLAG = 1i32;
pub const REMFILTERF_LEAVECONNECTED: _REM_FILTER_FLAGS = 1i32;
pub const ReadData: OUTPUT_STATE = 1i32;
pub const RenderData: OUTPUT_STATE = 2i32;
pub const RenderPrefs9_DoNotRenderBorder: VMR9RenderPrefs = 1i32;
pub const RenderPrefs9_Mask: VMR9RenderPrefs = 1i32;
pub const RenderPrefs_AllowOffscreen: VMRRenderPrefs = 0i32;
pub const RenderPrefs_AllowOverlays: VMRRenderPrefs = 0i32;
pub const RenderPrefs_DoNotRenderColorKeyAndBorder: VMRRenderPrefs = 8i32;
pub const RenderPrefs_ForceOffscreen: VMRRenderPrefs = 1i32;
pub const RenderPrefs_ForceOverlays: VMRRenderPrefs = 2i32;
pub const RenderPrefs_Mask: VMRRenderPrefs = 63i32;
pub const RenderPrefs_PreferAGPMemWhenMixing: VMRRenderPrefs = 32i32;
pub const RenderPrefs_Reserved: VMRRenderPrefs = 16i32;
pub const RenderPrefs_RestrictToInitialMonitor: VMRRenderPrefs = 0i32;
pub const Reserved1: MPEG2StreamType = 0i32;
pub const SCTE28_ConditionalAccess: ApplicationTypeType = 0i32;
pub const SCTE28_CopyProtection: ApplicationTypeType = 5i32;
pub const SCTE28_Diagnostic: ApplicationTypeType = 6i32;
pub const SCTE28_IPService: ApplicationTypeType = 2i32;
pub const SCTE28_NetworkInterface_SCTE55_1: ApplicationTypeType = 4i32;
pub const SCTE28_NetworkInterface_SCTE55_2: ApplicationTypeType = 3i32;
pub const SCTE28_POD_Host_Binding_Information: ApplicationTypeType = 1i32;
pub const SCTE28_Reserved: ApplicationTypeType = 8i32;
pub const SCTE28_Undesignated: ApplicationTypeType = 7i32;
pub const SCTE_18: LocationCodeSchemeType = 0i32;
pub const SNDDEV_ERROR_AddBuffer: SNDDEV_ERR = 13i32;
pub const SNDDEV_ERROR_Close: SNDDEV_ERR = 2i32;
pub const SNDDEV_ERROR_GetCaps: SNDDEV_ERR = 3i32;
pub const SNDDEV_ERROR_GetPosition: SNDDEV_ERR = 8i32;
pub const SNDDEV_ERROR_Open: SNDDEV_ERR = 1i32;
pub const SNDDEV_ERROR_Pause: SNDDEV_ERR = 10i32;
pub const SNDDEV_ERROR_PrepareHeader: SNDDEV_ERR = 4i32;
pub const SNDDEV_ERROR_Query: SNDDEV_ERR = 14i32;
pub const SNDDEV_ERROR_Reset: SNDDEV_ERR = 6i32;
pub const SNDDEV_ERROR_Restart: SNDDEV_ERR = 7i32;
pub const SNDDEV_ERROR_Start: SNDDEV_ERR = 12i32;
pub const SNDDEV_ERROR_Stop: SNDDEV_ERR = 11i32;
pub const SNDDEV_ERROR_UnprepareHeader: SNDDEV_ERR = 5i32;
pub const SNDDEV_ERROR_Write: SNDDEV_ERR = 9i32;
pub const SPECIFYPAGES_STATISTICS: windows_core::GUID = windows_core::GUID::from_u128(0x4c437b92_6e9e_11d1_a704_006097c4e476);
pub const SSUPDATE_ASYNC: SSUPDATE_TYPE = 1i32;
pub const SSUPDATE_CONTINUOUS: SSUPDATE_TYPE = 2i32;
pub const STDINDEXSIZE: u32 = 16384u32;
pub const STREAMSTATE_RUN: STREAM_STATE = 1i32;
pub const STREAMSTATE_STOP: STREAM_STATE = 0i32;
pub const STREAMTYPE_READ: STREAM_TYPE = 0i32;
pub const STREAMTYPE_TRANSFORM: STREAM_TYPE = 2i32;
pub const STREAMTYPE_WRITE: STREAM_TYPE = 1i32;
pub const SUBSTREAM_FILTER_VAL_NONE: u32 = 268435456u32;
pub const ScanModulationTypesMask_DVBC: ScanModulationTypes = 75i32;
pub const ScanModulationTypesMask_MCE_All_TV: ScanModulationTypes = -1i32;
pub const ScanModulationTypesMask_MCE_AnalogTv: ScanModulationTypes = 28i32;
pub const ScanModulationTypesMask_MCE_DigitalCable: ScanModulationTypes = 11i32;
pub const ScanModulationTypesMask_MCE_TerrestrialATSC: ScanModulationTypes = 23i32;
pub const State_Paused: FILTER_STATE = 1i32;
pub const State_Running: FILTER_STATE = 2i32;
pub const State_Stopped: FILTER_STATE = 0i32;
pub const StatusActive: ComponentStatus = 0i32;
pub const StatusInactive: ComponentStatus = 1i32;
pub const StatusUnavailable: ComponentStatus = 2i32;
pub const SystemClosed: UICloseReasonType = 2i32;
pub const TIMECODE_RATE_30DROP: u32 = 0u32;
pub const TIMECODE_SMPTE_BINARY_GROUP: u32 = 7u32;
pub const TIMECODE_SMPTE_COLOR_FRAME: u32 = 8u32;
pub const TechnicalFailure: EntitlementType = 2i32;
pub const TunerInputAntenna: TunerInputType = 1i32;
pub const TunerInputCable: TunerInputType = 0i32;
pub const UOP_FLAG_Pause_On: VALID_UOP_FLAG = 524288i32;
pub const UOP_FLAG_PlayNext_Chapter: VALID_UOP_FLAG = 128i32;
pub const UOP_FLAG_PlayPrev_Or_Replay_Chapter: VALID_UOP_FLAG = 64i32;
pub const UOP_FLAG_Play_Backwards: VALID_UOP_FLAG = 512i32;
pub const UOP_FLAG_Play_Chapter: VALID_UOP_FLAG = 2i32;
pub const UOP_FLAG_Play_Chapter_Or_AtTime: VALID_UOP_FLAG = 32i32;
pub const UOP_FLAG_Play_Forwards: VALID_UOP_FLAG = 256i32;
pub const UOP_FLAG_Play_Title: VALID_UOP_FLAG = 4i32;
pub const UOP_FLAG_Play_Title_Or_AtTime: VALID_UOP_FLAG = 1i32;
pub const UOP_FLAG_Resume: VALID_UOP_FLAG = 65536i32;
pub const UOP_FLAG_ReturnFromSubMenu: VALID_UOP_FLAG = 16i32;
pub const UOP_FLAG_Select_Angle: VALID_UOP_FLAG = 4194304i32;
pub const UOP_FLAG_Select_Audio_Stream: VALID_UOP_FLAG = 1048576i32;
pub const UOP_FLAG_Select_Karaoke_Audio_Presentation_Mode: VALID_UOP_FLAG = 8388608i32;
pub const UOP_FLAG_Select_Or_Activate_Button: VALID_UOP_FLAG = 131072i32;
pub const UOP_FLAG_Select_SubPic_Stream: VALID_UOP_FLAG = 2097152i32;
pub const UOP_FLAG_Select_Video_Mode_Preference: VALID_UOP_FLAG = 16777216i32;
pub const UOP_FLAG_ShowMenu_Angle: VALID_UOP_FLAG = 16384i32;
pub const UOP_FLAG_ShowMenu_Audio: VALID_UOP_FLAG = 8192i32;
pub const UOP_FLAG_ShowMenu_Chapter: VALID_UOP_FLAG = 32768i32;
pub const UOP_FLAG_ShowMenu_Root: VALID_UOP_FLAG = 2048i32;
pub const UOP_FLAG_ShowMenu_SubPic: VALID_UOP_FLAG = 4096i32;
pub const UOP_FLAG_ShowMenu_Title: VALID_UOP_FLAG = 1024i32;
pub const UOP_FLAG_Still_Off: VALID_UOP_FLAG = 262144i32;
pub const UOP_FLAG_Stop: VALID_UOP_FLAG = 8i32;
pub const USER_PRIVATE: MPEG2StreamType = 16i32;
pub const UserClosed: UICloseReasonType = 1i32;
pub const VFW_E_ADVISE_ALREADY_SET: windows_core::HRESULT = 0x80040236_u32 as _;
pub const VFW_E_ALREADY_CANCELLED: windows_core::HRESULT = 0x80040234_u32 as _;
pub const VFW_E_ALREADY_COMMITTED: windows_core::HRESULT = 0x8004020F_u32 as _;
pub const VFW_E_ALREADY_CONNECTED: windows_core::HRESULT = 0x80040204_u32 as _;
pub const VFW_E_BADALIGN: windows_core::HRESULT = 0x8004020E_u32 as _;
pub const VFW_E_BAD_KEY: windows_core::HRESULT = 0x800403F2_u32 as _;
pub const VFW_E_BAD_VIDEOCD: windows_core::HRESULT = 0x80040269_u32 as _;
pub const VFW_E_BUFFERS_OUTSTANDING: windows_core::HRESULT = 0x80040210_u32 as _;
pub const VFW_E_BUFFER_NOTSET: windows_core::HRESULT = 0x8004020C_u32 as _;
pub const VFW_E_BUFFER_OVERFLOW: windows_core::HRESULT = 0x8004020D_u32 as _;
pub const VFW_E_BUFFER_UNDERFLOW: windows_core::HRESULT = 0x80040264_u32 as _;
pub const VFW_E_CANNOT_CONNECT: windows_core::HRESULT = 0x80040217_u32 as _;
pub const VFW_E_CANNOT_LOAD_SOURCE_FILTER: windows_core::HRESULT = 0x80040241_u32 as _;
pub const VFW_E_CANNOT_RENDER: windows_core::HRESULT = 0x80040218_u32 as _;
pub const VFW_E_CERTIFICATION_FAILURE: windows_core::HRESULT = 0x80040295_u32 as _;
pub const VFW_E_CHANGING_FORMAT: windows_core::HRESULT = 0x80040219_u32 as _;
pub const VFW_E_CIRCULAR_GRAPH: windows_core::HRESULT = 0x80040231_u32 as _;
pub const VFW_E_CODECAPI_ENUMERATED: windows_core::HRESULT = 0x80040311_u32 as _;
pub const VFW_E_CODECAPI_LINEAR_RANGE: windows_core::HRESULT = 0x80040310_u32 as _;
pub const VFW_E_CODECAPI_NO_CURRENT_VALUE: windows_core::HRESULT = 0x80040314_u32 as _;
pub const VFW_E_CODECAPI_NO_DEFAULT: windows_core::HRESULT = 0x80040313_u32 as _;
pub const VFW_E_COLOR_KEY_SET: windows_core::HRESULT = 0x8004021E_u32 as _;
pub const VFW_E_COPYPROT_FAILED: windows_core::HRESULT = 0x8004027D_u32 as _;
pub const VFW_E_CORRUPT_GRAPH_FILE: windows_core::HRESULT = 0x80040235_u32 as _;
pub const VFW_E_DDRAW_CAPS_NOT_SUITABLE: windows_core::HRESULT = 0x80040273_u32 as _;
pub const VFW_E_DDRAW_VERSION_NOT_SUITABLE: windows_core::HRESULT = 0x8004027C_u32 as _;
pub const VFW_E_DUPLICATE_NAME: windows_core::HRESULT = 0x8004022D_u32 as _;
pub const VFW_E_DVD_CHAPTER_DOES_NOT_EXIST: windows_core::HRESULT = 0x80040315_u32 as _;
pub const VFW_E_DVD_CMD_CANCELLED: windows_core::HRESULT = 0x80040283_u32 as _;
pub const VFW_E_DVD_DECNOTENOUGH: windows_core::HRESULT = 0x8004027B_u32 as _;
pub const VFW_E_DVD_GRAPHNOTREADY: windows_core::HRESULT = 0x80040279_u32 as _;
pub const VFW_E_DVD_INCOMPATIBLE_REGION: windows_core::HRESULT = 0x80040287_u32 as _;
pub const VFW_E_DVD_INVALIDDOMAIN: windows_core::HRESULT = 0x80040277_u32 as _;
pub const VFW_E_DVD_INVALID_DISC: windows_core::HRESULT = 0x80040291_u32 as _;
pub const VFW_E_DVD_LOW_PARENTAL_LEVEL: windows_core::HRESULT = 0x8004028A_u32 as _;
pub const VFW_E_DVD_MENU_DOES_NOT_EXIST: windows_core::HRESULT = 0x80040282_u32 as _;
pub const VFW_E_DVD_NONBLOCKING: windows_core::HRESULT = 0x8004029C_u32 as _;
pub const VFW_E_DVD_NON_EVR_RENDERER_IN_FILTER_GRAPH: windows_core::HRESULT = 0x8004029E_u32 as _;
pub const VFW_E_DVD_NOT_IN_KARAOKE_MODE: windows_core::HRESULT = 0x8004028B_u32 as _;
pub const VFW_E_DVD_NO_ATTRIBUTES: windows_core::HRESULT = 0x80040288_u32 as _;
pub const VFW_E_DVD_NO_BUTTON: windows_core::HRESULT = 0x80040278_u32 as _;
pub const VFW_E_DVD_NO_GOUP_PGC: windows_core::HRESULT = 0x80040289_u32 as _;
pub const VFW_E_DVD_NO_RESUME_INFORMATION: windows_core::HRESULT = 0x80040292_u32 as _;
pub const VFW_E_DVD_OPERATION_INHIBITED: windows_core::HRESULT = 0x80040276_u32 as _;
pub const VFW_E_DVD_RENDERFAIL: windows_core::HRESULT = 0x8004027A_u32 as _;
pub const VFW_E_DVD_RESOLUTION_ERROR: windows_core::HRESULT = 0x8004029F_u32 as _;
pub const VFW_E_DVD_STATE_CORRUPT: windows_core::HRESULT = 0x80040285_u32 as _;
pub const VFW_E_DVD_STATE_WRONG_DISC: windows_core::HRESULT = 0x80040286_u32 as _;
pub const VFW_E_DVD_STATE_WRONG_VERSION: windows_core::HRESULT = 0x80040284_u32 as _;
pub const VFW_E_DVD_STREAM_DISABLED: windows_core::HRESULT = 0x8004028F_u32 as _;
pub const VFW_E_DVD_TITLE_UNKNOWN: windows_core::HRESULT = 0x80040290_u32 as _;
pub const VFW_E_DVD_TOO_MANY_RENDERERS_IN_FILTER_GRAPH: windows_core::HRESULT = 0x8004029D_u32 as _;
pub const VFW_E_DVD_VMR9_INCOMPATIBLEDEC: windows_core::HRESULT = 0x8004029A_u32 as _;
pub const VFW_E_DVD_WRONG_SPEED: windows_core::HRESULT = 0x80040281_u32 as _;
pub const VFW_E_ENUM_OUT_OF_RANGE: windows_core::HRESULT = 0x80040230_u32 as _;
pub const VFW_E_ENUM_OUT_OF_SYNC: windows_core::HRESULT = 0x80040203_u32 as _;
pub const VFW_E_FILE_TOO_SHORT: windows_core::HRESULT = 0x80040243_u32 as _;
pub const VFW_E_FILTER_ACTIVE: windows_core::HRESULT = 0x80040205_u32 as _;
pub const VFW_E_FRAME_STEP_UNSUPPORTED: windows_core::HRESULT = 0x8004028E_u32 as _;
pub const VFW_E_INVALIDMEDIATYPE: windows_core::HRESULT = 0x80040200_u32 as _;
pub const VFW_E_INVALIDSUBTYPE: windows_core::HRESULT = 0x80040201_u32 as _;
pub const VFW_E_INVALID_CLSID: windows_core::HRESULT = 0x80040247_u32 as _;
pub const VFW_E_INVALID_DIRECTION: windows_core::HRESULT = 0x80040208_u32 as _;
pub const VFW_E_INVALID_FILE_FORMAT: windows_core::HRESULT = 0x8004022F_u32 as _;
pub const VFW_E_INVALID_FILE_VERSION: windows_core::HRESULT = 0x80040244_u32 as _;
pub const VFW_E_INVALID_MEDIA_TYPE: windows_core::HRESULT = 0x80040248_u32 as _;
pub const VFW_E_INVALID_RECT: windows_core::HRESULT = 0x80040229_u32 as _;
pub const VFW_E_IN_FULLSCREEN_MODE: windows_core::HRESULT = 0x8004023B_u32 as _;
pub const VFW_E_MEDIA_TIME_NOT_SET: windows_core::HRESULT = 0x80040251_u32 as _;
pub const VFW_E_MONO_AUDIO_HW: windows_core::HRESULT = 0x80040253_u32 as _;
pub const VFW_E_MPEG_NOT_CONSTRAINED: windows_core::HRESULT = 0x8004025E_u32 as _;
pub const VFW_E_NEED_OWNER: windows_core::HRESULT = 0x80040202_u32 as _;
pub const VFW_E_NOT_ALLOWED_TO_SAVE: windows_core::HRESULT = 0x80040232_u32 as _;
pub const VFW_E_NOT_COMMITTED: windows_core::HRESULT = 0x80040211_u32 as _;
pub const VFW_E_NOT_CONNECTED: windows_core::HRESULT = 0x80040209_u32 as _;
pub const VFW_E_NOT_FOUND: windows_core::HRESULT = 0x80040216_u32 as _;
pub const VFW_E_NOT_IN_GRAPH: windows_core::HRESULT = 0x8004025F_u32 as _;
pub const VFW_E_NOT_OVERLAY_CONNECTION: windows_core::HRESULT = 0x8004021B_u32 as _;
pub const VFW_E_NOT_PAUSED: windows_core::HRESULT = 0x80040225_u32 as _;
pub const VFW_E_NOT_RUNNING: windows_core::HRESULT = 0x80040226_u32 as _;
pub const VFW_E_NOT_SAMPLE_CONNECTION: windows_core::HRESULT = 0x8004021C_u32 as _;
pub const VFW_E_NOT_STOPPED: windows_core::HRESULT = 0x80040224_u32 as _;
pub const VFW_E_NO_ACCEPTABLE_TYPES: windows_core::HRESULT = 0x80040207_u32 as _;
pub const VFW_E_NO_ADVISE_SET: windows_core::HRESULT = 0x80040239_u32 as _;
pub const VFW_E_NO_ALLOCATOR: windows_core::HRESULT = 0x8004020A_u32 as _;
pub const VFW_E_NO_AUDIO_HARDWARE: windows_core::HRESULT = 0x80040256_u32 as _;
pub const VFW_E_NO_CAPTURE_HARDWARE: windows_core::HRESULT = 0x80040275_u32 as _;
pub const VFW_E_NO_CLOCK: windows_core::HRESULT = 0x80040213_u32 as _;
pub const VFW_E_NO_COLOR_KEY_FOUND: windows_core::HRESULT = 0x8004021F_u32 as _;
pub const VFW_E_NO_COLOR_KEY_SET: windows_core::HRESULT = 0x8004021A_u32 as _;
pub const VFW_E_NO_COPP_HW: windows_core::HRESULT = 0x8004029B_u32 as _;
pub const VFW_E_NO_DECOMPRESSOR: windows_core::HRESULT = 0x80040255_u32 as _;
pub const VFW_E_NO_DISPLAY_PALETTE: windows_core::HRESULT = 0x80040221_u32 as _;
pub const VFW_E_NO_FULLSCREEN: windows_core::HRESULT = 0x8004023A_u32 as _;
pub const VFW_E_NO_INTERFACE: windows_core::HRESULT = 0x80040215_u32 as _;
pub const VFW_E_NO_MODEX_AVAILABLE: windows_core::HRESULT = 0x80040238_u32 as _;
pub const VFW_E_NO_PALETTE_AVAILABLE: windows_core::HRESULT = 0x80040220_u32 as _;
pub const VFW_E_NO_SINK: windows_core::HRESULT = 0x80040214_u32 as _;
pub const VFW_E_NO_TIME_FORMAT: windows_core::HRESULT = 0x80040261_u32 as _;
pub const VFW_E_NO_TIME_FORMAT_SET: windows_core::HRESULT = 0x80040252_u32 as _;
pub const VFW_E_NO_TRANSPORT: windows_core::HRESULT = 0x80040266_u32 as _;
pub const VFW_E_NO_TYPES: windows_core::HRESULT = 0x80040206_u32 as _;
pub const VFW_E_NO_VP_HARDWARE: windows_core::HRESULT = 0x80040274_u32 as _;
pub const VFW_E_OUT_OF_VIDEO_MEMORY: windows_core::HRESULT = 0x80040271_u32 as _;
pub const VFW_E_PALETTE_SET: windows_core::HRESULT = 0x8004021D_u32 as _;
pub const VFW_E_PIN_ALREADY_BLOCKED: windows_core::HRESULT = 0x80040294_u32 as _;
pub const VFW_E_PIN_ALREADY_BLOCKED_ON_THIS_THREAD: windows_core::HRESULT = 0x80040293_u32 as _;
pub const VFW_E_PROCESSOR_NOT_SUITABLE: windows_core::HRESULT = 0x8004025B_u32 as _;
pub const VFW_E_READ_ONLY: windows_core::HRESULT = 0x80040262_u32 as _;
pub const VFW_E_RPZA: windows_core::HRESULT = 0x80040259_u32 as _;
pub const VFW_E_RUNTIME_ERROR: windows_core::HRESULT = 0x8004020B_u32 as _;
pub const VFW_E_SAMPLE_REJECTED: windows_core::HRESULT = 0x8004022B_u32 as _;
pub const VFW_E_SAMPLE_REJECTED_EOS: windows_core::HRESULT = 0x8004022C_u32 as _;
pub const VFW_E_SAMPLE_TIME_NOT_SET: windows_core::HRESULT = 0x80040249_u32 as _;
pub const VFW_E_SIZENOTSET: windows_core::HRESULT = 0x80040212_u32 as _;
pub const VFW_E_START_TIME_AFTER_END: windows_core::HRESULT = 0x80040228_u32 as _;
pub const VFW_E_STATE_CHANGED: windows_core::HRESULT = 0x80040223_u32 as _;
pub const VFW_E_TIMEOUT: windows_core::HRESULT = 0x8004022E_u32 as _;
pub const VFW_E_TIME_ALREADY_PASSED: windows_core::HRESULT = 0x80040233_u32 as _;
pub const VFW_E_TIME_EXPIRED: windows_core::HRESULT = 0x8004027F_u32 as _;
pub const VFW_E_TOO_MANY_COLORS: windows_core::HRESULT = 0x80040222_u32 as _;
pub const VFW_E_TYPE_NOT_ACCEPTED: windows_core::HRESULT = 0x8004022A_u32 as _;
pub const VFW_E_UNKNOWN_FILE_TYPE: windows_core::HRESULT = 0x80040240_u32 as _;
pub const VFW_E_UNSUPPORTED_AUDIO: windows_core::HRESULT = 0x8004025C_u32 as _;
pub const VFW_E_UNSUPPORTED_STREAM: windows_core::HRESULT = 0x80040265_u32 as _;
pub const VFW_E_UNSUPPORTED_VIDEO: windows_core::HRESULT = 0x8004025D_u32 as _;
pub const VFW_E_VMR_NOT_IN_MIXER_MODE: windows_core::HRESULT = 0x80040296_u32 as _;
pub const VFW_E_VMR_NO_AP_SUPPLIED: windows_core::HRESULT = 0x80040297_u32 as _;
pub const VFW_E_VMR_NO_DEINTERLACE_HW: windows_core::HRESULT = 0x80040298_u32 as _;
pub const VFW_E_VMR_NO_PROCAMP_HW: windows_core::HRESULT = 0x80040299_u32 as _;
pub const VFW_E_VP_NEGOTIATION_FAILED: windows_core::HRESULT = 0x80040272_u32 as _;
pub const VFW_E_WRONG_STATE: windows_core::HRESULT = 0x80040227_u32 as _;
pub const VFW_FIRST_CODE: u32 = 512u32;
pub const VFW_S_AUDIO_NOT_RENDERED: windows_core::HRESULT = 0x40258_u32 as _;
pub const VFW_S_CANT_CUE: windows_core::HRESULT = 0x40268_u32 as _;
pub const VFW_S_CONNECTIONS_DEFERRED: windows_core::HRESULT = 0x40246_u32 as _;
pub const VFW_S_DUPLICATE_NAME: windows_core::HRESULT = 0x4022D_u32 as _;
pub const VFW_S_DVD_CHANNEL_CONTENTS_NOT_AVAILABLE: windows_core::HRESULT = 0x4028C_u32 as _;
pub const VFW_S_DVD_NON_ONE_SEQUENTIAL: windows_core::HRESULT = 0x40280_u32 as _;
pub const VFW_S_DVD_NOT_ACCURATE: windows_core::HRESULT = 0x4028D_u32 as _;
pub const VFW_S_DVD_RENDER_STATUS: windows_core::HRESULT = 0x40320_u32 as _;
pub const VFW_S_ESTIMATED: windows_core::HRESULT = 0x40260_u32 as _;
pub const VFW_S_MEDIA_TYPE_IGNORED: windows_core::HRESULT = 0x40254_u32 as _;
pub const VFW_S_NOPREVIEWPIN: windows_core::HRESULT = 0x4027E_u32 as _;
pub const VFW_S_NO_MORE_ITEMS: windows_core::HRESULT = 0x40103_u32 as _;
pub const VFW_S_NO_STOP_TIME: windows_core::HRESULT = 0x40270_u32 as _;
pub const VFW_S_PARTIAL_RENDER: windows_core::HRESULT = 0x40242_u32 as _;
pub const VFW_S_RESERVED: windows_core::HRESULT = 0x40263_u32 as _;
pub const VFW_S_RESOURCE_NOT_NEEDED: windows_core::HRESULT = 0x40250_u32 as _;
pub const VFW_S_RPZA: windows_core::HRESULT = 0x4025A_u32 as _;
pub const VFW_S_SOME_DATA_IGNORED: windows_core::HRESULT = 0x40245_u32 as _;
pub const VFW_S_STATE_INTERMEDIATE: windows_core::HRESULT = 0x40237_u32 as _;
pub const VFW_S_STREAM_OFF: windows_core::HRESULT = 0x40267_u32 as _;
pub const VFW_S_VIDEO_NOT_RENDERED: windows_core::HRESULT = 0x40257_u32 as _;
pub const VMR9ARMode_LetterBox: VMR9AspectRatioMode = 1i32;
pub const VMR9ARMode_None: VMR9AspectRatioMode = 0i32;
pub const VMR9AllocFlag_3DRenderTarget: VMR9SurfaceAllocationFlags = 1i32;
pub const VMR9AllocFlag_DXVATarget: VMR9SurfaceAllocationFlags = 2i32;
pub const VMR9AllocFlag_OffscreenSurface: VMR9SurfaceAllocationFlags = 8i32;
pub const VMR9AllocFlag_RGBDynamicSwitch: VMR9SurfaceAllocationFlags = 16i32;
pub const VMR9AllocFlag_TextureSurface: VMR9SurfaceAllocationFlags = 4i32;
pub const VMR9AllocFlag_UsageMask: VMR9SurfaceAllocationFlags = 255i32;
pub const VMR9AllocFlag_UsageReserved: VMR9SurfaceAllocationFlags = 224i32;
pub const VMR9AlphaBitmap_Disable: VMR9AlphaBitmapFlags = 1i32;
pub const VMR9AlphaBitmap_EntireDDS: VMR9AlphaBitmapFlags = 4i32;
pub const VMR9AlphaBitmap_FilterMode: VMR9AlphaBitmapFlags = 32i32;
pub const VMR9AlphaBitmap_SrcColorKey: VMR9AlphaBitmapFlags = 8i32;
pub const VMR9AlphaBitmap_SrcRect: VMR9AlphaBitmapFlags = 16i32;
pub const VMR9AlphaBitmap_hDC: VMR9AlphaBitmapFlags = 2i32;
pub const VMR9Mode_Mask: VMR9Mode = 7i32;
pub const VMR9Mode_Renderless: VMR9Mode = 4i32;
pub const VMR9Mode_Windowed: VMR9Mode = 1i32;
pub const VMR9Mode_Windowless: VMR9Mode = 2i32;
pub const VMR9Sample_Discontinuity: VMR9PresentationFlags = 4i32;
pub const VMR9Sample_Preroll: VMR9PresentationFlags = 2i32;
pub const VMR9Sample_SrcDstRectsValid: VMR9PresentationFlags = 16i32;
pub const VMR9Sample_SyncPoint: VMR9PresentationFlags = 1i32;
pub const VMR9Sample_TimeValid: VMR9PresentationFlags = 8i32;
pub const VMR9_SampleFieldInterleavedEvenFirst: VMR9_SampleFormat = 3i32;
pub const VMR9_SampleFieldInterleavedOddFirst: VMR9_SampleFormat = 4i32;
pub const VMR9_SampleFieldSingleEven: VMR9_SampleFormat = 5i32;
pub const VMR9_SampleFieldSingleOdd: VMR9_SampleFormat = 6i32;
pub const VMR9_SampleProgressiveFrame: VMR9_SampleFormat = 2i32;
pub const VMR9_SampleReserved: VMR9_SampleFormat = 1i32;
pub const VMRBITMAP_DISABLE: u32 = 1u32;
pub const VMRBITMAP_ENTIREDDS: u32 = 4u32;
pub const VMRBITMAP_HDC: u32 = 2u32;
pub const VMRBITMAP_SRCCOLORKEY: u32 = 8u32;
pub const VMRBITMAP_SRCRECT: u32 = 16u32;
pub const VMRMode_Mask: VMRMode = 7i32;
pub const VMRMode_Renderless: VMRMode = 4i32;
pub const VMRMode_Windowed: VMRMode = 1i32;
pub const VMRMode_Windowless: VMRMode = 2i32;
pub const VMRSample_Discontinuity: VMRPresentationFlags = 4i32;
pub const VMRSample_Preroll: VMRPresentationFlags = 2i32;
pub const VMRSample_SrcDstRectsValid: VMRPresentationFlags = 16i32;
pub const VMRSample_SyncPoint: VMRPresentationFlags = 1i32;
pub const VMRSample_TimeValid: VMRPresentationFlags = 8i32;
pub const VMR_ARMODE_LETTER_BOX: VMR_ASPECT_RATIO_MODE = 1i32;
pub const VMR_ARMODE_NONE: VMR_ASPECT_RATIO_MODE = 0i32;
pub const VMR_NOTSUPPORTED: u32 = 0u32;
pub const VMR_RENDER_DEVICE_OVERLAY: u32 = 1u32;
pub const VMR_RENDER_DEVICE_SYSMEM: u32 = 4u32;
pub const VMR_RENDER_DEVICE_VIDMEM: u32 = 2u32;
pub const VMR_SUPPORTED: u32 = 1u32;
pub const VariableBitRateAverage: VIDEOENCODER_BITRATE_MODE = 1i32;
pub const VariableBitRatePeak: VIDEOENCODER_BITRATE_MODE = 2i32;
pub const VfwCaptureDialog_Display: VfwCaptureDialogs = 4i32;
pub const VfwCaptureDialog_Format: VfwCaptureDialogs = 2i32;
pub const VfwCaptureDialog_Source: VfwCaptureDialogs = 1i32;
pub const VfwCompressDialog_About: VfwCompressDialogs = 2i32;
pub const VfwCompressDialog_Config: VfwCompressDialogs = 1i32;
pub const VfwCompressDialog_QueryAbout: VfwCompressDialogs = 8i32;
pub const VfwCompressDialog_QueryConfig: VfwCompressDialogs = 4i32;
pub const VideoControlFlag_ExternalTriggerEnable: VideoControlFlags = 4i32;
pub const VideoControlFlag_FlipHorizontal: VideoControlFlags = 1i32;
pub const VideoControlFlag_FlipVertical: VideoControlFlags = 2i32;
pub const VideoControlFlag_Trigger: VideoControlFlags = 8i32;
pub const VideoCopyProtectionMacrovisionBasic: VideoCopyProtectionType = 0i32;
pub const VideoCopyProtectionMacrovisionCBI: VideoCopyProtectionType = 1i32;
pub const VideoProcAmp_BacklightCompensation: VideoProcAmpProperty = 8i32;
pub const VideoProcAmp_Brightness: VideoProcAmpProperty = 0i32;
pub const VideoProcAmp_ColorEnable: VideoProcAmpProperty = 6i32;
pub const VideoProcAmp_Contrast: VideoProcAmpProperty = 1i32;
pub const VideoProcAmp_Flags_Auto: VideoProcAmpFlags = 1i32;
pub const VideoProcAmp_Flags_Manual: VideoProcAmpFlags = 2i32;
pub const VideoProcAmp_Gain: VideoProcAmpProperty = 9i32;
pub const VideoProcAmp_Gamma: VideoProcAmpProperty = 5i32;
pub const VideoProcAmp_Hue: VideoProcAmpProperty = 2i32;
pub const VideoProcAmp_Saturation: VideoProcAmpProperty = 3i32;
pub const VideoProcAmp_Sharpness: VideoProcAmpProperty = 4i32;
pub const VideoProcAmp_WhiteBalance: VideoProcAmpProperty = 7i32;
pub const g_wszExcludeScriptStreamDeliverySynchronization: windows_core::PCWSTR = windows_core::w!("ExcludeScriptStreamDeliverySynchronization");
pub const iBLUE: u32 = 2u32;
pub const iEGA_COLORS: u32 = 16u32;
pub const iGREEN: u32 = 1u32;
pub const iMASK_COLORS: u32 = 3u32;
pub const iMAXBITS: u32 = 8u32;
pub const iPALETTE: u32 = 8u32;
pub const iPALETTE_COLORS: u32 = 256u32;
pub const iRED: u32 = 0u32;
pub const iTRUECOLOR: u32 = 16u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ADVISE_TYPE(pub i32);
impl windows_core::TypeKind for ADVISE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMExtendedSeekingCapabilities(pub i32);
impl windows_core::TypeKind for AMExtendedSeekingCapabilities {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMMSF_MMS_INIT_FLAGS(pub i32);
impl windows_core::TypeKind for AMMSF_MMS_INIT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMMSF_MS_FLAGS(pub i32);
impl windows_core::TypeKind for AMMSF_MS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMMSF_RENDER_FLAGS(pub i32);
impl windows_core::TypeKind for AMMSF_RENDER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMOVERLAYFX(pub i32);
impl windows_core::TypeKind for AMOVERLAYFX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMPROPERTY_PIN(pub i32);
impl windows_core::TypeKind for AMPROPERTY_PIN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMPlayListEventFlags(pub i32);
impl windows_core::TypeKind for AMPlayListEventFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMPlayListFlags(pub i32);
impl windows_core::TypeKind for AMPlayListFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMPlayListItemFlags(pub i32);
impl windows_core::TypeKind for AMPlayListItemFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMTVAudioEventType(pub i32);
impl windows_core::TypeKind for AMTVAudioEventType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMTunerEventType(pub i32);
impl windows_core::TypeKind for AMTunerEventType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMTunerModeType(pub i32);
impl windows_core::TypeKind for AMTunerModeType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMTunerSignalStrength(pub i32);
impl windows_core::TypeKind for AMTunerSignalStrength {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMTunerSubChannel(pub i32);
impl windows_core::TypeKind for AMTunerSubChannel {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMVP_MODE(pub i32);
impl windows_core::TypeKind for AMVP_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AMVP_SELECT_FORMAT_BY(pub i32);
impl windows_core::TypeKind for AMVP_SELECT_FORMAT_BY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_ASPECT_RATIO_MODE(pub i32);
impl windows_core::TypeKind for AM_ASPECT_RATIO_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_COPY_MACROVISION_LEVEL(pub i32);
impl windows_core::TypeKind for AM_COPY_MACROVISION_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_DIGITAL_CP(pub i32);
impl windows_core::TypeKind for AM_DIGITAL_CP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_DVDCOPYSTATE(pub i32);
impl windows_core::TypeKind for AM_DVDCOPYSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_DVD_GRAPH_FLAGS(pub i32);
impl windows_core::TypeKind for AM_DVD_GRAPH_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_DVD_STREAM_FLAGS(pub i32);
impl windows_core::TypeKind for AM_DVD_STREAM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_FILESINK_FLAGS(pub i32);
impl windows_core::TypeKind for AM_FILESINK_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_FILTER_FLAGS(pub i32);
impl windows_core::TypeKind for AM_FILTER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_GRAPH_CONFIG_RECONNECT_FLAGS(pub i32);
impl windows_core::TypeKind for AM_GRAPH_CONFIG_RECONNECT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_LINE21_CCLEVEL(pub i32);
impl windows_core::TypeKind for AM_LINE21_CCLEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_LINE21_CCSERVICE(pub i32);
impl windows_core::TypeKind for AM_LINE21_CCSERVICE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_LINE21_CCSTATE(pub i32);
impl windows_core::TypeKind for AM_LINE21_CCSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_LINE21_CCSTYLE(pub i32);
impl windows_core::TypeKind for AM_LINE21_CCSTYLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_LINE21_DRAWBGMODE(pub i32);
impl windows_core::TypeKind for AM_LINE21_DRAWBGMODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_MEDIAEVENT_FLAGS(pub i32);
impl windows_core::TypeKind for AM_MEDIAEVENT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_MPEG2Level(pub i32);
impl windows_core::TypeKind for AM_MPEG2Level {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_MPEG2Profile(pub i32);
impl windows_core::TypeKind for AM_MPEG2Profile {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_PROPERTY_AC3(pub i32);
impl windows_core::TypeKind for AM_PROPERTY_AC3 {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_PROPERTY_DVDCOPYPROT(pub i32);
impl windows_core::TypeKind for AM_PROPERTY_DVDCOPYPROT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_PROPERTY_DVDKARAOKE(pub i32);
impl windows_core::TypeKind for AM_PROPERTY_DVDKARAOKE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_PROPERTY_DVDSUBPIC(pub i32);
impl windows_core::TypeKind for AM_PROPERTY_DVDSUBPIC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_PROPERTY_DVD_RATE_CHANGE(pub i32);
impl windows_core::TypeKind for AM_PROPERTY_DVD_RATE_CHANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_PROPERTY_FRAMESTEP(pub i32);
impl windows_core::TypeKind for AM_PROPERTY_FRAMESTEP {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_PROPERTY_TS_RATE_CHANGE(pub i32);
impl windows_core::TypeKind for AM_PROPERTY_TS_RATE_CHANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_SAMPLE_PROPERTY_FLAGS(pub i32);
impl windows_core::TypeKind for AM_SAMPLE_PROPERTY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_SEEKING_SEEKING_CAPABILITIES(pub i32);
impl windows_core::TypeKind for AM_SEEKING_SEEKING_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_SEEKING_SEEKING_FLAGS(pub i32);
impl windows_core::TypeKind for AM_SEEKING_SEEKING_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_STREAM_INFO_FLAGS(pub i32);
impl windows_core::TypeKind for AM_STREAM_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_WST_DRAWBGMODE(pub i32);
impl windows_core::TypeKind for AM_WST_DRAWBGMODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_WST_LEVEL(pub i32);
impl windows_core::TypeKind for AM_WST_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_WST_SERVICE(pub i32);
impl windows_core::TypeKind for AM_WST_SERVICE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_WST_STATE(pub i32);
impl windows_core::TypeKind for AM_WST_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AM_WST_STYLE(pub i32);
impl windows_core::TypeKind for AM_WST_STYLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ATSCComponentTypeFlags(pub i32);
impl windows_core::TypeKind for ATSCComponentTypeFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AnalogVideoStandard(pub i32);
impl windows_core::TypeKind for AnalogVideoStandard {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ApplicationTypeType(pub i32);
impl windows_core::TypeKind for ApplicationTypeType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_CHANGE_STATE(pub i32);
impl windows_core::TypeKind for BDA_CHANGE_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_CONDITIONALACCESS_MMICLOSEREASON(pub i32);
impl windows_core::TypeKind for BDA_CONDITIONALACCESS_MMICLOSEREASON {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_CONDITIONALACCESS_REQUESTTYPE(pub i32);
impl windows_core::TypeKind for BDA_CONDITIONALACCESS_REQUESTTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_CONDITIONALACCESS_SESSION_RESULT(pub i32);
impl windows_core::TypeKind for BDA_CONDITIONALACCESS_SESSION_RESULT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_Channel(pub i32);
impl windows_core::TypeKind for BDA_Channel {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_Channel_Bandwidth(pub i32);
impl windows_core::TypeKind for BDA_Channel_Bandwidth {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_Comp_Flags(pub i32);
impl windows_core::TypeKind for BDA_Comp_Flags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_DISCOVERY_STATE(pub i32);
impl windows_core::TypeKind for BDA_DISCOVERY_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_DrmPairingError(pub i32);
impl windows_core::TypeKind for BDA_DrmPairingError {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_EVENT_ID(pub i32);
impl windows_core::TypeKind for BDA_EVENT_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_Frequency(pub i32);
impl windows_core::TypeKind for BDA_Frequency {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_Frequency_Multiplier(pub i32);
impl windows_core::TypeKind for BDA_Frequency_Multiplier {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_MULTICAST_MODE(pub i32);
impl windows_core::TypeKind for BDA_MULTICAST_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_Range(pub i32);
impl windows_core::TypeKind for BDA_Range {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BDA_SIGNAL_STATE(pub i32);
impl windows_core::TypeKind for BDA_SIGNAL_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BinaryConvolutionCodeRate(pub i32);
impl windows_core::TypeKind for BinaryConvolutionCodeRate {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COLORKEY_TYPE(pub i32);
impl windows_core::TypeKind for COLORKEY_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COMPLETION_STATUS_FLAGS(pub i32);
impl windows_core::TypeKind for COMPLETION_STATUS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_ACP_Protection_Level(pub i32);
impl windows_core::TypeKind for COPP_ACP_Protection_Level {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_BusType(pub i32);
impl windows_core::TypeKind for COPP_BusType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_CGMSA_Protection_Level(pub i32);
impl windows_core::TypeKind for COPP_CGMSA_Protection_Level {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_ConnectorType(pub i32);
impl windows_core::TypeKind for COPP_ConnectorType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_HDCP_Protection_Level(pub i32);
impl windows_core::TypeKind for COPP_HDCP_Protection_Level {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_ImageAspectRatio_EN300294(pub i32);
impl windows_core::TypeKind for COPP_ImageAspectRatio_EN300294 {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_StatusFlags(pub i32);
impl windows_core::TypeKind for COPP_StatusFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_StatusHDCPFlags(pub i32);
impl windows_core::TypeKind for COPP_StatusHDCPFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COPP_TVProtectionStandard(pub i32);
impl windows_core::TypeKind for COPP_TVProtectionStandard {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CameraControlFlags(pub i32);
impl windows_core::TypeKind for CameraControlFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CameraControlProperty(pub i32);
impl windows_core::TypeKind for CameraControlProperty {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ComponentCategory(pub i32);
impl windows_core::TypeKind for ComponentCategory {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ComponentStatus(pub i32);
impl windows_core::TypeKind for ComponentStatus {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CompressionCaps(pub i32);
impl windows_core::TypeKind for CompressionCaps {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DDSFF_FLAGS(pub i32);
impl windows_core::TypeKind for DDSFF_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DECIMATION_USAGE(pub i32);
impl windows_core::TypeKind for DECIMATION_USAGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVBSystemType(pub i32);
impl windows_core::TypeKind for DVBSystemType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_AUDIO_APPMODE(pub i32);
impl windows_core::TypeKind for DVD_AUDIO_APPMODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_AUDIO_FORMAT(pub i32);
impl windows_core::TypeKind for DVD_AUDIO_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_AUDIO_LANG_EXT(pub i32);
impl windows_core::TypeKind for DVD_AUDIO_LANG_EXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_CMD_FLAGS(pub i32);
impl windows_core::TypeKind for DVD_CMD_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_DISC_SIDE(pub i32);
impl windows_core::TypeKind for DVD_DISC_SIDE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_DOMAIN(pub i32);
impl windows_core::TypeKind for DVD_DOMAIN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_ERROR(pub i32);
impl windows_core::TypeKind for DVD_ERROR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_FRAMERATE(pub i32);
impl windows_core::TypeKind for DVD_FRAMERATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_KARAOKE_ASSIGNMENT(pub i32);
impl windows_core::TypeKind for DVD_KARAOKE_ASSIGNMENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_KARAOKE_CONTENTS(pub i32);
impl windows_core::TypeKind for DVD_KARAOKE_CONTENTS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_KARAOKE_DOWNMIX(pub i32);
impl windows_core::TypeKind for DVD_KARAOKE_DOWNMIX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_MENU_ID(pub i32);
impl windows_core::TypeKind for DVD_MENU_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_NavCmdType(pub i32);
impl windows_core::TypeKind for DVD_NavCmdType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_OPTION_FLAG(pub i32);
impl windows_core::TypeKind for DVD_OPTION_FLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_PARENTAL_LEVEL(pub i32);
impl windows_core::TypeKind for DVD_PARENTAL_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_PB_STOPPED(pub i32);
impl windows_core::TypeKind for DVD_PB_STOPPED {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_PLAY_DIRECTION(pub i32);
impl windows_core::TypeKind for DVD_PLAY_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_PREFERRED_DISPLAY_MODE(pub i32);
impl windows_core::TypeKind for DVD_PREFERRED_DISPLAY_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_RELATIVE_BUTTON(pub i32);
impl windows_core::TypeKind for DVD_RELATIVE_BUTTON {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_SUBPICTURE_CODING(pub i32);
impl windows_core::TypeKind for DVD_SUBPICTURE_CODING {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_SUBPICTURE_LANG_EXT(pub i32);
impl windows_core::TypeKind for DVD_SUBPICTURE_LANG_EXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_SUBPICTURE_TYPE(pub i32);
impl windows_core::TypeKind for DVD_SUBPICTURE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_TIMECODE_FLAGS(pub i32);
impl windows_core::TypeKind for DVD_TIMECODE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_TITLE_APPMODE(pub i32);
impl windows_core::TypeKind for DVD_TITLE_APPMODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_TextCharSet(pub i32);
impl windows_core::TypeKind for DVD_TextCharSet {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_TextStringType(pub i32);
impl windows_core::TypeKind for DVD_TextStringType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_VIDEO_COMPRESSION(pub i32);
impl windows_core::TypeKind for DVD_VIDEO_COMPRESSION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVD_WARNING(pub i32);
impl windows_core::TypeKind for DVD_WARNING {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXVA2_DestinationFlags(pub i32);
impl windows_core::TypeKind for DXVA2_DestinationFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DXVA2_SampleFlags(pub i32);
impl windows_core::TypeKind for DXVA2_SampleFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntitlementType(pub i32);
impl windows_core::TypeKind for EntitlementType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FECMethod(pub i32);
impl windows_core::TypeKind for FECMethod {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FILTER_STATE(pub i32);
impl windows_core::TypeKind for FILTER_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GuardInterval(pub i32);
impl windows_core::TypeKind for GuardInterval {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HierarchyAlpha(pub i32);
impl windows_core::TypeKind for HierarchyAlpha {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IFILTERMAPPER_MERIT(pub i32);
impl windows_core::TypeKind for IFILTERMAPPER_MERIT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ISDBCAS_REQUEST_ID(pub i32);
impl windows_core::TypeKind for ISDBCAS_REQUEST_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InterleavingMode(pub i32);
impl windows_core::TypeKind for InterleavingMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KSPROPERTY_IPSINK(pub i32);
impl windows_core::TypeKind for KSPROPERTY_IPSINK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LNB_Source(pub i32);
impl windows_core::TypeKind for LNB_Source {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LocationCodeSchemeType(pub i32);
impl windows_core::TypeKind for LocationCodeSchemeType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MEDIA_SAMPLE_CONTENT(pub i32);
impl windows_core::TypeKind for MEDIA_SAMPLE_CONTENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MMSSF_GET_INFORMATION_FLAGS(pub i32);
impl windows_core::TypeKind for MMSSF_GET_INFORMATION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MPEG2StreamType(pub i32);
impl windows_core::TypeKind for MPEG2StreamType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MPEGLAYER3WAVEFORMAT_FLAGS(pub u32);
impl windows_core::TypeKind for MPEGLAYER3WAVEFORMAT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MP_CURVE_TYPE(pub i32);
impl windows_core::TypeKind for MP_CURVE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MP_TYPE(pub i32);
impl windows_core::TypeKind for MP_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MUX_PID_TYPE(pub i32);
impl windows_core::TypeKind for MUX_PID_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ModulationType(pub i32);
impl windows_core::TypeKind for ModulationType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OA_BOOL(pub i32);
impl windows_core::TypeKind for OA_BOOL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OUTPUT_STATE(pub i32);
impl windows_core::TypeKind for OUTPUT_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PIN_DIRECTION(pub i32);
impl windows_core::TypeKind for PIN_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PhysicalConnectorType(pub i32);
impl windows_core::TypeKind for PhysicalConnectorType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pilot(pub i32);
impl windows_core::TypeKind for Pilot {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Polarisation(pub i32);
impl windows_core::TypeKind for Polarisation {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QualityMessageType(pub i32);
impl windows_core::TypeKind for QualityMessageType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct REG_PINFLAG(pub i32);
impl windows_core::TypeKind for REG_PINFLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RollOff(pub i32);
impl windows_core::TypeKind for RollOff {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SNDDEV_ERR(pub i32);
impl windows_core::TypeKind for SNDDEV_ERR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SSUPDATE_TYPE(pub i32);
impl windows_core::TypeKind for SSUPDATE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STREAMIF_CONSTANTS(pub i32);
impl windows_core::TypeKind for STREAMIF_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STREAM_STATE(pub i32);
impl windows_core::TypeKind for STREAM_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STREAM_TYPE(pub i32);
impl windows_core::TypeKind for STREAM_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScanModulationTypes(pub i32);
impl windows_core::TypeKind for ScanModulationTypes {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SmartCardAssociationType(pub i32);
impl windows_core::TypeKind for SmartCardAssociationType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SmartCardStatusType(pub i32);
impl windows_core::TypeKind for SmartCardStatusType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SpectralInversion(pub i32);
impl windows_core::TypeKind for SpectralInversion {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TVAudioMode(pub i32);
impl windows_core::TypeKind for TVAudioMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransmissionMode(pub i32);
impl windows_core::TypeKind for TransmissionMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TunerInputType(pub i32);
impl windows_core::TypeKind for TunerInputType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UICloseReasonType(pub i32);
impl windows_core::TypeKind for UICloseReasonType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VALID_UOP_FLAG(pub i32);
impl windows_core::TypeKind for VALID_UOP_FLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VIDEOENCODER_BITRATE_MODE(pub i32);
impl windows_core::TypeKind for VIDEOENCODER_BITRATE_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9AlphaBitmapFlags(pub i32);
impl windows_core::TypeKind for VMR9AlphaBitmapFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9AspectRatioMode(pub i32);
impl windows_core::TypeKind for VMR9AspectRatioMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9DeinterlacePrefs(pub i32);
impl windows_core::TypeKind for VMR9DeinterlacePrefs {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9DeinterlaceTech(pub i32);
impl windows_core::TypeKind for VMR9DeinterlaceTech {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9MixerPrefs(pub i32);
impl windows_core::TypeKind for VMR9MixerPrefs {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9Mode(pub i32);
impl windows_core::TypeKind for VMR9Mode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9PresentationFlags(pub i32);
impl windows_core::TypeKind for VMR9PresentationFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9ProcAmpControlFlags(pub i32);
impl windows_core::TypeKind for VMR9ProcAmpControlFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9RenderPrefs(pub i32);
impl windows_core::TypeKind for VMR9RenderPrefs {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9SurfaceAllocationFlags(pub i32);
impl windows_core::TypeKind for VMR9SurfaceAllocationFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR9_SampleFormat(pub i32);
impl windows_core::TypeKind for VMR9_SampleFormat {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMRDeinterlacePrefs(pub i32);
impl windows_core::TypeKind for VMRDeinterlacePrefs {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMRDeinterlaceTech(pub i32);
impl windows_core::TypeKind for VMRDeinterlaceTech {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMRMixerPrefs(pub i32);
impl windows_core::TypeKind for VMRMixerPrefs {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMRMode(pub i32);
impl windows_core::TypeKind for VMRMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMRPresentationFlags(pub i32);
impl windows_core::TypeKind for VMRPresentationFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMRRenderPrefs(pub i32);
impl windows_core::TypeKind for VMRRenderPrefs {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMRSurfaceAllocationFlags(pub i32);
impl windows_core::TypeKind for VMRSurfaceAllocationFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VMR_ASPECT_RATIO_MODE(pub i32);
impl windows_core::TypeKind for VMR_ASPECT_RATIO_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VfwCaptureDialogs(pub i32);
impl windows_core::TypeKind for VfwCaptureDialogs {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VfwCompressDialogs(pub i32);
impl windows_core::TypeKind for VfwCompressDialogs {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VideoControlFlags(pub i32);
impl windows_core::TypeKind for VideoControlFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VideoCopyProtectionType(pub i32);
impl windows_core::TypeKind for VideoCopyProtectionType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VideoProcAmpFlags(pub i32);
impl windows_core::TypeKind for VideoProcAmpFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VideoProcAmpProperty(pub i32);
impl windows_core::TypeKind for VideoProcAmpProperty {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AMRESCTL_RESERVEFLAGS(pub i32);
impl windows_core::TypeKind for _AMRESCTL_RESERVEFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AMSTREAMSELECTENABLEFLAGS(pub i32);
impl windows_core::TypeKind for _AMSTREAMSELECTENABLEFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AMSTREAMSELECTINFOFLAGS(pub i32);
impl windows_core::TypeKind for _AMSTREAMSELECTINFOFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AM_AUDIO_RENDERER_STAT_PARAM(pub i32);
impl windows_core::TypeKind for _AM_AUDIO_RENDERER_STAT_PARAM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AM_FILTER_MISC_FLAGS(pub i32);
impl windows_core::TypeKind for _AM_FILTER_MISC_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AM_INTF_SEARCH_FLAGS(pub i32);
impl windows_core::TypeKind for _AM_INTF_SEARCH_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AM_OVERLAY_NOTIFY_FLAGS(pub i32);
impl windows_core::TypeKind for _AM_OVERLAY_NOTIFY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS(pub i32);
impl windows_core::TypeKind for _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AM_PUSHSOURCE_FLAGS(pub i32);
impl windows_core::TypeKind for _AM_PUSHSOURCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _AM_RENSDEREXFLAGS(pub i32);
impl windows_core::TypeKind for _AM_RENSDEREXFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _DVDECODERRESOLUTION(pub i32);
impl windows_core::TypeKind for _DVDECODERRESOLUTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _DVENCODERFORMAT(pub i32);
impl windows_core::TypeKind for _DVENCODERFORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _DVENCODERRESOLUTION(pub i32);
impl windows_core::TypeKind for _DVENCODERRESOLUTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _DVENCODERVIDEOFORMAT(pub i32);
impl windows_core::TypeKind for _DVENCODERVIDEOFORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _DVRESOLUTION(pub i32);
impl windows_core::TypeKind for _DVRESOLUTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct _REM_FILTER_FLAGS(pub i32);
impl windows_core::TypeKind for _REM_FILTER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ALLOCATOR_PROPERTIES {
    pub cBuffers: i32,
    pub cbBuffer: i32,
    pub cbAlign: i32,
    pub cbPrefix: i32,
}
impl Default for ALLOCATOR_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ALLOCATOR_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMCOPPCommand {
    pub macKDI: windows_core::GUID,
    pub guidCommandID: windows_core::GUID,
    pub dwSequence: u32,
    pub cbSizeData: u32,
    pub CommandData: [u8; 4056],
}
impl Default for AMCOPPCommand {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMCOPPCommand {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMCOPPSignature {
    pub Signature: [u8; 256],
}
impl Default for AMCOPPSignature {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMCOPPSignature {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMCOPPStatusInput {
    pub rApp: windows_core::GUID,
    pub guidStatusRequestID: windows_core::GUID,
    pub dwSequence: u32,
    pub cbSizeData: u32,
    pub StatusData: [u8; 4056],
}
impl Default for AMCOPPStatusInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMCOPPStatusInput {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMCOPPStatusOutput {
    pub macKDI: windows_core::GUID,
    pub cbSizeData: u32,
    pub COPPStatus: [u8; 4076],
}
impl Default for AMCOPPStatusOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMCOPPStatusOutput {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVABUFFERINFO {
    pub dwTypeIndex: u32,
    pub dwBufferIndex: u32,
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
}
impl Default for AMVABUFFERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMVABUFFERINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVABeginFrameInfo {
    pub dwDestSurfaceIndex: u32,
    pub pInputData: *mut core::ffi::c_void,
    pub dwSizeInputData: u32,
    pub pOutputData: *mut core::ffi::c_void,
    pub dwSizeOutputData: u32,
}
impl Default for AMVABeginFrameInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMVABeginFrameInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVACompBufferInfo {
    pub dwNumCompBuffers: u32,
    pub dwWidthToCreate: u32,
    pub dwHeightToCreate: u32,
    pub dwBytesToAllocate: u32,
    pub ddCompCaps: super::super::Graphics::DirectDraw::DDSCAPS2,
    pub ddPixelFormat: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl Default for AMVACompBufferInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl windows_core::TypeKind for AMVACompBufferInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVAEndFrameInfo {
    pub dwSizeMiscData: u32,
    pub pMiscData: *mut core::ffi::c_void,
}
impl Default for AMVAEndFrameInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMVAEndFrameInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVAInternalMemInfo {
    pub dwScratchMemAlloc: u32,
}
impl Default for AMVAInternalMemInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMVAInternalMemInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVAUncompBufferInfo {
    pub dwMinNumSurfaces: u32,
    pub dwMaxNumSurfaces: u32,
    pub ddUncompPixelFormat: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl Default for AMVAUncompBufferInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl windows_core::TypeKind for AMVAUncompBufferInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVAUncompDataInfo {
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl Default for AMVAUncompDataInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl windows_core::TypeKind for AMVAUncompDataInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVPDATAINFO {
    pub dwSize: u32,
    pub dwMicrosecondsPerField: u32,
    pub amvpDimInfo: AMVPDIMINFO,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub bEnableDoubleClock: super::super::Foundation::BOOL,
    pub bEnableVACT: super::super::Foundation::BOOL,
    pub bDataIsInterlaced: super::super::Foundation::BOOL,
    pub lHalfLinesOdd: i32,
    pub bFieldPolarityInverted: super::super::Foundation::BOOL,
    pub dwNumLinesInVREF: u32,
    pub lHalfLinesEven: i32,
    pub dwReserved1: u32,
}
impl Default for AMVPDATAINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMVPDATAINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVPDIMINFO {
    pub dwFieldWidth: u32,
    pub dwFieldHeight: u32,
    pub dwVBIWidth: u32,
    pub dwVBIHeight: u32,
    pub rcValidRegion: super::super::Foundation::RECT,
}
impl Default for AMVPDIMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMVPDIMINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMVPSIZE {
    pub dwWidth: u32,
    pub dwHeight: u32,
}
impl Default for AMVPSIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AMVPSIZE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_AC3_ALTERNATE_AUDIO {
    pub fStereo: super::super::Foundation::BOOL,
    pub DualMode: u32,
}
impl Default for AM_AC3_ALTERNATE_AUDIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_AC3_ALTERNATE_AUDIO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_AC3_BIT_STREAM_MODE {
    pub BitStreamMode: i32,
}
impl Default for AM_AC3_BIT_STREAM_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_AC3_BIT_STREAM_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_AC3_DIALOGUE_LEVEL {
    pub DialogueLevel: u32,
}
impl Default for AM_AC3_DIALOGUE_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_AC3_DIALOGUE_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_AC3_DOWNMIX {
    pub fDownMix: super::super::Foundation::BOOL,
    pub fDolbySurround: super::super::Foundation::BOOL,
}
impl Default for AM_AC3_DOWNMIX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_AC3_DOWNMIX {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_AC3_ERROR_CONCEALMENT {
    pub fRepeatPreviousBlock: super::super::Foundation::BOOL,
    pub fErrorInCurrentBlock: super::super::Foundation::BOOL,
}
impl Default for AM_AC3_ERROR_CONCEALMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_AC3_ERROR_CONCEALMENT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_AC3_ROOM_TYPE {
    pub fLargeRoom: super::super::Foundation::BOOL,
}
impl Default for AM_AC3_ROOM_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_AC3_ROOM_TYPE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_COLCON {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
}
impl Default for AM_COLCON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_COLCON {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_COPY_MACROVISION {
    pub MACROVISIONLevel: u32,
}
impl Default for AM_COPY_MACROVISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_COPY_MACROVISION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DVDCOPY_BUSKEY {
    pub BusKey: [u8; 5],
    pub Reserved: [u8; 1],
}
impl Default for AM_DVDCOPY_BUSKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DVDCOPY_BUSKEY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DVDCOPY_CHLGKEY {
    pub ChlgKey: [u8; 10],
    pub Reserved: [u8; 2],
}
impl Default for AM_DVDCOPY_CHLGKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DVDCOPY_CHLGKEY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DVDCOPY_DISCKEY {
    pub DiscKey: [u8; 2048],
}
impl Default for AM_DVDCOPY_DISCKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DVDCOPY_DISCKEY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DVDCOPY_SET_COPY_STATE {
    pub DVDCopyState: u32,
}
impl Default for AM_DVDCOPY_SET_COPY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DVDCOPY_SET_COPY_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DVDCOPY_TITLEKEY {
    pub KeyFlags: u32,
    pub Reserved1: [u32; 2],
    pub TitleKey: [u8; 6],
    pub Reserved2: [u8; 2],
}
impl Default for AM_DVDCOPY_TITLEKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DVDCOPY_TITLEKEY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DVD_ChangeRate {
    pub StartInTime: i64,
    pub StartOutTime: i64,
    pub Rate: i32,
}
impl Default for AM_DVD_ChangeRate {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DVD_ChangeRate {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DVD_RENDERSTATUS {
    pub hrVPEStatus: windows_core::HRESULT,
    pub bDvdVolInvalid: super::super::Foundation::BOOL,
    pub bDvdVolUnknown: super::super::Foundation::BOOL,
    pub bNoLine21In: super::super::Foundation::BOOL,
    pub bNoLine21Out: super::super::Foundation::BOOL,
    pub iNumStreams: i32,
    pub iNumStreamsFailed: i32,
    pub dwFailedStreamsFlag: u32,
}
impl Default for AM_DVD_RENDERSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DVD_RENDERSTATUS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DVD_YUV {
    pub Reserved: u8,
    pub Y: u8,
    pub U: u8,
    pub V: u8,
}
impl Default for AM_DVD_YUV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DVD_YUV {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_DvdKaraokeData {
    pub dwDownmix: u32,
    pub dwSpeakerAssignment: u32,
}
impl Default for AM_DvdKaraokeData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_DvdKaraokeData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_ExactRateChange {
    pub OutputZeroTime: i64,
    pub Rate: i32,
}
impl Default for AM_ExactRateChange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_ExactRateChange {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_FRAMESTEP_STEP {
    pub dwFramesToStep: u32,
}
impl Default for AM_FRAMESTEP_STEP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_FRAMESTEP_STEP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Media_MediaFoundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_MPEGSTREAMTYPE {
    pub dwStreamId: u32,
    pub dwReserved: u32,
    pub mt: super::MediaFoundation::AM_MEDIA_TYPE,
    pub bFormat: [u8; 1],
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl Default for AM_MPEGSTREAMTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::TypeKind for AM_MPEGSTREAMTYPE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Media_MediaFoundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_MPEGSYSTEMTYPE {
    pub dwBitRate: u32,
    pub cStreams: u32,
    pub Streams: [AM_MPEGSTREAMTYPE; 1],
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl Default for AM_MPEGSYSTEMTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::TypeKind for AM_MPEGSYSTEMTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_PROPERTY_SPHLI {
    pub HLISS: u16,
    pub Reserved: u16,
    pub StartPTM: u32,
    pub EndPTM: u32,
    pub StartX: u16,
    pub StartY: u16,
    pub StopX: u16,
    pub StopY: u16,
    pub ColCon: AM_COLCON,
}
impl Default for AM_PROPERTY_SPHLI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_PROPERTY_SPHLI {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_PROPERTY_SPPAL {
    pub sppal: [AM_DVD_YUV; 16],
}
impl Default for AM_PROPERTY_SPPAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_PROPERTY_SPPAL {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_QueryRate {
    pub lMaxForwardFullFrame: i32,
    pub lMaxReverseFullFrame: i32,
}
impl Default for AM_QueryRate {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_QueryRate {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Media_MediaFoundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_SAMPLE2_PROPERTIES {
    pub cbData: u32,
    pub dwTypeSpecificFlags: u32,
    pub dwSampleFlags: u32,
    pub lActual: i32,
    pub tStart: i64,
    pub tStop: i64,
    pub dwStreamId: u32,
    pub pMediaType: *mut super::MediaFoundation::AM_MEDIA_TYPE,
    pub pbBuffer: *mut u8,
    pub cbBuffer: i32,
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl Default for AM_SAMPLE2_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::TypeKind for AM_SAMPLE2_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_STREAM_INFO {
    pub tStart: i64,
    pub tStop: i64,
    pub dwStartCookie: u32,
    pub dwStopCookie: u32,
    pub dwFlags: u32,
}
impl Default for AM_STREAM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_STREAM_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_SimpleRateChange {
    pub StartTime: i64,
    pub Rate: i32,
}
impl Default for AM_SimpleRateChange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_SimpleRateChange {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AM_WST_PAGE {
    pub dwPageNr: u32,
    pub dwSubPageNr: u32,
    pub pucPageData: *mut u8,
}
impl Default for AM_WST_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AM_WST_PAGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ANALOGVIDEOINFO {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: i64,
}
impl Default for ANALOGVIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ANALOGVIDEOINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIO_STREAM_CONFIG_CAPS {
    pub guid: windows_core::GUID,
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
impl Default for AUDIO_STREAM_CONFIG_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUDIO_STREAM_CONFIG_CAPS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIEXTHEADER {
    pub fcc: u32,
    pub cb: u32,
    pub dwGrandFrames: u32,
    pub dwFuture: [u32; 61],
}
impl Default for AVIEXTHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIEXTHEADER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIFIELDINDEX {
    pub fcc: u32,
    pub cb: u32,
    pub wLongsPerEntry: u16,
    pub bIndexSubType: u8,
    pub bIndexType: u8,
    pub nEntriesInUse: u32,
    pub dwChunkId: u32,
    pub qwBaseOffset: u64,
    pub dwReserved3: u32,
    pub aIndex: [AVIFIELDINDEX_0; 1],
}
impl Default for AVIFIELDINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIFIELDINDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIFIELDINDEX_0 {
    pub dwOffset: u32,
    pub dwSize: u32,
    pub dwOffsetField2: u32,
}
impl Default for AVIFIELDINDEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIFIELDINDEX_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIINDEXENTRY {
    pub ckid: u32,
    pub dwFlags: u32,
    pub dwChunkOffset: u32,
    pub dwChunkLength: u32,
}
impl Default for AVIINDEXENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIINDEXENTRY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIMAINHEADER {
    pub fcc: u32,
    pub cb: u32,
    pub dwMicroSecPerFrame: u32,
    pub dwMaxBytesPerSec: u32,
    pub dwPaddingGranularity: u32,
    pub dwFlags: u32,
    pub dwTotalFrames: u32,
    pub dwInitialFrames: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwReserved: [u32; 4],
}
impl Default for AVIMAINHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIMAINHEADER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIMETAINDEX {
    pub fcc: u32,
    pub cb: u32,
    pub wLongsPerEntry: u16,
    pub bIndexSubType: u8,
    pub bIndexType: u8,
    pub nEntriesInUse: u32,
    pub dwChunkId: u32,
    pub dwReserved: [u32; 3],
    pub adwIndex: [u32; 1],
}
impl Default for AVIMETAINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIMETAINDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIOLDINDEX {
    pub fcc: u32,
    pub cb: u32,
    pub aIndex: [AVIOLDINDEX_0; 1],
}
impl Default for AVIOLDINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIOLDINDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIOLDINDEX_0 {
    pub dwChunkId: u32,
    pub dwFlags: u32,
    pub dwOffset: u32,
    pub dwSize: u32,
}
impl Default for AVIOLDINDEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIOLDINDEX_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIPALCHANGE {
    pub bFirstEntry: u8,
    pub bNumEntries: u8,
    pub wFlags: u16,
    pub peNew: [super::super::Graphics::Gdi::PALETTEENTRY; 1],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for AVIPALCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for AVIPALCHANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVISTDINDEX {
    pub fcc: u32,
    pub cb: u32,
    pub wLongsPerEntry: u16,
    pub bIndexSubType: u8,
    pub bIndexType: u8,
    pub nEntriesInUse: u32,
    pub dwChunkId: u32,
    pub qwBaseOffset: u64,
    pub dwReserved_3: u32,
    pub aIndex: [AVISTDINDEX_ENTRY; 2044],
}
impl Default for AVISTDINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVISTDINDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVISTDINDEX_ENTRY {
    pub dwOffset: u32,
    pub dwSize: u32,
}
impl Default for AVISTDINDEX_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVISTDINDEX_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVISTREAMHEADER {
    pub fcc: u32,
    pub cb: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwInitialFrames: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: AVISTREAMHEADER_0,
}
impl Default for AVISTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVISTREAMHEADER {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVISTREAMHEADER_0 {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}
impl Default for AVISTREAMHEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVISTREAMHEADER_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVISUPERINDEX {
    pub fcc: u32,
    pub cb: u32,
    pub wLongsPerEntry: u16,
    pub bIndexSubType: u8,
    pub bIndexType: u8,
    pub nEntriesInUse: u32,
    pub dwChunkId: u32,
    pub dwReserved: [u32; 3],
    pub aIndex: [AVISUPERINDEX_0; 1022],
}
impl Default for AVISUPERINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVISUPERINDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVISUPERINDEX_0 {
    pub qwOffset: u64,
    pub dwSize: u32,
    pub dwDuration: u32,
}
impl Default for AVISUPERINDEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVISUPERINDEX_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIStreamHeader {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwInitialFrames: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::super::Foundation::RECT,
}
impl Default for AVIStreamHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVIStreamHeader {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVITCDLINDEX {
    pub fcc: u32,
    pub cb: u32,
    pub wLongsPerEntry: u16,
    pub bIndexSubType: u8,
    pub bIndexType: u8,
    pub nEntriesInUse: u32,
    pub dwChunkId: u32,
    pub dwReserved: [u32; 3],
    pub aIndex: [AVITCDLINDEX_ENTRY; 584],
    pub adwTrailingFill: [u32; 3512],
}
impl Default for AVITCDLINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVITCDLINDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVITCDLINDEX_ENTRY {
    pub dwTick: u32,
    pub time: super::TIMECODE,
    pub dwSMPTEflags: u32,
    pub dwUser: u32,
    pub szReelId: [i8; 12],
}
impl Default for AVITCDLINDEX_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVITCDLINDEX_ENTRY {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVITIMECODEINDEX {
    pub fcc: u32,
    pub cb: u32,
    pub wLongsPerEntry: u16,
    pub bIndexSubType: u8,
    pub bIndexType: u8,
    pub nEntriesInUse: u32,
    pub dwChunkId: u32,
    pub dwReserved: [u32; 3],
    pub aIndex: [TIMECODEDATA; 1022],
}
impl Default for AVITIMECODEINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVITIMECODEINDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVITIMEDINDEX {
    pub fcc: u32,
    pub cb: u32,
    pub wLongsPerEntry: u16,
    pub bIndexSubType: u8,
    pub bIndexType: u8,
    pub nEntriesInUse: u32,
    pub dwChunkId: u32,
    pub qwBaseOffset: u64,
    pub dwReserved_3: u32,
    pub aIndex: [AVITIMEDINDEX_ENTRY; 1362],
    pub adwTrailingFill: [u32; 2734],
}
impl Default for AVITIMEDINDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVITIMEDINDEX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVITIMEDINDEX_ENTRY {
    pub dwOffset: u32,
    pub dwSize: u32,
    pub dwDuration: u32,
}
impl Default for AVITIMEDINDEX_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AVITIMEDINDEX_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDANODE_DESCRIPTOR {
    pub ulBdaNodeType: u32,
    pub guidFunction: windows_core::GUID,
    pub guidName: windows_core::GUID,
}
impl Default for BDANODE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDANODE_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_BUFFER {
    pub lResult: i32,
    pub ulBufferSize: u32,
    pub argbBuffer: [u8; 1],
}
impl Default for BDA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_BUFFER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_CAS_CHECK_ENTITLEMENTTOKEN {
    pub lResult: i32,
    pub ulDescrambleStatus: u32,
}
impl Default for BDA_CAS_CHECK_ENTITLEMENTTOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_CAS_CHECK_ENTITLEMENTTOKEN {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_CAS_CLOSEMMIDATA {
    pub ulDialogNumber: u32,
}
impl Default for BDA_CAS_CLOSEMMIDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_CAS_CLOSEMMIDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_CAS_CLOSE_MMIDIALOG {
    pub lResult: i32,
    pub SessionResult: u32,
}
impl Default for BDA_CAS_CLOSE_MMIDIALOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_CAS_CLOSE_MMIDIALOG {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_CAS_OPENMMIDATA {
    pub ulDialogNumber: u32,
    pub ulDialogRequest: u32,
    pub uuidDialogType: windows_core::GUID,
    pub usDialogDataLength: u16,
    pub argbDialogData: [u8; 1],
}
impl Default for BDA_CAS_OPENMMIDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_CAS_OPENMMIDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_CAS_REQUESTTUNERDATA {
    pub ucRequestPriority: u8,
    pub ucRequestReason: u8,
    pub ucRequestConsequences: u8,
    pub ulEstimatedTime: u32,
}
impl Default for BDA_CAS_REQUESTTUNERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_CAS_REQUESTTUNERDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_CA_MODULE_UI {
    pub ulFormat: u32,
    pub ulbcDesc: u32,
    pub ulDesc: [u32; 1],
}
impl Default for BDA_CA_MODULE_UI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_CA_MODULE_UI {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_DISEQC_RESPONSE {
    pub ulRequestId: u32,
    pub ulPacketLength: u32,
    pub argbPacketData: [u8; 8],
}
impl Default for BDA_DISEQC_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_DISEQC_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_DISEQC_SEND {
    pub ulRequestId: u32,
    pub ulPacketLength: u32,
    pub argbPacketData: [u8; 8],
}
impl Default for BDA_DISEQC_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_DISEQC_SEND {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_DRM_DRMSTATUS {
    pub lResult: i32,
    pub DRMuuid: windows_core::GUID,
    pub ulDrmUuidListStringSize: u32,
    pub argbDrmUuidListString: [windows_core::GUID; 1],
}
impl Default for BDA_DRM_DRMSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_DRM_DRMSTATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_DVBT2_L1_SIGNALLING_DATA {
    pub L1Pre_TYPE: u8,
    pub L1Pre_BWT_S1_S2: u8,
    pub L1Pre_REPETITION_GUARD_PAPR: u8,
    pub L1Pre_MOD_COD_FEC: u8,
    pub L1Pre_POSTSIZE_INFO_PILOT: [u8; 5],
    pub L1Pre_TX_ID_AVAIL: u8,
    pub L1Pre_CELL_ID: [u8; 2],
    pub L1Pre_NETWORK_ID: [u8; 2],
    pub L1Pre_T2SYSTEM_ID: [u8; 2],
    pub L1Pre_NUM_T2_FRAMES: u8,
    pub L1Pre_NUM_DATA_REGENFLAG_L1POSTEXT: [u8; 2],
    pub L1Pre_NUMRF_CURRENTRF_RESERVED: [u8; 2],
    pub L1Pre_CRC32: [u8; 4],
    pub L1PostData: [u8; 1],
}
impl Default for BDA_DVBT2_L1_SIGNALLING_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_DVBT2_L1_SIGNALLING_DATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_ETHERNET_ADDRESS {
    pub rgbAddress: [u8; 6],
}
impl Default for BDA_ETHERNET_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_ETHERNET_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_ETHERNET_ADDRESS_LIST {
    pub ulcAddresses: u32,
    pub rgAddressl: [BDA_ETHERNET_ADDRESS; 1],
}
impl Default for BDA_ETHERNET_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_ETHERNET_ADDRESS_LIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_GDDS_DATA {
    pub lResult: i32,
    pub ulDataLength: u32,
    pub ulPercentageProgress: u32,
    pub argbData: [u8; 1],
}
impl Default for BDA_GDDS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_GDDS_DATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_GDDS_DATATYPE {
    pub lResult: i32,
    pub uuidDataType: windows_core::GUID,
}
impl Default for BDA_GDDS_DATATYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_GDDS_DATATYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_IPv4_ADDRESS {
    pub rgbAddress: [u8; 4],
}
impl Default for BDA_IPv4_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_IPv4_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_IPv4_ADDRESS_LIST {
    pub ulcAddresses: u32,
    pub rgAddressl: [BDA_IPv4_ADDRESS; 1],
}
impl Default for BDA_IPv4_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_IPv4_ADDRESS_LIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_IPv6_ADDRESS {
    pub rgbAddress: [u8; 6],
}
impl Default for BDA_IPv6_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_IPv6_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_IPv6_ADDRESS_LIST {
    pub ulcAddresses: u32,
    pub rgAddressl: [BDA_IPv6_ADDRESS; 1],
}
impl Default for BDA_IPv6_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_IPv6_ADDRESS_LIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_ISDBCAS_EMG_REQ {
    pub bCLA: u8,
    pub bINS: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bLC: u8,
    pub bCardId: [u8; 6],
    pub bProtocol: u8,
    pub bCABroadcasterGroupId: u8,
    pub bMessageControl: u8,
    pub bMessageCode: [u8; 1],
}
impl Default for BDA_ISDBCAS_EMG_REQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_ISDBCAS_EMG_REQ {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_ISDBCAS_REQUESTHEADER {
    pub bInstruction: u8,
    pub bReserved: [u8; 3],
    pub ulDataLength: u32,
    pub argbIsdbCommand: [u8; 1],
}
impl Default for BDA_ISDBCAS_REQUESTHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_ISDBCAS_REQUESTHEADER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_ISDBCAS_RESPONSEDATA {
    pub lResult: i32,
    pub ulRequestID: u32,
    pub ulIsdbStatus: u32,
    pub ulIsdbDataSize: u32,
    pub argbIsdbCommandData: [u8; 1],
}
impl Default for BDA_ISDBCAS_RESPONSEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_ISDBCAS_RESPONSEDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_MUX_PIDLISTITEM {
    pub usPIDNumber: u16,
    pub usProgramNumber: u16,
    pub ePIDType: MUX_PID_TYPE,
}
impl Default for BDA_MUX_PIDLISTITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_MUX_PIDLISTITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_PID_MAP {
    pub MediaSampleContent: MEDIA_SAMPLE_CONTENT,
    pub ulcPIDs: u32,
    pub aulPIDs: [u32; 1],
}
impl Default for BDA_PID_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_PID_MAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_PID_UNMAP {
    pub ulcPIDs: u32,
    pub aulPIDs: [u32; 1],
}
impl Default for BDA_PID_UNMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_PID_UNMAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_PROGRAM_PID_LIST {
    pub ulProgramNumber: u32,
    pub ulcPIDs: u32,
    pub ulPID: [u32; 1],
}
impl Default for BDA_PROGRAM_PID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_PROGRAM_PID_LIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_RATING_PINRESET {
    pub bPinLength: u8,
    pub argbNewPin: [u8; 1],
}
impl Default for BDA_RATING_PINRESET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_RATING_PINRESET {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_SCAN_CAPABILTIES {
    pub lResult: i32,
    pub ul64AnalogStandardsSupported: u64,
}
impl Default for BDA_SCAN_CAPABILTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_SCAN_CAPABILTIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_SCAN_START {
    pub lResult: i32,
    pub LowerFrequency: u32,
    pub HigerFrequency: u32,
}
impl Default for BDA_SCAN_START {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_SCAN_START {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_SCAN_STATE {
    pub lResult: i32,
    pub ulSignalLock: u32,
    pub ulSecondsLeft: u32,
    pub ulCurrentFrequency: u32,
}
impl Default for BDA_SCAN_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_SCAN_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_SIGNAL_TIMEOUTS {
    pub ulCarrierTimeoutMs: u32,
    pub ulScanningTimeoutMs: u32,
    pub ulTuningTimeoutMs: u32,
}
impl Default for BDA_SIGNAL_TIMEOUTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_SIGNAL_TIMEOUTS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_STRING {
    pub lResult: i32,
    pub ulStringSize: u32,
    pub argbString: [u8; 1],
}
impl Default for BDA_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_STRING {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_TABLE_SECTION {
    pub ulPrimarySectionId: u32,
    pub ulSecondarySectionId: u32,
    pub ulcbSectionLength: u32,
    pub argbSectionData: [u32; 1],
}
impl Default for BDA_TABLE_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_TABLE_SECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_TEMPLATE_CONNECTION {
    pub FromNodeType: u32,
    pub FromNodePinType: u32,
    pub ToNodeType: u32,
    pub ToNodePinType: u32,
}
impl Default for BDA_TEMPLATE_CONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_TEMPLATE_CONNECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_TEMPLATE_PIN_JOINT {
    pub uliTemplateConnection: u32,
    pub ulcInstancesMax: u32,
}
impl Default for BDA_TEMPLATE_PIN_JOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_TEMPLATE_PIN_JOINT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_TS_SELECTORINFO {
    pub bTSInfolength: u8,
    pub bReserved: [u8; 2],
    pub guidNetworkType: windows_core::GUID,
    pub bTSIDCount: u8,
    pub usTSID: [u16; 1],
}
impl Default for BDA_TS_SELECTORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_TS_SELECTORINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_TS_SELECTORINFO_ISDBS_EXT {
    pub bTMCC: [u8; 48],
}
impl Default for BDA_TS_SELECTORINFO_ISDBS_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_TS_SELECTORINFO_ISDBS_EXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_TUNER_DIAGNOSTICS {
    pub lResult: i32,
    pub ulSignalLevel: u32,
    pub ulSignalLevelQuality: u32,
    pub ulSignalNoiseRatio: u32,
}
impl Default for BDA_TUNER_DIAGNOSTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_TUNER_DIAGNOSTICS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_TUNER_TUNERSTATE {
    pub lResult: i32,
    pub ulTuneLength: u32,
    pub argbTuneData: [u8; 1],
}
impl Default for BDA_TUNER_TUNERSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_TUNER_TUNERSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_USERACTIVITY_INTERVAL {
    pub lResult: i32,
    pub ulActivityInterval: u32,
}
impl Default for BDA_USERACTIVITY_INTERVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_USERACTIVITY_INTERVAL {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_WMDRMTUNER_PIDPROTECTION {
    pub lResult: i32,
    pub uuidKeyID: windows_core::GUID,
}
impl Default for BDA_WMDRMTUNER_PIDPROTECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_WMDRMTUNER_PIDPROTECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_WMDRMTUNER_PURCHASEENTITLEMENT {
    pub lResult: i32,
    pub ulDescrambleStatus: u32,
    pub ulCaptureTokenLength: u32,
    pub argbCaptureTokenBuffer: [u8; 1],
}
impl Default for BDA_WMDRMTUNER_PURCHASEENTITLEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_WMDRMTUNER_PURCHASEENTITLEMENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_WMDRM_KEYINFOLIST {
    pub lResult: i32,
    pub ulKeyuuidBufferLen: u32,
    pub argKeyuuidBuffer: [windows_core::GUID; 1],
}
impl Default for BDA_WMDRM_KEYINFOLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_WMDRM_KEYINFOLIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_WMDRM_RENEWLICENSE {
    pub lResult: i32,
    pub ulDescrambleStatus: u32,
    pub ulXmrLicenseOutputLength: u32,
    pub argbXmrLicenceOutputBuffer: [u8; 1],
}
impl Default for BDA_WMDRM_RENEWLICENSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_WMDRM_RENEWLICENSE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BDA_WMDRM_STATUS {
    pub lResult: i32,
    pub ulMaxCaptureTokenSize: u32,
    pub uMaxStreamingPid: u32,
    pub ulMaxLicense: u32,
    pub ulMinSecurityLevel: u32,
    pub ulRevInfoSequenceNumber: u32,
    pub ulRevInfoIssuedTime: u64,
    pub ulRevListVersion: u32,
    pub ulRevInfoTTL: u32,
    pub ulState: u32,
}
impl Default for BDA_WMDRM_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BDA_WMDRM_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COLORKEY {
    pub KeyType: u32,
    pub PaletteIndex: u32,
    pub LowColorValue: super::super::Foundation::COLORREF,
    pub HighColorValue: super::super::Foundation::COLORREF,
}
impl Default for COLORKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COLORKEY {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_ATR {
    pub ulCAT: u32,
    pub pbATRI: [u8; 768],
}
impl Default for DVD_ATR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_ATR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_AudioAttributes {
    pub AppMode: DVD_AUDIO_APPMODE,
    pub AppModeData: u8,
    pub AudioFormat: DVD_AUDIO_FORMAT,
    pub Language: u32,
    pub LanguageExtension: DVD_AUDIO_LANG_EXT,
    pub fHasMultichannelInfo: super::super::Foundation::BOOL,
    pub dwFrequency: u32,
    pub bQuantization: u8,
    pub bNumberOfChannels: u8,
    pub dwReserved: [u32; 2],
}
impl Default for DVD_AudioAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_AudioAttributes {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for DVD_DECODER_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_DECODER_CAPS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_HMSF_TIMECODE {
    pub bHours: u8,
    pub bMinutes: u8,
    pub bSeconds: u8,
    pub bFrames: u8,
}
impl Default for DVD_HMSF_TIMECODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_HMSF_TIMECODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_KaraokeAttributes {
    pub bVersion: u8,
    pub fMasterOfCeremoniesInGuideVocal1: super::super::Foundation::BOOL,
    pub fDuet: super::super::Foundation::BOOL,
    pub ChannelAssignment: DVD_KARAOKE_ASSIGNMENT,
    pub wChannelContents: [u16; 8],
}
impl Default for DVD_KaraokeAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_KaraokeAttributes {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_MUA_Coeff {
    pub log2_alpha: f64,
    pub log2_beta: f64,
}
impl Default for DVD_MUA_Coeff {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_MUA_Coeff {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_MUA_MixingInfo {
    pub fMixTo0: super::super::Foundation::BOOL,
    pub fMixTo1: super::super::Foundation::BOOL,
    pub fMix0InPhase: super::super::Foundation::BOOL,
    pub fMix1InPhase: super::super::Foundation::BOOL,
    pub dwSpeakerPosition: u32,
}
impl Default for DVD_MUA_MixingInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_MUA_MixingInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_MenuAttributes {
    pub fCompatibleRegion: [super::super::Foundation::BOOL; 8],
    pub VideoAttributes: DVD_VideoAttributes,
    pub fAudioPresent: super::super::Foundation::BOOL,
    pub AudioAttributes: DVD_AudioAttributes,
    pub fSubpicturePresent: super::super::Foundation::BOOL,
    pub SubpictureAttributes: DVD_SubpictureAttributes,
}
impl Default for DVD_MenuAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_MenuAttributes {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_MultichannelAudioAttributes {
    pub Info: [DVD_MUA_MixingInfo; 8],
    pub Coeff: [DVD_MUA_Coeff; 8],
}
impl Default for DVD_MultichannelAudioAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_MultichannelAudioAttributes {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_PLAYBACK_LOCATION {
    pub TitleNum: u32,
    pub ChapterNum: u32,
    pub TimeCode: u32,
}
impl Default for DVD_PLAYBACK_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_PLAYBACK_LOCATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_PLAYBACK_LOCATION2 {
    pub TitleNum: u32,
    pub ChapterNum: u32,
    pub TimeCode: DVD_HMSF_TIMECODE,
    pub TimeCodeFlags: u32,
}
impl Default for DVD_PLAYBACK_LOCATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_PLAYBACK_LOCATION2 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_REGION {
    pub CopySystem: u8,
    pub RegionData: u8,
    pub SystemRegion: u8,
    pub ResetCount: u8,
}
impl Default for DVD_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_REGION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_SubpictureAttributes {
    pub Type: DVD_SUBPICTURE_TYPE,
    pub CodingMode: DVD_SUBPICTURE_CODING,
    pub Language: u32,
    pub LanguageExtension: DVD_SUBPICTURE_LANG_EXT,
}
impl Default for DVD_SubpictureAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_SubpictureAttributes {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_TIMECODE {
    pub _bitfield: u32,
}
impl Default for DVD_TIMECODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_TIMECODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_TitleAttributes {
    pub Anonymous: DVD_TitleAttributes_0,
    pub VideoAttributes: DVD_VideoAttributes,
    pub ulNumberOfAudioStreams: u32,
    pub AudioAttributes: [DVD_AudioAttributes; 8],
    pub MultichannelAudioAttributes: [DVD_MultichannelAudioAttributes; 8],
    pub ulNumberOfSubpictureStreams: u32,
    pub SubpictureAttributes: [DVD_SubpictureAttributes; 32],
}
impl Default for DVD_TitleAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_TitleAttributes {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union DVD_TitleAttributes_0 {
    pub AppMode: DVD_TITLE_APPMODE,
    pub TitleLength: DVD_HMSF_TIMECODE,
}
impl Default for DVD_TitleAttributes_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_TitleAttributes_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVD_VideoAttributes {
    pub fPanscanPermitted: super::super::Foundation::BOOL,
    pub fLetterboxPermitted: super::super::Foundation::BOOL,
    pub ulAspectX: u32,
    pub ulAspectY: u32,
    pub ulFrameRate: u32,
    pub ulFrameHeight: u32,
    pub Compression: DVD_VIDEO_COMPRESSION,
    pub fLine21Field1InGOP: super::super::Foundation::BOOL,
    pub fLine21Field2InGOP: super::super::Foundation::BOOL,
    pub ulSourceResolutionX: u32,
    pub ulSourceResolutionY: u32,
    pub fIsSourceLetterboxed: super::super::Foundation::BOOL,
    pub fIsFilmMode: super::super::Foundation::BOOL,
}
impl Default for DVD_VideoAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVD_VideoAttributes {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl windows_core::TypeKind for DVINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2SW_CALLBACKS {
    pub Size: u32,
    pub GetVideoProcessorRenderTargetCount: PDXVA2SW_GETVIDEOPROCESSORRENDERTARGETCOUNT,
    pub GetVideoProcessorRenderTargets: PDXVA2SW_GETVIDEOPROCESSORRENDERTARGETS,
    pub GetVideoProcessorCaps: PDXVA2SW_GETVIDEOPROCESSORCAPS,
    pub GetVideoProcessorSubStreamFormatCount: PDXVA2SW_GETVIDEOPROCESSORSUBSTREAMFORMATCOUNT,
    pub GetVideoProcessorSubStreamFormats: PDXVA2SW_GETVIDEOPROCESSORSUBSTREAMFORMATS,
    pub GetProcAmpRange: PDXVA2SW_GETPROCAMPRANGE,
    pub GetFilterPropertyRange: PDXVA2SW_GETFILTERPROPERTYRANGE,
    pub CreateVideoProcessDevice: PDXVA2SW_CREATEVIDEOPROCESSDEVICE,
    pub DestroyVideoProcessDevice: PDXVA2SW_DESTROYVIDEOPROCESSDEVICE,
    pub VideoProcessBeginFrame: PDXVA2SW_VIDEOPROCESSBEGINFRAME,
    pub VideoProcessEndFrame: PDXVA2SW_VIDEOPROCESSENDFRAME,
    pub VideoProcessSetRenderTarget: PDXVA2SW_VIDEOPROCESSSETRENDERTARGET,
    pub VideoProcessBlt: PDXVA2SW_VIDEOPROCESSBLT,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
impl Default for DXVA2SW_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::TypeKind for DXVA2SW_CALLBACKS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2TraceVideoProcessBltData {
    pub wmiHeader: super::super::System::Diagnostics::Etw::EVENT_TRACE_HEADER,
    pub pObject: u64,
    pub pRenderTarget: u64,
    pub TargetFrameTime: u64,
    pub TargetRect: super::super::Foundation::RECT,
    pub Enter: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl Default for DXVA2TraceVideoProcessBltData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl windows_core::TypeKind for DXVA2TraceVideoProcessBltData {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2Trace_DecodeDevBeginFrameData {
    pub wmiHeader: super::super::System::Diagnostics::Etw::EVENT_TRACE_HEADER,
    pub pObject: u64,
    pub pRenderTarget: u64,
    pub Enter: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl Default for DXVA2Trace_DecodeDevBeginFrameData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl windows_core::TypeKind for DXVA2Trace_DecodeDevBeginFrameData {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2Trace_DecodeDevCreatedData {
    pub wmiHeader: super::super::System::Diagnostics::Etw::EVENT_TRACE_HEADER,
    pub pObject: u64,
    pub pD3DDevice: u64,
    pub DeviceGuid: windows_core::GUID,
    pub Width: u32,
    pub Height: u32,
    pub Enter: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl Default for DXVA2Trace_DecodeDevCreatedData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl windows_core::TypeKind for DXVA2Trace_DecodeDevCreatedData {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2Trace_DecodeDevGetBufferData {
    pub wmiHeader: super::super::System::Diagnostics::Etw::EVENT_TRACE_HEADER,
    pub pObject: u64,
    pub BufferType: u32,
    pub Enter: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl Default for DXVA2Trace_DecodeDevGetBufferData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl windows_core::TypeKind for DXVA2Trace_DecodeDevGetBufferData {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2Trace_DecodeDeviceData {
    pub wmiHeader: super::super::System::Diagnostics::Etw::EVENT_TRACE_HEADER,
    pub pObject: u64,
    pub Enter: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl Default for DXVA2Trace_DecodeDeviceData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl windows_core::TypeKind for DXVA2Trace_DecodeDeviceData {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2Trace_VideoProcessDevCreatedData {
    pub wmiHeader: super::super::System::Diagnostics::Etw::EVENT_TRACE_HEADER,
    pub pObject: u64,
    pub pD3DDevice: u64,
    pub DeviceGuid: windows_core::GUID,
    pub RTFourCC: u32,
    pub Width: u32,
    pub Height: u32,
    pub Enter: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl Default for DXVA2Trace_VideoProcessDevCreatedData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl windows_core::TypeKind for DXVA2Trace_VideoProcessDevCreatedData {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2Trace_VideoProcessDeviceData {
    pub wmiHeader: super::super::System::Diagnostics::Etw::EVENT_TRACE_HEADER,
    pub pObject: u64,
    pub Enter: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl Default for DXVA2Trace_VideoProcessDeviceData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
impl windows_core::TypeKind for DXVA2Trace_VideoProcessDeviceData {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Media_MediaFoundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2_VIDEOPROCESSBLT {
    pub TargetFrame: i64,
    pub TargetRect: super::super::Foundation::RECT,
    pub ConstrictionSize: super::super::Foundation::SIZE,
    pub StreamingFlags: u32,
    pub BackgroundColor: super::MediaFoundation::DXVA2_AYUVSample16,
    pub DestFormat: super::MediaFoundation::DXVA2_ExtendedFormat,
    pub DestFlags: u32,
    pub ProcAmpValues: super::MediaFoundation::DXVA2_ProcAmpValues,
    pub Alpha: super::MediaFoundation::DXVA2_Fixed32,
    pub NoiseFilterLuma: super::MediaFoundation::DXVA2_FilterValues,
    pub NoiseFilterChroma: super::MediaFoundation::DXVA2_FilterValues,
    pub DetailFilterLuma: super::MediaFoundation::DXVA2_FilterValues,
    pub DetailFilterChroma: super::MediaFoundation::DXVA2_FilterValues,
    pub pSrcSurfaces: *mut DXVA2_VIDEOSAMPLE,
    pub NumSrcSurfaces: u32,
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl Default for DXVA2_VIDEOPROCESSBLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::TypeKind for DXVA2_VIDEOPROCESSBLT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Media_MediaFoundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA2_VIDEOSAMPLE {
    pub Start: i64,
    pub End: i64,
    pub SampleFormat: super::MediaFoundation::DXVA2_ExtendedFormat,
    pub SampleFlags: u32,
    pub SrcResource: *mut core::ffi::c_void,
    pub SrcRect: super::super::Foundation::RECT,
    pub DstRect: super::super::Foundation::RECT,
    pub Pal: [super::MediaFoundation::DXVA2_AYUVSample8; 16],
    pub PlanarAlpha: super::MediaFoundation::DXVA2_Fixed32,
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl Default for DXVA2_VIDEOSAMPLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::TypeKind for DXVA2_VIDEOSAMPLE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA_COPPSetProtectionLevelCmdData {
    pub ProtType: u32,
    pub ProtLevel: u32,
    pub ExtendedInfoChangeMask: u32,
    pub ExtendedInfoData: u32,
}
impl Default for DXVA_COPPSetProtectionLevelCmdData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DXVA_COPPSetProtectionLevelCmdData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl windows_core::TypeKind for DXVA_COPPSetSignalingCmdData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA_COPPStatusData {
    pub rApp: windows_core::GUID,
    pub dwFlags: u32,
    pub dwData: u32,
    pub ExtendedInfoValidMask: u32,
    pub ExtendedInfoData: u32,
}
impl Default for DXVA_COPPStatusData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DXVA_COPPStatusData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA_COPPStatusDisplayData {
    pub rApp: windows_core::GUID,
    pub dwFlags: u32,
    pub DisplayWidth: u32,
    pub DisplayHeight: u32,
    pub Format: u32,
    pub d3dFormat: u32,
    pub FreqNumerator: u32,
    pub FreqDenominator: u32,
}
impl Default for DXVA_COPPStatusDisplayData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DXVA_COPPStatusDisplayData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA_COPPStatusHDCPKeyData {
    pub rApp: windows_core::GUID,
    pub dwFlags: u32,
    pub dwHDCPFlags: u32,
    pub BKey: windows_core::GUID,
    pub Reserved1: windows_core::GUID,
    pub Reserved2: windows_core::GUID,
}
impl Default for DXVA_COPPStatusHDCPKeyData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DXVA_COPPStatusHDCPKeyData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXVA_COPPStatusSignalingCmdData {
    pub rApp: windows_core::GUID,
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
impl windows_core::TypeKind for DXVA_COPPStatusSignalingCmdData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EALocationCodeType {
    pub LocationCodeScheme: LocationCodeSchemeType,
    pub state_code: u8,
    pub county_subdivision: u8,
    pub county_code: u16,
}
impl Default for EALocationCodeType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EALocationCodeType {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILTER_INFO {
    pub achName: [u16; 128],
    pub pGraph: Option<IFilterGraph>,
}
impl Default for FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FILTER_INFO {
    type TypeKind = windows_core::CloneType;
}
pub const FilgraphManager: windows_core::GUID = windows_core::GUID::from_u128(0xe436ebb3_524f_11ce_9f53_0020af0ba770);
#[repr(C)]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HEAACWAVEFORMAT {
    pub wfInfo: HEAACWAVEINFO,
    pub pbAudioSpecificConfig: [u8; 1],
}
#[cfg(feature = "Win32_Media_Audio")]
impl Default for HEAACWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::TypeKind for HEAACWAVEFORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HEAACWAVEINFO {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wPayloadType: u16,
    pub wAudioProfileLevelIndication: u16,
    pub wStructType: u16,
    pub wReserved1: u16,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Media_Audio")]
impl Default for HEAACWAVEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::TypeKind for HEAACWAVEINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KS_BDA_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub ulEvent: u32,
    pub ulChannelNumber: u32,
    pub ulSubchannelNumber: u32,
    pub ulReason: u32,
}
impl Default for KS_BDA_FRAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for KS_BDA_FRAME_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPEG1WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub fwHeadLayer: u16,
    pub dwHeadBitrate: u32,
    pub fwHeadMode: u16,
    pub fwHeadModeExt: u16,
    pub wHeadEmphasis: u16,
    pub fwHeadFlags: u16,
    pub dwPTSLow: u32,
    pub dwPTSHigh: u32,
}
#[cfg(feature = "Win32_Media_Audio")]
impl Default for MPEG1WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::TypeKind for MPEG1WAVEFORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPEG2_TRANSPORT_STRIDE {
    pub dwOffset: u32,
    pub dwPacketLength: u32,
    pub dwStride: u32,
}
impl Default for MPEG2_TRANSPORT_STRIDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MPEG2_TRANSPORT_STRIDE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Audio")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPEGLAYER3WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wID: u16,
    pub fdwFlags: MPEGLAYER3WAVEFORMAT_FLAGS,
    pub nBlockSize: u16,
    pub nFramesPerBlock: u16,
    pub nCodecDelay: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl Default for MPEGLAYER3WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::TypeKind for MPEGLAYER3WAVEFORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MP_ENVELOPE_SEGMENT {
    pub rtStart: i64,
    pub rtEnd: i64,
    pub valStart: f32,
    pub valEnd: f32,
    pub iCurve: MP_CURVE_TYPE,
    pub flags: u32,
}
impl Default for MP_ENVELOPE_SEGMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MP_ENVELOPE_SEGMENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MP_PARAMINFO {
    pub mpType: MP_TYPE,
    pub mopCaps: u32,
    pub mpdMinValue: f32,
    pub mpdMaxValue: f32,
    pub mpdNeutralValue: f32,
    pub szUnitText: [u16; 32],
    pub szLabel: [u16; 32],
}
impl Default for MP_PARAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MP_PARAMINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MainAVIHeader {
    pub dwMicroSecPerFrame: u32,
    pub dwMaxBytesPerSec: u32,
    pub dwPaddingGranularity: u32,
    pub dwFlags: u32,
    pub dwTotalFrames: u32,
    pub dwInitialFrames: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwReserved: [u32; 4],
}
impl Default for MainAVIHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MainAVIHeader {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NORMALIZEDRECT {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl Default for NORMALIZEDRECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NORMALIZEDRECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PID_MAP {
    pub ulPID: u32,
    pub MediaSampleContent: MEDIA_SAMPLE_CONTENT,
}
impl Default for PID_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PID_MAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PIN_INFO {
    pub pFilter: Option<IBaseFilter>,
    pub dir: PIN_DIRECTION,
    pub achName: [u16; 128],
}
impl Default for PIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PIN_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quality {
    pub Type: QualityMessageType,
    pub Proportion: i32,
    pub Late: i64,
    pub TimeStamp: i64,
}
impl Default for Quality {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Quality {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REGFILTER {
    pub Clsid: windows_core::GUID,
    pub Name: windows_core::PWSTR,
}
impl Default for REGFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REGFILTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl windows_core::TypeKind for REGFILTER2 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union REGFILTER2_0 {
    pub Anonymous1: REGFILTER2_0_0,
    pub Anonymous2: REGFILTER2_0_1,
}
impl Default for REGFILTER2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REGFILTER2_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REGFILTER2_0_0 {
    pub cPins: u32,
    pub rgPins: *const REGFILTERPINS,
}
impl Default for REGFILTER2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REGFILTER2_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REGFILTER2_0_1 {
    pub cPins2: u32,
    pub rgPins2: *const REGFILTERPINS2,
}
impl Default for REGFILTER2_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REGFILTER2_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REGFILTERPINS {
    pub strName: windows_core::PWSTR,
    pub bRendered: super::super::Foundation::BOOL,
    pub bOutput: super::super::Foundation::BOOL,
    pub bZero: super::super::Foundation::BOOL,
    pub bMany: super::super::Foundation::BOOL,
    pub clsConnectsToFilter: *const windows_core::GUID,
    pub strConnectsToPin: windows_core::PCWSTR,
    pub nMediaTypes: u32,
    pub lpMediaType: *const REGPINTYPES,
}
impl Default for REGFILTERPINS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REGFILTERPINS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REGFILTERPINS2 {
    pub dwFlags: u32,
    pub cInstances: u32,
    pub nMediaTypes: u32,
    pub lpMediaType: *const REGPINTYPES,
    pub nMediums: u32,
    pub lpMedium: *const REGPINMEDIUM,
    pub clsPinCategory: *const windows_core::GUID,
}
impl Default for REGFILTERPINS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REGFILTERPINS2 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REGPINMEDIUM {
    pub clsMedium: windows_core::GUID,
    pub dw1: u32,
    pub dw2: u32,
}
impl Default for REGPINMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REGPINMEDIUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REGPINTYPES {
    pub clsMajorType: *const windows_core::GUID,
    pub clsMinorType: *const windows_core::GUID,
}
impl Default for REGPINTYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REGPINTYPES {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RIFFCHUNK {
    pub fcc: u32,
    pub cb: u32,
}
impl Default for RIFFCHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RIFFCHUNK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RIFFLIST {
    pub fcc: u32,
    pub cb: u32,
    pub fccListType: u32,
}
impl Default for RIFFLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RIFFLIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STREAM_ID_MAP {
    pub stream_id: u32,
    pub dwMediaSampleContent: u32,
    pub ulSubstreamFilterValue: u32,
    pub iDataOffset: i32,
}
impl Default for STREAM_ID_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STREAM_ID_MAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SmartCardApplication {
    pub ApplicationType: ApplicationTypeType,
    pub ApplicationVersion: u16,
    pub pbstrApplicationName: windows_core::BSTR,
    pub pbstrApplicationURL: windows_core::BSTR,
}
impl Default for SmartCardApplication {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SmartCardApplication {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TIMECODEDATA {
    pub time: super::TIMECODE,
    pub dwSMPTEflags: u32,
    pub dwUser: u32,
}
impl Default for TIMECODEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TIMECODEDATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRUECOLORINFO {
    pub dwBitMasks: [u32; 3],
    pub bmiColors: [super::super::Graphics::Gdi::RGBQUAD; 256],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for TRUECOLORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for TRUECOLORINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VFW_FILTERLIST {
    pub cFilters: u32,
    pub aClsId: [windows_core::GUID; 1],
}
impl Default for VFW_FILTERLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VFW_FILTERLIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEOINFO {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub Anonymous: VIDEOINFO_0,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for VIDEOINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union VIDEOINFO_0 {
    pub bmiColors: [super::super::Graphics::Gdi::RGBQUAD; 256],
    pub dwBitMasks: [u32; 3],
    pub TrueColorInfo: TRUECOLORINFO,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for VIDEOINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for VIDEOINFO_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEO_STREAM_CONFIG_CAPS {
    pub guid: windows_core::GUID,
    pub VideoStandard: u32,
    pub InputSize: super::super::Foundation::SIZE,
    pub MinCroppingSize: super::super::Foundation::SIZE,
    pub MaxCroppingSize: super::super::Foundation::SIZE,
    pub CropGranularityX: i32,
    pub CropGranularityY: i32,
    pub CropAlignX: i32,
    pub CropAlignY: i32,
    pub MinOutputSize: super::super::Foundation::SIZE,
    pub MaxOutputSize: super::super::Foundation::SIZE,
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
impl Default for VIDEO_STREAM_CONFIG_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VIDEO_STREAM_CONFIG_CAPS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9AllocationInfo {
    pub dwFlags: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub Format: super::super::Graphics::Direct3D9::D3DFORMAT,
    pub Pool: super::super::Graphics::Direct3D9::D3DPOOL,
    pub MinBuffers: u32,
    pub szAspectRatio: super::super::Foundation::SIZE,
    pub szNativeSize: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl Default for VMR9AllocationInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl windows_core::TypeKind for VMR9AllocationInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9AlphaBitmap {
    pub dwFlags: u32,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub pDDS: Option<super::super::Graphics::Direct3D9::IDirect3DSurface9>,
    pub rSrc: super::super::Foundation::RECT,
    pub rDest: VMR9NormalizedRect,
    pub fAlpha: f32,
    pub clrSrcKey: super::super::Foundation::COLORREF,
    pub dwFilterMode: u32,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
impl Default for VMR9AlphaBitmap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
impl windows_core::TypeKind for VMR9AlphaBitmap {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9DeinterlaceCaps {
    pub dwSize: u32,
    pub dwNumPreviousOutputFrames: u32,
    pub dwNumForwardRefSamples: u32,
    pub dwNumBackwardRefSamples: u32,
    pub DeinterlaceTechnology: VMR9DeinterlaceTech,
}
impl Default for VMR9DeinterlaceCaps {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMR9DeinterlaceCaps {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9Frequency {
    pub dwNumerator: u32,
    pub dwDenominator: u32,
}
impl Default for VMR9Frequency {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMR9Frequency {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9MonitorInfo {
    pub uDevID: u32,
    pub rcMonitor: super::super::Foundation::RECT,
    pub hMon: super::super::Graphics::Gdi::HMONITOR,
    pub dwFlags: u32,
    pub szDevice: [u16; 32],
    pub szDescription: [u16; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for VMR9MonitorInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for VMR9MonitorInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9NormalizedRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl Default for VMR9NormalizedRect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMR9NormalizedRect {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9PresentationInfo {
    pub dwFlags: u32,
    pub lpSurf: Option<super::super::Graphics::Direct3D9::IDirect3DSurface9>,
    pub rtStart: i64,
    pub rtEnd: i64,
    pub szAspectRatio: super::super::Foundation::SIZE,
    pub rcSrc: super::super::Foundation::RECT,
    pub rcDst: super::super::Foundation::RECT,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl Default for VMR9PresentationInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl windows_core::TypeKind for VMR9PresentationInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9ProcAmpControl {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub Brightness: f32,
    pub Contrast: f32,
    pub Hue: f32,
    pub Saturation: f32,
}
impl Default for VMR9ProcAmpControl {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMR9ProcAmpControl {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9ProcAmpControlRange {
    pub dwSize: u32,
    pub dwProperty: VMR9ProcAmpControlFlags,
    pub MinValue: f32,
    pub MaxValue: f32,
    pub DefaultValue: f32,
    pub StepSize: f32,
}
impl Default for VMR9ProcAmpControlRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMR9ProcAmpControlRange {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9VideoDesc {
    pub dwSize: u32,
    pub dwSampleWidth: u32,
    pub dwSampleHeight: u32,
    pub SampleFormat: VMR9_SampleFormat,
    pub dwFourCC: u32,
    pub InputSampleFreq: VMR9Frequency,
    pub OutputFrameFreq: VMR9Frequency,
}
impl Default for VMR9VideoDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMR9VideoDesc {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMR9VideoStreamInfo {
    pub pddsVideoSurface: Option<super::super::Graphics::Direct3D9::IDirect3DSurface9>,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwStrmID: u32,
    pub fAlpha: f32,
    pub rNormal: VMR9NormalizedRect,
    pub rtStart: i64,
    pub rtEnd: i64,
    pub SampleFormat: VMR9_SampleFormat,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl Default for VMR9VideoStreamInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl windows_core::TypeKind for VMR9VideoStreamInfo {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRALLOCATIONINFO {
    pub dwFlags: u32,
    pub lpHdr: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpPixFmt: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub szAspectRatio: super::super::Foundation::SIZE,
    pub dwMinBuffers: u32,
    pub dwMaxBuffers: u32,
    pub dwInterlaceFlags: u32,
    pub szNativeSize: super::super::Foundation::SIZE,
}
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl Default for VMRALLOCATIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl windows_core::TypeKind for VMRALLOCATIONINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRALPHABITMAP {
    pub dwFlags: u32,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub pDDS: Option<super::super::Graphics::DirectDraw::IDirectDrawSurface7>,
    pub rSrc: super::super::Foundation::RECT,
    pub rDest: NORMALIZEDRECT,
    pub fAlpha: f32,
    pub clrSrcKey: super::super::Foundation::COLORREF,
}
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl Default for VMRALPHABITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl windows_core::TypeKind for VMRALPHABITMAP {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRDeinterlaceCaps {
    pub dwSize: u32,
    pub dwNumPreviousOutputFrames: u32,
    pub dwNumForwardRefSamples: u32,
    pub dwNumBackwardRefSamples: u32,
    pub DeinterlaceTechnology: VMRDeinterlaceTech,
}
impl Default for VMRDeinterlaceCaps {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMRDeinterlaceCaps {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRFrequency {
    pub dwNumerator: u32,
    pub dwDenominator: u32,
}
impl Default for VMRFrequency {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMRFrequency {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRGUID {
    pub pGUID: *mut windows_core::GUID,
    pub GUID: windows_core::GUID,
}
impl Default for VMRGUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMRGUID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRMONITORINFO {
    pub guid: VMRGUID,
    pub rcMonitor: super::super::Foundation::RECT,
    pub hMon: super::super::Graphics::Gdi::HMONITOR,
    pub dwFlags: u32,
    pub szDevice: [u16; 32],
    pub szDescription: [u16; 256],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for VMRMONITORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for VMRMONITORINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRPRESENTATIONINFO {
    pub dwFlags: u32,
    pub lpSurf: Option<super::super::Graphics::DirectDraw::IDirectDrawSurface7>,
    pub rtStart: i64,
    pub rtEnd: i64,
    pub szAspectRatio: super::super::Foundation::SIZE,
    pub rcSrc: super::super::Foundation::RECT,
    pub rcDst: super::super::Foundation::RECT,
    pub dwTypeSpecificFlags: u32,
    pub dwInterlaceFlags: u32,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl Default for VMRPRESENTATIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl windows_core::TypeKind for VMRPRESENTATIONINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRVIDEOSTREAMINFO {
    pub pddsVideoSurface: Option<super::super::Graphics::DirectDraw::IDirectDrawSurface7>,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwStrmID: u32,
    pub fAlpha: f32,
    pub ddClrKey: super::super::Graphics::DirectDraw::DDCOLORKEY,
    pub rNormal: NORMALIZEDRECT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl Default for VMRVIDEOSTREAMINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl windows_core::TypeKind for VMRVIDEOSTREAMINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VMRVideoDesc {
    pub dwSize: u32,
    pub dwSampleWidth: u32,
    pub dwSampleHeight: u32,
    pub SingleFieldPerSample: super::super::Foundation::BOOL,
    pub dwFourCC: u32,
    pub InputSampleFreq: VMRFrequency,
    pub OutputFrameFreq: VMRFrequency,
}
impl Default for VMRVideoDesc {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for VMRVideoDesc {
    type TypeKind = windows_core::CloneType;
}
pub type AMGETERRORTEXTPROCA = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: windows_core::PCSTR, param2: u32) -> super::super::Foundation::BOOL>;
pub type AMGETERRORTEXTPROCW = Option<unsafe extern "system" fn(param0: windows_core::HRESULT, param1: windows_core::PCWSTR, param2: u32) -> super::super::Foundation::BOOL>;
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
pub type PDXVA2SW_CREATEVIDEOPROCESSDEVICE = Option<unsafe extern "system" fn(pd3dd9: Option<super::super::Graphics::Direct3D9::IDirect3DDevice9>, pvideodesc: *const super::MediaFoundation::DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, maxsubstreams: u32, phdevice: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT>;
pub type PDXVA2SW_DESTROYVIDEOPROCESSDEVICE = Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
pub type PDXVA2SW_GETFILTERPROPERTYRANGE = Option<unsafe extern "system" fn(pvideodesc: *const super::MediaFoundation::DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, filtersetting: u32, prange: *mut super::MediaFoundation::DXVA2_ValueRange) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
pub type PDXVA2SW_GETPROCAMPRANGE = Option<unsafe extern "system" fn(pvideodesc: *const super::MediaFoundation::DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, procampcap: u32, prange: *mut super::MediaFoundation::DXVA2_ValueRange) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
pub type PDXVA2SW_GETVIDEOPROCESSORCAPS = Option<unsafe extern "system" fn(pvideodesc: *const super::MediaFoundation::DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, pcaps: *mut super::MediaFoundation::DXVA2_VideoProcessorCaps) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
pub type PDXVA2SW_GETVIDEOPROCESSORRENDERTARGETCOUNT = Option<unsafe extern "system" fn(pvideodesc: *const super::MediaFoundation::DXVA2_VideoDesc, pcount: *mut u32) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
pub type PDXVA2SW_GETVIDEOPROCESSORRENDERTARGETS = Option<unsafe extern "system" fn(pvideodesc: *const super::MediaFoundation::DXVA2_VideoDesc, count: u32, pformats: *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
pub type PDXVA2SW_GETVIDEOPROCESSORSUBSTREAMFORMATCOUNT = Option<unsafe extern "system" fn(pvideodesc: *const super::MediaFoundation::DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, pcount: *mut u32) -> windows_core::HRESULT>;
#[cfg(all(feature = "Win32_Graphics_Direct3D9", feature = "Win32_Media_MediaFoundation"))]
pub type PDXVA2SW_GETVIDEOPROCESSORSUBSTREAMFORMATS = Option<unsafe extern "system" fn(pvideodesc: *const super::MediaFoundation::DXVA2_VideoDesc, rendertargetformat: super::super::Graphics::Direct3D9::D3DFORMAT, count: u32, pformats: *mut super::super::Graphics::Direct3D9::D3DFORMAT) -> windows_core::HRESULT>;
pub type PDXVA2SW_VIDEOPROCESSBEGINFRAME = Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub type PDXVA2SW_VIDEOPROCESSBLT = Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, pblt: *const DXVA2_VIDEOPROCESSBLT) -> windows_core::HRESULT>;
pub type PDXVA2SW_VIDEOPROCESSENDFRAME = Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, phandlecomplete: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_Graphics_Direct3D9")]
pub type PDXVA2SW_VIDEOPROCESSSETRENDERTARGET = Option<unsafe extern "system" fn(hdevice: super::super::Foundation::HANDLE, prendertarget: Option<super::super::Graphics::Direct3D9::IDirect3DSurface9>) -> windows_core::HRESULT>;
