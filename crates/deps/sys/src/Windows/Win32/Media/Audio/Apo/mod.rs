#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for APOInitBaseStruct {}
impl ::core::clone::Clone for APOInitBaseStruct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pAPOSystemEffectsProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: super::IMMDeviceCollection,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for APOInitSystemEffects {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APOInitSystemEffects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pAPOSystemEffectsProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: super::IMMDeviceCollection,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_sys::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub pServiceProvider: super::super::super::System::Com::IServiceProvider,
    pub pDeviceCollection: super::IMMDeviceCollection,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows_sys::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APOInitSystemEffects3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APO_BUFFER_FLAGS(pub i32);
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(0i32);
pub const BUFFER_VALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(1i32);
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(2i32);
impl ::core::marker::Copy for APO_BUFFER_FLAGS {}
impl ::core::clone::Clone for APO_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APO_CONNECTION_BUFFER_TYPE(pub i32);
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(0i32);
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(1i32);
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(2i32);
impl ::core::marker::Copy for APO_CONNECTION_BUFFER_TYPE {}
impl ::core::clone::Clone for APO_CONNECTION_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: IAudioMediaType,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_DESCRIPTOR {}
impl ::core::clone::Clone for APO_CONNECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APO_CONNECTION_PROPERTY {
    pub pBuffer: usize,
    pub u32ValidFrameCount: u32,
    pub u32BufferFlags: APO_BUFFER_FLAGS,
    pub u32Signature: u32,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APO_CONNECTION_PROPERTY_V2 {
    pub property: APO_CONNECTION_PROPERTY,
    pub u64QPCTime: u64,
}
impl ::core::marker::Copy for APO_CONNECTION_PROPERTY_V2 {}
impl ::core::clone::Clone for APO_CONNECTION_PROPERTY_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APO_FLAG(pub i32);
pub const APO_FLAG_NONE: APO_FLAG = APO_FLAG(0i32);
pub const APO_FLAG_INPLACE: APO_FLAG = APO_FLAG(1i32);
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = APO_FLAG(2i32);
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = APO_FLAG(4i32);
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = APO_FLAG(8i32);
pub const APO_FLAG_MIXER: APO_FLAG = APO_FLAG(16i32);
pub const APO_FLAG_DEFAULT: APO_FLAG = APO_FLAG(14i32);
impl ::core::marker::Copy for APO_FLAG {}
impl ::core::clone::Clone for APO_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APO_LOG_LEVEL(pub i32);
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = APO_LOG_LEVEL(0i32);
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = APO_LOG_LEVEL(1i32);
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = APO_LOG_LEVEL(2i32);
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = APO_LOG_LEVEL(3i32);
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = APO_LOG_LEVEL(4i32);
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = APO_LOG_LEVEL(5i32);
impl ::core::marker::Copy for APO_LOG_LEVEL {}
impl ::core::clone::Clone for APO_LOG_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APO_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION,
    pub audioEndpointPropertyChange: AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION,
    pub audioSystemEffectsPropertyChange: AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for APO_NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl ::core::marker::Copy for APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR,
    pub audioEndpointPropertyChange: AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR,
    pub audioSystemEffectsPropertyChange: AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR,
}
impl ::core::marker::Copy for APO_NOTIFICATION_DESCRIPTOR_0 {}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APO_NOTIFICATION_TYPE(pub i32);
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(0i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(1i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(2i32);
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for APO_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for APO_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct APO_REG_PROPERTIES {
    pub clsid: ::windows_sys::core::GUID,
    pub Flags: APO_FLAG,
    pub szFriendlyName: [u16; 256],
    pub szCopyrightInfo: [u16; 256],
    pub u32MajorVersion: u32,
    pub u32MinorVersion: u32,
    pub u32MinInputConnections: u32,
    pub u32MaxInputConnections: u32,
    pub u32MinOutputConnections: u32,
    pub u32MaxOutputConnections: u32,
    pub u32MaxInstances: u32,
    pub u32NumAPOInterfaces: u32,
    pub iidAPOInterfaceList: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for APO_REG_PROPERTIES {}
impl ::core::clone::Clone for APO_REG_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[repr(C)]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub propertyStore: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AUDIO_FLOW_TYPE(pub i32);
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(0i32);
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(1i32);
impl ::core::marker::Copy for AUDIO_FLOW_TYPE {}
impl ::core::clone::Clone for AUDIO_FLOW_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: ::windows_sys::core::GUID,
    pub canSetState: super::super::super::Foundation::BOOL,
    pub state: AUDIO_SYSTEMEFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: super::IMMDevice,
    pub propertyStoreContext: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: super::IMMDevice,
    pub propertyStoreContext: ::windows_sys::core::GUID,
    pub propertyStoreType: super::__MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002,
    pub propertyStore: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AUDIO_SYSTEMEFFECT_STATE(pub i32);
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(0i32);
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(1i32);
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT_STATE {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct AudioFXExtensionParams {
    pub AddPageParam: super::super::super::Foundation::LPARAM,
    pub pwstrEndpointID: super::super::super::Foundation::PWSTR,
    pub pFxProperties: super::super::super::UI::Shell::PropertiesSystem::IPropertyStore,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::marker::Copy for AudioFXExtensionParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for AudioFXExtensionParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EAudioConstriction(pub i32);
pub const eAudioConstrictionOff: EAudioConstriction = EAudioConstriction(0i32);
pub const eAudioConstriction48_16: EAudioConstriction = EAudioConstriction(1i32);
pub const eAudioConstriction44_16: EAudioConstriction = EAudioConstriction(2i32);
pub const eAudioConstriction14_14: EAudioConstriction = EAudioConstriction(3i32);
pub const eAudioConstrictionMute: EAudioConstriction = EAudioConstriction(4i32);
impl ::core::marker::Copy for EAudioConstriction {}
impl ::core::clone::Clone for EAudioConstriction {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FNAPONOTIFICATIONCALLBACK = unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[repr(transparent)]
pub struct IApoAcousticEchoCancellation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApoAcousticEchoCancellation {}
impl ::core::clone::Clone for IApoAcousticEchoCancellation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApoAuxiliaryInputConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApoAuxiliaryInputConfiguration {}
impl ::core::clone::Clone for IApoAuxiliaryInputConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApoAuxiliaryInputRT(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApoAuxiliaryInputRT {}
impl ::core::clone::Clone for IApoAuxiliaryInputRT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioDeviceModulesClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioDeviceModulesClient {}
impl ::core::clone::Clone for IAudioDeviceModulesClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioMediaType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioMediaType {}
impl ::core::clone::Clone for IAudioMediaType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioProcessingObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioProcessingObject {}
impl ::core::clone::Clone for IAudioProcessingObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioProcessingObjectConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioProcessingObjectConfiguration {}
impl ::core::clone::Clone for IAudioProcessingObjectConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioProcessingObjectLoggingService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioProcessingObjectLoggingService {}
impl ::core::clone::Clone for IAudioProcessingObjectLoggingService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioProcessingObjectNotifications(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioProcessingObjectNotifications {}
impl ::core::clone::Clone for IAudioProcessingObjectNotifications {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioProcessingObjectRT(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioProcessingObjectRT {}
impl ::core::clone::Clone for IAudioProcessingObjectRT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioProcessingObjectRTQueueService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioProcessingObjectRTQueueService {}
impl ::core::clone::Clone for IAudioProcessingObjectRTQueueService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioProcessingObjectVBR(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioProcessingObjectVBR {}
impl ::core::clone::Clone for IAudioProcessingObjectVBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioSystemEffects(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioSystemEffects {}
impl ::core::clone::Clone for IAudioSystemEffects {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioSystemEffects2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioSystemEffects2 {}
impl ::core::clone::Clone for IAudioSystemEffects2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioSystemEffects3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioSystemEffects3 {}
impl ::core::clone::Clone for IAudioSystemEffects3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioSystemEffectsCustomFormats(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioSystemEffectsCustomFormats {}
impl ::core::clone::Clone for IAudioSystemEffectsCustomFormats {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_APO_SWFallback_ProcessingModes: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Association: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_FriendlyName: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PostMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PreMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_UserInterfaceClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3494774182, data2: 22859, data3: 20406, data4: [168, 13, 1, 175, 94, 237, 125, 29] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 3550034495, data2: 39362, data3: 17410, data4: [181, 236, 169, 42, 3, 103, 102, 75] },
    pid: 5u32,
};
pub const SID_AudioProcessingObjectLoggingService: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2340423855,
    data2: 2553,
    data3: 17774,
    data4: [161, 115, 189, 181, 132, 153, 188, 231],
};
pub const SID_AudioProcessingObjectRTQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1166809631, data2: 26777, data3: 19474, data4: [153, 172, 226, 230, 172, 37, 49, 4] };
#[repr(C)]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows_sys::core::GUID,
    pub dwSamplesPerFrame: u32,
    pub dwBytesPerSampleContainer: u32,
    pub dwValidBitsPerSample: u32,
    pub fFramesPerSecond: f32,
    pub dwChannelMask: u32,
}
impl ::core::marker::Copy for UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::clone::Clone for UNCOMPRESSEDAUDIOFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
