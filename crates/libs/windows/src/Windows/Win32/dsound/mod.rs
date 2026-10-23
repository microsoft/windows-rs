#[cfg(all(feature = "guiddef", feature = "unknwnbase"))]
#[inline]
pub unsafe fn DirectSoundCaptureCreate(pcguiddevice: Option<super::LPCGUID>, ppdsc: *mut Option<LPDIRECTSOUNDCAPTURE>, punkouter: &Option<super::LPUNKNOWN>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundCaptureCreate(pcguiddevice : super::LPCGUID, ppdsc : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCaptureCreate(pcguiddevice.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppdsc), core::mem::transmute_copy(punkouter)) }
}
#[cfg(all(feature = "guiddef", feature = "unknwnbase"))]
#[inline]
pub unsafe fn DirectSoundCaptureCreate8(pcguiddevice: Option<super::LPCGUID>, ppdsc8: *mut Option<LPDIRECTSOUNDCAPTURE8>, punkouter: &Option<super::LPUNKNOWN>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundCaptureCreate8(pcguiddevice : super::LPCGUID, ppdsc8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCaptureCreate8(pcguiddevice.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppdsc8), core::mem::transmute_copy(punkouter)) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: Option<super::LPVOID>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : super::LPVOID) -> windows_core::HRESULT);
    unsafe { DirectSoundCaptureEnumerateA(pdsenumcallback, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: Option<super::LPVOID>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : super::LPVOID) -> windows_core::HRESULT);
    unsafe { DirectSoundCaptureEnumerateW(pdsenumcallback, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "unknwnbase"))]
