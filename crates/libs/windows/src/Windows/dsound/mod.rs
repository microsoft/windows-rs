#[inline]
pub unsafe fn DirectSoundCaptureCreate<P2>(pcguiddevice: Option<*const windows_core::GUID>, ppdsc: *mut Option<IDirectSoundCapture>, punkouter: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("dsound.dll" "system" fn DirectSoundCaptureCreate(pcguiddevice : *const windows_core::GUID, ppdsc : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCaptureCreate(pcguiddevice.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppdsc), punkouter.param().abi()) }
}
#[inline]
pub unsafe fn DirectSoundCaptureCreate8<P2>(pcguiddevice: Option<*const windows_core::GUID>, ppdsc8: *mut Option<IDirectSoundCapture>, punkouter: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("dsound.dll" "system" fn DirectSoundCaptureCreate8(pcguiddevice : *const windows_core::GUID, ppdsc8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCaptureCreate8(pcguiddevice.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppdsc8), punkouter.param().abi()) }
}
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCaptureEnumerateA(pdsenumcallback, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCaptureEnumerateW(pdsenumcallback, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DirectSoundCreate<P2>(pcguiddevice: Option<*const windows_core::GUID>, ppds: *mut Option<IDirectSound>, punkouter: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("dsound.dll" "system" fn DirectSoundCreate(pcguiddevice : *const windows_core::GUID, ppds : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCreate(pcguiddevice.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppds), punkouter.param().abi()) }
}
#[inline]
pub unsafe fn DirectSoundCreate8<P2>(pcguiddevice: Option<*const windows_core::GUID>, ppds8: *mut Option<IDirectSound8>, punkouter: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("dsound.dll" "system" fn DirectSoundCreate8(pcguiddevice : *const windows_core::GUID, ppds8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundCreate8(pcguiddevice.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(ppds8), punkouter.param().abi()) }
}
#[inline]
pub unsafe fn DirectSoundEnumerateA(pdsenumcallback: LPDSENUMCALLBACKA, pcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundEnumerateA(pdsenumcallback, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DirectSoundEnumerateW(pdsenumcallback: LPDSENUMCALLBACKW, pcontext: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_core::link!("dsound.dll" "system" fn DirectSoundEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundEnumerateW(pdsenumcallback, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mmeapi", feature = "windef"))]
#[inline]
pub unsafe fn DirectSoundFullDuplexCreate<P9>(pcguidcapturedevice: Option<*const windows_core::GUID>, pcguidrenderdevice: Option<*const windows_core::GUID>, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: super::windef::HWND, dwlevel: u32, ppdsfd: *mut Option<IDirectSoundFullDuplex>, ppdscbuffer8: *mut Option<IDirectSoundCaptureBuffer8>, ppdsbuffer8: *mut Option<IDirectSoundBuffer8>, punkouter: P9) -> windows_core::HRESULT
where
    P9: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("dsound.dll" "system" fn DirectSoundFullDuplexCreate(pcguidcapturedevice : *const windows_core::GUID, pcguidrenderdevice : *const windows_core::GUID, pcdscbufferdesc : *const DSCBUFFERDESC, pcdsbufferdesc : *const DSBUFFERDESC, hwnd : super::windef::HWND, dwlevel : u32, ppdsfd : *mut *mut core::ffi::c_void, ppdscbuffer8 : *mut *mut core::ffi::c_void, ppdsbuffer8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectSoundFullDuplexCreate(pcguidcapturedevice.unwrap_or(core::mem::zeroed()) as _, pcguidrenderdevice.unwrap_or(core::mem::zeroed()) as _, pcdscbufferdesc, pcdsbufferdesc, hwnd, dwlevel, core::mem::transmute(ppdsfd), core::mem::transmute(ppdscbuffer8), core::mem::transmute(ppdsbuffer8), punkouter.param().abi()) }
}
#[inline]
pub unsafe fn GetDeviceID(pguidsrc: Option<*const windows_core::GUID>) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("dsound.dll" "system" fn GetDeviceID(pguidsrc : *const windows_core::GUID, pguiddest : *mut windows_core::GUID) -> windows_core::HRESULT);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D3DCOLOR(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DVECTOR {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub const DIRECTSOUND_VERSION: u32 = 2304;
pub const DS3DALG_HRTF_FULL: windows_core::GUID = windows_core::GUID::from_u128(0xc2413340_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_HRTF_LIGHT: windows_core::GUID = windows_core::GUID::from_u128(0xc2413342_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_NO_VIRTUALIZATION: windows_core::GUID = windows_core::GUID::from_u128(0xc241333f_1c1b_11d2_94f5_00c04fc28aca);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS3DBUFFER {
    pub dwSize: u32,
    pub vPosition: D3DVECTOR,
    pub vVelocity: D3DVECTOR,
    pub dwInsideConeAngle: u32,
    pub dwOutsideConeAngle: u32,
    pub vConeOrientation: D3DVECTOR,
    pub lConeOutsideVolume: i32,
    pub flMinDistance: f32,
    pub flMaxDistance: f32,
    pub dwMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS3DLISTENER {
    pub dwSize: u32,
    pub vPosition: D3DVECTOR,
    pub vVelocity: D3DVECTOR,
    pub vOrientFront: D3DVECTOR,
    pub vOrientTop: D3DVECTOR,
    pub flDistanceFactor: f32,
    pub flRolloffFactor: f32,
    pub flDopplerFactor: f32,
}
pub const DS3DMODE_DISABLE: u32 = 2;
pub const DS3DMODE_HEADRELATIVE: u32 = 1;
pub const DS3DMODE_NORMAL: u32 = 0;
pub const DS3D_DEFAULTCONEANGLE: u32 = 360;
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: u32 = 0;
pub const DS3D_DEFAULTDISTANCEFACTOR: f32 = 1.0;
pub const DS3D_DEFAULTDOPPLERFACTOR: f32 = 1.0;
pub const DS3D_DEFAULTMAXDISTANCE: f32 = 1000000000.0;
pub const DS3D_DEFAULTMINDISTANCE: f32 = 1.0;
pub const DS3D_DEFAULTROLLOFFFACTOR: f32 = 1.0;
pub const DS3D_DEFERRED: u32 = 1;
pub const DS3D_IMMEDIATE: u32 = 0;
pub const DS3D_MAXCONEANGLE: u32 = 360;
pub const DS3D_MAXDOPPLERFACTOR: f32 = 10.0;
pub const DS3D_MAXROLLOFFFACTOR: f32 = 10.0;
pub const DS3D_MINCONEANGLE: u32 = 0;
pub const DS3D_MINDOPPLERFACTOR: f32 = 0.0;
pub const DS3D_MINROLLOFFFACTOR: f32 = 0.0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwUnlockTransferRate: u32,
    pub dwPlayCpuOverhead: u32,
}
pub const DSBCAPS_CTRL3D: u32 = 16;
pub const DSBCAPS_CTRLFREQUENCY: u32 = 32;
pub const DSBCAPS_CTRLFX: u32 = 512;
pub const DSBCAPS_CTRLPAN: u32 = 64;
pub const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 256;
pub const DSBCAPS_CTRLVOLUME: u32 = 128;
pub const DSBCAPS_GETCURRENTPOSITION2: u32 = 65536;
pub const DSBCAPS_GLOBALFOCUS: u32 = 32768;
pub const DSBCAPS_LOCDEFER: u32 = 262144;
pub const DSBCAPS_LOCHARDWARE: u32 = 4;
pub const DSBCAPS_LOCSOFTWARE: u32 = 8;
pub const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 131072;
pub const DSBCAPS_PRIMARYBUFFER: u32 = 1;
pub const DSBCAPS_STATIC: u32 = 2;
pub const DSBCAPS_STICKYFOCUS: u32 = 16384;
pub const DSBCAPS_TRUEPLAYPOSITION: u32 = 524288;
pub const DSBFREQUENCY_MAX: u32 = 200000;
pub const DSBFREQUENCY_MIN: u32 = 100;
pub const DSBFREQUENCY_ORIGINAL: u32 = 0;
pub const DSBLOCK_ENTIREBUFFER: u32 = 2;
pub const DSBLOCK_FROMWRITECURSOR: u32 = 1;
pub const DSBNOTIFICATIONS_MAX: u32 = 100000;
pub const DSBPAN_CENTER: u32 = 0;
pub const DSBPAN_LEFT: i32 = -10000;
pub const DSBPAN_RIGHT: u32 = 10000;
pub const DSBPLAY_LOCHARDWARE: u32 = 2;
pub const DSBPLAY_LOCSOFTWARE: u32 = 4;
pub const DSBPLAY_LOOPING: u32 = 1;
pub const DSBPLAY_TERMINATEBY_DISTANCE: u32 = 16;
pub const DSBPLAY_TERMINATEBY_PRIORITY: u32 = 32;
pub const DSBPLAY_TERMINATEBY_TIME: u32 = 8;
pub const DSBPN_OFFSETSTOP: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: u32,
    pub hEventNotify: super::winnt::HANDLE,
}
pub const DSBSIZE_FX_MIN: u32 = 150;
pub const DSBSIZE_MAX: u32 = 268435455;
pub const DSBSIZE_MIN: u32 = 4;
pub const DSBSTATUS_BUFFERLOST: u32 = 2;
pub const DSBSTATUS_LOCHARDWARE: u32 = 8;
pub const DSBSTATUS_LOCSOFTWARE: u32 = 16;
pub const DSBSTATUS_LOOPING: u32 = 4;
pub const DSBSTATUS_PLAYING: u32 = 1;
pub const DSBSTATUS_TERMINATED: u32 = 32;
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::mmeapi::LPWAVEFORMATEX,
    pub guid3DAlgorithm: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::mmeapi::LPWAVEFORMATEX,
}
pub const DSBVOLUME_MAX: u32 = 0;
pub const DSBVOLUME_MIN: i32 = -10000;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const DSCAPS_CERTIFIED: u32 = 64;
pub const DSCAPS_CONTINUOUSRATE: u32 = 16;
pub const DSCAPS_EMULDRIVER: u32 = 32;
pub const DSCAPS_PRIMARY16BIT: u32 = 8;
pub const DSCAPS_PRIMARY8BIT: u32 = 4;
pub const DSCAPS_PRIMARYMONO: u32 = 1;
pub const DSCAPS_PRIMARYSTEREO: u32 = 2;
pub const DSCAPS_SECONDARY16BIT: u32 = 2048;
pub const DSCAPS_SECONDARY8BIT: u32 = 1024;
pub const DSCAPS_SECONDARYMONO: u32 = 256;
pub const DSCAPS_SECONDARYSTEREO: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSCBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
}
pub const DSCBCAPS_CTRLFX: u32 = 512;
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648;
pub const DSCBLOCK_ENTIREBUFFER: u32 = 1;
pub const DSCBSTART_LOOPING: u32 = 1;
pub const DSCBSTATUS_CAPTURING: u32 = 1;
pub const DSCBSTATUS_LOOPING: u32 = 2;
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSCBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::mmeapi::LPWAVEFORMATEX,
    pub dwFXCount: u32,
    pub lpDSCFXDesc: LPDSCEFFECTDESC,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSCBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::mmeapi::LPWAVEFORMATEX,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSCCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFormats: u32,
    pub dwChannels: u32,
}
pub const DSCCAPS_CERTIFIED: u32 = 64;
pub const DSCCAPS_EMULDRIVER: u32 = 32;
pub const DSCCAPS_MULTIPLECAPTURE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSCEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSCFXClass: windows_core::GUID,
    pub guidDSCFXInstance: windows_core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSCFXAec {
    pub fEnable: windows_core::BOOL,
    pub fNoiseFill: windows_core::BOOL,
    pub dwMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSCFXNoiseSuppress {
    pub fEnable: windows_core::BOOL,
}
pub const DSCFXR_LOCHARDWARE: u32 = 16;
pub const DSCFXR_LOCSOFTWARE: u32 = 32;
pub const DSCFX_AEC_MODE_FULL_DUPLEX: u32 = 2;
pub const DSCFX_AEC_MODE_HALF_DUPLEX: u32 = 1;
pub const DSCFX_AEC_MODE_PASS_THROUGH: u32 = 0;
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: u32 = 8;
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1;
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2;
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: u32 = 0;
pub const DSCFX_LOCHARDWARE: u32 = 1;
pub const DSCFX_LOCSOFTWARE: u32 = 2;
pub const DSDEVID_DefaultCapture: windows_core::GUID = windows_core::GUID::from_u128(0xdef00001_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultPlayback: windows_core::GUID = windows_core::GUID::from_u128(0xdef00000_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoiceCapture: windows_core::GUID = windows_core::GUID::from_u128(0xdef00003_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoicePlayback: windows_core::GUID = windows_core::GUID::from_u128(0xdef00002_9c6d_47ed_aaf1_4dda8f2b5c03);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSFXClass: windows_core::GUID,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
pub const DSERR_ACCESSDENIED: i32 = -2147024891;
pub const DSERR_ALLOCATED: i32 = -2005401590;
pub const DSERR_ALREADYINITIALIZED: i32 = -2005401470;
pub const DSERR_BADFORMAT: i32 = -2005401500;
pub const DSERR_BADSENDBUFFERGUID: i32 = -2005401390;
pub const DSERR_BUFFERLOST: i32 = -2005401450;
pub const DSERR_BUFFERTOOSMALL: i32 = -2005401420;
pub const DSERR_CONTROLUNAVAIL: i32 = -2005401570;
pub const DSERR_DS8_REQUIRED: i32 = -2005401410;
pub const DSERR_FXUNAVAILABLE: i32 = -2005401380;
pub const DSERR_GENERIC: i32 = -2147467259;
pub const DSERR_INVALIDCALL: i32 = -2005401550;
pub const DSERR_INVALIDPARAM: i32 = -2147024809;
pub const DSERR_NOAGGREGATION: i32 = -2147221232;
pub const DSERR_NODRIVER: i32 = -2005401480;
pub const DSERR_NOINTERFACE: i32 = -2147467262;
pub const DSERR_OBJECTNOTFOUND: i32 = -2005397151;
pub const DSERR_OTHERAPPHASPRIO: i32 = -2005401440;
pub const DSERR_OUTOFMEMORY: i32 = -2147024882;
pub const DSERR_PRIOLEVELNEEDED: i32 = -2005401530;
pub const DSERR_SENDLOOP: i32 = -2005401400;
pub const DSERR_UNINITIALIZED: i32 = -2005401430;
pub const DSERR_UNSUPPORTED: i32 = -2147467263;
pub const DSFXCHORUS_DELAY_MAX: f32 = 20.0;
pub const DSFXCHORUS_DELAY_MIN: f32 = 0.0;
pub const DSFXCHORUS_DEPTH_MAX: f32 = 100.0;
pub const DSFXCHORUS_DEPTH_MIN: f32 = 0.0;
pub const DSFXCHORUS_FEEDBACK_MAX: f32 = 99.0;
pub const DSFXCHORUS_FEEDBACK_MIN: f32 = -99.0;
pub const DSFXCHORUS_FREQUENCY_MAX: f32 = 10.0;
pub const DSFXCHORUS_FREQUENCY_MIN: f32 = 0.0;
pub const DSFXCHORUS_PHASE_180: u32 = 4;
pub const DSFXCHORUS_PHASE_90: u32 = 3;
pub const DSFXCHORUS_PHASE_MAX: u32 = 4;
pub const DSFXCHORUS_PHASE_MIN: u32 = 0;
pub const DSFXCHORUS_PHASE_NEG_180: u32 = 0;
pub const DSFXCHORUS_PHASE_NEG_90: u32 = 1;
pub const DSFXCHORUS_PHASE_ZERO: u32 = 2;
pub const DSFXCHORUS_WAVE_SIN: u32 = 1;
pub const DSFXCHORUS_WAVE_TRIANGLE: u32 = 0;
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXChorus {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXCompressor {
    pub fGain: f32,
    pub fAttack: f32,
    pub fRelease: f32,
    pub fThreshold: f32,
    pub fRatio: f32,
    pub fPredelay: f32,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXDistortion {
    pub fGain: f32,
    pub fEdge: f32,
    pub fPostEQCenterFrequency: f32,
    pub fPostEQBandwidth: f32,
    pub fPreLowpassCutoff: f32,
}
pub const DSFXECHO_FEEDBACK_MAX: f32 = 100.0;
pub const DSFXECHO_FEEDBACK_MIN: f32 = 0.0;
pub const DSFXECHO_LEFTDELAY_MAX: f32 = 2000.0;
pub const DSFXECHO_LEFTDELAY_MIN: f32 = 1.0;
pub const DSFXECHO_PANDELAY_MAX: u32 = 1;
pub const DSFXECHO_PANDELAY_MIN: u32 = 0;
pub const DSFXECHO_RIGHTDELAY_MAX: f32 = 2000.0;
pub const DSFXECHO_RIGHTDELAY_MIN: f32 = 1.0;
pub const DSFXECHO_WETDRYMIX_MAX: f32 = 100.0;
pub const DSFXECHO_WETDRYMIX_MIN: f32 = 0.0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXEcho {
    pub fWetDryMix: f32,
    pub fFeedback: f32,
    pub fLeftDelay: f32,
    pub fRightDelay: f32,
    pub lPanDelay: i32,
}
pub const DSFXFLANGER_DELAY_MAX: f32 = 4.0;
pub const DSFXFLANGER_DELAY_MIN: f32 = 0.0;
pub const DSFXFLANGER_DEPTH_MAX: f32 = 100.0;
pub const DSFXFLANGER_DEPTH_MIN: f32 = 0.0;
pub const DSFXFLANGER_FEEDBACK_MAX: f32 = 99.0;
pub const DSFXFLANGER_FEEDBACK_MIN: f32 = -99.0;
pub const DSFXFLANGER_FREQUENCY_MAX: f32 = 10.0;
pub const DSFXFLANGER_FREQUENCY_MIN: f32 = 0.0;
pub const DSFXFLANGER_PHASE_180: u32 = 4;
pub const DSFXFLANGER_PHASE_90: u32 = 3;
pub const DSFXFLANGER_PHASE_MAX: u32 = 4;
pub const DSFXFLANGER_PHASE_MIN: u32 = 0;
pub const DSFXFLANGER_PHASE_NEG_180: u32 = 0;
pub const DSFXFLANGER_PHASE_NEG_90: u32 = 1;
pub const DSFXFLANGER_PHASE_ZERO: u32 = 2;
pub const DSFXFLANGER_WAVE_SIN: u32 = 1;
pub const DSFXFLANGER_WAVE_TRIANGLE: u32 = 0;
pub const DSFXFLANGER_WETDRYMIX_MAX: f32 = 100.0;
pub const DSFXFLANGER_WETDRYMIX_MIN: f32 = 0.0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXFlanger {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
pub const DSFXGARGLE_RATEHZ_MAX: u32 = 1000;
pub const DSFXGARGLE_RATEHZ_MIN: u32 = 1;
pub const DSFXGARGLE_WAVE_SQUARE: u32 = 1;
pub const DSFXGARGLE_WAVE_TRIANGLE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXGargle {
    pub dwRateHz: u32,
    pub dwWaveShape: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const DSFXPARAMEQ_BANDWIDTH_MAX: f32 = 36.0;
pub const DSFXPARAMEQ_BANDWIDTH_MIN: f32 = 1.0;
pub const DSFXPARAMEQ_CENTER_MAX: f32 = 16000.0;
pub const DSFXPARAMEQ_CENTER_MIN: f32 = 80.0;
pub const DSFXPARAMEQ_GAIN_MAX: f32 = 15.0;
pub const DSFXPARAMEQ_GAIN_MIN: f32 = -15.0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXParamEq {
    pub fCenter: f32,
    pub fBandwidth: f32,
    pub fGain: f32,
}
pub const DSFXR_FAILED: i32 = 4;
pub const DSFXR_LOCHARDWARE: i32 = 1;
pub const DSFXR_LOCSOFTWARE: i32 = 2;
pub const DSFXR_PRESENT: i32 = 0;
pub const DSFXR_SENDLOOP: i32 = 6;
pub const DSFXR_UNALLOCATED: i32 = 3;
pub const DSFXR_UNKNOWN: i32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSFXWavesReverb {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
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
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: u32 = 2;
pub const DSFX_I3DL2REVERB_QUALITY_MAX: u32 = 3;
pub const DSFX_I3DL2REVERB_QUALITY_MIN: u32 = 0;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: f32 = 0.007;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: f32 = 0.3;
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: f32 = 0.0;
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: i32 = -2602;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: u32 = 1000;
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: i32 = -10000;
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: f32 = 0.011;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: f32 = 0.1;
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: f32 = 0.0;
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: u32 = 200;
pub const DSFX_I3DL2REVERB_REVERB_MAX: u32 = 2000;
pub const DSFX_I3DL2REVERB_REVERB_MIN: i32 = -10000;
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: i32 = -100;
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: u32 = 0;
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: i32 = -10000;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: f32 = 0.0;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: f32 = 10.0;
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: f32 = 0.0;
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: i32 = -1000;
pub const DSFX_I3DL2REVERB_ROOM_MAX: u32 = 0;
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
pub const DSFX_LOCHARDWARE: u32 = 1;
pub const DSFX_LOCSOFTWARE: u32 = 2;
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
pub const DSSCL_EXCLUSIVE: u32 = 3;
pub const DSSCL_NORMAL: u32 = 1;
pub const DSSCL_PRIORITY: u32 = 2;
pub const DSSCL_WRITEPRIMARY: u32 = 4;
pub const DSSPEAKER_5POINT1: u32 = 6;
pub const DSSPEAKER_5POINT1_BACK: u32 = 6;
pub const DSSPEAKER_5POINT1_SURROUND: u32 = 9;
pub const DSSPEAKER_7POINT1: u32 = 7;
pub const DSSPEAKER_7POINT1_SURROUND: u32 = 8;
pub const DSSPEAKER_7POINT1_WIDE: u32 = 7;
pub const DSSPEAKER_DIRECTOUT: u32 = 0;
pub const DSSPEAKER_GEOMETRY_MAX: u32 = 180;
pub const DSSPEAKER_GEOMETRY_MIN: u32 = 5;
pub const DSSPEAKER_GEOMETRY_NARROW: u32 = 10;
pub const DSSPEAKER_GEOMETRY_WIDE: u32 = 20;
pub const DSSPEAKER_HEADPHONE: u32 = 1;
pub const DSSPEAKER_MONO: u32 = 2;
pub const DSSPEAKER_QUAD: u32 = 3;
pub const DSSPEAKER_STEREO: u32 = 4;
pub const DSSPEAKER_SURROUND: u32 = 5;
pub const DS_CERTIFIED: u32 = 0;
pub const DS_NO_VIRTUALIZATION: u32 = 142082058;
pub const DS_OK: u32 = 0;
pub const DS_UNCERTIFIED: u32 = 1;
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
windows_core::imp::define_interface!(IDirectSound, IDirectSound_Vtbl, 0x279afa83_4981_11ce_a521_0020af0be560);
windows_core::imp::interface_hierarchy!(IDirectSound, windows_core::IUnknown);
impl IDirectSound {
    #[cfg(feature = "mmeapi")]
    pub unsafe fn CreateSoundBuffer<P2>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut Option<IDirectSoundBuffer>, punkouter: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateSoundBuffer)(windows_core::Interface::as_raw(self), pcdsbufferdesc, core::mem::transmute(ppdsbuffer), punkouter.param().abi()) }
    }
    pub unsafe fn GetCaps(&self, pdscaps: *mut DSCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pdscaps as _) }
    }
    pub unsafe fn DuplicateSoundBuffer<P0>(&self, pdsbufferoriginal: P0) -> windows_core::Result<IDirectSoundBuffer>
    where
        P0: windows_core::Param<IDirectSoundBuffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateSoundBuffer)(windows_core::Interface::as_raw(self), pdsbufferoriginal.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetCooperativeLevel(&self, hwnd: super::windef::HWND, dwlevel: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), hwnd, dwlevel) }
    }
    pub unsafe fn Compact(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Compact)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSpeakerConfig(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpeakerConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSpeakerConfig)(windows_core::Interface::as_raw(self), dwspeakerconfig) }
    }
    pub unsafe fn Initialize(&self, pcguiddevice: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcguiddevice.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mmeapi")]
    pub CreateSoundBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSBUFFERDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    CreateSoundBuffer: usize,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCAPS) -> windows_core::HRESULT,
    pub DuplicateSoundBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetCooperativeLevel: usize,
    pub Compact: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpeakerConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSpeakerConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mmeapi", feature = "windef"))]
pub trait IDirectSound_Impl: windows_core::IUnknownImpl {
    fn CreateSoundBuffer(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: windows_core::OutRef<IDirectSoundBuffer>, punkouter: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCaps(&self, pdscaps: *mut DSCAPS) -> windows_core::Result<()>;
    fn DuplicateSoundBuffer(&self, pdsbufferoriginal: windows_core::Ref<IDirectSoundBuffer>) -> windows_core::Result<IDirectSoundBuffer>;
    fn SetCooperativeLevel(&self, hwnd: super::windef::HWND, dwlevel: u32) -> windows_core::Result<()>;
    fn Compact(&self) -> windows_core::Result<()>;
    fn GetSpeakerConfig(&self) -> windows_core::Result<u32>;
    fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> windows_core::Result<()>;
    fn Initialize(&self, pcguiddevice: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mmeapi", feature = "windef"))]
impl IDirectSound_Vtbl {
    pub const fn new<Identity: IDirectSound_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSoundBuffer<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound_Impl::CreateSoundBuffer(this, core::mem::transmute_copy(&pcdsbufferdesc), core::mem::transmute_copy(&ppdsbuffer), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscaps: *mut DSCAPS) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, dwlevel: u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetSpeakerConfig<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwspeakerconfig: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetSpeakerConfig<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspeakerconfig: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound_Impl::SetSpeakerConfig(this, core::mem::transmute_copy(&dwspeakerconfig)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectSound_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcguiddevice: *const windows_core::GUID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mmeapi", feature = "windef"))]
impl windows_core::RuntimeName for IDirectSound {}
windows_core::imp::define_interface!(IDirectSound3DBuffer, IDirectSound3DBuffer_Vtbl, 0x279afa86_4981_11ce_a521_0020af0be560);
windows_core::imp::interface_hierarchy!(IDirectSound3DBuffer, windows_core::IUnknown);
impl IDirectSound3DBuffer {
    pub unsafe fn GetAllParameters(&self, pds3dbuffer: *mut DS3DBUFFER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pds3dbuffer as _) }
    }
    pub unsafe fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConeAngles)(windows_core::Interface::as_raw(self), pdwinsideconeangle as _, pdwoutsideconeangle as _) }
    }
    pub unsafe fn GetConeOrientation(&self) -> windows_core::Result<D3DVECTOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConeOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConeOutsideVolume(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConeOutsideVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxDistance(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxDistance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinDistance(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinDistance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
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
    pub unsafe fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcds3dbuffer, dwapply) }
    }
    pub unsafe fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConeAngles)(windows_core::Interface::as_raw(self), dwinsideconeangle, dwoutsideconeangle, dwapply) }
    }
    pub unsafe fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConeOrientation)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
    pub unsafe fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConeOutsideVolume)(windows_core::Interface::as_raw(self), lconeoutsidevolume, dwapply) }
    }
    pub unsafe fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxDistance)(windows_core::Interface::as_raw(self), flmaxdistance, dwapply) }
    }
    pub unsafe fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinDistance)(windows_core::Interface::as_raw(self), flmindistance, dwapply) }
    }
    pub unsafe fn SetMode(&self, dwmode: u32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), dwmode, dwapply) }
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVelocity)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DS3DBUFFER) -> windows_core::HRESULT,
    pub GetConeAngles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetConeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub GetConeOutsideVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetMaxDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMinDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub GetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DS3DBUFFER, u32) -> windows_core::HRESULT,
    pub SetConeAngles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub SetConeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub SetConeOutsideVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub SetMaxDistance: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetMinDistance: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub SetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
}
pub trait IDirectSound3DBuffer_Impl: windows_core::IUnknownImpl {
    fn GetAllParameters(&self, pds3dbuffer: *mut DS3DBUFFER) -> windows_core::Result<()>;
    fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> windows_core::Result<()>;
    fn GetConeOrientation(&self) -> windows_core::Result<D3DVECTOR>;
    fn GetConeOutsideVolume(&self) -> windows_core::Result<i32>;
    fn GetMaxDistance(&self) -> windows_core::Result<f32>;
    fn GetMinDistance(&self) -> windows_core::Result<f32>;
    fn GetMode(&self) -> windows_core::Result<u32>;
    fn GetPosition(&self) -> windows_core::Result<D3DVECTOR>;
    fn GetVelocity(&self) -> windows_core::Result<D3DVECTOR>;
    fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> windows_core::Result<()>;
    fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> windows_core::Result<()>;
    fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> windows_core::Result<()>;
    fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetMode(&self, dwmode: u32, dwapply: u32) -> windows_core::Result<()>;
    fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
}
impl IDirectSound3DBuffer_Vtbl {
    pub const fn new<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pds3dbuffer: *mut DS3DBUFFER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::GetAllParameters(this, core::mem::transmute_copy(&pds3dbuffer)).into()
            }
        }
        unsafe extern "system" fn GetConeAngles<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetConeOutsideVolume<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plconeoutsidevolume: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetMaxDistance<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflmaxdistance: *mut f32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetMinDistance<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflmindistance: *mut f32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetMode<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcds3dbuffer), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetConeAngles<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetConeAngles(this, core::mem::transmute_copy(&dwinsideconeangle), core::mem::transmute_copy(&dwoutsideconeangle), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetConeOrientation<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetConeOrientation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetConeOutsideVolume<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lconeoutsidevolume: i32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetConeOutsideVolume(this, core::mem::transmute_copy(&lconeoutsidevolume), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetMaxDistance<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flmaxdistance: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetMaxDistance(this, core::mem::transmute_copy(&flmaxdistance), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetMinDistance<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flmindistance: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetMinDistance(this, core::mem::transmute_copy(&flmindistance), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetMode<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetMode(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetPosition<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DBuffer_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetVelocity<Identity: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSound3DBuffer {}
windows_core::imp::define_interface!(IDirectSound3DListener, IDirectSound3DListener_Vtbl, 0x279afa84_4981_11ce_a521_0020af0be560);
windows_core::imp::interface_hierarchy!(IDirectSound3DListener, windows_core::IUnknown);
impl IDirectSound3DListener {
    pub unsafe fn GetAllParameters(&self, plistener: *mut DS3DLISTENER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), plistener as _) }
    }
    pub unsafe fn GetDistanceFactor(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDistanceFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDopplerFactor(&self) -> windows_core::Result<f32> {
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
    pub unsafe fn GetRolloffFactor(&self) -> windows_core::Result<f32> {
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
    pub unsafe fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pclistener, dwapply) }
    }
    pub unsafe fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDistanceFactor)(windows_core::Interface::as_raw(self), fldistancefactor, dwapply) }
    }
    pub unsafe fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDopplerFactor)(windows_core::Interface::as_raw(self), fldopplerfactor, dwapply) }
    }
    pub unsafe fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOrientation)(windows_core::Interface::as_raw(self), xfront, yfront, zfront, xtop, ytop, ztop, dwapply) }
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z, dwapply) }
    }
    pub unsafe fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRolloffFactor)(windows_core::Interface::as_raw(self), flrollofffactor, dwapply) }
    }
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
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
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DS3DLISTENER) -> windows_core::HRESULT,
    pub GetDistanceFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetDopplerFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub GetRolloffFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3DVECTOR) -> windows_core::HRESULT,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DS3DLISTENER, u32) -> windows_core::HRESULT,
    pub SetDistanceFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetDopplerFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub SetRolloffFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub SetVelocity: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, u32) -> windows_core::HRESULT,
    pub CommitDeferredSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectSound3DListener_Impl: windows_core::IUnknownImpl {
    fn GetAllParameters(&self, plistener: *mut DS3DLISTENER) -> windows_core::Result<()>;
    fn GetDistanceFactor(&self) -> windows_core::Result<f32>;
    fn GetDopplerFactor(&self) -> windows_core::Result<f32>;
    fn GetOrientation(&self, pvorientfront: *mut D3DVECTOR, pvorienttop: *mut D3DVECTOR) -> windows_core::Result<()>;
    fn GetPosition(&self) -> windows_core::Result<D3DVECTOR>;
    fn GetRolloffFactor(&self) -> windows_core::Result<f32>;
    fn GetVelocity(&self) -> windows_core::Result<D3DVECTOR>;
    fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> windows_core::Result<()>;
    fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
    fn CommitDeferredSettings(&self) -> windows_core::Result<()>;
}
impl IDirectSound3DListener_Vtbl {
    pub const fn new<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plistener: *mut DS3DLISTENER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::GetAllParameters(this, core::mem::transmute_copy(&plistener)).into()
            }
        }
        unsafe extern "system" fn GetDistanceFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfldistancefactor: *mut f32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDopplerFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfldopplerfactor: *mut f32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetRolloffFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflrollofffactor: *mut f32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclistener: *const DS3DLISTENER, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetAllParameters(this, core::mem::transmute_copy(&pclistener), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetDistanceFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fldistancefactor: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetDistanceFactor(this, core::mem::transmute_copy(&fldistancefactor), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetDopplerFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fldopplerfactor: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetDopplerFactor(this, core::mem::transmute_copy(&fldopplerfactor), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetOrientation<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetOrientation(this, core::mem::transmute_copy(&xfront), core::mem::transmute_copy(&yfront), core::mem::transmute_copy(&zfront), core::mem::transmute_copy(&xtop), core::mem::transmute_copy(&ytop), core::mem::transmute_copy(&ztop), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetPosition<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetRolloffFactor<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flrollofffactor: f32, dwapply: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSound3DListener_Impl::SetRolloffFactor(this, core::mem::transmute_copy(&flrollofffactor), core::mem::transmute_copy(&dwapply)).into()
            }
        }
        unsafe extern "system" fn SetVelocity<Identity: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSound3DListener {}
windows_core::imp::define_interface!(IDirectSound8, IDirectSound8_Vtbl, 0xc50a7e93_f395_4834_9ef6_7fa99de50966);
impl core::ops::Deref for IDirectSound8 {
    type Target = IDirectSound;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSound8, windows_core::IUnknown, IDirectSound);
impl IDirectSound8 {
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
    pub VerifyCertification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mmeapi", feature = "windef"))]
pub trait IDirectSound8_Impl: IDirectSound_Impl {
    fn VerifyCertification(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "mmeapi", feature = "windef"))]
impl IDirectSound8_Vtbl {
    pub const fn new<Identity: IDirectSound8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn VerifyCertification<Identity: IDirectSound8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcertified: *mut u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mmeapi", feature = "windef"))]
impl windows_core::RuntimeName for IDirectSound8 {}
windows_core::imp::define_interface!(IDirectSoundBuffer, IDirectSoundBuffer_Vtbl, 0x279afa85_4981_11ce_a521_0020af0be560);
windows_core::imp::interface_hierarchy!(IDirectSoundBuffer, windows_core::IUnknown);
impl IDirectSoundBuffer {
    pub unsafe fn GetCaps(&self, pdsbuffercaps: *mut DSBCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pdsbuffercaps as _) }
    }
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: Option<*mut u32>, pdwcurrentwritecursor: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPosition)(windows_core::Interface::as_raw(self), pdwcurrentplaycursor.unwrap_or(core::mem::zeroed()) as _, pdwcurrentwritecursor.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mmeapi")]
    pub unsafe fn GetFormat(&self, pwfxformat: Option<*mut super::mmeapi::WAVEFORMATEX>, dwsizeallocated: u32, pdwsizewritten: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pwfxformat.unwrap_or(core::mem::zeroed()) as _, dwsizeallocated, pdwsizewritten.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetVolume(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFrequency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mmeapi")]
    pub unsafe fn Initialize<P0>(&self, pdirectsound: P0, pcdsbufferdesc: *const DSBUFFERDESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectSound>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pdirectsound.param().abi(), pcdsbufferdesc) }
    }
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: Option<*mut u32>, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwoffset, dwbytes, ppvaudioptr1 as _, pdwaudiobytes1 as _, ppvaudioptr2 as _, pdwaudiobytes2.unwrap_or(core::mem::zeroed()) as _, dwflags) }
    }
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self), dwreserved1, dwpriority, dwflags) }
    }
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentPosition)(windows_core::Interface::as_raw(self), dwnewposition) }
    }
    #[cfg(feature = "mmeapi")]
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), pcfxformat) }
    }
    pub unsafe fn SetVolume(&self, lvolume: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), lvolume) }
    }
    pub unsafe fn SetPan(&self, lpan: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPan)(windows_core::Interface::as_raw(self), lpan) }
    }
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFrequency)(windows_core::Interface::as_raw(self), dwfrequency) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Unlock(&self, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: Option<*const core::ffi::c_void>, dwaudiobytes2: u32) -> windows_core::HRESULT {
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
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSBCAPS) -> windows_core::HRESULT,
    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mmeapi")]
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mmeapi::WAVEFORMATEX, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    GetFormat: usize,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetPan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mmeapi")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DSBUFFERDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    Initialize: usize,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "mmeapi")]
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    SetFormat: usize,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetPan: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Restore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mmeapi")]
pub trait IDirectSoundBuffer_Impl: windows_core::IUnknownImpl {
    fn GetCaps(&self, pdsbuffercaps: *mut DSBCAPS) -> windows_core::Result<()>;
    fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> windows_core::Result<()>;
    fn GetFormat(&self, pwfxformat: *mut super::mmeapi::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> windows_core::Result<()>;
    fn GetVolume(&self) -> windows_core::Result<i32>;
    fn GetPan(&self) -> windows_core::Result<i32>;
    fn GetFrequency(&self) -> windows_core::Result<u32>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, pdirectsound: windows_core::Ref<IDirectSound>, pcdsbufferdesc: *const DSBUFFERDESC) -> windows_core::Result<()>;
    fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> windows_core::Result<()>;
    fn SetCurrentPosition(&self, dwnewposition: u32) -> windows_core::Result<()>;
    fn SetFormat(&self, pcfxformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::Result<()>;
    fn SetVolume(&self, lvolume: i32) -> windows_core::Result<()>;
    fn SetPan(&self, lpan: i32) -> windows_core::Result<()>;
    fn SetFrequency(&self, dwfrequency: u32) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Unlock(&self, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const core::ffi::c_void, dwaudiobytes2: u32) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "mmeapi")]
