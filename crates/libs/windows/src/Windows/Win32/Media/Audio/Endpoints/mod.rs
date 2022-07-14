#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
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
unsafe impl ::windows::core::Abi for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIO_ENDPOINT_SHARED_CREATE_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {}
impl ::core::default::Default for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f2f7b66_65ac_4fa6_8ae4_123c78b89313);
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EndpointConnectorType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eHostProcessConnector: EndpointConnectorType = EndpointConnectorType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eOffloadConnector: EndpointConnectorType = EndpointConnectorType(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eLoopbackConnector: EndpointConnectorType = EndpointConnectorType(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eKeywordDetectorConnector: EndpointConnectorType = EndpointConnectorType(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
pub const eConnectorCount: EndpointConnectorType = EndpointConnectorType(4i32);
impl ::core::marker::Copy for EndpointConnectorType {}
impl ::core::clone::Clone for EndpointConnectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EndpointConnectorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EndpointConnectorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EndpointConnectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointConnectorType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointFormatControl(::windows::core::IUnknown);
impl IAudioEndpointFormatControl {
    pub unsafe fn ResetToDefault(&self, resetflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetToDefault)(::windows::core::Interface::as_raw(self), resetflags).ok()
    }
}
impl ::core::convert::From<IAudioEndpointFormatControl> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointFormatControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointFormatControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioEndpointFormatControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointFormatControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointFormatControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioEndpointFormatControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointFormatControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointFormatControl {}
impl ::core::fmt::Debug for IAudioEndpointFormatControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointFormatControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointFormatControl {
    type Vtable = IAudioEndpointFormatControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x784cfd40_9f89_456e_a1a6_873b006a664e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointFormatControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ResetToDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resetflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointLastBufferControl(::windows::core::IUnknown);
impl IAudioEndpointLastBufferControl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsLastBufferControlSupported(&self) -> super::super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsLastBufferControlSupported)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Media_Audio_Apo\"`*"]
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn ReleaseOutputDataPointerForLastBuffer(&self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
        (::windows::core::Interface::vtable(self).ReleaseOutputDataPointerForLastBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pconnectionproperty))
    }
}
impl ::core::convert::From<IAudioEndpointLastBufferControl> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointLastBufferControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointLastBufferControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioEndpointLastBufferControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointLastBufferControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointLastBufferControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioEndpointLastBufferControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointLastBufferControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointLastBufferControl {}
impl ::core::fmt::Debug for IAudioEndpointLastBufferControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointLastBufferControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointLastBufferControl {
    type Vtable = IAudioEndpointLastBufferControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8520dd3_8f9d_4437_9861_62f584c33dd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointLastBufferControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsLastBufferControlSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsLastBufferControlSupported: usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointerForLastBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointerForLastBuffer: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamMeter(::windows::core::IUnknown);
impl IAudioEndpointOffloadStreamMeter {
    pub unsafe fn GetMeterChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMeterChannelCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMeteringData(&self, u32channelcount: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMeteringData)(::windows::core::Interface::as_raw(self), u32channelcount, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
}
impl ::core::convert::From<IAudioEndpointOffloadStreamMeter> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamMeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointOffloadStreamMeter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioEndpointOffloadStreamMeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamMeter> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamMeter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamMeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointOffloadStreamMeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointOffloadStreamMeter {}
impl ::core::fmt::Debug for IAudioEndpointOffloadStreamMeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointOffloadStreamMeter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointOffloadStreamMeter {
    type Vtable = IAudioEndpointOffloadStreamMeter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1546dce_9dd1_418b_9ab2_348ced161c86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamMeter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetMeterChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetMeteringData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamMute(::windows::core::IUnknown);
impl IAudioEndpointOffloadStreamMute {
    pub unsafe fn SetMute(&self, bmuted: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMute)(::windows::core::Interface::as_raw(self), bmuted).ok()
    }
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMute)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
}
impl ::core::convert::From<IAudioEndpointOffloadStreamMute> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamMute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointOffloadStreamMute> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioEndpointOffloadStreamMute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamMute> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamMute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamMute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointOffloadStreamMute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointOffloadStreamMute {}
impl ::core::fmt::Debug for IAudioEndpointOffloadStreamMute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointOffloadStreamMute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointOffloadStreamMute {
    type Vtable = IAudioEndpointOffloadStreamMute_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfe21355_5ec2_40e0_8d6b_710ac3c00249);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamMute_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmuted: u8) -> ::windows::core::HRESULT,
    pub GetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmuted: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointOffloadStreamVolume(::windows::core::IUnknown);
