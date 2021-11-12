#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_DirectSound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47d4d946_62e8_11cf_93bc_444553540000);
pub const CLSID_DirectSound8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3901cc3f_84b5_4fa4_ba35_aa8172b8a09b);
pub const CLSID_DirectSoundCapture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0210780_89cd_11d0_af08_00a0c925cd16);
pub const CLSID_DirectSoundCapture8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4bcac13_7f99_4908_9a8e_74e3bf24b6e1);
pub const CLSID_DirectSoundFullDuplex: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea4300c_7959_4147_b26a_2377b9e7a91d);
pub const DIRECTSOUND_VERSION: u32 = 1792u32;
pub const DS3DALG_HRTF_FULL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2413340_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_HRTF_LIGHT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2413342_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_NO_VIRTUALIZATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc241333f_1c1b_11d2_94f5_00c04fc28aca);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl DS3DBUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for DS3DBUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for DS3DBUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DS3DBUFFER")
            .field("dwSize", &self.dwSize)
            .field("vPosition", &self.vPosition)
            .field("vVelocity", &self.vVelocity)
            .field("dwInsideConeAngle", &self.dwInsideConeAngle)
            .field("dwOutsideConeAngle", &self.dwOutsideConeAngle)
            .field("vConeOrientation", &self.vConeOrientation)
            .field("lConeOutsideVolume", &self.lConeOutsideVolume)
            .field("flMinDistance", &self.flMinDistance)
            .field("flMaxDistance", &self.flMaxDistance)
            .field("dwMode", &self.dwMode)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for DS3DBUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.vPosition == other.vPosition && self.vVelocity == other.vVelocity && self.dwInsideConeAngle == other.dwInsideConeAngle && self.dwOutsideConeAngle == other.dwOutsideConeAngle && self.vConeOrientation == other.vConeOrientation && self.lConeOutsideVolume == other.lConeOutsideVolume && self.flMinDistance == other.flMinDistance && self.flMaxDistance == other.flMaxDistance && self.dwMode == other.dwMode
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for DS3DBUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for DS3DBUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl DS3DLISTENER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for DS3DLISTENER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for DS3DLISTENER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DS3DLISTENER")
            .field("dwSize", &self.dwSize)
            .field("vPosition", &self.vPosition)
            .field("vVelocity", &self.vVelocity)
            .field("vOrientFront", &self.vOrientFront)
            .field("vOrientTop", &self.vOrientTop)
            .field("flDistanceFactor", &self.flDistanceFactor)
            .field("flRolloffFactor", &self.flRolloffFactor)
            .field("flDopplerFactor", &self.flDopplerFactor)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for DS3DLISTENER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.vPosition == other.vPosition && self.vVelocity == other.vVelocity && self.vOrientFront == other.vOrientFront && self.vOrientTop == other.vOrientTop && self.flDistanceFactor == other.flDistanceFactor && self.flRolloffFactor == other.flRolloffFactor && self.flDopplerFactor == other.flDopplerFactor
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for DS3DLISTENER {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
unsafe impl ::windows::core::Abi for DS3DLISTENER {
    type Abi = Self;
}
pub const DS3DMODE_DISABLE: u32 = 2u32;
pub const DS3DMODE_HEADRELATIVE: u32 = 1u32;
pub const DS3DMODE_NORMAL: u32 = 0u32;
pub const DS3D_DEFAULTCONEANGLE: u32 = 360u32;
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: u32 = 0u32;
pub const DS3D_DEFAULTDISTANCEFACTOR: f32 = 1f32;
pub const DS3D_DEFAULTDOPPLERFACTOR: f32 = 1f32;
pub const DS3D_DEFAULTMAXDISTANCE: f32 = 1000000000f32;
pub const DS3D_DEFAULTMINDISTANCE: f32 = 1f32;
pub const DS3D_DEFAULTROLLOFFFACTOR: f32 = 1f32;
pub const DS3D_DEFERRED: u32 = 1u32;
pub const DS3D_IMMEDIATE: u32 = 0u32;
pub const DS3D_MAXCONEANGLE: u32 = 360u32;
pub const DS3D_MAXDOPPLERFACTOR: f32 = 10f32;
pub const DS3D_MAXROLLOFFFACTOR: f32 = 10f32;
pub const DS3D_MINCONEANGLE: u32 = 0u32;
pub const DS3D_MINDOPPLERFACTOR: f32 = 0f32;
pub const DS3D_MINROLLOFFFACTOR: f32 = 0f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwUnlockTransferRate: u32,
    pub dwPlayCpuOverhead: u32,
}
impl DSBCAPS {}
impl ::core::default::Default for DSBCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSBCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSBCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwUnlockTransferRate", &self.dwUnlockTransferRate).field("dwPlayCpuOverhead", &self.dwPlayCpuOverhead).finish()
    }
}
impl ::core::cmp::PartialEq for DSBCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwUnlockTransferRate == other.dwUnlockTransferRate && self.dwPlayCpuOverhead == other.dwPlayCpuOverhead
    }
}
impl ::core::cmp::Eq for DSBCAPS {}
unsafe impl ::windows::core::Abi for DSBCAPS {
    type Abi = Self;
}
pub const DSBCAPS_CTRL3D: u32 = 16u32;
pub const DSBCAPS_CTRLFREQUENCY: u32 = 32u32;
pub const DSBCAPS_CTRLFX: u32 = 512u32;
pub const DSBCAPS_CTRLPAN: u32 = 64u32;
pub const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 256u32;
pub const DSBCAPS_CTRLVOLUME: u32 = 128u32;
pub const DSBCAPS_GETCURRENTPOSITION2: u32 = 65536u32;
pub const DSBCAPS_GLOBALFOCUS: u32 = 32768u32;
pub const DSBCAPS_LOCDEFER: u32 = 262144u32;
pub const DSBCAPS_LOCHARDWARE: u32 = 4u32;
pub const DSBCAPS_LOCSOFTWARE: u32 = 8u32;
pub const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 131072u32;
pub const DSBCAPS_PRIMARYBUFFER: u32 = 1u32;
pub const DSBCAPS_STATIC: u32 = 2u32;
pub const DSBCAPS_STICKYFOCUS: u32 = 16384u32;
pub const DSBCAPS_TRUEPLAYPOSITION: u32 = 524288u32;
pub const DSBFREQUENCY_MAX: u32 = 200000u32;
pub const DSBFREQUENCY_MIN: u32 = 100u32;
pub const DSBFREQUENCY_ORIGINAL: u32 = 0u32;
pub const DSBLOCK_ENTIREBUFFER: u32 = 2u32;
pub const DSBLOCK_FROMWRITECURSOR: u32 = 1u32;
pub const DSBNOTIFICATIONS_MAX: u32 = 100000u32;
pub const DSBPAN_CENTER: u32 = 0u32;
pub const DSBPAN_LEFT: i32 = -10000i32;
pub const DSBPAN_RIGHT: u32 = 10000u32;
pub const DSBPLAY_LOCHARDWARE: u32 = 2u32;
pub const DSBPLAY_LOCSOFTWARE: u32 = 4u32;
pub const DSBPLAY_LOOPING: u32 = 1u32;
pub const DSBPLAY_TERMINATEBY_DISTANCE: u64 = 16u64;
pub const DSBPLAY_TERMINATEBY_PRIORITY: u64 = 32u64;
pub const DSBPLAY_TERMINATEBY_TIME: u32 = 8u32;
pub const DSBPN_OFFSETSTOP: u32 = 4294967295u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: u32,
    pub hEventNotify: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl DSBPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSBPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSBPOSITIONNOTIFY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSBPOSITIONNOTIFY").field("dwOffset", &self.dwOffset).field("hEventNotify", &self.hEventNotify).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSBPOSITIONNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.dwOffset == other.dwOffset && self.hEventNotify == other.hEventNotify
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSBPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSBPOSITIONNOTIFY {
    type Abi = Self;
}
pub const DSBSIZE_FX_MIN: u32 = 150u32;
pub const DSBSIZE_MAX: u32 = 268435455u32;
pub const DSBSIZE_MIN: u32 = 4u32;
pub const DSBSTATUS_BUFFERLOST: u32 = 2u32;
pub const DSBSTATUS_LOCHARDWARE: u32 = 8u32;
pub const DSBSTATUS_LOCSOFTWARE: u32 = 16u32;
pub const DSBSTATUS_LOOPING: u32 = 4u32;
pub const DSBSTATUS_PLAYING: u32 = 1u32;
pub const DSBSTATUS_TERMINATED: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub guid3DAlgorithm: ::windows::core::GUID,
}
impl DSBUFFERDESC {}
impl ::core::default::Default for DSBUFFERDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSBUFFERDESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSBUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).field("guid3DAlgorithm", &self.guid3DAlgorithm).finish()
    }
}
impl ::core::cmp::PartialEq for DSBUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved && self.lpwfxFormat == other.lpwfxFormat && self.guid3DAlgorithm == other.guid3DAlgorithm
    }
}
impl ::core::cmp::Eq for DSBUFFERDESC {}
unsafe impl ::windows::core::Abi for DSBUFFERDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
}
impl DSBUFFERDESC1 {}
impl ::core::default::Default for DSBUFFERDESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSBUFFERDESC1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSBUFFERDESC1").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).finish()
    }
}
impl ::core::cmp::PartialEq for DSBUFFERDESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved && self.lpwfxFormat == other.lpwfxFormat
    }
}
impl ::core::cmp::Eq for DSBUFFERDESC1 {}
unsafe impl ::windows::core::Abi for DSBUFFERDESC1 {
    type Abi = Self;
}
pub const DSBVOLUME_MAX: u32 = 0u32;
pub const DSBVOLUME_MIN: i32 = -10000i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl DSCAPS {}
impl ::core::default::Default for DSCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSCAPS")
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
impl ::core::cmp::PartialEq for DSCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.dwMinSecondarySampleRate == other.dwMinSecondarySampleRate
            && self.dwMaxSecondarySampleRate == other.dwMaxSecondarySampleRate
            && self.dwPrimaryBuffers == other.dwPrimaryBuffers
            && self.dwMaxHwMixingAllBuffers == other.dwMaxHwMixingAllBuffers
            && self.dwMaxHwMixingStaticBuffers == other.dwMaxHwMixingStaticBuffers
            && self.dwMaxHwMixingStreamingBuffers == other.dwMaxHwMixingStreamingBuffers
            && self.dwFreeHwMixingAllBuffers == other.dwFreeHwMixingAllBuffers
            && self.dwFreeHwMixingStaticBuffers == other.dwFreeHwMixingStaticBuffers
            && self.dwFreeHwMixingStreamingBuffers == other.dwFreeHwMixingStreamingBuffers
            && self.dwMaxHw3DAllBuffers == other.dwMaxHw3DAllBuffers
            && self.dwMaxHw3DStaticBuffers == other.dwMaxHw3DStaticBuffers
            && self.dwMaxHw3DStreamingBuffers == other.dwMaxHw3DStreamingBuffers
            && self.dwFreeHw3DAllBuffers == other.dwFreeHw3DAllBuffers
            && self.dwFreeHw3DStaticBuffers == other.dwFreeHw3DStaticBuffers
            && self.dwFreeHw3DStreamingBuffers == other.dwFreeHw3DStreamingBuffers
            && self.dwTotalHwMemBytes == other.dwTotalHwMemBytes
            && self.dwFreeHwMemBytes == other.dwFreeHwMemBytes
            && self.dwMaxContigFreeHwMemBytes == other.dwMaxContigFreeHwMemBytes
            && self.dwUnlockTransferRateHwBuffers == other.dwUnlockTransferRateHwBuffers
            && self.dwPlayCpuOverheadSwBuffers == other.dwPlayCpuOverheadSwBuffers
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for DSCAPS {}
unsafe impl ::windows::core::Abi for DSCAPS {
    type Abi = Self;
}
pub const DSCAPS_CERTIFIED: u32 = 64u32;
pub const DSCAPS_CONTINUOUSRATE: u32 = 16u32;
pub const DSCAPS_EMULDRIVER: u32 = 32u32;
pub const DSCAPS_PRIMARY16BIT: u32 = 8u32;
pub const DSCAPS_PRIMARY8BIT: u32 = 4u32;
pub const DSCAPS_PRIMARYMONO: u32 = 1u32;
pub const DSCAPS_PRIMARYSTEREO: u32 = 2u32;
pub const DSCAPS_SECONDARY16BIT: u32 = 2048u32;
pub const DSCAPS_SECONDARY8BIT: u32 = 1024u32;
pub const DSCAPS_SECONDARYMONO: u32 = 256u32;
pub const DSCAPS_SECONDARYSTEREO: u32 = 512u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSCBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
}
impl DSCBCAPS {}
impl ::core::default::Default for DSCBCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSCBCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSCBCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::cmp::PartialEq for DSCBCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for DSCBCAPS {}
unsafe impl ::windows::core::Abi for DSCBCAPS {
    type Abi = Self;
}
pub const DSCBCAPS_CTRLFX: u32 = 512u32;
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648u32;
pub const DSCBLOCK_ENTIREBUFFER: u32 = 1u32;
pub const DSCBSTART_LOOPING: u32 = 1u32;
pub const DSCBSTATUS_CAPTURING: u32 = 1u32;
pub const DSCBSTATUS_LOOPING: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSCBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub dwFXCount: u32,
    pub lpDSCFXDesc: *mut DSCEFFECTDESC,
}
impl DSCBUFFERDESC {}
impl ::core::default::Default for DSCBUFFERDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSCBUFFERDESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSCBUFFERDESC")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwBufferBytes", &self.dwBufferBytes)
            .field("dwReserved", &self.dwReserved)
            .field("lpwfxFormat", &self.lpwfxFormat)
            .field("dwFXCount", &self.dwFXCount)
            .field("lpDSCFXDesc", &self.lpDSCFXDesc)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DSCBUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved && self.lpwfxFormat == other.lpwfxFormat && self.dwFXCount == other.dwFXCount && self.lpDSCFXDesc == other.lpDSCFXDesc
    }
}
impl ::core::cmp::Eq for DSCBUFFERDESC {}
unsafe impl ::windows::core::Abi for DSCBUFFERDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSCBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
}
impl DSCBUFFERDESC1 {}
impl ::core::default::Default for DSCBUFFERDESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSCBUFFERDESC1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSCBUFFERDESC1").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).finish()
    }
}
impl ::core::cmp::PartialEq for DSCBUFFERDESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved && self.lpwfxFormat == other.lpwfxFormat
    }
}
impl ::core::cmp::Eq for DSCBUFFERDESC1 {}
unsafe impl ::windows::core::Abi for DSCBUFFERDESC1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSCCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFormats: u32,
    pub dwChannels: u32,
}
impl DSCCAPS {}
impl ::core::default::Default for DSCCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSCCAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSCCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwFormats", &self.dwFormats).field("dwChannels", &self.dwChannels).finish()
    }
}
impl ::core::cmp::PartialEq for DSCCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwFormats == other.dwFormats && self.dwChannels == other.dwChannels
    }
}
impl ::core::cmp::Eq for DSCCAPS {}
unsafe impl ::windows::core::Abi for DSCCAPS {
    type Abi = Self;
}
pub const DSCCAPS_CERTIFIED: u32 = 64u32;
pub const DSCCAPS_EMULDRIVER: u32 = 32u32;
pub const DSCCAPS_MULTIPLECAPTURE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSCEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSCFXClass: ::windows::core::GUID,
    pub guidDSCFXInstance: ::windows::core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl DSCEFFECTDESC {}
