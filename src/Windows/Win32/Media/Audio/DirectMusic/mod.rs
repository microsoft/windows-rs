#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CLSID_DirectMusic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x636b9f10_0c7d_11d1_95b2_0020afdc7421);
pub const CLSID_DirectMusicCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x480ff4b0_28b2_11d1_bef7_00c04fbf8fef);
pub const CLSID_DirectMusicSynth: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58c2b4d0_46e7_11d1_89ac_00a0c9054129);
pub const CLSID_DirectMusicSynthSink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec17ce3_a514_11d1_afa6_00aa0024d8b6);
pub const CLSID_DirectSoundPrivate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11ab3ec0_25ec_11d1_a4d8_00c04fc28aca);
#[repr(C)]
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
unsafe impl ::windows::core::Abi for CONNECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONNECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONNECTION {}
impl ::core::default::Default for CONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for CONNECTIONLIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONNECTIONLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONNECTIONLIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONNECTIONLIST {}
impl ::core::default::Default for CONNECTIONLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CONN_DST_ATTENUATION: u32 = 1u32;
pub const CONN_DST_CENTER: u32 = 18u32;
pub const CONN_DST_CHORUS: u32 = 128u32;
pub const CONN_DST_EG1_ATTACKTIME: u32 = 518u32;
pub const CONN_DST_EG1_DECAYTIME: u32 = 519u32;
pub const CONN_DST_EG1_DELAYTIME: u32 = 523u32;
pub const CONN_DST_EG1_HOLDTIME: u32 = 524u32;
pub const CONN_DST_EG1_RELEASETIME: u32 = 521u32;
pub const CONN_DST_EG1_SHUTDOWNTIME: u32 = 525u32;
pub const CONN_DST_EG1_SUSTAINLEVEL: u32 = 522u32;
pub const CONN_DST_EG2_ATTACKTIME: u32 = 778u32;
pub const CONN_DST_EG2_DECAYTIME: u32 = 779u32;
pub const CONN_DST_EG2_DELAYTIME: u32 = 783u32;
pub const CONN_DST_EG2_HOLDTIME: u32 = 784u32;
pub const CONN_DST_EG2_RELEASETIME: u32 = 781u32;
pub const CONN_DST_EG2_SUSTAINLEVEL: u32 = 782u32;
pub const CONN_DST_FILTER_CUTOFF: u32 = 1280u32;
pub const CONN_DST_FILTER_Q: u32 = 1281u32;
pub const CONN_DST_GAIN: u32 = 1u32;
pub const CONN_DST_KEYNUMBER: u32 = 5u32;
pub const CONN_DST_LEFT: u32 = 16u32;
pub const CONN_DST_LEFTREAR: u32 = 19u32;
pub const CONN_DST_LFE_CHANNEL: u32 = 21u32;
pub const CONN_DST_LFO_FREQUENCY: u32 = 260u32;
pub const CONN_DST_LFO_STARTDELAY: u32 = 261u32;
pub const CONN_DST_NONE: u32 = 0u32;
pub const CONN_DST_PAN: u32 = 4u32;
pub const CONN_DST_PITCH: u32 = 3u32;
pub const CONN_DST_REVERB: u32 = 129u32;
pub const CONN_DST_RIGHT: u32 = 17u32;
pub const CONN_DST_RIGHTREAR: u32 = 20u32;
pub const CONN_DST_VIB_FREQUENCY: u32 = 276u32;
pub const CONN_DST_VIB_STARTDELAY: u32 = 277u32;
pub const CONN_SRC_CC1: u32 = 129u32;
pub const CONN_SRC_CC10: u32 = 138u32;
pub const CONN_SRC_CC11: u32 = 139u32;
pub const CONN_SRC_CC7: u32 = 135u32;
pub const CONN_SRC_CC91: u32 = 219u32;
pub const CONN_SRC_CC93: u32 = 221u32;
pub const CONN_SRC_CHANNELPRESSURE: u32 = 8u32;
pub const CONN_SRC_EG1: u32 = 4u32;
pub const CONN_SRC_EG2: u32 = 5u32;
pub const CONN_SRC_KEYNUMBER: u32 = 3u32;
pub const CONN_SRC_KEYONVELOCITY: u32 = 2u32;
pub const CONN_SRC_LFO: u32 = 1u32;
pub const CONN_SRC_MONOPRESSURE: u32 = 10u32;
pub const CONN_SRC_NONE: u32 = 0u32;
pub const CONN_SRC_PITCHWHEEL: u32 = 6u32;
pub const CONN_SRC_POLYPRESSURE: u32 = 7u32;
pub const CONN_SRC_VIBRATO: u32 = 9u32;
pub const CONN_TRN_CONCAVE: u32 = 1u32;
pub const CONN_TRN_CONVEX: u32 = 2u32;
pub const CONN_TRN_NONE: u32 = 0u32;
pub const CONN_TRN_SWITCH: u32 = 3u32;
pub const DAUD_CHAN10_VOICE_PRIORITY_OFFSET: u32 = 15u32;
pub const DAUD_CHAN11_VOICE_PRIORITY_OFFSET: u32 = 5u32;
pub const DAUD_CHAN12_VOICE_PRIORITY_OFFSET: u32 = 4u32;
pub const DAUD_CHAN13_VOICE_PRIORITY_OFFSET: u32 = 3u32;
pub const DAUD_CHAN14_VOICE_PRIORITY_OFFSET: u32 = 2u32;
pub const DAUD_CHAN15_VOICE_PRIORITY_OFFSET: u32 = 1u32;
pub const DAUD_CHAN16_VOICE_PRIORITY_OFFSET: u32 = 0u32;
pub const DAUD_CHAN1_VOICE_PRIORITY_OFFSET: u32 = 14u32;
pub const DAUD_CHAN2_VOICE_PRIORITY_OFFSET: u32 = 13u32;
pub const DAUD_CHAN3_VOICE_PRIORITY_OFFSET: u32 = 12u32;
pub const DAUD_CHAN4_VOICE_PRIORITY_OFFSET: u32 = 11u32;
pub const DAUD_CHAN5_VOICE_PRIORITY_OFFSET: u32 = 10u32;
pub const DAUD_CHAN6_VOICE_PRIORITY_OFFSET: u32 = 9u32;
pub const DAUD_CHAN7_VOICE_PRIORITY_OFFSET: u32 = 8u32;
pub const DAUD_CHAN8_VOICE_PRIORITY_OFFSET: u32 = 7u32;
pub const DAUD_CHAN9_VOICE_PRIORITY_OFFSET: u32 = 6u32;
pub const DAUD_CRITICAL_VOICE_PRIORITY: u32 = 4026531840u32;
pub const DAUD_HIGH_VOICE_PRIORITY: u32 = 3221225472u32;
pub const DAUD_LOW_VOICE_PRIORITY: u32 = 1073741824u32;
pub const DAUD_PERSIST_VOICE_PRIORITY: u32 = 268435456u32;
pub const DAUD_STANDARD_VOICE_PRIORITY: u32 = 2147483648u32;
pub type DIRECTSOUNDDEVICE_DATAFLOW = i32;
pub const DIRECTSOUNDDEVICE_DATAFLOW_RENDER: DIRECTSOUNDDEVICE_DATAFLOW = 0i32;
pub const DIRECTSOUNDDEVICE_DATAFLOW_CAPTURE: DIRECTSOUNDDEVICE_DATAFLOW = 1i32;
pub type DIRECTSOUNDDEVICE_TYPE = i32;
pub const DIRECTSOUNDDEVICE_TYPE_EMULATED: DIRECTSOUNDDEVICE_TYPE = 0i32;
pub const DIRECTSOUNDDEVICE_TYPE_VXD: DIRECTSOUNDDEVICE_TYPE = 1i32;
pub const DIRECTSOUNDDEVICE_TYPE_WDM: DIRECTSOUNDDEVICE_TYPE = 2i32;
#[repr(C)]
pub struct DLSHEADER {
    pub cInstruments: u32,
}
impl ::core::marker::Copy for DLSHEADER {}
impl ::core::clone::Clone for DLSHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DLSHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DLSHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DLSHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DLSHEADER {}
impl ::core::default::Default for DLSHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DLSID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DLSID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DLSID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DLSID {}
impl ::core::default::Default for DLSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DLSID_GMInHardware: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f24_c364_11d1_a760_0000f875ac12);
pub const DLSID_GSInHardware: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f25_c364_11d1_a760_0000f875ac12);
pub const DLSID_ManufacturersID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb03e1181_8095_11d2_a1ef_00600833dbd8);
pub const DLSID_ProductID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb03e1182_8095_11d2_a1ef_00600833dbd8);
pub const DLSID_SampleMemorySize: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const DLSID_SamplePlaybackRate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a91f713_a4bf_11d2_bbdf_00600833dbd8);
pub const DLSID_SupportsDLS1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f27_c364_11d1_a760_0000f875ac12);
pub const DLSID_SupportsDLS2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf14599e5_4689_11d2_afa6_00aa0024d8b6);
pub const DLSID_XGInHardware: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f26_c364_11d1_a760_0000f875ac12);
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DLSVERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DLSVERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DLSVERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DLSVERSION {}
impl ::core::default::Default for DLSVERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DLS_CDL_ADD: u32 = 4u32;
pub const DLS_CDL_AND: u32 = 1u32;
pub const DLS_CDL_CONST: u32 = 16u32;
pub const DLS_CDL_DIVIDE: u32 = 7u32;
pub const DLS_CDL_EQ: u32 = 14u32;
pub const DLS_CDL_GE: u32 = 13u32;
pub const DLS_CDL_GT: u32 = 12u32;
pub const DLS_CDL_LE: u32 = 11u32;
pub const DLS_CDL_LOGICAL_AND: u32 = 8u32;
pub const DLS_CDL_LOGICAL_OR: u32 = 9u32;
pub const DLS_CDL_LT: u32 = 10u32;
pub const DLS_CDL_MULTIPLY: u32 = 6u32;
pub const DLS_CDL_NOT: u32 = 15u32;
pub const DLS_CDL_OR: u32 = 2u32;
pub const DLS_CDL_QUERY: u32 = 17u32;
pub const DLS_CDL_QUERYSUPPORTED: u32 = 18u32;
pub const DLS_CDL_SUBTRACT: u32 = 5u32;
pub const DLS_CDL_XOR: u32 = 3u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_ARTICPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_ARTICPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_ARTICPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_ARTICPARAMS {}
impl ::core::default::Default for DMUS_ARTICPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_ARTICULATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_ARTICULATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_ARTICULATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_ARTICULATION {}
impl ::core::default::Default for DMUS_ARTICULATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_ARTICULATION2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_ARTICULATION2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_ARTICULATION2>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_ARTICULATION2 {}
impl ::core::default::Default for DMUS_ARTICULATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_BUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidBufferFormat: ::windows::core::GUID,
    pub cbBuffer: u32,
}
impl ::core::marker::Copy for DMUS_BUFFERDESC {}
impl ::core::clone::Clone for DMUS_BUFFERDESC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMUS_BUFFERDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_BUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_BUFFERDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_BUFFERDESC {}
impl ::core::default::Default for DMUS_BUFFERDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_CLOCKF_GLOBAL: u32 = 1u32;
#[repr(C)]
pub struct DMUS_CLOCKINFO7 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: ::windows::core::GUID,
    pub wszDescription: [u16; 128],
}
impl ::core::marker::Copy for DMUS_CLOCKINFO7 {}
impl ::core::clone::Clone for DMUS_CLOCKINFO7 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMUS_CLOCKINFO7 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_CLOCKINFO7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_CLOCKINFO7>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_CLOCKINFO7 {}
impl ::core::default::Default for DMUS_CLOCKINFO7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_CLOCKINFO8 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: ::windows::core::GUID,
    pub wszDescription: [u16; 128],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DMUS_CLOCKINFO8 {}
