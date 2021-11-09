#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_ALREADY_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073919i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_ALREADY_UNLOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073914i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_APO_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073910i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_BUFFERS_OVERLAP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073915i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073917i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_INVALID_APO_CLSID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073916i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_INVALID_COEFFCOUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073909i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_INVALID_COEFFICIENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073908i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073911i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_INVALID_CURVE_PARAM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073907i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_INVALID_INPUTID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073906i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073912i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_NOT_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073918i32 as _);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2005073913i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows::runtime::GUID,
}
impl APOInitBaseStruct {}
impl ::core::default::Default for APOInitBaseStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for APOInitBaseStruct {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APOInitBaseStruct").field("cbSize", &self.cbSize).field("clsid", &self.clsid).finish()
    }
}
impl ::core::cmp::PartialEq for APOInitBaseStruct {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.clsid == other.clsid
    }
}
impl ::core::cmp::Eq for APOInitBaseStruct {}
unsafe impl ::windows::runtime::Abi for APOInitBaseStruct {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pAPOSystemEffectsProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl APOInitSystemEffects {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for APOInitSystemEffects {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for APOInitSystemEffects {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APOInitSystemEffects")
            .field("APOInit", &self.APOInit)
            .field("pAPOEndpointProperties", &self.pAPOEndpointProperties)
            .field("pAPOSystemEffectsProperties", &self.pAPOSystemEffectsProperties)
            .field("pReserved", &self.pReserved)
            .field("pDeviceCollection", &self.pDeviceCollection)
            .finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for APOInitSystemEffects {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pAPOSystemEffectsProperties == other.pAPOSystemEffectsProperties && self.pReserved == other.pReserved && self.pDeviceCollection == other.pDeviceCollection
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for APOInitSystemEffects {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::runtime::Abi for APOInitSystemEffects {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pAPOSystemEffectsProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows::runtime::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APOInitSystemEffects2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for APOInitSystemEffects2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APOInitSystemEffects2 {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for APOInitSystemEffects2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`, `Win32_System_Com`, `Win32_UI_Shell_PropertiesSystem`*"]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pServiceProvider: ::core::option::Option<super::super::super::System::Com::IServiceProvider>,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows::runtime::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl APOInitSystemEffects3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APOInitSystemEffects3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for APOInitSystemEffects3 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APOInitSystemEffects3")
            .field("APOInit", &self.APOInit)
            .field("pAPOEndpointProperties", &self.pAPOEndpointProperties)
            .field("pServiceProvider", &self.pServiceProvider)
            .field("pDeviceCollection", &self.pDeviceCollection)
            .field("nSoftwareIoDeviceInCollection", &self.nSoftwareIoDeviceInCollection)
            .field("nSoftwareIoConnectorIndex", &self.nSoftwareIoConnectorIndex)
            .field("AudioProcessingMode", &self.AudioProcessingMode)
            .field("InitializeForDiscoveryOnly", &self.InitializeForDiscoveryOnly)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APOInitSystemEffects3 {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pServiceProvider == other.pServiceProvider && self.pDeviceCollection == other.pDeviceCollection && self.nSoftwareIoDeviceInCollection == other.nSoftwareIoDeviceInCollection && self.nSoftwareIoConnectorIndex == other.nSoftwareIoConnectorIndex && self.AudioProcessingMode == other.AudioProcessingMode && self.InitializeForDiscoveryOnly == other.InitializeForDiscoveryOnly
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APOInitSystemEffects3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for APOInitSystemEffects3 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APO_BUFFER_FLAGS(pub i32);
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(0i32);
pub const BUFFER_VALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(1i32);
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(2i32);
impl ::core::convert::From<i32> for APO_BUFFER_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APO_BUFFER_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APO_CONNECTION_BUFFER_TYPE(pub i32);
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(0i32);
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(1i32);
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(2i32);
impl ::core::convert::From<i32> for APO_CONNECTION_BUFFER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APO_CONNECTION_BUFFER_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: ::core::option::Option<IAudioMediaType>,
    pub u32Signature: u32,
}
impl APO_CONNECTION_DESCRIPTOR {}
impl ::core::default::Default for APO_CONNECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for APO_CONNECTION_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APO_CONNECTION_DESCRIPTOR").field("Type", &self.Type).field("pBuffer", &self.pBuffer).field("u32MaxFrameCount", &self.u32MaxFrameCount).field("pFormat", &self.pFormat).field("u32Signature", &self.u32Signature).finish()
    }
}
impl ::core::cmp::PartialEq for APO_CONNECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pBuffer == other.pBuffer && self.u32MaxFrameCount == other.u32MaxFrameCount && self.pFormat == other.pFormat && self.u32Signature == other.u32Signature
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for APO_CONNECTION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct APO_CONNECTION_PROPERTY {
    pub pBuffer: usize,
    pub u32ValidFrameCount: u32,
    pub u32BufferFlags: APO_BUFFER_FLAGS,
    pub u32Signature: u32,
}
impl APO_CONNECTION_PROPERTY {}
impl ::core::default::Default for APO_CONNECTION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for APO_CONNECTION_PROPERTY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APO_CONNECTION_PROPERTY").field("pBuffer", &self.pBuffer).field("u32ValidFrameCount", &self.u32ValidFrameCount).field("u32BufferFlags", &self.u32BufferFlags).field("u32Signature", &self.u32Signature).finish()
    }
}
impl ::core::cmp::PartialEq for APO_CONNECTION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.u32ValidFrameCount == other.u32ValidFrameCount && self.u32BufferFlags == other.u32BufferFlags && self.u32Signature == other.u32Signature
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_PROPERTY {}
unsafe impl ::windows::runtime::Abi for APO_CONNECTION_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct APO_CONNECTION_PROPERTY_V2 {
    pub property: APO_CONNECTION_PROPERTY,
    pub u64QPCTime: u64,
}
impl APO_CONNECTION_PROPERTY_V2 {}
impl ::core::default::Default for APO_CONNECTION_PROPERTY_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for APO_CONNECTION_PROPERTY_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APO_CONNECTION_PROPERTY_V2").field("property", &self.property).field("u64QPCTime", &self.u64QPCTime).finish()
    }
}
impl ::core::cmp::PartialEq for APO_CONNECTION_PROPERTY_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.property == other.property && self.u64QPCTime == other.u64QPCTime
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_PROPERTY_V2 {}
unsafe impl ::windows::runtime::Abi for APO_CONNECTION_PROPERTY_V2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APO_FLAG(pub i32);
pub const APO_FLAG_NONE: APO_FLAG = APO_FLAG(0i32);
pub const APO_FLAG_INPLACE: APO_FLAG = APO_FLAG(1i32);
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = APO_FLAG(2i32);
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = APO_FLAG(4i32);
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = APO_FLAG(8i32);
pub const APO_FLAG_MIXER: APO_FLAG = APO_FLAG(16i32);
pub const APO_FLAG_DEFAULT: APO_FLAG = APO_FLAG(14i32);
impl ::core::convert::From<i32> for APO_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APO_FLAG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APO_LOG_LEVEL(pub i32);
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = APO_LOG_LEVEL(0i32);
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = APO_LOG_LEVEL(1i32);
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = APO_LOG_LEVEL(2i32);
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = APO_LOG_LEVEL(3i32);
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = APO_LOG_LEVEL(4i32);
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = APO_LOG_LEVEL(5i32);
impl ::core::convert::From<i32> for APO_LOG_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APO_LOG_LEVEL {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl APO_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APO_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APO_NOTIFICATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APO_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for APO_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION>,
    pub audioEndpointPropertyChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION>,
    pub audioSystemEffectsPropertyChange: ::core::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl APO_NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APO_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APO_NOTIFICATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APO_NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for APO_NOTIFICATION_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for APO_NOTIFICATION_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioEndpointPropertyChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioSystemEffectsPropertyChange: ::core::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
}
impl APO_NOTIFICATION_DESCRIPTOR_0 {}
impl ::core::default::Default for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for APO_NOTIFICATION_DESCRIPTOR_0 {}
unsafe impl ::windows::runtime::Abi for APO_NOTIFICATION_DESCRIPTOR_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APO_NOTIFICATION_TYPE(pub i32);
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(0i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(1i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(2i32);
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(3i32);
impl ::core::convert::From<i32> for APO_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APO_NOTIFICATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
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
impl ::core::default::Default for APO_REG_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for APO_REG_PROPERTIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for APO_REG_PROPERTIES {
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
impl ::core::cmp::Eq for APO_REG_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for APO_REG_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
}
impl AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub propertyStore: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("propertyStore", &self.propertyStore).field("propertyKey", &self.propertyKey).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.propertyStore == other.propertyStore && self.propertyKey == other.propertyKey
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::runtime::Abi for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
}
impl AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`*"]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("volume", &self.volume).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.volume == other.volume
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIO_FLOW_TYPE(pub i32);
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(0i32);
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(1i32);
impl ::core::convert::From<i32> for AUDIO_FLOW_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUDIO_FLOW_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`*"]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: ::windows::runtime::GUID,
    pub canSetState: super::super::super::Foundation::BOOL,
    pub state: AUDIO_SYSTEMEFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl AUDIO_SYSTEMEFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_SYSTEMEFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_SYSTEMEFFECT").field("id", &self.id).field("canSetState", &self.canSetState).field("state", &self.state).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.canSetState == other.canSetState && self.state == other.state
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AUDIO_SYSTEMEFFECT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
    pub propertyStoreContext: ::windows::runtime::GUID,
}
impl AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).field("propertyStoreContext", &self.propertyStoreContext).finish()
    }
}
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device && self.propertyStoreContext == other.propertyStoreContext
    }
}
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub propertyStoreContext: ::windows::runtime::GUID,
    pub propertyStoreType: super::__MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002,
    pub propertyStore: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION")
            .field("endpoint", &self.endpoint)
            .field("propertyStoreContext", &self.propertyStoreContext)
            .field("propertyStoreType", &self.propertyStoreType)
            .field("propertyStore", &self.propertyStore)
            .field("propertyKey", &self.propertyKey)
            .finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint && self.propertyStoreContext == other.propertyStoreContext && self.propertyStoreType == other.propertyStoreType && self.propertyStore == other.propertyStore && self.propertyKey == other.propertyKey
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::runtime::Abi for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIO_SYSTEMEFFECT_STATE(pub i32);
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(0i32);
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(1i32);
impl ::core::convert::From<i32> for AUDIO_SYSTEMEFFECT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUDIO_SYSTEMEFFECT_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
pub struct AudioFXExtensionParams {
    pub AddPageParam: super::super::super::Foundation::LPARAM,
    pub pwstrEndpointID: super::super::super::Foundation::PWSTR,
    pub pFxProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl AudioFXExtensionParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for AudioFXExtensionParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for AudioFXExtensionParams {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AudioFXExtensionParams").field("AddPageParam", &self.AddPageParam).field("pwstrEndpointID", &self.pwstrEndpointID).field("pFxProperties", &self.pFxProperties).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for AudioFXExtensionParams {
    fn eq(&self, other: &Self) -> bool {
        self.AddPageParam == other.AddPageParam && self.pwstrEndpointID == other.pwstrEndpointID && self.pFxProperties == other.pFxProperties
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for AudioFXExtensionParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::runtime::Abi for AudioFXExtensionParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EAudioConstriction(pub i32);
pub const eAudioConstrictionOff: EAudioConstriction = EAudioConstriction(0i32);
pub const eAudioConstriction48_16: EAudioConstriction = EAudioConstriction(1i32);
pub const eAudioConstriction44_16: EAudioConstriction = EAudioConstriction(2i32);
pub const eAudioConstriction14_14: EAudioConstriction = EAudioConstriction(3i32);
pub const eAudioConstrictionMute: EAudioConstriction = EAudioConstriction(4i32);
impl ::core::convert::From<i32> for EAudioConstriction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAudioConstriction {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub type FNAPONOTIFICATIONCALLBACK = unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IApoAcousticEchoCancellation(pub ::windows::runtime::IUnknown);
impl IApoAcousticEchoCancellation {}
unsafe impl ::windows::runtime::Interface for IApoAcousticEchoCancellation {
    type Vtable = IApoAcousticEchoCancellation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(624449369, 12854, 16641, [169, 67, 37, 105, 61, 251, 93, 45]);
}
impl ::core::convert::From<IApoAcousticEchoCancellation> for ::windows::runtime::IUnknown {
    fn from(value: IApoAcousticEchoCancellation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IApoAcousticEchoCancellation> for ::windows::runtime::IUnknown {
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
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IApoAuxiliaryInputConfiguration(pub ::windows::runtime::IUnknown);
impl IApoAuxiliaryInputConfiguration {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn AddAuxiliaryInput(&self, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pbydata), ::core::mem::transmute(pinputconnection)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn RemoveAuxiliaryInput(&self, dwinputid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputid)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn IsInputFormatSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioMediaType>>(&self, prequestedinputformat: Param0) -> ::windows::runtime::Result<IAudioMediaType> {
        let mut result__: <IAudioMediaType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), prequestedinputformat.into_param().abi(), &mut result__).from_abi::<IAudioMediaType>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IApoAuxiliaryInputConfiguration {
    type Vtable = IApoAuxiliaryInputConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1290472107, 64025, 18669, [168, 87, 135, 119, 26, 225, 183, 104]);
}
impl ::core::convert::From<IApoAuxiliaryInputConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: IApoAuxiliaryInputConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IApoAuxiliaryInputConfiguration> for ::windows::runtime::IUnknown {
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const ::core::mem::ManuallyDrop<APO_CONNECTION_DESCRIPTOR>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequestedinputformat: ::windows::runtime::RawPtr, ppsupportedinputformat: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IApoAuxiliaryInputRT(pub ::windows::runtime::IUnknown);
impl IApoAuxiliaryInputRT {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn AcceptInput(&self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputid), ::core::mem::transmute(pinputconnection)))
    }
}
unsafe impl ::windows::runtime::Interface for IApoAuxiliaryInputRT {
    type Vtable = IApoAuxiliaryInputRT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4166090908, 49527, 18848, [177, 178, 182, 111, 1, 121, 67, 171]);
}
impl ::core::convert::From<IApoAuxiliaryInputRT> for ::windows::runtime::IUnknown {
    fn from(value: IApoAuxiliaryInputRT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IApoAuxiliaryInputRT> for ::windows::runtime::IUnknown {
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY),
);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioDeviceModulesClient(pub ::windows::runtime::IUnknown);
impl IAudioDeviceModulesClient {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn SetAudioDeviceModulesManager<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, paudiodevicemodulesmanager: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), paudiodevicemodulesmanager.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioDeviceModulesClient {
    type Vtable = IAudioDeviceModulesClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2566094252, 53430, 18933, [137, 106, 170, 77, 22, 154, 76, 72]);
}
impl ::core::convert::From<IAudioDeviceModulesClient> for ::windows::runtime::IUnknown {
    fn from(value: IAudioDeviceModulesClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioDeviceModulesClient> for ::windows::runtime::IUnknown {
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
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioMediaType(pub ::windows::runtime::IUnknown);
impl IAudioMediaType {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`*"]
    pub unsafe fn IsCompressedFormat(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioMediaType>>(&self, piaudiotype: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), piaudiotype.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetAudioFormat(&self) -> *mut super::WAVEFORMATEX {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetUncompressedAudioFormat(&self) -> ::windows::runtime::Result<UNCOMPRESSEDAUDIOFORMAT> {
        let mut result__: <UNCOMPRESSEDAUDIOFORMAT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<UNCOMPRESSEDAUDIOFORMAT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioMediaType {
    type Vtable = IAudioMediaType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1318682483, 46879, 18328, [135, 59, 237, 125, 252, 241, 91, 77]);
}
impl ::core::convert::From<IAudioMediaType> for ::windows::runtime::IUnknown {
    fn from(value: IAudioMediaType) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioMediaType> for ::windows::runtime::IUnknown {
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> *mut super::WAVEFORMATEX,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioProcessingObject(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObject {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetLatency(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetRegistrationProperties(&self) -> ::windows::runtime::Result<*mut APO_REG_PROPERTIES> {
        let mut result__: <*mut APO_REG_PROPERTIES as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut APO_REG_PROPERTIES>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn Initialize(&self, cbdatasize: u32, pbydata: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pbydata)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn IsInputFormatSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioMediaType>, Param1: ::windows::runtime::IntoParam<'a, IAudioMediaType>>(&self, poppositeformat: Param0, prequestedinputformat: Param1) -> ::windows::runtime::Result<IAudioMediaType> {
        let mut result__: <IAudioMediaType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), poppositeformat.into_param().abi(), prequestedinputformat.into_param().abi(), &mut result__).from_abi::<IAudioMediaType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn IsOutputFormatSupported<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioMediaType>, Param1: ::windows::runtime::IntoParam<'a, IAudioMediaType>>(&self, poppositeformat: Param0, prequestedoutputformat: Param1) -> ::windows::runtime::Result<IAudioMediaType> {
        let mut result__: <IAudioMediaType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), poppositeformat.into_param().abi(), prequestedoutputformat.into_param().abi(), &mut result__).from_abi::<IAudioMediaType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetInputChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObject {
    type Vtable = IAudioProcessingObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4252969769, 9424, 19292, [177, 119, 89, 44, 57, 249, 202, 16]);
}
impl ::core::convert::From<IAudioProcessingObject> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioProcessingObject> for ::windows::runtime::IUnknown {
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
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioProcessingObjectConfiguration(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectConfiguration {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn LockForProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32numinputconnections), ::core::mem::transmute(ppinputconnections), ::core::mem::transmute(u32numoutputconnections), ::core::mem::transmute(ppoutputconnections)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn UnlockForProcess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectConfiguration {
    type Vtable = IAudioProcessingObjectConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(241096709, 43942, 18883, [143, 154, 43, 140, 136, 156, 79, 168]);
}
impl ::core::convert::From<IAudioProcessingObjectConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioProcessingObjectConfiguration> for ::windows::runtime::IUnknown {
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
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioProcessingObjectLoggingService(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectLoggingService {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`*"]
    pub unsafe fn ApoLog<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, level: APO_LOG_LEVEL, format: Param1) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), format.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectLoggingService {
    type Vtable = IAudioProcessingObjectLoggingService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1770979591, 5957, 18184, [149, 165, 216, 68, 120, 166, 42, 101]);
}
impl ::core::convert::From<IAudioProcessingObjectLoggingService> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectLoggingService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioProcessingObjectLoggingService> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioProcessingObjectLoggingService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioProcessingObjectLoggingService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioProcessingObjectLoggingService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectLoggingService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: APO_LOG_LEVEL, format: super::super::super::Foundation::PWSTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioProcessingObjectNotifications(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectNotifications {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetApoNotificationRegistrationInfo(&self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(aponotifications), ::core::mem::transmute(count)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn HandleNotification(&self, aponotification: *const APO_NOTIFICATION) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(aponotification)))
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectNotifications {
    type Vtable = IAudioProcessingObjectNotifications_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1454425967, 765, 19233, [165, 46, 159, 130, 25, 252, 134, 228]);
}
impl ::core::convert::From<IAudioProcessingObjectNotifications> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectNotifications) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioProcessingObjectNotifications> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioProcessingObjectNotifications) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioProcessingObjectNotifications {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioProcessingObjectNotifications {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectNotifications_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aponotification: *const ::core::mem::ManuallyDrop<APO_NOTIFICATION>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioProcessingObjectRT(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectRT {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn APOProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32numinputconnections), ::core::mem::transmute(ppinputconnections), ::core::mem::transmute(u32numoutputconnections), ::core::mem::transmute(ppoutputconnections)))
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn CalcInputFrames(&self, u32outputframecount: u32) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32outputframecount)))
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn CalcOutputFrames(&self, u32inputframecount: u32) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32inputframecount)))
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectRT {
    type Vtable = IAudioProcessingObjectRT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2652727917, 56764, 20117, [164, 199, 173, 100, 186, 55, 132, 108]);
}
impl ::core::convert::From<IAudioProcessingObjectRT> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectRT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioProcessingObjectRT> for ::windows::runtime::IUnknown {
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32outputframecount: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32inputframecount: u32) -> u32,
);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioProcessingObjectRTQueueService(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectRTQueueService {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetRealTimeWorkQueue(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectRTQueueService {
    type Vtable = IAudioProcessingObjectRTQueueService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2899729967, 38235, 19287, [185, 191, 172, 41, 123, 183, 82, 201]);
}
impl ::core::convert::From<IAudioProcessingObjectRTQueueService> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectRTQueueService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioProcessingObjectRTQueueService> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioProcessingObjectRTQueueService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioProcessingObjectRTQueueService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioProcessingObjectRTQueueService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectRTQueueService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, workqueueid: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioProcessingObjectVBR(pub ::windows::runtime::IUnknown);
impl IAudioProcessingObjectVBR {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn CalcMaxInputFrames(&self, u32maxoutputframecount: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32maxoutputframecount), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn CalcMaxOutputFrames(&self, u32maxinputframecount: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32maxinputframecount), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioProcessingObjectVBR {
    type Vtable = IAudioProcessingObjectVBR_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2074205071, 30893, 18893, [149, 145, 247, 157, 128, 161, 124, 129]);
}
impl ::core::convert::From<IAudioProcessingObjectVBR> for ::windows::runtime::IUnknown {
    fn from(value: IAudioProcessingObjectVBR) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioProcessingObjectVBR> for ::windows::runtime::IUnknown {
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
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSystemEffects(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffects {}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffects {
    type Vtable = IAudioSystemEffects_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1604325159, 44502, 18842, [138, 157, 107, 152, 82, 31, 167, 91]);
}
impl ::core::convert::From<IAudioSystemEffects> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffects) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSystemEffects> for ::windows::runtime::IUnknown {
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
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSystemEffects2(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffects2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`*"]
    pub unsafe fn GetEffectsList<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, ppeffectsids: *mut *mut ::windows::runtime::GUID, pceffects: *mut u32, event: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppeffectsids), ::core::mem::transmute(pceffects), event.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffects2 {
    type Vtable = IAudioSystemEffects2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3137247698, 29750, 17614, [158, 14, 77, 137, 175, 191, 255, 86]);
}
impl ::core::convert::From<IAudioSystemEffects2> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffects2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSystemEffects2> for ::windows::runtime::IUnknown {
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
impl ::core::convert::From<IAudioSystemEffects2> for IAudioSystemEffects {
    fn from(value: IAudioSystemEffects2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects2> for IAudioSystemEffects {
    fn from(value: &IAudioSystemEffects2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSystemEffects> for IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSystemEffects> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSystemEffects> for &IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSystemEffects> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSystemEffects3(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffects3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`*"]
    pub unsafe fn GetEffectsList<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, ppeffectsids: *mut *mut ::windows::runtime::GUID, pceffects: *mut u32, event: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppeffectsids), ::core::mem::transmute(pceffects), event.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`*"]
    pub unsafe fn GetControllableSystemEffectsList<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(effects), ::core::mem::transmute(numeffects), event.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn SetAudioSystemEffectState<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, effectid: Param0, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), effectid.into_param().abi(), ::core::mem::transmute(state)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffects3 {
    type Vtable = IAudioSystemEffects3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3314233805, 64618, 16981, [188, 31, 173, 41, 187, 10, 74, 23]);
}
impl ::core::convert::From<IAudioSystemEffects3> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffects3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSystemEffects3> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSystemEffects3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioSystemEffects3> for IAudioSystemEffects2 {
    fn from(value: IAudioSystemEffects3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects3> for IAudioSystemEffects2 {
    fn from(value: &IAudioSystemEffects3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSystemEffects2> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSystemEffects2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSystemEffects2> for &IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSystemEffects2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioSystemEffects3> for IAudioSystemEffects {
    fn from(value: IAudioSystemEffects3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects3> for IAudioSystemEffects {
    fn from(value: &IAudioSystemEffects3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSystemEffects> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSystemEffects> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSystemEffects> for &IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSystemEffects> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppeffectsids: *mut *mut ::windows::runtime::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effectid: ::windows::runtime::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSystemEffectsCustomFormats(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffectsCustomFormats {
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetFormatCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
    pub unsafe fn GetFormat(&self, nformat: u32) -> ::windows::runtime::Result<IAudioMediaType> {
        let mut result__: <IAudioMediaType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nformat), &mut result__).from_abi::<IAudioMediaType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_Foundation`*"]
    pub unsafe fn GetFormatRepresentation(&self, nformat: u32) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nformat), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffectsCustomFormats {
    type Vtable = IAudioSystemEffectsCustomFormats_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2971102772, 47999, 20229, [190, 189, 27, 24, 165, 52, 224, 151]);
}
impl ::core::convert::From<IAudioSystemEffectsCustomFormats> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffectsCustomFormats) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSystemEffectsCustomFormats> for ::windows::runtime::IUnknown {
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
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_APO_SWFallback_ProcessingModes: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CompositeFX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CompositeFX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CompositeFX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_Association: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_FriendlyName: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_PostMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_PreMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FX_UserInterfaceClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494774182, 22859, 20406, [168, 13, 1, 175, 94, 237, 125, 29]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Apo`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3550034495, 39362, 17410, [181, 236, 169, 42, 3, 103, 102, 75]),
    pid: 5u32,
};
pub const SID_AudioProcessingObjectLoggingService: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2340423855, 2553, 17774, [161, 115, 189, 181, 132, 153, 188, 231]);
pub const SID_AudioProcessingObjectRTQueue: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1166809631, 26777, 19474, [153, 172, 226, 230, 172, 37, 49, 4]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Apo`*"]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows::runtime::GUID,
    pub dwSamplesPerFrame: u32,
    pub dwBytesPerSampleContainer: u32,
    pub dwValidBitsPerSample: u32,
    pub fFramesPerSecond: f32,
    pub dwChannelMask: u32,
}
impl UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::default::Default for UNCOMPRESSEDAUDIOFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for UNCOMPRESSEDAUDIOFORMAT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for UNCOMPRESSEDAUDIOFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.guidFormatType == other.guidFormatType && self.dwSamplesPerFrame == other.dwSamplesPerFrame && self.dwBytesPerSampleContainer == other.dwBytesPerSampleContainer && self.dwValidBitsPerSample == other.dwValidBitsPerSample && self.fFramesPerSecond == other.fFramesPerSecond && self.dwChannelMask == other.dwChannelMask
    }
}
impl ::core::cmp::Eq for UNCOMPRESSEDAUDIOFORMAT {}
unsafe impl ::windows::runtime::Abi for UNCOMPRESSEDAUDIOFORMAT {
    type Abi = Self;
}
