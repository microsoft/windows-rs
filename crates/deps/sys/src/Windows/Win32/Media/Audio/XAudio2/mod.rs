#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn CreateAudioReverb(ppapo: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn CreateAudioVolumeMeter(ppapo: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn CreateFX(clsid: *const ::windows_sys::core::GUID, peffect: *mut ::windows_sys::core::IUnknown, pinitdat: *const ::core::ffi::c_void, initdatabytesize: u32) -> ::windows_sys::core::HRESULT;
    pub fn CreateHrtfApo(init: *const HrtfApoInit, xapo: *mut IXAPO) -> ::windows_sys::core::HRESULT;
    pub fn XAudio2CreateWithVersionInfo(ppxaudio2: *mut IXAudio2, flags: u32, xaudio2processor: u32, ntddiversion: u32) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct AudioReverb(i32);
#[repr(C)]
pub struct AudioVolumeMeter(i32);
pub const FACILITY_XAPO: u32 = 2199u32;
pub const FACILITY_XAUDIO2: u32 = 2198u32;
pub const FXECHO_DEFAULT_DELAY: f32 = 500f32;
pub const FXECHO_DEFAULT_FEEDBACK: f32 = 0.5f32;
pub const FXECHO_DEFAULT_WETDRYMIX: f32 = 0.5f32;
#[repr(C)]
pub struct FXECHO_INITDATA(i32);
pub const FXECHO_MAX_DELAY: f32 = 2000f32;
pub const FXECHO_MAX_FEEDBACK: f32 = 1f32;
pub const FXECHO_MAX_WETDRYMIX: f32 = 1f32;
pub const FXECHO_MIN_DELAY: f32 = 1f32;
pub const FXECHO_MIN_FEEDBACK: f32 = 0f32;
pub const FXECHO_MIN_WETDRYMIX: f32 = 0f32;
#[repr(C)]
pub struct FXECHO_PARAMETERS(i32);
#[repr(C)]
pub struct FXEQ(i32);
pub const FXEQ_DEFAULT_BANDWIDTH: f32 = 1f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_0: f32 = 100f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_1: f32 = 800f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_2: f32 = 2000f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_3: f32 = 10000f32;
pub const FXEQ_DEFAULT_GAIN: f32 = 1f32;
pub const FXEQ_MAX_BANDWIDTH: f32 = 2f32;
pub const FXEQ_MAX_FRAMERATE: u32 = 48000u32;
pub const FXEQ_MAX_FREQUENCY_CENTER: f32 = 20000f32;
pub const FXEQ_MAX_GAIN: f32 = 7.94f32;
pub const FXEQ_MIN_BANDWIDTH: f32 = 0.1f32;
pub const FXEQ_MIN_FRAMERATE: u32 = 22000u32;
pub const FXEQ_MIN_FREQUENCY_CENTER: f32 = 20f32;
pub const FXEQ_MIN_GAIN: f32 = 0.126f32;
#[repr(C)]
pub struct FXEQ_PARAMETERS(i32);
#[repr(C)]
pub struct FXEcho(i32);
pub const FXLOUDNESS_DEFAULT_MOMENTARY_MS: u32 = 400u32;
pub const FXLOUDNESS_DEFAULT_SHORTTERM_MS: u32 = 3000u32;
pub const FXMASTERINGLIMITER_DEFAULT_LOUDNESS: u32 = 1000u32;
pub const FXMASTERINGLIMITER_DEFAULT_RELEASE: u32 = 6u32;
pub const FXMASTERINGLIMITER_MAX_LOUDNESS: u32 = 1800u32;
pub const FXMASTERINGLIMITER_MAX_RELEASE: u32 = 20u32;
pub const FXMASTERINGLIMITER_MIN_LOUDNESS: u32 = 1u32;
pub const FXMASTERINGLIMITER_MIN_RELEASE: u32 = 1u32;
#[repr(C)]
pub struct FXMASTERINGLIMITER_PARAMETERS(i32);
#[repr(C)]
pub struct FXMasteringLimiter(i32);
pub const FXREVERB_DEFAULT_DIFFUSION: f32 = 0.9f32;
pub const FXREVERB_DEFAULT_ROOMSIZE: f32 = 0.6f32;
pub const FXREVERB_MAX_DIFFUSION: f32 = 1f32;
pub const FXREVERB_MAX_ROOMSIZE: f32 = 1f32;
pub const FXREVERB_MIN_DIFFUSION: f32 = 0f32;
pub const FXREVERB_MIN_ROOMSIZE: f32 = 0.0001f32;
#[repr(C)]
pub struct FXREVERB_PARAMETERS(i32);
#[repr(C)]
pub struct FXReverb(i32);
pub const HRTF_DEFAULT_UNITY_GAIN_DISTANCE: f32 = 1f32;
pub const HRTF_MAX_GAIN_LIMIT: f32 = 12f32;
pub const HRTF_MIN_GAIN_LIMIT: f32 = -96f32;
pub const HRTF_MIN_UNITY_GAIN_DISTANCE: f32 = 0.05f32;
#[repr(C)]
pub struct HrtfApoInit(i32);
#[repr(C)]
pub struct HrtfDirectivity(i32);
#[repr(C)]
pub struct HrtfDirectivityCardioid(i32);
#[repr(C)]
pub struct HrtfDirectivityCone(i32);
#[repr(transparent)]
pub struct HrtfDirectivityType(pub i32);
pub const OmniDirectional: HrtfDirectivityType = HrtfDirectivityType(0i32);
pub const Cardioid: HrtfDirectivityType = HrtfDirectivityType(1i32);
pub const Cone: HrtfDirectivityType = HrtfDirectivityType(2i32);
#[repr(C)]
pub struct HrtfDistanceDecay(i32);
#[repr(transparent)]
pub struct HrtfDistanceDecayType(pub i32);
pub const NaturalDecay: HrtfDistanceDecayType = HrtfDistanceDecayType(0i32);
pub const CustomDecay: HrtfDistanceDecayType = HrtfDistanceDecayType(1i32);
#[repr(transparent)]
pub struct HrtfEnvironment(pub i32);
pub const Small: HrtfEnvironment = HrtfEnvironment(0i32);
pub const Medium: HrtfEnvironment = HrtfEnvironment(1i32);
pub const Large: HrtfEnvironment = HrtfEnvironment(2i32);
pub const Outdoors: HrtfEnvironment = HrtfEnvironment(3i32);
#[repr(C)]
pub struct HrtfOrientation(i32);
#[repr(C)]
pub struct HrtfPosition(i32);
#[repr(transparent)]
pub struct IXAPO(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAPOHrtfParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAPOParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAudio2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAudio2EngineCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAudio2Extension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAudio2MasteringVoice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAudio2SourceVoice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAudio2SubmixVoice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAudio2Voice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAudio2VoiceCallback(pub *mut ::core::ffi::c_void);
pub const Processor1: u32 = 1u32;
pub const Processor10: u32 = 512u32;
pub const Processor11: u32 = 1024u32;
pub const Processor12: u32 = 2048u32;
pub const Processor13: u32 = 4096u32;
pub const Processor14: u32 = 8192u32;
pub const Processor15: u32 = 16384u32;
pub const Processor16: u32 = 32768u32;
pub const Processor17: u32 = 65536u32;
pub const Processor18: u32 = 131072u32;
pub const Processor19: u32 = 262144u32;
pub const Processor2: u32 = 2u32;
pub const Processor20: u32 = 524288u32;
pub const Processor21: u32 = 1048576u32;
pub const Processor22: u32 = 2097152u32;
pub const Processor23: u32 = 4194304u32;
pub const Processor24: u32 = 8388608u32;
pub const Processor25: u32 = 16777216u32;
pub const Processor26: u32 = 33554432u32;
pub const Processor27: u32 = 67108864u32;
pub const Processor28: u32 = 134217728u32;
pub const Processor29: u32 = 268435456u32;
pub const Processor3: u32 = 4u32;
pub const Processor30: u32 = 536870912u32;
pub const Processor31: u32 = 1073741824u32;
pub const Processor32: u32 = 2147483648u32;
pub const Processor4: u32 = 8u32;
pub const Processor5: u32 = 16u32;
pub const Processor6: u32 = 32u32;
pub const Processor7: u32 = 64u32;
pub const Processor8: u32 = 128u32;
pub const Processor9: u32 = 256u32;
pub const SPEAKER_MONO: u32 = 4u32;
pub const X3DAUDIO_2PI: f32 = 6.2831855f32;
pub const X3DAUDIO_CALCULATE_DELAY: u32 = 2u32;
pub const X3DAUDIO_CALCULATE_DOPPLER: u32 = 32u32;
pub const X3DAUDIO_CALCULATE_EMITTER_ANGLE: u32 = 64u32;
pub const X3DAUDIO_CALCULATE_LPF_DIRECT: u32 = 4u32;
pub const X3DAUDIO_CALCULATE_LPF_REVERB: u32 = 8u32;
pub const X3DAUDIO_CALCULATE_MATRIX: u32 = 1u32;
pub const X3DAUDIO_CALCULATE_REDIRECT_TO_LFE: u32 = 131072u32;
pub const X3DAUDIO_CALCULATE_REVERB: u32 = 16u32;
pub const X3DAUDIO_CALCULATE_ZEROCENTER: u32 = 65536u32;
pub const X3DAUDIO_HANDLE_BYTESIZE: u32 = 20u32;
pub const X3DAUDIO_PI: f32 = 3.1415927f32;
pub const X3DAUDIO_SPEED_OF_SOUND: f32 = 343.5f32;
#[repr(transparent)]
pub struct XAPO_BUFFER_FLAGS(pub i32);
pub const XAPO_BUFFER_SILENT: XAPO_BUFFER_FLAGS = XAPO_BUFFER_FLAGS(0i32);
pub const XAPO_BUFFER_VALID: XAPO_BUFFER_FLAGS = XAPO_BUFFER_FLAGS(1i32);
pub const XAPO_E_FORMAT_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003369983i32 as _);
pub const XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH: u32 = 4u32;
pub const XAPO_FLAG_BUFFERCOUNT_MUST_MATCH: u32 = 8u32;
pub const XAPO_FLAG_CHANNELS_MUST_MATCH: u32 = 1u32;
pub const XAPO_FLAG_FRAMERATE_MUST_MATCH: u32 = 2u32;
pub const XAPO_FLAG_INPLACE_REQUIRED: u32 = 32u32;
pub const XAPO_FLAG_INPLACE_SUPPORTED: u32 = 16u32;
#[repr(C)]
pub struct XAPO_LOCKFORPROCESS_PARAMETERS(i32);
pub const XAPO_MAX_CHANNELS: u32 = 64u32;
pub const XAPO_MAX_FRAMERATE: u32 = 200000u32;
pub const XAPO_MIN_CHANNELS: u32 = 1u32;
pub const XAPO_MIN_FRAMERATE: u32 = 1000u32;
#[repr(C)]
pub struct XAPO_PROCESS_BUFFER_PARAMETERS(i32);
#[repr(C)]
pub struct XAPO_REGISTRATION_PROPERTIES(i32);
pub const XAPO_REGISTRATION_STRING_LENGTH: u32 = 256u32;
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_REAR_DELAY: u32 = 20u32;
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_SIDE_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_DECAY_TIME: f32 = 1f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DENSITY: f32 = 100f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DISABLE_LATE_FIELD: u32 = 0u32;
pub const XAUDIO2FX_REVERB_DEFAULT_EARLY_DIFFUSION: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_CUTOFF: u32 = 4u32;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LATE_DIFFUSION: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_CUTOFF: u32 = 4u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION: u32 = 6u32;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION_MATRIX: u32 = 27u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REAR_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_GAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_GAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_FREQ: f32 = 5000f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_HF: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_MAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_SIZE: f32 = 100f32;
pub const XAUDIO2FX_REVERB_DEFAULT_WET_DRY_MIX: f32 = 100f32;
#[repr(C)]
pub struct XAUDIO2FX_REVERB_I3DL2_PARAMETERS(i32);
pub const XAUDIO2FX_REVERB_MAX_7POINT1_REAR_DELAY: u32 = 20u32;
pub const XAUDIO2FX_REVERB_MAX_7POINT1_SIDE_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_MAX_DENSITY: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_DIFFUSION: u32 = 15u32;
pub const XAUDIO2FX_REVERB_MAX_FRAMERATE: u32 = 48000u32;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_CUTOFF: u32 = 14u32;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_CUTOFF: u32 = 9u32;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_GAIN: u32 = 12u32;
pub const XAUDIO2FX_REVERB_MAX_POSITION: u32 = 30u32;
pub const XAUDIO2FX_REVERB_MAX_REAR_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_DELAY: u32 = 300u32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_GAIN: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_DELAY: u32 = 85u32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_GAIN: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_FREQ: f32 = 20000f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_HF: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_MAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_SIZE: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_WET_DRY_MIX: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_REAR_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_SIDE_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_DECAY_TIME: f32 = 0.1f32;
pub const XAUDIO2FX_REVERB_MIN_DENSITY: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MIN_DIFFUSION: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_FRAMERATE: u32 = 20000u32;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_CUTOFF: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_GAIN: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_CUTOFF: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_GAIN: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_POSITION: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REAR_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_GAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_GAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_FREQ: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_HF: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_MAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_SIZE: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MIN_WET_DRY_MIX: f32 = 0f32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct XAUDIO2FX_REVERB_PARAMETERS(i32);
#[repr(C)]
pub struct XAUDIO2FX_VOLUMEMETER_LEVELS(i32);
pub const XAUDIO2_1024_QUANTUM: u32 = 32768u32;
pub const XAUDIO2_ANY_PROCESSOR: u32 = 4294967295u32;
#[repr(C)]
pub struct XAUDIO2_BUFFER(i32);
#[repr(C)]
pub struct XAUDIO2_BUFFER_WMA(i32);
pub const XAUDIO2_COMMIT_ALL: u32 = 0u32;
pub const XAUDIO2_COMMIT_NOW: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct XAUDIO2_DEBUG_CONFIGURATION(i32);
pub const XAUDIO2_DEBUG_ENGINE: u32 = 1u32;
pub const XAUDIO2_DEFAULT_CHANNELS: u32 = 0u32;
pub const XAUDIO2_DEFAULT_FILTER_FREQUENCY: f32 = 1f32;
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ: f32 = 1f32;
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2f32;
pub const XAUDIO2_DEFAULT_PROCESSOR: u32 = 1u32;
pub const XAUDIO2_DEFAULT_SAMPLERATE: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct XAUDIO2_EFFECT_CHAIN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct XAUDIO2_EFFECT_DESCRIPTOR(i32);
pub const XAUDIO2_END_OF_STREAM: u32 = 64u32;
pub const XAUDIO2_E_DEVICE_INVALIDATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003435516i32 as _);
pub const XAUDIO2_E_INVALID_CALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003435519i32 as _);
pub const XAUDIO2_E_XAPO_CREATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003435517i32 as _);
pub const XAUDIO2_E_XMA_DECODER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003435518i32 as _);
#[repr(C)]
pub struct XAUDIO2_FILTER_PARAMETERS(i32);
#[repr(transparent)]
pub struct XAUDIO2_FILTER_TYPE(pub i32);
pub const LowPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(0i32);
pub const BandPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(1i32);
pub const HighPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(2i32);
pub const NotchFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(3i32);
pub const LowPassOnePoleFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(4i32);
pub const HighPassOnePoleFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(5i32);
pub const XAUDIO2_LOG_API_CALLS: u32 = 16u32;
pub const XAUDIO2_LOG_DETAIL: u32 = 8u32;
pub const XAUDIO2_LOG_ERRORS: u32 = 1u32;
pub const XAUDIO2_LOG_FUNC_CALLS: u32 = 32u32;
pub const XAUDIO2_LOG_INFO: u32 = 4u32;
pub const XAUDIO2_LOG_LOCKS: u32 = 128u32;
pub const XAUDIO2_LOG_MEMORY: u32 = 256u32;
pub const XAUDIO2_LOG_STREAMING: u32 = 4096u32;
pub const XAUDIO2_LOG_TIMING: u32 = 64u32;
pub const XAUDIO2_LOG_WARNINGS: u32 = 2u32;
pub const XAUDIO2_LOOP_INFINITE: u32 = 255u32;
pub const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64u32;
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: u32 = 2u32;
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 2147483648u32;
pub const XAUDIO2_MAX_FILTER_FREQUENCY: f32 = 1f32;
pub const XAUDIO2_MAX_FILTER_ONEOVERQ: f32 = 1.5f32;
pub const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024f32;
pub const XAUDIO2_MAX_INSTANCES: u32 = 8u32;
pub const XAUDIO2_MAX_LOOP_COUNT: u32 = 254u32;
pub const XAUDIO2_MAX_QUEUED_BUFFERS: u32 = 64u32;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: u32 = 600000u32;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: u32 = 300000u32;
pub const XAUDIO2_MAX_SAMPLE_RATE: u32 = 200000u32;
pub const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16777216f32;
pub const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1000u32;
pub const XAUDIO2_NO_LOOP_REGION: u32 = 0u32;
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: u32 = 65536u32;
#[repr(C)]
pub struct XAUDIO2_PERFORMANCE_DATA(i32);
pub const XAUDIO2_PLAY_TAILS: u32 = 32u32;
pub const XAUDIO2_QUANTUM_DENOMINATOR: u32 = 100u32;
pub const XAUDIO2_QUANTUM_NUMERATOR: u32 = 1u32;
#[repr(C)]
pub struct XAUDIO2_SEND_DESCRIPTOR(i32);
pub const XAUDIO2_SEND_USEFILTER: u32 = 128u32;
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: u32 = 8192u32;
pub const XAUDIO2_USE_DEFAULT_PROCESSOR: u32 = 0u32;
#[repr(C)]
pub struct XAUDIO2_VOICE_DETAILS(i32);
pub const XAUDIO2_VOICE_NOPITCH: u32 = 2u32;
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 256u32;
pub const XAUDIO2_VOICE_NOSRC: u32 = 4u32;
#[repr(C)]
pub struct XAUDIO2_VOICE_SENDS(i32);
#[repr(C)]
pub struct XAUDIO2_VOICE_STATE(i32);
pub const XAUDIO2_VOICE_USEFILTER: u32 = 8u32;