impl ::core::clone::Clone for DMUS_CLOCKINFO8 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMUS_CLOCKINFO8 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_CLOCKINFO8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_CLOCKINFO8>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_CLOCKINFO8 {}
impl ::core::default::Default for DMUS_CLOCKINFO8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DMUS_CLOCKTYPE = i32;
pub const DMUS_CLOCK_SYSTEM: DMUS_CLOCKTYPE = 0i32;
pub const DMUS_CLOCK_WAVE: DMUS_CLOCKTYPE = 1i32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_COPYRIGHT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_COPYRIGHT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_COPYRIGHT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_COPYRIGHT {}
impl ::core::default::Default for DMUS_COPYRIGHT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_DEFAULT_SIZE_OFFSETTABLE: u32 = 1u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_DOWNLOADINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_DOWNLOADINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_DOWNLOADINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_DOWNLOADINFO {}
impl ::core::default::Default for DMUS_DOWNLOADINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_DOWNLOADINFO_INSTRUMENT: u32 = 1u32;
pub const DMUS_DOWNLOADINFO_INSTRUMENT2: u32 = 3u32;
pub const DMUS_DOWNLOADINFO_ONESHOTWAVE: u32 = 6u32;
pub const DMUS_DOWNLOADINFO_STREAMINGWAVE: u32 = 5u32;
pub const DMUS_DOWNLOADINFO_WAVE: u32 = 2u32;
pub const DMUS_DOWNLOADINFO_WAVEARTICULATION: u32 = 4u32;
pub const DMUS_EFFECT_CHORUS: u32 = 2u32;
pub const DMUS_EFFECT_DELAY: u32 = 4u32;
pub const DMUS_EFFECT_NONE: u32 = 0u32;
pub const DMUS_EFFECT_REVERB: u32 = 1u32;
#[repr(C, packed(4))]
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
unsafe impl ::windows::core::Abi for DMUS_EVENTHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_EVENTHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_EVENTHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_EVENTHEADER {}
impl ::core::default::Default for DMUS_EVENTHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_EVENT_STRUCTURED: u32 = 1u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_EXTENSIONCHUNK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_EXTENSIONCHUNK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_EXTENSIONCHUNK>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_EXTENSIONCHUNK {}
impl ::core::default::Default for DMUS_EXTENSIONCHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_INSTRUMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_INSTRUMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_INSTRUMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_INSTRUMENT {}
impl ::core::default::Default for DMUS_INSTRUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_INSTRUMENT_GM_INSTRUMENT: u32 = 1u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_LFOPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_LFOPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_LFOPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_LFOPARAMS {}
impl ::core::default::Default for DMUS_LFOPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_MAX_DESCRIPTION: u32 = 128u32;
pub const DMUS_MAX_DRIVER: u32 = 128u32;
pub const DMUS_MIN_DATA_SIZE: u32 = 4u32;
#[repr(C)]
pub struct DMUS_MSCPARAMS {
    pub ptDefaultPan: i32,
}
impl ::core::marker::Copy for DMUS_MSCPARAMS {}
impl ::core::clone::Clone for DMUS_MSCPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMUS_MSCPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_MSCPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_MSCPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_MSCPARAMS {}
impl ::core::default::Default for DMUS_MSCPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_NOTERANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_NOTERANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_NOTERANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_NOTERANGE {}
impl ::core::default::Default for DMUS_NOTERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_OFFSETTABLE {
    pub ulOffsetTable: [u32; 1],
}
impl ::core::marker::Copy for DMUS_OFFSETTABLE {}
impl ::core::clone::Clone for DMUS_OFFSETTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMUS_OFFSETTABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_OFFSETTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_OFFSETTABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_OFFSETTABLE {}
impl ::core::default::Default for DMUS_OFFSETTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_PC_AUDIOPATH: u32 = 1024u32;
pub const DMUS_PC_DIRECTSOUND: u32 = 128u32;
pub const DMUS_PC_DLS: u32 = 1u32;
pub const DMUS_PC_DLS2: u32 = 512u32;
pub const DMUS_PC_EXTERNAL: u32 = 2u32;
pub const DMUS_PC_GMINHARDWARE: u32 = 16u32;
pub const DMUS_PC_GSINHARDWARE: u32 = 32u32;
pub const DMUS_PC_INPUTCLASS: u32 = 0u32;
pub const DMUS_PC_MEMORYSIZEFIXED: u32 = 8u32;
pub const DMUS_PC_OUTPUTCLASS: u32 = 1u32;
pub const DMUS_PC_SHAREABLE: u32 = 256u32;
pub const DMUS_PC_SOFTWARESYNTH: u32 = 4u32;
pub const DMUS_PC_SYSTEMMEMORY: u32 = 2147483647u32;
pub const DMUS_PC_WAVE: u32 = 2048u32;
pub const DMUS_PC_XGINHARDWARE: u32 = 64u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_PEGPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_PEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_PEGPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_PEGPARAMS {}
impl ::core::default::Default for DMUS_PEGPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_PORTCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidPort: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for DMUS_PORTCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_PORTCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_PORTCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_PORTCAPS {}
impl ::core::default::Default for DMUS_PORTCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DMUS_PORTPARAMS8 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMUS_PORTPARAMS8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_PORTPARAMS8>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMUS_PORTPARAMS8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMUS_PORTPARAMS8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_PORTPARAMS_AUDIOCHANNELS: u32 = 4u32;
pub const DMUS_PORTPARAMS_CHANNELGROUPS: u32 = 2u32;
pub const DMUS_PORTPARAMS_EFFECTS: u32 = 32u32;
pub const DMUS_PORTPARAMS_FEATURES: u32 = 128u32;
pub const DMUS_PORTPARAMS_SAMPLERATE: u32 = 8u32;
pub const DMUS_PORTPARAMS_SHARE: u32 = 64u32;
pub const DMUS_PORTPARAMS_VOICES: u32 = 1u32;
pub const DMUS_PORT_FEATURE_AUDIOPATH: u32 = 1u32;
pub const DMUS_PORT_FEATURE_STREAMING: u32 = 2u32;
pub const DMUS_PORT_KERNEL_MODE: u32 = 2u32;
pub const DMUS_PORT_USER_MODE_SYNTH: u32 = 1u32;
pub const DMUS_PORT_WINMM_DRIVER: u32 = 0u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_REGION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_REGION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_REGION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_REGION {}
impl ::core::default::Default for DMUS_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_SYNTHSTATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_SYNTHSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_SYNTHSTATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_SYNTHSTATS {}
impl ::core::default::Default for DMUS_SYNTHSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_SYNTHSTATS8 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_SYNTHSTATS8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_SYNTHSTATS8>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_SYNTHSTATS8 {}
impl ::core::default::Default for DMUS_SYNTHSTATS8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_SYNTHSTATS_CPU_PER_VOICE: u32 = 4u32;
pub const DMUS_SYNTHSTATS_FREE_MEMORY: u32 = 32u32;
pub const DMUS_SYNTHSTATS_LOST_NOTES: u32 = 8u32;
pub const DMUS_SYNTHSTATS_PEAK_VOLUME: u32 = 16u32;
pub const DMUS_SYNTHSTATS_SYSTEMMEMORY: u32 = 2147483647u32;
pub const DMUS_SYNTHSTATS_TOTAL_CPU: u32 = 2u32;
pub const DMUS_SYNTHSTATS_VOICES: u32 = 1u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_VEGPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_VEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_VEGPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_VEGPARAMS {}
impl ::core::default::Default for DMUS_VEGPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DMUS_VOICE_STATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMUS_VOICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_VOICE_STATE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMUS_VOICE_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMUS_VOICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DMUS_VOLUME_MAX: u32 = 2000u32;
pub const DMUS_VOLUME_MIN: i32 = -20000i32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_WAVE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_WAVE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_WAVE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_WAVE {}
impl ::core::default::Default for DMUS_WAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_WAVEARTDL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_WAVEARTDL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_WAVEARTDL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_WAVEARTDL {}
impl ::core::default::Default for DMUS_WAVEARTDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_WAVEDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_WAVEDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_WAVEDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_WAVEDATA {}
impl ::core::default::Default for DMUS_WAVEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DMUS_WAVEDL {
    pub cbWaveData: u32,
}
impl ::core::marker::Copy for DMUS_WAVEDL {}
impl ::core::clone::Clone for DMUS_WAVEDL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DMUS_WAVEDL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_WAVEDL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_WAVEDL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_WAVEDL {}
impl ::core::default::Default for DMUS_WAVEDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DMUS_WAVES_REVERB_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DMUS_WAVES_REVERB_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DMUS_WAVES_REVERB_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DMUS_WAVES_REVERB_PARAMS {}
impl ::core::default::Default for DMUS_WAVES_REVERB_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DSBUSID_BACK_CENTER: u32 = 8u32;
pub const DSBUSID_BACK_LEFT: u32 = 4u32;
pub const DSBUSID_BACK_RIGHT: u32 = 5u32;
pub const DSBUSID_CHORUS_SEND: u32 = 65u32;
pub const DSBUSID_DYNAMIC_0: u32 = 512u32;
pub const DSBUSID_FIRST_SPKR_LOC: u32 = 0u32;
pub const DSBUSID_FRONT_CENTER: u32 = 2u32;
pub const DSBUSID_FRONT_LEFT: u32 = 0u32;
pub const DSBUSID_FRONT_LEFT_OF_CENTER: u32 = 6u32;
pub const DSBUSID_FRONT_RIGHT: u32 = 1u32;
pub const DSBUSID_FRONT_RIGHT_OF_CENTER: u32 = 7u32;
pub const DSBUSID_LAST_SPKR_LOC: u32 = 17u32;
pub const DSBUSID_LEFT: u32 = 0u32;
pub const DSBUSID_LOW_FREQUENCY: u32 = 3u32;
pub const DSBUSID_NULL: u32 = 4294967295u32;
pub const DSBUSID_REVERB_SEND: u32 = 64u32;
pub const DSBUSID_RIGHT: u32 = 1u32;
pub const DSBUSID_SIDE_LEFT: u32 = 9u32;
pub const DSBUSID_SIDE_RIGHT: u32 = 10u32;
pub const DSBUSID_TOP_BACK_CENTER: u32 = 16u32;
pub const DSBUSID_TOP_BACK_LEFT: u32 = 15u32;
pub const DSBUSID_TOP_BACK_RIGHT: u32 = 17u32;
pub const DSBUSID_TOP_CENTER: u32 = 11u32;
pub const DSBUSID_TOP_FRONT_CENTER: u32 = 13u32;
pub const DSBUSID_TOP_FRONT_LEFT: u32 = 12u32;
pub const DSBUSID_TOP_FRONT_RIGHT: u32 = 14u32;
pub type DSPROPERTY_DIRECTSOUNDDEVICE = i32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A: DSPROPERTY_DIRECTSOUNDDEVICE = 1i32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1: DSPROPERTY_DIRECTSOUNDDEVICE = 2i32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1: DSPROPERTY_DIRECTSOUNDDEVICE = 3i32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W: DSPROPERTY_DIRECTSOUNDDEVICE = 4i32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A: DSPROPERTY_DIRECTSOUNDDEVICE = 5i32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W: DSPROPERTY_DIRECTSOUNDDEVICE = 6i32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A: DSPROPERTY_DIRECTSOUNDDEVICE = 7i32;
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W: DSPROPERTY_DIRECTSOUNDDEVICE = 8i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    pub DeviceId: ::windows::core::GUID,
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::core::GUID,
    pub Description: super::super::super::Foundation::PSTR,
    pub Module: super::super::super::Foundation::PSTR,
    pub Interface: super::super::super::Foundation::PSTR,
    pub WaveDeviceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::core::GUID,
    pub Description: super::super::super::Foundation::PWSTR,
    pub Module: super::super::super::Foundation::PWSTR,
    pub Interface: super::super::super::Foundation::PWSTR,
    pub WaveDeviceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    pub DeviceName: super::super::super::Foundation::PSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    pub DeviceName: super::super::super::Foundation::PWSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DSPROPSETID_DirectSoundDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84624f82_25ec_11d1_a4d8_00c04fc28aca);
