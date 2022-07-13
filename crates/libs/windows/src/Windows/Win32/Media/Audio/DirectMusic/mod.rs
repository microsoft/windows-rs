pub const CLSID_DirectMusic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x636b9f10_0c7d_11d1_95b2_0020afdc7421);
pub const CLSID_DirectMusicCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x480ff4b0_28b2_11d1_bef7_00c04fbf8fef);
pub const CLSID_DirectMusicSynth: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58c2b4d0_46e7_11d1_89ac_00a0c9054129);
pub const CLSID_DirectMusicSynthSink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec17ce3_a514_11d1_afa6_00aa0024d8b6);
pub const CLSID_DirectSoundPrivate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11ab3ec0_25ec_11d1_a4d8_00c04fc28aca);
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
impl ::core::fmt::Debug for CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTION").field("usSource", &self.usSource).field("usControl", &self.usControl).field("usDestination", &self.usDestination).field("usTransform", &self.usTransform).field("lScale", &self.lScale).finish()
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
impl ::core::fmt::Debug for CONNECTIONLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTIONLIST").field("cbSize", &self.cbSize).field("cConnections", &self.cConnections).finish()
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTSOUNDDEVICE_DATAFLOW(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_DATAFLOW_RENDER: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_DATAFLOW_CAPTURE: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(1i32);
impl ::core::marker::Copy for DIRECTSOUNDDEVICE_DATAFLOW {}
impl ::core::clone::Clone for DIRECTSOUNDDEVICE_DATAFLOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTSOUNDDEVICE_DATAFLOW {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTSOUNDDEVICE_DATAFLOW {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTSOUNDDEVICE_DATAFLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTSOUNDDEVICE_DATAFLOW").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIRECTSOUNDDEVICE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_TYPE_EMULATED: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_TYPE_VXD: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DIRECTSOUNDDEVICE_TYPE_WDM: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(2i32);
impl ::core::marker::Copy for DIRECTSOUNDDEVICE_TYPE {}
impl ::core::clone::Clone for DIRECTSOUNDDEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIRECTSOUNDDEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIRECTSOUNDDEVICE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIRECTSOUNDDEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTSOUNDDEVICE_TYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for DLSHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DLSHEADER").field("cInstruments", &self.cInstruments).finish()
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
impl ::core::fmt::Debug for DLSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DLSID").field("ulData1", &self.ulData1).field("usData2", &self.usData2).field("usData3", &self.usData3).field("abData4", &self.abData4).finish()
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
impl ::core::fmt::Debug for DLSVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DLSVERSION").field("dwVersionMS", &self.dwVersionMS).field("dwVersionLS", &self.dwVersionLS).finish()
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
impl ::core::fmt::Debug for DMUS_ARTICPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_ARTICPARAMS").field("LFO", &self.LFO).field("VolEG", &self.VolEG).field("PitchEG", &self.PitchEG).field("Misc", &self.Misc).finish()
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
impl ::core::fmt::Debug for DMUS_ARTICULATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_ARTICULATION").field("ulArt1Idx", &self.ulArt1Idx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).finish()
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
impl ::core::fmt::Debug for DMUS_ARTICULATION2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_ARTICULATION2").field("ulArtIdx", &self.ulArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulNextArtIdx", &self.ulNextArtIdx).finish()
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
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
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
impl ::core::fmt::Debug for DMUS_BUFFERDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_BUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidBufferFormat", &self.guidBufferFormat).field("cbBuffer", &self.cbBuffer).finish()
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
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_CLOCKF_GLOBAL: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
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
impl ::core::fmt::Debug for DMUS_CLOCKINFO7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_CLOCKINFO7").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).finish()
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
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
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
impl ::core::fmt::Debug for DMUS_CLOCKINFO8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_CLOCKINFO8").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).field("dwFlags", &self.dwFlags).finish()
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
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DMUS_CLOCKTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_CLOCK_SYSTEM: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DMUS_CLOCK_WAVE: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(1i32);
impl ::core::marker::Copy for DMUS_CLOCKTYPE {}
impl ::core::clone::Clone for DMUS_CLOCKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DMUS_CLOCKTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DMUS_CLOCKTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DMUS_CLOCKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMUS_CLOCKTYPE").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for DMUS_COPYRIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_COPYRIGHT").field("cbSize", &self.cbSize).field("byCopyright", &self.byCopyright).finish()
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
impl ::core::fmt::Debug for DMUS_DOWNLOADINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_DOWNLOADINFO").field("dwDLType", &self.dwDLType).field("dwDLId", &self.dwDLId).field("dwNumOffsetTableEntries", &self.dwNumOffsetTableEntries).field("cbSize", &self.cbSize).finish()
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
impl ::core::fmt::Debug for DMUS_EXTENSIONCHUNK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_EXTENSIONCHUNK").field("cbSize", &self.cbSize).field("ulNextExtCkIdx", &self.ulNextExtCkIdx).field("ExtCkID", &self.ExtCkID).field("byExtCk", &self.byExtCk).finish()
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
impl ::core::fmt::Debug for DMUS_INSTRUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_INSTRUMENT").field("ulPatch", &self.ulPatch).field("ulFirstRegionIdx", &self.ulFirstRegionIdx).field("ulGlobalArtIdx", &self.ulGlobalArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulCopyrightIdx", &self.ulCopyrightIdx).field("ulFlags", &self.ulFlags).finish()
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
impl ::core::fmt::Debug for DMUS_LFOPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_LFOPARAMS").field("pcFrequency", &self.pcFrequency).field("tcDelay", &self.tcDelay).field("gcVolumeScale", &self.gcVolumeScale).field("pcPitchScale", &self.pcPitchScale).field("gcMWToVolume", &self.gcMWToVolume).field("pcMWToPitch", &self.pcMWToPitch).finish()
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
impl ::core::fmt::Debug for DMUS_MSCPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_MSCPARAMS").field("ptDefaultPan", &self.ptDefaultPan).finish()
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
impl ::core::fmt::Debug for DMUS_NOTERANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_NOTERANGE").field("dwLowNote", &self.dwLowNote).field("dwHighNote", &self.dwHighNote).finish()
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
impl ::core::fmt::Debug for DMUS_OFFSETTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_OFFSETTABLE").field("ulOffsetTable", &self.ulOffsetTable).finish()
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
impl ::core::fmt::Debug for DMUS_PEGPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_PEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).field("pcRange", &self.pcRange).finish()
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
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
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
impl ::core::fmt::Debug for DMUS_PORTCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_PORTCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidPort", &self.guidPort).field("dwClass", &self.dwClass).field("dwType", &self.dwType).field("dwMemorySize", &self.dwMemorySize).field("dwMaxChannelGroups", &self.dwMaxChannelGroups).field("dwMaxVoices", &self.dwMaxVoices).field("dwMaxAudioChannels", &self.dwMaxAudioChannels).field("dwEffectFlags", &self.dwEffectFlags).field("wszDescription", &self.wszDescription).finish()
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMUS_PORTPARAMS8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_PORTPARAMS8").field("dwSize", &self.dwSize).field("dwValidParams", &self.dwValidParams).field("dwVoices", &self.dwVoices).field("dwChannelGroups", &self.dwChannelGroups).field("dwAudioChannels", &self.dwAudioChannels).field("dwSampleRate", &self.dwSampleRate).field("dwEffectFlags", &self.dwEffectFlags).field("fShare", &self.fShare).field("dwFeatures", &self.dwFeatures).finish()
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
impl ::core::fmt::Debug for DMUS_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_REGION").field("RangeKey", &self.RangeKey).field("RangeVelocity", &self.RangeVelocity).field("fusOptions", &self.fusOptions).field("usKeyGroup", &self.usKeyGroup).field("ulRegionArtIdx", &self.ulRegionArtIdx).field("ulNextRegionIdx", &self.ulNextRegionIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("WaveLink", &self.WaveLink).field("WSMP", &self.WSMP).field("WLOOP", &self.WLOOP).finish()
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
impl ::core::fmt::Debug for DMUS_SYNTHSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_SYNTHSTATS").field("dwSize", &self.dwSize).field("dwValidStats", &self.dwValidStats).field("dwVoices", &self.dwVoices).field("dwTotalCPU", &self.dwTotalCPU).field("dwCPUPerVoice", &self.dwCPUPerVoice).field("dwLostNotes", &self.dwLostNotes).field("dwFreeMemory", &self.dwFreeMemory).field("lPeakVolume", &self.lPeakVolume).finish()
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
impl ::core::fmt::Debug for DMUS_SYNTHSTATS8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_SYNTHSTATS8").field("dwSize", &self.dwSize).field("dwValidStats", &self.dwValidStats).field("dwVoices", &self.dwVoices).field("dwTotalCPU", &self.dwTotalCPU).field("dwCPUPerVoice", &self.dwCPUPerVoice).field("dwLostNotes", &self.dwLostNotes).field("dwFreeMemory", &self.dwFreeMemory).field("lPeakVolume", &self.lPeakVolume).field("dwSynthMemUse", &self.dwSynthMemUse).finish()
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
impl ::core::fmt::Debug for DMUS_VEGPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_VEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).finish()
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMUS_VOICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_VOICE_STATE").field("bExists", &self.bExists).field("spPosition", &self.spPosition).finish()
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
impl ::core::fmt::Debug for DMUS_WAVEARTDL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_WAVEARTDL").field("ulDownloadIdIdx", &self.ulDownloadIdIdx).field("ulBus", &self.ulBus).field("ulBuffers", &self.ulBuffers).field("ulMasterDLId", &self.ulMasterDLId).field("usOptions", &self.usOptions).finish()
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
impl ::core::fmt::Debug for DMUS_WAVEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_WAVEDATA").field("cbSize", &self.cbSize).field("byData", &self.byData).finish()
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
impl ::core::fmt::Debug for DMUS_WAVEDL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_WAVEDL").field("cbWaveData", &self.cbWaveData).finish()
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
impl ::core::fmt::Debug for DMUS_WAVES_REVERB_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMUS_WAVES_REVERB_PARAMS").field("fInGain", &self.fInGain).field("fReverbMix", &self.fReverbMix).field("fReverbTime", &self.fReverbTime).field("fHighFreqRTRatio", &self.fHighFreqRTRatio).finish()
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(5i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(6i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(7i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(8i32);
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSPROPERTY_DIRECTSOUNDDEVICE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA").field("DeviceId", &self.DeviceId).field("DescriptionA", &self.DescriptionA).field("DescriptionW", &self.DescriptionW).field("ModuleA", &self.ModuleA).field("ModuleW", &self.ModuleW).field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("WaveDeviceId", &self.WaveDeviceId).field("Devnode", &self.Devnode).finish()
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
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::core::GUID,
    pub Description: ::windows::core::PSTR,
    pub Module: ::windows::core::PSTR,
    pub Interface: ::windows::core::PSTR,
    pub WaveDeviceId: u32,
}
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA").field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).field("Description", &self.Description).field("Module", &self.Module).field("Interface", &self.Interface).field("WaveDeviceId", &self.WaveDeviceId).finish()
    }
}
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::core::GUID,
    pub Description: ::windows::core::PWSTR,
    pub Module: ::windows::core::PWSTR,
    pub Interface: ::windows::core::PWSTR,
    pub WaveDeviceId: u32,
}
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA").field("Type", &self.Type).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).field("Description", &self.Description).field("Module", &self.Module).field("Interface", &self.Interface).field("WaveDeviceId", &self.WaveDeviceId).finish()
    }
}
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA").field("Callback", &self.Callback.map(|f| f as usize)).field("Context", &self.Context).finish()
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA").field("Callback", &self.Callback.map(|f| f as usize)).field("Context", &self.Context).finish()
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA").field("Callback", &self.Callback.map(|f| f as usize)).field("Context", &self.Context).finish()
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
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    pub DeviceName: ::windows::core::PSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::core::GUID,
}
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    pub DeviceName: ::windows::core::PWSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::core::GUID,
}
impl ::core::marker::Copy for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
impl ::core::clone::Clone for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
unsafe impl ::windows::core::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
impl ::core::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DSPROPSETID_DirectSoundDevice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84624f82_25ec_11d1_a4d8_00c04fc28aca);
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
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusic(::windows::core::IUnknown);
impl IDirectMusic {
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumPort)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pportcaps)).ok()
    }
    pub unsafe fn CreateMusicBuffer<'a, P0>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).CreateMusicBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbufferdesc), ::core::mem::transmute(ppbuffer), punkouter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePort<'a, P0>(&self, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).CreatePort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rclsidport), ::core::mem::transmute(pportparams), ::core::mem::transmute(ppport), punkouter.into().abi()).ok()
    }
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumMasterClock)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(lpclockinfo)).ok()
    }
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMasterClock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidclock), ::core::mem::transmute(ppreferenceclock)).ok()
    }
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMasterClock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rguidclock)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDefaultPort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio_DirectSound\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
    pub unsafe fn SetDirectSound<'a, P0, P1>(&self, pdirectsound: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DirectSound::IDirectSound>>,
        P1: ::std::convert::Into<super::super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetDirectSound)(::windows::core::Interface::as_raw(self), pdirectsound.into().abi(), hwnd.into()).ok()
    }
}
impl ::core::convert::From<IDirectMusic> for ::windows::core::IUnknown {
    fn from(value: IDirectMusic) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusic> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusic) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusic> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusic) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusic").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusic {
    type Vtable = IDirectMusic_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6536115a_7b2d_11d2_ba18_0000f875ac12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub EnumPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT,
    pub CreateMusicBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePort: usize,
    pub EnumMasterClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::HRESULT,
    pub GetMasterClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidclock: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    pub GetDefaultPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidport: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
    pub SetDirectSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound")))]
    SetDirectSound: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusic8(::windows::core::IUnknown);
