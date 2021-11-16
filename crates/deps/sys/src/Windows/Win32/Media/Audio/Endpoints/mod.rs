#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2670689126,
    data2: 26028,
    data3: 20390,
    data4: [138, 228, 18, 60, 120, 184, 147, 19],
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::core::GUID { data1: 316160983, data2: 53010, data3: 18110, data4: [133, 64, 129, 39, 16, 211, 2, 28] },
    pid: 3u32,
};
pub const eHostProcessConnector: i32 = 0i32;
pub const eOffloadConnector: i32 = 1i32;
pub const eLoopbackConnector: i32 = 2i32;
pub const eKeywordDetectorConnector: i32 = 3i32;
pub const eConnectorCount: i32 = 4i32;
#[repr(transparent)]
pub struct IAudioEndpointFormatControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointFormatControl {}
impl ::core::clone::Clone for IAudioEndpointFormatControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointLastBufferControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointLastBufferControl {}
impl ::core::clone::Clone for IAudioEndpointLastBufferControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamMeter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointOffloadStreamMeter {}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamMeter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamMute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointOffloadStreamMute {}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamMute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamVolume(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointOffloadStreamVolume {}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamVolume {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointVolume(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointVolume {}
impl ::core::clone::Clone for IAudioEndpointVolume {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointVolumeCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointVolumeCallback {}
impl ::core::clone::Clone for IAudioEndpointVolumeCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioEndpointVolumeEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioEndpointVolumeEx {}
impl ::core::clone::Clone for IAudioEndpointVolumeEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioLfxControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioLfxControl {}
impl ::core::clone::Clone for IAudioLfxControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioMeterInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioMeterInformation {}
impl ::core::clone::Clone for IAudioMeterInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHardwareAudioEngineBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHardwareAudioEngineBase {}
impl ::core::clone::Clone for IHardwareAudioEngineBase {
    fn clone(&self) -> Self {
        *self
    }
}