#[inline]
pub unsafe fn DirectSoundCreate(pcguiddevice: Option<super::LPCGUID>, ppds: *mut Option<LPDIRECTSOUND>, punkouter: &Option<super::LPUNKNOWN>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundCreate(pcguiddevice : super::LPCGUID, ppds : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCreate(pcguiddevice.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppds), core::mem::transmute_copy(punkouter)) }
}
#[cfg(all(feature = "guiddef", feature = "unknwnbase"))]
#[inline]
pub unsafe fn DirectSoundCreate8(pcguiddevice: Option<super::LPCGUID>, ppds8: *mut Option<LPDIRECTSOUND8>, punkouter: &Option<super::LPUNKNOWN>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundCreate8(pcguiddevice : super::LPCGUID, ppds8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCreate8(pcguiddevice.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppds8), core::mem::transmute_copy(punkouter)) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
#[inline]
pub unsafe fn DirectSoundEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: Option<super::LPVOID>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : super::LPVOID) -> windows_core::HRESULT);
    unsafe { DirectSoundEnumerateA(pdsenumcallback, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
#[inline]
pub unsafe fn DirectSoundEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: Option<super::LPVOID>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : super::LPVOID) -> windows_core::HRESULT);
    unsafe { DirectSoundEnumerateW(pdsenumcallback, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[inline]
pub unsafe fn DirectSoundFullDuplexCreate(pcguidcapturedevice: Option<super::LPCGUID>, pcguidrenderdevice: Option<super::LPCGUID>, pcdscbufferdesc: LPCDSCBUFFERDESC, pcdsbufferdesc: LPCDSBUFFERDESC, hwnd: super::HWND, dwlevel: super::DWORD, ppdsfd: *mut Option<LPDIRECTSOUNDFULLDUPLEX>, ppdscbuffer8: *mut Option<LPDIRECTSOUNDCAPTUREBUFFER8>, ppdsbuffer8: *mut Option<LPDIRECTSOUNDBUFFER8>, punkouter: &Option<super::LPUNKNOWN>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundFullDuplexCreate(pcguidcapturedevice : super::LPCGUID, pcguidrenderdevice : super::LPCGUID, pcdscbufferdesc : LPCDSCBUFFERDESC, pcdsbufferdesc : LPCDSBUFFERDESC, hwnd : super::HWND, dwlevel : super::DWORD, ppdsfd : *mut *mut core::ffi::c_void, ppdscbuffer8 : *mut *mut core::ffi::c_void, ppdsbuffer8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundFullDuplexCreate(pcguidcapturedevice.unwrap_or(core::mem::zeroed()) as _, pcguidrenderdevice.unwrap_or(core::mem::zeroed()) as _, pcdscbufferdesc, pcdsbufferdesc, hwnd as _, dwlevel, core::mem::transmute(ppdsfd), core::mem::transmute(ppdscbuffer8), core::mem::transmute(ppdsbuffer8), core::mem::transmute_copy(punkouter)) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn GetDeviceID(pguidsrc: Option<super::LPCGUID>) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("dsound.dll" "system" fn GetDeviceID(pguidsrc : super::LPCGUID, pguiddest : super::LPGUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetDeviceID(pguidsrc.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
pub const CLSID_DirectSound: windows_core::GUID = windows_core::GUID::from_u128(0x47d4d946_62e8_11cf_93bc_444553540000);
pub const CLSID_DirectSound8: windows_core::GUID = windows_core::GUID::from_u128(0x3901cc3f_84b5_4fa4_ba35_aa8172b8a09b);
pub const CLSID_DirectSoundCapture: windows_core::GUID = windows_core::GUID::from_u128(0xb0210780_89cd_11d0_af08_00a0c925cd16);
pub const CLSID_DirectSoundCapture8: windows_core::GUID = windows_core::GUID::from_u128(0xe4bcac13_7f99_4908_9a8e_74e3bf24b6e1);
pub const CLSID_DirectSoundFullDuplex: windows_core::GUID = windows_core::GUID::from_u128(0xfea4300c_7959_4147_b26a_2377b9e7a91d);
pub type D3DCOLOR = u32;
pub type D3DVALUE = f32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVECTOR {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub const DIRECTSOUND_VERSION: i32 = 2304;
pub const DS3DALG_DEFAULT: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const DS3DALG_HRTF_FULL: windows_core::GUID = windows_core::GUID::from_u128(0xc2413340_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_HRTF_LIGHT: windows_core::GUID = windows_core::GUID::from_u128(0xc2413342_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_NO_VIRTUALIZATION: windows_core::GUID = windows_core::GUID::from_u128(0xc241333f_1c1b_11d2_94f5_00c04fc28aca);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS3DBUFFER {
    pub dwSize: super::DWORD,
    pub vPosition: D3DVECTOR,
    pub vVelocity: D3DVECTOR,
    pub dwInsideConeAngle: super::DWORD,
    pub dwOutsideConeAngle: super::DWORD,
    pub vConeOrientation: D3DVECTOR,
    pub lConeOutsideVolume: super::LONG,
    pub flMinDistance: D3DVALUE,
    pub flMaxDistance: D3DVALUE,
    pub dwMode: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS3DLISTENER {
    pub dwSize: super::DWORD,
    pub vPosition: D3DVECTOR,
    pub vVelocity: D3DVECTOR,
    pub vOrientFront: D3DVECTOR,
    pub vOrientTop: D3DVECTOR,
    pub flDistanceFactor: D3DVALUE,
    pub flRolloffFactor: D3DVALUE,
    pub flDopplerFactor: D3DVALUE,
}
pub const DS3DMODE_DISABLE: i32 = 2;
pub const DS3DMODE_HEADRELATIVE: i32 = 1;
pub const DS3DMODE_NORMAL: i32 = 0;
pub const DS3D_DEFAULTCONEANGLE: i32 = 360;
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: i32 = 0;
pub const DS3D_DEFAULTDISTANCEFACTOR: f32 = 1.0;
pub const DS3D_DEFAULTDOPPLERFACTOR: f32 = 1.0;
pub const DS3D_DEFAULTMAXDISTANCE: f32 = 1000000000.0;
pub const DS3D_DEFAULTMINDISTANCE: f32 = 1.0;
pub const DS3D_DEFAULTROLLOFFFACTOR: f32 = 1.0;
pub const DS3D_DEFERRED: i32 = 1;
pub const DS3D_IMMEDIATE: i32 = 0;
pub const DS3D_MAXCONEANGLE: i32 = 360;
pub const DS3D_MAXDISTANCEFACTOR: f32 = 340282350000000000000000000000000000000.0;
pub const DS3D_MAXDOPPLERFACTOR: f32 = 10.0;
pub const DS3D_MAXROLLOFFFACTOR: f32 = 10.0;
pub const DS3D_MINCONEANGLE: i32 = 0;
pub const DS3D_MINDISTANCEFACTOR: f32 = 0.000000000000000000000000000000000000011754944;
pub const DS3D_MINDOPPLERFACTOR: f32 = 0.0;
pub const DS3D_MINROLLOFFFACTOR: f32 = 0.0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSBCAPS {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwBufferBytes: super::DWORD,
    pub dwUnlockTransferRate: super::DWORD,
    pub dwPlayCpuOverhead: super::DWORD,
}
pub const DSBCAPS_CTRL3D: i32 = 16;
pub const DSBCAPS_CTRLFREQUENCY: i32 = 32;
pub const DSBCAPS_CTRLFX: i32 = 512;
pub const DSBCAPS_CTRLPAN: i32 = 64;
pub const DSBCAPS_CTRLPOSITIONNOTIFY: i32 = 256;
pub const DSBCAPS_CTRLVOLUME: i32 = 128;
pub const DSBCAPS_GETCURRENTPOSITION2: i32 = 65536;
pub const DSBCAPS_GLOBALFOCUS: i32 = 32768;
pub const DSBCAPS_LOCDEFER: i32 = 262144;
pub const DSBCAPS_LOCHARDWARE: i32 = 4;
pub const DSBCAPS_LOCSOFTWARE: i32 = 8;
pub const DSBCAPS_MUTE3DATMAXDISTANCE: i32 = 131072;
pub const DSBCAPS_PRIMARYBUFFER: i32 = 1;
pub const DSBCAPS_STATIC: i32 = 2;
pub const DSBCAPS_STICKYFOCUS: i32 = 16384;
pub const DSBCAPS_TRUEPLAYPOSITION: i32 = 524288;
pub const DSBFREQUENCY_MAX: i32 = 200000;
pub const DSBFREQUENCY_MIN: i32 = 100;
pub const DSBFREQUENCY_ORIGINAL: i32 = 0;
pub const DSBLOCK_ENTIREBUFFER: i32 = 2;
pub const DSBLOCK_FROMWRITECURSOR: i32 = 1;
pub const DSBNOTIFICATIONS_MAX: u32 = 100000;
pub const DSBPAN_CENTER: i32 = 0;
pub const DSBPAN_LEFT: i32 = -10000;
pub const DSBPAN_RIGHT: i32 = 10000;
pub const DSBPLAY_LOCHARDWARE: i32 = 2;
pub const DSBPLAY_LOCSOFTWARE: i32 = 4;
pub const DSBPLAY_LOOPING: i32 = 1;
pub const DSBPLAY_TERMINATEBY_DISTANCE: i32 = 16;
pub const DSBPLAY_TERMINATEBY_PRIORITY: i32 = 32;
pub const DSBPLAY_TERMINATEBY_TIME: i32 = 8;
pub const DSBPN_OFFSETSTOP: u32 = 4294967295;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: super::DWORD,
    pub hEventNotify: super::HANDLE,
}
pub const DSBSIZE_FX_MIN: i32 = 150;
pub const DSBSIZE_MAX: i32 = 268435455;
pub const DSBSIZE_MIN: i32 = 4;
pub const DSBSTATUS_BUFFERLOST: i32 = 2;
pub const DSBSTATUS_LOCHARDWARE: i32 = 8;
pub const DSBSTATUS_LOCSOFTWARE: i32 = 16;
pub const DSBSTATUS_LOOPING: i32 = 4;
pub const DSBSTATUS_PLAYING: i32 = 1;
pub const DSBSTATUS_TERMINATED: i32 = 32;
#[repr(C)]
#[cfg(all(feature = "mmeapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSBUFFERDESC {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwBufferBytes: super::DWORD,
    pub dwReserved: super::DWORD,
    pub lpwfxFormat: super::LPWAVEFORMATEX,
    pub guid3DAlgorithm: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "mmeapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSBUFFERDESC1 {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwBufferBytes: super::DWORD,
    pub dwReserved: super::DWORD,
    pub lpwfxFormat: super::LPWAVEFORMATEX,
}
pub const DSBVOLUME_MAX: i32 = 0;
pub const DSBVOLUME_MIN: i32 = -10000;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSCAPS {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwMinSecondarySampleRate: super::DWORD,
    pub dwMaxSecondarySampleRate: super::DWORD,
    pub dwPrimaryBuffers: super::DWORD,
    pub dwMaxHwMixingAllBuffers: super::DWORD,
    pub dwMaxHwMixingStaticBuffers: super::DWORD,
    pub dwMaxHwMixingStreamingBuffers: super::DWORD,
    pub dwFreeHwMixingAllBuffers: super::DWORD,
    pub dwFreeHwMixingStaticBuffers: super::DWORD,
    pub dwFreeHwMixingStreamingBuffers: super::DWORD,
    pub dwMaxHw3DAllBuffers: super::DWORD,
    pub dwMaxHw3DStaticBuffers: super::DWORD,
    pub dwMaxHw3DStreamingBuffers: super::DWORD,
    pub dwFreeHw3DAllBuffers: super::DWORD,
    pub dwFreeHw3DStaticBuffers: super::DWORD,
    pub dwFreeHw3DStreamingBuffers: super::DWORD,
    pub dwTotalHwMemBytes: super::DWORD,
    pub dwFreeHwMemBytes: super::DWORD,
    pub dwMaxContigFreeHwMemBytes: super::DWORD,
    pub dwUnlockTransferRateHwBuffers: super::DWORD,
    pub dwPlayCpuOverheadSwBuffers: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
}
pub const DSCAPS_CERTIFIED: i32 = 64;
pub const DSCAPS_CONTINUOUSRATE: i32 = 16;
pub const DSCAPS_EMULDRIVER: i32 = 32;
pub const DSCAPS_PRIMARY16BIT: i32 = 8;
pub const DSCAPS_PRIMARY8BIT: i32 = 4;
pub const DSCAPS_PRIMARYMONO: i32 = 1;
pub const DSCAPS_PRIMARYSTEREO: i32 = 2;
pub const DSCAPS_SECONDARY16BIT: i32 = 2048;
pub const DSCAPS_SECONDARY8BIT: i32 = 1024;
pub const DSCAPS_SECONDARYMONO: i32 = 256;
pub const DSCAPS_SECONDARYSTEREO: i32 = 512;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSCBCAPS {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwBufferBytes: super::DWORD,
    pub dwReserved: super::DWORD,
}
pub const DSCBCAPS_CTRLFX: i32 = 512;
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648;
pub const DSCBLOCK_ENTIREBUFFER: i32 = 1;
pub const DSCBSTART_LOOPING: i32 = 1;
pub const DSCBSTATUS_CAPTURING: i32 = 1;
pub const DSCBSTATUS_LOOPING: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "mmeapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSCBUFFERDESC {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwBufferBytes: super::DWORD,
    pub dwReserved: super::DWORD,
    pub lpwfxFormat: super::LPWAVEFORMATEX,
    pub dwFXCount: super::DWORD,
    pub lpDSCFXDesc: LPDSCEFFECTDESC,
}
#[repr(C)]
#[cfg(all(feature = "mmeapi", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSCBUFFERDESC1 {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwBufferBytes: super::DWORD,
    pub dwReserved: super::DWORD,
    pub lpwfxFormat: super::LPWAVEFORMATEX,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSCCAPS {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwFormats: super::DWORD,
    pub dwChannels: super::DWORD,
}
pub const DSCCAPS_CERTIFIED: i32 = 64;
pub const DSCCAPS_EMULDRIVER: i32 = 32;
pub const DSCCAPS_MULTIPLECAPTURE: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSCEFFECTDESC {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub guidDSCFXClass: windows_core::GUID,
    pub guidDSCFXInstance: windows_core::GUID,
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSCFXAec {
    pub fEnable: windows_core::BOOL,
    pub fNoiseFill: windows_core::BOOL,
    pub dwMode: super::DWORD,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSCFXNoiseSuppress {
    pub fEnable: windows_core::BOOL,
}
pub const DSCFXR_LOCHARDWARE: i32 = 16;
pub const DSCFXR_LOCSOFTWARE: i32 = 32;
pub const DSCFX_AEC_MODE_FULL_DUPLEX: i32 = 2;
pub const DSCFX_AEC_MODE_HALF_DUPLEX: i32 = 1;
pub const DSCFX_AEC_MODE_PASS_THROUGH: i32 = 0;
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: i32 = 8;
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: i32 = 1;
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: i32 = 2;
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: i32 = 0;
pub const DSCFX_LOCHARDWARE: i32 = 1;
pub const DSCFX_LOCSOFTWARE: i32 = 2;
pub const DSDEVID_DefaultCapture: windows_core::GUID = windows_core::GUID::from_u128(0xdef00001_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultPlayback: windows_core::GUID = windows_core::GUID::from_u128(0xdef00000_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoiceCapture: windows_core::GUID = windows_core::GUID::from_u128(0xdef00003_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoicePlayback: windows_core::GUID = windows_core::GUID::from_u128(0xdef00002_9c6d_47ed_aaf1_4dda8f2b5c03);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSEFFECTDESC {
    pub dwSize: super::DWORD,
    pub dwFlags: super::DWORD,
    pub guidDSFXClass: windows_core::GUID,
    pub dwReserved1: super::DWORD_PTR,
    pub dwReserved2: super::DWORD_PTR,
}
pub const DSERR_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80070005_u32 as _);
pub const DSERR_ALLOCATED: windows_core::HRESULT = windows_core::HRESULT(0x8878000A_u32 as _);
pub const DSERR_ALREADYINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x88780082_u32 as _);
pub const DSERR_BADFORMAT: windows_core::HRESULT = windows_core::HRESULT(0x88780064_u32 as _);
pub const DSERR_BADSENDBUFFERGUID: windows_core::HRESULT = windows_core::HRESULT(0x887800D2_u32 as _);
pub const DSERR_BUFFERLOST: windows_core::HRESULT = windows_core::HRESULT(0x88780096_u32 as _);
pub const DSERR_BUFFERTOOSMALL: windows_core::HRESULT = windows_core::HRESULT(0x887800B4_u32 as _);
pub const DSERR_CONTROLUNAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x8878001E_u32 as _);
pub const DSERR_DS8_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x887800BE_u32 as _);
pub const DSERR_FXUNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x887800DC_u32 as _);
pub const DSERR_GENERIC: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
pub const DSERR_INVALIDCALL: windows_core::HRESULT = windows_core::HRESULT(0x88780032_u32 as _);
pub const DSERR_INVALIDPARAM: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
pub const DSERR_NOAGGREGATION: windows_core::HRESULT = windows_core::HRESULT(0x80040110_u32 as _);
pub const DSERR_NODRIVER: windows_core::HRESULT = windows_core::HRESULT(0x88780078_u32 as _);
pub const DSERR_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80004002_u32 as _);
pub const DSERR_OBJECTNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x88781161_u32 as _);
pub const DSERR_OTHERAPPHASPRIO: windows_core::HRESULT = windows_core::HRESULT(0x887800A0_u32 as _);
pub const DSERR_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8007000E_u32 as _);
pub const DSERR_PRIOLEVELNEEDED: windows_core::HRESULT = windows_core::HRESULT(0x88780046_u32 as _);
pub const DSERR_SENDLOOP: windows_core::HRESULT = windows_core::HRESULT(0x887800C8_u32 as _);
pub const DSERR_UNINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x887800AA_u32 as _);
pub const DSERR_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80004001_u32 as _);
pub const DSFXCHORUS_DELAY_MAX: f32 = 20.0;
pub const DSFXCHORUS_DELAY_MIN: f32 = 0.0;
pub const DSFXCHORUS_DEPTH_MAX: f32 = 100.0;
pub const DSFXCHORUS_DEPTH_MIN: f32 = 0.0;
pub const DSFXCHORUS_FEEDBACK_MAX: f32 = 99.0;
pub const DSFXCHORUS_FEEDBACK_MIN: f32 = -99.0;
pub const DSFXCHORUS_FREQUENCY_MAX: f32 = 10.0;
pub const DSFXCHORUS_FREQUENCY_MIN: f32 = 0.0;
pub const DSFXCHORUS_PHASE_180: i32 = 4;
pub const DSFXCHORUS_PHASE_90: i32 = 3;
pub const DSFXCHORUS_PHASE_MAX: i32 = 4;
pub const DSFXCHORUS_PHASE_MIN: i32 = 0;
pub const DSFXCHORUS_PHASE_NEG_180: i32 = 0;
pub const DSFXCHORUS_PHASE_NEG_90: i32 = 1;
pub const DSFXCHORUS_PHASE_ZERO: i32 = 2;
pub const DSFXCHORUS_WAVE_SIN: i32 = 1;
pub const DSFXCHORUS_WAVE_TRIANGLE: i32 = 0;
pub const DSFXCHORUS_WETDRYMIX_MAX: f32 = 100.0;
pub const DSFXCHORUS_WETDRYMIX_MIN: f32 = 0.0;
pub const DSFXCOMPRESSOR_ATTACK_MAX: f32 = 500.0;
pub const DSFXCOMPRESSOR_ATTACK_MIN: f32 = 0.01;
pub const DSFXCOMPRESSOR_GAIN_MAX: f32 = 60.0;
pub const DSFXCOMPRESSOR_GAIN_MIN: f32 = -60.0;
pub const DSFXCOMPRESSOR_PREDELAY_MAX: f32 = 4.0;
pub const DSFXCOMPRESSOR_PREDELAY_MIN: f32 = 0.0;
pub const DSFXCOMPRESSOR_RATIO_MAX: f32 = 100.0;
pub const DSFXCOMPRESSOR_RATIO_MIN: f32 = 1.0;
pub const DSFXCOMPRESSOR_RELEASE_MAX: f32 = 3000.0;
pub const DSFXCOMPRESSOR_RELEASE_MIN: f32 = 50.0;
pub const DSFXCOMPRESSOR_THRESHOLD_MAX: f32 = 0.0;
pub const DSFXCOMPRESSOR_THRESHOLD_MIN: f32 = -60.0;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXChorus {
    pub fWetDryMix: super::FLOAT,
    pub fDepth: super::FLOAT,
    pub fFeedback: super::FLOAT,
    pub fFrequency: super::FLOAT,
    pub lWaveform: super::LONG,
    pub fDelay: super::FLOAT,
    pub lPhase: super::LONG,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXCompressor {
    pub fGain: super::FLOAT,
    pub fAttack: super::FLOAT,
    pub fRelease: super::FLOAT,
    pub fThreshold: super::FLOAT,
    pub fRatio: super::FLOAT,
    pub fPredelay: super::FLOAT,
}
pub const DSFXDISTORTION_EDGE_MAX: f32 = 100.0;
pub const DSFXDISTORTION_EDGE_MIN: f32 = 0.0;
pub const DSFXDISTORTION_GAIN_MAX: f32 = 0.0;
pub const DSFXDISTORTION_GAIN_MIN: f32 = -60.0;
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MAX: f32 = 8000.0;
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MIN: f32 = 100.0;
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MAX: f32 = 8000.0;
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MIN: f32 = 100.0;
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MAX: f32 = 8000.0;
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MIN: f32 = 100.0;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXDistortion {
    pub fGain: super::FLOAT,
    pub fEdge: super::FLOAT,
    pub fPostEQCenterFrequency: super::FLOAT,
    pub fPostEQBandwidth: super::FLOAT,
    pub fPreLowpassCutoff: super::FLOAT,
}
pub const DSFXECHO_FEEDBACK_MAX: f32 = 100.0;
pub const DSFXECHO_FEEDBACK_MIN: f32 = 0.0;
pub const DSFXECHO_LEFTDELAY_MAX: f32 = 2000.0;
pub const DSFXECHO_LEFTDELAY_MIN: f32 = 1.0;
pub const DSFXECHO_PANDELAY_MAX: i32 = 1;
pub const DSFXECHO_PANDELAY_MIN: i32 = 0;
pub const DSFXECHO_RIGHTDELAY_MAX: f32 = 2000.0;
pub const DSFXECHO_RIGHTDELAY_MIN: f32 = 1.0;
pub const DSFXECHO_WETDRYMIX_MAX: f32 = 100.0;
pub const DSFXECHO_WETDRYMIX_MIN: f32 = 0.0;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXEcho {
    pub fWetDryMix: super::FLOAT,
    pub fFeedback: super::FLOAT,
    pub fLeftDelay: super::FLOAT,
    pub fRightDelay: super::FLOAT,
    pub lPanDelay: super::LONG,
}
pub const DSFXFLANGER_DELAY_MAX: f32 = 4.0;
pub const DSFXFLANGER_DELAY_MIN: f32 = 0.0;
pub const DSFXFLANGER_DEPTH_MAX: f32 = 100.0;
pub const DSFXFLANGER_DEPTH_MIN: f32 = 0.0;
pub const DSFXFLANGER_FEEDBACK_MAX: f32 = 99.0;
pub const DSFXFLANGER_FEEDBACK_MIN: f32 = -99.0;
pub const DSFXFLANGER_FREQUENCY_MAX: f32 = 10.0;
pub const DSFXFLANGER_FREQUENCY_MIN: f32 = 0.0;
pub const DSFXFLANGER_PHASE_180: i32 = 4;
pub const DSFXFLANGER_PHASE_90: i32 = 3;
pub const DSFXFLANGER_PHASE_MAX: i32 = 4;
pub const DSFXFLANGER_PHASE_MIN: i32 = 0;
pub const DSFXFLANGER_PHASE_NEG_180: i32 = 0;
pub const DSFXFLANGER_PHASE_NEG_90: i32 = 1;
pub const DSFXFLANGER_PHASE_ZERO: i32 = 2;
pub const DSFXFLANGER_WAVE_SIN: i32 = 1;
pub const DSFXFLANGER_WAVE_TRIANGLE: i32 = 0;
pub const DSFXFLANGER_WETDRYMIX_MAX: f32 = 100.0;
pub const DSFXFLANGER_WETDRYMIX_MIN: f32 = 0.0;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXFlanger {
    pub fWetDryMix: super::FLOAT,
    pub fDepth: super::FLOAT,
    pub fFeedback: super::FLOAT,
    pub fFrequency: super::FLOAT,
    pub lWaveform: super::LONG,
    pub fDelay: super::FLOAT,
    pub lPhase: super::LONG,
}
pub const DSFXGARGLE_RATEHZ_MAX: i32 = 1000;
pub const DSFXGARGLE_RATEHZ_MIN: i32 = 1;
pub const DSFXGARGLE_WAVE_SQUARE: i32 = 1;
pub const DSFXGARGLE_WAVE_TRIANGLE: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSFXGargle {
    pub dwRateHz: super::DWORD,
    pub dwWaveShape: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXI3DL2Reverb {
    pub lRoom: super::LONG,
    pub lRoomHF: super::LONG,
    pub flRoomRolloffFactor: super::FLOAT,
    pub flDecayTime: super::FLOAT,
    pub flDecayHFRatio: super::FLOAT,
    pub lReflections: super::LONG,
    pub flReflectionsDelay: super::FLOAT,
    pub lReverb: super::LONG,
    pub flReverbDelay: super::FLOAT,
    pub flDiffusion: super::FLOAT,
    pub flDensity: super::FLOAT,
    pub flHFReference: super::FLOAT,
}
pub const DSFXPARAMEQ_BANDWIDTH_MAX: f32 = 36.0;
pub const DSFXPARAMEQ_BANDWIDTH_MIN: f32 = 1.0;
pub const DSFXPARAMEQ_CENTER_MAX: f32 = 16000.0;
pub const DSFXPARAMEQ_CENTER_MIN: f32 = 80.0;
pub const DSFXPARAMEQ_GAIN_MAX: f32 = 15.0;
pub const DSFXPARAMEQ_GAIN_MIN: f32 = -15.0;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXParamEq {
    pub fCenter: super::FLOAT,
    pub fBandwidth: super::FLOAT,
    pub fGain: super::FLOAT,
}
pub const DSFXR_FAILED: i32 = 4;
pub const DSFXR_LOCHARDWARE: i32 = 1;
pub const DSFXR_LOCSOFTWARE: i32 = 2;
pub const DSFXR_PRESENT: i32 = 0;
pub const DSFXR_SENDLOOP: i32 = 6;
pub const DSFXR_UNALLOCATED: i32 = 3;
pub const DSFXR_UNKNOWN: i32 = 5;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXWavesReverb {
    pub fInGain: super::FLOAT,
    pub fReverbMix: super::FLOAT,
    pub fReverbTime: super::FLOAT,
    pub fHighFreqRTRatio: super::FLOAT,
}
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_DEFAULT: f32 = 0.83;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MAX: f32 = 2.0;
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MIN: f32 = 0.1;
pub const DSFX_I3DL2REVERB_DECAYTIME_DEFAULT: f32 = 1.49;
pub const DSFX_I3DL2REVERB_DECAYTIME_MAX: f32 = 20.0;
pub const DSFX_I3DL2REVERB_DECAYTIME_MIN: f32 = 0.1;
pub const DSFX_I3DL2REVERB_DENSITY_DEFAULT: f32 = 100.0;
pub const DSFX_I3DL2REVERB_DENSITY_MAX: f32 = 100.0;
pub const DSFX_I3DL2REVERB_DENSITY_MIN: f32 = 0.0;
pub const DSFX_I3DL2REVERB_DIFFUSION_DEFAULT: f32 = 100.0;
pub const DSFX_I3DL2REVERB_DIFFUSION_MAX: f32 = 100.0;
pub const DSFX_I3DL2REVERB_DIFFUSION_MIN: f32 = 0.0;
pub const DSFX_I3DL2REVERB_HFREFERENCE_DEFAULT: f32 = 5000.0;
pub const DSFX_I3DL2REVERB_HFREFERENCE_MAX: f32 = 20000.0;
pub const DSFX_I3DL2REVERB_HFREFERENCE_MIN: f32 = 20.0;
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: i32 = 2;
pub const DSFX_I3DL2REVERB_QUALITY_MAX: i32 = 3;
pub const DSFX_I3DL2REVERB_QUALITY_MIN: i32 = 0;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: f32 = 0.007;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: f32 = 0.3;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: f32 = 0.0;
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: i32 = -2602;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: i32 = 1000;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: i32 = -10000;
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: f32 = 0.011;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: f32 = 0.1;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: f32 = 0.0;
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: i32 = 200;
pub const DSFX_I3DL2REVERB_REVERB_MAX: i32 = 2000;
pub const DSFX_I3DL2REVERB_REVERB_MIN: i32 = -10000;
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: i32 = -100;
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: i32 = 0;
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: i32 = -10000;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: f32 = 0.0;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: f32 = 10.0;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: f32 = 0.0;
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: i32 = -1000;
pub const DSFX_I3DL2REVERB_ROOM_MAX: i32 = 0;
pub const DSFX_I3DL2REVERB_ROOM_MIN: i32 = -10000;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ALLEY: i32 = 15;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ARENA: i32 = 10;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: i32 = 7;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_BATHROOM: i32 = 4;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: i32 = 12;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CAVE: i32 = 9;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CITY: i32 = 17;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: i32 = 8;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_DEFAULT: i32 = 0;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_FOREST: i32 = 16;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_GENERIC: i32 = 1;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HALLWAY: i32 = 13;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HANGAR: i32 = 11;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEHALL: i32 = 28;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEROOM: i32 = 26;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: i32 = 5;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: i32 = 27;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: i32 = 25;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: i32 = 18;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: i32 = 2;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: i32 = 21;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLAIN: i32 = 20;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLATE: i32 = 29;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_QUARRY: i32 = 19;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ROOM: i32 = 3;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: i32 = 22;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SMALLROOM: i32 = 24;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: i32 = 14;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONEROOM: i32 = 6;
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_UNDERWATER: i32 = 23;
pub const DSFX_I3DL2_MATERIAL_PRESET_BRICKWALL: i32 = 5;
pub const DSFX_I3DL2_MATERIAL_PRESET_CURTAIN: i32 = 7;
pub const DSFX_I3DL2_MATERIAL_PRESET_DOUBLEWINDOW: i32 = 1;
pub const DSFX_I3DL2_MATERIAL_PRESET_SINGLEWINDOW: i32 = 0;
pub const DSFX_I3DL2_MATERIAL_PRESET_STONEWALL: i32 = 6;
pub const DSFX_I3DL2_MATERIAL_PRESET_THICKDOOR: i32 = 3;
pub const DSFX_I3DL2_MATERIAL_PRESET_THINDOOR: i32 = 2;
pub const DSFX_I3DL2_MATERIAL_PRESET_WOODWALL: i32 = 4;
pub const DSFX_LOCHARDWARE: i32 = 1;
pub const DSFX_LOCSOFTWARE: i32 = 2;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_DEFAULT: f32 = 0.001;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MAX: f32 = 0.999;
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MIN: f32 = 0.001;
pub const DSFX_WAVESREVERB_INGAIN_DEFAULT: f32 = 0.0;
pub const DSFX_WAVESREVERB_INGAIN_MAX: f32 = 0.0;
pub const DSFX_WAVESREVERB_INGAIN_MIN: f32 = -96.0;
pub const DSFX_WAVESREVERB_REVERBMIX_DEFAULT: f32 = 0.0;
pub const DSFX_WAVESREVERB_REVERBMIX_MAX: f32 = 0.0;
pub const DSFX_WAVESREVERB_REVERBMIX_MIN: f32 = -96.0;
pub const DSFX_WAVESREVERB_REVERBTIME_DEFAULT: f32 = 1000.0;
pub const DSFX_WAVESREVERB_REVERBTIME_MAX: f32 = 3000.0;
pub const DSFX_WAVESREVERB_REVERBTIME_MIN: f32 = 0.001;
pub const DSSCL_EXCLUSIVE: i32 = 3;
pub const DSSCL_NORMAL: i32 = 1;
pub const DSSCL_PRIORITY: i32 = 2;
pub const DSSCL_WRITEPRIMARY: i32 = 4;
pub const DSSPEAKER_5POINT1: i32 = 6;
pub const DSSPEAKER_5POINT1_BACK: i32 = 6;
pub const DSSPEAKER_5POINT1_SURROUND: i32 = 9;
pub const DSSPEAKER_7POINT1: i32 = 7;
pub const DSSPEAKER_7POINT1_SURROUND: i32 = 8;
pub const DSSPEAKER_7POINT1_WIDE: i32 = 7;
pub const DSSPEAKER_DIRECTOUT: i32 = 0;
pub const DSSPEAKER_GEOMETRY_MAX: i32 = 180;
pub const DSSPEAKER_GEOMETRY_MIN: i32 = 5;
pub const DSSPEAKER_GEOMETRY_NARROW: i32 = 10;
pub const DSSPEAKER_GEOMETRY_WIDE: i32 = 20;
pub const DSSPEAKER_HEADPHONE: i32 = 1;
pub const DSSPEAKER_MONO: i32 = 2;
pub const DSSPEAKER_QUAD: i32 = 3;
pub const DSSPEAKER_STEREO: i32 = 4;
pub const DSSPEAKER_SURROUND: i32 = 5;
pub const DS_CERTIFIED: i32 = 0;
pub const DS_NO_VIRTUALIZATION: windows_core::HRESULT = windows_core::HRESULT(0x878000A_u32 as _);
pub const DS_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub const DS_UNCERTIFIED: i32 = 1;
pub const GUID_All_Objects: windows_core::GUID = windows_core::GUID::from_u128(0xaa114de5_c262_4169_a1c8_23d698cc73b5);
pub const GUID_DSCFX_CLASS_AEC: windows_core::GUID = windows_core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const GUID_DSCFX_CLASS_NS: windows_core::GUID = windows_core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const GUID_DSCFX_MS_AEC: windows_core::GUID = windows_core::GUID::from_u128(0xcdebb919_379a_488a_8765_f53cfd36de40);
pub const GUID_DSCFX_MS_NS: windows_core::GUID = windows_core::GUID::from_u128(0x11c5c73b_66e9_4ba1_a0ba_e814c6eed92d);
pub const GUID_DSCFX_SYSTEM_AEC: windows_core::GUID = windows_core::GUID::from_u128(0x1c22c56d_9879_4f5b_a389_27996ddc2810);
pub const GUID_DSCFX_SYSTEM_NS: windows_core::GUID = windows_core::GUID::from_u128(0x5ab0882e_7274_4516_877d_4eee99ba4fd0);
pub const GUID_DSFX_STANDARD_CHORUS: windows_core::GUID = windows_core::GUID::from_u128(0xefe6629c_81f7_4281_bd91_c9d604a95af6);
pub const GUID_DSFX_STANDARD_COMPRESSOR: windows_core::GUID = windows_core::GUID::from_u128(0xef011f79_4000_406d_87af_bffb3fc39d57);
pub const GUID_DSFX_STANDARD_DISTORTION: windows_core::GUID = windows_core::GUID::from_u128(0xef114c90_cd1d_484e_96e5_09cfaf912a21);
pub const GUID_DSFX_STANDARD_ECHO: windows_core::GUID = windows_core::GUID::from_u128(0xef3e932c_d40b_4f51_8ccf_3f98f1b29d5d);
pub const GUID_DSFX_STANDARD_FLANGER: windows_core::GUID = windows_core::GUID::from_u128(0xefca3d92_dfd8_4672_a603_7420894bad98);
pub const GUID_DSFX_STANDARD_GARGLE: windows_core::GUID = windows_core::GUID::from_u128(0xdafd8210_5711_4b91_9fe3_f75b7ae279bf);
pub const GUID_DSFX_STANDARD_I3DL2REVERB: windows_core::GUID = windows_core::GUID::from_u128(0xef985e71_d5c7_42d4_ba4d_2d073e2e96f4);
pub const GUID_DSFX_STANDARD_PARAMEQ: windows_core::GUID = windows_core::GUID::from_u128(0x120ced89_3bf4_4173_a132_3cb406cf3231);
pub const GUID_DSFX_WAVES_REVERB: windows_core::GUID = windows_core::GUID::from_u128(0x87fc0268_9a55_4360_95aa_004a1d9de26c);
windows_core::imp::define_interface!(IDirectSound, IDirectSound_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSound, windows_core::IUnknown);
impl IDirectSound {
    #[cfg(all(feature = "mmeapi", feature = "shared", feature = "unknwnbase"))]
    pub unsafe fn CreateSoundBuffer(&self, pcdsbufferdesc: LPCDSBUFFERDESC, ppdsbuffer: *mut Option<LPDIRECTSOUNDBUFFER>, punkouter: &Option<super::LPUNKNOWN>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateSoundBuffer)(windows_core::Interface::as_raw(self), pcdsbufferdesc, core::mem::transmute(ppdsbuffer), core::mem::transmute_copy(punkouter)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCaps(&self, pdscaps: LPDSCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pdscaps as _) }
    }
    pub unsafe fn DuplicateSoundBuffer<P0>(&self, pdsbufferoriginal: P0) -> windows_core::Result<LPDIRECTSOUNDBUFFER>
    where
        P0: windows_core::Param<LPDIRECTSOUNDBUFFER>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateSoundBuffer)(windows_core::Interface::as_raw(self), pdsbufferoriginal.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn SetCooperativeLevel(&self, hwnd: super::HWND, dwlevel: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), hwnd as _, dwlevel) }
    }
    pub unsafe fn Compact(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetSpeakerConfig(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpeakerConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSpeakerConfig)(windows_core::Interface::as_raw(self), dwspeakerconfig) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn Initialize(&self, pcguiddevice: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcguiddevice.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mmeapi", feature = "shared", feature = "unknwnbase"))]
    pub CreateSoundBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSBUFFERDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "shared", feature = "unknwnbase")))]
    CreateSoundBuffer: usize,
    #[cfg(feature = "shared")]
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCaps: usize,
    pub DuplicateSoundBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    SetCooperativeLevel: usize,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub GetSpeakerConfig: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetSpeakerConfig: usize,
    #[cfg(feature = "shared")]
    pub SetSpeakerConfig: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSpeakerConfig: usize,
    #[cfg(feature = "guiddef")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    Initialize: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "unknwnbase", feature = "windef"))]