impl IAudioEndpointOffloadStreamVolume {
    pub unsafe fn GetVolumeChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetVolumeChannelCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_KernelStreaming\"`*"]
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub unsafe fn SetChannelVolumes(&self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelVolumes)(::windows::core::Interface::as_raw(self), u32channelcount, ::core::mem::transmute(pf32volumes), u32curvetype, ::core::mem::transmute(pcurveduration)).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, u32channelcount: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelVolumes)(::windows::core::Interface::as_raw(self), u32channelcount, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
}
impl ::core::convert::From<IAudioEndpointOffloadStreamVolume> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointOffloadStreamVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointOffloadStreamVolume> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioEndpointOffloadStreamVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointOffloadStreamVolume> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointOffloadStreamVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioEndpointOffloadStreamVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointOffloadStreamVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointOffloadStreamVolume {}
impl ::core::fmt::Debug for IAudioEndpointOffloadStreamVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointOffloadStreamVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointOffloadStreamVolume {
    type Vtable = IAudioEndpointOffloadStreamVolume_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64f1dd49_71ca_4281_8672_3a9eddd1d0b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamVolume_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetVolumeChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub SetChannelVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_KernelStreaming"))]
    SetChannelVolumes: usize,
    pub GetChannelVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointVolume(::windows::core::IUnknown);
impl IAudioEndpointVolume {
    pub unsafe fn RegisterControlChangeNotify<'a, P0>(&self, pnotify: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAudioEndpointVolumeCallback>>,
    {
        (::windows::core::Interface::vtable(self).RegisterControlChangeNotify)(::windows::core::Interface::as_raw(self), pnotify.into().abi()).ok()
    }
    pub unsafe fn UnregisterControlChangeNotify<'a, P0>(&self, pnotify: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAudioEndpointVolumeCallback>>,
    {
        (::windows::core::Interface::vtable(self).UnregisterControlChangeNotify)(::windows::core::Interface::as_raw(self), pnotify.into().abi()).ok()
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMasterVolumeLevel)(::windows::core::Interface::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMasterVolumeLevelScalar)(::windows::core::Interface::as_raw(self), flevel, ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetMasterVolumeLevel(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMasterVolumeLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMasterVolumeLevelScalar)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelVolumeLevel)(::windows::core::Interface::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelVolumeLevelScalar)(::windows::core::Interface::as_raw(self), nchannel, flevel, ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelVolumeLevel)(::windows::core::Interface::as_raw(self), nchannel, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelVolumeLevelScalar)(::windows::core::Interface::as_raw(self), nchannel, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMute<'a, P0>(&self, bmute: P0, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMute)(::windows::core::Interface::as_raw(self), bmute.into(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMute)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVolumeStepInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pnstep), ::core::mem::transmute(pnstepcount)).ok()
    }
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VolumeStepUp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).VolumeStepDown)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryHardwareSupport)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVolumeRange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointVolume> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointVolume> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioEndpointVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolume> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioEndpointVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointVolume {}
impl ::core::fmt::Debug for IAudioEndpointVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointVolume {
    type Vtable = IAudioEndpointVolume_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cdf2c82_841e_4546_9722_0cf74078229a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolume_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub RegisterControlChangeNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnregisterControlChangeNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT,
    pub SetMasterVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetMasterVolumeLevelScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMasterVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub GetMasterVolumeLevelScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT,
    pub SetChannelVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetChannelVolumeLevelScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetChannelVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub GetChannelVolumeLevelScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMute: usize,
    pub GetVolumeStepInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::HRESULT,
    pub VolumeStepUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub VolumeStepDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT,
    pub GetVolumeRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointVolumeCallback(::windows::core::IUnknown);
impl IAudioEndpointVolumeCallback {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnNotify(&self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnNotify)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pnotify)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointVolumeCallback> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointVolumeCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointVolumeCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioEndpointVolumeCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeCallback> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointVolumeCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioEndpointVolumeCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointVolumeCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointVolumeCallback {}
impl ::core::fmt::Debug for IAudioEndpointVolumeCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointVolumeCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointVolumeCallback {
    type Vtable = IAudioEndpointVolumeCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x657804fa_d6ad_4496_8a60_352752af4f89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnNotify: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioEndpointVolumeEx(::windows::core::IUnknown);
impl IAudioEndpointVolumeEx {
    pub unsafe fn RegisterControlChangeNotify<'a, P0>(&self, pnotify: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAudioEndpointVolumeCallback>>,
    {
        (::windows::core::Interface::vtable(self).base__.RegisterControlChangeNotify)(::windows::core::Interface::as_raw(self), pnotify.into().abi()).ok()
    }
    pub unsafe fn UnregisterControlChangeNotify<'a, P0>(&self, pnotify: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IAudioEndpointVolumeCallback>>,
    {
        (::windows::core::Interface::vtable(self).base__.UnregisterControlChangeNotify)(::windows::core::Interface::as_raw(self), pnotify.into().abi()).ok()
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetChannelCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMasterVolumeLevel)(::windows::core::Interface::as_raw(self), fleveldb, ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMasterVolumeLevelScalar)(::windows::core::Interface::as_raw(self), flevel, ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetMasterVolumeLevel(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMasterVolumeLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMasterVolumeLevelScalar)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetChannelVolumeLevel)(::windows::core::Interface::as_raw(self), nchannel, fleveldb, ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetChannelVolumeLevelScalar)(::windows::core::Interface::as_raw(self), nchannel, flevel, ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetChannelVolumeLevel)(::windows::core::Interface::as_raw(self), nchannel, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetChannelVolumeLevelScalar)(::windows::core::Interface::as_raw(self), nchannel, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMute<'a, P0>(&self, bmute: P0, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetMute)(::windows::core::Interface::as_raw(self), bmute.into(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMute)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVolumeStepInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pnstep), ::core::mem::transmute(pnstepcount)).ok()
    }
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.VolumeStepUp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.VolumeStepDown)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueryHardwareSupport)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVolumeRange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
    pub unsafe fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVolumeRangeChannel)(::windows::core::Interface::as_raw(self), ichannel, ::core::mem::transmute(pflvolumemindb), ::core::mem::transmute(pflvolumemaxdb), ::core::mem::transmute(pflvolumeincrementdb)).ok()
    }
}
impl ::core::convert::From<IAudioEndpointVolumeEx> for ::windows::core::IUnknown {
    fn from(value: IAudioEndpointVolumeEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointVolumeEx> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioEndpointVolumeEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeEx> for ::windows::core::IUnknown {
    fn from(value: &IAudioEndpointVolumeEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IAudioEndpointVolumeEx> for IAudioEndpointVolume {
    fn from(value: IAudioEndpointVolumeEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioEndpointVolumeEx> for &'a IAudioEndpointVolume {
    fn from(value: &'a IAudioEndpointVolumeEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEndpointVolumeEx> for IAudioEndpointVolume {
    fn from(value: &IAudioEndpointVolumeEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioEndpointVolumeEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointVolumeEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointVolumeEx {}
impl ::core::fmt::Debug for IAudioEndpointVolumeEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointVolumeEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEndpointVolumeEx {
    type Vtable = IAudioEndpointVolumeEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66e11784_f695_4f28_a505_a7080081a78f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeEx_Vtbl {
    pub base__: IAudioEndpointVolume_Vtbl,
    pub GetVolumeRangeChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioLfxControl(::windows::core::IUnknown);
impl IAudioLfxControl {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLocalEffectsState<'a, P0>(&self, benabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetLocalEffectsState)(::windows::core::Interface::as_raw(self), benabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLocalEffectsState(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLocalEffectsState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IAudioLfxControl> for ::windows::core::IUnknown {
    fn from(value: IAudioLfxControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioLfxControl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioLfxControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioLfxControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioLfxControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioLfxControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioLfxControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioLfxControl {}
impl ::core::fmt::Debug for IAudioLfxControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioLfxControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioLfxControl {
    type Vtable = IAudioLfxControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076a6922_d802_4f83_baf6_409d9ca11bfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLfxControl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLocalEffectsState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLocalEffectsState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLocalEffectsState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLocalEffectsState: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IAudioMeterInformation(::windows::core::IUnknown);
impl IAudioMeterInformation {
    pub unsafe fn GetPeakValue(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPeakValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetMeteringChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMeteringChannelCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChannelsPeakValues(&self, afpeakvalues: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChannelsPeakValues)(::windows::core::Interface::as_raw(self), afpeakvalues.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(afpeakvalues))).ok()
    }
    pub unsafe fn QueryHardwareSupport(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryHardwareSupport)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioMeterInformation> for ::windows::core::IUnknown {
    fn from(value: IAudioMeterInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAudioMeterInformation> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAudioMeterInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioMeterInformation> for ::windows::core::IUnknown {
    fn from(value: &IAudioMeterInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAudioMeterInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioMeterInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMeterInformation {}
impl ::core::fmt::Debug for IAudioMeterInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMeterInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioMeterInformation {
    type Vtable = IAudioMeterInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc02216f6_8c67_4b5b_9d00_d008e73e0064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMeterInformation_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetPeakValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfpeak: *mut f32) -> ::windows::core::HRESULT,
    pub GetMeteringChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetChannelsPeakValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio_Endpoints\"`*"]
#[repr(transparent)]
pub struct IHardwareAudioEngineBase(::windows::core::IUnknown);
impl IHardwareAudioEngineBase {
    pub unsafe fn GetAvailableOffloadConnectorCount<'a, P0>(&self, _pwstrdeviceid: P0, _uconnectorid: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAvailableOffloadConnectorCount)(::windows::core::Interface::as_raw(self), _pwstrdeviceid.into(), _uconnectorid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEngineFormat<'a, P0, P1>(&self, pdevice: P0, _brequestdeviceformat: P1, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IMMDevice>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetEngineFormat)(::windows::core::Interface::as_raw(self), pdevice.into().abi(), _brequestdeviceformat.into(), ::core::mem::transmute(_ppwfxformat)).ok()
    }
    pub unsafe fn SetEngineDeviceFormat<'a, P0>(&self, pdevice: P0, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IMMDevice>>,
    {
        (::windows::core::Interface::vtable(self).SetEngineDeviceFormat)(::windows::core::Interface::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(_pwfxformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGfxState<'a, P0, P1>(&self, pdevice: P0, _benable: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IMMDevice>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetGfxState)(::windows::core::Interface::as_raw(self), pdevice.into().abi(), _benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGfxState<'a, P0>(&self, pdevice: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IMMDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetGfxState)(::windows::core::Interface::as_raw(self), pdevice.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IHardwareAudioEngineBase> for ::windows::core::IUnknown {
    fn from(value: IHardwareAudioEngineBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IHardwareAudioEngineBase> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IHardwareAudioEngineBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHardwareAudioEngineBase> for ::windows::core::IUnknown {
    fn from(value: &IHardwareAudioEngineBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IHardwareAudioEngineBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHardwareAudioEngineBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHardwareAudioEngineBase {}
impl ::core::fmt::Debug for IHardwareAudioEngineBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHardwareAudioEngineBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHardwareAudioEngineBase {
    type Vtable = IHardwareAudioEngineBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeddce3e4_f3c1_453a_b461_223563cbd886);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareAudioEngineBase_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetAvailableOffloadConnectorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _pwstrdeviceid: ::windows::core::PCWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEngineFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEngineFormat: usize,
    pub SetEngineDeviceFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGfxState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _benable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGfxState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGfxState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _pbenable: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGfxState: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
