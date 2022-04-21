pub const CLSID_DirectMusic: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1667997456, data2: 3197, data3: 4561, data4: [149, 178, 0, 32, 175, 220, 116, 33] };
pub const CLSID_DirectMusicCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1209005232, data2: 10418, data3: 4561, data4: [190, 247, 0, 192, 79, 191, 143, 239] };
pub const CLSID_DirectMusicSynth: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1489155280, data2: 18151, data3: 4561, data4: [137, 172, 0, 160, 201, 5, 65, 41] };
pub const CLSID_DirectMusicSynthSink: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2931916003, data2: 42260, data3: 4561, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const CLSID_DirectSoundPrivate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 296435392, data2: 9708, data3: 4561, data4: [164, 216, 0, 192, 79, 194, 138, 202] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct CONNECTION {
    pub usSource: u16,
    pub usControl: u16,
    pub usDestination: u16,
    pub usTransform: u16,
    pub lScale: i32,
}
impl ::core::marker::Copy for CONNECTION {}
impl ::core::clone::Clone for CONNECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct CONNECTIONLIST {
    pub cbSize: u32,
    pub cConnections: u32,
}
impl ::core::marker::Copy for CONNECTIONLIST {}
impl ::core::clone::Clone for CONNECTIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_ATTENUATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_CENTER: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_CHORUS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG1_ATTACKTIME: u32 = 518u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG1_DECAYTIME: u32 = 519u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG1_DELAYTIME: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG1_HOLDTIME: u32 = 524u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG1_RELEASETIME: u32 = 521u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG1_SHUTDOWNTIME: u32 = 525u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG1_SUSTAINLEVEL: u32 = 522u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG2_ATTACKTIME: u32 = 778u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG2_DECAYTIME: u32 = 779u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG2_DELAYTIME: u32 = 783u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG2_HOLDTIME: u32 = 784u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG2_RELEASETIME: u32 = 781u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_EG2_SUSTAINLEVEL: u32 = 782u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_FILTER_CUTOFF: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_FILTER_Q: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_GAIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_KEYNUMBER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_LEFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_LEFTREAR: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_LFE_CHANNEL: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_LFO_FREQUENCY: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_LFO_STARTDELAY: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_PAN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_PITCH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_REVERB: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_RIGHT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_RIGHTREAR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_VIB_FREQUENCY: u32 = 276u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_DST_VIB_STARTDELAY: u32 = 277u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_CC1: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_CC10: u32 = 138u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_CC11: u32 = 139u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_CC7: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_CC91: u32 = 219u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_CC93: u32 = 221u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_CHANNELPRESSURE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_EG1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_EG2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_KEYNUMBER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_KEYONVELOCITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_LFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_MONOPRESSURE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_PITCHWHEEL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_POLYPRESSURE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_SRC_VIBRATO: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_TRN_CONCAVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_TRN_CONVEX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_TRN_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const CONN_TRN_SWITCH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN10_VOICE_PRIORITY_OFFSET: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN11_VOICE_PRIORITY_OFFSET: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN12_VOICE_PRIORITY_OFFSET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN13_VOICE_PRIORITY_OFFSET: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN14_VOICE_PRIORITY_OFFSET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN15_VOICE_PRIORITY_OFFSET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN16_VOICE_PRIORITY_OFFSET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN1_VOICE_PRIORITY_OFFSET: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN2_VOICE_PRIORITY_OFFSET: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN3_VOICE_PRIORITY_OFFSET: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN4_VOICE_PRIORITY_OFFSET: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN5_VOICE_PRIORITY_OFFSET: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN6_VOICE_PRIORITY_OFFSET: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN7_VOICE_PRIORITY_OFFSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN8_VOICE_PRIORITY_OFFSET: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CHAN9_VOICE_PRIORITY_OFFSET: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_CRITICAL_VOICE_PRIORITY: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_HIGH_VOICE_PRIORITY: u32 = 3221225472u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_LOW_VOICE_PRIORITY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_PERSIST_VOICE_PRIORITY: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DAUD_STANDARD_VOICE_PRIORITY: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub type DIRECTSOUNDDEVICE_DATAFLOW = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_DATAFLOW_RENDER: DIRECTSOUNDDEVICE_DATAFLOW = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_DATAFLOW_CAPTURE: DIRECTSOUNDDEVICE_DATAFLOW = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub type DIRECTSOUNDDEVICE_TYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_TYPE_EMULATED: DIRECTSOUNDDEVICE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_TYPE_VXD: DIRECTSOUNDDEVICE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_TYPE_WDM: DIRECTSOUNDDEVICE_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DLSHEADER {
    pub cInstruments: u32,
}
impl ::core::marker::Copy for DLSHEADER {}
impl ::core::clone::Clone for DLSHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DLSID {
    pub ulData1: u32,
    pub usData2: u16,
    pub usData3: u16,
    pub abData4: [u8; 8],
}
impl ::core::marker::Copy for DLSID {}
impl ::core::clone::Clone for DLSID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DLSID_GMInHardware: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259684, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const DLSID_GSInHardware: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259685, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const DLSID_ManufacturersID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2956857729, data2: 32917, data3: 4562, data4: [161, 239, 0, 96, 8, 51, 219, 216] };
pub const DLSID_ProductID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2956857730, data2: 32917, data3: 4562, data4: [161, 239, 0, 96, 8, 51, 219, 216] };
pub const DLSID_SampleMemorySize: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259688, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const DLSID_SamplePlaybackRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 714209043, data2: 42175, data3: 4562, data4: [187, 223, 0, 96, 8, 51, 219, 216] };
pub const DLSID_SupportsDLS1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259687, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const DLSID_SupportsDLS2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4047870437, data2: 18057, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const DLSID_XGInHardware: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259686, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DLSVERSION {
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
}
impl ::core::marker::Copy for DLSVERSION {}
impl ::core::clone::Clone for DLSVERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_ADD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_AND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_CONST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_DIVIDE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_EQ: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_GE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_GT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_LE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_LOGICAL_AND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_LOGICAL_OR: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_LT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_MULTIPLY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_NOT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_OR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_QUERY: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_QUERYSUPPORTED: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_SUBTRACT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DLS_CDL_XOR: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_ARTICPARAMS {
    pub LFO: DMUS_LFOPARAMS,
    pub VolEG: DMUS_VEGPARAMS,
    pub PitchEG: DMUS_PEGPARAMS,
    pub Misc: DMUS_MSCPARAMS,
}
impl ::core::marker::Copy for DMUS_ARTICPARAMS {}
impl ::core::clone::Clone for DMUS_ARTICPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_ARTICULATION {
    pub ulArt1Idx: u32,
    pub ulFirstExtCkIdx: u32,
}
impl ::core::marker::Copy for DMUS_ARTICULATION {}
impl ::core::clone::Clone for DMUS_ARTICULATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_ARTICULATION2 {
    pub ulArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulNextArtIdx: u32,
}
impl ::core::marker::Copy for DMUS_ARTICULATION2 {}
impl ::core::clone::Clone for DMUS_ARTICULATION2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_BUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidBufferFormat: ::windows_sys::core::GUID,
    pub cbBuffer: u32,
}
impl ::core::marker::Copy for DMUS_BUFFERDESC {}
impl ::core::clone::Clone for DMUS_BUFFERDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_CLOCKF_GLOBAL: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_CLOCKINFO7 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: ::windows_sys::core::GUID,
    pub wszDescription: [u16; 128],
}
impl ::core::marker::Copy for DMUS_CLOCKINFO7 {}
impl ::core::clone::Clone for DMUS_CLOCKINFO7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_CLOCKINFO8 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: ::windows_sys::core::GUID,
    pub wszDescription: [u16; 128],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DMUS_CLOCKINFO8 {}