impl IDirectMusic8 {
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumPort)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pportcaps)).ok()
    }
    pub unsafe fn CreateMusicBuffer<'a, P0>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.CreateMusicBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbufferdesc), ::core::mem::transmute(ppbuffer), punkouter.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePort<'a, P0>(&self, rclsidport: *const ::windows::core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).base__.CreatePort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rclsidport), ::core::mem::transmute(pportparams), ::core::mem::transmute(ppport), punkouter.into().abi()).ok()
    }
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumMasterClock)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(lpclockinfo)).ok()
    }
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::core::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMasterClock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidclock), ::core::mem::transmute(ppreferenceclock)).ok()
    }
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMasterClock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rguidclock)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Activate)(::windows::core::Interface::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDefaultPort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidport)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio_DirectSound\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
    pub unsafe fn SetDirectSound<'a, P0, P1>(&self, pdirectsound: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DirectSound::IDirectSound>>,
        P1: ::std::convert::Into<super::super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDirectSound)(::windows::core::Interface::as_raw(self), pdirectsound.into().abi(), hwnd.into()).ok()
    }
    pub unsafe fn SetExternalMasterClock<'a, P0>(&self, pclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::IReferenceClock>>,
    {
        (::windows::core::Interface::vtable(self).SetExternalMasterClock)(::windows::core::Interface::as_raw(self), pclock.into().abi()).ok()
    }
}
impl ::core::convert::From<IDirectMusic8> for ::windows::core::IUnknown {
    fn from(value: IDirectMusic8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusic8> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusic8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusic8> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusic8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDirectMusic8> for IDirectMusic {
    fn from(value: IDirectMusic8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusic8> for &'a IDirectMusic {
    fn from(value: &'a IDirectMusic8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusic8> for IDirectMusic {
    fn from(value: &IDirectMusic8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusic8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusic8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusic8 {
    type Vtable = IDirectMusic8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d3629f7_813d_4939_8508_f05c6b75fd97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic8_Vtbl {
    pub base__: IDirectMusic_Vtbl,
    pub SetExternalMasterClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclock: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicBuffer(::windows::core::IUnknown);
impl IDirectMusicBuffer {
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TotalTime(&self, prttime: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TotalTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prttime)).ok()
    }
    pub unsafe fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PackStructured)(::windows::core::Interface::as_raw(self), rt, dwchannelgroup, dwchannelmessage).ok()
    }
    pub unsafe fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PackUnstructured)(::windows::core::Interface::as_raw(self), rt, dwchannelgroup, cb, ::core::mem::transmute(lpb)).ok()
    }
    pub unsafe fn ResetReadPtr(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetReadPtr)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextEvent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prt), ::core::mem::transmute(pdwchannelgroup), ::core::mem::transmute(pdwlength), ::core::mem::transmute(ppdata)).ok()
    }
    pub unsafe fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRawBufferPtr)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppdata)).ok()
    }
    pub unsafe fn GetStartTime(&self, prt: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStartTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prt)).ok()
    }
    pub unsafe fn GetUsedBytes(&self, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUsedBytes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcb)).ok()
    }
    pub unsafe fn GetMaxBytes(&self, pcb: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMaxBytes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcb)).ok()
    }
    pub unsafe fn GetBufferFormat(&self, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidformat)).ok()
    }
    pub unsafe fn SetStartTime(&self, rt: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartTime)(::windows::core::Interface::as_raw(self), rt).ok()
    }
    pub unsafe fn SetUsedBytes(&self, cb: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUsedBytes)(::windows::core::Interface::as_raw(self), cb).ok()
    }
}
impl ::core::convert::From<IDirectMusicBuffer> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicBuffer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicBuffer> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicBuffer {
    type Vtable = IDirectMusicBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac2878_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicBuffer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prttime: *mut i64) -> ::windows::core::HRESULT,
    pub PackStructured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::core::HRESULT,
    pub PackUnstructured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::core::HRESULT,
    pub ResetReadPtr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNextEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetRawBufferPtr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows::core::HRESULT,
    pub GetUsedBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaxBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT,
    pub GetBufferFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64) -> ::windows::core::HRESULT,
    pub SetUsedBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicCollection(::windows::core::IUnknown);