impl IDirectSoundBuffer_Vtbl {
    pub const fn new<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaps<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsbuffercaps: *mut DSBCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::GetCaps(this, core::mem::transmute_copy(&pdsbuffercaps)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::GetCurrentPosition(this, core::mem::transmute_copy(&pdwcurrentplaycursor), core::mem::transmute_copy(&pdwcurrentwritecursor)).into()
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwfxformat: *mut super::mmeapi::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::GetFormat(this, core::mem::transmute_copy(&pwfxformat), core::mem::transmute_copy(&dwsizeallocated), core::mem::transmute_copy(&pdwsizewritten)).into()
            }
        }
        unsafe extern "system" fn GetVolume<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetPan<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpan: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetFrequency<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfrequency: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetStatus<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Initialize<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Initialize(this, core::mem::transmute_copy(&pdirectsound), core::mem::transmute_copy(&pcdsbufferdesc)).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Lock(this, core::mem::transmute_copy(&dwoffset), core::mem::transmute_copy(&dwbytes), core::mem::transmute_copy(&ppvaudioptr1), core::mem::transmute_copy(&pdwaudiobytes1), core::mem::transmute_copy(&ppvaudioptr2), core::mem::transmute_copy(&pdwaudiobytes2), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Play<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::Play(this, core::mem::transmute_copy(&dwreserved1), core::mem::transmute_copy(&dwpriority), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnewposition: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetCurrentPosition(this, core::mem::transmute_copy(&dwnewposition)).into()
            }
        }
        unsafe extern "system" fn SetFormat<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcfxformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetFormat(this, core::mem::transmute_copy(&pcfxformat)).into()
            }
        }
        unsafe extern "system" fn SetVolume<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetVolume(this, core::mem::transmute_copy(&lvolume)).into()
            }
        }
        unsafe extern "system" fn SetPan<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpan: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer_Impl::SetPan(this, core::mem::transmute_copy(&lpan)).into()
            }
        }
        unsafe extern "system" fn SetFrequency<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfrequency: u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Unlock<Identity: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const core::ffi::c_void, dwaudiobytes2: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "mmeapi")]