pub const DV_AUDIOMODE: u32 = 3840u32;
pub const DV_AUDIOQU: u32 = 117440512u32;
pub const DV_AUDIOSMP: u32 = 939524096u32;
pub const DV_CAP_AUD12Bits: u32 = 1u32;
pub const DV_CAP_AUD16Bits: u32 = 0u32;
pub const DV_DVSD_NTSC_FRAMESIZE: i32 = 120000i32;
pub const DV_DVSD_PAL_FRAMESIZE: i32 = 144000i32;
pub const DV_HD: u32 = 1u32;
pub const DV_NTSC: u32 = 0u32;
pub const DV_NTSCPAL: u32 = 2097152u32;
pub const DV_PAL: u32 = 1u32;
pub const DV_SD: u32 = 0u32;
pub const DV_SL: u32 = 2u32;
pub const DV_SMCHN: u32 = 57344u32;
pub const DV_STYPE: u32 = 2031616u32;
pub const F_INSTRUMENT_DRUMS: u32 = 2147483648u32;
pub const F_RGN_OPTION_SELFNONEXCLUSIVE: u32 = 1u32;
pub const F_WAVELINK_MULTICHANNEL: u32 = 2u32;
pub const F_WAVELINK_PHASE_MASTER: u32 = 1u32;
pub const F_WSMP_NO_COMPRESSION: i32 = 2i32;
pub const F_WSMP_NO_TRUNCATION: i32 = 1i32;
pub const GUID_DMUS_PROP_DLS1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f27_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_DLS2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf14599e5_4689_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_Effects: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcda8d611_684a_11d2_871e_00600893b1bd);
pub const GUID_DMUS_PROP_GM_Hardware: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f24_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_GS_Capable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6496aba2_61b0_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_GS_Hardware: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f25_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_INSTRUMENT2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x865fd372_9f67_11d2_872a_00600893b1bd);
pub const GUID_DMUS_PROP_LegacyCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfa7cdc2_00a1_11d2_aad5_0000f875ac12);
pub const GUID_DMUS_PROP_MemorySize: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_SampleMemorySize: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f28_c364_11d1_a760_0000f875ac12);
pub const GUID_DMUS_PROP_SamplePlaybackRate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a91f713_a4bf_11d2_bbdf_00600833dbd8);
pub const GUID_DMUS_PROP_SetSynthSink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a3a5ba5_37b6_11d2_b9f9_0000f875ac12);
pub const GUID_DMUS_PROP_SinkUsesDSound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe208857_8952_11d2_ba1c_0000f875ac12);
pub const GUID_DMUS_PROP_SynthSink_DSOUND: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aa97844_c877_11d1_870c_00600893b1bd);
pub const GUID_DMUS_PROP_SynthSink_WAVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aa97845_c877_11d1_870c_00600893b1bd);
pub const GUID_DMUS_PROP_Volume: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfedfae25_e46e_11d1_aace_0000f875ac12);
pub const GUID_DMUS_PROP_WavesReverb: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04cb5622_32e5_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_WriteLatency: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x268a0fa0_60f2_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_WritePeriod: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x268a0fa1_60f2_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_XG_Capable: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6496aba1_61b0_11d2_afa6_00aa0024d8b6);
pub const GUID_DMUS_PROP_XG_Hardware: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x178f2f26_c364_11d1_a760_0000f875ac12);
#[repr(transparent)]
pub struct IDirectMusic(::windows::core::IUnknown);
impl IDirectMusic {
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pportcaps)).ok()
    }
    pub unsafe fn CreateMusicBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbufferdesc), ::core::mem::transmute(ppbuffer), punkouter.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePort<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsidport), ::core::mem::transmute(pportparams), ::core::mem::transmute(ppport), punkouter.into_param().abi()).ok()
    }
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpclockinfo)).ok()
    }
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidclock), ::core::mem::transmute(ppreferenceclock)).ok()
    }
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidclock)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidport)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::core::IntoParam<'a, super::DirectSound::IDirectSound>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, pdirectsound: Param0, hwnd: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), hwnd.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDirectMusic> for ::windows::core::IUnknown {
    fn from(value: IDirectMusic) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusic> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusic) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusic {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusic {}
unsafe impl ::windows::core::Interface for IDirectMusic {
    type Vtable = IDirectMusicVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6536115a_7b2d_11d2_ba18_0000f875ac12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidclock: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidport: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound")))] usize,
);
#[repr(transparent)]
pub struct IDirectMusic8(::windows::core::IUnknown);
impl IDirectMusic8 {
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pportcaps)).ok()
    }
    pub unsafe fn CreateMusicBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbufferdesc), ::core::mem::transmute(ppbuffer), punkouter.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePort<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsidport), ::core::mem::transmute(pportparams), ::core::mem::transmute(ppport), punkouter.into_param().abi()).ok()
    }
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(lpclockinfo)).ok()
    }
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidclock), ::core::mem::transmute(ppreferenceclock)).ok()
    }
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidclock)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidport)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::core::IntoParam<'a, super::DirectSound::IDirectSound>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, pdirectsound: Param0, hwnd: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), hwnd.into_param().abi()).ok()
    }
    pub unsafe fn SetExternalMasterClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::IReferenceClock>>(&self, pclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pclock.into_param().abi()).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IDirectMusic> for IDirectMusic8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectMusic> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectMusic> for &IDirectMusic8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectMusic> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectMusic8> for ::windows::core::IUnknown {
    fn from(value: IDirectMusic8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusic8> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusic8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusic8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusic8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusic8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusic8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusic8 {}
unsafe impl ::windows::core::Interface for IDirectMusic8 {
    type Vtable = IDirectMusic8Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d3629f7_813d_4939_8508_f05c6b75fd97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic8Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidclock: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidport: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDirectMusicBuffer(::windows::core::IUnknown);
impl IDirectMusicBuffer {
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn TotalTime(&self, prttime: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(prttime)).ok()
    }
    pub unsafe fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannelmessage)).ok()
    }
    pub unsafe fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(cb), ::core::mem::transmute(lpb)).ok()
    }
    pub unsafe fn ResetReadPtr(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(prt), ::core::mem::transmute(pdwchannelgroup), ::core::mem::transmute(pdwlength), ::core::mem::transmute(ppdata)).ok()
    }
    pub unsafe fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdata)).ok()
    }
    pub unsafe fn GetStartTime(&self, prt: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(prt)).ok()
    }
    pub unsafe fn GetUsedBytes(&self, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcb)).ok()
    }
    pub unsafe fn GetMaxBytes(&self, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcb)).ok()
    }
    pub unsafe fn GetBufferFormat(&self, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidformat)).ok()
    }
    pub unsafe fn SetStartTime(&self, rt: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt)).ok()
    }
    pub unsafe fn SetUsedBytes(&self, cb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(cb)).ok()
    }
}
impl ::core::convert::From<IDirectMusicBuffer> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicBuffer> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicBuffer {}
unsafe impl ::windows::core::Interface for IDirectMusicBuffer {
    type Vtable = IDirectMusicBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac2878_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prttime: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDirectMusicCollection(::windows::core::IUnknown);
impl IDirectMusicCollection {
    pub unsafe fn GetInstrument(&self, dwpatch: u32) -> ::windows::core::Result<IDirectMusicInstrument> {
        let mut result__: <IDirectMusicInstrument as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpatch), &mut result__).from_abi::<IDirectMusicInstrument>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumInstrument<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: Param2, dwnamelen: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pdwpatch), pwszname.into_param().abi(), ::core::mem::transmute(dwnamelen)).ok()
    }
}
impl ::core::convert::From<IDirectMusicCollection> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicCollection> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicCollection {}
unsafe impl ::windows::core::Interface for IDirectMusicCollection {
    type Vtable = IDirectMusicCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287c_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpatch: u32, ppinstrument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pdwpatch: *mut u32, pwszname: super::super::super::Foundation::PWSTR, dwnamelen: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct IDirectMusicDownload(::windows::core::IUnknown);
impl IDirectMusicDownload {
    pub unsafe fn GetBuffer(&self, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvbuffer), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IDirectMusicDownload> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicDownload) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicDownload> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicDownload) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicDownload {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicDownload {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicDownload {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicDownload {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicDownload {}
unsafe impl ::windows::core::Interface for IDirectMusicDownload {
    type Vtable = IDirectMusicDownloadVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287b_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownloadVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IDirectMusicDownloadedInstrument(::windows::core::IUnknown);
impl IDirectMusicDownloadedInstrument {}
impl ::core::convert::From<IDirectMusicDownloadedInstrument> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicDownloadedInstrument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicDownloadedInstrument> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicDownloadedInstrument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicDownloadedInstrument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicDownloadedInstrument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicDownloadedInstrument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicDownloadedInstrument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicDownloadedInstrument {}
unsafe impl ::windows::core::Interface for IDirectMusicDownloadedInstrument {
    type Vtable = IDirectMusicDownloadedInstrumentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287e_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownloadedInstrumentVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32);
#[repr(transparent)]
pub struct IDirectMusicInstrument(::windows::core::IUnknown);
impl IDirectMusicInstrument {
    pub unsafe fn GetPatch(&self, pdwpatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpatch)).ok()
    }
    pub unsafe fn SetPatch(&self, dwpatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpatch)).ok()
    }
}
impl ::core::convert::From<IDirectMusicInstrument> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicInstrument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicInstrument> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicInstrument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicInstrument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicInstrument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicInstrument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicInstrument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicInstrument {}
unsafe impl ::windows::core::Interface for IDirectMusicInstrument {
    type Vtable = IDirectMusicInstrumentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287d_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicInstrumentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpatch: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpatch: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDirectMusicPort(::windows::core::IUnknown);
impl IDirectMusicPort {
    pub unsafe fn PlayBuffer<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicBuffer>>(&self, pbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadNotificationHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
    pub unsafe fn Read<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicBuffer>>(&self, pbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    pub unsafe fn DownloadInstrument<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicInstrument>>(&self, pinstrument: Param0, ppdownloadedinstrument: *mut ::core::option::Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pinstrument.into_param().abi(), ::core::mem::transmute(ppdownloadedinstrument), ::core::mem::transmute(pnoteranges), ::core::mem::transmute(dwnumnoteranges)).ok()
    }
    pub unsafe fn UnloadInstrument<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicDownloadedInstrument>>(&self, pdownloadedinstrument: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdownloadedinstrument.into_param().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__: <super::super::IReferenceClock as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::IReferenceClock>(result__)
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstats)).ok()
    }
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pportcaps)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwiocontrolcode), ::core::mem::transmute(lpinbuffer), ::core::mem::transmute(ninbuffersize), ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(noutbuffersize), ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpoverlapped)).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroups)).ok()
    }
    pub unsafe fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwchannelgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, factive: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), factive.into_param().abi()).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(dwpriority)).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(pdwpriority)).ok()
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::core::IntoParam<'a, super::DirectSound::IDirectSound>, Param1: ::windows::core::IntoParam<'a, super::DirectSound::IDirectSoundBuffer>>(&self, pdirectsound: Param0, pdirectsoundbuffer: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), pdirectsoundbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize), ::core::mem::transmute(pdwbuffersize)).ok()
    }
}
impl ::core::convert::From<IDirectMusicPort> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicPort> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicPort {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicPort {}
unsafe impl ::windows::core::Interface for IDirectMusicPort {
    type Vtable = IDirectMusicPortVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f2d8c9_37c2_11d2_b9f9_0000f875ac12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPortVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstrument: ::windows::core::RawPtr, ppdownloadedinstrument: *mut ::windows::core::RawPtr, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdownloadedinstrument: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroups: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwchannelgroups: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factive: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pdirectsoundbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDirectMusicPortDownload(::windows::core::IUnknown);
impl IDirectMusicPortDownload {
    pub unsafe fn GetBuffer(&self, dwdlid: u32) -> ::windows::core::Result<IDirectMusicDownload> {
        let mut result__: <IDirectMusicDownload as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdlid), &mut result__).from_abi::<IDirectMusicDownload>(result__)
    }
    pub unsafe fn AllocateBuffer(&self, dwsize: u32) -> ::windows::core::Result<IDirectMusicDownload> {
        let mut result__: <IDirectMusicDownload as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsize), &mut result__).from_abi::<IDirectMusicDownload>(result__)
    }
    pub unsafe fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwstartdlid), ::core::mem::transmute(dwcount)).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwappend)).ok()
    }
    pub unsafe fn Download<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicDownload>>(&self, pidmdownload: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pidmdownload.into_param().abi()).ok()
    }
    pub unsafe fn Unload<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicDownload>>(&self, pidmdownload: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pidmdownload.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDirectMusicPortDownload> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicPortDownload) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicPortDownload> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicPortDownload) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicPortDownload {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicPortDownload {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicPortDownload {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicPortDownload {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicPortDownload {}
unsafe impl ::windows::core::Interface for IDirectMusicPortDownload {
    type Vtable = IDirectMusicPortDownloadVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287a_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPortDownloadVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdlid: u32, ppidmdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsize: u32, ppidmdownload: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidmdownload: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidmdownload: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDirectMusicSynth(::windows::core::IUnknown);
impl IDirectMusicSynth {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pportparams)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(phdownload), ::core::mem::transmute(pvdata), ::core::mem::transmute(pbfree)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unload<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hdownload: Param0, lpfreehandle: isize, huserdata: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), hdownload.into_param().abi(), ::core::mem::transmute(lpfreehandle), huserdata.into_param().abi()).ok()
    }
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(cbbuffer)).ok()
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstats)).ok()
    }
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcaps)).ok()
    }
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::IReferenceClock>>(&self, pclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__: <super::super::IReferenceClock as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn SetSynthSink<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicSynthSink>>(&self, psynthsink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psynthsink.into_param().abi()).ok()
    }
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwlength), ::core::mem::transmute(llposition)).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(dwpriority)).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(pdwpriority)).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize)).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwappend)).ok()
    }
}
impl ::core::convert::From<IDirectMusicSynth> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicSynth) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicSynth> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicSynth) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicSynth {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicSynth {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicSynth {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicSynth {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicSynth {}
unsafe impl ::windows::core::Interface for IDirectMusicSynth {
    type Vtable = IDirectMusicSynthVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09823661_5c85_11d2_afa6_00aa0024d8b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynthVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwgroups: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psynthsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDirectMusicSynth8(::windows::core::IUnknown);
impl IDirectMusicSynth8 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pportparams)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(phdownload), ::core::mem::transmute(pvdata), ::core::mem::transmute(pbfree)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unload<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hdownload: Param0, lpfreehandle: isize, huserdata: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), hdownload.into_param().abi(), ::core::mem::transmute(lpfreehandle), huserdata.into_param().abi()).ok()
    }
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(cbbuffer)).ok()
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstats)).ok()
    }
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcaps)).ok()
    }
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::IReferenceClock>>(&self, pclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__: <super::super::IReferenceClock as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn SetSynthSink<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicSynthSink>>(&self, psynthsink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), psynthsink.into_param().abi()).ok()
    }
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwlength), ::core::mem::transmute(llposition)).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(dwpriority)).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(pdwpriority)).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize)).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwappend)).ok()
    }
    pub unsafe fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(dwvoiceid), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(dwdlid), ::core::mem::transmute(prpitch), ::core::mem::transmute(vrvolume), ::core::mem::transmute(stvoicestart), ::core::mem::transmute(stloopstart), ::core::mem::transmute(stloopend)).ok()
    }
    pub unsafe fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(rt), ::core::mem::transmute(dwvoiceid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwvoice), ::core::mem::transmute(cbvoice), ::core::mem::transmute(dwvoicestate)).ok()
    }
    pub unsafe fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdownloadid), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwchannelgroup), ::core::mem::transmute(dwchannel), ::core::mem::transmute(pdwbuses), ::core::mem::transmute(cbuses)).ok()
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
impl<'a> ::windows::core::IntoParam<'a, IDirectMusicSynth> for IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectMusicSynth> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectMusicSynth> for &IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectMusicSynth> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectMusicSynth8> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicSynth8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicSynth8> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicSynth8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicSynth8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicSynth8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicSynth8 {}
unsafe impl ::windows::core::Interface for IDirectMusicSynth8 {
    type Vtable = IDirectMusicSynth8Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53cab625_2711_4c9f_9de7_1b7f925f6fc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth8Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwgroups: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psynthsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdownloadid: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDirectMusicSynthSink(::windows::core::IUnknown);
impl IDirectMusicSynthSink {
    pub unsafe fn Init<'a, Param0: ::windows::core::IntoParam<'a, IDirectMusicSynth>>(&self, psynth: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psynth.into_param().abi()).ok()
    }
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::IReferenceClock>>(&self, pclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__: <super::super::IReferenceClock as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    pub unsafe fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(llsampletime), ::core::mem::transmute(prftime)).ok()
    }
    pub unsafe fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rftime), ::core::mem::transmute(pllsampletime)).ok()
    }
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::core::IntoParam<'a, super::DirectSound::IDirectSound>, Param1: ::windows::core::IntoParam<'a, super::DirectSound::IDirectSoundBuffer>>(&self, pdirectsound: Param0, pdirectsoundbuffer: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), pdirectsoundbuffer.into_param().abi()).ok()
    }
    pub unsafe fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbuffersizeinsamples)).ok()
    }
}
impl ::core::convert::From<IDirectMusicSynthSink> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicSynthSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicSynthSink> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicSynthSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicSynthSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicSynthSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicSynthSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicSynthSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicSynthSink {}
unsafe impl ::windows::core::Interface for IDirectMusicSynthSink {
    type Vtable = IDirectMusicSynthSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09823663_5c85_11d2_afa6_00aa0024d8b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynthSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psynth: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclock: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclock: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llsampletime: i64, prftime: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pdirectsoundbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDirectMusicThru(::windows::core::IUnknown);
impl IDirectMusicThru {
    pub unsafe fn ThruChannel<'a, Param4: ::windows::core::IntoParam<'a, IDirectMusicPort>>(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsourcechannelgroup), ::core::mem::transmute(dwsourcechannel), ::core::mem::transmute(dwdestinationchannelgroup), ::core::mem::transmute(dwdestinationchannel), pdestinationport.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDirectMusicThru> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicThru) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicThru> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicThru) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectMusicThru {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirectMusicThru {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectMusicThru {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectMusicThru {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectMusicThru {}
unsafe impl ::windows::core::Interface for IDirectMusicThru {
    type Vtable = IDirectMusicThruVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xced153e7_3606_11d2_b9f9_0000f875ac12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicThruVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
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
unsafe impl ::windows::core::Abi for INSTHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTHEADER {}
impl ::core::default::Default for INSTHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1 = ::core::option::Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA, param1: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
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
unsafe impl ::windows::core::Abi for MDEVICECAPSEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MDEVICECAPSEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MDEVICECAPSEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for MDEVICECAPSEX {}
impl ::core::default::Default for MDEVICECAPSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for MIDILOCALE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDILOCALE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDILOCALE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDILOCALE {}
impl ::core::default::Default for MIDILOCALE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
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
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::core::Abi for MIDIOPENDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::cmp::PartialEq for MIDIOPENDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIOPENDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::cmp::Eq for MIDIOPENDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::core::default::Default for MIDIOPENDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct POOLCUE {
    pub ulOffset: u32,
}
impl ::core::marker::Copy for POOLCUE {}
impl ::core::clone::Clone for POOLCUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for POOLCUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POOLCUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POOLCUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for POOLCUE {}
impl ::core::default::Default for POOLCUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for POOLTABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POOLTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POOLTABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for POOLTABLE {}
impl ::core::default::Default for POOLTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const POOL_CUE_NULL: i32 = -1i32;
pub const REFRESH_F_LASTBUFFER: u32 = 1u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for RGNHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RGNHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RGNHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for RGNHEADER {}
impl ::core::default::Default for RGNHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for RGNRANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RGNRANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RGNRANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RGNRANGE {}
impl ::core::default::Default for RGNRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SIZE_DVINFO: u32 = 32u32;
#[repr(C)]
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
unsafe impl ::windows::core::Abi for Tag_DVAudInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Tag_DVAudInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Tag_DVAudInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for Tag_DVAudInfo {}
impl ::core::default::Default for Tag_DVAudInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WAVELINK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVELINK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVELINK>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVELINK {}
impl ::core::default::Default for WAVELINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WAVELINK_CHANNEL_LEFT: i32 = 1i32;
pub const WAVELINK_CHANNEL_RIGHT: i32 = 2i32;
pub const WLOOP_TYPE_FORWARD: u32 = 0u32;
pub const WLOOP_TYPE_RELEASE: u32 = 2u32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _DMUS_PORTPARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _DMUS_PORTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_DMUS_PORTPARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _DMUS_PORTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _DMUS_PORTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for _rloop {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _rloop {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_rloop>()) == 0 }
    }
}
impl ::core::cmp::Eq for _rloop {}
impl ::core::default::Default for _rloop {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
unsafe impl ::windows::core::Abi for _rwsmp {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _rwsmp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_rwsmp>()) == 0 }
    }
}
impl ::core::cmp::Eq for _rwsmp {}
impl ::core::default::Default for _rwsmp {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
