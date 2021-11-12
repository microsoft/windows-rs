#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_DirectMusic: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1667997456, data2: 3197, data3: 4561, data4: [149, 178, 0, 32, 175, 220, 116, 33] };
pub const CLSID_DirectMusicCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1209005232, data2: 10418, data3: 4561, data4: [190, 247, 0, 192, 79, 191, 143, 239] };
pub const CLSID_DirectMusicSynth: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1489155280, data2: 18151, data3: 4561, data4: [137, 172, 0, 160, 201, 5, 65, 41] };
pub const CLSID_DirectMusicSynthSink: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2931916003, data2: 42260, data3: 4561, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const CLSID_DirectSoundPrivate: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 296435392, data2: 9708, data3: 4561, data4: [164, 216, 0, 192, 79, 194, 138, 202] };
pub struct CONNECTION(i32);
pub struct CONNECTIONLIST(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_ATTENUATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_CENTER: u32 = 18u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_CHORUS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_ATTACKTIME: u32 = 518u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_DECAYTIME: u32 = 519u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_DELAYTIME: u32 = 523u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_HOLDTIME: u32 = 524u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_RELEASETIME: u32 = 521u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_SHUTDOWNTIME: u32 = 525u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_SUSTAINLEVEL: u32 = 522u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_ATTACKTIME: u32 = 778u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_DECAYTIME: u32 = 779u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_DELAYTIME: u32 = 783u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_HOLDTIME: u32 = 784u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_RELEASETIME: u32 = 781u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_SUSTAINLEVEL: u32 = 782u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_FILTER_CUTOFF: u32 = 1280u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_FILTER_Q: u32 = 1281u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_GAIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_KEYNUMBER: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LEFT: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LEFTREAR: u32 = 19u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LFE_CHANNEL: u32 = 21u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LFO_FREQUENCY: u32 = 260u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LFO_STARTDELAY: u32 = 261u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_PAN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_PITCH: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_REVERB: u32 = 129u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_RIGHT: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_RIGHTREAR: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_VIB_FREQUENCY: u32 = 276u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_VIB_STARTDELAY: u32 = 277u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC1: u32 = 129u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC10: u32 = 138u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC11: u32 = 139u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC7: u32 = 135u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC91: u32 = 219u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC93: u32 = 221u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CHANNELPRESSURE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_EG1: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_EG2: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_KEYNUMBER: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_KEYONVELOCITY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_LFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_MONOPRESSURE: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_PITCHWHEEL: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_POLYPRESSURE: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_VIBRATO: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_TRN_CONCAVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_TRN_CONVEX: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_TRN_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_TRN_SWITCH: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN10_VOICE_PRIORITY_OFFSET: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN11_VOICE_PRIORITY_OFFSET: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN12_VOICE_PRIORITY_OFFSET: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN13_VOICE_PRIORITY_OFFSET: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN14_VOICE_PRIORITY_OFFSET: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN15_VOICE_PRIORITY_OFFSET: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN16_VOICE_PRIORITY_OFFSET: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN1_VOICE_PRIORITY_OFFSET: u32 = 14u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN2_VOICE_PRIORITY_OFFSET: u32 = 13u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN3_VOICE_PRIORITY_OFFSET: u32 = 12u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN4_VOICE_PRIORITY_OFFSET: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN5_VOICE_PRIORITY_OFFSET: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN6_VOICE_PRIORITY_OFFSET: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN7_VOICE_PRIORITY_OFFSET: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN8_VOICE_PRIORITY_OFFSET: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN9_VOICE_PRIORITY_OFFSET: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CRITICAL_VOICE_PRIORITY: u32 = 4026531840u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_HIGH_VOICE_PRIORITY: u32 = 3221225472u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_LOW_VOICE_PRIORITY: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_PERSIST_VOICE_PRIORITY: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_STANDARD_VOICE_PRIORITY: u32 = 2147483648u32;
pub struct DIRECTSOUNDDEVICE_DATAFLOW(i32);
pub struct DIRECTSOUNDDEVICE_TYPE(i32);
pub struct DLSHEADER(i32);
pub struct DLSID(i32);
pub const DLSID_GMInHardware: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259684, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const DLSID_GSInHardware: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259685, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const DLSID_ManufacturersID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2956857729, data2: 32917, data3: 4562, data4: [161, 239, 0, 96, 8, 51, 219, 216] };
pub const DLSID_ProductID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2956857730, data2: 32917, data3: 4562, data4: [161, 239, 0, 96, 8, 51, 219, 216] };
pub const DLSID_SampleMemorySize: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259688, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const DLSID_SamplePlaybackRate: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 714209043, data2: 42175, data3: 4562, data4: [187, 223, 0, 96, 8, 51, 219, 216] };
pub const DLSID_SupportsDLS1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259687, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const DLSID_SupportsDLS2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4047870437, data2: 18057, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const DLSID_XGInHardware: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259686, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub struct DLSVERSION(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_ADD: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_AND: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_CONST: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_DIVIDE: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_EQ: u32 = 14u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_GE: u32 = 13u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_GT: u32 = 12u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_LE: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_LOGICAL_AND: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_LOGICAL_OR: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_LT: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_MULTIPLY: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_NOT: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_OR: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_QUERY: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_QUERYSUPPORTED: u32 = 18u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_SUBTRACT: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_XOR: u32 = 3u32;
pub struct DMUS_ARTICPARAMS(i32);
pub struct DMUS_ARTICULATION(i32);
pub struct DMUS_ARTICULATION2(i32);
pub struct DMUS_BUFFERDESC(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_CLOCKF_GLOBAL: u32 = 1u32;
pub struct DMUS_CLOCKINFO7(i32);
pub struct DMUS_CLOCKINFO8(i32);
pub struct DMUS_CLOCKTYPE(i32);
pub struct DMUS_COPYRIGHT(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DEFAULT_SIZE_OFFSETTABLE: u32 = 1u32;
pub struct DMUS_DOWNLOADINFO(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_INSTRUMENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_INSTRUMENT2: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_ONESHOTWAVE: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_STREAMINGWAVE: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_WAVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_WAVEARTICULATION: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EFFECT_CHORUS: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EFFECT_DELAY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EFFECT_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EFFECT_REVERB: u32 = 1u32;
pub struct DMUS_EVENTHEADER(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EVENT_STRUCTURED: u32 = 1u32;
pub struct DMUS_EXTENSIONCHUNK(i32);
pub struct DMUS_INSTRUMENT(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_INSTRUMENT_GM_INSTRUMENT: u32 = 1u32;
pub struct DMUS_LFOPARAMS(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MAX_DESCRIPTION: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MAX_DRIVER: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MIN_DATA_SIZE: u32 = 4u32;
pub struct DMUS_MSCPARAMS(i32);
pub struct DMUS_NOTERANGE(i32);
pub struct DMUS_OFFSETTABLE(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_AUDIOPATH: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_DIRECTSOUND: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_DLS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_DLS2: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_EXTERNAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_GMINHARDWARE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_GSINHARDWARE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_INPUTCLASS: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_MEMORYSIZEFIXED: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_OUTPUTCLASS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_SHAREABLE: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_SOFTWARESYNTH: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_SYSTEMMEMORY: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_WAVE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_XGINHARDWARE: u32 = 64u32;
pub struct DMUS_PEGPARAMS(i32);
pub struct DMUS_PORTCAPS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DMUS_PORTPARAMS8(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_AUDIOCHANNELS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_CHANNELGROUPS: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_EFFECTS: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_FEATURES: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_SAMPLERATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_SHARE: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_VOICES: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_FEATURE_AUDIOPATH: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_FEATURE_STREAMING: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_KERNEL_MODE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_USER_MODE_SYNTH: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_WINMM_DRIVER: u32 = 0u32;
pub struct DMUS_REGION(i32);
pub struct DMUS_SYNTHSTATS(i32);
pub struct DMUS_SYNTHSTATS8(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_CPU_PER_VOICE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_FREE_MEMORY: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_LOST_NOTES: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_PEAK_VOLUME: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_SYSTEMMEMORY: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_TOTAL_CPU: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_VOICES: u32 = 1u32;
pub struct DMUS_VEGPARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DMUS_VOICE_STATE(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_VOLUME_MAX: u32 = 2000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_VOLUME_MIN: i32 = -20000i32;
pub struct DMUS_WAVE(i32);
pub struct DMUS_WAVEARTDL(i32);
pub struct DMUS_WAVEDATA(i32);
pub struct DMUS_WAVEDL(i32);
pub struct DMUS_WAVES_REVERB_PARAMS(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_BACK_CENTER: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_BACK_LEFT: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_BACK_RIGHT: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_CHORUS_SEND: u32 = 65u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_DYNAMIC_0: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FIRST_SPKR_LOC: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_CENTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_LEFT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_LEFT_OF_CENTER: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_RIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_RIGHT_OF_CENTER: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_LAST_SPKR_LOC: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_LEFT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_LOW_FREQUENCY: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_NULL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_REVERB_SEND: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_RIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_SIDE_LEFT: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_SIDE_RIGHT: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_BACK_CENTER: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_BACK_LEFT: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_BACK_RIGHT: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_CENTER: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_FRONT_CENTER: u32 = 13u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_FRONT_LEFT: u32 = 12u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_FRONT_RIGHT: u32 = 14u32;
pub struct DSPROPERTY_DIRECTSOUNDDEVICE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA(i32);
pub const DSPROPSETID_DirectSoundDevice: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2221035394, data2: 9708, data3: 4561, data4: [164, 216, 0, 192, 79, 194, 138, 202] };
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_AUDIOMODE: u32 = 3840u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_AUDIOQU: u32 = 117440512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_AUDIOSMP: u32 = 939524096u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_CAP_AUD12Bits: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_CAP_AUD16Bits: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_DVSD_NTSC_FRAMESIZE: i32 = 120000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_DVSD_PAL_FRAMESIZE: i32 = 144000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_HD: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_NTSC: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_NTSCPAL: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_PAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_SD: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_SL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_SMCHN: u32 = 57344u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_STYPE: u32 = 2031616u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_INSTRUMENT_DRUMS: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_RGN_OPTION_SELFNONEXCLUSIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_WAVELINK_MULTICHANNEL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_WAVELINK_PHASE_MASTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_WSMP_NO_COMPRESSION: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_WSMP_NO_TRUNCATION: i32 = 1i32;
pub const GUID_DMUS_PROP_DLS1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259687, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_DLS2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4047870437, data2: 18057, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_Effects: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3450394129, data2: 26698, data3: 4562, data4: [135, 30, 0, 96, 8, 147, 177, 189] };
pub const GUID_DMUS_PROP_GM_Hardware: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259684, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_GS_Capable: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1687595938, data2: 25008, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_GS_Hardware: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259685, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_INSTRUMENT2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2254426994, data2: 40807, data3: 4562, data4: [135, 42, 0, 96, 8, 147, 177, 189] };
pub const GUID_DMUS_PROP_LegacyCaps: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3483880898, data2: 161, data3: 4562, data4: [170, 213, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_MemorySize: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259688, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_SampleMemorySize: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259688, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_SamplePlaybackRate: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 714209043, data2: 42175, data3: 4562, data4: [187, 223, 0, 96, 8, 51, 219, 216] };
pub const GUID_DMUS_PROP_SetSynthSink: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 171596709, data2: 14262, data3: 4562, data4: [185, 249, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_SinkUsesDSound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3189803095, data2: 35154, data3: 4562, data4: [186, 28, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_SynthSink_DSOUND: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 178878532, data2: 51319, data3: 4561, data4: [135, 12, 0, 96, 8, 147, 177, 189] };
pub const GUID_DMUS_PROP_SynthSink_WAVE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 178878533, data2: 51319, data3: 4561, data4: [135, 12, 0, 96, 8, 147, 177, 189] };
pub const GUID_DMUS_PROP_Volume: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4276071973, data2: 58478, data3: 4561, data4: [170, 206, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_WavesReverb: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 80434722, data2: 13029, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_WriteLatency: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 646582176, data2: 24818, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_WritePeriod: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 646582177, data2: 24818, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_XG_Capable: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1687595937, data2: 25008, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_XG_Hardware: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395259686, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub struct IDirectMusic(pub *mut ::core::ffi::c_void);
pub struct IDirectMusic8(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicBuffer(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicCollection(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicDownload(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicDownloadedInstrument(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicInstrument(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicPort(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicPortDownload(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicSynth(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicSynth8(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicSynthSink(pub *mut ::core::ffi::c_void);
pub struct IDirectMusicThru(pub *mut ::core::ffi::c_void);
pub struct INSTHEADER(i32);
pub struct LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1(i32);
pub struct LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA(i32);
pub struct LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW(i32);
pub struct MDEVICECAPSEX(i32);
pub struct MIDILOCALE(i32);
#[cfg(feature = "Win32_Media_Multimedia")]
pub struct MIDIOPENDESC(i32);
pub struct POOLCUE(i32);
pub struct POOLTABLE(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const POOL_CUE_NULL: i32 = -1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const REFRESH_F_LASTBUFFER: u32 = 1u32;
pub struct RGNHEADER(i32);
pub struct RGNRANGE(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const SIZE_DVINFO: u32 = 32u32;
pub struct Tag_DVAudInfo(i32);
pub struct WAVELINK(i32);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WAVELINK_CHANNEL_LEFT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WAVELINK_CHANNEL_RIGHT: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WLOOP_TYPE_FORWARD: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WLOOP_TYPE_RELEASE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct _DMUS_PORTPARAMS(i32);
pub struct _rloop(i32);
pub struct _rwsmp(i32);
