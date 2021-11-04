#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_ALREADY_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073919i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_ALREADY_UNLOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073914i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_APO_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073910i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_BUFFERS_OVERLAP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073915i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073917i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_INVALID_APO_CLSID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073916i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_INVALID_COEFFCOUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073909i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_INVALID_COEFFICIENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073908i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073911i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_INVALID_CURVE_PARAM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073907i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_INVALID_INPUTID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073906i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073912i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_NOT_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073918i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073913i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows::runtime::GUID,
}
impl APOInitBaseStruct {}
impl ::std::default::Default for APOInitBaseStruct {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for APOInitBaseStruct {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APOInitBaseStruct").field("cbSize", &self.cbSize).field("clsid", &self.clsid).finish()
    }
}
impl ::std::cmp::PartialEq for APOInitBaseStruct {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.clsid == other.clsid
    }
}
impl ::std::cmp::Eq for APOInitBaseStruct {}
unsafe impl ::windows::runtime::Abi for APOInitBaseStruct {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Audio_CoreAudio`, `Win32_System_PropertiesSystem`*"]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::std::option::Option<super::super::super::System::PropertiesSystem::IPropertyStore>,
    pub pAPOSystemEffectsProperties: ::std::option::Option<super::super::super::System::PropertiesSystem::IPropertyStore>,
    pub pReserved: *mut ::std::ffi::c_void,
    pub pDeviceCollection: ::std::option::Option<super::CoreAudio::IMMDeviceCollection>,
}
#[cfg(all(feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl APOInitSystemEffects {}
#[cfg(all(feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl ::std::default::Default for APOInitSystemEffects {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl ::std::fmt::Debug for APOInitSystemEffects {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APOInitSystemEffects")
            .field("APOInit", &self.APOInit)
            .field("pAPOEndpointProperties", &self.pAPOEndpointProperties)
            .field("pAPOSystemEffectsProperties", &self.pAPOSystemEffectsProperties)
            .field("pReserved", &self.pReserved)
            .field("pDeviceCollection", &self.pDeviceCollection)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::PartialEq for APOInitSystemEffects {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pAPOSystemEffectsProperties == other.pAPOSystemEffectsProperties && self.pReserved == other.pReserved && self.pDeviceCollection == other.pDeviceCollection
    }
}
#[cfg(all(feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::Eq for APOInitSystemEffects {}
#[cfg(all(feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for APOInitSystemEffects {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`, `Win32_Media_Audio_CoreAudio`, `Win32_System_PropertiesSystem`*"]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::std::option::Option<super::super::super::System::PropertiesSystem::IPropertyStore>,
    pub pAPOSystemEffectsProperties: ::std::option::Option<super::super::super::System::PropertiesSystem::IPropertyStore>,
    pub pReserved: *mut ::std::ffi::c_void,
    pub pDeviceCollection: ::std::option::Option<super::CoreAudio::IMMDeviceCollection>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows::runtime::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl ::std::default::Default for APOInitSystemEffects2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl ::std::fmt::Debug for APOInitSystemEffects2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APOInitSystemEffects2")
            .field("APOInit", &self.APOInit)
            .field("pAPOEndpointProperties", &self.pAPOEndpointProperties)
            .field("pAPOSystemEffectsProperties", &self.pAPOSystemEffectsProperties)
            .field("pReserved", &self.pReserved)
            .field("pDeviceCollection", &self.pDeviceCollection)
            .field("nSoftwareIoDeviceInCollection", &self.nSoftwareIoDeviceInCollection)
            .field("nSoftwareIoConnectorIndex", &self.nSoftwareIoConnectorIndex)
            .field("AudioProcessingMode", &self.AudioProcessingMode)
            .field("InitializeForDiscoveryOnly", &self.InitializeForDiscoveryOnly)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::PartialEq for APOInitSystemEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit
            && self.pAPOEndpointProperties == other.pAPOEndpointProperties
            && self.pAPOSystemEffectsProperties == other.pAPOSystemEffectsProperties
            && self.pReserved == other.pReserved
            && self.pDeviceCollection == other.pDeviceCollection
            && self.nSoftwareIoDeviceInCollection == other.nSoftwareIoDeviceInCollection
            && self.nSoftwareIoConnectorIndex == other.nSoftwareIoConnectorIndex
            && self.AudioProcessingMode == other.AudioProcessingMode
            && self.InitializeForDiscoveryOnly == other.InitializeForDiscoveryOnly
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::Eq for APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio", feature = "Win32_System_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for APOInitSystemEffects2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APO_CONNECTION_BUFFER_TYPE(pub i32);
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(0i32);
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(1i32);
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(2i32);
impl ::std::convert::From<i32> for APO_CONNECTION_BUFFER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APO_CONNECTION_BUFFER_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: ::std::option::Option<IAudioMediaType>,
    pub u32Signature: u32,
}
impl APO_CONNECTION_DESCRIPTOR {}
impl ::std::default::Default for APO_CONNECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for APO_CONNECTION_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APO_CONNECTION_DESCRIPTOR").field("Type", &self.Type).field("pBuffer", &self.pBuffer).field("u32MaxFrameCount", &self.u32MaxFrameCount).field("pFormat", &self.pFormat).field("u32Signature", &self.u32Signature).finish()
    }
}
impl ::std::cmp::PartialEq for APO_CONNECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pBuffer == other.pBuffer && self.u32MaxFrameCount == other.u32MaxFrameCount && self.pFormat == other.pFormat && self.u32Signature == other.u32Signature
    }
}
impl ::std::cmp::Eq for APO_CONNECTION_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for APO_CONNECTION_DESCRIPTOR {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APO_FLAG(pub i32);
pub const APO_FLAG_NONE: APO_FLAG = APO_FLAG(0i32);
pub const APO_FLAG_INPLACE: APO_FLAG = APO_FLAG(1i32);
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = APO_FLAG(2i32);
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = APO_FLAG(4i32);
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = APO_FLAG(8i32);
pub const APO_FLAG_MIXER: APO_FLAG = APO_FLAG(16i32);
pub const APO_FLAG_DEFAULT: APO_FLAG = APO_FLAG(14i32);
impl ::std::convert::From<i32> for APO_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APO_FLAG {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct APO_REG_PROPERTIES {
    pub clsid: ::windows::runtime::GUID,
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
    pub iidAPOInterfaceList: [::windows::runtime::GUID; 1],
}
impl APO_REG_PROPERTIES {}
impl ::std::default::Default for APO_REG_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for APO_REG_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("APO_REG_PROPERTIES")
            .field("clsid", &self.clsid)
            .field("Flags", &self.Flags)
            .field("szFriendlyName", &self.szFriendlyName)
            .field("szCopyrightInfo", &self.szCopyrightInfo)
            .field("u32MajorVersion", &self.u32MajorVersion)
            .field("u32MinorVersion", &self.u32MinorVersion)
            .field("u32MinInputConnections", &self.u32MinInputConnections)
            .field("u32MaxInputConnections", &self.u32MaxInputConnections)
            .field("u32MinOutputConnections", &self.u32MinOutputConnections)
            .field("u32MaxOutputConnections", &self.u32MaxOutputConnections)
            .field("u32MaxInstances", &self.u32MaxInstances)
            .field("u32NumAPOInterfaces", &self.u32NumAPOInterfaces)
            .field("iidAPOInterfaceList", &self.iidAPOInterfaceList)
            .finish()
    }
}
impl ::std::cmp::PartialEq for APO_REG_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid
            && self.Flags == other.Flags
            && self.szFriendlyName == other.szFriendlyName
            && self.szCopyrightInfo == other.szCopyrightInfo
            && self.u32MajorVersion == other.u32MajorVersion
            && self.u32MinorVersion == other.u32MinorVersion
            && self.u32MinInputConnections == other.u32MinInputConnections
            && self.u32MaxInputConnections == other.u32MaxInputConnections
            && self.u32MinOutputConnections == other.u32MinOutputConnections
            && self.u32MaxOutputConnections == other.u32MaxOutputConnections
            && self.u32MaxInstances == other.u32MaxInstances
            && self.u32NumAPOInterfaces == other.u32NumAPOInterfaces
            && self.iidAPOInterfaceList == other.iidAPOInterfaceList
    }
}
impl ::std::cmp::Eq for APO_REG_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for APO_REG_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIO_FLOW_TYPE(pub i32);
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(0i32);
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(1i32);
impl ::std::convert::From<i32> for AUDIO_FLOW_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUDIO_FLOW_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`, `Win32_System_PropertiesSystem`*"]
pub struct AudioFXExtensionParams {
    pub AddPageParam: super::super::super::Foundation::LPARAM,
    pub pwstrEndpointID: super::super::super::Foundation::PWSTR,
    pub pFxProperties: ::std::option::Option<super::super::super::System::PropertiesSystem::IPropertyStore>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
impl AudioFXExtensionParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::default::Default for AudioFXExtensionParams {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::fmt::Debug for AudioFXExtensionParams {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AudioFXExtensionParams").field("AddPageParam", &self.AddPageParam).field("pwstrEndpointID", &self.pwstrEndpointID).field("pFxProperties", &self.pFxProperties).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::PartialEq for AudioFXExtensionParams {
    fn eq(&self, other: &Self) -> bool {
        self.AddPageParam == other.AddPageParam && self.pwstrEndpointID == other.pwstrEndpointID && self.pFxProperties == other.pFxProperties
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
impl ::std::cmp::Eq for AudioFXExtensionParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for AudioFXExtensionParams {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
pub const CLSID_DirectMusic: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1667997456, 3197, 4561, [149, 178, 0, 32, 175, 220, 116, 33]);
pub const CLSID_DirectMusicCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1209005232, 10418, 4561, [190, 247, 0, 192, 79, 191, 143, 239]);
pub const CLSID_DirectMusicSynth: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1489155280, 18151, 4561, [137, 172, 0, 160, 201, 5, 65, 41]);
pub const CLSID_DirectMusicSynthSink: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2931916003, 42260, 4561, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const CLSID_DirectSound: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1205131590, 25320, 4559, [147, 188, 68, 69, 83, 84, 0, 0]);
pub const CLSID_DirectSound8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(956419135, 33973, 20388, [186, 53, 170, 129, 114, 184, 160, 155]);
pub const CLSID_DirectSoundCapture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2954954624, 35277, 4560, [175, 8, 0, 160, 201, 37, 205, 22]);
pub const CLSID_DirectSoundCapture8: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3837570067, 32665, 18696, [154, 142, 116, 227, 191, 36, 182, 225]);
pub const CLSID_DirectSoundFullDuplex: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4272173068, 31065, 16711, [178, 106, 35, 119, 185, 231, 169, 29]);
pub const CLSID_DirectSoundPrivate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(296435392, 9708, 4561, [164, 216, 0, 192, 79, 194, 138, 202]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct CONNECTION {
    pub usSource: u16,
    pub usControl: u16,
    pub usDestination: u16,
    pub usTransform: u16,
    pub lScale: i32,
}
impl CONNECTION {}
impl ::std::default::Default for CONNECTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONNECTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONNECTION").field("usSource", &self.usSource).field("usControl", &self.usControl).field("usDestination", &self.usDestination).field("usTransform", &self.usTransform).field("lScale", &self.lScale).finish()
    }
}
impl ::std::cmp::PartialEq for CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.usSource == other.usSource && self.usControl == other.usControl && self.usDestination == other.usDestination && self.usTransform == other.usTransform && self.lScale == other.lScale
    }
}
impl ::std::cmp::Eq for CONNECTION {}
unsafe impl ::windows::runtime::Abi for CONNECTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct CONNECTIONLIST {
    pub cbSize: u32,
    pub cConnections: u32,
}
impl CONNECTIONLIST {}
impl ::std::default::Default for CONNECTIONLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONNECTIONLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONNECTIONLIST").field("cbSize", &self.cbSize).field("cConnections", &self.cConnections).finish()
    }
}
impl ::std::cmp::PartialEq for CONNECTIONLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cConnections == other.cConnections
    }
}
impl ::std::cmp::Eq for CONNECTIONLIST {}
unsafe impl ::windows::runtime::Abi for CONNECTIONLIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_ATTENUATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_CENTER: u32 = 18u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_CHORUS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_ATTACKTIME: u32 = 518u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_DECAYTIME: u32 = 519u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_DELAYTIME: u32 = 523u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_HOLDTIME: u32 = 524u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_RELEASETIME: u32 = 521u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_SHUTDOWNTIME: u32 = 525u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG1_SUSTAINLEVEL: u32 = 522u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_ATTACKTIME: u32 = 778u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_DECAYTIME: u32 = 779u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_DELAYTIME: u32 = 783u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_HOLDTIME: u32 = 784u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_RELEASETIME: u32 = 781u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_EG2_SUSTAINLEVEL: u32 = 782u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_FILTER_CUTOFF: u32 = 1280u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_FILTER_Q: u32 = 1281u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_GAIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_KEYNUMBER: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LEFT: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LEFTREAR: u32 = 19u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LFE_CHANNEL: u32 = 21u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LFO_FREQUENCY: u32 = 260u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_LFO_STARTDELAY: u32 = 261u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_PAN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_PITCH: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_REVERB: u32 = 129u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_RIGHT: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_RIGHTREAR: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_VIB_FREQUENCY: u32 = 276u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_DST_VIB_STARTDELAY: u32 = 277u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC1: u32 = 129u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC10: u32 = 138u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC11: u32 = 139u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC7: u32 = 135u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC91: u32 = 219u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CC93: u32 = 221u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_CHANNELPRESSURE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_EG1: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_EG2: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_KEYNUMBER: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_KEYONVELOCITY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_LFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_MONOPRESSURE: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_PITCHWHEEL: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_POLYPRESSURE: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_SRC_VIBRATO: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_TRN_CONCAVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_TRN_CONVEX: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_TRN_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const CONN_TRN_SWITCH: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN10_VOICE_PRIORITY_OFFSET: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN11_VOICE_PRIORITY_OFFSET: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN12_VOICE_PRIORITY_OFFSET: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN13_VOICE_PRIORITY_OFFSET: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN14_VOICE_PRIORITY_OFFSET: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN15_VOICE_PRIORITY_OFFSET: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN16_VOICE_PRIORITY_OFFSET: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN1_VOICE_PRIORITY_OFFSET: u32 = 14u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN2_VOICE_PRIORITY_OFFSET: u32 = 13u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN3_VOICE_PRIORITY_OFFSET: u32 = 12u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN4_VOICE_PRIORITY_OFFSET: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN5_VOICE_PRIORITY_OFFSET: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN6_VOICE_PRIORITY_OFFSET: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN7_VOICE_PRIORITY_OFFSET: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN8_VOICE_PRIORITY_OFFSET: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CHAN9_VOICE_PRIORITY_OFFSET: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_CRITICAL_VOICE_PRIORITY: u32 = 4026531840u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_HIGH_VOICE_PRIORITY: u32 = 3221225472u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_LOW_VOICE_PRIORITY: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_PERSIST_VOICE_PRIORITY: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DAUD_STANDARD_VOICE_PRIORITY: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTSOUNDDEVICE_DATAFLOW(pub i32);
pub const DIRECTSOUNDDEVICE_DATAFLOW_RENDER: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(0i32);
pub const DIRECTSOUNDDEVICE_DATAFLOW_CAPTURE: DIRECTSOUNDDEVICE_DATAFLOW = DIRECTSOUNDDEVICE_DATAFLOW(1i32);
impl ::std::convert::From<i32> for DIRECTSOUNDDEVICE_DATAFLOW {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTSOUNDDEVICE_DATAFLOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIRECTSOUNDDEVICE_TYPE(pub i32);
pub const DIRECTSOUNDDEVICE_TYPE_EMULATED: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(0i32);
pub const DIRECTSOUNDDEVICE_TYPE_VXD: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(1i32);
pub const DIRECTSOUNDDEVICE_TYPE_WDM: DIRECTSOUNDDEVICE_TYPE = DIRECTSOUNDDEVICE_TYPE(2i32);
impl ::std::convert::From<i32> for DIRECTSOUNDDEVICE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIRECTSOUNDDEVICE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DIRECTSOUND_VERSION: u32 = 1792u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DLSHEADER {
    pub cInstruments: u32,
}
impl DLSHEADER {}
impl ::std::default::Default for DLSHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DLSHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DLSHEADER").field("cInstruments", &self.cInstruments).finish()
    }
}
impl ::std::cmp::PartialEq for DLSHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cInstruments == other.cInstruments
    }
}
impl ::std::cmp::Eq for DLSHEADER {}
unsafe impl ::windows::runtime::Abi for DLSHEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DLSID {
    pub ulData1: u32,
    pub usData2: u16,
    pub usData3: u16,
    pub abData4: [u8; 8],
}
impl DLSID {}
impl ::std::default::Default for DLSID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DLSID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DLSID").field("ulData1", &self.ulData1).field("usData2", &self.usData2).field("usData3", &self.usData3).field("abData4", &self.abData4).finish()
    }
}
impl ::std::cmp::PartialEq for DLSID {
    fn eq(&self, other: &Self) -> bool {
        self.ulData1 == other.ulData1 && self.usData2 == other.usData2 && self.usData3 == other.usData3 && self.abData4 == other.abData4
    }
}
impl ::std::cmp::Eq for DLSID {}
unsafe impl ::windows::runtime::Abi for DLSID {
    type Abi = Self;
}
pub const DLSID_GMInHardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259684, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const DLSID_GSInHardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259685, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const DLSID_ManufacturersID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2956857729, 32917, 4562, [161, 239, 0, 96, 8, 51, 219, 216]);
pub const DLSID_ProductID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2956857730, 32917, 4562, [161, 239, 0, 96, 8, 51, 219, 216]);
pub const DLSID_SampleMemorySize: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259688, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const DLSID_SamplePlaybackRate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(714209043, 42175, 4562, [187, 223, 0, 96, 8, 51, 219, 216]);
pub const DLSID_SupportsDLS1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259687, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const DLSID_SupportsDLS2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4047870437, 18057, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const DLSID_XGInHardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259686, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DLSVERSION {
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
}
impl DLSVERSION {}
impl ::std::default::Default for DLSVERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DLSVERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DLSVERSION").field("dwVersionMS", &self.dwVersionMS).field("dwVersionLS", &self.dwVersionLS).finish()
    }
}
impl ::std::cmp::PartialEq for DLSVERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersionMS == other.dwVersionMS && self.dwVersionLS == other.dwVersionLS
    }
}
impl ::std::cmp::Eq for DLSVERSION {}
unsafe impl ::windows::runtime::Abi for DLSVERSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_ADD: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_AND: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_CONST: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_DIVIDE: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_EQ: u32 = 14u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_GE: u32 = 13u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_GT: u32 = 12u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_LE: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_LOGICAL_AND: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_LOGICAL_OR: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_LT: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_MULTIPLY: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_NOT: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_OR: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_QUERY: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_QUERYSUPPORTED: u32 = 18u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_SUBTRACT: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DLS_CDL_XOR: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_ARTICPARAMS {
    pub LFO: DMUS_LFOPARAMS,
    pub VolEG: DMUS_VEGPARAMS,
    pub PitchEG: DMUS_PEGPARAMS,
    pub Misc: DMUS_MSCPARAMS,
}
impl DMUS_ARTICPARAMS {}
impl ::std::default::Default for DMUS_ARTICPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_ARTICPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_ARTICPARAMS").field("LFO", &self.LFO).field("VolEG", &self.VolEG).field("PitchEG", &self.PitchEG).field("Misc", &self.Misc).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_ARTICPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.LFO == other.LFO && self.VolEG == other.VolEG && self.PitchEG == other.PitchEG && self.Misc == other.Misc
    }
}
impl ::std::cmp::Eq for DMUS_ARTICPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_ARTICPARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_ARTICULATION {
    pub ulArt1Idx: u32,
    pub ulFirstExtCkIdx: u32,
}
impl DMUS_ARTICULATION {}
impl ::std::default::Default for DMUS_ARTICULATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_ARTICULATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_ARTICULATION").field("ulArt1Idx", &self.ulArt1Idx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_ARTICULATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulArt1Idx == other.ulArt1Idx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx
    }
}
impl ::std::cmp::Eq for DMUS_ARTICULATION {}
unsafe impl ::windows::runtime::Abi for DMUS_ARTICULATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_ARTICULATION2 {
    pub ulArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulNextArtIdx: u32,
}
impl DMUS_ARTICULATION2 {}
impl ::std::default::Default for DMUS_ARTICULATION2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_ARTICULATION2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_ARTICULATION2").field("ulArtIdx", &self.ulArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulNextArtIdx", &self.ulNextArtIdx).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_ARTICULATION2 {
    fn eq(&self, other: &Self) -> bool {
        self.ulArtIdx == other.ulArtIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.ulNextArtIdx == other.ulNextArtIdx
    }
}
impl ::std::cmp::Eq for DMUS_ARTICULATION2 {}
unsafe impl ::windows::runtime::Abi for DMUS_ARTICULATION2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_BUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidBufferFormat: ::windows::runtime::GUID,
    pub cbBuffer: u32,
}
impl DMUS_BUFFERDESC {}
impl ::std::default::Default for DMUS_BUFFERDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_BUFFERDESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_BUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidBufferFormat", &self.guidBufferFormat).field("cbBuffer", &self.cbBuffer).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_BUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidBufferFormat == other.guidBufferFormat && self.cbBuffer == other.cbBuffer
    }
}
impl ::std::cmp::Eq for DMUS_BUFFERDESC {}
unsafe impl ::windows::runtime::Abi for DMUS_BUFFERDESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_CLOCKF_GLOBAL: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_CLOCKINFO7 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: ::windows::runtime::GUID,
    pub wszDescription: [u16; 128],
}
impl DMUS_CLOCKINFO7 {}
impl ::std::default::Default for DMUS_CLOCKINFO7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_CLOCKINFO7 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_CLOCKINFO7").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_CLOCKINFO7 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ctType == other.ctType && self.guidClock == other.guidClock && self.wszDescription == other.wszDescription
    }
}
impl ::std::cmp::Eq for DMUS_CLOCKINFO7 {}
unsafe impl ::windows::runtime::Abi for DMUS_CLOCKINFO7 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_CLOCKINFO8 {
    pub dwSize: u32,
    pub ctType: DMUS_CLOCKTYPE,
    pub guidClock: ::windows::runtime::GUID,
    pub wszDescription: [u16; 128],
    pub dwFlags: u32,
}
impl DMUS_CLOCKINFO8 {}
impl ::std::default::Default for DMUS_CLOCKINFO8 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_CLOCKINFO8 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_CLOCKINFO8").field("dwSize", &self.dwSize).field("ctType", &self.ctType).field("guidClock", &self.guidClock).field("wszDescription", &self.wszDescription).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_CLOCKINFO8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ctType == other.ctType && self.guidClock == other.guidClock && self.wszDescription == other.wszDescription && self.dwFlags == other.dwFlags
    }
}
impl ::std::cmp::Eq for DMUS_CLOCKINFO8 {}
unsafe impl ::windows::runtime::Abi for DMUS_CLOCKINFO8 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DMUS_CLOCKTYPE(pub i32);
pub const DMUS_CLOCK_SYSTEM: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(0i32);
pub const DMUS_CLOCK_WAVE: DMUS_CLOCKTYPE = DMUS_CLOCKTYPE(1i32);
impl ::std::convert::From<i32> for DMUS_CLOCKTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DMUS_CLOCKTYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_COPYRIGHT {
    pub cbSize: u32,
    pub byCopyright: [u8; 4],
}
impl DMUS_COPYRIGHT {}
impl ::std::default::Default for DMUS_COPYRIGHT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_COPYRIGHT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_COPYRIGHT").field("cbSize", &self.cbSize).field("byCopyright", &self.byCopyright).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_COPYRIGHT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.byCopyright == other.byCopyright
    }
}
impl ::std::cmp::Eq for DMUS_COPYRIGHT {}
unsafe impl ::windows::runtime::Abi for DMUS_COPYRIGHT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DEFAULT_SIZE_OFFSETTABLE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_DOWNLOADINFO {
    pub dwDLType: u32,
    pub dwDLId: u32,
    pub dwNumOffsetTableEntries: u32,
    pub cbSize: u32,
}
impl DMUS_DOWNLOADINFO {}
impl ::std::default::Default for DMUS_DOWNLOADINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_DOWNLOADINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_DOWNLOADINFO").field("dwDLType", &self.dwDLType).field("dwDLId", &self.dwDLId).field("dwNumOffsetTableEntries", &self.dwNumOffsetTableEntries).field("cbSize", &self.cbSize).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_DOWNLOADINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwDLType == other.dwDLType && self.dwDLId == other.dwDLId && self.dwNumOffsetTableEntries == other.dwNumOffsetTableEntries && self.cbSize == other.cbSize
    }
}
impl ::std::cmp::Eq for DMUS_DOWNLOADINFO {}
unsafe impl ::windows::runtime::Abi for DMUS_DOWNLOADINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_INSTRUMENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_INSTRUMENT2: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_ONESHOTWAVE: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_STREAMINGWAVE: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_WAVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_DOWNLOADINFO_WAVEARTICULATION: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EFFECT_CHORUS: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EFFECT_DELAY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EFFECT_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EFFECT_REVERB: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_EVENTHEADER {
    pub cbEvent: u32,
    pub dwChannelGroup: u32,
    pub rtDelta: i64,
    pub dwFlags: u32,
}
impl DMUS_EVENTHEADER {}
impl ::std::default::Default for DMUS_EVENTHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DMUS_EVENTHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DMUS_EVENTHEADER {}
unsafe impl ::windows::runtime::Abi for DMUS_EVENTHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_EVENT_STRUCTURED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_EXTENSIONCHUNK {
    pub cbSize: u32,
    pub ulNextExtCkIdx: u32,
    pub ExtCkID: u32,
    pub byExtCk: [u8; 4],
}
impl DMUS_EXTENSIONCHUNK {}
impl ::std::default::Default for DMUS_EXTENSIONCHUNK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_EXTENSIONCHUNK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_EXTENSIONCHUNK").field("cbSize", &self.cbSize).field("ulNextExtCkIdx", &self.ulNextExtCkIdx).field("ExtCkID", &self.ExtCkID).field("byExtCk", &self.byExtCk).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_EXTENSIONCHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulNextExtCkIdx == other.ulNextExtCkIdx && self.ExtCkID == other.ExtCkID && self.byExtCk == other.byExtCk
    }
}
impl ::std::cmp::Eq for DMUS_EXTENSIONCHUNK {}
unsafe impl ::windows::runtime::Abi for DMUS_EXTENSIONCHUNK {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_INSTRUMENT {
    pub ulPatch: u32,
    pub ulFirstRegionIdx: u32,
    pub ulGlobalArtIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulFlags: u32,
}
impl DMUS_INSTRUMENT {}
impl ::std::default::Default for DMUS_INSTRUMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_INSTRUMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_INSTRUMENT").field("ulPatch", &self.ulPatch).field("ulFirstRegionIdx", &self.ulFirstRegionIdx).field("ulGlobalArtIdx", &self.ulGlobalArtIdx).field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx).field("ulCopyrightIdx", &self.ulCopyrightIdx).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_INSTRUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ulPatch == other.ulPatch && self.ulFirstRegionIdx == other.ulFirstRegionIdx && self.ulGlobalArtIdx == other.ulGlobalArtIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.ulCopyrightIdx == other.ulCopyrightIdx && self.ulFlags == other.ulFlags
    }
}
impl ::std::cmp::Eq for DMUS_INSTRUMENT {}
unsafe impl ::windows::runtime::Abi for DMUS_INSTRUMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_INSTRUMENT_GM_INSTRUMENT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_LFOPARAMS {
    pub pcFrequency: i32,
    pub tcDelay: i32,
    pub gcVolumeScale: i32,
    pub pcPitchScale: i32,
    pub gcMWToVolume: i32,
    pub pcMWToPitch: i32,
}
impl DMUS_LFOPARAMS {}
impl ::std::default::Default for DMUS_LFOPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_LFOPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_LFOPARAMS").field("pcFrequency", &self.pcFrequency).field("tcDelay", &self.tcDelay).field("gcVolumeScale", &self.gcVolumeScale).field("pcPitchScale", &self.pcPitchScale).field("gcMWToVolume", &self.gcMWToVolume).field("pcMWToPitch", &self.pcMWToPitch).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_LFOPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.pcFrequency == other.pcFrequency && self.tcDelay == other.tcDelay && self.gcVolumeScale == other.gcVolumeScale && self.pcPitchScale == other.pcPitchScale && self.gcMWToVolume == other.gcMWToVolume && self.pcMWToPitch == other.pcMWToPitch
    }
}
impl ::std::cmp::Eq for DMUS_LFOPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_LFOPARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MAX_DESCRIPTION: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MAX_DRIVER: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_MIN_DATA_SIZE: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_MSCPARAMS {
    pub ptDefaultPan: i32,
}
impl DMUS_MSCPARAMS {}
impl ::std::default::Default for DMUS_MSCPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_MSCPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_MSCPARAMS").field("ptDefaultPan", &self.ptDefaultPan).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_MSCPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ptDefaultPan == other.ptDefaultPan
    }
}
impl ::std::cmp::Eq for DMUS_MSCPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_MSCPARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_NOTERANGE {
    pub dwLowNote: u32,
    pub dwHighNote: u32,
}
impl DMUS_NOTERANGE {}
impl ::std::default::Default for DMUS_NOTERANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_NOTERANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_NOTERANGE").field("dwLowNote", &self.dwLowNote).field("dwHighNote", &self.dwHighNote).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_NOTERANGE {
    fn eq(&self, other: &Self) -> bool {
        self.dwLowNote == other.dwLowNote && self.dwHighNote == other.dwHighNote
    }
}
impl ::std::cmp::Eq for DMUS_NOTERANGE {}
unsafe impl ::windows::runtime::Abi for DMUS_NOTERANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_OFFSETTABLE {
    pub ulOffsetTable: [u32; 1],
}
impl DMUS_OFFSETTABLE {}
impl ::std::default::Default for DMUS_OFFSETTABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_OFFSETTABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_OFFSETTABLE").field("ulOffsetTable", &self.ulOffsetTable).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_OFFSETTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffsetTable == other.ulOffsetTable
    }
}
impl ::std::cmp::Eq for DMUS_OFFSETTABLE {}
unsafe impl ::windows::runtime::Abi for DMUS_OFFSETTABLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_AUDIOPATH: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_DIRECTSOUND: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_DLS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_DLS2: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_EXTERNAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_GMINHARDWARE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_GSINHARDWARE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_INPUTCLASS: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_MEMORYSIZEFIXED: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_OUTPUTCLASS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_SHAREABLE: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_SOFTWARESYNTH: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_SYSTEMMEMORY: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_WAVE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PC_XGINHARDWARE: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_PEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
    pub pcRange: i32,
}
impl DMUS_PEGPARAMS {}
impl ::std::default::Default for DMUS_PEGPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_PEGPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_PEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).field("pcRange", &self.pcRange).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_PEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.tcAttack == other.tcAttack && self.tcDecay == other.tcDecay && self.ptSustain == other.ptSustain && self.tcRelease == other.tcRelease && self.tcVel2Attack == other.tcVel2Attack && self.tcKey2Decay == other.tcKey2Decay && self.pcRange == other.pcRange
    }
}
impl ::std::cmp::Eq for DMUS_PEGPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_PEGPARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_PORTCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidPort: ::windows::runtime::GUID,
    pub dwClass: u32,
    pub dwType: u32,
    pub dwMemorySize: u32,
    pub dwMaxChannelGroups: u32,
    pub dwMaxVoices: u32,
    pub dwMaxAudioChannels: u32,
    pub dwEffectFlags: u32,
    pub wszDescription: [u16; 128],
}
impl DMUS_PORTCAPS {}
impl ::std::default::Default for DMUS_PORTCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_PORTCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_PORTCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("guidPort", &self.guidPort)
            .field("dwClass", &self.dwClass)
            .field("dwType", &self.dwType)
            .field("dwMemorySize", &self.dwMemorySize)
            .field("dwMaxChannelGroups", &self.dwMaxChannelGroups)
            .field("dwMaxVoices", &self.dwMaxVoices)
            .field("dwMaxAudioChannels", &self.dwMaxAudioChannels)
            .field("dwEffectFlags", &self.dwEffectFlags)
            .field("wszDescription", &self.wszDescription)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_PORTCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidPort == other.guidPort && self.dwClass == other.dwClass && self.dwType == other.dwType && self.dwMemorySize == other.dwMemorySize && self.dwMaxChannelGroups == other.dwMaxChannelGroups && self.dwMaxVoices == other.dwMaxVoices && self.dwMaxAudioChannels == other.dwMaxAudioChannels && self.dwEffectFlags == other.dwEffectFlags && self.wszDescription == other.wszDescription
    }
}
impl ::std::cmp::Eq for DMUS_PORTCAPS {}
unsafe impl ::windows::runtime::Abi for DMUS_PORTCAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DMUS_PORTPARAMS8 {
    pub dwSize: u32,
    pub dwValidParams: u32,
    pub dwVoices: u32,
    pub dwChannelGroups: u32,
    pub dwAudioChannels: u32,
    pub dwSampleRate: u32,
    pub dwEffectFlags: u32,
    pub fShare: super::super::super::Foundation::BOOL,
    pub dwFeatures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DMUS_PORTPARAMS8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DMUS_PORTPARAMS8 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DMUS_PORTPARAMS8 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_PORTPARAMS8")
            .field("dwSize", &self.dwSize)
            .field("dwValidParams", &self.dwValidParams)
            .field("dwVoices", &self.dwVoices)
            .field("dwChannelGroups", &self.dwChannelGroups)
            .field("dwAudioChannels", &self.dwAudioChannels)
            .field("dwSampleRate", &self.dwSampleRate)
            .field("dwEffectFlags", &self.dwEffectFlags)
            .field("fShare", &self.fShare)
            .field("dwFeatures", &self.dwFeatures)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DMUS_PORTPARAMS8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidParams == other.dwValidParams && self.dwVoices == other.dwVoices && self.dwChannelGroups == other.dwChannelGroups && self.dwAudioChannels == other.dwAudioChannels && self.dwSampleRate == other.dwSampleRate && self.dwEffectFlags == other.dwEffectFlags && self.fShare == other.fShare && self.dwFeatures == other.dwFeatures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DMUS_PORTPARAMS8 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DMUS_PORTPARAMS8 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_AUDIOCHANNELS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_CHANNELGROUPS: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_EFFECTS: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_FEATURES: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_SAMPLERATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_SHARE: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORTPARAMS_VOICES: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_FEATURE_AUDIOPATH: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_FEATURE_STREAMING: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_KERNEL_MODE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_USER_MODE_SYNTH: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_PORT_WINMM_DRIVER: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_REGION {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
    pub ulRegionArtIdx: u32,
    pub ulNextRegionIdx: u32,
    pub ulFirstExtCkIdx: u32,
    pub WaveLink: WAVELINK,
    pub WSMP: _rwsmp,
    pub WLOOP: [_rloop; 1],
}
impl DMUS_REGION {}
impl ::std::default::Default for DMUS_REGION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_REGION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_REGION")
            .field("RangeKey", &self.RangeKey)
            .field("RangeVelocity", &self.RangeVelocity)
            .field("fusOptions", &self.fusOptions)
            .field("usKeyGroup", &self.usKeyGroup)
            .field("ulRegionArtIdx", &self.ulRegionArtIdx)
            .field("ulNextRegionIdx", &self.ulNextRegionIdx)
            .field("ulFirstExtCkIdx", &self.ulFirstExtCkIdx)
            .field("WaveLink", &self.WaveLink)
            .field("WSMP", &self.WSMP)
            .field("WLOOP", &self.WLOOP)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.RangeKey == other.RangeKey && self.RangeVelocity == other.RangeVelocity && self.fusOptions == other.fusOptions && self.usKeyGroup == other.usKeyGroup && self.ulRegionArtIdx == other.ulRegionArtIdx && self.ulNextRegionIdx == other.ulNextRegionIdx && self.ulFirstExtCkIdx == other.ulFirstExtCkIdx && self.WaveLink == other.WaveLink && self.WSMP == other.WSMP && self.WLOOP == other.WLOOP
    }
}
impl ::std::cmp::Eq for DMUS_REGION {}
unsafe impl ::windows::runtime::Abi for DMUS_REGION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_SYNTHSTATS {
    pub dwSize: u32,
    pub dwValidStats: u32,
    pub dwVoices: u32,
    pub dwTotalCPU: u32,
    pub dwCPUPerVoice: u32,
    pub dwLostNotes: u32,
    pub dwFreeMemory: u32,
    pub lPeakVolume: i32,
}
impl DMUS_SYNTHSTATS {}
impl ::std::default::Default for DMUS_SYNTHSTATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_SYNTHSTATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_SYNTHSTATS")
            .field("dwSize", &self.dwSize)
            .field("dwValidStats", &self.dwValidStats)
            .field("dwVoices", &self.dwVoices)
            .field("dwTotalCPU", &self.dwTotalCPU)
            .field("dwCPUPerVoice", &self.dwCPUPerVoice)
            .field("dwLostNotes", &self.dwLostNotes)
            .field("dwFreeMemory", &self.dwFreeMemory)
            .field("lPeakVolume", &self.lPeakVolume)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_SYNTHSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidStats == other.dwValidStats && self.dwVoices == other.dwVoices && self.dwTotalCPU == other.dwTotalCPU && self.dwCPUPerVoice == other.dwCPUPerVoice && self.dwLostNotes == other.dwLostNotes && self.dwFreeMemory == other.dwFreeMemory && self.lPeakVolume == other.lPeakVolume
    }
}
impl ::std::cmp::Eq for DMUS_SYNTHSTATS {}
unsafe impl ::windows::runtime::Abi for DMUS_SYNTHSTATS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_SYNTHSTATS8 {
    pub dwSize: u32,
    pub dwValidStats: u32,
    pub dwVoices: u32,
    pub dwTotalCPU: u32,
    pub dwCPUPerVoice: u32,
    pub dwLostNotes: u32,
    pub dwFreeMemory: u32,
    pub lPeakVolume: i32,
    pub dwSynthMemUse: u32,
}
impl DMUS_SYNTHSTATS8 {}
impl ::std::default::Default for DMUS_SYNTHSTATS8 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_SYNTHSTATS8 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_SYNTHSTATS8")
            .field("dwSize", &self.dwSize)
            .field("dwValidStats", &self.dwValidStats)
            .field("dwVoices", &self.dwVoices)
            .field("dwTotalCPU", &self.dwTotalCPU)
            .field("dwCPUPerVoice", &self.dwCPUPerVoice)
            .field("dwLostNotes", &self.dwLostNotes)
            .field("dwFreeMemory", &self.dwFreeMemory)
            .field("lPeakVolume", &self.lPeakVolume)
            .field("dwSynthMemUse", &self.dwSynthMemUse)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_SYNTHSTATS8 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidStats == other.dwValidStats && self.dwVoices == other.dwVoices && self.dwTotalCPU == other.dwTotalCPU && self.dwCPUPerVoice == other.dwCPUPerVoice && self.dwLostNotes == other.dwLostNotes && self.dwFreeMemory == other.dwFreeMemory && self.lPeakVolume == other.lPeakVolume && self.dwSynthMemUse == other.dwSynthMemUse
    }
}
impl ::std::cmp::Eq for DMUS_SYNTHSTATS8 {}
unsafe impl ::windows::runtime::Abi for DMUS_SYNTHSTATS8 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_CPU_PER_VOICE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_FREE_MEMORY: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_LOST_NOTES: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_PEAK_VOLUME: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_SYSTEMMEMORY: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_TOTAL_CPU: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_SYNTHSTATS_VOICES: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_VEGPARAMS {
    pub tcAttack: i32,
    pub tcDecay: i32,
    pub ptSustain: i32,
    pub tcRelease: i32,
    pub tcVel2Attack: i32,
    pub tcKey2Decay: i32,
}
impl DMUS_VEGPARAMS {}
impl ::std::default::Default for DMUS_VEGPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_VEGPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_VEGPARAMS").field("tcAttack", &self.tcAttack).field("tcDecay", &self.tcDecay).field("ptSustain", &self.ptSustain).field("tcRelease", &self.tcRelease).field("tcVel2Attack", &self.tcVel2Attack).field("tcKey2Decay", &self.tcKey2Decay).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_VEGPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.tcAttack == other.tcAttack && self.tcDecay == other.tcDecay && self.ptSustain == other.ptSustain && self.tcRelease == other.tcRelease && self.tcVel2Attack == other.tcVel2Attack && self.tcKey2Decay == other.tcKey2Decay
    }
}
impl ::std::cmp::Eq for DMUS_VEGPARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_VEGPARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DMUS_VOICE_STATE {
    pub bExists: super::super::super::Foundation::BOOL,
    pub spPosition: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl DMUS_VOICE_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DMUS_VOICE_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DMUS_VOICE_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_VOICE_STATE").field("bExists", &self.bExists).field("spPosition", &self.spPosition).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DMUS_VOICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.bExists == other.bExists && self.spPosition == other.spPosition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DMUS_VOICE_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DMUS_VOICE_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_VOLUME_MAX: u32 = 2000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DMUS_VOLUME_MIN: i32 = -20000i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Media_Multimedia")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
pub struct DMUS_WAVE {
    pub ulFirstExtCkIdx: u32,
    pub ulCopyrightIdx: u32,
    pub ulWaveDataIdx: u32,
    pub WaveformatEx: super::super::Multimedia::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl DMUS_WAVE {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::default::Default for DMUS_WAVE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::PartialEq for DMUS_WAVE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::Eq for DMUS_WAVE {}
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::runtime::Abi for DMUS_WAVE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVEARTDL {
    pub ulDownloadIdIdx: u32,
    pub ulBus: u32,
    pub ulBuffers: u32,
    pub ulMasterDLId: u32,
    pub usOptions: u16,
}
impl DMUS_WAVEARTDL {}
impl ::std::default::Default for DMUS_WAVEARTDL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_WAVEARTDL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_WAVEARTDL").field("ulDownloadIdIdx", &self.ulDownloadIdIdx).field("ulBus", &self.ulBus).field("ulBuffers", &self.ulBuffers).field("ulMasterDLId", &self.ulMasterDLId).field("usOptions", &self.usOptions).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_WAVEARTDL {
    fn eq(&self, other: &Self) -> bool {
        self.ulDownloadIdIdx == other.ulDownloadIdIdx && self.ulBus == other.ulBus && self.ulBuffers == other.ulBuffers && self.ulMasterDLId == other.ulMasterDLId && self.usOptions == other.usOptions
    }
}
impl ::std::cmp::Eq for DMUS_WAVEARTDL {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVEARTDL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVEDATA {
    pub cbSize: u32,
    pub byData: [u8; 4],
}
impl DMUS_WAVEDATA {}
impl ::std::default::Default for DMUS_WAVEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_WAVEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_WAVEDATA").field("cbSize", &self.cbSize).field("byData", &self.byData).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_WAVEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.byData == other.byData
    }
}
impl ::std::cmp::Eq for DMUS_WAVEDATA {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVEDL {
    pub cbWaveData: u32,
}
impl DMUS_WAVEDL {}
impl ::std::default::Default for DMUS_WAVEDL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_WAVEDL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_WAVEDL").field("cbWaveData", &self.cbWaveData).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_WAVEDL {
    fn eq(&self, other: &Self) -> bool {
        self.cbWaveData == other.cbWaveData
    }
}
impl ::std::cmp::Eq for DMUS_WAVEDL {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVEDL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DMUS_WAVES_REVERB_PARAMS {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl DMUS_WAVES_REVERB_PARAMS {}
impl ::std::default::Default for DMUS_WAVES_REVERB_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DMUS_WAVES_REVERB_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DMUS_WAVES_REVERB_PARAMS").field("fInGain", &self.fInGain).field("fReverbMix", &self.fReverbMix).field("fReverbTime", &self.fReverbTime).field("fHighFreqRTRatio", &self.fHighFreqRTRatio).finish()
    }
}
impl ::std::cmp::PartialEq for DMUS_WAVES_REVERB_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.fInGain == other.fInGain && self.fReverbMix == other.fReverbMix && self.fReverbTime == other.fReverbTime && self.fHighFreqRTRatio == other.fHighFreqRTRatio
    }
}
impl ::std::cmp::Eq for DMUS_WAVES_REVERB_PARAMS {}
unsafe impl ::windows::runtime::Abi for DMUS_WAVES_REVERB_PARAMS {
    type Abi = Self;
}
pub const DS3DALG_HRTF_FULL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3259052864, 7195, 4562, [148, 245, 0, 192, 79, 194, 138, 202]);
pub const DS3DALG_HRTF_LIGHT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3259052866, 7195, 4562, [148, 245, 0, 192, 79, 194, 138, 202]);
pub const DS3DALG_NO_VIRTUALIZATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3259052863, 7195, 4562, [148, 245, 0, 192, 79, 194, 138, 202]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
pub struct DS3DBUFFER {
    pub dwSize: u32,
    pub vPosition: super::super::super::Graphics::Direct3D9::D3DVECTOR,
    pub vVelocity: super::super::super::Graphics::Direct3D9::D3DVECTOR,
    pub dwInsideConeAngle: u32,
    pub dwOutsideConeAngle: u32,
    pub vConeOrientation: super::super::super::Graphics::Direct3D9::D3DVECTOR,
    pub lConeOutsideVolume: i32,
    pub flMinDistance: f32,
    pub flMaxDistance: f32,
    pub dwMode: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl DS3DBUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::std::default::Default for DS3DBUFFER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::std::fmt::Debug for DS3DBUFFER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DS3DBUFFER")
            .field("dwSize", &self.dwSize)
            .field("vPosition", &self.vPosition)
            .field("vVelocity", &self.vVelocity)
            .field("dwInsideConeAngle", &self.dwInsideConeAngle)
            .field("dwOutsideConeAngle", &self.dwOutsideConeAngle)
            .field("vConeOrientation", &self.vConeOrientation)
            .field("lConeOutsideVolume", &self.lConeOutsideVolume)
            .field("flMinDistance", &self.flMinDistance)
            .field("flMaxDistance", &self.flMaxDistance)
            .field("dwMode", &self.dwMode)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::std::cmp::PartialEq for DS3DBUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.vPosition == other.vPosition && self.vVelocity == other.vVelocity && self.dwInsideConeAngle == other.dwInsideConeAngle && self.dwOutsideConeAngle == other.dwOutsideConeAngle && self.vConeOrientation == other.vConeOrientation && self.lConeOutsideVolume == other.lConeOutsideVolume && self.flMinDistance == other.flMinDistance && self.flMaxDistance == other.flMaxDistance && self.dwMode == other.dwMode
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::std::cmp::Eq for DS3DBUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
unsafe impl ::windows::runtime::Abi for DS3DBUFFER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
pub struct DS3DLISTENER {
    pub dwSize: u32,
    pub vPosition: super::super::super::Graphics::Direct3D9::D3DVECTOR,
    pub vVelocity: super::super::super::Graphics::Direct3D9::D3DVECTOR,
    pub vOrientFront: super::super::super::Graphics::Direct3D9::D3DVECTOR,
    pub vOrientTop: super::super::super::Graphics::Direct3D9::D3DVECTOR,
    pub flDistanceFactor: f32,
    pub flRolloffFactor: f32,
    pub flDopplerFactor: f32,
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl DS3DLISTENER {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::std::default::Default for DS3DLISTENER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::std::fmt::Debug for DS3DLISTENER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DS3DLISTENER")
            .field("dwSize", &self.dwSize)
            .field("vPosition", &self.vPosition)
            .field("vVelocity", &self.vVelocity)
            .field("vOrientFront", &self.vOrientFront)
            .field("vOrientTop", &self.vOrientTop)
            .field("flDistanceFactor", &self.flDistanceFactor)
            .field("flRolloffFactor", &self.flRolloffFactor)
            .field("flDopplerFactor", &self.flDopplerFactor)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::std::cmp::PartialEq for DS3DLISTENER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.vPosition == other.vPosition && self.vVelocity == other.vVelocity && self.vOrientFront == other.vOrientFront && self.vOrientTop == other.vOrientTop && self.flDistanceFactor == other.flDistanceFactor && self.flRolloffFactor == other.flRolloffFactor && self.flDopplerFactor == other.flDopplerFactor
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::std::cmp::Eq for DS3DLISTENER {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
unsafe impl ::windows::runtime::Abi for DS3DLISTENER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3DMODE_DISABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3DMODE_HEADRELATIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3DMODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_DEFAULTCONEANGLE: u32 = 360u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_DEFAULTCONEOUTSIDEVOLUME: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_DEFAULTDISTANCEFACTOR: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_DEFAULTDOPPLERFACTOR: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_DEFAULTMAXDISTANCE: f32 = 1000000000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_DEFAULTMINDISTANCE: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_DEFAULTROLLOFFFACTOR: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_DEFERRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_IMMEDIATE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_MAXCONEANGLE: u32 = 360u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_MAXDOPPLERFACTOR: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_MAXROLLOFFFACTOR: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_MINCONEANGLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_MINDOPPLERFACTOR: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS3D_MINROLLOFFFACTOR: f32 = 0f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwUnlockTransferRate: u32,
    pub dwPlayCpuOverhead: u32,
}
impl DSBCAPS {}
impl ::std::default::Default for DSBCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSBCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSBCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwUnlockTransferRate", &self.dwUnlockTransferRate).field("dwPlayCpuOverhead", &self.dwPlayCpuOverhead).finish()
    }
}
impl ::std::cmp::PartialEq for DSBCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwUnlockTransferRate == other.dwUnlockTransferRate && self.dwPlayCpuOverhead == other.dwPlayCpuOverhead
    }
}
impl ::std::cmp::Eq for DSBCAPS {}
unsafe impl ::windows::runtime::Abi for DSBCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_CTRL3D: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_CTRLFREQUENCY: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_CTRLFX: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_CTRLPAN: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_CTRLVOLUME: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_GETCURRENTPOSITION2: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_GLOBALFOCUS: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_LOCDEFER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_LOCHARDWARE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_LOCSOFTWARE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_PRIMARYBUFFER: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_STATIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_STICKYFOCUS: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBCAPS_TRUEPLAYPOSITION: u32 = 524288u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBFREQUENCY_MAX: u32 = 200000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBFREQUENCY_MIN: u32 = 100u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBFREQUENCY_ORIGINAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBLOCK_ENTIREBUFFER: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBLOCK_FROMWRITECURSOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBNOTIFICATIONS_MAX: u32 = 100000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPAN_CENTER: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPAN_LEFT: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPAN_RIGHT: u32 = 10000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPLAY_LOCHARDWARE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPLAY_LOCSOFTWARE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPLAY_LOOPING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPLAY_TERMINATEBY_DISTANCE: u64 = 16u64;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPLAY_TERMINATEBY_PRIORITY: u64 = 32u64;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPLAY_TERMINATEBY_TIME: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBPN_OFFSETSTOP: u32 = 4294967295u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSBPOSITIONNOTIFY {
    pub dwOffset: u32,
    pub hEventNotify: super::super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl DSBPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSBPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSBPOSITIONNOTIFY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSBPOSITIONNOTIFY").field("dwOffset", &self.dwOffset).field("hEventNotify", &self.hEventNotify).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSBPOSITIONNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.dwOffset == other.dwOffset && self.hEventNotify == other.hEventNotify
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSBPOSITIONNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSBPOSITIONNOTIFY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSIZE_FX_MIN: u32 = 150u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSIZE_MAX: u32 = 268435455u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSIZE_MIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSTATUS_BUFFERLOST: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSTATUS_LOCHARDWARE: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSTATUS_LOCSOFTWARE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSTATUS_LOOPING: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSTATUS_PLAYING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBSTATUS_TERMINATED: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Media_Multimedia")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::super::Multimedia::WAVEFORMATEX,
    pub guid3DAlgorithm: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl DSBUFFERDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::default::Default for DSBUFFERDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::fmt::Debug for DSBUFFERDESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSBUFFERDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).field("guid3DAlgorithm", &self.guid3DAlgorithm).finish()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::PartialEq for DSBUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved && self.lpwfxFormat == other.lpwfxFormat && self.guid3DAlgorithm == other.guid3DAlgorithm
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::Eq for DSBUFFERDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::runtime::Abi for DSBUFFERDESC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Media_Multimedia")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
pub struct DSBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::super::Multimedia::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl DSBUFFERDESC1 {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::default::Default for DSBUFFERDESC1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::fmt::Debug for DSBUFFERDESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSBUFFERDESC1").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).finish()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::PartialEq for DSBUFFERDESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved && self.lpwfxFormat == other.lpwfxFormat
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::Eq for DSBUFFERDESC1 {}
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::runtime::Abi for DSBUFFERDESC1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_BACK_CENTER: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_BACK_LEFT: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_BACK_RIGHT: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_CHORUS_SEND: u32 = 65u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_DYNAMIC_0: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FIRST_SPKR_LOC: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_CENTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_LEFT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_LEFT_OF_CENTER: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_RIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_FRONT_RIGHT_OF_CENTER: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_LAST_SPKR_LOC: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_LEFT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_LOW_FREQUENCY: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_NULL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_REVERB_SEND: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_RIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_SIDE_LEFT: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_SIDE_RIGHT: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_BACK_CENTER: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_BACK_LEFT: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_BACK_RIGHT: u32 = 17u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_CENTER: u32 = 11u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_FRONT_CENTER: u32 = 13u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_FRONT_LEFT: u32 = 12u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBUSID_TOP_FRONT_RIGHT: u32 = 14u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBVOLUME_MAX: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSBVOLUME_MIN: i32 = -10000i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwMinSecondarySampleRate: u32,
    pub dwMaxSecondarySampleRate: u32,
    pub dwPrimaryBuffers: u32,
    pub dwMaxHwMixingAllBuffers: u32,
    pub dwMaxHwMixingStaticBuffers: u32,
    pub dwMaxHwMixingStreamingBuffers: u32,
    pub dwFreeHwMixingAllBuffers: u32,
    pub dwFreeHwMixingStaticBuffers: u32,
    pub dwFreeHwMixingStreamingBuffers: u32,
    pub dwMaxHw3DAllBuffers: u32,
    pub dwMaxHw3DStaticBuffers: u32,
    pub dwMaxHw3DStreamingBuffers: u32,
    pub dwFreeHw3DAllBuffers: u32,
    pub dwFreeHw3DStaticBuffers: u32,
    pub dwFreeHw3DStreamingBuffers: u32,
    pub dwTotalHwMemBytes: u32,
    pub dwFreeHwMemBytes: u32,
    pub dwMaxContigFreeHwMemBytes: u32,
    pub dwUnlockTransferRateHwBuffers: u32,
    pub dwPlayCpuOverheadSwBuffers: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl DSCAPS {}
impl ::std::default::Default for DSCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwMinSecondarySampleRate", &self.dwMinSecondarySampleRate)
            .field("dwMaxSecondarySampleRate", &self.dwMaxSecondarySampleRate)
            .field("dwPrimaryBuffers", &self.dwPrimaryBuffers)
            .field("dwMaxHwMixingAllBuffers", &self.dwMaxHwMixingAllBuffers)
            .field("dwMaxHwMixingStaticBuffers", &self.dwMaxHwMixingStaticBuffers)
            .field("dwMaxHwMixingStreamingBuffers", &self.dwMaxHwMixingStreamingBuffers)
            .field("dwFreeHwMixingAllBuffers", &self.dwFreeHwMixingAllBuffers)
            .field("dwFreeHwMixingStaticBuffers", &self.dwFreeHwMixingStaticBuffers)
            .field("dwFreeHwMixingStreamingBuffers", &self.dwFreeHwMixingStreamingBuffers)
            .field("dwMaxHw3DAllBuffers", &self.dwMaxHw3DAllBuffers)
            .field("dwMaxHw3DStaticBuffers", &self.dwMaxHw3DStaticBuffers)
            .field("dwMaxHw3DStreamingBuffers", &self.dwMaxHw3DStreamingBuffers)
            .field("dwFreeHw3DAllBuffers", &self.dwFreeHw3DAllBuffers)
            .field("dwFreeHw3DStaticBuffers", &self.dwFreeHw3DStaticBuffers)
            .field("dwFreeHw3DStreamingBuffers", &self.dwFreeHw3DStreamingBuffers)
            .field("dwTotalHwMemBytes", &self.dwTotalHwMemBytes)
            .field("dwFreeHwMemBytes", &self.dwFreeHwMemBytes)
            .field("dwMaxContigFreeHwMemBytes", &self.dwMaxContigFreeHwMemBytes)
            .field("dwUnlockTransferRateHwBuffers", &self.dwUnlockTransferRateHwBuffers)
            .field("dwPlayCpuOverheadSwBuffers", &self.dwPlayCpuOverheadSwBuffers)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DSCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.dwMinSecondarySampleRate == other.dwMinSecondarySampleRate
            && self.dwMaxSecondarySampleRate == other.dwMaxSecondarySampleRate
            && self.dwPrimaryBuffers == other.dwPrimaryBuffers
            && self.dwMaxHwMixingAllBuffers == other.dwMaxHwMixingAllBuffers
            && self.dwMaxHwMixingStaticBuffers == other.dwMaxHwMixingStaticBuffers
            && self.dwMaxHwMixingStreamingBuffers == other.dwMaxHwMixingStreamingBuffers
            && self.dwFreeHwMixingAllBuffers == other.dwFreeHwMixingAllBuffers
            && self.dwFreeHwMixingStaticBuffers == other.dwFreeHwMixingStaticBuffers
            && self.dwFreeHwMixingStreamingBuffers == other.dwFreeHwMixingStreamingBuffers
            && self.dwMaxHw3DAllBuffers == other.dwMaxHw3DAllBuffers
            && self.dwMaxHw3DStaticBuffers == other.dwMaxHw3DStaticBuffers
            && self.dwMaxHw3DStreamingBuffers == other.dwMaxHw3DStreamingBuffers
            && self.dwFreeHw3DAllBuffers == other.dwFreeHw3DAllBuffers
            && self.dwFreeHw3DStaticBuffers == other.dwFreeHw3DStaticBuffers
            && self.dwFreeHw3DStreamingBuffers == other.dwFreeHw3DStreamingBuffers
            && self.dwTotalHwMemBytes == other.dwTotalHwMemBytes
            && self.dwFreeHwMemBytes == other.dwFreeHwMemBytes
            && self.dwMaxContigFreeHwMemBytes == other.dwMaxContigFreeHwMemBytes
            && self.dwUnlockTransferRateHwBuffers == other.dwUnlockTransferRateHwBuffers
            && self.dwPlayCpuOverheadSwBuffers == other.dwPlayCpuOverheadSwBuffers
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
    }
}
impl ::std::cmp::Eq for DSCAPS {}
unsafe impl ::windows::runtime::Abi for DSCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_CERTIFIED: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_CONTINUOUSRATE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_EMULDRIVER: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_PRIMARY16BIT: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_PRIMARY8BIT: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_PRIMARYMONO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_PRIMARYSTEREO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_SECONDARY16BIT: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_SECONDARY8BIT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_SECONDARYMONO: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCAPS_SECONDARYSTEREO: u32 = 512u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSCBCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
}
impl DSCBCAPS {}
impl ::std::default::Default for DSCBCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSCBCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSCBCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::std::cmp::PartialEq for DSCBCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved
    }
}
impl ::std::cmp::Eq for DSCBCAPS {}
unsafe impl ::windows::runtime::Abi for DSCBCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCBCAPS_CTRLFX: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCBCAPS_WAVEMAPPED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCBLOCK_ENTIREBUFFER: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCBSTART_LOOPING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCBSTATUS_CAPTURING: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCBSTATUS_LOOPING: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Media_Multimedia")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
pub struct DSCBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::super::Multimedia::WAVEFORMATEX,
    pub dwFXCount: u32,
    pub lpDSCFXDesc: *mut DSCEFFECTDESC,
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl DSCBUFFERDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::default::Default for DSCBUFFERDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::fmt::Debug for DSCBUFFERDESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSCBUFFERDESC")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwBufferBytes", &self.dwBufferBytes)
            .field("dwReserved", &self.dwReserved)
            .field("lpwfxFormat", &self.lpwfxFormat)
            .field("dwFXCount", &self.dwFXCount)
            .field("lpDSCFXDesc", &self.lpDSCFXDesc)
            .finish()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::PartialEq for DSCBUFFERDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved && self.lpwfxFormat == other.lpwfxFormat && self.dwFXCount == other.dwFXCount && self.lpDSCFXDesc == other.lpDSCFXDesc
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::Eq for DSCBUFFERDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::runtime::Abi for DSCBUFFERDESC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Media_Multimedia")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
pub struct DSCBUFFERDESC1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: *mut super::super::Multimedia::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl DSCBUFFERDESC1 {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::default::Default for DSCBUFFERDESC1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::fmt::Debug for DSCBUFFERDESC1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSCBUFFERDESC1").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwBufferBytes", &self.dwBufferBytes).field("dwReserved", &self.dwReserved).field("lpwfxFormat", &self.lpwfxFormat).finish()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::PartialEq for DSCBUFFERDESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwBufferBytes == other.dwBufferBytes && self.dwReserved == other.dwReserved && self.lpwfxFormat == other.lpwfxFormat
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::Eq for DSCBUFFERDESC1 {}
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::runtime::Abi for DSCBUFFERDESC1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSCCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFormats: u32,
    pub dwChannels: u32,
}
impl DSCCAPS {}
impl ::std::default::Default for DSCCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSCCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSCCAPS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwFormats", &self.dwFormats).field("dwChannels", &self.dwChannels).finish()
    }
}
impl ::std::cmp::PartialEq for DSCCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwFormats == other.dwFormats && self.dwChannels == other.dwChannels
    }
}
impl ::std::cmp::Eq for DSCCAPS {}
unsafe impl ::windows::runtime::Abi for DSCCAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCCAPS_CERTIFIED: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCCAPS_EMULDRIVER: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCCAPS_MULTIPLECAPTURE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSCEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSCFXClass: ::windows::runtime::GUID,
    pub guidDSCFXInstance: ::windows::runtime::GUID,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl DSCEFFECTDESC {}
impl ::std::default::Default for DSCEFFECTDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSCEFFECTDESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSCEFFECTDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidDSCFXClass", &self.guidDSCFXClass).field("guidDSCFXInstance", &self.guidDSCFXInstance).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::std::cmp::PartialEq for DSCEFFECTDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidDSCFXClass == other.guidDSCFXClass && self.guidDSCFXInstance == other.guidDSCFXInstance && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::std::cmp::Eq for DSCEFFECTDESC {}
unsafe impl ::windows::runtime::Abi for DSCEFFECTDESC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSCFXAec {
    pub fEnable: super::super::super::Foundation::BOOL,
    pub fNoiseFill: super::super::super::Foundation::BOOL,
    pub dwMode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DSCFXAec {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSCFXAec {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSCFXAec {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSCFXAec").field("fEnable", &self.fEnable).field("fNoiseFill", &self.fNoiseFill).field("dwMode", &self.dwMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSCFXAec {
    fn eq(&self, other: &Self) -> bool {
        self.fEnable == other.fEnable && self.fNoiseFill == other.fNoiseFill && self.dwMode == other.dwMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSCFXAec {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSCFXAec {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSCFXNoiseSuppress {
    pub fEnable: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DSCFXNoiseSuppress {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSCFXNoiseSuppress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSCFXNoiseSuppress {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSCFXNoiseSuppress").field("fEnable", &self.fEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSCFXNoiseSuppress {
    fn eq(&self, other: &Self) -> bool {
        self.fEnable == other.fEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSCFXNoiseSuppress {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSCFXNoiseSuppress {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFXR_LOCHARDWARE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFXR_LOCSOFTWARE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_AEC_MODE_FULL_DUPLEX: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_AEC_MODE_HALF_DUPLEX: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_AEC_MODE_PASS_THROUGH: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_AEC_STATUS_CURRENTLY_CONVERGED: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_AEC_STATUS_HISTORY_CONTINUOUSLY_CONVERGED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_AEC_STATUS_HISTORY_PREVIOUSLY_DIVERGED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_AEC_STATUS_HISTORY_UNINITIALIZED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_LOCHARDWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSCFX_LOCSOFTWARE: u32 = 2u32;
pub const DSDEVID_DefaultCapture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740270593, 40045, 18413, [170, 241, 77, 218, 143, 43, 92, 3]);
pub const DSDEVID_DefaultPlayback: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740270592, 40045, 18413, [170, 241, 77, 218, 143, 43, 92, 3]);
pub const DSDEVID_DefaultVoiceCapture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740270595, 40045, 18413, [170, 241, 77, 218, 143, 43, 92, 3]);
pub const DSDEVID_DefaultVoicePlayback: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3740270594, 40045, 18413, [170, 241, 77, 218, 143, 43, 92, 3]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSEFFECTDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidDSFXClass: ::windows::runtime::GUID,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl DSEFFECTDESC {}
impl ::std::default::Default for DSEFFECTDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSEFFECTDESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSEFFECTDESC").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidDSFXClass", &self.guidDSFXClass).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::std::cmp::PartialEq for DSEFFECTDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidDSFXClass == other.guidDSFXClass && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::std::cmp::Eq for DSEFFECTDESC {}
unsafe impl ::windows::runtime::Abi for DSEFFECTDESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_DELAY_MAX: f32 = 20f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_DELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_DEPTH_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_DEPTH_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_FEEDBACK_MAX: f32 = 99f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_FEEDBACK_MIN: f32 = -99f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_FREQUENCY_MAX: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_FREQUENCY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_PHASE_180: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_PHASE_90: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_PHASE_MAX: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_PHASE_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_PHASE_NEG_180: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_PHASE_NEG_90: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_PHASE_ZERO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_WAVE_SIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_WAVE_TRIANGLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCHORUS_WETDRYMIX_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_ATTACK_MAX: f32 = 500f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_ATTACK_MIN: f32 = 0.01f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_GAIN_MAX: f32 = 60f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_GAIN_MIN: f32 = -60f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_PREDELAY_MAX: f32 = 4f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_PREDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_RATIO_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_RATIO_MIN: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_RELEASE_MAX: f32 = 3000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_RELEASE_MIN: f32 = 50f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_THRESHOLD_MAX: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXCOMPRESSOR_THRESHOLD_MIN: f32 = -60f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXChorus {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl DSFXChorus {}
impl ::std::default::Default for DSFXChorus {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXChorus {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXChorus").field("fWetDryMix", &self.fWetDryMix).field("fDepth", &self.fDepth).field("fFeedback", &self.fFeedback).field("fFrequency", &self.fFrequency).field("lWaveform", &self.lWaveform).field("fDelay", &self.fDelay).field("lPhase", &self.lPhase).finish()
    }
}
impl ::std::cmp::PartialEq for DSFXChorus {
    fn eq(&self, other: &Self) -> bool {
        self.fWetDryMix == other.fWetDryMix && self.fDepth == other.fDepth && self.fFeedback == other.fFeedback && self.fFrequency == other.fFrequency && self.lWaveform == other.lWaveform && self.fDelay == other.fDelay && self.lPhase == other.lPhase
    }
}
impl ::std::cmp::Eq for DSFXChorus {}
unsafe impl ::windows::runtime::Abi for DSFXChorus {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXCompressor {
    pub fGain: f32,
    pub fAttack: f32,
    pub fRelease: f32,
    pub fThreshold: f32,
    pub fRatio: f32,
    pub fPredelay: f32,
}
impl DSFXCompressor {}
impl ::std::default::Default for DSFXCompressor {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXCompressor {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXCompressor").field("fGain", &self.fGain).field("fAttack", &self.fAttack).field("fRelease", &self.fRelease).field("fThreshold", &self.fThreshold).field("fRatio", &self.fRatio).field("fPredelay", &self.fPredelay).finish()
    }
}
impl ::std::cmp::PartialEq for DSFXCompressor {
    fn eq(&self, other: &Self) -> bool {
        self.fGain == other.fGain && self.fAttack == other.fAttack && self.fRelease == other.fRelease && self.fThreshold == other.fThreshold && self.fRatio == other.fRatio && self.fPredelay == other.fPredelay
    }
}
impl ::std::cmp::Eq for DSFXCompressor {}
unsafe impl ::windows::runtime::Abi for DSFXCompressor {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_EDGE_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_EDGE_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_GAIN_MAX: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_GAIN_MIN: f32 = -60f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MAX: f32 = 8000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_POSTEQBANDWIDTH_MIN: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MAX: f32 = 8000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_POSTEQCENTERFREQUENCY_MIN: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MAX: f32 = 8000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXDISTORTION_PRELOWPASSCUTOFF_MIN: f32 = 100f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXDistortion {
    pub fGain: f32,
    pub fEdge: f32,
    pub fPostEQCenterFrequency: f32,
    pub fPostEQBandwidth: f32,
    pub fPreLowpassCutoff: f32,
}
impl DSFXDistortion {}
impl ::std::default::Default for DSFXDistortion {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXDistortion {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXDistortion").field("fGain", &self.fGain).field("fEdge", &self.fEdge).field("fPostEQCenterFrequency", &self.fPostEQCenterFrequency).field("fPostEQBandwidth", &self.fPostEQBandwidth).field("fPreLowpassCutoff", &self.fPreLowpassCutoff).finish()
    }
}
impl ::std::cmp::PartialEq for DSFXDistortion {
    fn eq(&self, other: &Self) -> bool {
        self.fGain == other.fGain && self.fEdge == other.fEdge && self.fPostEQCenterFrequency == other.fPostEQCenterFrequency && self.fPostEQBandwidth == other.fPostEQBandwidth && self.fPreLowpassCutoff == other.fPreLowpassCutoff
    }
}
impl ::std::cmp::Eq for DSFXDistortion {}
unsafe impl ::windows::runtime::Abi for DSFXDistortion {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_FEEDBACK_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_FEEDBACK_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_LEFTDELAY_MAX: f32 = 2000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_LEFTDELAY_MIN: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_PANDELAY_MAX: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_PANDELAY_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_RIGHTDELAY_MAX: f32 = 2000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_RIGHTDELAY_MIN: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXECHO_WETDRYMIX_MIN: f32 = 0f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXEcho {
    pub fWetDryMix: f32,
    pub fFeedback: f32,
    pub fLeftDelay: f32,
    pub fRightDelay: f32,
    pub lPanDelay: i32,
}
impl DSFXEcho {}
impl ::std::default::Default for DSFXEcho {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXEcho {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXEcho").field("fWetDryMix", &self.fWetDryMix).field("fFeedback", &self.fFeedback).field("fLeftDelay", &self.fLeftDelay).field("fRightDelay", &self.fRightDelay).field("lPanDelay", &self.lPanDelay).finish()
    }
}
impl ::std::cmp::PartialEq for DSFXEcho {
    fn eq(&self, other: &Self) -> bool {
        self.fWetDryMix == other.fWetDryMix && self.fFeedback == other.fFeedback && self.fLeftDelay == other.fLeftDelay && self.fRightDelay == other.fRightDelay && self.lPanDelay == other.lPanDelay
    }
}
impl ::std::cmp::Eq for DSFXEcho {}
unsafe impl ::windows::runtime::Abi for DSFXEcho {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_DELAY_MAX: f32 = 4f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_DELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_DEPTH_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_DEPTH_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_FEEDBACK_MAX: f32 = 99f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_FEEDBACK_MIN: f32 = -99f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_FREQUENCY_MAX: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_FREQUENCY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_PHASE_180: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_PHASE_90: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_PHASE_MAX: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_PHASE_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_PHASE_NEG_180: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_PHASE_NEG_90: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_PHASE_ZERO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_WAVE_SIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_WAVE_TRIANGLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_WETDRYMIX_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXFLANGER_WETDRYMIX_MIN: f32 = 0f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXFlanger {
    pub fWetDryMix: f32,
    pub fDepth: f32,
    pub fFeedback: f32,
    pub fFrequency: f32,
    pub lWaveform: i32,
    pub fDelay: f32,
    pub lPhase: i32,
}
impl DSFXFlanger {}
impl ::std::default::Default for DSFXFlanger {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXFlanger {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXFlanger").field("fWetDryMix", &self.fWetDryMix).field("fDepth", &self.fDepth).field("fFeedback", &self.fFeedback).field("fFrequency", &self.fFrequency).field("lWaveform", &self.lWaveform).field("fDelay", &self.fDelay).field("lPhase", &self.lPhase).finish()
    }
}
impl ::std::cmp::PartialEq for DSFXFlanger {
    fn eq(&self, other: &Self) -> bool {
        self.fWetDryMix == other.fWetDryMix && self.fDepth == other.fDepth && self.fFeedback == other.fFeedback && self.fFrequency == other.fFrequency && self.lWaveform == other.lWaveform && self.fDelay == other.fDelay && self.lPhase == other.lPhase
    }
}
impl ::std::cmp::Eq for DSFXFlanger {}
unsafe impl ::windows::runtime::Abi for DSFXFlanger {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXGARGLE_RATEHZ_MAX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXGARGLE_RATEHZ_MIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXGARGLE_WAVE_SQUARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXGARGLE_WAVE_TRIANGLE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXGargle {
    pub dwRateHz: u32,
    pub dwWaveShape: u32,
}
impl DSFXGargle {}
impl ::std::default::Default for DSFXGargle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXGargle {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXGargle").field("dwRateHz", &self.dwRateHz).field("dwWaveShape", &self.dwWaveShape).finish()
    }
}
impl ::std::cmp::PartialEq for DSFXGargle {
    fn eq(&self, other: &Self) -> bool {
        self.dwRateHz == other.dwRateHz && self.dwWaveShape == other.dwWaveShape
    }
}
impl ::std::cmp::Eq for DSFXGargle {}
unsafe impl ::windows::runtime::Abi for DSFXGargle {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXI3DL2Reverb {
    pub lRoom: i32,
    pub lRoomHF: i32,
    pub flRoomRolloffFactor: f32,
    pub flDecayTime: f32,
    pub flDecayHFRatio: f32,
    pub lReflections: i32,
    pub flReflectionsDelay: f32,
    pub lReverb: i32,
    pub flReverbDelay: f32,
    pub flDiffusion: f32,
    pub flDensity: f32,
    pub flHFReference: f32,
}
impl DSFXI3DL2Reverb {}
impl ::std::default::Default for DSFXI3DL2Reverb {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXI3DL2Reverb {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXI3DL2Reverb")
            .field("lRoom", &self.lRoom)
            .field("lRoomHF", &self.lRoomHF)
            .field("flRoomRolloffFactor", &self.flRoomRolloffFactor)
            .field("flDecayTime", &self.flDecayTime)
            .field("flDecayHFRatio", &self.flDecayHFRatio)
            .field("lReflections", &self.lReflections)
            .field("flReflectionsDelay", &self.flReflectionsDelay)
            .field("lReverb", &self.lReverb)
            .field("flReverbDelay", &self.flReverbDelay)
            .field("flDiffusion", &self.flDiffusion)
            .field("flDensity", &self.flDensity)
            .field("flHFReference", &self.flHFReference)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DSFXI3DL2Reverb {
    fn eq(&self, other: &Self) -> bool {
        self.lRoom == other.lRoom
            && self.lRoomHF == other.lRoomHF
            && self.flRoomRolloffFactor == other.flRoomRolloffFactor
            && self.flDecayTime == other.flDecayTime
            && self.flDecayHFRatio == other.flDecayHFRatio
            && self.lReflections == other.lReflections
            && self.flReflectionsDelay == other.flReflectionsDelay
            && self.lReverb == other.lReverb
            && self.flReverbDelay == other.flReverbDelay
            && self.flDiffusion == other.flDiffusion
            && self.flDensity == other.flDensity
            && self.flHFReference == other.flHFReference
    }
}
impl ::std::cmp::Eq for DSFXI3DL2Reverb {}
unsafe impl ::windows::runtime::Abi for DSFXI3DL2Reverb {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXPARAMEQ_BANDWIDTH_MAX: f32 = 36f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXPARAMEQ_BANDWIDTH_MIN: f32 = 1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXPARAMEQ_CENTER_MAX: f32 = 16000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXPARAMEQ_CENTER_MIN: f32 = 80f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXPARAMEQ_GAIN_MAX: f32 = 15f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXPARAMEQ_GAIN_MIN: f32 = -15f32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXParamEq {
    pub fCenter: f32,
    pub fBandwidth: f32,
    pub fGain: f32,
}
impl DSFXParamEq {}
impl ::std::default::Default for DSFXParamEq {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXParamEq {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXParamEq").field("fCenter", &self.fCenter).field("fBandwidth", &self.fBandwidth).field("fGain", &self.fGain).finish()
    }
}
impl ::std::cmp::PartialEq for DSFXParamEq {
    fn eq(&self, other: &Self) -> bool {
        self.fCenter == other.fCenter && self.fBandwidth == other.fBandwidth && self.fGain == other.fGain
    }
}
impl ::std::cmp::Eq for DSFXParamEq {}
unsafe impl ::windows::runtime::Abi for DSFXParamEq {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXR_FAILED: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXR_LOCHARDWARE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXR_LOCSOFTWARE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXR_PRESENT: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXR_SENDLOOP: i32 = 6i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXR_UNALLOCATED: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFXR_UNKNOWN: i32 = 5i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct DSFXWavesReverb {
    pub fInGain: f32,
    pub fReverbMix: f32,
    pub fReverbTime: f32,
    pub fHighFreqRTRatio: f32,
}
impl DSFXWavesReverb {}
impl ::std::default::Default for DSFXWavesReverb {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DSFXWavesReverb {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSFXWavesReverb").field("fInGain", &self.fInGain).field("fReverbMix", &self.fReverbMix).field("fReverbTime", &self.fReverbTime).field("fHighFreqRTRatio", &self.fHighFreqRTRatio).finish()
    }
}
impl ::std::cmp::PartialEq for DSFXWavesReverb {
    fn eq(&self, other: &Self) -> bool {
        self.fInGain == other.fInGain && self.fReverbMix == other.fReverbMix && self.fReverbTime == other.fReverbTime && self.fHighFreqRTRatio == other.fHighFreqRTRatio
    }
}
impl ::std::cmp::Eq for DSFXWavesReverb {}
unsafe impl ::windows::runtime::Abi for DSFXWavesReverb {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_DEFAULT: f32 = 0.83f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MAX: f32 = 2f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DECAYHFRATIO_MIN: f32 = 0.1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_DEFAULT: f32 = 1.49f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_MAX: f32 = 20f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DECAYTIME_MIN: f32 = 0.1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DENSITY_DEFAULT: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DENSITY_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DENSITY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_DEFAULT: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_MAX: f32 = 100f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_DIFFUSION_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_DEFAULT: f32 = 5000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_MAX: f32 = 20000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_HFREFERENCE_MIN: f32 = 20f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_QUALITY_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_QUALITY_MAX: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_QUALITY_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_DEFAULT: f32 = 0.007f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MAX: f32 = 0.3f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONSDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_DEFAULT: i32 = -2602i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_MAX: u32 = 1000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REFLECTIONS_MIN: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_DEFAULT: f32 = 0.011f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_MAX: f32 = 0.1f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REVERBDELAY_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REVERB_DEFAULT: u32 = 200u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REVERB_MAX: u32 = 2000u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_REVERB_MIN: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_DEFAULT: i32 = -100i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_MAX: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOMHF_MIN: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MAX: f32 = 10f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOMROLLOFFFACTOR_MIN: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOM_DEFAULT: i32 = -1000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOM_MAX: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2REVERB_ROOM_MIN: i32 = -10000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ALLEY: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ARENA: i32 = 10i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_AUDITORIUM: i32 = 7i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_BATHROOM: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CARPETEDHALLWAY: i32 = 12i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CAVE: i32 = 9i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CITY: i32 = 17i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_CONCERTHALL: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_FOREST: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_GENERIC: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HALLWAY: i32 = 13i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_HANGAR: i32 = 11i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEHALL: i32 = 28i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LARGEROOM: i32 = 26i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_LIVINGROOM: i32 = 5i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMHALL: i32 = 27i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MEDIUMROOM: i32 = 25i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_MOUNTAINS: i32 = 18i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PADDEDCELL: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PARKINGLOT: i32 = 21i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLAIN: i32 = 20i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_PLATE: i32 = 29i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_QUARRY: i32 = 19i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_ROOM: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SEWERPIPE: i32 = 22i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_SMALLROOM: i32 = 24i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONECORRIDOR: i32 = 14i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_STONEROOM: i32 = 6i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_ENVIRONMENT_PRESET_UNDERWATER: i32 = 23i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_BRICKWALL: i32 = 5i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_CURTAIN: i32 = 7i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_DOUBLEWINDOW: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_SINGLEWINDOW: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_STONEWALL: i32 = 6i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_THICKDOOR: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_THINDOOR: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_I3DL2_MATERIAL_PRESET_WOODWALL: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_LOCHARDWARE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_LOCSOFTWARE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_DEFAULT: f32 = 0.001f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MAX: f32 = 0.999f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_HIGHFREQRTRATIO_MIN: f32 = 0.001f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_INGAIN_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_INGAIN_MAX: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_INGAIN_MIN: f32 = -96f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_DEFAULT: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_MAX: f32 = 0f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_REVERBMIX_MIN: f32 = -96f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_DEFAULT: f32 = 1000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_MAX: f32 = 3000f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSFX_WAVESREVERB_REVERBTIME_MIN: f32 = 0.001f32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE(pub i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(1i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(2i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(3i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(4i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(5i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(6i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(7i32);
pub const DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W: DSPROPERTY_DIRECTSOUNDDEVICE = DSPROPERTY_DIRECTSOUNDDEVICE(8i32);
impl ::std::convert::From<i32> for DSPROPERTY_DIRECTSOUNDDEVICE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    pub DeviceId: ::windows::runtime::GUID,
    pub DescriptionA: [super::super::super::Foundation::CHAR; 256],
    pub DescriptionW: [u16; 256],
    pub ModuleA: [super::super::super::Foundation::CHAR; 260],
    pub ModuleW: [u16; 260],
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub WaveDeviceId: u32,
    pub Devnode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA")
            .field("DeviceId", &self.DeviceId)
            .field("DescriptionA", &self.DescriptionA)
            .field("DescriptionW", &self.DescriptionW)
            .field("ModuleA", &self.ModuleA)
            .field("ModuleW", &self.ModuleW)
            .field("Type", &self.Type)
            .field("DataFlow", &self.DataFlow)
            .field("WaveDeviceId", &self.WaveDeviceId)
            .field("Devnode", &self.Devnode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceId == other.DeviceId && self.DescriptionA == other.DescriptionA && self.DescriptionW == other.DescriptionW && self.ModuleA == other.ModuleA && self.ModuleW == other.ModuleW && self.Type == other.Type && self.DataFlow == other.DataFlow && self.WaveDeviceId == other.WaveDeviceId && self.Devnode == other.Devnode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::runtime::GUID,
    pub Description: super::super::super::Foundation::PSTR,
    pub Module: super::super::super::Foundation::PSTR,
    pub Interface: super::super::super::Foundation::PSTR,
    pub WaveDeviceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA")
            .field("Type", &self.Type)
            .field("DataFlow", &self.DataFlow)
            .field("DeviceId", &self.DeviceId)
            .field("Description", &self.Description)
            .field("Module", &self.Module)
            .field("Interface", &self.Interface)
            .field("WaveDeviceId", &self.WaveDeviceId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId && self.Description == other.Description && self.Module == other.Module && self.Interface == other.Interface && self.WaveDeviceId == other.WaveDeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    pub Type: DIRECTSOUNDDEVICE_TYPE,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::runtime::GUID,
    pub Description: super::super::super::Foundation::PWSTR,
    pub Module: super::super::super::Foundation::PWSTR,
    pub Interface: super::super::super::Foundation::PWSTR,
    pub WaveDeviceId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA")
            .field("Type", &self.Type)
            .field("DataFlow", &self.DataFlow)
            .field("DeviceId", &self.DeviceId)
            .field("Description", &self.Description)
            .field("Module", &self.Module)
            .field("Interface", &self.Interface)
            .field("WaveDeviceId", &self.WaveDeviceId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId && self.Description == other.Description && self.Module == other.Module && self.Interface == other.Interface && self.WaveDeviceId == other.WaveDeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    pub Callback: ::std::option::Option<LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1>,
    pub Context: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA").field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize) && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_1_DATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    pub Callback: ::std::option::Option<LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA>,
    pub Context: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA").field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize) && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_A_DATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    pub Callback: ::std::option::Option<LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW>,
    pub Context: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA").field("Context", &self.Context).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize) && self.Context == other.Context
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_ENUMERATE_W_DATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    pub DeviceName: super::super::super::Foundation::PSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_A_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    pub DeviceName: super::super::super::Foundation::PWSTR,
    pub DataFlow: DIRECTSOUNDDEVICE_DATAFLOW,
    pub DeviceId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA").field("DeviceName", &self.DeviceName).field("DataFlow", &self.DataFlow).field("DeviceId", &self.DeviceId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DataFlow == other.DataFlow && self.DeviceId == other.DeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DSPROPERTY_DIRECTSOUNDDEVICE_WAVEDEVICEMAPPING_W_DATA {
    type Abi = Self;
}
pub const DSPROPSETID_DirectSoundDevice: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2221035394, 9708, 4561, [164, 216, 0, 192, 79, 194, 138, 202]);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSCL_EXCLUSIVE: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSCL_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSCL_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSCL_WRITEPRIMARY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_5POINT1: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_5POINT1_BACK: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_5POINT1_SURROUND: u32 = 9u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_7POINT1: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_7POINT1_SURROUND: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_7POINT1_WIDE: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_DIRECTOUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_GEOMETRY_MAX: u32 = 180u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_GEOMETRY_MIN: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_GEOMETRY_NARROW: u32 = 10u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_GEOMETRY_WIDE: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_HEADPHONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_MONO: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_QUAD: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_STEREO: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DSSPEAKER_SURROUND: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS_CERTIFIED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS_NO_VIRTUALIZATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(142082058i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DS_UNCERTIFIED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_AUDIOMODE: u32 = 3840u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_AUDIOQU: u32 = 117440512u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_AUDIOSMP: u32 = 939524096u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_CAP_AUD12Bits: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_CAP_AUD16Bits: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_DVSD_NTSC_FRAMESIZE: i32 = 120000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_DVSD_PAL_FRAMESIZE: i32 = 144000i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_HD: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_NTSC: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_NTSCPAL: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_PAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_SD: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_SL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_SMCHN: u32 = 57344u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const DV_STYPE: u32 = 2031616u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[inline]
pub unsafe fn DirectSoundCaptureCreate<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcguiddevice: *const ::windows::runtime::GUID, ppdsc: *mut ::std::option::Option<IDirectSoundCapture>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureCreate(pcguiddevice: *const ::windows::runtime::GUID, ppdsc: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCaptureCreate(::std::mem::transmute(pcguiddevice), ::std::mem::transmute(ppdsc), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[inline]
pub unsafe fn DirectSoundCaptureCreate8<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcguiddevice: *const ::windows::runtime::GUID, ppdsc8: *mut ::std::option::Option<IDirectSoundCapture>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureCreate8(pcguiddevice: *const ::windows::runtime::GUID, ppdsc8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCaptureCreate8(::std::mem::transmute(pcguiddevice), ::std::mem::transmute(ppdsc8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateA(pdsenumcallback: ::std::option::Option<LPDSENUMCALLBACKA>, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureEnumerateA(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCaptureEnumerateA(::std::mem::transmute(pdsenumcallback), ::std::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundCaptureEnumerateW(pdsenumcallback: ::std::option::Option<LPDSENUMCALLBACKW>, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCaptureEnumerateW(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCaptureEnumerateW(::std::mem::transmute(pdsenumcallback), ::std::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[inline]
pub unsafe fn DirectSoundCreate<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcguiddevice: *const ::windows::runtime::GUID, ppds: *mut ::std::option::Option<IDirectSound>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCreate(pcguiddevice: *const ::windows::runtime::GUID, ppds: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCreate(::std::mem::transmute(pcguiddevice), ::std::mem::transmute(ppds), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[inline]
pub unsafe fn DirectSoundCreate8<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pcguiddevice: *const ::windows::runtime::GUID, ppds8: *mut ::std::option::Option<IDirectSound8>, punkouter: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundCreate8(pcguiddevice: *const ::windows::runtime::GUID, ppds8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundCreate8(::std::mem::transmute(pcguiddevice), ::std::mem::transmute(ppds8), punkouter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundEnumerateA(pdsenumcallback: ::std::option::Option<LPDSENUMCALLBACKA>, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundEnumerateA(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectSoundEnumerateA(::std::mem::transmute(pdsenumcallback), ::std::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectSoundEnumerateW(pdsenumcallback: ::std::option::Option<LPDSENUMCALLBACKW>, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundEnumerateW(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DirectSoundEnumerateW(::std::mem::transmute(pdsenumcallback), ::std::mem::transmute(pcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`, `Win32_Media_Multimedia`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Multimedia"))]
#[inline]
pub unsafe fn DirectSoundFullDuplexCreate<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, Param9: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(
    pcguidcapturedevice: *const ::windows::runtime::GUID,
    pcguidrenderdevice: *const ::windows::runtime::GUID,
    pcdscbufferdesc: *const DSCBUFFERDESC,
    pcdsbufferdesc: *const DSBUFFERDESC,
    hwnd: Param4,
    dwlevel: u32,
    ppdsfd: *mut ::std::option::Option<IDirectSoundFullDuplex>,
    ppdscbuffer8: *mut ::std::option::Option<IDirectSoundCaptureBuffer8>,
    ppdsbuffer8: *mut ::std::option::Option<IDirectSoundBuffer8>,
    punkouter: Param9,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DirectSoundFullDuplexCreate(pcguidcapturedevice: *const ::windows::runtime::GUID, pcguidrenderdevice: *const ::windows::runtime::GUID, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, ppdsfd: *mut ::windows::runtime::RawPtr, ppdscbuffer8: *mut ::windows::runtime::RawPtr, ppdsbuffer8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        DirectSoundFullDuplexCreate(
            ::std::mem::transmute(pcguidcapturedevice),
            ::std::mem::transmute(pcguidrenderdevice),
            ::std::mem::transmute(pcdscbufferdesc),
            ::std::mem::transmute(pcdsbufferdesc),
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwlevel),
            ::std::mem::transmute(ppdsfd),
            ::std::mem::transmute(ppdscbuffer8),
            ::std::mem::transmute(ppdsbuffer8),
            punkouter.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EAudioConstriction(pub i32);
pub const eAudioConstrictionOff: EAudioConstriction = EAudioConstriction(0i32);
pub const eAudioConstriction48_16: EAudioConstriction = EAudioConstriction(1i32);
pub const eAudioConstriction44_16: EAudioConstriction = EAudioConstriction(2i32);
pub const eAudioConstriction14_14: EAudioConstriction = EAudioConstriction(3i32);
pub const eAudioConstrictionMute: EAudioConstriction = EAudioConstriction(4i32);
impl ::std::convert::From<i32> for EAudioConstriction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAudioConstriction {
    type Abi = Self;
}
pub type FNAPONOTIFICATIONCALLBACK = unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_INSTRUMENT_DRUMS: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_RGN_OPTION_SELFNONEXCLUSIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_WAVELINK_MULTICHANNEL: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_WAVELINK_PHASE_MASTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_WSMP_NO_COMPRESSION: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const F_WSMP_NO_TRUNCATION: i32 = 1i32;
pub const GUID_All_Objects: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2853260773, 49762, 16745, [161, 200, 35, 214, 152, 204, 115, 181]);
pub const GUID_DMUS_PROP_DLS1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259687, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_DLS2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4047870437, 18057, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_Effects: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3450394129, 26698, 4562, [135, 30, 0, 96, 8, 147, 177, 189]);
pub const GUID_DMUS_PROP_GM_Hardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259684, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_GS_Capable: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1687595938, 25008, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_GS_Hardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259685, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_INSTRUMENT2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2254426994, 40807, 4562, [135, 42, 0, 96, 8, 147, 177, 189]);
pub const GUID_DMUS_PROP_LegacyCaps: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3483880898, 161, 4562, [170, 213, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_MemorySize: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259688, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_SampleMemorySize: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259688, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_SamplePlaybackRate: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(714209043, 42175, 4562, [187, 223, 0, 96, 8, 51, 219, 216]);
pub const GUID_DMUS_PROP_SetSynthSink: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(171596709, 14262, 4562, [185, 249, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_SinkUsesDSound: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3189803095, 35154, 4562, [186, 28, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_SynthSink_DSOUND: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(178878532, 51319, 4561, [135, 12, 0, 96, 8, 147, 177, 189]);
pub const GUID_DMUS_PROP_SynthSink_WAVE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(178878533, 51319, 4561, [135, 12, 0, 96, 8, 147, 177, 189]);
pub const GUID_DMUS_PROP_Volume: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4276071973, 58478, 4561, [170, 206, 0, 0, 248, 117, 172, 18]);
pub const GUID_DMUS_PROP_WavesReverb: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(80434722, 13029, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_WriteLatency: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(646582176, 24818, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_WritePeriod: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(646582177, 24818, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_XG_Capable: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1687595937, 25008, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
pub const GUID_DMUS_PROP_XG_Hardware: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(395259686, 50020, 4561, [167, 96, 0, 0, 248, 117, 172, 18]);
pub const GUID_DSCFX_CLASS_AEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3214294400, 50521, 4560, [138, 43, 0, 160, 201, 37, 90, 193]);
pub const GUID_DSCFX_CLASS_NS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3766456383, 25341, 20064, [140, 221, 222, 167, 35, 102, 101, 181]);
pub const GUID_DSCFX_MS_AEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3454777625, 14234, 18570, [135, 101, 245, 60, 253, 54, 222, 64]);
pub const GUID_DSCFX_MS_NS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(298174267, 26345, 19361, [160, 186, 232, 20, 198, 238, 217, 45]);
pub const GUID_DSCFX_SYSTEM_AEC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(472040813, 39033, 20315, [163, 137, 39, 153, 109, 220, 40, 16]);
pub const GUID_DSCFX_SYSTEM_NS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1521518638, 29300, 17686, [135, 125, 78, 238, 153, 186, 79, 208]);
pub const GUID_DSFX_STANDARD_CHORUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4024853148, 33271, 17025, [189, 145, 201, 214, 4, 169, 90, 246]);
pub const GUID_DSFX_STANDARD_COMPRESSOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4009828217, 16384, 16493, [135, 175, 191, 251, 63, 195, 157, 87]);
pub const GUID_DSFX_STANDARD_DISTORTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4010888336, 52509, 18510, [150, 229, 9, 207, 175, 145, 42, 33]);
pub const GUID_DSFX_STANDARD_ECHO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4013855532, 54283, 20305, [140, 207, 63, 152, 241, 178, 157, 93]);
pub const GUID_DSFX_STANDARD_FLANGER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023008658, 57304, 18034, [166, 3, 116, 32, 137, 75, 173, 152]);
pub const GUID_DSFX_STANDARD_GARGLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3674046992, 22289, 19345, [159, 227, 247, 91, 122, 226, 121, 191]);
pub const GUID_DSFX_STANDARD_I3DL2REVERB: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4019740273, 54727, 17108, [186, 77, 45, 7, 62, 46, 150, 244]);
pub const GUID_DSFX_STANDARD_PARAMEQ: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(302837129, 15348, 16755, [161, 50, 60, 180, 6, 207, 50, 49]);
pub const GUID_DSFX_WAVES_REVERB: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2281439848, 39509, 17248, [149, 170, 0, 74, 29, 157, 226, 108]);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[inline]
pub unsafe fn GetDeviceID(pguidsrc: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceID(pguidsrc: *const ::windows::runtime::GUID, pguiddest: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetDeviceID(::std::mem::transmute(pguidsrc), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IApoAcousticEchoCancellation(pub ::windows::runtime::IUnknown);
impl IApoAcousticEchoCancellation {}
unsafe impl ::windows::runtime::Interface for IApoAcousticEchoCancellation {
    type Vtable = IApoAcousticEchoCancellation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(624449369, 12854, 16641, [169, 67, 37, 105, 61, 251, 93, 45]);
}
impl ::std::convert::From<IApoAcousticEchoCancellation> for ::windows::runtime::IUnknown {
    fn from(value: IApoAcousticEchoCancellation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IApoAcousticEchoCancellation> for ::windows::runtime::IUnknown {
    fn from(value: &IApoAcousticEchoCancellation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IApoAcousticEchoCancellation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IApoAcousticEchoCancellation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAcousticEchoCancellation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IApoAuxiliaryInputConfiguration(pub ::windows::runtime::IUnknown);
impl IApoAuxiliaryInputConfiguration {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn AddAuxiliaryInput(&self, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputid), ::std::mem::transmute(cbdatasize), ::std::mem::transmute(pbydata), ::std::mem::transmute(pinputconnection)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn RemoveAuxiliaryInput(&self, dwinputid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputid)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn IsInputFormatSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioMediaType>>(&self, prequestedinputformat: Param0) -> ::windows::runtime::Result<IAudioMediaType> {
        let mut result__: <IAudioMediaType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), prequestedinputformat.into_param().abi(), &mut result__).from_abi::<IAudioMediaType>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IApoAuxiliaryInputConfiguration {
    type Vtable = IApoAuxiliaryInputConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1290472107, 64025, 18669, [168, 87, 135, 119, 26, 225, 183, 104]);
}
impl ::std::convert::From<IApoAuxiliaryInputConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: IApoAuxiliaryInputConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IApoAuxiliaryInputConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &IApoAuxiliaryInputConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IApoAuxiliaryInputConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IApoAuxiliaryInputConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAuxiliaryInputConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const ::std::mem::ManuallyDrop<APO_CONNECTION_DESCRIPTOR>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequestedinputformat: ::windows::runtime::RawPtr, ppsupportedinputformat: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IApoAuxiliaryInputRT(pub ::windows::runtime::IUnknown);
impl IApoAuxiliaryInputRT {
    #[cfg(feature = "Win32_System_RemoteDesktop")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_System_RemoteDesktop`*"]
    pub unsafe fn AcceptInput(&self, dwinputid: u32, pinputconnection: *const super::super::super::System::RemoteDesktop::APO_CONNECTION_PROPERTY) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputid), ::std::mem::transmute(pinputconnection)))
    }
}
unsafe impl ::windows::runtime::Interface for IApoAuxiliaryInputRT {
    type Vtable = IApoAuxiliaryInputRT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4166090908, 49527, 18848, [177, 178, 182, 111, 1, 121, 67, 171]);
}
impl ::std::convert::From<IApoAuxiliaryInputRT> for ::windows::runtime::IUnknown {
    fn from(value: IApoAuxiliaryInputRT) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IApoAuxiliaryInputRT> for ::windows::runtime::IUnknown {
    fn from(value: &IApoAuxiliaryInputRT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IApoAuxiliaryInputRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IApoAuxiliaryInputRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAuxiliaryInputRT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_RemoteDesktop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputid: u32, pinputconnection: *const super::super::super::System::RemoteDesktop::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_System_RemoteDesktop"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioDeviceModulesClient(pub ::windows::runtime::IUnknown);
impl IAudioDeviceModulesClient {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAudioDeviceModulesManager<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, paudiodevicemodulesmanager: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), paudiodevicemodulesmanager.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioDeviceModulesClient {
    type Vtable = IAudioDeviceModulesClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2566094252, 53430, 18933, [137, 106, 170, 77, 22, 154, 76, 72]);
}
impl ::std::convert::From<IAudioDeviceModulesClient> for ::windows::runtime::IUnknown {
    fn from(value: IAudioDeviceModulesClient) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioDeviceModulesClient> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioDeviceModulesClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioDeviceModulesClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioDeviceModulesClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paudiodevicemodulesmanager: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioMediaType(pub ::windows::runtime::IUnknown);
impl IAudioMediaType {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn IsCompressedFormat(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioMediaType>>(&self, piaudiotype: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), piaudiotype.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn GetAudioFormat(&self) -> *mut super::super::Multimedia::WAVEFORMATEX {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetUncompressedAudioFormat(&self) -> ::windows::runtime::Result<UNCOMPRESSEDAUDIOFORMAT> {
        let mut result__: <UNCOMPRESSEDAUDIOFORMAT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<UNCOMPRESSEDAUDIOFORMAT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioMediaType {
    type Vtable = IAudioMediaType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1318682483, 46879, 18328, [135, 59, 237, 125, 252, 241, 91, 77]);
}
impl ::std::convert::From<IAudioMediaType> for ::windows::runtime::IUnknown {
    fn from(value: IAudioMediaType) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioMediaType> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioMediaType) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioMediaType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioMediaType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMediaType_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piaudiotype: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut super::super::Multimedia::WAVEFORMATEX,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioProcessingObject(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObject {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetLatency(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRegistrationProperties(&self) -> ::windows::runtime::Result<*mut APO_REG_PROPERTIES> {
        let mut result__: <*mut APO_REG_PROPERTIES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut APO_REG_PROPERTIES>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Initialize(&self, cbdatasize: u32, pbydata: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbdatasize), ::std::mem::transmute(pbydata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn IsInputFormatSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioMediaType>, Param1: ::windows::runtime::IntoParam<'a, IAudioMediaType>>(&self, poppositeformat: Param0, prequestedinputformat: Param1) -> ::windows::runtime::Result<IAudioMediaType> {
        let mut result__: <IAudioMediaType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), poppositeformat.into_param().abi(), prequestedinputformat.into_param().abi(), &mut result__).from_abi::<IAudioMediaType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn IsOutputFormatSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioMediaType>, Param1: ::windows::runtime::IntoParam<'a, IAudioMediaType>>(&self, poppositeformat: Param0, prequestedoutputformat: Param1) -> ::windows::runtime::Result<IAudioMediaType> {
        let mut result__: <IAudioMediaType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), poppositeformat.into_param().abi(), prequestedoutputformat.into_param().abi(), &mut result__).from_abi::<IAudioMediaType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetInputChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObject {
    type Vtable = IAudioProcessingObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4252969769, 9424, 19292, [177, 119, 89, 44, 57, 249, 202, 16]);
}
impl ::std::convert::From<IAudioProcessingObject> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObject) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioProcessingObject> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioProcessingObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioProcessingObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioProcessingObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptime: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbdatasize: u32, pbydata: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poppositeformat: ::windows::runtime::RawPtr, prequestedinputformat: ::windows::runtime::RawPtr, ppsupportedinputformat: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poppositeformat: ::windows::runtime::RawPtr, prequestedoutputformat: ::windows::runtime::RawPtr, ppsupportedoutputformat: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pu32channelcount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioProcessingObjectConfiguration(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectConfiguration {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn LockForProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(u32numinputconnections), ::std::mem::transmute(ppinputconnections), ::std::mem::transmute(u32numoutputconnections), ::std::mem::transmute(ppoutputconnections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn UnlockForProcess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectConfiguration {
    type Vtable = IAudioProcessingObjectConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(241096709, 43942, 18883, [143, 154, 43, 140, 136, 156, 79, 168]);
}
impl ::std::convert::From<IAudioProcessingObjectConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioProcessingObjectConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioProcessingObjectConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioProcessingObjectConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioProcessingObjectConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioProcessingObjectRT(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectRT {
    #[cfg(feature = "Win32_System_RemoteDesktop")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_System_RemoteDesktop`*"]
    pub unsafe fn APOProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const super::super::super::System::RemoteDesktop::APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut super::super::super::System::RemoteDesktop::APO_CONNECTION_PROPERTY) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(u32numinputconnections), ::std::mem::transmute(ppinputconnections), ::std::mem::transmute(u32numoutputconnections), ::std::mem::transmute(ppoutputconnections)))
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CalcInputFrames(&self, u32outputframecount: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(u32outputframecount)))
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CalcOutputFrames(&self, u32inputframecount: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(u32inputframecount)))
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectRT {
    type Vtable = IAudioProcessingObjectRT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2652727917, 56764, 20117, [164, 199, 173, 100, 186, 55, 132, 108]);
}
impl ::std::convert::From<IAudioProcessingObjectRT> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectRT) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioProcessingObjectRT> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioProcessingObjectRT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioProcessingObjectRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioProcessingObjectRT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectRT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_RemoteDesktop")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32numinputconnections: u32, ppinputconnections: *const *const super::super::super::System::RemoteDesktop::APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut super::super::super::System::RemoteDesktop::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_System_RemoteDesktop"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32outputframecount: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32inputframecount: u32) -> u32,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioProcessingObjectVBR(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectVBR {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CalcMaxInputFrames(&self, u32maxoutputframecount: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(u32maxoutputframecount), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CalcMaxOutputFrames(&self, u32maxinputframecount: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(u32maxinputframecount), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectVBR {
    type Vtable = IAudioProcessingObjectVBR_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2074205071, 30893, 18893, [149, 145, 247, 157, 128, 161, 124, 129]);
}
impl ::std::convert::From<IAudioProcessingObjectVBR> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectVBR) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioProcessingObjectVBR> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioProcessingObjectVBR) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioProcessingObjectVBR {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioProcessingObjectVBR {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectVBR_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioSystemEffects(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffects {}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffects {
    type Vtable = IAudioSystemEffects_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1604325159, 44502, 18842, [138, 157, 107, 152, 82, 31, 167, 91]);
}
impl ::std::convert::From<IAudioSystemEffects> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffects) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioSystemEffects> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSystemEffects) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSystemEffects {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSystemEffects {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioSystemEffects2(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffects2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn GetEffectsList<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, ppeffectsids: *mut *mut ::windows::runtime::GUID, pceffects: *mut u32, event: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppeffectsids), ::std::mem::transmute(pceffects), event.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffects2 {
    type Vtable = IAudioSystemEffects2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3137247698, 29750, 17614, [158, 14, 77, 137, 175, 191, 255, 86]);
}
impl ::std::convert::From<IAudioSystemEffects2> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffects2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioSystemEffects2> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSystemEffects2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IAudioSystemEffects2> for IAudioSystemEffects {
    fn from(value: IAudioSystemEffects2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAudioSystemEffects2> for IAudioSystemEffects {
    fn from(value: &IAudioSystemEffects2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSystemEffects> for IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSystemEffects> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSystemEffects> for &IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSystemEffects> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppeffectsids: *mut *mut ::windows::runtime::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAudioSystemEffectsCustomFormats(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffectsCustomFormats {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetFormatCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetFormat(&self, nformat: u32) -> ::windows::runtime::Result<IAudioMediaType> {
        let mut result__: <IAudioMediaType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(nformat), &mut result__).from_abi::<IAudioMediaType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatRepresentation(&self, nformat: u32) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(nformat), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffectsCustomFormats {
    type Vtable = IAudioSystemEffectsCustomFormats_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2971102772, 47999, 20229, [190, 189, 27, 24, 165, 52, 224, 151]);
}
impl ::std::convert::From<IAudioSystemEffectsCustomFormats> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffectsCustomFormats) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioSystemEffectsCustomFormats> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSystemEffectsCustomFormats) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSystemEffectsCustomFormats {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSystemEffectsCustomFormats {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsCustomFormats_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nformat: u32, ppformat: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nformat: u32, ppwstrformatrep: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusic(pub ::windows::runtime::IUnknown);
impl IDirectMusic {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(pportcaps)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CreateMusicBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::std::option::Option<IDirectMusicBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbufferdesc), ::std::mem::transmute(ppbuffer), punkouter.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn CreatePort<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, rclsidport: *const ::windows::runtime::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::std::option::Option<IDirectMusicPort>, punkouter: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsidport), ::std::mem::transmute(pportparams), ::std::mem::transmute(ppport), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(lpclockinfo)).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::runtime::GUID, ppreferenceclock: *mut ::std::option::Option<super::super::super::Graphics::DirectShow::IReferenceClock>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidclock), ::std::mem::transmute(ppreferenceclock)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rguidclock)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSound>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, pdirectsound: Param0, hwnd: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pdirectsound.into_param().abi(), hwnd.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusic {
    type Vtable = IDirectMusic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1698042202, 31533, 4562, [186, 24, 0, 0, 248, 117, 172, 18]);
}
impl ::std::convert::From<IDirectMusic> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusic) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusic> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusic) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusic {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsidport: *const ::windows::runtime::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidclock: *mut ::windows::runtime::GUID, ppreferenceclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidclock: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidport: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusic8(pub ::windows::runtime::IUnknown);
impl IDirectMusic8 {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn EnumPort(&self, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(pportcaps)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CreateMusicBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::std::option::Option<IDirectMusicBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbufferdesc), ::std::mem::transmute(ppbuffer), punkouter.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn CreatePort<'a, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, rclsidport: *const ::windows::runtime::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::std::option::Option<IDirectMusicPort>, punkouter: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsidport), ::std::mem::transmute(pportparams), ::std::mem::transmute(ppport), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn EnumMasterClock(&self, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(lpclockinfo)).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn GetMasterClock(&self, pguidclock: *mut ::windows::runtime::GUID, ppreferenceclock: *mut ::std::option::Option<super::super::super::Graphics::DirectShow::IReferenceClock>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidclock), ::std::mem::transmute(ppreferenceclock)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMasterClock(&self, rguidclock: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rguidclock)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDefaultPort(&self, pguidport: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSound>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, pdirectsound: Param0, hwnd: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pdirectsound.into_param().abi(), hwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn SetExternalMasterClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::DirectShow::IReferenceClock>>(&self, pclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusic8 {
    type Vtable = IDirectMusic8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(758524407, 33085, 18745, [133, 8, 240, 92, 107, 117, 253, 151]);
}
impl ::std::convert::From<IDirectMusic8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusic8) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusic8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusic8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusic8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusic8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDirectMusic8> for IDirectMusic {
    fn from(value: IDirectMusic8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectMusic8> for IDirectMusic {
    fn from(value: &IDirectMusic8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectMusic> for IDirectMusic8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectMusic> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectMusic> for &IDirectMusic8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectMusic> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusic8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsidport: *const ::windows::runtime::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidclock: *mut ::windows::runtime::GUID, ppreferenceclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidclock: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidport: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclock: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicBuffer(pub ::windows::runtime::IUnknown);
impl IDirectMusicBuffer {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn TotalTime(&self, prttime: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(prttime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PackStructured(&self, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(rt), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(dwchannelmessage)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PackUnstructured(&self, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(rt), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(cb), ::std::mem::transmute(lpb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn ResetReadPtr(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetNextEvent(&self, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(prt), ::std::mem::transmute(pdwchannelgroup), ::std::mem::transmute(pdwlength), ::std::mem::transmute(ppdata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRawBufferPtr(&self, ppdata: *mut *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppdata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetStartTime(&self, prt: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(prt)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetUsedBytes(&self, pcb: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetMaxBytes(&self, pcb: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetBufferFormat(&self, pguidformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetStartTime(&self, rt: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(rt)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetUsedBytes(&self, cb: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(cb)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicBuffer {
    type Vtable = IDirectMusicBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497912, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::std::convert::From<IDirectMusicBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicBuffer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prttime: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdata: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prt: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcb: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcb: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cb: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicCollection(pub ::windows::runtime::IUnknown);
impl IDirectMusicCollection {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetInstrument(&self, dwpatch: u32) -> ::windows::runtime::Result<IDirectMusicInstrument> {
        let mut result__: <IDirectMusicInstrument as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwpatch), &mut result__).from_abi::<IDirectMusicInstrument>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn EnumInstrument<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, dwindex: u32, pdwpatch: *mut u32, pwszname: Param2, dwnamelen: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwindex), ::std::mem::transmute(pdwpatch), pwszname.into_param().abi(), ::std::mem::transmute(dwnamelen)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicCollection {
    type Vtable = IDirectMusicCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497916, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::std::convert::From<IDirectMusicCollection> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpatch: u32, ppinstrument: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pdwpatch: *mut u32, pwszname: super::super::super::Foundation::PWSTR, dwnamelen: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicDownload(pub ::windows::runtime::IUnknown);
impl IDirectMusicDownload {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetBuffer(&self, ppvbuffer: *mut *mut ::std::ffi::c_void, pdwsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppvbuffer), ::std::mem::transmute(pdwsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicDownload {
    type Vtable = IDirectMusicDownload_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497915, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::std::convert::From<IDirectMusicDownload> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicDownload) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicDownload> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicDownload) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicDownload {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicDownload {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownload_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppvbuffer: *mut *mut ::std::ffi::c_void, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicDownloadedInstrument(pub ::windows::runtime::IUnknown);
impl IDirectMusicDownloadedInstrument {}
unsafe impl ::windows::runtime::Interface for IDirectMusicDownloadedInstrument {
    type Vtable = IDirectMusicDownloadedInstrument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497918, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::std::convert::From<IDirectMusicDownloadedInstrument> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicDownloadedInstrument) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicDownloadedInstrument> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicDownloadedInstrument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicDownloadedInstrument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicDownloadedInstrument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicDownloadedInstrument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicInstrument(pub ::windows::runtime::IUnknown);
impl IDirectMusicInstrument {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPatch(&self, pdwpatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwpatch)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetPatch(&self, dwpatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwpatch)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicInstrument {
    type Vtable = IDirectMusicInstrument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497917, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::std::convert::From<IDirectMusicInstrument> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicInstrument) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicInstrument> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicInstrument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicInstrument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicInstrument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicInstrument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpatch: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicPort(pub ::windows::runtime::IUnknown);
impl IDirectMusicPort {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PlayBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicBuffer>>(&self, pbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetReadNotificationHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Read<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicBuffer>>(&self, pbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn DownloadInstrument<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicInstrument>>(&self, pinstrument: Param0, ppdownloadedinstrument: *mut ::std::option::Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pinstrument.into_param().abi(), ::std::mem::transmute(ppdownloadedinstrument), ::std::mem::transmute(pnoteranges), ::std::mem::transmute(dwnumnoteranges)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn UnloadInstrument<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicDownloadedInstrument>>(&self, pdownloadedinstrument: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pdownloadedinstrument.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn GetLatencyClock(&self) -> ::windows::runtime::Result<super::super::super::Graphics::DirectShow::IReferenceClock> {
        let mut result__: <super::super::super::Graphics::DirectShow::IReferenceClock as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::DirectShow::IReferenceClock>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstats)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pportcaps)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`, `Win32_System_IO`*"]
    pub unsafe fn DeviceIoControl(&self, dwiocontrolcode: u32, lpinbuffer: *mut ::std::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::std::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwiocontrolcode), ::std::mem::transmute(lpinbuffer), ::std::mem::transmute(ninbuffersize), ::std::mem::transmute(lpoutbuffer), ::std::mem::transmute(noutbuffersize), ::std::mem::transmute(lpbytesreturned), ::std::mem::transmute(lpoverlapped)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetNumChannelGroups(&self, dwchannelgroups: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchannelgroups)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetNumChannelGroups(&self, pdwchannelgroups: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwchannelgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, factive: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), factive.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(dwchannel), ::std::mem::transmute(dwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(dwchannel), ::std::mem::transmute(pdwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSound>, Param1: ::windows::runtime::IntoParam<'a, IDirectSoundBuffer>>(&self, pdirectsound: Param0, pdirectsoundbuffer: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pdirectsound.into_param().abi(), pdirectsoundbuffer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::super::Multimedia::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwaveformatex), ::std::mem::transmute(pdwwaveformatexsize), ::std::mem::transmute(pdwbuffersize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicPort {
    type Vtable = IDirectMusicPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(150132937, 14274, 4562, [185, 249, 0, 0, 248, 117, 172, 18]);
}
impl ::std::convert::From<IDirectMusicPort> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicPort) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicPort> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicPort) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hevent: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinstrument: ::windows::runtime::RawPtr, ppdownloadedinstrument: *mut ::windows::runtime::RawPtr, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdownloadedinstrument: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwiocontrolcode: u32, lpinbuffer: *mut ::std::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::std::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_IO")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroups: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwchannelgroups: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factive: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, pdirectsoundbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwaveformatex: *mut super::super::Multimedia::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicPortDownload(pub ::windows::runtime::IUnknown);
impl IDirectMusicPortDownload {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetBuffer(&self, dwdlid: u32) -> ::windows::runtime::Result<IDirectMusicDownload> {
        let mut result__: <IDirectMusicDownload as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdlid), &mut result__).from_abi::<IDirectMusicDownload>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn AllocateBuffer(&self, dwsize: u32) -> ::windows::runtime::Result<IDirectMusicDownload> {
        let mut result__: <IDirectMusicDownload as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsize), &mut result__).from_abi::<IDirectMusicDownload>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDLId(&self, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwstartdlid), ::std::mem::transmute(dwcount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwappend)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Download<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicDownload>>(&self, pidmdownload: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pidmdownload.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Unload<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicDownload>>(&self, pidmdownload: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pidmdownload.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicPortDownload {
    type Vtable = IDirectMusicPortDownload_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3534497914, 45979, 4561, [135, 4, 0, 96, 8, 147, 177, 189]);
}
impl ::std::convert::From<IDirectMusicPortDownload> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicPortDownload) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicPortDownload> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicPortDownload) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicPortDownload {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicPortDownload {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicPortDownload_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdlid: u32, ppidmdownload: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsize: u32, ppidmdownload: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwappend: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidmdownload: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidmdownload: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicSynth(pub ::windows::runtime::IUnknown);
impl IDirectMusicSynth {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pportparams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::std::ffi::c_void, pbfree: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(phdownload), ::std::mem::transmute(pvdata), ::std::mem::transmute(pbfree)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Unload<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hdownload: Param0, lpfreehandle: isize, huserdata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hdownload.into_param().abi(), ::std::mem::transmute(lpfreehandle), huserdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rt), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(cbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstats)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcaps)).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::DirectShow::IReferenceClock>>(&self, pclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn GetLatencyClock(&self) -> ::windows::runtime::Result<super::super::super::Graphics::DirectShow::IReferenceClock> {
        let mut result__: <super::super::super::Graphics::DirectShow::IReferenceClock as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::DirectShow::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetSynthSink<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicSynthSink>>(&self, psynthsink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), psynthsink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer), ::std::mem::transmute(dwlength), ::std::mem::transmute(llposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(dwchannel), ::std::mem::transmute(dwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(dwchannel), ::std::mem::transmute(pdwpriority)).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::super::Multimedia::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwaveformatex), ::std::mem::transmute(pdwwaveformatexsize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwappend)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicSynth {
    type Vtable = IDirectMusicSynth_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(159528545, 23685, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
}
impl ::std::convert::From<IDirectMusicSynth> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicSynth) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicSynth> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicSynth) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicSynth {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicSynth {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwgroups: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::std::ffi::c_void, pbfree: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclock: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psynthsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwaveformatex: *mut super::super::Multimedia::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwappend: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicSynth8(pub ::windows::runtime::IUnknown);
impl IDirectMusicSynth8 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Open(&self, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pportparams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetNumChannelGroups(&self, dwgroups: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Download(&self, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::std::ffi::c_void, pbfree: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(phdownload), ::std::mem::transmute(pvdata), ::std::mem::transmute(pbfree)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Unload<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, hdownload: Param0, lpfreehandle: isize, huserdata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hdownload.into_param().abi(), ::std::mem::transmute(lpfreehandle), huserdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PlayBuffer(&self, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rt), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(cbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRunningStats(&self, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstats)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPortCaps(&self, pcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcaps)).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::DirectShow::IReferenceClock>>(&self, pclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn GetLatencyClock(&self) -> ::windows::runtime::Result<super::super::super::Graphics::DirectShow::IReferenceClock> {
        let mut result__: <super::super::super::Graphics::DirectShow::IReferenceClock as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::DirectShow::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetSynthSink<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicSynthSink>>(&self, psynthsink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), psynthsink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Render(&self, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbuffer), ::std::mem::transmute(dwlength), ::std::mem::transmute(llposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(dwchannel), ::std::mem::transmute(dwpriority)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetChannelPriority(&self, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(dwchannel), ::std::mem::transmute(pdwpriority)).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn GetFormat(&self, pwaveformatex: *mut super::super::Multimedia::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwaveformatex), ::std::mem::transmute(pdwwaveformatexsize)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAppend(&self, pdwappend: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwappend)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn PlayVoice(&self, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rt),
            ::std::mem::transmute(dwvoiceid),
            ::std::mem::transmute(dwchannelgroup),
            ::std::mem::transmute(dwchannel),
            ::std::mem::transmute(dwdlid),
            ::std::mem::transmute(prpitch),
            ::std::mem::transmute(vrvolume),
            ::std::mem::transmute(stvoicestart),
            ::std::mem::transmute(stloopstart),
            ::std::mem::transmute(stloopend),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn StopVoice(&self, rt: i64, dwvoiceid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(rt), ::std::mem::transmute(dwvoiceid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn GetVoiceState(&self, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwvoice), ::std::mem::transmute(cbvoice), ::std::mem::transmute(dwvoicestate)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Refresh(&self, dwdownloadid: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdownloadid), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn AssignChannelToBuses(&self, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchannelgroup), ::std::mem::transmute(dwchannel), ::std::mem::transmute(pdwbuses), ::std::mem::transmute(cbuses)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicSynth8 {
    type Vtable = IDirectMusicSynth8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1405793829, 10001, 19615, [157, 231, 27, 127, 146, 95, 111, 200]);
}
impl ::std::convert::From<IDirectMusicSynth8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicSynth8) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicSynth8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicSynth8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDirectMusicSynth8> for IDirectMusicSynth {
    fn from(value: IDirectMusicSynth8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectMusicSynth8> for IDirectMusicSynth {
    fn from(value: &IDirectMusicSynth8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectMusicSynth> for IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectMusicSynth> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectMusicSynth> for &IDirectMusicSynth8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectMusicSynth> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynth8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwgroups: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::std::ffi::c_void, pbfree: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstats: *mut DMUS_SYNTHSTATS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcaps: *mut DMUS_PORTCAPS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclock: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psynthsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwaveformatex: *mut super::super::Multimedia::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwappend: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rt: i64, dwvoiceid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdownloadid: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicSynthSink(pub ::windows::runtime::IUnknown);
impl IDirectMusicSynthSink {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Init<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectMusicSynth>>(&self, psynth: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), psynth.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn SetMasterClock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::DirectShow::IReferenceClock>>(&self, pclock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pclock.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_DirectShow")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_DirectShow`*"]
    pub unsafe fn GetLatencyClock(&self) -> ::windows::runtime::Result<super::super::super::Graphics::DirectShow::IReferenceClock> {
        let mut result__: <super::super::super::Graphics::DirectShow::IReferenceClock as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::DirectShow::IReferenceClock>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn Activate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SampleToRefTime(&self, llsampletime: i64, prftime: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(llsampletime), ::std::mem::transmute(prftime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn RefTimeToSample(&self, rftime: i64, pllsampletime: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rftime), ::std::mem::transmute(pllsampletime)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetDirectSound<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSound>, Param1: ::windows::runtime::IntoParam<'a, IDirectSoundBuffer>>(&self, pdirectsound: Param0, pdirectsoundbuffer: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pdirectsound.into_param().abi(), pdirectsoundbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDesiredBufferSize(&self, pdwbuffersizeinsamples: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwbuffersizeinsamples)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicSynthSink {
    type Vtable = IDirectMusicSynthSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(159528547, 23685, 4562, [175, 166, 0, 170, 0, 36, 216, 182]);
}
impl ::std::convert::From<IDirectMusicSynthSink> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicSynthSink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicSynthSink> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicSynthSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicSynthSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicSynthSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicSynthSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psynth: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclock: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    #[cfg(feature = "Win32_Graphics_DirectShow")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclock: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_DirectShow"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, llsampletime: i64, prftime: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rftime: i64, pllsampletime: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, pdirectsoundbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwbuffersizeinsamples: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectMusicThru(pub ::windows::runtime::IUnknown);
impl IDirectMusicThru {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn ThruChannel<'a, Param4: ::windows::runtime::IntoParam<'a, IDirectMusicPort>>(&self, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsourcechannelgroup), ::std::mem::transmute(dwsourcechannel), ::std::mem::transmute(dwdestinationchannelgroup), ::std::mem::transmute(dwdestinationchannel), pdestinationport.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectMusicThru {
    type Vtable = IDirectMusicThru_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3469824999, 13830, 4562, [185, 249, 0, 0, 248, 117, 172, 18]);
}
impl ::std::convert::From<IDirectMusicThru> for ::windows::runtime::IUnknown {
    fn from(value: IDirectMusicThru) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectMusicThru> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectMusicThru) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectMusicThru {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectMusicThru {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectMusicThru_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSound(pub ::windows::runtime::IUnknown);
impl IDirectSound {
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn CreateSoundBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::std::option::Option<IDirectSoundBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsbufferdesc), ::std::mem::transmute(ppdsbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCAPS> {
        let mut result__: <DSCAPS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn DuplicateSoundBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSoundBuffer>>(&self, pdsbufferoriginal: Param0) -> ::windows::runtime::Result<IDirectSoundBuffer> {
        let mut result__: <IDirectSoundBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pdsbufferoriginal.into_param().abi(), &mut result__).from_abi::<IDirectSoundBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwlevel: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), ::std::mem::transmute(dwlevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetSpeakerConfig(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwspeakerconfig)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcguiddevice)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSound {
    type Vtable = IDirectSound_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664468099, 18817, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
impl ::std::convert::From<IDirectSound> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSound) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSound> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSound) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSound {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSound {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscaps: *mut DSCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsbufferoriginal: ::windows::runtime::RawPtr, ppdsbufferduplicate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwspeakerconfig: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwspeakerconfig: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSound3DBuffer(pub ::windows::runtime::IUnknown);
impl IDirectSound3DBuffer {
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DS3DBUFFER> {
        let mut result__: <DS3DBUFFER as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DS3DBUFFER>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwinsideconeangle), ::std::mem::transmute(pdwoutsideconeangle)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn GetConeOrientation(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D9::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D9::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D9::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetConeOutsideVolume(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetMaxDistance(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetMinDistance(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetMode(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn GetPosition(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D9::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D9::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D9::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn GetVelocity(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D9::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D9::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D9::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcds3dbuffer), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinsideconeangle), ::std::mem::transmute(dwoutsideconeangle), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(z), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(lconeoutsidevolume), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(flmaxdistance), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(flmindistance), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetMode(&self, dwmode: u32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwmode), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(z), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(z), ::std::mem::transmute(dwapply)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSound3DBuffer {
    type Vtable = IDirectSound3DBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664468102, 18817, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
impl ::std::convert::From<IDirectSound3DBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSound3DBuffer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSound3DBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSound3DBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSound3DBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSound3DBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pds3dbuffer: *mut DS3DBUFFER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvorientation: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plconeoutsidevolume: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflmaxdistance: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflmindistance: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmode: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvposition: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvvelocity: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flmaxdistance: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flmindistance: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwmode: u32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSound3DListener(pub ::windows::runtime::IUnknown);
impl IDirectSound3DListener {
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DS3DLISTENER> {
        let mut result__: <DS3DLISTENER as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DS3DLISTENER>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDistanceFactor(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetDopplerFactor(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn GetOrientation(&self, pvorientfront: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvorientfront), ::std::mem::transmute(pvorienttop)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn GetPosition(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D9::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D9::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D9::D3DVECTOR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetRolloffFactor(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn GetVelocity(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Direct3D9::D3DVECTOR> {
        let mut result__: <super::super::super::Graphics::Direct3D9::D3DVECTOR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Graphics::Direct3D9::D3DVECTOR>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Graphics_Direct3D9`*"]
    pub unsafe fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclistener), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(fldistancefactor), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(fldopplerfactor), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(xfront), ::std::mem::transmute(yfront), ::std::mem::transmute(zfront), ::std::mem::transmute(xtop), ::std::mem::transmute(ytop), ::std::mem::transmute(ztop), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(z), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(flrollofffactor), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(z), ::std::mem::transmute(dwapply)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn CommitDeferredSettings(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSound3DListener {
    type Vtable = IDirectSound3DListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664468100, 18817, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
impl ::std::convert::From<IDirectSound3DListener> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSound3DListener) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSound3DListener> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSound3DListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSound3DListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSound3DListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound3DListener_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plistener: *mut DS3DLISTENER) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfldistancefactor: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfldopplerfactor: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvorientfront: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvposition: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflrollofffactor: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvvelocity: *mut super::super::super::Graphics::Direct3D9::D3DVECTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D9")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fldistancefactor: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fldopplerfactor: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flrollofffactor: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSound8(pub ::windows::runtime::IUnknown);
impl IDirectSound8 {
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn CreateSoundBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::std::option::Option<IDirectSoundBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsbufferdesc), ::std::mem::transmute(ppdsbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCAPS> {
        let mut result__: <DSCAPS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn DuplicateSoundBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSoundBuffer>>(&self, pdsbufferoriginal: Param0) -> ::windows::runtime::Result<IDirectSoundBuffer> {
        let mut result__: <IDirectSoundBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pdsbufferoriginal.into_param().abi(), &mut result__).from_abi::<IDirectSoundBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetCooperativeLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, dwlevel: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), ::std::mem::transmute(dwlevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Compact(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetSpeakerConfig(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwspeakerconfig)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcguiddevice)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn VerifyCertification(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSound8 {
    type Vtable = IDirectSound8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3305799315, 62357, 18484, [158, 246, 127, 169, 157, 229, 9, 102]);
}
impl ::std::convert::From<IDirectSound8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSound8) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSound8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSound8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSound8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSound8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDirectSound8> for IDirectSound {
    fn from(value: IDirectSound8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectSound8> for IDirectSound {
    fn from(value: &IDirectSound8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSound> for IDirectSound8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSound> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSound> for &IDirectSound8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSound> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSound8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscaps: *mut DSCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsbufferoriginal: ::windows::runtime::RawPtr, ppdsbufferduplicate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwspeakerconfig: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwspeakerconfig: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcertified: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundBuffer(pub ::windows::runtime::IUnknown);
impl IDirectSoundBuffer {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSBCAPS> {
        let mut result__: <DSBCAPS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSBCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcurrentplaycursor), ::std::mem::transmute(pdwcurrentwritecursor)).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::super::Multimedia::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwfxformat), ::std::mem::transmute(dwsizeallocated), ::std::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetVolume(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPan(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetFrequency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSound>>(&self, pdirectsound: Param0, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pdirectsound.into_param().abi(), ::std::mem::transmute(pcdsbufferdesc)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::std::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::std::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoffset), ::std::mem::transmute(dwbytes), ::std::mem::transmute(ppvaudioptr1), ::std::mem::transmute(pdwaudiobytes1), ::std::mem::transmute(ppvaudioptr2), ::std::mem::transmute(pdwaudiobytes2), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved1), ::std::mem::transmute(dwpriority), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwnewposition)).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::super::Multimedia::WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcfxformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(lvolume)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetPan(&self, lpan: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpan)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfrequency)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::std::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::std::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvaudioptr1), ::std::mem::transmute(dwaudiobytes1), ::std::mem::transmute(pvaudioptr2), ::std::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundBuffer {
    type Vtable = IDirectSoundBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(664468101, 18817, 4558, [165, 33, 0, 32, 175, 11, 229, 96]);
}
impl ::std::convert::From<IDirectSoundBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundBuffer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsbuffercaps: *mut DSBCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwfxformat: *mut super::super::Multimedia::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plvolume: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpan: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfrequency: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::std::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::std::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwnewposition: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcfxformat: *const super::super::Multimedia::WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lvolume: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpan: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfrequency: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaudioptr1: *const ::std::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::std::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundBuffer8(pub ::windows::runtime::IUnknown);
impl IDirectSoundBuffer8 {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSBCAPS> {
        let mut result__: <DSBCAPS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSBCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcurrentplaycursor), ::std::mem::transmute(pdwcurrentwritecursor)).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::super::Multimedia::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwfxformat), ::std::mem::transmute(dwsizeallocated), ::std::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetVolume(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPan(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetFrequency(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSound>>(&self, pdirectsound: Param0, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pdirectsound.into_param().abi(), ::std::mem::transmute(pcdsbufferdesc)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::std::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::std::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoffset), ::std::mem::transmute(dwbytes), ::std::mem::transmute(ppvaudioptr1), ::std::mem::transmute(pdwaudiobytes1), ::std::mem::transmute(ppvaudioptr2), ::std::mem::transmute(pdwaudiobytes2), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved1), ::std::mem::transmute(dwpriority), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwnewposition)).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn SetFormat(&self, pcfxformat: *const super::super::Multimedia::WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcfxformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetVolume(&self, lvolume: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(lvolume)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetPan(&self, lpan: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpan)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetFrequency(&self, dwfrequency: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfrequency)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::std::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::std::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvaudioptr1), ::std::mem::transmute(dwaudiobytes1), ::std::mem::transmute(pvaudioptr2), ::std::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Restore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetFX(&self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(dweffectscount), ::std::mem::transmute(pdsfxdesc), ::std::mem::transmute(pdwresultcodes)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn AcquireResources(&self, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), ::std::mem::transmute(dweffectscount), ::std::mem::transmute(pdwresultcodes)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const ::windows::runtime::GUID, dwindex: u32, rguidinterface: *const ::windows::runtime::GUID, ppobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(rguidobject), ::std::mem::transmute(dwindex), ::std::mem::transmute(rguidinterface), ::std::mem::transmute(ppobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundBuffer8 {
    type Vtable = IDirectSoundBuffer8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1747297353, 29988, 19842, [146, 15, 80, 227, 106, 179, 171, 30]);
}
impl ::std::convert::From<IDirectSoundBuffer8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundBuffer8) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundBuffer8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundBuffer8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDirectSoundBuffer8> for IDirectSoundBuffer {
    fn from(value: IDirectSoundBuffer8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectSoundBuffer8> for IDirectSoundBuffer {
    fn from(value: &IDirectSoundBuffer8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSoundBuffer> for IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSoundBuffer> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSoundBuffer> for &IDirectSoundBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSoundBuffer> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundBuffer8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsbuffercaps: *mut DSBCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwfxformat: *mut super::super::Multimedia::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plvolume: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plpan: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfrequency: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsound: ::windows::runtime::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::std::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::std::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwnewposition: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcfxformat: *const super::super::Multimedia::WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lvolume: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpan: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfrequency: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaudioptr1: *const ::std::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::std::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidobject: *const ::windows::runtime::GUID, dwindex: u32, rguidinterface: *const ::windows::runtime::GUID, ppobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundCapture(pub ::windows::runtime::IUnknown);
impl IDirectSoundCapture {
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn CreateCaptureBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::std::option::Option<IDirectSoundCaptureBuffer>, punkouter: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdscbufferdesc), ::std::mem::transmute(ppdscbuffer), punkouter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCCAPS> {
        let mut result__: <DSCCAPS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSCCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Initialize(&self, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcguiddevice)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCapture {
    type Vtable = IDirectSoundCapture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2954954625, 35277, 4560, [175, 8, 0, 160, 201, 37, 205, 22]);
}
impl ::std::convert::From<IDirectSoundCapture> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCapture) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundCapture> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCapture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCapture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCapture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCapture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsccaps: *mut DSCCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcguiddevice: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundCaptureBuffer(pub ::windows::runtime::IUnknown);
impl IDirectSoundCaptureBuffer {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCBCAPS> {
        let mut result__: <DSCBCAPS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSCBCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcaptureposition), ::std::mem::transmute(pdwreadposition)).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::super::Multimedia::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwfxformat), ::std::mem::transmute(dwsizeallocated), ::std::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSoundCapture>>(&self, pdirectsoundcapture: Param0, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pdirectsoundcapture.into_param().abi(), ::std::mem::transmute(pcdscbufferdesc)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::std::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::std::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoffset), ::std::mem::transmute(dwbytes), ::std::mem::transmute(ppvaudioptr1), ::std::mem::transmute(pdwaudiobytes1), ::std::mem::transmute(ppvaudioptr2), ::std::mem::transmute(pdwaudiobytes2), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Start(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::std::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::std::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvaudioptr1), ::std::mem::transmute(dwaudiobytes1), ::std::mem::transmute(pvaudioptr2), ::std::mem::transmute(dwaudiobytes2)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCaptureBuffer {
    type Vtable = IDirectSoundCaptureBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2954954626, 35277, 4560, [175, 8, 0, 160, 201, 37, 205, 22]);
}
impl ::std::convert::From<IDirectSoundCaptureBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCaptureBuffer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundCaptureBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCaptureBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCaptureBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCaptureBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscbcaps: *mut DSCBCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwfxformat: *mut super::super::Multimedia::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsoundcapture: ::windows::runtime::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::std::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::std::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaudioptr1: *const ::std::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::std::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundCaptureBuffer8(pub ::windows::runtime::IUnknown);
impl IDirectSoundCaptureBuffer8 {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCaps(&self) -> ::windows::runtime::Result<DSCBCAPS> {
        let mut result__: <DSCBCAPS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSCBCAPS>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcaptureposition), ::std::mem::transmute(pdwreadposition)).ok()
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn GetFormat(&self, pwfxformat: *mut super::super::Multimedia::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pwfxformat), ::std::mem::transmute(dwsizeallocated), ::std::mem::transmute(pdwsizewritten)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Media_Multimedia")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, IDirectSoundCapture>>(&self, pdirectsoundcapture: Param0, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pdirectsoundcapture.into_param().abi(), ::std::mem::transmute(pcdscbufferdesc)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::std::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::std::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoffset), ::std::mem::transmute(dwbytes), ::std::mem::transmute(ppvaudioptr1), ::std::mem::transmute(pdwaudiobytes1), ::std::mem::transmute(ppvaudioptr2), ::std::mem::transmute(pdwaudiobytes2), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Start(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Unlock(&self, pvaudioptr1: *const ::std::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::std::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvaudioptr1), ::std::mem::transmute(dwaudiobytes1), ::std::mem::transmute(pvaudioptr2), ::std::mem::transmute(dwaudiobytes2)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetObjectInPath(&self, rguidobject: *const ::windows::runtime::GUID, dwindex: u32, rguidinterface: *const ::windows::runtime::GUID, ppobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(rguidobject), ::std::mem::transmute(dwindex), ::std::mem::transmute(rguidinterface), ::std::mem::transmute(ppobject)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetFXStatus(&self, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dweffectscount), ::std::mem::transmute(pdwfxstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCaptureBuffer8 {
    type Vtable = IDirectSoundCaptureBuffer8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(10030580, 3515, 18546, [131, 62, 109, 48, 62, 128, 174, 182]);
}
impl ::std::convert::From<IDirectSoundCaptureBuffer8> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCaptureBuffer8) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundCaptureBuffer8> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCaptureBuffer8) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IDirectSoundCaptureBuffer8> for IDirectSoundCaptureBuffer {
    fn from(value: IDirectSoundCaptureBuffer8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDirectSoundCaptureBuffer8> for IDirectSoundCaptureBuffer {
    fn from(value: &IDirectSoundCaptureBuffer8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSoundCaptureBuffer> for IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSoundCaptureBuffer> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDirectSoundCaptureBuffer> for &IDirectSoundCaptureBuffer8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDirectSoundCaptureBuffer> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureBuffer8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscbcaps: *mut DSCBCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwfxformat: *mut super::super::Multimedia::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_Multimedia")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdirectsoundcapture: ::windows::runtime::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_Multimedia"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::std::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::std::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaudioptr1: *const ::std::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::std::ffi::c_void, dwaudiobytes2: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidobject: *const ::windows::runtime::GUID, dwindex: u32, rguidinterface: *const ::windows::runtime::GUID, ppobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundCaptureFXAec(pub ::windows::runtime::IUnknown);
impl IDirectSoundCaptureFXAec {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdscfxaec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSCFXAec> {
        let mut result__: <DSCFXAec as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSCFXAec>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetStatus(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCaptureFXAec {
    type Vtable = IDirectSoundCaptureFXAec_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2910065725, 36925, 19127, [128, 102, 40, 211, 99, 3, 109, 101]);
}
impl ::std::convert::From<IDirectSoundCaptureFXAec> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCaptureFXAec) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundCaptureFXAec> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCaptureFXAec) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCaptureFXAec {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCaptureFXAec {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXAec_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscfxaec: *const DSCFXAec) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscfxaec: *mut DSCFXAec) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundCaptureFXNoiseSuppress(pub ::windows::runtime::IUnknown);
impl IDirectSoundCaptureFXNoiseSuppress {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdscfxnoisesuppress)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSCFXNoiseSuppress> {
        let mut result__: <DSCFXNoiseSuppress as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSCFXNoiseSuppress>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundCaptureFXNoiseSuppress {
    type Vtable = IDirectSoundCaptureFXNoiseSuppress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3979419201, 64430, 16757, [150, 37, 205, 8, 84, 246, 147, 202]);
}
impl ::std::convert::From<IDirectSoundCaptureFXNoiseSuppress> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundCaptureFXNoiseSuppress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundCaptureFXNoiseSuppress> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundCaptureFXNoiseSuppress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundCaptureFXNoiseSuppress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundCaptureFXNoiseSuppress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundCaptureFXNoiseSuppress_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXChorus(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXChorus {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxchorus)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXChorus> {
        let mut result__: <DSFXChorus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXChorus>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXChorus {
    type Vtable = IDirectSoundFXChorus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2282242787, 5215, 17382, [169, 52, 167, 24, 6, 229, 5, 71]);
}
impl ::std::convert::From<IDirectSoundFXChorus> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXChorus) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXChorus> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXChorus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXChorus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXChorus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXChorus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxchorus: *const DSFXChorus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxchorus: *mut DSFXChorus) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXCompressor(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXCompressor {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxcompressor)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXCompressor> {
        let mut result__: <DSFXCompressor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXCompressor>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXCompressor {
    type Vtable = IDirectSoundFXCompressor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1270681940, 25334, 20012, [161, 92, 211, 182, 196, 23, 247, 160]);
}
impl ::std::convert::From<IDirectSoundFXCompressor> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXCompressor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXCompressor> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXCompressor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXCompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXCompressor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXCompressor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxcompressor: *mut DSFXCompressor) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXDistortion(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXDistortion {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxdistortion)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXDistortion> {
        let mut result__: <DSFXDistortion as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXDistortion>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXDistortion {
    type Vtable = IDirectSoundFXDistortion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2395947814, 17759, 19851, [189, 169, 141, 93, 62, 158, 62, 11]);
}
impl ::std::convert::From<IDirectSoundFXDistortion> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXDistortion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXDistortion> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXDistortion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXDistortion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXDistortion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXDistortion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxdistortion: *mut DSFXDistortion) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXEcho(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXEcho {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxecho)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXEcho> {
        let mut result__: <DSFXEcho as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXEcho>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXEcho {
    type Vtable = IDirectSoundFXEcho_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2345832159, 20699, 20114, [162, 189, 68, 84, 136, 209, 237, 66]);
}
impl ::std::convert::From<IDirectSoundFXEcho> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXEcho) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXEcho> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXEcho) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXEcho {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXEcho {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXEcho_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxecho: *const DSFXEcho) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxecho: *mut DSFXEcho) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXFlanger(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXFlanger {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxflanger)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXFlanger> {
        let mut result__: <DSFXFlanger as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXFlanger>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXFlanger {
    type Vtable = IDirectSoundFXFlanger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2420021368, 11410, 16498, [155, 44, 234, 104, 245, 57, 103, 131]);
}
impl ::std::convert::From<IDirectSoundFXFlanger> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXFlanger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXFlanger> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXFlanger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXFlanger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXFlanger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXFlanger_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxflanger: *const DSFXFlanger) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxflanger: *mut DSFXFlanger) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXGargle(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXGargle {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxgargle)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXGargle> {
        let mut result__: <DSFXGargle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXGargle>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXGargle {
    type Vtable = IDirectSoundFXGargle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3591828306, 54818, 4558, [170, 197, 0, 32, 175, 11, 153, 163]);
}
impl ::std::convert::From<IDirectSoundFXGargle> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXGargle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXGargle> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXGargle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXGargle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXGargle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXGargle_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxgargle: *const DSFXGargle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxgargle: *mut DSFXGargle) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXI3DL2Reverb(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXI3DL2Reverb {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxi3dl2reverb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXI3DL2Reverb> {
        let mut result__: <DSFXI3DL2Reverb as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXI3DL2Reverb>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetPreset(&self, dwpreset: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwpreset)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetPreset(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetQuality(&self, lquality: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(lquality)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetQuality(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXI3DL2Reverb {
    type Vtable = IDirectSoundFXI3DL2Reverb_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1259760234, 3430, 17395, [128, 227, 238, 98, 128, 222, 225, 164]);
}
impl ::std::convert::From<IDirectSoundFXI3DL2Reverb> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXI3DL2Reverb) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXI3DL2Reverb> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXI3DL2Reverb) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXI3DL2Reverb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXI3DL2Reverb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXI3DL2Reverb_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpreset: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwpreset: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lquality: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plquality: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXParamEq(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXParamEq {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxparameq)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXParamEq> {
        let mut result__: <DSFXParamEq as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXParamEq>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXParamEq {
    type Vtable = IDirectSoundFXParamEq_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3225201150, 65168, 16900, [128, 120, 130, 51, 76, 209, 119, 218]);
}
impl ::std::convert::From<IDirectSoundFXParamEq> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXParamEq) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXParamEq> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXParamEq) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXParamEq {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXParamEq {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXParamEq_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxparameq: *const DSFXParamEq) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxparameq: *mut DSFXParamEq) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFXWavesReverb(pub ::windows::runtime::IUnknown);
impl IDirectSoundFXWavesReverb {
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcdsfxwavesreverb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
    pub unsafe fn GetAllParameters(&self) -> ::windows::runtime::Result<DSFXWavesReverb> {
        let mut result__: <DSFXWavesReverb as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<DSFXWavesReverb>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFXWavesReverb {
    type Vtable = IDirectSoundFXWavesReverb_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1183157306, 3526, 17891, [183, 96, 212, 238, 241, 108, 179, 37]);
}
impl ::std::convert::From<IDirectSoundFXWavesReverb> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFXWavesReverb) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFXWavesReverb> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFXWavesReverb) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFXWavesReverb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFXWavesReverb {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFXWavesReverb_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundFullDuplex(pub ::windows::runtime::IUnknown);
impl IDirectSoundFullDuplex {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Multimedia"))]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`, `Win32_Media_Multimedia`*"]
    pub unsafe fn Initialize<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(
        &self,
        pcaptureguid: *const ::windows::runtime::GUID,
        prenderguid: *const ::windows::runtime::GUID,
        lpdscbufferdesc: *const DSCBUFFERDESC,
        lpdsbufferdesc: *const DSBUFFERDESC,
        hwnd: Param4,
        dwlevel: u32,
        lplpdirectsoundcapturebuffer8: *mut ::std::option::Option<IDirectSoundCaptureBuffer8>,
        lplpdirectsoundbuffer8: *mut ::std::option::Option<IDirectSoundBuffer8>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pcaptureguid),
            ::std::mem::transmute(prenderguid),
            ::std::mem::transmute(lpdscbufferdesc),
            ::std::mem::transmute(lpdsbufferdesc),
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwlevel),
            ::std::mem::transmute(lplpdirectsoundcapturebuffer8),
            ::std::mem::transmute(lplpdirectsoundbuffer8),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundFullDuplex {
    type Vtable = IDirectSoundFullDuplex_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3989523578, 55979, 16918, [164, 46, 108, 80, 89, 109, 220, 29]);
}
impl ::std::convert::From<IDirectSoundFullDuplex> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundFullDuplex) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundFullDuplex> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundFullDuplex) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundFullDuplex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundFullDuplex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundFullDuplex_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Multimedia"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcaptureguid: *const ::windows::runtime::GUID, prenderguid: *const ::windows::runtime::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::windows::runtime::RawPtr, lplpdirectsoundbuffer8: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_Multimedia")))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectSoundNotify(pub ::windows::runtime::IUnknown);
impl IDirectSoundNotify {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
    pub unsafe fn SetNotificationPositions(&self, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwpositionnotifies), ::std::mem::transmute(pcpositionnotifies)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectSoundNotify {
    type Vtable = IDirectSoundNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2954954627, 35277, 4560, [175, 8, 0, 160, 201, 37, 205, 22]);
}
impl ::std::convert::From<IDirectSoundNotify> for ::windows::runtime::IUnknown {
    fn from(value: IDirectSoundNotify) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectSoundNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectSoundNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectSoundNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectSoundNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectSoundNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct INSTHEADER {
    pub cRegions: u32,
    pub Locale: MIDILOCALE,
}
impl INSTHEADER {}
impl ::std::default::Default for INSTHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INSTHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INSTHEADER").field("cRegions", &self.cRegions).field("Locale", &self.Locale).finish()
    }
}
impl ::std::cmp::PartialEq for INSTHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cRegions == other.cRegions && self.Locale == other.Locale
    }
}
impl ::std::cmp::Eq for INSTHEADER {}
unsafe impl ::windows::runtime::Abi for INSTHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KSPROPERTY_AUDIOEFFECTSDISCOVERY(pub i32);
pub const KSPROPERTY_AUDIOEFFECTSDISCOVERY_EFFECTSLIST: KSPROPERTY_AUDIOEFFECTSDISCOVERY = KSPROPERTY_AUDIOEFFECTSDISCOVERY(1i32);
impl ::std::convert::From<i32> for KSPROPERTY_AUDIOEFFECTSDISCOVERY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KSPROPERTY_AUDIOEFFECTSDISCOVERY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const KSPROPERTY_SUPPORT_GET: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const KSPROPERTY_SUPPORT_SET: u32 = 2u32;
pub const KSPROPSETID_AudioEffectsDiscovery: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(186743410, 5816, 19021, [189, 237, 249, 214, 187, 237, 205, 143]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Media_Audio_CoreAudio")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Audio_CoreAudio`*"]
pub struct KSP_PINMODE {
    pub PinProperty: super::CoreAudio::KSP_PIN,
    pub AudioProcessingMode: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Media_Audio_CoreAudio")]
impl KSP_PINMODE {}
#[cfg(feature = "Win32_Media_Audio_CoreAudio")]
impl ::std::default::Default for KSP_PINMODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio_CoreAudio")]
impl ::std::cmp::PartialEq for KSP_PINMODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Media_Audio_CoreAudio")]
impl ::std::cmp::Eq for KSP_PINMODE {}
#[cfg(feature = "Win32_Media_Audio_CoreAudio")]
unsafe impl ::windows::runtime::Abi for KSP_PINMODE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKA = unsafe extern "system" fn(param0: *mut ::windows::runtime::GUID, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: *mut ::std::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPDSENUMCALLBACKW = unsafe extern "system" fn(param0: *mut ::windows::runtime::GUID, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: *mut ::std::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACK1 = unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_1_DATA, param1: *mut ::std::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKA = unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_A_DATA, param1: *mut ::std::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNDIRECTSOUNDDEVICEENUMERATECALLBACKW = unsafe extern "system" fn(param0: *mut DSPROPERTY_DIRECTSOUNDDEVICE_DESCRIPTION_W_DATA, param1: *mut ::std::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct MDEVICECAPSEX {
    pub cbSize: u32,
    pub pCaps: *mut ::std::ffi::c_void,
}
impl MDEVICECAPSEX {}
impl ::std::default::Default for MDEVICECAPSEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MDEVICECAPSEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MDEVICECAPSEX {}
unsafe impl ::windows::runtime::Abi for MDEVICECAPSEX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct MIDILOCALE {
    pub ulBank: u32,
    pub ulInstrument: u32,
}
impl MIDILOCALE {}
impl ::std::default::Default for MIDILOCALE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIDILOCALE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDILOCALE").field("ulBank", &self.ulBank).field("ulInstrument", &self.ulInstrument).finish()
    }
}
impl ::std::cmp::PartialEq for MIDILOCALE {
    fn eq(&self, other: &Self) -> bool {
        self.ulBank == other.ulBank && self.ulInstrument == other.ulInstrument
    }
}
impl ::std::cmp::Eq for MIDILOCALE {}
unsafe impl ::windows::runtime::Abi for MIDILOCALE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Media_Multimedia")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Media_Multimedia`*"]
pub struct MIDIOPENDESC {
    pub hMidi: super::super::Multimedia::HMIDI,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub dnDevNode: usize,
    pub cIds: u32,
    pub rgIds: [super::super::Multimedia::MIDIOPENSTRMID; 1],
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl MIDIOPENDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::default::Default for MIDIOPENDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::PartialEq for MIDIOPENDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Media_Multimedia")]
impl ::std::cmp::Eq for MIDIOPENDESC {}
#[cfg(feature = "Win32_Media_Multimedia")]
unsafe impl ::windows::runtime::Abi for MIDIOPENDESC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct POOLCUE {
    pub ulOffset: u32,
}
impl POOLCUE {}
impl ::std::default::Default for POOLCUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for POOLCUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POOLCUE").field("ulOffset", &self.ulOffset).finish()
    }
}
impl ::std::cmp::PartialEq for POOLCUE {
    fn eq(&self, other: &Self) -> bool {
        self.ulOffset == other.ulOffset
    }
}
impl ::std::cmp::Eq for POOLCUE {}
unsafe impl ::windows::runtime::Abi for POOLCUE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct POOLTABLE {
    pub cbSize: u32,
    pub cCues: u32,
}
impl POOLTABLE {}
impl ::std::default::Default for POOLTABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for POOLTABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POOLTABLE").field("cbSize", &self.cbSize).field("cCues", &self.cCues).finish()
    }
}
impl ::std::cmp::PartialEq for POOLTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cCues == other.cCues
    }
}
impl ::std::cmp::Eq for POOLTABLE {}
unsafe impl ::windows::runtime::Abi for POOLTABLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const POOL_CUE_NULL: i32 = -1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const REFRESH_F_LASTBUFFER: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct RGNHEADER {
    pub RangeKey: RGNRANGE,
    pub RangeVelocity: RGNRANGE,
    pub fusOptions: u16,
    pub usKeyGroup: u16,
}
impl RGNHEADER {}
impl ::std::default::Default for RGNHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RGNHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RGNHEADER").field("RangeKey", &self.RangeKey).field("RangeVelocity", &self.RangeVelocity).field("fusOptions", &self.fusOptions).field("usKeyGroup", &self.usKeyGroup).finish()
    }
}
impl ::std::cmp::PartialEq for RGNHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RangeKey == other.RangeKey && self.RangeVelocity == other.RangeVelocity && self.fusOptions == other.fusOptions && self.usKeyGroup == other.usKeyGroup
    }
}
impl ::std::cmp::Eq for RGNHEADER {}
unsafe impl ::windows::runtime::Abi for RGNHEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct RGNRANGE {
    pub usLow: u16,
    pub usHigh: u16,
}
impl RGNRANGE {}
impl ::std::default::Default for RGNRANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RGNRANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RGNRANGE").field("usLow", &self.usLow).field("usHigh", &self.usHigh).finish()
    }
}
impl ::std::cmp::PartialEq for RGNRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.usLow == other.usLow && self.usHigh == other.usHigh
    }
}
impl ::std::cmp::Eq for RGNRANGE {}
unsafe impl ::windows::runtime::Abi for RGNRANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const SIZE_DVINFO: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct Tag_DVAudInfo {
    pub bAudStyle: [u8; 2],
    pub bAudQu: [u8; 2],
    pub bNumAudPin: u8,
    pub wAvgSamplesPerPinPerFrm: [u16; 2],
    pub wBlkMode: u16,
    pub wDIFMode: u16,
    pub wBlkDiv: u16,
}
impl Tag_DVAudInfo {}
impl ::std::default::Default for Tag_DVAudInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Tag_DVAudInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Tag_DVAudInfo")
            .field("bAudStyle", &self.bAudStyle)
            .field("bAudQu", &self.bAudQu)
            .field("bNumAudPin", &self.bNumAudPin)
            .field("wAvgSamplesPerPinPerFrm", &self.wAvgSamplesPerPinPerFrm)
            .field("wBlkMode", &self.wBlkMode)
            .field("wDIFMode", &self.wDIFMode)
            .field("wBlkDiv", &self.wBlkDiv)
            .finish()
    }
}
impl ::std::cmp::PartialEq for Tag_DVAudInfo {
    fn eq(&self, other: &Self) -> bool {
        self.bAudStyle == other.bAudStyle && self.bAudQu == other.bAudQu && self.bNumAudPin == other.bNumAudPin && self.wAvgSamplesPerPinPerFrm == other.wAvgSamplesPerPinPerFrm && self.wBlkMode == other.wBlkMode && self.wDIFMode == other.wDIFMode && self.wBlkDiv == other.wBlkDiv
    }
}
impl ::std::cmp::Eq for Tag_DVAudInfo {}
unsafe impl ::windows::runtime::Abi for Tag_DVAudInfo {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows::runtime::GUID,
    pub dwSamplesPerFrame: u32,
    pub dwBytesPerSampleContainer: u32,
    pub dwValidBitsPerSample: u32,
    pub fFramesPerSecond: f32,
    pub dwChannelMask: u32,
}
impl UNCOMPRESSEDAUDIOFORMAT {}
impl ::std::default::Default for UNCOMPRESSEDAUDIOFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for UNCOMPRESSEDAUDIOFORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UNCOMPRESSEDAUDIOFORMAT")
            .field("guidFormatType", &self.guidFormatType)
            .field("dwSamplesPerFrame", &self.dwSamplesPerFrame)
            .field("dwBytesPerSampleContainer", &self.dwBytesPerSampleContainer)
            .field("dwValidBitsPerSample", &self.dwValidBitsPerSample)
            .field("fFramesPerSecond", &self.fFramesPerSecond)
            .field("dwChannelMask", &self.dwChannelMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for UNCOMPRESSEDAUDIOFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.guidFormatType == other.guidFormatType && self.dwSamplesPerFrame == other.dwSamplesPerFrame && self.dwBytesPerSampleContainer == other.dwBytesPerSampleContainer && self.dwValidBitsPerSample == other.dwValidBitsPerSample && self.fFramesPerSecond == other.fFramesPerSecond && self.dwChannelMask == other.dwChannelMask
    }
}
impl ::std::cmp::Eq for UNCOMPRESSEDAUDIOFORMAT {}
unsafe impl ::windows::runtime::Abi for UNCOMPRESSEDAUDIOFORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct WAVELINK {
    pub fusOptions: u16,
    pub usPhaseGroup: u16,
    pub ulChannel: u32,
    pub ulTableIndex: u32,
}
impl WAVELINK {}
impl ::std::default::Default for WAVELINK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WAVELINK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WAVELINK").field("fusOptions", &self.fusOptions).field("usPhaseGroup", &self.usPhaseGroup).field("ulChannel", &self.ulChannel).field("ulTableIndex", &self.ulTableIndex).finish()
    }
}
impl ::std::cmp::PartialEq for WAVELINK {
    fn eq(&self, other: &Self) -> bool {
        self.fusOptions == other.fusOptions && self.usPhaseGroup == other.usPhaseGroup && self.ulChannel == other.ulChannel && self.ulTableIndex == other.ulTableIndex
    }
}
impl ::std::cmp::Eq for WAVELINK {}
unsafe impl ::windows::runtime::Abi for WAVELINK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WAVELINK_CHANNEL_LEFT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WAVELINK_CHANNEL_RIGHT: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WLOOP_TYPE_FORWARD: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const WLOOP_TYPE_RELEASE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`, `Win32_Foundation`*"]
pub struct _DMUS_PORTPARAMS {
    pub dwSize: u32,
    pub dwValidParams: u32,
    pub dwVoices: u32,
    pub dwChannelGroups: u32,
    pub dwAudioChannels: u32,
    pub dwSampleRate: u32,
    pub dwEffectFlags: u32,
    pub fShare: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl _DMUS_PORTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for _DMUS_PORTPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for _DMUS_PORTPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_DMUS_PORTPARAMS")
            .field("dwSize", &self.dwSize)
            .field("dwValidParams", &self.dwValidParams)
            .field("dwVoices", &self.dwVoices)
            .field("dwChannelGroups", &self.dwChannelGroups)
            .field("dwAudioChannels", &self.dwAudioChannels)
            .field("dwSampleRate", &self.dwSampleRate)
            .field("dwEffectFlags", &self.dwEffectFlags)
            .field("fShare", &self.fShare)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for _DMUS_PORTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwValidParams == other.dwValidParams && self.dwVoices == other.dwVoices && self.dwChannelGroups == other.dwChannelGroups && self.dwAudioChannels == other.dwAudioChannels && self.dwSampleRate == other.dwSampleRate && self.dwEffectFlags == other.dwEffectFlags && self.fShare == other.fShare
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for _DMUS_PORTPARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for _DMUS_PORTPARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub const _FACDS: u32 = 2168u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct _rloop {
    pub cbSize: u32,
    pub ulType: u32,
    pub ulStart: u32,
    pub ulLength: u32,
}
impl _rloop {}
impl ::std::default::Default for _rloop {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _rloop {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_rloop").field("cbSize", &self.cbSize).field("ulType", &self.ulType).field("ulStart", &self.ulStart).field("ulLength", &self.ulLength).finish()
    }
}
impl ::std::cmp::PartialEq for _rloop {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulType == other.ulType && self.ulStart == other.ulStart && self.ulLength == other.ulLength
    }
}
impl ::std::cmp::Eq for _rloop {}
unsafe impl ::windows::runtime::Abi for _rloop {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_DirectMusic`*"]
pub struct _rwsmp {
    pub cbSize: u32,
    pub usUnityNote: u16,
    pub sFineTune: i16,
    pub lAttenuation: i32,
    pub fulOptions: u32,
    pub cSampleLoops: u32,
}
impl _rwsmp {}
impl ::std::default::Default for _rwsmp {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _rwsmp {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_rwsmp").field("cbSize", &self.cbSize).field("usUnityNote", &self.usUnityNote).field("sFineTune", &self.sFineTune).field("lAttenuation", &self.lAttenuation).field("fulOptions", &self.fulOptions).field("cSampleLoops", &self.cSampleLoops).finish()
    }
}
impl ::std::cmp::PartialEq for _rwsmp {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.usUnityNote == other.usUnityNote && self.sFineTune == other.sFineTune && self.lAttenuation == other.lAttenuation && self.fulOptions == other.fulOptions && self.cSampleLoops == other.cSampleLoops
    }
}
impl ::std::cmp::Eq for _rwsmp {}
unsafe impl ::windows::runtime::Abi for _rwsmp {
    type Abi = Self;
}
