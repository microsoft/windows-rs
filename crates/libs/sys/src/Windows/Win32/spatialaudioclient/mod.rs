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
pub const SPTLAUDCLNT_E_DESTROYED: i32 = -2004287232;
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: i32 = -2004287227;
pub const SPTLAUDCLNT_E_INTERNAL: i32 = -2004287219;
pub const SPTLAUDCLNT_E_INVALID_LICENSE: i32 = -2004287224;
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: i32 = -2004287226;
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: i32 = -2004287229;
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: i32 = -2004287220;
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: i32 = -2004287231;
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: i32 = -2004287228;
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: i32 = -2004287230;
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: i32 = -2004287221;
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: i32 = -2004287225;
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: i32 = -2004287222;
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
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *const super::mmeapi::WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::winnt::HANDLE,
    pub NotifyObject: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi", feature = "Win32_winnt"))]
impl Default for SpatialAudioObjectRenderStreamActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *const super::mmeapi::WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::winnt::HANDLE,
    pub NotifyObject: *mut core::ffi::c_void,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi", feature = "Win32_winnt"))]
impl Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
