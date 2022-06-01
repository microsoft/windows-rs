pub const CLSID_DirectSound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47d4d946_62e8_11cf_93bc_444553540000);
pub const CLSID_DirectSound8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3901cc3f_84b5_4fa4_ba35_aa8172b8a09b);
pub const CLSID_DirectSoundCapture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0210780_89cd_11d0_af08_00a0c925cd16);
pub const CLSID_DirectSoundCapture8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4bcac13_7f99_4908_9a8e_74e3bf24b6e1);
pub const CLSID_DirectSoundFullDuplex: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea4300c_7959_4147_b26a_2377b9e7a91d);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DIRECTSOUND_VERSION: u32 = 1792u32;
pub const DS3DALG_HRTF_FULL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2413340_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_HRTF_LIGHT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2413342_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_NO_VIRTUALIZATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241333f_1c1b_11d2_94f5_00c04fc28aca);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct DS3DBUFFER {
    pub dwSize: u32,
    pub vPosition: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vVelocity: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub dwInsideConeAngle: u32,
    pub dwOutsideConeAngle: u32,
    pub vConeOrientation: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub lConeOutsideVolume: i32,
    pub flMinDistance: f32,
    pub flMaxDistance: f32,
    pub dwMode: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for DS3DBUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for DS3DBUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for DS3DBUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS3DBUFFER").field("dwSize", &self.dwSize).field("vPosition", &self.vPosition).field("vVelocity", &self.vVelocity).field("dwInsideConeAngle", &self.dwInsideConeAngle).field("dwOutsideConeAngle", &self.dwOutsideConeAngle).field("vConeOrientation", &self.vConeOrientation).field("lConeOutsideVolume", &self.lConeOutsideVolume).field("flMinDistance", &self.flMinDistance).field("flMaxDistance", &self.flMaxDistance).field("dwMode", &self.dwMode).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for DS3DBUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for DS3DBUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DS3DBUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for DS3DBUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for DS3DBUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct DS3DLISTENER {
    pub dwSize: u32,
    pub vPosition: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vVelocity: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vOrientFront: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub vOrientTop: super::super::super::Graphics::Direct3D::D3DVECTOR,
    pub flDistanceFactor: f32,
    pub flRolloffFactor: f32,
    pub flDopplerFactor: f32,
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::marker::Copy for DS3DLISTENER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::clone::Clone for DS3DLISTENER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for DS3DLISTENER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DS3DLISTENER").field("dwSize", &self.dwSize).field("vPosition", &self.vPosition).field("vVelocity", &self.vVelocity).field("vOrientFront", &self.vOrientFront).field("vOrientTop", &self.vOrientTop).field("flDistanceFactor", &self.flDistanceFactor).field("flRolloffFactor", &self.flRolloffFactor).field("flDopplerFactor", &self.flDopplerFactor).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for DS3DLISTENER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for DS3DLISTENER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DS3DLISTENER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for DS3DLISTENER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for DS3DLISTENER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3DMODE_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3DMODE_HEADRELATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3DMODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTCONEANGLE: u32 = 360u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTDISTANCEFACTOR: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTDOPPLERFACTOR: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTMAXDISTANCE: f32 = 1000000000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTMINDISTANCE: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFAULTROLLOFFFACTOR: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_DEFERRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_IMMEDIATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MAXCONEANGLE: u32 = 360u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MAXDOPPLERFACTOR: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MAXROLLOFFFACTOR: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MINCONEANGLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MINDOPPLERFACTOR: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS3D_MINROLLOFFFACTOR: f32 = 0f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwUnlockTransferRate: u32,
    pub dwPlayCpuOverhead: u32,
}
impl ::core::marker::Copy for DSBCAPS {}
impl ::core::clone::Clone for DSBCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSBCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwUnlockTransferRate", &self.dwUnlockTransferRate).field("dwPlayCpuOverhead", &self.dwPlayCpuOverhead).finish()
    }
}
unsafe impl ::windows::core::Abi for DSBCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSBCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSBCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSBCAPS {}
impl ::core::default::Default for DSBCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRL3D: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLFREQUENCY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLFX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLPAN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_CTRLVOLUME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_GETCURRENTPOSITION2: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_GLOBALFOCUS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_LOCDEFER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_LOCHARDWARE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_LOCSOFTWARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_PRIMARYBUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_STATIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_STICKYFOCUS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBCAPS_TRUEPLAYPOSITION: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBFREQUENCY_MAX: u32 = 200000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBFREQUENCY_MIN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBFREQUENCY_ORIGINAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBLOCK_ENTIREBUFFER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBLOCK_FROMWRITECURSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBNOTIFICATIONS_MAX: u32 = 100000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPAN_CENTER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPAN_LEFT: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPAN_RIGHT: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_LOCHARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_LOCSOFTWARE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_LOOPING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_TERMINATEBY_DISTANCE: u64 = 16u64;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_TERMINATEBY_PRIORITY: u64 = 32u64;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPLAY_TERMINATEBY_TIME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBPN_OFFSETSTOP: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: u32,
    pub hEventNotify: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSBPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSBPOSITIONNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSBPOSITIONNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBPOSITIONNOTIFY").field("dwOffset", &self.dwOffset).field("hEventNotify", &self.hEventNotify).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSBPOSITIONNOTIFY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSBPOSITIONNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSBPOSITIONNOTIFY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSBPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSBPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSIZE_FX_MIN: u32 = 150u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSIZE_MAX: u32 = 268435455u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSIZE_MIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_BUFFERLOST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_LOCHARDWARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_LOCSOFTWARE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_LOOPING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_PLAYING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBSTATUS_TERMINATED: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub guid3DAlgorithm: ::windows::core::GUID,
}
impl ::core::marker::Copy for DSBUFFERDESC {}
impl ::core::clone::Clone for DSBUFFERDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSBUFFERDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).field("guid3DAlgorithm", &self.guid3DAlgorithm).finish()
    }
}
unsafe impl ::windows::core::Abi for DSBUFFERDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSBUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSBUFFERDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSBUFFERDESC {}
impl ::core::default::Default for DSBUFFERDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
}
impl ::core::marker::Copy for DSBUFFERDESC1 {}
impl ::core::clone::Clone for DSBUFFERDESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSBUFFERDESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSBUFFERDESC1").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).finish()
    }
}
unsafe impl ::windows::core::Abi for DSBUFFERDESC1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSBUFFERDESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSBUFFERDESC1>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSBUFFERDESC1 {}
impl ::core::default::Default for DSBUFFERDESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBVOLUME_MAX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSBVOLUME_MIN: i32 = -10000i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwMinSecondarySampleRate: u32,
    pub dwMaxSecondarySampleRate: u32,
    pub dwPrimaryBuffers: u32,
    pub dwMaxHwMixingAllBuffers: u32,
    pub dwMaxHwMixingStaticBuffers: u32,
    pub dwMaxHwMixingStreamingBuffers: u32,
    pub dwFreeHwMixingAllBuffers: u32,
    pub dwFreeHwMixingStaticBuffers: u32,
    pub dwFreeHwMixingStreamingBuffers: u32,
    pub dwMaxHw3DAllBuffers: u32,
    pub dwMaxHw3DStaticBuffers: u32,
    pub dwMaxHw3DStreamingBuffers: u32,
    pub dwFreeHw3DAllBuffers: u32,
    pub dwFreeHw3DStaticBuffers: u32,
    pub dwFreeHw3DStreamingBuffers: u32,
    pub dwTotalHwMemBytes: u32,
    pub dwFreeHwMemBytes: u32,
    pub dwMaxContigFreeHwMemBytes: u32,
    pub dwUnlockTransferRateHwBuffers: u32,
    pub dwPlayCpuOverheadSwBuffers: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for DSCAPS {}