pub trait IDirectSound_Impl: windows_core::IUnknownImpl {
    fn CreateSoundBuffer(&self, pcdsbufferdesc: LPCDSBUFFERDESC, ppdsbuffer: windows_core::OutRef<LPDIRECTSOUNDBUFFER>, punkouter: windows_core::Ref<super::LPUNKNOWN>) -> windows_core::Result<()>;
    fn GetCaps(&self, pdscaps: LPDSCAPS) -> windows_core::Result<()>;
    fn DuplicateSoundBuffer(&self, pdsbufferoriginal: windows_core::Ref<LPDIRECTSOUNDBUFFER>) -> windows_core::Result<LPDIRECTSOUNDBUFFER>;
    fn SetCooperativeLevel(&self, hwnd: super::HWND, dwlevel: super::DWORD) -> windows_core::Result<()>;
    fn Compact(&self) -> windows_core::Result<()>;
    fn GetSpeakerConfig(&self) -> windows_core::Result<u32>;
    fn SetSpeakerConfig(&self, dwspeakerconfig: super::DWORD) -> windows_core::Result<()>;
    fn Initialize(&self, pcguiddevice: super::LPCGUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "unknwnbase", feature = "windef"))]
impl IDirectSound_Vtbl {
    pub const fn new<Identity: IDirectSound_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSoundBuffer<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsbufferdesc: LPCDSBUFFERDESC, ppdsbuffer: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound_Impl::CreateSoundBuffer(this, core::mem::transmute_copy(&pcdsbufferdesc), core::mem::transmute_copy(&ppdsbuffer), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscaps: LPDSCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound_Impl::GetCaps(this, core::mem::transmute_copy(&pdscaps)).into()
            }
        }
        unsafe extern "system" fn DuplicateSoundBuffer<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsbufferoriginal: *mut core::ffi::c_void, ppdsbufferduplicate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound_Impl::DuplicateSoundBuffer(this, core::mem::transmute_copy(&pdsbufferoriginal)) {
                    Ok(ok__) => {
                        ppdsbufferduplicate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, dwlevel: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwlevel)).into()
            }
        }
        unsafe extern "system" fn Compact<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound_Impl::Compact(this).into()
            }
        }
        unsafe extern "system" fn GetSpeakerConfig<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwspeakerconfig: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound_Impl::GetSpeakerConfig(this) {
                    Ok(ok__) => {
                        pdwspeakerconfig.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSpeakerConfig<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspeakerconfig: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound_Impl::SetSpeakerConfig(this, core::mem::transmute_copy(&dwspeakerconfig)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcguiddevice: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound_Impl::Initialize(this, core::mem::transmute_copy(&pcguiddevice)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSoundBuffer: CreateSoundBuffer::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            DuplicateSoundBuffer: DuplicateSoundBuffer::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            Compact: Compact::<Identity, OFFSET>,
            GetSpeakerConfig: GetSpeakerConfig::<Identity, OFFSET>,
            SetSpeakerConfig: SetSpeakerConfig::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSound as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "unknwnbase", feature = "windef"))]
impl windows_core::RuntimeName for IDirectSound {}
windows_core::imp::define_interface!(IDirectSound3DBuffer, IDirectSound3DBuffer_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSound3DBuffer, windows_core::IUnknown);
impl IDirectSound3DBuffer {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllParameters(&self, pds3dbuffer: LPDS3DBUFFER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pds3dbuffer as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetConeAngles(&self, pdwinsideconeangle: super::LPDWORD, pdwoutsideconeangle: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConeAngles)(windows_core::Interface::as_raw(self), pdwinsideconeangle as _, pdwoutsideconeangle as _) }
    }
    pub unsafe fn GetConeOrientation(&self) -> windows_core::Result<D3DVECTOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConeOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetConeOutsideVolume(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConeOutsideVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxDistance(&self) -> windows_core::Result<D3DVALUE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxDistance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinDistance(&self) -> windows_core::Result<D3DVALUE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinDistance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetMode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPosition(&self) -> windows_core::Result<D3DVECTOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVelocity(&self) -> windows_core::Result<D3DVECTOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllParameters(&self, pcds3dbuffer: LPCDS3DBUFFER, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcds3dbuffer, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetConeAngles(&self, dwinsideconeangle: super::DWORD, dwoutsideconeangle: super::DWORD, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConeAngles)(windows_core::Interface::as_raw(self), dwinsideconeangle, dwoutsideconeangle, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetConeOrientation(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConeOrientation)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetConeOutsideVolume(&self, lconeoutsidevolume: super::LONG, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConeOutsideVolume)(windows_core::Interface::as_raw(self), lconeoutsidevolume, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMaxDistance(&self, flmaxdistance: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxDistance)(windows_core::Interface::as_raw(self), flmaxdistance, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMinDistance(&self, flmindistance: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinDistance)(windows_core::Interface::as_raw(self), flmindistance, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetMode(&self, dwmode: super::DWORD, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), dwmode, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPosition(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVelocity(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVelocity)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDS3DBUFFER) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllParameters: usize,
    #[cfg(feature = "minwindef")]
    pub GetConeAngles: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetConeAngles: usize,
    pub GetConeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub GetConeOutsideVolume: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetConeOutsideVolume: usize,
    pub GetMaxDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVALUE) -> windows_core::HRESULT,
    pub GetMinDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVALUE) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub GetMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetMode: usize,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub GetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDS3DBUFFER, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllParameters: usize,
    #[cfg(feature = "shared")]
    pub SetConeAngles: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetConeAngles: usize,
    #[cfg(feature = "shared")]
    pub SetConeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, D3DVALUE, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetConeOrientation: usize,
    #[cfg(feature = "shared")]
    pub SetConeOutsideVolume: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetConeOutsideVolume: usize,
    #[cfg(feature = "shared")]
    pub SetMaxDistance: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMaxDistance: usize,
    #[cfg(feature = "shared")]
    pub SetMinDistance: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMinDistance: usize,
    #[cfg(feature = "shared")]
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetMode: usize,
    #[cfg(feature = "shared")]
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, D3DVALUE, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPosition: usize,
    #[cfg(feature = "shared")]
    pub SetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, D3DVALUE, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVelocity: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDirectSound3DBuffer_Impl: windows_core::IUnknownImpl {
    fn GetAllParameters(&self, pds3dbuffer: LPDS3DBUFFER) -> windows_core::Result<()>;
    fn GetConeAngles(&self, pdwinsideconeangle: super::LPDWORD, pdwoutsideconeangle: super::LPDWORD) -> windows_core::Result<()>;
    fn GetConeOrientation(&self) -> windows_core::Result<D3DVECTOR>;
    fn GetConeOutsideVolume(&self) -> windows_core::Result<i32>;
    fn GetMaxDistance(&self) -> windows_core::Result<D3DVALUE>;
    fn GetMinDistance(&self) -> windows_core::Result<D3DVALUE>;
    fn GetMode(&self) -> windows_core::Result<u32>;
    fn GetPosition(&self) -> windows_core::Result<D3DVECTOR>;
    fn GetVelocity(&self) -> windows_core::Result<D3DVECTOR>;
    fn SetAllParameters(&self, pcds3dbuffer: LPCDS3DBUFFER, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetConeAngles(&self, dwinsideconeangle: super::DWORD, dwoutsideconeangle: super::DWORD, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetConeOrientation(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetConeOutsideVolume(&self, lconeoutsidevolume: super::LONG, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetMaxDistance(&self, flmaxdistance: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetMinDistance(&self, flmindistance: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetMode(&self, dwmode: super::DWORD, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetPosition(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetVelocity(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDirectSound3DBuffer_Vtbl {
    pub const fn new<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pds3dbuffer: LPDS3DBUFFER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::GetAllParameters(this, core::mem::transmute_copy(&pds3dbuffer)).into()
            }
        }
        unsafe extern "system" fn GetConeAngles<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwinsideconeangle: super::LPDWORD, pdwoutsideconeangle: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::GetConeAngles(this, core::mem::transmute_copy(&pdwinsideconeangle), core::mem::transmute_copy(&pdwoutsideconeangle)).into()
            }
        }
        unsafe extern "system" fn GetConeOrientation<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvorientation: *mut D3DVECTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DBuffer_Impl::GetConeOrientation(this) {
                    Ok(ok__) => {
                        pvorientation.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConeOutsideVolume<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plconeoutsidevolume: super::LPLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DBuffer_Impl::GetConeOutsideVolume(this) {
                    Ok(ok__) => {
                        plconeoutsidevolume.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxDistance<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflmaxdistance: *mut D3DVALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DBuffer_Impl::GetMaxDistance(this) {
                    Ok(ok__) => {
                        pflmaxdistance.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMinDistance<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflmindistance: *mut D3DVALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DBuffer_Impl::GetMinDistance(this) {
                    Ok(ok__) => {
                        pflmindistance.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMode<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DBuffer_Impl::GetMode(this) {
                    Ok(ok__) => {
                        pdwmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvposition: *mut D3DVECTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DBuffer_Impl::GetPosition(this) {
                    Ok(ok__) => {
                        pvposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVelocity<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvelocity: *mut D3DVECTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DBuffer_Impl::GetVelocity(this) {
                    Ok(ok__) => {
                        pvvelocity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcds3dbuffer: LPCDS3DBUFFER, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcds3dbuffer), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetConeAngles<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinsideconeangle: super::DWORD, dwoutsideconeangle: super::DWORD, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetConeAngles(this, core::mem::transmute_copy(&dwinsideconeangle), core::mem::transmute_copy(&dwoutsideconeangle), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetConeOrientation<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetConeOrientation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetConeOutsideVolume<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lconeoutsidevolume: super::LONG, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetConeOutsideVolume(this, core::mem::transmute_copy(&lconeoutsidevolume), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetMaxDistance<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flmaxdistance: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetMaxDistance(this, core::mem::transmute_copy(&flmaxdistance), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetMinDistance<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flmindistance: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetMinDistance(this, core::mem::transmute_copy(&flmindistance), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetMode<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: super::DWORD, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetMode(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetPosition<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetVelocity<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetVelocity(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            GetConeAngles: GetConeAngles::<Identity, OFFSET>,
            GetConeOrientation: GetConeOrientation::<Identity, OFFSET>,
            GetConeOutsideVolume: GetConeOutsideVolume::<Identity, OFFSET>,
            GetMaxDistance: GetMaxDistance::<Identity, OFFSET>,
            GetMinDistance: GetMinDistance::<Identity, OFFSET>,
            GetMode: GetMode::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            GetVelocity: GetVelocity::<Identity, OFFSET>,
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            SetConeAngles: SetConeAngles::<Identity, OFFSET>,
            SetConeOrientation: SetConeOrientation::<Identity, OFFSET>,
            SetConeOutsideVolume: SetConeOutsideVolume::<Identity, OFFSET>,
            SetMaxDistance: SetMaxDistance::<Identity, OFFSET>,
            SetMinDistance: SetMinDistance::<Identity, OFFSET>,
            SetMode: SetMode::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            SetVelocity: SetVelocity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSound3DBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSound3DBuffer {}
windows_core::imp::define_interface!(IDirectSound3DListener, IDirectSound3DListener_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSound3DListener, windows_core::IUnknown);
impl IDirectSound3DListener {
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllParameters(&self, plistener: LPDS3DLISTENER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), plistener as _) }
    }
    pub unsafe fn GetDistanceFactor(&self) -> windows_core::Result<D3DVALUE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDistanceFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDopplerFactor(&self) -> windows_core::Result<D3DVALUE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDopplerFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOrientation(&self, pvorientfront: *mut D3DVECTOR, pvorienttop: *mut D3DVECTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOrientation)(windows_core::Interface::as_raw(self), pvorientfront as _, pvorienttop as _) }
    }
    pub unsafe fn GetPosition(&self) -> windows_core::Result<D3DVECTOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRolloffFactor(&self) -> windows_core::Result<D3DVALUE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRolloffFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVelocity(&self) -> windows_core::Result<D3DVECTOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVelocity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllParameters(&self, pclistener: LPCDS3DLISTENER, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pclistener, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDistanceFactor(&self, fldistancefactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDistanceFactor)(windows_core::Interface::as_raw(self), fldistancefactor, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDopplerFactor(&self, fldopplerfactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDopplerFactor)(windows_core::Interface::as_raw(self), fldopplerfactor, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOrientation(&self, xfront: D3DVALUE, yfront: D3DVALUE, zfront: D3DVALUE, xtop: D3DVALUE, ytop: D3DVALUE, ztop: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOrientation)(windows_core::Interface::as_raw(self), xfront, yfront, zfront, xtop, ytop, ztop, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPosition(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRolloffFactor(&self, flrollofffactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRolloffFactor)(windows_core::Interface::as_raw(self), flrollofffactor, dwapply) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVelocity(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVelocity)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
    pub unsafe fn CommitDeferredSettings(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitDeferredSettings)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDS3DLISTENER) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllParameters: usize,
    pub GetDistanceFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVALUE) -> windows_core::HRESULT,
    pub GetDopplerFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVALUE) -> windows_core::HRESULT,
    pub GetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub GetRolloffFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVALUE) -> windows_core::HRESULT,
    pub GetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDS3DLISTENER, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllParameters: usize,
    #[cfg(feature = "shared")]
    pub SetDistanceFactor: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDistanceFactor: usize,
    #[cfg(feature = "shared")]
    pub SetDopplerFactor: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetDopplerFactor: usize,
    #[cfg(feature = "shared")]
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, D3DVALUE, D3DVALUE, D3DVALUE, D3DVALUE, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOrientation: usize,
    #[cfg(feature = "shared")]
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, D3DVALUE, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPosition: usize,
    #[cfg(feature = "shared")]
    pub SetRolloffFactor: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRolloffFactor: usize,
    #[cfg(feature = "shared")]
    pub SetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, D3DVALUE, D3DVALUE, D3DVALUE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVelocity: usize,
    pub CommitDeferredSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IDirectSound3DListener_Impl: windows_core::IUnknownImpl {
    fn GetAllParameters(&self, plistener: LPDS3DLISTENER) -> windows_core::Result<()>;
    fn GetDistanceFactor(&self) -> windows_core::Result<D3DVALUE>;
    fn GetDopplerFactor(&self) -> windows_core::Result<D3DVALUE>;
    fn GetOrientation(&self, pvorientfront: *mut D3DVECTOR, pvorienttop: *mut D3DVECTOR) -> windows_core::Result<()>;
    fn GetPosition(&self) -> windows_core::Result<D3DVECTOR>;
    fn GetRolloffFactor(&self) -> windows_core::Result<D3DVALUE>;
    fn GetVelocity(&self) -> windows_core::Result<D3DVECTOR>;
    fn SetAllParameters(&self, pclistener: LPCDS3DLISTENER, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetDistanceFactor(&self, fldistancefactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetDopplerFactor(&self, fldopplerfactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetOrientation(&self, xfront: D3DVALUE, yfront: D3DVALUE, zfront: D3DVALUE, xtop: D3DVALUE, ytop: D3DVALUE, ztop: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetPosition(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetRolloffFactor(&self, flrollofffactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn SetVelocity(&self, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::Result<()>;
    fn CommitDeferredSettings(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDirectSound3DListener_Vtbl {
    pub const fn new<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plistener: LPDS3DLISTENER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::GetAllParameters(this, core::mem::transmute_copy(&plistener)).into()
            }
        }
        unsafe extern "system" fn GetDistanceFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfldistancefactor: *mut D3DVALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DListener_Impl::GetDistanceFactor(this) {
                    Ok(ok__) => {
                        pfldistancefactor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDopplerFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfldopplerfactor: *mut D3DVALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DListener_Impl::GetDopplerFactor(this) {
                    Ok(ok__) => {
                        pfldopplerfactor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOrientation<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvorientfront: *mut D3DVECTOR, pvorienttop: *mut D3DVECTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::GetOrientation(this, core::mem::transmute_copy(&pvorientfront), core::mem::transmute_copy(&pvorienttop)).into()
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvposition: *mut D3DVECTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DListener_Impl::GetPosition(this) {
                    Ok(ok__) => {
                        pvposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRolloffFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflrollofffactor: *mut D3DVALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DListener_Impl::GetRolloffFactor(this) {
                    Ok(ok__) => {
                        pflrollofffactor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVelocity<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvelocity: *mut D3DVECTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound3DListener_Impl::GetVelocity(this) {
                    Ok(ok__) => {
                        pvvelocity.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclistener: LPCDS3DLISTENER, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetAllParameters(this, core::mem::transmute_copy(&pclistener), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetDistanceFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fldistancefactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetDistanceFactor(this, core::mem::transmute_copy(&fldistancefactor), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetDopplerFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fldopplerfactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetDopplerFactor(this, core::mem::transmute_copy(&fldopplerfactor), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetOrientation<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xfront: D3DVALUE, yfront: D3DVALUE, zfront: D3DVALUE, xtop: D3DVALUE, ytop: D3DVALUE, ztop: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetOrientation(this, core::mem::transmute_copy(&xfront), core::mem::transmute_copy(&yfront), core::mem::transmute_copy(&zfront), core::mem::transmute_copy(&xtop), core::mem::transmute_copy(&ytop), core::mem::transmute_copy(&ztop), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetPosition<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetRolloffFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flrollofffactor: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetRolloffFactor(this, core::mem::transmute_copy(&flrollofffactor), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetVelocity<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: D3DVALUE, y: D3DVALUE, z: D3DVALUE, dwapply: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetVelocity(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn CommitDeferredSettings<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::CommitDeferredSettings(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            GetDistanceFactor: GetDistanceFactor::<Identity, OFFSET>,
            GetDopplerFactor: GetDopplerFactor::<Identity, OFFSET>,
            GetOrientation: GetOrientation::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            GetRolloffFactor: GetRolloffFactor::<Identity, OFFSET>,
            GetVelocity: GetVelocity::<Identity, OFFSET>,
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            SetDistanceFactor: SetDistanceFactor::<Identity, OFFSET>,
            SetDopplerFactor: SetDopplerFactor::<Identity, OFFSET>,
            SetOrientation: SetOrientation::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            SetRolloffFactor: SetRolloffFactor::<Identity, OFFSET>,
            SetVelocity: SetVelocity::<Identity, OFFSET>,
            CommitDeferredSettings: CommitDeferredSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSound3DListener as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirectSound3DListener {}
windows_core::imp::define_interface!(IDirectSound8, IDirectSound8_Vtbl, 0);
impl core::ops::Deref for IDirectSound8 {
    type Target = IDirectSound;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSound8, windows_core::IUnknown, IDirectSound);
impl IDirectSound8 {
    #[cfg(feature = "minwindef")]
    pub unsafe fn VerifyCertification(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerifyCertification)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound8_Vtbl {
    pub base__: IDirectSound_Vtbl,
    #[cfg(feature = "minwindef")]
    pub VerifyCertification: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    VerifyCertification: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "unknwnbase", feature = "windef"))]
pub trait IDirectSound8_Impl: IDirectSound_Impl {
    fn VerifyCertification(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "unknwnbase", feature = "windef"))]
impl IDirectSound8_Vtbl {
    pub const fn new<Identity: IDirectSound8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VerifyCertification<Identity: IDirectSound8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcertified: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSound8_Impl::VerifyCertification(this) {
                    Ok(ok__) => {
                        pdwcertified.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IDirectSound_Vtbl::new::<Identity, OFFSET>(), VerifyCertification: VerifyCertification::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSound8 as windows_core::Interface>::IID || iid == &<IDirectSound as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "unknwnbase", feature = "windef"))]
impl windows_core::RuntimeName for IDirectSound8 {}
windows_core::imp::define_interface!(IDirectSoundBuffer, IDirectSoundBuffer_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundBuffer, windows_core::IUnknown);
impl IDirectSoundBuffer {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCaps(&self, pdsbuffercaps: LPDSBCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pdsbuffercaps as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: Option<super::LPDWORD>, pdwcurrentwritecursor: Option<super::LPDWORD>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPosition)(windows_core::Interface::as_raw(self), pdwcurrentplaycursor.unwrap_or(core::mem::zeroed()) as _, pdwcurrentwritecursor.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
    pub unsafe fn GetFormat(&self, pwfxformat: Option<super::LPWAVEFORMATEX>, dwsizeallocated: super::DWORD, pdwsizewritten: Option<super::LPDWORD>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pwfxformat.unwrap_or(core::mem::zeroed()) as _, dwsizeallocated, pdwsizewritten.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetVolume(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetPan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetFrequency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub unsafe fn Initialize<P0>(&self, pdirectsound: P0, pcdsbufferdesc: LPCDSBUFFERDESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<LPDIRECTSOUND>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), pcdsbufferdesc) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Lock(&self, dwoffset: super::DWORD, dwbytes: super::DWORD, ppvaudioptr1: *mut super::LPVOID, pdwaudiobytes1: super::LPDWORD, ppvaudioptr2: Option<*mut super::LPVOID>, pdwaudiobytes2: Option<super::LPDWORD>, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwoffset, dwbytes, ppvaudioptr1 as _, pdwaudiobytes1 as _, ppvaudioptr2.unwrap_or(core::mem::zeroed()) as _, pdwaudiobytes2.unwrap_or(core::mem::zeroed()) as _, dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Play(&self, dwreserved1: super::DWORD, dwpriority: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self), dwreserved1, dwpriority, dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPosition)(windows_core::Interface::as_raw(self), dwnewposition) }
    }
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub unsafe fn SetFormat(&self, pcfxformat: super::LPCWAVEFORMATEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), pcfxformat) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVolume(&self, lvolume: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), lvolume) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPan(&self, lpan: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPan)(windows_core::Interface::as_raw(self), lpan) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFrequency(&self, dwfrequency: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFrequency)(windows_core::Interface::as_raw(self), dwfrequency) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Unlock(&self, pvaudioptr1: super::LPVOID, dwaudiobytes1: super::DWORD, pvaudioptr2: Option<super::LPVOID>, dwaudiobytes2: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), pvaudioptr1, dwaudiobytes1, pvaudioptr2.unwrap_or(core::mem::zeroed()) as _, dwaudiobytes2) }
    }
    pub unsafe fn Restore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Restore)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSBCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCaps: usize,
    #[cfg(feature = "minwindef")]
    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetCurrentPosition: usize,
    #[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPWAVEFORMATEX, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "mmeapi", feature = "shared")))]
    GetFormat: usize,
    #[cfg(feature = "minwindef")]
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetVolume: usize,
    #[cfg(feature = "minwindef")]
    pub GetPan: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetPan: usize,
    #[cfg(feature = "minwindef")]
    pub GetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetFrequency: usize,
    #[cfg(feature = "minwindef")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetStatus: usize,
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LPCDSBUFFERDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "shared")))]
    Initialize: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut super::LPVOID, super::LPDWORD, *mut super::LPVOID, super::LPDWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Lock: usize,
    #[cfg(feature = "shared")]
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Play: usize,
    #[cfg(feature = "shared")]
    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetCurrentPosition: usize,
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCWAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "shared")))]
    SetFormat: usize,
    #[cfg(feature = "shared")]
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVolume: usize,
    #[cfg(feature = "shared")]
    pub SetPan: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPan: usize,
    #[cfg(feature = "shared")]
    pub SetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFrequency: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPVOID, super::DWORD, super::LPVOID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Unlock: usize,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
pub trait IDirectSoundBuffer_Impl: windows_core::IUnknownImpl {
    fn GetCaps(&self, pdsbuffercaps: LPDSBCAPS) -> windows_core::Result<()>;
    fn GetCurrentPosition(&self, pdwcurrentplaycursor: super::LPDWORD, pdwcurrentwritecursor: super::LPDWORD) -> windows_core::Result<()>;
    fn GetFormat(&self, pwfxformat: super::LPWAVEFORMATEX, dwsizeallocated: super::DWORD, pdwsizewritten: super::LPDWORD) -> windows_core::Result<()>;
    fn GetVolume(&self) -> windows_core::Result<i32>;
    fn GetPan(&self) -> windows_core::Result<i32>;
    fn GetFrequency(&self) -> windows_core::Result<u32>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, pdirectsound: windows_core::Ref<LPDIRECTSOUND>, pcdsbufferdesc: LPCDSBUFFERDESC) -> windows_core::Result<()>;
    fn Lock(&self, dwoffset: super::DWORD, dwbytes: super::DWORD, ppvaudioptr1: *mut super::LPVOID, pdwaudiobytes1: super::LPDWORD, ppvaudioptr2: *mut super::LPVOID, pdwaudiobytes2: super::LPDWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn Play(&self, dwreserved1: super::DWORD, dwpriority: super::DWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn SetCurrentPosition(&self, dwnewposition: super::DWORD) -> windows_core::Result<()>;
    fn SetFormat(&self, pcfxformat: super::LPCWAVEFORMATEX) -> windows_core::Result<()>;
    fn SetVolume(&self, lvolume: super::LONG) -> windows_core::Result<()>;
    fn SetPan(&self, lpan: super::LONG) -> windows_core::Result<()>;
    fn SetFrequency(&self, dwfrequency: super::DWORD) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Unlock(&self, pvaudioptr1: super::LPVOID, dwaudiobytes1: super::DWORD, pvaudioptr2: super::LPVOID, dwaudiobytes2: super::DWORD) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
impl IDirectSoundBuffer_Vtbl {
    pub const fn new<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaps<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsbuffercaps: LPDSBCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::GetCaps(this, core::mem::transmute_copy(&pdsbuffercaps)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcurrentplaycursor: super::LPDWORD, pdwcurrentwritecursor: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::GetCurrentPosition(this, core::mem::transmute_copy(&pdwcurrentplaycursor), core::mem::transmute_copy(&pdwcurrentwritecursor)).into()
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwfxformat: super::LPWAVEFORMATEX, dwsizeallocated: super::DWORD, pdwsizewritten: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::GetFormat(this, core::mem::transmute_copy(&pwfxformat), core::mem::transmute_copy(&dwsizeallocated), core::mem::transmute_copy(&pdwsizewritten)).into()
            }
        }
        unsafe extern "system" fn GetVolume<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: super::LPLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundBuffer_Impl::GetVolume(this) {
                    Ok(ok__) => {
                        plvolume.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPan<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpan: super::LPLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundBuffer_Impl::GetPan(this) {
                    Ok(ok__) => {
                        plpan.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFrequency<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfrequency: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundBuffer_Impl::GetFrequency(this) {
                    Ok(ok__) => {
                        pdwfrequency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundBuffer_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, pcdsbufferdesc: LPCDSBUFFERDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Initialize(this, core::mem::transmute_copy(&pdirectsound), core::mem::transmute_copy(&pcdsbufferdesc)).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: super::DWORD, dwbytes: super::DWORD, ppvaudioptr1: *mut super::LPVOID, pdwaudiobytes1: super::LPDWORD, ppvaudioptr2: *mut super::LPVOID, pdwaudiobytes2: super::LPDWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Lock(this, core::mem::transmute_copy(&dwoffset), core::mem::transmute_copy(&dwbytes), core::mem::transmute_copy(&ppvaudioptr1), core::mem::transmute_copy(&pdwaudiobytes1), core::mem::transmute_copy(&ppvaudioptr2), core::mem::transmute_copy(&pdwaudiobytes2), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Play<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved1: super::DWORD, dwpriority: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Play(this, core::mem::transmute_copy(&dwreserved1), core::mem::transmute_copy(&dwpriority), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnewposition: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetCurrentPosition(this, core::mem::transmute_copy(&dwnewposition)).into()
            }
        }
        unsafe extern "system" fn SetFormat<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcfxformat: super::LPCWAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetFormat(this, core::mem::transmute_copy(&pcfxformat)).into()
            }
        }
        unsafe extern "system" fn SetVolume<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetVolume(this, core::mem::transmute_copy(&lvolume)).into()
            }
        }
        unsafe extern "system" fn SetPan<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpan: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetPan(this, core::mem::transmute_copy(&lpan)).into()
            }
        }
        unsafe extern "system" fn SetFrequency<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfrequency: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetFrequency(this, core::mem::transmute_copy(&dwfrequency)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaudioptr1: super::LPVOID, dwaudiobytes1: super::DWORD, pvaudioptr2: super::LPVOID, dwaudiobytes2: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Unlock(this, core::mem::transmute_copy(&pvaudioptr1), core::mem::transmute_copy(&dwaudiobytes1), core::mem::transmute_copy(&pvaudioptr2), core::mem::transmute_copy(&dwaudiobytes2)).into()
            }
        }
        unsafe extern "system" fn Restore<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Restore(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            GetPan: GetPan::<Identity, OFFSET>,
            GetFrequency: GetFrequency::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Play: Play::<Identity, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
            SetPan: SetPan::<Identity, OFFSET>,
            SetFrequency: SetFrequency::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundBuffer {}
windows_core::imp::define_interface!(IDirectSoundBuffer8, IDirectSoundBuffer8_Vtbl, 0);
impl core::ops::Deref for IDirectSoundBuffer8 {
    type Target = IDirectSoundBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundBuffer8, windows_core::IUnknown, IDirectSoundBuffer);
impl IDirectSoundBuffer8 {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn SetFX(&self, dweffectscount: super::DWORD, pdsfxdesc: Option<LPDSEFFECTDESC>, pdwresultcodes: Option<super::LPDWORD>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFX)(windows_core::Interface::as_raw(self), dweffectscount, pdsfxdesc.unwrap_or(core::mem::zeroed()) as _, pdwresultcodes.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn AcquireResources(&self, dwflags: super::DWORD, dweffectscount: super::DWORD, pdwresultcodes: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireResources)(windows_core::Interface::as_raw(self), dwflags, dweffectscount, pdwresultcodes as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: super::DWORD, rguidinterface: *const windows_core::GUID, ppobject: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInPath)(windows_core::Interface::as_raw(self), rguidobject, dwindex, rguidinterface, ppobject as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer8_Vtbl {
    pub base__: IDirectSoundBuffer_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub SetFX: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, LPDSEFFECTDESC, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    SetFX: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub AcquireResources: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    AcquireResources: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetObjectInPath: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, *const windows_core::GUID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetObjectInPath: usize,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
pub trait IDirectSoundBuffer8_Impl: IDirectSoundBuffer_Impl {
    fn SetFX(&self, dweffectscount: super::DWORD, pdsfxdesc: LPDSEFFECTDESC, pdwresultcodes: super::LPDWORD) -> windows_core::Result<()>;
    fn AcquireResources(&self, dwflags: super::DWORD, dweffectscount: super::DWORD, pdwresultcodes: super::LPDWORD) -> windows_core::Result<()>;
    fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: super::DWORD, rguidinterface: *const windows_core::GUID, ppobject: *mut super::LPVOID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
impl IDirectSoundBuffer8_Vtbl {
    pub const fn new<Identity: IDirectSoundBuffer8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFX<Identity: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweffectscount: super::DWORD, pdsfxdesc: LPDSEFFECTDESC, pdwresultcodes: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer8_Impl::SetFX(this, core::mem::transmute_copy(&dweffectscount), core::mem::transmute_copy(&pdsfxdesc), core::mem::transmute_copy(&pdwresultcodes)).into()
            }
        }
        unsafe extern "system" fn AcquireResources<Identity: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, dweffectscount: super::DWORD, pdwresultcodes: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer8_Impl::AcquireResources(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dweffectscount), core::mem::transmute_copy(&pdwresultcodes)).into()
            }
        }
        unsafe extern "system" fn GetObjectInPath<Identity: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidobject: *const windows_core::GUID, dwindex: super::DWORD, rguidinterface: *const windows_core::GUID, ppobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer8_Impl::GetObjectInPath(this, core::mem::transmute_copy(&rguidobject), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&rguidinterface), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        Self {
            base__: IDirectSoundBuffer_Vtbl::new::<Identity, OFFSET>(),
            SetFX: SetFX::<Identity, OFFSET>,
            AcquireResources: AcquireResources::<Identity, OFFSET>,
            GetObjectInPath: GetObjectInPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundBuffer8 as windows_core::Interface>::IID || iid == &<IDirectSoundBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundBuffer8 {}
windows_core::imp::define_interface!(IDirectSoundCapture, IDirectSoundCapture_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundCapture, windows_core::IUnknown);
impl IDirectSoundCapture {
    #[cfg(all(feature = "mmeapi", feature = "shared", feature = "unknwnbase"))]
    pub unsafe fn CreateCaptureBuffer(&self, pcdscbufferdesc: LPCDSCBUFFERDESC, ppdscbuffer: *mut Option<LPDIRECTSOUNDCAPTUREBUFFER>, punkouter: &Option<super::LPUNKNOWN>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateCaptureBuffer)(windows_core::Interface::as_raw(self), pcdscbufferdesc, core::mem::transmute(ppdscbuffer), core::mem::transmute_copy(punkouter)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DSCCAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn Initialize(&self, pcguiddevice: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcguiddevice.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCapture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mmeapi", feature = "shared", feature = "unknwnbase"))]
    pub CreateCaptureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSCBUFFERDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "shared", feature = "unknwnbase")))]
    CreateCaptureBuffer: usize,
    #[cfg(feature = "shared")]
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSCCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCaps: usize,
    #[cfg(feature = "guiddef")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    Initialize: usize,
}
#[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "unknwnbase"))]
pub trait IDirectSoundCapture_Impl: windows_core::IUnknownImpl {
    fn CreateCaptureBuffer(&self, pcdscbufferdesc: LPCDSCBUFFERDESC, ppdscbuffer: windows_core::OutRef<LPDIRECTSOUNDCAPTUREBUFFER>, punkouter: windows_core::Ref<super::LPUNKNOWN>) -> windows_core::Result<()>;
    fn GetCaps(&self) -> windows_core::Result<DSCCAPS>;
    fn Initialize(&self, pcguiddevice: super::LPCGUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "unknwnbase"))]
impl IDirectSoundCapture_Vtbl {
    pub const fn new<Identity: IDirectSoundCapture_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateCaptureBuffer<Identity: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdscbufferdesc: LPCDSCBUFFERDESC, ppdscbuffer: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCapture_Impl::CreateCaptureBuffer(this, core::mem::transmute_copy(&pcdscbufferdesc), core::mem::transmute_copy(&ppdscbuffer), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsccaps: LPDSCCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundCapture_Impl::GetCaps(this) {
                    Ok(ok__) => {
                        pdsccaps.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcguiddevice: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCapture_Impl::Initialize(this, core::mem::transmute_copy(&pcguiddevice)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateCaptureBuffer: CreateCaptureBuffer::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCapture as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "unknwnbase"))]
impl windows_core::RuntimeName for IDirectSoundCapture {}
windows_core::imp::define_interface!(IDirectSoundCaptureBuffer, IDirectSoundCaptureBuffer_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureBuffer, windows_core::IUnknown);
impl IDirectSoundCaptureBuffer {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DSCBCAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: Option<super::LPDWORD>, pdwreadposition: Option<super::LPDWORD>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPosition)(windows_core::Interface::as_raw(self), pdwcaptureposition.unwrap_or(core::mem::zeroed()) as _, pdwreadposition.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
    pub unsafe fn GetFormat(&self, pwfxformat: Option<super::LPWAVEFORMATEX>, dwsizeallocated: super::DWORD, pdwsizewritten: Option<super::LPDWORD>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pwfxformat.unwrap_or(core::mem::zeroed()) as _, dwsizeallocated, pdwsizewritten.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub unsafe fn Initialize<P0>(&self, pdirectsoundcapture: P0, pcdscbufferdesc: LPCDSCBUFFERDESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<LPDIRECTSOUNDCAPTURE>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pdirectsoundcapture.param().abi(), pcdscbufferdesc) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Lock(&self, dwoffset: super::DWORD, dwbytes: super::DWORD, ppvaudioptr1: *mut super::LPVOID, pdwaudiobytes1: super::LPDWORD, ppvaudioptr2: Option<*mut super::LPVOID>, pdwaudiobytes2: Option<super::LPDWORD>, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwoffset, dwbytes, ppvaudioptr1 as _, pdwaudiobytes1 as _, ppvaudioptr2.unwrap_or(core::mem::zeroed()) as _, pdwaudiobytes2.unwrap_or(core::mem::zeroed()) as _, dwflags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Start(&self, dwflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Unlock(&self, pvaudioptr1: super::LPVOID, dwaudiobytes1: super::DWORD, pvaudioptr2: Option<super::LPVOID>, dwaudiobytes2: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), pvaudioptr1, dwaudiobytes1, pvaudioptr2.unwrap_or(core::mem::zeroed()) as _, dwaudiobytes2) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSCBCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCaps: usize,
    #[cfg(feature = "minwindef")]
    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetCurrentPosition: usize,
    #[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPWAVEFORMATEX, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "mmeapi", feature = "shared")))]
    GetFormat: usize,
    #[cfg(feature = "minwindef")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetStatus: usize,
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, LPCDSCBUFFERDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "shared")))]
    Initialize: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut super::LPVOID, super::LPDWORD, *mut super::LPVOID, super::LPDWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Lock: usize,
    #[cfg(feature = "shared")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPVOID, super::DWORD, super::LPVOID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Unlock: usize,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
pub trait IDirectSoundCaptureBuffer_Impl: windows_core::IUnknownImpl {
    fn GetCaps(&self) -> windows_core::Result<DSCBCAPS>;
    fn GetCurrentPosition(&self, pdwcaptureposition: super::LPDWORD, pdwreadposition: super::LPDWORD) -> windows_core::Result<()>;
    fn GetFormat(&self, pwfxformat: super::LPWAVEFORMATEX, dwsizeallocated: super::DWORD, pdwsizewritten: super::LPDWORD) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, pdirectsoundcapture: windows_core::Ref<LPDIRECTSOUNDCAPTURE>, pcdscbufferdesc: LPCDSCBUFFERDESC) -> windows_core::Result<()>;
    fn Lock(&self, dwoffset: super::DWORD, dwbytes: super::DWORD, ppvaudioptr1: *mut super::LPVOID, pdwaudiobytes1: super::LPDWORD, ppvaudioptr2: *mut super::LPVOID, pdwaudiobytes2: super::LPDWORD, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn Start(&self, dwflags: super::DWORD) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Unlock(&self, pvaudioptr1: super::LPVOID, dwaudiobytes1: super::DWORD, pvaudioptr2: super::LPVOID, dwaudiobytes2: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
impl IDirectSoundCaptureBuffer_Vtbl {
    pub const fn new<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaps<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscbcaps: LPDSCBCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundCaptureBuffer_Impl::GetCaps(this) {
                    Ok(ok__) => {
                        pdscbcaps.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcaptureposition: super::LPDWORD, pdwreadposition: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::GetCurrentPosition(this, core::mem::transmute_copy(&pdwcaptureposition), core::mem::transmute_copy(&pdwreadposition)).into()
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwfxformat: super::LPWAVEFORMATEX, dwsizeallocated: super::DWORD, pdwsizewritten: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::GetFormat(this, core::mem::transmute_copy(&pwfxformat), core::mem::transmute_copy(&dwsizeallocated), core::mem::transmute_copy(&pdwsizewritten)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundCaptureBuffer_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsoundcapture: *mut core::ffi::c_void, pcdscbufferdesc: LPCDSCBUFFERDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::Initialize(this, core::mem::transmute_copy(&pdirectsoundcapture), core::mem::transmute_copy(&pcdscbufferdesc)).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: super::DWORD, dwbytes: super::DWORD, ppvaudioptr1: *mut super::LPVOID, pdwaudiobytes1: super::LPDWORD, ppvaudioptr2: *mut super::LPVOID, pdwaudiobytes2: super::LPDWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::Lock(this, core::mem::transmute_copy(&dwoffset), core::mem::transmute_copy(&dwbytes), core::mem::transmute_copy(&ppvaudioptr1), core::mem::transmute_copy(&pdwaudiobytes1), core::mem::transmute_copy(&ppvaudioptr2), core::mem::transmute_copy(&pdwaudiobytes2), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::Start(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaudioptr1: super::LPVOID, dwaudiobytes1: super::DWORD, pvaudioptr2: super::LPVOID, dwaudiobytes2: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::Unlock(this, core::mem::transmute_copy(&pvaudioptr1), core::mem::transmute_copy(&dwaudiobytes1), core::mem::transmute_copy(&pvaudioptr2), core::mem::transmute_copy(&dwaudiobytes2)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundCaptureBuffer {}
windows_core::imp::define_interface!(IDirectSoundCaptureBuffer8, IDirectSoundCaptureBuffer8_Vtbl, 0);
impl core::ops::Deref for IDirectSoundCaptureBuffer8 {
    type Target = IDirectSoundCaptureBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureBuffer8, windows_core::IUnknown, IDirectSoundCaptureBuffer);
impl IDirectSoundCaptureBuffer8 {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: super::DWORD, rguidinterface: *const windows_core::GUID, ppobject: *mut super::LPVOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInPath)(windows_core::Interface::as_raw(self), rguidobject, dwindex, rguidinterface, ppobject as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetFXStatus(&self, dweffectscount: super::DWORD, pdwfxstatus: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFXStatus)(windows_core::Interface::as_raw(self), dweffectscount, pdwfxstatus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer8_Vtbl {
    pub base__: IDirectSoundCaptureBuffer_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetObjectInPath: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::DWORD, *const windows_core::GUID, *mut super::LPVOID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetObjectInPath: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetFXStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetFXStatus: usize,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
pub trait IDirectSoundCaptureBuffer8_Impl: IDirectSoundCaptureBuffer_Impl {
    fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: super::DWORD, rguidinterface: *const windows_core::GUID, ppobject: *mut super::LPVOID) -> windows_core::Result<()>;
    fn GetFXStatus(&self, dweffectscount: super::DWORD, pdwfxstatus: super::LPDWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
impl IDirectSoundCaptureBuffer8_Vtbl {
    pub const fn new<Identity: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectInPath<Identity: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidobject: *const windows_core::GUID, dwindex: super::DWORD, rguidinterface: *const windows_core::GUID, ppobject: *mut super::LPVOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer8_Impl::GetObjectInPath(this, core::mem::transmute_copy(&rguidobject), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&rguidinterface), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn GetFXStatus<Identity: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweffectscount: super::DWORD, pdwfxstatus: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer8_Impl::GetFXStatus(this, core::mem::transmute_copy(&dweffectscount), core::mem::transmute_copy(&pdwfxstatus)).into()
            }
        }
        Self {
            base__: IDirectSoundCaptureBuffer_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInPath: GetObjectInPath::<Identity, OFFSET>,
            GetFXStatus: GetFXStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer8 as windows_core::Interface>::IID || iid == &<IDirectSoundCaptureBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundCaptureBuffer8 {}
windows_core::imp::define_interface!(IDirectSoundCaptureFXAec, IDirectSoundCaptureFXAec_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureFXAec, windows_core::IUnknown);
impl IDirectSoundCaptureFXAec {
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllParameters(&self, pdscfxaec: LPCDSCFXAec) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pdscfxaec) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSCFXAec> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXAec_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSCFXAec) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllParameters: usize,
    #[cfg(feature = "shared")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSCFXAec) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllParameters: usize,
    #[cfg(feature = "minwindef")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetStatus: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDirectSoundCaptureFXAec_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pdscfxaec: LPCDSCFXAec) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSCFXAec>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDirectSoundCaptureFXAec_Vtbl {
    pub const fn new<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxaec: LPCDSCFXAec) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureFXAec_Impl::SetAllParameters(this, core::mem::transmute_copy(&pdscfxaec)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxaec: LPDSCFXAec) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundCaptureFXAec_Impl::GetAllParameters(this) {
                    Ok(ok__) => {
                        pdscfxaec.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundCaptureFXAec_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        pdwstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureFXAec_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXAec as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundCaptureFXAec {}
windows_core::imp::define_interface!(IDirectSoundCaptureFXNoiseSuppress, IDirectSoundCaptureFXNoiseSuppress_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureFXNoiseSuppress, windows_core::IUnknown);
impl IDirectSoundCaptureFXNoiseSuppress {
    pub unsafe fn SetAllParameters(&self, pcdscfxnoisesuppress: LPCDSCFXNoiseSuppress) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdscfxnoisesuppress) }
    }
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSCFXNoiseSuppress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXNoiseSuppress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSCFXNoiseSuppress) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSCFXNoiseSuppress) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectSoundCaptureFXNoiseSuppress_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdscfxnoisesuppress: LPCDSCFXNoiseSuppress) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSCFXNoiseSuppress>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl IDirectSoundCaptureFXNoiseSuppress_Vtbl {
    pub const fn new<Identity: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdscfxnoisesuppress: LPCDSCFXNoiseSuppress) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureFXNoiseSuppress_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdscfxnoisesuppress)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxnoisesuppress: LPDSCFXNoiseSuppress) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundCaptureFXNoiseSuppress_Impl::GetAllParameters(this) {
                    Ok(ok__) => {
                        pdscfxnoisesuppress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reset<Identity: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureFXNoiseSuppress_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXNoiseSuppress as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectSoundCaptureFXNoiseSuppress {}
windows_core::imp::define_interface!(IDirectSoundFXChorus, IDirectSoundFXChorus_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXChorus, windows_core::IUnknown);
impl IDirectSoundFXChorus {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn SetAllParameters(&self, pcdsfxchorus: LPCDSFXChorus) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxchorus) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetAllParameters(&self, pdsfxchorus: LPDSFXChorus) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxchorus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXChorus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXChorus) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    SetAllParameters: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXChorus) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetAllParameters: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDirectSoundFXChorus_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxchorus: LPCDSFXChorus) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxchorus: LPDSFXChorus) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDirectSoundFXChorus_Vtbl {
    pub const fn new<Identity: IDirectSoundFXChorus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXChorus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxchorus: LPCDSFXChorus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXChorus_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxchorus)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXChorus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxchorus: LPDSFXChorus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXChorus_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxchorus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXChorus as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundFXChorus {}
windows_core::imp::define_interface!(IDirectSoundFXCompressor, IDirectSoundFXCompressor_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXCompressor, windows_core::IUnknown);
impl IDirectSoundFXCompressor {
    #[cfg(feature = "minwindef")]
    pub unsafe fn SetAllParameters(&self, pcdsfxcompressor: LPCDSFXCompressor) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxcompressor) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetAllParameters(&self, pdsfxcompressor: LPDSFXCompressor) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxcompressor as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXCompressor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXCompressor) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    SetAllParameters: usize,
    #[cfg(feature = "minwindef")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXCompressor) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetAllParameters: usize,
}
#[cfg(feature = "minwindef")]
pub trait IDirectSoundFXCompressor_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxcompressor: LPCDSFXCompressor) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxcompressor: LPDSFXCompressor) -> windows_core::Result<()>;
}
#[cfg(feature = "minwindef")]
impl IDirectSoundFXCompressor_Vtbl {
    pub const fn new<Identity: IDirectSoundFXCompressor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXCompressor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxcompressor: LPCDSFXCompressor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXCompressor_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxcompressor)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXCompressor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxcompressor: LPDSFXCompressor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXCompressor_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxcompressor)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXCompressor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IDirectSoundFXCompressor {}
windows_core::imp::define_interface!(IDirectSoundFXDistortion, IDirectSoundFXDistortion_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXDistortion, windows_core::IUnknown);
impl IDirectSoundFXDistortion {
    #[cfg(feature = "minwindef")]
    pub unsafe fn SetAllParameters(&self, pcdsfxdistortion: LPCDSFXDistortion) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxdistortion) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetAllParameters(&self, pdsfxdistortion: LPDSFXDistortion) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxdistortion as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXDistortion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXDistortion) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    SetAllParameters: usize,
    #[cfg(feature = "minwindef")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXDistortion) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetAllParameters: usize,
}
#[cfg(feature = "minwindef")]
pub trait IDirectSoundFXDistortion_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxdistortion: LPCDSFXDistortion) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxdistortion: LPDSFXDistortion) -> windows_core::Result<()>;
}
#[cfg(feature = "minwindef")]
impl IDirectSoundFXDistortion_Vtbl {
    pub const fn new<Identity: IDirectSoundFXDistortion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXDistortion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxdistortion: LPCDSFXDistortion) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXDistortion_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxdistortion)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXDistortion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxdistortion: LPDSFXDistortion) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXDistortion_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxdistortion)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXDistortion as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IDirectSoundFXDistortion {}
windows_core::imp::define_interface!(IDirectSoundFXEcho, IDirectSoundFXEcho_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXEcho, windows_core::IUnknown);
impl IDirectSoundFXEcho {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn SetAllParameters(&self, pcdsfxecho: LPCDSFXEcho) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxecho) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetAllParameters(&self, pdsfxecho: LPDSFXEcho) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxecho as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXEcho_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXEcho) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    SetAllParameters: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXEcho) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetAllParameters: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDirectSoundFXEcho_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxecho: LPCDSFXEcho) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxecho: LPDSFXEcho) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDirectSoundFXEcho_Vtbl {
    pub const fn new<Identity: IDirectSoundFXEcho_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXEcho_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxecho: LPCDSFXEcho) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXEcho_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxecho)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXEcho_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxecho: LPDSFXEcho) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXEcho_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxecho)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXEcho as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundFXEcho {}
windows_core::imp::define_interface!(IDirectSoundFXFlanger, IDirectSoundFXFlanger_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXFlanger, windows_core::IUnknown);
impl IDirectSoundFXFlanger {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn SetAllParameters(&self, pcdsfxflanger: LPCDSFXFlanger) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxflanger) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetAllParameters(&self, pdsfxflanger: LPDSFXFlanger) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxflanger as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXFlanger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXFlanger) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    SetAllParameters: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXFlanger) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetAllParameters: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDirectSoundFXFlanger_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxflanger: LPCDSFXFlanger) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxflanger: LPDSFXFlanger) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDirectSoundFXFlanger_Vtbl {
    pub const fn new<Identity: IDirectSoundFXFlanger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXFlanger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxflanger: LPCDSFXFlanger) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXFlanger_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxflanger)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXFlanger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxflanger: LPDSFXFlanger) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXFlanger_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxflanger)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXFlanger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundFXFlanger {}
windows_core::imp::define_interface!(IDirectSoundFXGargle, IDirectSoundFXGargle_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXGargle, windows_core::IUnknown);
impl IDirectSoundFXGargle {
    #[cfg(feature = "shared")]
    pub unsafe fn SetAllParameters(&self, pcdsfxgargle: LPCDSFXGargle) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxgargle) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSFXGargle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXGargle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXGargle) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetAllParameters: usize,
    #[cfg(feature = "shared")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXGargle) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAllParameters: usize,
}
#[cfg(feature = "shared")]
pub trait IDirectSoundFXGargle_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxgargle: LPCDSFXGargle) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXGargle>;
}
#[cfg(feature = "shared")]
impl IDirectSoundFXGargle_Vtbl {
    pub const fn new<Identity: IDirectSoundFXGargle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXGargle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxgargle: LPCDSFXGargle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXGargle_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxgargle)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXGargle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxgargle: LPDSFXGargle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundFXGargle_Impl::GetAllParameters(this) {
                    Ok(ok__) => {
                        pdsfxgargle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXGargle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDirectSoundFXGargle {}
windows_core::imp::define_interface!(IDirectSoundFXI3DL2Reverb, IDirectSoundFXI3DL2Reverb_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXI3DL2Reverb, windows_core::IUnknown);
impl IDirectSoundFXI3DL2Reverb {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn SetAllParameters(&self, pcdsfxi3dl2reverb: LPCDSFXI3DL2Reverb) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxi3dl2reverb) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn GetAllParameters(&self, pdsfxi3dl2reverb: LPDSFXI3DL2Reverb) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxi3dl2reverb as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPreset(&self, dwpreset: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPreset)(windows_core::Interface::as_raw(self), dwpreset) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetPreset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetQuality(&self, lquality: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQuality)(windows_core::Interface::as_raw(self), lquality) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetQuality(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQuality)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXI3DL2Reverb_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXI3DL2Reverb) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    SetAllParameters: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXI3DL2Reverb) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    GetAllParameters: usize,
    #[cfg(feature = "shared")]
    pub SetPreset: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPreset: usize,
    #[cfg(feature = "minwindef")]
    pub GetPreset: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetPreset: usize,
    #[cfg(feature = "shared")]
    pub SetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetQuality: usize,
    #[cfg(feature = "shared")]
    pub GetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetQuality: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IDirectSoundFXI3DL2Reverb_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxi3dl2reverb: LPCDSFXI3DL2Reverb) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxi3dl2reverb: LPDSFXI3DL2Reverb) -> windows_core::Result<()>;
    fn SetPreset(&self, dwpreset: super::DWORD) -> windows_core::Result<()>;
    fn GetPreset(&self) -> windows_core::Result<u32>;
    fn SetQuality(&self, lquality: super::LONG) -> windows_core::Result<()>;
    fn GetQuality(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IDirectSoundFXI3DL2Reverb_Vtbl {
    pub const fn new<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxi3dl2reverb: LPCDSFXI3DL2Reverb) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXI3DL2Reverb_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxi3dl2reverb)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxi3dl2reverb: LPDSFXI3DL2Reverb) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXI3DL2Reverb_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxi3dl2reverb)).into()
            }
        }
        unsafe extern "system" fn SetPreset<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpreset: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXI3DL2Reverb_Impl::SetPreset(this, core::mem::transmute_copy(&dwpreset)).into()
            }
        }
        unsafe extern "system" fn GetPreset<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpreset: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundFXI3DL2Reverb_Impl::GetPreset(this) {
                    Ok(ok__) => {
                        pdwpreset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQuality<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquality: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXI3DL2Reverb_Impl::SetQuality(this, core::mem::transmute_copy(&lquality)).into()
            }
        }
        unsafe extern "system" fn GetQuality<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquality: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundFXI3DL2Reverb_Impl::GetQuality(this) {
                    Ok(ok__) => {
                        plquality.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            SetPreset: SetPreset::<Identity, OFFSET>,
            GetPreset: GetPreset::<Identity, OFFSET>,
            SetQuality: SetQuality::<Identity, OFFSET>,
            GetQuality: GetQuality::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXI3DL2Reverb as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IDirectSoundFXI3DL2Reverb {}
windows_core::imp::define_interface!(IDirectSoundFXParamEq, IDirectSoundFXParamEq_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXParamEq, windows_core::IUnknown);
impl IDirectSoundFXParamEq {
    #[cfg(feature = "minwindef")]
    pub unsafe fn SetAllParameters(&self, pcdsfxparameq: LPCDSFXParamEq) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxparameq) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSFXParamEq> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXParamEq_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXParamEq) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    SetAllParameters: usize,
    #[cfg(feature = "minwindef")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXParamEq) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetAllParameters: usize,
}
#[cfg(feature = "minwindef")]
pub trait IDirectSoundFXParamEq_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxparameq: LPCDSFXParamEq) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXParamEq>;
}
#[cfg(feature = "minwindef")]
impl IDirectSoundFXParamEq_Vtbl {
    pub const fn new<Identity: IDirectSoundFXParamEq_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXParamEq_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxparameq: LPCDSFXParamEq) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXParamEq_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxparameq)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXParamEq_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxparameq: LPDSFXParamEq) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundFXParamEq_Impl::GetAllParameters(this) {
                    Ok(ok__) => {
                        pdsfxparameq.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXParamEq as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IDirectSoundFXParamEq {}
windows_core::imp::define_interface!(IDirectSoundFXWavesReverb, IDirectSoundFXWavesReverb_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXWavesReverb, windows_core::IUnknown);
impl IDirectSoundFXWavesReverb {
    #[cfg(feature = "minwindef")]
    pub unsafe fn SetAllParameters(&self, pcdsfxwavesreverb: LPCDSFXWavesReverb) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxwavesreverb) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSFXWavesReverb> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXWavesReverb_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPCDSFXWavesReverb) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    SetAllParameters: usize,
    #[cfg(feature = "minwindef")]
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, LPDSFXWavesReverb) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetAllParameters: usize,
}
#[cfg(feature = "minwindef")]
pub trait IDirectSoundFXWavesReverb_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxwavesreverb: LPCDSFXWavesReverb) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXWavesReverb>;
}
#[cfg(feature = "minwindef")]
impl IDirectSoundFXWavesReverb_Vtbl {
    pub const fn new<Identity: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxwavesreverb: LPCDSFXWavesReverb) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXWavesReverb_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxwavesreverb)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxwavesreverb: LPDSFXWavesReverb) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectSoundFXWavesReverb_Impl::GetAllParameters(this) {
                    Ok(ok__) => {
                        pdsfxwavesreverb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXWavesReverb as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IDirectSoundFXWavesReverb {}
windows_core::imp::define_interface!(IDirectSoundFullDuplex, IDirectSoundFullDuplex_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundFullDuplex, windows_core::IUnknown);
impl IDirectSoundFullDuplex {
    #[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "windef"))]
    pub unsafe fn Initialize(&self, pcaptureguid: super::LPCGUID, prenderguid: super::LPCGUID, lpdscbufferdesc: LPCDSCBUFFERDESC, lpdsbufferdesc: LPCDSBUFFERDESC, hwnd: super::HWND, dwlevel: super::DWORD, lplpdirectsoundcapturebuffer8: LPLPDIRECTSOUNDCAPTUREBUFFER8, lplpdirectsoundbuffer8: LPLPDIRECTSOUNDBUFFER8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcaptureguid, prenderguid, lpdscbufferdesc, lpdsbufferdesc, hwnd as _, dwlevel, lplpdirectsoundcapturebuffer8, lplpdirectsoundbuffer8) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFullDuplex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "windef"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPCGUID, super::LPCGUID, LPCDSCBUFFERDESC, LPCDSBUFFERDESC, super::HWND, super::DWORD, LPLPDIRECTSOUNDCAPTUREBUFFER8, LPLPDIRECTSOUNDBUFFER8) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "windef")))]
    Initialize: usize,
}
#[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "windef"))]
pub trait IDirectSoundFullDuplex_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pcaptureguid: super::LPCGUID, prenderguid: super::LPCGUID, lpdscbufferdesc: LPCDSCBUFFERDESC, lpdsbufferdesc: LPCDSBUFFERDESC, hwnd: super::HWND, dwlevel: super::DWORD, lplpdirectsoundcapturebuffer8: windows_core::OutRef<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: windows_core::OutRef<IDirectSoundBuffer8>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "windef"))]
impl IDirectSoundFullDuplex_Vtbl {
    pub const fn new<Identity: IDirectSoundFullDuplex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDirectSoundFullDuplex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaptureguid: super::LPCGUID, prenderguid: super::LPCGUID, lpdscbufferdesc: LPCDSCBUFFERDESC, lpdsbufferdesc: LPCDSBUFFERDESC, hwnd: super::HWND, dwlevel: super::DWORD, lplpdirectsoundcapturebuffer8: LPLPDIRECTSOUNDCAPTUREBUFFER8, lplpdirectsoundbuffer8: LPLPDIRECTSOUNDBUFFER8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFullDuplex_Impl::Initialize(this, core::mem::transmute_copy(&pcaptureguid), core::mem::transmute_copy(&prenderguid), core::mem::transmute_copy(&lpdscbufferdesc), core::mem::transmute_copy(&lpdsbufferdesc), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&lplpdirectsoundcapturebuffer8), core::mem::transmute_copy(&lplpdirectsoundbuffer8)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFullDuplex as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IDirectSoundFullDuplex {}
windows_core::imp::define_interface!(IDirectSoundNotify, IDirectSoundNotify_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectSoundNotify, windows_core::IUnknown);
impl IDirectSoundNotify {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetNotificationPositions(&self, dwpositionnotifies: super::DWORD, pcpositionnotifies: LPCDSBPOSITIONNOTIFY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationPositions)(windows_core::Interface::as_raw(self), dwpositionnotifies, pcpositionnotifies) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetNotificationPositions: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, LPCDSBPOSITIONNOTIFY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetNotificationPositions: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IDirectSoundNotify_Impl: windows_core::IUnknownImpl {
    fn SetNotificationPositions(&self, dwpositionnotifies: super::DWORD, pcpositionnotifies: LPCDSBPOSITIONNOTIFY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IDirectSoundNotify_Vtbl {
    pub const fn new<Identity: IDirectSoundNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNotificationPositions<Identity: IDirectSoundNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpositionnotifies: super::DWORD, pcpositionnotifies: LPCDSBPOSITIONNOTIFY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundNotify_Impl::SetNotificationPositions(this, core::mem::transmute_copy(&dwpositionnotifies), core::mem::transmute_copy(&pcpositionnotifies)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetNotificationPositions: SetNotificationPositions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundNotify as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IDirectSoundNotify {}
pub const IID_IDirectSound: windows_core::GUID = windows_core::GUID::from_u128(0x279afa83_4981_11ce_a521_0020af0be560);
pub const IID_IDirectSound3DBuffer: windows_core::GUID = windows_core::GUID::from_u128(0x279afa86_4981_11ce_a521_0020af0be560);
pub const IID_IDirectSound3DListener: windows_core::GUID = windows_core::GUID::from_u128(0x279afa84_4981_11ce_a521_0020af0be560);
pub const IID_IDirectSound8: windows_core::GUID = windows_core::GUID::from_u128(0xc50a7e93_f395_4834_9ef6_7fa99de50966);
pub const IID_IDirectSoundBuffer: windows_core::GUID = windows_core::GUID::from_u128(0x279afa85_4981_11ce_a521_0020af0be560);
pub const IID_IDirectSoundBuffer8: windows_core::GUID = windows_core::GUID::from_u128(0x6825a449_7524_4d82_920f_50e36ab3ab1e);
pub const IID_IDirectSoundCapture: windows_core::GUID = windows_core::GUID::from_u128(0xb0210781_89cd_11d0_af08_00a0c925cd16);
pub const IID_IDirectSoundCaptureBuffer: windows_core::GUID = windows_core::GUID::from_u128(0xb0210782_89cd_11d0_af08_00a0c925cd16);
pub const IID_IDirectSoundCaptureBuffer8: windows_core::GUID = windows_core::GUID::from_u128(0x00990df4_0dbb_4872_833e_6d303e80aeb6);
pub const IID_IDirectSoundCaptureFXAec: windows_core::GUID = windows_core::GUID::from_u128(0xad74143d_903d_4ab7_8066_28d363036d65);
pub const IID_IDirectSoundCaptureFXNoiseSuppress: windows_core::GUID = windows_core::GUID::from_u128(0xed311e41_fbae_4175_9625_cd0854f693ca);
pub const IID_IDirectSoundFXChorus: windows_core::GUID = windows_core::GUID::from_u128(0x880842e3_145f_43e6_a934_a71806e50547);
pub const IID_IDirectSoundFXCompressor: windows_core::GUID = windows_core::GUID::from_u128(0x4bbd1154_62f6_4e2c_a15c_d3b6c417f7a0);
pub const IID_IDirectSoundFXDistortion: windows_core::GUID = windows_core::GUID::from_u128(0x8ecf4326_455f_4d8b_bda9_8d5d3e9e3e0b);
pub const IID_IDirectSoundFXEcho: windows_core::GUID = windows_core::GUID::from_u128(0x8bd28edf_50db_4e92_a2bd_445488d1ed42);
pub const IID_IDirectSoundFXFlanger: windows_core::GUID = windows_core::GUID::from_u128(0x903e9878_2c92_4072_9b2c_ea68f5396783);
pub const IID_IDirectSoundFXGargle: windows_core::GUID = windows_core::GUID::from_u128(0xd616f352_d622_11ce_aac5_0020af0b99a3);
pub const IID_IDirectSoundFXI3DL2Reverb: windows_core::GUID = windows_core::GUID::from_u128(0x4b166a6a_0d66_43f3_80e3_ee6280dee1a4);
pub const IID_IDirectSoundFXParamEq: windows_core::GUID = windows_core::GUID::from_u128(0xc03ca9fe_fe90_4204_8078_82334cd177da);
pub const IID_IDirectSoundFXWavesReverb: windows_core::GUID = windows_core::GUID::from_u128(0x46858c3a_0dc6_45e3_b760_d4eef16cb325);
pub const IID_IDirectSoundFullDuplex: windows_core::GUID = windows_core::GUID::from_u128(0xedcb4c7a_daab_4216_a42e_6c50596ddc1d);
pub const IID_IDirectSoundNotify: windows_core::GUID = windows_core::GUID::from_u128(0xb0210783_89cd_11d0_af08_00a0c925cd16);
pub const IID_IKsPropertySet: windows_core::GUID = windows_core::GUID::from_u128(0x31efac30_515c_11d0_a9aa_00aa0061be93);
pub const IID_IReferenceClock: windows_core::GUID = windows_core::GUID::from_u128(0x56a86897_0ad4_11ce_b03a_0020af0ba770);
windows_core::imp::define_interface!(IKsPropertySet, IKsPropertySet_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IKsPropertySet, windows_core::IUnknown);
impl IKsPropertySet {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Get(&self, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pinstancedata: Option<super::LPVOID>, ulinstancelength: super::ULONG, ppropertydata: super::LPVOID, uldatalength: super::ULONG, pulbytesreturned: Option<super::PULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), rguidpropset, ulid, pinstancedata.unwrap_or(core::mem::zeroed()) as _, ulinstancelength, ppropertydata as _, uldatalength, pulbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Set(&self, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pinstancedata: Option<super::LPVOID>, ulinstancelength: super::ULONG, ppropertydata: super::LPVOID, uldatalength: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), rguidpropset, ulid, pinstancedata.unwrap_or(core::mem::zeroed()) as _, ulinstancelength, ppropertydata, uldatalength) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn QuerySupport(&self, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pultypesupport: super::PULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QuerySupport)(windows_core::Interface::as_raw(self), rguidpropset, ulid, pultypesupport as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsPropertySet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::ULONG, super::LPVOID, super::ULONG, super::LPVOID, super::ULONG, super::PULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Get: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::ULONG, super::LPVOID, super::ULONG, super::LPVOID, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Set: usize,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub QuerySupport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::ULONG, super::PULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    QuerySupport: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IKsPropertySet_Impl: windows_core::IUnknownImpl {
    fn Get(&self, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pinstancedata: super::LPVOID, ulinstancelength: super::ULONG, ppropertydata: super::LPVOID, uldatalength: super::ULONG, pulbytesreturned: super::PULONG) -> windows_core::Result<()>;
    fn Set(&self, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pinstancedata: super::LPVOID, ulinstancelength: super::ULONG, ppropertydata: super::LPVOID, uldatalength: super::ULONG) -> windows_core::Result<()>;
    fn QuerySupport(&self, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pultypesupport: super::PULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IKsPropertySet_Vtbl {
    pub const fn new<Identity: IKsPropertySet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pinstancedata: super::LPVOID, ulinstancelength: super::ULONG, ppropertydata: super::LPVOID, uldatalength: super::ULONG, pulbytesreturned: super::PULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsPropertySet_Impl::Get(this, core::mem::transmute_copy(&rguidpropset), core::mem::transmute_copy(&ulid), core::mem::transmute_copy(&pinstancedata), core::mem::transmute_copy(&ulinstancelength), core::mem::transmute_copy(&ppropertydata), core::mem::transmute_copy(&uldatalength), core::mem::transmute_copy(&pulbytesreturned)).into()
            }
        }
        unsafe extern "system" fn Set<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pinstancedata: super::LPVOID, ulinstancelength: super::ULONG, ppropertydata: super::LPVOID, uldatalength: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsPropertySet_Impl::Set(this, core::mem::transmute_copy(&rguidpropset), core::mem::transmute_copy(&ulid), core::mem::transmute_copy(&pinstancedata), core::mem::transmute_copy(&ulinstancelength), core::mem::transmute_copy(&ppropertydata), core::mem::transmute_copy(&uldatalength)).into()
            }
        }
        unsafe extern "system" fn QuerySupport<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidpropset: *const windows_core::GUID, ulid: super::ULONG, pultypesupport: super::PULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsPropertySet_Impl::QuerySupport(this, core::mem::transmute_copy(&rguidpropset), core::mem::transmute_copy(&ulid), core::mem::transmute_copy(&pultypesupport)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Get: Get::<Identity, OFFSET>,
            Set: Set::<Identity, OFFSET>,
            QuerySupport: QuerySupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IKsPropertySet {}
windows_core::imp::define_interface!(IReferenceClock, IReferenceClock_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IReferenceClock, windows_core::IUnknown);
impl IReferenceClock {
    #[cfg(feature = "shared")]
    pub unsafe fn GetTime(&self) -> windows_core::Result<super::REFERENCE_TIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub unsafe fn AdviseTime(&self, rtbasetime: super::REFERENCE_TIME, rtstreamtime: super::REFERENCE_TIME, hevent: super::HANDLE, pdwadvisecookie: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdviseTime)(windows_core::Interface::as_raw(self), rtbasetime, rtstreamtime, hevent as _, pdwadvisecookie as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub unsafe fn AdvisePeriodic(&self, rtstarttime: super::REFERENCE_TIME, rtperiodtime: super::REFERENCE_TIME, hsemaphore: super::HANDLE, pdwadvisecookie: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdvisePeriodic)(windows_core::Interface::as_raw(self), rtstarttime, rtperiodtime, hsemaphore as _, pdwadvisecookie as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Unadvise(&self, dwadvisecookie: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwadvisecookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTime: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub AdviseTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::REFERENCE_TIME, super::REFERENCE_TIME, super::HANDLE, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt")))]
    AdviseTime: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
    pub AdvisePeriodic: unsafe extern "system" fn(*mut core::ffi::c_void, super::REFERENCE_TIME, super::REFERENCE_TIME, super::HANDLE, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt")))]
    AdvisePeriodic: usize,
    #[cfg(feature = "shared")]
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Unadvise: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub trait IReferenceClock_Impl: windows_core::IUnknownImpl {
    fn GetTime(&self) -> windows_core::Result<super::REFERENCE_TIME>;
    fn AdviseTime(&self, rtbasetime: super::REFERENCE_TIME, rtstreamtime: super::REFERENCE_TIME, hevent: super::HANDLE, pdwadvisecookie: super::LPDWORD) -> windows_core::Result<()>;
    fn AdvisePeriodic(&self, rtstarttime: super::REFERENCE_TIME, rtperiodtime: super::REFERENCE_TIME, hsemaphore: super::HANDLE, pdwadvisecookie: super::LPDWORD) -> windows_core::Result<()>;
    fn Unadvise(&self, dwadvisecookie: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl IReferenceClock_Vtbl {
    pub const fn new<Identity: IReferenceClock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTime<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut super::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceClock_Impl::GetTime(this) {
                    Ok(ok__) => {
                        ptime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AdviseTime<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtbasetime: super::REFERENCE_TIME, rtstreamtime: super::REFERENCE_TIME, hevent: super::HANDLE, pdwadvisecookie: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceClock_Impl::AdviseTime(this, core::mem::transmute_copy(&rtbasetime), core::mem::transmute_copy(&rtstreamtime), core::mem::transmute_copy(&hevent), core::mem::transmute_copy(&pdwadvisecookie)).into()
            }
        }
        unsafe extern "system" fn AdvisePeriodic<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtstarttime: super::REFERENCE_TIME, rtperiodtime: super::REFERENCE_TIME, hsemaphore: super::HANDLE, pdwadvisecookie: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceClock_Impl::AdvisePeriodic(this, core::mem::transmute_copy(&rtstarttime), core::mem::transmute_copy(&rtperiodtime), core::mem::transmute_copy(&hsemaphore), core::mem::transmute_copy(&pdwadvisecookie)).into()
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwadvisecookie: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceClock_Impl::Unadvise(this, core::mem::transmute_copy(&dwadvisecookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, OFFSET>,
            AdviseTime: AdviseTime::<Identity, OFFSET>,
            AdvisePeriodic: AdvisePeriodic::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClock as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IReferenceClock {}
pub const KSPROPERTY_SUPPORT_GET: i32 = 1;
pub const KSPROPERTY_SUPPORT_SET: i32 = 2;
#[cfg(feature = "shared")]
pub type LPCDS3DBUFFER = *const DS3DBUFFER;
#[cfg(feature = "shared")]
pub type LPCDS3DLISTENER = *const DS3DLISTENER;
#[cfg(feature = "shared")]
pub type LPCDSBCAPS = *const DSBCAPS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCDSBPOSITIONNOTIFY = *const DSBPOSITIONNOTIFY;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type LPCDSBUFFERDESC = *const DSBUFFERDESC;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type LPCDSBUFFERDESC1 = *const DSBUFFERDESC1;
#[cfg(feature = "shared")]
pub type LPCDSCAPS = *const DSCAPS;
#[cfg(feature = "shared")]
pub type LPCDSCBCAPS = *const DSCBCAPS;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type LPCDSCBUFFERDESC = *const DSCBUFFERDESC;
#[cfg(feature = "shared")]
pub type LPCDSCCAPS = *const DSCCAPS;
#[cfg(feature = "shared")]
pub type LPCDSCEFFECTDESC = *const DSCEFFECTDESC;
#[cfg(feature = "shared")]
pub type LPCDSCFXAec = *const DSCFXAec;
pub type LPCDSCFXNoiseSuppress = *const DSCFXNoiseSuppress;
#[cfg(feature = "shared")]
pub type LPCDSEFFECTDESC = *const DSEFFECTDESC;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPCDSFXChorus = *const DSFXChorus;
#[cfg(feature = "minwindef")]
pub type LPCDSFXCompressor = *const DSFXCompressor;
#[cfg(feature = "minwindef")]
pub type LPCDSFXDistortion = *const DSFXDistortion;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPCDSFXEcho = *const DSFXEcho;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPCDSFXFlanger = *const DSFXFlanger;
#[cfg(feature = "shared")]
pub type LPCDSFXGargle = *const DSFXGargle;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPCDSFXI3DL2Reverb = *const DSFXI3DL2Reverb;
#[cfg(feature = "minwindef")]
pub type LPCDSFXParamEq = *const DSFXParamEq;
#[cfg(feature = "minwindef")]
pub type LPCDSFXWavesReverb = *const DSFXWavesReverb;
pub type LPD3DCOLOR = *mut u32;
pub type LPD3DVALUE = *mut f32;
pub type LPD3DVECTOR = *mut D3DVECTOR;
pub type LPDIRECTSOUND = IDirectSound;
pub type LPDIRECTSOUND3DBUFFER = IDirectSound3DBuffer;
pub type LPDIRECTSOUND3DBUFFER8 = IDirectSound3DBuffer;
pub type LPDIRECTSOUND3DLISTENER = IDirectSound3DListener;
pub type LPDIRECTSOUND3DLISTENER8 = IDirectSound3DListener;
pub type LPDIRECTSOUND8 = IDirectSound8;
pub type LPDIRECTSOUNDBUFFER = IDirectSoundBuffer;
pub type LPDIRECTSOUNDBUFFER8 = IDirectSoundBuffer8;
pub type LPDIRECTSOUNDCAPTURE = IDirectSoundCapture;
pub type LPDIRECTSOUNDCAPTURE8 = IDirectSoundCapture;
pub type LPDIRECTSOUNDCAPTUREBUFFER = IDirectSoundCaptureBuffer;
pub type LPDIRECTSOUNDCAPTUREBUFFER8 = IDirectSoundCaptureBuffer8;
pub type LPDIRECTSOUNDCAPTUREFXAEC = IDirectSoundCaptureFXAec;
pub type LPDIRECTSOUNDCAPTUREFXAEC8 = IDirectSoundCaptureFXAec;
pub type LPDIRECTSOUNDCAPTUREFXNOISESUPPRESS = IDirectSoundCaptureFXNoiseSuppress;
pub type LPDIRECTSOUNDCAPTUREFXNOISESUPPRESS8 = IDirectSoundCaptureFXNoiseSuppress;
pub type LPDIRECTSOUNDFULLDUPLEX = IDirectSoundFullDuplex;
pub type LPDIRECTSOUNDFULLDUPLEX8 = IDirectSoundFullDuplex;
pub type LPDIRECTSOUNDFXCHORUS = IDirectSoundFXChorus;
pub type LPDIRECTSOUNDFXCHORUS8 = IDirectSoundFXChorus;
pub type LPDIRECTSOUNDFXCOMPRESSOR = IDirectSoundFXCompressor;
pub type LPDIRECTSOUNDFXCOMPRESSOR8 = IDirectSoundFXCompressor;
pub type LPDIRECTSOUNDFXDISTORTION = IDirectSoundFXDistortion;
pub type LPDIRECTSOUNDFXDISTORTION8 = IDirectSoundFXDistortion;
pub type LPDIRECTSOUNDFXECHO = IDirectSoundFXEcho;
pub type LPDIRECTSOUNDFXECHO8 = IDirectSoundFXEcho;
pub type LPDIRECTSOUNDFXFLANGER = IDirectSoundFXFlanger;
pub type LPDIRECTSOUNDFXFLANGER8 = IDirectSoundFXFlanger;
pub type LPDIRECTSOUNDFXGARGLE = IDirectSoundFXGargle;
pub type LPDIRECTSOUNDFXGARGLE8 = IDirectSoundFXGargle;
pub type LPDIRECTSOUNDFXI3DL2REVERB = IDirectSoundFXI3DL2Reverb;
pub type LPDIRECTSOUNDFXI3DL2REVERB8 = IDirectSoundFXI3DL2Reverb;
pub type LPDIRECTSOUNDFXPARAMEQ = IDirectSoundFXParamEq;
pub type LPDIRECTSOUNDFXPARAMEQ8 = IDirectSoundFXParamEq;
pub type LPDIRECTSOUNDFXWAVESREVERB = IDirectSoundFXWavesReverb;
pub type LPDIRECTSOUNDFXWAVESREVERB8 = IDirectSoundFXWavesReverb;
pub type LPDIRECTSOUNDNOTIFY = IDirectSoundNotify;
pub type LPDIRECTSOUNDNOTIFY8 = IDirectSoundNotify;
#[cfg(feature = "shared")]
pub type LPDS3DBUFFER = *mut DS3DBUFFER;
#[cfg(feature = "shared")]
pub type LPDS3DLISTENER = *mut DS3DLISTENER;
#[cfg(feature = "shared")]
pub type LPDSBCAPS = *mut DSBCAPS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPDSBPOSITIONNOTIFY = *mut DSBPOSITIONNOTIFY;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type LPDSBUFFERDESC = *mut DSBUFFERDESC;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type LPDSBUFFERDESC1 = *mut DSBUFFERDESC1;
#[cfg(feature = "shared")]
pub type LPDSCAPS = *mut DSCAPS;
#[cfg(feature = "shared")]
pub type LPDSCBCAPS = *mut DSCBCAPS;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type LPDSCBUFFERDESC = *mut DSCBUFFERDESC;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type LPDSCBUFFERDESC1 = *mut DSCBUFFERDESC1;
#[cfg(feature = "shared")]
pub type LPDSCCAPS = *mut DSCCAPS;
#[cfg(feature = "shared")]
pub type LPDSCEFFECTDESC = *mut DSCEFFECTDESC;
#[cfg(feature = "shared")]
pub type LPDSCFXAec = *mut DSCFXAec;
pub type LPDSCFXNoiseSuppress = *mut DSCFXNoiseSuppress;
#[cfg(feature = "shared")]
pub type LPDSEFFECTDESC = *mut DSEFFECTDESC;
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub type LPDSENUMCALLBACKA = Option<unsafe extern "system" fn(param0: super::LPGUID, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: super::LPVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
pub type LPDSENUMCALLBACKW = Option<unsafe extern "system" fn(param0: super::LPGUID, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: super::LPVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDSFXChorus = *mut DSFXChorus;
#[cfg(feature = "minwindef")]
pub type LPDSFXCompressor = *mut DSFXCompressor;
#[cfg(feature = "minwindef")]
pub type LPDSFXDistortion = *mut DSFXDistortion;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDSFXEcho = *mut DSFXEcho;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDSFXFlanger = *mut DSFXFlanger;
#[cfg(feature = "shared")]
pub type LPDSFXGargle = *mut DSFXGargle;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPDSFXI3DL2Reverb = *mut DSFXI3DL2Reverb;
#[cfg(feature = "minwindef")]
pub type LPDSFXParamEq = *mut DSFXParamEq;
#[cfg(feature = "minwindef")]
pub type LPDSFXWavesReverb = *mut DSFXWavesReverb;
pub type LPKSPROPERTYSET = IKsPropertySet;
pub type LPLPDIRECTSOUND = *mut Option<IDirectSound>;
pub type LPLPDIRECTSOUND3DBUFFER = *mut Option<IDirectSound3DBuffer>;
pub type LPLPDIRECTSOUND3DLISTENER = *mut Option<IDirectSound3DListener>;
pub type LPLPDIRECTSOUND8 = *mut Option<IDirectSound8>;
pub type LPLPDIRECTSOUNDBUFFER = *mut Option<IDirectSoundBuffer>;
pub type LPLPDIRECTSOUNDBUFFER8 = *mut Option<IDirectSoundBuffer8>;
pub type LPLPDIRECTSOUNDCAPTURE = *mut Option<IDirectSoundCapture>;
pub type LPLPDIRECTSOUNDCAPTURE8 = *mut Option<IDirectSoundCapture>;
pub type LPLPDIRECTSOUNDCAPTUREBUFFER = *mut Option<IDirectSoundCaptureBuffer>;
pub type LPLPDIRECTSOUNDCAPTUREBUFFER8 = *mut Option<IDirectSoundCaptureBuffer8>;
pub type LPLPDIRECTSOUNDNOTIFY = *mut Option<IDirectSoundNotify>;
pub type LPREFERENCE_TIME = *mut i64;
pub const _FACDS: i32 = 2168;
