#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_DirectMusic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1667997456, 3197, 4561, [149, 178, 0, 32, 175, 220, 116, 33]);
pub const CLSID_DirectMusicCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1209005232, 10418, 4561, [190, 247, 0, 192, 79, 191, 143, 239]);
pub const CLSID_DirectMusicSynth: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1489155280, 18151, 4561, [137, 172, 0, 160, 201, 5, 65, 41]);
pub const CLSID_DirectMusicSynthSink: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2931916003, 42260, 4561, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const CLSID_DirectSoundPrivate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(296435392, 9708, 4561, [164, 216, 0, 192, 79, 194, 138, 202]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct CONNECTION {
    pub usSource: u16,
    pub usControl: u16,
    pub usDestination: u16,
    pub usTransform: u16,
    pub lScale: i32,
}
impl CONNECTION {}
impl ::core::default::Default for CONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CONNECTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CONNECTION").field("usSource", &self.usSource).field("usControl", &self.usControl).field("usDestination", &self.usDestination).field("usTransform", &self.usTransform).field("lScale", &self.lScale).finish()
    }
}
impl ::core::cmp::PartialEq for CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.usSource == other.usSource && self.usControl == other.usControl && self.usDestination == other.usDestination && self.usTransform == other.usTransform && self.lScale == other.lScale
    }
}
impl ::core::cmp::Eq for CONNECTION {}
unsafe impl ::windows::runtime::Abi for CONNECTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct CONNECTIONLIST {
    pub cbSize: u32,
    pub cConnections: u32,
}
impl CONNECTIONLIST {}
impl ::core::default::Default for CONNECTIONLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CONNECTIONLIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CONNECTIONLIST").field("cbSize", &self.cbSize).field("cConnections", &self.cConnections).finish()
    }
}
impl ::core::cmp::PartialEq for CONNECTIONLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cConnections == other.cConnections
    }
}
impl ::core::cmp::Eq for CONNECTIONLIST {}
unsafe impl ::windows::runtime::Abi for CONNECTIONLIST {
    type Abi = Self;
}
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
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTSOUNDDEVICE_DATAFLOW(pub i32);
pub const DIRECTSOUNDDEVICE_DATAFLOW_RENDER: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(0i32);
pub const DIRECTSOUNDDEVICE_DATAFLOW_CAPTURE: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(1i32);
impl ::core::convert::From<i32> for DIRECTSOUNDDEVICE_DATAFLOW {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTSOUNDDEVICE_DATAFLOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTSOUNDDEVICE_TYPE(pub i32);
pub const DIRECTSOUNDDEVICE_TYPE_EMULATED: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(0i32);
pub const DIRECTSOUNDDEVICE_TYPE_VXD: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(1i32);
pub const DIRECTSOUNDDEVICE_TYPE_WDM: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(2i32);
impl ::core::convert::From<i32> for DIRECTSOUNDDEVICE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTSOUNDDEVICE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DLSHEADER {
    pub cInstruments: u32,
}
impl DLSHEADER {}
impl ::core::default::Default for DLSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DLSHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DLSHEADER").field("cInstruments", &self.cInstruments).finish()
    }
}
impl ::core::cmp::PartialEq for DLSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cInstruments == other.cInstruments
    }
}
impl ::core::cmp::Eq for DLSHEADER {}
unsafe impl ::windows::runtime::Abi for DLSHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DLSID {
    pub ulData1: u32,
    pub usData2: u16,
    pub usData3: u16,
    pub abData4: [u8; 8],
}
impl DLSID {}
impl ::core::default::Default for DLSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DLSID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DLSID").field("ulData1", &self.ulData1).field("usData2", &self.usData2).field("usData3", &self.usData3).field("abData4", &self.abData4).finish()
    }
}
impl ::core::cmp::PartialEq for DLSID {
    fn eq(&self, other: &Self) -> bool {
        self.ulData1 == other.ulData1 && self.usData2 == other.usData2 && self.usData3 == other.usData3 && self.abData4 == other.abData4
    }
}
impl ::core::cmp::Eq for DLSID {}
unsafe impl ::windows::runtime::Abi for DLSID {
    type Abi = Self;
}
pub const DLSID_GMInHardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259684, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const DLSID_GSInHardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259685, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const DLSID_ManufacturersID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2956857729, 32917, 4562, [161, 239, 0, 96, 8, 51, 219, 216]);
pub const DLSID_ProductID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2956857730, 32917, 4562, [161, 239, 0, 96, 8, 51, 219, 216]);
pub const DLSID_SampleMemorySize: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259688, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const DLSID_SamplePlaybackRate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(714209043, 42175, 4562, [187, 223, 0, 96, 8, 51, 219, 216]);
pub const DLSID_SupportsDLS1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259687, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const DLSID_SupportsDLS2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4047870437, 18057, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const DLSID_XGInHardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259686, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DLSVERSION {
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
}
impl DLSVERSION {}
impl ::core::default::Default for DLSVERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DLSVERSION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DLSVERSION").field("dwVersionMS", &self.dwVersionMS).field("dwVersionLS", &self.dwVersionLS).finish()
    }
}
impl ::core::cmp::PartialEq for DLSVERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersionMS == other.dwVersionMS && self.dwVersionLS == other.dwVersionLS
    }
}
impl ::core::cmp::Eq for DLSVERSION {}
unsafe impl ::windows::runtime::Abi for DLSVERSION {
    type Abi = Self;
}
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_ARTICPARAMS {
    pub LFO: DMUS_LFOPARAMS,
    pub VolEG: DMUS_VEGPARAMS,
    pub PitchEG: DMUS_PEGPARAMS,
    pub Misc: DMUS_MSCPARAMS,
}
impl DMUS_ARTICPARAMS {}
impl ::core::default::Default for DMUS_ARTICPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_ARTICPARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_ARTICPARAMS").field("LFO", &self.LFO).field("VolEG", &self.VolEG).field("PitchEG", &self.PitchEG).field("Misc", &self.Misc).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_ARTICPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.LFO == other.LFO && self.VolEG == other.VolEG && self.PitchEG == other.PitchEG && self.Misc == other.Misc
    }
}
impl ::core::cmp::Eq for DMUS_ARTICPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_ARTICPARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_ARTICULATION {
    pub ulArt1Idx: u32,
    pub ulFirstExtCkIdx: u32,
}
impl DMUS_ARTICULATION {}
impl ::core::default::Default for DMUS_ARTICULATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_ARTICULATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_ARTICULATION").field("ulArt1Idx", &self.ulArt1Idx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_ARTICULATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulArt1Idx == other.ulArt1Idx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx
    }
}
impl ::core::cmp::Eq for DMUS_ARTICULATION {}
unsafe impl ::windows::runtime::Abi for DMUS_ARTICULATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_ARTICULATION2 {
    pub ulArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulNextArtIdx: u32,
}
impl DMUS_ARTICULATION2 {}
impl ::core::default::Default for DMUS_ARTICULATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_ARTICULATION2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_ARTICULATION2").field("ulArtIdx", &self.ulArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulNextArtIdx", &self.ulNextArtIdx).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_ARTICULATION2 {
    fn eq(&self, other: &Self) -> bool {
        self.ulArtIdx == other.ulArtIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.ulNextArtIdx == other.ulNextArtIdx
    }
}
impl ::core::cmp::Eq for DMUS_ARTICULATION2 {}
unsafe impl ::windows::runtime::Abi for DMUS_ARTICULATION2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_BUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidBufferFormat: ::windows::runtime::GUID,
    pub cbBuffer: u32,
}
impl DMUS_BUFFERDESC {}
impl ::core::default::Default for DMUS_BUFFERDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_BUFFERDESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_BUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidBufferFormat", &self.guidBufferFormat).field("cbBuffer", &self.cbBuffer).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_BUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidBufferFormat == other.guidBufferFormat && self.cbBuffer == other.cbBuffer
    }
}
impl ::core::cmp::Eq for DMUS_BUFFERDESC {}
unsafe impl ::windows::runtime::Abi for DMUS_BUFFERDESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_CLOCKF_GLOBAL: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_CLOCKINFO7 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: ::windows::runtime::GUID,
    pub wszDescription: [u16; 128],
}
impl DMUS_CLOCKINFO7 {}
impl ::core::default::Default for DMUS_CLOCKINFO7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_CLOCKINFO7 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_CLOCKINFO7").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_CLOCKINFO7 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ctType == other.ctType && self.guidClock == other.guidClock && self.wszDescription == other.wszDescription
    }
}
impl ::core::cmp::Eq for DMUS_CLOCKINFO7 {}
unsafe impl ::windows::runtime::Abi for DMUS_CLOCKINFO7 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_CLOCKINFO8 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: ::windows::runtime::GUID,
    pub wszDescription: [u16; 128],
    pub dwFlags: u32,
}
impl DMUS_CLOCKINFO8 {}
impl ::core::default::Default for DMUS_CLOCKINFO8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_CLOCKINFO8 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_CLOCKINFO8").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_CLOCKINFO8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ctType == other.ctType && self.guidClock == other.guidClock && self.wszDescription == other.wszDescription && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DMUS_CLOCKINFO8 {}
unsafe impl ::windows::runtime::Abi for DMUS_CLOCKINFO8 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DMUS_CLOCKTYPE(pub i32);
pub const DMUS_CLOCK_SYSTEM: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(0i32);
pub const DMUS_CLOCK_WAVE: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(1i32);
impl ::core::convert::From<i32> for DMUS_CLOCKTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DMUS_CLOCKTYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_COPYRIGHT {
    pub cbSize: u32,
    pub byCopyright: [u8; 4],
}
impl DMUS_COPYRIGHT {}
impl ::core::default::Default for DMUS_COPYRIGHT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_COPYRIGHT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_COPYRIGHT").field("cbSize", &self.cbSize).field("byCopyright", &self.byCopyright).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_COPYRIGHT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.byCopyright == other.byCopyright
    }
}
impl ::core::cmp::Eq for DMUS_COPYRIGHT {}
unsafe impl ::windows::runtime::Abi for DMUS_COPYRIGHT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DEFAULT_SIZE_OFFSETTABLE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_DOWNLOADINFO {
    pub dwDLType: u32,
    pub dwDLId: u32,
    pub dwNumOffsetTableEntries: u32,
    pub cbSize: u32,
}
impl DMUS_DOWNLOADINFO {}
impl ::core::default::Default for DMUS_DOWNLOADINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_DOWNLOADINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_DOWNLOADINFO").field("dwDLType", &self.dwDLType).field("dwDLId", &self.dwDLId).field("dwNumOffsetTableEntries", &self.dwNumOffsetTableEntries).field("cbSize", &self.cbSize).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_DOWNLOADINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDLType == other.dwDLType && self.dwDLId == other.dwDLId && self.dwNumOffsetTableEntries == other.dwNumOffsetTableEntries && self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for DMUS_DOWNLOADINFO {}
unsafe impl ::windows::runtime::Abi for DMUS_DOWNLOADINFO {
    type Abi = Self;
}
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(4))]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_EVENTHEADER {
    pub cbEvent: u32,
    pub dwChannelGroup: u32,
    pub rtDelta: i64,
    pub dwFlags: u32,
}
impl DMUS_EVENTHEADER {}
impl ::core::default::Default for DMUS_EVENTHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_EVENTHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DMUS_EVENTHEADER {}
unsafe impl ::windows::runtime::Abi for DMUS_EVENTHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EVENT_STRUCTURED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_EXTENSIONCHUNK {
    pub cbSize: u32,
    pub ulNextExtCkIdx: u32,
    pub ExtCkID: u32,
    pub byExtCk: [u8; 4],
}
impl DMUS_EXTENSIONCHUNK {}
impl ::core::default::Default for DMUS_EXTENSIONCHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_EXTENSIONCHUNK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_EXTENSIONCHUNK").field("cbSize", &self.cbSize).field("ulNextExtCkIdx", &self.ulNextExtCkIdx).field("ExtCkID", &self.ExtCkID).field("byExtCk", &self.byExtCk).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_EXTENSIONCHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulNextExtCkIdx == other.ulNextExtCkIdx && self.ExtCkID == other.ExtCkID && self.byExtCk == other.byExtCk
    }
}
impl ::core::cmp::Eq for DMUS_EXTENSIONCHUNK {}
unsafe impl ::windows::runtime::Abi for DMUS_EXTENSIONCHUNK {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_INSTRUMENT {
    pub ulPatch: u32,
    pub ulFirstRegionIdx: u32,
    pub ulGlobalArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulFlags: u32,
}
impl DMUS_INSTRUMENT {}
impl ::core::default::Default for DMUS_INSTRUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_INSTRUMENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_INSTRUMENT").field("ulPatch", &self.ulPatch).field("ulFirstRegionIdx", &self.ulFirstRegionIdx).field("ulGlobalArtIdx", &self.ulGlobalArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulCopyrightIdx", &self.ulCopyrightIdx).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_INSTRUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ulPatch == other.ulPatch && self.ulFirstRegionIdx == other.ulFirstRegionIdx && self.ulGlobalArtIdx == other.ulGlobalArtIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.ulCopyrightIdx == other.ulCopyrightIdx && self.ulFlags == other.ulFlags
    }
}
impl ::core::cmp::Eq for DMUS_INSTRUMENT {}
unsafe impl ::windows::runtime::Abi for DMUS_INSTRUMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_INSTRUMENT_GM_INSTRUMENT: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_LFOPARAMS {
    pub pcFrequency: i32,
    pub tcDelay: i32,
    pub gcVolumeScale: i32,
    pub pcPitchScale: i32,
    pub gcMWToVolume: i32,
    pub pcMWToPitch: i32,
}
impl DMUS_LFOPARAMS {}
impl ::core::default::Default for DMUS_LFOPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_LFOPARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_LFOPARAMS").field("pcFrequency", &self.pcFrequency).field("tcDelay", &self.tcDelay).field("gcVolumeScale", &self.gcVolumeScale).field("pcPitchScale", &self.pcPitchScale).field("gcMWToVolume", &self.gcMWToVolume).field("pcMWToPitch", &self.pcMWToPitch).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_LFOPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.pcFrequency == other.pcFrequency && self.tcDelay == other.tcDelay && self.gcVolumeScale == other.gcVolumeScale && self.pcPitchScale == other.pcPitchScale && self.gcMWToVolume == other.gcMWToVolume && self.pcMWToPitch == other.pcMWToPitch
    }
}
impl ::core::cmp::Eq for DMUS_LFOPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_LFOPARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MAX_DESCRIPTION: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MAX_DRIVER: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MIN_DATA_SIZE: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_MSCPARAMS {
    pub ptDefaultPan: i32,
}
impl DMUS_MSCPARAMS {}
impl ::core::default::Default for DMUS_MSCPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_MSCPARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_MSCPARAMS").field("ptDefaultPan", &self.ptDefaultPan).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_MSCPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ptDefaultPan == other.ptDefaultPan
    }
}
impl ::core::cmp::Eq for DMUS_MSCPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_MSCPARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_NOTERANGE {
    pub dwLowNote: u32,
    pub dwHighNote: u32,
}
impl DMUS_NOTERANGE {}
impl ::core::default::Default for DMUS_NOTERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_NOTERANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_NOTERANGE").field("dwLowNote", &self.dwLowNote).field("dwHighNote", &self.dwHighNote).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_NOTERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowNote == other.dwLowNote && self.dwHighNote == other.dwHighNote
    }
}
impl ::core::cmp::Eq for DMUS_NOTERANGE {}
unsafe impl ::windows::runtime::Abi for DMUS_NOTERANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_OFFSETTABLE {
    pub ulOffsetTable: [u32; 1],
}
impl DMUS_OFFSETTABLE {}
impl ::core::default::Default for DMUS_OFFSETTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_OFFSETTABLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_OFFSETTABLE").field("ulOffsetTable", &self.ulOffsetTable).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_OFFSETTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffsetTable == other.ulOffsetTable
    }
}
impl ::core::cmp::Eq for DMUS_OFFSETTABLE {}
unsafe impl ::windows::runtime::Abi for DMUS_OFFSETTABLE {
    type Abi = Self;
}
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_PEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
    pub pcRange: i32,
}
impl DMUS_PEGPARAMS {}
impl ::core::default::Default for DMUS_PEGPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_PEGPARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_PEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).field("pcRange", &self.pcRange).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_PEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.tcAttack == other.tcAttack && self.tcDecay == other.tcDecay && self.ptSustain == other.ptSustain && self.tcRelease == other.tcRelease && self.tcVel2Attack == other.tcVel2Attack && self.tcKey2Decay == other.tcKey2Decay && self.pcRange == other.pcRange
    }
}
impl ::core::cmp::Eq for DMUS_PEGPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_PEGPARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_PORTCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidPort: ::windows::runtime::GUID,
    pub dwClass: u32,
    pub dwType: u32,
    pub dwMemorySize: u32,
    pub dwMaxChannelGroups: u32,
    pub dwMaxVoices: u32,
    pub dwMaxAudioChannels: u32,
    pub dwEffectFlags: u32,
    pub wszDescription: [u16; 128],
}
impl DMUS_PORTCAPS {}
impl ::core::default::Default for DMUS_PORTCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_PORTCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_PORTCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("guidPort", &self.guidPort)
            .field("dwClass", &self.dwClass)
            .field("dwType", &self.dwType)
            .field("dwMemorySize", &self.dwMemorySize)
            .field("dwMaxChannelGroups", &self.dwMaxChannelGroups)
            .field("dwMaxVoices", &self.dwMaxVoices)
            .field("dwMaxAudioChannels", &self.dwMaxAudioChannels)
            .field("dwEffectFlags", &self.dwEffectFlags)
            .field("wszDescription", &self.wszDescription)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_PORTCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidPort == other.guidPort && self.dwClass == other.dwClass && self.dwType == other.dwType && self.dwMemorySize == other.dwMemorySize && self.dwMaxChannelGroups == other.dwMaxChannelGroups && self.dwMaxVoices == other.dwMaxVoices && self.dwMaxAudioChannels == other.dwMaxAudioChannels && self.dwEffectFlags == other.dwEffectFlags && self.wszDescription == other.wszDescription
    }
}
impl ::core::cmp::Eq for DMUS_PORTCAPS {}
unsafe impl ::windows::runtime::Abi for DMUS_PORTCAPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
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
impl DMUS_PORTPARAMS8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMUS_PORTPARAMS8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMUS_PORTPARAMS8 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_PORTPARAMS8")
            .field("dwSize", &self.dwSize)
            .field("dwValidParams", &self.dwValidParams)
            .field("dwVoices", &self.dwVoices)
            .field("dwChannelGroups", &self.dwChannelGroups)
            .field("dwAudioChannels", &self.dwAudioChannels)
            .field("dwSampleRate", &self.dwSampleRate)
            .field("dwEffectFlags", &self.dwEffectFlags)
            .field("fShare", &self.fShare)
            .field("dwFeatures", &self.dwFeatures)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMUS_PORTPARAMS8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidParams == other.dwValidParams && self.dwVoices == other.dwVoices && self.dwChannelGroups == other.dwChannelGroups && self.dwAudioChannels == other.dwAudioChannels && self.dwSampleRate == other.dwSampleRate && self.dwEffectFlags == other.dwEffectFlags && self.fShare == other.fShare && self.dwFeatures == other.dwFeatures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMUS_PORTPARAMS8 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DMUS_PORTPARAMS8 {
    type Abi = Self;
}
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
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
impl DMUS_REGION {}
impl ::core::default::Default for DMUS_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_REGION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_REGION")
            .field("RangeKey", &self.RangeKey)
            .field("RangeVelocity", &self.RangeVelocity)
            .field("fusOptions", &self.fusOptions)
            .field("usKeyGroup", &self.usKeyGroup)
            .field("ulRegionArtIdx", &self.ulRegionArtIdx)
            .field("ulNextRegionIdx", &self.ulNextRegionIdx)
            .field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx)
            .field("WaveLink", &self.WaveLink)
            .field("WSMP", &self.WSMP)
            .field("WLOOP", &self.WLOOP)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.RangeKey == other.RangeKey && self.RangeVelocity == other.RangeVelocity && self.fusOptions == other.fusOptions && self.usKeyGroup == other.usKeyGroup && self.ulRegionArtIdx == other.ulRegionArtIdx && self.ulNextRegionIdx == other.ulNextRegionIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.WaveLink == other.WaveLink && self.WSMP == other.WSMP && self.WLOOP == other.WLOOP
    }
}
impl ::core::cmp::Eq for DMUS_REGION {}
unsafe impl ::windows::runtime::Abi for DMUS_REGION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
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
impl DMUS_SYNTHSTATS {}
impl ::core::default::Default for DMUS_SYNTHSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_SYNTHSTATS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_SYNTHSTATS")
            .field("dwSize", &self.dwSize)
            .field("dwValidStats", &self.dwValidStats)
            .field("dwVoices", &self.dwVoices)
            .field("dwTotalCPU", &self.dwTotalCPU)
            .field("dwCPUPerVoice", &self.dwCPUPerVoice)
            .field("dwLostNotes", &self.dwLostNotes)
            .field("dwFreeMemory", &self.dwFreeMemory)
            .field("lPeakVolume", &self.lPeakVolume)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_SYNTHSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidStats == other.dwValidStats && self.dwVoices == other.dwVoices && self.dwTotalCPU == other.dwTotalCPU && self.dwCPUPerVoice == other.dwCPUPerVoice && self.dwLostNotes == other.dwLostNotes && self.dwFreeMemory == other.dwFreeMemory && self.lPeakVolume == other.lPeakVolume
    }
}
impl ::core::cmp::Eq for DMUS_SYNTHSTATS {}
unsafe impl ::windows::runtime::Abi for DMUS_SYNTHSTATS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
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
impl DMUS_SYNTHSTATS8 {}
impl ::core::default::Default for DMUS_SYNTHSTATS8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_SYNTHSTATS8 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_SYNTHSTATS8")
            .field("dwSize", &self.dwSize)
            .field("dwValidStats", &self.dwValidStats)
            .field("dwVoices", &self.dwVoices)
            .field("dwTotalCPU", &self.dwTotalCPU)
            .field("dwCPUPerVoice", &self.dwCPUPerVoice)
            .field("dwLostNotes", &self.dwLostNotes)
            .field("dwFreeMemory", &self.dwFreeMemory)
            .field("lPeakVolume", &self.lPeakVolume)
            .field("dwSynthMemUse", &self.dwSynthMemUse)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_SYNTHSTATS8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidStats == other.dwValidStats && self.dwVoices == other.dwVoices && self.dwTotalCPU == other.dwTotalCPU && self.dwCPUPerVoice == other.dwCPUPerVoice && self.dwLostNotes == other.dwLostNotes && self.dwFreeMemory == other.dwFreeMemory && self.lPeakVolume == other.lPeakVolume && self.dwSynthMemUse == other.dwSynthMemUse
    }
}
impl ::core::cmp::Eq for DMUS_SYNTHSTATS8 {}
unsafe impl ::windows::runtime::Abi for DMUS_SYNTHSTATS8 {
    type Abi = Self;
}
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_VEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
}
impl DMUS_VEGPARAMS {}
impl ::core::default::Default for DMUS_VEGPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_VEGPARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_VEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_VEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.tcAttack == other.tcAttack && self.tcDecay == other.tcDecay && self.ptSustain == other.ptSustain && self.tcRelease == other.tcRelease && self.tcVel2Attack == other.tcVel2Attack && self.tcKey2Decay == other.tcKey2Decay
    }
}
impl ::core::cmp::Eq for DMUS_VEGPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_VEGPARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DMUS_VOICE_STATE {
    pub bExists: super::super::super::Foundation::BOOL,
    pub spPosition: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl DMUS_VOICE_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMUS_VOICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMUS_VOICE_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_VOICE_STATE").field("bExists", &self.bExists).field("spPosition", &self.spPosition).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMUS_VOICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.bExists == other.bExists && self.spPosition == other.spPosition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMUS_VOICE_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DMUS_VOICE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_VOLUME_MAX: u32 = 2000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_VOLUME_MIN: i32 = -20000i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVE {
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulWaveDataIdx: u32,
    pub WaveformatEx: super::WAVEFORMATEX,
}
impl DMUS_WAVE {}
impl ::core::default::Default for DMUS_WAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DMUS_WAVE {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVEARTDL {
    pub ulDownloadIdIdx: u32,
    pub ulBus: u32,
    pub ulBuffers: u32,
    pub ulMasterDLId: u32,
    pub usOptions: u16,
}
impl DMUS_WAVEARTDL {}
impl ::core::default::Default for DMUS_WAVEARTDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_WAVEARTDL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_WAVEARTDL").field("ulDownloadIdIdx", &self.ulDownloadIdIdx).field("ulBus", &self.ulBus).field("ulBuffers", &self.ulBuffers).field("ulMasterDLId", &self.ulMasterDLId).field("usOptions", &self.usOptions).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVEARTDL {
    fn eq(&self, other: &Self) -> bool {
        self.ulDownloadIdIdx == other.ulDownloadIdIdx && self.ulBus == other.ulBus && self.ulBuffers == other.ulBuffers && self.ulMasterDLId == other.ulMasterDLId && self.usOptions == other.usOptions
    }
}
impl ::core::cmp::Eq for DMUS_WAVEARTDL {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVEARTDL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVEDATA {
    pub cbSize: u32,
    pub byData: [u8; 4],
}
impl DMUS_WAVEDATA {}
impl ::core::default::Default for DMUS_WAVEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_WAVEDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_WAVEDATA").field("cbSize", &self.cbSize).field("byData", &self.byData).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.byData == other.byData
    }
}
impl ::core::cmp::Eq for DMUS_WAVEDATA {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVEDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVEDL {
    pub cbWaveData: u32,
}
impl DMUS_WAVEDL {}
impl ::core::default::Default for DMUS_WAVEDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_WAVEDL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_WAVEDL").field("cbWaveData", &self.cbWaveData).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVEDL {
    fn eq(&self, other: &Self) -> bool {
        self.cbWaveData == other.cbWaveData
    }
}
impl ::core::cmp::Eq for DMUS_WAVEDL {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVEDL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVES_REVERB_PARAMS {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl DMUS_WAVES_REVERB_PARAMS {}
impl ::core::default::Default for DMUS_WAVES_REVERB_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DMUS_WAVES_REVERB_PARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DMUS_WAVES_REVERB_PARAMS").field("fInGain", &self.fInGain).field("fReverbMix", &self.fReverbMix).field("fReverbTime", &self.fReverbTime).field("fHighFreqRTRatio", &self.fHighFreqRTRatio).finish()
    }
}
impl ::core::cmp::PartialEq for DMUS_WAVES_REVERB_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.fInGain == other.fInGain && self.fReverbMix == other.fReverbMix && self.fReverbTime == other.fReverbTime && self.fHighFreqRTRatio == other.fHighFreqRTRatio
    }
}
impl ::core::cmp::Eq for DMUS_WAVES_REVERB_PARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVES_REVERB_PARAMS {
    type Abi = Self;
}
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
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE(pub i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(1i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(2i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(3i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(4i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(5i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(6i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(7i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(8i32);
impl ::core::convert::From<i32> for DSPROPERTY_DIRECTSOUNDDEVICE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    pub DeviceId: ::windows::runtime::GUID,
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
impl DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA")
            .field("DeviceId", &self.DeviceId)
            .field("DescriptionA", &self.DescriptionA)
            .field("DescriptionW", &self.DescriptionW)
            .field("ModuleA", &self.ModuleA)
            .field("ModuleW", &self.ModuleW)
            .field("Type", &self.Type)
            .field("DataFlow", &self.DataFlow)
            .field("WaveDeviceId", &self.WaveDeviceId)
            .field("Devnode", &self.Devnode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceId == other.DeviceId && self.DescriptionA == other.DescriptionA && self.DescriptionW == other.DescriptionW && self.ModuleA == other.ModuleA && self.ModuleW == other.ModuleW && self.Type == other.Type && self.DataFlow == other.DataFlow && self.WaveDeviceId == other.WaveDeviceId && self.Devnode == other.Devnode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::runtime::GUID,
    pub Description: super::super::super::Foundation::PSTR,
    pub Module: super::super::super::Foundation::PSTR,
    pub Interface: super::super::super::Foundation::PSTR,
    pub WaveDeviceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA")
            .field("Type", &self.Type)
            .field("DataFlow", &self.DataFlow)
            .field("DeviceId", &self.DeviceId)
            .field("Description", &self.Description)
            .field("Module", &self.Module)
            .field("Interface", &self.Interface)
            .field("WaveDeviceId", &self.WaveDeviceId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId && self.Description == other.Description && self.Module == other.Module && self.Interface == other.Interface && self.WaveDeviceId == other.WaveDeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::runtime::GUID,
    pub Description: super::super::super::Foundation::PWSTR,
    pub Module: super::super::super::Foundation::PWSTR,
    pub Interface: super::super::super::Foundation::PWSTR,
    pub WaveDeviceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA")
            .field("Type", &self.Type)
            .field("DataFlow", &self.DataFlow)
            .field("DeviceId", &self.DeviceId)
            .field("Description", &self.Description)
            .field("Module", &self.Module)
            .field("Interface", &self.Interface)
            .field("WaveDeviceId", &self.WaveDeviceId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId && self.Description == other.Description && self.Module == other.Module && self.Interface == other.Interface && self.WaveDeviceId == other.WaveDeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    pub Callback: ::core::option::Option<LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1>,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA").field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize) && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    pub Callback: ::core::option::Option<LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA>,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA").field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize) && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    pub Callback: ::core::option::Option<LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW>,
    pub Context: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA").field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize) && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    pub DeviceName: super::super::super::Foundation::PSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    pub DeviceName: super::super::super::Foundation::PWSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    type Abi = Self;
}
pub const DSPROPSETID_DirectSoundDevice: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2221035394, 9708, 4561, [164, 216, 0, 192, 79, 194, 138, 202]);
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
pub const GUID_DMUS_PROP_DLS1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259687, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_DLS2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4047870437, 18057, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_Effects: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3450394129, 26698, 4562, [135, 30, 0, 96, 8, 147, 177, 189]);
pub const GUID_DMUS_PROP_GM_Hardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259684, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_GS_Capable: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1687595938, 25008, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_GS_Hardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259685, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_INSTRUMENT2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2254426994, 40807, 4562, [135, 42, 0, 96, 8, 147, 177, 189]);
pub const GUID_DMUS_PROP_LegacyCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3483880898, 161, 4562, [170, 213, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_MemorySize: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259688, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_SampleMemorySize: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259688, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_SamplePlaybackRate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(714209043, 42175, 4562, [187, 223, 0, 96, 8, 51, 219, 216]);
pub const GUID_DMUS_PROP_SetSynthSink: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(171596709, 14262, 4562, [185, 249, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_SinkUsesDSound: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3189803095, 35154, 4562, [186, 28, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_SynthSink_DSOUND: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(178878532, 51319, 4561, [135, 12, 0, 96, 8, 147, 177, 189]);
pub const GUID_DMUS_PROP_SynthSink_WAVE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(178878533, 51319, 4561, [135, 12, 0, 96, 8, 147, 177, 189]);
pub const GUID_DMUS_PROP_Volume: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4276071973, 58478, 4561, [170, 206, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_WavesReverb: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(80434722, 13029, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_WriteLatency: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(646582176, 24818, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_WritePeriod: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(646582177, 24818, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_XG_Capable: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1687595937, 25008, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_XG_Hardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259686, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusic(pub ::windows::runtime::IUnknown);
impl IDirectMusic {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pportcaps)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CreateMusicBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbufferdesc), ::core::mem::transmute(ppbuffer), punkouter.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn CreatePort<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, rclsidport: *const ::windows::runtime::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsidport), ::core::mem::transmute(pportparams), ::core::mem::transmute(ppport), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpclockinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::runtime::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidclock), ::core::mem::transmute(ppreferenceclock)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidclock)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidport)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`, `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectSound::IDirectSound>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, pdirectsound: Param0, hwnd: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), hwnd.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusic {
    type Vtable = IDirectMusic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1698042202, 31533, 4562, [186, 24, 0, 0, 248, 117, 172, 18]);
}
impl ::core::convert::From<IDirectMusic> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusic) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusic> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusic) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsidport: *const ::windows::runtime::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidclock: *mut ::windows::runtime::GUID, ppreferenceclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidclock: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidport: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound")))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusic8(pub ::windows::runtime::IUnknown);
impl IDirectMusic8 {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pportcaps)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CreateMusicBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbufferdesc), ::core::mem::transmute(ppbuffer), punkouter.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn CreatePort<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, rclsidport: *const ::windows::runtime::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsidport), ::core::mem::transmute(pportparams), ::core::mem::transmute(ppport), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpclockinfo)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::runtime::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidclock), ::core::mem::transmute(ppreferenceclock)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidclock)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidport)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`, `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectSound::IDirectSound>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, pdirectsound: Param0, hwnd: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), hwnd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetExternalMasterClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::IReferenceClock>>(&self, pclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusic8 {
    type Vtable = IDirectMusic8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(758524407, 33085, 18745, [133, 8, 240, 92, 107, 117, 253, 151]);
}
impl ::core::convert::From<IDirectMusic8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusic8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusic8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusic8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusic8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusic8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDirectMusic8> for IDirectMusic {
    fn from(value: IDirectMusic8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusic8> for IDirectMusic {
    fn from(value: &IDirectMusic8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectMusic> for IDirectMusic8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectMusic> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectMusic> for &IDirectMusic8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectMusic> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsidport: *const ::windows::runtime::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidclock: *mut ::windows::runtime::GUID, ppreferenceclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidclock: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidport: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclock: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicBuffer(pub ::windows::runtime::IUnknown);
impl IDirectMusicBuffer {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn TotalTime(&self, prttime: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(prttime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannelmessage)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(cb), ::core::mem::transmute(lpb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn ResetReadPtr(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(prt), ::core::mem::transmute(pdwchannelgroup), ::core::mem::transmute(pdwlength), ::core::mem::transmute(ppdata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetStartTime(&self, prt: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(prt)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetUsedBytes(&self, pcb: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetMaxBytes(&self, pcb: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetBufferFormat(&self, pguidformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetStartTime(&self, rt: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetUsedBytes(&self, cb: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cb)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicBuffer {
    type Vtable = IDirectMusicBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497912, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::core::convert::From<IDirectMusicBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prttime: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdata: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prt: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcb: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcb: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cb: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicCollection(pub ::windows::runtime::IUnknown);
impl IDirectMusicCollection {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetInstrument(&self, dwpatch: u32) -> ::windows::runtime::Result<IDirectMusicInstrument> {
        let mut result__: <IDirectMusicInstrument as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpatch), &mut result__).from_abi::<IDirectMusicInstrument>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn EnumInstrument<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: Param2, dwnamelen: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pdwpatch), pwszname.into_param().abi(), ::core::mem::transmute(dwnamelen)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicCollection {
    type Vtable = IDirectMusicCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497916, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::core::convert::From<IDirectMusicCollection> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpatch: u32, ppinstrument: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pdwpatch: *mut u32, pwszname: super::super::super::Foundation::PWSTR, dwnamelen: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicDownload(pub ::windows::runtime::IUnknown);
impl IDirectMusicDownload {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetBuffer(&self, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvbuffer), ::core::mem::transmute(pdwsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicDownload {
    type Vtable = IDirectMusicDownload_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497915, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::core::convert::From<IDirectMusicDownload> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicDownload) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicDownload> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicDownload) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicDownload {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicDownload {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownload_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicDownloadedInstrument(pub ::windows::runtime::IUnknown);
impl IDirectMusicDownloadedInstrument {}
unsafe impl ::windows::runtime::Interface for IDirectMusicDownloadedInstrument {
    type Vtable = IDirectMusicDownloadedInstrument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497918, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::core::convert::From<IDirectMusicDownloadedInstrument> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicDownloadedInstrument) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicDownloadedInstrument> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicDownloadedInstrument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicDownloadedInstrument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicDownloadedInstrument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownloadedInstrument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicInstrument(pub ::windows::runtime::IUnknown);
impl IDirectMusicInstrument {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPatch(&self, pdwpatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpatch)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetPatch(&self, dwpatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpatch)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicInstrument {
    type Vtable = IDirectMusicInstrument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497917, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::core::convert::From<IDirectMusicInstrument> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicInstrument) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicInstrument> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicInstrument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicInstrument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicInstrument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicInstrument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpatch: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicPort(pub ::windows::runtime::IUnknown);
impl IDirectMusicPort {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PlayBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicBuffer>>(&self, pbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetReadNotificationHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Read<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicBuffer>>(&self, pbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn DownloadInstrument<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicInstrument>>(&self, pinstrument: Param0, ppdownloadedinstrument: *mut ::core::option::Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pinstrument.into_param().abi(), ::core::mem::transmute(ppdownloadedinstrument), ::core::mem::transmute(pnoteranges), ::core::mem::transmute(dwnumnoteranges)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn UnloadInstrument<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicDownloadedInstrument>>(&self, pdownloadedinstrument: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdownloadedinstrument.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetLatencyClock(&self) -> ::windows::runtime::Result<super::super::IReferenceClock> {
        let mut result__: <super::super::IReferenceClock as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstats)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pportcaps)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`, `Win32_System_IO`*"]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dwiocontrolcode),
            ::core::mem::transmute(lpinbuffer),
            ::core::mem::transmute(ninbuffersize),
            ::core::mem::transmute(lpoutbuffer),
            ::core::mem::transmute(noutbuffersize),
            ::core::mem::transmute(lpbytesreturned),
            ::core::mem::transmute(lpoverlapped),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroups)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwchannelgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, factive: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), factive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(dwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(pdwpriority)).ok()
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectSound::IDirectSound>, Param1: ::windows::runtime::IntoParam<'a, super::DirectSound::IDirectSoundBuffer>>(&self, pdirectsound: Param0, pdirectsoundbuffer: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), pdirectsoundbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize), ::core::mem::transmute(pdwbuffersize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicPort {
    type Vtable = IDirectMusicPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(150132937, 14274, 4562, [185, 249, 0, 0, 248, 117, 172, 18]);
}
impl ::core::convert::From<IDirectMusicPort> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicPort) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicPort> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hevent: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinstrument: ::windows::runtime::RawPtr, ppdownloadedinstrument: *mut ::windows::runtime::RawPtr, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdownloadedinstrument: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroups: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwchannelgroups: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factive: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, pdirectsoundbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicPortDownload(pub ::windows::runtime::IUnknown);
impl IDirectMusicPortDownload {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetBuffer(&self, dwdlid: u32) -> ::windows::runtime::Result<IDirectMusicDownload> {
        let mut result__: <IDirectMusicDownload as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdlid), &mut result__).from_abi::<IDirectMusicDownload>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn AllocateBuffer(&self, dwsize: u32) -> ::windows::runtime::Result<IDirectMusicDownload> {
        let mut result__: <IDirectMusicDownload as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsize), &mut result__).from_abi::<IDirectMusicDownload>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwstartdlid), ::core::mem::transmute(dwcount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwappend)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Download<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicDownload>>(&self, pidmdownload: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pidmdownload.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Unload<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicDownload>>(&self, pidmdownload: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pidmdownload.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicPortDownload {
    type Vtable = IDirectMusicPortDownload_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497914, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::core::convert::From<IDirectMusicPortDownload> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicPortDownload) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicPortDownload> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicPortDownload) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicPortDownload {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicPortDownload {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPortDownload_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdlid: u32, ppidmdownload: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsize: u32, ppidmdownload: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwappend: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidmdownload: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidmdownload: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicSynth(pub ::windows::runtime::IUnknown);
impl IDirectMusicSynth {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pportparams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(phdownload), ::core::mem::transmute(pvdata), ::core::mem::transmute(pbfree)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Unload<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hdownload: Param0, lpfreehandle: isize, huserdata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), hdownload.into_param().abi(), ::core::mem::transmute(lpfreehandle), huserdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(cbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstats)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcaps)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::IReferenceClock>>(&self, pclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetLatencyClock(&self) -> ::windows::runtime::Result<super::super::IReferenceClock> {
        let mut result__: <super::super::IReferenceClock as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetSynthSink<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicSynthSink>>(&self, psynthsink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psynthsink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwlength), ::core::mem::transmute(llposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(dwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(pdwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwappend)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicSynth {
    type Vtable = IDirectMusicSynth_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(159528545, 23685, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
}
impl ::core::convert::From<IDirectMusicSynth> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicSynth) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicSynth> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicSynth) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicSynth {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicSynth {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwgroups: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclock: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psynthsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwappend: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicSynth8(pub ::windows::runtime::IUnknown);
impl IDirectMusicSynth8 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pportparams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(phdownload), ::core::mem::transmute(pvdata), ::core::mem::transmute(pbfree)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Unload<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hdownload: Param0, lpfreehandle: isize, huserdata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), hdownload.into_param().abi(), ::core::mem::transmute(lpfreehandle), huserdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(cbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstats)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcaps)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::IReferenceClock>>(&self, pclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetLatencyClock(&self) -> ::windows::runtime::Result<super::super::IReferenceClock> {
        let mut result__: <super::super::IReferenceClock as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetSynthSink<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicSynthSink>>(&self, psynthsink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psynthsink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwlength), ::core::mem::transmute(llposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(dwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(pdwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwappend)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(rt),
            ::core::mem::transmute(dwvoiceid),
            ::core::mem::transmute(dwchannelgroup),
            ::core::mem::transmute(dwchannel),
            ::core::mem::transmute(dwdlid),
            ::core::mem::transmute(prpitch),
            ::core::mem::transmute(vrvolume),
            ::core::mem::transmute(stvoicestart),
            ::core::mem::transmute(stloopstart),
            ::core::mem::transmute(stloopend),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(dwvoiceid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwvoice), ::core::mem::transmute(cbvoice), ::core::mem::transmute(dwvoicestate)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdownloadid), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(pdwbuses), ::core::mem::transmute(cbuses)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicSynth8 {
    type Vtable = IDirectMusicSynth8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1405793829, 10001, 19615, [157, 231, 27, 127, 146, 95, 111, 200]);
}
impl ::core::convert::From<IDirectMusicSynth8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicSynth8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicSynth8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicSynth8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IDirectMusicSynth8> for IDirectMusicSynth {
    fn from(value: IDirectMusicSynth8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicSynth8> for IDirectMusicSynth {
    fn from(value: &IDirectMusicSynth8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectMusicSynth> for IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectMusicSynth> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectMusicSynth> for &IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectMusicSynth> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwgroups: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclock: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psynthsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwappend: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, dwvoiceid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdownloadid: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicSynthSink(pub ::windows::runtime::IUnknown);
impl IDirectMusicSynthSink {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Init<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicSynth>>(&self, psynth: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psynth.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::IReferenceClock>>(&self, pclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetLatencyClock(&self) -> ::windows::runtime::Result<super::super::IReferenceClock> {
        let mut result__: <super::super::IReferenceClock as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(llsampletime), ::core::mem::transmute(prftime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rftime), ::core::mem::transmute(pllsampletime)).ok()
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectSound::IDirectSound>, Param1: ::windows::runtime::IntoParam<'a, super::DirectSound::IDirectSoundBuffer>>(&self, pdirectsound: Param0, pdirectsoundbuffer: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), pdirectsoundbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbuffersizeinsamples)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicSynthSink {
    type Vtable = IDirectMusicSynthSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(159528547, 23685, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
}
impl ::core::convert::From<IDirectMusicSynthSink> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicSynthSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicSynthSink> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicSynthSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicSynthSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicSynthSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynthSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psynth: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclock: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, llsampletime: i64, prftime: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rftime: i64, pllsampletime: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, pdirectsoundbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbuffersizeinsamples: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectMusicThru(pub ::windows::runtime::IUnknown);
impl IDirectMusicThru {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn ThruChannel<'a, Param4: ::windows::runtime::IntoParam<'a, IDirectMusicPort>>(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsourcechannelgroup), ::core::mem::transmute(dwsourcechannel), ::core::mem::transmute(dwdestinationchannelgroup), ::core::mem::transmute(dwdestinationchannel), pdestinationport.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicThru {
    type Vtable = IDirectMusicThru_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3469824999, 13830, 4562, [185, 249, 0, 0, 248, 117, 172, 18]);
}
impl ::core::convert::From<IDirectMusicThru> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicThru) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectMusicThru> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicThru) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicThru {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicThru {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicThru_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct INSTHEADER {
    pub cRegions: u32,
    pub Locale: MIDILOCALE,
}
impl INSTHEADER {}
impl ::core::default::Default for INSTHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INSTHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INSTHEADER").field("cRegions", &self.cRegions).field("Locale", &self.Locale).finish()
    }
}
impl ::core::cmp::PartialEq for INSTHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cRegions == other.cRegions && self.Locale == other.Locale
    }
}
impl ::core::cmp::Eq for INSTHEADER {}
unsafe impl ::windows::runtime::Abi for INSTHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1 = unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA = unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW = unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct MDEVICECAPSEX {
    pub cbSize: u32,
    pub pCaps: *mut ::core::ffi::c_void,
}
impl MDEVICECAPSEX {}
impl ::core::default::Default for MDEVICECAPSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MDEVICECAPSEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MDEVICECAPSEX {}
unsafe impl ::windows::runtime::Abi for MDEVICECAPSEX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct MIDILOCALE {
    pub ulBank: u32,
    pub ulInstrument: u32,
}
impl MIDILOCALE {}
impl ::core::default::Default for MIDILOCALE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MIDILOCALE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MIDILOCALE").field("ulBank", &self.ulBank).field("ulInstrument", &self.ulInstrument).finish()
    }
}
impl ::core::cmp::PartialEq for MIDILOCALE {
    fn eq(&self, other: &Self) -> bool {
        self.ulBank == other.ulBank && self.ulInstrument == other.ulInstrument
    }
}
impl ::core::cmp::Eq for MIDILOCALE {}
unsafe impl ::windows::runtime::Abi for MIDILOCALE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Multimedia")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
pub struct MIDIOPENDESC {
    pub hMidi: super::HMIDI,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub dnDevNode: usize,
    pub cIds: u32,
    pub rgIds: [super::super::Multimedia::MIDIOPENSTRMID; 1],
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl MIDIOPENDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::default::Default for MIDIOPENDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::cmp::PartialEq for MIDIOPENDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::cmp::Eq for MIDIOPENDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::runtime::Abi for MIDIOPENDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct POOLCUE {
    pub ulOffset: u32,
}
impl POOLCUE {}
impl ::core::default::Default for POOLCUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for POOLCUE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POOLCUE").field("ulOffset", &self.ulOffset).finish()
    }
}
impl ::core::cmp::PartialEq for POOLCUE {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffset == other.ulOffset
    }
}
impl ::core::cmp::Eq for POOLCUE {}
unsafe impl ::windows::runtime::Abi for POOLCUE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct POOLTABLE {
    pub cbSize: u32,
    pub cCues: u32,
}
impl POOLTABLE {}
impl ::core::default::Default for POOLTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for POOLTABLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POOLTABLE").field("cbSize", &self.cbSize).field("cCues", &self.cCues).finish()
    }
}
impl ::core::cmp::PartialEq for POOLTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cCues == other.cCues
    }
}
impl ::core::cmp::Eq for POOLTABLE {}
unsafe impl ::windows::runtime::Abi for POOLTABLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const POOL_CUE_NULL: i32 = -1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const REFRESH_F_LASTBUFFER: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct RGNHEADER {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
}
impl RGNHEADER {}
impl ::core::default::Default for RGNHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RGNHEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RGNHEADER").field("RangeKey", &self.RangeKey).field("RangeVelocity", &self.RangeVelocity).field("fusOptions", &self.fusOptions).field("usKeyGroup", &self.usKeyGroup).finish()
    }
}
impl ::core::cmp::PartialEq for RGNHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RangeKey == other.RangeKey && self.RangeVelocity == other.RangeVelocity && self.fusOptions == other.fusOptions && self.usKeyGroup == other.usKeyGroup
    }
}
impl ::core::cmp::Eq for RGNHEADER {}
unsafe impl ::windows::runtime::Abi for RGNHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct RGNRANGE {
    pub usLow: u16,
    pub usHigh: u16,
}
impl RGNRANGE {}
impl ::core::default::Default for RGNRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RGNRANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RGNRANGE").field("usLow", &self.usLow).field("usHigh", &self.usHigh).finish()
    }
}
impl ::core::cmp::PartialEq for RGNRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.usLow == other.usLow && self.usHigh == other.usHigh
    }
}
impl ::core::cmp::Eq for RGNRANGE {}
unsafe impl ::windows::runtime::Abi for RGNRANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const SIZE_DVINFO: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct Tag_DVAudInfo {
    pub bAudStyle: [u8; 2],
    pub bAudQu: [u8; 2],
    pub bNumAudPin: u8,
    pub wAvgSamplesPerPinPerFrm: [u16; 2],
    pub wBlkMode: u16,
    pub wDIFMode: u16,
    pub wBlkDiv: u16,
}
impl Tag_DVAudInfo {}
impl ::core::default::Default for Tag_DVAudInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Tag_DVAudInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Tag_DVAudInfo")
            .field("bAudStyle", &self.bAudStyle)
            .field("bAudQu", &self.bAudQu)
            .field("bNumAudPin", &self.bNumAudPin)
            .field("wAvgSamplesPerPinPerFrm", &self.wAvgSamplesPerPinPerFrm)
            .field("wBlkMode", &self.wBlkMode)
            .field("wDIFMode", &self.wDIFMode)
            .field("wBlkDiv", &self.wBlkDiv)
            .finish()
    }
}
impl ::core::cmp::PartialEq for Tag_DVAudInfo {
    fn eq(&self, other: &Self) -> bool {
        self.bAudStyle == other.bAudStyle && self.bAudQu == other.bAudQu && self.bNumAudPin == other.bNumAudPin && self.wAvgSamplesPerPinPerFrm == other.wAvgSamplesPerPinPerFrm && self.wBlkMode == other.wBlkMode && self.wDIFMode == other.wDIFMode && self.wBlkDiv == other.wBlkDiv
    }
}
impl ::core::cmp::Eq for Tag_DVAudInfo {}
unsafe impl ::windows::runtime::Abi for Tag_DVAudInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct WAVELINK {
    pub fusOptions: u16,
    pub usPhaseGroup: u16,
    pub ulChannel: u32,
    pub ulTableIndex: u32,
}
impl WAVELINK {}
impl ::core::default::Default for WAVELINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WAVELINK {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WAVELINK").field("fusOptions", &self.fusOptions).field("usPhaseGroup", &self.usPhaseGroup).field("ulChannel", &self.ulChannel).field("ulTableIndex", &self.ulTableIndex).finish()
    }
}
impl ::core::cmp::PartialEq for WAVELINK {
    fn eq(&self, other: &Self) -> bool {
        self.fusOptions == other.fusOptions && self.usPhaseGroup == other.usPhaseGroup && self.ulChannel == other.ulChannel && self.ulTableIndex == other.ulTableIndex
    }
}
impl ::core::cmp::Eq for WAVELINK {}
unsafe impl ::windows::runtime::Abi for WAVELINK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WAVELINK_CHANNEL_LEFT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WAVELINK_CHANNEL_RIGHT: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WLOOP_TYPE_FORWARD: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WLOOP_TYPE_RELEASE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
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
impl _DMUS_PORTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _DMUS_PORTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _DMUS_PORTPARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_DMUS_PORTPARAMS")
            .field("dwSize", &self.dwSize)
            .field("dwValidParams", &self.dwValidParams)
            .field("dwVoices", &self.dwVoices)
            .field("dwChannelGroups", &self.dwChannelGroups)
            .field("dwAudioChannels", &self.dwAudioChannels)
            .field("dwSampleRate", &self.dwSampleRate)
            .field("dwEffectFlags", &self.dwEffectFlags)
            .field("fShare", &self.fShare)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _DMUS_PORTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidParams == other.dwValidParams && self.dwVoices == other.dwVoices && self.dwChannelGroups == other.dwChannelGroups && self.dwAudioChannels == other.dwAudioChannels && self.dwSampleRate == other.dwSampleRate && self.dwEffectFlags == other.dwEffectFlags && self.fShare == other.fShare
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _DMUS_PORTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for _DMUS_PORTPARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct _rloop {
    pub cbSize: u32,
    pub ulType: u32,
    pub ulStart: u32,
    pub ulLength: u32,
}
impl _rloop {}
impl ::core::default::Default for _rloop {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for _rloop {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_rloop").field("cbSize", &self.cbSize).field("ulType", &self.ulType).field("ulStart", &self.ulStart).field("ulLength", &self.ulLength).finish()
    }
}
impl ::core::cmp::PartialEq for _rloop {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulType == other.ulType && self.ulStart == other.ulStart && self.ulLength == other.ulLength
    }
}
impl ::core::cmp::Eq for _rloop {}
unsafe impl ::windows::runtime::Abi for _rloop {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct _rwsmp {
    pub cbSize: u32,
    pub usUnityNote: u16,
    pub sFineTune: i16,
    pub lAttenuation: i32,
    pub fulOptions: u32,
    pub cSampleLoops: u32,
}
impl _rwsmp {}
impl ::core::default::Default for _rwsmp {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for _rwsmp {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_rwsmp").field("cbSize", &self.cbSize).field("usUnityNote", &self.usUnityNote).field("sFineTune", &self.sFineTune).field("lAttenuation", &self.lAttenuation).field("fulOptions", &self.fulOptions).field("cSampleLoops", &self.cSampleLoops).finish()
    }
}
impl ::core::cmp::PartialEq for _rwsmp {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.usUnityNote == other.usUnityNote && self.sFineTune == other.sFineTune && self.lAttenuation == other.lAttenuation && self.fulOptions == other.fulOptions && self.cSampleLoops == other.cSampleLoops
    }
}
impl ::core::cmp::Eq for _rwsmp {}
unsafe impl ::windows::runtime::Abi for _rwsmp {
    type Abi = Self;
}
