#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioDeviceInputNode(pub ::windows::core::IInspectable);
impl AudioDeviceInputNode {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioDeviceInputNode;{b01b6be1-6f4e-49e2-ac01-559d62beb3a9})");
}
unsafe impl ::windows::core::Interface for AudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb01b6be1_6f4e_49e2_ac01_559d62beb3a9);
}
impl ::windows::core::RuntimeName for AudioDeviceInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceInputNode";
}
impl ::core::convert::From<AudioDeviceInputNode> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceInputNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioDeviceInputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceInputNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioDeviceInputNode> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceInputNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioDeviceInputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioDeviceInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::core::marker::Send for AudioDeviceInputNode {}
unsafe impl ::core::marker::Sync for AudioDeviceInputNode {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(0i32);
    pub const DeviceNotAvailable: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(1i32);
    pub const FormatNotSupported: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(2i32);
    pub const UnknownFailure: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(3i32);
    pub const AccessDenied: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(4i32);
}
impl ::core::convert::From<i32> for AudioDeviceNodeCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioDeviceNodeCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceNodeCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioDeviceNodeCreationStatus;i4)");
}
impl ::windows::core::DefaultType for AudioDeviceNodeCreationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioDeviceOutputNode(pub ::windows::core::IInspectable);
impl AudioDeviceOutputNode {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    pub fn SetListener<'a, Param0: ::windows::core::IntoParam<'a, AudioNodeListener>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Listener(&self) -> ::windows::core::Result<AudioNodeListener> {
        let this = &::windows::core::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeListener>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioDeviceOutputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioDeviceOutputNode;{362edbff-ff1c-4434-9e0f-bd2ef522ac82})");
}
unsafe impl ::windows::core::Interface for AudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x362edbff_ff1c_4434_9e0f_bd2ef522ac82);
}
impl ::windows::core::RuntimeName for AudioDeviceOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceOutputNode";
}
impl ::core::convert::From<AudioDeviceOutputNode> for ::windows::core::IUnknown {
    fn from(value: AudioDeviceOutputNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioDeviceOutputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioDeviceOutputNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioDeviceOutputNode> for ::windows::core::IInspectable {
    fn from(value: AudioDeviceOutputNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioDeviceOutputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioDeviceOutputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::core::marker::Send for AudioDeviceOutputNode {}
unsafe impl ::core::marker::Sync for AudioDeviceOutputNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioFileInputNode(pub ::windows::core::IInspectable);
impl AudioFileInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Seek<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, position: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLoopCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SourceFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FileCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioFileInputNode, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFileInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFileInputNode;{905b67c8-6f65-4cd4-8890-4694843c276d})");
}
unsafe impl ::windows::core::Interface for AudioFileInputNode {
    type Vtable = IAudioFileInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905b67c8_6f65_4cd4_8890_4694843c276d);
}
impl ::windows::core::RuntimeName for AudioFileInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileInputNode";
}
impl ::core::convert::From<AudioFileInputNode> for ::windows::core::IUnknown {
    fn from(value: AudioFileInputNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioFileInputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioFileInputNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioFileInputNode> for ::windows::core::IInspectable {
    fn from(value: AudioFileInputNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioFileInputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioFileInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFileInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::core::marker::Send for AudioFileInputNode {}
unsafe impl ::core::marker::Sync for AudioFileInputNode {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(0i32);
    pub const FileNotFound: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(1i32);
    pub const InvalidFileType: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(2i32);
    pub const FormatNotSupported: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(3i32);
    pub const UnknownFailure: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(4i32);
}
impl ::core::convert::From<i32> for AudioFileNodeCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioFileNodeCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioFileNodeCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioFileNodeCreationStatus;i4)");
}
impl ::windows::core::DefaultType for AudioFileNodeCreationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioFileOutputNode(pub ::windows::core::IInspectable);
impl AudioFileOutputNode {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FileEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaEncodingProfile>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))]
    pub fn FinalizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFileOutputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFileOutputNode;{50e01980-5166-4093-80f8-ada00089e9cf})");
}
unsafe impl ::windows::core::Interface for AudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50e01980_5166_4093_80f8_ada00089e9cf);
}
impl ::windows::core::RuntimeName for AudioFileOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileOutputNode";
}
impl ::core::convert::From<AudioFileOutputNode> for ::windows::core::IUnknown {
    fn from(value: AudioFileOutputNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioFileOutputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioFileOutputNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioFileOutputNode> for ::windows::core::IInspectable {
    fn from(value: AudioFileOutputNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioFileOutputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioFileOutputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFileOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::core::marker::Send for AudioFileOutputNode {}
unsafe impl ::core::marker::Sync for AudioFileOutputNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioFrameCompletedEventArgs(pub ::windows::core::IInspectable);
impl AudioFrameCompletedEventArgs {
    pub fn Frame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioFrame>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFrameCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameCompletedEventArgs;{dc7c829e-0208-4504-a5a8-f0f268920a65})");
}
unsafe impl ::windows::core::Interface for AudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc7c829e_0208_4504_a5a8_f0f268920a65);
}
impl ::windows::core::RuntimeName for AudioFrameCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameCompletedEventArgs";
}
impl ::core::convert::From<AudioFrameCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AudioFrameCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioFrameCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AudioFrameCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioFrameCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AudioFrameCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioFrameCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AudioFrameCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFrameCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioFrameCompletedEventArgs {}
unsafe impl ::core::marker::Sync for AudioFrameCompletedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioFrameInputNode(pub ::windows::core::IInspectable);
impl AudioFrameInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn AddFrame<'a, Param0: ::windows::core::IntoParam<'a, super::AudioFrame>>(&self, frame: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), frame.into_param().abi()).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn QueuedSampleCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AudioFrameCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioFrameCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn QuantumStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFrameInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameInputNode;{01b266c7-fd96-4ff5-a3c5-d27a9bf44237})");
}
unsafe impl ::windows::core::Interface for AudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b266c7_fd96_4ff5_a3c5_d27a9bf44237);
}
impl ::windows::core::RuntimeName for AudioFrameInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameInputNode";
}
impl ::core::convert::From<AudioFrameInputNode> for ::windows::core::IUnknown {
    fn from(value: AudioFrameInputNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioFrameInputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioFrameInputNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioFrameInputNode> for ::windows::core::IInspectable {
    fn from(value: AudioFrameInputNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioFrameInputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioFrameInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFrameInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::core::marker::Send for AudioFrameInputNode {}
unsafe impl ::core::marker::Sync for AudioFrameInputNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioFrameOutputNode(pub ::windows::core::IInspectable);
impl AudioFrameOutputNode {
    pub fn GetFrame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioFrameOutputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameOutputNode;{b847371b-3299-45f5-88b3-c9d12a3f1cc8})");
}
unsafe impl ::windows::core::Interface for AudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb847371b_3299_45f5_88b3_c9d12a3f1cc8);
}
impl ::windows::core::RuntimeName for AudioFrameOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameOutputNode";
}
impl ::core::convert::From<AudioFrameOutputNode> for ::windows::core::IUnknown {
    fn from(value: AudioFrameOutputNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioFrameOutputNode> for ::windows::core::IUnknown {
    fn from(value: &AudioFrameOutputNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioFrameOutputNode> for ::windows::core::IInspectable {
    fn from(value: AudioFrameOutputNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioFrameOutputNode> for ::windows::core::IInspectable {
    fn from(value: &AudioFrameOutputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFrameOutputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::core::marker::Send for AudioFrameOutputNode {}
unsafe impl ::core::marker::Sync for AudioFrameOutputNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioGraph(pub ::windows::core::IInspectable);
impl AudioGraph {
    pub fn CreateFrameInputNode(&self) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))]
    pub fn CreateDeviceInputNodeAsync(&self, category: super::Capture::MediaCategory) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), category, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, category: super::Capture::MediaCategory, encodingproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), category, encodingproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatOnDeviceAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param2: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(&self, category: super::Capture::MediaCategory, encodingproperties: Param1, device: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), category, encodingproperties.into_param().abi(), device.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    pub fn CreateFrameOutputNode(&self) -> ::windows::core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFrameOutputNode>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameOutputNodeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<AudioFrameOutputNode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateDeviceOutputNodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileInputNodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileOutputNodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CreateFileOutputNodeWithFileProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, file: Param0, fileencodingprofile: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), file.into_param().abi(), fileencodingprofile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>(result__)
        }
    }
    pub fn CreateSubmixNode(&self) -> ::windows::core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioSubmixNode>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormat<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<AudioSubmixNode>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn ResetAllNodes(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn QuantumStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn QuantumProcessed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumProcessed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn UnrecoverableErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnrecoverableErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn CompletedQuantumCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn LatencyInSamples(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    pub fn RenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__: super::AudioProcessing = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    pub fn SamplesPerQuantum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync<'a, Param0: ::windows::core::IntoParam<'a, AudioGraphSettings>>(settings: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>> {
        Self::IAudioGraphStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), settings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>>(result__)
        })
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormatAndEmitter<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, encodingproperties: Param0, emitter: Param1) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param2: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>, Param3: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(
        &self,
        category: super::Capture::MediaCategory,
        encodingproperties: Param1,
        device: Param2,
        emitter: Param3,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), category, encodingproperties.into_param().abi(), device.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileInputNodeWithEmitterAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, file: Param0, emitter: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), file.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormatAndEmitter<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, encodingproperties: Param0, emitter: Param1) -> ::windows::core::Result<AudioSubmixNode> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<AudioSubmixNode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateBatchUpdater(&self) -> ::windows::core::Result<AudioGraphBatchUpdater> {
        let this = &::windows::core::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioGraphBatchUpdater>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateMediaSourceAudioInputNodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Core::MediaSource>>(&self, mediasource: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>> {
        let this = &::windows::core::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mediasource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateMediaSourceAudioInputNodeWithEmitterAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Core::MediaSource>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitter>>(&self, mediasource: Param0, emitter: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>> {
        let this = &::windows::core::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mediasource.into_param().abi(), emitter.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>(result__)
        }
    }
    pub fn IAudioGraphStatics<R, F: FnOnce(&IAudioGraphStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioGraph, IAudioGraphStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraph {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraph;{1ad46eed-e48c-4e14-9660-2c4f83e9cdd8})");
}
unsafe impl ::windows::core::Interface for AudioGraph {
    type Vtable = IAudioGraph_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ad46eed_e48c_4e14_9660_2c4f83e9cdd8);
}
impl ::windows::core::RuntimeName for AudioGraph {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraph";
}
impl ::core::convert::From<AudioGraph> for ::windows::core::IUnknown {
    fn from(value: AudioGraph) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioGraph> for ::windows::core::IUnknown {
    fn from(value: &AudioGraph) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioGraph> for ::windows::core::IInspectable {
    fn from(value: AudioGraph) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioGraph> for ::windows::core::IInspectable {
    fn from(value: &AudioGraph) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[cfg(feature = "Foundation")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioGraphBatchUpdater(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl AudioGraphBatchUpdater {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for AudioGraphBatchUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphBatchUpdater;{30d5a829-7fa4-4026-83bb-d75bae4ea99e})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for AudioGraphBatchUpdater {
    type Vtable = super::super::Foundation::IClosable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d5a829_7fa4_4026_83bb_d75bae4ea99e);
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for AudioGraphBatchUpdater {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphBatchUpdater";
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<AudioGraphBatchUpdater> for ::windows::core::IUnknown {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&AudioGraphBatchUpdater> for ::windows::core::IUnknown {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<AudioGraphBatchUpdater> for ::windows::core::IInspectable {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&AudioGraphBatchUpdater> for ::windows::core::IInspectable {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<AudioGraphBatchUpdater> for super::super::Foundation::IClosable {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&AudioGraphBatchUpdater> for super::super::Foundation::IClosable {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AudioGraphBatchUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for AudioGraphBatchUpdater {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for AudioGraphBatchUpdater {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioGraphConnection(pub ::windows::core::IInspectable);
impl AudioGraphConnection {
    pub fn Destination(&self) -> ::windows::core::Result<IAudioNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IAudioNode>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraphConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphConnection;{763070ed-d04e-4fac-b233-600b42edd469})");
}
unsafe impl ::windows::core::Interface for AudioGraphConnection {
    type Vtable = IAudioGraphConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x763070ed_d04e_4fac_b233_600b42edd469);
}
impl ::windows::core::RuntimeName for AudioGraphConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphConnection";
}
impl ::core::convert::From<AudioGraphConnection> for ::windows::core::IUnknown {
    fn from(value: AudioGraphConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioGraphConnection> for ::windows::core::IUnknown {
    fn from(value: &AudioGraphConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraphConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraphConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioGraphConnection> for ::windows::core::IInspectable {
    fn from(value: AudioGraphConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioGraphConnection> for ::windows::core::IInspectable {
    fn from(value: &AudioGraphConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraphConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraphConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioGraphConnection {}
unsafe impl ::core::marker::Sync for AudioGraphConnection {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: AudioGraphCreationStatus = AudioGraphCreationStatus(0i32);
    pub const DeviceNotAvailable: AudioGraphCreationStatus = AudioGraphCreationStatus(1i32);
    pub const FormatNotSupported: AudioGraphCreationStatus = AudioGraphCreationStatus(2i32);
    pub const UnknownFailure: AudioGraphCreationStatus = AudioGraphCreationStatus(3i32);
}
impl ::core::convert::From<i32> for AudioGraphCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioGraphCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioGraphCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphCreationStatus;i4)");
}
impl ::windows::core::DefaultType for AudioGraphCreationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioGraphSettings(pub ::windows::core::IInspectable);
impl AudioGraphSettings {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPrimaryRenderDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn QuantumSizeSelectionMode(&self) -> ::windows::core::Result<QuantumSizeSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: QuantumSizeSelectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<QuantumSizeSelectionMode>(result__)
        }
    }
    pub fn SetQuantumSizeSelectionMode(&self, value: QuantumSizeSelectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DesiredSamplesPerQuantum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetDesiredSamplesPerQuantum(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Render")]
    pub fn AudioRenderCategory(&self) -> ::windows::core::Result<super::Render::AudioRenderCategory> {
        let this = self;
        unsafe {
            let mut result__: super::Render::AudioRenderCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Render::AudioRenderCategory>(result__)
        }
    }
    #[cfg(feature = "Media_Render")]
    pub fn SetAudioRenderCategory(&self, value: super::Render::AudioRenderCategory) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DesiredRenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__: super::AudioProcessing = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    pub fn SetDesiredRenderDeviceAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Media_Render")]
    pub fn Create(audiorendercategory: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioGraphSettings> {
        Self::IAudioGraphSettingsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), audiorendercategory, &mut result__).from_abi::<AudioGraphSettings>(result__)
        })
    }
    pub fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MaxPlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn IAudioGraphSettingsFactory<R, F: FnOnce(&IAudioGraphSettingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioGraphSettings, IAudioGraphSettingsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraphSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphSettings;{1d59647f-e6fe-4628-84f8-9d8bdba25785})");
}
unsafe impl ::windows::core::Interface for AudioGraphSettings {
    type Vtable = IAudioGraphSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d59647f_e6fe_4628_84f8_9d8bdba25785);
}
impl ::windows::core::RuntimeName for AudioGraphSettings {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphSettings";
}
impl ::core::convert::From<AudioGraphSettings> for ::windows::core::IUnknown {
    fn from(value: AudioGraphSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioGraphSettings> for ::windows::core::IUnknown {
    fn from(value: &AudioGraphSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraphSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraphSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioGraphSettings> for ::windows::core::IInspectable {
    fn from(value: AudioGraphSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioGraphSettings> for ::windows::core::IInspectable {
    fn from(value: &AudioGraphSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraphSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraphSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioGraphSettings {}
unsafe impl ::core::marker::Sync for AudioGraphSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(0i32);
    pub const AudioDeviceLost: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(1i32);
    pub const AudioSessionDisconnected: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(2i32);
    pub const UnknownFailure: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(3i32);
}
impl ::core::convert::From<i32> for AudioGraphUnrecoverableError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioGraphUnrecoverableError {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioGraphUnrecoverableError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphUnrecoverableError;i4)");
}
impl ::windows::core::DefaultType for AudioGraphUnrecoverableError {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(pub ::windows::core::IInspectable);
impl AudioGraphUnrecoverableErrorOccurredEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<AudioGraphUnrecoverableError> {
        let this = self;
        unsafe {
            let mut result__: AudioGraphUnrecoverableError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioGraphUnrecoverableError>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs;{c3d9cbe0-3ff6-4fb3-b262-50d435c55423})");
}
unsafe impl ::windows::core::Interface for AudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3d9cbe0_3ff6_4fb3_b262_50d435c55423);
}
impl ::windows::core::RuntimeName for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs";
}
impl ::core::convert::From<AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioGraphUnrecoverableErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioGraphUnrecoverableErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for AudioGraphUnrecoverableErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioNodeEmitter(pub ::windows::core::IInspectable);
impl AudioNodeEmitter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeEmitter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDirection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Shape(&self) -> ::windows::core::Result<AudioNodeEmitterShape> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterShape>(result__)
        }
    }
    pub fn DecayModel(&self) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterDecayModel>(result__)
        }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DistanceScale(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDistanceScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DopplerScale(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDopplerScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsDopplerDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SpatialAudioModel(&self) -> ::windows::core::Result<SpatialAudioModel> {
        let this = &::windows::core::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe {
            let mut result__: SpatialAudioModel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialAudioModel>(result__)
        }
    }
    pub fn SetSpatialAudioModel(&self, value: SpatialAudioModel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CreateAudioNodeEmitter<'a, Param0: ::windows::core::IntoParam<'a, AudioNodeEmitterShape>, Param1: ::windows::core::IntoParam<'a, AudioNodeEmitterDecayModel>>(shape: Param0, decaymodel: Param1, settings: AudioNodeEmitterSettings) -> ::windows::core::Result<AudioNodeEmitter> {
        Self::IAudioNodeEmitterFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), shape.into_param().abi(), decaymodel.into_param().abi(), settings, &mut result__).from_abi::<AudioNodeEmitter>(result__)
        })
    }
    pub fn IAudioNodeEmitterFactory<R, F: FnOnce(&IAudioNodeEmitterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeEmitter, IAudioNodeEmitterFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitter;{3676971d-880a-47b8-adf7-1323a9d965be})");
}
unsafe impl ::windows::core::Interface for AudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3676971d_880a_47b8_adf7_1323a9d965be);
}
impl ::windows::core::RuntimeName for AudioNodeEmitter {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitter";
}
impl ::core::convert::From<AudioNodeEmitter> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioNodeEmitter> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioNodeEmitter> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioNodeEmitter> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitter {}
unsafe impl ::core::marker::Sync for AudioNodeEmitter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioNodeEmitterConeProperties(pub ::windows::core::IInspectable);
impl AudioNodeEmitterConeProperties {
    pub fn InnerAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OuterAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn OuterAngleGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterConeProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterConeProperties;{e99b2cee-02ca-4375-9326-0c6ae4bcdfb5})");
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe99b2cee_02ca_4375_9326_0c6ae4bcdfb5);
}
impl ::windows::core::RuntimeName for AudioNodeEmitterConeProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterConeProperties";
}
impl ::core::convert::From<AudioNodeEmitterConeProperties> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitterConeProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioNodeEmitterConeProperties> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitterConeProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioNodeEmitterConeProperties> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitterConeProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioNodeEmitterConeProperties> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitterConeProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitterConeProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterConeProperties {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterConeProperties {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: AudioNodeEmitterDecayKind = AudioNodeEmitterDecayKind(0i32);
    pub const Custom: AudioNodeEmitterDecayKind = AudioNodeEmitterDecayKind(1i32);
}
impl ::core::convert::From<i32> for AudioNodeEmitterDecayKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioNodeEmitterDecayKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterDecayKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterDecayKind;i4)");
}
impl ::windows::core::DefaultType for AudioNodeEmitterDecayKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioNodeEmitterDecayModel(pub ::windows::core::IInspectable);
impl AudioNodeEmitterDecayModel {
    pub fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterDecayKind> {
        let this = self;
        unsafe {
            let mut result__: AudioNodeEmitterDecayKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterDecayKind>(result__)
        }
    }
    pub fn MinGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn MaxGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn NaturalProperties(&self) -> ::windows::core::Result<AudioNodeEmitterNaturalDecayModelProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterNaturalDecayModelProperties>(result__)
        }
    }
    pub fn CreateNatural(mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mingain, maxgain, unitygaindistance, cutoffdistance, &mut result__).from_abi::<AudioNodeEmitterDecayModel>(result__)
        })
    }
    pub fn CreateCustom(mingain: f64, maxgain: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mingain, maxgain, &mut result__).from_abi::<AudioNodeEmitterDecayModel>(result__)
        })
    }
    pub fn IAudioNodeEmitterDecayModelStatics<R, F: FnOnce(&IAudioNodeEmitterDecayModelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeEmitterDecayModel, IAudioNodeEmitterDecayModelStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterDecayModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterDecayModel;{1d1d5af7-0d53-4fa9-bd84-d5816a86f3ff})");
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d1d5af7_0d53_4fa9_bd84_d5816a86f3ff);
}
impl ::windows::core::RuntimeName for AudioNodeEmitterDecayModel {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterDecayModel";
}
impl ::core::convert::From<AudioNodeEmitterDecayModel> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitterDecayModel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioNodeEmitterDecayModel> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitterDecayModel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioNodeEmitterDecayModel> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitterDecayModel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioNodeEmitterDecayModel> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitterDecayModel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitterDecayModel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterDecayModel {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterDecayModel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(pub ::windows::core::IInspectable);
impl AudioNodeEmitterNaturalDecayModelProperties {
    pub fn UnityGainDistance(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn CutoffDistance(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterNaturalDecayModelProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties;{48934bcf-cf2c-4efc-9331-75bd22df1f0c})");
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48934bcf_cf2c_4efc_9331_75bd22df1f0c);
}
impl ::windows::core::RuntimeName for AudioNodeEmitterNaturalDecayModelProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties";
}
impl ::core::convert::From<AudioNodeEmitterNaturalDecayModelProperties> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioNodeEmitterNaturalDecayModelProperties> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioNodeEmitterNaturalDecayModelProperties> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioNodeEmitterNaturalDecayModelProperties> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitterNaturalDecayModelProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterNaturalDecayModelProperties {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterNaturalDecayModelProperties {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: AudioNodeEmitterSettings = AudioNodeEmitterSettings(0u32);
    pub const DisableDoppler: AudioNodeEmitterSettings = AudioNodeEmitterSettings(1u32);
}
impl ::core::convert::From<u32> for AudioNodeEmitterSettings {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioNodeEmitterSettings {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterSettings;u4)");
}
impl ::windows::core::DefaultType for AudioNodeEmitterSettings {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AudioNodeEmitterSettings {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AudioNodeEmitterSettings {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AudioNodeEmitterSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioNodeEmitterShape(pub ::windows::core::IInspectable);
impl AudioNodeEmitterShape {
    pub fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterShapeKind> {
        let this = self;
        unsafe {
            let mut result__: AudioNodeEmitterShapeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterShapeKind>(result__)
        }
    }
    pub fn ConeProperties(&self) -> ::windows::core::Result<AudioNodeEmitterConeProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterConeProperties>(result__)
        }
    }
    pub fn CreateCone(innerangle: f64, outerangle: f64, outeranglegain: f64) -> ::windows::core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), innerangle, outerangle, outeranglegain, &mut result__).from_abi::<AudioNodeEmitterShape>(result__)
        })
    }
    pub fn CreateOmnidirectional() -> ::windows::core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitterShape>(result__)
        })
    }
    pub fn IAudioNodeEmitterShapeStatics<R, F: FnOnce(&IAudioNodeEmitterShapeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeEmitterShape, IAudioNodeEmitterShapeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterShape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterShape;{ea0311c5-e73d-44bc-859c-45553bbc4828})");
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea0311c5_e73d_44bc_859c_45553bbc4828);
}
impl ::windows::core::RuntimeName for AudioNodeEmitterShape {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterShape";
}
impl ::core::convert::From<AudioNodeEmitterShape> for ::windows::core::IUnknown {
    fn from(value: AudioNodeEmitterShape) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioNodeEmitterShape> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeEmitterShape) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioNodeEmitterShape> for ::windows::core::IInspectable {
    fn from(value: AudioNodeEmitterShape) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioNodeEmitterShape> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeEmitterShape) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioNodeEmitterShape {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterShape {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: AudioNodeEmitterShapeKind = AudioNodeEmitterShapeKind(0i32);
    pub const Cone: AudioNodeEmitterShapeKind = AudioNodeEmitterShapeKind(1i32);
}
impl ::core::convert::From<i32> for AudioNodeEmitterShapeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioNodeEmitterShapeKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioNodeEmitterShapeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterShapeKind;i4)");
}
impl ::windows::core::DefaultType for AudioNodeEmitterShapeKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioNodeListener(pub ::windows::core::IInspectable);
impl AudioNodeListener {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioNodeListener, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SpeedOfSound(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpeedOfSound(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioNodeListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeListener;{d9722e16-0c0a-41da-b755-6c77835fb1eb})");
}
unsafe impl ::windows::core::Interface for AudioNodeListener {
    type Vtable = IAudioNodeListener_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9722e16_0c0a_41da_b755_6c77835fb1eb);
}
impl ::windows::core::RuntimeName for AudioNodeListener {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeListener";
}
impl ::core::convert::From<AudioNodeListener> for ::windows::core::IUnknown {
    fn from(value: AudioNodeListener) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioNodeListener> for ::windows::core::IUnknown {
    fn from(value: &AudioNodeListener) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioNodeListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioNodeListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioNodeListener> for ::windows::core::IInspectable {
    fn from(value: AudioNodeListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioNodeListener> for ::windows::core::IInspectable {
    fn from(value: &AudioNodeListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioNodeListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioNodeListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioNodeListener {}
unsafe impl ::core::marker::Sync for AudioNodeListener {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioPlaybackConnection(pub ::windows::core::IInspectable);
impl AudioPlaybackConnection {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<AudioPlaybackConnectionState> {
        let this = self;
        unsafe {
            let mut result__: AudioPlaybackConnectionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioPlaybackConnectionState>(result__)
        }
    }
    pub fn Open(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioPlaybackConnectionOpenResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioPlaybackConnection, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn TryCreateFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<AudioPlaybackConnection> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<AudioPlaybackConnection>(result__)
        })
    }
    pub fn IAudioPlaybackConnectionStatics<R, F: FnOnce(&IAudioPlaybackConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioPlaybackConnection, IAudioPlaybackConnectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioPlaybackConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioPlaybackConnection;{1a4c1dea-cafc-50e7-8718-ea3f81cbfa51})");
}
unsafe impl ::windows::core::Interface for AudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a4c1dea_cafc_50e7_8718_ea3f81cbfa51);
}
impl ::windows::core::RuntimeName for AudioPlaybackConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnection";
}
impl ::core::convert::From<AudioPlaybackConnection> for ::windows::core::IUnknown {
    fn from(value: AudioPlaybackConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioPlaybackConnection> for ::windows::core::IUnknown {
    fn from(value: &AudioPlaybackConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioPlaybackConnection> for ::windows::core::IInspectable {
    fn from(value: AudioPlaybackConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioPlaybackConnection> for ::windows::core::IInspectable {
    fn from(value: &AudioPlaybackConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioPlaybackConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioPlaybackConnectionOpenResult(pub ::windows::core::IInspectable);
impl AudioPlaybackConnectionOpenResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResultStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioPlaybackConnectionOpenResultStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioPlaybackConnectionOpenResultStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioPlaybackConnectionOpenResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioPlaybackConnectionOpenResult;{4e656aef-39f9-5fc9-a519-a5bbfd9fe921})");
}
unsafe impl ::windows::core::Interface for AudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e656aef_39f9_5fc9_a519_a5bbfd9fe921);
}
impl ::windows::core::RuntimeName for AudioPlaybackConnectionOpenResult {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnectionOpenResult";
}
impl ::core::convert::From<AudioPlaybackConnectionOpenResult> for ::windows::core::IUnknown {
    fn from(value: AudioPlaybackConnectionOpenResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioPlaybackConnectionOpenResult> for ::windows::core::IUnknown {
    fn from(value: &AudioPlaybackConnectionOpenResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioPlaybackConnectionOpenResult> for ::windows::core::IInspectable {
    fn from(value: AudioPlaybackConnectionOpenResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioPlaybackConnectionOpenResult> for ::windows::core::IInspectable {
    fn from(value: &AudioPlaybackConnectionOpenResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioPlaybackConnectionOpenResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioPlaybackConnectionOpenResult {}
unsafe impl ::core::marker::Sync for AudioPlaybackConnectionOpenResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: AudioPlaybackConnectionOpenResultStatus = AudioPlaybackConnectionOpenResultStatus(0i32);
    pub const RequestTimedOut: AudioPlaybackConnectionOpenResultStatus = AudioPlaybackConnectionOpenResultStatus(1i32);
    pub const DeniedBySystem: AudioPlaybackConnectionOpenResultStatus = AudioPlaybackConnectionOpenResultStatus(2i32);
    pub const UnknownFailure: AudioPlaybackConnectionOpenResultStatus = AudioPlaybackConnectionOpenResultStatus(3i32);
}
impl ::core::convert::From<i32> for AudioPlaybackConnectionOpenResultStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioPlaybackConnectionOpenResultStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioPlaybackConnectionOpenResultStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionOpenResultStatus;i4)");
}
impl ::windows::core::DefaultType for AudioPlaybackConnectionOpenResultStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: AudioPlaybackConnectionState = AudioPlaybackConnectionState(0i32);
    pub const Opened: AudioPlaybackConnectionState = AudioPlaybackConnectionState(1i32);
}
impl ::core::convert::From<i32> for AudioPlaybackConnectionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioPlaybackConnectionState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AudioPlaybackConnectionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionState;i4)");
}
impl ::windows::core::DefaultType for AudioPlaybackConnectionState {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioStateMonitor(pub ::windows::core::IInspectable);
impl AudioStateMonitor {
    #[cfg(feature = "Foundation")]
    pub fn SoundLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AudioStateMonitor, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSoundLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn SoundLevel(&self) -> ::windows::core::Result<super::SoundLevel> {
        let this = self;
        unsafe {
            let mut result__: super::SoundLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SoundLevel>(result__)
        }
    }
    pub fn CreateForRenderMonitoring() -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategory(category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), category, &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceRole(category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), category, role, &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceId<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(category: super::Render::AudioRenderCategory, deviceid: Param1) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), category, deviceid.into_param().abi(), &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    pub fn CreateForCaptureMonitoring() -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategory(category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), category, &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceRole(category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), category, role, &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceId<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(category: super::Capture::MediaCategory, deviceid: Param1) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), category, deviceid.into_param().abi(), &mut result__).from_abi::<AudioStateMonitor>(result__)
        })
    }
    pub fn IAudioStateMonitorStatics<R, F: FnOnce(&IAudioStateMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioStateMonitor, IAudioStateMonitorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioStateMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioStateMonitor;{1d13d136-0199-4cdc-b84e-e72c2b581ece})");
}
unsafe impl ::windows::core::Interface for AudioStateMonitor {
    type Vtable = IAudioStateMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d13d136_0199_4cdc_b84e_e72c2b581ece);
}
impl ::windows::core::RuntimeName for AudioStateMonitor {
    const NAME: &'static str = "Windows.Media.Audio.AudioStateMonitor";
}
impl ::core::convert::From<AudioStateMonitor> for ::windows::core::IUnknown {
    fn from(value: AudioStateMonitor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioStateMonitor> for ::windows::core::IUnknown {
    fn from(value: &AudioStateMonitor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioStateMonitor> for ::windows::core::IInspectable {
    fn from(value: AudioStateMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioStateMonitor> for ::windows::core::IInspectable {
    fn from(value: &AudioStateMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AudioStateMonitor {}
unsafe impl ::core::marker::Sync for AudioStateMonitor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AudioSubmixNode(pub ::windows::core::IInspectable);
impl AudioSubmixNode {
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AudioSubmixNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioSubmixNode;{d148005c-8428-4784-b7fd-a99d468c5d20})");
}
unsafe impl ::windows::core::Interface for AudioSubmixNode {
    type Vtable = IAudioInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd148005c_8428_4784_b7fd_a99d468c5d20);
}
impl ::windows::core::RuntimeName for AudioSubmixNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioSubmixNode";
}
impl ::core::convert::From<AudioSubmixNode> for ::windows::core::IUnknown {
    fn from(value: AudioSubmixNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AudioSubmixNode> for ::windows::core::IUnknown {
    fn from(value: &AudioSubmixNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AudioSubmixNode> for ::windows::core::IInspectable {
    fn from(value: AudioSubmixNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AudioSubmixNode> for ::windows::core::IInspectable {
    fn from(value: &AudioSubmixNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AudioSubmixNode> for IAudioInputNode {
    fn from(value: AudioSubmixNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioSubmixNode> for IAudioInputNode {
    fn from(value: &AudioSubmixNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioInputNode> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioInputNode> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::core::marker::Send for AudioSubmixNode {}
unsafe impl ::core::marker::Sync for AudioSubmixNode {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CreateAudioDeviceInputNodeResult(pub ::windows::core::IInspectable);
impl CreateAudioDeviceInputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceNodeCreationStatus>(result__)
        }
    }
    pub fn DeviceInputNode(&self) -> ::windows::core::Result<AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioDeviceInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioDeviceInputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioDeviceInputNodeResult;{16eec7a8-1ca7-40ef-91a4-d346e0aa1bba})");
}
unsafe impl ::windows::core::Interface for CreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16eec7a8_1ca7_40ef_91a4_d346e0aa1bba);
}
impl ::windows::core::RuntimeName for CreateAudioDeviceInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceInputNodeResult";
}
impl ::core::convert::From<CreateAudioDeviceInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioDeviceInputNodeResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreateAudioDeviceInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioDeviceInputNodeResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreateAudioDeviceInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioDeviceInputNodeResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreateAudioDeviceInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioDeviceInputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioDeviceInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreateAudioDeviceInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioDeviceInputNodeResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CreateAudioDeviceOutputNodeResult(pub ::windows::core::IInspectable);
impl CreateAudioDeviceOutputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceNodeCreationStatus>(result__)
        }
    }
    pub fn DeviceOutputNode(&self) -> ::windows::core::Result<AudioDeviceOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioDeviceOutputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioDeviceOutputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioDeviceOutputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioDeviceOutputNodeResult;{f7776d27-1d9a-47f7-9cd4-2859cc1b7bff})");
}
unsafe impl ::windows::core::Interface for CreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7776d27_1d9a_47f7_9cd4_2859cc1b7bff);
}
impl ::windows::core::RuntimeName for CreateAudioDeviceOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
}
impl ::core::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioDeviceOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreateAudioDeviceOutputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioDeviceOutputNodeResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CreateAudioFileInputNodeResult(pub ::windows::core::IInspectable);
impl CreateAudioFileInputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioFileNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFileNodeCreationStatus>(result__)
        }
    }
    pub fn FileInputNode(&self) -> ::windows::core::Result<AudioFileInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFileInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioFileInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioFileInputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioFileInputNodeResult;{ce83d61c-e297-4c50-9ce7-1c7a69d6bd09})");
}
unsafe impl ::windows::core::Interface for CreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce83d61c_e297_4c50_9ce7_1c7a69d6bd09);
}
impl ::windows::core::RuntimeName for CreateAudioFileInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileInputNodeResult";
}
impl ::core::convert::From<CreateAudioFileInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioFileInputNodeResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreateAudioFileInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioFileInputNodeResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreateAudioFileInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioFileInputNodeResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreateAudioFileInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioFileInputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioFileInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreateAudioFileInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioFileInputNodeResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CreateAudioFileOutputNodeResult(pub ::windows::core::IInspectable);
impl CreateAudioFileOutputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioFileNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFileNodeCreationStatus>(result__)
        }
    }
    pub fn FileOutputNode(&self) -> ::windows::core::Result<AudioFileOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioFileOutputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioFileOutputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioFileOutputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioFileOutputNodeResult;{47d6ba7b-e909-453f-866e-5540cda734ff})");
}
unsafe impl ::windows::core::Interface for CreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47d6ba7b_e909_453f_866e_5540cda734ff);
}
impl ::windows::core::RuntimeName for CreateAudioFileOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileOutputNodeResult";
}
impl ::core::convert::From<CreateAudioFileOutputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioFileOutputNodeResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreateAudioFileOutputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioFileOutputNodeResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreateAudioFileOutputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioFileOutputNodeResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreateAudioFileOutputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioFileOutputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioFileOutputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreateAudioFileOutputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioFileOutputNodeResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CreateAudioGraphResult(pub ::windows::core::IInspectable);
impl CreateAudioGraphResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioGraphCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioGraphCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioGraphCreationStatus>(result__)
        }
    }
    pub fn Graph(&self) -> ::windows::core::Result<AudioGraph> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioGraph>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateAudioGraphResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CreateAudioGraphResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioGraphResult;{5453ef7e-7bde-4b76-bb5d-48f79cfc8c0b})");
}
unsafe impl ::windows::core::Interface for CreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5453ef7e_7bde_4b76_bb5d_48f79cfc8c0b);
}
impl ::windows::core::RuntimeName for CreateAudioGraphResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioGraphResult";
}
impl ::core::convert::From<CreateAudioGraphResult> for ::windows::core::IUnknown {
    fn from(value: CreateAudioGraphResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreateAudioGraphResult> for ::windows::core::IUnknown {
    fn from(value: &CreateAudioGraphResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateAudioGraphResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateAudioGraphResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreateAudioGraphResult> for ::windows::core::IInspectable {
    fn from(value: CreateAudioGraphResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreateAudioGraphResult> for ::windows::core::IInspectable {
    fn from(value: &CreateAudioGraphResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateAudioGraphResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateAudioGraphResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreateAudioGraphResult {}
unsafe impl ::core::marker::Sync for CreateAudioGraphResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CreateMediaSourceAudioInputNodeResult(pub ::windows::core::IInspectable);
impl CreateMediaSourceAudioInputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<MediaSourceAudioInputNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaSourceAudioInputNodeCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceAudioInputNodeCreationStatus>(result__)
        }
    }
    pub fn Node(&self) -> ::windows::core::Result<MediaSourceAudioInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaSourceAudioInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<ICreateMediaSourceAudioInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CreateMediaSourceAudioInputNodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult;{46a658a3-53c0-4d59-9e51-cc1d1044a4c4})");
}
unsafe impl ::windows::core::Interface for CreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a658a3_53c0_4d59_9e51_cc1d1044a4c4);
}
impl ::windows::core::RuntimeName for CreateMediaSourceAudioInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult";
}
impl ::core::convert::From<CreateMediaSourceAudioInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: CreateMediaSourceAudioInputNodeResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CreateMediaSourceAudioInputNodeResult> for ::windows::core::IUnknown {
    fn from(value: &CreateMediaSourceAudioInputNodeResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CreateMediaSourceAudioInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: CreateMediaSourceAudioInputNodeResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CreateMediaSourceAudioInputNodeResult> for ::windows::core::IInspectable {
    fn from(value: &CreateMediaSourceAudioInputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CreateMediaSourceAudioInputNodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CreateMediaSourceAudioInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateMediaSourceAudioInputNodeResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EchoEffectDefinition(pub ::windows::core::IInspectable);
impl EchoEffectDefinition {
    pub fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WetDryMix(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFeedback(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Feedback(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDelay(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Delay(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows::core::Result<EchoEffectDefinition> {
        Self::IEchoEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), audiograph.into_param().abi(), &mut result__).from_abi::<EchoEffectDefinition>(result__)
        })
    }
    pub fn IEchoEffectDefinitionFactory<R, F: FnOnce(&IEchoEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EchoEffectDefinition, IEchoEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EchoEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EchoEffectDefinition;{0e4d3faa-36b8-4c91-b9da-11f44a8a6610})");
}
unsafe impl ::windows::core::Interface for EchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e4d3faa_36b8_4c91_b9da_11f44a8a6610);
}
impl ::windows::core::RuntimeName for EchoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EchoEffectDefinition";
}
impl ::core::convert::From<EchoEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: EchoEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EchoEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &EchoEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EchoEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: EchoEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EchoEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &EchoEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EchoEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EqualizerBand(pub ::windows::core::IInspectable);
impl EqualizerBand {
    pub fn Bandwidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetBandwidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FrequencyCenter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetFrequencyCenter(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for EqualizerBand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EqualizerBand;{c00a5a6a-262d-4b85-9bb7-43280b62ed0c})");
}
unsafe impl ::windows::core::Interface for EqualizerBand {
    type Vtable = IEqualizerBand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00a5a6a_262d_4b85_9bb7_43280b62ed0c);
}
impl ::windows::core::RuntimeName for EqualizerBand {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerBand";
}
impl ::core::convert::From<EqualizerBand> for ::windows::core::IUnknown {
    fn from(value: EqualizerBand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EqualizerBand> for ::windows::core::IUnknown {
    fn from(value: &EqualizerBand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EqualizerBand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EqualizerBand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EqualizerBand> for ::windows::core::IInspectable {
    fn from(value: EqualizerBand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EqualizerBand> for ::windows::core::IInspectable {
    fn from(value: &EqualizerBand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EqualizerBand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EqualizerBand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EqualizerBand {}
unsafe impl ::core::marker::Sync for EqualizerBand {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EqualizerEffectDefinition(pub ::windows::core::IInspectable);
impl EqualizerEffectDefinition {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EqualizerBand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<EqualizerBand>>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows::core::Result<EqualizerEffectDefinition> {
        Self::IEqualizerEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), audiograph.into_param().abi(), &mut result__).from_abi::<EqualizerEffectDefinition>(result__)
        })
    }
    pub fn IEqualizerEffectDefinitionFactory<R, F: FnOnce(&IEqualizerEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EqualizerEffectDefinition, IEqualizerEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EqualizerEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EqualizerEffectDefinition;{023f6f1f-83fe-449a-a822-c696442d16b0})");
}
unsafe impl ::windows::core::Interface for EqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023f6f1f_83fe_449a_a822_c696442d16b0);
}
impl ::windows::core::RuntimeName for EqualizerEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerEffectDefinition";
}
impl ::core::convert::From<EqualizerEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: EqualizerEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EqualizerEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &EqualizerEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EqualizerEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: EqualizerEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EqualizerEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &EqualizerEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EqualizerEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FrameInputNodeQuantumStartedEventArgs(pub ::windows::core::IInspectable);
impl FrameInputNodeQuantumStartedEventArgs {
    pub fn RequiredSamples(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FrameInputNodeQuantumStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs;{3d9bd498-a306-4f06-bd9f-e9efc8226304})");
}
unsafe impl ::windows::core::Interface for FrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d9bd498_a306_4f06_bd9f_e9efc8226304);
}
impl ::windows::core::RuntimeName for FrameInputNodeQuantumStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs";
}
impl ::core::convert::From<FrameInputNodeQuantumStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FrameInputNodeQuantumStartedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FrameInputNodeQuantumStartedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FrameInputNodeQuantumStartedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FrameInputNodeQuantumStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FrameInputNodeQuantumStartedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FrameInputNodeQuantumStartedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FrameInputNodeQuantumStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FrameInputNodeQuantumStartedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FrameInputNodeQuantumStartedEventArgs {}
unsafe impl ::core::marker::Sync for FrameInputNodeQuantumStartedEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioDeviceInputNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb01b6be1_6f4e_49e2_ac01_559d62beb3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceInputNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioDeviceOutputNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x362edbff_ff1c_4434_9e0f_bd2ef522ac82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceOutputNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioFileInputNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioFileInputNode {
    type Vtable = IAudioFileInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905b67c8_6f65_4cd4_8890_4694843c276d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileInputNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioFileOutputNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50e01980_5166_4093_80f8_ada00089e9cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileOutputNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioFrameCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc7c829e_0208_4504_a5a8_f0f268920a65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioFrameInputNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b266c7_fd96_4ff5_a3c5_d27a9bf44237);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameInputNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frame: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioFrameOutputNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb847371b_3299_45f5_88b3_c9d12a3f1cc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameOutputNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraph(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraph {
    type Vtable = IAudioGraph_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ad46eed_e48c_4e14_9660_2c4f83e9cdd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, fileencodingprofile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraph2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraph2 {
    type Vtable = IAudioGraph2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e4c3bd5_4fc1_45f6_a947_3cd38f4fd839);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingproperties: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Capture::MediaCategory, encodingproperties: ::windows::core::RawPtr, device: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingproperties: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraph3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraph3 {
    type Vtable = IAudioGraph3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddcd25ae_1185_42a7_831d_6a9b0fc86820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mediasource: ::windows::core::RawPtr, emitter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraphConnection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraphConnection {
    type Vtable = IAudioGraphConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x763070ed_d04e_4fac_b233_600b42edd469);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraphSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraphSettings {
    type Vtable = IAudioGraphSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d59647f_e6fe_4628_84f8_9d8bdba25785);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut QuantumSizeSelectionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: QuantumSizeSelectionMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Render::AudioRenderCategory) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::Render::AudioRenderCategory) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::AudioProcessing) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraphSettings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraphSettings2 {
    type Vtable = IAudioGraphSettings2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72919787_4dab_46e3_b4c9_d8e1a2636062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraphSettingsFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraphSettingsFactory {
    type Vtable = IAudioGraphSettingsFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5d91cc6_c2eb_4a61_a214_1d66d75f83da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettingsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiorendercategory: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraphStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraphStatics {
    type Vtable = IAudioGraphStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76ec3132_e159_4ab7_a82a_17beb4b31e94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3d9cbe0_3ff6_4fb3_b262_50d435c55423);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioGraphUnrecoverableError) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioInputNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioInputNode {
    type Vtable = IAudioInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd148005c_8428_4784_b7fd_a99d468c5d20);
}
impl IAudioInputNode {
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d148005c-8428-4784-b7fd-a99d468c5d20}");
}
impl ::core::convert::From<IAudioInputNode> for ::windows::core::IUnknown {
    fn from(value: IAudioInputNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAudioInputNode> for ::windows::core::IUnknown {
    fn from(value: &IAudioInputNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAudioInputNode> for ::windows::core::IInspectable {
    fn from(value: IAudioInputNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioInputNode> for ::windows::core::IInspectable {
    fn from(value: &IAudioInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, gain: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioInputNode2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioInputNode2 {
    type Vtable = IAudioInputNode2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905156b7_ca68_4c6d_a8bc_e3ee17fe3fd2);
}
impl IAudioInputNode2 {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioInputNode2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905156b7-ca68-4c6d-a8bc-e3ee17fe3fd2}");
}
impl ::core::convert::From<IAudioInputNode2> for ::windows::core::IUnknown {
    fn from(value: IAudioInputNode2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAudioInputNode2> for ::windows::core::IUnknown {
    fn from(value: &IAudioInputNode2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAudioInputNode2> for ::windows::core::IInspectable {
    fn from(value: IAudioInputNode2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioInputNode2> for ::windows::core::IInspectable {
    fn from(value: &IAudioInputNode2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioInputNode2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNode {
    type Vtable = IAudioNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15389d7f_dbd8_4819_bf03_668e9357cd6d);
}
impl IAudioNode {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{15389d7f-dbd8-4819-bf03-668e9357cd6d}");
}
impl ::core::convert::From<IAudioNode> for ::windows::core::IUnknown {
    fn from(value: IAudioNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAudioNode> for ::windows::core::IUnknown {
    fn from(value: &IAudioNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAudioNode> for ::windows::core::IInspectable {
    fn from(value: IAudioNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioNode> for ::windows::core::IInspectable {
    fn from(value: &IAudioNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Effects")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, definition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))] usize,
    #[cfg(feature = "Media_Effects")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, definition: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3676971d_880a_47b8_adf7_1323a9d965be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitter2 {
    type Vtable = IAudioNodeEmitter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ab6eecb_ec29_47f8_818c_b6b660a5aeb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SpatialAudioModel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: SpatialAudioModel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitterConeProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe99b2cee_02ca_4375_9326_0c6ae4bcdfb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterConeProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModel(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d1d5af7_0d53_4fa9_bd84_d5816a86f3ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioNodeEmitterDecayKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModelStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterDecayModelStatics {
    type Vtable = IAudioNodeEmitterDecayModelStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7787ca8_f178_462f_bc81_8dd5cbe5dae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModelStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mingain: f64, maxgain: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitterFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterFactory {
    type Vtable = IAudioNodeEmitterFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdc8489a_6ad6_4ce4_b7f7_a99370df7ee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shape: ::windows::core::RawPtr, decaymodel: ::windows::core::RawPtr, settings: AudioNodeEmitterSettings, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48934bcf_cf2c_4efc_9331_75bd22df1f0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShape(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea0311c5_e73d_44bc_859c_45553bbc4828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShape_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioNodeEmitterShapeKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShapeStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterShapeStatics {
    type Vtable = IAudioNodeEmitterShapeStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57bb2771_ffa5_4b86_a779_e264aeb9145f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShapeStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, innerangle: f64, outerangle: f64, outeranglegain: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioNodeListener(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeListener {
    type Vtable = IAudioNodeListener_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9722e16_0c0a_41da_b755_6c77835fb1eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeListener_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioNodeWithListener(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioNodeWithListener {
    type Vtable = IAudioNodeWithListener_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e0f907c_79ff_4544_9eeb_01257b15105a);
}
impl IAudioNodeWithListener {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    pub fn SetListener<'a, Param0: ::windows::core::IntoParam<'a, AudioNodeListener>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Listener(&self) -> ::windows::core::Result<AudioNodeListener> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeListener>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAudioNodeWithListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0e0f907c-79ff-4544-9eeb-01257b15105a}");
}
impl ::core::convert::From<IAudioNodeWithListener> for ::windows::core::IUnknown {
    fn from(value: IAudioNodeWithListener) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAudioNodeWithListener> for ::windows::core::IUnknown {
    fn from(value: &IAudioNodeWithListener) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAudioNodeWithListener> for ::windows::core::IInspectable {
    fn from(value: IAudioNodeWithListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioNodeWithListener> for ::windows::core::IInspectable {
    fn from(value: &IAudioNodeWithListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAudioNodeWithListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeWithListener_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioPlaybackConnection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a4c1dea_cafc_50e7_8718_ea3f81cbfa51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioPlaybackConnectionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionOpenResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e656aef_39f9_5fc9_a519_a5bbfd9fe921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionOpenResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioPlaybackConnectionOpenResultStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnectionStatics {
    type Vtable = IAudioPlaybackConnectionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe60963a2_69e6_5ffc_9e13_824a85213daf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioStateMonitor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d13d136_0199_4cdc_b84e_e72c2b581ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::SoundLevel) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAudioStateMonitorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAudioStateMonitorStatics {
    type Vtable = IAudioStateMonitorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6374ea4c_1b3b_4001_94d9_dd225330fa40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Render::AudioRenderCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices", feature = "Media_Render")))] usize,
    #[cfg(feature = "Media_Render")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Render::AudioRenderCategory, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Capture::MediaCategory, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Media_Capture")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: super::Capture::MediaCategory, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16eec7a8_1ca7_40ef_91a4_d346e0aa1bba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceInputNodeResult2 {
    type Vtable = ICreateAudioDeviceInputNodeResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x921c69ce_3f35_41c7_9622_79f608baedc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7776d27_1d9a_47f7_9cd4_2859cc1b7bff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceOutputNodeResult2 {
    type Vtable = ICreateAudioDeviceOutputNodeResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4864269f_bdce_4ab1_bd38_fbae93aedaca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce83d61c_e297_4c50_9ce7_1c7a69d6bd09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioFileNodeCreationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioFileInputNodeResult2 {
    type Vtable = ICreateAudioFileInputNodeResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9082020_3d80_4fe0_81c1_768fea7ca7e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47d6ba7b_e909_453f_866e_5540cda734ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioFileNodeCreationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioFileOutputNodeResult2 {
    type Vtable = ICreateAudioFileOutputNodeResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f01b50d_3318_47b3_a60a_1b492be7fc0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5453ef7e_7bde_4b76_bb5d_48f79cfc8c0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AudioGraphCreationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateAudioGraphResult2 {
    type Vtable = ICreateAudioGraphResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d738dfc_88c6_4fcb_a534_85cedd4050a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a658a3_53c0_4d59_9e51_cc1d1044a4c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MediaSourceAudioInputNodeCreationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICreateMediaSourceAudioInputNodeResult2 {
    type Vtable = ICreateMediaSourceAudioInputNodeResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63514ce8_6a1a_49e3_97ec_28fd5be114e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEchoEffectDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e4d3faa_36b8_4c91_b9da_11f44a8a6610);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEchoEffectDefinitionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEchoEffectDefinitionFactory {
    type Vtable = IEchoEffectDefinitionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d4e2257_aaf2_4e86_a54c_fb79db8f6c12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEqualizerBand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEqualizerBand {
    type Vtable = IEqualizerBand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00a5a6a_262d_4b85_9bb7_43280b62ed0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerBand_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023f6f1f_83fe_449a_a822_c696442d16b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinitionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEqualizerEffectDefinitionFactory {
    type Vtable = IEqualizerEffectDefinitionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2876fc4_d410_4eb5_9e69_c9aa1277eaf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFrameInputNodeQuantumStartedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d9bd498_a306_4f06_bd9f_e9efc8226304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameInputNodeQuantumStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILimiterEffectDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b755d19_2603_47ba_bdeb_39055e3486dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILimiterEffectDefinitionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILimiterEffectDefinitionFactory {
    type Vtable = ILimiterEffectDefinitionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecbae6f1_61ff_45ef_b8f5_48659a57c72d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaSourceAudioInputNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99d8983b_a88a_4041_8e4f_ddbac0c91fd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAudioInputNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IReverbEffectDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4606aa89_f563_4d0a_8f6e_f0cddff35d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IReverbEffectDefinitionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IReverbEffectDefinitionFactory {
    type Vtable = IReverbEffectDefinitionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7d5cbfe_100b_4ff0_9da6_dc4e05a759f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinitionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiograph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISetDefaultSpatialAudioFormatResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2aa511_1400_5e70_9ea9_ae151241e8ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetDefaultSpatialAudioFormatResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SetDefaultSpatialAudioFormatStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee830034_61cf_5749_9da4_10f0fe028199);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfigurationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAudioDeviceConfigurationStatics {
    type Vtable = ISpatialAudioDeviceConfigurationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ec37f7b_936d_4e04_9728_2827d9f758c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfigurationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32df09a8_50f0_5395_9923_7d44ca71ed6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfigurationStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatConfigurationStatics {
    type Vtable = ISpatialAudioFormatConfigurationStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b5fef71_67c9_4e5f_a35b_41680711f8c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfigurationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatSubtypeStatics {
    type Vtable = ISpatialAudioFormatSubtypeStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3de8a47_83ee_4266_a945_bedf507afeed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatSubtypeStatics2 {
    type Vtable = ISpatialAudioFormatSubtypeStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4565e6cb_d95b_5621_b6af_0e8849c57c80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LimiterEffectDefinition(pub ::windows::core::IInspectable);
impl LimiterEffectDefinition {
    pub fn SetRelease(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Release(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetLoudness(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Loudness(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows::core::Result<LimiterEffectDefinition> {
        Self::ILimiterEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), audiograph.into_param().abi(), &mut result__).from_abi::<LimiterEffectDefinition>(result__)
        })
    }
    pub fn ILimiterEffectDefinitionFactory<R, F: FnOnce(&ILimiterEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LimiterEffectDefinition, ILimiterEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LimiterEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.LimiterEffectDefinition;{6b755d19-2603-47ba-bdeb-39055e3486dc})");
}
unsafe impl ::windows::core::Interface for LimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b755d19_2603_47ba_bdeb_39055e3486dc);
}
impl ::windows::core::RuntimeName for LimiterEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.LimiterEffectDefinition";
}
impl ::core::convert::From<LimiterEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: LimiterEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LimiterEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &LimiterEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LimiterEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: LimiterEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LimiterEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &LimiterEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LimiterEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaSourceAudioInputNode(pub ::windows::core::IInspectable);
impl MediaSourceAudioInputNode {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(result__)
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0, gain: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), destination.into_param().abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::core::IntoParam<'a, IAudioNode>>(&self, destination: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), destination.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), definition.into_param().abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AudioNodeEmitter>(result__)
        }
    }
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Seek<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, position: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLoopCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::MediaSource>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MediaSourceCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaSourceAudioInputNode, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaSourceCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaSourceAudioInputNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.MediaSourceAudioInputNode;{99d8983b-a88a-4041-8e4f-ddbac0c91fd3})");
}
unsafe impl ::windows::core::Interface for MediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99d8983b_a88a_4041_8e4f_ddbac0c91fd3);
}
impl ::windows::core::RuntimeName for MediaSourceAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.MediaSourceAudioInputNode";
}
impl ::core::convert::From<MediaSourceAudioInputNode> for ::windows::core::IUnknown {
    fn from(value: MediaSourceAudioInputNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaSourceAudioInputNode> for ::windows::core::IUnknown {
    fn from(value: &MediaSourceAudioInputNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaSourceAudioInputNode> for ::windows::core::IInspectable {
    fn from(value: MediaSourceAudioInputNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaSourceAudioInputNode> for ::windows::core::IInspectable {
    fn from(value: &MediaSourceAudioInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
unsafe impl ::core::marker::Send for MediaSourceAudioInputNode {}
unsafe impl ::core::marker::Sync for MediaSourceAudioInputNode {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: MediaSourceAudioInputNodeCreationStatus = MediaSourceAudioInputNodeCreationStatus(0i32);
    pub const FormatNotSupported: MediaSourceAudioInputNodeCreationStatus = MediaSourceAudioInputNodeCreationStatus(1i32);
    pub const NetworkError: MediaSourceAudioInputNodeCreationStatus = MediaSourceAudioInputNodeCreationStatus(2i32);
    pub const UnknownFailure: MediaSourceAudioInputNodeCreationStatus = MediaSourceAudioInputNodeCreationStatus(3i32);
}
impl ::core::convert::From<i32> for MediaSourceAudioInputNodeCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaSourceAudioInputNodeCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaSourceAudioInputNodeCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MediaSourceAudioInputNodeCreationStatus;i4)");
}
impl ::windows::core::DefaultType for MediaSourceAudioInputNodeCreationStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: MixedRealitySpatialAudioFormatPolicy = MixedRealitySpatialAudioFormatPolicy(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: MixedRealitySpatialAudioFormatPolicy = MixedRealitySpatialAudioFormatPolicy(1i32);
}
impl ::core::convert::From<i32> for MixedRealitySpatialAudioFormatPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MixedRealitySpatialAudioFormatPolicy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MixedRealitySpatialAudioFormatPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MixedRealitySpatialAudioFormatPolicy;i4)");
}
impl ::windows::core::DefaultType for MixedRealitySpatialAudioFormatPolicy {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: QuantumSizeSelectionMode = QuantumSizeSelectionMode(0i32);
    pub const LowestLatency: QuantumSizeSelectionMode = QuantumSizeSelectionMode(1i32);
    pub const ClosestToDesired: QuantumSizeSelectionMode = QuantumSizeSelectionMode(2i32);
}
impl ::core::convert::From<i32> for QuantumSizeSelectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for QuantumSizeSelectionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for QuantumSizeSelectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.QuantumSizeSelectionMode;i4)");
}
impl ::windows::core::DefaultType for QuantumSizeSelectionMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ReverbEffectDefinition(pub ::windows::core::IInspectable);
impl ReverbEffectDefinition {
    pub fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn WetDryMix(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetReflectionsDelay(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReflectionsDelay(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReverbDelay(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReverbDelay(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetRearDelay(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RearDelay(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionLeft(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PositionLeft(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionRight(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PositionRight(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionMatrixLeft(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PositionMatrixLeft(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionMatrixRight(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PositionMatrixRight(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetEarlyDiffusion(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn EarlyDiffusion(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetLateDiffusion(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LateDiffusion(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetLowEQGain(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LowEQGain(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetLowEQCutoff(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LowEQCutoff(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetHighEQGain(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HighEQGain(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetHighEQCutoff(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HighEQCutoff(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    pub fn SetRoomFilterFreq(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RoomFilterFreq(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomFilterMain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RoomFilterMain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomFilterHF(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RoomFilterHF(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetReflectionsGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReflectionsGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetReverbGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReverbGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDecayTime(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DecayTime(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDensity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Density(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RoomSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDisableLateField(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DisableLateField(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, AudioGraph>>(audiograph: Param0) -> ::windows::core::Result<ReverbEffectDefinition> {
        Self::IReverbEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), audiograph.into_param().abi(), &mut result__).from_abi::<ReverbEffectDefinition>(result__)
        })
    }
    pub fn IReverbEffectDefinitionFactory<R, F: FnOnce(&IReverbEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ReverbEffectDefinition, IReverbEffectDefinitionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ReverbEffectDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.ReverbEffectDefinition;{4606aa89-f563-4d0a-8f6e-f0cddff35d84})");
}
unsafe impl ::windows::core::Interface for ReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4606aa89_f563_4d0a_8f6e_f0cddff35d84);
}
impl ::windows::core::RuntimeName for ReverbEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.ReverbEffectDefinition";
}
impl ::core::convert::From<ReverbEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: ReverbEffectDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ReverbEffectDefinition> for ::windows::core::IUnknown {
    fn from(value: &ReverbEffectDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ReverbEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: ReverbEffectDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ReverbEffectDefinition> for ::windows::core::IInspectable {
    fn from(value: &ReverbEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReverbEffectDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SetDefaultSpatialAudioFormatResult(pub ::windows::core::IInspectable);
impl SetDefaultSpatialAudioFormatResult {
    pub fn Status(&self) -> ::windows::core::Result<SetDefaultSpatialAudioFormatStatus> {
        let this = self;
        unsafe {
            let mut result__: SetDefaultSpatialAudioFormatStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SetDefaultSpatialAudioFormatStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SetDefaultSpatialAudioFormatResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SetDefaultSpatialAudioFormatResult;{1c2aa511-1400-5e70-9ea9-ae151241e8ea})");
}
unsafe impl ::windows::core::Interface for SetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2aa511_1400_5e70_9ea9_ae151241e8ea);
}
impl ::windows::core::RuntimeName for SetDefaultSpatialAudioFormatResult {
    const NAME: &'static str = "Windows.Media.Audio.SetDefaultSpatialAudioFormatResult";
}
impl ::core::convert::From<SetDefaultSpatialAudioFormatResult> for ::windows::core::IUnknown {
    fn from(value: SetDefaultSpatialAudioFormatResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SetDefaultSpatialAudioFormatResult> for ::windows::core::IUnknown {
    fn from(value: &SetDefaultSpatialAudioFormatResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SetDefaultSpatialAudioFormatResult> for ::windows::core::IInspectable {
    fn from(value: SetDefaultSpatialAudioFormatResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SetDefaultSpatialAudioFormatResult> for ::windows::core::IInspectable {
    fn from(value: &SetDefaultSpatialAudioFormatResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SetDefaultSpatialAudioFormatResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SetDefaultSpatialAudioFormatResult {}
unsafe impl ::core::marker::Sync for SetDefaultSpatialAudioFormatResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(0i32);
    pub const AccessDenied: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(1i32);
    pub const LicenseExpired: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(2i32);
    pub const LicenseNotValidForAudioEndpoint: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(3i32);
    pub const NotSupportedOnAudioEndpoint: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(4i32);
    pub const UnknownError: SetDefaultSpatialAudioFormatStatus = SetDefaultSpatialAudioFormatStatus(5i32);
}
impl ::core::convert::From<i32> for SetDefaultSpatialAudioFormatStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SetDefaultSpatialAudioFormatStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SetDefaultSpatialAudioFormatStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SetDefaultSpatialAudioFormatStatus;i4)");
}
impl ::windows::core::DefaultType for SetDefaultSpatialAudioFormatStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialAudioDeviceConfiguration(pub ::windows::core::IInspectable);
impl SpatialAudioDeviceConfiguration {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsSpatialAudioSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsSpatialAudioFormatSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, subtype: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ActiveSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DefaultSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultSpatialAudioFormatAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, subtype: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConfigurationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetForDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<SpatialAudioDeviceConfiguration> {
        Self::ISpatialAudioDeviceConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<SpatialAudioDeviceConfiguration>(result__)
        })
    }
    pub fn ISpatialAudioDeviceConfigurationStatics<R, F: FnOnce(&ISpatialAudioDeviceConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAudioDeviceConfiguration, ISpatialAudioDeviceConfigurationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAudioDeviceConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SpatialAudioDeviceConfiguration;{ee830034-61cf-5749-9da4-10f0fe028199})");
}
unsafe impl ::windows::core::Interface for SpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee830034_61cf_5749_9da4_10f0fe028199);
}
impl ::windows::core::RuntimeName for SpatialAudioDeviceConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioDeviceConfiguration";
}
impl ::core::convert::From<SpatialAudioDeviceConfiguration> for ::windows::core::IUnknown {
    fn from(value: SpatialAudioDeviceConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialAudioDeviceConfiguration> for ::windows::core::IUnknown {
    fn from(value: &SpatialAudioDeviceConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialAudioDeviceConfiguration> for ::windows::core::IInspectable {
    fn from(value: SpatialAudioDeviceConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialAudioDeviceConfiguration> for ::windows::core::IInspectable {
    fn from(value: &SpatialAudioDeviceConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAudioDeviceConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialAudioDeviceConfiguration {}
unsafe impl ::core::marker::Sync for SpatialAudioDeviceConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SpatialAudioFormatConfiguration(pub ::windows::core::IInspectable);
impl SpatialAudioFormatConfiguration {
    #[cfg(feature = "Foundation")]
    pub fn ReportLicenseChangedAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, subtype: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportConfigurationChangedAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, subtype: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn MixedRealityExclusiveModePolicy(&self) -> ::windows::core::Result<MixedRealitySpatialAudioFormatPolicy> {
        let this = self;
        unsafe {
            let mut result__: MixedRealitySpatialAudioFormatPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MixedRealitySpatialAudioFormatPolicy>(result__)
        }
    }
    pub fn SetMixedRealityExclusiveModePolicy(&self, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<SpatialAudioFormatConfiguration> {
        Self::ISpatialAudioFormatConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SpatialAudioFormatConfiguration>(result__)
        })
    }
    pub fn ISpatialAudioFormatConfigurationStatics<R, F: FnOnce(&ISpatialAudioFormatConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAudioFormatConfiguration, ISpatialAudioFormatConfigurationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialAudioFormatConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SpatialAudioFormatConfiguration;{32df09a8-50f0-5395-9923-7d44ca71ed6d})");
}
unsafe impl ::windows::core::Interface for SpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32df09a8_50f0_5395_9923_7d44ca71ed6d);
}
impl ::windows::core::RuntimeName for SpatialAudioFormatConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatConfiguration";
}
impl ::core::convert::From<SpatialAudioFormatConfiguration> for ::windows::core::IUnknown {
    fn from(value: SpatialAudioFormatConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SpatialAudioFormatConfiguration> for ::windows::core::IUnknown {
    fn from(value: &SpatialAudioFormatConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SpatialAudioFormatConfiguration> for ::windows::core::IInspectable {
    fn from(value: SpatialAudioFormatConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SpatialAudioFormatConfiguration> for ::windows::core::IInspectable {
    fn from(value: &SpatialAudioFormatConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialAudioFormatConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for SpatialAudioFormatConfiguration {}
unsafe impl ::core::marker::Sync for SpatialAudioFormatConfiguration {}
pub struct SpatialAudioFormatSubtype {}
impl SpatialAudioFormatSubtype {
    pub fn WindowsSonic() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForHeadphones() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForHomeTheater() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForSpeakers() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DTSHeadphoneX() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DTSXUltra() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn DTSXForHomeTheater() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ISpatialAudioFormatSubtypeStatics<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpatialAudioFormatSubtypeStatics2<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SpatialAudioFormatSubtype {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatSubtype";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: SpatialAudioModel = SpatialAudioModel(0i32);
    pub const FoldDown: SpatialAudioModel = SpatialAudioModel(1i32);
}
impl ::core::convert::From<i32> for SpatialAudioModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioModel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SpatialAudioModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SpatialAudioModel;i4)");
}
impl ::windows::core::DefaultType for SpatialAudioModel {
    type DefaultType = Self;
}
