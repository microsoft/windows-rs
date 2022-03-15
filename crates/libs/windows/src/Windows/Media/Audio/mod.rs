#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioDeviceInputNode(::windows::core::IUnknown);
impl AudioDeviceInputNode {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Device)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Emitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioDeviceInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioDeviceInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceInputNode {}
impl ::core::fmt::Debug for AudioDeviceInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioDeviceInputNode;{b01b6be1-6f4e-49e2-ac01-559d62beb3a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_Vtbl;
    const IID: ::windows::core::GUID = <IAudioDeviceInputNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioDeviceInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceInputNode";
}
impl ::core::convert::From<AudioDeviceInputNode> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceInputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioDeviceInputNode> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceInputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioDeviceInputNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceInputNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioDeviceInputNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceInputNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioDeviceInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioDeviceInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioDeviceInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioDeviceInputNode {}
unsafe impl ::core::marker::Sync for AudioDeviceInputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioDeviceNodeCreationStatus {}
impl ::core::clone::Clone for AudioDeviceNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDeviceNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioDeviceNodeCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioDeviceNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceNodeCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceNodeCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioDeviceNodeCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioDeviceOutputNode(::windows::core::IUnknown);
impl AudioDeviceOutputNode {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Device)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetListener<'a, Param0: ::windows::core::IntoParam<'a, AudioNodeListener>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetListener)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Listener(&self) -> ::windows::core::Result<AudioNodeListener> {
        let this = &::windows::core::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Listener)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeListener>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioDeviceOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioDeviceOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceOutputNode {}