impl windows_core::RuntimeName for IDirectSoundBuffer {}
windows_core::imp::define_interface!(IDirectSoundBuffer8, IDirectSoundBuffer8_Vtbl, 0x6825a449_7524_4d82_920f_50e36ab3ab1e);
impl core::ops::Deref for IDirectSoundBuffer8 {
    type Target = IDirectSoundBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundBuffer8, windows_core::IUnknown, IDirectSoundBuffer);
impl IDirectSoundBuffer8 {
    pub unsafe fn SetFX(&self, dweffectscount: u32, pdsfxdesc: Option<*const DSEFFECTDESC>, pdwresultcodes: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFX)(windows_core::Interface::as_raw(self), dweffectscount, pdsfxdesc.unwrap_or(core::mem::zeroed()) as _, pdwresultcodes.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn AcquireResources(&self, dwflags: u32, pdwresultcodes: &mut [u32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireResources)(windows_core::Interface::as_raw(self), dwflags, pdwresultcodes.len().try_into().unwrap(), core::mem::transmute(pdwresultcodes.as_ptr())) }
    }
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInPath)(windows_core::Interface::as_raw(self), rguidobject, dwindex, rguidinterface, ppobject as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer8_Vtbl {
    pub base__: IDirectSoundBuffer_Vtbl,
    pub SetFX: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DSEFFECTDESC, *mut u32) -> windows_core::HRESULT,
    pub AcquireResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub GetObjectInPath: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mmeapi")]
pub trait IDirectSoundBuffer8_Impl: IDirectSoundBuffer_Impl {
    fn SetFX(&self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> windows_core::Result<()>;
    fn AcquireResources(&self, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> windows_core::Result<()>;
    fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "mmeapi")]
impl IDirectSoundBuffer8_Vtbl {
    pub const fn new<Identity: IDirectSoundBuffer8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFX<Identity: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer8_Impl::SetFX(this, core::mem::transmute_copy(&dweffectscount), core::mem::transmute_copy(&pdsfxdesc), core::mem::transmute_copy(&pdwresultcodes)).into()
            }
        }
        unsafe extern "system" fn AcquireResources<Identity: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundBuffer8_Impl::AcquireResources(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dweffectscount), core::mem::transmute_copy(&pdwresultcodes)).into()
            }
        }
        unsafe extern "system" fn GetObjectInPath<Identity: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "mmeapi")]
