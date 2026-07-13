windows_link::link!("dsound.dll" "system" fn DirectSoundCaptureCreate(pcguiddevice : *const windows_sys::core::GUID, ppdsc : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dsound.dll" "system" fn DirectSoundCaptureCreate8(pcguiddevice : *const windows_sys::core::GUID, ppdsc8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dsound.dll" "system" fn DirectSoundCreate(pcguiddevice : *const windows_sys::core::GUID, ppds : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dsound.dll" "system" fn DirectSoundCreate8(pcguiddevice : *const windows_sys::core::GUID, ppds8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dsound.dll" "system" fn DirectSoundEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dsound.dll" "system" fn DirectSoundEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mmeapi", feature = "windef"))]
windows_link::link!("dsound.dll" "system" fn DirectSoundFullDuplexCreate(pcguidcapturedevice : *const windows_sys::core::GUID, pcguidrenderdevice : *const windows_sys::core::GUID, pcdscbufferdesc : *const DSCBUFFERDESC, pcdsbufferdesc : *const DSBUFFERDESC, hwnd : super::windef::HWND, dwlevel : u32, ppdsfd : *mut *mut core::ffi::c_void, ppdscbuffer8 : *mut *mut core::ffi::c_void, ppdsbuffer8 : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dsound.dll" "system" fn GetDeviceID(pguidsrc : *const windows_sys::core::GUID, pguiddest : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
pub const CLSID_DirectSound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x47d4d946_62e8_11cf_93bc_444553540000);
pub const CLSID_DirectSound8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3901cc3f_84b5_4fa4_ba35_aa8172b8a09b);
pub const CLSID_DirectSoundCapture: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb0210780_89cd_11d0_af08_00a0c925cd16);
pub const CLSID_DirectSoundCapture8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe4bcac13_7f99_4908_9a8e_74e3bf24b6e1);
pub const CLSID_DirectSoundFullDuplex: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfea4300c_7959_4147_b26a_2377b9e7a91d);
pub type D3DCOLOR = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DVECTOR {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub const DIRECTSOUND_VERSION: u32 = 2304;
pub const DS3DALG_HRTF_FULL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc2413340_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_HRTF_LIGHT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc2413342_1c1b_11d2_94f5_00c04fc28aca);
pub const DS3DALG_NO_VIRTUALIZATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc241333f_1c1b_11d2_94f5_00c04fc28aca);
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: u32,
    pub hEventNotify: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for DSBPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy)]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::mmeapi::LPWAVEFORMATEX,
    pub guid3DAlgorithm: windows_sys::core::GUID,
}
#[cfg(feature = "mmeapi")]
impl Default for DSBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct DSBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::mmeapi::LPWAVEFORMATEX,
}
#[cfg(feature = "mmeapi")]
impl Default for DSBUFFERDESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DSBVOLUME_MAX: u32 = 0;
pub const DSBVOLUME_MIN: i32 = -10000;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct DSCBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::mmeapi::LPWAVEFORMATEX,
    pub dwFXCount: u32,
    pub lpDSCFXDesc: LPDSCEFFECTDESC,
}
#[cfg(feature = "mmeapi")]
impl Default for DSCBUFFERDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct DSCBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::mmeapi::LPWAVEFORMATEX,
}
#[cfg(feature = "mmeapi")]
impl Default for DSCBUFFERDESC1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct DSCEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSCFXClass: windows_sys::core::GUID,
    pub guidDSCFXInstance: windows_sys::core::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DSCFXAec {
    pub fEnable: windows_sys::core::BOOL,
    pub fNoiseFill: windows_sys::core::BOOL,
    pub dwMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DSCFXNoiseSuppress {
    pub fEnable: windows_sys::core::BOOL,
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
pub const DSDEVID_DefaultCapture: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdef00001_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultPlayback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdef00000_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoiceCapture: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdef00003_9c6d_47ed_aaf1_4dda8f2b5c03);
pub const DSDEVID_DefaultVoicePlayback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdef00002_9c6d_47ed_aaf1_4dda8f2b5c03);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DSEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSFXClass: windows_sys::core::GUID,
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct DSFXGargle {
    pub dwRateHz: u32,
    pub dwWaveShape: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
pub const GUID_All_Objects: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa114de5_c262_4169_a1c8_23d698cc73b5);
pub const GUID_DSCFX_CLASS_AEC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbf963d80_c559_11d0_8a2b_00a0c9255ac1);
pub const GUID_DSCFX_CLASS_NS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe07f903f_62fd_4e60_8cdd_dea7236665b5);
pub const GUID_DSCFX_MS_AEC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcdebb919_379a_488a_8765_f53cfd36de40);
pub const GUID_DSCFX_MS_NS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x11c5c73b_66e9_4ba1_a0ba_e814c6eed92d);
pub const GUID_DSCFX_SYSTEM_AEC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1c22c56d_9879_4f5b_a389_27996ddc2810);
pub const GUID_DSCFX_SYSTEM_NS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5ab0882e_7274_4516_877d_4eee99ba4fd0);
pub const GUID_DSFX_STANDARD_CHORUS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xefe6629c_81f7_4281_bd91_c9d604a95af6);
pub const GUID_DSFX_STANDARD_COMPRESSOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef011f79_4000_406d_87af_bffb3fc39d57);
pub const GUID_DSFX_STANDARD_DISTORTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef114c90_cd1d_484e_96e5_09cfaf912a21);
pub const GUID_DSFX_STANDARD_ECHO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef3e932c_d40b_4f51_8ccf_3f98f1b29d5d);
pub const GUID_DSFX_STANDARD_FLANGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xefca3d92_dfd8_4672_a603_7420894bad98);
pub const GUID_DSFX_STANDARD_GARGLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdafd8210_5711_4b91_9fe3_f75b7ae279bf);
pub const GUID_DSFX_STANDARD_I3DL2REVERB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef985e71_d5c7_42d4_ba4d_2d073e2e96f4);
pub const GUID_DSFX_STANDARD_PARAMEQ: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x120ced89_3bf4_4173_a132_3cb406cf3231);
pub const GUID_DSFX_WAVES_REVERB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x87fc0268_9a55_4360_95aa_004a1d9de26c);
pub const KSPROPERTY_SUPPORT_GET: u32 = 1;
pub const KSPROPERTY_SUPPORT_SET: u32 = 2;
pub type LPCDS3DBUFFER = *const DS3DBUFFER;
pub type LPCDS3DLISTENER = *const DS3DLISTENER;
pub type LPCDSBCAPS = *const DSBCAPS;
#[cfg(feature = "winnt")]
pub type LPCDSBPOSITIONNOTIFY = *const DSBPOSITIONNOTIFY;
#[cfg(feature = "mmeapi")]
pub type LPCDSBUFFERDESC = *const DSBUFFERDESC;
#[cfg(feature = "mmeapi")]
pub type LPCDSBUFFERDESC1 = *const DSBUFFERDESC1;
pub type LPCDSCAPS = *const DSCAPS;
pub type LPCDSCBCAPS = *const DSCBCAPS;
#[cfg(feature = "mmeapi")]
pub type LPCDSCBUFFERDESC = *const DSCBUFFERDESC;
pub type LPCDSCCAPS = *const DSCCAPS;
pub type LPCDSCEFFECTDESC = *const DSCEFFECTDESC;
pub type LPCDSCFXAec = *const DSCFXAec;
pub type LPCDSCFXNoiseSuppress = *const DSCFXNoiseSuppress;
pub type LPCDSEFFECTDESC = *const DSEFFECTDESC;
pub type LPCDSFXChorus = *const DSFXChorus;
pub type LPCDSFXCompressor = *const DSFXCompressor;
pub type LPCDSFXDistortion = *const DSFXDistortion;
pub type LPCDSFXEcho = *const DSFXEcho;
pub type LPCDSFXFlanger = *const DSFXFlanger;
pub type LPCDSFXGargle = *const DSFXGargle;
pub type LPCDSFXI3DL2Reverb = *const DSFXI3DL2Reverb;
pub type LPCDSFXParamEq = *const DSFXParamEq;
pub type LPCDSFXWavesReverb = *const DSFXWavesReverb;
pub type LPD3DCOLOR = *mut u32;
pub type LPD3DVALUE = *mut f32;
pub type LPD3DVECTOR = *mut D3DVECTOR;
pub type LPDS3DBUFFER = *mut DS3DBUFFER;
pub type LPDS3DLISTENER = *mut DS3DLISTENER;
pub type LPDSBCAPS = *mut DSBCAPS;
#[cfg(feature = "winnt")]
pub type LPDSBPOSITIONNOTIFY = *mut DSBPOSITIONNOTIFY;
#[cfg(feature = "mmeapi")]
pub type LPDSBUFFERDESC = *mut DSBUFFERDESC;
#[cfg(feature = "mmeapi")]
pub type LPDSBUFFERDESC1 = *mut DSBUFFERDESC1;
pub type LPDSCAPS = *mut DSCAPS;
pub type LPDSCBCAPS = *mut DSCBCAPS;
#[cfg(feature = "mmeapi")]
pub type LPDSCBUFFERDESC = *mut DSCBUFFERDESC;
#[cfg(feature = "mmeapi")]
pub type LPDSCBUFFERDESC1 = *mut DSCBUFFERDESC1;
pub type LPDSCCAPS = *mut DSCCAPS;
pub type LPDSCEFFECTDESC = *mut DSCEFFECTDESC;
pub type LPDSCFXAec = *mut DSCFXAec;
pub type LPDSCFXNoiseSuppress = *mut DSCFXNoiseSuppress;
pub type LPDSEFFECTDESC = *mut DSEFFECTDESC;
pub type LPDSENUMCALLBACKA = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDSENUMCALLBACKW = Option<unsafe extern "system" fn(param0: *mut windows_sys::core::GUID, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
pub type LPDSFXChorus = *mut DSFXChorus;
pub type LPDSFXCompressor = *mut DSFXCompressor;
pub type LPDSFXDistortion = *mut DSFXDistortion;
pub type LPDSFXEcho = *mut DSFXEcho;
pub type LPDSFXFlanger = *mut DSFXFlanger;
pub type LPDSFXGargle = *mut DSFXGargle;
pub type LPDSFXI3DL2Reverb = *mut DSFXI3DL2Reverb;
pub type LPDSFXParamEq = *mut DSFXParamEq;
pub type LPDSFXWavesReverb = *mut DSFXWavesReverb;
pub type LPLPDIRECTSOUND = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUND3DBUFFER = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUND3DLISTENER = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUND8 = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUNDBUFFER = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUNDBUFFER8 = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUNDCAPTURE = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUNDCAPTURE8 = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUNDCAPTUREBUFFER = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUNDCAPTUREBUFFER8 = *mut *mut core::ffi::c_void;
pub type LPLPDIRECTSOUNDNOTIFY = *mut *mut core::ffi::c_void;
#[cfg(feature = "mediaobj")]
pub type LPREFERENCE_TIME = *mut super::mediaobj::REFERENCE_TIME;