impl IDirectMusicCollection {
    pub unsafe fn GetInstrument(&self, dwpatch: u32) -> ::windows::core::Result<IDirectMusicInstrument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInstrument)(::windows::core::Interface::as_raw(self), dwpatch, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDirectMusicInstrument>(result__)
    }
    pub unsafe fn EnumInstrument<'a, P0>(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: P0, dwnamelen: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EnumInstrument)(::windows::core::Interface::as_raw(self), dwindex, ::core::mem::transmute(pdwpatch), pwszname.into(), dwnamelen).ok()
    }
}
impl ::core::convert::From<IDirectMusicCollection> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicCollection> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicCollection {
    type Vtable = IDirectMusicCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287c_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicCollection_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetInstrument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpatch: u32, ppinstrument: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumInstrument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pdwpatch: *mut u32, pwszname: ::windows::core::PCWSTR, dwnamelen: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicDownload(::windows::core::IUnknown);
impl IDirectMusicDownload {
    pub unsafe fn GetBuffer(&self, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppvbuffer), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IDirectMusicDownload> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicDownload) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicDownload> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicDownload) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicDownload> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicDownload) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicDownload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicDownload").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicDownload {
    type Vtable = IDirectMusicDownload_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287b_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownload_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicDownloadedInstrument(::windows::core::IUnknown);
