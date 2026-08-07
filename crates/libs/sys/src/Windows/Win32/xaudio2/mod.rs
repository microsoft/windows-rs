windows_link::link!("xaudio2_9.dll" "system" fn XAudio2CreateWithVersionInfo(ppxaudio2 : *mut *mut core::ffi::c_void, flags : u32, xaudio2processor : XAUDIO2_PROCESSOR, ntddiversion : u32) -> windows_sys::core::HRESULT);
pub const BandPassFilter: XAUDIO2_FILTER_TYPE = 1;
pub const FACILITY_XAUDIO2: i32 = 2198;
pub const HighPassFilter: XAUDIO2_FILTER_TYPE = 2;
pub const HighPassOnePoleFilter: XAUDIO2_FILTER_TYPE = 5;
pub const LowPassFilter: XAUDIO2_FILTER_TYPE = 0;
pub const LowPassOnePoleFilter: XAUDIO2_FILTER_TYPE = 4;
pub const NotchFilter: XAUDIO2_FILTER_TYPE = 3;
pub const Processor1: i32 = 1;
pub const Processor10: i32 = 512;
pub const Processor11: i32 = 1024;
pub const Processor12: i32 = 2048;
pub const Processor13: i32 = 4096;
pub const Processor14: i32 = 8192;
pub const Processor15: i32 = 16384;
pub const Processor16: i32 = 32768;
pub const Processor17: i32 = 65536;
pub const Processor18: i32 = 131072;
pub const Processor19: i32 = 262144;
pub const Processor2: i32 = 2;
pub const Processor20: i32 = 524288;
pub const Processor21: i32 = 1048576;
pub const Processor22: i32 = 2097152;
pub const Processor23: i32 = 4194304;
pub const Processor24: i32 = 8388608;
pub const Processor25: i32 = 16777216;
pub const Processor26: i32 = 33554432;
pub const Processor27: i32 = 67108864;
pub const Processor28: i32 = 134217728;
pub const Processor29: i32 = 268435456;
pub const Processor3: i32 = 4;
pub const Processor30: i32 = 536870912;
pub const Processor31: i32 = 1073741824;
pub const Processor32: u32 = 2147483648;
pub const Processor4: i32 = 8;
pub const Processor5: i32 = 16;
pub const Processor6: i32 = 32;
pub const Processor7: i32 = 64;
pub const Processor8: i32 = 128;
pub const Processor9: i32 = 256;
pub const XAUDIO2D_DLL_A: windows_sys::core::PCSTR = windows_sys::core::s!("xaudio2_9d.dll");
pub const XAUDIO2D_DLL_W: windows_sys::core::PCWSTR = windows_sys::core::w!("xaudio2_9d.dll");
pub const XAUDIO2_1024_QUANTUM: i32 = 32768;
pub const XAUDIO2_ANY_PROCESSOR: u32 = 4294967295;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_BUFFER {
    pub Flags: u32,
    pub AudioBytes: u32,
    pub pAudioData: *const u8,
    pub PlayBegin: u32,
    pub PlayLength: u32,
    pub LoopBegin: u32,
    pub LoopLength: u32,
    pub LoopCount: u32,
    pub pContext: *mut core::ffi::c_void,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_BUFFER_WMA {
    pub pDecodedPacketCumulativeBytes: *const u32,
    pub PacketCount: u32,
}
pub const XAUDIO2_COMMIT_ALL: i32 = 0;
pub const XAUDIO2_COMMIT_NOW: i32 = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_DEBUG_CONFIGURATION {
    pub TraceMask: u32,
    pub BreakMask: u32,
    pub LogThreadID: windows_sys::core::BOOL,
    pub LogFileline: windows_sys::core::BOOL,
    pub LogFunctionName: windows_sys::core::BOOL,
    pub LogTiming: windows_sys::core::BOOL,
}
pub const XAUDIO2_DEBUG_ENGINE: i32 = 1;
pub const XAUDIO2_DEFAULT_CHANNELS: i32 = 0;
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ: f32 = 1.0;
pub const XAUDIO2_DEFAULT_FILTER_TYPE: i32 = 0;
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2.0;
pub const XAUDIO2_DEFAULT_PROCESSOR: i32 = 1;
pub const XAUDIO2_DEFAULT_SAMPLERATE: i32 = 0;
pub const XAUDIO2_DLL_A: windows_sys::core::PCSTR = windows_sys::core::s!("xaudio2_9.dll");
pub const XAUDIO2_DLL_W: windows_sys::core::PCWSTR = windows_sys::core::w!("xaudio2_9.dll");
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_EFFECT_CHAIN {
    pub EffectCount: u32,
    pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_EFFECT_DESCRIPTOR {
    pub pEffect: *mut core::ffi::c_void,
    pub InitialState: windows_sys::core::BOOL,
    pub OutputChannels: u32,
}
pub const XAUDIO2_END_OF_STREAM: i32 = 64;
pub const XAUDIO2_E_DEVICE_INVALIDATED: windows_sys::core::HRESULT = 0x88960004_u32 as _;
pub const XAUDIO2_E_INVALID_CALL: windows_sys::core::HRESULT = 0x88960001_u32 as _;
pub const XAUDIO2_E_XAPO_CREATION_FAILED: windows_sys::core::HRESULT = 0x88960003_u32 as _;
pub const XAUDIO2_E_XMA_DECODER_ERROR: windows_sys::core::HRESULT = 0x88960002_u32 as _;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_FILTER_PARAMETERS {
    pub Type: XAUDIO2_FILTER_TYPE,
    pub Frequency: f32,
    pub OneOverQ: f32,
}
pub type XAUDIO2_FILTER_TYPE = i32;
pub const XAUDIO2_INVALID_OPSET: u32 = 4294967295;
pub const XAUDIO2_LOG_API_CALLS: i32 = 16;
pub const XAUDIO2_LOG_DETAIL: i32 = 8;
pub const XAUDIO2_LOG_ERRORS: i32 = 1;
pub const XAUDIO2_LOG_FUNC_CALLS: i32 = 32;
pub const XAUDIO2_LOG_INFO: i32 = 4;
pub const XAUDIO2_LOG_LOCKS: i32 = 128;
pub const XAUDIO2_LOG_MEMORY: i32 = 256;
pub const XAUDIO2_LOG_STREAMING: i32 = 4096;
pub const XAUDIO2_LOG_TIMING: i32 = 64;
pub const XAUDIO2_LOG_WARNINGS: i32 = 2;
pub const XAUDIO2_LOOP_INFINITE: i32 = 255;
pub const XAUDIO2_MAX_AUDIO_CHANNELS: i32 = 64;
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: i32 = 2;
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 2147483648;
pub const XAUDIO2_MAX_FILTER_FREQUENCY: f32 = 1.0;
pub const XAUDIO2_MAX_FILTER_ONEOVERQ: f32 = 1.5;
pub const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024.0;
pub const XAUDIO2_MAX_INSTANCES: i32 = 8;
pub const XAUDIO2_MAX_LOOP_COUNT: i32 = 254;
pub const XAUDIO2_MAX_QUEUED_BUFFERS: i32 = 64;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: i32 = 600000;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: i32 = 300000;
pub const XAUDIO2_MAX_SAMPLE_RATE: i32 = 384000;
pub const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16777216.0;
pub const XAUDIO2_MIN_SAMPLE_RATE: i32 = 1000;
pub const XAUDIO2_NO_LOOP_REGION: i32 = 0;
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: i32 = 65536;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_PERFORMANCE_DATA {
    pub AudioCyclesSinceLastQuery: u64,
    pub TotalCyclesSinceLastQuery: u64,
    pub MinimumCyclesPerQuantum: u32,
    pub MaximumCyclesPerQuantum: u32,
    pub MemoryUsageInBytes: u32,
    pub CurrentLatencyInSamples: u32,
    pub GlitchesSinceEngineStarted: u32,
    pub ActiveSourceVoiceCount: u32,
    pub TotalSourceVoiceCount: u32,
    pub ActiveSubmixVoiceCount: u32,
    pub ActiveResamplerCount: u32,
    pub ActiveMatrixMixCount: u32,
    pub ActiveXmaSourceVoices: u32,
    pub ActiveXmaStreams: u32,
}
pub const XAUDIO2_PLAY_TAILS: i32 = 32;
pub type XAUDIO2_PROCESSOR = u32;
pub const XAUDIO2_QUANTUM_DENOMINATOR: i32 = 100;
pub const XAUDIO2_QUANTUM_NUMERATOR: i32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags: u32,
    pub pOutputVoice: *mut core::ffi::c_void,
}
pub const XAUDIO2_SEND_USEFILTER: i32 = 128;
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: i32 = 8192;
pub const XAUDIO2_USE_DEFAULT_PROCESSOR: i32 = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_VOICE_DETAILS {
    pub CreationFlags: u32,
    pub ActiveFlags: u32,
    pub InputChannels: u32,
    pub InputSampleRate: u32,
}
pub const XAUDIO2_VOICE_NOPITCH: i32 = 2;
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: i32 = 256;
pub const XAUDIO2_VOICE_NOSRC: i32 = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount: u32,
    pub pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_VOICE_STATE {
    pub pCurrentBufferContext: *mut core::ffi::c_void,
    pub BuffersQueued: u32,
    pub SamplesPlayed: u64,
}
pub const XAUDIO2_VOICE_USEFILTER: i32 = 8;
