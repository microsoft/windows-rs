#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_DirectSound: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1205131590, 25320, 4559, [147, 188, 68, 69, 83, 84, 0, 0]);
pub const CLSID_DirectSound8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(956419135, 33973, 20388, [186, 53, 170, 129, 114, 184, 160, 155]);
pub const CLSID_DirectSoundCapture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2954954624, 35277, 4560, [175, 8, 0, 160, 201, 37, 205, 22]);
pub const CLSID_DirectSoundCapture8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3837570067, 32665, 18696, [154, 142, 116, 227, 191, 36, 182, 225]);
pub const CLSID_DirectSoundFullDuplex: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4272173068, 31065, 16711, [178, 106, 35, 119, 185, 231, 169, 29]);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DIRECTSOUND_VERSION: u32 = 1792u32;
pub const DS3DALG_HRTF_FULL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3259052864, 7195, 4562, [148, 245, 0, 192, 79, 194, 138, 202]);
pub const DS3DALG_HRTF_LIGHT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3259052866, 7195, 4562, [148, 245, 0, 192, 79, 194, 138, 202]);
pub const DS3DALG_NO_VIRTUALIZATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3259052863, 7195, 4562, [148, 245, 0, 192, 79, 194, 138, 202]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
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
unsafe impl ::windows::runtime::Abi for DS3DBUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
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
unsafe impl ::windows::runtime::Abi for DS3DLISTENER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3DMODE_DISABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3DMODE_HEADRELATIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3DMODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_DEFAULTCONEANGLE: u32 = 360u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_DEFAULTDISTANCEFACTOR: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_DEFAULTDOPPLERFACTOR: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_DEFAULTMAXDISTANCE: f32 = 1000000000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_DEFAULTMINDISTANCE: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_DEFAULTROLLOFFFACTOR: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_DEFERRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_IMMEDIATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_MAXCONEANGLE: u32 = 360u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_MAXDOPPLERFACTOR: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_MAXROLLOFFFACTOR: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_MINCONEANGLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_MINDOPPLERFACTOR: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS3D_MINROLLOFFFACTOR: f32 = 0f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSBCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_CTRL3D: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_CTRLFREQUENCY: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_CTRLFX: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_CTRLPAN: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_CTRLVOLUME: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_GETCURRENTPOSITION2: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_GLOBALFOCUS: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_LOCDEFER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_LOCHARDWARE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_LOCSOFTWARE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_PRIMARYBUFFER: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_STATIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_STICKYFOCUS: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBCAPS_TRUEPLAYPOSITION: u32 = 524288u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBFREQUENCY_MAX: u32 = 200000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBFREQUENCY_MIN: u32 = 100u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBFREQUENCY_ORIGINAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBLOCK_ENTIREBUFFER: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBLOCK_FROMWRITECURSOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBNOTIFICATIONS_MAX: u32 = 100000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPAN_CENTER: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPAN_LEFT: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPAN_RIGHT: u32 = 10000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPLAY_LOCHARDWARE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPLAY_LOCSOFTWARE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPLAY_LOOPING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPLAY_TERMINATEBY_DISTANCE: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPLAY_TERMINATEBY_PRIORITY: u64 = 32u64;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPLAY_TERMINATEBY_TIME: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBPN_OFFSETSTOP: u32 = 4294967295u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for DSBPOSITIONNOTIFY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSIZE_FX_MIN: u32 = 150u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSIZE_MAX: u32 = 268435455u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSIZE_MIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSTATUS_BUFFERLOST: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSTATUS_LOCHARDWARE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSTATUS_LOCSOFTWARE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSTATUS_LOOPING: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSTATUS_PLAYING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBSTATUS_TERMINATED: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::WAVEFORMATEX,
    pub guid3DAlgorithm: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for DSBUFFERDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSBUFFERDESC1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBVOLUME_MAX: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSBVOLUME_MIN: i32 = -10000i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_CERTIFIED: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_CONTINUOUSRATE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_EMULDRIVER: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_PRIMARY16BIT: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_PRIMARY8BIT: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_PRIMARYMONO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_PRIMARYSTEREO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_SECONDARY16BIT: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_SECONDARY8BIT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_SECONDARYMONO: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCAPS_SECONDARYSTEREO: u32 = 512u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSCBCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCBCAPS_CTRLFX: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCBLOCK_ENTIREBUFFER: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCBSTART_LOOPING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCBSTATUS_CAPTURING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCBSTATUS_LOOPING: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSCBUFFERDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSCBUFFERDESC1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSCCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCCAPS_CERTIFIED: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCCAPS_EMULDRIVER: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCCAPS_MULTIPLECAPTURE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub struct DSCEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSCFXClass: ::windows::runtime::GUID,
    pub guidDSCFXInstance: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for DSCEFFECTDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for DSCFXAec {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for DSCFXNoiseSuppress {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFXR_LOCHARDWARE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFXR_LOCSOFTWARE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_AEC_MODE_FULL_DUPLEX: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_AEC_MODE_HALF_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_AEC_MODE_PASS_THROUGH: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_LOCHARDWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSCFX_LOCSOFTWARE: u32 = 2u32;
pub const DSDEVID_DefaultCapture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740270593, 40045, 18413, [170, 241, 77, 218, 143, 43, 92, 3]);
pub const DSDEVID_DefaultPlayback: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740270592, 40045, 18413, [170, 241, 77, 218, 143, 43, 92, 3]);
pub const DSDEVID_DefaultVoiceCapture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740270595, 40045, 18413, [170, 241, 77, 218, 143, 43, 92, 3]);
pub const DSDEVID_DefaultVoicePlayback: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740270594, 40045, 18413, [170, 241, 77, 218, 143, 43, 92, 3]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub struct DSEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSFXClass: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for DSEFFECTDESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_DELAY_MAX: f32 = 20f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_DELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_DEPTH_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_DEPTH_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_FEEDBACK_MAX: f32 = 99f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_FEEDBACK_MIN: f32 = -99f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_FREQUENCY_MAX: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_FREQUENCY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_PHASE_180: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_PHASE_90: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_PHASE_MAX: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_PHASE_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_PHASE_NEG_180: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_PHASE_NEG_90: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_PHASE_ZERO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_WAVE_SIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_WAVE_TRIANGLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCHORUS_WETDRYMIX_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_ATTACK_MAX: f32 = 500f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_ATTACK_MIN: f32 = 0.01f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_GAIN_MAX: f32 = 60f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_GAIN_MIN: f32 = -60f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_PREDELAY_MAX: f32 = 4f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_PREDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_RATIO_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_RATIO_MIN: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_RELEASE_MAX: f32 = 3000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_RELEASE_MIN: f32 = 50f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_THRESHOLD_MAX: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXCOMPRESSOR_THRESHOLD_MIN: f32 = -60f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXChorus {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXCompressor {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_EDGE_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_EDGE_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_GAIN_MAX: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_GAIN_MIN: f32 = -60f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MAX: f32 = 8000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MIN: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MAX: f32 = 8000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MIN: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MAX: f32 = 8000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MIN: f32 = 100f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXDistortion {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_FEEDBACK_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_FEEDBACK_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_LEFTDELAY_MAX: f32 = 2000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_LEFTDELAY_MIN: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_PANDELAY_MAX: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_PANDELAY_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_RIGHTDELAY_MAX: f32 = 2000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_RIGHTDELAY_MIN: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXECHO_WETDRYMIX_MIN: f32 = 0f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXEcho {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_DELAY_MAX: f32 = 4f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_DELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_DEPTH_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_DEPTH_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_FEEDBACK_MAX: f32 = 99f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_FEEDBACK_MIN: f32 = -99f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_FREQUENCY_MAX: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_FREQUENCY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_PHASE_180: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_PHASE_90: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_PHASE_MAX: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_PHASE_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_PHASE_NEG_180: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_PHASE_NEG_90: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_PHASE_ZERO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_WAVE_SIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_WAVE_TRIANGLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXFLANGER_WETDRYMIX_MIN: f32 = 0f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXFlanger {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXGARGLE_RATEHZ_MAX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXGARGLE_RATEHZ_MIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXGARGLE_WAVE_SQUARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXGARGLE_WAVE_TRIANGLE: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXGargle {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXI3DL2Reverb {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXPARAMEQ_BANDWIDTH_MAX: f32 = 36f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXPARAMEQ_BANDWIDTH_MIN: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXPARAMEQ_CENTER_MAX: f32 = 16000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXPARAMEQ_CENTER_MIN: f32 = 80f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXPARAMEQ_GAIN_MAX: f32 = 15f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXPARAMEQ_GAIN_MIN: f32 = -15f32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXParamEq {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXR_FAILED: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXR_LOCHARDWARE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXR_LOCSOFTWARE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXR_PRESENT: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXR_SENDLOOP: i32 = 6i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXR_UNALLOCATED: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFXR_UNKNOWN: i32 = 5i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
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
unsafe impl ::windows::runtime::Abi for DSFXWavesReverb {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_DEFAULT: f32 = 0.83f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MAX: f32 = 2f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MIN: f32 = 0.1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_DEFAULT: f32 = 1.49f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_MAX: f32 = 20f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_MIN: f32 = 0.1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DENSITY_DEFAULT: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DENSITY_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DENSITY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_DEFAULT: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_DEFAULT: f32 = 5000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_MAX: f32 = 20000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_MIN: f32 = 20f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_QUALITY_MAX: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_QUALITY_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: f32 = 0.007f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: f32 = 0.3f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: i32 = -2602i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: f32 = 0.011f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: f32 = 0.1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: u32 = 200u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REVERB_MAX: u32 = 2000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_REVERB_MIN: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: i32 = -100i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: i32 = -1000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOM_MAX: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2REVERB_ROOM_MIN: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ALLEY: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ARENA: i32 = 10i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: i32 = 7i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_BATHROOM: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: i32 = 12i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CAVE: i32 = 9i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CITY: i32 = 17i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_FOREST: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_GENERIC: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HALLWAY: i32 = 13i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HANGAR: i32 = 11i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEHALL: i32 = 28i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEROOM: i32 = 26i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: i32 = 5i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: i32 = 27i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: i32 = 25i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: i32 = 18i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: i32 = 21i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLAIN: i32 = 20i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLATE: i32 = 29i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_QUARRY: i32 = 19i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ROOM: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: i32 = 22i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SMALLROOM: i32 = 24i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: i32 = 14i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONEROOM: i32 = 6i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_UNDERWATER: i32 = 23i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_BRICKWALL: i32 = 5i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_CURTAIN: i32 = 7i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_DOUBLEWINDOW: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_SINGLEWINDOW: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_STONEWALL: i32 = 6i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_THICKDOOR: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_THINDOOR: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_WOODWALL: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_LOCHARDWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_LOCSOFTWARE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_DEFAULT: f32 = 0.001f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MAX: f32 = 0.999f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MIN: f32 = 0.001f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_INGAIN_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_INGAIN_MAX: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_INGAIN_MIN: f32 = -96f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_MAX: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_MIN: f32 = -96f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_DEFAULT: f32 = 1000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_MAX: f32 = 3000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_MIN: f32 = 0.001f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSCL_EXCLUSIVE: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSCL_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSCL_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSCL_WRITEPRIMARY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_5POINT1: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_5POINT1_BACK: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_5POINT1_SURROUND: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_7POINT1: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_7POINT1_SURROUND: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_7POINT1_WIDE: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_DIRECTOUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_GEOMETRY_MAX: u32 = 180u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_GEOMETRY_MIN: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_GEOMETRY_NARROW: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_GEOMETRY_WIDE: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_HEADPHONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_MONO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_QUAD: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_STEREO: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DSSPEAKER_SURROUND: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS_CERTIFIED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS_NO_VIRTUALIZATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(142082058i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const DS_UNCERTIFIED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[inline]
pub unsafe fn DirectSoundCaptureCreate<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcguiddevice: *const ::windows::runtime::GUID, ppdsc: *mut ::core::option::Option<IDirectSoundCapture>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureCreate(pcguiddevice: *const ::windows::runtime::GUID, ppdsc: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCaptureCreate(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppdsc), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[inline]
pub unsafe fn DirectSoundCaptureCreate8<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcguiddevice: *const ::windows::runtime::GUID, ppdsc8: *mut ::core::option::Option<IDirectSoundCapture>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureCreate8(pcguiddevice: *const ::windows::runtime::GUID, ppdsc8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCaptureCreate8(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppdsc8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateA(pdsenumcallback: ::core::option::Option<LPDSENUMCALLBACKA>, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureEnumerateA(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCaptureEnumerateA(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateW(pdsenumcallback: ::core::option::Option<LPDSENUMCALLBACKW>, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureEnumerateW(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCaptureEnumerateW(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[inline]
pub unsafe fn DirectSoundCreate<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcguiddevice: *const ::windows::runtime::GUID, ppds: *mut ::core::option::Option<IDirectSound>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCreate(pcguiddevice: *const ::windows::runtime::GUID, ppds: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCreate(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppds), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[inline]
pub unsafe fn DirectSoundCreate8<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcguiddevice: *const ::windows::runtime::GUID, ppds8: *mut ::core::option::Option<IDirectSound8>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCreate8(pcguiddevice: *const ::windows::runtime::GUID, ppds8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCreate8(::core::mem::transmute(pcguiddevice), ::core::mem::transmute(ppds8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundEnumerateA(pdsenumcallback: ::core::option::Option<LPDSENUMCALLBACKA>, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundEnumerateA(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectSoundEnumerateA(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundEnumerateW(pdsenumcallback: ::core::option::Option<LPDSENUMCALLBACKW>, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundEnumerateW(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectSoundEnumerateW(::core::mem::transmute(pdsenumcallback), ::core::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundFullDuplexCreate<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, Param9: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
    pcguidcapturedevice: *const ::windows::runtime::GUID,
    pcguidrenderdevice: *const ::windows::runtime::GUID,
    pcdscbufferdesc: *const DSCBUFFERDESC,
    pcdsbufferdesc: *const DSBUFFERDESC,
    hwnd: Param4,
    dwlevel: u32,
    ppdsfd: *mut ::core::option::Option<IDirectSoundFullDuplex>,
    ppdscbuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>,
    ppdsbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>,
    punkouter: Param9,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundFullDuplexCreate(pcguidcapturedevice: *const ::windows::runtime::GUID, pcguidrenderdevice: *const ::windows::runtime::GUID, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, ppdsfd: *mut ::windows::runtime::RawPtr, ppdscbuffer8: *mut ::windows::runtime::RawPtr, ppdsbuffer8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
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
pub const GUID_All_Objects: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2853260773, 49762, 16745, [161, 200, 35, 214, 152, 204, 115, 181]);
pub const GUID_DSCFX_CLASS_AEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3214294400, 50521, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const GUID_DSCFX_CLASS_NS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3766456383, 25341, 20064, [140, 221, 222, 167, 35, 102, 101, 181]);
pub const GUID_DSCFX_MS_AEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3454777625, 14234, 18570, [135, 101, 245, 60, 253, 54, 222, 64]);
pub const GUID_DSCFX_MS_NS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(298174267, 26345, 19361, [160, 186, 232, 20, 198, 238, 217, 45]);
pub const GUID_DSCFX_SYSTEM_AEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(472040813, 39033, 20315, [163, 137, 39, 153, 109, 220, 40, 16]);
pub const GUID_DSCFX_SYSTEM_NS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1521518638, 29300, 17686, [135, 125, 78, 238, 153, 186, 79, 208]);
pub const GUID_DSFX_STANDARD_CHORUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4024853148, 33271, 17025, [189, 145, 201, 214, 4, 169, 90, 246]);
pub const GUID_DSFX_STANDARD_COMPRESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4009828217, 16384, 16493, [135, 175, 191, 251, 63, 195, 157, 87]);
pub const GUID_DSFX_STANDARD_DISTORTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4010888336, 52509, 18510, [150, 229, 9, 207, 175, 145, 42, 33]);
pub const GUID_DSFX_STANDARD_ECHO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4013855532, 54283, 20305, [140, 207, 63, 152, 241, 178, 157, 93]);
pub const GUID_DSFX_STANDARD_FLANGER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023008658, 57304, 18034, [166, 3, 116, 32, 137, 75, 173, 152]);
pub const GUID_DSFX_STANDARD_GARGLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3674046992, 22289, 19345, [159, 227, 247, 91, 122, 226, 121, 191]);
pub const GUID_DSFX_STANDARD_I3DL2REVERB: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4019740273, 54727, 17108, [186, 77, 45, 7, 62, 46, 150, 244]);
pub const GUID_DSFX_STANDARD_PARAMEQ: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(302837129, 15348, 16755, [161, 50, 60, 180, 6, 207, 50, 49]);
pub const GUID_DSFX_WAVES_REVERB: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2281439848, 39509, 17248, [149, 170, 0, 74, 29, 157, 226, 108]);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[inline]
pub unsafe fn GetDeviceID(pguidsrc: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceID(pguidsrc: *const ::windows::runtime::GUID, pguiddest: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        GetDeviceID(::core::mem::transmute(pguidsrc), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSound(pub ::windows::runtime::IUnknown);
impl IDirectSound {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn CreateSoundBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsbufferdesc), ::core::mem::transmute(ppdsbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCAPS> {
        let mut result__: <DSCAPS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn DuplicateSoundBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSoundBuffer>>(&self, pdsbufferoriginal: Param0) -> ::windows::runtime::Result<IDirectSoundBuffer> {
        let mut result__: <IDirectSoundBuffer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdsbufferoriginal.into_param().abi(), &mut result__).from_abi::<IDirectSoundBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwlevel: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(dwlevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetSpeakerConfig(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwspeakerconfig)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSound {
    type Vtable = IDirectSound_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664468099, 18817, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
impl ::core::convert::From<IDirectSound> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSound) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSound> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSound) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSound {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSound {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscaps: *mut DSCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsbufferoriginal: ::windows::runtime::RawPtr, ppdsbufferduplicate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwspeakerconfig: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwspeakerconfig: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSound3DBuffer(pub ::windows::runtime::IUnknown);
impl IDirectSound3DBuffer {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DS3DBUFFER> {
        let mut result__: <DS3DBUFFER as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DS3DBUFFER>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwinsideconeangle), ::core::mem::transmute(pdwoutsideconeangle)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn GetConeOrientation(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetConeOutsideVolume(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetMaxDistance(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetMinDistance(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetMode(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn GetPosition(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn GetVelocity(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcds3dbuffer), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinsideconeangle), ::core::mem::transmute(dwoutsideconeangle), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lconeoutsidevolume), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(flmaxdistance), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(flmindistance), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetMode(&self, dwmode: u32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmode), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSound3DBuffer {
    type Vtable = IDirectSound3DBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664468102, 18817, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
impl ::core::convert::From<IDirectSound3DBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSound3DBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSound3DBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSound3DBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSound3DBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSound3DBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pds3dbuffer: *mut DS3DBUFFER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plconeoutsidevolume: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflmaxdistance: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflmindistance: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmode: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flmaxdistance: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flmindistance: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmode: u32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSound3DListener(pub ::windows::runtime::IUnknown);
impl IDirectSound3DListener {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DS3DLISTENER> {
        let mut result__: <DS3DLISTENER as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DS3DLISTENER>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetDistanceFactor(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetDopplerFactor(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn GetOrientation(&self, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvorientfront), ::core::mem::transmute(pvorienttop)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn GetPosition(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetRolloffFactor(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn GetVelocity(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclistener), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(fldistancefactor), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(fldopplerfactor), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(xfront), ::core::mem::transmute(yfront), ::core::mem::transmute(zfront), ::core::mem::transmute(xtop), ::core::mem::transmute(ytop), ::core::mem::transmute(ztop), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(flrollofffactor), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn CommitDeferredSettings(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSound3DListener {
    type Vtable = IDirectSound3DListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664468100, 18817, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
impl ::core::convert::From<IDirectSound3DListener> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSound3DListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSound3DListener> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSound3DListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSound3DListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSound3DListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DListener_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plistener: *mut DS3DLISTENER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfldistancefactor: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfldopplerfactor: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflrollofffactor: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fldistancefactor: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fldopplerfactor: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flrollofffactor: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSound8(pub ::windows::runtime::IUnknown);
impl IDirectSound8 {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn CreateSoundBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsbufferdesc), ::core::mem::transmute(ppdsbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCAPS> {
        let mut result__: <DSCAPS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn DuplicateSoundBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSoundBuffer>>(&self, pdsbufferoriginal: Param0) -> ::windows::runtime::Result<IDirectSoundBuffer> {
        let mut result__: <IDirectSoundBuffer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdsbufferoriginal.into_param().abi(), &mut result__).from_abi::<IDirectSoundBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwlevel: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(dwlevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetSpeakerConfig(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwspeakerconfig)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn VerifyCertification(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSound8 {
    type Vtable = IDirectSound8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3305799315, 62357, 18484, [158, 246, 127, 169, 157, 229, 9, 102]);
}
impl ::core::convert::From<IDirectSound8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSound8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSound8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSound8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSound8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSound8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSound> for IDirectSound8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSound> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSound> for &IDirectSound8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSound> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscaps: *mut DSCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsbufferoriginal: ::windows::runtime::RawPtr, ppdsbufferduplicate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwspeakerconfig: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwspeakerconfig: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcertified: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundBuffer(pub ::windows::runtime::IUnknown);
impl IDirectSoundBuffer {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSBCAPS> {
        let mut result__: <DSBCAPS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSBCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcurrentplaycursor), ::core::mem::transmute(pdwcurrentwritecursor)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetVolume(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetPan(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetFrequency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSound>>(&self, pdirectsound: Param0, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), ::core::mem::transmute(pcdsbufferdesc)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved1), ::core::mem::transmute(dwpriority), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwnewposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcfxformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lvolume)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetPan(&self, lpan: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpan)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwfrequency)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundBuffer {
    type Vtable = IDirectSoundBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664468101, 18817, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
impl ::core::convert::From<IDirectSoundBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsbuffercaps: *mut DSBCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plvolume: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpan: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfrequency: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwnewposition: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lvolume: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpan: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfrequency: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundBuffer8(pub ::windows::runtime::IUnknown);
impl IDirectSoundBuffer8 {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSBCAPS> {
        let mut result__: <DSBCAPS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSBCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcurrentplaycursor), ::core::mem::transmute(pdwcurrentwritecursor)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetVolume(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetPan(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetFrequency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSound>>(&self, pdirectsound: Param0, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pdirectsound.into_param().abi(), ::core::mem::transmute(pcdsbufferdesc)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved1), ::core::mem::transmute(dwpriority), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwnewposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcfxformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(lvolume)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetPan(&self, lpan: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpan)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwfrequency)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetFX(&self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dweffectscount), ::core::mem::transmute(pdsfxdesc), ::core::mem::transmute(pdwresultcodes)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn AcquireResources(&self, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dweffectscount), ::core::mem::transmute(pdwresultcodes)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const ::windows::runtime::GUID, dwindex: u32, rguidinterface: *const ::windows::runtime::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidobject), ::core::mem::transmute(dwindex), ::core::mem::transmute(rguidinterface), ::core::mem::transmute(ppobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundBuffer8 {
    type Vtable = IDirectSoundBuffer8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1747297353, 29988, 19842, [146, 15, 80, 227, 106, 179, 171, 30]);
}
impl ::core::convert::From<IDirectSoundBuffer8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundBuffer8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundBuffer8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundBuffer8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSoundBuffer> for IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSoundBuffer> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSoundBuffer> for &IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSoundBuffer> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsbuffercaps: *mut DSBCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plvolume: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpan: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfrequency: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwnewposition: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lvolume: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpan: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfrequency: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidobject: *const ::windows::runtime::GUID, dwindex: u32, rguidinterface: *const ::windows::runtime::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCapture(pub ::windows::runtime::IUnknown);
impl IDirectSoundCapture {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn CreateCaptureBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::core::option::Option<IDirectSoundCaptureBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdscbufferdesc), ::core::mem::transmute(ppdscbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCCAPS> {
        let mut result__: <DSCCAPS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcguiddevice)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCapture {
    type Vtable = IDirectSoundCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2954954625, 35277, 4560, [175, 8, 0, 160, 201, 37, 205, 22]);
}
impl ::core::convert::From<IDirectSoundCapture> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCapture) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCapture> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCapture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCapture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsccaps: *mut DSCCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCaptureBuffer(pub ::windows::runtime::IUnknown);
impl IDirectSoundCaptureBuffer {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCBCAPS> {
        let mut result__: <DSCBCAPS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCBCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcaptureposition), ::core::mem::transmute(pdwreadposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSoundCapture>>(&self, pdirectsoundcapture: Param0, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdirectsoundcapture.into_param().abi(), ::core::mem::transmute(pcdscbufferdesc)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Start(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCaptureBuffer {
    type Vtable = IDirectSoundCaptureBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2954954626, 35277, 4560, [175, 8, 0, 160, 201, 37, 205, 22]);
}
impl ::core::convert::From<IDirectSoundCaptureBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCaptureBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCaptureBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCaptureBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCaptureBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCaptureBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscbcaps: *mut DSCBCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsoundcapture: ::windows::runtime::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCaptureBuffer8(pub ::windows::runtime::IUnknown);
impl IDirectSoundCaptureBuffer8 {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCBCAPS> {
        let mut result__: <DSCBCAPS as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCBCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcaptureposition), ::core::mem::transmute(pdwreadposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwfxformat), ::core::mem::transmute(dwsizeallocated), ::core::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSoundCapture>>(&self, pdirectsoundcapture: Param0, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdirectsoundcapture.into_param().abi(), ::core::mem::transmute(pcdscbufferdesc)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoffset), ::core::mem::transmute(dwbytes), ::core::mem::transmute(ppvaudioptr1), ::core::mem::transmute(pdwaudiobytes1), ::core::mem::transmute(ppvaudioptr2), ::core::mem::transmute(pdwaudiobytes2), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Start(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvaudioptr1), ::core::mem::transmute(dwaudiobytes1), ::core::mem::transmute(pvaudioptr2), ::core::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const ::windows::runtime::GUID, dwindex: u32, rguidinterface: *const ::windows::runtime::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidobject), ::core::mem::transmute(dwindex), ::core::mem::transmute(rguidinterface), ::core::mem::transmute(ppobject)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetFXStatus(&self, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dweffectscount), ::core::mem::transmute(pdwfxstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCaptureBuffer8 {
    type Vtable = IDirectSoundCaptureBuffer8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(10030580, 3515, 18546, [131, 62, 109, 48, 62, 128, 174, 182]);
}
impl ::core::convert::From<IDirectSoundCaptureBuffer8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCaptureBuffer8) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCaptureBuffer8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCaptureBuffer8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSoundCaptureBuffer> for IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSoundCaptureBuffer> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSoundCaptureBuffer> for &IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSoundCaptureBuffer> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscbcaps: *mut DSCBCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsoundcapture: ::windows::runtime::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidobject: *const ::windows::runtime::GUID, dwindex: u32, rguidinterface: *const ::windows::runtime::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCaptureFXAec(pub ::windows::runtime::IUnknown);
impl IDirectSoundCaptureFXAec {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    pub unsafe fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdscfxaec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSCFXAec> {
        let mut result__: <DSCFXAec as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCFXAec>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCaptureFXAec {
    type Vtable = IDirectSoundCaptureFXAec_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2910065725, 36925, 19127, [128, 102, 40, 211, 99, 3, 109, 101]);
}
impl ::core::convert::From<IDirectSoundCaptureFXAec> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCaptureFXAec) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCaptureFXAec> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCaptureFXAec) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCaptureFXAec {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCaptureFXAec {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXAec_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscfxaec: *const DSCFXAec) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscfxaec: *mut DSCFXAec) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundCaptureFXNoiseSuppress(pub ::windows::runtime::IUnknown);
impl IDirectSoundCaptureFXNoiseSuppress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    pub unsafe fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdscfxnoisesuppress)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSCFXNoiseSuppress> {
        let mut result__: <DSCFXNoiseSuppress as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSCFXNoiseSuppress>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCaptureFXNoiseSuppress {
    type Vtable = IDirectSoundCaptureFXNoiseSuppress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3979419201, 64430, 16757, [150, 37, 205, 8, 84, 246, 147, 202]);
}
impl ::core::convert::From<IDirectSoundCaptureFXNoiseSuppress> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCaptureFXNoiseSuppress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundCaptureFXNoiseSuppress> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCaptureFXNoiseSuppress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCaptureFXNoiseSuppress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCaptureFXNoiseSuppress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXNoiseSuppress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXChorus(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXChorus {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxchorus)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXChorus> {
        let mut result__: <DSFXChorus as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXChorus>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXChorus {
    type Vtable = IDirectSoundFXChorus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2282242787, 5215, 17382, [169, 52, 167, 24, 6, 229, 5, 71]);
}
impl ::core::convert::From<IDirectSoundFXChorus> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXChorus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXChorus> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXChorus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXChorus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXChorus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXChorus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxchorus: *const DSFXChorus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxchorus: *mut DSFXChorus) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXCompressor(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXCompressor {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxcompressor)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXCompressor> {
        let mut result__: <DSFXCompressor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXCompressor>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXCompressor {
    type Vtable = IDirectSoundFXCompressor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1270681940, 25334, 20012, [161, 92, 211, 182, 196, 23, 247, 160]);
}
impl ::core::convert::From<IDirectSoundFXCompressor> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXCompressor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXCompressor> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXCompressor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXCompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXCompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXCompressor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxcompressor: *mut DSFXCompressor) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXDistortion(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXDistortion {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxdistortion)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXDistortion> {
        let mut result__: <DSFXDistortion as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXDistortion>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXDistortion {
    type Vtable = IDirectSoundFXDistortion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2395947814, 17759, 19851, [189, 169, 141, 93, 62, 158, 62, 11]);
}
impl ::core::convert::From<IDirectSoundFXDistortion> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXDistortion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXDistortion> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXDistortion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXDistortion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXDistortion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXDistortion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxdistortion: *mut DSFXDistortion) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXEcho(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXEcho {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxecho)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXEcho> {
        let mut result__: <DSFXEcho as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXEcho>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXEcho {
    type Vtable = IDirectSoundFXEcho_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2345832159, 20699, 20114, [162, 189, 68, 84, 136, 209, 237, 66]);
}
impl ::core::convert::From<IDirectSoundFXEcho> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXEcho) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXEcho> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXEcho) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXEcho {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXEcho {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXEcho_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxecho: *const DSFXEcho) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxecho: *mut DSFXEcho) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXFlanger(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXFlanger {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxflanger)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXFlanger> {
        let mut result__: <DSFXFlanger as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXFlanger>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXFlanger {
    type Vtable = IDirectSoundFXFlanger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2420021368, 11410, 16498, [155, 44, 234, 104, 245, 57, 103, 131]);
}
impl ::core::convert::From<IDirectSoundFXFlanger> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXFlanger) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXFlanger> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXFlanger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXFlanger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXFlanger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXFlanger_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxflanger: *const DSFXFlanger) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxflanger: *mut DSFXFlanger) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXGargle(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXGargle {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxgargle)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXGargle> {
        let mut result__: <DSFXGargle as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXGargle>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXGargle {
    type Vtable = IDirectSoundFXGargle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3591828306, 54818, 4558, [170, 197, 0, 32, 175, 11, 153, 163]);
}
impl ::core::convert::From<IDirectSoundFXGargle> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXGargle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXGargle> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXGargle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXGargle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXGargle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXGargle_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxgargle: *const DSFXGargle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxgargle: *mut DSFXGargle) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXI3DL2Reverb(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXI3DL2Reverb {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxi3dl2reverb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXI3DL2Reverb> {
        let mut result__: <DSFXI3DL2Reverb as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXI3DL2Reverb>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetPreset(&self, dwpreset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpreset)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetPreset(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetQuality(&self, lquality: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(lquality)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetQuality(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXI3DL2Reverb {
    type Vtable = IDirectSoundFXI3DL2Reverb_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1259760234, 3430, 17395, [128, 227, 238, 98, 128, 222, 225, 164]);
}
impl ::core::convert::From<IDirectSoundFXI3DL2Reverb> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXI3DL2Reverb) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXI3DL2Reverb> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXI3DL2Reverb) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXI3DL2Reverb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXI3DL2Reverb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXI3DL2Reverb_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpreset: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpreset: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lquality: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plquality: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXParamEq(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXParamEq {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxparameq)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXParamEq> {
        let mut result__: <DSFXParamEq as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXParamEq>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXParamEq {
    type Vtable = IDirectSoundFXParamEq_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3225201150, 65168, 16900, [128, 120, 130, 51, 76, 209, 119, 218]);
}
impl ::core::convert::From<IDirectSoundFXParamEq> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXParamEq) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXParamEq> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXParamEq) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXParamEq {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXParamEq {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXParamEq_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxparameq: *const DSFXParamEq) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxparameq: *mut DSFXParamEq) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFXWavesReverb(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXWavesReverb {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcdsfxwavesreverb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXWavesReverb> {
        let mut result__: <DSFXWavesReverb as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DSFXWavesReverb>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXWavesReverb {
    type Vtable = IDirectSoundFXWavesReverb_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1183157306, 3526, 17891, [183, 96, 212, 238, 241, 108, 179, 37]);
}
impl ::core::convert::From<IDirectSoundFXWavesReverb> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXWavesReverb) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFXWavesReverb> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXWavesReverb) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXWavesReverb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXWavesReverb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXWavesReverb_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundFullDuplex(pub ::windows::runtime::IUnknown);
impl IDirectSoundFullDuplex {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(
        &self,
        pcaptureguid: *const ::windows::runtime::GUID,
        prenderguid: *const ::windows::runtime::GUID,
        lpdscbufferdesc: *const DSCBUFFERDESC,
        lpdsbufferdesc: *const DSBUFFERDESC,
        hwnd: Param4,
        dwlevel: u32,
        lplpdirectsoundcapturebuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>,
        lplpdirectsoundbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
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
unsafe impl ::windows::runtime::Interface for IDirectSoundFullDuplex {
    type Vtable = IDirectSoundFullDuplex_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3989523578, 55979, 16918, [164, 46, 108, 80, 89, 109, 220, 29]);
}
impl ::core::convert::From<IDirectSoundFullDuplex> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFullDuplex) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundFullDuplex> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFullDuplex) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFullDuplex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFullDuplex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFullDuplex_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcaptureguid: *const ::windows::runtime::GUID, prenderguid: *const ::windows::runtime::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::windows::runtime::RawPtr, lplpdirectsoundbuffer8: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirectSoundNotify(pub ::windows::runtime::IUnknown);
impl IDirectSoundNotify {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    pub unsafe fn SetNotificationPositions(&self, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpositionnotifies), ::core::mem::transmute(pcpositionnotifies)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundNotify {
    type Vtable = IDirectSoundNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2954954627, 35277, 4560, [175, 8, 0, 160, 201, 37, 205, 22]);
}
impl ::core::convert::From<IDirectSoundNotify> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirectSoundNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const KSPROPERTY_SUPPORT_GET: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const KSPROPERTY_SUPPORT_SET: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKA = unsafe extern "system" fn(param0: *mut ::windows::runtime::GUID, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKW = unsafe extern "system" fn(param0: *mut ::windows::runtime::GUID, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
pub const _FACDS: u32 = 2168u32;
