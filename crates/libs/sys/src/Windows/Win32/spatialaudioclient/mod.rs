pub type AudioObjectType = u32;
pub const AudioObjectType_BackCenter: AudioObjectType = 131072;
pub const AudioObjectType_BackLeft: AudioObjectType = 128;
pub const AudioObjectType_BackRight: AudioObjectType = 256;
pub const AudioObjectType_BottomBackLeft: AudioObjectType = 32768;
pub const AudioObjectType_BottomBackRight: AudioObjectType = 65536;
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = 8192;
pub const AudioObjectType_BottomFrontRight: AudioObjectType = 16384;
pub const AudioObjectType_Dynamic: AudioObjectType = 1;
pub const AudioObjectType_FrontCenter: AudioObjectType = 8;
pub const AudioObjectType_FrontLeft: AudioObjectType = 2;
pub const AudioObjectType_FrontRight: AudioObjectType = 4;
pub const AudioObjectType_LowFrequency: AudioObjectType = 16;
pub const AudioObjectType_None: AudioObjectType = 0;
pub const AudioObjectType_SideLeft: AudioObjectType = 32;
pub const AudioObjectType_SideRight: AudioObjectType = 64;
pub const AudioObjectType_StereoLeft: AudioObjectType = 262144;
pub const AudioObjectType_StereoRight: AudioObjectType = 524288;
pub const AudioObjectType_TopBackLeft: AudioObjectType = 2048;
pub const AudioObjectType_TopBackRight: AudioObjectType = 4096;
pub const AudioObjectType_TopFrontLeft: AudioObjectType = 512;
pub const AudioObjectType_TopFrontRight: AudioObjectType = 1024;
pub type SPATIAL_AUDIO_STREAM_OPTIONS = u32;
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = 0;
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = 1;
pub const SPTLAUDCLNT_E_DESTROYED: windows_sys::core::HRESULT = 0x88890100_u32 as _;
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: windows_sys::core::HRESULT = 0x88890105_u32 as _;
pub const SPTLAUDCLNT_E_INTERNAL: windows_sys::core::HRESULT = 0x8889010D_u32 as _;
pub const SPTLAUDCLNT_E_INVALID_LICENSE: windows_sys::core::HRESULT = 0x88890108_u32 as _;
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: windows_sys::core::HRESULT = 0x88890106_u32 as _;
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: windows_sys::core::HRESULT = 0x88890103_u32 as _;
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: windows_sys::core::HRESULT = 0x8889010C_u32 as _;
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: windows_sys::core::HRESULT = 0x88890101_u32 as _;
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: windows_sys::core::HRESULT = 0x88890104_u32 as _;
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: windows_sys::core::HRESULT = 0x88890102_u32 as _;
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: windows_sys::core::HRESULT = 0x8889010B_u32 as _;
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: windows_sys::core::HRESULT = 0x88890107_u32 as _;
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: windows_sys::core::HRESULT = 0x8889010A_u32 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: windows_sys::core::GUID,
    pub appId: windows_sys::core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "audiosessiontypes", feature = "mmeapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *const super::WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::HANDLE,
    pub NotifyObject: *mut core::ffi::c_void,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "audiosessiontypes", feature = "mmeapi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *const super::WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::HANDLE,
    pub NotifyObject: *mut core::ffi::c_void,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