impl ::core::clone::Clone for DSCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwMinSecondarySampleRate", &self.dwMinSecondarySampleRate)
            .field("dwMaxSecondarySampleRate", &self.dwMaxSecondarySampleRate)
            .field("dwPrimaryBuffers", &self.dwPrimaryBuffers)
            .field("dwMaxHwMixingAllBuffers", &self.dwMaxHwMixingAllBuffers)
            .field("dwMaxHwMixingStaticBuffers", &self.dwMaxHwMixingStaticBuffers)
            .field("dwMaxHwMixingStreamingBuffers", &self.dwMaxHwMixingStreamingBuffers)
            .field("dwFreeHwMixingAllBuffers", &self.dwFreeHwMixingAllBuffers)
            .field("dwFreeHwMixingStaticBuffers", &self.dwFreeHwMixingStaticBuffers)
            .field("dwFreeHwMixingStreamingBuffers", &self.dwFreeHwMixingStreamingBuffers)
            .field("dwMaxHw3DAllBuffers", &self.dwMaxHw3DAllBuffers)
            .field("dwMaxHw3DStaticBuffers", &self.dwMaxHw3DStaticBuffers)
            .field("dwMaxHw3DStreamingBuffers", &self.dwMaxHw3DStreamingBuffers)
            .field("dwFreeHw3DAllBuffers", &self.dwFreeHw3DAllBuffers)
            .field("dwFreeHw3DStaticBuffers", &self.dwFreeHw3DStaticBuffers)
            .field("dwFreeHw3DStreamingBuffers", &self.dwFreeHw3DStreamingBuffers)
            .field("dwTotalHwMemBytes", &self.dwTotalHwMemBytes)
            .field("dwFreeHwMemBytes", &self.dwFreeHwMemBytes)
            .field("dwMaxContigFreeHwMemBytes", &self.dwMaxContigFreeHwMemBytes)
            .field("dwUnlockTransferRateHwBuffers", &self.dwUnlockTransferRateHwBuffers)
            .field("dwPlayCpuOverheadSwBuffers", &self.dwPlayCpuOverheadSwBuffers)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSCAPS {}
impl ::core::default::Default for DSCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_CERTIFIED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_CONTINUOUSRATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_EMULDRIVER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_PRIMARY16BIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_PRIMARY8BIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_PRIMARYMONO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_PRIMARYSTEREO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_SECONDARY16BIT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_SECONDARY8BIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_SECONDARYMONO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCAPS_SECONDARYSTEREO: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for DSCBCAPS {}
impl ::core::clone::Clone for DSCBCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSCBCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCBCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for DSCBCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSCBCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSCBCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSCBCAPS {}
impl ::core::default::Default for DSCBCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBCAPS_CTRLFX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBLOCK_ENTIREBUFFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBSTART_LOOPING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBSTATUS_CAPTURING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCBSTATUS_LOOPING: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub dwFXCount: u32,
    pub lpDSCFXDesc: *mut DSCEFFECTDESC,
}
impl ::core::marker::Copy for DSCBUFFERDESC {}
impl ::core::clone::Clone for DSCBUFFERDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSCBUFFERDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCBUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).field("dwFXCount", &self.dwFXCount).field("lpDSCFXDesc", &self.lpDSCFXDesc).finish()
    }
}
unsafe impl ::windows::core::Abi for DSCBUFFERDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSCBUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSCBUFFERDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSCBUFFERDESC {}
impl ::core::default::Default for DSCBUFFERDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
}
impl ::core::marker::Copy for DSCBUFFERDESC1 {}
impl ::core::clone::Clone for DSCBUFFERDESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSCBUFFERDESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCBUFFERDESC1").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).finish()
    }
}
unsafe impl ::windows::core::Abi for DSCBUFFERDESC1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSCBUFFERDESC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSCBUFFERDESC1>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSCBUFFERDESC1 {}
impl ::core::default::Default for DSCBUFFERDESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFormats: u32,
    pub dwChannels: u32,
}
impl ::core::marker::Copy for DSCCAPS {}
impl ::core::clone::Clone for DSCCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSCCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwFormats", &self.dwFormats).field("dwChannels", &self.dwChannels).finish()
    }
}
unsafe impl ::windows::core::Abi for DSCCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSCCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSCCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSCCAPS {}
impl ::core::default::Default for DSCCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCCAPS_CERTIFIED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCCAPS_EMULDRIVER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCCAPS_MULTIPLECAPTURE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSCEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSCFXClass: ::windows::core::GUID,
    pub guidDSCFXInstance: ::windows::core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for DSCEFFECTDESC {}
