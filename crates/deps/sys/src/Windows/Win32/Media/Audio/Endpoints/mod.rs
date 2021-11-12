#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS(i32);
pub struct DEVINTERFACE_AUDIOENDPOINTPLUGIN(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 3u32,
};
pub struct EndpointConnectorType(i32);
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
