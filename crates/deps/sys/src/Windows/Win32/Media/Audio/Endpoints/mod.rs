#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    pub u32Size: u32,
    pub u32TSSessionId: u32,
    pub targetEndpointConnectorType: EndpointConnectorType,
    pub wfxDeviceFormat: super::WAVEFORMATEX,
}
impl ::core::marker::Copy for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {}
impl ::core::clone::Clone for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2670689126,
    data2: 26028,
    data3: 20390,
    data4: [138, 228, 18, 60, 120, 184, 147, 19],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 3u32,
};
#[repr(transparent)]
pub struct EndpointConnectorType(pub i32);
pub const eHostProcessConnector: EndpointConnectorType = EndpointConnectorType(0i32);
pub const eOffloadConnector: EndpointConnectorType = EndpointConnectorType(1i32);
pub const eLoopbackConnector: EndpointConnectorType = EndpointConnectorType(2i32);
pub const eKeywordDetectorConnector: EndpointConnectorType = EndpointConnectorType(3i32);
pub const eConnectorCount: EndpointConnectorType = EndpointConnectorType(4i32);
impl ::core::marker::Copy for EndpointConnectorType {}
impl ::core::clone::Clone for EndpointConnectorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointFormatControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEndpointLastBufferControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamMeter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamMute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEndpointVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEndpointVolumeCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioEndpointVolumeEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioLfxControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioMeterInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHardwareAudioEngineBase(pub *mut ::core::ffi::c_void);
