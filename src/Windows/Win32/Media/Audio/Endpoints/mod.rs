#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
pub struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    pub u32Size: u32,
    pub u32TSSessionId: u32,
    pub targetEndpointConnectorType: EndpointConnectorType,
    pub wfxDeviceFormat: super::WAVEFORMATEX,
}
impl AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {}
impl ::core::default::Default for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {}
unsafe impl ::windows::runtime::Abi for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    type Abi = Self;
}
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2670689126, 26028, 20390, [138, 228, 18, 60, 120, 184, 147, 19]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(316160983, 53010, 18110, [133, 64, 129, 39, 16, 211, 2, 28]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(316160983, 53010, 18110, [133, 64, 129, 39, 16, 211, 2, 28]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(316160983, 53010, 18110, [133, 64, 129, 39, 16, 211, 2, 28]),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(316160983, 53010, 18110, [133, 64, 129, 39, 16, 211, 2, 28]),
    pid: 3u32,
};
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EndpointConnectorType(pub i32);
pub const eHostProcessConnector: EndpointConnectorType = EndpointConnectorType(0i32);
pub const eOffloadConnector: EndpointConnectorType = EndpointConnectorType(1i32);
pub const eLoopbackConnector: EndpointConnectorType = EndpointConnectorType(2i32);
pub const eKeywordDetectorConnector: EndpointConnectorType = EndpointConnectorType(3i32);
pub const eConnectorCount: EndpointConnectorType = EndpointConnectorType(4i32);
impl ::core::convert::From<i32> for EndpointConnectorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EndpointConnectorType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointFormatControl(pub ::windows::runtime::IUnknown);
impl IAudioEndpointFormatControl {
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn ResetToDefault(&self, resetflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(resetflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointFormatControl {
    type Vtable = IAudioEndpointFormatControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2018311488, 40841, 17774, [161, 166, 135, 59, 0, 106, 102, 78]);
}
impl ::core::convert::From<IAudioEndpointFormatControl> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointFormatControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointFormatControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointFormatControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointFormatControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointFormatControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointFormatControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resetflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointLastBufferControl(pub ::windows::runtime::IUnknown);
impl IAudioEndpointLastBufferControl {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn IsLastBufferControlSupported(&self) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Media_Audio_Apo`*"]
    pub unsafe fn ReleaseOutputDataPointerForLastBuffer(&self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pconnectionproperty)))
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointLastBufferControl {
    type Vtable = IAudioEndpointLastBufferControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4166127059, 36765, 17463, [152, 97, 98, 245, 132, 195, 61, 214]);
}
impl ::core::convert::From<IAudioEndpointLastBufferControl> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointLastBufferControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointLastBufferControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointLastBufferControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointLastBufferControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointLastBufferControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointLastBufferControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointOffloadStreamMeter(pub ::windows::runtime::IUnknown);
impl IAudioEndpointOffloadStreamMeter {
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetMeterChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetMeteringData(&self, u32channelcount: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32channelcount), &mut result__).from_abi::<f32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointOffloadStreamMeter {
    type Vtable = IAudioEndpointOffloadStreamMeter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3780406734, 40401, 16779, [154, 178, 52, 140, 237, 22, 28, 134]);
}
impl ::core::convert::From<IAudioEndpointOffloadStreamMeter> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamMeter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamMeter> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamMeter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointOffloadStreamMeter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointOffloadStreamMeter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamMeter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pu32channelcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointOffloadStreamMute(pub ::windows::runtime::IUnknown);
impl IAudioEndpointOffloadStreamMute {
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetMute(&self, bmuted: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(bmuted)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetMute(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u8>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointOffloadStreamMute {
    type Vtable = IAudioEndpointOffloadStreamMute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3756135253, 24258, 16608, [141, 107, 113, 10, 195, 192, 2, 73]);
}
impl ::core::convert::From<IAudioEndpointOffloadStreamMute> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamMute) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamMute> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamMute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointOffloadStreamMute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointOffloadStreamMute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamMute_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmuted: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmuted: *mut u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointOffloadStreamVolume(pub ::windows::runtime::IUnknown);
impl IAudioEndpointOffloadStreamVolume {
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetVolumeChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Media_KernelStreaming`*"]
    pub unsafe fn SetChannelVolumes(&self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32channelcount), ::core::mem::transmute(pf32volumes), ::core::mem::transmute(u32curvetype), ::core::mem::transmute(pcurveduration)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetChannelVolumes(&self, u32channelcount: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32channelcount), &mut result__).from_abi::<f32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointOffloadStreamVolume {
    type Vtable = IAudioEndpointOffloadStreamVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1693572425, 29130, 17025, [134, 114, 58, 158, 221, 209, 208, 182]);
}
impl ::core::convert::From<IAudioEndpointOffloadStreamVolume> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamVolume> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointOffloadStreamVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointOffloadStreamVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pu32channelcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_KernelStreaming")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_KernelStreaming"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointVolume(pub ::windows::runtime::IUnknown);
impl IAudioEndpointVolume {
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn RegisterControlChangeNotify<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioEndpointVolumeCallback>>(&self, pnotify: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pnotify.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn UnregisterControlChangeNotify<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioEndpointVolumeCallback>>(&self, pnotify: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pnotify.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(flevel), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetMasterVolumeLevel(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(flevel), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn SetMute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, bmute: Param0, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bmute.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn GetMute(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnstep), ::core::mem::transmute(pnstepcount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointVolume {
    type Vtable = IAudioEndpointVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1558129794, 33822, 17734, [151, 34, 12, 247, 64, 120, 34, 154]);
}
impl ::core::convert::From<IAudioEndpointVolume> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointVolume> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnotify: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnotify: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnchannelcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flevel: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflevel: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pflevel: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhardwaresupportmask: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointVolumeCallback(pub ::windows::runtime::IUnknown);
impl IAudioEndpointVolumeCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn OnNotify(&self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnotify)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointVolumeCallback {
    type Vtable = IAudioEndpointVolumeCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1702364410, 54957, 17558, [138, 96, 53, 39, 82, 175, 79, 137]);
}
impl ::core::convert::From<IAudioEndpointVolumeCallback> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointVolumeCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointVolumeCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointVolumeCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointVolumeCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEndpointVolumeEx(pub ::windows::runtime::IUnknown);
impl IAudioEndpointVolumeEx {
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn RegisterControlChangeNotify<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioEndpointVolumeCallback>>(&self, pnotify: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pnotify.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn UnregisterControlChangeNotify<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioEndpointVolumeCallback>>(&self, pnotify: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pnotify.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(flevel), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetMasterVolumeLevel(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(flevel), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn SetMute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, bmute: Param0, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), bmute.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn GetMute(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnstep), ::core::mem::transmute(pnstepcount)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(ichannel), ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEndpointVolumeEx {
    type Vtable = IAudioEndpointVolumeEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1726027652, 63125, 20264, [165, 5, 167, 8, 0, 129, 167, 143]);
}
impl ::core::convert::From<IAudioEndpointVolumeEx> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEndpointVolumeEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeEx> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEndpointVolumeEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEndpointVolumeEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEndpointVolumeEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioEndpointVolumeEx> for IAudioEndpointVolume {
    fn from(value: IAudioEndpointVolumeEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeEx> for IAudioEndpointVolume {
    fn from(value: &IAudioEndpointVolumeEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioEndpointVolume> for IAudioEndpointVolumeEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioEndpointVolume> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioEndpointVolume> for &IAudioEndpointVolumeEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioEndpointVolume> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnotify: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnotify: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnchannelcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flevel: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflevel: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pflevel: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhardwaresupportmask: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioLfxControl(pub ::windows::runtime::IUnknown);
impl IAudioLfxControl {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn SetLocalEffectsState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, benabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), benabled.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn GetLocalEffectsState(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioLfxControl {
    type Vtable = IAudioLfxControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(124414242, 55298, 20355, [186, 246, 64, 157, 156, 161, 27, 254]);
}
impl ::core::convert::From<IAudioLfxControl> for ::windows::runtime::IUnknown {
    fn from(value: IAudioLfxControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioLfxControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioLfxControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioLfxControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioLfxControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLfxControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabled: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioMeterInformation(pub ::windows::runtime::IUnknown);
impl IAudioMeterInformation {
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetPeakValue(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetMeteringChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn GetChannelsPeakValues(&self, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(u32channelcount), ::core::mem::transmute(afpeakvalues)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioMeterInformation {
    type Vtable = IAudioMeterInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3223459574, 35943, 19291, [157, 0, 208, 8, 231, 62, 0, 100]);
}
impl ::core::convert::From<IAudioMeterInformation> for ::windows::runtime::IUnknown {
    fn from(value: IAudioMeterInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioMeterInformation> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioMeterInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioMeterInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioMeterInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMeterInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfpeak: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnchannelcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhardwaresupportmask: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IHardwareAudioEngineBase(pub ::windows::runtime::IUnknown);
impl IHardwareAudioEngineBase {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn GetAvailableOffloadConnectorCount<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, _pwstrdeviceid: Param0, _uconnectorid: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), _pwstrdeviceid.into_param().abi(), ::core::mem::transmute(_uconnectorid), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn GetEngineFormat<'a, Param0: ::windows::runtime::IntoParam<'a, super::IMMDevice>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, pdevice: Param0, _brequestdeviceformat: Param1, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), _brequestdeviceformat.into_param().abi(), ::core::mem::transmute(_ppwfxformat)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`*"]
    pub unsafe fn SetEngineDeviceFormat<'a, Param0: ::windows::runtime::IntoParam<'a, super::IMMDevice>>(&self, pdevice: Param0, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), ::core::mem::transmute(_pwfxformat)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn SetGfxState<'a, Param0: ::windows::runtime::IntoParam<'a, super::IMMDevice>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, pdevice: Param0, _benable: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), _benable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio_Endpoints`, `Win32_Foundation`*"]
    pub unsafe fn GetGfxState<'a, Param0: ::windows::runtime::IntoParam<'a, super::IMMDevice>>(&self, pdevice: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::BOOL> {
        let mut result__: <super::super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pdevice.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IHardwareAudioEngineBase {
    type Vtable = IHardwareAudioEngineBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3990676452, 62401, 17722, [180, 97, 34, 53, 99, 203, 216, 134]);
}
impl ::core::convert::From<IHardwareAudioEngineBase> for ::windows::runtime::IUnknown {
    fn from(value: IHardwareAudioEngineBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IHardwareAudioEngineBase> for ::windows::runtime::IUnknown {
    fn from(value: &IHardwareAudioEngineBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHardwareAudioEngineBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IHardwareAudioEngineBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareAudioEngineBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _pwstrdeviceid: super::super::super::Foundation::PWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, _benable: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, _pbenable: *mut super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