impl windows_core::RuntimeName for IDirectSoundBuffer8 {}
windows_core::imp::define_interface!(IDirectSoundCapture, IDirectSoundCapture_Vtbl, 0xb0210781_89cd_11d0_af08_00a0c925cd16);
windows_core::imp::interface_hierarchy!(IDirectSoundCapture, windows_core::IUnknown);
impl IDirectSoundCapture {
    #[cfg(feature = "mmeapi")]
    pub unsafe fn CreateCaptureBuffer<P2>(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut Option<IDirectSoundCaptureBuffer>, punkouter: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateCaptureBuffer)(windows_core::Interface::as_raw(self), pcdscbufferdesc, core::mem::transmute(ppdscbuffer), punkouter.param().abi()) }
    }
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DSCCAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Initialize(&self, pcguiddevice: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcguiddevice.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCapture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mmeapi")]
    pub CreateCaptureBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSCBUFFERDESC, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    CreateCaptureBuffer: usize,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCCAPS) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "mmeapi")]
pub trait IDirectSoundCapture_Impl: windows_core::IUnknownImpl {
    fn CreateCaptureBuffer(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: windows_core::OutRef<IDirectSoundCaptureBuffer>, punkouter: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCaps(&self) -> windows_core::Result<DSCCAPS>;
    fn Initialize(&self, pcguiddevice: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "mmeapi")]
impl IDirectSoundCapture_Vtbl {
    pub const fn new<Identity: IDirectSoundCapture_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateCaptureBuffer<Identity: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCapture_Impl::CreateCaptureBuffer(this, core::mem::transmute_copy(&pcdscbufferdesc), core::mem::transmute_copy(&ppdscbuffer), core::mem::transmute_copy(&punkouter)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsccaps: *mut DSCCAPS) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Initialize<Identity: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcguiddevice: *const windows_core::GUID) -> windows_core::HRESULT {
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
#[cfg(feature = "mmeapi")]
impl windows_core::RuntimeName for IDirectSoundCapture {}
windows_core::imp::define_interface!(IDirectSoundCaptureBuffer, IDirectSoundCaptureBuffer_Vtbl, 0xb0210782_89cd_11d0_af08_00a0c925cd16);
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureBuffer, windows_core::IUnknown);
impl IDirectSoundCaptureBuffer {
    pub unsafe fn GetCaps(&self) -> windows_core::Result<DSCBCAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: Option<*mut u32>, pdwreadposition: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPosition)(windows_core::Interface::as_raw(self), pdwcaptureposition.unwrap_or(core::mem::zeroed()) as _, pdwreadposition.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mmeapi")]
    pub unsafe fn GetFormat(&self, pwfxformat: Option<*mut super::mmeapi::WAVEFORMATEX>, dwsizeallocated: u32, pdwsizewritten: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), pwfxformat.unwrap_or(core::mem::zeroed()) as _, dwsizeallocated, pdwsizewritten.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mmeapi")]
    pub unsafe fn Initialize<P0>(&self, pdirectsoundcapture: P0, pcdscbufferdesc: *const DSCBUFFERDESC) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectSoundCapture>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pdirectsoundcapture.param().abi(), pcdscbufferdesc) }
    }
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: Option<*mut u32>, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwoffset, dwbytes, ppvaudioptr1 as _, pdwaudiobytes1 as _, ppvaudioptr2 as _, pdwaudiobytes2.unwrap_or(core::mem::zeroed()) as _, dwflags) }
    }
    pub unsafe fn Start(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Unlock(&self, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: Option<*const core::ffi::c_void>, dwaudiobytes2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), pvaudioptr1, dwaudiobytes1, pvaudioptr2.unwrap_or(core::mem::zeroed()) as _, dwaudiobytes2) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCBCAPS) -> windows_core::HRESULT,
    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mmeapi")]
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mmeapi::WAVEFORMATEX, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    GetFormat: usize,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mmeapi")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DSCBUFFERDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    Initialize: usize,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "mmeapi")]
pub trait IDirectSoundCaptureBuffer_Impl: windows_core::IUnknownImpl {
    fn GetCaps(&self) -> windows_core::Result<DSCBCAPS>;
    fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> windows_core::Result<()>;
    fn GetFormat(&self, pwfxformat: *mut super::mmeapi::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, pdirectsoundcapture: windows_core::Ref<IDirectSoundCapture>, pcdscbufferdesc: *const DSCBUFFERDESC) -> windows_core::Result<()>;
    fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn Start(&self, dwflags: u32) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Unlock(&self, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const core::ffi::c_void, dwaudiobytes2: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "mmeapi")]
