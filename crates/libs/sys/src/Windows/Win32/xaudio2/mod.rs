windows_link::link!("xaudio2_9.dll" "system" fn XAudio2CreateWithVersionInfo(ppxaudio2 : *mut *mut core::ffi::c_void, flags : u32, xaudio2processor : XAUDIO2_PROCESSOR, ntddiversion : u32) -> windows_sys::core::HRESULT);
pub const BandPassFilter: XAUDIO2_FILTER_TYPE = 1;
pub const FACILITY_XAUDIO2: u32 = 2198;
pub const HighPassFilter: XAUDIO2_FILTER_TYPE = 2;
pub const HighPassOnePoleFilter: XAUDIO2_FILTER_TYPE = 5;
pub const LowPassFilter: XAUDIO2_FILTER_TYPE = 0;
pub const LowPassOnePoleFilter: XAUDIO2_FILTER_TYPE = 4;
pub const NotchFilter: XAUDIO2_FILTER_TYPE = 3;
pub const Processor1: u32 = 1;
pub const Processor10: u32 = 512;
pub const Processor11: u32 = 1024;
pub const Processor12: u32 = 2048;
pub const Processor13: u32 = 4096;
pub const Processor14: u32 = 8192;
pub const Processor15: u32 = 16384;
pub const Processor16: u32 = 32768;
pub const Processor17: u32 = 65536;
pub const Processor18: u32 = 131072;
pub const Processor19: u32 = 262144;
pub const Processor2: u32 = 2;
pub const Processor20: u32 = 524288;
pub const Processor21: u32 = 1048576;
pub const Processor22: u32 = 2097152;
pub const Processor23: u32 = 4194304;
pub const Processor24: u32 = 8388608;
pub const Processor25: u32 = 16777216;
pub const Processor26: u32 = 33554432;
pub const Processor27: u32 = 67108864;
pub const Processor28: u32 = 134217728;
pub const Processor29: u32 = 268435456;
pub const Processor3: u32 = 4;
pub const Processor30: u32 = 536870912;
pub const Processor31: u32 = 1073741824;
pub const Processor32: u32 = 2147483648;
pub const Processor4: u32 = 8;
pub const Processor5: u32 = 16;
pub const Processor6: u32 = 32;
pub const Processor7: u32 = 64;
pub const Processor8: u32 = 128;
pub const Processor9: u32 = 256;
pub const XAUDIO2D_DLL_A: windows_sys::core::PCSTR = windows_sys::core::s!("xaudio2_9d.dll");
pub const XAUDIO2D_DLL_W: windows_sys::core::PCWSTR = windows_sys::core::w!("xaudio2_9d.dll");
pub const XAUDIO2_1024_QUANTUM: u32 = 32768;
pub const XAUDIO2_ANY_PROCESSOR: u32 = 4294967295;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
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
impl Default for XAUDIO2_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_BUFFER_WMA {
    pub pDecodedPacketCumulativeBytes: *const u32,
    pub PacketCount: u32,
}
impl Default for XAUDIO2_BUFFER_WMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAUDIO2_COMMIT_ALL: u32 = 0;
pub const XAUDIO2_COMMIT_NOW: u32 = 0;
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
pub const XAUDIO2_DEBUG_ENGINE: u32 = 1;
pub const XAUDIO2_DEFAULT_CHANNELS: u32 = 0;
pub const XAUDIO2_DEFAULT_FILTER_TYPE: u32 = 0;
pub const XAUDIO2_DEFAULT_PROCESSOR: u32 = 1;
pub const XAUDIO2_DEFAULT_SAMPLERATE: u32 = 0;
pub const XAUDIO2_DLL_A: windows_sys::core::PCSTR = windows_sys::core::s!("xaudio2_9.dll");
pub const XAUDIO2_DLL_W: windows_sys::core::PCWSTR = windows_sys::core::w!("xaudio2_9.dll");
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_EFFECT_CHAIN {
    pub EffectCount: u32,
    pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}
impl Default for XAUDIO2_EFFECT_CHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_EFFECT_DESCRIPTOR {
    pub pEffect: *mut core::ffi::c_void,
    pub InitialState: windows_sys::core::BOOL,
    pub OutputChannels: u32,
}
impl Default for XAUDIO2_EFFECT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAUDIO2_END_OF_STREAM: u32 = 64;
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
pub const XAUDIO2_INVALID_OPSET: i32 = -1;
pub const XAUDIO2_LOG_API_CALLS: u32 = 16;
pub const XAUDIO2_LOG_DETAIL: u32 = 8;
pub const XAUDIO2_LOG_ERRORS: u32 = 1;
pub const XAUDIO2_LOG_FUNC_CALLS: u32 = 32;
pub const XAUDIO2_LOG_INFO: u32 = 4;
pub const XAUDIO2_LOG_LOCKS: u32 = 128;
pub const XAUDIO2_LOG_MEMORY: u32 = 256;
pub const XAUDIO2_LOG_STREAMING: u32 = 4096;
pub const XAUDIO2_LOG_TIMING: u32 = 64;
pub const XAUDIO2_LOG_WARNINGS: u32 = 2;
pub const XAUDIO2_LOOP_INFINITE: u32 = 255;
pub const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64;
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: u32 = 2;
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 2147483648;
pub const XAUDIO2_MAX_INSTANCES: u32 = 8;
pub const XAUDIO2_MAX_LOOP_COUNT: u32 = 254;
pub const XAUDIO2_MAX_QUEUED_BUFFERS: u32 = 64;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: u32 = 600000;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: u32 = 300000;
pub const XAUDIO2_MAX_SAMPLE_RATE: u32 = 384000;
pub const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1000;
pub const XAUDIO2_NO_LOOP_REGION: u32 = 0;
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: u32 = 65536;
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
pub const XAUDIO2_PLAY_TAILS: u32 = 32;
pub type XAUDIO2_PROCESSOR = u32;
pub const XAUDIO2_QUANTUM_DENOMINATOR: u32 = 100;
pub const XAUDIO2_QUANTUM_NUMERATOR: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags: u32,
    pub pOutputVoice: *mut core::ffi::c_void,
}
impl Default for XAUDIO2_SEND_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAUDIO2_SEND_USEFILTER: u32 = 128;
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: u32 = 8192;
pub const XAUDIO2_USE_DEFAULT_PROCESSOR: u32 = 0;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_VOICE_DETAILS {
    pub CreationFlags: u32,
    pub ActiveFlags: u32,
    pub InputChannels: u32,
    pub InputSampleRate: u32,
}
pub const XAUDIO2_VOICE_NOPITCH: u32 = 2;
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 256;
pub const XAUDIO2_VOICE_NOSRC: u32 = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount: u32,
    pub pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}
impl Default for XAUDIO2_VOICE_SENDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_VOICE_STATE {
    pub pCurrentBufferContext: *mut core::ffi::c_void,
    pub BuffersQueued: u32,
    pub SamplesPlayed: u64,
}
impl Default for XAUDIO2_VOICE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAUDIO2_VOICE_USEFILTER: u32 = 8;
