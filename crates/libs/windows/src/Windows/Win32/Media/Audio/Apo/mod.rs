#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073919i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_ALREADY_UNLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073914i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_APO_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073910i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_BUFFERS_OVERLAP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073915i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073917i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_APO_CLSID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073916i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_COEFFCOUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073909i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_COEFFICIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073908i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_CONNECTION_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073911i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_CURVE_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073907i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_INPUTID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073906i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073912i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073918i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APOERR_NUM_CONNECTIONS_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2005073913i32);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: ::windows::core::GUID,
}
impl ::core::marker::Copy for APOInitBaseStruct {}
impl ::core::clone::Clone for APOInitBaseStruct {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APOInitBaseStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitBaseStruct").field("cbSize", &self.cbSize).field("clsid", &self.clsid).finish()
    }
}
unsafe impl ::windows::core::Abi for APOInitBaseStruct {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APOInitBaseStruct {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APOInitBaseStruct>()) == 0 }
    }
}
impl ::core::cmp::Eq for APOInitBaseStruct {}
impl ::core::default::Default for APOInitBaseStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pAPOSystemEffectsProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for APOInitSystemEffects {
    fn clone(&self) -> Self {
        Self {
            APOInit: self.APOInit,
            pAPOEndpointProperties: self.pAPOEndpointProperties.clone(),
            pAPOSystemEffectsProperties: self.pAPOSystemEffectsProperties.clone(),
            pReserved: self.pReserved,
            pDeviceCollection: self.pDeviceCollection.clone(),
        }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for APOInitSystemEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects").field("APOInit", &self.APOInit).field("pAPOEndpointProperties", &self.pAPOEndpointProperties).field("pAPOSystemEffectsProperties", &self.pAPOSystemEffectsProperties).field("pReserved", &self.pReserved).field("pDeviceCollection", &self.pDeviceCollection).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::core::Abi for APOInitSystemEffects {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for APOInitSystemEffects {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pAPOSystemEffectsProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects2 {
    fn clone(&self) -> Self {
        Self {
            APOInit: self.APOInit,
            pAPOEndpointProperties: self.pAPOEndpointProperties.clone(),
            pAPOSystemEffectsProperties: self.pAPOSystemEffectsProperties.clone(),
            pReserved: self.pReserved,
            pDeviceCollection: self.pDeviceCollection.clone(),
            nSoftwareIoDeviceInCollection: self.nSoftwareIoDeviceInCollection,
            nSoftwareIoConnectorIndex: self.nSoftwareIoConnectorIndex,
            AudioProcessingMode: self.AudioProcessingMode,
            InitializeForDiscoveryOnly: self.InitializeForDiscoveryOnly,
        }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for APOInitSystemEffects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects2")
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
unsafe impl ::windows::core::Abi for APOInitSystemEffects2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APOInitSystemEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.APOInit == other.APOInit && self.pAPOEndpointProperties == other.pAPOEndpointProperties && self.pAPOSystemEffectsProperties == other.pAPOSystemEffectsProperties && self.pReserved == other.pReserved && self.pDeviceCollection == other.pDeviceCollection && self.nSoftwareIoDeviceInCollection == other.nSoftwareIoDeviceInCollection && self.nSoftwareIoConnectorIndex == other.nSoftwareIoConnectorIndex && self.AudioProcessingMode == other.AudioProcessingMode && self.InitializeForDiscoveryOnly == other.InitializeForDiscoveryOnly
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APOInitSystemEffects2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APOInitSystemEffects2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub pServiceProvider: ::core::option::Option<super::super::super::System::Com::IServiceProvider>,
    pub pDeviceCollection: ::core::option::Option<super::IMMDeviceCollection>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: ::windows::core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APOInitSystemEffects3 {
    fn clone(&self) -> Self {
        Self {
            APOInit: self.APOInit,
            pAPOEndpointProperties: self.pAPOEndpointProperties.clone(),
            pServiceProvider: self.pServiceProvider.clone(),
            pDeviceCollection: self.pDeviceCollection.clone(),
            nSoftwareIoDeviceInCollection: self.nSoftwareIoDeviceInCollection,
            nSoftwareIoConnectorIndex: self.nSoftwareIoConnectorIndex,
            AudioProcessingMode: self.AudioProcessingMode,
            InitializeForDiscoveryOnly: self.InitializeForDiscoveryOnly,
        }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for APOInitSystemEffects3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APOInitSystemEffects3")
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
unsafe impl ::windows::core::Abi for APOInitSystemEffects3 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for APOInitSystemEffects3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_BUFFER_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_VALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(2i32);
impl ::core::marker::Copy for APO_BUFFER_FLAGS {}
impl ::core::clone::Clone for APO_BUFFER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APO_BUFFER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_CONNECTION_BUFFER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(2i32);
impl ::core::marker::Copy for APO_CONNECTION_BUFFER_TYPE {}
impl ::core::clone::Clone for APO_CONNECTION_BUFFER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_CONNECTION_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APO_CONNECTION_BUFFER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_CONNECTION_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_CONNECTION_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: ::core::option::Option<IAudioMediaType>,
    pub u32Signature: u32,
}
impl ::core::clone::Clone for APO_CONNECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { Type: self.Type, pBuffer: self.pBuffer, u32MaxFrameCount: self.u32MaxFrameCount, pFormat: self.pFormat.clone(), u32Signature: self.u32Signature }
    }
}
impl ::core::fmt::Debug for APO_CONNECTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_DESCRIPTOR").field("Type", &self.Type).field("pBuffer", &self.pBuffer).field("u32MaxFrameCount", &self.u32MaxFrameCount).field("pFormat", &self.pFormat).field("u32Signature", &self.u32Signature).finish()
    }
}
unsafe impl ::windows::core::Abi for APO_CONNECTION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pBuffer == other.pBuffer && self.u32MaxFrameCount == other.u32MaxFrameCount && self.pFormat == other.pFormat && self.u32Signature == other.u32Signature
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_DESCRIPTOR {}
impl ::core::default::Default for APO_CONNECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
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
impl ::core::fmt::Debug for APO_CONNECTION_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_PROPERTY").field("pBuffer", &self.pBuffer).field("u32ValidFrameCount", &self.u32ValidFrameCount).field("u32BufferFlags", &self.u32BufferFlags).field("u32Signature", &self.u32Signature).finish()
    }
}
unsafe impl ::windows::core::Abi for APO_CONNECTION_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_CONNECTION_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_PROPERTY {}
impl ::core::default::Default for APO_CONNECTION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
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
impl ::core::fmt::Debug for APO_CONNECTION_PROPERTY_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_CONNECTION_PROPERTY_V2").field("property", &self.property).field("u64QPCTime", &self.u64QPCTime).finish()
    }
}
unsafe impl ::windows::core::Abi for APO_CONNECTION_PROPERTY_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APO_CONNECTION_PROPERTY_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_CONNECTION_PROPERTY_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for APO_CONNECTION_PROPERTY_V2 {}
impl ::core::default::Default for APO_CONNECTION_PROPERTY_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_NONE: APO_FLAG = APO_FLAG(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_INPLACE: APO_FLAG = APO_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = APO_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = APO_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = APO_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_MIXER: APO_FLAG = APO_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_FLAG_DEFAULT: APO_FLAG = APO_FLAG(14i32);
impl ::core::marker::Copy for APO_FLAG {}
impl ::core::clone::Clone for APO_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APO_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_LOG_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = APO_LOG_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = APO_LOG_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = APO_LOG_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = APO_LOG_LEVEL(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = APO_LOG_LEVEL(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = APO_LOG_LEVEL(5i32);
impl ::core::marker::Copy for APO_LOG_LEVEL {}
impl ::core::clone::Clone for APO_LOG_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_LOG_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APO_LOG_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_LOG_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_LOG_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        Self { r#type: self.r#type, Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::core::Abi for APO_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APO_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.Anonymous == other.Anonymous
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APO_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APO_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION>,
    pub audioEndpointPropertyChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION>,
    pub audioSystemEffectsPropertyChange: ::core::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::core::Abi for APO_NOTIFICATION_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for APO_NOTIFICATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_NOTIFICATION_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for APO_NOTIFICATION_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for APO_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { r#type: self.r#type, Anonymous: self.Anonymous.clone() }
    }
}
unsafe impl ::windows::core::Abi for APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.Anonymous == other.Anonymous
    }
}
impl ::core::cmp::Eq for APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioEndpointPropertyChange: ::core::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioSystemEffectsPropertyChange: ::core::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
}
impl ::core::clone::Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
unsafe impl ::windows::core::Abi for APO_NOTIFICATION_DESCRIPTOR_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_NOTIFICATION_DESCRIPTOR_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for APO_NOTIFICATION_DESCRIPTOR_0 {}
impl ::core::default::Default for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APO_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for APO_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for APO_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APO_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APO_NOTIFICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APO_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APO_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct APO_REG_PROPERTIES {
    pub clsid: ::windows::core::GUID,
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
    pub iidAPOInterfaceList: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for APO_REG_PROPERTIES {}
impl ::core::clone::Clone for APO_REG_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APO_REG_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APO_REG_PROPERTIES")
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
unsafe impl ::windows::core::Abi for APO_REG_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APO_REG_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APO_REG_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for APO_REG_PROPERTIES {}
impl ::core::default::Default for APO_REG_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
}
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { device: self.device.clone() }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
unsafe impl ::windows::core::Abi for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub propertyStore: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        Self { endpoint: self.endpoint.clone(), propertyStore: self.propertyStore.clone(), propertyKey: self.propertyKey }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("propertyStore", &self.propertyStore).field("propertyKey", &self.propertyKey).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::core::Abi for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
}
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { device: self.device.clone() }
    }
}
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).finish()
    }
}
unsafe impl ::windows::core::Abi for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        Self { endpoint: self.endpoint.clone(), volume: self.volume }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("volume", &self.volume).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_FLOW_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(1i32);
impl ::core::marker::Copy for AUDIO_FLOW_TYPE {}
impl ::core::clone::Clone for AUDIO_FLOW_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_FLOW_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_FLOW_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIO_FLOW_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_FLOW_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: ::windows::core::GUID,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECT").field("id", &self.id).field("canSetState", &self.canSetState).field("state", &self.state).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUDIO_SYSTEMEFFECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIO_SYSTEMEFFECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_SYSTEMEFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: ::core::option::Option<super::IMMDevice>,
    pub propertyStoreContext: ::windows::core::GUID,
}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        Self { device: self.device.clone(), propertyStoreContext: self.propertyStoreContext }
    }
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR").field("device", &self.device).field("propertyStoreContext", &self.propertyStoreContext).finish()
    }
}
unsafe impl ::windows::core::Abi for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.device == other.device && self.propertyStoreContext == other.propertyStoreContext
    }
}
impl ::core::cmp::Eq for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {}
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: ::core::option::Option<super::IMMDevice>,
    pub propertyStoreContext: ::windows::core::GUID,
    pub propertyStoreType: super::__MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002,
    pub propertyStore: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        Self {
            endpoint: self.endpoint.clone(),
            propertyStoreContext: self.propertyStoreContext,
            propertyStoreType: self.propertyStoreType,
            propertyStore: self.propertyStore.clone(),
            propertyKey: self.propertyKey,
        }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION").field("endpoint", &self.endpoint).field("propertyStoreContext", &self.propertyStoreContext).field("propertyStoreType", &self.propertyStoreType).field("propertyStore", &self.propertyStore).field("propertyKey", &self.propertyKey).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::core::Abi for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_SYSTEMEFFECT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(1i32);