impl IDirectMusicDownloadedInstrument {}
impl ::core::convert::From<IDirectMusicDownloadedInstrument> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicDownloadedInstrument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicDownloadedInstrument> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicDownloadedInstrument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicDownloadedInstrument> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicDownloadedInstrument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicDownloadedInstrument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicDownloadedInstrument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicDownloadedInstrument {
    type Vtable = IDirectMusicDownloadedInstrument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287e_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownloadedInstrument_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicInstrument(::windows::core::IUnknown);
impl IDirectMusicInstrument {
    pub unsafe fn GetPatch(&self, pdwpatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwpatch)).ok()
    }
    pub unsafe fn SetPatch(&self, dwpatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPatch)(::windows::core::Interface::as_raw(self), dwpatch).ok()
    }
}
impl ::core::convert::From<IDirectMusicInstrument> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicInstrument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicInstrument> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicInstrument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicInstrument> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicInstrument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicInstrument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicInstrument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicInstrument {
    type Vtable = IDirectMusicInstrument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287d_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicInstrument_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetPatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpatch: *mut u32) -> ::windows::core::HRESULT,
    pub SetPatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpatch: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicPort(::windows::core::IUnknown);
impl IDirectMusicPort {
    pub unsafe fn PlayBuffer<'a, P0>(&self, pbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicBuffer>>,
    {
        (::windows::core::Interface::vtable(self).PlayBuffer)(::windows::core::Interface::as_raw(self), pbuffer.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadNotificationHandle<'a, P0>(&self, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).SetReadNotificationHandle)(::windows::core::Interface::as_raw(self), hevent.into()).ok()
    }
    pub unsafe fn Read<'a, P0>(&self, pbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicBuffer>>,
    {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), pbuffer.into().abi()).ok()
    }
    pub unsafe fn DownloadInstrument<'a, P0>(&self, pinstrument: P0, ppdownloadedinstrument: *mut ::core::option::Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicInstrument>>,
    {
        (::windows::core::Interface::vtable(self).DownloadInstrument)(::windows::core::Interface::as_raw(self), pinstrument.into().abi(), ::core::mem::transmute(ppdownloadedinstrument), ::core::mem::transmute(pnoteranges), dwnumnoteranges).ok()
    }
    pub unsafe fn UnloadInstrument<'a, P0>(&self, pdownloadedinstrument: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicDownloadedInstrument>>,
    {
        (::windows::core::Interface::vtable(self).UnloadInstrument)(::windows::core::Interface::as_raw(self), pdownloadedinstrument.into().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLatencyClock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::IReferenceClock>(result__)
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRunningStats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstats)).ok()
    }
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Compact)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pportcaps)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceIoControl)(::windows::core::Interface::as_raw(self), dwiocontrolcode, ::core::mem::transmute(lpinbuffer), ninbuffersize, ::core::mem::transmute(lpoutbuffer), noutbuffersize, ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpoverlapped)).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNumChannelGroups)(::windows::core::Interface::as_raw(self), dwchannelgroups).ok()
    }
    pub unsafe fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNumChannelGroups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwchannelgroups)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, P0>(&self, factive: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), factive.into()).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelPriority)(::windows::core::Interface::as_raw(self), dwchannelgroup, dwchannel, dwpriority).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChannelPriority)(::windows::core::Interface::as_raw(self), dwchannelgroup, dwchannel, ::core::mem::transmute(pdwpriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<'a, P0, P1>(&self, pdirectsound: P0, pdirectsoundbuffer: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DirectSound::IDirectSound>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::DirectSound::IDirectSoundBuffer>>,
    {
        (::windows::core::Interface::vtable(self).SetDirectSound)(::windows::core::Interface::as_raw(self), pdirectsound.into().abi(), pdirectsoundbuffer.into().abi()).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize), ::core::mem::transmute(pdwbuffersize)).ok()
    }
}
impl ::core::convert::From<IDirectMusicPort> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicPort> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicPort) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicPort> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicPort) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicPort {
    type Vtable = IDirectMusicPort_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f2d8c9_37c2_11d2_b9f9_0000f875ac12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPort_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub PlayBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReadNotificationHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReadNotificationHandle: usize,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DownloadInstrument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstrument: *mut ::core::ffi::c_void, ppdownloadedinstrument: *mut *mut ::core::ffi::c_void, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::core::HRESULT,
    pub UnloadInstrument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdownloadedinstrument: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRunningStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT,
    pub Compact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub DeviceIoControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))]
    DeviceIoControl: usize,
    pub SetNumChannelGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroups: u32) -> ::windows::core::HRESULT,
    pub GetNumChannelGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwchannelgroups: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factive: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    pub SetChannelPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT,
    pub GetChannelPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub SetDirectSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, pdirectsoundbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))]
    SetDirectSound: usize,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicPortDownload(::windows::core::IUnknown);
