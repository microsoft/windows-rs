#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const APOERR_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073919i32 as _);
pub const APOERR_ALREADY_UNLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073914i32 as _);
pub const APOERR_APO_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073910i32 as _);
pub const APOERR_BUFFERS_OVERLAP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073915i32 as _);
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073917i32 as _);
pub const APOERR_INVALID_APO_CLSID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073916i32 as _);
pub const APOERR_INVALID_COEFFCOUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073909i32 as _);
pub const APOERR_INVALID_COEFFICIENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073908i32 as _);
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073911i32 as _);
pub const APOERR_INVALID_CURVE_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073907i32 as _);
pub const APOERR_INVALID_INPUTID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073906i32 as _);
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073912i32 as _);
pub const APOERR_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073918i32 as _);
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2005073913i32 as _);
#[repr(C)]
pub struct APOInitBaseStruct(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct APOInitSystemEffects(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[repr(C)]
pub struct APOInitSystemEffects2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[repr(C)]
pub struct APOInitSystemEffects3(i32);
#[repr(transparent)]
pub struct APO_BUFFER_FLAGS(pub i32);
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(0i32);
pub const BUFFER_VALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(1i32);
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(2i32);
#[repr(transparent)]
pub struct APO_CONNECTION_BUFFER_TYPE(pub i32);
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(0i32);
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(1i32);
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(2i32);
#[repr(C)]
pub struct APO_CONNECTION_DESCRIPTOR(i32);
#[repr(C)]
pub struct APO_CONNECTION_PROPERTY(i32);
#[repr(C)]
pub struct APO_CONNECTION_PROPERTY_V2(i32);
#[repr(transparent)]
pub struct APO_FLAG(pub i32);
pub const APO_FLAG_NONE: APO_FLAG = APO_FLAG(0i32);
pub const APO_FLAG_INPLACE: APO_FLAG = APO_FLAG(1i32);
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = APO_FLAG(2i32);
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = APO_FLAG(4i32);
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = APO_FLAG(8i32);
pub const APO_FLAG_MIXER: APO_FLAG = APO_FLAG(16i32);
pub const APO_FLAG_DEFAULT: APO_FLAG = APO_FLAG(14i32);
#[repr(transparent)]
pub struct APO_LOG_LEVEL(pub i32);
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = APO_LOG_LEVEL(0i32);
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = APO_LOG_LEVEL(1i32);
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = APO_LOG_LEVEL(2i32);
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = APO_LOG_LEVEL(3i32);
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = APO_LOG_LEVEL(4i32);
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = APO_LOG_LEVEL(5i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[repr(C)]
pub struct APO_NOTIFICATION(i32);
#[repr(C)]
pub struct APO_NOTIFICATION_DESCRIPTOR(i32);
#[repr(transparent)]
pub struct APO_NOTIFICATION_TYPE(pub i32);
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(0i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(1i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(2i32);
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(3i32);
#[repr(C)]
pub struct APO_REG_PROPERTIES(i32);
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[repr(C)]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION(i32);
#[repr(C)]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION(i32);
#[repr(transparent)]
pub struct AUDIO_FLOW_TYPE(pub i32);
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(0i32);
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(1i32);
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AUDIO_SYSTEMEFFECT(i32);
#[repr(C)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION(i32);
#[repr(transparent)]
pub struct AUDIO_SYSTEMEFFECT_STATE(pub i32);
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(0i32);
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(1i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[repr(C)]
pub struct AudioFXExtensionParams(i32);
#[repr(transparent)]
pub struct EAudioConstriction(pub i32);
pub const eAudioConstrictionOff: EAudioConstriction = EAudioConstriction(0i32);
pub const eAudioConstriction48_16: EAudioConstriction = EAudioConstriction(1i32);
pub const eAudioConstriction44_16: EAudioConstriction = EAudioConstriction(2i32);
pub const eAudioConstriction14_14: EAudioConstriction = EAudioConstriction(3i32);
pub const eAudioConstrictionMute: EAudioConstriction = EAudioConstriction(4i32);
#[repr(C)]
pub struct FNAPONOTIFICATIONCALLBACK(i32);
#[repr(transparent)]
pub struct IApoAcousticEchoCancellation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApoAuxiliaryInputConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApoAuxiliaryInputRT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioDeviceModulesClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioMediaType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioProcessingObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioProcessingObjectConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioProcessingObjectLoggingService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioProcessingObjectNotifications(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioProcessingObjectRT(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioProcessingObjectRTQueueService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioProcessingObjectVBR(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSystemEffects(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSystemEffects2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSystemEffects3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioSystemEffectsCustomFormats(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_APO_SWFallback_ProcessingModes: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Association: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_FriendlyName: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PostMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PreMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_UserInterfaceClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 5u32,
};
pub const SID_AudioProcessingObjectLoggingService: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2340423855,
    data2: 2553,
    data3: 17774,
    data4: [161, 115, 189, 181, 132, 153, 188, 231],
};
pub const SID_AudioProcessingObjectRTQueue: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1166809631, data2: 26777, data3: 19474, data4: [153, 172, 226, 230, 172, 37, 49, 4] };
#[repr(C)]
pub struct UNCOMPRESSEDAUDIOFORMAT(i32);