impl IDirectSoundCaptureBuffer_Vtbl {
    pub const fn new<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaps<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscbcaps: *mut DSCBCAPS) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCurrentPosition<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::GetCurrentPosition(this, core::mem::transmute_copy(&pdwcaptureposition), core::mem::transmute_copy(&pdwreadposition)).into()
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwfxformat: *mut super::mmeapi::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::GetFormat(this, core::mem::transmute_copy(&pwfxformat), core::mem::transmute_copy(&dwsizeallocated), core::mem::transmute_copy(&pdwsizewritten)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Initialize<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsoundcapture: *mut core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::Initialize(this, core::mem::transmute_copy(&pdirectsoundcapture), core::mem::transmute_copy(&pcdscbufferdesc)).into()
            }
        }
        unsafe extern "system" fn Lock<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer_Impl::Lock(this, core::mem::transmute_copy(&dwoffset), core::mem::transmute_copy(&dwbytes), core::mem::transmute_copy(&ppvaudioptr1), core::mem::transmute_copy(&pdwaudiobytes1), core::mem::transmute_copy(&ppvaudioptr2), core::mem::transmute_copy(&pdwaudiobytes2), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Unlock<Identity: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const core::ffi::c_void, dwaudiobytes2: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "mmeapi")]
impl windows_core::RuntimeName for IDirectSoundCaptureBuffer {}
windows_core::imp::define_interface!(IDirectSoundCaptureBuffer8, IDirectSoundCaptureBuffer8_Vtbl, 0x00990df4_0dbb_4872_833e_6d303e80aeb6);
impl core::ops::Deref for IDirectSoundCaptureBuffer8 {
    type Target = IDirectSoundCaptureBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureBuffer8, windows_core::IUnknown, IDirectSoundCaptureBuffer);
