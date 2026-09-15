#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn DirectSoundCaptureCreate(pcguiddevice : super::LPCGUID, ppdsc : *mut LPDIRECTSOUNDCAPTURE, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn DirectSoundCaptureCreate8(pcguiddevice : super::LPCGUID, ppdsc8 : *mut LPDIRECTSOUNDCAPTURE8, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn DirectSoundCaptureEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn DirectSoundCreate(pcguiddevice : super::LPCGUID, ppds : *mut LPDIRECTSOUND, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn DirectSoundCreate8(pcguiddevice : super::LPCGUID, ppds8 : *mut LPDIRECTSOUND8, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn DirectSoundEnumerateA(pdsenumcallback : LPDSENUMCALLBACKA, pcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn DirectSoundEnumerateW(pdsenumcallback : LPDSENUMCALLBACKW, pcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "mmeapi", feature = "windef"))]
windows_link::link!("dsound.dll" "system" fn DirectSoundFullDuplexCreate(pcguidcapturedevice : super::LPCGUID, pcguidrenderdevice : super::LPCGUID, pcdscbufferdesc : LPCDSCBUFFERDESC, pcdsbufferdesc : LPCDSBUFFERDESC, hwnd : super::HWND, dwlevel : u32, ppdsfd : *mut LPDIRECTSOUNDFULLDUPLEX, ppdscbuffer8 : *mut LPDIRECTSOUNDCAPTUREBUFFER8, ppdsbuffer8 : *mut LPDIRECTSOUNDBUFFER8, punkouter : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("dsound.dll" "system" fn GetDeviceID(pguidsrc : super::LPCGUID, pguiddest : super::LPGUID) -> windows_sys::core::HRESULT);
pub const CLSID_DirectSound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x47d4d946_62e8_11cf_93bc_444553540000);
pub const CLSID_DirectSound8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3901cc3f_84b5_4fa4_ba35_aa8172b8a09b);
pub const CLSID_DirectSoundCapture: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb0210780_89cd_11d0_af08_00a0c925cd16);
pub const CLSID_DirectSoundCapture8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe4bcac13_7f99_4908_9a8e_74e3bf24b6e1);
pub const CLSID_DirectSoundFullDuplex: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfea4300c_7959_4147_b26a_2377b9e7a91d);
pub type D3DCOLOR = u32;
pub type D3DVALUE = f32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DVECTOR {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub const DIRECTSOUND_VERSION: i32 = 2304;
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
    pub flMinDistance: D3DVALUE,
    pub flMaxDistance: D3DVALUE,
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
#[derive(Clone, Copy, Default)]
pub struct DSBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwUnlockTransferRate: u32,
    pub dwPlayCpuOverhead: u32,
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: u32,
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
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::LPWAVEFORMATEX,
    pub guid3DAlgorithm: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DSBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::LPWAVEFORMATEX,
}
pub const DSBVOLUME_MAX: i32 = 0;
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
#[derive(Clone, Copy, Default)]
pub struct DSCBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
}
pub const DSCBCAPS_CTRLFX: i32 = 512;
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648;
pub const DSCBLOCK_ENTIREBUFFER: i32 = 1;
pub const DSCBSTART_LOOPING: i32 = 1;
pub const DSCBSTATUS_CAPTURING: i32 = 1;
pub const DSCBSTATUS_LOOPING: i32 = 2;
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DSCBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::LPWAVEFORMATEX,
    pub dwFXCount: u32,
    pub lpDSCFXDesc: LPDSCEFFECTDESC,
}
#[repr(C)]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy, Default)]
pub struct DSCBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: super::LPWAVEFORMATEX,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DSCCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFormats: u32,
    pub dwChannels: u32,
}
pub const DSCCAPS_CERTIFIED: i32 = 64;
pub const DSCCAPS_EMULDRIVER: i32 = 32;
pub const DSCCAPS_MULTIPLECAPTURE: i32 = 1;
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
pub const DSERR_ACCESSDENIED: windows_sys::core::HRESULT = 0x80070005_u32 as _;
pub const DSERR_ALLOCATED: windows_sys::core::HRESULT = 0x8878000A_u32 as _;
pub const DSERR_ALREADYINITIALIZED: windows_sys::core::HRESULT = 0x88780082_u32 as _;
pub const DSERR_BADFORMAT: windows_sys::core::HRESULT = 0x88780064_u32 as _;
pub const DSERR_BADSENDBUFFERGUID: windows_sys::core::HRESULT = 0x887800D2_u32 as _;
pub const DSERR_BUFFERLOST: windows_sys::core::HRESULT = 0x88780096_u32 as _;
pub const DSERR_BUFFERTOOSMALL: windows_sys::core::HRESULT = 0x887800B4_u32 as _;
pub const DSERR_CONTROLUNAVAIL: windows_sys::core::HRESULT = 0x8878001E_u32 as _;
pub const DSERR_DS8_REQUIRED: windows_sys::core::HRESULT = 0x887800BE_u32 as _;
pub const DSERR_FXUNAVAILABLE: windows_sys::core::HRESULT = 0x887800DC_u32 as _;
pub const DSERR_GENERIC: windows_sys::core::HRESULT = 0x80004005_u32 as _;
pub const DSERR_INVALIDCALL: windows_sys::core::HRESULT = 0x88780032_u32 as _;
pub const DSERR_INVALIDPARAM: windows_sys::core::HRESULT = 0x80070057_u32 as _;
pub const DSERR_NOAGGREGATION: windows_sys::core::HRESULT = 0x80040110_u32 as _;
pub const DSERR_NODRIVER: windows_sys::core::HRESULT = 0x88780078_u32 as _;
pub const DSERR_NOINTERFACE: windows_sys::core::HRESULT = 0x80004002_u32 as _;
pub const DSERR_OBJECTNOTFOUND: windows_sys::core::HRESULT = 0x88781161_u32 as _;
pub const DSERR_OTHERAPPHASPRIO: windows_sys::core::HRESULT = 0x887800A0_u32 as _;
pub const DSERR_OUTOFMEMORY: windows_sys::core::HRESULT = 0x8007000E_u32 as _;
pub const DSERR_PRIOLEVELNEEDED: windows_sys::core::HRESULT = 0x88780046_u32 as _;
pub const DSERR_SENDLOOP: windows_sys::core::HRESULT = 0x887800C8_u32 as _;
pub const DSERR_UNINITIALIZED: windows_sys::core::HRESULT = 0x887800AA_u32 as _;
pub const DSERR_UNSUPPORTED: windows_sys::core::HRESULT = 0x80004001_u32 as _;
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DSFXChorus {
    pub fWetDryMix: super::FLOAT,
    pub fDepth: super::FLOAT,
    pub fFeedback: super::FLOAT,
    pub fFrequency: super::FLOAT,
    pub lWaveform: i32,
    pub fDelay: super::FLOAT,
    pub lPhase: i32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DSFXEcho {
    pub fWetDryMix: super::FLOAT,
    pub fFeedback: super::FLOAT,
    pub fLeftDelay: super::FLOAT,
    pub fRightDelay: super::FLOAT,
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DSFXFlanger {
    pub fWetDryMix: super::FLOAT,
    pub fDepth: super::FLOAT,
    pub fFeedback: super::FLOAT,
    pub fFrequency: super::FLOAT,
    pub lWaveform: i32,
    pub fDelay: super::FLOAT,
    pub lPhase: i32,
}
pub const DSFXGARGLE_RATEHZ_MAX: i32 = 1000;
pub const DSFXGARGLE_RATEHZ_MIN: i32 = 1;
pub const DSFXGARGLE_WAVE_SQUARE: i32 = 1;
pub const DSFXGARGLE_WAVE_TRIANGLE: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DSFXGargle {
    pub dwRateHz: u32,
    pub dwWaveShape: u32,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DSFXI3DL2Reverb {
    pub lRoom: i32,
    pub lRoomHF: i32,
    pub flRoomRolloffFactor: super::FLOAT,
    pub flDecayTime: super::FLOAT,
    pub flDecayHFRatio: super::FLOAT,
    pub lReflections: i32,
    pub flReflectionsDelay: super::FLOAT,
    pub lReverb: i32,
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
pub const DS_NO_VIRTUALIZATION: windows_sys::core::HRESULT = 0x878000A_u32 as _;
pub const DS_OK: windows_sys::core::HRESULT = 0x0_u32 as _;
pub const DS_UNCERTIFIED: i32 = 1;
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
pub const KSPROPERTY_SUPPORT_GET: i32 = 1;
pub const KSPROPERTY_SUPPORT_SET: i32 = 2;
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
#[cfg(feature = "minwindef")]
pub type LPCDSFXChorus = *const DSFXChorus;
#[cfg(feature = "minwindef")]
pub type LPCDSFXCompressor = *const DSFXCompressor;
#[cfg(feature = "minwindef")]
pub type LPCDSFXDistortion = *const DSFXDistortion;
#[cfg(feature = "minwindef")]
pub type LPCDSFXEcho = *const DSFXEcho;
#[cfg(feature = "minwindef")]
pub type LPCDSFXFlanger = *const DSFXFlanger;
pub type LPCDSFXGargle = *const DSFXGargle;
#[cfg(feature = "minwindef")]
pub type LPCDSFXI3DL2Reverb = *const DSFXI3DL2Reverb;
#[cfg(feature = "minwindef")]
pub type LPCDSFXParamEq = *const DSFXParamEq;
#[cfg(feature = "minwindef")]
pub type LPCDSFXWavesReverb = *const DSFXWavesReverb;
pub type LPD3DCOLOR = *mut u32;
pub type LPD3DVALUE = *mut f32;
pub type LPD3DVECTOR = *mut D3DVECTOR;
pub type LPDIRECTSOUND = *mut core::ffi::c_void;
pub type LPDIRECTSOUND3DBUFFER = *mut core::ffi::c_void;
pub type LPDIRECTSOUND3DBUFFER8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUND3DLISTENER = *mut core::ffi::c_void;
pub type LPDIRECTSOUND3DLISTENER8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUND8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDBUFFER = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDBUFFER8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDCAPTURE = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDCAPTURE8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDCAPTUREBUFFER = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDCAPTUREBUFFER8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDCAPTUREFXAEC = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDCAPTUREFXAEC8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDCAPTUREFXNOISESUPPRESS = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDCAPTUREFXNOISESUPPRESS8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFULLDUPLEX = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFULLDUPLEX8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXCHORUS = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXCHORUS8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXCOMPRESSOR = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXCOMPRESSOR8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXDISTORTION = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXDISTORTION8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXECHO = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXECHO8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXFLANGER = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXFLANGER8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXGARGLE = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXGARGLE8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXI3DL2REVERB = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXI3DL2REVERB8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXPARAMEQ = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXPARAMEQ8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXWAVESREVERB = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDFXWAVESREVERB8 = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDNOTIFY = *mut core::ffi::c_void;
pub type LPDIRECTSOUNDNOTIFY8 = *mut core::ffi::c_void;
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
#[cfg(feature = "guiddef")]
pub type LPDSENUMCALLBACKA = Option<unsafe extern "system" fn(param0: super::LPGUID, param1: windows_sys::core::PCSTR, param2: windows_sys::core::PCSTR, param3: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "guiddef")]
pub type LPDSENUMCALLBACKW = Option<unsafe extern "system" fn(param0: super::LPGUID, param1: windows_sys::core::PCWSTR, param2: windows_sys::core::PCWSTR, param3: *mut core::ffi::c_void) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type LPDSFXChorus = *mut DSFXChorus;
#[cfg(feature = "minwindef")]
pub type LPDSFXCompressor = *mut DSFXCompressor;
#[cfg(feature = "minwindef")]
pub type LPDSFXDistortion = *mut DSFXDistortion;
#[cfg(feature = "minwindef")]
pub type LPDSFXEcho = *mut DSFXEcho;
#[cfg(feature = "minwindef")]
pub type LPDSFXFlanger = *mut DSFXFlanger;
pub type LPDSFXGargle = *mut DSFXGargle;
#[cfg(feature = "minwindef")]
pub type LPDSFXI3DL2Reverb = *mut DSFXI3DL2Reverb;
#[cfg(feature = "minwindef")]
pub type LPDSFXParamEq = *mut DSFXParamEq;
#[cfg(feature = "minwindef")]
pub type LPDSFXWavesReverb = *mut DSFXWavesReverb;
pub type LPKSPROPERTYSET = *mut core::ffi::c_void;
pub type LPLPDIRECTSOUND = *mut LPDIRECTSOUND;
pub type LPLPDIRECTSOUND3DBUFFER = *mut LPDIRECTSOUND3DBUFFER;
pub type LPLPDIRECTSOUND3DLISTENER = *mut LPDIRECTSOUND3DLISTENER;
pub type LPLPDIRECTSOUND8 = *mut LPDIRECTSOUND8;
pub type LPLPDIRECTSOUNDBUFFER = *mut LPDIRECTSOUNDBUFFER;
pub type LPLPDIRECTSOUNDBUFFER8 = *mut LPDIRECTSOUNDBUFFER8;
pub type LPLPDIRECTSOUNDCAPTURE = *mut LPDIRECTSOUNDCAPTURE;
pub type LPLPDIRECTSOUNDCAPTURE8 = *mut LPDIRECTSOUNDCAPTURE8;
pub type LPLPDIRECTSOUNDCAPTUREBUFFER = *mut LPDIRECTSOUNDCAPTUREBUFFER;
pub type LPLPDIRECTSOUNDCAPTUREBUFFER8 = *mut LPDIRECTSOUNDCAPTUREBUFFER8;
pub type LPLPDIRECTSOUNDNOTIFY = *mut LPDIRECTSOUNDNOTIFY;
#[cfg(feature = "mediaobj")]
pub type LPREFERENCE_TIME = *mut super::REFERENCE_TIME;
pub const _FACDS: i32 = 2168;
