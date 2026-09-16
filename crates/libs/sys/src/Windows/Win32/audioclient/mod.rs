pub type AMBISONICS_CHANNEL_ORDERING = i32;
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = 0;
pub type AMBISONICS_NORMALIZATION = i32;
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = 1;
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AMBISONICS_PARAMS {
    pub u32Size: u32,
    pub u32Version: u32,
    pub u32Type: AMBISONICS_TYPE,
    pub u32ChannelOrdering: AMBISONICS_CHANNEL_ORDERING,
    pub u32Normalization: AMBISONICS_NORMALIZATION,
    pub u32Order: u32,
    pub u32NumChannels: u32,
    pub pu32ChannelMap: *mut u32,
}
pub const AMBISONICS_PARAM_VERSION_1: i32 = 1;
pub type AMBISONICS_TYPE = i32;
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = 0;
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = 1;
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = 2;
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = 4;
pub const AUDCLNT_E_ALREADY_INITIALIZED: windows_sys::core::HRESULT = 0x88890002_u32 as _;
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: windows_sys::core::HRESULT = 0x88890013_u32 as _;
pub const AUDCLNT_E_BUFFER_ERROR: windows_sys::core::HRESULT = 0x88890018_u32 as _;
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: windows_sys::core::HRESULT = 0x8889000B_u32 as _;
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: windows_sys::core::HRESULT = 0x88890016_u32 as _;
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: windows_sys::core::HRESULT = 0x88890019_u32 as _;
pub const AUDCLNT_E_BUFFER_TOO_LARGE: windows_sys::core::HRESULT = 0x88890006_u32 as _;
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: windows_sys::core::HRESULT = 0x88890017_u32 as _;
pub const AUDCLNT_E_DEVICE_INVALIDATED: windows_sys::core::HRESULT = 0x88890004_u32 as _;
pub const AUDCLNT_E_DEVICE_IN_USE: windows_sys::core::HRESULT = 0x8889000A_u32 as _;
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: windows_sys::core::HRESULT = 0x88890041_u32 as _;
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: windows_sys::core::HRESULT = 0x88890042_u32 as _;
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: windows_sys::core::HRESULT = 0x8889000F_u32 as _;
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: windows_sys::core::HRESULT = 0x88890022_u32 as _;
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: windows_sys::core::HRESULT = 0x88890029_u32 as _;
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: windows_sys::core::HRESULT = 0x88890028_u32 as _;
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: windows_sys::core::HRESULT = 0x88890011_u32 as _;
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: windows_sys::core::HRESULT = 0x88890014_u32 as _;
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: windows_sys::core::HRESULT = 0x8889000E_u32 as _;
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: windows_sys::core::HRESULT = 0x88890012_u32 as _;
pub const AUDCLNT_E_HEADTRACKING_ENABLED: windows_sys::core::HRESULT = 0x88890030_u32 as _;
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: windows_sys::core::HRESULT = 0x88890040_u32 as _;
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: windows_sys::core::HRESULT = 0x88890015_u32 as _;
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: windows_sys::core::HRESULT = 0x88890020_u32 as _;
pub const AUDCLNT_E_INVALID_SIZE: windows_sys::core::HRESULT = 0x88890009_u32 as _;
pub const AUDCLNT_E_INVALID_STREAM_FLAG: windows_sys::core::HRESULT = 0x88890021_u32 as _;
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: windows_sys::core::HRESULT = 0x88890025_u32 as _;
pub const AUDCLNT_E_NOT_INITIALIZED: windows_sys::core::HRESULT = 0x88890001_u32 as _;
pub const AUDCLNT_E_NOT_STOPPED: windows_sys::core::HRESULT = 0x88890005_u32 as _;
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: windows_sys::core::HRESULT = 0x88890024_u32 as _;
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: windows_sys::core::HRESULT = 0x88890023_u32 as _;
pub const AUDCLNT_E_OUT_OF_ORDER: windows_sys::core::HRESULT = 0x88890007_u32 as _;
pub const AUDCLNT_E_POST_VOLUME_LOOPBACK_UNSUPPORTED: windows_sys::core::HRESULT = 0x88890043_u32 as _;
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: windows_sys::core::HRESULT = 0x88890027_u32 as _;
pub const AUDCLNT_E_RESOURCES_INVALIDATED: windows_sys::core::HRESULT = 0x88890026_u32 as _;
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: windows_sys::core::HRESULT = 0x88890010_u32 as _;
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: windows_sys::core::HRESULT = 0x8889000C_u32 as _;
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: windows_sys::core::HRESULT = 0x88890008_u32 as _;
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: windows_sys::core::HRESULT = 0x88890003_u32 as _;
pub type AUDCLNT_STREAMOPTIONS = u32;
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = 4;
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = 2;
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = 0;
pub const AUDCLNT_STREAMOPTIONS_POST_VOLUME_LOOPBACK: AUDCLNT_STREAMOPTIONS = 8;
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = 1;
#[cfg(feature = "wtypesbase")]
pub const AUDCLNT_S_BUFFER_EMPTY: super::SCODE = 0x8890001_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AUDCLNT_S_POSITION_STALLED: super::SCODE = 0x8890003_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: super::SCODE = 0x8890002_u32 as _;
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: i32 = 1;
pub type AUDIO_DUCKING_OPTIONS = u32;
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = 0;
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT {
    pub id: windows_sys::core::GUID,
    pub canSetState: windows_sys::core::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
pub type AUDIO_EFFECT_STATE = i32;
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = 0;
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "audiosessiontypes")]
#[derive(Clone, Copy, Default)]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: windows_sys::core::BOOL,
    pub eCategory: super::AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
pub type _AUDCLNT_BUFFERFLAGS = i32;