impl IDirectSoundCaptureBuffer8 {
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInPath)(windows_core::Interface::as_raw(self), rguidobject, dwindex, rguidinterface, ppobject as _) }
    }
    pub unsafe fn GetFXStatus(&self, pdwfxstatus: &mut [u32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFXStatus)(windows_core::Interface::as_raw(self), pdwfxstatus.len().try_into().unwrap(), core::mem::transmute(pdwfxstatus.as_ptr())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer8_Vtbl {
    pub base__: IDirectSoundCaptureBuffer_Vtbl,
    pub GetObjectInPath: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFXStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "mmeapi")]
pub trait IDirectSoundCaptureBuffer8_Impl: IDirectSoundCaptureBuffer_Impl {
    fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetFXStatus(&self, dweffectscount: u32, pdwfxstatus: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "mmeapi")]
impl IDirectSoundCaptureBuffer8_Vtbl {
    pub const fn new<Identity: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectInPath<Identity: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureBuffer8_Impl::GetObjectInPath(this, core::mem::transmute_copy(&rguidobject), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&rguidinterface), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn GetFXStatus<Identity: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweffectscount: u32, pdwfxstatus: *mut u32) -> windows_core::HRESULT {
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
#[cfg(feature = "mmeapi")]
impl windows_core::RuntimeName for IDirectSoundCaptureBuffer8 {}
windows_core::imp::define_interface!(IDirectSoundCaptureFXAec, IDirectSoundCaptureFXAec_Vtbl, 0xad74143d_903d_4ab7_8066_28d363036d65);
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureFXAec, windows_core::IUnknown);
impl IDirectSoundCaptureFXAec {
    pub unsafe fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pdscfxaec) }
    }
    pub unsafe fn GetAllParameters(&self) -> windows_core::Result<DSCFXAec> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
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
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSCFXAec) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCFXAec) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectSoundCaptureFXAec_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSCFXAec>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl IDirectSoundCaptureFXAec_Vtbl {
    pub const fn new<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxaec: *const DSCFXAec) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureFXAec_Impl::SetAllParameters(this, core::mem::transmute_copy(&pdscfxaec)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxaec: *mut DSCFXAec) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetStatus<Identity: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundCaptureFXAec {}
windows_core::imp::define_interface!(IDirectSoundCaptureFXNoiseSuppress, IDirectSoundCaptureFXNoiseSuppress_Vtbl, 0xed311e41_fbae_4175_9625_cd0854f693ca);
windows_core::imp::interface_hierarchy!(IDirectSoundCaptureFXNoiseSuppress, windows_core::IUnknown);
impl IDirectSoundCaptureFXNoiseSuppress {
    pub unsafe fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> windows_core::HRESULT {
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
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSCFXNoiseSuppress) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSCFXNoiseSuppress) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectSoundCaptureFXNoiseSuppress_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSCFXNoiseSuppress>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl IDirectSoundCaptureFXNoiseSuppress_Vtbl {
    pub const fn new<Identity: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundCaptureFXNoiseSuppress_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdscfxnoisesuppress)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> windows_core::HRESULT {
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
windows_core::imp::define_interface!(IDirectSoundFXChorus, IDirectSoundFXChorus_Vtbl, 0x880842e3_145f_43e6_a934_a71806e50547);
windows_core::imp::interface_hierarchy!(IDirectSoundFXChorus, windows_core::IUnknown);
impl IDirectSoundFXChorus {
    pub unsafe fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxchorus) }
    }
    pub unsafe fn GetAllParameters(&self, pdsfxchorus: *mut DSFXChorus) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxchorus as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXChorus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXChorus) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXChorus) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXChorus_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxchorus: *mut DSFXChorus) -> windows_core::Result<()>;
}
impl IDirectSoundFXChorus_Vtbl {
    pub const fn new<Identity: IDirectSoundFXChorus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXChorus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxchorus: *const DSFXChorus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXChorus_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxchorus)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXChorus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxchorus: *mut DSFXChorus) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXChorus {}
windows_core::imp::define_interface!(IDirectSoundFXCompressor, IDirectSoundFXCompressor_Vtbl, 0x4bbd1154_62f6_4e2c_a15c_d3b6c417f7a0);
windows_core::imp::interface_hierarchy!(IDirectSoundFXCompressor, windows_core::IUnknown);
impl IDirectSoundFXCompressor {
    pub unsafe fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxcompressor) }
    }
    pub unsafe fn GetAllParameters(&self, pdsfxcompressor: *mut DSFXCompressor) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxcompressor as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXCompressor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXCompressor) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXCompressor) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXCompressor_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxcompressor: *mut DSFXCompressor) -> windows_core::Result<()>;
}
impl IDirectSoundFXCompressor_Vtbl {
    pub const fn new<Identity: IDirectSoundFXCompressor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXCompressor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxcompressor: *const DSFXCompressor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXCompressor_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxcompressor)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXCompressor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxcompressor: *mut DSFXCompressor) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXCompressor {}
windows_core::imp::define_interface!(IDirectSoundFXDistortion, IDirectSoundFXDistortion_Vtbl, 0x8ecf4326_455f_4d8b_bda9_8d5d3e9e3e0b);
windows_core::imp::interface_hierarchy!(IDirectSoundFXDistortion, windows_core::IUnknown);
impl IDirectSoundFXDistortion {
    pub unsafe fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxdistortion) }
    }
    pub unsafe fn GetAllParameters(&self, pdsfxdistortion: *mut DSFXDistortion) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxdistortion as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXDistortion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXDistortion) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXDistortion) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXDistortion_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxdistortion: *mut DSFXDistortion) -> windows_core::Result<()>;
}
impl IDirectSoundFXDistortion_Vtbl {
    pub const fn new<Identity: IDirectSoundFXDistortion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXDistortion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxdistortion: *const DSFXDistortion) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXDistortion_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxdistortion)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXDistortion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxdistortion: *mut DSFXDistortion) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXDistortion {}
windows_core::imp::define_interface!(IDirectSoundFXEcho, IDirectSoundFXEcho_Vtbl, 0x8bd28edf_50db_4e92_a2bd_445488d1ed42);
windows_core::imp::interface_hierarchy!(IDirectSoundFXEcho, windows_core::IUnknown);
impl IDirectSoundFXEcho {
    pub unsafe fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxecho) }
    }
    pub unsafe fn GetAllParameters(&self, pdsfxecho: *mut DSFXEcho) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxecho as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXEcho_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXEcho) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXEcho) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXEcho_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxecho: *mut DSFXEcho) -> windows_core::Result<()>;
}
impl IDirectSoundFXEcho_Vtbl {
    pub const fn new<Identity: IDirectSoundFXEcho_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXEcho_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxecho: *const DSFXEcho) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXEcho_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxecho)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXEcho_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxecho: *mut DSFXEcho) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXEcho {}
windows_core::imp::define_interface!(IDirectSoundFXFlanger, IDirectSoundFXFlanger_Vtbl, 0x903e9878_2c92_4072_9b2c_ea68f5396783);
windows_core::imp::interface_hierarchy!(IDirectSoundFXFlanger, windows_core::IUnknown);
impl IDirectSoundFXFlanger {
    pub unsafe fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxflanger) }
    }
    pub unsafe fn GetAllParameters(&self, pdsfxflanger: *mut DSFXFlanger) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxflanger as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXFlanger_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXFlanger) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXFlanger) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXFlanger_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxflanger: *mut DSFXFlanger) -> windows_core::Result<()>;
}
impl IDirectSoundFXFlanger_Vtbl {
    pub const fn new<Identity: IDirectSoundFXFlanger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXFlanger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxflanger: *const DSFXFlanger) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXFlanger_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxflanger)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXFlanger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxflanger: *mut DSFXFlanger) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXFlanger {}
windows_core::imp::define_interface!(IDirectSoundFXGargle, IDirectSoundFXGargle_Vtbl, 0xd616f352_d622_11ce_aac5_0020af0b99a3);
windows_core::imp::interface_hierarchy!(IDirectSoundFXGargle, windows_core::IUnknown);
impl IDirectSoundFXGargle {
    pub unsafe fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxgargle) }
    }
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
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXGargle) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXGargle) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXGargle_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXGargle>;
}
impl IDirectSoundFXGargle_Vtbl {
    pub const fn new<Identity: IDirectSoundFXGargle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXGargle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxgargle: *const DSFXGargle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXGargle_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxgargle)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXGargle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxgargle: *mut DSFXGargle) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXGargle {}
windows_core::imp::define_interface!(IDirectSoundFXI3DL2Reverb, IDirectSoundFXI3DL2Reverb_Vtbl, 0x4b166a6a_0d66_43f3_80e3_ee6280dee1a4);
windows_core::imp::interface_hierarchy!(IDirectSoundFXI3DL2Reverb, windows_core::IUnknown);
impl IDirectSoundFXI3DL2Reverb {
    pub unsafe fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxi3dl2reverb) }
    }
    pub unsafe fn GetAllParameters(&self, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllParameters)(windows_core::Interface::as_raw(self), pdsfxi3dl2reverb as _) }
    }
    pub unsafe fn SetPreset(&self, dwpreset: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPreset)(windows_core::Interface::as_raw(self), dwpreset) }
    }
    pub unsafe fn GetPreset(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetQuality(&self, lquality: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQuality)(windows_core::Interface::as_raw(self), lquality) }
    }
    pub unsafe fn GetQuality(&self) -> windows_core::Result<i32> {
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
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXI3DL2Reverb) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXI3DL2Reverb) -> windows_core::HRESULT,
    pub SetPreset: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPreset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXI3DL2Reverb_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> windows_core::Result<()>;
    fn SetPreset(&self, dwpreset: u32) -> windows_core::Result<()>;
    fn GetPreset(&self) -> windows_core::Result<u32>;
    fn SetQuality(&self, lquality: i32) -> windows_core::Result<()>;
    fn GetQuality(&self) -> windows_core::Result<i32>;
}
impl IDirectSoundFXI3DL2Reverb_Vtbl {
    pub const fn new<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXI3DL2Reverb_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxi3dl2reverb)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXI3DL2Reverb_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxi3dl2reverb)).into()
            }
        }
        unsafe extern "system" fn SetPreset<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpreset: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXI3DL2Reverb_Impl::SetPreset(this, core::mem::transmute_copy(&dwpreset)).into()
            }
        }
        unsafe extern "system" fn GetPreset<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpreset: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetQuality<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquality: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXI3DL2Reverb_Impl::SetQuality(this, core::mem::transmute_copy(&lquality)).into()
            }
        }
        unsafe extern "system" fn GetQuality<Identity: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquality: *mut i32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXI3DL2Reverb {}
windows_core::imp::define_interface!(IDirectSoundFXParamEq, IDirectSoundFXParamEq_Vtbl, 0xc03ca9fe_fe90_4204_8078_82334cd177da);
windows_core::imp::interface_hierarchy!(IDirectSoundFXParamEq, windows_core::IUnknown);
impl IDirectSoundFXParamEq {
    pub unsafe fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxparameq) }
    }
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
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXParamEq) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXParamEq) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXParamEq_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXParamEq>;
}
impl IDirectSoundFXParamEq_Vtbl {
    pub const fn new<Identity: IDirectSoundFXParamEq_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXParamEq_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxparameq: *const DSFXParamEq) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXParamEq_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxparameq)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXParamEq_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxparameq: *mut DSFXParamEq) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXParamEq {}
