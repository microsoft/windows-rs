#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioDeviceInputNode(::windows::runtime::IInspectable);
impl AudioDeviceInputNode {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<AudioGraphConnection>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(
                result__,
            )
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddOutgoingConnectionWithGain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IAudioNode>,
    >(
        &self,
        destination: Param0,
        gain: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
                gain,
            )
            .ok()
        }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Emitter(&self) -> ::windows::runtime::Result<AudioNodeEmitter> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioDeviceInputNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioDeviceInputNode;{b01b6be1-6f4e-49e2-ac01-559d62beb3a9})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2954587105,
        28494,
        18914,
        [172, 1, 85, 157, 98, 190, 179, 169],
    );
}
impl ::windows::runtime::RuntimeName for AudioDeviceInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceInputNode";
}
impl ::std::convert::From<AudioDeviceInputNode> for ::windows::runtime::IUnknown {
    fn from(value: AudioDeviceInputNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioDeviceInputNode> for ::windows::runtime::IUnknown {
    fn from(value: &AudioDeviceInputNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioDeviceInputNode> for ::windows::runtime::IInspectable {
    fn from(value: AudioDeviceInputNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioDeviceInputNode> for ::windows::runtime::IInspectable {
    fn from(value: &AudioDeviceInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioDeviceInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioDeviceInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioDeviceInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioDeviceInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioDeviceInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioDeviceInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioDeviceInputNode> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioDeviceInputNode> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::std::convert::TryInto::<IAudioInputNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioDeviceInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioDeviceInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioDeviceInputNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioDeviceInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioDeviceInputNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioDeviceInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for AudioDeviceInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for &AudioDeviceInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::std::convert::TryInto::<IAudioInputNode2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioDeviceInputNode {}
unsafe impl ::std::marker::Sync for AudioDeviceInputNode {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(0i32);
    pub const DeviceNotAvailable: AudioDeviceNodeCreationStatus =
        AudioDeviceNodeCreationStatus(1i32);
    pub const FormatNotSupported: AudioDeviceNodeCreationStatus =
        AudioDeviceNodeCreationStatus(2i32);
    pub const UnknownFailure: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(3i32);
    pub const AccessDenied: AudioDeviceNodeCreationStatus = AudioDeviceNodeCreationStatus(4i32);
}
impl ::std::convert::From<i32> for AudioDeviceNodeCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioDeviceNodeCreationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioDeviceNodeCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioDeviceNodeCreationStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioDeviceOutputNode(::windows::runtime::IInspectable);
impl AudioDeviceOutputNode {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetListener<'a, Param0: ::windows::runtime::IntoParam<'a, AudioNodeListener>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Listener(&self) -> ::windows::runtime::Result<AudioNodeListener> {
        let this = &::windows::runtime::Interface::cast::<IAudioNodeWithListener>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeListener>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioDeviceOutputNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioDeviceOutputNode;{362edbff-ff1c-4434-9e0f-bd2ef522ac82})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        909040639,
        65308,
        17460,
        [158, 15, 189, 46, 245, 34, 172, 130],
    );
}
impl ::windows::runtime::RuntimeName for AudioDeviceOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceOutputNode";
}
impl ::std::convert::From<AudioDeviceOutputNode> for ::windows::runtime::IUnknown {
    fn from(value: AudioDeviceOutputNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioDeviceOutputNode> for ::windows::runtime::IUnknown {
    fn from(value: &AudioDeviceOutputNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioDeviceOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioDeviceOutputNode> for ::windows::runtime::IInspectable {
    fn from(value: AudioDeviceOutputNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioDeviceOutputNode> for ::windows::runtime::IInspectable {
    fn from(value: &AudioDeviceOutputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioDeviceOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioDeviceOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioDeviceOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioDeviceOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioDeviceOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioDeviceOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioDeviceOutputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioDeviceOutputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioDeviceOutputNode> for IAudioNodeWithListener {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioDeviceOutputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioDeviceOutputNode> for IAudioNodeWithListener {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioDeviceOutputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNodeWithListener> for AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNodeWithListener> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNodeWithListener> for &AudioDeviceOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNodeWithListener> {
        ::std::convert::TryInto::<IAudioNodeWithListener>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioDeviceOutputNode {}
unsafe impl ::std::marker::Sync for AudioDeviceOutputNode {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioFileInputNode(::windows::runtime::IInspectable);
impl AudioFileInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Seek<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
        &self,
        position: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                position.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoopCount(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLoopCount<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SourceFile(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FileCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                AudioFileInputNode,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<AudioGraphConnection>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(
                result__,
            )
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddOutgoingConnectionWithGain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IAudioNode>,
    >(
        &self,
        destination: Param0,
        gain: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
                gain,
            )
            .ok()
        }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Emitter(&self) -> ::windows::runtime::Result<AudioNodeEmitter> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioFileInputNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioFileInputNode;{905b67c8-6f65-4cd4-8890-4694843c276d})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioFileInputNode {
    type Vtable = IAudioFileInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2421909448,
        28517,
        19668,
        [136, 144, 70, 148, 132, 60, 39, 109],
    );
}
impl ::windows::runtime::RuntimeName for AudioFileInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileInputNode";
}
impl ::std::convert::From<AudioFileInputNode> for ::windows::runtime::IUnknown {
    fn from(value: AudioFileInputNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioFileInputNode> for ::windows::runtime::IUnknown {
    fn from(value: &AudioFileInputNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioFileInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioFileInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioFileInputNode> for ::windows::runtime::IInspectable {
    fn from(value: AudioFileInputNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioFileInputNode> for ::windows::runtime::IInspectable {
    fn from(value: &AudioFileInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioFileInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioFileInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioFileInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioFileInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioFileInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioFileInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioFileInputNode> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioFileInputNode> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for AudioFileInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for &AudioFileInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::std::convert::TryInto::<IAudioInputNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioFileInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioFileInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for AudioFileInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &AudioFileInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioFileInputNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFileInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioFileInputNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFileInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for AudioFileInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for &AudioFileInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::std::convert::TryInto::<IAudioInputNode2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioFileInputNode {}
unsafe impl ::std::marker::Sync for AudioFileInputNode {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(0i32);
    pub const FileNotFound: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(1i32);
    pub const InvalidFileType: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(2i32);
    pub const FormatNotSupported: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(3i32);
    pub const UnknownFailure: AudioFileNodeCreationStatus = AudioFileNodeCreationStatus(4i32);
}
impl ::std::convert::From<i32> for AudioFileNodeCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioFileNodeCreationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioFileNodeCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioFileNodeCreationStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioFileOutputNode(::windows::runtime::IInspectable);
impl AudioFileOutputNode {
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FileEncodingProfile(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::MediaEncodingProfile>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))]
    pub fn FinalizeAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::Transcoding::TranscodeFailureReason,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioFileOutputNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioFileOutputNode;{50e01980-5166-4093-80f8-ada00089e9cf})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1356863872,
        20838,
        16531,
        [128, 248, 173, 160, 0, 137, 233, 207],
    );
}
impl ::windows::runtime::RuntimeName for AudioFileOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileOutputNode";
}
impl ::std::convert::From<AudioFileOutputNode> for ::windows::runtime::IUnknown {
    fn from(value: AudioFileOutputNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioFileOutputNode> for ::windows::runtime::IUnknown {
    fn from(value: &AudioFileOutputNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioFileOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioFileOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioFileOutputNode> for ::windows::runtime::IInspectable {
    fn from(value: AudioFileOutputNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioFileOutputNode> for ::windows::runtime::IInspectable {
    fn from(value: &AudioFileOutputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioFileOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioFileOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioFileOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFileOutputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioFileOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFileOutputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioFileOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioFileOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioFileOutputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFileOutputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioFileOutputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFileOutputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for AudioFileOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &AudioFileOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioFileOutputNode {}
unsafe impl ::std::marker::Sync for AudioFileOutputNode {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioFrameCompletedEventArgs(::windows::runtime::IInspectable);
impl AudioFrameCompletedEventArgs {
    pub fn Frame(&self) -> ::windows::runtime::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::AudioFrame>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioFrameCompletedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioFrameCompletedEventArgs;{dc7c829e-0208-4504-a5a8-f0f268920a65})" ) ;
}
unsafe impl ::windows::runtime::Interface for AudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3699147422,
        520,
        17668,
        [165, 168, 240, 242, 104, 146, 10, 101],
    );
}
impl ::windows::runtime::RuntimeName for AudioFrameCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameCompletedEventArgs";
}
impl ::std::convert::From<AudioFrameCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AudioFrameCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioFrameCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AudioFrameCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AudioFrameCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioFrameCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioFrameCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AudioFrameCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioFrameCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AudioFrameCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioFrameCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioFrameCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioFrameCompletedEventArgs {}
unsafe impl ::std::marker::Sync for AudioFrameCompletedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioFrameInputNode(::windows::runtime::IInspectable);
impl AudioFrameInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn AddFrame<'a, Param0: ::windows::runtime::IntoParam<'a, super::AudioFrame>>(
        &self,
        frame: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                frame.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn QueuedSampleCount(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AudioFrameCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                AudioFrameInputNode,
                AudioFrameCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioFrameCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn QuantumStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                AudioFrameInputNode,
                FrameInputNodeQuantumStartedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<AudioGraphConnection>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(
                result__,
            )
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddOutgoingConnectionWithGain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IAudioNode>,
    >(
        &self,
        destination: Param0,
        gain: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
                gain,
            )
            .ok()
        }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Emitter(&self) -> ::windows::runtime::Result<AudioNodeEmitter> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioFrameInputNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioFrameInputNode;{01b266c7-fd96-4ff5-a3c5-d27a9bf44237})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        28468935,
        64918,
        20469,
        [163, 197, 210, 122, 155, 244, 66, 55],
    );
}
impl ::windows::runtime::RuntimeName for AudioFrameInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameInputNode";
}
impl ::std::convert::From<AudioFrameInputNode> for ::windows::runtime::IUnknown {
    fn from(value: AudioFrameInputNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioFrameInputNode> for ::windows::runtime::IUnknown {
    fn from(value: &AudioFrameInputNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioFrameInputNode> for ::windows::runtime::IInspectable {
    fn from(value: AudioFrameInputNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioFrameInputNode> for ::windows::runtime::IInspectable {
    fn from(value: &AudioFrameInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioFrameInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioFrameInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioFrameInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioFrameInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioFrameInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioFrameInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioFrameInputNode> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioFrameInputNode> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::std::convert::TryInto::<IAudioInputNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioFrameInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioFrameInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioFrameInputNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFrameInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioFrameInputNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFrameInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for AudioFrameInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for &AudioFrameInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::std::convert::TryInto::<IAudioInputNode2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioFrameInputNode {}
unsafe impl ::std::marker::Sync for AudioFrameInputNode {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioFrameOutputNode(::windows::runtime::IInspectable);
impl AudioFrameOutputNode {
    pub fn GetFrame(&self) -> ::windows::runtime::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::AudioFrame>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioFrameOutputNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioFrameOutputNode;{b847371b-3299-45f5-88b3-c9d12a3f1cc8})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3091674907,
        12953,
        17909,
        [136, 179, 201, 209, 42, 63, 28, 200],
    );
}
impl ::windows::runtime::RuntimeName for AudioFrameOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameOutputNode";
}
impl ::std::convert::From<AudioFrameOutputNode> for ::windows::runtime::IUnknown {
    fn from(value: AudioFrameOutputNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioFrameOutputNode> for ::windows::runtime::IUnknown {
    fn from(value: &AudioFrameOutputNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioFrameOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioFrameOutputNode> for ::windows::runtime::IInspectable {
    fn from(value: AudioFrameOutputNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioFrameOutputNode> for ::windows::runtime::IInspectable {
    fn from(value: &AudioFrameOutputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioFrameOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioFrameOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioFrameOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFrameOutputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioFrameOutputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFrameOutputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioFrameOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioFrameOutputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioFrameOutputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioFrameOutputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioFrameOutputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioFrameOutputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for AudioFrameOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &AudioFrameOutputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioFrameOutputNode {}
unsafe impl ::std::marker::Sync for AudioFrameOutputNode {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioGraph(::windows::runtime::IInspectable);
impl AudioGraph {
    pub fn CreateFrameInputNode(&self) -> ::windows::runtime::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
    >(
        &self,
        encodingproperties: Param0,
    ) -> ::windows::runtime::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                encodingproperties.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))]
    pub fn CreateDeviceInputNodeAsync(
        &self,
        category: super::Capture::MediaCategory,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , category , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < CreateAudioDeviceInputNodeResult > > ( result__ )
        }
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    ))]
    pub fn CreateDeviceInputNodeWithFormatAsync<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
    >(
        &self,
        category: super::Capture::MediaCategory,
        encodingproperties: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .9 ) ( :: std :: mem :: transmute_copy ( this ) , category , encodingproperties . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < CreateAudioDeviceInputNodeResult > > ( result__ )
        }
    }
    #[cfg(all(
        feature = "Devices_Enumeration",
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    ))]
    pub fn CreateDeviceInputNodeWithFormatOnDeviceAsync<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>,
    >(
        &self,
        category: super::Capture::MediaCategory,
        encodingproperties: Param1,
        device: Param2,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .10 ) ( :: std :: mem :: transmute_copy ( this ) , category , encodingproperties . into_param ( ) . abi ( ) , device . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < CreateAudioDeviceInputNodeResult > > ( result__ )
        }
    }
    pub fn CreateFrameOutputNode(&self) -> ::windows::runtime::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioFrameOutputNode>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameOutputNodeWithFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
    >(
        &self,
        encodingproperties: Param0,
    ) -> ::windows::runtime::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                encodingproperties.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AudioFrameOutputNode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateDeviceOutputNodeAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                CreateAudioDeviceOutputNodeResult,
            >>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileInputNodeAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
    >(
        &self,
        file: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                file.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>(
                result__,
            )
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileOutputNodeAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
    >(
        &self,
        file: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                file.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>(
                result__,
            )
        }
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Media_MediaProperties",
        feature = "Storage"
    ))]
    pub fn CreateFileOutputNodeWithFileProfileAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
        Param1: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>,
    >(
        &self,
        file: Param0,
        fileencodingprofile: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                file.into_param().abi(),
                fileencodingprofile.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>(
                result__,
            )
        }
    }
    pub fn CreateSubmixNode(&self) -> ::windows::runtime::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioSubmixNode>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormat<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
    >(
        &self,
        encodingproperties: Param0,
    ) -> ::windows::runtime::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                encodingproperties.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AudioSubmixNode>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn ResetAllNodes(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn QuantumStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                AudioGraph,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumStarted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn QuantumProcessed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                AudioGraph,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumProcessed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UnrecoverableErrorOccurred<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                AudioGraph,
                AudioGraphUnrecoverableErrorOccurredEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnrecoverableErrorOccurred<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CompletedQuantumCount(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn LatencyInSamples(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    pub fn RenderDeviceAudioProcessing(
        &self,
    ) -> ::windows::runtime::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__: super::AudioProcessing = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::AudioProcessing>(result__)
        }
    }
    pub fn SamplesPerQuantum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync<'a, Param0: ::windows::runtime::IntoParam<'a, AudioGraphSettings>>(
        settings: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>>
    {
        Self::IAudioGraphStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                settings.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>>(result__)
        })
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormatAndEmitter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
        Param1: ::windows::runtime::IntoParam<'a, AudioNodeEmitter>,
    >(
        &self,
        encodingproperties: Param0,
        emitter: Param1,
    ) -> ::windows::runtime::Result<AudioFrameInputNode> {
        let this = &::windows::runtime::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                encodingproperties.into_param().abi(),
                emitter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AudioFrameInputNode>(result__)
        }
    }
    #[cfg(all(
        feature = "Devices_Enumeration",
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    ))]
    pub fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>,
        Param3: ::windows::runtime::IntoParam<'a, AudioNodeEmitter>,
    >(
        &self,
        category: super::Capture::MediaCategory,
        encodingproperties: Param1,
        device: Param2,
        emitter: Param3,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .7 ) ( :: std :: mem :: transmute_copy ( this ) , category , encodingproperties . into_param ( ) . abi ( ) , device . into_param ( ) . abi ( ) , emitter . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < CreateAudioDeviceInputNodeResult > > ( result__ )
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileInputNodeWithEmitterAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
        Param1: ::windows::runtime::IntoParam<'a, AudioNodeEmitter>,
    >(
        &self,
        file: Param0,
        emitter: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                file.into_param().abi(),
                emitter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormatAndEmitter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
        Param1: ::windows::runtime::IntoParam<'a, AudioNodeEmitter>,
    >(
        &self,
        encodingproperties: Param0,
        emitter: Param1,
    ) -> ::windows::runtime::Result<AudioSubmixNode> {
        let this = &::windows::runtime::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                encodingproperties.into_param().abi(),
                emitter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AudioSubmixNode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateBatchUpdater(&self) -> ::windows::runtime::Result<AudioGraphBatchUpdater> {
        let this = &::windows::runtime::Interface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioGraphBatchUpdater>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateMediaSourceAudioInputNodeAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Core::MediaSource>,
    >(
        &self,
        mediasource: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                mediasource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                CreateMediaSourceAudioInputNodeResult,
            >>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateMediaSourceAudioInputNodeWithEmitterAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Core::MediaSource>,
        Param1: ::windows::runtime::IntoParam<'a, AudioNodeEmitter>,
    >(
        &self,
        mediasource: Param0,
        emitter: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                mediasource.into_param().abi(),
                emitter.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                CreateMediaSourceAudioInputNodeResult,
            >>(result__)
        }
    }
    pub fn IAudioGraphStatics<
        R,
        F: FnOnce(&IAudioGraphStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AudioGraph, IAudioGraphStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioGraph {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioGraph;{1ad46eed-e48c-4e14-9660-2c4f83e9cdd8})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioGraph {
    type Vtable = IAudioGraph_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        450129645,
        58508,
        19988,
        [150, 96, 44, 79, 131, 233, 205, 216],
    );
}
impl ::windows::runtime::RuntimeName for AudioGraph {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraph";
}
impl ::std::convert::From<AudioGraph> for ::windows::runtime::IUnknown {
    fn from(value: AudioGraph) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioGraph> for ::windows::runtime::IUnknown {
    fn from(value: &AudioGraph) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioGraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioGraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioGraph> for ::windows::runtime::IInspectable {
    fn from(value: AudioGraph) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioGraph> for ::windows::runtime::IInspectable {
    fn from(value: &AudioGraph) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioGraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioGraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioGraph> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioGraph) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioGraph> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioGraph) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for AudioGraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &AudioGraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioGraph {}
unsafe impl ::std::marker::Sync for AudioGraph {}
#[cfg(feature = "Foundation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioGraphBatchUpdater(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl AudioGraphBatchUpdater {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for AudioGraphBatchUpdater {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioGraphBatchUpdater;{30d5a829-7fa4-4026-83bb-d75bae4ea99e})",
    );
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for AudioGraphBatchUpdater {
    type Vtable = super::super::Foundation::IClosable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        819308585,
        32676,
        16422,
        [131, 187, 215, 91, 174, 78, 169, 158],
    );
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for AudioGraphBatchUpdater {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphBatchUpdater";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<AudioGraphBatchUpdater> for ::windows::runtime::IUnknown {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&AudioGraphBatchUpdater> for ::windows::runtime::IUnknown {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AudioGraphBatchUpdater
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioGraphBatchUpdater
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<AudioGraphBatchUpdater> for ::windows::runtime::IInspectable {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&AudioGraphBatchUpdater> for ::windows::runtime::IInspectable {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioGraphBatchUpdater
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioGraphBatchUpdater
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<AudioGraphBatchUpdater> for super::super::Foundation::IClosable {
    fn from(value: AudioGraphBatchUpdater) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&AudioGraphBatchUpdater> for super::super::Foundation::IClosable {
    fn from(value: &AudioGraphBatchUpdater) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioGraphBatchUpdater
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::Foundation::IClosable>::into(self),
        )
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioGraphBatchUpdater
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::Foundation::IClosable>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Send for AudioGraphBatchUpdater {}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Sync for AudioGraphBatchUpdater {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioGraphConnection(::windows::runtime::IInspectable);
impl AudioGraphConnection {
    pub fn Destination(&self) -> ::windows::runtime::Result<IAudioNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IAudioNode>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Gain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioGraphConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioGraphConnection;{763070ed-d04e-4fac-b233-600b42edd469})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioGraphConnection {
    type Vtable = IAudioGraphConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1982886125,
        53326,
        20396,
        [178, 51, 96, 11, 66, 237, 212, 105],
    );
}
impl ::windows::runtime::RuntimeName for AudioGraphConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphConnection";
}
impl ::std::convert::From<AudioGraphConnection> for ::windows::runtime::IUnknown {
    fn from(value: AudioGraphConnection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioGraphConnection> for ::windows::runtime::IUnknown {
    fn from(value: &AudioGraphConnection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioGraphConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioGraphConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioGraphConnection> for ::windows::runtime::IInspectable {
    fn from(value: AudioGraphConnection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioGraphConnection> for ::windows::runtime::IInspectable {
    fn from(value: &AudioGraphConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioGraphConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioGraphConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioGraphConnection {}
unsafe impl ::std::marker::Sync for AudioGraphConnection {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: AudioGraphCreationStatus = AudioGraphCreationStatus(0i32);
    pub const DeviceNotAvailable: AudioGraphCreationStatus = AudioGraphCreationStatus(1i32);
    pub const FormatNotSupported: AudioGraphCreationStatus = AudioGraphCreationStatus(2i32);
    pub const UnknownFailure: AudioGraphCreationStatus = AudioGraphCreationStatus(3i32);
}
impl ::std::convert::From<i32> for AudioGraphCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioGraphCreationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioGraphCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioGraphCreationStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioGraphSettings(::windows::runtime::IInspectable);
impl AudioGraphSettings {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::AudioEncodingProperties>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPrimaryRenderDevice<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn QuantumSizeSelectionMode(&self) -> ::windows::runtime::Result<QuantumSizeSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: QuantumSizeSelectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<QuantumSizeSelectionMode>(result__)
        }
    }
    pub fn SetQuantumSizeSelectionMode(
        &self,
        value: QuantumSizeSelectionMode,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn DesiredSamplesPerQuantum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn SetDesiredSamplesPerQuantum(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Render")]
    pub fn AudioRenderCategory(
        &self,
    ) -> ::windows::runtime::Result<super::Render::AudioRenderCategory> {
        let this = self;
        unsafe {
            let mut result__: super::Render::AudioRenderCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Render::AudioRenderCategory>(result__)
        }
    }
    #[cfg(feature = "Media_Render")]
    pub fn SetAudioRenderCategory(
        &self,
        value: super::Render::AudioRenderCategory,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn DesiredRenderDeviceAudioProcessing(
        &self,
    ) -> ::windows::runtime::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__: super::AudioProcessing = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::AudioProcessing>(result__)
        }
    }
    pub fn SetDesiredRenderDeviceAudioProcessing(
        &self,
        value: super::AudioProcessing,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Render")]
    pub fn Create(
        audiorendercategory: super::Render::AudioRenderCategory,
    ) -> ::windows::runtime::Result<AudioGraphSettings> {
        Self::IAudioGraphSettingsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                audiorendercategory,
                &mut result__,
            )
            .from_abi::<AudioGraphSettings>(result__)
        })
    }
    pub fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn MaxPlaybackSpeedFactor(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioGraphSettings2>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn IAudioGraphSettingsFactory<
        R,
        F: FnOnce(&IAudioGraphSettingsFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AudioGraphSettings,
            IAudioGraphSettingsFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioGraphSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioGraphSettings;{1d59647f-e6fe-4628-84f8-9d8bdba25785})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioGraphSettings {
    type Vtable = IAudioGraphSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        492397695,
        59134,
        17960,
        [132, 248, 157, 139, 219, 162, 87, 133],
    );
}
impl ::windows::runtime::RuntimeName for AudioGraphSettings {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphSettings";
}
impl ::std::convert::From<AudioGraphSettings> for ::windows::runtime::IUnknown {
    fn from(value: AudioGraphSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioGraphSettings> for ::windows::runtime::IUnknown {
    fn from(value: &AudioGraphSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioGraphSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioGraphSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioGraphSettings> for ::windows::runtime::IInspectable {
    fn from(value: AudioGraphSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioGraphSettings> for ::windows::runtime::IInspectable {
    fn from(value: &AudioGraphSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioGraphSettings
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioGraphSettings
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioGraphSettings {}
unsafe impl ::std::marker::Sync for AudioGraphSettings {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(0i32);
    pub const AudioDeviceLost: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(1i32);
    pub const AudioSessionDisconnected: AudioGraphUnrecoverableError =
        AudioGraphUnrecoverableError(2i32);
    pub const UnknownFailure: AudioGraphUnrecoverableError = AudioGraphUnrecoverableError(3i32);
}
impl ::std::convert::From<i32> for AudioGraphUnrecoverableError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioGraphUnrecoverableError {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioGraphUnrecoverableError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioGraphUnrecoverableError;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(::windows::runtime::IInspectable);
impl AudioGraphUnrecoverableErrorOccurredEventArgs {
    pub fn Error(&self) -> ::windows::runtime::Result<AudioGraphUnrecoverableError> {
        let this = self;
        unsafe {
            let mut result__: AudioGraphUnrecoverableError = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioGraphUnrecoverableError>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs;{c3d9cbe0-3ff6-4fb3-b262-50d435c55423})" ) ;
}
unsafe impl ::windows::runtime::Interface for AudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3285830624,
        16374,
        20403,
        [178, 98, 80, 212, 53, 197, 84, 35],
    );
}
impl ::windows::runtime::RuntimeName for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs";
}
impl ::std::convert::From<AudioGraphUnrecoverableErrorOccurredEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioGraphUnrecoverableErrorOccurredEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AudioGraphUnrecoverableErrorOccurredEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioGraphUnrecoverableErrorOccurredEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioGraphUnrecoverableErrorOccurredEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioGraphUnrecoverableErrorOccurredEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &AudioGraphUnrecoverableErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioGraphUnrecoverableErrorOccurredEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioGraphUnrecoverableErrorOccurredEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioGraphUnrecoverableErrorOccurredEventArgs {}
unsafe impl ::std::marker::Sync for AudioGraphUnrecoverableErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioNodeEmitter(::windows::runtime::IInspectable);
impl AudioNodeEmitter {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AudioNodeEmitter,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Direction(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDirection<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Shape(&self) -> ::windows::runtime::Result<AudioNodeEmitterShape> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterShape>(result__)
        }
    }
    pub fn DecayModel(&self) -> ::windows::runtime::Result<AudioNodeEmitterDecayModel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterDecayModel>(result__)
        }
    }
    pub fn Gain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn DistanceScale(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDistanceScale(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn DopplerScale(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDopplerScale(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IsDopplerDisabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SpatialAudioModel(&self) -> ::windows::runtime::Result<SpatialAudioModel> {
        let this = &::windows::runtime::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe {
            let mut result__: SpatialAudioModel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialAudioModel>(result__)
        }
    }
    pub fn SetSpatialAudioModel(&self, value: SpatialAudioModel) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn CreateAudioNodeEmitter<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, AudioNodeEmitterShape>,
        Param1: ::windows::runtime::IntoParam<'a, AudioNodeEmitterDecayModel>,
    >(
        shape: Param0,
        decaymodel: Param1,
        settings: AudioNodeEmitterSettings,
    ) -> ::windows::runtime::Result<AudioNodeEmitter> {
        Self::IAudioNodeEmitterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                shape.into_param().abi(),
                decaymodel.into_param().abi(),
                settings,
                &mut result__,
            )
            .from_abi::<AudioNodeEmitter>(result__)
        })
    }
    pub fn IAudioNodeEmitterFactory<
        R,
        F: FnOnce(&IAudioNodeEmitterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AudioNodeEmitter,
            IAudioNodeEmitterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeEmitter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioNodeEmitter;{3676971d-880a-47b8-adf7-1323a9d965be})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        913741597,
        34826,
        18360,
        [173, 247, 19, 35, 169, 217, 101, 190],
    );
}
impl ::windows::runtime::RuntimeName for AudioNodeEmitter {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitter";
}
impl ::std::convert::From<AudioNodeEmitter> for ::windows::runtime::IUnknown {
    fn from(value: AudioNodeEmitter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioNodeEmitter> for ::windows::runtime::IUnknown {
    fn from(value: &AudioNodeEmitter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioNodeEmitter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioNodeEmitter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioNodeEmitter> for ::windows::runtime::IInspectable {
    fn from(value: AudioNodeEmitter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioNodeEmitter> for ::windows::runtime::IInspectable {
    fn from(value: &AudioNodeEmitter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioNodeEmitter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioNodeEmitter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioNodeEmitter {}
unsafe impl ::std::marker::Sync for AudioNodeEmitter {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioNodeEmitterConeProperties(::windows::runtime::IInspectable);
impl AudioNodeEmitterConeProperties {
    pub fn InnerAngle(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn OuterAngle(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn OuterAngleGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeEmitterConeProperties {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioNodeEmitterConeProperties;{e99b2cee-02ca-4375-9326-0c6ae4bcdfb5})" ) ;
}
unsafe impl ::windows::runtime::Interface for AudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3919260910,
        714,
        17269,
        [147, 38, 12, 106, 228, 188, 223, 181],
    );
}
impl ::windows::runtime::RuntimeName for AudioNodeEmitterConeProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterConeProperties";
}
impl ::std::convert::From<AudioNodeEmitterConeProperties> for ::windows::runtime::IUnknown {
    fn from(value: AudioNodeEmitterConeProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioNodeEmitterConeProperties> for ::windows::runtime::IUnknown {
    fn from(value: &AudioNodeEmitterConeProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AudioNodeEmitterConeProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioNodeEmitterConeProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioNodeEmitterConeProperties> for ::windows::runtime::IInspectable {
    fn from(value: AudioNodeEmitterConeProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioNodeEmitterConeProperties> for ::windows::runtime::IInspectable {
    fn from(value: &AudioNodeEmitterConeProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioNodeEmitterConeProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioNodeEmitterConeProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioNodeEmitterConeProperties {}
unsafe impl ::std::marker::Sync for AudioNodeEmitterConeProperties {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: AudioNodeEmitterDecayKind = AudioNodeEmitterDecayKind(0i32);
    pub const Custom: AudioNodeEmitterDecayKind = AudioNodeEmitterDecayKind(1i32);
}
impl ::std::convert::From<i32> for AudioNodeEmitterDecayKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioNodeEmitterDecayKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeEmitterDecayKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioNodeEmitterDecayKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioNodeEmitterDecayModel(::windows::runtime::IInspectable);
impl AudioNodeEmitterDecayModel {
    pub fn Kind(&self) -> ::windows::runtime::Result<AudioNodeEmitterDecayKind> {
        let this = self;
        unsafe {
            let mut result__: AudioNodeEmitterDecayKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterDecayKind>(result__)
        }
    }
    pub fn MinGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn MaxGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn NaturalProperties(
        &self,
    ) -> ::windows::runtime::Result<AudioNodeEmitterNaturalDecayModelProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterNaturalDecayModelProperties>(result__)
        }
    }
    pub fn CreateNatural(
        mingain: f64,
        maxgain: f64,
        unitygaindistance: f64,
        cutoffdistance: f64,
    ) -> ::windows::runtime::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                mingain,
                maxgain,
                unitygaindistance,
                cutoffdistance,
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterDecayModel>(result__)
        })
    }
    pub fn CreateCustom(
        mingain: f64,
        maxgain: f64,
    ) -> ::windows::runtime::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                mingain,
                maxgain,
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterDecayModel>(result__)
        })
    }
    pub fn IAudioNodeEmitterDecayModelStatics<
        R,
        F: FnOnce(&IAudioNodeEmitterDecayModelStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AudioNodeEmitterDecayModel,
            IAudioNodeEmitterDecayModelStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeEmitterDecayModel {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioNodeEmitterDecayModel;{1d1d5af7-0d53-4fa9-bd84-d5816a86f3ff})" ) ;
}
unsafe impl ::windows::runtime::Interface for AudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        488463095,
        3411,
        20393,
        [189, 132, 213, 129, 106, 134, 243, 255],
    );
}
impl ::windows::runtime::RuntimeName for AudioNodeEmitterDecayModel {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterDecayModel";
}
impl ::std::convert::From<AudioNodeEmitterDecayModel> for ::windows::runtime::IUnknown {
    fn from(value: AudioNodeEmitterDecayModel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioNodeEmitterDecayModel> for ::windows::runtime::IUnknown {
    fn from(value: &AudioNodeEmitterDecayModel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AudioNodeEmitterDecayModel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioNodeEmitterDecayModel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioNodeEmitterDecayModel> for ::windows::runtime::IInspectable {
    fn from(value: AudioNodeEmitterDecayModel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioNodeEmitterDecayModel> for ::windows::runtime::IInspectable {
    fn from(value: &AudioNodeEmitterDecayModel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioNodeEmitterDecayModel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioNodeEmitterDecayModel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioNodeEmitterDecayModel {}
unsafe impl ::std::marker::Sync for AudioNodeEmitterDecayModel {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(::windows::runtime::IInspectable);
impl AudioNodeEmitterNaturalDecayModelProperties {
    pub fn UnityGainDistance(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn CutoffDistance(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeEmitterNaturalDecayModelProperties {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties;{48934bcf-cf2c-4efc-9331-75bd22df1f0c})" ) ;
}
unsafe impl ::windows::runtime::Interface for AudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1217612751,
        53036,
        20220,
        [147, 49, 117, 189, 34, 223, 31, 12],
    );
}
impl ::windows::runtime::RuntimeName for AudioNodeEmitterNaturalDecayModelProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties";
}
impl ::std::convert::From<AudioNodeEmitterNaturalDecayModelProperties>
    for ::windows::runtime::IUnknown
{
    fn from(value: AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioNodeEmitterNaturalDecayModelProperties>
    for ::windows::runtime::IUnknown
{
    fn from(value: &AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AudioNodeEmitterNaturalDecayModelProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioNodeEmitterNaturalDecayModelProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioNodeEmitterNaturalDecayModelProperties>
    for ::windows::runtime::IInspectable
{
    fn from(value: AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioNodeEmitterNaturalDecayModelProperties>
    for ::windows::runtime::IInspectable
{
    fn from(value: &AudioNodeEmitterNaturalDecayModelProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioNodeEmitterNaturalDecayModelProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioNodeEmitterNaturalDecayModelProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioNodeEmitterNaturalDecayModelProperties {}
unsafe impl ::std::marker::Sync for AudioNodeEmitterNaturalDecayModelProperties {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: AudioNodeEmitterSettings = AudioNodeEmitterSettings(0u32);
    pub const DisableDoppler: AudioNodeEmitterSettings = AudioNodeEmitterSettings(1u32);
}
impl ::std::convert::From<u32> for AudioNodeEmitterSettings {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioNodeEmitterSettings {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeEmitterSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioNodeEmitterSettings;u4)",
    );
}
impl ::std::ops::BitOr for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for AudioNodeEmitterSettings {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for AudioNodeEmitterSettings {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for AudioNodeEmitterSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioNodeEmitterShape(::windows::runtime::IInspectable);
impl AudioNodeEmitterShape {
    pub fn Kind(&self) -> ::windows::runtime::Result<AudioNodeEmitterShapeKind> {
        let this = self;
        unsafe {
            let mut result__: AudioNodeEmitterShapeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterShapeKind>(result__)
        }
    }
    pub fn ConeProperties(&self) -> ::windows::runtime::Result<AudioNodeEmitterConeProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterConeProperties>(result__)
        }
    }
    pub fn CreateCone(
        innerangle: f64,
        outerangle: f64,
        outeranglegain: f64,
    ) -> ::windows::runtime::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                innerangle,
                outerangle,
                outeranglegain,
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterShape>(result__)
        })
    }
    pub fn CreateOmnidirectional() -> ::windows::runtime::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitterShape>(result__)
        })
    }
    pub fn IAudioNodeEmitterShapeStatics<
        R,
        F: FnOnce(&IAudioNodeEmitterShapeStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AudioNodeEmitterShape,
            IAudioNodeEmitterShapeStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeEmitterShape {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioNodeEmitterShape;{ea0311c5-e73d-44bc-859c-45553bbc4828})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3926069701,
        59197,
        17596,
        [133, 156, 69, 85, 59, 188, 72, 40],
    );
}
impl ::windows::runtime::RuntimeName for AudioNodeEmitterShape {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterShape";
}
impl ::std::convert::From<AudioNodeEmitterShape> for ::windows::runtime::IUnknown {
    fn from(value: AudioNodeEmitterShape) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioNodeEmitterShape> for ::windows::runtime::IUnknown {
    fn from(value: &AudioNodeEmitterShape) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioNodeEmitterShape {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioNodeEmitterShape
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioNodeEmitterShape> for ::windows::runtime::IInspectable {
    fn from(value: AudioNodeEmitterShape) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioNodeEmitterShape> for ::windows::runtime::IInspectable {
    fn from(value: &AudioNodeEmitterShape) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioNodeEmitterShape
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioNodeEmitterShape
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioNodeEmitterShape {}
unsafe impl ::std::marker::Sync for AudioNodeEmitterShape {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: AudioNodeEmitterShapeKind = AudioNodeEmitterShapeKind(0i32);
    pub const Cone: AudioNodeEmitterShapeKind = AudioNodeEmitterShapeKind(1i32);
}
impl ::std::convert::From<i32> for AudioNodeEmitterShapeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioNodeEmitterShapeKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeEmitterShapeKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioNodeEmitterShapeKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioNodeListener(::windows::runtime::IInspectable);
impl AudioNodeListener {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AudioNodeListener,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Quaternion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Quaternion>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SpeedOfSound(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetSpeedOfSound(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Vector3 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioNodeListener {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioNodeListener;{d9722e16-0c0a-41da-b755-6c77835fb1eb})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioNodeListener {
    type Vtable = IAudioNodeListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3648138774,
        3082,
        16858,
        [183, 85, 108, 119, 131, 95, 177, 235],
    );
}
impl ::windows::runtime::RuntimeName for AudioNodeListener {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeListener";
}
impl ::std::convert::From<AudioNodeListener> for ::windows::runtime::IUnknown {
    fn from(value: AudioNodeListener) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioNodeListener> for ::windows::runtime::IUnknown {
    fn from(value: &AudioNodeListener) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioNodeListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioNodeListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioNodeListener> for ::windows::runtime::IInspectable {
    fn from(value: AudioNodeListener) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioNodeListener> for ::windows::runtime::IInspectable {
    fn from(value: &AudioNodeListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioNodeListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioNodeListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioNodeListener {}
unsafe impl ::std::marker::Sync for AudioNodeListener {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioPlaybackConnection(::windows::runtime::IInspectable);
impl AudioPlaybackConnection {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows::runtime::Result<AudioPlaybackConnectionState> {
        let this = self;
        unsafe {
            let mut result__: AudioPlaybackConnectionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioPlaybackConnectionState>(result__)
        }
    }
    pub fn Open(&self) -> ::windows::runtime::Result<AudioPlaybackConnectionOpenResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioPlaybackConnectionOpenResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                AudioPlaybackConnectionOpenResult,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                AudioPlaybackConnection,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn TryCreateFromId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        id: Param0,
    ) -> ::windows::runtime::Result<AudioPlaybackConnection> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                id.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AudioPlaybackConnection>(result__)
        })
    }
    pub fn IAudioPlaybackConnectionStatics<
        R,
        F: FnOnce(&IAudioPlaybackConnectionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AudioPlaybackConnection,
            IAudioPlaybackConnectionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioPlaybackConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioPlaybackConnection;{1a4c1dea-cafc-50e7-8718-ea3f81cbfa51})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        441196010,
        51964,
        20711,
        [135, 24, 234, 63, 129, 203, 250, 81],
    );
}
impl ::windows::runtime::RuntimeName for AudioPlaybackConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnection";
}
impl ::std::convert::From<AudioPlaybackConnection> for ::windows::runtime::IUnknown {
    fn from(value: AudioPlaybackConnection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioPlaybackConnection> for ::windows::runtime::IUnknown {
    fn from(value: &AudioPlaybackConnection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AudioPlaybackConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioPlaybackConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioPlaybackConnection> for ::windows::runtime::IInspectable {
    fn from(value: AudioPlaybackConnection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioPlaybackConnection> for ::windows::runtime::IInspectable {
    fn from(value: &AudioPlaybackConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioPlaybackConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioPlaybackConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioPlaybackConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioPlaybackConnection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioPlaybackConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioPlaybackConnection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioPlaybackConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioPlaybackConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioPlaybackConnection {}
unsafe impl ::std::marker::Sync for AudioPlaybackConnection {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioPlaybackConnectionOpenResult(::windows::runtime::IInspectable);
impl AudioPlaybackConnectionOpenResult {
    pub fn Status(&self) -> ::windows::runtime::Result<AudioPlaybackConnectionOpenResultStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioPlaybackConnectionOpenResultStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioPlaybackConnectionOpenResultStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioPlaybackConnectionOpenResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioPlaybackConnectionOpenResult;{4e656aef-39f9-5fc9-a519-a5bbfd9fe921})" ) ;
}
unsafe impl ::windows::runtime::Interface for AudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1315269359,
        14841,
        24521,
        [165, 25, 165, 187, 253, 159, 233, 33],
    );
}
impl ::windows::runtime::RuntimeName for AudioPlaybackConnectionOpenResult {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnectionOpenResult";
}
impl ::std::convert::From<AudioPlaybackConnectionOpenResult> for ::windows::runtime::IUnknown {
    fn from(value: AudioPlaybackConnectionOpenResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioPlaybackConnectionOpenResult> for ::windows::runtime::IUnknown {
    fn from(value: &AudioPlaybackConnectionOpenResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AudioPlaybackConnectionOpenResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &AudioPlaybackConnectionOpenResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioPlaybackConnectionOpenResult> for ::windows::runtime::IInspectable {
    fn from(value: AudioPlaybackConnectionOpenResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioPlaybackConnectionOpenResult> for ::windows::runtime::IInspectable {
    fn from(value: &AudioPlaybackConnectionOpenResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AudioPlaybackConnectionOpenResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioPlaybackConnectionOpenResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioPlaybackConnectionOpenResult {}
unsafe impl ::std::marker::Sync for AudioPlaybackConnectionOpenResult {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: AudioPlaybackConnectionOpenResultStatus =
        AudioPlaybackConnectionOpenResultStatus(0i32);
    pub const RequestTimedOut: AudioPlaybackConnectionOpenResultStatus =
        AudioPlaybackConnectionOpenResultStatus(1i32);
    pub const DeniedBySystem: AudioPlaybackConnectionOpenResultStatus =
        AudioPlaybackConnectionOpenResultStatus(2i32);
    pub const UnknownFailure: AudioPlaybackConnectionOpenResultStatus =
        AudioPlaybackConnectionOpenResultStatus(3i32);
}
impl ::std::convert::From<i32> for AudioPlaybackConnectionOpenResultStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioPlaybackConnectionOpenResultStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioPlaybackConnectionOpenResultStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioPlaybackConnectionOpenResultStatus;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: AudioPlaybackConnectionState = AudioPlaybackConnectionState(0i32);
    pub const Opened: AudioPlaybackConnectionState = AudioPlaybackConnectionState(1i32);
}
impl ::std::convert::From<i32> for AudioPlaybackConnectionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioPlaybackConnectionState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioPlaybackConnectionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.AudioPlaybackConnectionState;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioStateMonitor(::windows::runtime::IInspectable);
impl AudioStateMonitor {
    #[cfg(feature = "Foundation")]
    pub fn SoundLevelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                AudioStateMonitor,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSoundLevelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SoundLevel(&self) -> ::windows::runtime::Result<super::SoundLevel> {
        let this = self;
        unsafe {
            let mut result__: super::SoundLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::SoundLevel>(result__)
        }
    }
    pub fn CreateForRenderMonitoring() -> ::windows::runtime::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategory(
        category: super::Render::AudioRenderCategory,
    ) -> ::windows::runtime::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                category,
                &mut result__,
            )
            .from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceRole(
        category: super::Render::AudioRenderCategory,
        role: super::Devices::AudioDeviceRole,
    ) -> ::windows::runtime::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                category,
                role,
                &mut result__,
            )
            .from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceId<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        category: super::Render::AudioRenderCategory,
        deviceid: Param1,
    ) -> ::windows::runtime::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                category,
                deviceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AudioStateMonitor>(result__)
        })
    }
    pub fn CreateForCaptureMonitoring() -> ::windows::runtime::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategory(
        category: super::Capture::MediaCategory,
    ) -> ::windows::runtime::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                category,
                &mut result__,
            )
            .from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceRole(
        category: super::Capture::MediaCategory,
        role: super::Devices::AudioDeviceRole,
    ) -> ::windows::runtime::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                category,
                role,
                &mut result__,
            )
            .from_abi::<AudioStateMonitor>(result__)
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceId<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        category: super::Capture::MediaCategory,
        deviceid: Param1,
    ) -> ::windows::runtime::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                category,
                deviceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AudioStateMonitor>(result__)
        })
    }
    pub fn IAudioStateMonitorStatics<
        R,
        F: FnOnce(&IAudioStateMonitorStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AudioStateMonitor,
            IAudioStateMonitorStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioStateMonitor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioStateMonitor;{1d13d136-0199-4cdc-b84e-e72c2b581ece})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioStateMonitor {
    type Vtable = IAudioStateMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        487838006,
        409,
        19676,
        [184, 78, 231, 44, 43, 88, 30, 206],
    );
}
impl ::windows::runtime::RuntimeName for AudioStateMonitor {
    const NAME: &'static str = "Windows.Media.Audio.AudioStateMonitor";
}
impl ::std::convert::From<AudioStateMonitor> for ::windows::runtime::IUnknown {
    fn from(value: AudioStateMonitor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioStateMonitor> for ::windows::runtime::IUnknown {
    fn from(value: &AudioStateMonitor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioStateMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioStateMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioStateMonitor> for ::windows::runtime::IInspectable {
    fn from(value: AudioStateMonitor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioStateMonitor> for ::windows::runtime::IInspectable {
    fn from(value: &AudioStateMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioStateMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioStateMonitor
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AudioStateMonitor {}
unsafe impl ::std::marker::Sync for AudioStateMonitor {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AudioSubmixNode(::windows::runtime::IInspectable);
impl AudioSubmixNode {
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<AudioGraphConnection>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(
                result__,
            )
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddOutgoingConnectionWithGain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IAudioNode>,
    >(
        &self,
        destination: Param0,
        gain: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
                gain,
            )
            .ok()
        }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Emitter(&self) -> ::windows::runtime::Result<AudioNodeEmitter> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioSubmixNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.AudioSubmixNode;{d148005c-8428-4784-b7fd-a99d468c5d20})",
    );
}
unsafe impl ::windows::runtime::Interface for AudioSubmixNode {
    type Vtable = IAudioInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3511156828,
        33832,
        18308,
        [183, 253, 169, 157, 70, 140, 93, 32],
    );
}
impl ::windows::runtime::RuntimeName for AudioSubmixNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioSubmixNode";
}
impl ::std::convert::From<AudioSubmixNode> for ::windows::runtime::IUnknown {
    fn from(value: AudioSubmixNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioSubmixNode> for ::windows::runtime::IUnknown {
    fn from(value: &AudioSubmixNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<AudioSubmixNode> for ::windows::runtime::IInspectable {
    fn from(value: AudioSubmixNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioSubmixNode> for ::windows::runtime::IInspectable {
    fn from(value: &AudioSubmixNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AudioSubmixNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AudioSubmixNode> for IAudioInputNode {
    fn from(value: AudioSubmixNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioSubmixNode> for IAudioInputNode {
    fn from(value: &AudioSubmixNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IAudioInputNode>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IAudioInputNode>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AudioSubmixNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AudioSubmixNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for AudioSubmixNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &AudioSubmixNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioSubmixNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioSubmixNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<AudioSubmixNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioSubmixNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioSubmixNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioSubmixNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for &AudioSubmixNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::std::convert::TryInto::<IAudioInputNode2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioSubmixNode {}
unsafe impl ::std::marker::Sync for AudioSubmixNode {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CreateAudioDeviceInputNodeResult(::windows::runtime::IInspectable);
impl CreateAudioDeviceInputNodeResult {
    pub fn Status(&self) -> ::windows::runtime::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceNodeCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioDeviceNodeCreationStatus>(result__)
        }
    }
    pub fn DeviceInputNode(&self) -> ::windows::runtime::Result<AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioDeviceInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this =
            &::windows::runtime::Interface::cast::<ICreateAudioDeviceInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreateAudioDeviceInputNodeResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.CreateAudioDeviceInputNodeResult;{16eec7a8-1ca7-40ef-91a4-d346e0aa1bba})" ) ;
}
unsafe impl ::windows::runtime::Interface for CreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        384747432,
        7335,
        16623,
        [145, 164, 211, 70, 224, 170, 27, 186],
    );
}
impl ::windows::runtime::RuntimeName for CreateAudioDeviceInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceInputNodeResult";
}
impl ::std::convert::From<CreateAudioDeviceInputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: CreateAudioDeviceInputNodeResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CreateAudioDeviceInputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: &CreateAudioDeviceInputNodeResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CreateAudioDeviceInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CreateAudioDeviceInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CreateAudioDeviceInputNodeResult> for ::windows::runtime::IInspectable {
    fn from(value: CreateAudioDeviceInputNodeResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreateAudioDeviceInputNodeResult> for ::windows::runtime::IInspectable {
    fn from(value: &CreateAudioDeviceInputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CreateAudioDeviceInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CreateAudioDeviceInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreateAudioDeviceInputNodeResult {}
unsafe impl ::std::marker::Sync for CreateAudioDeviceInputNodeResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CreateAudioDeviceOutputNodeResult(::windows::runtime::IInspectable);
impl CreateAudioDeviceOutputNodeResult {
    pub fn Status(&self) -> ::windows::runtime::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioDeviceNodeCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioDeviceNodeCreationStatus>(result__)
        }
    }
    pub fn DeviceOutputNode(&self) -> ::windows::runtime::Result<AudioDeviceOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioDeviceOutputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this =
            &::windows::runtime::Interface::cast::<ICreateAudioDeviceOutputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreateAudioDeviceOutputNodeResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.CreateAudioDeviceOutputNodeResult;{f7776d27-1d9a-47f7-9cd4-2859cc1b7bff})" ) ;
}
unsafe impl ::windows::runtime::Interface for CreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4151799079,
        7578,
        18423,
        [156, 212, 40, 89, 204, 27, 123, 255],
    );
}
impl ::windows::runtime::RuntimeName for CreateAudioDeviceOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
}
impl ::std::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CreateAudioDeviceOutputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CreateAudioDeviceOutputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows::runtime::IInspectable {
    fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows::runtime::IInspectable {
    fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CreateAudioDeviceOutputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CreateAudioDeviceOutputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreateAudioDeviceOutputNodeResult {}
unsafe impl ::std::marker::Sync for CreateAudioDeviceOutputNodeResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CreateAudioFileInputNodeResult(::windows::runtime::IInspectable);
impl CreateAudioFileInputNodeResult {
    pub fn Status(&self) -> ::windows::runtime::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioFileNodeCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioFileNodeCreationStatus>(result__)
        }
    }
    pub fn FileInputNode(&self) -> ::windows::runtime::Result<AudioFileInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioFileInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<ICreateAudioFileInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreateAudioFileInputNodeResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.CreateAudioFileInputNodeResult;{ce83d61c-e297-4c50-9ce7-1c7a69d6bd09})" ) ;
}
unsafe impl ::windows::runtime::Interface for CreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3464746524,
        58007,
        19536,
        [156, 231, 28, 122, 105, 214, 189, 9],
    );
}
impl ::windows::runtime::RuntimeName for CreateAudioFileInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileInputNodeResult";
}
impl ::std::convert::From<CreateAudioFileInputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: CreateAudioFileInputNodeResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CreateAudioFileInputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: &CreateAudioFileInputNodeResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CreateAudioFileInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CreateAudioFileInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CreateAudioFileInputNodeResult> for ::windows::runtime::IInspectable {
    fn from(value: CreateAudioFileInputNodeResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreateAudioFileInputNodeResult> for ::windows::runtime::IInspectable {
    fn from(value: &CreateAudioFileInputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CreateAudioFileInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CreateAudioFileInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreateAudioFileInputNodeResult {}
unsafe impl ::std::marker::Sync for CreateAudioFileInputNodeResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CreateAudioFileOutputNodeResult(::windows::runtime::IInspectable);
impl CreateAudioFileOutputNodeResult {
    pub fn Status(&self) -> ::windows::runtime::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioFileNodeCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioFileNodeCreationStatus>(result__)
        }
    }
    pub fn FileOutputNode(&self) -> ::windows::runtime::Result<AudioFileOutputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioFileOutputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<ICreateAudioFileOutputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreateAudioFileOutputNodeResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.CreateAudioFileOutputNodeResult;{47d6ba7b-e909-453f-866e-5540cda734ff})" ) ;
}
unsafe impl ::windows::runtime::Interface for CreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1205254779,
        59657,
        17727,
        [134, 110, 85, 64, 205, 167, 52, 255],
    );
}
impl ::windows::runtime::RuntimeName for CreateAudioFileOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileOutputNodeResult";
}
impl ::std::convert::From<CreateAudioFileOutputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: CreateAudioFileOutputNodeResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CreateAudioFileOutputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: &CreateAudioFileOutputNodeResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CreateAudioFileOutputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CreateAudioFileOutputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CreateAudioFileOutputNodeResult> for ::windows::runtime::IInspectable {
    fn from(value: CreateAudioFileOutputNodeResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreateAudioFileOutputNodeResult> for ::windows::runtime::IInspectable {
    fn from(value: &CreateAudioFileOutputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CreateAudioFileOutputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CreateAudioFileOutputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreateAudioFileOutputNodeResult {}
unsafe impl ::std::marker::Sync for CreateAudioFileOutputNodeResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CreateAudioGraphResult(::windows::runtime::IInspectable);
impl CreateAudioGraphResult {
    pub fn Status(&self) -> ::windows::runtime::Result<AudioGraphCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AudioGraphCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioGraphCreationStatus>(result__)
        }
    }
    pub fn Graph(&self) -> ::windows::runtime::Result<AudioGraph> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioGraph>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<ICreateAudioGraphResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreateAudioGraphResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.CreateAudioGraphResult;{5453ef7e-7bde-4b76-bb5d-48f79cfc8c0b})",
    );
}
unsafe impl ::windows::runtime::Interface for CreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1414786942,
        31710,
        19318,
        [187, 93, 72, 247, 156, 252, 140, 11],
    );
}
impl ::windows::runtime::RuntimeName for CreateAudioGraphResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioGraphResult";
}
impl ::std::convert::From<CreateAudioGraphResult> for ::windows::runtime::IUnknown {
    fn from(value: CreateAudioGraphResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CreateAudioGraphResult> for ::windows::runtime::IUnknown {
    fn from(value: &CreateAudioGraphResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CreateAudioGraphResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CreateAudioGraphResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CreateAudioGraphResult> for ::windows::runtime::IInspectable {
    fn from(value: CreateAudioGraphResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreateAudioGraphResult> for ::windows::runtime::IInspectable {
    fn from(value: &CreateAudioGraphResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CreateAudioGraphResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CreateAudioGraphResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreateAudioGraphResult {}
unsafe impl ::std::marker::Sync for CreateAudioGraphResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct CreateMediaSourceAudioInputNodeResult(::windows::runtime::IInspectable);
impl CreateMediaSourceAudioInputNodeResult {
    pub fn Status(&self) -> ::windows::runtime::Result<MediaSourceAudioInputNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaSourceAudioInputNodeCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MediaSourceAudioInputNodeCreationStatus>(result__)
        }
    }
    pub fn Node(&self) -> ::windows::runtime::Result<MediaSourceAudioInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MediaSourceAudioInputNode>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this =
            &::windows::runtime::Interface::cast::<ICreateMediaSourceAudioInputNodeResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CreateMediaSourceAudioInputNodeResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult;{46a658a3-53c0-4d59-9e51-cc1d1044a4c4})" ) ;
}
unsafe impl ::windows::runtime::Interface for CreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1185306787,
        21440,
        19801,
        [158, 81, 204, 29, 16, 68, 164, 196],
    );
}
impl ::windows::runtime::RuntimeName for CreateMediaSourceAudioInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult";
}
impl ::std::convert::From<CreateMediaSourceAudioInputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: CreateMediaSourceAudioInputNodeResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CreateMediaSourceAudioInputNodeResult> for ::windows::runtime::IUnknown {
    fn from(value: &CreateMediaSourceAudioInputNodeResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for CreateMediaSourceAudioInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &CreateMediaSourceAudioInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<CreateMediaSourceAudioInputNodeResult>
    for ::windows::runtime::IInspectable
{
    fn from(value: CreateMediaSourceAudioInputNodeResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CreateMediaSourceAudioInputNodeResult>
    for ::windows::runtime::IInspectable
{
    fn from(value: &CreateMediaSourceAudioInputNodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for CreateMediaSourceAudioInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a CreateMediaSourceAudioInputNodeResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CreateMediaSourceAudioInputNodeResult {}
unsafe impl ::std::marker::Sync for CreateMediaSourceAudioInputNodeResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct EchoEffectDefinition(::windows::runtime::IInspectable);
impl EchoEffectDefinition {
    pub fn SetWetDryMix(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn WetDryMix(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFeedback(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Feedback(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDelay(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Delay(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this =
            &::windows::runtime::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this =
            &::windows::runtime::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, AudioGraph>>(
        audiograph: Param0,
    ) -> ::windows::runtime::Result<EchoEffectDefinition> {
        Self::IEchoEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                audiograph.into_param().abi(),
                &mut result__,
            )
            .from_abi::<EchoEffectDefinition>(result__)
        })
    }
    pub fn IEchoEffectDefinitionFactory<
        R,
        F: FnOnce(&IEchoEffectDefinitionFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            EchoEffectDefinition,
            IEchoEffectDefinitionFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EchoEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.EchoEffectDefinition;{0e4d3faa-36b8-4c91-b9da-11f44a8a6610})",
    );
}
unsafe impl ::windows::runtime::Interface for EchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        239943594,
        14008,
        19601,
        [185, 218, 17, 244, 74, 138, 102, 16],
    );
}
impl ::windows::runtime::RuntimeName for EchoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EchoEffectDefinition";
}
impl ::std::convert::From<EchoEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: EchoEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EchoEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &EchoEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EchoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EchoEffectDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<EchoEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: EchoEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EchoEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &EchoEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for EchoEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a EchoEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::std::convert::TryFrom<EchoEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: EchoEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::std::convert::TryFrom<&EchoEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &EchoEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>
    for EchoEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>
    for &EchoEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::std::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for EchoEffectDefinition {}
unsafe impl ::std::marker::Sync for EchoEffectDefinition {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct EqualizerBand(::windows::runtime::IInspectable);
impl EqualizerBand {
    pub fn Bandwidth(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetBandwidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn FrequencyCenter(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetFrequencyCenter(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Gain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EqualizerBand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.EqualizerBand;{c00a5a6a-262d-4b85-9bb7-43280b62ed0c})",
    );
}
unsafe impl ::windows::runtime::Interface for EqualizerBand {
    type Vtable = IEqualizerBand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3221903978,
        9773,
        19333,
        [155, 183, 67, 40, 11, 98, 237, 12],
    );
}
impl ::windows::runtime::RuntimeName for EqualizerBand {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerBand";
}
impl ::std::convert::From<EqualizerBand> for ::windows::runtime::IUnknown {
    fn from(value: EqualizerBand) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EqualizerBand> for ::windows::runtime::IUnknown {
    fn from(value: &EqualizerBand) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EqualizerBand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EqualizerBand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<EqualizerBand> for ::windows::runtime::IInspectable {
    fn from(value: EqualizerBand) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EqualizerBand> for ::windows::runtime::IInspectable {
    fn from(value: &EqualizerBand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EqualizerBand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EqualizerBand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EqualizerBand {}
unsafe impl ::std::marker::Sync for EqualizerBand {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct EqualizerEffectDefinition(::windows::runtime::IInspectable);
impl EqualizerEffectDefinition {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bands(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<EqualizerBand>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<EqualizerBand>>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this =
            &::windows::runtime::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this =
            &::windows::runtime::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, AudioGraph>>(
        audiograph: Param0,
    ) -> ::windows::runtime::Result<EqualizerEffectDefinition> {
        Self::IEqualizerEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                audiograph.into_param().abi(),
                &mut result__,
            )
            .from_abi::<EqualizerEffectDefinition>(result__)
        })
    }
    pub fn IEqualizerEffectDefinitionFactory<
        R,
        F: FnOnce(&IEqualizerEffectDefinitionFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            EqualizerEffectDefinition,
            IEqualizerEffectDefinitionFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EqualizerEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.EqualizerEffectDefinition;{023f6f1f-83fe-449a-a822-c696442d16b0})",
    );
}
unsafe impl ::windows::runtime::Interface for EqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        37711647,
        33790,
        17562,
        [168, 34, 198, 150, 68, 45, 22, 176],
    );
}
impl ::windows::runtime::RuntimeName for EqualizerEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerEffectDefinition";
}
impl ::std::convert::From<EqualizerEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: EqualizerEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EqualizerEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &EqualizerEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for EqualizerEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &EqualizerEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<EqualizerEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: EqualizerEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EqualizerEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &EqualizerEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for EqualizerEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a EqualizerEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::std::convert::TryFrom<EqualizerEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: EqualizerEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::std::convert::TryFrom<&EqualizerEffectDefinition>
    for super::Effects::IAudioEffectDefinition
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &EqualizerEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>
    for EqualizerEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>
    for &EqualizerEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::std::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for EqualizerEffectDefinition {}
unsafe impl ::std::marker::Sync for EqualizerEffectDefinition {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct FrameInputNodeQuantumStartedEventArgs(::windows::runtime::IInspectable);
impl FrameInputNodeQuantumStartedEventArgs {
    pub fn RequiredSamples(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FrameInputNodeQuantumStartedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs;{3d9bd498-a306-4f06-bd9f-e9efc8226304})" ) ;
}
unsafe impl ::windows::runtime::Interface for FrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1033622680,
        41734,
        20230,
        [189, 159, 233, 239, 200, 34, 99, 4],
    );
}
impl ::windows::runtime::RuntimeName for FrameInputNodeQuantumStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs";
}
impl ::std::convert::From<FrameInputNodeQuantumStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FrameInputNodeQuantumStartedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FrameInputNodeQuantumStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FrameInputNodeQuantumStartedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for FrameInputNodeQuantumStartedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &FrameInputNodeQuantumStartedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<FrameInputNodeQuantumStartedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: FrameInputNodeQuantumStartedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FrameInputNodeQuantumStartedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &FrameInputNodeQuantumStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for FrameInputNodeQuantumStartedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a FrameInputNodeQuantumStartedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FrameInputNodeQuantumStartedEventArgs {}
unsafe impl ::std::marker::Sync for FrameInputNodeQuantumStartedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioDeviceInputNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2954587105,
        28494,
        18914,
        [172, 1, 85, 157, 98, 190, 179, 169],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceInputNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioDeviceOutputNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        909040639,
        65308,
        17460,
        [158, 15, 189, 46, 245, 34, 172, 130],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceOutputNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioFileInputNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioFileInputNode {
    type Vtable = IAudioFileInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2421909448,
        28517,
        19668,
        [136, 144, 70, 148, 132, 60, 39, 109],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileInputNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        position: super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioFileOutputNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1356863872,
        20838,
        16531,
        [128, 248, 173, 160, 0, 137, 233, 207],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileOutputNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioFrameCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3699147422,
        520,
        17668,
        [165, 168, 240, 242, 104, 146, 10, 101],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameCompletedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioFrameInputNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        28468935,
        64918,
        20469,
        [163, 197, 210, 122, 155, 244, 66, 55],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameInputNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        frame: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioFrameOutputNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3091674907,
        12953,
        17909,
        [136, 179, 201, 209, 42, 63, 28, 200],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameOutputNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraph(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraph {
    type Vtable = IAudioGraph_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        450129645,
        58508,
        19988,
        [150, 96, 44, 79, 131, 233, 205, 216],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        encodingproperties: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Capture::MediaCategory,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture")))] usize,
    #[cfg(all(
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Capture::MediaCategory,
        encodingproperties: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    )))]
    usize,
    #[cfg(all(
        feature = "Devices_Enumeration",
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Capture::MediaCategory,
        encodingproperties: ::windows::runtime::RawPtr,
        device: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Devices_Enumeration",
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        encodingproperties: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        file: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        file: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(
        feature = "Foundation",
        feature = "Media_MediaProperties",
        feature = "Storage"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        file: ::windows::runtime::RawPtr,
        fileencodingprofile: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Foundation",
        feature = "Media_MediaProperties",
        feature = "Storage"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        encodingproperties: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::AudioProcessing,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraph2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraph2 {
    type Vtable = IAudioGraph2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1313618901,
        20417,
        17910,
        [169, 71, 60, 211, 143, 79, 216, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        encodingproperties: ::windows::runtime::RawPtr,
        emitter: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(
        feature = "Devices_Enumeration",
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Capture::MediaCategory,
        encodingproperties: ::windows::runtime::RawPtr,
        device: ::windows::runtime::RawPtr,
        emitter: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Devices_Enumeration",
        feature = "Foundation",
        feature = "Media_Capture",
        feature = "Media_MediaProperties"
    )))]
    usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        file: ::windows::runtime::RawPtr,
        emitter: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        encodingproperties: ::windows::runtime::RawPtr,
        emitter: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraph3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraph3 {
    type Vtable = IAudioGraph3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3721209262,
        4485,
        17063,
        [131, 29, 106, 155, 15, 200, 104, 32],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mediasource: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mediasource: ::windows::runtime::RawPtr,
        emitter: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraphConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraphConnection {
    type Vtable = IAudioGraphConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1982886125,
        53326,
        20396,
        [178, 51, 96, 11, 66, 237, 212, 105],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphConnection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraphSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraphSettings {
    type Vtable = IAudioGraphSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        492397695,
        59134,
        17960,
        [132, 248, 157, 139, 219, 162, 87, 133],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut QuantumSizeSelectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: QuantumSizeSelectionMode,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::Render::AudioRenderCategory,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    #[cfg(feature = "Media_Render")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::Render::AudioRenderCategory,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::AudioProcessing,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::AudioProcessing,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraphSettings2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraphSettings2 {
    type Vtable = IAudioGraphSettings2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1922144135,
        19883,
        18147,
        [180, 201, 216, 225, 162, 99, 96, 98],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraphSettingsFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraphSettingsFactory {
    type Vtable = IAudioGraphSettingsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2782469318,
        49899,
        19041,
        [162, 20, 29, 102, 215, 95, 131, 218],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettingsFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        audiorendercategory: super::Render::AudioRenderCategory,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraphStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraphStatics {
    type Vtable = IAudioGraphStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1995190578,
        57689,
        19127,
        [168, 42, 23, 190, 180, 179, 30, 148],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        settings: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3285830624,
        16374,
        20403,
        [178, 98, 80, 212, 53, 197, 84, 35],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioGraphUnrecoverableError,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAudioInputNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioInputNode {
    type Vtable = IAudioInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3511156828,
        33832,
        18308,
        [183, 253, 169, 157, 70, 140, 93, 32],
    );
}
impl IAudioInputNode {
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<AudioGraphConnection>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(
                result__,
            )
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddOutgoingConnectionWithGain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IAudioNode>,
    >(
        &self,
        destination: Param0,
        gain: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
                gain,
            )
            .ok()
        }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAudioInputNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{d148005c-8428-4784-b7fd-a99d468c5d20}");
}
impl ::std::convert::From<IAudioInputNode> for ::windows::runtime::IUnknown {
    fn from(value: IAudioInputNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAudioInputNode> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioInputNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IAudioInputNode> for ::windows::runtime::IInspectable {
    fn from(value: IAudioInputNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioInputNode> for ::windows::runtime::IInspectable {
    fn from(value: &IAudioInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IAudioInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IAudioInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for IAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &IAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IAudioInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IAudioInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for IAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &IAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        destination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        destination: ::windows::runtime::RawPtr,
        gain: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        destination: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAudioInputNode2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioInputNode2 {
    type Vtable = IAudioInputNode2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2421249719,
        51816,
        19565,
        [168, 188, 227, 238, 23, 254, 63, 210],
    );
}
impl IAudioInputNode2 {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<AudioGraphConnection>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(
                result__,
            )
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddOutgoingConnectionWithGain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IAudioNode>,
    >(
        &self,
        destination: Param0,
        gain: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
                gain,
            )
            .ok()
        }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Emitter(&self) -> ::windows::runtime::Result<AudioNodeEmitter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitter>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAudioInputNode2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{905156b7-ca68-4c6d-a8bc-e3ee17fe3fd2}");
}
impl ::std::convert::From<IAudioInputNode2> for ::windows::runtime::IUnknown {
    fn from(value: IAudioInputNode2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAudioInputNode2> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioInputNode2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IAudioInputNode2> for ::windows::runtime::IInspectable {
    fn from(value: IAudioInputNode2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioInputNode2> for ::windows::runtime::IInspectable {
    fn from(value: &IAudioInputNode2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IAudioInputNode2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IAudioInputNode2> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IAudioInputNode2> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for IAudioInputNode2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &IAudioInputNode2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IAudioInputNode2> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IAudioInputNode2> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::std::convert::TryInto::<IAudioInputNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IAudioInputNode2> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAudioInputNode2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IAudioInputNode2> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAudioInputNode2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for IAudioInputNode2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &IAudioInputNode2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAudioNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNode {
    type Vtable = IAudioNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        356031871,
        56280,
        18457,
        [191, 3, 102, 142, 147, 87, 205, 109],
    );
}
impl IAudioNode {
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAudioNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{15389d7f-dbd8-4819-bf03-668e9357cd6d}");
}
impl ::std::convert::From<IAudioNode> for ::windows::runtime::IUnknown {
    fn from(value: IAudioNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAudioNode> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAudioNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IAudioNode> for ::windows::runtime::IInspectable {
    fn from(value: IAudioNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioNode> for ::windows::runtime::IInspectable {
    fn from(value: &IAudioNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAudioNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAudioNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IAudioNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAudioNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IAudioNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAudioNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for IAudioNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &IAudioNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Effects")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        definition: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Effects"))] usize,
    #[cfg(feature = "Media_Effects")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        definition: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Effects"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        913741597,
        34826,
        18360,
        [173, 247, 19, 35, 169, 217, 101, 190],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitter2 {
    type Vtable = IAudioNodeEmitter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1253502667,
        60457,
        18424,
        [129, 140, 182, 182, 96, 165, 174, 177],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SpatialAudioModel,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: SpatialAudioModel,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitterConeProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3919260910,
        714,
        17269,
        [147, 38, 12, 106, 228, 188, 223, 181],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterConeProperties_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModel(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        488463095,
        3411,
        20393,
        [189, 132, 213, 129, 106, 134, 243, 255],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModel_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioNodeEmitterDecayKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModelStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitterDecayModelStatics {
    type Vtable = IAudioNodeEmitterDecayModelStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3346562216,
        61816,
        17967,
        [188, 129, 141, 213, 203, 229, 218, 232],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModelStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mingain: f64,
        maxgain: f64,
        unitygaindistance: f64,
        cutoffdistance: f64,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mingain: f64,
        maxgain: f64,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitterFactory {
    type Vtable = IAudioNodeEmitterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4257761434,
        27350,
        19684,
        [183, 247, 169, 147, 112, 223, 126, 233],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        shape: ::windows::runtime::RawPtr,
        decaymodel: ::windows::runtime::RawPtr,
        settings: AudioNodeEmitterSettings,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1217612751,
        53036,
        20220,
        [147, 49, 117, 189, 34, 223, 31, 12],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShape(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3926069701,
        59197,
        17596,
        [133, 156, 69, 85, 59, 188, 72, 40],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShape_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioNodeEmitterShapeKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShapeStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeEmitterShapeStatics {
    type Vtable = IAudioNodeEmitterShapeStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1471883121,
        65445,
        19334,
        [167, 121, 226, 100, 174, 185, 20, 95],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShapeStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        innerangle: f64,
        outerangle: f64,
        outeranglegain: f64,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioNodeListener(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeListener {
    type Vtable = IAudioNodeListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3648138774,
        3082,
        16858,
        [183, 85, 108, 119, 131, 95, 177, 235],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeListener_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Quaternion,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::Numerics::Quaternion,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::Numerics::Vector3,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAudioNodeWithListener(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioNodeWithListener {
    type Vtable = IAudioNodeWithListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        235901052,
        31231,
        17732,
        [158, 235, 1, 37, 123, 21, 16, 90],
    );
}
impl IAudioNodeWithListener {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SetListener<'a, Param0: ::windows::runtime::IntoParam<'a, AudioNodeListener>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Listener(&self) -> ::windows::runtime::Result<AudioNodeListener> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeListener>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAudioNodeWithListener {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{0e0f907c-79ff-4544-9eeb-01257b15105a}");
}
impl ::std::convert::From<IAudioNodeWithListener> for ::windows::runtime::IUnknown {
    fn from(value: IAudioNodeWithListener) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAudioNodeWithListener> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioNodeWithListener) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IAudioNodeWithListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IAudioNodeWithListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IAudioNodeWithListener> for ::windows::runtime::IInspectable {
    fn from(value: IAudioNodeWithListener) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAudioNodeWithListener> for ::windows::runtime::IInspectable {
    fn from(value: &IAudioNodeWithListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IAudioNodeWithListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IAudioNodeWithListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<IAudioNodeWithListener> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAudioNodeWithListener) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&IAudioNodeWithListener> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAudioNodeWithListener) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for IAudioNodeWithListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &IAudioNodeWithListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IAudioNodeWithListener> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAudioNodeWithListener) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IAudioNodeWithListener> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAudioNodeWithListener) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for IAudioNodeWithListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &IAudioNodeWithListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeWithListener_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioPlaybackConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        441196010,
        51964,
        20711,
        [135, 24, 234, 63, 129, 203, 250, 81],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioPlaybackConnectionState,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionOpenResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1315269359,
        14841,
        24521,
        [165, 25, 165, 187, 253, 159, 233, 33],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionOpenResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioPlaybackConnectionOpenResultStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioPlaybackConnectionStatics {
    type Vtable = IAudioPlaybackConnectionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3859375010,
        27110,
        24572,
        [158, 19, 130, 74, 133, 33, 61, 175],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioStateMonitor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        487838006,
        409,
        19676,
        [184, 78, 231, 44, 43, 88, 30, 206],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::SoundLevel,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IAudioStateMonitorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioStateMonitorStatics {
    type Vtable = IAudioStateMonitorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1668606540,
        6971,
        16385,
        [148, 217, 221, 34, 83, 48, 250, 64],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitorStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Render::AudioRenderCategory,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Render::AudioRenderCategory,
        role: super::Devices::AudioDeviceRole,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Media_Devices", feature = "Media_Render")))] usize,
    #[cfg(feature = "Media_Render")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Render::AudioRenderCategory,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Render"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Capture::MediaCategory,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Capture::MediaCategory,
        role: super::Devices::AudioDeviceRole,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices")))] usize,
    #[cfg(feature = "Media_Capture")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        category: super::Capture::MediaCategory,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Capture"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        384747432,
        7335,
        16623,
        [145, 164, 211, 70, 224, 170, 27, 186],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioDeviceNodeCreationStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioDeviceInputNodeResult2 {
    type Vtable = ICreateAudioDeviceInputNodeResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2451335630,
        16181,
        16839,
        [150, 34, 121, 246, 8, 186, 237, 194],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4151799079,
        7578,
        18423,
        [156, 212, 40, 89, 204, 27, 123, 255],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioDeviceNodeCreationStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioDeviceOutputNodeResult2 {
    type Vtable = ICreateAudioDeviceOutputNodeResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1214523039,
        48590,
        19121,
        [189, 56, 251, 174, 147, 174, 218, 202],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3464746524,
        58007,
        19536,
        [156, 231, 28, 122, 105, 214, 189, 9],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioFileNodeCreationStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioFileInputNodeResult2 {
    type Vtable = ICreateAudioFileInputNodeResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4178059296,
        15744,
        20448,
        [129, 193, 118, 143, 234, 124, 167, 224],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1205254779,
        59657,
        17727,
        [134, 110, 85, 64, 205, 167, 52, 255],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioFileNodeCreationStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioFileOutputNodeResult2 {
    type Vtable = ICreateAudioFileOutputNodeResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2667689229,
        13080,
        18355,
        [166, 10, 27, 73, 43, 231, 252, 13],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1414786942,
        31710,
        19318,
        [187, 93, 72, 247, 156, 252, 140, 11],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut AudioGraphCreationStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateAudioGraphResult2 {
    type Vtable = ICreateAudioGraphResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1836289532,
        35014,
        20427,
        [165, 52, 133, 206, 221, 64, 80, 161],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1185306787,
        21440,
        19801,
        [158, 81, 204, 29, 16, 68, 164, 196],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut MediaSourceAudioInputNodeCreationStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICreateMediaSourceAudioInputNodeResult2 {
    type Vtable = ICreateMediaSourceAudioInputNodeResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1666272488,
        27162,
        18915,
        [151, 236, 40, 253, 91, 225, 20, 229],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IEchoEffectDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        239943594,
        14008,
        19601,
        [185, 218, 17, 244, 74, 138, 102, 16],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinition_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IEchoEffectDefinitionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEchoEffectDefinitionFactory {
    type Vtable = IEchoEffectDefinitionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        223224407,
        43762,
        20102,
        [165, 76, 251, 121, 219, 143, 108, 18],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinitionFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        audiograph: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IEqualizerBand(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEqualizerBand {
    type Vtable = IEqualizerBand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3221903978,
        9773,
        19333,
        [155, 183, 67, 40, 11, 98, 237, 12],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerBand_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        37711647,
        33790,
        17562,
        [168, 34, 198, 150, 68, 45, 22, 176],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinition_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinitionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEqualizerEffectDefinitionFactory {
    type Vtable = IEqualizerEffectDefinitionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3532091332,
        54288,
        20149,
        [158, 105, 201, 170, 18, 119, 234, 240],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinitionFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        audiograph: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IFrameInputNodeQuantumStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1033622680,
        41734,
        20230,
        [189, 159, 233, 239, 200, 34, 99, 4],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameInputNodeQuantumStartedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ILimiterEffectDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1802853657,
        9731,
        18362,
        [189, 235, 57, 5, 94, 52, 134, 220],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinition_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ILimiterEffectDefinitionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILimiterEffectDefinitionFactory {
    type Vtable = ILimiterEffectDefinitionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3971671793,
        25087,
        17903,
        [184, 245, 72, 101, 154, 87, 199, 45],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinitionFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        audiograph: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IMediaSourceAudioInputNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2581108795,
        43146,
        16449,
        [142, 79, 221, 186, 192, 201, 31, 211],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAudioInputNode_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        position: super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Media_Core")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IReverbEffectDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1174841993,
        62819,
        19722,
        [143, 110, 240, 205, 223, 243, 93, 132],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinition_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IReverbEffectDefinitionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReverbEffectDefinitionFactory {
    type Vtable = IReverbEffectDefinitionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2815806462,
        4107,
        20464,
        [157, 166, 220, 78, 5, 167, 89, 240],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinitionFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        audiograph: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISetDefaultSpatialAudioFormatResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        472556817,
        5120,
        24176,
        [158, 169, 174, 21, 18, 65, 232, 234],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetDefaultSpatialAudioFormatResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut SetDefaultSpatialAudioFormatStatus,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4001562676,
        25039,
        22345,
        [157, 164, 16, 240, 254, 2, 129, 153],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfiguration_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        subtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        subtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfigurationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAudioDeviceConfigurationStatics {
    type Vtable = ISpatialAudioDeviceConfigurationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1052999547,
        37741,
        19972,
        [151, 40, 40, 39, 217, 247, 88, 196],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfigurationStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        853477800,
        20720,
        21397,
        [153, 35, 125, 68, 202, 113, 237, 109],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfiguration_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        subtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        subtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut MixedRealitySpatialAudioFormatPolicy,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: MixedRealitySpatialAudioFormatPolicy,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfigurationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAudioFormatConfigurationStatics {
    type Vtable = ISpatialAudioFormatConfigurationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        727707505,
        26569,
        20063,
        [163, 91, 65, 104, 7, 17, 248, 199],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfigurationStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAudioFormatSubtypeStatics {
    type Vtable = ISpatialAudioFormatSubtypeStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3017706055,
        33774,
        16998,
        [169, 69, 190, 223, 80, 122, 254, 237],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialAudioFormatSubtypeStatics2 {
    type Vtable = ISpatialAudioFormatSubtypeStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1164306123,
        55643,
        22049,
        [182, 175, 14, 136, 73, 197, 124, 128],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct LimiterEffectDefinition(::windows::runtime::IInspectable);
impl LimiterEffectDefinition {
    pub fn SetRelease(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Release(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetLoudness(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn Loudness(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this =
            &::windows::runtime::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this =
            &::windows::runtime::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, AudioGraph>>(
        audiograph: Param0,
    ) -> ::windows::runtime::Result<LimiterEffectDefinition> {
        Self::ILimiterEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                audiograph.into_param().abi(),
                &mut result__,
            )
            .from_abi::<LimiterEffectDefinition>(result__)
        })
    }
    pub fn ILimiterEffectDefinitionFactory<
        R,
        F: FnOnce(&ILimiterEffectDefinitionFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            LimiterEffectDefinition,
            ILimiterEffectDefinitionFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LimiterEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.LimiterEffectDefinition;{6b755d19-2603-47ba-bdeb-39055e3486dc})",
    );
}
unsafe impl ::windows::runtime::Interface for LimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1802853657,
        9731,
        18362,
        [189, 235, 57, 5, 94, 52, 134, 220],
    );
}
impl ::windows::runtime::RuntimeName for LimiterEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.LimiterEffectDefinition";
}
impl ::std::convert::From<LimiterEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: LimiterEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LimiterEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &LimiterEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for LimiterEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &LimiterEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<LimiterEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: LimiterEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LimiterEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &LimiterEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for LimiterEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a LimiterEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::std::convert::TryFrom<LimiterEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LimiterEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::std::convert::TryFrom<&LimiterEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LimiterEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>
    for LimiterEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>
    for &LimiterEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::std::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LimiterEffectDefinition {}
unsafe impl ::std::marker::Sync for LimiterEffectDefinition {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MediaSourceAudioInputNode(::windows::runtime::IInspectable);
impl MediaSourceAudioInputNode {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<AudioGraphConnection>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>(
                result__,
            )
        }
    }
    pub fn AddOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AddOutgoingConnectionWithGain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IAudioNode>,
    >(
        &self,
        destination: Param0,
        gain: f64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
                gain,
            )
            .ok()
        }
    }
    pub fn RemoveOutgoingConnection<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioNode>>(
        &self,
        destination: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                destination.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>,
    > {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::Effects::IAudioEffectDefinition,
            >>(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn OutgoingGain(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(
        &self,
    ) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>,
    >(
        &self,
        definition: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioNode>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                definition.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Emitter(&self) -> ::windows::runtime::Result<AudioNodeEmitter> {
        let this = &::windows::runtime::Interface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AudioNodeEmitter>(result__)
        }
    }
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Seek<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
        &self,
        position: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                position.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndTime(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::IReference<super::super::Foundation::TimeSpan>,
        >,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoopCount(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLoopCount<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn MediaSource(&self) -> ::windows::runtime::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Core::MediaSource>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MediaSourceCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                MediaSourceAudioInputNode,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaSourceCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaSourceAudioInputNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.MediaSourceAudioInputNode;{99d8983b-a88a-4041-8e4f-ddbac0c91fd3})",
    );
}
unsafe impl ::windows::runtime::Interface for MediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2581108795,
        43146,
        16449,
        [142, 79, 221, 186, 192, 201, 31, 211],
    );
}
impl ::windows::runtime::RuntimeName for MediaSourceAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.MediaSourceAudioInputNode";
}
impl ::std::convert::From<MediaSourceAudioInputNode> for ::windows::runtime::IUnknown {
    fn from(value: MediaSourceAudioInputNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaSourceAudioInputNode> for ::windows::runtime::IUnknown {
    fn from(value: &MediaSourceAudioInputNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for MediaSourceAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &MediaSourceAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<MediaSourceAudioInputNode> for ::windows::runtime::IInspectable {
    fn from(value: MediaSourceAudioInputNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaSourceAudioInputNode> for ::windows::runtime::IInspectable {
    fn from(value: &MediaSourceAudioInputNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MediaSourceAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MediaSourceAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MediaSourceAudioInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MediaSourceAudioInputNode> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for MediaSourceAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &MediaSourceAudioInputNode
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<MediaSourceAudioInputNode> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioInputNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode> {
        ::std::convert::TryInto::<IAudioInputNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<MediaSourceAudioInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioNode> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioNode> {
        ::std::convert::TryInto::<IAudioNode>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<MediaSourceAudioInputNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaSourceAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MediaSourceAudioInputNode> for IAudioInputNode2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaSourceAudioInputNode) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioInputNode2> for &MediaSourceAudioInputNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioInputNode2> {
        ::std::convert::TryInto::<IAudioInputNode2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MediaSourceAudioInputNode {}
unsafe impl ::std::marker::Sync for MediaSourceAudioInputNode {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: MediaSourceAudioInputNodeCreationStatus =
        MediaSourceAudioInputNodeCreationStatus(0i32);
    pub const FormatNotSupported: MediaSourceAudioInputNodeCreationStatus =
        MediaSourceAudioInputNodeCreationStatus(1i32);
    pub const NetworkError: MediaSourceAudioInputNodeCreationStatus =
        MediaSourceAudioInputNodeCreationStatus(2i32);
    pub const UnknownFailure: MediaSourceAudioInputNodeCreationStatus =
        MediaSourceAudioInputNodeCreationStatus(3i32);
}
impl ::std::convert::From<i32> for MediaSourceAudioInputNodeCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaSourceAudioInputNodeCreationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaSourceAudioInputNodeCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.MediaSourceAudioInputNodeCreationStatus;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: MixedRealitySpatialAudioFormatPolicy =
        MixedRealitySpatialAudioFormatPolicy(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat:
        MixedRealitySpatialAudioFormatPolicy = MixedRealitySpatialAudioFormatPolicy(1i32);
}
impl ::std::convert::From<i32> for MixedRealitySpatialAudioFormatPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MixedRealitySpatialAudioFormatPolicy {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MixedRealitySpatialAudioFormatPolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.MixedRealitySpatialAudioFormatPolicy;i4)",
    );
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: QuantumSizeSelectionMode = QuantumSizeSelectionMode(0i32);
    pub const LowestLatency: QuantumSizeSelectionMode = QuantumSizeSelectionMode(1i32);
    pub const ClosestToDesired: QuantumSizeSelectionMode = QuantumSizeSelectionMode(2i32);
}
impl ::std::convert::From<i32> for QuantumSizeSelectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QuantumSizeSelectionMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for QuantumSizeSelectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.QuantumSizeSelectionMode;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ReverbEffectDefinition(::windows::runtime::IInspectable);
impl ReverbEffectDefinition {
    pub fn SetWetDryMix(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn WetDryMix(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetReflectionsDelay(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn ReflectionsDelay(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    pub fn SetReverbDelay(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ReverbDelay(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetRearDelay(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn RearDelay(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionLeft(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PositionLeft(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionRight(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PositionRight(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionMatrixLeft(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PositionMatrixLeft(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetPositionMatrixRight(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PositionMatrixRight(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetEarlyDiffusion(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn EarlyDiffusion(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetLateDiffusion(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn LateDiffusion(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetLowEQGain(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn LowEQGain(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetLowEQCutoff(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn LowEQCutoff(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetHighEQGain(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighEQGain(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetHighEQCutoff(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn HighEQCutoff(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u8>(result__)
        }
    }
    pub fn SetRoomFilterFreq(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn RoomFilterFreq(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomFilterMain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn RoomFilterMain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomFilterHF(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn RoomFilterHF(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetReflectionsGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ReflectionsGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetReverbGain(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ReverbGain(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDecayTime(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn DecayTime(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDensity(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn Density(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetRoomSize(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn RoomSize(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        }
    }
    pub fn SetDisableLateField(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn DisableLateField(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this =
            &::windows::runtime::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this =
            &::windows::runtime::Interface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, AudioGraph>>(
        audiograph: Param0,
    ) -> ::windows::runtime::Result<ReverbEffectDefinition> {
        Self::IReverbEffectDefinitionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                audiograph.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ReverbEffectDefinition>(result__)
        })
    }
    pub fn IReverbEffectDefinitionFactory<
        R,
        F: FnOnce(&IReverbEffectDefinitionFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ReverbEffectDefinition,
            IReverbEffectDefinitionFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ReverbEffectDefinition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Media.Audio.ReverbEffectDefinition;{4606aa89-f563-4d0a-8f6e-f0cddff35d84})",
    );
}
unsafe impl ::windows::runtime::Interface for ReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1174841993,
        62819,
        19722,
        [143, 110, 240, 205, 223, 243, 93, 132],
    );
}
impl ::windows::runtime::RuntimeName for ReverbEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.ReverbEffectDefinition";
}
impl ::std::convert::From<ReverbEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: ReverbEffectDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ReverbEffectDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &ReverbEffectDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ReverbEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &ReverbEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ReverbEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: ReverbEffectDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ReverbEffectDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &ReverbEffectDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ReverbEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ReverbEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::std::convert::TryFrom<ReverbEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ReverbEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Effects")]
impl ::std::convert::TryFrom<&ReverbEffectDefinition> for super::Effects::IAudioEffectDefinition {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ReverbEffectDefinition) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>
    for ReverbEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Effects::IAudioEffectDefinition>
    for &ReverbEffectDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Effects::IAudioEffectDefinition> {
        ::std::convert::TryInto::<super::Effects::IAudioEffectDefinition>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ReverbEffectDefinition {}
unsafe impl ::std::marker::Sync for ReverbEffectDefinition {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SetDefaultSpatialAudioFormatResult(::windows::runtime::IInspectable);
impl SetDefaultSpatialAudioFormatResult {
    pub fn Status(&self) -> ::windows::runtime::Result<SetDefaultSpatialAudioFormatStatus> {
        let this = self;
        unsafe {
            let mut result__: SetDefaultSpatialAudioFormatStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SetDefaultSpatialAudioFormatStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SetDefaultSpatialAudioFormatResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.SetDefaultSpatialAudioFormatResult;{1c2aa511-1400-5e70-9ea9-ae151241e8ea})" ) ;
}
unsafe impl ::windows::runtime::Interface for SetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        472556817,
        5120,
        24176,
        [158, 169, 174, 21, 18, 65, 232, 234],
    );
}
impl ::windows::runtime::RuntimeName for SetDefaultSpatialAudioFormatResult {
    const NAME: &'static str = "Windows.Media.Audio.SetDefaultSpatialAudioFormatResult";
}
impl ::std::convert::From<SetDefaultSpatialAudioFormatResult> for ::windows::runtime::IUnknown {
    fn from(value: SetDefaultSpatialAudioFormatResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SetDefaultSpatialAudioFormatResult> for ::windows::runtime::IUnknown {
    fn from(value: &SetDefaultSpatialAudioFormatResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SetDefaultSpatialAudioFormatResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SetDefaultSpatialAudioFormatResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SetDefaultSpatialAudioFormatResult> for ::windows::runtime::IInspectable {
    fn from(value: SetDefaultSpatialAudioFormatResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SetDefaultSpatialAudioFormatResult>
    for ::windows::runtime::IInspectable
{
    fn from(value: &SetDefaultSpatialAudioFormatResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SetDefaultSpatialAudioFormatResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SetDefaultSpatialAudioFormatResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SetDefaultSpatialAudioFormatResult {}
unsafe impl ::std::marker::Sync for SetDefaultSpatialAudioFormatResult {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: SetDefaultSpatialAudioFormatStatus =
        SetDefaultSpatialAudioFormatStatus(0i32);
    pub const AccessDenied: SetDefaultSpatialAudioFormatStatus =
        SetDefaultSpatialAudioFormatStatus(1i32);
    pub const LicenseExpired: SetDefaultSpatialAudioFormatStatus =
        SetDefaultSpatialAudioFormatStatus(2i32);
    pub const LicenseNotValidForAudioEndpoint: SetDefaultSpatialAudioFormatStatus =
        SetDefaultSpatialAudioFormatStatus(3i32);
    pub const NotSupportedOnAudioEndpoint: SetDefaultSpatialAudioFormatStatus =
        SetDefaultSpatialAudioFormatStatus(4i32);
    pub const UnknownError: SetDefaultSpatialAudioFormatStatus =
        SetDefaultSpatialAudioFormatStatus(5i32);
}
impl ::std::convert::From<i32> for SetDefaultSpatialAudioFormatStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SetDefaultSpatialAudioFormatStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SetDefaultSpatialAudioFormatStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.SetDefaultSpatialAudioFormatStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialAudioDeviceConfiguration(::windows::runtime::IInspectable);
impl SpatialAudioDeviceConfiguration {
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn IsSpatialAudioSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn IsSpatialAudioFormatSupported<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        subtype: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                subtype.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ActiveSpatialAudioFormat(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DefaultSpatialAudioFormat(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultSpatialAudioFormatAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        subtype: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                subtype.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                SetDefaultSpatialAudioFormatResult,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConfigurationChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                SpatialAudioDeviceConfiguration,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn GetForDeviceId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        deviceid: Param0,
    ) -> ::windows::runtime::Result<SpatialAudioDeviceConfiguration> {
        Self::ISpatialAudioDeviceConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                deviceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<SpatialAudioDeviceConfiguration>(result__)
        })
    }
    pub fn ISpatialAudioDeviceConfigurationStatics<
        R,
        F: FnOnce(&ISpatialAudioDeviceConfigurationStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialAudioDeviceConfiguration,
            ISpatialAudioDeviceConfigurationStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAudioDeviceConfiguration {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.SpatialAudioDeviceConfiguration;{ee830034-61cf-5749-9da4-10f0fe028199})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4001562676,
        25039,
        22345,
        [157, 164, 16, 240, 254, 2, 129, 153],
    );
}
impl ::windows::runtime::RuntimeName for SpatialAudioDeviceConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioDeviceConfiguration";
}
impl ::std::convert::From<SpatialAudioDeviceConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: SpatialAudioDeviceConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialAudioDeviceConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialAudioDeviceConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialAudioDeviceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialAudioDeviceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialAudioDeviceConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: SpatialAudioDeviceConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialAudioDeviceConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialAudioDeviceConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialAudioDeviceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialAudioDeviceConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialAudioDeviceConfiguration {}
unsafe impl ::std::marker::Sync for SpatialAudioDeviceConfiguration {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpatialAudioFormatConfiguration(::windows::runtime::IInspectable);
impl SpatialAudioFormatConfiguration {
    #[cfg(feature = "Foundation")]
    pub fn ReportLicenseChangedAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        subtype: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                subtype.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportConfigurationChangedAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        subtype: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                subtype.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn MixedRealityExclusiveModePolicy(
        &self,
    ) -> ::windows::runtime::Result<MixedRealitySpatialAudioFormatPolicy> {
        let this = self;
        unsafe {
            let mut result__: MixedRealitySpatialAudioFormatPolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<MixedRealitySpatialAudioFormatPolicy>(result__)
        }
    }
    pub fn SetMixedRealityExclusiveModePolicy(
        &self,
        value: MixedRealitySpatialAudioFormatPolicy,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn GetDefault() -> ::windows::runtime::Result<SpatialAudioFormatConfiguration> {
        Self::ISpatialAudioFormatConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<SpatialAudioFormatConfiguration>(result__)
        })
    }
    pub fn ISpatialAudioFormatConfigurationStatics<
        R,
        F: FnOnce(&ISpatialAudioFormatConfigurationStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialAudioFormatConfiguration,
            ISpatialAudioFormatConfigurationStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAudioFormatConfiguration {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.SpatialAudioFormatConfiguration;{32df09a8-50f0-5395-9923-7d44ca71ed6d})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        853477800,
        20720,
        21397,
        [153, 35, 125, 68, 202, 113, 237, 109],
    );
}
impl ::windows::runtime::RuntimeName for SpatialAudioFormatConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatConfiguration";
}
impl ::std::convert::From<SpatialAudioFormatConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: SpatialAudioFormatConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SpatialAudioFormatConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &SpatialAudioFormatConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpatialAudioFormatConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &SpatialAudioFormatConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<SpatialAudioFormatConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: SpatialAudioFormatConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpatialAudioFormatConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &SpatialAudioFormatConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpatialAudioFormatConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpatialAudioFormatConfiguration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpatialAudioFormatConfiguration {}
unsafe impl ::std::marker::Sync for SpatialAudioFormatConfiguration {}
pub struct SpatialAudioFormatSubtype {}
impl SpatialAudioFormatSubtype {
    pub fn WindowsSonic() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForHeadphones() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForHomeTheater() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn DolbyAtmosForSpeakers() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn DTSHeadphoneX() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn DTSXUltra() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn DTSXForHomeTheater() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn ISpatialAudioFormatSubtypeStatics<
        R,
        F: FnOnce(&ISpatialAudioFormatSubtypeStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialAudioFormatSubtype,
            ISpatialAudioFormatSubtypeStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpatialAudioFormatSubtypeStatics2<
        R,
        F: FnOnce(&ISpatialAudioFormatSubtypeStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpatialAudioFormatSubtype,
            ISpatialAudioFormatSubtypeStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SpatialAudioFormatSubtype {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatSubtype";
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: SpatialAudioModel = SpatialAudioModel(0i32);
    pub const FoldDown: SpatialAudioModel = SpatialAudioModel(1i32);
}
impl ::std::convert::From<i32> for SpatialAudioModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SpatialAudioModel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SpatialAudioModel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Media.Audio.SpatialAudioModel;i4)",
    );
}