impl ::core::marker::Copy for AUDIO_SYSTEMEFFECT_STATE {}
impl ::core::clone::Clone for AUDIO_SYSTEMEFFECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_SYSTEMEFFECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_SYSTEMEFFECT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIO_SYSTEMEFFECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_SYSTEMEFFECT_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub struct AudioFXExtensionParams {
    pub AddPageParam: super::super::super::Foundation::LPARAM,
    pub pwstrEndpointID: ::windows::core::PWSTR,
    pub pFxProperties: ::core::option::Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::clone::Clone for AudioFXExtensionParams {
    fn clone(&self) -> Self {
        Self { AddPageParam: self.AddPageParam, pwstrEndpointID: self.pwstrEndpointID, pFxProperties: self.pFxProperties.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for AudioFXExtensionParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioFXExtensionParams").field("AddPageParam", &self.AddPageParam).field("pwstrEndpointID", &self.pwstrEndpointID).field("pFxProperties", &self.pFxProperties).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
unsafe impl ::windows::core::Abi for AudioFXExtensionParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for AudioFXExtensionParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EAudioConstriction(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstrictionOff: EAudioConstriction = EAudioConstriction(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction48_16: EAudioConstriction = EAudioConstriction(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction44_16: EAudioConstriction = EAudioConstriction(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstriction14_14: EAudioConstriction = EAudioConstriction(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub const eAudioConstrictionMute: EAudioConstriction = EAudioConstriction(4i32);
impl ::core::marker::Copy for EAudioConstriction {}
impl ::core::clone::Clone for EAudioConstriction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAudioConstriction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAudioConstriction {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAudioConstriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAudioConstriction").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub type FNAPONOTIFICATIONCALLBACK = ::core::option::Option<unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IApoAcousticEchoCancellation(::windows::core::IUnknown);
impl IApoAcousticEchoCancellation {}
impl ::core::convert::From<IApoAcousticEchoCancellation> for ::windows::core::IUnknown {
    fn from(value: IApoAcousticEchoCancellation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApoAcousticEchoCancellation> for ::windows::core::IUnknown {
    fn from(value: &IApoAcousticEchoCancellation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IApoAcousticEchoCancellation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IApoAcousticEchoCancellation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IApoAcousticEchoCancellation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApoAcousticEchoCancellation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAcousticEchoCancellation {}
impl ::core::fmt::Debug for IApoAcousticEchoCancellation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAcousticEchoCancellation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApoAcousticEchoCancellation {
    type Vtable = IApoAcousticEchoCancellation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25385759_3236_4101_a943_25693dfb5d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAcousticEchoCancellation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IApoAuxiliaryInputConfiguration(::windows::core::IUnknown);
impl IApoAuxiliaryInputConfiguration {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn AddAuxiliaryInput(&self, dwinputid: u32, pbydata: &[u8], pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddAuxiliaryInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputid), pbydata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbydata)), ::core::mem::transmute(pinputconnection)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn RemoveAuxiliaryInput(&self, dwinputid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAuxiliaryInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn IsInputFormatSupported<'a, Param0: ::windows::core::IntoParam<'a, IAudioMediaType>>(&self, prequestedinputformat: Param0) -> ::windows::core::Result<IAudioMediaType> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsInputFormatSupported)(::core::mem::transmute_copy(self), prequestedinputformat.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IAudioMediaType>(result__)
    }
}
impl ::core::convert::From<IApoAuxiliaryInputConfiguration> for ::windows::core::IUnknown {
    fn from(value: IApoAuxiliaryInputConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApoAuxiliaryInputConfiguration> for ::windows::core::IUnknown {
    fn from(value: &IApoAuxiliaryInputConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IApoAuxiliaryInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IApoAuxiliaryInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IApoAuxiliaryInputConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApoAuxiliaryInputConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAuxiliaryInputConfiguration {}
impl ::core::fmt::Debug for IApoAuxiliaryInputConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAuxiliaryInputConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApoAuxiliaryInputConfiguration {
    type Vtable = IApoAuxiliaryInputConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ceb0aab_fa19_48ed_a857_87771ae1b768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAuxiliaryInputConfiguration_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddAuxiliaryInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT,
    pub RemoveAuxiliaryInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32) -> ::windows::core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IApoAuxiliaryInputRT(::windows::core::IUnknown);
impl IApoAuxiliaryInputRT {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn AcceptInput(&self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
        (::windows::core::Interface::vtable(self).AcceptInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputid), ::core::mem::transmute(pinputconnection))
    }
}
impl ::core::convert::From<IApoAuxiliaryInputRT> for ::windows::core::IUnknown {
    fn from(value: IApoAuxiliaryInputRT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApoAuxiliaryInputRT> for ::windows::core::IUnknown {
    fn from(value: &IApoAuxiliaryInputRT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IApoAuxiliaryInputRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IApoAuxiliaryInputRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IApoAuxiliaryInputRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApoAuxiliaryInputRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApoAuxiliaryInputRT {}
impl ::core::fmt::Debug for IApoAuxiliaryInputRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApoAuxiliaryInputRT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IApoAuxiliaryInputRT {
    type Vtable = IApoAuxiliaryInputRT_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf851809c_c177_49a0_b1b2_b66f017943ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApoAuxiliaryInputRT_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AcceptInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY),
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioDeviceModulesClient(::windows::core::IUnknown);
impl IAudioDeviceModulesClient {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn SetAudioDeviceModulesManager<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, paudiodevicemodulesmanager: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAudioDeviceModulesManager)(::core::mem::transmute_copy(self), paudiodevicemodulesmanager.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioDeviceModulesClient> for ::windows::core::IUnknown {
    fn from(value: IAudioDeviceModulesClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioDeviceModulesClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioDeviceModulesClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioDeviceModulesClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioDeviceModulesClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioDeviceModulesClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioDeviceModulesClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioDeviceModulesClient {}
impl ::core::fmt::Debug for IAudioDeviceModulesClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioDeviceModulesClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioDeviceModulesClient {
    type Vtable = IAudioDeviceModulesClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98f37dac_d0b6_49f5_896a_aa4d169a4c48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceModulesClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetAudioDeviceModulesManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioMediaType(::windows::core::IUnknown);
impl IAudioMediaType {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCompressedFormat(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsCompressedFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IAudioMediaType>>(&self, piaudiotype: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsEqual)(::core::mem::transmute_copy(self), piaudiotype.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetAudioFormat(&self) -> *mut super::WAVEFORMATEX {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetAudioFormat)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetUncompressedAudioFormat(&self) -> ::windows::core::Result<UNCOMPRESSEDAUDIOFORMAT> {
        let mut result__: UNCOMPRESSEDAUDIOFORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUncompressedAudioFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<UNCOMPRESSEDAUDIOFORMAT>(result__)
    }
}
impl ::core::convert::From<IAudioMediaType> for ::windows::core::IUnknown {
    fn from(value: IAudioMediaType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioMediaType> for ::windows::core::IUnknown {
    fn from(value: &IAudioMediaType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioMediaType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioMediaType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioMediaType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioMediaType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMediaType {}
impl ::core::fmt::Debug for IAudioMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMediaType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioMediaType {
    type Vtable = IAudioMediaType_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e997f73_b71f_4798_873b_ed7dfcf15b4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMediaType_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCompressedFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCompressedFormat: usize,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piaudiotype: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut super::WAVEFORMATEX,
    pub GetUncompressedAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObject(::windows::core::IUnknown);
impl IAudioProcessingObject {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetRegistrationProperties(&self) -> ::windows::core::Result<*mut APO_REG_PROPERTIES> {
        let mut result__: *mut APO_REG_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegistrationProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut APO_REG_PROPERTIES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn Initialize(&self, pbydata: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), pbydata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbydata))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn IsInputFormatSupported<'a, Param0: ::windows::core::IntoParam<'a, IAudioMediaType>, Param1: ::windows::core::IntoParam<'a, IAudioMediaType>>(&self, poppositeformat: Param0, prequestedinputformat: Param1) -> ::windows::core::Result<IAudioMediaType> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsInputFormatSupported)(::core::mem::transmute_copy(self), poppositeformat.into_param().abi(), prequestedinputformat.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IAudioMediaType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn IsOutputFormatSupported<'a, Param0: ::windows::core::IntoParam<'a, IAudioMediaType>, Param1: ::windows::core::IntoParam<'a, IAudioMediaType>>(&self, poppositeformat: Param0, prequestedoutputformat: Param1) -> ::windows::core::Result<IAudioMediaType> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsOutputFormatSupported)(::core::mem::transmute_copy(self), poppositeformat.into_param().abi(), prequestedoutputformat.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IAudioMediaType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetInputChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioProcessingObject> for ::windows::core::IUnknown {
    fn from(value: IAudioProcessingObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObject> for ::windows::core::IUnknown {
    fn from(value: &IAudioProcessingObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioProcessingObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioProcessingObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObject {}
impl ::core::fmt::Debug for IAudioProcessingObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObject {
    type Vtable = IAudioProcessingObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd7f2b29_24d0_4b5c_b177_592c39f9ca10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT,
    pub GetRegistrationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> ::windows::core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsOutputFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedoutputformat: ::windows::core::RawPtr, ppsupportedoutputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetInputChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectConfiguration(::windows::core::IUnknown);
impl IAudioProcessingObjectConfiguration {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn LockForProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockForProcess)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32numinputconnections), ::core::mem::transmute(ppinputconnections), ::core::mem::transmute(u32numoutputconnections), ::core::mem::transmute(ppoutputconnections)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn UnlockForProcess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockForProcess)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IAudioProcessingObjectConfiguration> for ::windows::core::IUnknown {
    fn from(value: IAudioProcessingObjectConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectConfiguration> for ::windows::core::IUnknown {
    fn from(value: &IAudioProcessingObjectConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioProcessingObjectConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioProcessingObjectConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectConfiguration {}
impl ::core::fmt::Debug for IAudioProcessingObjectConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectConfiguration {
    type Vtable = IAudioProcessingObjectConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e5ed805_aba6_49c3_8f9a_2b8c889c4fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectConfiguration_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub LockForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT,
    pub UnlockForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectLoggingService(::windows::core::IUnknown);
impl IAudioProcessingObjectLoggingService {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn ApoLog<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, level: APO_LOG_LEVEL, format: Param1) {
        (::windows::core::Interface::vtable(self).ApoLog)(::core::mem::transmute_copy(self), ::core::mem::transmute(level), format.into_param().abi())
    }
}
impl ::core::convert::From<IAudioProcessingObjectLoggingService> for ::windows::core::IUnknown {
    fn from(value: IAudioProcessingObjectLoggingService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectLoggingService> for ::windows::core::IUnknown {
    fn from(value: &IAudioProcessingObjectLoggingService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioProcessingObjectLoggingService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioProcessingObjectLoggingService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectLoggingService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectLoggingService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectLoggingService {}
impl ::core::fmt::Debug for IAudioProcessingObjectLoggingService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectLoggingService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectLoggingService {
    type Vtable = IAudioProcessingObjectLoggingService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x698f0107_1745_4708_95a5_d84478a62a65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectLoggingService_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ApoLog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: APO_LOG_LEVEL, format: ::windows::core::PCWSTR),
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectNotifications(::windows::core::IUnknown);
impl IAudioProcessingObjectNotifications {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetApoNotificationRegistrationInfo(&self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetApoNotificationRegistrationInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(aponotifications), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn HandleNotification(&self, aponotification: *const APO_NOTIFICATION) {
        (::windows::core::Interface::vtable(self).HandleNotification)(::core::mem::transmute_copy(self), ::core::mem::transmute(aponotification))
    }
}
impl ::core::convert::From<IAudioProcessingObjectNotifications> for ::windows::core::IUnknown {
    fn from(value: IAudioProcessingObjectNotifications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectNotifications> for ::windows::core::IUnknown {
    fn from(value: &IAudioProcessingObjectNotifications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioProcessingObjectNotifications {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioProcessingObjectNotifications {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectNotifications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectNotifications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectNotifications {}
impl ::core::fmt::Debug for IAudioProcessingObjectNotifications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectNotifications").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectNotifications {
    type Vtable = IAudioProcessingObjectNotifications_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56b0c76f_02fd_4b21_a52e_9f8219fc86e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectNotifications_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetApoNotificationRegistrationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub HandleNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aponotification: *const APO_NOTIFICATION),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    HandleNotification: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectRT(::windows::core::IUnknown);
impl IAudioProcessingObjectRT {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn APOProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
        (::windows::core::Interface::vtable(self).APOProcess)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32numinputconnections), ::core::mem::transmute(ppinputconnections), ::core::mem::transmute(u32numoutputconnections), ::core::mem::transmute(ppoutputconnections))
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn CalcInputFrames(&self, u32outputframecount: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).CalcInputFrames)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32outputframecount)))
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn CalcOutputFrames(&self, u32inputframecount: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).CalcOutputFrames)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32inputframecount)))
    }
}
impl ::core::convert::From<IAudioProcessingObjectRT> for ::windows::core::IUnknown {
    fn from(value: IAudioProcessingObjectRT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectRT> for ::windows::core::IUnknown {
    fn from(value: &IAudioProcessingObjectRT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioProcessingObjectRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioProcessingObjectRT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectRT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectRT {}
impl ::core::fmt::Debug for IAudioProcessingObjectRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectRT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectRT {
    type Vtable = IAudioProcessingObjectRT_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e1d6a6d_ddbc_4e95_a4c7_ad64ba37846c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectRT_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub APOProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY),
    pub CalcInputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32outputframecount: u32) -> u32,
    pub CalcOutputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32inputframecount: u32) -> u32,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectRTQueueService(::windows::core::IUnknown);
impl IAudioProcessingObjectRTQueueService {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetRealTimeWorkQueue(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRealTimeWorkQueue)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioProcessingObjectRTQueueService> for ::windows::core::IUnknown {
    fn from(value: IAudioProcessingObjectRTQueueService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectRTQueueService> for ::windows::core::IUnknown {
    fn from(value: &IAudioProcessingObjectRTQueueService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioProcessingObjectRTQueueService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioProcessingObjectRTQueueService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectRTQueueService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectRTQueueService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectRTQueueService {}
impl ::core::fmt::Debug for IAudioProcessingObjectRTQueueService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectRTQueueService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectRTQueueService {
    type Vtable = IAudioProcessingObjectRTQueueService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacd65e2f_955b_4b57_b9bf_ac297bb752c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectRTQueueService_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetRealTimeWorkQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workqueueid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioProcessingObjectVBR(::windows::core::IUnknown);
impl IAudioProcessingObjectVBR {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn CalcMaxInputFrames(&self, u32maxoutputframecount: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CalcMaxInputFrames)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32maxoutputframecount), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn CalcMaxOutputFrames(&self, u32maxinputframecount: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CalcMaxOutputFrames)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32maxinputframecount), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioProcessingObjectVBR> for ::windows::core::IUnknown {
    fn from(value: IAudioProcessingObjectVBR) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioProcessingObjectVBR> for ::windows::core::IUnknown {
    fn from(value: &IAudioProcessingObjectVBR) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioProcessingObjectVBR {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioProcessingObjectVBR {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioProcessingObjectVBR {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioProcessingObjectVBR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioProcessingObjectVBR {}
impl ::core::fmt::Debug for IAudioProcessingObjectVBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioProcessingObjectVBR").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioProcessingObjectVBR {
    type Vtable = IAudioProcessingObjectVBR_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba1db8f_78ad_49cd_9591_f79d80a17c81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioProcessingObjectVBR_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CalcMaxInputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows::core::HRESULT,
    pub CalcMaxOutputFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffects(::windows::core::IUnknown);
impl IAudioSystemEffects {}
impl ::core::convert::From<IAudioSystemEffects> for ::windows::core::IUnknown {
    fn from(value: IAudioSystemEffects) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects> for ::windows::core::IUnknown {
    fn from(value: &IAudioSystemEffects) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSystemEffects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSystemEffects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffects {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects {}
impl ::core::fmt::Debug for IAudioSystemEffects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffects {
    type Vtable = IAudioSystemEffects_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fa00f27_add6_499a_8a9d_6b98521fa75b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffects2(::windows::core::IUnknown);
impl IAudioSystemEffects2 {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectsList<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEffectsList)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppeffectsids), ::core::mem::transmute(pceffects), event.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioSystemEffects2> for ::windows::core::IUnknown {
    fn from(value: IAudioSystemEffects2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects2> for ::windows::core::IUnknown {
    fn from(value: &IAudioSystemEffects2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAudioSystemEffects> for IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSystemEffects> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioSystemEffects> for &'a IAudioSystemEffects2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSystemEffects> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffects2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffects2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects2 {}
impl ::core::fmt::Debug for IAudioSystemEffects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffects2 {
    type Vtable = IAudioSystemEffects2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafe99d2_7436_44ce_9e0e_4d89afbfff56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects2_Vtbl {
    pub base: IAudioSystemEffects_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEffectsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEffectsList: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffects3(::windows::core::IUnknown);
impl IAudioSystemEffects3 {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectsList<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetEffectsList)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppeffectsids), ::core::mem::transmute(pceffects), event.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetControllableSystemEffectsList<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetControllableSystemEffectsList)(::core::mem::transmute_copy(self), ::core::mem::transmute(effects), ::core::mem::transmute(numeffects), event.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn SetAudioSystemEffectState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, effectid: Param0, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAudioSystemEffectState)(::core::mem::transmute_copy(self), effectid.into_param().abi(), ::core::mem::transmute(state)).ok()
    }
}
impl ::core::convert::From<IAudioSystemEffects3> for ::windows::core::IUnknown {
    fn from(value: IAudioSystemEffects3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffects3> for ::windows::core::IUnknown {
    fn from(value: &IAudioSystemEffects3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAudioSystemEffects> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSystemEffects> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioSystemEffects> for &'a IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSystemEffects> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAudioSystemEffects2> for IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSystemEffects2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioSystemEffects2> for &'a IAudioSystemEffects3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSystemEffects2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffects3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffects3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffects3 {}
impl ::core::fmt::Debug for IAudioSystemEffects3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffects3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffects3 {
    type Vtable = IAudioSystemEffects3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc58b31cd_fc6a_4255_bc1f_ad29bb0a4a17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffects3_Vtbl {
    pub base: IAudioSystemEffects2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetControllableSystemEffectsList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetControllableSystemEffectsList: usize,
    pub SetAudioSystemEffectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffectsCustomFormats(::windows::core::IUnknown);
impl IAudioSystemEffectsCustomFormats {
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetFormatCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetFormat(&self, nformat: u32) -> ::windows::core::Result<IAudioMediaType> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(nformat), ::core::mem::transmute(&mut result__)).from_abi::<IAudioMediaType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    pub unsafe fn GetFormatRepresentation(&self, nformat: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFormatRepresentation)(::core::mem::transmute_copy(self), ::core::mem::transmute(nformat), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IAudioSystemEffectsCustomFormats> for ::windows::core::IUnknown {
    fn from(value: IAudioSystemEffectsCustomFormats) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffectsCustomFormats> for ::windows::core::IUnknown {
    fn from(value: &IAudioSystemEffectsCustomFormats) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSystemEffectsCustomFormats {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSystemEffectsCustomFormats {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffectsCustomFormats {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffectsCustomFormats {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsCustomFormats {}
impl ::core::fmt::Debug for IAudioSystemEffectsCustomFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsCustomFormats").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffectsCustomFormats {
    type Vtable = IAudioSystemEffectsCustomFormats_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1176e34_bb7f_4f05_bebd_1b18a534e097);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsCustomFormats_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nformat: u32, ppformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFormatRepresentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_APO_SWFallback_ProcessingModes: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 18u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 17u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 16u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 20u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Association: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_FriendlyName: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PostMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PreMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_UserInterfaceClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 5u32 };
pub const SID_AudioProcessingObjectLoggingService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b8008af_09f9_456e_a173_bdb58499bce7);
pub const SID_AudioProcessingObjectRTQueue: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458c1a1f_6899_4c12_99ac_e2e6ac253104);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: ::windows::core::GUID,
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
impl ::core::fmt::Debug for UNCOMPRESSEDAUDIOFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNCOMPRESSEDAUDIOFORMAT").field("guidFormatType", &self.guidFormatType).field("dwSamplesPerFrame", &self.dwSamplesPerFrame).field("dwBytesPerSampleContainer", &self.dwBytesPerSampleContainer).field("dwValidBitsPerSample", &self.dwValidBitsPerSample).field("fFramesPerSecond", &self.fFramesPerSecond).field("dwChannelMask", &self.dwChannelMask).finish()
    }
}
unsafe impl ::windows::core::Abi for UNCOMPRESSEDAUDIOFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UNCOMPRESSEDAUDIOFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UNCOMPRESSEDAUDIOFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for UNCOMPRESSEDAUDIOFORMAT {}
impl ::core::default::Default for UNCOMPRESSEDAUDIOFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