impl ::core::clone::Clone for DSCEFFECTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSCEFFECTDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCEFFECTDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidDSCFXClass", &self.guidDSCFXClass).field("guidDSCFXInstance", &self.guidDSCFXInstance).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for DSCEFFECTDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSCEFFECTDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSCEFFECTDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSCEFFECTDESC {}
impl ::core::default::Default for DSCEFFECTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSCFXAec {
    pub fEnable: super::super::super::Foundation::BOOL,
    pub fNoiseFill: super::super::super::Foundation::BOOL,
    pub dwMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSCFXAec {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSCFXAec {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSCFXAec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCFXAec").field("fEnable", &self.fEnable).field("fNoiseFill", &self.fNoiseFill).field("dwMode", &self.dwMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSCFXAec {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSCFXAec {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSCFXAec>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSCFXAec {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSCFXAec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DSCFXNoiseSuppress {
    pub fEnable: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DSCFXNoiseSuppress {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DSCFXNoiseSuppress {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSCFXNoiseSuppress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSCFXNoiseSuppress").field("fEnable", &self.fEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSCFXNoiseSuppress {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSCFXNoiseSuppress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSCFXNoiseSuppress>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSCFXNoiseSuppress {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSCFXNoiseSuppress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFXR_LOCHARDWARE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFXR_LOCSOFTWARE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_MODE_FULL_DUPLEX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_MODE_HALF_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_MODE_PASS_THROUGH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_LOCHARDWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSCFX_LOCSOFTWARE: u32 = 2u32;
pub const DSDEVID_DefaultCapture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef00001_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultPlayback: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef00000_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoiceCapture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef00003_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoicePlayback: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef00002_9c6d_47ed_aaf1_4dda8f2b5c03);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSFXClass: ::windows::core::GUID,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl ::core::marker::Copy for DSEFFECTDESC {}
impl ::core::clone::Clone for DSEFFECTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSEFFECTDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSEFFECTDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidDSFXClass", &self.guidDSFXClass).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for DSEFFECTDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSEFFECTDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSEFFECTDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSEFFECTDESC {}
impl ::core::default::Default for DSEFFECTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_DELAY_MAX: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_DELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_DEPTH_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_DEPTH_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_FEEDBACK_MAX: f32 = 99f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_FEEDBACK_MIN: f32 = -99f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_FREQUENCY_MAX: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_FREQUENCY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_180: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_90: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_NEG_180: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_NEG_90: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_PHASE_ZERO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_WAVE_SIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_WAVE_TRIANGLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCHORUS_WETDRYMIX_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_ATTACK_MAX: f32 = 500f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_ATTACK_MIN: f32 = 0.01f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_GAIN_MAX: f32 = 60f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_GAIN_MIN: f32 = -60f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_PREDELAY_MAX: f32 = 4f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_PREDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_RATIO_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_RATIO_MIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_RELEASE_MAX: f32 = 3000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_RELEASE_MIN: f32 = 50f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_THRESHOLD_MAX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXCOMPRESSOR_THRESHOLD_MIN: f32 = -60f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXChorus {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl ::core::marker::Copy for DSFXChorus {}
impl ::core::clone::Clone for DSFXChorus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXChorus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXChorus").field("fWetDryMix", &self.fWetDryMix).field("fDepth", &self.fDepth).field("fFeedback", &self.fFeedback).field("fFrequency", &self.fFrequency).field("lWaveform", &self.lWaveform).field("fDelay", &self.fDelay).field("lPhase", &self.lPhase).finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXChorus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXChorus {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXChorus>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXChorus {}
impl ::core::default::Default for DSFXChorus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXCompressor {
    pub fGain: f32,
    pub fAttack: f32,
    pub fRelease: f32,
    pub fThreshold: f32,
    pub fRatio: f32,
    pub fPredelay: f32,
}
impl ::core::marker::Copy for DSFXCompressor {}
impl ::core::clone::Clone for DSFXCompressor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXCompressor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXCompressor").field("fGain", &self.fGain).field("fAttack", &self.fAttack).field("fRelease", &self.fRelease).field("fThreshold", &self.fThreshold).field("fRatio", &self.fRatio).field("fPredelay", &self.fPredelay).finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXCompressor {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXCompressor {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXCompressor>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXCompressor {}
impl ::core::default::Default for DSFXCompressor {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_EDGE_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_EDGE_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_GAIN_MAX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_GAIN_MIN: f32 = -60f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MAX: f32 = 8000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MIN: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MAX: f32 = 8000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MIN: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MAX: f32 = 8000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MIN: f32 = 100f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXDistortion {
    pub fGain: f32,
    pub fEdge: f32,
    pub fPostEQCenterFrequency: f32,
    pub fPostEQBandwidth: f32,
    pub fPreLowpassCutoff: f32,
}
impl ::core::marker::Copy for DSFXDistortion {}
impl ::core::clone::Clone for DSFXDistortion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXDistortion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXDistortion").field("fGain", &self.fGain).field("fEdge", &self.fEdge).field("fPostEQCenterFrequency", &self.fPostEQCenterFrequency).field("fPostEQBandwidth", &self.fPostEQBandwidth).field("fPreLowpassCutoff", &self.fPreLowpassCutoff).finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXDistortion {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXDistortion {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXDistortion>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXDistortion {}
impl ::core::default::Default for DSFXDistortion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_FEEDBACK_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_FEEDBACK_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_LEFTDELAY_MAX: f32 = 2000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_LEFTDELAY_MIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_PANDELAY_MAX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_PANDELAY_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_RIGHTDELAY_MAX: f32 = 2000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_RIGHTDELAY_MIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXECHO_WETDRYMIX_MIN: f32 = 0f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXEcho {
    pub fWetDryMix: f32,
    pub fFeedback: f32,
    pub fLeftDelay: f32,
    pub fRightDelay: f32,
    pub lPanDelay: i32,
}
impl ::core::marker::Copy for DSFXEcho {}
impl ::core::clone::Clone for DSFXEcho {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXEcho {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXEcho").field("fWetDryMix", &self.fWetDryMix).field("fFeedback", &self.fFeedback).field("fLeftDelay", &self.fLeftDelay).field("fRightDelay", &self.fRightDelay).field("lPanDelay", &self.lPanDelay).finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXEcho {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXEcho {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXEcho>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXEcho {}
impl ::core::default::Default for DSFXEcho {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_DELAY_MAX: f32 = 4f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_DELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_DEPTH_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_DEPTH_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_FEEDBACK_MAX: f32 = 99f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_FEEDBACK_MIN: f32 = -99f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_FREQUENCY_MAX: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_FREQUENCY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_180: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_90: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_MAX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_NEG_180: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_NEG_90: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_PHASE_ZERO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_WAVE_SIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_WAVE_TRIANGLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXFLANGER_WETDRYMIX_MIN: f32 = 0f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXFlanger {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl ::core::marker::Copy for DSFXFlanger {}
impl ::core::clone::Clone for DSFXFlanger {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXFlanger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXFlanger").field("fWetDryMix", &self.fWetDryMix).field("fDepth", &self.fDepth).field("fFeedback", &self.fFeedback).field("fFrequency", &self.fFrequency).field("lWaveform", &self.lWaveform).field("fDelay", &self.fDelay).field("lPhase", &self.lPhase).finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXFlanger {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXFlanger {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXFlanger>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXFlanger {}
impl ::core::default::Default for DSFXFlanger {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXGARGLE_RATEHZ_MAX: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXGARGLE_RATEHZ_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXGARGLE_WAVE_SQUARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXGARGLE_WAVE_TRIANGLE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXGargle {
    pub dwRateHz: u32,
    pub dwWaveShape: u32,
}
impl ::core::marker::Copy for DSFXGargle {}
impl ::core::clone::Clone for DSFXGargle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXGargle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXGargle").field("dwRateHz", &self.dwRateHz).field("dwWaveShape", &self.dwWaveShape).finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXGargle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXGargle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXGargle>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXGargle {}
impl ::core::default::Default for DSFXGargle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXI3DL2Reverb {
    pub lRoom: i32,
    pub lRoomHF: i32,
    pub flRoomRolloffFactor: f32,
    pub flDecayTime: f32,
    pub flDecayHFRatio: f32,
    pub lReflections: i32,
    pub flReflectionsDelay: f32,
    pub lReverb: i32,
    pub flReverbDelay: f32,
    pub flDiffusion: f32,
    pub flDensity: f32,
    pub flHFReference: f32,
}
impl ::core::marker::Copy for DSFXI3DL2Reverb {}
impl ::core::clone::Clone for DSFXI3DL2Reverb {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXI3DL2Reverb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXI3DL2Reverb")
            .field("lRoom", &self.lRoom)
            .field("lRoomHF", &self.lRoomHF)
            .field("flRoomRolloffFactor", &self.flRoomRolloffFactor)
            .field("flDecayTime", &self.flDecayTime)
            .field("flDecayHFRatio", &self.flDecayHFRatio)
            .field("lReflections", &self.lReflections)
            .field("flReflectionsDelay", &self.flReflectionsDelay)
            .field("lReverb", &self.lReverb)
            .field("flReverbDelay", &self.flReverbDelay)
            .field("flDiffusion", &self.flDiffusion)
            .field("flDensity", &self.flDensity)
            .field("flHFReference", &self.flHFReference)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXI3DL2Reverb {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXI3DL2Reverb {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXI3DL2Reverb>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXI3DL2Reverb {}
impl ::core::default::Default for DSFXI3DL2Reverb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_BANDWIDTH_MAX: f32 = 36f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_BANDWIDTH_MIN: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_CENTER_MAX: f32 = 16000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_CENTER_MIN: f32 = 80f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_GAIN_MAX: f32 = 15f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXPARAMEQ_GAIN_MIN: f32 = -15f32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXParamEq {
    pub fCenter: f32,
    pub fBandwidth: f32,
    pub fGain: f32,
}
impl ::core::marker::Copy for DSFXParamEq {}
impl ::core::clone::Clone for DSFXParamEq {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXParamEq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXParamEq").field("fCenter", &self.fCenter).field("fBandwidth", &self.fBandwidth).field("fGain", &self.fGain).finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXParamEq {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXParamEq {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXParamEq>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXParamEq {}
impl ::core::default::Default for DSFXParamEq {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_FAILED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_LOCHARDWARE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_LOCSOFTWARE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_PRESENT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_SENDLOOP: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_UNALLOCATED: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFXR_UNKNOWN: i32 = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub struct DSFXWavesReverb {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl ::core::marker::Copy for DSFXWavesReverb {}
impl ::core::clone::Clone for DSFXWavesReverb {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSFXWavesReverb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSFXWavesReverb").field("fInGain", &self.fInGain).field("fReverbMix", &self.fReverbMix).field("fReverbTime", &self.fReverbTime).field("fHighFreqRTRatio", &self.fHighFreqRTRatio).finish()
    }
}
unsafe impl ::windows::core::Abi for DSFXWavesReverb {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DSFXWavesReverb {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DSFXWavesReverb>()) == 0 }
    }
}
impl ::core::cmp::Eq for DSFXWavesReverb {}
impl ::core::default::Default for DSFXWavesReverb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_DEFAULT: f32 = 0.83f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MAX: f32 = 2f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MIN: f32 = 0.1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_DEFAULT: f32 = 1.49f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_MAX: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_MIN: f32 = 0.1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DENSITY_DEFAULT: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DENSITY_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DENSITY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_DEFAULT: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_MAX: f32 = 100f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_DEFAULT: f32 = 5000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_MAX: f32 = 20000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_MIN: f32 = 20f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_QUALITY_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_QUALITY_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: f32 = 0.007f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: f32 = 0.3f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: i32 = -2602i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: f32 = 0.011f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: f32 = 0.1f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERB_MAX: u32 = 2000u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_REVERB_MIN: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: i32 = -100i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: f32 = 10f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: i32 = -1000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOM_MAX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2REVERB_ROOM_MIN: i32 = -10000i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ALLEY: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ARENA: i32 = 10i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_BATHROOM: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: i32 = 12i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CAVE: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CITY: i32 = 17i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_FOREST: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_GENERIC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HALLWAY: i32 = 13i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HANGAR: i32 = 11i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEHALL: i32 = 28i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEROOM: i32 = 26i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: i32 = 27i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: i32 = 25i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: i32 = 18i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: i32 = 21i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLAIN: i32 = 20i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLATE: i32 = 29i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_QUARRY: i32 = 19i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ROOM: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: i32 = 22i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SMALLROOM: i32 = 24i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: i32 = 14i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONEROOM: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_UNDERWATER: i32 = 23i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_BRICKWALL: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_CURTAIN: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_DOUBLEWINDOW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_SINGLEWINDOW: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_STONEWALL: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_THICKDOOR: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_THINDOOR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_WOODWALL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_LOCHARDWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_LOCSOFTWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_DEFAULT: f32 = 0.001f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MAX: f32 = 0.999f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MIN: f32 = 0.001f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_INGAIN_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_INGAIN_MAX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_INGAIN_MIN: f32 = -96f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_MAX: f32 = 0f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_MIN: f32 = -96f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_DEFAULT: f32 = 1000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_MAX: f32 = 3000f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_MIN: f32 = 0.001f32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSCL_EXCLUSIVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSCL_NORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSCL_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSCL_WRITEPRIMARY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_5POINT1: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_5POINT1_BACK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_5POINT1_SURROUND: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_7POINT1: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_7POINT1_SURROUND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_7POINT1_WIDE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_DIRECTOUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_GEOMETRY_MAX: u32 = 180u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_GEOMETRY_MIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_GEOMETRY_NARROW: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_GEOMETRY_WIDE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_HEADPHONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_MONO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_QUAD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_STEREO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DSSPEAKER_SURROUND: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS_CERTIFIED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS_NO_VIRTUALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(142082058i32);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const DS_UNCERTIFIED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[inline]
pub unsafe fn DirectSoundCaptureCreate<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguiddevice: *const ::windows::core::GUID, ppdsc: *mut ::core::option::Option<IDirectSoundCapture>, punkouter: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureCreate(pcguiddevice: *const ::windows::core::GUID, ppdsc: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundCaptureCreate(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppdsc), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[inline]
pub unsafe fn DirectSoundCaptureCreate8<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguiddevice: *const ::windows::core::GUID, ppdsc8: *mut ::core::option::Option<IDirectSoundCapture>, punkouter: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureCreate8(pcguiddevice: *const ::windows::core::GUID, ppdsc8: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundCaptureCreate8(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppdsc8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureEnumerateA(pdsenumcallback: *mut ::core::ffi::c_void, pcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundCaptureEnumerateA(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureEnumerateW(pdsenumcallback: *mut ::core::ffi::c_void, pcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundCaptureEnumerateW(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[inline]
pub unsafe fn DirectSoundCreate<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguiddevice: *const ::windows::core::GUID, ppds: *mut ::core::option::Option<IDirectSound>, punkouter: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCreate(pcguiddevice: *const ::windows::core::GUID, ppds: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundCreate(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppds), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[inline]
pub unsafe fn DirectSoundCreate8<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguiddevice: *const ::windows::core::GUID, ppds8: *mut ::core::option::Option<IDirectSound8>, punkouter: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCreate8(pcguiddevice: *const ::windows::core::GUID, ppds8: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundCreate8(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppds8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundEnumerateA(pdsenumcallback: *mut ::core::ffi::c_void, pcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundEnumerateA(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundEnumerateW(pdsenumcallback: *mut ::core::ffi::c_void, pcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundEnumerateW(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundFullDuplexCreate<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param9: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguidcapturedevice: *const ::windows::core::GUID, pcguidrenderdevice: *const ::windows::core::GUID, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: Param4, dwlevel: u32, ppdsfd: *mut ::core::option::Option<IDirectSoundFullDuplex>, ppdscbuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>, ppdsbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>, punkouter: Param9) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundFullDuplexCreate(pcguidcapturedevice: *const ::windows::core::GUID, pcguidrenderdevice: *const ::windows::core::GUID, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, ppdsfd: *mut *mut ::core::ffi::c_void, ppdscbuffer8: *mut *mut ::core::ffi::c_void, ppdsbuffer8: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundFullDuplexCreate(::core::mem::transmute(pcguidcapturedevice), ::core::mem::transmute(pcguidrenderdevice), ::core::mem::transmute(pcdscbufferdesc), ::core::mem::transmute(pcdsbufferdesc), hwnd.into_param().abi(), ::core::mem::transmute(dwlevel), ::core::mem::transmute(ppdsfd), ::core::mem::transmute(ppdscbuffer8), ::core::mem::transmute(ppdsbuffer8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GUID_All_Objects: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa114de5_c262_4169_a1c8_23d698cc73b5);
pub const GUID_DSCFX_CLASS_AEC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const GUID_DSCFX_CLASS_NS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const GUID_DSCFX_MS_AEC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdebb919_379a_488a_8765_f53cfd36de40);
pub const GUID_DSCFX_MS_NS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c5c73b_66e9_4ba1_a0ba_e814c6eed92d);
pub const GUID_DSCFX_SYSTEM_AEC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c22c56d_9879_4f5b_a389_27996ddc2810);
pub const GUID_DSCFX_SYSTEM_NS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ab0882e_7274_4516_877d_4eee99ba4fd0);
pub const GUID_DSFX_STANDARD_CHORUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefe6629c_81f7_4281_bd91_c9d604a95af6);
pub const GUID_DSFX_STANDARD_COMPRESSOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef011f79_4000_406d_87af_bffb3fc39d57);
pub const GUID_DSFX_STANDARD_DISTORTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef114c90_cd1d_484e_96e5_09cfaf912a21);
pub const GUID_DSFX_STANDARD_ECHO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef3e932c_d40b_4f51_8ccf_3f98f1b29d5d);
pub const GUID_DSFX_STANDARD_FLANGER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefca3d92_dfd8_4672_a603_7420894bad98);
pub const GUID_DSFX_STANDARD_GARGLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdafd8210_5711_4b91_9fe3_f75b7ae279bf);
pub const GUID_DSFX_STANDARD_I3DL2REVERB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef985e71_d5c7_42d4_ba4d_2d073e2e96f4);
pub const GUID_DSFX_STANDARD_PARAMEQ: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x120ced89_3bf4_4173_a132_3cb406cf3231);
pub const GUID_DSFX_WAVES_REVERB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87fc0268_9a55_4360_95aa_004a1d9de26c);
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[inline]
pub unsafe fn GetDeviceID(pguidsrc: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceID(pguidsrc: *const ::windows::core::GUID, pguiddest: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::windows::core::GUID>::zeroed();
        GetDeviceID(::core::mem::transmute(pguidsrc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSound(::windows::core::IUnknown);
impl IDirectSound {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn CreateSoundBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateSoundBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsbufferdesc), ::core::mem::transmute(ppdsbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<DSCAPS>::zeroed();
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSCAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn DuplicateSoundBuffer<'a, Param0: ::windows::core::IntoParam<'a, IDirectSoundBuffer>>(&self, pdsbufferoriginal: Param0) -> ::windows::core::Result<IDirectSoundBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows::core::Interface::vtable(self).DuplicateSoundBuffer)(::windows::core::Interface::as_raw(self), pdsbufferoriginal.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDirectSoundBuffer>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwlevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCooperativeLevel)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(dwlevel)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Compact)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetSpeakerConfig(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetSpeakerConfig)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSpeakerConfig)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwspeakerconfig)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
}
impl ::core::convert::From<IDirectSound> for ::windows::core::IUnknown {
    fn from(value: IDirectSound) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSound> for ::windows::core::IUnknown {
    fn from(value: &IDirectSound) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSound {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSound {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSound {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSound {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSound {}
impl ::core::fmt::Debug for IDirectSound {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSound").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSound {
    type Vtable = IDirectSound_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x279afa83_4981_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateSoundBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdscaps: *mut DSCAPS) -> ::windows::core::HRESULT,
    pub DuplicateSoundBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsbufferoriginal: *mut ::core::ffi::c_void, ppdsbufferduplicate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub Compact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSpeakerConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwspeakerconfig: *mut u32) -> ::windows::core::HRESULT,
    pub SetSpeakerConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwspeakerconfig: u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSound3DBuffer(::windows::core::IUnknown);
impl IDirectSound3DBuffer {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DS3DBUFFER> {
        let mut result__ = ::core::mem::MaybeUninit::<DS3DBUFFER>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DS3DBUFFER>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConeAngles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwinsideconeangle), ::core::mem::transmute(pdwoutsideconeangle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetConeOrientation(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Direct3D::D3DVECTOR>::zeroed();
        (::windows::core::Interface::vtable(self).GetConeOrientation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetConeOutsideVolume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows::core::Interface::vtable(self).GetConeOutsideVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetMaxDistance(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxDistance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetMinDistance(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows::core::Interface::vtable(self).GetMinDistance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetMode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetMode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPosition(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Direct3D::D3DVECTOR>::zeroed();
        (::windows::core::Interface::vtable(self).GetPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetVelocity(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Direct3D::D3DVECTOR>::zeroed();
        (::windows::core::Interface::vtable(self).GetVelocity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcds3dbuffer), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConeAngles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwinsideconeangle), ::core::mem::transmute(dwoutsideconeangle), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConeOrientation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConeOutsideVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lconeoutsidevolume), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxDistance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(flmaxdistance), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinDistance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(flmindistance), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetMode(&self, dwmode: u32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwmode), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVelocity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
}
impl ::core::convert::From<IDirectSound3DBuffer> for ::windows::core::IUnknown {
    fn from(value: IDirectSound3DBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSound3DBuffer> for ::windows::core::IUnknown {
    fn from(value: &IDirectSound3DBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSound3DBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSound3DBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSound3DBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSound3DBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSound3DBuffer {}
impl ::core::fmt::Debug for IDirectSound3DBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSound3DBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSound3DBuffer {
    type Vtable = IDirectSound3DBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x279afa86_4981_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DBuffer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pds3dbuffer: *mut DS3DBUFFER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetAllParameters: usize,
    pub GetConeAngles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetConeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetConeOrientation: usize,
    pub GetConeOutsideVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plconeoutsidevolume: *mut i32) -> ::windows::core::HRESULT,
    pub GetMaxDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflmaxdistance: *mut f32) -> ::windows::core::HRESULT,
    pub GetMinDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflmindistance: *mut f32) -> ::windows::core::HRESULT,
    pub GetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmode: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPosition: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetVelocity: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetAllParameters: usize,
    pub SetConeAngles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetConeOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetConeOutsideVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetMaxDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flmaxdistance: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetMinDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flmindistance: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmode: u32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSound3DListener(::windows::core::IUnknown);
impl IDirectSound3DListener {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DS3DLISTENER> {
        let mut result__ = ::core::mem::MaybeUninit::<DS3DLISTENER>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DS3DLISTENER>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetDistanceFactor(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows::core::Interface::vtable(self).GetDistanceFactor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetDopplerFactor(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows::core::Interface::vtable(self).GetDopplerFactor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOrientation(&self, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOrientation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvorientfront), ::core::mem::transmute(pvorienttop)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPosition(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Direct3D::D3DVECTOR>::zeroed();
        (::windows::core::Interface::vtable(self).GetPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetRolloffFactor(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
        (::windows::core::Interface::vtable(self).GetRolloffFactor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetVelocity(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Graphics::Direct3D::D3DVECTOR>::zeroed();
        (::windows::core::Interface::vtable(self).GetVelocity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pclistener), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDistanceFactor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fldistancefactor), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDopplerFactor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fldopplerfactor), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOrientation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(xfront), ::core::mem::transmute(yfront), ::core::mem::transmute(zfront), ::core::mem::transmute(xtop), ::core::mem::transmute(ytop), ::core::mem::transmute(ztop), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRolloffFactor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(flrollofffactor), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVelocity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn CommitDeferredSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CommitDeferredSettings)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDirectSound3DListener> for ::windows::core::IUnknown {
    fn from(value: IDirectSound3DListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSound3DListener> for ::windows::core::IUnknown {
    fn from(value: &IDirectSound3DListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSound3DListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSound3DListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSound3DListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSound3DListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSound3DListener {}
impl ::core::fmt::Debug for IDirectSound3DListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSound3DListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSound3DListener {
    type Vtable = IDirectSound3DListener_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x279afa84_4981_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DListener_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plistener: *mut DS3DLISTENER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetAllParameters: usize,
    pub GetDistanceFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfldistancefactor: *mut f32) -> ::windows::core::HRESULT,
    pub GetDopplerFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfldopplerfactor: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetOrientation: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetPosition: usize,
    pub GetRolloffFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflrollofffactor: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetVelocity: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetAllParameters: usize,
    pub SetDistanceFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fldistancefactor: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetDopplerFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fldopplerfactor: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetRolloffFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flrollofffactor: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub SetVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub CommitDeferredSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSound8(::windows::core::IUnknown);
impl IDirectSound8 {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn CreateSoundBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CreateSoundBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsbufferdesc), ::core::mem::transmute(ppdsbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<DSCAPS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSCAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn DuplicateSoundBuffer<'a, Param0: ::windows::core::IntoParam<'a, IDirectSoundBuffer>>(&self, pdsbufferoriginal: Param0) -> ::windows::core::Result<IDirectSoundBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows::core::Interface::vtable(self).base__.DuplicateSoundBuffer)(::windows::core::Interface::as_raw(self), pdsbufferoriginal.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDirectSoundBuffer>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwlevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCooperativeLevel)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), ::core::mem::transmute(dwlevel)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Compact)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetSpeakerConfig(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetSpeakerConfig)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSpeakerConfig)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwspeakerconfig)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn VerifyCertification(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).VerifyCertification)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDirectSound8> for ::windows::core::IUnknown {
    fn from(value: IDirectSound8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSound8> for ::windows::core::IUnknown {
    fn from(value: &IDirectSound8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSound8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSound8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectSound8> for IDirectSound {
    fn from(value: IDirectSound8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSound8> for IDirectSound {
    fn from(value: &IDirectSound8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectSound> for IDirectSound8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSound> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectSound> for &'a IDirectSound8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSound> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSound8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSound8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSound8 {}
impl ::core::fmt::Debug for IDirectSound8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSound8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSound8 {
    type Vtable = IDirectSound8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc50a7e93_f395_4834_9ef6_7fa99de50966);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound8_Vtbl {
    pub base__: IDirectSound_Vtbl,
    pub VerifyCertification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcertified: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundBuffer(::windows::core::IUnknown);
impl IDirectSoundBuffer {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSBCAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<DSBCAPS>::zeroed();
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSBCAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwcurrentplaycursor), ::core::mem::transmute(pdwcurrentwritecursor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetVolume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows::core::Interface::vtable(self).GetVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetPan(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows::core::Interface::vtable(self).GetPan)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetFrequency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetFrequency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IDirectSound>>(&self, pdirectsound: Param0, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pdirectsound.into_param().abi(), ::core::mem::transmute(pcdsbufferdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Play)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwreserved1), ::core::mem::transmute(dwpriority), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCurrentPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwnewposition)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcfxformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lvolume)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetPan(&self, lpan: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPan)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpan)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFrequency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwfrequency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDirectSoundBuffer> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundBuffer> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundBuffer {}
impl ::core::fmt::Debug for IDirectSoundBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundBuffer {
    type Vtable = IDirectSoundBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x279afa85_4981_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsbuffercaps: *mut DSBCAPS) -> ::windows::core::HRESULT,
    pub GetCurrentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub GetPan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpan: *mut i32) -> ::windows::core::HRESULT,
    pub GetFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfrequency: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub SetCurrentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwnewposition: u32) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT,
    pub SetPan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpan: i32) -> ::windows::core::HRESULT,
    pub SetFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfrequency: u32) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundBuffer8(::windows::core::IUnknown);
impl IDirectSoundBuffer8 {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSBCAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<DSBCAPS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSBCAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCurrentPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwcurrentplaycursor), ::core::mem::transmute(pdwcurrentwritecursor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetVolume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetPan(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPan)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetFrequency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetFrequency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IDirectSound>>(&self, pdirectsound: Param0, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), pdirectsound.into_param().abi(), ::core::mem::transmute(pcdsbufferdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Lock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Play)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwreserved1), ::core::mem::transmute(dwpriority), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCurrentPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwnewposition)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcfxformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lvolume)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetPan(&self, lpan: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPan)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lpan)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFrequency)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwfrequency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Unlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Restore)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetFX(&self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFX)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dweffectscount), ::core::mem::transmute(pdsfxdesc), ::core::mem::transmute(pdwresultcodes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn AcquireResources(&self, dwflags: u32, pdwresultcodes: &mut [u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireResources)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwflags), pdwresultcodes.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pdwresultcodes))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectInPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rguidobject), ::core::mem::transmute(dwindex), ::core::mem::transmute(rguidinterface), ::core::mem::transmute(ppobject)).ok()
    }
}
impl ::core::convert::From<IDirectSoundBuffer8> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundBuffer8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundBuffer8> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundBuffer8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectSoundBuffer8> for IDirectSoundBuffer {
    fn from(value: IDirectSoundBuffer8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundBuffer8> for IDirectSoundBuffer {
    fn from(value: &IDirectSoundBuffer8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectSoundBuffer> for IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSoundBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectSoundBuffer> for &'a IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSoundBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundBuffer8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundBuffer8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundBuffer8 {}
impl ::core::fmt::Debug for IDirectSoundBuffer8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundBuffer8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundBuffer8 {
    type Vtable = IDirectSoundBuffer8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6825a449_7524_4d82_920f_50e36ab3ab1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer8_Vtbl {
    pub base__: IDirectSoundBuffer_Vtbl,
    pub SetFX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT,
    pub AcquireResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT,
    pub GetObjectInPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundCapture(::windows::core::IUnknown);
impl IDirectSoundCapture {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn CreateCaptureBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::core::option::Option<IDirectSoundCaptureBuffer>, punkouter: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateCaptureBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdscbufferdesc), ::core::mem::transmute(ppdscbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCCAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<DSCCAPS>::zeroed();
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSCCAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
}
impl ::core::convert::From<IDirectSoundCapture> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundCapture> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundCapture {}
impl ::core::fmt::Debug for IDirectSoundCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundCapture").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCapture {
    type Vtable = IDirectSoundCapture_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0210781_89cd_11d0_af08_00a0c925cd16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCapture_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateCaptureBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsccaps: *mut DSCCAPS) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundCaptureBuffer(::windows::core::IUnknown);
impl IDirectSoundCaptureBuffer {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCBCAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<DSCBCAPS>::zeroed();
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSCBCAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwcaptureposition), ::core::mem::transmute(pdwreadposition)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IDirectSoundCapture>>(&self, pdirectsoundcapture: Param0, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pdirectsoundcapture.into_param().abi(), ::core::mem::transmute(pcdscbufferdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Start(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
}
impl ::core::convert::From<IDirectSoundCaptureBuffer> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCaptureBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundCaptureBuffer> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCaptureBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCaptureBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCaptureBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundCaptureBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundCaptureBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundCaptureBuffer {}
impl ::core::fmt::Debug for IDirectSoundCaptureBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundCaptureBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCaptureBuffer {
    type Vtable = IDirectSoundCaptureBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0210782_89cd_11d0_af08_00a0c925cd16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdscbcaps: *mut DSCBCAPS) -> ::windows::core::HRESULT,
    pub GetCurrentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdirectsoundcapture: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::HRESULT,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundCaptureBuffer8(::windows::core::IUnknown);
impl IDirectSoundCaptureBuffer8 {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCBCAPS> {
        let mut result__ = ::core::mem::MaybeUninit::<DSCBCAPS>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCaps)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSCBCAPS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCurrentPosition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwcaptureposition), ::core::mem::transmute(pdwreadposition)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IDirectSoundCapture>>(&self, pdirectsoundcapture: Param0, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), pdirectsoundcapture.into_param().abi(), ::core::mem::transmute(pcdscbufferdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Lock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Start(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Start)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Unlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectInPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rguidobject), ::core::mem::transmute(dwindex), ::core::mem::transmute(rguidinterface), ::core::mem::transmute(ppobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetFXStatus(&self, pdwfxstatus: &mut [u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFXStatus)(::windows::core::Interface::as_raw(self), pdwfxstatus.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pdwfxstatus))).ok()
    }
}
impl ::core::convert::From<IDirectSoundCaptureBuffer8> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCaptureBuffer8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundCaptureBuffer8> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCaptureBuffer8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirectSoundCaptureBuffer8> for IDirectSoundCaptureBuffer {
    fn from(value: IDirectSoundCaptureBuffer8) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundCaptureBuffer8> for IDirectSoundCaptureBuffer {
    fn from(value: &IDirectSoundCaptureBuffer8) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectSoundCaptureBuffer> for IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSoundCaptureBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDirectSoundCaptureBuffer> for &'a IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSoundCaptureBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundCaptureBuffer8 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundCaptureBuffer8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundCaptureBuffer8 {}
impl ::core::fmt::Debug for IDirectSoundCaptureBuffer8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundCaptureBuffer8").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCaptureBuffer8 {
    type Vtable = IDirectSoundCaptureBuffer8_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00990df4_0dbb_4872_833e_6d303e80aeb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer8_Vtbl {
    pub base__: IDirectSoundCaptureBuffer_Vtbl,
    pub GetObjectInPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFXStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundCaptureFXAec(::windows::core::IUnknown);
impl IDirectSoundCaptureFXAec {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdscfxaec)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSCFXAec> {
        let mut result__ = ::core::mem::MaybeUninit::<DSCFXAec>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSCFXAec>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDirectSoundCaptureFXAec> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCaptureFXAec) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundCaptureFXAec> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCaptureFXAec) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCaptureFXAec {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCaptureFXAec {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundCaptureFXAec {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundCaptureFXAec {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundCaptureFXAec {}
impl ::core::fmt::Debug for IDirectSoundCaptureFXAec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundCaptureFXAec").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCaptureFXAec {
    type Vtable = IDirectSoundCaptureFXAec_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad74143d_903d_4ab7_8066_28d363036d65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXAec_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdscfxaec: *const DSCFXAec) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllParameters: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdscfxaec: *mut DSCFXAec) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllParameters: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundCaptureFXNoiseSuppress(::windows::core::IUnknown);
impl IDirectSoundCaptureFXNoiseSuppress {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdscfxnoisesuppress)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSCFXNoiseSuppress> {
        let mut result__ = ::core::mem::MaybeUninit::<DSCFXNoiseSuppress>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSCFXNoiseSuppress>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDirectSoundCaptureFXNoiseSuppress> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCaptureFXNoiseSuppress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundCaptureFXNoiseSuppress> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCaptureFXNoiseSuppress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCaptureFXNoiseSuppress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCaptureFXNoiseSuppress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundCaptureFXNoiseSuppress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundCaptureFXNoiseSuppress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundCaptureFXNoiseSuppress {}
impl ::core::fmt::Debug for IDirectSoundCaptureFXNoiseSuppress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundCaptureFXNoiseSuppress").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCaptureFXNoiseSuppress {
    type Vtable = IDirectSoundCaptureFXNoiseSuppress_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed311e41_fbae_4175_9625_cd0854f693ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXNoiseSuppress_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllParameters: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllParameters: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXChorus(::windows::core::IUnknown);
impl IDirectSoundFXChorus {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxchorus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXChorus> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXChorus>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXChorus>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXChorus> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXChorus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXChorus> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXChorus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXChorus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXChorus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXChorus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXChorus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXChorus {}
impl ::core::fmt::Debug for IDirectSoundFXChorus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXChorus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXChorus {
    type Vtable = IDirectSoundFXChorus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x880842e3_145f_43e6_a934_a71806e50547);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXChorus_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxchorus: *const DSFXChorus) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxchorus: *mut DSFXChorus) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXCompressor(::windows::core::IUnknown);
impl IDirectSoundFXCompressor {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxcompressor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXCompressor> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXCompressor>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXCompressor>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXCompressor> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXCompressor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXCompressor> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXCompressor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXCompressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXCompressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXCompressor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXCompressor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXCompressor {}
impl ::core::fmt::Debug for IDirectSoundFXCompressor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXCompressor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXCompressor {
    type Vtable = IDirectSoundFXCompressor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bbd1154_62f6_4e2c_a15c_d3b6c417f7a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXCompressor_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxcompressor: *mut DSFXCompressor) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXDistortion(::windows::core::IUnknown);
impl IDirectSoundFXDistortion {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxdistortion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXDistortion> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXDistortion>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXDistortion>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXDistortion> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXDistortion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXDistortion> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXDistortion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXDistortion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXDistortion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXDistortion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXDistortion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXDistortion {}
impl ::core::fmt::Debug for IDirectSoundFXDistortion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXDistortion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXDistortion {
    type Vtable = IDirectSoundFXDistortion_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ecf4326_455f_4d8b_bda9_8d5d3e9e3e0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXDistortion_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxdistortion: *mut DSFXDistortion) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXEcho(::windows::core::IUnknown);
impl IDirectSoundFXEcho {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxecho)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXEcho> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXEcho>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXEcho>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXEcho> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXEcho) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXEcho> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXEcho) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXEcho {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXEcho {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXEcho {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXEcho {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXEcho {}
impl ::core::fmt::Debug for IDirectSoundFXEcho {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXEcho").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXEcho {
    type Vtable = IDirectSoundFXEcho_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bd28edf_50db_4e92_a2bd_445488d1ed42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXEcho_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxecho: *const DSFXEcho) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxecho: *mut DSFXEcho) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXFlanger(::windows::core::IUnknown);
impl IDirectSoundFXFlanger {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxflanger)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXFlanger> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXFlanger>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXFlanger>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXFlanger> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXFlanger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXFlanger> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXFlanger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXFlanger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXFlanger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXFlanger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXFlanger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXFlanger {}
impl ::core::fmt::Debug for IDirectSoundFXFlanger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXFlanger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXFlanger {
    type Vtable = IDirectSoundFXFlanger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x903e9878_2c92_4072_9b2c_ea68f5396783);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXFlanger_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxflanger: *const DSFXFlanger) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxflanger: *mut DSFXFlanger) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXGargle(::windows::core::IUnknown);
impl IDirectSoundFXGargle {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxgargle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXGargle> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXGargle>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXGargle>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXGargle> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXGargle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXGargle> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXGargle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXGargle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXGargle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXGargle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXGargle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXGargle {}
impl ::core::fmt::Debug for IDirectSoundFXGargle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXGargle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXGargle {
    type Vtable = IDirectSoundFXGargle_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd616f352_d622_11ce_aac5_0020af0b99a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXGargle_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxgargle: *const DSFXGargle) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxgargle: *mut DSFXGargle) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXI3DL2Reverb(::windows::core::IUnknown);
impl IDirectSoundFXI3DL2Reverb {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxi3dl2reverb)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXI3DL2Reverb> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXI3DL2Reverb>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXI3DL2Reverb>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetPreset(&self, dwpreset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPreset)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(dwpreset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetPreset(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetPreset)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetQuality(&self, lquality: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuality)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lquality)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetQuality(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows::core::Interface::vtable(self).GetQuality)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXI3DL2Reverb> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXI3DL2Reverb) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXI3DL2Reverb> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXI3DL2Reverb) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXI3DL2Reverb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXI3DL2Reverb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXI3DL2Reverb {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXI3DL2Reverb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXI3DL2Reverb {}
impl ::core::fmt::Debug for IDirectSoundFXI3DL2Reverb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXI3DL2Reverb").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXI3DL2Reverb {
    type Vtable = IDirectSoundFXI3DL2Reverb_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b166a6a_0d66_43f3_80e3_ee6280dee1a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXI3DL2Reverb_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows::core::HRESULT,
    pub SetPreset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpreset: u32) -> ::windows::core::HRESULT,
    pub GetPreset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpreset: *mut u32) -> ::windows::core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lquality: i32) -> ::windows::core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plquality: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXParamEq(::windows::core::IUnknown);
impl IDirectSoundFXParamEq {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxparameq)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXParamEq> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXParamEq>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXParamEq>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXParamEq> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXParamEq) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXParamEq> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXParamEq) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXParamEq {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXParamEq {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXParamEq {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXParamEq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXParamEq {}
impl ::core::fmt::Debug for IDirectSoundFXParamEq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXParamEq").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXParamEq {
    type Vtable = IDirectSoundFXParamEq_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc03ca9fe_fe90_4204_8078_82334cd177da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXParamEq_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxparameq: *const DSFXParamEq) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxparameq: *mut DSFXParamEq) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFXWavesReverb(::windows::core::IUnknown);
impl IDirectSoundFXWavesReverb {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdsfxwavesreverb)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXWavesReverb> {
        let mut result__ = ::core::mem::MaybeUninit::<DSFXWavesReverb>::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DSFXWavesReverb>(result__)
    }
}
impl ::core::convert::From<IDirectSoundFXWavesReverb> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXWavesReverb) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFXWavesReverb> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXWavesReverb) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXWavesReverb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXWavesReverb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFXWavesReverb {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFXWavesReverb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFXWavesReverb {}
impl ::core::fmt::Debug for IDirectSoundFXWavesReverb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFXWavesReverb").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXWavesReverb {
    type Vtable = IDirectSoundFXWavesReverb_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46858c3a_0dc6_45e3_b760_d4eef16cb325);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXWavesReverb_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundFullDuplex(::windows::core::IUnknown);
impl IDirectSoundFullDuplex {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, pcaptureguid: *const ::windows::core::GUID, prenderguid: *const ::windows::core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: Param4, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcaptureguid), ::core::mem::transmute(prenderguid), ::core::mem::transmute(lpdscbufferdesc), ::core::mem::transmute(lpdsbufferdesc), hwnd.into_param().abi(), ::core::mem::transmute(dwlevel), ::core::mem::transmute(lplpdirectsoundcapturebuffer8), ::core::mem::transmute(lplpdirectsoundbuffer8)).ok()
    }
}
impl ::core::convert::From<IDirectSoundFullDuplex> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFullDuplex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundFullDuplex> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFullDuplex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFullDuplex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFullDuplex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundFullDuplex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundFullDuplex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundFullDuplex {}
impl ::core::fmt::Debug for IDirectSoundFullDuplex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundFullDuplex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFullDuplex {
    type Vtable = IDirectSoundFullDuplex_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedcb4c7a_daab_4216_a42e_6c50596ddc1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFullDuplex_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcaptureguid: *const ::windows::core::GUID, prenderguid: *const ::windows::core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut *mut ::core::ffi::c_void, lplpdirectsoundbuffer8: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
#[repr(transparent)]
pub struct IDirectSoundNotify(::windows::core::IUnknown);
impl IDirectSoundNotify {
    #[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotificationPositions(&self, pcpositionnotifies: &[DSBPOSITIONNOTIFY]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNotificationPositions)(::windows::core::Interface::as_raw(self), pcpositionnotifies.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pcpositionnotifies))).ok()
    }
}
impl ::core::convert::From<IDirectSoundNotify> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirectSoundNotify> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDirectSoundNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirectSoundNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectSoundNotify {}
impl ::core::fmt::Debug for IDirectSoundNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectSoundNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundNotify {
    type Vtable = IDirectSoundNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0210783_89cd_11d0_af08_00a0c925cd16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundNotify_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotificationPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotificationPositions: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const KSPROPERTY_SUPPORT_GET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const KSPROPERTY_SUPPORT_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::windows::core::GUID, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::windows::core::GUID, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`*"]
pub const _FACDS: u32 = 2168u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