impl ::core::clone::Clone for DMUS_CLOCKINFO8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub type DMUS_CLOCKTYPE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_CLOCK_SYSTEM: DMUS_CLOCKTYPE = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_CLOCK_WAVE: DMUS_CLOCKTYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_COPYRIGHT {
    pub cbSize: u32,
    pub byCopyright: [u8; 4],
}
impl ::core::marker::Copy for DMUS_COPYRIGHT {}
impl ::core::clone::Clone for DMUS_COPYRIGHT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_DEFAULT_SIZE_OFFSETTABLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_DOWNLOADINFO {
    pub dwDLType: u32,
    pub dwDLId: u32,
    pub dwNumOffsetTableEntries: u32,
    pub cbSize: u32,
}
impl ::core::marker::Copy for DMUS_DOWNLOADINFO {}
impl ::core::clone::Clone for DMUS_DOWNLOADINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_DOWNLOADINFO_INSTRUMENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_DOWNLOADINFO_INSTRUMENT2: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_DOWNLOADINFO_ONESHOTWAVE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_DOWNLOADINFO_STREAMINGWAVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_DOWNLOADINFO_WAVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_DOWNLOADINFO_WAVEARTICULATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_EFFECT_CHORUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_EFFECT_DELAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_EFFECT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_EFFECT_REVERB: u32 = 1u32;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_EVENTHEADER {
    pub cbEvent: u32,
    pub dwChannelGroup: u32,
    pub rtDelta: i64,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DMUS_EVENTHEADER {}
impl ::core::clone::Clone for DMUS_EVENTHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_EVENT_STRUCTURED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_EXTENSIONCHUNK {
    pub cbSize: u32,
    pub ulNextExtCkIdx: u32,
    pub ExtCkID: u32,
    pub byExtCk: [u8; 4],
}
impl ::core::marker::Copy for DMUS_EXTENSIONCHUNK {}
impl ::core::clone::Clone for DMUS_EXTENSIONCHUNK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_INSTRUMENT {
    pub ulPatch: u32,
    pub ulFirstRegionIdx: u32,
    pub ulGlobalArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for DMUS_INSTRUMENT {}
impl ::core::clone::Clone for DMUS_INSTRUMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_INSTRUMENT_GM_INSTRUMENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_LFOPARAMS {
    pub pcFrequency: i32,
    pub tcDelay: i32,
    pub gcVolumeScale: i32,
    pub pcPitchScale: i32,
    pub gcMWToVolume: i32,
    pub pcMWToPitch: i32,
}
impl ::core::marker::Copy for DMUS_LFOPARAMS {}
impl ::core::clone::Clone for DMUS_LFOPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_MAX_DESCRIPTION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_MAX_DRIVER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_MIN_DATA_SIZE: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_MSCPARAMS {
    pub ptDefaultPan: i32,
}
impl ::core::marker::Copy for DMUS_MSCPARAMS {}
impl ::core::clone::Clone for DMUS_MSCPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_NOTERANGE {
    pub dwLowNote: u32,
    pub dwHighNote: u32,
}
impl ::core::marker::Copy for DMUS_NOTERANGE {}
impl ::core::clone::Clone for DMUS_NOTERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_OFFSETTABLE {
    pub ulOffsetTable: [u32; 1],
}
impl ::core::marker::Copy for DMUS_OFFSETTABLE {}
impl ::core::clone::Clone for DMUS_OFFSETTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_AUDIOPATH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_DIRECTSOUND: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_DLS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_DLS2: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_EXTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_GMINHARDWARE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_GSINHARDWARE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_INPUTCLASS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_MEMORYSIZEFIXED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_OUTPUTCLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_SHAREABLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_SOFTWARESYNTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_SYSTEMMEMORY: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_WAVE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PC_XGINHARDWARE: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_PEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
    pub pcRange: i32,
}
impl ::core::marker::Copy for DMUS_PEGPARAMS {}
impl ::core::clone::Clone for DMUS_PEGPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_PORTCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidPort: ::windows_sys::core::GUID,
    pub dwClass: u32,
    pub dwType: u32,
    pub dwMemorySize: u32,
    pub dwMaxChannelGroups: u32,
    pub dwMaxVoices: u32,
    pub dwMaxAudioChannels: u32,
    pub dwEffectFlags: u32,
    pub wszDescription: [u16; 128],
}
impl ::core::marker::Copy for DMUS_PORTCAPS {}
impl ::core::clone::Clone for DMUS_PORTCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DMUS_PORTPARAMS8 {
    pub dwSize: u32,
    pub dwValidParams: u32,
    pub dwVoices: u32,
    pub dwChannelGroups: u32,
    pub dwAudioChannels: u32,
    pub dwSampleRate: u32,
    pub dwEffectFlags: u32,
    pub fShare: super::super::super::Foundation::BOOL,
    pub dwFeatures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DMUS_PORTPARAMS8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DMUS_PORTPARAMS8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORTPARAMS_AUDIOCHANNELS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORTPARAMS_CHANNELGROUPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORTPARAMS_EFFECTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORTPARAMS_FEATURES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORTPARAMS_SAMPLERATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORTPARAMS_SHARE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORTPARAMS_VOICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORT_FEATURE_AUDIOPATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORT_FEATURE_STREAMING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORT_KERNEL_MODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORT_USER_MODE_SYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_PORT_WINMM_DRIVER: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_REGION {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
    pub ulRegionArtIdx: u32,
    pub ulNextRegionIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub WaveLink: WAVELINK,
    pub WSMP: _rwsmp,
    pub WLOOP: [_rloop; 1],
}
impl ::core::marker::Copy for DMUS_REGION {}
impl ::core::clone::Clone for DMUS_REGION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_SYNTHSTATS {
    pub dwSize: u32,
    pub dwValidStats: u32,
    pub dwVoices: u32,
    pub dwTotalCPU: u32,
    pub dwCPUPerVoice: u32,
    pub dwLostNotes: u32,
    pub dwFreeMemory: u32,
    pub lPeakVolume: i32,
}
impl ::core::marker::Copy for DMUS_SYNTHSTATS {}
impl ::core::clone::Clone for DMUS_SYNTHSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_SYNTHSTATS8 {
    pub dwSize: u32,
    pub dwValidStats: u32,
    pub dwVoices: u32,
    pub dwTotalCPU: u32,
    pub dwCPUPerVoice: u32,
    pub dwLostNotes: u32,
    pub dwFreeMemory: u32,
    pub lPeakVolume: i32,
    pub dwSynthMemUse: u32,
}
impl ::core::marker::Copy for DMUS_SYNTHSTATS8 {}
impl ::core::clone::Clone for DMUS_SYNTHSTATS8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_SYNTHSTATS_CPU_PER_VOICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_SYNTHSTATS_FREE_MEMORY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_SYNTHSTATS_LOST_NOTES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_SYNTHSTATS_PEAK_VOLUME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_SYNTHSTATS_SYSTEMMEMORY: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_SYNTHSTATS_TOTAL_CPU: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_SYNTHSTATS_VOICES: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_VEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
}
impl ::core::marker::Copy for DMUS_VEGPARAMS {}
impl ::core::clone::Clone for DMUS_VEGPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DMUS_VOICE_STATE {
    pub bExists: super::super::super::Foundation::BOOL,
    pub spPosition: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DMUS_VOICE_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DMUS_VOICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_VOLUME_MAX: u32 = 2000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_VOLUME_MIN: i32 = -20000i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_WAVE {
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulWaveDataIdx: u32,
    pub WaveformatEx: super::WAVEFORMATEX,
}
impl ::core::marker::Copy for DMUS_WAVE {}
impl ::core::clone::Clone for DMUS_WAVE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_WAVEARTDL {
    pub ulDownloadIdIdx: u32,
    pub ulBus: u32,
    pub ulBuffers: u32,
    pub ulMasterDLId: u32,
    pub usOptions: u16,
}
impl ::core::marker::Copy for DMUS_WAVEARTDL {}
impl ::core::clone::Clone for DMUS_WAVEARTDL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_WAVEDATA {
    pub cbSize: u32,
    pub byData: [u8; 4],
}
impl ::core::marker::Copy for DMUS_WAVEDATA {}
impl ::core::clone::Clone for DMUS_WAVEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_WAVEDL {
    pub cbWaveData: u32,
}
impl ::core::marker::Copy for DMUS_WAVEDL {}
impl ::core::clone::Clone for DMUS_WAVEDL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DMUS_WAVES_REVERB_PARAMS {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl ::core::marker::Copy for DMUS_WAVES_REVERB_PARAMS {}
impl ::core::clone::Clone for DMUS_WAVES_REVERB_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_BACK_CENTER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_BACK_LEFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_BACK_RIGHT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_CHORUS_SEND: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_DYNAMIC_0: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_FIRST_SPKR_LOC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_FRONT_CENTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_FRONT_LEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_FRONT_LEFT_OF_CENTER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_FRONT_RIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_FRONT_RIGHT_OF_CENTER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_LAST_SPKR_LOC: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_LEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_LOW_FREQUENCY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_NULL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_REVERB_SEND: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_RIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_SIDE_LEFT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_SIDE_RIGHT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_TOP_BACK_CENTER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_TOP_BACK_LEFT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_TOP_BACK_RIGHT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_TOP_CENTER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_TOP_FRONT_CENTER: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_TOP_FRONT_LEFT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSBUSID_TOP_FRONT_RIGHT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub type DSPROPERTY_DIRECTSOUNDDEVICE = i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A: DSPROPERTY_DIRECTSOUNDDEVICE = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1: DSPROPERTY_DIRECTSOUNDDEVICE = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1: DSPROPERTY_DIRECTSOUNDDEVICE = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W: DSPROPERTY_DIRECTSOUNDDEVICE = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A: DSPROPERTY_DIRECTSOUNDDEVICE = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W: DSPROPERTY_DIRECTSOUNDDEVICE = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A: DSPROPERTY_DIRECTSOUNDDEVICE = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W: DSPROPERTY_DIRECTSOUNDDEVICE = 8i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    pub DeviceId: ::windows_sys::core::GUID,
    pub DescriptionA: [super::super::super::Foundation::CHAR; 256],
    pub DescriptionW: [u16; 256],
    pub ModuleA: [super::super::super::Foundation::CHAR; 260],
    pub ModuleW: [u16; 260],
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub WaveDeviceId: u32,
    pub Devnode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows_sys::core::GUID,
    pub Description: ::windows_sys::core::PSTR,
    pub Module: ::windows_sys::core::PSTR,
    pub Interface: ::windows_sys::core::PSTR,
    pub WaveDeviceId: u32,
}
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows_sys::core::GUID,
    pub Description: ::windows_sys::core::PWSTR,
    pub Module: ::windows_sys::core::PWSTR,
    pub Interface: ::windows_sys::core::PWSTR,
    pub WaveDeviceId: u32,
}
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    pub Callback: LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    pub DeviceName: ::windows_sys::core::PSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    pub DeviceName: ::windows_sys::core::PWSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DSPROPSETID_DirectSoundDevice: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2221035394, data2: 9708, data3: 4561, data4: [164, 216, 0, 192, 79, 194, 138, 202] };
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_AUDIOMODE: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_AUDIOQU: u32 = 117440512u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_AUDIOSMP: u32 = 939524096u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_CAP_AUD12Bits: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_CAP_AUD16Bits: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_DVSD_NTSC_FRAMESIZE: i32 = 120000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_DVSD_PAL_FRAMESIZE: i32 = 144000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_HD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_NTSC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_NTSCPAL: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_PAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_SD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_SL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_SMCHN: u32 = 57344u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DV_STYPE: u32 = 2031616u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const F_INSTRUMENT_DRUMS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const F_RGN_OPTION_SELFNONEXCLUSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const F_WAVELINK_MULTICHANNEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const F_WAVELINK_PHASE_MASTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const F_WSMP_NO_COMPRESSION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const F_WSMP_NO_TRUNCATION: i32 = 1i32;
pub const GUID_DMUS_PROP_DLS1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259687, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_DLS2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4047870437, data2: 18057, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_Effects: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3450394129, data2: 26698, data3: 4562, data4: [135, 30, 0, 96, 8, 147, 177, 189] };
pub const GUID_DMUS_PROP_GM_Hardware: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259684, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_GS_Capable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1687595938, data2: 25008, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_GS_Hardware: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259685, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_INSTRUMENT2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2254426994, data2: 40807, data3: 4562, data4: [135, 42, 0, 96, 8, 147, 177, 189] };
pub const GUID_DMUS_PROP_LegacyCaps: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3483880898, data2: 161, data3: 4562, data4: [170, 213, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_MemorySize: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259688, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_SampleMemorySize: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259688, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_SamplePlaybackRate: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 714209043, data2: 42175, data3: 4562, data4: [187, 223, 0, 96, 8, 51, 219, 216] };
pub const GUID_DMUS_PROP_SetSynthSink: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 171596709, data2: 14262, data3: 4562, data4: [185, 249, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_SinkUsesDSound: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3189803095, data2: 35154, data3: 4562, data4: [186, 28, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_SynthSink_DSOUND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 178878532, data2: 51319, data3: 4561, data4: [135, 12, 0, 96, 8, 147, 177, 189] };
pub const GUID_DMUS_PROP_SynthSink_WAVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 178878533, data2: 51319, data3: 4561, data4: [135, 12, 0, 96, 8, 147, 177, 189] };
pub const GUID_DMUS_PROP_Volume: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4276071973, data2: 58478, data3: 4561, data4: [170, 206, 0, 0, 248, 117, 172, 18] };
pub const GUID_DMUS_PROP_WavesReverb: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 80434722, data2: 13029, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_WriteLatency: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 646582176, data2: 24818, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_WritePeriod: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 646582177, data2: 24818, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_XG_Capable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1687595937, data2: 25008, data3: 4562, data4: [175, 166, 0, 170, 0, 36, 216, 182] };
pub const GUID_DMUS_PROP_XG_Hardware: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 395259686, data2: 50020, data3: 4561, data4: [167, 96, 0, 0, 248, 117, 172, 18] };
pub type IDirectMusic = *mut ::core::ffi::c_void;
pub type IDirectMusic8 = *mut ::core::ffi::c_void;
pub type IDirectMusicBuffer = *mut ::core::ffi::c_void;
pub type IDirectMusicCollection = *mut ::core::ffi::c_void;
pub type IDirectMusicDownload = *mut ::core::ffi::c_void;
pub type IDirectMusicDownloadedInstrument = *mut ::core::ffi::c_void;
pub type IDirectMusicInstrument = *mut ::core::ffi::c_void;
pub type IDirectMusicPort = *mut ::core::ffi::c_void;
pub type IDirectMusicPortDownload = *mut ::core::ffi::c_void;
pub type IDirectMusicSynth = *mut ::core::ffi::c_void;
pub type IDirectMusicSynth8 = *mut ::core::ffi::c_void;
pub type IDirectMusicSynthSink = *mut ::core::ffi::c_void;
pub type IDirectMusicThru = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct INSTHEADER {
    pub cRegions: u32,
    pub Locale: MIDILOCALE,
}
impl ::core::marker::Copy for INSTHEADER {}
impl ::core::clone::Clone for INSTHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1 = ::core::option::Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct MDEVICECAPSEX {
    pub cbSize: u32,
    pub pCaps: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MDEVICECAPSEX {}
impl ::core::clone::Clone for MDEVICECAPSEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct MIDILOCALE {
    pub ulBank: u32,
    pub ulInstrument: u32,
}
impl ::core::marker::Copy for MIDILOCALE {}
impl ::core::clone::Clone for MIDILOCALE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Media_Multimedia\"`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub struct MIDIOPENDESC {
    pub hMidi: super::HMIDI,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub dnDevNode: usize,
    pub cIds: u32,
    pub rgIds: [super::super::Multimedia::MIDIOPENSTRMID; 1],
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::marker::Copy for MIDIOPENDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::clone::Clone for MIDIOPENDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct POOLCUE {
    pub ulOffset: u32,
}
impl ::core::marker::Copy for POOLCUE {}
impl ::core::clone::Clone for POOLCUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct POOLTABLE {
    pub cbSize: u32,
    pub cCues: u32,
}
impl ::core::marker::Copy for POOLTABLE {}
impl ::core::clone::Clone for POOLTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const POOL_CUE_NULL: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const REFRESH_F_LASTBUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const REGSTR_PATH_SOFTWARESYNTHS: &str = "Software\\Microsoft\\DirectMusic\\SoftwareSynths";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct RGNHEADER {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
}
impl ::core::marker::Copy for RGNHEADER {}
impl ::core::clone::Clone for RGNHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct RGNRANGE {
    pub usLow: u16,
    pub usHigh: u16,
}
impl ::core::marker::Copy for RGNRANGE {}
impl ::core::clone::Clone for RGNRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const SIZE_DVINFO: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct Tag_DVAudInfo {
    pub bAudStyle: [u8; 2],
    pub bAudQu: [u8; 2],
    pub bNumAudPin: u8,
    pub wAvgSamplesPerPinPerFrm: [u16; 2],
    pub wBlkMode: u16,
    pub wDIFMode: u16,
    pub wBlkDiv: u16,
}
impl ::core::marker::Copy for Tag_DVAudInfo {}
impl ::core::clone::Clone for Tag_DVAudInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct WAVELINK {
    pub fusOptions: u16,
    pub usPhaseGroup: u16,
    pub ulChannel: u32,
    pub ulTableIndex: u32,
}
impl ::core::marker::Copy for WAVELINK {}
impl ::core::clone::Clone for WAVELINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const WAVELINK_CHANNEL_LEFT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const WAVELINK_CHANNEL_RIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const WLOOP_TYPE_FORWARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const WLOOP_TYPE_RELEASE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _DMUS_PORTPARAMS {
    pub dwSize: u32,
    pub dwValidParams: u32,
    pub dwVoices: u32,
    pub dwChannelGroups: u32,
    pub dwAudioChannels: u32,
    pub dwSampleRate: u32,
    pub dwEffectFlags: u32,
    pub fShare: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _DMUS_PORTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _DMUS_PORTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct _rloop {
    pub cbSize: u32,
    pub ulType: u32,
    pub ulStart: u32,
    pub ulLength: u32,
}
impl ::core::marker::Copy for _rloop {}
impl ::core::clone::Clone for _rloop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct _rwsmp {
    pub cbSize: u32,
    pub usUnityNote: u16,
    pub sFineTune: i16,
    pub lAttenuation: i32,
    pub fulOptions: u32,
    pub cSampleLoops: u32,
}
impl ::core::marker::Copy for _rwsmp {}
impl ::core::clone::Clone for _rwsmp {
    fn clone(&self) -> Self {
        *self
    }
}