impl IDirectMusicPortDownload {
    pub unsafe fn GetBuffer(&self, dwdlid: u32) -> ::windows::core::Result<IDirectMusicDownload> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), dwdlid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDirectMusicDownload>(result__)
    }
    pub unsafe fn AllocateBuffer(&self, dwsize: u32) -> ::windows::core::Result<IDirectMusicDownload> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AllocateBuffer)(::windows::core::Interface::as_raw(self), dwsize, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDirectMusicDownload>(result__)
    }
    pub unsafe fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDLId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwstartdlid), dwcount).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAppend)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwappend)).ok()
    }
    pub unsafe fn Download<'a, P0>(&self, pidmdownload: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicDownload>>,
    {
        (::windows::core::Interface::vtable(self).Download)(::windows::core::Interface::as_raw(self), pidmdownload.into().abi()).ok()
    }
    pub unsafe fn Unload<'a, P0>(&self, pidmdownload: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicDownload>>,
    {
        (::windows::core::Interface::vtable(self).Unload)(::windows::core::Interface::as_raw(self), pidmdownload.into().abi()).ok()
    }
}
impl ::core::convert::From<IDirectMusicPortDownload> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicPortDownload) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicPortDownload> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicPortDownload) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicPortDownload> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicPortDownload) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicPortDownload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicPortDownload").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicPortDownload {
    type Vtable = IDirectMusicPortDownload_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ac287a_b39b_11d1_8704_00600893b1bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPortDownload_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdlid: u32, ppidmdownload: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsize: u32, ppidmdownload: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDLId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::core::HRESULT,
    pub GetAppend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT,
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidmdownload: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidmdownload: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicSynth(::windows::core::IUnknown);
impl IDirectMusicSynth {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pportparams)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNumChannelGroups)(::windows::core::Interface::as_raw(self), dwgroups).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Download)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phdownload), ::core::mem::transmute(pvdata), ::core::mem::transmute(pbfree)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unload<'a, P0, P1>(&self, hdownload: P0, lpfreehandle: isize, huserdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).Unload)(::windows::core::Interface::as_raw(self), hdownload.into(), lpfreehandle, huserdata.into()).ok()
    }
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PlayBuffer)(::windows::core::Interface::as_raw(self), rt, ::core::mem::transmute(pbbuffer), cbbuffer).ok()
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRunningStats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstats)).ok()
    }
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPortCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcaps)).ok()
    }
    pub unsafe fn SetMasterClock<'a, P0>(&self, pclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::IReferenceClock>>,
    {
        (::windows::core::Interface::vtable(self).SetMasterClock)(::windows::core::Interface::as_raw(self), pclock.into().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLatencyClock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn SetSynthSink<'a, P0>(&self, psynthsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicSynthSink>>,
    {
        (::windows::core::Interface::vtable(self).SetSynthSink)(::windows::core::Interface::as_raw(self), psynthsink.into().abi()).ok()
    }
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Render)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbuffer), dwlength, llposition).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelPriority)(::windows::core::Interface::as_raw(self), dwchannelgroup, dwchannel, dwpriority).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChannelPriority)(::windows::core::Interface::as_raw(self), dwchannelgroup, dwchannel, ::core::mem::transmute(pdwpriority)).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize)).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAppend)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwappend)).ok()
    }
}
impl ::core::convert::From<IDirectMusicSynth> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicSynth) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicSynth> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicSynth) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicSynth> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicSynth) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicSynth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicSynth").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicSynth {
    type Vtable = IDirectMusicSynth_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09823661_5c85_11d2_afa6_00aa0024d8b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Open: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNumChannelGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwgroups: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Download: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Unload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unload: usize,
    pub PlayBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::HRESULT,
    pub GetRunningStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::HRESULT,
    pub GetPortCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclock: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    pub SetSynthSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psynthsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Render: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::HRESULT,
    pub SetChannelPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::HRESULT,
    pub GetChannelPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetAppend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicSynth8(::windows::core::IUnknown);