impl ::core::fmt::Debug for AudioDeviceOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceOutputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceOutputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioDeviceOutputNode;{362edbff-ff1c-4434-9e0f-bd2ef522ac82})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_Vtbl;
    const IID: ::windows::core::GUID = <IAudioDeviceOutputNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioDeviceOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceOutputNode";
}
impl ::core::convert::From<AudioDeviceOutputNode> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceOutputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioDeviceOutputNode> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioDeviceOutputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioDeviceOutputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceOutputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioDeviceOutputNode> for IAudioNodeWithListener {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioDeviceOutputNode> for IAudioNodeWithListener {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNodeWithListener> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNodeWithListener> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNodeWithListener> for &AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNodeWithListener> {
        ::core::convert::TryInto::<IAudioNodeWithListener>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioDeviceOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioDeviceOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioDeviceOutputNode {}
unsafe impl ::core::marker::Sync for AudioDeviceOutputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFileInputNode(::windows::core::IUnknown);
impl AudioFileInputNode {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlaybackSpeedFactor)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackSpeedFactor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Seek<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, position: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Seek)(::core::mem::transmute_copy(this), position.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoopCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLoopCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLoopCount)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SourceFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceFile)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioFileInputNode, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFileCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Emitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioFileInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFileInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFileInputNode {}
impl ::core::fmt::Debug for AudioFileInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFileInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFileInputNode;{905b67c8-6f65-4cd4-8890-4694843c276d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioFileInputNode {
    type Vtable = IAudioFileInputNode_Vtbl;
    const IID: ::windows::core::GUID = <IAudioFileInputNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioFileInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileInputNode";
}
impl ::core::convert::From<AudioFileInputNode> for ::windows::core::IUnknown {
    fn from(value: AudioFileInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFileInputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioFileInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFileInputNode> for ::windows::core::IInspectable {
    fn from(value: AudioFileInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFileInputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioFileInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFileInputNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileInputNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for &AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFileInputNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileInputNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for &AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFileInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioFileInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioFileInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFileInputNode {}
unsafe impl ::core::marker::Sync for AudioFileInputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FileNotFound: Self = Self(1i32);
    pub const InvalidFileType: Self = Self(2i32);
    pub const FormatNotSupported: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioFileNodeCreationStatus {}
impl ::core::clone::Clone for AudioFileNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioFileNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioFileNodeCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioFileNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileNodeCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFileNodeCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioFileNodeCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFileOutputNode(::windows::core::IUnknown);
impl AudioFileOutputNode {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FileEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileEncodingProfile)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaEncodingProfile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Media_Transcoding\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))]
    pub fn FinalizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FinalizeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioFileOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFileOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFileOutputNode {}
impl ::core::fmt::Debug for AudioFileOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileOutputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFileOutputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFileOutputNode;{50e01980-5166-4093-80f8-ada00089e9cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_Vtbl;
    const IID: ::windows::core::GUID = <IAudioFileOutputNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioFileOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileOutputNode";
}
impl ::core::convert::From<AudioFileOutputNode> for ::windows::core::IUnknown {
    fn from(value: AudioFileOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFileOutputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioFileOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFileOutputNode> for ::windows::core::IInspectable {
    fn from(value: AudioFileOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFileOutputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioFileOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFileOutputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFileOutputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFileOutputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFileOutputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioFileOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFileOutputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioFileOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFileOutputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFileOutputNode {}
unsafe impl ::core::marker::Sync for AudioFileOutputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFrameCompletedEventArgs(::windows::core::IUnknown);
impl AudioFrameCompletedEventArgs {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Frame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Frame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioFrameCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFrameCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameCompletedEventArgs {}
impl ::core::fmt::Debug for AudioFrameCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFrameCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameCompletedEventArgs;{dc7c829e-0208-4504-a5a8-f0f268920a65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAudioFrameCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioFrameCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameCompletedEventArgs";
}
impl ::core::convert::From<AudioFrameCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AudioFrameCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AudioFrameCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFrameCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AudioFrameCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AudioFrameCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioFrameCompletedEventArgs {}
unsafe impl ::core::marker::Sync for AudioFrameCompletedEventArgs {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFrameInputNode(::windows::core::IUnknown);
impl AudioFrameInputNode {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlaybackSpeedFactor)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackSpeedFactor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddFrame<'a, Param0: ::windows::core::IntoParam<'a, super::AudioFrame>>(&self, frame: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddFrame)(::core::mem::transmute_copy(this), frame.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DiscardQueuedFrames)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn QueuedSampleCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QueuedSampleCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioFrameCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioFrameCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioFrameCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAudioFrameCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QuantumStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QuantumStarted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveQuantumStarted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Emitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioFrameInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFrameInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameInputNode {}
impl ::core::fmt::Debug for AudioFrameInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFrameInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameInputNode;{01b266c7-fd96-4ff5-a3c5-d27a9bf44237})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_Vtbl;
    const IID: ::windows::core::GUID = <IAudioFrameInputNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioFrameInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameInputNode";
}
impl ::core::convert::From<AudioFrameInputNode> for ::windows::core::IUnknown {
    fn from(value: AudioFrameInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameInputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioFrameInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFrameInputNode> for ::windows::core::IInspectable {
    fn from(value: AudioFrameInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameInputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioFrameInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFrameInputNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameInputNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrameInputNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameInputNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrameInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioFrameInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioFrameInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFrameInputNode {}
unsafe impl ::core::marker::Sync for AudioFrameInputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFrameOutputNode(::windows::core::IUnknown);
impl AudioFrameOutputNode {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn GetFrame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioFrameOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioFrameOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameOutputNode {}
impl ::core::fmt::Debug for AudioFrameOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameOutputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFrameOutputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameOutputNode;{b847371b-3299-45f5-88b3-c9d12a3f1cc8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_Vtbl;
    const IID: ::windows::core::GUID = <IAudioFrameOutputNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioFrameOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameOutputNode";
}
impl ::core::convert::From<AudioFrameOutputNode> for ::windows::core::IUnknown {
    fn from(value: AudioFrameOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameOutputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioFrameOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFrameOutputNode> for ::windows::core::IInspectable {
    fn from(value: AudioFrameOutputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrameOutputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioFrameOutputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioFrameOutputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFrameOutputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrameOutputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFrameOutputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioFrameOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFrameOutputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioFrameOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFrameOutputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFrameOutputNode {}
unsafe impl ::core::marker::Sync for AudioFrameOutputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraph(::windows::core::IUnknown);
impl AudioGraph {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateFrameInputNode(&self) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrameInputNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrameInputNodeWithFormat)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Media_Capture\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))]
    pub fn CreateDeviceInputNodeAsync(&self, category: super::Capture::MediaCategory) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDeviceInputNodeAsync)(::core::mem::transmute_copy(this), category, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, category: super::Capture::MediaCategory, encodingproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDeviceInputNodeWithFormatAsync)(::core::mem::transmute_copy(this), category, encodingproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatOnDeviceAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param2: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(&self, category: super::Capture::MediaCategory, encodingproperties: Param1, device: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDeviceInputNodeWithFormatOnDeviceAsync)(::core::mem::transmute_copy(this), category, encodingproperties.into_param().abi(), device.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateFrameOutputNode(&self) -> ::windows::core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrameOutputNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFrameOutputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameOutputNodeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrameOutputNodeWithFormat)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<AudioFrameOutputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateDeviceOutputNodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDeviceOutputNodeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileInputNodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileInputNodeAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileOutputNodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileOutputNodeAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CreateFileOutputNodeWithFileProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, file: Param0, fileencodingprofile: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileOutputNodeWithFileProfileAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), fileencodingprofile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateSubmixNode(&self) -> ::windows::core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSubmixNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioSubmixNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSubmixNodeWithFormat)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<AudioSubmixNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ResetAllNodes(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResetAllNodes)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QuantumStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QuantumStarted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveQuantumStarted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QuantumProcessed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QuantumProcessed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumProcessed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveQuantumProcessed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnrecoverableErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnrecoverableErrorOccurred)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnrecoverableErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnrecoverableErrorOccurred)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CompletedQuantumCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompletedQuantumCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn LatencyInSamples(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LatencyInSamples)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryRenderDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__: super::AudioProcessing = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RenderDeviceAudioProcessing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SamplesPerQuantum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SamplesPerQuantum)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormatAndEmitter<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, encodingproperties: Param0, emitter: Param1) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrameInputNodeWithFormatAndEmitter)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param2: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>, Param3: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, category: super::Capture::MediaCategory, encodingproperties: Param1, device: Param2, emitter: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync)(::core::mem::transmute_copy(this), category, encodingproperties.into_param().abi(), device.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileInputNodeWithEmitterAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, file: Param0, emitter: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileInputNodeWithEmitterAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormatAndEmitter<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, encodingproperties: Param0, emitter: Param1) -> ::windows::core::Result<AudioSubmixNode> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSubmixNodeWithFormatAndEmitter)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<AudioSubmixNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateBatchUpdater(&self) -> ::windows::core::Result<AudioGraphBatchUpdater> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateBatchUpdater)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioGraphBatchUpdater>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateMediaSourceAudioInputNodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Core::MediaSource>>(&self, mediasource: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>> {
        let this = &::windows::core::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMediaSourceAudioInputNodeAsync)(::core::mem::transmute_copy(this), mediasource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateMediaSourceAudioInputNodeWithEmitterAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Core::MediaSource>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, mediasource: Param0, emitter: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>> {
        let this = &::windows::core::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMediaSourceAudioInputNodeWithEmitterAsync)(::core::mem::transmute_copy(this), mediasource.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync<'a, Param0: ::windows::core::IntoParam<'a, AudioGraphSettings>>(settings: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>> {
        Self::IAudioGraphStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsync)(::core::mem::transmute_copy(this), settings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAudioGraphStatics<R, F: FnOnce(&IAudioGraphStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioGraph, IAudioGraphStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioGraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraph {}
impl ::core::fmt::Debug for AudioGraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraph").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraph {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraph;{1ad46eed-e48c-4e14-9660-2c4f83e9cdd8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioGraph {
    type Vtable = IAudioGraph_Vtbl;
    const IID: ::windows::core::GUID = <IAudioGraph as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioGraph {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraph";
}
impl ::core::convert::From<AudioGraph> for ::windows::core::IUnknown {
    fn from(value: AudioGraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraph> for ::windows::core::IUnknown {
    fn from(value: &AudioGraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraph> for ::windows::core::IInspectable {
    fn from(value: AudioGraph) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraph> for ::windows::core::IInspectable {
    fn from(value: &AudioGraph) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioGraph> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioGraph) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioGraph> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioGraph) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioGraph {}
unsafe impl ::core::marker::Sync for AudioGraph {}
#[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct AudioGraphBatchUpdater(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl AudioGraphBatchUpdater {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for AudioGraphBatchUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for AudioGraphBatchUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for AudioGraphBatchUpdater {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for AudioGraphBatchUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphBatchUpdater").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for AudioGraphBatchUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphBatchUpdater;{30d5a829-7fa4-4026-83bb-d75bae4ea99e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for AudioGraphBatchUpdater {
    type Vtable = super::super::Foundation::IClosable_Vtbl;
    const IID: ::windows::core::GUID = <super::super::Foundation::IClosable as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for AudioGraphBatchUpdater {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphBatchUpdater";
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<AudioGraphBatchUpdater> for ::windows::core::IUnknown {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&AudioGraphBatchUpdater> for ::windows::core::IUnknown {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<AudioGraphBatchUpdater> for ::windows::core::IInspectable {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&AudioGraphBatchUpdater> for ::windows::core::IInspectable {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioGraphBatchUpdater> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioGraphBatchUpdater) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioGraphBatchUpdater> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioGraphBatchUpdater) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for AudioGraphBatchUpdater {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for AudioGraphBatchUpdater {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraphConnection(::windows::core::IUnknown);
impl AudioGraphConnection {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Destination(&self) -> ::windows::core::Result<IAudioNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Destination)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IAudioNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioGraphConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraphConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphConnection {}
impl ::core::fmt::Debug for AudioGraphConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraphConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphConnection;{763070ed-d04e-4fac-b233-600b42edd469})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioGraphConnection {
    type Vtable = IAudioGraphConnection_Vtbl;
    const IID: ::windows::core::GUID = <IAudioGraphConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioGraphConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphConnection";
}
impl ::core::convert::From<AudioGraphConnection> for ::windows::core::IUnknown {
    fn from(value: AudioGraphConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphConnection> for ::windows::core::IUnknown {
    fn from(value: &AudioGraphConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraphConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraphConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraphConnection> for ::windows::core::IInspectable {
    fn from(value: AudioGraphConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphConnection> for ::windows::core::IInspectable {
    fn from(value: &AudioGraphConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraphConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraphConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioGraphConnection {}
unsafe impl ::core::marker::Sync for AudioGraphConnection {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphCreationStatus {}
impl ::core::clone::Clone for AudioGraphCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioGraphCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioGraphCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioGraphCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraphCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraphSettings(::windows::core::IUnknown);
impl AudioGraphSettings {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncodingProperties)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryRenderDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPrimaryRenderDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrimaryRenderDevice)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn QuantumSizeSelectionMode(&self) -> ::windows::core::Result<QuantumSizeSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: QuantumSizeSelectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QuantumSizeSelectionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<QuantumSizeSelectionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetQuantumSizeSelectionMode(&self, value: QuantumSizeSelectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQuantumSizeSelectionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DesiredSamplesPerQuantum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredSamplesPerQuantum)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetDesiredSamplesPerQuantum(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredSamplesPerQuantum)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn AudioRenderCategory(&self) -> ::windows::core::Result<super::Render::AudioRenderCategory> {
        let this = self;
        unsafe {
            let mut result__: super::Render::AudioRenderCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioRenderCategory)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Render::AudioRenderCategory>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn SetAudioRenderCategory(&self, value: super::Render::AudioRenderCategory) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioRenderCategory)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DesiredRenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__: super::AudioProcessing = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredRenderDeviceAudioProcessing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetDesiredRenderDeviceAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredRenderDeviceAudioProcessing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxPlaybackSpeedFactor)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn MaxPlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxPlaybackSpeedFactor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn Create(audiorendercategory: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioGraphSettings> {
        Self::IAudioGraphSettingsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), audiorendercategory, &mut result__).from_abi::<AudioGraphSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioGraphSettingsFactory<R, F: FnOnce(&IAudioGraphSettingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioGraphSettings, IAudioGraphSettingsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioGraphSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraphSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphSettings {}
impl ::core::fmt::Debug for AudioGraphSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraphSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphSettings;{1d59647f-e6fe-4628-84f8-9d8bdba25785})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioGraphSettings {
    type Vtable = IAudioGraphSettings_Vtbl;
    const IID: ::windows::core::GUID = <IAudioGraphSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioGraphSettings {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphSettings";
}
impl ::core::convert::From<AudioGraphSettings> for ::windows::core::IUnknown {
    fn from(value: AudioGraphSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphSettings> for ::windows::core::IUnknown {
    fn from(value: &AudioGraphSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraphSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraphSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraphSettings> for ::windows::core::IInspectable {
    fn from(value: AudioGraphSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphSettings> for ::windows::core::IInspectable {
    fn from(value: &AudioGraphSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraphSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraphSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioGraphSettings {}
unsafe impl ::core::marker::Sync for AudioGraphSettings {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: Self = Self(0i32);
    pub const AudioDeviceLost: Self = Self(1i32);
    pub const AudioSessionDisconnected: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphUnrecoverableError {}
impl ::core::clone::Clone for AudioGraphUnrecoverableError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioGraphUnrecoverableError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioGraphUnrecoverableError {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioGraphUnrecoverableError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphUnrecoverableError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraphUnrecoverableError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphUnrecoverableError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(::windows::core::IUnknown);
impl AudioGraphUnrecoverableErrorOccurredEventArgs {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Error(&self) -> ::windows::core::Result<AudioGraphUnrecoverableError> {
        let this = self;
        unsafe {
            let mut result__: AudioGraphUnrecoverableError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Error)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioGraphUnrecoverableError>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphUnrecoverableErrorOccurredEventArgs {}
impl ::core::fmt::Debug for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphUnrecoverableErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs;{c3d9cbe0-3ff6-4fb3-b262-50d435c55423})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAudioGraphUnrecoverableErrorOccurredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs";
}
impl ::core::convert::From<AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioGraphUnrecoverableErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for AudioGraphUnrecoverableErrorOccurredEventArgs {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitter(::windows::core::IUnknown);
impl AudioNodeEmitter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeEmitter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPosition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direction)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDirection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDirection)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Shape(&self) -> ::windows::core::Result<AudioNodeEmitterShape> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterShape>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DecayModel(&self) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DecayModel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterDecayModel>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DistanceScale(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DistanceScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetDistanceScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDistanceScale)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DopplerScale(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DopplerScale)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetDopplerScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDopplerScale)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DopplerVelocity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDopplerVelocity)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn IsDopplerDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDopplerDisabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SpatialAudioModel(&self) -> ::windows::core::Result<SpatialAudioModel> {
        let this = &::windows::core::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe {
            let mut result__: SpatialAudioModel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SpatialAudioModel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialAudioModel>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetSpatialAudioModel(&self, value: SpatialAudioModel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSpatialAudioModel)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateAudioNodeEmitter<'a, Param0: ::windows::core::IntoParam<'a, AudioNodeEmitterShape>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitterDecayModel>>(shape: Param0, decaymodel: Param1, settings: AudioNodeEmitterSettings) -> ::windows::core::Result<AudioNodeEmitter> {
        Self::IAudioNodeEmitterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAudioNodeEmitter)(::core::mem::transmute_copy(this), shape.into_param().abi(), decaymodel.into_param().abi(), settings, &mut result__).from_abi::<AudioNodeEmitter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterFactory<R, F: FnOnce(&IAudioNodeEmitterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeEmitter, IAudioNodeEmitterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioNodeEmitter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitter {}
impl ::core::fmt::Debug for AudioNodeEmitter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitter;{3676971d-880a-47b8-adf7-1323a9d965be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_Vtbl;
    const IID: ::windows::core::GUID = <IAudioNodeEmitter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitter {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitter";
}
impl ::core::convert::From<AudioNodeEmitter> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitter> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitter> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitter> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitter {}
unsafe impl ::core::marker::Sync for AudioNodeEmitter {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterConeProperties(::windows::core::IUnknown);
impl AudioNodeEmitterConeProperties {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn InnerAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InnerAngle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OuterAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OuterAngle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OuterAngleGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OuterAngleGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioNodeEmitterConeProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterConeProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterConeProperties {}
impl ::core::fmt::Debug for AudioNodeEmitterConeProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterConeProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterConeProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterConeProperties;{e99b2cee-02ca-4375-9326-0c6ae4bcdfb5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_Vtbl;
    const IID: ::windows::core::GUID = <IAudioNodeEmitterConeProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitterConeProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterConeProperties";
}
impl ::core::convert::From<AudioNodeEmitterConeProperties> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitterConeProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterConeProperties> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitterConeProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitterConeProperties> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitterConeProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterConeProperties> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitterConeProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterConeProperties {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterConeProperties {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterDecayKind {}
impl ::core::clone::Clone for AudioNodeEmitterDecayKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterDecayKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioNodeEmitterDecayKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioNodeEmitterDecayKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterDecayKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterDecayKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterDecayKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterDecayModel(::windows::core::IUnknown);
impl AudioNodeEmitterDecayModel {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterDecayKind> {
        let this = self;
        unsafe {
            let mut result__: AudioNodeEmitterDecayKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterDecayKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn MinGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn MaxGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn NaturalProperties(&self) -> ::windows::core::Result<AudioNodeEmitterNaturalDecayModelProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NaturalProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterNaturalDecayModelProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateNatural(mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateNatural)(::core::mem::transmute_copy(this), mingain, maxgain, unitygaindistance, cutoffdistance, &mut result__).from_abi::<AudioNodeEmitterDecayModel>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateCustom(mingain: f64, maxgain: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCustom)(::core::mem::transmute_copy(this), mingain, maxgain, &mut result__).from_abi::<AudioNodeEmitterDecayModel>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterDecayModelStatics<R, F: FnOnce(&IAudioNodeEmitterDecayModelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeEmitterDecayModel, IAudioNodeEmitterDecayModelStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioNodeEmitterDecayModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterDecayModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterDecayModel {}
impl ::core::fmt::Debug for AudioNodeEmitterDecayModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterDecayModel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterDecayModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterDecayModel;{1d1d5af7-0d53-4fa9-bd84-d5816a86f3ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_Vtbl;
    const IID: ::windows::core::GUID = <IAudioNodeEmitterDecayModel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitterDecayModel {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterDecayModel";
}
impl ::core::convert::From<AudioNodeEmitterDecayModel> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitterDecayModel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterDecayModel> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitterDecayModel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitterDecayModel> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitterDecayModel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterDecayModel> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitterDecayModel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterDecayModel {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterDecayModel {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(::windows::core::IUnknown);
impl AudioNodeEmitterNaturalDecayModelProperties {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn UnityGainDistance(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnityGainDistance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CutoffDistance(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CutoffDistance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioNodeEmitterNaturalDecayModelProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterNaturalDecayModelProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterNaturalDecayModelProperties {}
impl ::core::fmt::Debug for AudioNodeEmitterNaturalDecayModelProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterNaturalDecayModelProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterNaturalDecayModelProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties;{48934bcf-cf2c-4efc-9331-75bd22df1f0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_Vtbl;
    const IID: ::windows::core::GUID = <IAudioNodeEmitterNaturalDecayModelProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitterNaturalDecayModelProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties";
}
impl ::core::convert::From<AudioNodeEmitterNaturalDecayModelProperties> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterNaturalDecayModelProperties> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitterNaturalDecayModelProperties> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterNaturalDecayModelProperties> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterNaturalDecayModelProperties {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterNaturalDecayModelProperties {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: Self = Self(0u32);
    pub const DisableDoppler: Self = Self(1u32);
}
impl ::core::marker::Copy for AudioNodeEmitterSettings {}
impl ::core::clone::Clone for AudioNodeEmitterSettings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterSettings {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioNodeEmitterSettings {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioNodeEmitterSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterSettings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AudioNodeEmitterSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AudioNodeEmitterSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AudioNodeEmitterSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterSettings;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterShape(::windows::core::IUnknown);
impl AudioNodeEmitterShape {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterShapeKind> {
        let this = self;
        unsafe {
            let mut result__: AudioNodeEmitterShapeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterShapeKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConeProperties(&self) -> ::windows::core::Result<AudioNodeEmitterConeProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConeProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterConeProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateCone(innerangle: f64, outerangle: f64, outeranglegain: f64) -> ::windows::core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCone)(::core::mem::transmute_copy(this), innerangle, outerangle, outeranglegain, &mut result__).from_abi::<AudioNodeEmitterShape>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateOmnidirectional() -> ::windows::core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateOmnidirectional)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterShape>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterShapeStatics<R, F: FnOnce(&IAudioNodeEmitterShapeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeEmitterShape, IAudioNodeEmitterShapeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioNodeEmitterShape {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterShape {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterShape {}
impl ::core::fmt::Debug for AudioNodeEmitterShape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterShape").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterShape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterShape;{ea0311c5-e73d-44bc-859c-45553bbc4828})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_Vtbl;
    const IID: ::windows::core::GUID = <IAudioNodeEmitterShape as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitterShape {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterShape";
}
impl ::core::convert::From<AudioNodeEmitterShape> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitterShape) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterShape> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitterShape) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeEmitterShape> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitterShape) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeEmitterShape> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitterShape) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterShape {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterShape {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: Self = Self(0i32);
    pub const Cone: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterShapeKind {}
impl ::core::clone::Clone for AudioNodeEmitterShapeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterShapeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioNodeEmitterShapeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioNodeEmitterShapeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterShapeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterShapeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterShapeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeListener(::windows::core::IUnknown);
impl AudioNodeListener {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeListener, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPosition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOrientation)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SpeedOfSound(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SpeedOfSound)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetSpeedOfSound(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSpeedOfSound)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DopplerVelocity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDopplerVelocity)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AudioNodeListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioNodeListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeListener {}
impl ::core::fmt::Debug for AudioNodeListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeListener;{d9722e16-0c0a-41da-b755-6c77835fb1eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioNodeListener {
    type Vtable = IAudioNodeListener_Vtbl;
    const IID: ::windows::core::GUID = <IAudioNodeListener as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeListener {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeListener";
}
impl ::core::convert::From<AudioNodeListener> for ::windows::core::IUnknown {
    fn from(value: AudioNodeListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeListener> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioNodeListener> for ::windows::core::IInspectable {
    fn from(value: AudioNodeListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioNodeListener> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioNodeListener {}
unsafe impl ::core::marker::Sync for AudioNodeListener {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioPlaybackConnection(::windows::core::IUnknown);
impl AudioPlaybackConnection {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn State(&self) -> ::windows::core::Result<AudioPlaybackConnectionState> {
        let this = self;
        unsafe {
            let mut result__: AudioPlaybackConnectionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioPlaybackConnectionState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Open(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Open)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioPlaybackConnectionOpenResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioPlaybackConnection, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn TryCreateFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<AudioPlaybackConnection> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateFromId)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<AudioPlaybackConnection>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAudioPlaybackConnectionStatics<R, F: FnOnce(&IAudioPlaybackConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioPlaybackConnection, IAudioPlaybackConnectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioPlaybackConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioPlaybackConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioPlaybackConnection {}
impl ::core::fmt::Debug for AudioPlaybackConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioPlaybackConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioPlaybackConnection;{1a4c1dea-cafc-50e7-8718-ea3f81cbfa51})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_Vtbl;
    const IID: ::windows::core::GUID = <IAudioPlaybackConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioPlaybackConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnection";
}
impl ::core::convert::From<AudioPlaybackConnection> for ::windows::core::IUnknown {
    fn from(value: AudioPlaybackConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioPlaybackConnection> for ::windows::core::IUnknown {
    fn from(value: &AudioPlaybackConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioPlaybackConnection> for ::windows::core::IInspectable {
    fn from(value: AudioPlaybackConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioPlaybackConnection> for ::windows::core::IInspectable {
    fn from(value: &AudioPlaybackConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioPlaybackConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioPlaybackConnection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioPlaybackConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioPlaybackConnection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioPlaybackConnection {}
unsafe impl ::core::marker::Sync for AudioPlaybackConnection {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResult(::windows::core::IUnknown);
impl AudioPlaybackConnectionOpenResult {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResultStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioPlaybackConnectionOpenResultStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioPlaybackConnectionOpenResultStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioPlaybackConnectionOpenResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioPlaybackConnectionOpenResult {}
impl ::core::fmt::Debug for AudioPlaybackConnectionOpenResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionOpenResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioPlaybackConnectionOpenResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioPlaybackConnectionOpenResult;{4e656aef-39f9-5fc9-a519-a5bbfd9fe921})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_Vtbl;
    const IID: ::windows::core::GUID = <IAudioPlaybackConnectionOpenResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioPlaybackConnectionOpenResult {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnectionOpenResult";
}
impl ::core::convert::From<AudioPlaybackConnectionOpenResult> for ::windows::core::IUnknown {
    fn from(value: AudioPlaybackConnectionOpenResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioPlaybackConnectionOpenResult> for ::windows::core::IUnknown {
    fn from(value: &AudioPlaybackConnectionOpenResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioPlaybackConnectionOpenResult> for ::windows::core::IInspectable {
    fn from(value: AudioPlaybackConnectionOpenResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioPlaybackConnectionOpenResult> for ::windows::core::IInspectable {
    fn from(value: &AudioPlaybackConnectionOpenResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioPlaybackConnectionOpenResult {}
unsafe impl ::core::marker::Sync for AudioPlaybackConnectionOpenResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: Self = Self(0i32);
    pub const RequestTimedOut: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionOpenResultStatus {}
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioPlaybackConnectionOpenResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioPlaybackConnectionOpenResultStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioPlaybackConnectionOpenResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionOpenResultStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioPlaybackConnectionOpenResultStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionOpenResultStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionState {}
impl ::core::clone::Clone for AudioPlaybackConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioPlaybackConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioPlaybackConnectionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioPlaybackConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioPlaybackConnectionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioStateMonitor(::windows::core::IUnknown);
impl AudioStateMonitor {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SoundLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioStateMonitor, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoundLevelChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSoundLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSoundLevelChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SoundLevel(&self) -> ::windows::core::Result<super::SoundLevel> {
        let this = self;
        unsafe {
            let mut result__: super::SoundLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoundLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SoundLevel>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateForRenderMonitoring() -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForRenderMonitoring)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategory(category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForRenderMonitoringWithCategory)(::core::mem::transmute_copy(this), category, &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Devices\"`, `\"Media_Render\"`*"]
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceRole(category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForRenderMonitoringWithCategoryAndDeviceRole)(::core::mem::transmute_copy(this), category, role, &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceId<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(category: super::Render::AudioRenderCategory, deviceid: Param1) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForRenderMonitoringWithCategoryAndDeviceId)(::core::mem::transmute_copy(this), category, deviceid.into_param().abi(), &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn CreateForCaptureMonitoring() -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForCaptureMonitoring)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategory(category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForCaptureMonitoringWithCategory)(::core::mem::transmute_copy(this), category, &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Capture\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceRole(category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForCaptureMonitoringWithCategoryAndDeviceRole)(::core::mem::transmute_copy(this), category, role, &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceId<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(category: super::Capture::MediaCategory, deviceid: Param1) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForCaptureMonitoringWithCategoryAndDeviceId)(::core::mem::transmute_copy(this), category, deviceid.into_param().abi(), &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioStateMonitorStatics<R, F: FnOnce(&IAudioStateMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioStateMonitor, IAudioStateMonitorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioStateMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioStateMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioStateMonitor {}
impl ::core::fmt::Debug for AudioStateMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStateMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioStateMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioStateMonitor;{1d13d136-0199-4cdc-b84e-e72c2b581ece})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
    const IID: ::windows::core::GUID = <IAudioStateMonitor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioStateMonitor {
    const NAME: &'static str = "Windows.Media.Audio.AudioStateMonitor";
}
impl ::core::convert::From<AudioStateMonitor> for ::windows::core::IUnknown {
    fn from(value: AudioStateMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioStateMonitor> for ::windows::core::IUnknown {
    fn from(value: &AudioStateMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioStateMonitor> for ::windows::core::IInspectable {
    fn from(value: AudioStateMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioStateMonitor> for ::windows::core::IInspectable {
    fn from(value: &AudioStateMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AudioStateMonitor {}
unsafe impl ::core::marker::Sync for AudioStateMonitor {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioSubmixNode(::windows::core::IUnknown);
impl AudioSubmixNode {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Emitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AudioSubmixNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AudioSubmixNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioSubmixNode {}
impl ::core::fmt::Debug for AudioSubmixNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSubmixNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioSubmixNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioSubmixNode;{d148005c-8428-4784-b7fd-a99d468c5d20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioSubmixNode {
    type Vtable = IAudioInputNode_Vtbl;
    const IID: ::windows::core::GUID = <IAudioInputNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioSubmixNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioSubmixNode";
}
impl ::core::convert::From<AudioSubmixNode> for ::windows::core::IUnknown {
    fn from(value: AudioSubmixNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioSubmixNode> for ::windows::core::IUnknown {
    fn from(value: &AudioSubmixNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioSubmixNode> for ::windows::core::IInspectable {
    fn from(value: AudioSubmixNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioSubmixNode> for ::windows::core::IInspectable {
    fn from(value: &AudioSubmixNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AudioSubmixNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioSubmixNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioSubmixNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioSubmixNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioSubmixNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioSubmixNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioSubmixNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioSubmixNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioSubmixNode {}
unsafe impl ::core::marker::Sync for AudioSubmixNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioDeviceInputNodeResult(::windows::core::IUnknown);
impl CreateAudioDeviceInputNodeResult {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceNodeCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DeviceInputNode(&self) -> ::windows::core::Result<AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInputNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceInputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioDeviceInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioDeviceInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioDeviceInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioDeviceInputNodeResult {}
impl ::core::fmt::Debug for CreateAudioDeviceInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioDeviceInputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioDeviceInputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioDeviceInputNodeResult;{16eec7a8-1ca7-40ef-91a4-d346e0aa1bba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = <ICreateAudioDeviceInputNodeResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioDeviceInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceInputNodeResult";
}
impl ::core::convert::From<CreateAudioDeviceInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioDeviceInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioDeviceInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioDeviceInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioDeviceInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioDeviceInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioDeviceInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioDeviceInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioDeviceInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioDeviceInputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioDeviceOutputNodeResult(::windows::core::IUnknown);
impl CreateAudioDeviceOutputNodeResult {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceNodeCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DeviceOutputNode(&self) -> ::windows::core::Result<AudioDeviceOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceOutputNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceOutputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioDeviceOutputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioDeviceOutputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioDeviceOutputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioDeviceOutputNodeResult {}
impl ::core::fmt::Debug for CreateAudioDeviceOutputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioDeviceOutputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioDeviceOutputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioDeviceOutputNodeResult;{f7776d27-1d9a-47f7-9cd4-2859cc1b7bff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = <ICreateAudioDeviceOutputNodeResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioDeviceOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
}
impl ::core::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioDeviceOutputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioDeviceOutputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioFileInputNodeResult(::windows::core::IUnknown);
impl CreateAudioFileInputNodeResult {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioFileNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFileNodeCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn FileInputNode(&self) -> ::windows::core::Result<AudioFileInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileInputNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFileInputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioFileInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioFileInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioFileInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioFileInputNodeResult {}
impl ::core::fmt::Debug for CreateAudioFileInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioFileInputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioFileInputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioFileInputNodeResult;{ce83d61c-e297-4c50-9ce7-1c7a69d6bd09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = <ICreateAudioFileInputNodeResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioFileInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileInputNodeResult";
}
impl ::core::convert::From<CreateAudioFileInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioFileInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioFileInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioFileInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioFileInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioFileInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioFileInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioFileInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioFileInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioFileInputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioFileOutputNodeResult(::windows::core::IUnknown);
impl CreateAudioFileOutputNodeResult {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioFileNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFileNodeCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn FileOutputNode(&self) -> ::windows::core::Result<AudioFileOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileOutputNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFileOutputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioFileOutputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioFileOutputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioFileOutputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioFileOutputNodeResult {}
impl ::core::fmt::Debug for CreateAudioFileOutputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioFileOutputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioFileOutputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioFileOutputNodeResult;{47d6ba7b-e909-453f-866e-5540cda734ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = <ICreateAudioFileOutputNodeResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioFileOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileOutputNodeResult";
}
impl ::core::convert::From<CreateAudioFileOutputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioFileOutputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioFileOutputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioFileOutputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioFileOutputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioFileOutputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioFileOutputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioFileOutputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioFileOutputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioFileOutputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioGraphResult(::windows::core::IUnknown);
impl CreateAudioGraphResult {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AudioGraphCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioGraphCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioGraphCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Graph(&self) -> ::windows::core::Result<AudioGraph> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Graph)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioGraph>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioGraphResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateAudioGraphResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateAudioGraphResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioGraphResult {}
impl ::core::fmt::Debug for CreateAudioGraphResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioGraphResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioGraphResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioGraphResult;{5453ef7e-7bde-4b76-bb5d-48f79cfc8c0b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_Vtbl;
    const IID: ::windows::core::GUID = <ICreateAudioGraphResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioGraphResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioGraphResult";
}
impl ::core::convert::From<CreateAudioGraphResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioGraphResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioGraphResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioGraphResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioGraphResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioGraphResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateAudioGraphResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioGraphResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateAudioGraphResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioGraphResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioGraphResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioGraphResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateAudioGraphResult {}
unsafe impl ::core::marker::Sync for CreateAudioGraphResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateMediaSourceAudioInputNodeResult(::windows::core::IUnknown);
impl CreateMediaSourceAudioInputNodeResult {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MediaSourceAudioInputNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaSourceAudioInputNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceAudioInputNodeCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Node(&self) -> ::windows::core::Result<MediaSourceAudioInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Node)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceAudioInputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateMediaSourceAudioInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for CreateMediaSourceAudioInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CreateMediaSourceAudioInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateMediaSourceAudioInputNodeResult {}
impl ::core::fmt::Debug for CreateMediaSourceAudioInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateMediaSourceAudioInputNodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CreateMediaSourceAudioInputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult;{46a658a3-53c0-4d59-9e51-cc1d1044a4c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = <ICreateMediaSourceAudioInputNodeResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CreateMediaSourceAudioInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult";
}
impl ::core::convert::From<CreateMediaSourceAudioInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateMediaSourceAudioInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateMediaSourceAudioInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateMediaSourceAudioInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CreateMediaSourceAudioInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateMediaSourceAudioInputNodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CreateMediaSourceAudioInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateMediaSourceAudioInputNodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CreateMediaSourceAudioInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateMediaSourceAudioInputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct EchoEffectDefinition(::windows::core::IUnknown);
impl EchoEffectDefinition {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWetDryMix)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn WetDryMix(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WetDryMix)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetFeedback(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFeedback)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Feedback(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Feedback)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetDelay(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDelay)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Delay(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Delay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows::core::Result<EchoEffectDefinition> {
        Self::IEchoEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), audiograph.into_param().abi(), &mut result__).from_abi::<EchoEffectDefinition>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEchoEffectDefinitionFactory<R, F: FnOnce(&IEchoEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EchoEffectDefinition, IEchoEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EchoEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EchoEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EchoEffectDefinition {}
impl ::core::fmt::Debug for EchoEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EchoEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EchoEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EchoEffectDefinition;{0e4d3faa-36b8-4c91-b9da-11f44a8a6610})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_Vtbl;
    const IID: ::windows::core::GUID = <IEchoEffectDefinition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EchoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EchoEffectDefinition";
}
impl ::core::convert::From<EchoEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: EchoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EchoEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &EchoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EchoEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: EchoEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EchoEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &EchoEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<EchoEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: EchoEffectDefinition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<&EchoEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: &EchoEffectDefinition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for &EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for EchoEffectDefinition {}
unsafe impl ::core::marker::Sync for EchoEffectDefinition {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct EqualizerBand(::windows::core::IUnknown);
impl EqualizerBand {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Bandwidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bandwidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetBandwidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBandwidth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn FrequencyCenter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrequencyCenter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetFrequencyCenter(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFrequencyCenter)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for EqualizerBand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EqualizerBand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EqualizerBand {}
impl ::core::fmt::Debug for EqualizerBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EqualizerBand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EqualizerBand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EqualizerBand;{c00a5a6a-262d-4b85-9bb7-43280b62ed0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EqualizerBand {
    type Vtable = IEqualizerBand_Vtbl;
    const IID: ::windows::core::GUID = <IEqualizerBand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EqualizerBand {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerBand";
}
impl ::core::convert::From<EqualizerBand> for ::windows::core::IUnknown {
    fn from(value: EqualizerBand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EqualizerBand> for ::windows::core::IUnknown {
    fn from(value: &EqualizerBand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EqualizerBand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EqualizerBand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EqualizerBand> for ::windows::core::IInspectable {
    fn from(value: EqualizerBand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EqualizerBand> for ::windows::core::IInspectable {
    fn from(value: &EqualizerBand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EqualizerBand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EqualizerBand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EqualizerBand {}
unsafe impl ::core::marker::Sync for EqualizerBand {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct EqualizerEffectDefinition(::windows::core::IUnknown);
impl EqualizerEffectDefinition {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EqualizerBand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<EqualizerBand>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows::core::Result<EqualizerEffectDefinition> {
        Self::IEqualizerEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), audiograph.into_param().abi(), &mut result__).from_abi::<EqualizerEffectDefinition>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEqualizerEffectDefinitionFactory<R, F: FnOnce(&IEqualizerEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EqualizerEffectDefinition, IEqualizerEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EqualizerEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EqualizerEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EqualizerEffectDefinition {}
impl ::core::fmt::Debug for EqualizerEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EqualizerEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EqualizerEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EqualizerEffectDefinition;{023f6f1f-83fe-449a-a822-c696442d16b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_Vtbl;
    const IID: ::windows::core::GUID = <IEqualizerEffectDefinition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EqualizerEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerEffectDefinition";
}
impl ::core::convert::From<EqualizerEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: EqualizerEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EqualizerEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &EqualizerEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EqualizerEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: EqualizerEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EqualizerEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &EqualizerEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<EqualizerEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: EqualizerEffectDefinition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<&EqualizerEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: &EqualizerEffectDefinition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for &EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for EqualizerEffectDefinition {}
unsafe impl ::core::marker::Sync for EqualizerEffectDefinition {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct FrameInputNodeQuantumStartedEventArgs(::windows::core::IUnknown);
impl FrameInputNodeQuantumStartedEventArgs {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RequiredSamples(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequiredSamples)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameInputNodeQuantumStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameInputNodeQuantumStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameInputNodeQuantumStartedEventArgs {}
impl ::core::fmt::Debug for FrameInputNodeQuantumStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameInputNodeQuantumStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameInputNodeQuantumStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs;{3d9bd498-a306-4f06-bd9f-e9efc8226304})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFrameInputNodeQuantumStartedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameInputNodeQuantumStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs";
}
impl ::core::convert::From<FrameInputNodeQuantumStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FrameInputNodeQuantumStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameInputNodeQuantumStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FrameInputNodeQuantumStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FrameInputNodeQuantumStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FrameInputNodeQuantumStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameInputNodeQuantumStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FrameInputNodeQuantumStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FrameInputNodeQuantumStartedEventArgs {}
unsafe impl ::core::marker::Sync for FrameInputNodeQuantumStartedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceInputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb01b6be1_6f4e_49e2_ac01_559d62beb3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceInputNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceOutputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x362edbff_ff1c_4434_9e0f_bd2ef522ac82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceOutputNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFileInputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFileInputNode {
    type Vtable = IAudioFileInputNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905b67c8_6f65_4cd4_8890_4694843c276d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileInputNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seek: usize,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub LoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub SetLoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage")]
    pub SourceFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SourceFile: usize,
    #[cfg(feature = "Foundation")]
    pub FileCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFileOutputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50e01980_5166_4093_80f8_ada00089e9cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileOutputNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub FileEncodingProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FileEncodingProfile: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))]
    pub FinalizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding")))]
    FinalizeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc7c829e_0208_4504_a5a8_f0f268920a65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameInputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b266c7_fd96_4ff5_a3c5_d27a9bf44237);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameInputNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AddFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueuedSampleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioFrameCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioFrameCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioFrameCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioFrameCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub QuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumStarted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameOutputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb847371b_3299_45f5_88b3_c9d12a3f1cc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameOutputNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraph {
    type Vtable = IAudioGraph_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ad46eed_e48c_4e14_9660_2c4f83e9cdd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFrameInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameInputNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameInputNodeWithFormat: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))]
    pub CreateDeviceInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture")))]
    CreateDeviceInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatOnDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatOnDeviceAsync: usize,
    pub CreateFrameOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameOutputNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameOutputNodeWithFormat: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDeviceOutputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDeviceOutputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileOutputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileOutputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub CreateFileOutputNodeWithFileProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, fileencodingprofile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    CreateFileOutputNodeWithFileProfileAsync: usize,
    pub CreateSubmixNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateSubmixNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateSubmixNodeWithFormat: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResetAllNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub QuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub QuantumProcessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumProcessed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumProcessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumProcessed: usize,
    #[cfg(feature = "Foundation")]
    pub UnrecoverableErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnrecoverableErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnrecoverableErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnrecoverableErrorOccurred: usize,
    pub CompletedQuantumCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub LatencyInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    PrimaryRenderDevice: usize,
    pub RenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
    pub SamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraph2 {
    type Vtable = IAudioGraph2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e4c3bd5_4fc1_45f6_a947_3cd38f4fd839);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameInputNodeWithFormatAndEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameInputNodeWithFormatAndEmitter: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileInputNodeWithEmitterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileInputNodeWithEmitterAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateSubmixNodeWithFormatAndEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateSubmixNodeWithFormatAndEmitter: usize,
    #[cfg(feature = "Foundation")]
    pub CreateBatchUpdater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateBatchUpdater: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraph3 {
    type Vtable = IAudioGraph3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddcd25ae_1185_42a7_831d_6a9b0fc86820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateMediaSourceAudioInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateMediaSourceAudioInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateMediaSourceAudioInputNodeWithEmitterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateMediaSourceAudioInputNodeWithEmitterAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphConnection {
    type Vtable = IAudioGraphConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x763070ed_d04e_4fac_b233_600b42edd469);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphConnection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphSettings {
    type Vtable = IAudioGraphSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d59647f_e6fe_4628_84f8_9d8bdba25785);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetEncodingProperties: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    PrimaryRenderDevice: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPrimaryRenderDevice: usize,
    pub QuantumSizeSelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut QuantumSizeSelectionMode) -> ::windows::core::HRESULT,
    pub SetQuantumSizeSelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: QuantumSizeSelectionMode) -> ::windows::core::HRESULT,
    pub DesiredSamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetDesiredSamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub AudioRenderCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Render::AudioRenderCategory) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    AudioRenderCategory: usize,
    #[cfg(feature = "Media_Render")]
    pub SetAudioRenderCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Render::AudioRenderCategory) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    SetAudioRenderCategory: usize,
    pub DesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
    pub SetDesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphSettings2 {
    type Vtable = IAudioGraphSettings2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72919787_4dab_46e3_b4c9_d8e1a2636062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetMaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub MaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettingsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphSettingsFactory {
    type Vtable = IAudioGraphSettingsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5d91cc6_c2eb_4a61_a214_1d66d75f83da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettingsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_Render")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiorendercategory: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphStatics {
    type Vtable = IAudioGraphStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76ec3132_e159_4ab7_a82a_17beb4b31e94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3d9cbe0_3ff6_4fb3_b262_50d435c55423);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioGraphUnrecoverableError) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioInputNode(::windows::core::IUnknown);
impl IAudioInputNode {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IAudioInputNode> for ::windows::core::IUnknown {
    fn from(value: IAudioInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputNode> for ::windows::core::IUnknown {
    fn from(value: &IAudioInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioInputNode> for ::windows::core::IInspectable {
    fn from(value: IAudioInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputNode> for ::windows::core::IInspectable {
    fn from(value: &IAudioInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAudioInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: IAudioInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAudioInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IAudioInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IAudioInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IAudioInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAudioInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAudioInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputNode {}
impl ::core::fmt::Debug for IAudioInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d148005c-8428-4784-b7fd-a99d468c5d20}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAudioInputNode {
    type Vtable = IAudioInputNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd148005c_8428_4784_b7fd_a99d468c5d20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub OutgoingConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutgoingConnections: usize,
    pub AddOutgoingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddOutgoingConnectionWithGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, gain: f64) -> ::windows::core::HRESULT,
    pub RemoveOutgoingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioInputNode2(::windows::core::IUnknown);
impl IAudioInputNode2 {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Emitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IAudioInputNode2> for ::windows::core::IUnknown {
    fn from(value: IAudioInputNode2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputNode2> for ::windows::core::IUnknown {
    fn from(value: &IAudioInputNode2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioInputNode2> for ::windows::core::IInspectable {
    fn from(value: IAudioInputNode2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputNode2> for ::windows::core::IInspectable {
    fn from(value: &IAudioInputNode2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAudioInputNode2> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioInputNode2> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAudioInputNode2> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioInputNode2> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IAudioInputNode2> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IAudioInputNode2> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAudioInputNode2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioInputNode2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputNode2 {}
impl ::core::fmt::Debug for IAudioInputNode2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputNode2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioInputNode2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905156b7-ca68-4c6d-a8bc-e3ee17fe3fd2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAudioInputNode2 {
    type Vtable = IAudioInputNode2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905156b7_ca68_4c6d_a8bc_e3ee17fe3fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Emitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioNode(::windows::core::IUnknown);
impl IAudioNode {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IAudioNode> for ::windows::core::IUnknown {
    fn from(value: IAudioNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioNode> for ::windows::core::IUnknown {
    fn from(value: &IAudioNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioNode> for ::windows::core::IInspectable {
    fn from(value: IAudioNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioNode> for ::windows::core::IInspectable {
    fn from(value: &IAudioNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IAudioNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IAudioNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IAudioNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAudioNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAudioNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioNode {}
impl ::core::fmt::Debug for IAudioNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{15389d7f-dbd8-4819-bf03-668e9357cd6d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAudioNode {
    type Vtable = IAudioNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15389d7f_dbd8_4819_bf03_668e9357cd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub EffectDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    EffectDefinitions: usize,
    pub SetOutgoingGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub OutgoingGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub ConsumeInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetConsumeInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Effects")]
    pub DisableEffectsByDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    DisableEffectsByDefinition: usize,
    #[cfg(feature = "Media_Effects")]
    pub EnableEffectsByDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    EnableEffectsByDefinition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3676971d_880a_47b8_adf7_1323a9d965be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Direction: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDirection: usize,
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DecayModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DistanceScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDistanceScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DopplerScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDopplerScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DopplerVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDopplerVelocity: usize,
    pub IsDopplerDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitter2 {
    type Vtable = IAudioNodeEmitter2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ab6eecb_ec29_47f8_818c_b6b660a5aeb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SpatialAudioModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialAudioModel) -> ::windows::core::HRESULT,
    pub SetSpatialAudioModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpatialAudioModel) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterConeProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe99b2cee_02ca_4375_9326_0c6ae4bcdfb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterConeProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub InnerAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub OuterAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub OuterAngleGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d1d5af7_0d53_4fa9_bd84_d5816a86f3ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModel_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioNodeEmitterDecayKind) -> ::windows::core::HRESULT,
    pub MinGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub NaturalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModelStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterDecayModelStatics {
    type Vtable = IAudioNodeEmitterDecayModelStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7787ca8_f178_462f_bc81_8dd5cbe5dae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModelStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateNatural: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mingain: f64, maxgain: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterFactory {
    type Vtable = IAudioNodeEmitterFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdc8489a_6ad6_4ce4_b7f7_a99370df7ee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateAudioNodeEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: ::windows::core::RawPtr, decaymodel: ::windows::core::RawPtr, settings: AudioNodeEmitterSettings, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48934bcf_cf2c_4efc_9331_75bd22df1f0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UnityGainDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub CutoffDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterShape(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea0311c5_e73d_44bc_859c_45553bbc4828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShape_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioNodeEmitterShapeKind) -> ::windows::core::HRESULT,
    pub ConeProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterShapeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterShapeStatics {
    type Vtable = IAudioNodeEmitterShapeStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57bb2771_ffa5_4b86_a779_e264aeb9145f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShapeStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateCone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, innerangle: f64, outerangle: f64, outeranglegain: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateOmnidirectional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeListener {
    type Vtable = IAudioNodeListener_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9722e16_0c0a_41da_b755_6c77835fb1eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeListener_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOrientation: usize,
    pub SpeedOfSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetSpeedOfSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DopplerVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDopplerVelocity: usize,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioNodeWithListener(::windows::core::IUnknown);
impl IAudioNodeWithListener {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetListener<'a, Param0: ::windows::core::IntoParam<'a, AudioNodeListener>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetListener)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Listener(&self) -> ::windows::core::Result<AudioNodeListener> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Listener)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeListener>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IAudioNodeWithListener> for ::windows::core::IUnknown {
    fn from(value: IAudioNodeWithListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioNodeWithListener> for ::windows::core::IUnknown {
    fn from(value: &IAudioNodeWithListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioNodeWithListener> for ::windows::core::IInspectable {
    fn from(value: IAudioNodeWithListener) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioNodeWithListener> for ::windows::core::IInspectable {
    fn from(value: &IAudioNodeWithListener) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAudioNodeWithListener> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: IAudioNodeWithListener) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAudioNodeWithListener> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAudioNodeWithListener) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IAudioNodeWithListener> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IAudioNodeWithListener) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IAudioNodeWithListener> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAudioNodeWithListener) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IAudioNodeWithListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioNodeWithListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioNodeWithListener {}
impl ::core::fmt::Debug for IAudioNodeWithListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioNodeWithListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioNodeWithListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0e0f907c-79ff-4544-9eeb-01257b15105a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAudioNodeWithListener {
    type Vtable = IAudioNodeWithListener_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e0f907c_79ff_4544_9eeb_01257b15105a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeWithListener_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Listener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a4c1dea_cafc_50e7_8718_ea3f81cbfa51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioPlaybackConnectionState) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnectionOpenResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e656aef_39f9_5fc9_a519_a5bbfd9fe921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionOpenResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioPlaybackConnectionOpenResultStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnectionStatics {
    type Vtable = IAudioPlaybackConnectionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe60963a2_69e6_5ffc_9e13_824a85213daf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TryCreateFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStateMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d13d136_0199_4cdc_b84e_e72c2b581ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSoundLevelChanged: usize,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SoundLevel) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStateMonitorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioStateMonitorStatics {
    type Vtable = IAudioStateMonitorStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6374ea4c_1b3b_4001_94d9_dd225330fa40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitorStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateForRenderMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub CreateForRenderMonitoringWithCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateForRenderMonitoringWithCategory: usize,
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub CreateForRenderMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices", feature = "Media_Render")))]
    CreateForRenderMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "Media_Render")]
    pub CreateForRenderMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateForRenderMonitoringWithCategoryAndDeviceId: usize,
    pub CreateForCaptureMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub CreateForCaptureMonitoringWithCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateForCaptureMonitoringWithCategory: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices")))]
    CreateForCaptureMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateForCaptureMonitoringWithCategoryAndDeviceId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16eec7a8_1ca7_40ef_91a4_d346e0aa1bba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows::core::HRESULT,
    pub DeviceInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceInputNodeResult2 {
    type Vtable = ICreateAudioDeviceInputNodeResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x921c69ce_3f35_41c7_9622_79f608baedc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7776d27_1d9a_47f7_9cd4_2859cc1b7bff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows::core::HRESULT,
    pub DeviceOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceOutputNodeResult2 {
    type Vtable = ICreateAudioDeviceOutputNodeResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4864269f_bdce_4ab1_bd38_fbae93aedaca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce83d61c_e297_4c50_9ce7_1c7a69d6bd09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioFileNodeCreationStatus) -> ::windows::core::HRESULT,
    pub FileInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioFileInputNodeResult2 {
    type Vtable = ICreateAudioFileInputNodeResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9082020_3d80_4fe0_81c1_768fea7ca7e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47d6ba7b_e909_453f_866e_5540cda734ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioFileNodeCreationStatus) -> ::windows::core::HRESULT,
    pub FileOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioFileOutputNodeResult2 {
    type Vtable = ICreateAudioFileOutputNodeResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f01b50d_3318_47b3_a60a_1b492be7fc0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioGraphResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5453ef7e_7bde_4b76_bb5d_48f79cfc8c0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioGraphCreationStatus) -> ::windows::core::HRESULT,
    pub Graph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioGraphResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioGraphResult2 {
    type Vtable = ICreateAudioGraphResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d738dfc_88c6_4fcb_a534_85cedd4050a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a658a3_53c0_4d59_9e51_cc1d1044a4c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceAudioInputNodeCreationStatus) -> ::windows::core::HRESULT,
    pub Node: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateMediaSourceAudioInputNodeResult2 {
    type Vtable = ICreateMediaSourceAudioInputNodeResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63514ce8_6a1a_49e3_97ec_28fd5be114e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEchoEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e4d3faa_36b8_4c91_b9da_11f44a8a6610);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinition_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetWetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Feedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEchoEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEchoEffectDefinitionFactory {
    type Vtable = IEchoEffectDefinitionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d4e2257_aaf2_4e86_a54c_fb79db8f6c12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinitionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerBand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEqualizerBand {
    type Vtable = IEqualizerBand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00a5a6a_262d_4b85_9bb7_43280b62ed0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerBand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Bandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub FrequencyCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetFrequencyCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023f6f1f_83fe_449a_a822_c696442d16b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinition_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Bands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bands: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEqualizerEffectDefinitionFactory {
    type Vtable = IEqualizerEffectDefinitionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2876fc4_d410_4eb5_9e69_c9aa1277eaf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinitionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameInputNodeQuantumStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d9bd498_a306_4f06_bd9f_e9efc8226304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameInputNodeQuantumStartedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequiredSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimiterEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b755d19_2603_47ba_bdeb_39055e3486dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinition_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLoudness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Loudness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimiterEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILimiterEffectDefinitionFactory {
    type Vtable = ILimiterEffectDefinitionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecbae6f1_61ff_45ef_b8f5_48659a57c72d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinitionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAudioInputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99d8983b_a88a_4041_8e4f_ddbac0c91fd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAudioInputNode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seek: usize,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub LoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub SetLoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    #[cfg(feature = "Foundation")]
    pub MediaSourceCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaSourceCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaSourceCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaSourceCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReverbEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4606aa89_f563_4d0a_8f6e_f0cddff35d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinition_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetWetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetReflectionsDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReflectionsDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReverbDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub ReverbDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetRearDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub RearDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetPositionLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub PositionLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetPositionRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub PositionRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetPositionMatrixLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub PositionMatrixLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetPositionMatrixRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub PositionMatrixRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetEarlyDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub EarlyDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetLateDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub LateDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetLowEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub LowEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetLowEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub LowEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetHighEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub HighEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetHighEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub HighEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetRoomFilterFreq: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RoomFilterFreq: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRoomFilterMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RoomFilterMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRoomFilterHF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RoomFilterHF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetReflectionsGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub ReflectionsGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetReverbGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub ReverbGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDecayTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DecayTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDensity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Density: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRoomSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RoomSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDisableLateField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DisableLateField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReverbEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReverbEffectDefinitionFactory {
    type Vtable = IReverbEffectDefinitionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7d5cbfe_100b_4ff0_9da6_dc4e05a759f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinitionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISetDefaultSpatialAudioFormatResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2aa511_1400_5e70_9ea9_ae151241e8ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetDefaultSpatialAudioFormatResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SetDefaultSpatialAudioFormatStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee830034_61cf_5749_9da4_10f0fe028199);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsSpatialAudioSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSpatialAudioFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ActiveSpatialAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DefaultSpatialAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultSpatialAudioFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultSpatialAudioFormatAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfigurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfigurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioDeviceConfigurationStatics {
    type Vtable = ISpatialAudioDeviceConfigurationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ec37f7b_936d_4e04_9728_2827d9f758c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfigurationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32df09a8_50f0_5395_9923_7d44ca71ed6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ReportLicenseChangedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportLicenseChangedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportConfigurationChangedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConfigurationChangedAsync: usize,
    pub MixedRealityExclusiveModePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::HRESULT,
    pub SetMixedRealityExclusiveModePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatConfigurationStatics {
    type Vtable = ISpatialAudioFormatConfigurationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b5fef71_67c9_4e5f_a35b_41680711f8c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfigurationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatSubtypeStatics {
    type Vtable = ISpatialAudioFormatSubtypeStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3de8a47_83ee_4266_a945_bedf507afeed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub WindowsSonic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DolbyAtmosForHeadphones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DolbyAtmosForHomeTheater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DolbyAtmosForSpeakers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DTSHeadphoneX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DTSXUltra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatSubtypeStatics2 {
    type Vtable = ISpatialAudioFormatSubtypeStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4565e6cb_d95b_5621_b6af_0e8849c57c80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DTSXForHomeTheater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct LimiterEffectDefinition(::windows::core::IUnknown);
impl LimiterEffectDefinition {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetRelease(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRelease)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Release(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Release)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetLoudness(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLoudness)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Loudness(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Loudness)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows::core::Result<LimiterEffectDefinition> {
        Self::ILimiterEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), audiograph.into_param().abi(), &mut result__).from_abi::<LimiterEffectDefinition>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILimiterEffectDefinitionFactory<R, F: FnOnce(&ILimiterEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LimiterEffectDefinition, ILimiterEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LimiterEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LimiterEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LimiterEffectDefinition {}
impl ::core::fmt::Debug for LimiterEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LimiterEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LimiterEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.LimiterEffectDefinition;{6b755d19-2603-47ba-bdeb-39055e3486dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_Vtbl;
    const IID: ::windows::core::GUID = <ILimiterEffectDefinition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LimiterEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.LimiterEffectDefinition";
}
impl ::core::convert::From<LimiterEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: LimiterEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LimiterEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &LimiterEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LimiterEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: LimiterEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LimiterEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &LimiterEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<LimiterEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: LimiterEffectDefinition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<&LimiterEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: &LimiterEffectDefinition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for &LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LimiterEffectDefinition {}
unsafe impl ::core::marker::Sync for LimiterEffectDefinition {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct MediaSourceAudioInputNode(::windows::core::IUnknown);
impl MediaSourceAudioInputNode {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Emitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlaybackSpeedFactor)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackSpeedFactor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Seek<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, position: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Seek)(::core::mem::transmute_copy(this), position.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoopCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLoopCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLoopCount)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::MediaSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaSourceCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaSourceAudioInputNode, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaSourceCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaSourceCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMediaSourceCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaSourceAudioInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaSourceAudioInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceAudioInputNode {}
impl ::core::fmt::Debug for MediaSourceAudioInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAudioInputNode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceAudioInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.MediaSourceAudioInputNode;{99d8983b-a88a-4041-8e4f-ddbac0c91fd3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_Vtbl;
    const IID: ::windows::core::GUID = <IMediaSourceAudioInputNode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaSourceAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.MediaSourceAudioInputNode";
}
impl ::core::convert::From<MediaSourceAudioInputNode> for ::windows::core::IUnknown {
    fn from(value: MediaSourceAudioInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceAudioInputNode> for ::windows::core::IUnknown {
    fn from(value: &MediaSourceAudioInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaSourceAudioInputNode> for ::windows::core::IInspectable {
    fn from(value: MediaSourceAudioInputNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaSourceAudioInputNode> for ::windows::core::IInspectable {
    fn from(value: &MediaSourceAudioInputNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MediaSourceAudioInputNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioInputNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::core::convert::TryInto::<IAudioInputNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<MediaSourceAudioInputNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioInputNode2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode2> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode2> {
        ::core::convert::TryInto::<IAudioInputNode2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<MediaSourceAudioInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioNode> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioNode> {
        ::core::convert::TryInto::<IAudioNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaSourceAudioInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaSourceAudioInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaSourceAudioInputNode {}
unsafe impl ::core::marker::Sync for MediaSourceAudioInputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaSourceAudioInputNodeCreationStatus {}
impl ::core::clone::Clone for MediaSourceAudioInputNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaSourceAudioInputNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaSourceAudioInputNodeCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaSourceAudioInputNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAudioInputNodeCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceAudioInputNodeCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MediaSourceAudioInputNodeCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: Self = Self(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: Self = Self(1i32);
}
impl ::core::marker::Copy for MixedRealitySpatialAudioFormatPolicy {}
impl ::core::clone::Clone for MixedRealitySpatialAudioFormatPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MixedRealitySpatialAudioFormatPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MixedRealitySpatialAudioFormatPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for MixedRealitySpatialAudioFormatPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MixedRealitySpatialAudioFormatPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MixedRealitySpatialAudioFormatPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MixedRealitySpatialAudioFormatPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: Self = Self(0i32);
    pub const LowestLatency: Self = Self(1i32);
    pub const ClosestToDesired: Self = Self(2i32);
}
impl ::core::marker::Copy for QuantumSizeSelectionMode {}
impl ::core::clone::Clone for QuantumSizeSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QuantumSizeSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QuantumSizeSelectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for QuantumSizeSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuantumSizeSelectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for QuantumSizeSelectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.QuantumSizeSelectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct ReverbEffectDefinition(::windows::core::IUnknown);
impl ReverbEffectDefinition {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWetDryMix)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn WetDryMix(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WetDryMix)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetReflectionsDelay(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReflectionsDelay)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ReflectionsDelay(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReflectionsDelay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetReverbDelay(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReverbDelay)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ReverbDelay(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReverbDelay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetRearDelay(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRearDelay)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RearDelay(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RearDelay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetPositionLeft(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionLeft)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn PositionLeft(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionLeft)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetPositionRight(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionRight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn PositionRight(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionRight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetPositionMatrixLeft(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionMatrixLeft)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn PositionMatrixLeft(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionMatrixLeft)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetPositionMatrixRight(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionMatrixRight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn PositionMatrixRight(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionMatrixRight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetEarlyDiffusion(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEarlyDiffusion)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn EarlyDiffusion(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EarlyDiffusion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetLateDiffusion(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLateDiffusion)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn LateDiffusion(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LateDiffusion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetLowEQGain(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLowEQGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn LowEQGain(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LowEQGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetLowEQCutoff(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLowEQCutoff)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn LowEQCutoff(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LowEQCutoff)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetHighEQGain(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHighEQGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn HighEQGain(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HighEQGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetHighEQCutoff(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHighEQCutoff)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn HighEQCutoff(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HighEQCutoff)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetRoomFilterFreq(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoomFilterFreq)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RoomFilterFreq(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoomFilterFreq)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetRoomFilterMain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoomFilterMain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RoomFilterMain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoomFilterMain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetRoomFilterHF(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoomFilterHF)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RoomFilterHF(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoomFilterHF)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetReflectionsGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReflectionsGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ReflectionsGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReflectionsGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetReverbGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReverbGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ReverbGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReverbGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetDecayTime(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDecayTime)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DecayTime(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DecayTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetDensity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDensity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Density(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Density)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetRoomSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoomSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn RoomSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RoomSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetDisableLateField(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisableLateField)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DisableLateField(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableLateField)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows::core::Result<ReverbEffectDefinition> {
        Self::IReverbEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), audiograph.into_param().abi(), &mut result__).from_abi::<ReverbEffectDefinition>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IReverbEffectDefinitionFactory<R, F: FnOnce(&IReverbEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ReverbEffectDefinition, IReverbEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ReverbEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReverbEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReverbEffectDefinition {}
impl ::core::fmt::Debug for ReverbEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReverbEffectDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReverbEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.ReverbEffectDefinition;{4606aa89-f563-4d0a-8f6e-f0cddff35d84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_Vtbl;
    const IID: ::windows::core::GUID = <IReverbEffectDefinition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ReverbEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.ReverbEffectDefinition";
}
impl ::core::convert::From<ReverbEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: ReverbEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReverbEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &ReverbEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReverbEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: ReverbEffectDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReverbEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &ReverbEffectDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<ReverbEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: ReverbEffectDefinition) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::convert::TryFrom<&ReverbEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReverbEffectDefinition) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition> for &ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::core::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ReverbEffectDefinition {}
unsafe impl ::core::marker::Sync for ReverbEffectDefinition {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatResult(::windows::core::IUnknown);
impl SetDefaultSpatialAudioFormatResult {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<SetDefaultSpatialAudioFormatStatus> {
        let this = self;
        unsafe {
            let mut result__: SetDefaultSpatialAudioFormatStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SetDefaultSpatialAudioFormatStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SetDefaultSpatialAudioFormatResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SetDefaultSpatialAudioFormatResult {}
impl ::core::fmt::Debug for SetDefaultSpatialAudioFormatResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetDefaultSpatialAudioFormatResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SetDefaultSpatialAudioFormatResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SetDefaultSpatialAudioFormatResult;{1c2aa511-1400-5e70-9ea9-ae151241e8ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_Vtbl;
    const IID: ::windows::core::GUID = <ISetDefaultSpatialAudioFormatResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SetDefaultSpatialAudioFormatResult {
    const NAME: &'static str = "Windows.Media.Audio.SetDefaultSpatialAudioFormatResult";
}
impl ::core::convert::From<SetDefaultSpatialAudioFormatResult> for ::windows::core::IUnknown {
    fn from(value: SetDefaultSpatialAudioFormatResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SetDefaultSpatialAudioFormatResult> for ::windows::core::IUnknown {
    fn from(value: &SetDefaultSpatialAudioFormatResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SetDefaultSpatialAudioFormatResult> for ::windows::core::IInspectable {
    fn from(value: SetDefaultSpatialAudioFormatResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SetDefaultSpatialAudioFormatResult> for ::windows::core::IInspectable {
    fn from(value: &SetDefaultSpatialAudioFormatResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SetDefaultSpatialAudioFormatResult {}
unsafe impl ::core::marker::Sync for SetDefaultSpatialAudioFormatResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const LicenseExpired: Self = Self(2i32);
    pub const LicenseNotValidForAudioEndpoint: Self = Self(3i32);
    pub const NotSupportedOnAudioEndpoint: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl ::core::marker::Copy for SetDefaultSpatialAudioFormatStatus {}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetDefaultSpatialAudioFormatStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SetDefaultSpatialAudioFormatStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for SetDefaultSpatialAudioFormatStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetDefaultSpatialAudioFormatStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SetDefaultSpatialAudioFormatStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SetDefaultSpatialAudioFormatStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct SpatialAudioDeviceConfiguration(::windows::core::IUnknown);
impl SpatialAudioDeviceConfiguration {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn IsSpatialAudioSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSpatialAudioSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn IsSpatialAudioFormatSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, subtype: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSpatialAudioFormatSupported)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn ActiveSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActiveSpatialAudioFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DefaultSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultSpatialAudioFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultSpatialAudioFormatAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, subtype: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetDefaultSpatialAudioFormatAsync)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConfigurationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConfigurationChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConfigurationChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn GetForDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<SpatialAudioDeviceConfiguration> {
        Self::ISpatialAudioDeviceConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForDeviceId)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<SpatialAudioDeviceConfiguration>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioDeviceConfigurationStatics<R, F: FnOnce(&ISpatialAudioDeviceConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAudioDeviceConfiguration, ISpatialAudioDeviceConfigurationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialAudioDeviceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAudioDeviceConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAudioDeviceConfiguration {}
impl ::core::fmt::Debug for SpatialAudioDeviceConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioDeviceConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAudioDeviceConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SpatialAudioDeviceConfiguration;{ee830034-61cf-5749-9da4-10f0fe028199})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialAudioDeviceConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAudioDeviceConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioDeviceConfiguration";
}
impl ::core::convert::From<SpatialAudioDeviceConfiguration> for ::windows::core::IUnknown {
    fn from(value: SpatialAudioDeviceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAudioDeviceConfiguration> for ::windows::core::IUnknown {
    fn from(value: &SpatialAudioDeviceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAudioDeviceConfiguration> for ::windows::core::IInspectable {
    fn from(value: SpatialAudioDeviceConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAudioDeviceConfiguration> for ::windows::core::IInspectable {
    fn from(value: &SpatialAudioDeviceConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAudioDeviceConfiguration {}
unsafe impl ::core::marker::Sync for SpatialAudioDeviceConfiguration {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct SpatialAudioFormatConfiguration(::windows::core::IUnknown);
impl SpatialAudioFormatConfiguration {
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportLicenseChangedAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, subtype: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportLicenseChangedAsync)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportConfigurationChangedAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, subtype: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportConfigurationChangedAsync)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn MixedRealityExclusiveModePolicy(&self) -> ::windows::core::Result<MixedRealitySpatialAudioFormatPolicy> {
        let this = self;
        unsafe {
            let mut result__: MixedRealitySpatialAudioFormatPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MixedRealityExclusiveModePolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MixedRealitySpatialAudioFormatPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn SetMixedRealityExclusiveModePolicy(&self, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMixedRealityExclusiveModePolicy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<SpatialAudioFormatConfiguration> {
        Self::ISpatialAudioFormatConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialAudioFormatConfiguration>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatConfigurationStatics<R, F: FnOnce(&ISpatialAudioFormatConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAudioFormatConfiguration, ISpatialAudioFormatConfigurationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpatialAudioFormatConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialAudioFormatConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAudioFormatConfiguration {}
impl ::core::fmt::Debug for SpatialAudioFormatConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioFormatConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAudioFormatConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SpatialAudioFormatConfiguration;{32df09a8-50f0-5395-9923-7d44ca71ed6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialAudioFormatConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAudioFormatConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatConfiguration";
}
impl ::core::convert::From<SpatialAudioFormatConfiguration> for ::windows::core::IUnknown {
    fn from(value: SpatialAudioFormatConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAudioFormatConfiguration> for ::windows::core::IUnknown {
    fn from(value: &SpatialAudioFormatConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialAudioFormatConfiguration> for ::windows::core::IInspectable {
    fn from(value: SpatialAudioFormatConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialAudioFormatConfiguration> for ::windows::core::IInspectable {
    fn from(value: &SpatialAudioFormatConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialAudioFormatConfiguration {}
unsafe impl ::core::marker::Sync for SpatialAudioFormatConfiguration {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
pub struct SpatialAudioFormatSubtype {}
impl SpatialAudioFormatSubtype {
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn WindowsSonic() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowsSonic)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DolbyAtmosForHeadphones() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DolbyAtmosForHeadphones)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DolbyAtmosForHomeTheater() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DolbyAtmosForHomeTheater)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DolbyAtmosForSpeakers() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DolbyAtmosForSpeakers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DTSHeadphoneX() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DTSHeadphoneX)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DTSXUltra() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DTSXUltra)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    pub fn DTSXForHomeTheater() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DTSXForHomeTheater)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatSubtypeStatics<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatSubtypeStatics2<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SpatialAudioFormatSubtype {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatSubtype";
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: Self = Self(0i32);
    pub const FoldDown: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAudioModel {}
impl ::core::clone::Clone for SpatialAudioModel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioModel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioModel {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAudioModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioModel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAudioModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SpatialAudioModel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