windows_core::imp::define_interface!(IDirectSoundFXWavesReverb, IDirectSoundFXWavesReverb_Vtbl, 0x46858c3a_0dc6_45e3_b760_d4eef16cb325);
windows_core::imp::interface_hierarchy!(IDirectSoundFXWavesReverb, windows_core::IUnknown);
impl IDirectSoundFXWavesReverb {
    pub unsafe fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllParameters)(windows_core::Interface::as_raw(self), pcdsfxwavesreverb) }
    }
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
    pub SetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const DSFXWavesReverb) -> windows_core::HRESULT,
    pub GetAllParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSFXWavesReverb) -> windows_core::HRESULT,
}
pub trait IDirectSoundFXWavesReverb_Impl: windows_core::IUnknownImpl {
    fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXWavesReverb>;
}
impl IDirectSoundFXWavesReverb_Vtbl {
    pub const fn new<Identity: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllParameters<Identity: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxwavesreverb: *const DSFXWavesReverb) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectSoundFXWavesReverb_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxwavesreverb)).into()
            }
        }
        unsafe extern "system" fn GetAllParameters<Identity: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxwavesreverb: *mut DSFXWavesReverb) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDirectSoundFXWavesReverb {}
windows_core::imp::define_interface!(IDirectSoundFullDuplex, IDirectSoundFullDuplex_Vtbl, 0xedcb4c7a_daab_4216_a42e_6c50596ddc1d);
windows_core::imp::interface_hierarchy!(IDirectSoundFullDuplex, windows_core::IUnknown);
impl IDirectSoundFullDuplex {
    #[cfg(all(feature = "mmeapi", feature = "windef"))]
    pub unsafe fn Initialize(&self, pcaptureguid: *const windows_core::GUID, prenderguid: *const windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::windef::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut Option<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: *mut Option<IDirectSoundBuffer8>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pcaptureguid, prenderguid, lpdscbufferdesc, lpdsbufferdesc, hwnd, dwlevel, core::mem::transmute(lplpdirectsoundcapturebuffer8), core::mem::transmute(lplpdirectsoundbuffer8)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFullDuplex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mmeapi", feature = "windef"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *const DSCBUFFERDESC, *const DSBUFFERDESC, super::windef::HWND, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "windef")))]
    Initialize: usize,
}
#[cfg(all(feature = "mmeapi", feature = "windef"))]
pub trait IDirectSoundFullDuplex_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pcaptureguid: *const windows_core::GUID, prenderguid: *const windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::windef::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: windows_core::OutRef<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: windows_core::OutRef<IDirectSoundBuffer8>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mmeapi", feature = "windef"))]
impl IDirectSoundFullDuplex_Vtbl {
    pub const fn new<Identity: IDirectSoundFullDuplex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDirectSoundFullDuplex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaptureguid: *const windows_core::GUID, prenderguid: *const windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::windef::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut *mut core::ffi::c_void, lplpdirectsoundbuffer8: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mmeapi", feature = "windef"))]
impl windows_core::RuntimeName for IDirectSoundFullDuplex {}
windows_core::imp::define_interface!(IDirectSoundNotify, IDirectSoundNotify_Vtbl, 0xb0210783_89cd_11d0_af08_00a0c925cd16);
windows_core::imp::interface_hierarchy!(IDirectSoundNotify, windows_core::IUnknown);
impl IDirectSoundNotify {
    #[cfg(feature = "winnt")]
    pub unsafe fn SetNotificationPositions(&self, pcpositionnotifies: &[DSBPOSITIONNOTIFY]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNotificationPositions)(windows_core::Interface::as_raw(self), pcpositionnotifies.len().try_into().unwrap(), core::mem::transmute(pcpositionnotifies.as_ptr())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub SetNotificationPositions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DSBPOSITIONNOTIFY) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetNotificationPositions: usize,
}
#[cfg(feature = "winnt")]
pub trait IDirectSoundNotify_Impl: windows_core::IUnknownImpl {
    fn SetNotificationPositions(&self, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IDirectSoundNotify_Vtbl {
    pub const fn new<Identity: IDirectSoundNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNotificationPositions<Identity: IDirectSoundNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> windows_core::HRESULT {
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
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IDirectSoundNotify {}
windows_core::imp::define_interface!(IKsPropertySet, IKsPropertySet_Vtbl, 0x31efac30_515c_11d0_a9aa_00aa0061be93);
windows_core::imp::interface_hierarchy!(IKsPropertySet, windows_core::IUnknown);
impl IKsPropertySet {
    pub unsafe fn Get(&self, rguidpropset: *const windows_core::GUID, ulid: u32, pinstancedata: Option<*const core::ffi::c_void>, ulinstancelength: u32, ppropertydata: *mut core::ffi::c_void, uldatalength: u32, pulbytesreturned: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), rguidpropset, ulid, pinstancedata.unwrap_or(core::mem::zeroed()) as _, ulinstancelength, ppropertydata as _, uldatalength, pulbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Set(&self, rguidpropset: *const windows_core::GUID, ulid: u32, pinstancedata: Option<*const core::ffi::c_void>, ulinstancelength: u32, ppropertydata: *const core::ffi::c_void, uldatalength: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), rguidpropset, ulid, pinstancedata.unwrap_or(core::mem::zeroed()) as _, ulinstancelength, ppropertydata, uldatalength) }
    }
    pub unsafe fn QuerySupport(&self, rguidpropset: *const windows_core::GUID, ulid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuerySupport)(windows_core::Interface::as_raw(self), rguidpropset, ulid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsPropertySet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QuerySupport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IKsPropertySet_Impl: windows_core::IUnknownImpl {
    fn Get(&self, rguidpropset: *const windows_core::GUID, ulid: u32, pinstancedata: *const core::ffi::c_void, ulinstancelength: u32, ppropertydata: *mut core::ffi::c_void, uldatalength: u32, pulbytesreturned: *mut u32) -> windows_core::Result<()>;
    fn Set(&self, rguidpropset: *const windows_core::GUID, ulid: u32, pinstancedata: *const core::ffi::c_void, ulinstancelength: u32, ppropertydata: *const core::ffi::c_void, uldatalength: u32) -> windows_core::Result<()>;
    fn QuerySupport(&self, rguidpropset: *const windows_core::GUID, ulid: u32) -> windows_core::Result<u32>;
}
impl IKsPropertySet_Vtbl {
    pub const fn new<Identity: IKsPropertySet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Get<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidpropset: *const windows_core::GUID, ulid: u32, pinstancedata: *const core::ffi::c_void, ulinstancelength: u32, ppropertydata: *mut core::ffi::c_void, uldatalength: u32, pulbytesreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsPropertySet_Impl::Get(this, core::mem::transmute_copy(&rguidpropset), core::mem::transmute_copy(&ulid), core::mem::transmute_copy(&pinstancedata), core::mem::transmute_copy(&ulinstancelength), core::mem::transmute_copy(&ppropertydata), core::mem::transmute_copy(&uldatalength), core::mem::transmute_copy(&pulbytesreturned)).into()
            }
        }
        unsafe extern "system" fn Set<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidpropset: *const windows_core::GUID, ulid: u32, pinstancedata: *const core::ffi::c_void, ulinstancelength: u32, ppropertydata: *const core::ffi::c_void, uldatalength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsPropertySet_Impl::Set(this, core::mem::transmute_copy(&rguidpropset), core::mem::transmute_copy(&ulid), core::mem::transmute_copy(&pinstancedata), core::mem::transmute_copy(&ulinstancelength), core::mem::transmute_copy(&ppropertydata), core::mem::transmute_copy(&uldatalength)).into()
            }
        }
        unsafe extern "system" fn QuerySupport<Identity: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidpropset: *const windows_core::GUID, ulid: u32, pultypesupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsPropertySet_Impl::QuerySupport(this, core::mem::transmute_copy(&rguidpropset), core::mem::transmute_copy(&ulid)) {
                    Ok(ok__) => {
                        pultypesupport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IKsPropertySet {}
windows_core::imp::define_interface!(IReferenceClock, IReferenceClock_Vtbl, 0x56a86897_0ad4_11ce_b03a_0020af0ba770);
windows_core::imp::interface_hierarchy!(IReferenceClock, windows_core::IUnknown);
impl IReferenceClock {
    #[cfg(feature = "mediaobj")]
    pub unsafe fn GetTime(&self) -> windows_core::Result<super::mediaobj::REFERENCE_TIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "mediaobj", feature = "winnt"))]
    pub unsafe fn AdviseTime(&self, rtbasetime: super::mediaobj::REFERENCE_TIME, rtstreamtime: super::mediaobj::REFERENCE_TIME, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdviseTime)(windows_core::Interface::as_raw(self), rtbasetime, rtstreamtime, hevent, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "mediaobj", feature = "winnt"))]
    pub unsafe fn AdvisePeriodic(&self, rtstarttime: super::mediaobj::REFERENCE_TIME, rtperiodtime: super::mediaobj::REFERENCE_TIME, hsemaphore: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdvisePeriodic)(windows_core::Interface::as_raw(self), rtstarttime, rtperiodtime, hsemaphore, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwadvisecookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwadvisecookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mediaobj")]
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "mediaobj"))]
    GetTime: usize,
    #[cfg(all(feature = "mediaobj", feature = "winnt"))]
    pub AdviseTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::mediaobj::REFERENCE_TIME, super::mediaobj::REFERENCE_TIME, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mediaobj", feature = "winnt")))]
    AdviseTime: usize,
    #[cfg(all(feature = "mediaobj", feature = "winnt"))]
    pub AdvisePeriodic: unsafe extern "system" fn(*mut core::ffi::c_void, super::mediaobj::REFERENCE_TIME, super::mediaobj::REFERENCE_TIME, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mediaobj", feature = "winnt")))]
    AdvisePeriodic: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mediaobj", feature = "winnt"))]
pub trait IReferenceClock_Impl: windows_core::IUnknownImpl {
    fn GetTime(&self) -> windows_core::Result<super::mediaobj::REFERENCE_TIME>;
    fn AdviseTime(&self, rtbasetime: super::mediaobj::REFERENCE_TIME, rtstreamtime: super::mediaobj::REFERENCE_TIME, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn AdvisePeriodic(&self, rtstarttime: super::mediaobj::REFERENCE_TIME, rtperiodtime: super::mediaobj::REFERENCE_TIME, hsemaphore: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwadvisecookie: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mediaobj", feature = "winnt"))]
impl IReferenceClock_Vtbl {
    pub const fn new<Identity: IReferenceClock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTime<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AdviseTime<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtbasetime: super::mediaobj::REFERENCE_TIME, rtstreamtime: super::mediaobj::REFERENCE_TIME, hevent: super::winnt::HANDLE, pdwadvisecookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceClock_Impl::AdviseTime(this, core::mem::transmute_copy(&rtbasetime), core::mem::transmute_copy(&rtstreamtime), core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwadvisecookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AdvisePeriodic<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtstarttime: super::mediaobj::REFERENCE_TIME, rtperiodtime: super::mediaobj::REFERENCE_TIME, hsemaphore: super::winnt::HANDLE, pdwadvisecookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceClock_Impl::AdvisePeriodic(this, core::mem::transmute_copy(&rtstarttime), core::mem::transmute_copy(&rtperiodtime), core::mem::transmute_copy(&hsemaphore)) {
                    Ok(ok__) => {
                        pdwadvisecookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwadvisecookie: u32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "mediaobj", feature = "winnt"))]
impl windows_core::RuntimeName for IReferenceClock {}
pub const KSPROPERTY_SUPPORT_GET: u32 = 1;
pub const KSPROPERTY_SUPPORT_SET: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDS3DBUFFER(pub *const DS3DBUFFER);
impl LPCDS3DBUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDS3DBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDS3DLISTENER(pub *const DS3DLISTENER);
impl LPCDS3DLISTENER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDS3DLISTENER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSBCAPS(pub *const DSBCAPS);
impl LPCDSBCAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSBCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSBPOSITIONNOTIFY(pub *const DSBPOSITIONNOTIFY);
#[cfg(feature = "winnt")]
impl LPCDSBPOSITIONNOTIFY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPCDSBPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSBUFFERDESC(pub *const DSBUFFERDESC);
#[cfg(feature = "mmeapi")]
impl LPCDSBUFFERDESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPCDSBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSBUFFERDESC1(pub *const DSBUFFERDESC1);
#[cfg(feature = "mmeapi")]
impl LPCDSBUFFERDESC1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPCDSBUFFERDESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSCAPS(pub *const DSCAPS);
impl LPCDSCAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSCBCAPS(pub *const DSCBCAPS);
impl LPCDSCBCAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSCBCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSCBUFFERDESC(pub *const DSCBUFFERDESC);
#[cfg(feature = "mmeapi")]
impl LPCDSCBUFFERDESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPCDSCBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSCCAPS(pub *const DSCCAPS);
impl LPCDSCCAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSCCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSCEFFECTDESC(pub *const DSCEFFECTDESC);
impl LPCDSCEFFECTDESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSCEFFECTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSCFXAec(pub *const DSCFXAec);
impl LPCDSCFXAec {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSCFXAec {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSCFXNoiseSuppress(pub *const DSCFXNoiseSuppress);
impl LPCDSCFXNoiseSuppress {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSCFXNoiseSuppress {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSEFFECTDESC(pub *const DSEFFECTDESC);
impl LPCDSEFFECTDESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSEFFECTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXChorus(pub *const DSFXChorus);
impl LPCDSFXChorus {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXChorus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXCompressor(pub *const DSFXCompressor);
impl LPCDSFXCompressor {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXCompressor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXDistortion(pub *const DSFXDistortion);
impl LPCDSFXDistortion {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXDistortion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXEcho(pub *const DSFXEcho);
impl LPCDSFXEcho {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXEcho {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXFlanger(pub *const DSFXFlanger);
impl LPCDSFXFlanger {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXFlanger {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXGargle(pub *const DSFXGargle);
impl LPCDSFXGargle {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXGargle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXI3DL2Reverb(pub *const DSFXI3DL2Reverb);
impl LPCDSFXI3DL2Reverb {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXI3DL2Reverb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXParamEq(pub *const DSFXParamEq);
impl LPCDSFXParamEq {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXParamEq {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCDSFXWavesReverb(pub *const DSFXWavesReverb);
impl LPCDSFXWavesReverb {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCDSFXWavesReverb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3DCOLOR(pub *mut u32);
impl LPD3DCOLOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPD3DCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3DVALUE(pub *mut f32);
impl LPD3DVALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPD3DVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPD3DVECTOR(pub *mut D3DVECTOR);
impl LPD3DVECTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPD3DVECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDS3DBUFFER(pub *mut DS3DBUFFER);
impl LPDS3DBUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDS3DBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDS3DLISTENER(pub *mut DS3DLISTENER);
impl LPDS3DLISTENER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDS3DLISTENER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSBCAPS(pub *mut DSBCAPS);
impl LPDSBCAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSBCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSBPOSITIONNOTIFY(pub *mut DSBPOSITIONNOTIFY);
#[cfg(feature = "winnt")]
impl LPDSBPOSITIONNOTIFY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for LPDSBPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSBUFFERDESC(pub *mut DSBUFFERDESC);
#[cfg(feature = "mmeapi")]
impl LPDSBUFFERDESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPDSBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSBUFFERDESC1(pub *mut DSBUFFERDESC1);
#[cfg(feature = "mmeapi")]
impl LPDSBUFFERDESC1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPDSBUFFERDESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSCAPS(pub *mut DSCAPS);
impl LPDSCAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSCBCAPS(pub *mut DSCBCAPS);
impl LPDSCBCAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSCBCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSCBUFFERDESC(pub *mut DSCBUFFERDESC);
#[cfg(feature = "mmeapi")]
impl LPDSCBUFFERDESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPDSCBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mmeapi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSCBUFFERDESC1(pub *mut DSCBUFFERDESC1);
#[cfg(feature = "mmeapi")]
impl LPDSCBUFFERDESC1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mmeapi")]
impl Default for LPDSCBUFFERDESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSCCAPS(pub *mut DSCCAPS);
impl LPDSCCAPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSCCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSCEFFECTDESC(pub *mut DSCEFFECTDESC);
impl LPDSCEFFECTDESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSCEFFECTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSCFXAec(pub *mut DSCFXAec);
impl LPDSCFXAec {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSCFXAec {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSCFXNoiseSuppress(pub *mut DSCFXNoiseSuppress);
impl LPDSCFXNoiseSuppress {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSCFXNoiseSuppress {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSEFFECTDESC(pub *mut DSEFFECTDESC);
impl LPDSEFFECTDESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSEFFECTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPDSENUMCALLBACKA = Option<unsafe extern "system" fn(param0: *mut windows_core::GUID, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDSENUMCALLBACKW = Option<unsafe extern "system" fn(param0: *mut windows_core::GUID, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXChorus(pub *mut DSFXChorus);
impl LPDSFXChorus {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXChorus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXCompressor(pub *mut DSFXCompressor);
impl LPDSFXCompressor {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXCompressor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXDistortion(pub *mut DSFXDistortion);
impl LPDSFXDistortion {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXDistortion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXEcho(pub *mut DSFXEcho);
impl LPDSFXEcho {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXEcho {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXFlanger(pub *mut DSFXFlanger);
impl LPDSFXFlanger {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXFlanger {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXGargle(pub *mut DSFXGargle);
impl LPDSFXGargle {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXGargle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXI3DL2Reverb(pub *mut DSFXI3DL2Reverb);
impl LPDSFXI3DL2Reverb {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXI3DL2Reverb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXParamEq(pub *mut DSFXParamEq);
impl LPDSFXParamEq {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXParamEq {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDSFXWavesReverb(pub *mut DSFXWavesReverb);
impl LPDSFXWavesReverb {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDSFXWavesReverb {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUND(pub *mut Option<IDirectSound>);
impl LPLPDIRECTSOUND {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUND3DBUFFER(pub *mut Option<IDirectSound3DBuffer>);
impl LPLPDIRECTSOUND3DBUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUND3DBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUND3DLISTENER(pub *mut Option<IDirectSound3DListener>);
impl LPLPDIRECTSOUND3DLISTENER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUND3DLISTENER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUND8(pub *mut Option<IDirectSound8>);
impl LPLPDIRECTSOUND8 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUND8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUNDBUFFER(pub *mut Option<IDirectSoundBuffer>);
impl LPLPDIRECTSOUNDBUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUNDBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUNDBUFFER8(pub *mut Option<IDirectSoundBuffer8>);
impl LPLPDIRECTSOUNDBUFFER8 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUNDBUFFER8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUNDCAPTURE(pub *mut Option<IDirectSoundCapture>);
impl LPLPDIRECTSOUNDCAPTURE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUNDCAPTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUNDCAPTURE8(pub *mut Option<IDirectSoundCapture>);
impl LPLPDIRECTSOUNDCAPTURE8 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUNDCAPTURE8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUNDCAPTUREBUFFER(pub *mut Option<IDirectSoundCaptureBuffer>);
impl LPLPDIRECTSOUNDCAPTUREBUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUNDCAPTUREBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUNDCAPTUREBUFFER8(pub *mut Option<IDirectSoundCaptureBuffer8>);
impl LPLPDIRECTSOUNDCAPTUREBUFFER8 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUNDCAPTUREBUFFER8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPLPDIRECTSOUNDNOTIFY(pub *mut Option<IDirectSoundNotify>);
impl LPLPDIRECTSOUNDNOTIFY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPLPDIRECTSOUNDNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mediaobj")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPREFERENCE_TIME(pub *mut super::mediaobj::REFERENCE_TIME);
#[cfg(feature = "mediaobj")]
impl LPREFERENCE_TIME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "mediaobj")]
impl Default for LPREFERENCE_TIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