impl IDirectMusicSynth8 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Open)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pportparams)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetNumChannelGroups)(::windows::core::Interface::as_raw(self), dwgroups).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Download)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phdownload), ::core::mem::transmute(pvdata), ::core::mem::transmute(pbfree)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unload<'a, P0, P1>(&self, hdownload: P0, lpfreehandle: isize, huserdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
        P1: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).base__.Unload)(::windows::core::Interface::as_raw(self), hdownload.into(), lpfreehandle, huserdata.into()).ok()
    }
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.PlayBuffer)(::windows::core::Interface::as_raw(self), rt, ::core::mem::transmute(pbbuffer), cbbuffer).ok()
    }
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRunningStats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstats)).ok()
    }
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPortCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcaps)).ok()
    }
    pub unsafe fn SetMasterClock<'a, P0>(&self, pclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::IReferenceClock>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetMasterClock)(::windows::core::Interface::as_raw(self), pclock.into().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLatencyClock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Activate)(::windows::core::Interface::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn SetSynthSink<'a, P0>(&self, psynthsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicSynthSink>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetSynthSink)(::windows::core::Interface::as_raw(self), psynthsink.into().abi()).ok()
    }
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Render)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbuffer), dwlength, llposition).ok()
    }
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetChannelPriority)(::windows::core::Interface::as_raw(self), dwchannelgroup, dwchannel, dwpriority).ok()
    }
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetChannelPriority)(::windows::core::Interface::as_raw(self), dwchannelgroup, dwchannel, ::core::mem::transmute(pdwpriority)).ok()
    }
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwaveformatex), ::core::mem::transmute(pdwwaveformatexsize)).ok()
    }
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAppend)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwappend)).ok()
    }
    pub unsafe fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PlayVoice)(::windows::core::Interface::as_raw(self), rt, dwvoiceid, dwchannelgroup, dwchannel, dwdlid, prpitch, vrvolume, stvoicestart, stloopstart, stloopend).ok()
    }
    pub unsafe fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopVoice)(::windows::core::Interface::as_raw(self), rt, dwvoiceid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVoiceState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwvoice), cbvoice, ::core::mem::transmute(dwvoicestate)).ok()
    }
    pub unsafe fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self), dwdownloadid, dwflags).ok()
    }
    pub unsafe fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssignChannelToBuses)(::windows::core::Interface::as_raw(self), dwchannelgroup, dwchannel, ::core::mem::transmute(pdwbuses), cbuses).ok()
    }
}
impl ::core::convert::From<IDirectMusicSynth8> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicSynth8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicSynth8> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicSynth8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicSynth8> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicSynth8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IDirectMusicSynth8> for IDirectMusicSynth {
    fn from(value: IDirectMusicSynth8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicSynth8> for &'a IDirectMusicSynth {
    fn from(value: &'a IDirectMusicSynth8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicSynth8> for IDirectMusicSynth {
    fn from(value: &IDirectMusicSynth8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicSynth8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicSynth8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicSynth8 {
    type Vtable = IDirectMusicSynth8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53cab625_2711_4c9f_9de7_1b7f925f6fc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth8_Vtbl {
    pub base__: IDirectMusicSynth_Vtbl,
    pub PlayVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::core::HRESULT,
    pub StopVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVoiceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVoiceState: usize,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdownloadid: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub AssignChannelToBuses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicSynthSink(::windows::core::IUnknown);
impl IDirectMusicSynthSink {
    pub unsafe fn Init<'a, P0>(&self, psynth: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicSynth>>,
    {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), psynth.into().abi()).ok()
    }
    pub unsafe fn SetMasterClock<'a, P0>(&self, pclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::IReferenceClock>>,
    {
        (::windows::core::Interface::vtable(self).SetMasterClock)(::windows::core::Interface::as_raw(self), pclock.into().abi()).ok()
    }
    pub unsafe fn GetLatencyClock(&self) -> ::windows::core::Result<super::super::IReferenceClock> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLatencyClock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::IReferenceClock>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Activate<'a, P0>(&self, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), fenable.into()).ok()
    }
    pub unsafe fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SampleToRefTime)(::windows::core::Interface::as_raw(self), llsampletime, ::core::mem::transmute(prftime)).ok()
    }
    pub unsafe fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefTimeToSample)(::windows::core::Interface::as_raw(self), rftime, ::core::mem::transmute(pllsampletime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub unsafe fn SetDirectSound<'a, P0, P1>(&self, pdirectsound: P0, pdirectsoundbuffer: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::DirectSound::IDirectSound>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::DirectSound::IDirectSoundBuffer>>,
    {
        (::windows::core::Interface::vtable(self).SetDirectSound)(::windows::core::Interface::as_raw(self), pdirectsound.into().abi(), pdirectsoundbuffer.into().abi()).ok()
    }
    pub unsafe fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesiredBufferSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwbuffersizeinsamples)).ok()
    }
}
impl ::core::convert::From<IDirectMusicSynthSink> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicSynthSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicSynthSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicSynthSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicSynthSink> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicSynthSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicSynthSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicSynthSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicSynthSink {
    type Vtable = IDirectMusicSynthSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09823663_5c85_11d2_afa6_00aa0024d8b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynthSink_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psynth: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMasterClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclock: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLatencyClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Activate: usize,
    pub SampleToRefTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llsampletime: i64, prftime: *mut i64) -> ::windows::core::HRESULT,
    pub RefTimeToSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rftime: i64, pllsampletime: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio_DirectSound")]
    pub SetDirectSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, pdirectsoundbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio_DirectSound"))]
    SetDirectSound: usize,
    pub GetDesiredBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbuffersizeinsamples: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectMusic\"`*"]
