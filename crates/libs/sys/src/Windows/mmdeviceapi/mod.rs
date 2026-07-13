#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("mmdevapi.dll" "system" fn ActivateAudioInterfaceAsync(deviceinterfacepath : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, activationparams : *const super::propidlbase::PROPVARIANT, completionhandler : *mut core::ffi::c_void, activationoperation : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = i32;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 0;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 3;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 1;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct AudioExtensionParams {
    pub AddPageParam: super::minwindef::LPARAM,
    pub pEndpoint: *mut core::ffi::c_void,
    pub pPnpInterface: *mut core::ffi::c_void,
    pub pPnpDevnode: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for AudioExtensionParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_STATEMASK_ALL: u32 = 15;
pub const DEVICE_STATE_ACTIVE: u32 = 1;
pub const DEVICE_STATE_DISABLED: u32 = 2;
pub const DEVICE_STATE_NOTPRESENT: u32 = 4;
pub const DEVICE_STATE_UNPLUGGED: u32 = 8;
pub const DEVINTERFACE_AUDIO_CAPTURE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
pub const DEVINTERFACE_AUDIO_RENDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
pub const DEVINTERFACE_MIDI_INPUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
pub const DEVINTERFACE_MIDI_OUTPUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: windows_sys::core::GUID,
    pub dwAudioStreamFlags: u32,
}
pub const DigitalAudioDisplayDevice: EndpointFormFactor = 9;
pub type EDataFlow = i32;
pub const EDataFlow_enum_count: EDataFlow = 3;
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1;
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0;
pub type ERole = i32;
pub const ERole_enum_count: ERole = 3;
pub const E_NOTFOUND: i32 = -2147023728;
pub const E_UNSUPPORTED_TYPE: i32 = -2147023266;
pub type EndpointFormFactor = i32;
pub const EndpointFormFactor_enum_count: EndpointFormFactor = 11;
pub const HDMI: u32 = 9;
pub const Handset: EndpointFormFactor = 6;
pub const Headphones: EndpointFormFactor = 3;
pub const Headset: EndpointFormFactor = 5;
pub const LineLevel: EndpointFormFactor = 2;
pub const MMDeviceEnumerator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
pub const Microphone: EndpointFormFactor = 4;
pub type PDIRECTX_AUDIO_ACTIVATION_PARAMS = *mut DIRECTX_AUDIO_ACTIVATION_PARAMS;
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpointLogo_IconEffects: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 0 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpointLogo_IconPath: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpointSettings_MenuText: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 0 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Association: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_FormFactor: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 0 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_GUID: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_JackSubType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 8 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Max_VolumeInDb: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Min_VolumeInDb: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 11 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEngine_DeviceFormat: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c), pid: 0 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEngine_OEMFormat: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04), pid: 3 };
pub const RemoteNetworkDevice: EndpointFormFactor = 0;
pub const SPDIF: EndpointFormFactor = 8;
pub const Speakers: EndpointFormFactor = 1;
pub const UnknownDigitalPassthrough: EndpointFormFactor = 7;
pub const UnknownFormFactor: EndpointFormFactor = 10;
pub const eAll: EDataFlow = 2;
pub const eCapture: EDataFlow = 1;
pub const eCommunications: ERole = 2;
pub const eConsole: ERole = 0;
pub const eMultimedia: ERole = 1;
pub const eRender: EDataFlow = 0;
