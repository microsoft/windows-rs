#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_MediaProperties`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AudioEncodingProperties(::windows::runtime::IInspectable);
impl AudioEncodingProperties {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AudioEncodingProperties, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetBitrate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Bitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetChannelCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn ChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetSampleRate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SampleRate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetBitsPerSample(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn BitsPerSample(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetFormatUserData(&self, value: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioEncodingPropertiesWithFormatUserData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn GetFormatUserData(&self, value: &mut ::windows::runtime::Array<u8>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAudioEncodingPropertiesWithFormatUserData>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<MediaPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetSubtype<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Subtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateAac(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::runtime::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), samplerate, channelcount, bitrate, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateAacAdts(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::runtime::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), samplerate, channelcount, bitrate, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateMp3(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::runtime::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), samplerate, channelcount, bitrate, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreatePcm(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::runtime::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), samplerate, channelcount, bitspersample, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateWma(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::runtime::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), samplerate, channelcount, bitrate, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn IsSpatial(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IAudioEncodingProperties2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateAlac(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::runtime::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), samplerate, channelcount, bitspersample, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateFlac(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::runtime::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), samplerate, channelcount, bitspersample, &mut result__).from_abi::<AudioEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Copy(&self) -> ::windows::runtime::Result<AudioEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IAudioEncodingProperties3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AudioEncodingProperties>(result__)
        }
    }
    pub fn IAudioEncodingPropertiesStatics<R, F: FnOnce(&IAudioEncodingPropertiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AudioEncodingProperties, IAudioEncodingPropertiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAudioEncodingPropertiesStatics2<R, F: FnOnce(&IAudioEncodingPropertiesStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AudioEncodingProperties, IAudioEncodingPropertiesStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AudioEncodingProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.AudioEncodingProperties;{62bc7a16-005c-4b3b-8a0b-0a090e9687f3})");
}
unsafe impl ::windows::runtime::Interface for AudioEncodingProperties {
    type Vtable = IAudioEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1656519190, 92, 19259, [138, 11, 10, 9, 14, 150, 135, 243]);
}
impl ::windows::runtime::RuntimeName for AudioEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.AudioEncodingProperties";
}
impl ::std::convert::From<AudioEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: AudioEncodingProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AudioEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: &AudioEncodingProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AudioEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AudioEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AudioEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: AudioEncodingProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AudioEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: &AudioEncodingProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AudioEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AudioEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<AudioEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AudioEncodingProperties) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AudioEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AudioEncodingProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for AudioEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for &AudioEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::std::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AudioEncodingProperties {}
unsafe impl ::std::marker::Sync for AudioEncodingProperties {}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioEncodingQuality(pub i32);
impl AudioEncodingQuality {
    pub const Auto: AudioEncodingQuality = AudioEncodingQuality(0i32);
    pub const High: AudioEncodingQuality = AudioEncodingQuality(1i32);
    pub const Medium: AudioEncodingQuality = AudioEncodingQuality(2i32);
    pub const Low: AudioEncodingQuality = AudioEncodingQuality(3i32);
}
impl ::std::convert::From<i32> for AudioEncodingQuality {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AudioEncodingQuality {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AudioEncodingQuality {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.AudioEncodingQuality;i4)");
}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContainerEncodingProperties(::windows::runtime::IInspectable);
impl ContainerEncodingProperties {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContainerEncodingProperties, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<MediaPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetSubtype<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Subtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Copy(&self) -> ::windows::runtime::Result<ContainerEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IContainerEncodingProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContainerEncodingProperties>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContainerEncodingProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.ContainerEncodingProperties;{59ac2a57-b32a-479e-8a61-4b7f2e9e7ea0})");
}
unsafe impl ::windows::runtime::Interface for ContainerEncodingProperties {
    type Vtable = IContainerEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1504455255, 45866, 18334, [138, 97, 75, 127, 46, 158, 126, 160]);
}
impl ::windows::runtime::RuntimeName for ContainerEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.ContainerEncodingProperties";
}
impl ::std::convert::From<ContainerEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: ContainerEncodingProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContainerEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: &ContainerEncodingProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContainerEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContainerEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContainerEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: ContainerEncodingProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContainerEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: &ContainerEncodingProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContainerEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContainerEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ContainerEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContainerEncodingProperties) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ContainerEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContainerEncodingProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for ContainerEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for &ContainerEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::std::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ContainerEncodingProperties {}
unsafe impl ::std::marker::Sync for ContainerEncodingProperties {}
#[doc = "*Required features: `Media_MediaProperties`*"]
pub struct H264ProfileIds {}
impl H264ProfileIds {
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn ConstrainedBaseline() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Baseline() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Extended() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Main() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn High() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn High10() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn High422() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn High444() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn StereoHigh() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn MultiviewHigh() -> ::windows::runtime::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn IH264ProfileIdsStatics<R, F: FnOnce(&IH264ProfileIdsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<H264ProfileIds, IH264ProfileIdsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for H264ProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.H264ProfileIds";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAudioEncodingProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEncodingProperties {
    type Vtable = IAudioEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1656519190, 92, 19259, [138, 11, 10, 9, 14, 150, 135, 243]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAudioEncodingProperties2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEncodingProperties2 {
    type Vtable = IAudioEncodingProperties2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3294450906, 32957, 19491, [128, 213, 114, 212, 161, 129, 232, 148]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAudioEncodingProperties3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEncodingProperties3 {
    type Vtable = IAudioEncodingProperties3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2271216449, 29836, 20365, [176, 253, 16, 202, 240, 143, 240, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEncodingPropertiesStatics {
    type Vtable = IAudioEncodingPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(212677420, 60393, 17703, [179, 109, 228, 42, 19, 207, 56, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEncodingPropertiesStatics2 {
    type Vtable = IAudioEncodingPropertiesStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1955148143, 30624, 17213, [142, 213, 64, 64, 40, 14, 134, 101]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesWithFormatUserData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAudioEncodingPropertiesWithFormatUserData {
    type Vtable = IAudioEncodingPropertiesWithFormatUserData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2565934457, 5098, 18943, [190, 112, 38, 115, 219, 105, 112, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesWithFormatUserData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IContainerEncodingProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContainerEncodingProperties {
    type Vtable = IContainerEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1504455255, 45866, 18334, [138, 97, 75, 127, 46, 158, 126, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerEncodingProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IContainerEncodingProperties2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContainerEncodingProperties2 {
    type Vtable = IContainerEncodingProperties2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2993864745, 44582, 18457, [186, 173, 173, 122, 73, 176, 168, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerEncodingProperties2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IH264ProfileIdsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IH264ProfileIdsStatics {
    type Vtable = IH264ProfileIdsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(946162855, 33898, 20375, [162, 229, 195, 161, 91, 191, 112, 253]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IH264ProfileIdsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageEncodingProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageEncodingProperties {
    type Vtable = IImageEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2019710517, 62257, 16777, [177, 195, 180, 141, 90, 224, 52, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageEncodingProperties2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageEncodingProperties2 {
    type Vtable = IImageEncodingProperties2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3360989919, 51491, 18075, [172, 142, 106, 159, 60, 28, 217, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingProperties2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageEncodingPropertiesStatics {
    type Vtable = IImageEncodingPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(628910300, 35737, 17310, [170, 89, 145, 58, 54, 22, 18, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageEncodingPropertiesStatics2 {
    type Vtable = IImageEncodingPropertiesStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4139932457, 14372, 18096, [149, 110, 80, 19, 41, 225, 190, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: MediaPixelFormat, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IImageEncodingPropertiesStatics3 {
    type Vtable = IImageEncodingPropertiesStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1223983437, 41727, 18652, [142, 160, 233, 6, 128, 102, 54, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingProfile(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingProfile {
    type Vtable = IMediaEncodingProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3889952168, 7609, 18307, [135, 107, 61, 254, 18, 172, 253, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingProfile2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingProfile2 {
    type Vtable = IMediaEncodingProfile2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(882589194, 16437, 18574, [152, 119, 133, 99, 40, 101, 237, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingProfile3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingProfile3 {
    type Vtable = IMediaEncodingProfile3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3127819912, 30064, 20073, [172, 207, 86, 17, 173, 1, 95, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingProfileStatics {
    type Vtable = IMediaEncodingProfileStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(427767084, 11998, 19013, [168, 150, 129, 122, 72, 84, 248, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: AudioEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: AudioEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: AudioEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: VideoEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: VideoEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingProfileStatics2 {
    type Vtable = IMediaEncodingProfileStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3465406287, 27380, 17032, [143, 226, 121, 173, 241, 247, 154, 67]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: AudioEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: VideoEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingProfileStatics3 {
    type Vtable = IMediaEncodingProfileStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2430256554, 53110, 17044, [169, 237, 26, 20, 32, 245, 31, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: AudioEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: AudioEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quality: VideoEncodingQuality, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Media_MediaProperties`*"]
pub struct IMediaEncodingProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingProperties {
    type Vtable = IMediaEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3019909878, 44244, 20058, [162, 75, 93, 116, 152, 168, 184, 196]);
}
impl IMediaEncodingProperties {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<MediaPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetSubtype<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Subtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMediaEncodingProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b4002af6-acd4-4e5a-a24b-5d7498a8b8c4}");
}
impl ::std::convert::From<IMediaEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: IMediaEncodingProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMediaEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaEncodingProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMediaEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMediaEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: IMediaEncodingProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMediaEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: &IMediaEncodingProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMediaEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMediaEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingSubtypesStatics {
    type Vtable = IMediaEncodingSubtypesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(934696974, 41329, 17508, [186, 90, 83, 24, 158, 72, 193, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingSubtypesStatics2 {
    type Vtable = IMediaEncodingSubtypesStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1266471485, 17151, 19763, [133, 49, 6, 38, 190, 228, 181, 45]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingSubtypesStatics3 {
    type Vtable = IMediaEncodingSubtypesStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3122926820, 34877, 17998, [164, 79, 9, 125, 160, 142, 247, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingSubtypesStatics4 {
    type Vtable = IMediaEncodingSubtypesStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3723289994, 14665, 17988, [138, 44, 89, 239, 2, 198, 66, 250]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingSubtypesStatics5 {
    type Vtable = IMediaEncodingSubtypesStatics5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1523884039, 65486, 18272, [152, 40, 93, 12, 153, 99, 126, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics6(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaEncodingSubtypesStatics6 {
    type Vtable = IMediaEncodingSubtypesStatics6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2703567219, 43396, 22802, [147, 187, 84, 231, 229, 105, 224, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaRatio(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaRatio {
    type Vtable = IMediaRatio_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3536912101, 35113, 16413, [172, 120, 125, 53, 126, 55, 129, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRatio_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMpeg2ProfileIdsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMpeg2ProfileIdsStatics {
    type Vtable = IMpeg2ProfileIdsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2757885829, 58746, 16680, [155, 33, 213, 51, 27, 4, 35, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMpeg2ProfileIdsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITimedMetadataEncodingProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITimedMetadataEncodingProperties {
    type Vtable = ITimedMetadataEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1372401875, 54928, 19706, [151, 244, 74, 57, 142, 157, 180, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataEncodingProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITimedMetadataEncodingPropertiesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITimedMetadataEncodingPropertiesStatics {
    type Vtable = ITimedMetadataEncodingPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1714010983, 28245, 22083, [137, 160, 122, 126, 141, 133, 181, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataEncodingPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IVideoEncodingProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEncodingProperties {
    type Vtable = IVideoEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1995336858, 14274, 20266, [136, 10, 18, 130, 187, 180, 55, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IVideoEncodingProperties2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEncodingProperties2 {
    type Vtable = IVideoEncodingProperties2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4148404719, 54373, 17040, [169, 75, 239, 15, 21, 40, 248, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IVideoEncodingProperties3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEncodingProperties3 {
    type Vtable = IVideoEncodingProperties3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(946589124, 34618, 18335, [179, 235, 86, 193, 252, 190, 198, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut StereoscopicVideoPackingMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IVideoEncodingProperties4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEncodingProperties4 {
    type Vtable = IVideoEncodingProperties4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1917775892, 49420, 16626, [157, 114, 62, 225, 59, 69, 250, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SphericalVideoFrameFormat) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IVideoEncodingProperties5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEncodingProperties5 {
    type Vtable = IVideoEncodingProperties5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1230571535, 10031, 20174, [164, 223, 192, 204, 219, 51, 216, 64]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEncodingPropertiesStatics {
    type Vtable = IVideoEncodingPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1021398340, 7621, 17371, [159, 56, 235, 235, 249, 1, 82, 203]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, width: u32, height: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVideoEncodingPropertiesStatics2 {
    type Vtable = IVideoEncodingPropertiesStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3474898269, 18942, 19712, [181, 154, 207, 164, 223, 197, 25, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Media_MediaProperties`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ImageEncodingProperties(::windows::runtime::IInspectable);
impl ImageEncodingProperties {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ImageEncodingProperties, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetWidth(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<MediaPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetSubtype<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Subtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateJpeg() -> ::windows::runtime::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreatePng() -> ::windows::runtime::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateJpegXR() -> ::windows::runtime::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateUncompressed(format: MediaPixelFormat) -> ::windows::runtime::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateBmp() -> ::windows::runtime::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Copy(&self) -> ::windows::runtime::Result<ImageEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IImageEncodingProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateHeif() -> ::windows::runtime::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ImageEncodingProperties>(result__)
        })
    }
    pub fn IImageEncodingPropertiesStatics<R, F: FnOnce(&IImageEncodingPropertiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IImageEncodingPropertiesStatics2<R, F: FnOnce(&IImageEncodingPropertiesStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IImageEncodingPropertiesStatics3<R, F: FnOnce(&IImageEncodingPropertiesStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ImageEncodingProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.ImageEncodingProperties;{78625635-f331-4189-b1c3-b48d5ae034f1})");
}
unsafe impl ::windows::runtime::Interface for ImageEncodingProperties {
    type Vtable = IImageEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2019710517, 62257, 16777, [177, 195, 180, 141, 90, 224, 52, 241]);
}
impl ::windows::runtime::RuntimeName for ImageEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.ImageEncodingProperties";
}
impl ::std::convert::From<ImageEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: ImageEncodingProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ImageEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: &ImageEncodingProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ImageEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ImageEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ImageEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: ImageEncodingProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ImageEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: &ImageEncodingProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ImageEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ImageEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ImageEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ImageEncodingProperties) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ImageEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ImageEncodingProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for ImageEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for &ImageEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::std::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ImageEncodingProperties {}
unsafe impl ::std::marker::Sync for ImageEncodingProperties {}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaEncodingProfile(::windows::runtime::IInspectable);
impl MediaEncodingProfile {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingProfile, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetAudio<'a, Param0: ::windows::runtime::IntoParam<'a, AudioEncodingProperties>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Audio(&self) -> ::windows::runtime::Result<AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AudioEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetVideo<'a, Param0: ::windows::runtime::IntoParam<'a, VideoEncodingProperties>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Video(&self) -> ::windows::runtime::Result<VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetContainer<'a, Param0: ::windows::runtime::IntoParam<'a, ContainerEncodingProperties>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Container(&self) -> ::windows::runtime::Result<ContainerEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContainerEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateM4a(quality: AudioEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateMp3(quality: AudioEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateWma(quality: AudioEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateMp4(quality: VideoEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateWmv(quality: VideoEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation`, `Storage`*"]
    pub fn CreateFromFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation`, `Storage_Streams`*"]
    pub fn CreateFromStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(stream: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateWav(quality: AudioEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateAvi(quality: VideoEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateAlac(quality: AudioEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateFlac(quality: AudioEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateHevc(quality: VideoEncodingQuality) -> ::windows::runtime::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), quality, &mut result__).from_abi::<MediaEncodingProfile>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`, `Media_Core`*"]
    pub fn SetAudioTracks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`, `Media_Core`*"]
    pub fn GetAudioTracks(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Core::AudioStreamDescriptor>> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Core::AudioStreamDescriptor>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`, `Media_Core`*"]
    pub fn SetVideoTracks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`, `Media_Core`*"]
    pub fn GetVideoTracks(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Core::VideoStreamDescriptor>> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Core::VideoStreamDescriptor>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`, `Media_Core`*"]
    pub fn SetTimedMetadataTracks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProfile3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`, `Media_Core`*"]
    pub fn GetTimedMetadataTracks(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Core::TimedMetadataStreamDescriptor>> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProfile3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Core::TimedMetadataStreamDescriptor>>(result__)
        }
    }
    pub fn IMediaEncodingProfileStatics<R, F: FnOnce(&IMediaEncodingProfileStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaEncodingProfileStatics2<R, F: FnOnce(&IMediaEncodingProfileStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaEncodingProfileStatics3<R, F: FnOnce(&IMediaEncodingProfileStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaEncodingProfile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.MediaEncodingProfile;{e7dbf5a8-1db9-4783-876b-3dfe12acfdb3})");
}
unsafe impl ::windows::runtime::Interface for MediaEncodingProfile {
    type Vtable = IMediaEncodingProfile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3889952168, 7609, 18307, [135, 107, 61, 254, 18, 172, 253, 179]);
}
impl ::windows::runtime::RuntimeName for MediaEncodingProfile {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaEncodingProfile";
}
impl ::std::convert::From<MediaEncodingProfile> for ::windows::runtime::IUnknown {
    fn from(value: MediaEncodingProfile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaEncodingProfile> for ::windows::runtime::IUnknown {
    fn from(value: &MediaEncodingProfile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaEncodingProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaEncodingProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaEncodingProfile> for ::windows::runtime::IInspectable {
    fn from(value: MediaEncodingProfile) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaEncodingProfile> for ::windows::runtime::IInspectable {
    fn from(value: &MediaEncodingProfile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaEncodingProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaEncodingProfile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaEncodingProfile {}
unsafe impl ::std::marker::Sync for MediaEncodingProfile {}
#[doc = "*Required features: `Media_MediaProperties`*"]
pub struct MediaEncodingSubtypes {}
impl MediaEncodingSubtypes {
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Aac() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn AacAdts() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Ac3() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn AmrNb() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn AmrWb() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Argb32() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Asf() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Avi() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Bgra8() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Bmp() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Eac3() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Float() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Gif() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn H263() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn H264() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn H264Es() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Hevc() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn HevcEs() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Iyuv() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Jpeg() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn JpegXr() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Mjpg() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Mpeg() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Mpeg1() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Mpeg2() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Mp3() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Mpeg4() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Nv12() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Pcm() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Png() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Rgb24() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Rgb32() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Tiff() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Wave() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Wma8() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Wma9() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Wmv3() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Wvc1() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Yuy2() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Yv12() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Vp9() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn L8() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn L16() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn D16() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Alac() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics3(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Flac() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics3(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn P010() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Heif() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics5(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Pgs() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Srt() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Ssa() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn VobSub() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IMediaEncodingSubtypesStatics<R, F: FnOnce(&IMediaEncodingSubtypesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaEncodingSubtypesStatics2<R, F: FnOnce(&IMediaEncodingSubtypesStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaEncodingSubtypesStatics3<R, F: FnOnce(&IMediaEncodingSubtypesStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaEncodingSubtypesStatics4<R, F: FnOnce(&IMediaEncodingSubtypesStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaEncodingSubtypesStatics5<R, F: FnOnce(&IMediaEncodingSubtypesStatics5) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics5> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaEncodingSubtypesStatics6<R, F: FnOnce(&IMediaEncodingSubtypesStatics6) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics6> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MediaEncodingSubtypes {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaEncodingSubtypes";
}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaMirroringOptions(pub u32);
impl MediaMirroringOptions {
    pub const None: MediaMirroringOptions = MediaMirroringOptions(0u32);
    pub const Horizontal: MediaMirroringOptions = MediaMirroringOptions(1u32);
    pub const Vertical: MediaMirroringOptions = MediaMirroringOptions(2u32);
}
impl ::std::convert::From<u32> for MediaMirroringOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaMirroringOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaMirroringOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaMirroringOptions;u4)");
}
impl ::std::ops::BitOr for MediaMirroringOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MediaMirroringOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MediaMirroringOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MediaMirroringOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MediaMirroringOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaPixelFormat(pub i32);
impl MediaPixelFormat {
    pub const Nv12: MediaPixelFormat = MediaPixelFormat(0i32);
    pub const Bgra8: MediaPixelFormat = MediaPixelFormat(1i32);
    pub const P010: MediaPixelFormat = MediaPixelFormat(2i32);
}
impl ::std::convert::From<i32> for MediaPixelFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaPixelFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaPixelFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaPixelFormat;i4)");
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaPropertySet(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl MediaPropertySet {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaPropertySet, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::GUID, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for MediaPropertySet {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.MediaPropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};g16;cinterface(IInspectable)))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for MediaPropertySet {
    type Vtable = super::super::Foundation::Collections::IMap_abi<::windows::runtime::GUID, ::windows::runtime::IInspectable>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for MediaPropertySet {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaPropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<MediaPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: MediaPropertySet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&MediaPropertySet> for ::windows::runtime::IUnknown {
    fn from(value: &MediaPropertySet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<MediaPropertySet> for ::windows::runtime::IInspectable {
    fn from(value: MediaPropertySet) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&MediaPropertySet> for ::windows::runtime::IInspectable {
    fn from(value: &MediaPropertySet) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<MediaPropertySet> for super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable> {
    fn from(value: MediaPropertySet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&MediaPropertySet> for super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable> {
    fn from(value: &MediaPropertySet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable>> for MediaPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable>> for &MediaPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::Collections::IMap<::windows::runtime::GUID, ::windows::runtime::IInspectable>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<MediaPropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MediaPropertySet) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&MediaPropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MediaPropertySet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>>> for MediaPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>>> for &MediaPropertySet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::std::marker::Send for MediaPropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::std::marker::Sync for MediaPropertySet {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for MediaPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &MediaPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::GUID, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaRatio(::windows::runtime::IInspectable);
impl MediaRatio {
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetNumerator(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Numerator(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetDenominator(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Denominator(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaRatio {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.MediaRatio;{d2d0fee5-8929-401d-ac78-7d357e378163})");
}
unsafe impl ::windows::runtime::Interface for MediaRatio {
    type Vtable = IMediaRatio_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3536912101, 35113, 16413, [172, 120, 125, 53, 126, 55, 129, 99]);
}
impl ::windows::runtime::RuntimeName for MediaRatio {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaRatio";
}
impl ::std::convert::From<MediaRatio> for ::windows::runtime::IUnknown {
    fn from(value: MediaRatio) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaRatio> for ::windows::runtime::IUnknown {
    fn from(value: &MediaRatio) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaRatio {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaRatio {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaRatio> for ::windows::runtime::IInspectable {
    fn from(value: MediaRatio) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaRatio> for ::windows::runtime::IInspectable {
    fn from(value: &MediaRatio) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaRatio {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaRatio {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaRatio {}
unsafe impl ::std::marker::Sync for MediaRatio {}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaRotation(pub i32);
impl MediaRotation {
    pub const None: MediaRotation = MediaRotation(0i32);
    pub const Clockwise90Degrees: MediaRotation = MediaRotation(1i32);
    pub const Clockwise180Degrees: MediaRotation = MediaRotation(2i32);
    pub const Clockwise270Degrees: MediaRotation = MediaRotation(3i32);
}
impl ::std::convert::From<i32> for MediaRotation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaRotation {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaRotation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaRotation;i4)");
}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaThumbnailFormat(pub i32);
impl MediaThumbnailFormat {
    pub const Bmp: MediaThumbnailFormat = MediaThumbnailFormat(0i32);
    pub const Bgra8: MediaThumbnailFormat = MediaThumbnailFormat(1i32);
}
impl ::std::convert::From<i32> for MediaThumbnailFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaThumbnailFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaThumbnailFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaThumbnailFormat;i4)");
}
#[doc = "*Required features: `Media_MediaProperties`*"]
pub struct Mpeg2ProfileIds {}
impl Mpeg2ProfileIds {
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Simple() -> ::windows::runtime::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Main() -> ::windows::runtime::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SignalNoiseRatioScalable() -> ::windows::runtime::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SpatiallyScalable() -> ::windows::runtime::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn High() -> ::windows::runtime::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn IMpeg2ProfileIdsStatics<R, F: FnOnce(&IMpeg2ProfileIdsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Mpeg2ProfileIds, IMpeg2ProfileIdsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for Mpeg2ProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.Mpeg2ProfileIds";
}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SphericalVideoFrameFormat(pub i32);
impl SphericalVideoFrameFormat {
    pub const None: SphericalVideoFrameFormat = SphericalVideoFrameFormat(0i32);
    pub const Unsupported: SphericalVideoFrameFormat = SphericalVideoFrameFormat(1i32);
    pub const Equirectangular: SphericalVideoFrameFormat = SphericalVideoFrameFormat(2i32);
}
impl ::std::convert::From<i32> for SphericalVideoFrameFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SphericalVideoFrameFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SphericalVideoFrameFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.SphericalVideoFrameFormat;i4)");
}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct StereoscopicVideoPackingMode(pub i32);
impl StereoscopicVideoPackingMode {
    pub const None: StereoscopicVideoPackingMode = StereoscopicVideoPackingMode(0i32);
    pub const SideBySide: StereoscopicVideoPackingMode = StereoscopicVideoPackingMode(1i32);
    pub const TopBottom: StereoscopicVideoPackingMode = StereoscopicVideoPackingMode(2i32);
}
impl ::std::convert::From<i32> for StereoscopicVideoPackingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StereoscopicVideoPackingMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for StereoscopicVideoPackingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.StereoscopicVideoPackingMode;i4)");
}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TimedMetadataEncodingProperties(::windows::runtime::IInspectable);
impl TimedMetadataEncodingProperties {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TimedMetadataEncodingProperties, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<MediaPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetSubtype<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Subtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetFormatUserData(&self, value: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn GetFormatUserData(&self, value: &mut ::windows::runtime::Array<u8>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Copy(&self) -> ::windows::runtime::Result<TimedMetadataEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreatePgs() -> ::windows::runtime::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateSrt() -> ::windows::runtime::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateSsa(formatuserdata: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), formatuserdata.len() as u32, ::std::mem::transmute(formatuserdata.as_ptr()), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateVobSub(formatuserdata: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), formatuserdata.len() as u32, ::std::mem::transmute(formatuserdata.as_ptr()), &mut result__).from_abi::<TimedMetadataEncodingProperties>(result__)
        })
    }
    pub fn ITimedMetadataEncodingPropertiesStatics<R, F: FnOnce(&ITimedMetadataEncodingPropertiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TimedMetadataEncodingProperties, ITimedMetadataEncodingPropertiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TimedMetadataEncodingProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.TimedMetadataEncodingProperties;{b4002af6-acd4-4e5a-a24b-5d7498a8b8c4})");
}
unsafe impl ::windows::runtime::Interface for TimedMetadataEncodingProperties {
    type Vtable = IMediaEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3019909878, 44244, 20058, [162, 75, 93, 116, 152, 168, 184, 196]);
}
impl ::windows::runtime::RuntimeName for TimedMetadataEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.TimedMetadataEncodingProperties";
}
impl ::std::convert::From<TimedMetadataEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: TimedMetadataEncodingProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TimedMetadataEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: &TimedMetadataEncodingProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TimedMetadataEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: TimedMetadataEncodingProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TimedMetadataEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: &TimedMetadataEncodingProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<TimedMetadataEncodingProperties> for IMediaEncodingProperties {
    fn from(value: TimedMetadataEncodingProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TimedMetadataEncodingProperties> for IMediaEncodingProperties {
    fn from(value: &TimedMetadataEncodingProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMediaEncodingProperties>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for &TimedMetadataEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMediaEncodingProperties>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for TimedMetadataEncodingProperties {}
unsafe impl ::std::marker::Sync for TimedMetadataEncodingProperties {}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VideoEncodingProperties(::windows::runtime::IInspectable);
impl VideoEncodingProperties {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoEncodingProperties, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetBitrate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Bitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetWidth(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn FrameRate(&self) -> ::windows::runtime::Result<MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn PixelAspectRatio(&self) -> ::windows::runtime::Result<MediaRatio> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaRatio>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_MediaProperties`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<MediaPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetSubtype<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Subtype(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetFormatUserData(&self, value: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn GetFormatUserData(&self, value: &mut ::windows::runtime::Array<u8>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SetProfileId(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn ProfileId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateH264() -> ::windows::runtime::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateMpeg2() -> ::windows::runtime::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateUncompressed<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(subtype: Param0, width: u32, height: u32) -> ::windows::runtime::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), subtype.into_param().abi(), width, height, &mut result__).from_abi::<VideoEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn StereoscopicVideoPackingMode(&self) -> ::windows::runtime::Result<StereoscopicVideoPackingMode> {
        let this = &::windows::runtime::Interface::cast::<IVideoEncodingProperties3>(self)?;
        unsafe {
            let mut result__: StereoscopicVideoPackingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StereoscopicVideoPackingMode>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn SphericalVideoFrameFormat(&self) -> ::windows::runtime::Result<SphericalVideoFrameFormat> {
        let this = &::windows::runtime::Interface::cast::<IVideoEncodingProperties4>(self)?;
        unsafe {
            let mut result__: SphericalVideoFrameFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SphericalVideoFrameFormat>(result__)
        }
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn CreateHevc() -> ::windows::runtime::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        })
    }
    #[doc = "*Required features: `Media_MediaProperties`*"]
    pub fn Copy(&self) -> ::windows::runtime::Result<VideoEncodingProperties> {
        let this = &::windows::runtime::Interface::cast::<IVideoEncodingProperties5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VideoEncodingProperties>(result__)
        }
    }
    pub fn IVideoEncodingPropertiesStatics<R, F: FnOnce(&IVideoEncodingPropertiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoEncodingProperties, IVideoEncodingPropertiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IVideoEncodingPropertiesStatics2<R, F: FnOnce(&IVideoEncodingPropertiesStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<VideoEncodingProperties, IVideoEncodingPropertiesStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VideoEncodingProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProperties.VideoEncodingProperties;{76ee6c9a-37c2-4f2a-880a-1282bbb4373d})");
}
unsafe impl ::windows::runtime::Interface for VideoEncodingProperties {
    type Vtable = IVideoEncodingProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1995336858, 14274, 20266, [136, 10, 18, 130, 187, 180, 55, 61]);
}
impl ::windows::runtime::RuntimeName for VideoEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.VideoEncodingProperties";
}
impl ::std::convert::From<VideoEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: VideoEncodingProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VideoEncodingProperties> for ::windows::runtime::IUnknown {
    fn from(value: &VideoEncodingProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VideoEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VideoEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VideoEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: VideoEncodingProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VideoEncodingProperties> for ::windows::runtime::IInspectable {
    fn from(value: &VideoEncodingProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VideoEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VideoEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<VideoEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VideoEncodingProperties) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&VideoEncodingProperties> for IMediaEncodingProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VideoEncodingProperties) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for VideoEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMediaEncodingProperties> for &VideoEncodingProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMediaEncodingProperties> {
        ::std::convert::TryInto::<IMediaEncodingProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for VideoEncodingProperties {}
unsafe impl ::std::marker::Sync for VideoEncodingProperties {}
#[doc = "*Required features: `Media_MediaProperties`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoEncodingQuality(pub i32);
impl VideoEncodingQuality {
    pub const Auto: VideoEncodingQuality = VideoEncodingQuality(0i32);
    pub const HD1080p: VideoEncodingQuality = VideoEncodingQuality(1i32);
    pub const HD720p: VideoEncodingQuality = VideoEncodingQuality(2i32);
    pub const Wvga: VideoEncodingQuality = VideoEncodingQuality(3i32);
    pub const Ntsc: VideoEncodingQuality = VideoEncodingQuality(4i32);
    pub const Pal: VideoEncodingQuality = VideoEncodingQuality(5i32);
    pub const Vga: VideoEncodingQuality = VideoEncodingQuality(6i32);
    pub const Qvga: VideoEncodingQuality = VideoEncodingQuality(7i32);
    pub const Uhd2160p: VideoEncodingQuality = VideoEncodingQuality(8i32);
    pub const Uhd4320p: VideoEncodingQuality = VideoEncodingQuality(9i32);
}
impl ::std::convert::From<i32> for VideoEncodingQuality {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VideoEncodingQuality {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VideoEncodingQuality {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.VideoEncodingQuality;i4)");
}