#[repr(transparent)]
pub struct IDirectMusicThru(::windows::core::IUnknown);
impl IDirectMusicThru {
    pub unsafe fn ThruChannel<'a, P0>(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDirectMusicPort>>,
    {
        (::windows::core::Interface::vtable(self).ThruChannel)(::windows::core::Interface::as_raw(self), dwsourcechannelgroup, dwsourcechannel, dwdestinationchannelgroup, dwdestinationchannel, pdestinationport.into().abi()).ok()
    }
}
impl ::core::convert::From<IDirectMusicThru> for ::windows::core::IUnknown {
    fn from(value: IDirectMusicThru) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirectMusicThru> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirectMusicThru) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectMusicThru> for ::windows::core::IUnknown {
    fn from(value: &IDirectMusicThru) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
impl ::core::fmt::Debug for IDirectMusicThru {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectMusicThru").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectMusicThru {
    type Vtable = IDirectMusicThru_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xced153e7_3606_11d2_b9f9_0000f875ac12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicThru_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ThruChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
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
impl ::core::fmt::Debug for INSTHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTHEADER").field("cRegions", &self.cRegions).field("Locale", &self.Locale).finish()
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
impl ::core::fmt::Debug for MIDILOCALE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDILOCALE").field("ulBank", &self.ulBank).field("ulInstrument", &self.ulInstrument).finish()
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
impl ::core::fmt::Debug for POOLCUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POOLCUE").field("ulOffset", &self.ulOffset).finish()
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
impl ::core::fmt::Debug for POOLTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POOLTABLE").field("cbSize", &self.cbSize).field("cCues", &self.cCues).finish()
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
impl ::core::fmt::Debug for RGNHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNHEADER").field("RangeKey", &self.RangeKey).field("RangeVelocity", &self.RangeVelocity).field("fusOptions", &self.fusOptions).field("usKeyGroup", &self.usKeyGroup).finish()
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
impl ::core::fmt::Debug for RGNRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNRANGE").field("usLow", &self.usLow).field("usHigh", &self.usHigh).finish()
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
impl ::core::fmt::Debug for Tag_DVAudInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Tag_DVAudInfo").field("bAudStyle", &self.bAudStyle).field("bAudQu", &self.bAudQu).field("bNumAudPin", &self.bNumAudPin).field("wAvgSamplesPerPinPerFrm", &self.wAvgSamplesPerPinPerFrm).field("wBlkMode", &self.wBlkMode).field("wDIFMode", &self.wDIFMode).field("wBlkDiv", &self.wBlkDiv).finish()
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
impl ::core::fmt::Debug for WAVELINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WAVELINK").field("fusOptions", &self.fusOptions).field("usPhaseGroup", &self.usPhaseGroup).field("ulChannel", &self.ulChannel).field("ulTableIndex", &self.ulTableIndex).finish()
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for _DMUS_PORTPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_DMUS_PORTPARAMS").field("dwSize", &self.dwSize).field("dwValidParams", &self.dwValidParams).field("dwVoices", &self.dwVoices).field("dwChannelGroups", &self.dwChannelGroups).field("dwAudioChannels", &self.dwAudioChannels).field("dwSampleRate", &self.dwSampleRate).field("dwEffectFlags", &self.dwEffectFlags).field("fShare", &self.fShare).finish()
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
impl ::core::fmt::Debug for _rloop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_rloop").field("cbSize", &self.cbSize).field("ulType", &self.ulType).field("ulStart", &self.ulStart).field("ulLength", &self.ulLength).finish()
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
impl ::core::fmt::Debug for _rwsmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_rwsmp").field("cbSize", &self.cbSize).field("usUnityNote", &self.usUnityNote).field("sFineTune", &self.sFineTune).field("lAttenuation", &self.lAttenuation).field("fulOptions", &self.fulOptions).field("cSampleLoops", &self.cSampleLoops).finish()
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