impl ::core::default::Default for DSCEFFECTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSCEFFECTDESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSCEFFECTDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidDSCFXClass", &self.guidDSCFXClass).field("guidDSCFXInstance", &self.guidDSCFXInstance).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::core::cmp::PartialEq for DSCEFFECTDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidDSCFXClass == other.guidDSCFXClass && self.guidDSCFXInstance == other.guidDSCFXInstance && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for DSCEFFECTDESC {}
unsafe impl ::windows::core::Abi for DSCEFFECTDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DSCFXAec {
    pub fEnable: super::super::super::Foundation::BOOL,
    pub fNoiseFill: super::super::super::Foundation::BOOL,
    pub dwMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DSCFXAec {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSCFXAec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSCFXAec {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSCFXAec").field("fEnable", &self.fEnable).field("fNoiseFill", &self.fNoiseFill).field("dwMode", &self.dwMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSCFXAec {
    fn eq(&self, other: &Self) -> bool {
        self.fEnable == other.fEnable && self.fNoiseFill == other.fNoiseFill && self.dwMode == other.dwMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSCFXAec {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSCFXAec {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DSCFXNoiseSuppress {
    pub fEnable: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DSCFXNoiseSuppress {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DSCFXNoiseSuppress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DSCFXNoiseSuppress {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSCFXNoiseSuppress").field("fEnable", &self.fEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DSCFXNoiseSuppress {
    fn eq(&self, other: &Self) -> bool {
        self.fEnable == other.fEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DSCFXNoiseSuppress {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DSCFXNoiseSuppress {
    type Abi = Self;
}
pub const DSCFXR_LOCHARDWARE: u32 = 16u32;
pub const DSCFXR_LOCSOFTWARE: u32 = 32u32;
pub const DSCFX_AEC_MODE_FULL_DUPLEX: u32 = 2u32;
pub const DSCFX_AEC_MODE_HALF_DUPLEX: u32 = 1u32;
pub const DSCFX_AEC_MODE_PASS_THROUGH: u32 = 0u32;
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: u32 = 8u32;
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: u32 = 0u32;
pub const DSCFX_LOCHARDWARE: u32 = 1u32;
pub const DSCFX_LOCSOFTWARE: u32 = 2u32;
pub const DSDEVID_DefaultCapture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef00001_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultPlayback: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef00000_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoiceCapture: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef00003_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoicePlayback: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdef00002_9c6d_47ed_aaf1_4dda8f2b5c03);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSFXClass: ::windows::core::GUID,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl DSEFFECTDESC {}
impl ::core::default::Default for DSEFFECTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSEFFECTDESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSEFFECTDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidDSFXClass", &self.guidDSFXClass).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::core::cmp::PartialEq for DSEFFECTDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidDSFXClass == other.guidDSFXClass && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for DSEFFECTDESC {}
unsafe impl ::windows::core::Abi for DSEFFECTDESC {
    type Abi = Self;
}
pub const DSFXCHORUS_DELAY_MAX: f32 = 20f32;
pub const DSFXCHORUS_DELAY_MIN: f32 = 0f32;
pub const DSFXCHORUS_DEPTH_MAX: f32 = 100f32;
pub const DSFXCHORUS_DEPTH_MIN: f32 = 0f32;
pub const DSFXCHORUS_FEEDBACK_MAX: f32 = 99f32;
pub const DSFXCHORUS_FEEDBACK_MIN: f32 = -99f32;
pub const DSFXCHORUS_FREQUENCY_MAX: f32 = 10f32;
pub const DSFXCHORUS_FREQUENCY_MIN: f32 = 0f32;
pub const DSFXCHORUS_PHASE_180: u32 = 4u32;
pub const DSFXCHORUS_PHASE_90: u32 = 3u32;
pub const DSFXCHORUS_PHASE_MAX: u32 = 4u32;
pub const DSFXCHORUS_PHASE_MIN: u32 = 0u32;
pub const DSFXCHORUS_PHASE_NEG_180: u32 = 0u32;
pub const DSFXCHORUS_PHASE_NEG_90: u32 = 1u32;
pub const DSFXCHORUS_PHASE_ZERO: u32 = 2u32;
pub const DSFXCHORUS_WAVE_SIN: u32 = 1u32;
pub const DSFXCHORUS_WAVE_TRIANGLE: u32 = 0u32;
pub const DSFXCHORUS_WETDRYMIX_MAX: f32 = 100f32;
pub const DSFXCHORUS_WETDRYMIX_MIN: f32 = 0f32;
pub const DSFXCOMPRESSOR_ATTACK_MAX: f32 = 500f32;
pub const DSFXCOMPRESSOR_ATTACK_MIN: f32 = 0.01f32;
pub const DSFXCOMPRESSOR_GAIN_MAX: f32 = 60f32;
pub const DSFXCOMPRESSOR_GAIN_MIN: f32 = -60f32;
pub const DSFXCOMPRESSOR_PREDELAY_MAX: f32 = 4f32;
pub const DSFXCOMPRESSOR_PREDELAY_MIN: f32 = 0f32;
pub const DSFXCOMPRESSOR_RATIO_MAX: f32 = 100f32;
pub const DSFXCOMPRESSOR_RATIO_MIN: f32 = 1f32;
pub const DSFXCOMPRESSOR_RELEASE_MAX: f32 = 3000f32;
pub const DSFXCOMPRESSOR_RELEASE_MIN: f32 = 50f32;
pub const DSFXCOMPRESSOR_THRESHOLD_MAX: f32 = 0f32;
pub const DSFXCOMPRESSOR_THRESHOLD_MIN: f32 = -60f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSFXChorus {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl DSFXChorus {}
impl ::core::default::Default for DSFXChorus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXChorus {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXChorus").field("fWetDryMix", &self.fWetDryMix).field("fDepth", &self.fDepth).field("fFeedback", &self.fFeedback).field("fFrequency", &self.fFrequency).field("lWaveform", &self.lWaveform).field("fDelay", &self.fDelay).field("lPhase", &self.lPhase).finish()
    }
}
impl ::core::cmp::PartialEq for DSFXChorus {
    fn eq(&self, other: &Self) -> bool {
        self.fWetDryMix == other.fWetDryMix && self.fDepth == other.fDepth && self.fFeedback == other.fFeedback && self.fFrequency == other.fFrequency && self.lWaveform == other.lWaveform && self.fDelay == other.fDelay && self.lPhase == other.lPhase
    }
}
impl ::core::cmp::Eq for DSFXChorus {}
unsafe impl ::windows::core::Abi for DSFXChorus {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSFXCompressor {
    pub fGain: f32,
    pub fAttack: f32,
    pub fRelease: f32,
    pub fThreshold: f32,
    pub fRatio: f32,
    pub fPredelay: f32,
}
impl DSFXCompressor {}
impl ::core::default::Default for DSFXCompressor {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXCompressor {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXCompressor").field("fGain", &self.fGain).field("fAttack", &self.fAttack).field("fRelease", &self.fRelease).field("fThreshold", &self.fThreshold).field("fRatio", &self.fRatio).field("fPredelay", &self.fPredelay).finish()
    }
}
impl ::core::cmp::PartialEq for DSFXCompressor {
    fn eq(&self, other: &Self) -> bool {
        self.fGain == other.fGain && self.fAttack == other.fAttack && self.fRelease == other.fRelease && self.fThreshold == other.fThreshold && self.fRatio == other.fRatio && self.fPredelay == other.fPredelay
    }
}
impl ::core::cmp::Eq for DSFXCompressor {}
unsafe impl ::windows::core::Abi for DSFXCompressor {
    type Abi = Self;
}
pub const DSFXDISTORTION_EDGE_MAX: f32 = 100f32;
pub const DSFXDISTORTION_EDGE_MIN: f32 = 0f32;
pub const DSFXDISTORTION_GAIN_MAX: f32 = 0f32;
pub const DSFXDISTORTION_GAIN_MIN: f32 = -60f32;
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MAX: f32 = 8000f32;
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MIN: f32 = 100f32;
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MAX: f32 = 8000f32;
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MIN: f32 = 100f32;
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MAX: f32 = 8000f32;
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MIN: f32 = 100f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSFXDistortion {
    pub fGain: f32,
    pub fEdge: f32,
    pub fPostEQCenterFrequency: f32,
    pub fPostEQBandwidth: f32,
    pub fPreLowpassCutoff: f32,
}
impl DSFXDistortion {}
impl ::core::default::Default for DSFXDistortion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXDistortion {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXDistortion").field("fGain", &self.fGain).field("fEdge", &self.fEdge).field("fPostEQCenterFrequency", &self.fPostEQCenterFrequency).field("fPostEQBandwidth", &self.fPostEQBandwidth).field("fPreLowpassCutoff", &self.fPreLowpassCutoff).finish()
    }
}
impl ::core::cmp::PartialEq for DSFXDistortion {
    fn eq(&self, other: &Self) -> bool {
        self.fGain == other.fGain && self.fEdge == other.fEdge && self.fPostEQCenterFrequency == other.fPostEQCenterFrequency && self.fPostEQBandwidth == other.fPostEQBandwidth && self.fPreLowpassCutoff == other.fPreLowpassCutoff
    }
}
impl ::core::cmp::Eq for DSFXDistortion {}
unsafe impl ::windows::core::Abi for DSFXDistortion {
    type Abi = Self;
}
pub const DSFXECHO_FEEDBACK_MAX: f32 = 100f32;
pub const DSFXECHO_FEEDBACK_MIN: f32 = 0f32;
pub const DSFXECHO_LEFTDELAY_MAX: f32 = 2000f32;
pub const DSFXECHO_LEFTDELAY_MIN: f32 = 1f32;
pub const DSFXECHO_PANDELAY_MAX: u32 = 1u32;
pub const DSFXECHO_PANDELAY_MIN: u32 = 0u32;
pub const DSFXECHO_RIGHTDELAY_MAX: f32 = 2000f32;
pub const DSFXECHO_RIGHTDELAY_MIN: f32 = 1f32;
pub const DSFXECHO_WETDRYMIX_MAX: f32 = 100f32;
pub const DSFXECHO_WETDRYMIX_MIN: f32 = 0f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSFXEcho {
    pub fWetDryMix: f32,
    pub fFeedback: f32,
    pub fLeftDelay: f32,
    pub fRightDelay: f32,
    pub lPanDelay: i32,
}
impl DSFXEcho {}
impl ::core::default::Default for DSFXEcho {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXEcho {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXEcho").field("fWetDryMix", &self.fWetDryMix).field("fFeedback", &self.fFeedback).field("fLeftDelay", &self.fLeftDelay).field("fRightDelay", &self.fRightDelay).field("lPanDelay", &self.lPanDelay).finish()
    }
}
impl ::core::cmp::PartialEq for DSFXEcho {
    fn eq(&self, other: &Self) -> bool {
        self.fWetDryMix == other.fWetDryMix && self.fFeedback == other.fFeedback && self.fLeftDelay == other.fLeftDelay && self.fRightDelay == other.fRightDelay && self.lPanDelay == other.lPanDelay
    }
}
impl ::core::cmp::Eq for DSFXEcho {}
unsafe impl ::windows::core::Abi for DSFXEcho {
    type Abi = Self;
}
pub const DSFXFLANGER_DELAY_MAX: f32 = 4f32;
pub const DSFXFLANGER_DELAY_MIN: f32 = 0f32;
pub const DSFXFLANGER_DEPTH_MAX: f32 = 100f32;
pub const DSFXFLANGER_DEPTH_MIN: f32 = 0f32;
pub const DSFXFLANGER_FEEDBACK_MAX: f32 = 99f32;
pub const DSFXFLANGER_FEEDBACK_MIN: f32 = -99f32;
pub const DSFXFLANGER_FREQUENCY_MAX: f32 = 10f32;
pub const DSFXFLANGER_FREQUENCY_MIN: f32 = 0f32;
pub const DSFXFLANGER_PHASE_180: u32 = 4u32;
pub const DSFXFLANGER_PHASE_90: u32 = 3u32;
pub const DSFXFLANGER_PHASE_MAX: u32 = 4u32;
pub const DSFXFLANGER_PHASE_MIN: u32 = 0u32;
pub const DSFXFLANGER_PHASE_NEG_180: u32 = 0u32;
pub const DSFXFLANGER_PHASE_NEG_90: u32 = 1u32;
pub const DSFXFLANGER_PHASE_ZERO: u32 = 2u32;
pub const DSFXFLANGER_WAVE_SIN: u32 = 1u32;
pub const DSFXFLANGER_WAVE_TRIANGLE: u32 = 0u32;
pub const DSFXFLANGER_WETDRYMIX_MAX: f32 = 100f32;
pub const DSFXFLANGER_WETDRYMIX_MIN: f32 = 0f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSFXFlanger {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl DSFXFlanger {}
impl ::core::default::Default for DSFXFlanger {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXFlanger {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXFlanger").field("fWetDryMix", &self.fWetDryMix).field("fDepth", &self.fDepth).field("fFeedback", &self.fFeedback).field("fFrequency", &self.fFrequency).field("lWaveform", &self.lWaveform).field("fDelay", &self.fDelay).field("lPhase", &self.lPhase).finish()
    }
}
impl ::core::cmp::PartialEq for DSFXFlanger {
    fn eq(&self, other: &Self) -> bool {
        self.fWetDryMix == other.fWetDryMix && self.fDepth == other.fDepth && self.fFeedback == other.fFeedback && self.fFrequency == other.fFrequency && self.lWaveform == other.lWaveform && self.fDelay == other.fDelay && self.lPhase == other.lPhase
    }
}
impl ::core::cmp::Eq for DSFXFlanger {}
unsafe impl ::windows::core::Abi for DSFXFlanger {
    type Abi = Self;
}
pub const DSFXGARGLE_RATEHZ_MAX: u32 = 1000u32;
pub const DSFXGARGLE_RATEHZ_MIN: u32 = 1u32;
pub const DSFXGARGLE_WAVE_SQUARE: u32 = 1u32;
pub const DSFXGARGLE_WAVE_TRIANGLE: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSFXGargle {
    pub dwRateHz: u32,
    pub dwWaveShape: u32,
}
impl DSFXGargle {}
impl ::core::default::Default for DSFXGargle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXGargle {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXGargle").field("dwRateHz", &self.dwRateHz).field("dwWaveShape", &self.dwWaveShape).finish()
    }
}
impl ::core::cmp::PartialEq for DSFXGargle {
    fn eq(&self, other: &Self) -> bool {
        self.dwRateHz == other.dwRateHz && self.dwWaveShape == other.dwWaveShape
    }
}
impl ::core::cmp::Eq for DSFXGargle {}
unsafe impl ::windows::core::Abi for DSFXGargle {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl DSFXI3DL2Reverb {}
impl ::core::default::Default for DSFXI3DL2Reverb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXI3DL2Reverb {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXI3DL2Reverb")
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
impl ::core::cmp::PartialEq for DSFXI3DL2Reverb {
    fn eq(&self, other: &Self) -> bool {
        self.lRoom == other.lRoom
            && self.lRoomHF == other.lRoomHF
            && self.flRoomRolloffFactor == other.flRoomRolloffFactor
            && self.flDecayTime == other.flDecayTime
            && self.flDecayHFRatio == other.flDecayHFRatio
            && self.lReflections == other.lReflections
            && self.flReflectionsDelay == other.flReflectionsDelay
            && self.lReverb == other.lReverb
            && self.flReverbDelay == other.flReverbDelay
            && self.flDiffusion == other.flDiffusion
            && self.flDensity == other.flDensity
            && self.flHFReference == other.flHFReference
    }
}
impl ::core::cmp::Eq for DSFXI3DL2Reverb {}
unsafe impl ::windows::core::Abi for DSFXI3DL2Reverb {
    type Abi = Self;
}
pub const DSFXPARAMEQ_BANDWIDTH_MAX: f32 = 36f32;
pub const DSFXPARAMEQ_BANDWIDTH_MIN: f32 = 1f32;
pub const DSFXPARAMEQ_CENTER_MAX: f32 = 16000f32;
pub const DSFXPARAMEQ_CENTER_MIN: f32 = 80f32;
pub const DSFXPARAMEQ_GAIN_MAX: f32 = 15f32;
pub const DSFXPARAMEQ_GAIN_MIN: f32 = -15f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSFXParamEq {
    pub fCenter: f32,
    pub fBandwidth: f32,
    pub fGain: f32,
}
impl DSFXParamEq {}
impl ::core::default::Default for DSFXParamEq {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXParamEq {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXParamEq").field("fCenter", &self.fCenter).field("fBandwidth", &self.fBandwidth).field("fGain", &self.fGain).finish()
    }
}
impl ::core::cmp::PartialEq for DSFXParamEq {
    fn eq(&self, other: &Self) -> bool {
        self.fCenter == other.fCenter && self.fBandwidth == other.fBandwidth && self.fGain == other.fGain
    }
}
impl ::core::cmp::Eq for DSFXParamEq {}
unsafe impl ::windows::core::Abi for DSFXParamEq {
    type Abi = Self;
}
pub const DSFXR_FAILED: i32 = 4i32;
pub const DSFXR_LOCHARDWARE: i32 = 1i32;
pub const DSFXR_LOCSOFTWARE: i32 = 2i32;
pub const DSFXR_PRESENT: i32 = 0i32;
pub const DSFXR_SENDLOOP: i32 = 6i32;
pub const DSFXR_UNALLOCATED: i32 = 3i32;
pub const DSFXR_UNKNOWN: i32 = 5i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DSFXWavesReverb {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl DSFXWavesReverb {}
impl ::core::default::Default for DSFXWavesReverb {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DSFXWavesReverb {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DSFXWavesReverb").field("fInGain", &self.fInGain).field("fReverbMix", &self.fReverbMix).field("fReverbTime", &self.fReverbTime).field("fHighFreqRTRatio", &self.fHighFreqRTRatio).finish()
    }
}
impl ::core::cmp::PartialEq for DSFXWavesReverb {
    fn eq(&self, other: &Self) -> bool {
        self.fInGain == other.fInGain && self.fReverbMix == other.fReverbMix && self.fReverbTime == other.fReverbTime && self.fHighFreqRTRatio == other.fHighFreqRTRatio
    }
}
impl ::core::cmp::Eq for DSFXWavesReverb {}
unsafe impl ::windows::core::Abi for DSFXWavesReverb {
    type Abi = Self;
}
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_DEFAULT: f32 = 0.83f32;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MAX: f32 = 2f32;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MIN: f32 = 0.1f32;
pub const DSFX_I3DL2REVERB_DECAYTIME_DEFAULT: f32 = 1.49f32;
pub const DSFX_I3DL2REVERB_DECAYTIME_MAX: f32 = 20f32;
pub const DSFX_I3DL2REVERB_DECAYTIME_MIN: f32 = 0.1f32;
pub const DSFX_I3DL2REVERB_DENSITY_DEFAULT: f32 = 100f32;
pub const DSFX_I3DL2REVERB_DENSITY_MAX: f32 = 100f32;
pub const DSFX_I3DL2REVERB_DENSITY_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_DIFFUSION_DEFAULT: f32 = 100f32;
pub const DSFX_I3DL2REVERB_DIFFUSION_MAX: f32 = 100f32;
pub const DSFX_I3DL2REVERB_DIFFUSION_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_HFREFERENCE_DEFAULT: f32 = 5000f32;
pub const DSFX_I3DL2REVERB_HFREFERENCE_MAX: f32 = 20000f32;
pub const DSFX_I3DL2REVERB_HFREFERENCE_MIN: f32 = 20f32;
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: u32 = 2u32;
pub const DSFX_I3DL2REVERB_QUALITY_MAX: u32 = 3u32;
pub const DSFX_I3DL2REVERB_QUALITY_MIN: u32 = 0u32;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: f32 = 0.007f32;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: f32 = 0.3f32;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: i32 = -2602i32;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: u32 = 1000u32;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: i32 = -10000i32;
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: f32 = 0.011f32;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: f32 = 0.1f32;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: u32 = 200u32;
pub const DSFX_I3DL2REVERB_REVERB_MAX: u32 = 2000u32;
pub const DSFX_I3DL2REVERB_REVERB_MIN: i32 = -10000i32;
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: i32 = -100i32;
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: u32 = 0u32;
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: i32 = -10000i32;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: f32 = 0f32;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: f32 = 10f32;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: f32 = 0f32;
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: i32 = -1000i32;
pub const DSFX_I3DL2REVERB_ROOM_MAX: u32 = 0u32;
pub const DSFX_I3DL2REVERB_ROOM_MIN: i32 = -10000i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ALLEY: i32 = 15i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ARENA: i32 = 10i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: i32 = 7i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_BATHROOM: i32 = 4i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: i32 = 12i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CAVE: i32 = 9i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CITY: i32 = 17i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: i32 = 8i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_DEFAULT: i32 = 0i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_FOREST: i32 = 16i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_GENERIC: i32 = 1i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HALLWAY: i32 = 13i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HANGAR: i32 = 11i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEHALL: i32 = 28i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEROOM: i32 = 26i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: i32 = 5i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: i32 = 27i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: i32 = 25i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: i32 = 18i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: i32 = 2i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: i32 = 21i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLAIN: i32 = 20i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLATE: i32 = 29i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_QUARRY: i32 = 19i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ROOM: i32 = 3i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: i32 = 22i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SMALLROOM: i32 = 24i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: i32 = 14i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONEROOM: i32 = 6i32;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_UNDERWATER: i32 = 23i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_BRICKWALL: i32 = 5i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_CURTAIN: i32 = 7i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_DOUBLEWINDOW: i32 = 1i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_SINGLEWINDOW: i32 = 0i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_STONEWALL: i32 = 6i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_THICKDOOR: i32 = 3i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_THINDOOR: i32 = 2i32;
pub const DSFX_I3DL2_MATERIAL_PRESET_WOODWALL: i32 = 4i32;
pub const DSFX_LOCHARDWARE: u32 = 1u32;
pub const DSFX_LOCSOFTWARE: u32 = 2u32;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_DEFAULT: f32 = 0.001f32;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MAX: f32 = 0.999f32;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MIN: f32 = 0.001f32;
pub const DSFX_WAVESREVERB_INGAIN_DEFAULT: f32 = 0f32;
pub const DSFX_WAVESREVERB_INGAIN_MAX: f32 = 0f32;
pub const DSFX_WAVESREVERB_INGAIN_MIN: f32 = -96f32;
pub const DSFX_WAVESREVERB_REVERBMIX_DEFAULT: f32 = 0f32;
pub const DSFX_WAVESREVERB_REVERBMIX_MAX: f32 = 0f32;
pub const DSFX_WAVESREVERB_REVERBMIX_MIN: f32 = -96f32;
pub const DSFX_WAVESREVERB_REVERBTIME_DEFAULT: f32 = 1000f32;
pub const DSFX_WAVESREVERB_REVERBTIME_MAX: f32 = 3000f32;
pub const DSFX_WAVESREVERB_REVERBTIME_MIN: f32 = 0.001f32;
pub const DSSCL_EXCLUSIVE: u32 = 3u32;
pub const DSSCL_NORMAL: u32 = 1u32;
pub const DSSCL_PRIORITY: u32 = 2u32;
pub const DSSCL_WRITEPRIMARY: u32 = 4u32;
pub const DSSPEAKER_5POINT1: u32 = 6u32;
pub const DSSPEAKER_5POINT1_BACK: u32 = 6u32;
pub const DSSPEAKER_5POINT1_SURROUND: u32 = 9u32;
pub const DSSPEAKER_7POINT1: u32 = 7u32;
pub const DSSPEAKER_7POINT1_SURROUND: u32 = 8u32;
pub const DSSPEAKER_7POINT1_WIDE: u32 = 7u32;
pub const DSSPEAKER_DIRECTOUT: u32 = 0u32;
pub const DSSPEAKER_GEOMETRY_MAX: u32 = 180u32;
pub const DSSPEAKER_GEOMETRY_MIN: u32 = 5u32;
pub const DSSPEAKER_GEOMETRY_NARROW: u32 = 10u32;
pub const DSSPEAKER_GEOMETRY_WIDE: u32 = 20u32;
pub const DSSPEAKER_HEADPHONE: u32 = 1u32;
pub const DSSPEAKER_MONO: u32 = 2u32;
pub const DSSPEAKER_QUAD: u32 = 3u32;
pub const DSSPEAKER_STEREO: u32 = 4u32;
pub const DSSPEAKER_SURROUND: u32 = 5u32;
pub const DS_CERTIFIED: u32 = 0u32;
pub const DS_NO_VIRTUALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(142082058i32 as _);
pub const DS_UNCERTIFIED: u32 = 1u32;
#[inline]
pub unsafe fn DirectSoundCaptureCreate<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguiddevice: *const ::windows::core::GUID, ppdsc: *mut ::core::option::Option<IDirectSoundCapture>, punkouter: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureCreate(pcguiddevice: *const ::windows::core::GUID, ppdsc: *mut ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        DirectSoundCaptureCreate(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppdsc), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DirectSoundCaptureCreate8<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguiddevice: *const ::windows::core::GUID, ppdsc8: *mut ::core::option::Option<IDirectSoundCapture>, punkouter: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureCreate8(pcguiddevice: *const ::windows::core::GUID, ppdsc8: *mut ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        DirectSoundCaptureCreate8(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppdsc8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateA(pdsenumcallback: ::core::option::Option<LPDSENUMCALLBACKA>, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureEnumerateA(pdsenumcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundCaptureEnumerateA(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateW(pdsenumcallback: ::core::option::Option<LPDSENUMCALLBACKW>, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureEnumerateW(pdsenumcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundCaptureEnumerateW(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DirectSoundCreate<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguiddevice: *const ::windows::core::GUID, ppds: *mut ::core::option::Option<IDirectSound>, punkouter: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCreate(pcguiddevice: *const ::windows::core::GUID, ppds: *mut ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        DirectSoundCreate(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppds), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DirectSoundCreate8<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcguiddevice: *const ::windows::core::GUID, ppds8: *mut ::core::option::Option<IDirectSound8>, punkouter: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCreate8(pcguiddevice: *const ::windows::core::GUID, ppds8: *mut ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        DirectSoundCreate8(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppds8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundEnumerateA(pdsenumcallback: ::core::option::Option<LPDSENUMCALLBACKA>, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundEnumerateA(pdsenumcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundEnumerateA(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundEnumerateW(pdsenumcallback: ::core::option::Option<LPDSENUMCALLBACKW>, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundEnumerateW(pdsenumcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DirectSoundEnumerateW(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundFullDuplexCreate<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param9: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(
    pcguidcapturedevice: *const ::windows::core::GUID,
    pcguidrenderdevice: *const ::windows::core::GUID,
    pcdscbufferdesc: *const DSCBUFFERDESC,
    pcdsbufferdesc: *const DSBUFFERDESC,
    hwnd: Param4,
    dwlevel: u32,
    ppdsfd: *mut ::core::option::Option<IDirectSoundFullDuplex>,
    ppdscbuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>,
    ppdsbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>,
    punkouter: Param9,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundFullDuplexCreate(pcguidcapturedevice: *const ::windows::core::GUID, pcguidrenderdevice: *const ::windows::core::GUID, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, ppdsfd: *mut ::windows::core::RawPtr, ppdscbuffer8: *mut ::windows::core::RawPtr, ppdsbuffer8: *mut ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        DirectSoundFullDuplexCreate(
            ::core::mem::transmute(pcguidcapturedevice),
            ::core::mem::transmute(pcguidrenderdevice),
            ::core::mem::transmute(pcdscbufferdesc),
            ::core::mem::transmute(pcdsbufferdesc),
            hwnd.into_param().abi(),
            ::core::mem::transmute(dwlevel),
            ::core::mem::transmute(ppdsfd),
            ::core::mem::transmute(ppdscbuffer8),
            ::core::mem::transmute(ppdsbuffer8),
            punkouter.into_param().abi(),
        )
        .ok()
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
#[inline]
pub unsafe fn GetDeviceID(pguidsrc: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceID(pguidsrc: *const ::windows::core::GUID, pguiddest: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetDeviceID(::core::mem::transmute(pguidsrc), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSound(pub ::windows::core::IUnknown);
impl IDirectSound {
    pub unsafe fn CreateSoundBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsbufferdesc), ::core::mem::transmute(ppdsbuffer), punkouter.into_param().abi()).ok()
    }
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCAPS> {
        let mut result__: <DSCAPS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCAPS>(result__)
    }
    pub unsafe fn DuplicateSoundBuffer<'a, Param0: ::windows::core::IntoParam<'a, IDirectSoundBuffer>>(&self, pdsbufferoriginal: Param0) -> ::windows::core::Result<IDirectSoundBuffer> {
        let mut result__: <IDirectSoundBuffer as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdsbufferoriginal.into_param().abi(), &mut result__).from_abi::<IDirectSoundBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwlevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(dwlevel)).ok()
    }
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetSpeakerConfig(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwspeakerconfig)).ok()
    }
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSound {
    type Vtable = IDirectSound_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x279afa83_4981_11ce_a521_0020af0be560);
}
impl ::core::convert::From<IDirectSound> for ::windows::core::IUnknown {
    fn from(value: IDirectSound) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSound> for ::windows::core::IUnknown {
    fn from(value: &IDirectSound) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSound {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSound {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdscaps: *mut DSCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsbufferoriginal: ::windows::core::RawPtr, ppdsbufferduplicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwspeakerconfig: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwspeakerconfig: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSound3DBuffer(pub ::windows::core::IUnknown);
impl IDirectSound3DBuffer {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DS3DBUFFER> {
        let mut result__: <DS3DBUFFER as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DS3DBUFFER>(result__)
    }
    pub unsafe fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwinsideconeangle), ::core::mem::transmute(pdwoutsideconeangle)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetConeOrientation(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    pub unsafe fn GetConeOutsideVolume(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn GetMaxDistance(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn GetMinDistance(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn GetMode(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPosition(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetVelocity(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcds3dbuffer), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinsideconeangle), ::core::mem::transmute(dwoutsideconeangle), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lconeoutsidevolume), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(flmaxdistance), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(flmindistance), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetMode(&self, dwmode: u32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmode), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSound3DBuffer {
    type Vtable = IDirectSound3DBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x279afa86_4981_11ce_a521_0020af0be560);
}
impl ::core::convert::From<IDirectSound3DBuffer> for ::windows::core::IUnknown {
    fn from(value: IDirectSound3DBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSound3DBuffer> for ::windows::core::IUnknown {
    fn from(value: &IDirectSound3DBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSound3DBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSound3DBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pds3dbuffer: *mut DS3DBUFFER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plconeoutsidevolume: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflmaxdistance: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflmindistance: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwmode: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flmaxdistance: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flmindistance: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwmode: u32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSound3DListener(pub ::windows::core::IUnknown);
impl IDirectSound3DListener {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DS3DLISTENER> {
        let mut result__: <DS3DLISTENER as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DS3DLISTENER>(result__)
    }
    pub unsafe fn GetDistanceFactor(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn GetDopplerFactor(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetOrientation(&self, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvorientfront), ::core::mem::transmute(pvorienttop)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetPosition(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    pub unsafe fn GetRolloffFactor(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetVelocity(&self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclistener), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(fldistancefactor), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(fldopplerfactor), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(xfront), ::core::mem::transmute(yfront), ::core::mem::transmute(zfront), ::core::mem::transmute(xtop), ::core::mem::transmute(ytop), ::core::mem::transmute(ztop), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(flrollofffactor), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    pub unsafe fn CommitDeferredSettings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSound3DListener {
    type Vtable = IDirectSound3DListener_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x279afa84_4981_11ce_a521_0020af0be560);
}
impl ::core::convert::From<IDirectSound3DListener> for ::windows::core::IUnknown {
    fn from(value: IDirectSound3DListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSound3DListener> for ::windows::core::IUnknown {
    fn from(value: &IDirectSound3DListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSound3DListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSound3DListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DListener_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plistener: *mut DS3DLISTENER) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfldistancefactor: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfldopplerfactor: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflrollofffactor: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fldistancefactor: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fldopplerfactor: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flrollofffactor: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSound8(pub ::windows::core::IUnknown);
impl IDirectSound8 {
    pub unsafe fn CreateSoundBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsbufferdesc), ::core::mem::transmute(ppdsbuffer), punkouter.into_param().abi()).ok()
    }
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCAPS> {
        let mut result__: <DSCAPS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCAPS>(result__)
    }
    pub unsafe fn DuplicateSoundBuffer<'a, Param0: ::windows::core::IntoParam<'a, IDirectSoundBuffer>>(&self, pdsbufferoriginal: Param0) -> ::windows::core::Result<IDirectSoundBuffer> {
        let mut result__: <IDirectSoundBuffer as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdsbufferoriginal.into_param().abi(), &mut result__).from_abi::<IDirectSoundBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwlevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(dwlevel)).ok()
    }
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetSpeakerConfig(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwspeakerconfig)).ok()
    }
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
    pub unsafe fn VerifyCertification(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSound8 {
    type Vtable = IDirectSound8_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc50a7e93_f395_4834_9ef6_7fa99de50966);
}
impl ::core::convert::From<IDirectSound8> for ::windows::core::IUnknown {
    fn from(value: IDirectSound8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSound8> for ::windows::core::IUnknown {
    fn from(value: &IDirectSound8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSound8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSound8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDirectSound> for &IDirectSound8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSound> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound8_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdscaps: *mut DSCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsbufferoriginal: ::windows::core::RawPtr, ppdsbufferduplicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwspeakerconfig: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwspeakerconfig: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcertified: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundBuffer(pub ::windows::core::IUnknown);
impl IDirectSoundBuffer {
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSBCAPS> {
        let mut result__: <DSBCAPS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSBCAPS>(result__)
    }
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcurrentplaycursor), ::core::mem::transmute(pdwcurrentwritecursor)).ok()
    }
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    pub unsafe fn GetVolume(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn GetPan(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn GetFrequency(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IDirectSound>>(&self, pdirectsound: Param0, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), ::core::mem::transmute(pcdsbufferdesc)).ok()
    }
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved1), ::core::mem::transmute(dwpriority), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwnewposition)).ok()
    }
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcfxformat)).ok()
    }
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn SetPan(&self, lpan: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpan)).ok()
    }
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwfrequency)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundBuffer {
    type Vtable = IDirectSoundBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x279afa85_4981_11ce_a521_0020af0be560);
}
impl ::core::convert::From<IDirectSoundBuffer> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundBuffer> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsbuffercaps: *mut DSBCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plpan: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwfrequency: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdirectsound: ::windows::core::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwnewposition: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lvolume: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpan: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwfrequency: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundBuffer8(pub ::windows::core::IUnknown);
impl IDirectSoundBuffer8 {
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSBCAPS> {
        let mut result__: <DSBCAPS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSBCAPS>(result__)
    }
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcurrentplaycursor), ::core::mem::transmute(pdwcurrentwritecursor)).ok()
    }
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    pub unsafe fn GetVolume(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn GetPan(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn GetFrequency(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IDirectSound>>(&self, pdirectsound: Param0, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), ::core::mem::transmute(pcdsbufferdesc)).ok()
    }
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved1), ::core::mem::transmute(dwpriority), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwnewposition)).ok()
    }
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcfxformat)).ok()
    }
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lvolume)).ok()
    }
    pub unsafe fn SetPan(&self, lpan: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpan)).ok()
    }
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwfrequency)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetFX(&self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dweffectscount), ::core::mem::transmute(pdsfxdesc), ::core::mem::transmute(pdwresultcodes)).ok()
    }
    pub unsafe fn AcquireResources(&self, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dweffectscount), ::core::mem::transmute(pdwresultcodes)).ok()
    }
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidobject), ::core::mem::transmute(dwindex), ::core::mem::transmute(rguidinterface), ::core::mem::transmute(ppobject)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundBuffer8 {
    type Vtable = IDirectSoundBuffer8_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6825a449_7524_4d82_920f_50e36ab3ab1e);
}
impl ::core::convert::From<IDirectSoundBuffer8> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundBuffer8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundBuffer8> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundBuffer8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDirectSoundBuffer> for &IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSoundBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer8_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsbuffercaps: *mut DSBCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plvolume: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plpan: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwfrequency: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdirectsound: ::windows::core::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwnewposition: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lvolume: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpan: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwfrequency: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCapture(pub ::windows::core::IUnknown);
impl IDirectSoundCapture {
    pub unsafe fn CreateCaptureBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::core::option::Option<IDirectSoundCaptureBuffer>, punkouter: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdscbufferdesc), ::core::mem::transmute(ppdscbuffer), punkouter.into_param().abi()).ok()
    }
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCCAPS> {
        let mut result__: <DSCCAPS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCCAPS>(result__)
    }
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCapture {
    type Vtable = IDirectSoundCapture_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0210781_89cd_11d0_af08_00a0c925cd16);
}
impl ::core::convert::From<IDirectSoundCapture> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCapture) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCapture> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsccaps: *mut DSCCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCaptureBuffer(pub ::windows::core::IUnknown);
impl IDirectSoundCaptureBuffer {
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCBCAPS> {
        let mut result__: <DSCBCAPS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCBCAPS>(result__)
    }
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcaptureposition), ::core::mem::transmute(pdwreadposition)).ok()
    }
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IDirectSoundCapture>>(&self, pdirectsoundcapture: Param0, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdirectsoundcapture.into_param().abi(), ::core::mem::transmute(pcdscbufferdesc)).ok()
    }
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Start(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCaptureBuffer {
    type Vtable = IDirectSoundCaptureBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0210782_89cd_11d0_af08_00a0c925cd16);
}
impl ::core::convert::From<IDirectSoundCaptureBuffer> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCaptureBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCaptureBuffer> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCaptureBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCaptureBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCaptureBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdscbcaps: *mut DSCBCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdirectsoundcapture: ::windows::core::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCaptureBuffer8(pub ::windows::core::IUnknown);
impl IDirectSoundCaptureBuffer8 {
    pub unsafe fn GetCaps(&self) -> ::windows::core::Result<DSCBCAPS> {
        let mut result__: <DSCBCAPS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCBCAPS>(result__)
    }
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcaptureposition), ::core::mem::transmute(pdwreadposition)).ok()
    }
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IDirectSoundCapture>>(&self, pdirectsoundcapture: Param0, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdirectsoundcapture.into_param().abi(), ::core::mem::transmute(pcdscbufferdesc)).ok()
    }
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Start(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidobject), ::core::mem::transmute(dwindex), ::core::mem::transmute(rguidinterface), ::core::mem::transmute(ppobject)).ok()
    }
    pub unsafe fn GetFXStatus(&self, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dweffectscount), ::core::mem::transmute(pdwfxstatus)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCaptureBuffer8 {
    type Vtable = IDirectSoundCaptureBuffer8_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00990df4_0dbb_4872_833e_6d303e80aeb6);
}
impl ::core::convert::From<IDirectSoundCaptureBuffer8> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCaptureBuffer8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCaptureBuffer8> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCaptureBuffer8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDirectSoundCaptureBuffer> for &IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::core::Param<'a, IDirectSoundCaptureBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer8_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdscbcaps: *mut DSCBCAPS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdirectsoundcapture: ::windows::core::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCaptureFXAec(pub ::windows::core::IUnknown);
impl IDirectSoundCaptureFXAec {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdscfxaec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSCFXAec> {
        let mut result__: <DSCFXAec as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCFXAec>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCaptureFXAec {
    type Vtable = IDirectSoundCaptureFXAec_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad74143d_903d_4ab7_8066_28d363036d65);
}
impl ::core::convert::From<IDirectSoundCaptureFXAec> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCaptureFXAec) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCaptureFXAec> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCaptureFXAec) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCaptureFXAec {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCaptureFXAec {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXAec_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdscfxaec: *const DSCFXAec) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdscfxaec: *mut DSCFXAec) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCaptureFXNoiseSuppress(pub ::windows::core::IUnknown);
impl IDirectSoundCaptureFXNoiseSuppress {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdscfxnoisesuppress)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSCFXNoiseSuppress> {
        let mut result__: <DSCFXNoiseSuppress as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCFXNoiseSuppress>(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundCaptureFXNoiseSuppress {
    type Vtable = IDirectSoundCaptureFXNoiseSuppress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed311e41_fbae_4175_9625_cd0854f693ca);
}
impl ::core::convert::From<IDirectSoundCaptureFXNoiseSuppress> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundCaptureFXNoiseSuppress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCaptureFXNoiseSuppress> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundCaptureFXNoiseSuppress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundCaptureFXNoiseSuppress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundCaptureFXNoiseSuppress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXNoiseSuppress_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXChorus(pub ::windows::core::IUnknown);
impl IDirectSoundFXChorus {
    pub unsafe fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxchorus)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXChorus> {
        let mut result__: <DSFXChorus as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXChorus>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXChorus {
    type Vtable = IDirectSoundFXChorus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x880842e3_145f_43e6_a934_a71806e50547);
}
impl ::core::convert::From<IDirectSoundFXChorus> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXChorus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXChorus> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXChorus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXChorus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXChorus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXChorus_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxchorus: *const DSFXChorus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxchorus: *mut DSFXChorus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXCompressor(pub ::windows::core::IUnknown);
impl IDirectSoundFXCompressor {
    pub unsafe fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxcompressor)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXCompressor> {
        let mut result__: <DSFXCompressor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXCompressor>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXCompressor {
    type Vtable = IDirectSoundFXCompressor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bbd1154_62f6_4e2c_a15c_d3b6c417f7a0);
}
impl ::core::convert::From<IDirectSoundFXCompressor> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXCompressor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXCompressor> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXCompressor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXCompressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXCompressor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXCompressor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxcompressor: *mut DSFXCompressor) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXDistortion(pub ::windows::core::IUnknown);
impl IDirectSoundFXDistortion {
    pub unsafe fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxdistortion)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXDistortion> {
        let mut result__: <DSFXDistortion as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXDistortion>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXDistortion {
    type Vtable = IDirectSoundFXDistortion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ecf4326_455f_4d8b_bda9_8d5d3e9e3e0b);
}
impl ::core::convert::From<IDirectSoundFXDistortion> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXDistortion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXDistortion> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXDistortion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXDistortion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXDistortion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXDistortion_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxdistortion: *mut DSFXDistortion) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXEcho(pub ::windows::core::IUnknown);
impl IDirectSoundFXEcho {
    pub unsafe fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxecho)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXEcho> {
        let mut result__: <DSFXEcho as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXEcho>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXEcho {
    type Vtable = IDirectSoundFXEcho_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bd28edf_50db_4e92_a2bd_445488d1ed42);
}
impl ::core::convert::From<IDirectSoundFXEcho> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXEcho) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXEcho> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXEcho) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXEcho {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXEcho {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXEcho_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxecho: *const DSFXEcho) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxecho: *mut DSFXEcho) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXFlanger(pub ::windows::core::IUnknown);
impl IDirectSoundFXFlanger {
    pub unsafe fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxflanger)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXFlanger> {
        let mut result__: <DSFXFlanger as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXFlanger>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXFlanger {
    type Vtable = IDirectSoundFXFlanger_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x903e9878_2c92_4072_9b2c_ea68f5396783);
}
impl ::core::convert::From<IDirectSoundFXFlanger> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXFlanger) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXFlanger> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXFlanger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXFlanger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXFlanger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXFlanger_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxflanger: *const DSFXFlanger) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxflanger: *mut DSFXFlanger) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXGargle(pub ::windows::core::IUnknown);
impl IDirectSoundFXGargle {
    pub unsafe fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxgargle)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXGargle> {
        let mut result__: <DSFXGargle as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXGargle>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXGargle {
    type Vtable = IDirectSoundFXGargle_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd616f352_d622_11ce_aac5_0020af0b99a3);
}
impl ::core::convert::From<IDirectSoundFXGargle> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXGargle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXGargle> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXGargle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXGargle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXGargle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXGargle_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxgargle: *const DSFXGargle) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxgargle: *mut DSFXGargle) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXI3DL2Reverb(pub ::windows::core::IUnknown);
impl IDirectSoundFXI3DL2Reverb {
    pub unsafe fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxi3dl2reverb)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXI3DL2Reverb> {
        let mut result__: <DSFXI3DL2Reverb as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXI3DL2Reverb>(result__)
    }
    pub unsafe fn SetPreset(&self, dwpreset: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpreset)).ok()
    }
    pub unsafe fn GetPreset(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetQuality(&self, lquality: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lquality)).ok()
    }
    pub unsafe fn GetQuality(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXI3DL2Reverb {
    type Vtable = IDirectSoundFXI3DL2Reverb_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b166a6a_0d66_43f3_80e3_ee6280dee1a4);
}
impl ::core::convert::From<IDirectSoundFXI3DL2Reverb> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXI3DL2Reverb) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXI3DL2Reverb> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXI3DL2Reverb) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXI3DL2Reverb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXI3DL2Reverb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXI3DL2Reverb_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwpreset: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwpreset: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lquality: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plquality: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXParamEq(pub ::windows::core::IUnknown);
impl IDirectSoundFXParamEq {
    pub unsafe fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxparameq)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXParamEq> {
        let mut result__: <DSFXParamEq as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXParamEq>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXParamEq {
    type Vtable = IDirectSoundFXParamEq_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc03ca9fe_fe90_4204_8078_82334cd177da);
}
impl ::core::convert::From<IDirectSoundFXParamEq> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXParamEq) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXParamEq> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXParamEq) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXParamEq {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXParamEq {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXParamEq_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxparameq: *const DSFXParamEq) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxparameq: *mut DSFXParamEq) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXWavesReverb(pub ::windows::core::IUnknown);
impl IDirectSoundFXWavesReverb {
    pub unsafe fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxwavesreverb)).ok()
    }
    pub unsafe fn GetAllParameters(&self) -> ::windows::core::Result<DSFXWavesReverb> {
        let mut result__: <DSFXWavesReverb as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXWavesReverb>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFXWavesReverb {
    type Vtable = IDirectSoundFXWavesReverb_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46858c3a_0dc6_45e3_b760_d4eef16cb325);
}
impl ::core::convert::From<IDirectSoundFXWavesReverb> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFXWavesReverb) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXWavesReverb> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFXWavesReverb) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFXWavesReverb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFXWavesReverb {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXWavesReverb_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFullDuplex(pub ::windows::core::IUnknown);
impl IDirectSoundFullDuplex {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(
        &self,
        pcaptureguid: *const ::windows::core::GUID,
        prenderguid: *const ::windows::core::GUID,
        lpdscbufferdesc: *const DSCBUFFERDESC,
        lpdsbufferdesc: *const DSBUFFERDESC,
        hwnd: Param4,
        dwlevel: u32,
        lplpdirectsoundcapturebuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>,
        lplpdirectsoundbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(pcaptureguid),
            ::core::mem::transmute(prenderguid),
            ::core::mem::transmute(lpdscbufferdesc),
            ::core::mem::transmute(lpdsbufferdesc),
            hwnd.into_param().abi(),
            ::core::mem::transmute(dwlevel),
            ::core::mem::transmute(lplpdirectsoundcapturebuffer8),
            ::core::mem::transmute(lplpdirectsoundbuffer8),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundFullDuplex {
    type Vtable = IDirectSoundFullDuplex_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedcb4c7a_daab_4216_a42e_6c50596ddc1d);
}
impl ::core::convert::From<IDirectSoundFullDuplex> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundFullDuplex) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFullDuplex> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundFullDuplex) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundFullDuplex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundFullDuplex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFullDuplex_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcaptureguid: *const ::windows::core::GUID, prenderguid: *const ::windows::core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::windows::core::RawPtr, lplpdirectsoundbuffer8: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundNotify(pub ::windows::core::IUnknown);
impl IDirectSoundNotify {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotificationPositions(&self, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpositionnotifies), ::core::mem::transmute(pcpositionnotifies)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDirectSoundNotify {
    type Vtable = IDirectSoundNotify_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0210783_89cd_11d0_af08_00a0c925cd16);
}
impl ::core::convert::From<IDirectSoundNotify> for ::windows::core::IUnknown {
    fn from(value: IDirectSoundNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundNotify> for ::windows::core::IUnknown {
    fn from(value: &IDirectSoundNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirectSoundNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirectSoundNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const KSPROPERTY_SUPPORT_GET: u32 = 1u32;
pub const KSPROPERTY_SUPPORT_SET: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKA = unsafe extern "system" fn(param0: *mut ::windows::core::GUID, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKW = unsafe extern "system" fn(param0: *mut ::windows::core::GUID, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
pub const _FACDS: u32 = 2168u32;
